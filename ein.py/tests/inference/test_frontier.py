"""Smallest contradiction frontier — S1.9.E19, renamed per R3, OR-aware per S1.21.7."""
from __future__ import annotations

from pathlib import Path

from ein.inference.config import SolverConfig
from ein.inference.contradiction import ContradictionDetector
from ein.inference.explain import (
    ExplanationBudget,
    minimal_contradiction_frontier,
)
from ein.inference.frontier import smallest_contradiction_frontier
from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]

# A single functional clash → one (false) witness, frontier = the clashing pair.
FUNCTIONAL = """
(rule functional (?R)
  :match  (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))
  :assert (false)
  :why    "fn" :priority 100)
(relation R T T)
(functional R)
(R x One :source "(1)")
(R x Two :source "(2)")
"""

CONSISTENT = """
(relation R T T)
(R x One :source "(1)")
"""

# Two derivations of the same fact (X a) plus one clash — the R3 report's E3
# fixture. Only the two deriving rules' :priority values are swapped between
# runs; lower priority fires first, so which derivation becomes (X a)'s
# *primary* provenance flips between runs. Since S1.21.7 the loser is kept as
# an alternative justification, so the reported frontier no longer flips
# with it.
TWO_DERIVATIONS = """
(relation A T)
(relation B T)
(relation C T)
(relation X T)
(relation Y T)
(rule join ()
  :match  (and (A ?o) (B ?o))
  :assert (X ?o)
  :why    "join" :priority {join})
(rule chain ()
  :match  (C ?o)
  :assert (X ?o)
  :why    "chain" :priority {chain})
(rule clash ()
  :match  (and (X ?o) (Y ?o))
  :assert (false)
  :why    "clash" :priority 300)
(A a :source "(A)")
(B a :source "(B)")
(C a :source "(C)")
(Y a :source "(Y)")
"""


def _saturated(text: str) -> KnowledgeBase:
    kb = KnowledgeBase.from_ir(parse(text))
    list(Saturator(kb).saturate())
    return kb


def _union_core(kb: KnowledgeBase) -> set:
    return set(kb.unsat_core(
        c.witness for c in ContradictionDetector(kb).detect()))


class TestSmallestContradictionFrontier:
    def test_no_contradiction_is_empty(self):
        assert smallest_contradiction_frontier(
            _saturated(CONSISTENT)) == frozenset()

    def test_single_contradiction_returns_a_sound_frontier(self):
        kb = _saturated(FUNCTIONAL)
        core = smallest_contradiction_frontier(kb)
        assert core                              # non-empty
        assert core <= _union_core(kb)           # a subset of the full frontier

    def test_zebra2_bad_shrinks_union_to_the_culprit(self):
        # 1 injected fact → 123 witnesses → 38-fact union; the smallest single
        # witness frontier is the tight, readable explanation. Provenance-only,
        # so this is fast (no re-saturation).
        kb = KnowledgeBase.from_file(
            str(REPO / "examples" / "ein-bugs" / "zebra2-bad.ein"))
        list(Saturator(kb).saturate())
        union = _union_core(kb)
        smallest = smallest_contradiction_frontier(kb)
        assert smallest <= union
        assert 0 < len(smallest) <= 5            # vs the 38-fact union
        assert len(smallest) < len(union)

    def test_result_is_independent_of_derivation_order(self):
        # S1.21.7 — the deliberate flip of
        # `test_result_depends_on_recorded_derivation_order`, which pinned the
        # R3 caveat as *current behaviour*: one justification per fact meant
        # the reported frontier was {C, Y} when `chain` fired first but
        # {A, B, Y} when `join` did, even though the 2-fact explanation still
        # existed in the second run. Re-derivations are now recorded as
        # alternative justifications and the frontier is searched over the
        # AND/OR graph, so both runs report the genuinely smallest
        # explanation. Priority no longer decides the answer.
        def run(join: int, chain: int):
            kb = _saturated(TWO_DERIVATIONS.format(join=join, chain=chain))
            return kb, smallest_contradiction_frontier(kb)

        kb_a, chain_first = run(join=100, chain=50)
        kb_b, join_first = run(join=50, chain=100)
        names_a = {f.relation_name for f in chain_first}
        names_b = {f.relation_name for f in join_first}
        assert names_a == {"C", "Y"}
        assert names_b == {"C", "Y"}
        assert names_a == names_b

        # The primary provenance still flips — the fix is in the *search*,
        # not in which derivation wins the dedup race.
        prov_a = kb_a._fact_by_id("X", ("a",)).provenance
        prov_b = kb_b._fact_by_id("X", ("a",)).provenance
        assert (prov_a.rule, prov_b.rule) == ("chain", "join")

        # ... and the loser is retained as an alternative justification.
        assert [p.rule for p in kb_b._alt_justifications[("X", ("a",))]] \
            == ["chain"]

        # Soundness envelope: an explanation only ever names facts the
        # OR-aware premise closure reaches. (It is NOT a subset of the
        # primary-only union core — in run b that core is {A, B, Y}, which is
        # exactly the over-report S1.21.7 removes.)
        for kb, core in ((kb_a, chain_first), (kb_b, join_first)):
            witnesses = [c.witness for c in ContradictionDetector(kb).detect()]
            assert core <= kb.unsat_core(witnesses, all_justifications=True)
        assert not join_first <= _union_core(kb_b)

    def test_search_is_budgeted_and_reports_exhaustion(self):
        # The minimality search is worst-case exponential, so it runs under an
        # explicit anytime budget; a fully-explored search says so.
        kb = _saturated(TWO_DERIVATIONS.format(join=50, chain=100))
        result = minimal_contradiction_frontier(kb)
        assert result.exhausted is True
        assert {f.relation_name for f in result.frontier} == {"C", "Y"}
        assert result.target is not None
        assert result.rounds > 0

        # A budget that keeps one environment per fact still returns a sound
        # frontier, and admits it may not be the smallest.
        tight = minimal_contradiction_frontier(
            kb, budget=ExplanationBudget(max_rounds=1))
        assert tight.exhausted is False
        assert tight.frontier <= _union_core(kb) | {
            f for f in kb.facts if f.relation_name == "C"}

    def test_alternatives_off_restores_single_justification_behaviour(self):
        # The config gate is a real off switch: with recording disabled the
        # frontier is the pre-S1.21.7 recorded-primary answer, order and all.
        text = TWO_DERIVATIONS.format(join=50, chain=100)
        kb = KnowledgeBase.from_ir(parse(text))
        kb.config = SolverConfig(record_alternative_justifications=False)
        list(Saturator(kb).saturate())
        assert kb._alt_justifications == {}
        assert {f.relation_name for f in smallest_contradiction_frontier(kb)} \
            == {"A", "B", "Y"}
