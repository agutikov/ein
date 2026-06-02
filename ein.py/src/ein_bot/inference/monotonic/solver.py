"""Monotonic engine main loop.

Set-indexed BFS over commitment sets. Single ``KnowledgeBase``
instance (root); each layer's candidates come from the Apriori
prefix-join (:mod:`ein_bot.inference.apriori`); each candidate
is entered via the common
:func:`ein_bot.inference.commitment.try_commitment_set` primitive.
Unconditional consequences of an alive commitment merge into
root; the loop terminates on the first goal-satisfying fork
(algorithm_layer_n.md §3d.vii — S1.5b.9), on a root contradiction,
or on layer exhaustion.

Termination conditions
----------------------

- :class:`Solution` at a fork — ``is_solution_node`` (solve) or
  ``is_solved(result.kb, Mode.SOLVE)`` (gaps / contradictions)
  on an alive entering. Returns ``Solution(kb=result.kb)`` so
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
``root_kb._nogoods`` via :func:`ein_bot.inference.nogoods.emit_nogood`
(``min_size=1`` so layer-1 singleton deaths land — Q1.5b.5.c).
Singleton dead clauses additionally write ``(not h)`` to
``root_kb._negated_facts`` via :func:`_emit_negated_fact_writeback`;
mirrors ``back_prop._write_negation`` without the ContextVar
coupling. (S1.7.24 — no symmetric-mirror: the counterpart dies on
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

Three entries (Q1.5b.7 / P1.7a)
-------------------------------

All three public entries — :func:`solve`, :func:`gaps_solve`,
:func:`contradictions_solve` — dispatch through the shared
per-candidate loop via the ``entry`` discriminator; they differ
only in the outcome recording and the Phase-3 verdict synthesis.
"""
from __future__ import annotations

import random
import time
from dataclasses import dataclass, field, replace
from typing import Literal

from ein_bot.inference import primitives
from ein_bot.inference.apriori import (
    CanonicalSetId,
    generate_layer,
    layer_1,
    order_candidates,
)
from ein_bot.inference.canon import state_hash
from ein_bot.inference.commitment import try_commitment_set
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.contradiction import ContradictionDetector
from ein_bot.inference.monotonic.lattice import (
    DeadCommitment,
    LatticeProof,
    LatticeStats,
    SetNode,
    SolutionRecord,
)
from ein_bot.inference.monotonic.state_dump import (
    LatticeDumper,
    MonotonicDumper,
)
from ein_bot.inference.nogoods import emit_nogood
from ein_bot.inference.saturator import Saturator
from ein_bot.inference.solution import is_solution_node, open_hypotheses
from ein_bot.inference.verdict import (
    Ambiguity,
    Contradiction,
    Mode,
    Solution,
    Verdict,
    is_solved,
)
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import FactId, Provenance
from ein_bot.kb.store import KnowledgeBase


class BudgetExceededError(RuntimeError):
    """Raised by :func:`solve` / :func:`gaps_solve` /
    :func:`contradictions_solve` when ``max_time`` or
    ``max_enterings`` is hit before the solve completes.

    Carries the partial :class:`MonotonicStats` so callers can
    print the work done before the abort.
    """

    def __init__(self, reason: str, stats: MonotonicStats) -> None:
        super().__init__(reason)
        self.reason = reason
        self.stats = stats


@dataclass
class MonotonicStats:
    """Cumulative counters for one set-search run (:func:`solve` /
    :func:`gaps_solve` / :func:`contradictions_solve`)."""

    enterings_total:     int = 0
    enterings_alive:     int = 0
    enterings_dead_pre:  int = 0
    enterings_dead_post: int = 0
    facts_merged:        int = 0
    forced_positives:    int = 0
    saturate_count:      int = 0
    layers_explored:     int = 0
    # S1.5b.6 — CDCL counters.
    nogoods_emitted:     int = 0
    nogoods_subsumed:    int = 0
    # P1.7a — solve entry: deduped solution-node count `k` and whether
    # the search was exhaustive (else `k` is a lower bound and a k=1
    # result is "a solution", not a proven-unique one).
    solution_nodes:      int = 0
    exhausted:           bool = True


