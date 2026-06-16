"""Monotonic engine main loop.

Set-indexed BFS over commitment sets. Single ``KnowledgeBase``
instance (root); each layer's candidates come from the Apriori
prefix-join (:mod:`ein.inference.apriori`); each candidate
is entered via the common
:func:`ein.inference.commitment.try_commitment_set` primitive.
Every solution node (``consistent ∧ complete``) is recorded, deduped
by ``state_hash``; the loop terminates on the ``stop_after`` count, on
a root contradiction, or on layer exhaustion, and the verdict is read
from the count ``k`` (:func:`verdict_of`) — one engine, three answers.

Termination conditions
----------------------

- :class:`Solution` at a fork — ``complete`` (consistent ∧ no open
  hypothesis) on an alive entering. Returns ``Solution(kb=result.kb)`` so
  the caller sees the hypothesis-and-derivations context the
  goal depended on. **Required** for puzzles whose goal directly
  references hypothesis facts (e.g. branching/05_mini_zebra).
- :class:`Solution` at root — after merging unconditional facts
  from an alive commitment, a forced-positive cascade may
  promote remaining singleton hypotheses to root and is_solved
  fires there (e.g. zebra2 — one alive commitment cascades into
  30 unconditional facts that complete the puzzle at root).
- :class:`Contradiction` — root saturates to ``(false)`` in
  Phase 1, or every layer-1 singleton dies and ``_compute_alive``
  returns empty in Phase 3.
- :class:`Ambiguity` — layer cap reached with alive ≠ ∅ and no
  satisfying commitment found.

CDCL (S1.5b.6)
--------------

Every dead entering emits ``frozenset(C)`` into
``root_kb._nogoods`` via :func:`ein.inference.nogoods.emit_nogood`
(``min_size=1`` so layer-1 singleton deaths land — Q1.5b.5.c).
Singleton dead clauses additionally write ``(not h)`` to
``root_kb._negated_facts`` via :func:`_emit_negated_fact_writeback`
— a flat root-write, no ancestor-chain coupling. (S1.7.24 — no
symmetric-mirror: the counterpart dies on
its own branch, recovered at the generic state_hash dedup.)

Budget (S1.5b.7 / bench CLI parity)
-----------------------------------

``max_time`` (wall-clock) + ``max_enterings`` (per-call cap) raise
:class:`BudgetExceededError` with partial stats. The dumper's
timeline is flushed via :meth:`MonotonicDumper.close` on abort;
``summary.json`` is **not** emitted then (the timeline events
suffice for diagnostic).

Dumper hooks (S1.5b.7)
----------------------

Optional :class:`MonotonicDumper` receives lifecycle callbacks
at five sites: ``root_initial``, ``layer_start``, ``entering``,
``layer_end``, ``summary``. The single ``_finish`` exit helper
guarantees ``summary`` lands on every non-abort path.

One entry (P1.7a)
-----------------

:func:`solve` is the single public entry. It records every solution node
and every refuted commitment, then reads the verdict from the count ``k`` of
distinct solution nodes via :func:`verdict_of` (``k = 0 / 1 / >1`` →
Contradiction / Solution / Ambiguity). The former ``gaps_solve`` /
``contradictions_solve`` sibling entries (which chose the verdict up front)
were removed; their views are read off ``solve``'s optional ``LatticeProof``.
"""
from __future__ import annotations

import random
import time
from collections.abc import Iterable
from dataclasses import dataclass, field, fields, replace
from typing import Literal

from ein.inference import primitives
from ein.inference.apriori import (
    CanonicalSetId,
    generate_layer,
    layer_1,
    order_candidates,
)
from ein.inference.canon import state_hash
from ein.inference.commitment import try_commitment_set
from ein.inference.config import SolverConfig
from ein.inference.contradiction import ContradictionDetector
from ein.inference.monotonic.lattice import (
    DeadCommitment,
    LatticeProof,
    LatticeStats,
    SetNode,
    SolutionRecord,
    _BaseStats,
)
from ein.inference.monotonic.state_dump import (
    LatticeDumper,
    MonotonicDumper,
)
from ein.inference.nogoods import emit_nogood
from ein.inference.saturator import Saturator
from ein.inference.solution import complete, open_hypotheses
from ein.inference.verdict import (
    Aborted,
    Ambiguity,
    Contradiction,
    Mode,
    Solution,
    Verdict,
    is_solved,
)
from ein.kb.entities import Fact, Layer
from ein.kb.provenance import FactId, Provenance
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


