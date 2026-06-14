"""`std.bijection` — signature-driven, is-a-free bijection inference (S1.8a.f20).

The module's rules generalise zebra2's inline bijection machinery: types are read
from the `(relation R A B)` signature, the type-membership relation arrives as the
`?isa` param, and a `(bijective R)` + two hierarchy knobs drive the whole pipeline
via the `*-setup` glue. These fast unit tests exercise each shipped symbol on a
tiny color-of/House puzzle (the full integration is the slow zebra2 acceptance).
"""
from __future__ import annotations

from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _derived(text: str, max_steps: int = 5000):
    fs = [f for f in Saturator(_kb(text)).saturate(max_steps=max_steps)
          if not f.redundant]
    return {(d.relation_name, d.args) for f in fs for d in f.derived}


def _fired(text: str, rule: str, max_steps: int = 5000) -> int:
    return sum(1 for f in Saturator(_kb(text)).saturate(max_steps=max_steps)
               if not f.redundant and f.rule == rule)


# Entry imports: the puzzle lists only the entry rules; auto-closure (S1.8a.f20)
# pulls the machinery they assert/match.
_SETUP = ("(import std.macro :symbols (forall))\n"
          "(import std.algebra :symbols (bijective-properties))\n"
          "(import std.bijection :symbols (bijective-setup typecheck-setup))\n")


# ── auto-closure ───────────────────────────────────────────────────


def test_auto_closure_pulls_machinery():
    """Listing only `bijective-setup`/`typecheck-setup` (+ `bijective-properties`)
    resolves the whole bijection family by `:symbols` auto-closure."""
    kb = _kb(_SETUP + "(relation color-of House Color)\n")
    for name in ("bijective-setup", "typecheck-setup", "bijective-properties",
                 "functional", "injective", "total", "surjective",
                 "domain-elimination", "range-elimination",
                 "functional-negative", "injective-negative",
                 "typecheck-arg-0", "typecheck-arg-1"):
        assert name in kb.rules, f"auto-closure did not pull {name}"


# ── bijective-setup fan-out ────────────────────────────────────────


def test_bijective_setup_fans_out_operational_activators():
    d = _derived(_SETUP + """
    (relation color-of House Color)
    (bijection-hierarchy is-a)
    (typecheck-hierarchy is-a)
    (bijective color-of)
    (is-a H1 House) (is-a Red Color)
    """)
    # the 1-arg markers (std.algebra bijective-properties) + 2-arg operational
    # activators carrying the hierarchy (std.bijection bijective-setup).
    assert ("functional", ("color-of",)) in d
    assert ("total", ("color-of", "is-a")) in d
    assert ("domain-elimination", ("color-of", "is-a")) in d
    assert ("typecheck-arg-0", ("color-of", "is-a", "House")) in d


# ── elimination ────────────────────────────────────────────────────

_ELIM = ("(import std.macro :symbols (forall))\n"
         "(import std.bijection :symbols (domain-elimination range-elimination))\n")


def test_domain_elimination_forces_survivor():
    """functional+total, every Color but Green excluded for H1 → force Green."""
    d = _derived(_ELIM + """
    (relation color-of House Color)
    (functional color-of) (total color-of)
    (domain-elimination color-of is-a)
    (is-a H1 House)
    (is-a Red Color) (is-a Blue Color) (is-a Green Color)
    (not (color-of H1 Red) :source "a") (not (color-of H1 Blue) :source "b")
    """)
    assert ("color-of", ("H1", "Green")) in d


def test_range_elimination_forces_survivor():
    """injective+surjective, every House but H3 excluded for Red → force H3."""
    d = _derived(_ELIM + """
    (relation color-of House Color)
    (injective color-of) (surjective color-of)
    (range-elimination color-of is-a)
    (is-a Red Color)
    (is-a H1 House) (is-a H2 House) (is-a H3 House)
    (not (color-of H1 Red) :source "a") (not (color-of H2 Red) :source "b")
    """)
    assert ("color-of", ("H3", "Red")) in d


def test_domain_elimination_silent_with_two_survivors():
    d = _derived(_ELIM + """
    (relation color-of House Color)
    (functional color-of) (total color-of)
    (domain-elimination color-of is-a)
    (is-a H1 House)
    (is-a Red Color) (is-a Blue Color) (is-a Green Color)
    (not (color-of H1 Red) :source "a")
    """)
    assert not any(rel == "color-of" for rel, _ in d)


# ── negative completion ────────────────────────────────────────────

_NEG = ("(import std.bijection "
        ":symbols (functional-negative injective-negative))\n")


def _negatives(text: str) -> set:
    """The inner (relation, args) of every derived `(not (…))` fact."""
    return {(a.relation_name, a.args)
            for rel, args in _derived(text) if rel == "not" for a in args}


def test_functional_negative_propagates():
    """(color-of H1 Red) on a functional R ⟹ H1 has no other colour."""
    negs = _negatives(_NEG + """
    (relation color-of House Color)
    (functional-negative color-of is-a)
    (is-a Red Color) (is-a Blue Color) (is-a Green Color)
    (color-of H1 Red :source "given")
    """)
    assert ("color-of", ("H1", "Blue")) in negs
    assert ("color-of", ("H1", "Green")) in negs


def test_injective_negative_propagates():
    """(color-of H1 Red) on an injective R ⟹ no other house is Red."""
    negs = _negatives(_NEG + """
    (relation color-of House Color)
    (injective-negative color-of is-a)
    (is-a H1 House) (is-a H2 House) (is-a H3 House)
    (color-of H1 Red :source "given")
    """)
    assert ("color-of", ("H2", "Red")) in negs
    assert ("color-of", ("H3", "Red")) in negs


# ── typecheck ──────────────────────────────────────────────────────

_TC = "(import std.bijection :symbols (typecheck-arg-0 typecheck-arg-1))\n"


def test_typecheck_arg0_fires_on_mistyped():
    n = _fired(_TC + """
    (relation color-of House Color)
    (typecheck-arg-0 color-of is-a House)
    (is-a H1 House) (is-a Red Color) (is-a Bob Person) (is-a Person T)
    (color-of Bob Red :source "bad")
    """, "typecheck-arg-0")
    assert n == 1


def test_typecheck_silent_when_well_typed():
    n = _fired(_TC + """
    (relation color-of House Color)
    (typecheck-arg-0 color-of is-a House)
    (typecheck-arg-1 color-of is-a Color)
    (is-a H1 House) (is-a Red Color)
    (color-of H1 Red :source "ok")
    """, "typecheck-arg-0") + _fired(_TC + """
    (relation color-of House Color)
    (typecheck-arg-1 color-of is-a Color)
    (is-a H1 House) (is-a Red Color)
    (color-of H1 Red :source "ok")
    """, "typecheck-arg-1")
    assert n == 0


# ── totality checks (std.algebra, is-a-free) ───────────────────────


def test_total_fires_when_every_value_excluded():
    """total CHECK: an object with every partner explicitly excluded is ⊥."""
    n = _fired("(import std.macro :symbols (forall))\n"
               "(import std.algebra :symbols (total))\n" + """
    (relation color-of House Color)
    (total color-of is-a)
    (is-a H1 House)
    (is-a Red Color) (is-a Blue Color)
    (not (color-of H1 Red) :source "a") (not (color-of H1 Blue) :source "b")
    """, "total")
    assert n == 1