@dataclass
class _LatticeLoopState:
    """Mutable accumulator threaded through :func:`_explore_layers`
    for ``entry in ("gaps", "contradictions")``.

    Bundles every lattice-specific local that the per-candidate
    loop mutates so the dozen return sites in the outer function
    can hand a single bag to :func:`_finalise_lattice_verdict`.

    Field semantics:

    - :attr:`solutions` — every satisfying commitment encountered
      under gaps (each carries a :meth:`KnowledgeBase.snapshot`
      of the fork's saturated kb so root-side mutation after
      return doesn't corrupt the branch view).
    - :attr:`dead_commitments` — every refuted commitment under
      contradictions (S1.5b.23 wires the contradictions writes;
      gaps leaves this empty by contract).
    - :attr:`kb_index` — per-SetNode storage. Empty when
      ``store_lattice=False``. Key encoding differs by entry:
      under gaps it is :func:`hash` of the canonical commitment
      tuple (so distinct commitments stay separate per the GAPS
      contract); under contradictions it is
      :func:`state_hash` of the post-saturation kb (so distinct
      commitments collapse on state-hash collision).
    - :attr:`alive_at_end_tuple` — the size-N surviving
      commitments captured at the last Phase 2 layer iff the
      depth cap was reached; ``()`` otherwise.
    - :attr:`state_hash_merges` — counter ticked whenever the
      contradictions-side dedup folds a fresh commitment into an
      existing :class:`SetNode`.
    - :attr:`root_was_solved` — guard against double-recording
      the root-side solution in gaps Phase 2.
    """

    solutions:         list[SolutionRecord] = field(default_factory=list)
    dead_commitments:  list[DeadCommitment] = field(default_factory=list)
    kb_index:          dict[int, SetNode] = field(default_factory=dict)
    alive_at_end_tuple: tuple[CanonicalSetId, ...] = ()
    state_hash_merges: int = 0
    root_was_solved:   bool = False
    # P1.7a — solve entry: deduped solution nodes (state_hash → record)
    # and whether the search ran out of lattice (exhausted) or was cut
    # short by ``stop_after`` / the depth cap (``truncated``).
    solution_nodes:    dict[int, SolutionRecord] = field(default_factory=dict)
    truncated:         bool = False


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
    cores: frozenset[Fact] = frozenset()
    for d in lstate.dead_commitments:
        cores = cores | d.unsat_core
    return Contradiction(unsat_core=cores)


def solve(
    root_kb: KnowledgeBase,
    *,
    stop_after: int | None = None,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    dumper: MonotonicDumper | LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Verdict, MonotonicStats]:
    """P1.7a sound search — the one entry whose verdict is *read* from the
    result rather than chosen up front.

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
    """
    return _explore_layers(
        root_kb,
        entry="solve",
        stop_after=stop_after,
        max_set_size=max_set_size,
        config=config,
        dumper=dumper,
        max_time=max_time,
        max_enterings=max_enterings,
    )


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

    Minimal equivalent of
    :func:`ein_bot.inference.back_prop._write_negation` —
    same shape, no :data:`_kb_chain_ctx` / :data:`_eager_pass_ctx`
    coupling (the monotonic loop has no chain and never operates
    under eager mode). Idempotent: a pre-existing ``(not h)`` at
    root is left untouched.
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
    been refuted (back-prop wrote ``(not h_other)`` at root, or
    hypgen filtered it). Combined with the puzzle's slot
    exclusivity constraint, ``h`` must be true. The
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

    Delegates to :func:`ein_bot.inference.solution.open_hypotheses`
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
    contras = ContradictionDetector(kb).detect()
    core = (
        frozenset(kb.unsat_core(c.witness for c in contras))
        if contras else frozenset()
    )
    return Contradiction(unsat_core=core)


# ── Shared core loop — _explore_layers ───────────────────────
#
# S1.5b.21: extracted from the pre-refactor monotonic-loop
# body. The `entry` discriminator dispatches outcomes for the
# three public functions (solve, gaps_solve,
# contradictions_solve). The shared core is NEVER DUPLICATED
# across the three entries — they're all thin wrappers.
#
# New behaviour for `entry="gaps"`: instead of early-terminate
# on is_solved, record into a local `solutions` list and
# continue the search. Phase 3 synthesises
# `Ambiguity(branches=[Solution for each recorded])`. The
# `entry="contradictions"` branch is reserved for S1.5b.23.


def _build_lattice_stats(
    mstats: MonotonicStats,
    *,
    solutions_found: int,
    state_hash_merges: int,
    elapsed_seconds: float,
) -> LatticeStats:
    """Project a :class:`MonotonicStats` plus the lattice-only
    counters into the public :class:`LatticeStats` shape.

    The shared counters are copied field-for-field; the
    lattice-only triple
    (``solutions_found`` / ``state_hash_merges`` / ``elapsed_seconds``)
    is supplied by the caller. Keeping :class:`MonotonicStats`
    free of the lattice counters preserves the :func:`solve`
    public surface unchanged.
    """
    return LatticeStats(
        enterings_total=mstats.enterings_total,
        enterings_alive=mstats.enterings_alive,
        enterings_dead_pre=mstats.enterings_dead_pre,
        enterings_dead_post=mstats.enterings_dead_post,
        solutions_found=solutions_found,
        facts_merged=mstats.facts_merged,
        forced_positives=mstats.forced_positives,
        saturate_count=mstats.saturate_count,
        layers_explored=mstats.layers_explored,
        nogoods_emitted=mstats.nogoods_emitted,
        nogoods_subsumed=mstats.nogoods_subsumed,
        state_hash_merges=state_hash_merges,
        elapsed_seconds=elapsed_seconds,
    )