def solve(
    root_kb: KnowledgeBase,
    *,
    stop_after: int | None = None,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    dumper: MonotonicDumper | LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
    store_lattice: bool = False,
    on_budget: Literal["raise", "verdict"] = "raise",
) -> tuple[Verdict | Aborted, MonotonicStats]:
    """The one solver entry — its verdict is *read* from the result
    (``k`` distinct solution nodes) rather than chosen up front (P1.7a).

    ``k = 0 / 1 / >1`` is read as ``Contradiction / Solution / Ambiguity``
    by :func:`verdict_of` — three *answers* to one problem (unsat / unique /
    gaps), never three problem statements. The former ``gaps_solve`` /
    ``contradictions_solve`` sibling entries — which chose ``Ambiguity`` /
    ``Contradiction`` by *which function was called* — are gone; their views
    are now read off this one engine's result and optional proof.

    ``store_lattice`` (opt-in) attaches a sound :class:`LatticeProof` to the
    verdict — the full solution set (gaps view) + refutation map
    (contradictions view) + the per-fact unsat cores — so ``render lattice``
    and the reductio markdown trace read their views off this run. Off by
    default (the fast path needn't pay for the proof packaging).

    Runs the set-indexed lattice exploration, recording every **solution
    node** (``consistent ∧ complete`` — no open hypothesis) deduped by
    :func:`state_hash`, and derives the verdict via :func:`verdict_of` from
    the count ``k``. ``stats.solution_nodes`` is ``k``; ``stats.exhausted``
    says whether the lattice was fully explored.

    ``stop_after`` bounds the search to the first ``n`` distinct solution
    nodes (``None`` = exhaust) — the orthogonal stop policy. ``stop_after=1``
    is the sound fast path: it stops on the first complete∧consistent node
    (never a partial goal-match, unlike the removed first-goal-match entry),
    but sets ``stats.exhausted=False`` so a ``k=1`` result reads as "a model",
    not certified-unique.

    Unlike the removed first-goal-match entry (first goal-pattern match —
    unsound) and ``gaps_solve`` (stops at root goal-match — masks
    ambiguity), this entry
    terminates only on lattice exhaustion, ``stop_after``, or budget. See
    ``plans/m1_core_graph_reasoning/p1.7a_solution_search_refactor/``.

    ``on_budget`` (S1.9.E17.2): ``"raise"`` (default) raises
    ``BudgetExceededError`` on a ``max_time`` / ``max_enterings`` cut;
    ``"verdict"`` instead **returns** an
    :class:`~ein.inference.verdict.Aborted` verdict (carrying the partial
    stats) so a caller needn't catch the exception.
    """
    try:
        return _explore_layers(
            root_kb,
            stop_after=stop_after,
            max_set_size=max_set_size,
            config=config,
            store_lattice=store_lattice,
            dumper=dumper,
            max_time=max_time,
            max_enterings=max_enterings,
        )
    except BudgetExceededError as e:
        if on_budget == "verdict":
            return Aborted(reason=e.reason, stats=e.stats), e.stats
        raise


def _finish(
    dumper: MonotonicDumper | LatticeDumper | None,
    verdict: Verdict,
    stats: MonotonicStats,
) -> tuple[Verdict, MonotonicStats]:
    """Single exit hook — emits ``dumper.proof_summary`` (when the
    verdict carries a :class:`LatticeProof`) followed by
    ``dumper.summary``. The two-step shape lets the
    :class:`LatticeDumper` materialise its ``kb_index/`` folder +
    top-level ``proof_summary.json`` index before the cumulative
    summary lands. Monotonic verdicts have ``proof = None``, so
    :meth:`proof_summary` is skipped on that path.
    """
    if dumper is not None:
        proof = getattr(verdict, "proof", None)
        if proof is not None and hasattr(dumper, "proof_summary"):
            dumper.proof_summary(proof)
        dumper.summary(verdict, stats)
    return verdict, stats


# ── Helpers ──────────────────────────────────────────────────


