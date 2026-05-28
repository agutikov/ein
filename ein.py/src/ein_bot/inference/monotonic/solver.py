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

- :class:`Solution` at a fork — ``is_solved(result.kb, mode)``
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
``root_kb._negated_facts`` via :func:`_emit_negated_fact_writeback`
(plus the symmetric-mirror if ``(symmetric R)``); mirrors
``tree/back_prop._write_negation`` without the ContextVar
coupling.

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
at six sites: ``root_initial``, ``layer_start``, ``entering``,
``layer_end``, ``early_terminate``, ``summary``. The single
``_finish`` exit helper guarantees ``summary`` lands on every
non-abort path.

SOLVE mode only (Q1.5b.7)
-------------------------

GAPS and CONTRADICTIONS belong to the lattice engine; the
monotonic loop raises :class:`NotImplementedError` if either
is requested.
"""
from __future__ import annotations

import time
from dataclasses import dataclass

from ein_bot.inference.apriori import (
    CanonicalSetId,
    FactId,
    generate_layer,
    layer_1,
)
from ein_bot.inference.commitment import try_commitment_set
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.contradiction import ContradictionDetector
from ein_bot.inference.hypgen import generate_hypotheses
from ein_bot.inference.monotonic.lattice import LatticeStats
from ein_bot.inference.monotonic.state_dump import (
    LatticeDumper,
    MonotonicDumper,
)
from ein_bot.inference.nogoods import emit_nogood
from ein_bot.inference.saturator import Saturator
from ein_bot.inference.tree.solver import (
    Ambiguity,
    Contradiction,
    Mode,
    Solution,
    Verdict,
    is_solved,
)
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase


class BudgetExceededError(RuntimeError):
    """Raised by :func:`monotonic_solve` when ``max_time`` or
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
    """Cumulative counters for one :func:`monotonic_solve` run."""

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


