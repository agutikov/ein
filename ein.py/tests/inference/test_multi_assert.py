"""Firing-level multi-fact assert + multi-plan match (P1.8 S1.8.A13).

`:assert (and c1 … ck)` and `:match (or d1 … dm)` no longer SPLIT the rule into
`__and<j>` / `__or<i>` clones. The rule keeps its single name; `compile_rule`
lowers the conjunction to several assert templates and the disjunction to
several match plans, and one match `fire()`s ONCE emitting all k facts — one
`Firing` whose `derived` is a k-tuple sharing one provenance. Because the rule
keeps its name, a *generic* rule may now multi-assert (the case S1.8.A11 had to
reject because the split names broke activator resolution).
"""
from __future__ import annotations

from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _kb(src: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(src))


def _firings(kb: KnowledgeBase):
    return [f for f in Saturator(kb).saturate(max_steps=2000) if not f.redundant]


def _facts(kb: KnowledgeBase):
    """{(rule, relation, args)} flattened over each firing's derived tuple."""
    return {(f.rule, d.relation_name, d.args)
            for f in _firings(kb) for d in f.derived}


def test_assert_and_emits_all_facts_in_one_firing():
    kb = _kb("""
    (rule decide-and-exclude ()
      :match  (and (at ?a ?x) (slot ?y) (neq ?x ?y))
      :assert (and (decided ?a ?x) (not (at ?a ?y)))
      :why "{?a} is {?x}, so not {?y}")
    (relation at T T) (relation decided T T) (relation slot T)
    (at Norwegian House-1 :source "(1)")
    (slot House-2 :source "(2)")
    """)
    # ONE rule — no `__and` clones.
    assert set(kb.rules) == {"decide-and-exclude"}
    # ONE firing concludes BOTH facts.
    [f] = [f for f in _firings(kb) if f.rule == "decide-and-exclude"]
    assert len(f.derived) == 2
    assert {d.relation_name for d in f.derived} == {"decided", "not"}
    # the negated inner fact is (at Norwegian House-2)
    neg = next(d for d in f.derived if d.relation_name == "not")
    inner = neg.args[0]
    assert (inner.relation_name, inner.args) == ("at", ("Norwegian", "House-2"))
    # all conclusions share one provenance (one application)
    assert f.derived[0].provenance is f.derived[1].provenance


def test_three_conjuncts_one_firing():
    kb = _kb("""
    (rule fan () :match (seed ?a)
      :assert (and (p ?a) (q ?a) (r ?a)) :why "w")
    (relation seed T) (relation p T) (relation q T) (relation r T)
    (seed X :source "(1)")
    """)
    assert set(kb.rules) == {"fan"}
    [f] = [f for f in _firings(kb) if f.rule == "fan"]
    assert {d.relation_name for d in f.derived} == {"p", "q", "r"}


def test_single_assert_name_unchanged():
    """A normal single-fact assert: one rule, one derived fact (1-tuple)."""
    kb = _kb("""
    (rule plain () :match (seed ?a) :assert (p ?a) :why "w")
    (relation seed T) (relation p T)
    (seed X :source "(1)")
    """)
    assert set(kb.rules) == {"plain"}
    [f] = [f for f in _firings(kb) if f.rule == "plain"]
    assert len(f.derived) == 1


def test_or_match_and_assert_one_rule():
    """`(or …)`-in-match × `(and …)`-in-assert is now ONE rule: each disjunct
    match fires once and emits both conclusions (no `__or<i>__and<j>` grid)."""
    kb = _kb("""
    (rule x () :match (or (a ?n) (b ?n))
              :assert (and (p ?n) (q ?n)) :why "w")
    (relation a T) (relation b T) (relation p T) (relation q T)
    (a A :source "(1)") (b B :source "(2)")
    """)
    assert set(kb.rules) == {"x"}
    assert {("x", "p", ("A",)), ("x", "q", ("A",)),
            ("x", "p", ("B",)), ("x", "q", ("B",))} <= _facts(kb)


def test_generic_multi_assert_now_works():
    """The S1.8.A13 headline: a GENERIC rule that scans its own activator-bound
    relation `(?rel …)` and concludes SEVERAL facts now loads, activates by its
    bare name, and fires all conclusions — the case A11 had to reject. This is
    the relation-polymorphic 'place a value, exclude the alternative' pattern."""
    kb = _kb("""
    (rule place-and-exclude (?rel)
      :match  (and (?rel ?a ?x) (slot ?y) (neq ?x ?y))
      :assert (and (?rel ?a ?x) (not (?rel ?a ?y)))
      :why "{?a} is {?x} via {?rel}, so not {?y}")
    (relation color-loc Color House) (relation slot T)
    (place-and-exclude color-loc)
    (color-loc Red House-1 :source "(1)")
    (slot House-2 :source "(2)")
    """)
    assert "place-and-exclude" in kb.rules          # one rule, name intact
    # excludes the alternative slot for the placed value
    negatives = [d for f in _firings(kb) for d in f.derived
                 if d.relation_name == "not"]
    assert any(n.args[0].relation_name == "color-loc"
               and n.args[0].args == ("Red", "House-2") for n in negatives)


def test_generic_single_assert_still_ok():
    kb = _kb("""
    (rule sym (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s")
    (relation knows T T)
    (sym knows)
    (knows A B :source "(1)")
    """)
    assert "sym" in kb.rules
    assert ("sym", "knows", ("B", "A")) in _facts(kb)