def _emit_negated_fact_writeback(
    root_kb: KnowledgeBase, h_id: FactId,
) -> None:
    """For a singleton dead clause ``{h_id}``, write ``(not h)``
    into ``root_kb`` so ``generate_hypotheses`` excludes ``h``
    on the next saturate and the next ``_compute_alive`` shrinks
    ``alive`` accordingly.

    S1.7.24 — no symmetric mirror: the counterpart ``(R b a)`` is
    NOT proactively negated. With the open-set canonicalisation also
    gone (`solution.open_hypotheses`), the two orientations are
    independent open entries; the counterpart dies on its own branch
    (re-derivation via the user's `(rule symmetric)` hits the same
    ⊥), and the two branches collapse at the generic
    `canon.state_hash` solution-node dedup. The kernel keys on
    ``is_symmetric`` nowhere.

    Writes ``(not h)`` into root with rule-provenance — no ancestor
    chain or eager-mode coupling (the lattice does flat root-writes).
    Idempotent: a pre-existing ``(not h)`` at root is left untouched.
    """
    rn, args = h_id
    _write_negation_local(root_kb, rn, args)


def _write_negation_local(
    root_kb: KnowledgeBase, rn: str, args: tuple,
) -> None:
    inner = Fact(
        relation_name=rn, args=args,
        layer=Layer.REASONING, provenance=None,
    )
    if root_kb._fact_by_id(primitives.NOT, (inner,)) is not None:
        return
    not_fact = Fact(
        relation_name=primitives.NOT,
        args=(inner,),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule="<monotonic-unconditional>",
            premises_raw=(),
        ),
    )
    root_kb.add_and_index_fact(not_fact)


def _promote_forced_positives(
    root_kb: KnowledgeBase,
    alive: frozenset[FactId],
    stats: MonotonicStats,
    *,
    check_goal: bool,
) -> tuple[frozenset[FactId], Verdict | None]:
    """Cascade: while ``alive`` is a singleton ``{h_unique}``,
    promote ``h_unique`` to a root fact, re-saturate, check
    contradiction + is_solved, recompute alive. Repeat.

    Returns ``(alive_after, verdict)`` — ``verdict`` is non-None
    iff the cascade hit a terminal condition (Solution /
    Contradiction); the caller should return it immediately.

    The promoted Fact uses ``Provenance.from_rule(
    "<forced-positive>", premises_raw=())`` so its provenance
    kind is "rule" with empty premises — this makes
    :func:`commitment._is_unconditional`'s walk pass through it as a
    non-hypothesis terminal, so future commit chains that pass
    through this fact don't get incorrectly flagged as
    conditional.

    Soundness: ``alive = {h}`` means every other slot-mate has
    been refuted (the singleton-death writeback wrote
    ``(not h_other)`` at root, or hypgen filtered it). Combined
    with the puzzle's slot exclusivity constraint, ``h`` must be
    true. The
    post-promotion :class:`ContradictionDetector` catches any
    surfacing inconsistency.
    """
    while len(alive) == 1:
        (rn, args) = next(iter(alive))
        promoted = Fact(
            relation_name=rn,
            args=args,
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="<forced-positive>", premises_raw=(),
            ),
        )
        root_kb.add_and_index_fact(promoted)
        stats.facts_merged += 1
        stats.forced_positives += 1

        _ = list(Saturator(root_kb).saturate())
        stats.saturate_count += 1

        if ContradictionDetector(root_kb).detect():
            return alive, _contradiction(root_kb)
        if check_goal and is_solved(root_kb, Mode.SOLVE):
            return alive, _solution(root_kb)
        alive = _compute_alive(root_kb)

    return alive, None


def _compute_alive(kb: KnowledgeBase) -> frozenset[FactId]:
    """The current alive set = the open-hypothesis set.

    Delegates to :func:`ein.inference.solution.open_hypotheses`
    (P1.7a) so there is exactly one canonical open-set definition;
    ``complete(kb) ≡ not _compute_alive(kb)``.
    """
    return open_hypotheses(kb)


def _solution(kb: KnowledgeBase) -> Verdict:
    return Solution(kb=kb, trace=())


def _contradiction(kb: KnowledgeBase) -> Verdict:
    """Contradiction verdict carrying the source-frontier unsat core.

    P1.7a: walk each contradiction witness's derivation DAG back to
    its ``source``-kind terminals via :meth:`KnowledgeBase.unsat_core`
    — the same call ``try_commitment_set`` makes per-commitment
    (``commitment.py``). Previously a stub returning ``frozenset()``,
    which left root-level contradictions (e.g. ``zebra2-bad``, where
    the clash fires during root saturation before any commitment)
    with an empty core.
    """
    return Contradiction(unsat_core=_source_frontier_core(kb))