def _dedup_solutions_by_state(
    records: list[SolutionRecord],
) -> tuple[SolutionRecord, ...]:
    """Collapse solution records that saturate to the same KB state, in
    deterministic order. S1.7.24 — recovers correct model counting
    generically (no symmetric-awareness): the two orientations of a
    symmetric pair share a ``state_hash`` and count once. The kept
    representative is the one with the lexicographically smallest
    ``commitment`` (deterministic regardless of candidate order, so the
    branch list is shuffle-invariant); records are returned in
    ``state_hash`` order."""
    best: dict[int, SolutionRecord] = {}
    for r in records:
        h = state_hash(r.kb)
        cur = best.get(h)
        if cur is None or tuple(sorted(r.commitment)) < tuple(sorted(cur.commitment)):
            best[h] = r
    return tuple(best[h] for h in sorted(best))


def _finalise_lattice_verdict(
    entry: Literal["gaps", "contradictions"],
    *,
    root_kb: KnowledgeBase,
    lstate: _LatticeLoopState,
    stats: MonotonicStats,
    elapsed_seconds: float,
    store_lattice: bool,
) -> Verdict:
    """Build the :class:`LatticeProof` from accumulated loop state
    and wrap it in the per-entry verdict shape.

    Under ``entry == "gaps"`` returns :class:`Ambiguity` whose
    ``branches`` enumerate the satisfying snapshots; under
    ``entry == "contradictions"`` returns :class:`Contradiction`
    whose ``unsat_core`` unions every recorded dead's core.
    ``kb_index`` is materialised on the proof only when
    ``store_lattice`` is set (when off, the proof carries an
    empty dict to keep the field's type stable).
    """
    # S1.7.24 — dedup recorded solutions by post-saturation `state_hash`
    # (the generic signal SOLVE's `solution_nodes` dict already keys on).
    # A model reached by two commitments — e.g. the two orientations
    # `{(R a b)}` / `{(R b a)}` of a symmetric relation, which the user's
    # `(rule symmetric)` saturates to the SAME KB — is ONE model. Distinct
    # models keep distinct hashes (no-op for non-colliding gaps); the
    # canonical (min-commitment) representative + state_hash ordering make
    # the branch list deterministic, so the kernel needs no symmetric
    # canonicalisation to count an undecided symmetric pair once.
    solutions = _dedup_solutions_by_state(lstate.solutions)
    lattice_stats = _build_lattice_stats(
        stats,
        solutions_found=len(solutions),
        state_hash_merges=lstate.state_hash_merges,
        elapsed_seconds=elapsed_seconds,
    )
    proof = LatticeProof(
        solutions=solutions,
        dead_commitments=tuple(lstate.dead_commitments),
        kb_index=dict(lstate.kb_index) if store_lattice else {},
        alive_at_end=lstate.alive_at_end_tuple,
        learned_nogoods=frozenset(root_kb._nogoods),
        stats=lattice_stats,
    )
    if entry == "gaps":
        branches = tuple(
            Solution(kb=s.kb, trace=s.firings)
            for s in solutions
        )
        return Ambiguity(
            branches=branches, proof=proof,
        )
    # entry == "contradictions"
    cores: frozenset[Fact] = frozenset()
    for d in lstate.dead_commitments:
        cores = cores | d.unsat_core
    return Contradiction(unsat_core=cores, proof=proof)


def _record_setnode(
    lstate: _LatticeLoopState,
    *,
    entry: Literal["gaps", "contradictions"],
    commitment: CanonicalSetId,
    result_kb: KnowledgeBase,
    verdict_label: Literal["alive", "dead", "solution"],
    layer: int,
) -> bool:
    """Insert or merge a :class:`SetNode` into ``lstate.kb_index``.

    Under ``entry == "contradictions"`` the dict is keyed by the
    post-saturation :func:`state_hash`; on collision the existing
    node's ``labels`` tuple grows and ``state_hash_merges`` ticks.
    Returns ``True`` if the call merged (caller may then skip
    downstream handling — the prior arrival already did the
    flat root-writes for this kb-state).

    Under ``entry == "gaps"`` the dict is keyed by
    ``hash(commitment)`` so distinct commitments stay separate
    even when their post-saturation kbs collide
    (:func:`gaps_solve` contract — multi-solution enumeration
    requires every satisfying commitment to register on its own
    node). The ``SetNode.state_hash`` field is still the
    post-saturation hash.
    """
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
    entry: Literal["gaps", "contradictions", "solve"]
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


def _finalise_gaps(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats]:
    """Build the gaps-side Ambiguity verdict from ``ctx.lstate`` + emit the
    dumper summary via :func:`_finish`."""
    verdict = _finalise_lattice_verdict(
        "gaps",
        root_kb=ctx.root_kb,
        lstate=ctx.lstate,
        stats=ctx.stats,
        elapsed_seconds=time.perf_counter() - ctx.t_start,
        store_lattice=ctx.store_lattice,
    )
    return _finish(ctx.dumper, verdict, ctx.stats)


def _finalise_contradictions(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats]:
    """Build the contradictions-side Contradiction verdict (``unsat_core``
    = set union over every recorded dead's ``unsat_core``)."""
    verdict = _finalise_lattice_verdict(
        "contradictions",
        root_kb=ctx.root_kb,
        lstate=ctx.lstate,
        stats=ctx.stats,
        elapsed_seconds=time.perf_counter() - ctx.t_start,
        store_lattice=ctx.store_lattice,
    )
    return _finish(ctx.dumper, verdict, ctx.stats)


