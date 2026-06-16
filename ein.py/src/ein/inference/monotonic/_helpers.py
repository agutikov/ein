"""Solve-loop helper functions (split out of solver.py).

The `_LoopCtx` shared-state dataclass plus every `_`-helper the phases call:
writeback / forced-positive promotion / alive recompute / node + dead recording
/ budget check / proof packaging / verdict finalisation. Imports leaf data from
`_state.py`; the phases that drive these live in `solver.py`.
"""
from __future__ import annotations

import random
import time
from dataclasses import dataclass, field, fields, replace
from typing import Literal

from ein.inference import primitives
from ein.inference.apriori import (
    CanonicalSetId,
)
from ein.inference.canon import state_hash
from ein.inference.config import SolverConfig
from ein.inference.contradiction import ContradictionDetector
from ein.inference.monotonic._state import (
    BudgetExceededError,
    MonotonicStats,
    _LatticeLoopState,
    _source_frontier_core,
    verdict_of,
)
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
from ein.inference.solution import open_hypotheses
from ein.inference.verdict import (
    Contradiction,
    Mode,
    Solution,
    Verdict,
    is_solved,
)
from ein.kb.entities import Fact, Layer
from ein.kb.provenance import FactId, Provenance
from ein.kb.store import KnowledgeBase


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