# ── Core loop — _explore_layers ──────────────────────────────
#
# S1.5b.21: extracted from the pre-refactor monotonic-loop body. The single
# `solve` entry records solution nodes + refuted commitments and reads the
# verdict from the count `k` via `verdict_of` (P1.7a).


def _build_lattice_stats(
    mstats: MonotonicStats,
    *,
    solutions_found: int,
    state_hash_merges: int,
    elapsed_seconds: float,
) -> LatticeStats:
    """Project a :class:`MonotonicStats` plus the lattice-only
    counters into the public :class:`LatticeStats` shape.

    The shared base counters are copied generically off
    :class:`_BaseStats` — no hand-maintained field list, so a counter
    added to the base can't silently go uncopied (F-ENG-9). The
    lattice-only triple
    (``solutions_found`` / ``state_hash_merges`` / ``elapsed_seconds``)
    is supplied by the caller. Keeping :class:`MonotonicStats`
    free of the lattice counters preserves the :func:`solve`
    public surface unchanged.
    """
    shared = {f.name: getattr(mstats, f.name) for f in fields(_BaseStats)}
    return LatticeStats(
        **shared,
        solutions_found=solutions_found,
        state_hash_merges=state_hash_merges,
        elapsed_seconds=elapsed_seconds,
    )


def _record_setnode(
    lstate: _LatticeLoopState,
    *,
    entry: Literal["gaps", "contradictions"],
    commitment: CanonicalSetId,
    result_kb: KnowledgeBase,
    verdict_label: Literal["alive", "dead", "solution"],
    layer: int,
) -> bool:
    """The state-hash dedup MERGE primitive for the per-SetNode lattice DAG.

    ``solve`` does not build the DAG (so it never calls this), but this is the
    home of the merge semantics, exercised directly at the unit level + reused
    by any DAG builder. Under ``entry == "contradictions"`` the dict is keyed
    by the post-saturation :func:`state_hash`; on collision the existing node's
    ``labels`` tuple grows and ``state_hash_merges`` ticks (returns ``True``).
    Under ``entry == "gaps"`` it is keyed by ``hash(commitment)`` so distinct
    commitments stay separate even when their post-saturation kbs collide."""
    h_state = state_hash(result_kb)
    if entry == "contradictions":
        existing = lstate.kb_index.get(h_state)
        if existing is not None:
            lstate.kb_index[h_state] = replace(
                existing,
                labels=(*existing.labels, commitment),
            )
            lstate.state_hash_merges += 1
            return True
        lstate.kb_index[h_state] = SetNode(
            state_hash=h_state,
            canonical_set=commitment,
            labels=(commitment,),
            verdict=verdict_label,
            layer=layer,
        )
        return False
    # entry == "gaps"
    lstate.kb_index[hash(commitment)] = SetNode(
        state_hash=h_state,
        canonical_set=commitment,
        labels=(commitment,),
        verdict=verdict_label,
        layer=layer,
    )
    return False


@dataclass
class _LoopCtx:
    """Shared state threaded through the :func:`_explore_layers` phase
    functions + helpers — lets them live at module level instead of as
    closures. The mutable populations (``stats``, ``lstate``, ``root_kb``,
    ``dumper``) are the same objects throughout, so a mutation in one phase is
    visible to the next; ``alive`` / ``a_prev`` carry the Phase-1 → Phase-2
    handoff."""
    root_kb: KnowledgeBase
    cfg: SolverConfig
    stats: MonotonicStats
    lstate: _LatticeLoopState
    dumper: MonotonicDumper | LatticeDumper | None
    store_lattice: bool
    t_start: float
    max_time: float | None
    max_enterings: int | None
    max_set_size: int
    stop_after: int | None
    shuffle_rng: random.Random | None
    alive: frozenset[FactId] = frozenset()
    a_prev: list[CanonicalSetId] = field(default_factory=list)