def _record_node(
    ctx: _LoopCtx,
    node_kb: KnowledgeBase,
    commitment: CanonicalSetId,
    firings: tuple = (),
    layer: int = 0,
) -> None:
    """Record a solution node (consistent ∧ complete), deduped by
    :func:`state_hash`. Stores a :meth:`snapshot` so it survives later root
    mutation."""
    h = state_hash(node_kb)
    if h not in ctx.lstate.solution_nodes:
        ctx.lstate.solution_nodes[h] = SolutionRecord(
            commitment=commitment,
            kb=node_kb.snapshot(),
            firings=tuple(firings),
            layer=layer,
        )


def _root_dead(ctx: _LoopCtx) -> None:
    """Record a root-level contradiction's unsat core (commitment ())."""
    contras = ContradictionDetector(ctx.root_kb).detect()
    core = frozenset(
        ctx.root_kb.unsat_core(c.witness for c in contras)
    ) if contras else frozenset()
    ctx.lstate.dead_commitments.append(DeadCommitment(
        commitment=(), unsat_core=core,
        learned_clause=frozenset(), layer=0, kind="dead-post",
        state_hash=state_hash(ctx.root_kb),
    ))


def _finalise_solve(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats]:
    """Build the solve verdict from the deduped solution-node set."""
    ctx.stats.solution_nodes = len(ctx.lstate.solution_nodes)
    ctx.stats.exhausted = not ctx.lstate.truncated
    verdict = verdict_of(ctx.lstate, exhausted=ctx.stats.exhausted)
    return _finish(ctx.dumper, verdict, ctx.stats)


def _check_budget(ctx: _LoopCtx) -> None:
    """Raise :class:`BudgetExceededError` if the entering-count or wall-time
    budget is spent (flushing the dumper first)."""
    if (
        ctx.max_enterings is not None
        and ctx.stats.enterings_total >= ctx.max_enterings
    ):
        if ctx.dumper is not None:
            ctx.dumper.close()
        raise BudgetExceededError(
            f"max-enterings ({ctx.max_enterings}) reached", ctx.stats,
        )
    if (
        ctx.max_time is not None
        and (time.perf_counter() - ctx.t_start) > ctx.max_time
    ):
        if ctx.dumper is not None:
            ctx.dumper.close()
        raise BudgetExceededError(
            f"max-time ({ctx.max_time}s) exceeded", ctx.stats,
        )


def _handle_dead(
    ctx: _LoopCtx, c: CanonicalSetId, layer: int, result,
) -> None:
    """Record a dead commitment: count it, emit its nogood (+ size-1
    ``(not h)`` writeback), append a :class:`DeadCommitment` for the
    contradictions / solve entries, and log it. Under ``store_lattice`` a
    contradictions-side state-hash merge may absorb it — then the downstream
    writes are skipped (the earlier arrival already did them)."""
    if result.kind == "dead-pre":
        ctx.stats.enterings_dead_pre += 1
    else:
        ctx.stats.enterings_dead_post += 1

    # S1.5b.22 state-hash dedup (contradictions) / SetNode storage (gaps).
    # dead-post only — dead-pre's kb is the unsaturated fork.
    skip_downstream = False
    if (
        ctx.store_lattice
        and result.kind == "dead-post"
        and ctx.entry in ("gaps", "contradictions")
    ):
        skip_downstream = _record_setnode(
            ctx.lstate,
            entry=ctx.entry,  # type: ignore[arg-type]
            commitment=c,
            result_kb=result.kb,
            verdict_label="dead",
            layer=layer,
        )
    if skip_downstream:
        return

    landed = emit_nogood(ctx.root_kb, frozenset(c), min_size=1)
    if landed:
        ctx.stats.nogoods_emitted += 1
    else:
        ctx.stats.nogoods_subsumed += 1
    if len(c) == 1:
        _emit_negated_fact_writeback(ctx.root_kb, c[0])

    # contradictions collects every dead; solve collects them too — the
    # k=0 verdict's core is the union of these.
    if ctx.entry in ("contradictions", "solve"):
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