def monotonic_solve(
    root_kb: KnowledgeBase,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    mode: Mode = Mode.SOLVE,
    dumper: MonotonicDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Verdict, MonotonicStats]:
    """Run the monotonic set-search engine on ``root_kb``.

    Returns ``(verdict, stats)``. SOLVE mode only (Q1.5b.7) —
    GAPS / CONTRADICTIONS belong to the lattice engine.

    The signature deviates from the S1.5b.5 spec (which returned
    just ``Verdict``) — the tuple form gives the bench script
    + tests direct access to the per-run counters without a
    side-channel on ``root_kb``. See stage Ship notes.

    Pass ``dumper`` to capture a per-layer filesystem audit
    (S1.5b.7) — root + per-layer ``.ein`` snapshots, a
    ``00_timeline.jsonl`` event log, and a ``summary.json``.
    ``dumper=None`` (default) makes every hook a no-op.

    ``max_time`` (wall-clock seconds) and ``max_enterings``
    (cumulative count of ``try_commitment_set`` calls) cap the
    work. When either is exceeded the function raises
    :class:`BudgetExceededError` with the partial stats; the
    dumper's ``summary.json`` is **not** emitted on abort (the
    timeline's events up to the abort are sufficient for
    diagnostic). ``None`` (default) disables the respective cap.
    """
    if mode is not Mode.SOLVE:
        raise NotImplementedError(
            "monotonic engine supports SOLVE mode only "
            "(Q1.5b.7); use the lattice engine for GAPS / "
            "CONTRADICTIONS",
        )

    cfg = config or root_kb.config or SolverConfig()
    root_kb.config = cfg

    stats = MonotonicStats()
    t_start = time.perf_counter()

    # ── Phase 1 — Initial saturation + alive ──────────────────
    _ = list(Saturator(root_kb).saturate())
    stats.saturate_count += 1
    if dumper is not None:
        dumper.root_initial(root_kb)
    # Root contradiction (e.g., a rule derived `(false)` directly):
    # spec didn't list this check but without it, a contradictory
    # root falls through to Phase 2 where every entering dead-pre's
    # and the final verdict mis-reports as Ambiguity.
    if ContradictionDetector(root_kb).detect():
        return _finish(dumper, _contradiction(root_kb), stats)
    if is_solved(root_kb, mode):
        return _finish(dumper, _solution(root_kb), stats)

    alive = _compute_alive(root_kb)
    alive, term = _promote_forced_positives(root_kb, alive, stats, mode)
    if term is not None:
        return _finish(dumper, term, stats)
    if not alive:
        return _finish(dumper, _contradiction(root_kb), stats)

    a_prev: list[CanonicalSetId] = layer_1(alive)

    # ── Phase 2 — Layer-by-layer iteration ───────────────────
    for layer in range(1, max_set_size + 1):
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

        candidates.sort()  # lex; scoring switch lands in monotonic followups

        a_layer: list[CanonicalSetId] = []
        for c in candidates:
            # Budget gate — check BEFORE the (potentially slow)
            # try_commitment_set call. Raises with partial stats.
            # The dumper (if any) is closed here so its
            # 00_timeline.jsonl is flushed; no summary.json on abort.
            if (
                max_enterings is not None
                and stats.enterings_total >= max_enterings
            ):
                if dumper is not None:
                    dumper.close()
                raise BudgetExceededError(
                    f"max-enterings ({max_enterings}) reached", stats,
                )
            if (
                max_time is not None
                and (time.perf_counter() - t_start) > max_time
            ):
                if dumper is not None:
                    dumper.close()
                raise BudgetExceededError(
                    f"max-time ({max_time}s) exceeded", stats,
                )

            stats.enterings_total += 1
            result = try_commitment_set(root_kb, c)

            if result.kind in ("dead-pre", "dead-post"):
                if result.kind == "dead-pre":
                    stats.enterings_dead_pre += 1
                else:
                    stats.enterings_dead_post += 1
                # CDCL — subsumption-aware nogood emit. min_size=1 so
                # layer-1 singleton deaths land (Q1.5b.5.c); without
                # that the layer-2 prefix-join would still pair the
                # dead singleton with every survivor.
                landed = emit_nogood(root_kb, frozenset(c), min_size=1)
                if landed:
                    stats.nogoods_emitted += 1
                else:
                    stats.nogoods_subsumed += 1
                # Singleton dead: writeback (not h) so the next
                # _compute_alive drops h, mirroring back_prop's
                # _write_negation in the tree engine (no ContextVar
                # coupling here — the monotonic loop has no chain).
                if len(c) == 1:
                    _emit_negated_fact_writeback(root_kb, c[0])
                if dumper is not None:
                    dumper.entering(
                        layer, c, result,
                        facts_merged=0,
                        nogood_emitted=landed,
                        nogood_subsumed=not landed,
                    )
                continue

            # Alive.
            stats.enterings_alive += 1

            # Fork-side is_solved (algorithm_layer_n.md §3d.vii):
            # if the fork's saturated kb already satisfies the
            # goal, this commitment IS the solution. Required for
            # puzzles whose goal directly references hypothesis
            # facts (e.g. branching/05_mini_zebra,
            # branching/07_lookahead_off, branching/10_backprop_on,
            # branching/11_backprop_off — their goal needs the
            # committed hypothesis in scope to bind, but
            # hypothesis facts never merge to root). The returned
            # Solution carries `result.kb` (the fork) so the
            # caller sees the hypothesis + derivations context.
            if is_solved(result.kb, mode):
                if dumper is not None:
                    dumper.entering(
                        layer, c, result,
                        facts_merged=0,
                        nogood_emitted=False,
                        nogood_subsumed=False,
                    )
                    dumper.early_terminate(layer, "is_solved_at_fork")
                return _finish(
                    dumper,
                    Solution(kb=result.kb, trace=(), tree=None),
                    stats,
                )

            this_merged = 0
            for f in result.unconditional_facts:
                if root_kb._fact_by_id(
                    f.relation_name, f.args,
                ) is None:
                    stored = root_kb.add_fact(f)
                    root_kb._index_fact(stored)
                    stats.facts_merged += 1
                    this_merged += 1

            if dumper is not None:
                dumper.entering(
                    layer, c, result,
                    facts_merged=this_merged,
                    nogood_emitted=False,
                    nogood_subsumed=False,
                )

            if this_merged:
                # Option A cadence (Q1.5b.2.a) — re-saturate +
                # recompute alive after every alive entering.
                _ = list(Saturator(root_kb).saturate())
                stats.saturate_count += 1
                # Merged facts could derive a contradiction at root.
                if ContradictionDetector(root_kb).detect():
                    return _finish(
                        dumper, _contradiction(root_kb), stats,
                    )
                alive = _compute_alive(root_kb)
                # Forced-positive promotion (S1.5b.5b): if alive
                # shrinks to a singleton, that hypothesis is forced.
                alive, term = _promote_forced_positives(
                    root_kb, alive, stats, mode,
                )
                if term is not None:
                    return _finish(dumper, term, stats)

                if is_solved(root_kb, mode):
                    if dumper is not None:
                        dumper.early_terminate(layer, "is_solved")
                    return _finish(
                        dumper, _solution(root_kb), stats,
                    )

                # Remaining in-flight candidates may contain
                # elements no longer alive; `try_commitment_set` handles
                # those gracefully via the dead-pre path
                # (committed fact + existing `(not h)` at root
                # → pre-saturation contradiction).

            a_layer.append(c)

        if dumper is not None:
            dumper.layer_end(layer, root_kb, len(alive), len(a_layer))
        if not a_layer:
            break
        a_prev = a_layer

    # ── Phase 3 — Verdict synthesis ──────────────────────────
    # S1.5b.6: singleton dead writebacks may have shrunk
    # `_negated_facts` since the last `_compute_alive` call (which
    # only fires on alive enterings with merged facts). Refresh
    # ``alive`` so the empty-alive contradiction check below is
    # observed when every layer-1 singleton died.
    alive = _compute_alive(root_kb)
    if is_solved(root_kb, mode):
        return _finish(dumper, _solution(root_kb), stats)
    if not alive:
        return _finish(dumper, _contradiction(root_kb), stats)
    return _finish(dumper, _ambiguity(root_kb), stats)