def _record_node(
    ctx: _LoopCtx,
    node_kb: KnowledgeBase,
    commitment: CanonicalSetId,
    firings: tuple = (),
    layer: int = 0,
) -> None:
    """Record a solution node (consistent ∧ complete), deduped by
    :func:`state_hash`. Stores a :meth:`snapshot` so it survives later root
    mutation. On a state_hash collision (the same model reached via two
    commitment paths — e.g. the two orientations of a symmetric pair) the
    lex-smallest commitment wins, so the recorded representative is
    deterministic regardless of (shuffled) traversal order — the proof's
    solution set, and any render of it, are shuffle-invariant (S1.5b.31)."""
    h = state_hash(node_kb)
    cur = ctx.lstate.solution_nodes.get(h)
    if cur is None or tuple(sorted(commitment)) < tuple(sorted(cur.commitment)):
        ctx.lstate.solution_nodes[h] = SolutionRecord(
            commitment=commitment,
            kb=node_kb.snapshot(),
            firings=tuple(firings),
            layer=layer,
        )


def _root_dead(ctx: _LoopCtx) -> None:
    """Record a root-level contradiction's unsat core (commitment ())."""
    core = _source_frontier_core(ctx.root_kb)
    ctx.lstate.dead_commitments.append(DeadCommitment(
        commitment=(), unsat_core=core,
        learned_clause=frozenset(), layer=0, kind="dead-post",
        state_hash=state_hash(ctx.root_kb),
    ))


def _solve_proof(ctx: _LoopCtx) -> LatticeProof:
    """Package the loop state into a sound :class:`LatticeProof`.

    Everything here is already collected by the solve loop: the deduped
    solution nodes (``lstate.solution_nodes``), the refuted commitments with
    their per-commitment unsat cores (``lstate.dead_commitments``), and the
    size-N frontier left alive at the depth cap. ``kb_index`` is left empty —
    ``render_lattice`` falls back to the solution-frontier view when no
    SetNode storage is present (the DAG's ``full`` view is a bench-only
    nicety; the sound data is the solutions + deads)."""
    solutions = tuple(ctx.lstate.solution_nodes.values())
    return LatticeProof(
        solutions=solutions,
        dead_commitments=tuple(ctx.lstate.dead_commitments),
        kb_index={},
        alive_at_end=ctx.lstate.alive_at_end_tuple,
        learned_nogoods=frozenset(ctx.root_kb._nogoods),
        stats=_build_lattice_stats(
            ctx.stats,
            solutions_found=len(solutions),
            state_hash_merges=ctx.lstate.state_hash_merges,
            elapsed_seconds=time.perf_counter() - ctx.t_start,
        ),
    )


def _finalise_solve(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats]:
    """Build the solve verdict from the deduped solution-node set.

    The verdict *type* is read from the count ``k`` (:func:`verdict_of`):
    ``k=0`` Contradiction, ``k=1`` Solution, ``k>1`` Ambiguity — three
    answers to one problem. With ``store_lattice`` the sound proof (solution
    set + refutation map) rides along for the trace / lattice-DAG views."""
    ctx.stats.solution_nodes = len(ctx.lstate.solution_nodes)
    ctx.stats.exhausted = not ctx.lstate.truncated
    verdict = verdict_of(ctx.lstate, exhausted=ctx.stats.exhausted)
    if ctx.store_lattice:
        verdict = replace(verdict, proof=_solve_proof(ctx))
    return _finish(ctx.dumper, verdict, ctx.stats)


def _check_budget(ctx: _LoopCtx) -> None:
    """Raise :class:`BudgetExceededError` if the entering-count or wall-time
    budget is spent (flushing the dumper first)."""
    reason: str | None = None
    if (
        ctx.max_enterings is not None
        and ctx.stats.enterings_total >= ctx.max_enterings
    ):
        reason = f"max-enterings ({ctx.max_enterings}) reached"
    elif (
        ctx.max_time is not None
        and (time.perf_counter() - ctx.t_start) > ctx.max_time
    ):
        reason = f"max-time ({ctx.max_time}s) exceeded"
    if reason is None:
        return
    # S1.9.E17.2 — the abort raises before `_finalise_solve` runs, so record
    # not-exhausted here; otherwise the partial stats keep the default `True`
    # and an Aborted run would look fully explored.
    ctx.stats.exhausted = False
    if ctx.dumper is not None:
        ctx.dumper.close()
    raise BudgetExceededError(reason, ctx.stats)


