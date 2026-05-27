"""Monotonic set-search engine.

The monotonic engine is one of two set-indexed search
implementations under ``inference/`` — the peer lattice engine
(coming in S1.5b.20+) is the exhaustive shape; the monotonic
engine is the first-solution-wins shape that suffices for SOLVE
mode (Q1.5b.7).

**Algorithm sketch.** Single :class:`KnowledgeBase` instance
(root); commitment sets generated layer-by-layer via the
Apriori prefix-join (:mod:`ein_bot.inference.apriori`) and
entered via the common
:func:`ein_bot.inference.commitment.try_commitment_set`
primitive. Termination conditions, in order of precedence:

- :class:`Solution` — the fork's saturated kb satisfies
  ``is_solved`` (algorithm_layer_n.md §3d.vii — the fork-side
  check fires before any unconditional-facts merge);
  alternatively, root.kb itself satisfies after a forced-positive
  cascade or after merging unconditional facts.
- :class:`Contradiction` — root is contradictory at any point,
  or every alive hypothesis dies and ``_compute_alive`` returns
  empty.
- :class:`Ambiguity` — layer-cap reached with alive ≠ ∅ and
  no satisfying commitment found (the partial-state outcome).

**Augmentations** (default-on, gated by SolverConfig):

- CDCL nogoods (S1.5b.6) — every dead entering emits
  ``frozenset(C)`` into ``root_kb._nogoods``; the next layer's
  ``generate_layer`` filters supersets.
- Singleton-death writeback — for size-1 dead clauses, writes
  ``(not h)`` to ``root_kb._negated_facts`` so subsequent
  ``_compute_alive`` drops h.
- Forced-positive promotion (S1.5b.5b) — when alive shrinks to
  a singleton, promote it to a root fact.

**Diagnostics.** Optional :class:`MonotonicDumper` captures
per-layer ``.ein`` snapshots + a ``00_timeline.jsonl`` event
log + ``summary.json`` (S1.5b.7).

**SOLVE mode only.** GAPS / CONTRADICTIONS belong to the
lattice engine (Q1.5b.7); the monotonic loop raises
``NotImplementedError`` for the other modes.

See [P1.5b README](../../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/README.md)
for the design rationale and the equivalence claim against the
lattice engine.
"""
from ein_bot.inference.monotonic.solver import (
    BudgetExceededError,
    monotonic_solve,
)

__all__ = ["BudgetExceededError", "monotonic_solve"]
