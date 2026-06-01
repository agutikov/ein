"""Incremental fact indexing — P1.7b S1.7b.4 (F-KB-3 regression).

The reasoning hot path used to be ``stored = kb.add_fact(f);
kb._index_fact(stored)``. When ``add_fact`` deduplicated (a fact
re-derived by a second rule), it returned the *pre-existing* instance —
but the caller still called ``_index_fact`` unconditionally, appending
that fact to ``_facts_by_relation`` / ``names`` a **second** time. The
external answer was masked by the saturator's firing-key dedup, but the
indexes silently accumulated duplicates (and over-counted).

``add_and_index_fact`` makes add-and-index one operation that indexes a
fact exactly once.
"""
from __future__ import annotations

from ein_bot.kb import Fact, KnowledgeBase, Layer


def _fact(rel: str, *args: str) -> Fact:
    return Fact(relation_name=rel, args=tuple(args), layer=Layer.REASONING)


def test_rederived_fact_indexed_exactly_once():
    kb = KnowledgeBase()
    s1 = kb.add_and_index_fact(_fact("q", "a", "b"))
    # A second, identical derivation (distinct object, same identity).
    s2 = kb.add_and_index_fact(_fact("q", "a", "b"))

    assert s2 is s1                                  # dedup returns the first
    assert kb._facts_by_relation["q"] == (s1,)       # relation index: once
    assert kb.facts.count(s1) == 1                   # fact list: once
    assert kb.names["q"].as_head.count(s1) == 1      # names index: once
    assert kb.names["a"].as_arg.count(s1) == 1


def test_distinct_facts_each_indexed():
    kb = KnowledgeBase()
    a = kb.add_and_index_fact(_fact("q", "a", "b"))
    b = kb.add_and_index_fact(_fact("q", "c", "d"))

    assert kb._facts_by_relation["q"] == (a, b)
    assert kb.facts == [a, b]


def test_two_rules_deriving_same_fact_index_once():
    """End-to-end: a conclusion produced by two rules from two premises
    is indexed once after saturation (the real double-index scenario)."""
    from ein_bot.inference.saturator import Saturator
    from ein_bot.ir import parse

    kb = KnowledgeBase.from_ir(parse(
        """
        (ontology (relation r T T) (relation p T T) (relation q T T))
        (facts (r a b) (p a b))
        (rules
          (rule from-r () :match (r ?x ?y) :assert (q ?x ?y) :why "" :priority 100)
          (rule from-p () :match (p ?x ?y) :assert (q ?x ?y) :why "" :priority 100))
        """
    ))
    list(Saturator(kb).saturate())

    q_facts = [f for f in kb._facts_by_relation.get("q", ()) if f.args == ("a", "b")]
    assert len(q_facts) == 1, "(q a b) was derived by two rules but must index once"