def _merge_and_recheck(
    ctx: _LoopCtx, c: CanonicalSetId, layer: int, result, solved: bool,
    alive: frozenset[FactId],
) -> tuple[bool, frozenset[FactId]]:
    """Merge an alive commitment's unconditional facts into root,
    re-saturate, and re-check root. Returns ``(stop, alive)`` — ``stop`` is
    True iff Phase 2 should stop (root went contradictory, was solved
    (gaps), or got fully determined); ``alive`` is the recomputed open set
    (unchanged when nothing merged). Only ``gaps`` / ``contradictions``
    reach here; ``solve`` keeps root stable and never calls this."""
    this_merged = 0
    for f in result.unconditional_facts:
        if ctx.root_kb._fact_by_id(
            f.relation_name, f.args,
        ) is None:
            ctx.root_kb.add_and_index_fact(f)
            ctx.stats.facts_merged += 1
            this_merged += 1

    if ctx.dumper is not None:
        ctx.dumper.entering(
            layer, c, result,
            outcome="solution" if solved else "alive",
            facts_merged=this_merged,
            nogood_emitted=False,
            nogood_subsumed=False,
        )

    if this_merged:
        # Option A cadence (Q1.5b.2.a) — re-saturate + recompute alive
        # after every alive entering.
        _ = list(Saturator(ctx.root_kb).saturate())
        ctx.stats.saturate_count += 1
        # Merged facts could derive a contradiction at root.
        if ContradictionDetector(ctx.root_kb).detect():
            if ctx.entry == "solve":
                # merged invariants contradict at root ⇒ no model; record
                # the core and stop.
                _root_dead(ctx)
            # gaps / contradictions: root contradictory; stop.
            return True, alive
        alive = _compute_alive(ctx.root_kb)
        alive, term = _promote_forced_positives(
            ctx.root_kb, alive, ctx.stats, check_goal=True,
        )
        if term is not None:
            if ctx.entry == "solve":
                # cascade hit ⊥ at root mid-merge → stop.
                _root_dead(ctx)
                return True, alive
            if ctx.entry == "gaps":
                if isinstance(term, Solution):
                    if not ctx.lstate.root_was_solved:
                        ctx.lstate.solutions.append(SolutionRecord(
                            commitment=c,
                            kb=ctx.root_kb.snapshot(),
                            firings=(), layer=layer,
                        ))
                        ctx.lstate.root_was_solved = True
                # gaps: cascade hit a terminal (Solution: root satisfies;
                # Contradiction: root contradictory) — exit Phase 2 either way.
                return True, alive
            # entry == "contradictions":
            if isinstance(term, Contradiction):
                # root contradicted by cascade — stop.
                return True, alive
            # Solution: continue exploring. Supersets extending the
            # cascade's now-solved root may still die.

        if ctx.entry == "solve" and not alive:
            # root became fully determined mid-merge (every hypothesis
            # decided by invariants alone) ⇒ unique model.
            _record_node(ctx, ctx.root_kb, ())
            return True, alive

        if is_solved(ctx.root_kb, Mode.SOLVE):
            if ctx.entry == "gaps":
                # record once (root_was_solved guard) then terminate Phase 2
                # — root satisfies, so remaining candidates would just
                # re-confirm via fork-side is_solved.
                if not ctx.lstate.root_was_solved:
                    ctx.lstate.solutions.append(SolutionRecord(
                        commitment=c, kb=ctx.root_kb.snapshot(),
                        firings=(), layer=layer,
                    ))
                    ctx.lstate.root_was_solved = True
                return True, alive
            # entry == "contradictions": root is_solved is not a stop
            # condition — supersets at higher layers may die.
    return False, alive


def _phase1_root(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats] | None:
    """Phase 1 — saturate the root and handle the terminal root states
    (contradictory / trivially-solved / forced-positive cascade /
    fully-determined). On fall-through, stash the open set + layer-1 frontier
    on ``ctx`` for Phase 2 and return None; otherwise return the verdict."""
    root_kb, stats, lstate = ctx.root_kb, ctx.stats, ctx.lstate
    dumper, entry = ctx.dumper, ctx.entry

    sat = Saturator(root_kb)
    _ = list(sat.saturate())
    stats.saturate_count += 1
    if ctx.cfg.warn_derived_naf:
        # S1.7.4 — once-per-solve, post-saturation so the cache holds
        # the plans of rules with rule-derived activators (adjacent-via-*,
        # the elimination rules). Reuse this saturator's fully-populated
        # engine cache rather than recompiling.
        from ein_bot.inference import naf_deps
        naf_deps.emit_derived_naf_warnings(sat.engine.cache)
    if dumper is not None:
        dumper.root_initial(root_kb)
    if ContradictionDetector(root_kb).detect():
        if entry == "solve":
            # root contradictory before any commitment (e.g. zebra2-bad:
            # injected fact clashes with (6) during root saturation) →
            # k=0, with the source-frontier core.
            _root_dead(ctx)
            return _finalise_solve(ctx)
        if entry == "gaps":
            # zero solutions; Ambiguity with empty branches.
            return _finalise_gaps(ctx)
        # contradictions: root itself is contradictory — empty
        # ``proof.dead_commitments``, ``verdict.unsat_core`` is
        # the empty frozenset (no commitments were tried).
        return _finalise_contradictions(ctx)
    if entry != "solve" and is_solved(root_kb, Mode.SOLVE):
        if entry == "gaps":
            # root satisfies trivially; record with empty commitment
            # carrier + return Ambiguity with 1 branch.
            lstate.solutions.append(SolutionRecord(
                commitment=(), kb=root_kb.snapshot(),
                firings=(), layer=0,
            ))
            return _finalise_gaps(ctx)
        # contradictions: root is_solved before any commitment. No early
        # return — supersets of singleton hypotheses can still surface
        # deads; fall through to Phase 2.

    alive = _compute_alive(root_kb)
    alive, term = _promote_forced_positives(
        root_kb, alive, stats, check_goal=(entry != "solve"),
    )
    if term is not None:
        if entry == "solve":
            # solve never goal-terminates the cascade (check_goal=False),
            # so a term here is a Contradiction (cascade hit ⊥) → k=0.
            _root_dead(ctx)
            return _finalise_solve(ctx)
        if entry == "gaps":
            # if cascade landed Solution, that's one branch; if
            # Contradiction, zero branches (Ambiguity with empty).
            if isinstance(term, Solution):
                lstate.solutions.append(SolutionRecord(
                    commitment=(), kb=root_kb.snapshot(),
                    firings=(), layer=0,
                ))
            return _finalise_gaps(ctx)
        # contradictions:
        if isinstance(term, Contradiction):
            # root contradicted by cascade — no deads collected.
            return _finalise_contradictions(ctx)
        # Solution: root satisfies after a forced-positive cascade. Don't
        # short-circuit — supersets may still die. Fall through to Phase 2.
    if not alive:
        if entry == "solve":
            # empty alive + consistent (no contradiction above) ⇒ root is
            # itself a complete, consistent model — the unique solution.
            _record_node(ctx, root_kb, ())
            return _finalise_solve(ctx)
        if entry == "gaps":
            return _finalise_gaps(ctx)
        # contradictions: empty alive + no Phase-2 work to do.
        return _finalise_contradictions(ctx)

    ctx.alive = alive
    ctx.a_prev = layer_1(alive)
    return None


