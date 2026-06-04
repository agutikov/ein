"""Typed-AST + round-trip tests (S1.1.2).

Covers:
- Each kernel form lowers to the expected `SForm` shape.
- Terminals lift to the right node types (Atom / Var / Keyword /
  Wildcard / String / Int / Range).
- `parse(dump_canonical(parse(text))) == parse(text)` modulo `Loc`
  for every example.
- A handful of broken fixtures raise `IRParseError` with
  `file:line:col` in the message.
"""
from pathlib import Path

import pytest

from ein_bot.ir import (
    Atom,
    Int,
    IRParseError,
    Keyword,
    KwPair,
    Range,
    SForm,
    String,
    Var,
    Wildcard,
    dump_canonical,
    dump_compact,
    parse,
)

REPO = Path(__file__).resolve().parents[2]
ZEBRA = REPO / "examples" / "zebra.ein"
ZEBRA2 = REPO / "examples" / "zebra2.ein"
EXAMPLE_FILES = [ZEBRA, ZEBRA2]


# ═══════════ Lowering: terminals ═══════════

def test_atom():
    (form,) = parse("(lives-in Norwegian House-1)")
    fact = form
    assert isinstance(fact, SForm)
    assert fact.head == Atom("lives-in")
    assert fact.args == (Atom("Norwegian"), Atom("House-1"))


def test_var_and_wildcard():
    (form,) = parse("(rule any () :match (_ ?a ?b) :assert ?a)")
    rule = form
    match = next(kp.value for kp in rule.args if isinstance(kp, KwPair)
                 and kp.key.name == "match")
    assert isinstance(match, SForm)
    assert isinstance(match.head, Wildcard)
    assert match.args == (Var("a"), Var("b"))


def test_keyword_kwpair():
    (form,) = parse("(query :mode solve :goal (lives-in _ House-1))")
    args = form.args
    assert isinstance(args[0], KwPair)
    assert args[0].key == Keyword("mode")
    assert args[0].value == Atom("solve")


def test_string_escapes():
    (form,) = parse(r'(msg :source "line1\nline2 \"quoted\"")')
    fact = form
    src = next(kp.value for kp in fact.args if isinstance(kp, KwPair))
    assert src == String('line1\nline2 "quoted"')


def test_int_and_range():
    (form,) = parse(
        "(relation r A B :cardinality 1..* :priority 5)"
    )
    rel = form
    cardinality = next(kp.value for kp in rel.args if isinstance(kp, KwPair)
                       and kp.key.name == "cardinality")
    priority = next(kp.value for kp in rel.args if isinstance(kp, KwPair)
                    and kp.key.name == "priority")
    assert cardinality == Range(1, None)
    assert priority == Int(5)


# ═══════════ Lowering: top-level forms ═══════════

def test_top_level_heads():
    # P1.7c: a flat sequence — each form classified by its own head.
    forms = parse("""
    (relation lives-in Person House)
    (type Person)
    (lives-in a b)
    (rule x () :match a :assert b)
    (query :mode solve :goal X)
    (trace)
    """)
    assert tuple(f.head.name for f in forms) == (
        "relation", "type", "lives-in", "rule", "query", "trace",
    )


def test_eq_fact():
    (form,) = parse("(= (color House-1) Red)")
    eq = form
    assert eq.head == Atom("=")
    assert isinstance(eq.args[0], SForm) and eq.args[0].head == Atom("color")
    assert eq.args[1] == Atom("Red")


def test_eq_inside_pattern():
    (form,) = parse("(rule eq-elim () :match (= ?a ?b) :assert ?a)")
    rule = form
    match = next(kp.value for kp in rule.args if isinstance(kp, KwPair)
                 and kp.key.name == "match")
    assert match.head == Atom("=")
    assert match.args == (Var("a"), Var("b"))


def test_rule_params():
    (form,) = parse("(rule symmetric (?rel) :match ?rel :assert ?rel)")
    rule = form
    # rule_decl args: [name, params, *kw_pairs]
    assert rule.args[0] == Atom("symmetric")
    params = rule.args[1]
    assert isinstance(params, SForm) and params.head == Atom("@params")
    assert params.args == (Var("rel"),)


def test_empty_rule_params():
    (form,) = parse("(rule x () :match a :assert b)")
    rule = form
    params = rule.args[1]
    assert params.head == Atom("@params") and params.args == ()


def test_macro_decl_shape():
    """`(macro NAME (?p…) BODY)` → SForm("macro", (name, @params, body))."""
    (form,) = parse("(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))))")
    assert form.head == Atom("macro")
    name, params, body = form.args
    assert name == Atom("forall")
    assert isinstance(params, SForm) and params.head == Atom("@params")
    assert params.args == (Var("b"), Var("G"), Var("B"))
    assert isinstance(body, SForm) and body.head == Atom("absent")


def test_import_decl_shape():
    """`(import MODULE :as A)` → SForm("import", (module_atom, KwPair)).
    The dotted module name lowers to a single Atom."""
    (form,) = parse("(import std.macro :as sg)")
    assert form.head == Atom("import")
    module, kw = form.args
    assert module == Atom("std.macro")          # dotted name = one atom
    assert isinstance(kw, KwPair) and kw.key.name == "as"
    assert kw.value == Atom("sg")


