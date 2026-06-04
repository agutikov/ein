"""ein-lang pattern macros — P1.8 S1.5.9 T1.5.9.2.

Two layers:

- The pure expander (:mod:`ein_bot.ir.macros`) — substitution, recursive
  (macro-invoking-macro) expansion, arity-mismatch + runaway-recursion
  rejection, and the identity property (a macro-free tree is unchanged).
- The loader integration (:mod:`ein_bot.kb.from_ir`) — a `(macro …)`
  declaration registers into ``kb.macros``; a rule clause's invocation is
  rewritten before compilation; reserved-name / duplicate rejection.

The migration of ``forall`` / ``open`` themselves (T1.5.9.3) is a later
step; here we exercise the mechanism with stand-alone macros plus a
user-defined ``forall`` that must reproduce the compile.py desugaring.
"""
from __future__ import annotations

import pytest

from ein_bot.ir import dump_compact, parse
from ein_bot.ir.macros import Macro, MacroError, expand_macros
from ein_bot.ir.types import Atom, SForm, Var
from ein_bot.kb.from_ir import KBLoadError
from ein_bot.kb.store import KnowledgeBase


def _macro(src: str) -> Macro:
    """Build a :class:`Macro` from one `(macro …)` source form."""
    (form,) = parse(src)
    name = form.args[0].name
    params = tuple(a.name for a in form.args[1].args if isinstance(a, Var))
    return Macro(name=name, params=params, body=form.args[2])


def _clause(src: str):
    """The single value of a `(probe :c <expr>)` fact — a handle on an
    arbitrary expression node to feed the expander (top-level bare
    `(and …)` / `(absent …)` are not valid forms on their own)."""
    (form,) = parse(f"(probe :c {src})")
    return form.kw_map()["c"]


# ── Pure expander ──────────────────────────────────────────────────


def test_simple_substitution():
    m = {"co": _macro("(macro co (?a ?b) (rel ?a ?b))")}
    out = expand_macros(_clause("(co X Y)"), m)
    assert dump_compact(out) == "(rel X Y)"


def test_substitution_into_nested_body():
    # A param may stand for a whole sub-tree, substituted structurally.
    m = {"forall": _macro("(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))))")}
    out = expand_macros(_clause("(forall ?q (player ?q) (beats ?p ?q))"), m)
    assert dump_compact(out) == (
        "(absent (and (player ?q) (absent (beats ?p ?q))))"
    )


def test_macro_free_tree_is_unchanged():
    out = expand_macros(_clause("(and (player ?p) (beats ?p ?q))"), {})
    assert out == _clause("(and (player ?p) (beats ?p ?q))")


def test_recursive_macro_expands_inside_out():
    macros = {
        "open": _macro("(macro open (?P) (and (absent ?P) (absent (not ?P))))"),
        "maybe": _macro("(macro maybe (?Q) (open ?Q))"),
    }
    out = expand_macros(_clause("(maybe (lives-in ?a ?b))"), macros)
    assert dump_compact(out) == (
        "(and (absent (lives-in ?a ?b)) (absent (not (lives-in ?a ?b))))"
    )


def test_arity_mismatch_rejected():
    m = {"co": _macro("(macro co (?a ?b) (rel ?a ?b))")}
    with pytest.raises(MacroError, match=r"co/2 invoked with 1 args"):
        expand_macros(_clause("(co X)"), m)


def test_self_expanding_macro_hits_depth_cap():
    loopy = {"loopy": Macro("loopy", ("x",), SForm(Atom("loopy"), (Var("x"),)))}
    with pytest.raises(MacroError, match=r"exceeded depth"):
        expand_macros(SForm(Atom("loopy"), (Atom("a"),)), loopy)


def test_relation_poly_head_substitution():
    # Forward-compat (S1.8.A7 imply/converse): a param in head position is
    # substituted when the replacement is atom-shaped.
    m = {"apply": _macro("(macro apply (?R ?a ?b) (?R ?a ?b))")}
    out = expand_macros(_clause("(apply right-of A B)"), m)
    assert dump_compact(out) == "(right-of A B)"


# ── Loader integration ─────────────────────────────────────────────


def _kb(src: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(src))


def test_macro_registers_in_kb():
    kb = _kb("(macro co (?a ?b) (rel ?a ?b))")
    assert set(kb.macros) == {"co"}
    assert kb.macros["co"].params == ("a", "b")


def test_macro_expands_in_rule_match():
    # A user-defined `forall` macro must compile to the same nested
    # AbsentGuard shape that compile.py's _desugar_forall produces.
    from ein_bot.inference.compile import AbsentGuard, compile_rule

    kb = _kb("""
    (macro forall (?b ?G ?B)
      (absent (and ?G (absent ?B))))
    (rule undefeated ()
      :match (and (player ?p)
                  (forall ?q
                    (and (player ?q) (neq ?p ?q))
                    (beats ?p ?q)))
      :assert (undefeated ?p)
      :why "u")
    (relation player T T) (relation beats T T)
    """)
    plan = compile_rule(kb.rules["undefeated"], None)
    outer = next(s for s in plan.steps if isinstance(s, AbsentGuard))
    assert any(isinstance(s, AbsentGuard) for s in outer.sub_steps)


def test_macro_arity_mismatch_is_load_error():
    with pytest.raises(KBLoadError, match=r"m/2 invoked with 1 args"):
        _kb("""
        (macro m (?a ?b) (rel ?a ?b))
        (rule x () :match (m ?p) :assert (y ?p) :why "w")
        """)


@pytest.mark.parametrize("name", ["absent", "false", "relation", "eq"])
def test_reserved_macro_name_rejected_at_load(name):
    # Valid SYMBOLs but reserved kernel vocabulary: the loader rejects them.
    with pytest.raises(KBLoadError, match=r"shadows a reserved kernel name"):
        _kb(f"(macro {name} (?p) (rel ?p))")


@pytest.mark.parametrize("name", ["not", "and", "or", "neq"])
def test_keyword_macro_name_rejected_at_parse(name):
    # SYMBOL-excluded keywords can't even be written as a macro NAME — the
    # negative-lookahead turns `(macro not …)` into a parse error.
    from ein_bot.ir import IRParseError
    with pytest.raises(IRParseError):
        parse(f"(macro {name} (?p) (rel ?p))")


@pytest.mark.parametrize("name", ["forall", "open"])
def test_forall_open_are_not_reserved_as_macro_names(name):
    # These are exactly the sugar slated to BECOME stdlib macros (T1.5.9.3),
    # so a puzzle must be allowed to define them.
    kb = _kb(f"(macro {name} (?p) (rel ?p))")
    assert name in kb.macros


def test_duplicate_macro_rejected():
    with pytest.raises(KBLoadError, match=r"duplicate macro 'm'"):
        _kb("(macro m (?a) (rel ?a)) (macro m (?a) (other ?a))")