def _phase3_verdict(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats]:
    """Phase 3 — synthesise the entry's verdict from the accumulated
    ``ctx.lstate`` (solve: count → verdict; gaps: Ambiguity; contradictions:
    Contradiction)."""
    if ctx.entry == "solve":
        # P1.7a — verdict read from the deduped solution-node count k.
        return _finalise_solve(ctx)
    if ctx.entry == "gaps":
        # always Ambiguity (mode contract).
        return _finalise_gaps(ctx)
    # entry == "contradictions": always Contradiction (mode contract).
    return _finalise_contradictions(ctx)


def _phase2_layers(ctx: _LoopCtx) -> tuple[Verdict, MonotonicStats] | None:
    """Phase 2 — explore the commitment lattice layer by layer. Returns the
    ``stop_after`` early-stop verdict (solve) or None (exhausted → Phase 3).
    Reads the Phase-1 handoff (``ctx.alive`` / ``ctx.a_prev``)."""
    root_kb, stats, lstate = ctx.root_kb, ctx.stats, ctx.lstate
    dumper, entry, cfg = ctx.dumper, ctx.entry, ctx.cfg
    store_lattice, max_set_size = ctx.store_lattice, ctx.max_set_size
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
            solved = (
                is_solution_node(result.kb) if entry == "solve"
                else is_solved(result.kb, Mode.SOLVE)
            )

            # SetNode recording: hoisted out of the entry-specific
            # branches so each commitment lands in ``kb_index`` at
            # most once per visit, with the correct kb-state
            # verdict label. Skipped for monotonic and for
            # ``store_lattice=False``.
            if store_lattice and entry in ("gaps", "contradictions"):
                _record_setnode(
                    lstate,
                    entry=entry,  # type: ignore[arg-type]
                    commitment=c,
                    result_kb=result.kb,
                    verdict_label="solution" if solved else "alive",
                    layer=layer,
                )

            # S1.5b.27 — saturation-commutativity sanity check.
            # Off by default; cfg.lattice_sanity_check triggers the
            # release regression that verifies every (k-1)-subset
            # parent path produces the same post-saturation kb as
            # the direct commitment. Runs orthogonally to
            # store_lattice / entry — the premise applies to every
            # alive commitment regardless of the dumper or proof
            # surface. Skipped for singletons (no parents).
            if cfg.lattice_sanity_check and len(c) >= 2:
                from ein_bot.inference.monotonic.sanity import (
                    check_commutativity,
                )
                check_commutativity(root_kb, c)

            # Fork-side is_solved (§3c.ii of algorithm_layer_n.md).
            if solved:
                if entry == "solve":
                    # Solution node (consistent ∧ complete): record it
                    # (deduped by state_hash); do NOT merge its facts
                    # (model-specific) and do NOT expand it (complete —
                    # supersets would be redundant or dead).
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
                if entry == "gaps":
                    # record + continue, do NOT add to a_layer
                    # (supersets of a satisfying commitment would
                    # trivially also satisfy — bloating the
                    # solutions list with redundant records).
                    lstate.solutions.append(SolutionRecord(
                        commitment=c, kb=result.kb.snapshot(),
                        firings=result.firings, layer=layer,
                    ))
                    if dumper is not None:
                        dumper.entering(
                            layer, c, result,
                            outcome="solution",
                            facts_merged=0,
                            nogood_emitted=False,
                            nogood_subsumed=False,
                        )
                    continue  # don't merge; don't append to a_layer
                # entry == "contradictions": no solution recording.
                # Fall through to the alive flow so unconditional
                # facts merge into root and ``c`` lands in
                # ``a_layer`` for next-layer pair generation —
                # supersets of a solved commitment can still die
                # under additional hypotheses. The ``entering``
                # call at the merge block below passes
                # ``outcome="solution"`` because the kb is solved.

            if entry == "solve":
                # P1.7a — PURE PER-BRANCH search; keep root STABLE.
                # Do NOT merge unconditional facts and do NOT promote
                # forced-positives into the shared root mid-search:
                #   * unconditional-fact extraction is UNSOUND under NAF
                #     (`absent`) — a fork fact derived via `absent X`
                #     looks unconditional by the provenance walk but
                #     actually depends on the commitment having
                #     suppressed X; merging it into root is wrong;
                #   * cumulative shared-root promotion is the monotonic
                #     SAT→⊥ pollution Phase A (S1.7a.1) flagged.
                # Each commitment is evaluated independently against the
                # post-Phase-1 root; nogoods (emitted above on deaths)
                # prune supersets. This incomplete commitment just
                # expands to the next layer.
                if dumper is not None:
                    dumper.entering(
                        layer, c, result, outcome="alive",
                        facts_merged=0, nogood_emitted=False,
                        nogood_subsumed=False,
                    )
                a_layer.append(c)
                continue

            stop, alive = _merge_and_recheck(
                ctx, c, layer, result, solved, alive,
            )
            if stop:
                phase_2_done = True
                break
            a_layer.append(c)

        if dumper is not None:
            dumper.layer_end(layer, root_kb, len(alive), len(a_layer))
        if phase_2_done:
            break
        if not a_layer:
            break
        if entry == "solve":
            # P1.7a — SOUND inter-layer prune. This layer's size-k
            # deaths wrote ``¬g`` (sound: ``{g}`` is genuinely
            # inconsistent with root). Recompute alive and promote any
            # backbone singletons via the forced-positive cascade
            # (sound: a sole-surviving slot value must hold) — NOT the
            # NAF-unsound unconditional-fact merge skipped per commitment
            # above. This collapses the candidate space the way the
            # legacy engines prune, without the SAT→⊥ pollution.
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
    entry: Literal["gaps", "contradictions", "solve"],
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,
    dumper: MonotonicDumper | LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
    stop_after: int | None = None,
) -> tuple[Verdict, MonotonicStats]:
    """The shared per-candidate loop. See
    `plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md`
    for the per-step contract.

    ``entry`` discriminator picks the outcome dispatch + Phase 3
    verdict synthesis:

    - ``"solve"`` (P1.7a) — exhaust the lattice (or stop after
      ``stop_after`` distinct solution nodes), recording every
      solution node (``consistent ∧ complete`` — keyed by
      :func:`is_solution_node`, not goal match) deduped by
      :func:`state_hash` into ``lstate.solution_nodes``; Phase 3
      reads the verdict from the count ``k`` via
      :func:`verdict_of`. Keeps root stable (no unconditional
      merge mid-search — unsound under NAF). Ignores
      ``store_lattice``.
    - ``"gaps"`` — record every goal-sat (fork or root) into
      ``lstate.solutions`` as :class:`SolutionRecord` (kb is a
      :meth:`KnowledgeBase.snapshot` for isolation), do NOT add
      satisfying commitments to ``a_layer`` so supersets aren't
      generated; Phase 3 returns Ambiguity(branches=…). Once
      root itself satisfies, Phase 2 terminates (further
      exploration is redundant under monotone semantics). When
      ``store_lattice=True`` every visited non-``dead-pre``
      commitment also lands in ``lstate.kb_index`` (keyed by
      ``hash(commitment)`` — distinct commitments stay
      separate).
    - ``"contradictions"`` (S1.5b.23) — every dead commitment is
      recorded into ``lstate.dead_commitments`` as a
      :class:`DeadCommitment`. Fork-side ``is_solved`` does NOT
      short-circuit (we fall through to the alive flow, merge
      unconditional facts, and add the commitment to
      ``a_layer`` so its supersets are explored — supersets of
      a solved commitment can still die under additional
      hypotheses). Root-side ``is_solved`` and cascade-Solution
      similarly do not terminate Phase 2 under contradictions.
      Root contradictions and cascade-Contradictions DO
      terminate (root is dead — no more commitments can land).
      Under ``store_lattice=True`` the state-hash dedup MERGE
      is active: distinct dead commitments saturating to the
      same kb collapse into one multilabel SetNode and skip
      downstream root-writes / DeadCommitment append.

    Stats: :class:`MonotonicStats` is the internal counter type;
    :func:`gaps_solve` / :func:`contradictions_solve` promote to
    :class:`LatticeStats` at the public boundary by reading
    ``verdict.proof.stats``.
    """
    cfg = config or root_kb.config or SolverConfig()
    root_kb.config = cfg

    # Goal satisfaction is checked with SOLVE-mode semantics, gated per
    # entry: solve never goal-terminates (its signal is is_solution_node —
    # consistent ∧ complete), so the is_solved checks are guarded by
    # ``entry != "solve"`` / ``check_goal`` at their sites rather than via a
    # neutralised Mode (F-KER-1 — the CONTRADICTIONS-as-always-False hack
    # is gone).
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
        root_kb=root_kb, entry=entry, cfg=cfg, stats=stats,
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


