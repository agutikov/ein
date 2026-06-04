"""Import resolution + the reserved-name guard (P1.8 S1.8.A2/A3).

S1.8.A3 resolves `(import …)` at the form level (flatten-then-load): each
import is replaced by its module's qualified forms before `load()` ingests the
union. Covered here: the three tiers against the packaged `std.macro`; logical
`std.*` resolution; file-relative two-file projects; transitive re-export;
cycle / not-found / `:as`-XOR-`:symbols` diagnostics; and the D3 reserved-name
guard (shared across declarators; facts may still carry reserved heads).
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.inference.compile import AbsentGuard, compile_rule
from ein_bot.ir import parse
from ein_bot.kb.from_ir import KBLoadError
from ein_bot.kb.store import KnowledgeBase


def _load(src: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(src))


def _write(tmp_path, name: str, text: str):
    p = tmp_path / name
    p.write_text(text, encoding="utf-8")
    return p


# ── std.macro: the three tiers ─────────────────────────────────────


def test_import_symbols_is_flat():
    """`:symbols (forall open)` → bare `forall` / `open`, and a `forall`
    invocation compiles to the same nested AbsentGuard as the inline macro."""
    kb = _load("""
    (import std.macro :symbols (forall open))
    (rule undefeated ()
      :match (and (player ?p)
                  (forall ?q (and (player ?q) (neq ?p ?q)) (beats ?p ?q)))
      :assert (undefeated ?p) :why "u")
    (relation player T T) (relation beats T T)
    """)
    assert set(kb.macros) == {"forall", "open"}
    plan = compile_rule(kb.rules["undefeated"], None)
    outer = next(s for s in plan.steps if isinstance(s, AbsentGuard))
    assert any(isinstance(s, AbsentGuard) for s in outer.sub_steps)


def test_import_whole_module_is_qualified():
    kb = _load("(import std.macro)")
    assert set(kb.macros) == {"std.macro.forall", "std.macro.open"}


def test_import_alias():
    kb = _load("(import std.macro :as m)")
    assert set(kb.macros) == {"m.forall", "m.open"}


def test_qualified_macro_is_invokable():
    """A whole-module import's `std.macro.forall` works as a macro head."""
    kb = _load("""
    (import std.macro)
    (rule r ()
      :match (and (player ?p)
                  (std.macro.forall ?q (player ?q) (beats ?p ?q)))
      :assert (ok ?p) :why "w")
    (relation player T T) (relation beats T T)
    """)
    plan = compile_rule(kb.rules["r"], None)
    outer = next(s for s in plan.steps if isinstance(s, AbsentGuard))
    assert any(isinstance(s, AbsentGuard) for s in outer.sub_steps)


# ── Diagnostics ────────────────────────────────────────────────────


def test_as_and_symbols_mutually_exclusive():
    with pytest.raises(KBLoadError, match=r":as and :symbols are mutually exclusive"):
        _load("(import std.macro :as m :symbols (forall))")


def test_module_not_found():
    with pytest.raises(KBLoadError, match=r"module not found"):
        _load("(import std.nope)")


def test_bare_std_is_not_a_module():
    with pytest.raises(KBLoadError, match=r"bare 'std' is not a module"):
        _load("(import std)")


def test_symbols_not_provided():
    with pytest.raises(KBLoadError, match=r"not provided by the module: nope"):
        _load("(import std.macro :symbols (nope))")


def test_file_relative_without_base_errors():
    with pytest.raises(KBLoadError, match=r"file-relative import needs a base"):
        _load("(import mymod.x)")


# ── File-relative projects ─────────────────────────────────────────


def test_file_relative_qualified(tmp_path):
    _write(tmp_path, "lib.ein",
           '(relation knows T T)\n'
           '(rule sym (?r) :match (?r ?a ?b) :assert (?r ?b ?a) :why "s")\n')
    main = _write(tmp_path, "main.ein", "(import lib)\n(relation x T T)\n")
    kb = KnowledgeBase.from_file(main)
    assert "lib.knows" in kb.relations
    assert "lib.sym" in kb.rules