def _handle_dead(
    ctx: _LoopCtx, c: CanonicalSetId, layer: int, result,
) -> None:
    """Record a dead commitment: count it, emit its nogood (+ size-1
    ``(not h)`` writeback), append a :class:`DeadCommitment` (the ``k=0``
    verdict's core is the union of these), and log it."""
    if result.kind == "dead-pre":
        ctx.stats.enterings_dead_pre += 1
    else:
        ctx.stats.enterings_dead_post += 1

    landed = emit_nogood(ctx.root_kb, frozenset(c), min_size=1)
    if landed:
        ctx.stats.nogoods_emitted += 1
    else:
        ctx.stats.nogoods_subsumed += 1
    if len(c) == 1:
        _emit_negated_fact_writeback(ctx.root_kb, c[0])

    ctx.lstate.dead_commitments.append(DeadCommitment(
        commitment=c,
        unsat_core=result.unsat_core,
        learned_clause=frozenset(c),
        layer=layer,
        kind=result.kind,
        state_hash=state_hash(result.kb),
    ))

    if ctx.dumper is not None:
        ctx.dumper.entering(
            layer, c, result,
            outcome=result.kind,  # "dead-pre" / "dead-post"
            facts_merged=0,
            nogood_emitted=landed,
            nogood_subsumed=not landed,
        )


#: Cadence (in firings) for the live root-saturation progress line under `-v`.
#: Fixed, not tied to `--progress-every` (which paces enterings) so `-g1`
#: can't flood with a line per firing.
_ROOT_SAT_PROGRESS_EVERY = 50


def _phase1_root(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats] | None:
    """Phase 1 — saturate the root and handle the terminal root states
    (contradictory / trivially-solved / forced-positive cascade /
    fully-determined). On fall-through, stash the open set + layer-1 frontier
    on ``ctx`` for Phase 2 and return None; otherwise return the verdict."""
    root_kb, stats = ctx.root_kb, ctx.stats
    dumper = ctx.dumper

    sat = Saturator(root_kb)
    # Root saturation is the slow part of a Phase-1 solve (a puzzle may resolve
    # entirely here, with enterings=0). Stream firing-count progress so `-v`
    # isn't silent meanwhile — but only when a dumper is attached; the common
    # no-dumper path keeps the C-speed `list()` drain (saturation is hot).
    if dumper is None:
        _ = list(sat.saturate())
    else:
        n_fired = 0
        for _fired in sat.saturate():
            n_fired += 1
            if n_fired % _ROOT_SAT_PROGRESS_EVERY == 0:
                dumper.root_saturating(n_fired)
    stats.saturate_count += 1
    if ctx.cfg.warn_derived_naf:
        # S1.7.4 — once-per-solve, post-saturation so the cache holds
        # the plans of rules with rule-derived activators (adjacent-via-*,
        # the elimination rules). Reuse this saturator's fully-populated
        # engine cache rather than recompiling.
        from ein.inference import naf_deps
        naf_deps.emit_derived_naf_warnings(sat.engine.cache)
    if dumper is not None:
        dumper.root_initial(root_kb)
    if ContradictionDetector(root_kb).detect():
        # root contradictory before any commitment (e.g. zebra2-bad: injected
        # fact clashes with (6) during root saturation) → k=0, with the
        # source-frontier core.
        _root_dead(ctx)
        return _finalise_solve(ctx)

    alive = _compute_alive(root_kb)
    alive, term = _promote_forced_positives(
        root_kb, alive, stats, check_goal=False,
    )
    if term is not None:
        # solve never goal-terminates the cascade (check_goal=False), so a
        # term here is a Contradiction (cascade hit ⊥) → k=0.
        _root_dead(ctx)
        return _finalise_solve(ctx)
    if not alive:
        # empty alive + consistent (no contradiction above) ⇒ root is itself a
        # complete, consistent model — the unique solution.
        _record_node(ctx, root_kb, ())
        return _finalise_solve(ctx)

    ctx.alive = alive
    ctx.a_prev = layer_1(alive)
    return None


def _phase3_verdict(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats]:
    """Phase 3 — synthesise the verdict from the accumulated ``ctx.lstate``:
    P1.7a reads the type from the deduped solution-node count ``k``."""
    return _finalise_solve(ctx)


