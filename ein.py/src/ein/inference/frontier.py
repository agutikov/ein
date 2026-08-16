"""Smallest recorded contradiction frontier — S1.9.E19, renamed per R3.

`smallest_contradiction_frontier(kb)` returns the smallest **recorded**
single-witness source frontier: over the contradiction witnesses the
detector finds, the smallest set of given/hypothesis leaves of **one**
witness's recorded derivation. Sound by construction and NAF-safe — it is a
real derivation's leaves, never a re-saturated guess. It is **not** a
subset-minimal MUS (no guarantee that every proper subset is satisfiable),
and it is minimal only over *recorded* derivations: the KB keeps one
justification per fact (first derivation wins — ``store.add_and_index_fact``),
so a shorter derivation that fired later is invisible and the result can
depend on rule-firing order.

Why this shape. `unsat_core(witnesses)` unions the source frontier of *every*
contradiction witness. When one cause propagates it fans out into many
witnesses, so the union over-states the conflict — e.g. `zebra2-bad`'s one
injected fact produces **123** witnesses whose frontiers union to **38** facts,
yet each witness is a complete contradiction on its own with a **1-5** fact
frontier, the smallest being exactly the injected culprit. The smallest
single-witness frontier is therefore the most legible explanation the recorded
provenance supports.

Why provenance, not re-saturation. The textbook MUS minimiser — drop a fact,
re-saturate, keep it dropped if still ⊥ — is **unsound under NAF here**: removing
a fact can flip an `(absent P)` premise and *fabricate* a contradiction that the
full KB never had, so "the subset still contradicts" need not mean "the subset
reproduces the real contradiction" (corollary C3 of the normative NAF
semantics, ``docs/kernel/inference/absent_semantics.md``). Measured on
`zebra2-bad`, that minimiser
degenerates to a structural-declaration residue (relation/property decls whose
removal merely breaks saturation), not the conflict. A single witness's frontier
is instead a *real* derivation's leaves — sound by construction, NAF-safe, and
cheap (provenance walks via E6's `walk_premises`, no saturation).

Lifting the recorded-derivations caveat needs OR/AND multi-justification
provenance (record every derivation at the dedup seam, then search the proof
DAG for a true minimum) — parked as P1.9 catalog entry **E25**
(``plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/
s1.9.e25_multi_justification_provenance.md``).
"""
from __future__ import annotations

from collections.abc import Iterable

from ein.inference.contradiction import ContradictionDetector
from ein.kb.entities import Fact
from ein.kb.store import KnowledgeBase


def smallest_contradiction_frontier(
    kb: KnowledgeBase, witnesses: Iterable[Fact] | None = None,
) -> frozenset[Fact]:
    """Smallest recorded single-witness source frontier of ``kb``'s contradictions.

    ``witnesses`` defaults to the witness fact of every
    :class:`~ein.inference.contradiction.Contradiction` the detector finds.
    Returns ``frozenset()`` when there is no contradiction. Each candidate
    frontier is ``kb.unsat_core([w])`` — one witness's recorded derivation
    leaves — so the result is a sound, NAF-safe explanation, never a
    re-saturated guess. **Not** a subset-minimal MUS, and minimal only over
    *recorded* derivations (one justification per fact, first derivation
    wins), so the result can depend on rule-firing order — see the module
    docstring.
    """
    if witnesses is None:
        witnesses = [c.witness for c in ContradictionDetector(kb).detect()]
    best: frozenset[Fact] | None = None
    for w in witnesses:
        frontier = frozenset(kb.unsat_core([w]))
        if best is None or len(frontier) < len(best):
            best = frontier
    return best if best is not None else frozenset()


__all__ = ["smallest_contradiction_frontier"]