# ── Sibling entries — gaps_solve + contradictions_solve ──────
#
# Per project_set_search_unified memory (2026-05-28): the
# engine is unified — all three public entries
# (solve, gaps_solve, contradictions_solve) live
# side-by-side in this package. They share the per-candidate
# flow from `algorithm_layer_n.md` (Apriori prefix-join +
# try_commitment_set + flat root-writes); the difference is
# whether the loop records solution nodes and reads the verdict
# from their count (solve) or exhausts to collect every
# satisfying / refuted commitment (gaps / contradictions).
# S1.5b.21 lifted the shared core into the private
# `_explore_layers` helper that all three entries call;
# S1.5b.23 filled `contradictions_solve`.


def gaps_solve(
    root_kb: KnowledgeBase,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,
    dumper: LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Ambiguity, LatticeStats]:
    """Run the unified set-search engine under the GAPS contract.

    Exhaustive Apriori-gen — no early termination. Collects every
    satisfying commitment into ``verdict.proof.solutions``;
    returns :class:`Ambiguity` (always; mode contract) whose
    branches enumerate the satisfying kbs (each carrying a
    :meth:`KnowledgeBase.snapshot` so the branch view is stable
    across later root mutations).

    Caller interpretation:
        - ``len(verdict.branches) == 0`` — no solution within
          depth cap.
        - ``len(verdict.branches) == 1`` — uniquely solvable.
        - ``len(verdict.branches) > 1`` — genuine multi-solution.

    ``store_lattice=True`` opts into per-SetNode
    ``verdict.proof.kb_index`` storage; under :func:`gaps_solve`
    the state-hash dedup MERGE step is auto-disabled (distinct
    satisfying commitments must register separately per the
    GAPS contract) — the dict is keyed by ``hash(commitment)`` so
    two distinct commitments with the same post-saturation
    :func:`state_hash` produce two separate :class:`SetNode`
    entries. The :attr:`LatticeStats.state_hash_merges` counter
    is guaranteed to stay zero under :func:`gaps_solve`
    regardless of input.
    """
    verdict, _mstats = _explore_layers(
        root_kb,
        entry="gaps",
        max_set_size=max_set_size,
        config=config,
        store_lattice=store_lattice,
        dumper=dumper,
        max_time=max_time,
        max_enterings=max_enterings,
    )
    # entry="gaps" always returns Ambiguity (per the mode
    # contract) carrying a non-None proof (LatticeProof.stats is
    # the full LatticeStats — built by ``_finalise_gaps``). The
    # MonotonicStats return value is discarded — the lattice
    # counter set is what the public contract advertises.
    assert isinstance(verdict, Ambiguity)
    assert verdict.proof is not None
    return verdict, verdict.proof.stats