def _phase2_layers(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats] | None:
    """Phase 2 — explore the commitment lattice layer by layer. Returns the
    ``stop_after`` early-stop verdict (solve) or None (exhausted → Phase 3).
    Reads the Phase-1 handoff (``ctx.alive`` / ``ctx.a_prev``)."""
    root_kb, stats, lstate = ctx.root_kb, ctx.stats, ctx.lstate
    dumper, cfg = ctx.dumper, ctx.cfg
    max_set_size = ctx.max_set_size
    shuffle_rng, stop_after = ctx.shuffle_rng, ctx.stop_after
    alive, a_prev = ctx.alive, ctx.a_prev

    phase_2_done = False
    for layer in range(1, max_set_size + 1):
        if phase_2_done:
            break
        stats.layers_explored = layer
        if dumper is not None:
            dumper.layer_start(layer, root_kb, len(alive))

        if layer == 1:
            candidates = list(a_prev)
        else:
            candidates = generate_layer(
                a_prev,
                alive=alive,
                nogoods=root_kb._nogoods,
            )

        # S1.5b.26 — within-layer scoring switch. cfg.lattice_order
        # default 'lex' falls through to canonical-tuple sort
        # (the previous inline candidates.sort() behaviour);
        # 'score-sum' descends by sum of per-element
        # score_hypothesis, lex tiebreak. order_candidates is
        # pure — no side effects on root_kb or stats.
        candidates = order_candidates(
            candidates, mode=cfg.lattice_order, kb=root_kb,
        )
        # S1.5b.31 — optional per-layer shuffle for the
        # shuffle-invariance harness. ``shuffle_rng`` carries
        # state across layers when seeded, so different layers
        # get different permutations of their candidates list.
        if shuffle_rng is not None:
            shuffle_rng.shuffle(candidates)

        a_layer: list[CanonicalSetId] = []
        for c in candidates:
            _check_budget(ctx)
            stats.enterings_total += 1
            result = try_commitment_set(root_kb, c)

            if result.kind in ("dead-pre", "dead-post"):
                _handle_dead(ctx, c, layer, result)
                continue

            # Alive.
            stats.enterings_alive += 1
            # P1.7a "solve": a "solved" fork is a *solution node*
            # (consistent ∧ complete), NOT a goal-pattern match —
            # this is what excludes the partial dead-end and what
            # forces incomplete goal-matchers to keep expanding.
            # F-ENG-12: consistency is already established on this ALIVE
            # branch (try_commitment_set returns kind="alive" only after
            # its post-saturation ContradictionDetector.detect() came
            # back empty, and result.kb is that unmutated fork), so check
            # completeness directly — is_solution_node would re-run a
            # full detect() on a kb already proved consistent.
            solved = complete(result.kb)

            # S1.5b.27 — saturation-commutativity sanity check.
            # Off by default; cfg.lattice_sanity_check triggers the
            # release regression that verifies every (k-1)-subset
            # parent path produces the same post-saturation kb as
            # the direct commitment. Runs orthogonally to
            # store_lattice / entry — the premise applies to every
            # alive commitment regardless of the dumper or proof
            # surface. Skipped for singletons (no parents).
            if cfg.lattice_sanity_check and len(c) >= 2:
                from ein.inference.monotonic.sanity import (
                    check_commutativity,
                )
                check_commutativity(root_kb, c)

            # Fork-side solution node (§3c.ii of algorithm_layer_n.md).
            if solved:
                # Solution node (consistent ∧ complete): record it (deduped by
                # state_hash); do NOT merge its facts (model-specific) and do
                # NOT expand it (complete — supersets would be redundant or
                # dead).
                _record_node(ctx, result.kb, c, result.firings, layer)
                if dumper is not None:
                    dumper.entering(
                        layer, c, result,
                        outcome="solution",
                        facts_merged=0,
                        nogood_emitted=False,
                        nogood_subsumed=False,
                    )
                if (
                    stop_after is not None
                    and len(lstate.solution_nodes) >= stop_after
                ):
                    lstate.truncated = True
                    return _finalise_solve(ctx)
                continue

            # P1.7a — PURE PER-BRANCH search; keep root STABLE. Do NOT merge
            # unconditional facts and do NOT promote forced-positives into the
            # shared root mid-search:
            #   * unconditional-fact extraction is UNSOUND under NAF
            #     (`absent`) — a fork fact derived via `absent X` looks
            #     unconditional by the provenance walk but actually depends on
            #     the commitment having suppressed X; merging it is wrong;
            #   * cumulative shared-root promotion is the monotonic SAT→⊥
            #     pollution Phase A (S1.7a.1) flagged.
            # Each commitment is evaluated independently against the
            # post-Phase-1 root; nogoods (emitted above on deaths) prune
            # supersets. This incomplete commitment just expands to the next
            # layer.
            if dumper is not None:
                dumper.entering(
                    layer, c, result, outcome="alive",
                    facts_merged=0, nogood_emitted=False,
                    nogood_subsumed=False,
                )
            a_layer.append(c)

        if dumper is not None:
            dumper.layer_end(layer, root_kb, len(alive), len(a_layer))
        if phase_2_done:
            break
        if not a_layer:
            break
        # P1.7a — SOUND inter-layer prune. This layer's size-k deaths wrote
        # ``¬g`` (sound: ``{g}`` is genuinely inconsistent with root).
        # Recompute alive and promote any backbone singletons via the
        # forced-positive cascade (sound: a sole-surviving slot value must
        # hold) — NOT the NAF-unsound unconditional-fact merge. This collapses
        # the candidate space the way the legacy engines prune, without the
        # SAT→⊥ pollution.
        alive = _compute_alive(root_kb)
        alive, term = _promote_forced_positives(
            root_kb, alive, stats, check_goal=False,
        )
        if term is not None:
            _root_dead(ctx)       # cascade hit ⊥ → no model exists
            break
        if not alive:
            _record_node(ctx, root_kb, ())  # backbone determines all cells
            break
        # drop commitments no longer entirely within `alive`
        # (an element got promoted into root or refuted).
        a_layer = [c for c in a_layer if all(e in alive for e in c)]
        if not a_layer:
            break
        a_prev = a_layer
        # Capture the surviving size-N frontier when the depth cap
        # is the natural loop terminator. ``alive_at_end`` stays
        # ``()`` on every early-exit path (contradiction, root
        # solved, frontier exhausted).
        if layer == max_set_size:
            lstate.alive_at_end_tuple = tuple(a_layer)
            # P1.7a: a non-empty frontier at the depth cap means the
            # lattice was NOT fully explored — `k` is a lower bound,
            # so a k=1 result is "a model", not certified unique, and
            # a k=0 is "no model within the cap", not proven unsat.
            if a_layer:
                lstate.truncated = True

    return None


