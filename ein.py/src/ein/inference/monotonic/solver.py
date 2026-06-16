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
from typing import Literal

from ein.inference.apriori import (
    CanonicalSetId,
    generate_layer,
    layer_1,
    order_candidates,
)
from ein.inference.commitment import try_commitment_set
from ein.inference.config import SolverConfig
from ein.inference.contradiction import ContradictionDetector
from ein.inference.monotonic._helpers import (
    _check_budget,
    _compute_alive,
    _finalise_solve,
    _handle_dead,
    _LoopCtx,
    _promote_forced_positives,
    _record_node,
    _record_setnode,
    _root_dead,
)
from ein.inference.monotonic._state import (
    BudgetExceededError,
    MonotonicStats,
    _LatticeLoopState,
    verdict_of,
)
from ein.inference.monotonic.state_dump import (
    LatticeDumper,
    MonotonicDumper,
)
from ein.inference.saturator import Saturator
from ein.inference.solution import complete
from ein.inference.verdict import (
    Aborted,
    Verdict,
)
from ein.kb.store import KnowledgeBase

#: Re-exported so `from ...monotonic.solver import X` keeps working for the
#: __init__ public surface + the lattice tests (the engine internals moved to
#: _state.py / _helpers.py in the <500-line split).
__all__ = [
    "BudgetExceededError",
    "MonotonicStats",
    "_LatticeLoopState",
    "_record_setnode",
    "solve",
    "verdict_of",
]


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