def test_dotted_atom_lowers_to_single_atom():
    (form,) = parse("(rel a.b c.d.e)")
    assert form.args == (Atom("a.b"), Atom("c.d.e"))


def test_relation_sig_flat():
    """Post-R10: relation args are flat — no @sig SForm wrapper."""
    (form,) = parse(
        "(relation lives-in Person House :cardinality 1..1)"
    )
    rel = form
    # rel.args == (name, T1, T2, KwPair(:cardinality 1..1))
    assert rel.args[0] == Atom("lives-in")
    assert rel.args[1] == Atom("Person")
    assert rel.args[2] == Atom("House")
    assert isinstance(rel.args[3], KwPair)
    assert rel.args[3].key.name == "cardinality"


def test_relation_wrapped_form_rejected_at_load():
    """`relation` is not SYMBOL-excluded (rules match `(relation ?R ?A ?B)`
    patterns), so the wrapped-arg form `(relation R (T1 T2))` PARSES (as a
    generic fact headed `relation`), but the loader's relation routing
    rejects it as malformed — validation at load time, with a clearer error.
    P1.7c: tested as a flat top-level form (no `(ontology …)` wrapper)."""
    from ein_bot.kb import KnowledgeBase
    from ein_bot.kb.from_ir import KBLoadError
    forms = parse("(relation lives-in (Person House))")
    with pytest.raises(KBLoadError, match=r"malformed .relation."):
        KnowledgeBase.from_ir(forms)


def test_former_wrapper_head_is_a_fact():
    """P1.7c S1.7c.4: the block wrappers are gone — a stray `(facts …)` is
    no longer a recognised wrapper. It loads as an ordinary fact whose
    relation is the (former-wrapper) head; its body is a nested arg, not a
    top-level fact."""
    from ein_bot.kb import KnowledgeBase
    kb = KnowledgeBase.from_ir(parse("(facts (foo a))"))
    rels = {f.relation_name for f in kb.facts}
    assert "facts" in rels        # `facts` is now a relation, not a wrapper
    assert "foo" not in rels      # `(foo a)` is a nested arg, not a top-level fact


# ═══════════ Loc tracking ═══════════

def test_loc_recorded():
    forms = parse("\n(type Person)", filename="x.ein")
    type_decl = forms[0]
    name = type_decl.args[0]
    assert name == Atom("Person")
    assert name.loc is not None
    assert name.loc.file == "x.ein"
    assert name.loc.line == 2  # second line


def test_loc_excluded_from_equality():
    """Two ASTs from differently-positioned sources still compare equal."""
    a = parse("(lives-in a b)")
    b = parse("\n\n  (lives-in a b)")
    assert a == b


# ═══════════ Round-trip ═══════════

ROUNDTRIP_CASES = [
    # ── Flat forms (P1.7c): no block wrappers — a sequence of forms ──
    "(type Person) (type Engineer Person)",
    "(= (color House-1) Red)",
    "(lives-in Norwegian House-1 :source \"condition (10)\")",
    "(symmetric co-located) (implies right-of next-to)",
    "(rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a)"
    " :why \"sym\")",
    "(rule t () :match (and (?r ?a ?b) (?r ?b ?c)"
    " :where (transitive ?r)) :assert (?r ?a ?c) :why \"tr\")",
    "(query :mode solve :goal (drinks Water ?h))",
    "(trace (step s1 :rule from-condition :using (c10)"
    " :derives (lives-in Norwegian House-1)))",
    "(trace (branch-open s3 :on (lives-in Englishman ?h)"
    " :choices (a b c)))",
    # ── Edge cases (S1.1.3 T1.1.3.3, 2026-05-18) ──
    # String escapes
    '(lives-in a b :source "tab\\there")',
    '(lives-in a b :source "newline\\nhere")',
    '(lives-in a b :source "quote\\"inside")',
    '(lives-in a b :source "back\\\\slash")',
    '(lives-in a b :source "")',           # empty string
    '(lives-in a b :source "unicode é·»→")',
    # Range edge cases
    "(relation r A B :cardinality 0..0)",
    "(relation r A B :cardinality 0..*)",
    "(relation r A B :cardinality 9999..*)",
    # Deep nesting
    "(rule deep () :match (and (and (and (and (rel ?a ?b)))))"
    " :assert ?a :why \"d\")",
    # Variable / wildcard heads
    "(rule var-head (?r) :match (?r ?a ?b) :assert ?a :why \"v\")",
    "(rule wild () :match (_ ?a ?b) :assert ?a :why \"w\")",
    "(rule mixed (?r ?s) :match (?r ?a (?s ?b ?c)) :assert ?a"
    " :why \"m\")",
    # Single forms + the trace sibling (empty `(trace)` stays valid)
    "(trace)",
    "(type T)",
    "(rule x () :match a :assert b :why \"x\")",
    # Kw-pair ordering — same form, kw-pairs swapped
    "(rule p () :match a :assert b :priority 1 :why \"p\")",
    "(rule p () :match a :priority 1 :assert b :why \"p\")",
    "(rule p () :why \"p\" :match a :assert b :priority 1)",
    # Equality patterns / facts
    "(= a b)",
    "(= (color House-1) Red)",
    "(rule eq () :match (= ?a ?b) :assert ?a :why \"e\")",
    # Negation
    "(not (lives-in Spaniard Coffee))",
    "(not (not (lives-in a b)))",
    "(rule n () :match (rel ?a ?b) :assert (not (rel ?b ?a))"
    " :why \"n\")",
    "(not (co-located N H) :rule type-exclusivity :using (s1)"
    " :layer reasoning)",
    # Mixed kernel primitives
    "(rule mix () :match (and (or (rel ?a ?b) (rel ?b ?a))"
    " (not (rel ?a ?a)) :where (neq ?a ?b)) :assert ?a :why \"mx\")",
    # Derived-fact provenance (re-classifies to REASONING via :rule)
    "(co-located Blue House-2 :rule square-fwd :using (c10 c15))",
    # Numbers (positive, negative, zero)
    "(relation r A B :priority 0)",
    "(relation r A B :priority -7)",
    "(relation r A B :priority 9999)",
    # Instance as fact + as pattern
    "(instance Norwegian Nationality)",
    "(rule i () :match (instance ?a ?T) :assert ?a :why \"i\")",
    # Macro declarators (P1.8 S1.5.9)
    "(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))))",
    "(macro open (?P) (and (absent ?P) (absent (not ?P))))",
    # Import declarators + dotted atoms (P1.8 S1.8.A2)
    "(import std.macro)",
    "(import std.macro :as m)",
    "(import std.macro :symbols (forall open))",
    "(rel a.b c.d.e)",
]


