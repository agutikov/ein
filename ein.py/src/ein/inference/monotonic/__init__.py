"""Unified monotonic + lattice set-search engine.

**One engine, one entry, three answers.** :func:`solve` is the single
public entry. It runs the set-indexed lattice exploration (Apriori
prefix-join over commitment sets, each entered via
:func:`ein.inference.commitment.try_commitment_set`), recording every
solution node (``consistent ∧ complete`` — no open hypothesis) deduped by
:func:`state_hash` *and* every refuted commitment, then derives the verdict
from the count ``k`` via :func:`verdict_of`:

- ``k = 1`` → :class:`Solution` — the model (certified unique iff the
  search exhausted).
- ``k > 1`` → :class:`Ambiguity` — ``k`` distinct models (a *gap*).
- ``k = 0`` → :class:`Contradiction` — unsat (``unsat_core`` = union of the
  refuted commitments' source-frontier cores), when exhausted.

These are three **answers to one problem** (unsat / unique / under-determined),
selected by the *input*, never by which function was called. The former
``gaps_solve`` / ``contradictions_solve`` sibling entries — which chose
``Ambiguity`` / ``Contradiction`` up front regardless of ``k`` — were unsound
and have been removed (2026-06-16); their views are now read off this one
engine's result.

**Stop policy** (orthogonal to the verdict). ``stop_after=1`` is the sound
fast path (stop at the first complete∧consistent node — ``stats.exhausted``
False, so a ``k=1`` reads as "a model", not certified-unique); ``stop_after=N``
collects up to ``N`` distinct models; ``stop_after=None`` exhausts the lattice
(certifies unique / ambiguous / unsat).

**``store_lattice`` (opt-in).** Attaches a sound :class:`LatticeProof` to the
verdict — the solution set (gaps view), the refutation map with per-commitment
unsat cores (contradictions view), and the learnt no-goods — so the markdown
reductio trace and ``render lattice`` read their views off this run. Off by
default (the fast path needn't pay for the proof packaging).

**Algorithm sketch.** Single :class:`KnowledgeBase` instance (root);
commitment sets generated layer-by-layer via the Apriori prefix-join
(:mod:`ein.inference.apriori`) and entered via
:func:`ein.inference.commitment.try_commitment_set`. Flat root-writes on every
outcome — saturation commutativity makes a per-parent bubble redundant (see
``algorithm_layer_n.md``).

**Augmentations** (default-on, gated by SolverConfig):

- CDCL nogoods — every dead entering emits ``frozenset(C)`` into
  ``root_kb._nogoods``; subsequent layers' candidate filter
  (:func:`ein.inference.apriori.filter_candidate`) catches supersets.
- Singleton-death writeback — for size-1 dead clauses, writes ``(not h)`` to
  ``root_kb._negated_facts`` so subsequent ``_compute_alive`` drops h.
- Forced-positive promotion — when alive shrinks to a singleton, promote it to
  a root fact.

**Diagnostics.** :class:`MonotonicDumper` (S1.5b.7) receives lifecycle
callbacks; :class:`ProgressDumper` streams live progress.

See [P1.5b README](../../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/README.md)
+ ``project_set_search_unified`` memory for the design rationale.
"""
from ein.inference.monotonic.contract import (
    validate_proof_for_explanation,
)
from ein.inference.monotonic.lattice import (
    DeadCommitment,
    LatticeProof,
    LatticeStats,
    SetNode,
    SolutionRecord,
)
from ein.inference.monotonic.snapshot import (
    LatticeSnapshotV1,
    lattice_snapshot,
)
from ein.inference.monotonic.solver import (
    BudgetExceededError,
    MonotonicStats,
    solve,
    verdict_of,
)
from ein.inference.monotonic.state_dump import (
    LatticeDumper,
    MonotonicDumper,
    ProgressDumper,
)

__all__ = [
    "BudgetExceededError",
    "DeadCommitment",
    "LatticeDumper",
    "LatticeProof",
    "LatticeSnapshotV1",
    "LatticeStats",
    "MonotonicDumper",
    "MonotonicStats",
    "ProgressDumper",
    "SetNode",
    "SolutionRecord",
    "lattice_snapshot",
    "solve",
    "validate_proof_for_explanation",
    "verdict_of",
]
