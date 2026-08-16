"""Smallest recorded contradiction frontier — S1.9.E19, renamed per R3."""
from __future__ import annotations

from pathlib import Path

from ein.inference.contradiction import ContradictionDetector
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
# runs; lower priority fires first, and the first derivation's provenance is
# the one the KB records (store.add_and_index_fact — first derivation wins).
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

    def test_result_depends_on_recorded_derivation_order(self):
        # The executable form of the "minimal only over recorded derivations"
        # caveat (R3): one justification per fact — first derivation wins — so
        # flipping the two deriving rules' priorities flips the recorded
        # provenance of (X a) and with it the reported frontier: {C, Y} when
        # chain fires first vs {A, B, Y} when join does, even though the
        # 2-fact explanation still exists in the second run. Each result is
        # still a sound frontier (⊆ the union core). A future
        # multi-justification fix (P1.21 S1.21.7) should flip this test
        # deliberately.
        def run(join: int, chain: int) -> tuple[frozenset, set]:
            kb = _saturated(TWO_DERIVATIONS.format(join=join, chain=chain))
            return smallest_contradiction_frontier(kb), _union_core(kb)

        chain_first, union_a = run(join=100, chain=50)
        join_first, union_b = run(join=50, chain=100)
        assert chain_first <= union_a            # sound: real derivation leaves
        assert join_first <= union_b
        names_a = {f.relation_name for f in chain_first}
        names_b = {f.relation_name for f in join_first}
        assert names_a == {"C", "Y"}             # the short recorded derivation
        assert names_b == {"A", "B", "Y"}        # {C, Y} exists but is invisible
        assert names_a != names_b                # priority alone flips the answer
