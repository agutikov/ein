"""Converse domain-check — std.algebra (b) + std.typing one-knob/reflexive (P1.8 S1.8.A10).

The `converse-illtyped-{dom,ran}` rules read relation signatures straight off the
`(relation …)` declaration facts and reject (⊥, a `(false)` fact) reverse-
incompatible pairings, parameterised over the puzzle's subtype relation ?isR*
(no `is-a` literal in the rule bodies). std.typing's `(type-hierarchy …)` knob
derives the per-pair activators (variant c); `(reflexive R)` closes a hierarchy
reflexively on both argument positions.

Proves the design point from S1.8.A10's reframing: domain-checking generic rules
is pure-stdlib-doable on the existing matcher — the signature *is* a matchable
fact, and an activator-bound relation head compiles to a concrete scan.
"""
from __future__ import annotations

from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

_ALGEBRA = ("(import std.algebra :symbols "
            "(converse-illtyped-dom converse-illtyped-ran))\n")
_TYPING = "(import std.typing :symbols (type-hierarchy-converse))\n"
_REFLEX = ("(import std.typing :symbols "
           "(derive-reflexive reflexive-dom reflexive-cod))\n")


def _saturate(src: str, max_steps: int = 3000):
    kb = KnowledgeBase.from_ir(parse(src))
    return [f for f in Saturator(kb).saturate(max_steps=max_steps)
            if not f.redundant]


def _false(firings):
    return [f for f in firings if f.derived.relation_name == "false"]


def _derived(firings):
    return {(f.derived.relation_name, f.derived.args) for f in firings}


# ── variant (b): raw per-pair activators ───────────────────────────


def test_incompatible_sigs_rejected():
    """(converse house-color pet-dog): (House,Color) vs (Person,Pet) → ⊥."""
    f = _saturate(_ALGEBRA + """
    (relation house-color House Color)
    (relation pet-dog Person Pet)
    (converse house-color pet-dog)
    (converse-illtyped-dom house-color pet-dog is-a*)
    (converse-illtyped-ran house-color pet-dog is-a*)
    """)
    assert _false(f)


def test_exact_reverse_sigs_ok():
    """(converse right-of left-of): both (House,House) → no ⊥.

    The `(neq …)` guard absorbs the identical-type case, so no is-a* fact and
    no reflexivity is needed."""
    f = _saturate(_ALGEBRA + """
    (relation right-of House House)
    (relation left-of House House)
    (converse right-of left-of)
    (converse-illtyped-dom right-of left-of is-a*)
    (converse-illtyped-ran right-of left-of is-a*)
    """)
    assert not _false(f)


def test_subtype_converse_ok_with_hierarchy():
    """owns:(Person,Pet) converse owned-by:(Animal,Person), Pet <: Animal → OK.

    range(owns)=Pet must be <: domain(owned-by)=Animal — and (is-a* Pet Animal)
    is declared, so the absent fails and the rule stays silent."""
    f = _saturate(_ALGEBRA + """
    (relation owns Person Pet)
    (relation owned-by Animal Person)
    (converse owns owned-by)
    (is-a* Pet Animal :source "(sub)")
    (converse-illtyped-dom owns owned-by is-a*)
    (converse-illtyped-ran owns owned-by is-a*)
    """)
    assert not _false(f)


def test_subtype_converse_rejected_without_hierarchy():
    """Same pair, but Pet <: Animal NOT declared → Pet neither = nor <: Animal → ⊥.

    Dual of the previous test: it is precisely the ?isR* lookup that flips the
    verdict, proving the hierarchy relation is genuinely consulted."""
    f = _saturate(_ALGEBRA + """
    (relation owns Person Pet)
    (relation owned-by Animal Person)
    (converse owns owned-by)
    (converse-illtyped-dom owns owned-by is-a*)
    (converse-illtyped-ran owns owned-by is-a*)
    """)
    assert _false(f)


# ── variant (c): the (type-hierarchy …) one-knob driver ────────────


def test_type_hierarchy_knob_drives_check():
    """One (type-hierarchy is-a*) → derives the activators (A9 reflective) → ⊥ on bad."""
    f = _saturate(_ALGEBRA + _TYPING + """
    (relation house-color House Color)
    (relation pet-dog Person Pet)
    (type-hierarchy is-a*)
    (converse house-color pet-dog)
    """)
    assert _false(f)
    assert ("converse-illtyped-dom",
            ("house-color", "pet-dog", "is-a*")) in _derived(f)


def test_type_hierarchy_knob_silent_on_good():
    f = _saturate(_ALGEBRA + _TYPING + """
    (relation right-of House House)
    (relation left-of House House)
    (type-hierarchy is-a*)
    (converse right-of left-of)
    """)
    assert not _false(f)


# ── reflexive closure ──────────────────────────────────────────────


def test_reflexive_closes_both_positions():
    """(reflexive is-a*) self-loops every node is-a* touches, in either position."""
    f = _saturate(_REFLEX + """
    (relation is-a* T T)
    (reflexive is-a*)
    (is-a* House-1 House :source "(e)")
    """)
    d = _derived(f)
    assert ("is-a*", ("House", "House")) in d          # codomain self-loop
    assert ("is-a*", ("House-1", "House-1")) in d       # domain self-loop