def contradictions_solve(
    root_kb: KnowledgeBase,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,
    dumper: LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Contradiction, LatticeStats]:
    """Run the unified set-search engine under the CONTRADICTIONS
    contract.

    Exhaustive Apriori-gen — no early termination on goal
    satisfaction. Collects every dead commitment into
    ``verdict.proof.dead_commitments``; returns
    :class:`Contradiction` (always; mode contract) whose
    ``unsat_core`` is the union of every recorded dead's core.

    Caller interpretation:
        - ``len(verdict.proof.dead_commitments) == 0`` — no
          deaths within depth cap (degenerate; possibly fully
          solvable).
        - non-empty — refutation map. Each
          :class:`DeadCommitment` carries its
          ``unsat_core`` + ``learned_clause`` + ``layer`` +
          ``kind`` ("dead-pre" / "dead-post").

    ``store_lattice=True`` activates state-hash dedup MERGE
    (distinct dead commitments saturating to the same kb
    collapse into one multilabel :class:`SetNode`; the
    ``state_hash_merges`` counter ticks per collision).
    Per the S1.5b.22 spec, on collision the per-commitment
    ``DeadCommitment`` append is also skipped (the prior
    arrival already covered the root-side writes); the
    multilabel SetNode is the authoritative record of the
    other commitments that landed in this kb-state.

    Goal-satisfying commitments under contradictions_solve are
    a no-op for solution-recording purposes (the contract
    doesn't track ``proof.solutions``) but the unconditional
    facts still merge into root via the alive flow, and the
    commitment is added to ``a_layer`` so its supersets are
    explored — supersets of a solved commitment can still die
    under additional hypotheses.
    """
    verdict, _mstats = _explore_layers(
        root_kb,
        entry="contradictions",
        max_set_size=max_set_size,
        config=config,
        store_lattice=store_lattice,
        dumper=dumper,
        max_time=max_time,
        max_enterings=max_enterings,
    )
    # entry="contradictions" always returns Contradiction (per
    # the mode contract) carrying a non-None LatticeProof.
    assert isinstance(verdict, Contradiction)
    assert verdict.proof is not None
    return verdict, verdict.proof.stats
