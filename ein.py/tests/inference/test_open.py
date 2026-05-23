"""Tests for the `(open P)` parser sugar — S1.5.8c T1.5.8c.3b.

`open` names the third state of a potential fact: neither
asserted (stored positive) nor negated (stored `(not P)`).
Desugars at compile time to `(and (absent P) (absent (not P)))`.
"""
from __future__ import annotations

from ein_bot.inference import match
from ein_bot.inference.compile import AbsentGuard, compile_rule
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _run(kb: KnowledgeBase, rule_name: str) -> list:
    rule = kb.rules[rule_name]
    activator = None if not rule.params else kb._facts_by_relation[rule_name][0]
    plan = compile_rule(rule, activator)
    return list(match.run(plan, kb))


def _setup_three_states():
    """Three pairs covering the three storage states of (likes …):
    (Alice Bob)   — asserted
    (Alice Carol) — negated (stored (not …))
    (Bob   Carol) — open    (neither stored)
    """
    return _kb("""
    (rules
      (rule find-open ()
        :match (and (is-a ?a Person) (is-a ?b Person) (neq ?a ?b)
                    (open (likes ?a ?b)))
        :assert (open-likes ?a ?b)
        :why "{?a}→{?b} undecided"))
    (ontology
      (relation likes T T) (relation is-a T T)
      (is-a Person T)
      (is-a Alice Person) (is-a Bob Person) (is-a Carol Person))
    (facts
      (likes Alice Bob)
      (not (likes Alice Carol)))
    """)


def test_open_desugars_to_two_absent_guards():
    """The compile output must contain two AbsentGuards
    (one for absent P, one for absent (not P))."""
    kb = _setup_three_states()
    rule = kb.rules["find-open"]
    plan = compile_rule(rule, None)
    absents = [s for s in plan.steps if isinstance(s, AbsentGuard)]
    assert len(absents) == 2


def test_open_fires_on_undecided_pair():
    """The (Bob, Carol) pair has neither (likes Bob Carol) nor
    its negation stored → open premise passes → rule fires."""
    kb = _setup_three_states()
    results = _run(kb, "find-open")
    pairs = sorted((r["a"], r["b"]) for r, _ in results)
    # All ordered (a, b) pairs of Person-instances except those
    # where (likes …) is committed positive or negative:
    # - (Alice, Bob)   blocked (asserted)
    # - (Alice, Carol) blocked (negated)
    # - all others open:
    expected = sorted([
        ("Alice", "Alice"), ("Bob", "Bob"), ("Carol", "Carol"),  # excluded by neq
    ])
    # (neq excludes the diagonals; remove them from expected)
    expected = [(a, b) for a, b in [
        ("Bob",   "Alice"),
        ("Bob",   "Carol"),
        ("Carol", "Alice"),
        ("Carol", "Bob"),
    ]]
    assert pairs == sorted(expected)


def test_open_does_not_fire_on_asserted_pair():
    """(Alice, Bob) — `(likes Alice Bob)` is asserted; absent
    P fails; open is false."""
    kb = _setup_three_states()
    results = _run(kb, "find-open")
    pairs = {(r["a"], r["b"]) for r, _ in results}
    assert ("Alice", "Bob") not in pairs


def test_open_does_not_fire_on_negated_pair():
    """(Alice, Carol) — `(not (likes Alice Carol))` is stored;
    absent (not P) fails; open is false."""
    kb = _setup_three_states()
    results = _run(kb, "find-open")
    pairs = {(r["a"], r["b"]) for r, _ in results}
    assert ("Alice", "Carol") not in pairs


def test_open_with_no_bindings_on_empty_kb():
    """An (open …) on a fact-pattern with vars bound by outer
    premises — when no facts of EITHER state exist, every
    ?a/?b combination from the outer enumeration qualifies
    as open."""
    kb = _kb("""
    (rules
      (rule find-open ()
        :match (and (is-a ?a Person) (is-a ?b Person) (neq ?a ?b)
                    (open (likes ?a ?b)))
        :assert (open-likes ?a ?b)
        :why "all-undecided"))
    (ontology
      (relation likes T T) (relation is-a T T)
      (is-a Person T)
      (is-a Alice Person) (is-a Bob Person))
    """)
    results = _run(kb, "find-open")
    # Alice—Bob and Bob—Alice, both open.
    assert len(results) == 2