def _finish(
    dumper: MonotonicDumper | None,
    verdict: Verdict,
    stats: MonotonicStats,
) -> tuple[Verdict, MonotonicStats]:
    """Single exit hook — emits ``dumper.summary`` if set."""
    if dumper is not None:
        dumper.summary(verdict, stats)
    return verdict, stats


# ── Helpers ──────────────────────────────────────────────────


def _emit_negated_fact_writeback(
    root_kb: KnowledgeBase, h_id: FactId,
) -> None:
    """For a singleton dead clause ``{h_id}``, write ``(not h)``
    into ``root_kb`` so ``generate_hypotheses`` excludes ``h``
    on the next saturate and the next ``_compute_alive`` shrinks
    ``alive`` accordingly. For symmetric relations, the mirror
    ``(not (R b a))`` is written too — matches the tree-side
    ``back_propagate(..., promote_symmetric=True)`` path, since
    :func:`_compute_alive`'s symmetric canonicalisation would
    otherwise resurrect the dead entry through the mirror
    orientation.

    Minimal equivalent of
    :func:`ein_bot.inference.tree.back_prop._write_negation` —
    same shape, no :data:`_kb_chain_ctx` / :data:`_eager_pass_ctx`
    coupling (the monotonic loop has no chain and never operates
    under eager mode). Idempotent: a pre-existing ``(not h)`` at
    root is left untouched.
    """
    rn, args = h_id
    _write_negation_local(root_kb, rn, args)
    if (
        len(args) == 2
        and args[0] != args[1]
        and rn in _symmetric_relations(root_kb)
    ):
        _write_negation_local(root_kb, rn, (args[1], args[0]))


def _write_negation_local(
    root_kb: KnowledgeBase, rn: str, args: tuple,
) -> None:
    inner = Fact(
        relation_name=rn, args=args,
        layer=Layer.REASONING, provenance=None,
    )
    if root_kb._fact_by_id("not", (inner,)) is not None:
        return
    not_fact = Fact(
        relation_name="not",
        args=(inner,),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule="<monotonic-unconditional>",
            premises_raw=(),
        ),
    )
    stored = root_kb.add_fact(not_fact)
    root_kb._index_fact(stored)


def _symmetric_relations(kb: KnowledgeBase) -> frozenset[str]:
    """Names declared ``(symmetric R)`` in the ontology."""
    return frozenset(
        f.args[0]
        for f in kb._facts_by_relation.get("symmetric", ())
        if len(f.args) >= 1
    )