def _explore_layers(
    root_kb: KnowledgeBase,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,
    dumper: MonotonicDumper | LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
    stop_after: int | None = None,
) -> tuple[Verdict, MonotonicStats]:
    """The single ``solve`` per-candidate loop. See
    `plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md`
    for the per-step contract.

    Exhausts the lattice (or stops after ``stop_after`` distinct solution
    nodes), recording every solution node (``consistent ∧ complete`` — keyed
    by :func:`complete`, not goal match) deduped by :func:`state_hash` into
    ``lstate.solution_nodes``, and every refuted commitment into
    ``lstate.dead_commitments``. Phase 3 reads the verdict from the count
    ``k`` via :func:`verdict_of`. Root is kept stable (no unconditional merge
    mid-search — unsound under NAF). With ``store_lattice`` the verdict
    carries a :class:`LatticeProof` (``verdict.proof``) whose ``stats`` is the
    :class:`LatticeStats` view.
    """
    cfg = config or root_kb.config or SolverConfig()
    root_kb.config = cfg

    # Solve never goal-terminates — its signal is a *solution node*
    # (consistent ∧ complete, via :func:`complete`), not a goal-pattern match;
    # the query ``:goal`` only projects an answer over the model afterwards.
    stats = MonotonicStats()
    t_start = time.perf_counter()
    lstate = _LatticeLoopState()
    # S1.5b.31 — shuffle rng, one per solve when configured.
    # ``random.Random(seed)`` is stateful; calling ``.shuffle``
    # per-layer advances its state so each layer gets a
    # different (but deterministic-given-seed) permutation.
    shuffle_rng: random.Random | None = (
        random.Random(cfg.lattice_order_seed)
        if cfg.lattice_order_seed is not None else None
    )
    ctx = _LoopCtx(
        root_kb=root_kb, cfg=cfg, stats=stats,
        lstate=lstate, dumper=dumper, store_lattice=store_lattice,
        t_start=t_start, max_time=max_time, max_enterings=max_enterings,
        max_set_size=max_set_size, stop_after=stop_after,
        shuffle_rng=shuffle_rng,
    )

    # ── Phase 1 — root saturation + terminal root states ──────
    if (r := _phase1_root(ctx)) is not None:
        return r
    # ── Phase 2 — Layer-by-layer iteration ───────────────────
    if (r := _phase2_layers(ctx)) is not None:
        return r

    # ── Phase 3 — Verdict synthesis ──────────────────────────
    return _phase3_verdict(ctx)
