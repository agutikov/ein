"""Solve-loop data types + verdict reading (split out of solver.py).

Leaf module of the monotonic engine: the `BudgetExceededError`, the
`MonotonicStats` / `_LatticeLoopState` dataclasses, the unsat-core synthesis
helpers, and `verdict_of` (the k → verdict reading). Imported by `_helpers.py`
and `solver.py`; depends on neither.
"""
from __future__ import annotations

from collections.abc import Iterable
from dataclasses import dataclass, field

from ein.inference.apriori import (
    CanonicalSetId,
)
from ein.inference.contradiction import ContradictionDetector
from ein.inference.monotonic.lattice import (
    DeadCommitment,
    SetNode,
    SolutionRecord,
    _BaseStats,
)
from ein.inference.verdict import (
    Ambiguity,
    Contradiction,
    Solution,
    Verdict,
)
from ein.kb.entities import Fact
from ein.kb.store import KnowledgeBase


class BudgetExceededError(RuntimeError):
    """Raised by :func:`solve` when ``max_time`` or ``max_enterings`` is hit
    before the solve completes.

    Carries the partial :class:`MonotonicStats` so callers can
    print the work done before the abort.
    """

    def __init__(self, reason: str, stats: MonotonicStats) -> None:
        super().__init__(reason)
        self.reason = reason
        self.stats = stats


@dataclass
class MonotonicStats(_BaseStats):
    """Cumulative counters for one :func:`solve` run.

    Inherits the shared per-candidate counters from :class:`_BaseStats`
    (defined in :mod:`ein.inference.monotonic.lattice`); adds the two
    ``solve``-entry extras below. :class:`LatticeStats` (the proof's stats,
    built by :func:`_build_lattice_stats`) is the sibling subclass — neither
    inherits the other. The base counters lead, so the ``summary.json`` field
    order is unchanged."""

    # P1.7a — solve entry: deduped solution-node count `k` and whether
    # the search was exhaustive (else `k` is a lower bound and a k=1
    # result is "a solution", not a proven-unique one).
    solution_nodes:      int = 0
    exhausted:           bool = True


@dataclass
class _LatticeLoopState:
    """Mutable accumulator threaded through :func:`_explore_layers` via
    :attr:`_LoopCtx.lstate`. Holds what the single ``solve`` loop collects:

    - :attr:`solution_nodes` / :attr:`truncated` — the deduped solution nodes
      (``state_hash`` → record), written by :func:`_record_node`, read by
      :func:`verdict_of` / :func:`_finalise_solve` / the ``stop_after`` +
      depth-cap gates. ``truncated`` records a ``stop_after`` / depth-cap cut
      (→ ``stats.exhausted = False``).
    - :attr:`dead_commitments` — every refuted commitment, written by
      :func:`_root_dead` / :func:`_handle_dead`, read by :func:`verdict_of`
      for the ``k=0`` core and packaged into the proof's refutation map.
    - :attr:`alive_at_end_tuple` — the size-N frontier left alive iff the
      depth cap was the loop terminator (``()`` otherwise).
    - :attr:`kb_index` / :attr:`state_hash_merges` — the per-SetNode DAG store
      + its merge counter. ``solve`` builds no DAG, so these stay empty / 0;
      they remain as the home of the merge semantics (:func:`_record_setnode`)
      and keep the proof's :class:`LatticeStats` field populated.
    """

    dead_commitments:  list[DeadCommitment] = field(default_factory=list)
    kb_index:          dict[int, SetNode] = field(default_factory=dict)
    alive_at_end_tuple: tuple[CanonicalSetId, ...] = ()
    state_hash_merges: int = 0
    # Deduped solution nodes (state_hash → record) and whether the search ran
    # out of lattice (exhausted) or was cut short by ``stop_after`` / the
    # depth cap (``truncated``).
    solution_nodes:    dict[int, SolutionRecord] = field(default_factory=dict)
    truncated:         bool = False


# ── Unsat-core synthesis (single home — F-ENG-7) ──────────────
#
# Two shapes recur across the verdict-building sites: a union over
# every recorded dead's core, and a fresh source-frontier walk of a
# kb that already holds a contradiction. Both live here so a fix to
# the core derivation can't miss a copy. (``commitment.py`` has a
# third source-frontier site, but it already holds the ``detect()``
# result in hand and sits below this module in the import graph, so
# it stays inline.)


def _union_dead_cores(deads: Iterable[DeadCommitment]) -> frozenset[Fact]:
    """Union the unsat cores of every recorded dead commitment — the
    payload of the ``k=0`` / contradictions verdict."""
    cores: frozenset[Fact] = frozenset()
    for d in deads:
        cores = cores | d.unsat_core
    return cores


def _source_frontier_core(kb: KnowledgeBase) -> frozenset[Fact]:
    """The unsat core for a contradiction already present in ``kb``:
    walk each witness's derivation DAG back to its ``source``-kind
    terminals via :meth:`KnowledgeBase.unsat_core`. ``frozenset()``
    when ``kb`` is in fact consistent."""
    contras = ContradictionDetector(kb).detect()
    if not contras:
        return frozenset()
    return frozenset(kb.unsat_core(c.witness for c in contras))


def verdict_of(lstate: _LatticeLoopState, *, exhausted: bool) -> Verdict:
    """Derive the verdict from the deduped solution-node count ``k`` (P1.7a).

    | k  | verdict          | meaning                                     |
    |----|------------------|---------------------------------------------|
    | 1  | ``Solution``     | the model (unique iff ``exhausted``)        |
    | >1 | ``Ambiguity``    | ``k`` distinct models (a gap)               |
    | 0  | ``Contradiction``| unsat (core = union of dead cores) if exhausted|

    The query ``:goal`` does NOT decide this — it projects over the model(s)
    afterwards (S1.7a.6). A solution is always the same thing; only how many
    we found and whether we exhausted the lattice pick the type. ``exhausted``
    is surfaced to the caller via ``stats.exhausted`` (a ``k=0`` from a
    truncated run is NOT proven-unsat; a ``k=1`` from a ``stop_after`` run is
    "a model", not proven-unique).
    """
    nodes = list(lstate.solution_nodes.values())
    k = len(nodes)
    if k == 1:
        n = nodes[0]
        return Solution(kb=n.kb, trace=n.firings)
    if k > 1:
        return Ambiguity(
            branches=tuple(Solution(kb=n.kb, trace=n.firings) for n in nodes),
        )
    return Contradiction(unsat_core=_union_dead_cores(lstate.dead_commitments))
