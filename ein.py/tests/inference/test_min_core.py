"""Minimal unsat core — S1.9.E19."""
from __future__ import annotations

from pathlib import Path

from ein.inference.contradiction import ContradictionDetector
from ein.inference.min_core import minimal_unsat_core
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


def _saturated(text: str) -> KnowledgeBase:
    kb = KnowledgeBase.from_ir(parse(text))
    list(Saturator(kb).saturate())
    return kb


def _union_core(kb: KnowledgeBase) -> set:
    return set(kb.unsat_core(
        c.witness for c in ContradictionDetector(kb).detect()))


class TestMinimalUnsatCore:
    def test_no_contradiction_is_empty(self):
        assert minimal_unsat_core(_saturated(CONSISTENT)) == frozenset()

    def test_single_contradiction_returns_a_sound_frontier(self):
        kb = _saturated(FUNCTIONAL)
        core = minimal_unsat_core(kb)
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
        minimal = minimal_unsat_core(kb)
        assert minimal <= union
        assert 0 < len(minimal) <= 5             # vs the 38-fact union
        assert len(minimal) < len(union)
