"""Solution-node predicates — `complete` / `consistent` / `open_hypotheses`.

`complete` is the emptiness test on the open-hypothesis set, and since
S1.9.E16 it asks the generator for its *first* element instead of building
the set. These pin the equivalence that makes that legal, on both sides
(a KB with candidates left, and a KB with none), plus the short-circuit
itself — that a non-complete KB is decided without draining the generator.
"""
from __future__ import annotations

from ein.inference.solution import (
    complete,
    consistent,
    is_solution_node,
    open_hypotheses,
)
from ein.ir import parse
from ein.kb.entities import Fact
from ein.kb.provenance import Provenance
from ein.kb.store import KnowledgeBase

# Two objects, one binary relation, nothing decided ⇒ the blind
# enumerator has candidates to offer.
_OPEN = """
(relation r T T)
(is-a a T) (is-a b T)
"""

# No relation to hypothesise over ⇒ the enumerator yields nothing.
_CLOSED = """
(is-a a T) (is-a b T)
"""


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def test_complete_agrees_with_the_open_set_when_candidates_exist():
    kb = _kb(_OPEN)
    assert open_hypotheses(kb)                    # non-empty
    assert complete(kb) is False
    assert complete(kb) == (not open_hypotheses(kb))


def test_complete_agrees_with_the_open_set_when_none_exist():
    kb = _kb(_CLOSED)
    assert open_hypotheses(kb) == frozenset()
    assert complete(kb) is True
    assert complete(kb) == (not open_hypotheses(kb))


def test_complete_short_circuits_on_the_first_candidate():
    """The S1.9.E16 point: deciding "not complete" must not enumerate.

    Counts what the generator is asked for by wrapping it — the
    materialising `open_hypotheses` drains it, `complete` takes one.
    """
    from ein.inference import solution as sol

    real = sol.generate_hypotheses
    taken = 0

    def counting(kb):
        nonlocal taken
        for fact in real(kb):
            taken += 1
            yield fact

    sol.generate_hypotheses = counting
    try:
        kb = _kb(_OPEN)
        taken = 0
        assert complete(kb) is False
        assert taken == 1, "complete() consumed more than the first candidate"

        taken = 0
        n = len(open_hypotheses(kb))
        assert taken >= n > 1, "the materialising path still drains the generator"
    finally:
        sol.generate_hypotheses = real


def test_is_solution_node_needs_both_halves():
    """consistent ∧ complete — a KB missing either is not a solution node."""
    open_kb = _kb(_OPEN)
    assert consistent(open_kb)
    assert not is_solution_node(open_kb)          # consistent, not complete

    closed_kb = _kb(_CLOSED)
    assert is_solution_node(closed_kb)            # both halves

    broken = _kb(_CLOSED)
    broken._index_fact(broken.add_fact(Fact(
        relation_name="false", args=(),
        provenance=Provenance.from_rule(rule="functional"),
    )))
    assert complete(broken)
    assert not consistent(broken)
    assert not is_solution_node(broken)           # complete, not consistent
