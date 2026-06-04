"""Multi-fact assertion — `:assert (and c1 … cn)` (P1.8 S1.8.A11).

A rule that should conclude several facts from one match is lowered at load
time to one rule per conjunct (`<rule>__and<j>`), the dual of the
`(or …)`-in-`:match` → `__or<i>` split. The firing model stays
one-fact-per-firing; each conjunct fires as an ordinary single-assert rule
sharing the parent `:match`. Generic rules (params) reject a multi-fact assert
(the split names break activator resolution).
"""
from __future__ import annotations

import pytest

from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.from_ir import KBLoadError
from ein_bot.kb.store import KnowledgeBase


def _kb(src: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(src))


def _derived(kb: KnowledgeBase):
    """{(rule, relation, args)} for each productive firing."""
    return {
        (f.rule, f.derived.relation_name, f.derived.args)
        for f in Saturator(kb).saturate(max_steps=2000)
        if not f.redundant
    }


def test_assert_and_splits_into_andN_rules():
    kb = _kb("""
    (rule decide-and-exclude ()
      :match  (and (at ?a ?x) (slot ?y) (neq ?x ?y))
      :assert (and (decided ?a ?x) (not (at ?a ?y)))
      :why "{?a} is {?x}, so not {?y}")
    (relation at T T) (relation decided T T) (relation slot T)
    (at Norwegian House-1 :source "(1)")
    (slot House-2 :source "(2)")
    """)
    assert set(kb.rules) == {"decide-and-exclude__and0", "decide-and-exclude__and1"}
    derived = _derived(kb)
    # Both conclusions fire from the one match.
    assert ("decide-and-exclude__and0", "decided", ("Norwegian", "House-1")) in derived
    neg = [d for d in derived if d[1] == "not"]
    assert len(neg) == 1
    # the negated inner fact is (at Norwegian House-2)
    inner = neg[0][2][0]
    assert (inner.relation_name, inner.args) == ("at", ("Norwegian", "House-2"))


def test_three_conjuncts():
    kb = _kb("""
    (rule fan () :match (seed ?a)
      :assert (and (p ?a) (q ?a) (r ?a)) :why "w")
    (relation seed T) (relation p T) (relation q T) (relation r T)
    (seed X :source "(1)")
    """)
    assert set(kb.rules) == {"fan__and0", "fan__and1", "fan__and2"}
    rels = {d[1] for d in _derived(kb)}
    assert {"p", "q", "r"} <= rels


def test_single_assert_name_unchanged():
    """A normal single-fact assert keeps the bare rule name (no `__and`)."""
    kb = _kb("""
    (rule plain () :match (seed ?a) :assert (p ?a) :why "w")
    (relation seed T) (relation p T)
    (seed X :source "(1)")
    """)
    assert set(kb.rules) == {"plain"}


def test_or_match_and_assert_cross_product():
    """`(or …)`-in-match x `(and …)`-in-assert → the `__or<i>__and<j>` grid."""
    kb = _kb("""
    (rule x () :match (or (a ?n) (b ?n))
              :assert (and (p ?n) (q ?n)) :why "w")
    (relation a T) (relation b T) (relation p T) (relation q T)
    """)
    assert set(kb.rules) == {"x__or0__and0", "x__or0__and1",
                             "x__or1__and0", "x__or1__and1"}


def test_generic_multi_assert_rejected():
    with pytest.raises(KBLoadError, match=r"multi-fact :assert .* unsupported on a generic rule"):
        _kb("""
        (rule place-and-exclude (?rel)
          :match  (and (?rel ?a ?x) (slot ?y) (neq ?x ?y))
          :assert (and (?rel ?a ?x) (not (?rel ?a ?y)))
          :why "w")
        (relation slot T)
        """)


def test_generic_single_assert_still_ok():
    """A generic rule with a *single* assert is unaffected by the guard."""
    kb = _kb("""
    (rule sym (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s")
    (relation knows T T)
    (sym knows)
    (knows A B :source "(1)")
    """)
    assert "sym" in kb.rules
    assert ("sym", "knows", ("B", "A")) in _derived(kb)