@pytest.mark.parametrize("src", ROUNDTRIP_CASES)
def test_roundtrip(src: str) -> None:
    ast1 = parse(src)
    text = dump_canonical(ast1)
    ast2 = parse(text)
    assert ast1 == ast2, f"roundtrip diverged:\n--src--\n{src}\n--dump--\n{text}"


@pytest.mark.parametrize("path", EXAMPLE_FILES, ids=lambda p: p.name)
def test_roundtrip_example(path):
    """Each bundled example survives dump∘parse without loss."""
    src = path.read_text(encoding="utf-8")
    ast1 = parse(src, filename=str(path))
    text = dump_canonical(ast1)
    ast2 = parse(text)
    assert ast1 == ast2


@pytest.mark.parametrize("path", EXAMPLE_FILES, ids=lambda p: p.name)
def test_dump_stable(path):
    """dump∘parse∘dump∘parse == dump∘parse — printer is a fixed point."""
    src = path.read_text(encoding="utf-8")
    t1 = dump_canonical(parse(src))
    t2 = dump_canonical(parse(t1))
    assert t1 == t2


@pytest.mark.parametrize("path", EXAMPLE_FILES, ids=lambda p: p.name)
def test_golden_example(path):
    """Snapshot: dump_canonical(parse(<example>)) matches tests/golden/<example>.golden.

    Catches IR drift across phases (a change in grammar, AST shape,
    or dumper output trips this; refresh the golden if the change is
    intentional with:
        python3 -c "from pathlib import Path; from ein_bot.ir import \\
            parse, dump_canonical; \\
            stem = 'zebra'  # or 'zebra2' \\
            Path(f'tests/golden/{stem}.golden').write_text( \\
                dump_canonical(parse(Path(f'examples/{stem}.ein').read_text())))"
    """
    src = path.read_text(encoding="utf-8")
    golden = REPO / "ein.py" / "tests" / "golden" / f"{path.stem}.golden"
    got = dump_canonical(parse(src))
    expected = golden.read_text(encoding="utf-8")
    if got != expected:
        import difflib
        diff = "".join(difflib.unified_diff(
            expected.splitlines(keepends=True),
            got.splitlines(keepends=True),
            fromfile=golden.name,
            tofile=f"dump_canonical(parse({path.name}))",
        ))
        raise AssertionError(f"golden snapshot drift:\n{diff}")


def test_dump_compact_one_line():
    (form,) = parse("(lives-in Norwegian House-1 :source \"x\")")
    compact = dump_compact(form)
    assert "\n" not in compact
    assert compact.startswith("(lives-in ") and compact.endswith(")")


# ═══════════ Error messages ═══════════

@pytest.mark.parametrize("src,marker", [
    ("(rule x () :match a", "1:"),              # unclosed paren
    ("Norwegian", "1:"),                         # top-level bare atom
    ("(query :mode :solve)", "1:"),              # keyword as value
    ("(neq Norwegian)", "1:"),                   # reserved-word arity (neq needs 2)
    ("(and a b)", "1:"),                          # `and` is not a top-level fact (P1.7c)
])
def test_error_has_location(src: str, marker: str) -> None:
    with pytest.raises(IRParseError) as ei:
        parse(src, filename="<demo>")
    msg = str(ei.value)
    assert "<demo>:" in msg
    assert marker in msg