def _promote_forced_positives(
    root_kb: KnowledgeBase,
    alive: frozenset[FactId],
    stats: MonotonicStats,
    mode: Mode,
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
    :func:`commitment._reaches_commitment` walk through it as a
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
        stored = root_kb.add_fact(promoted)
        root_kb._index_fact(stored)
        stats.facts_merged += 1
        stats.forced_positives += 1

        _ = list(Saturator(root_kb).saturate())
        stats.saturate_count += 1

        if ContradictionDetector(root_kb).detect():
            return alive, _contradiction(root_kb)
        if is_solved(root_kb, mode):
            return alive, _solution(root_kb)
        alive = _compute_alive(root_kb)

    return alive, None


def _compute_alive(kb: KnowledgeBase) -> frozenset[FactId]:
    """Build the current alive set as a frozenset of FactIds.

    Reuses :func:`ein_bot.inference.hypgen.generate_hypotheses`
    — the canonical "which hypotheses are still viable" query —
    and canonicalises pairs under symmetric-relation declarations
    so ``(R a b)`` and ``(R b a)`` collapse to one entry when
    ``(symmetric R)`` is in the ontology.

    The canonicalisation matters for forced-positive promotion:
    hypgen emits both orientations of a symmetric pair (the user
    might want to score each independently), but for "is this
    the *sole* surviving hypothesis?" the pair is one
    hypothesis. Without dedup, alive size always ≥ 2 for any
    symmetric-relation candidate, and the promotion cascade
    never fires.
    """
    symmetric_relations = {
        f.args[0]
        for f in kb._facts_by_relation.get("symmetric", ())
        if len(f.args) >= 1
    }
    canonical: set[FactId] = set()
    for f in generate_hypotheses(kb):
        rn = f.relation_name
        args = f.args
        if (
            rn in symmetric_relations
            and len(args) == 2
            and args[0] > args[1]
        ):
            canonical.add((rn, (args[1], args[0])))
        else:
            canonical.add((rn, args))
    return frozenset(canonical)


def _solution(kb: KnowledgeBase) -> Verdict:
    return Solution(kb=kb, trace=(), tree=None)


def _contradiction(kb: KnowledgeBase) -> Verdict:
    # The full source frontier is the responsibility of S1.5b.6
    # (nogoods accumulation). For the backbone, return an empty
    # unsat_core; consumers should re-run with nogoods enabled
    # for the rich diagnostic.
    _ = kb  # reserved for future provenance walk
    return Contradiction(unsat_core=frozenset(), tree=None)


def _ambiguity(kb: KnowledgeBase) -> Verdict:
    # Semantic mismatch acknowledged: tree-side `Ambiguity`
    # carries `branches: tuple[Solution, ...]` describing
    # multiple distinct solved KBs. For monotonic exhaustion
    # (didn't reach goal within `max_set_size`), root.kb is
    # NOT a solved branch — it's a partial state. Wrapping it
    # as `Solution(kb=kb, …)` preserves the kb for the bench
    # to display, at the cost of a misleading type name. The
    # lattice's `LatticeProof` (S1.5b.29) will carry the
    # proper richer artefact.
    return Ambiguity(
        branches=(Solution(kb=kb, trace=(), tree=None),),
        unresolved=(),
        tree=None,
    )


# ── Sibling entries — gaps_solve + contradictions_solve ──────
#
# Per project_set_search_unified memory (2026-05-28): the
# engine is unified — all three public entries
# (monotonic_solve, gaps_solve, contradictions_solve) live
# side-by-side in this package. They share the per-candidate
# flow from `algorithm_layer_n.md` (Apriori prefix-join +
# try_commitment_set + flat root-writes); the difference is
# whether the loop early-terminates on first goal-sat
# (monotonic) or exhausts to collect every satisfying /
# refuted commitment (gaps / contradictions). S1.5b.21 lifts
# the shared core out of `monotonic_solve` into a private
# `_explore_layers` helper that all three entries call;
# S1.5b.23 fills `contradictions_solve`.
#
# Skeleton stage — S1.5b.20 — both raise NotImplementedError.


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

    Exhaustive Apriori-gen — no early termination. Collects
    every satisfying commitment into ``proof.solutions``;
    returns :class:`Ambiguity` (always; mode contract) whose
    branches enumerate the satisfying kbs.

    Caller interpretation:
        - ``len(verdict.branches) == 0`` — no solution within
          depth cap.
        - ``len(verdict.branches) == 1`` — uniquely solvable.
        - ``len(verdict.branches) > 1`` — genuine multi-solution.

    ``store_lattice=True`` opts into per-SetNode
    ``proof.kb_index`` storage; under :func:`gaps_solve` the
    state-hash dedup MERGE step is auto-disabled (distinct
    satisfying commitments must register separately per the
    GAPS contract).

    **S1.5b.20 stub** — raises :class:`NotImplementedError`.
    Backbone lands in S1.5b.21
    (``s1.5b.21_lattice_backbone.md``).
    """
    raise NotImplementedError(
        "gaps_solve — backbone lands in S1.5b.21. See "
        "plans/m1_core_graph_reasoning/p1.5b_lattice_search/"
        "s1.5b.21_lattice_backbone.md",
    )


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

    Exhaustive Apriori-gen — no early termination. Collects
    every dead commitment into ``proof.dead_commitments``;
    returns :class:`Contradiction` (always; mode contract)
    whose ``unsat_core`` is the union of every dead's core
    plus the learned nogood clauses.

    Caller interpretation:
        - ``len(verdict.proof.dead_commitments) == 0`` — no
          deaths within depth cap (degenerate; possibly fully
          solvable).
        - non-empty — refutation map.

    ``store_lattice=True`` enables state-hash dedup MERGE
    (distinct dead commitments with identical post-saturation
    kbs collapse into one multilabel SetNode).

    **S1.5b.20 stub** — raises :class:`NotImplementedError`.
    Backbone lands in S1.5b.23
    (``s1.5b.23_lattice_dumper.md``).
    """
    raise NotImplementedError(
        "contradictions_solve — backbone lands in S1.5b.23. "
        "See plans/m1_core_graph_reasoning/p1.5b_lattice_search/"
        "s1.5b.23_lattice_dumper.md",
    )