def test_file_relative_symbols_flat(tmp_path):
    _write(tmp_path, "lib.ein",
           "(macro twice (?x) (and ?x ?x))\n(macro unused (?y) (not ?y))\n")
    main = _write(tmp_path, "main.ein", "(import lib :symbols (twice))\n")
    kb = KnowledgeBase.from_file(main)
    assert "twice" in kb.macros and "unused" not in kb.macros


def test_transitive_reexport_requalified(tmp_path):
    """`mid` imports std.macro flat and wraps it; importing `mid` whole-module
    re-qualifies BOTH `mid.wrap` and the re-exported `mid.forall`, and the
    composed macro still expands transitively."""
    _write(tmp_path, "mid.ein",
           "(import std.macro :symbols (forall))\n"
           "(macro wrap (?g ?b) (forall ?x ?g ?b))\n")
    main = _write(tmp_path, "main.ein",
                  "(import mid)\n"
                  '(rule r () :match (and (player ?p)'
                  '   (mid.wrap (player ?p) (beats ?p ?p)))'
                  '   :assert (ok ?p) :why "w")\n'
                  "(relation player T T) (relation beats T T)\n")
    kb = KnowledgeBase.from_file(main)
    assert {"mid.forall", "mid.wrap"} <= set(kb.macros)
    plan = compile_rule(kb.rules["r"], None)
    assert any(isinstance(s, AbsentGuard) for s in plan.steps)


def test_import_cycle_rejected(tmp_path):
    _write(tmp_path, "a.ein", "(import b)\n(relation x T T)\n")
    _write(tmp_path, "b.ein", "(import a)\n(relation y T T)\n")
    with pytest.raises(KBLoadError, match=r"import cycle"):
        KnowledgeBase.from_file(tmp_path / "a.ein")


def test_demo_mirror_matches_canonical():
    """`examples/stdlib/macro.ein` must stay content-identical to the
    package-canonical `ein_bot/stdlib/macro.ein` (only header comments differ,
    which parsing strips) — else the resolver and the inlined copies drift."""
    import ein_bot
    pkg = Path(ein_bot.__file__).resolve().parent / "stdlib" / "macro.ein"
    repo = Path(__file__).resolve().parents[3]
    demo = repo / "examples" / "stdlib" / "macro.ein"
    assert parse(pkg.read_text()) == parse(demo.read_text())


def test_existing_import_free_kb_unaffected():
    """zebra-style inline puzzle (no imports) loads exactly as before —
    resolution is a pass-through."""
    kb = _load("(relation x T T) (x A B :source \"(1)\")")
    assert "x" in kb.relations and len(kb.facts) >= 1


# ── Reserved-name guard (D3), shared across declarators ────────────


@pytest.mark.parametrize("name", ["absent", "false", "eq", "relation"])
def test_reserved_rule_name_rejected(name):
    with pytest.raises(KBLoadError, match=r"shadows a reserved kernel name"):
        _load(f'(rule {name} () :match (x ?a) :assert (y ?a) :why "w")')


@pytest.mark.parametrize("name", ["absent", "false", "eq", "relation"])
def test_reserved_relation_name_rejected(name):
    with pytest.raises(KBLoadError, match=r"shadows a reserved kernel name"):
        _load(f"(relation {name} T T)")


def test_reserved_hrule_name_rejected():
    with pytest.raises(KBLoadError, match=r"hrule 'absent' shadows"):
        _load('(hrule absent () :match (x ?a) :assert (y ?a) :why "w")')


def test_non_reserved_names_still_load():
    kb = _load("""
    (rule eq-elim () :match (x ?a) :assert (y ?a) :why "ok")
    (relation absent-of T T)
    (relation x T T) (relation y T T)
    """)
    assert "eq-elim" in kb.rules and "absent-of" in kb.relations


def test_fact_may_have_a_reserved_head():
    kb = _load('(not (likes A B) :source "(1)") (relation likes T T)')
    assert any(f.relation_name == "not" for f in kb.facts)
