"""Monotonic engine main loop — S1.5b.5 backbone.

Single ``KnowledgeBase`` instance (root); commitment sets entered
via :func:`ein_bot.inference.commitment.try_commitment_set`;
unconditional facts merged into root; re-saturate + recompute
alive after each merge (Option A cadence — Q1.5b.2.a); terminate
on :func:`is_solved` (``root.kb``) or layer exhaustion.

SOLVE mode only (Q1.5b.7). No CDCL nogoods yet (S1.5b.6 wires
that in). No dumper (S1.5b.7).
"""
from __future__ import annotations

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
from ein_bot.inference.saturator import Saturator
from ein_bot.inference.tree.solver import (
    Ambiguity,
    Contradiction,
    Mode,
    Solution,
    Verdict,
    is_solved,
)
from ein_bot.kb.store import KnowledgeBase


@dataclass
class MonotonicStats:
    """Cumulative counters for one :func:`monotonic_solve` run."""

    enterings_total:     int = 0
    enterings_alive:     int = 0
    enterings_dead_pre:  int = 0
    enterings_dead_post: int = 0
    facts_merged:        int = 0
    saturate_count:      int = 0
    layers_explored:     int = 0


def monotonic_solve(
    root_kb: KnowledgeBase,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    mode: Mode = Mode.SOLVE,
) -> tuple[Verdict, MonotonicStats]:
    """Run the monotonic set-search engine on ``root_kb``.

    Returns ``(verdict, stats)``. SOLVE mode only (Q1.5b.7) —
    GAPS / CONTRADICTIONS belong to the lattice engine.

    The signature deviates from the S1.5b.5 spec (which returned
    just ``Verdict``) — the tuple form gives the bench script
    + tests direct access to the per-run counters without a
    side-channel on ``root_kb``. See stage Ship notes.
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

    # ── Phase 1 — Initial saturation + alive ──────────────────
    _ = list(Saturator(root_kb).saturate())
    stats.saturate_count += 1
    # Root contradiction (e.g., a rule derived `(false)` directly):
    # spec didn't list this check but without it, a contradictory
    # root falls through to Phase 2 where every entering dead-pre's
    # and the final verdict mis-reports as Ambiguity.
    if ContradictionDetector(root_kb).detect():
        return _contradiction(root_kb), stats
    if is_solved(root_kb, mode):
        return _solution(root_kb), stats

    alive = _compute_alive(root_kb)
    if not alive:
        return _contradiction(root_kb), stats

    a_prev: list[CanonicalSetId] = layer_1(alive)

    # ── Phase 2 — Layer-by-layer iteration ───────────────────
    for layer in range(1, max_set_size + 1):
        stats.layers_explored = layer

        if layer == 1:
            candidates = list(a_prev)
        else:
            candidates = generate_layer(
                a_prev,
                alive=alive,
                nogoods=frozenset(),  # S1.5b.6 will pass root_kb._nogoods
            )

        candidates.sort()  # lex; scoring switch lands in monotonic followups

        a_layer: list[CanonicalSetId] = []
        for c in candidates:
            stats.enterings_total += 1
            result = try_commitment_set(root_kb, c)

            if result.kind == "dead-pre":
                stats.enterings_dead_pre += 1
                continue
            if result.kind == "dead-post":
                stats.enterings_dead_post += 1
                continue

            # Alive.
            stats.enterings_alive += 1
            root_changed = False
            for f in result.unconditional_facts:
                if root_kb._fact_by_id(
                    f.relation_name, f.args,
                ) is None:
                    stored = root_kb.add_fact(f)
                    root_kb._index_fact(stored)
                    stats.facts_merged += 1
                    root_changed = True

            if root_changed:
                # Option A cadence (Q1.5b.2.a) — re-saturate +
                # recompute alive after every alive entering.
                _ = list(Saturator(root_kb).saturate())
                stats.saturate_count += 1
                # Merged facts could derive a contradiction at root.
                if ContradictionDetector(root_kb).detect():
                    return _contradiction(root_kb), stats
                alive = _compute_alive(root_kb)

                if is_solved(root_kb, mode):
                    return _solution(root_kb), stats

                # Remaining in-flight candidates may contain
                # elements no longer alive; `try_commitment_set` handles
                # those gracefully via the dead-pre path
                # (committed fact + existing `(not h)` at root
                # → pre-saturation contradiction).

            a_layer.append(c)

        if not a_layer:
            break
        a_prev = a_layer

    # ── Phase 3 — Verdict synthesis ──────────────────────────
    if is_solved(root_kb, mode):
        return _solution(root_kb), stats
    if not alive:
        return _contradiction(root_kb), stats
    return _ambiguity(root_kb), stats


# ── Helpers ──────────────────────────────────────────────────


def _compute_alive(kb: KnowledgeBase) -> frozenset[FactId]:
    """Build the current alive set as a frozenset of FactIds.

    Reuses :func:`ein_bot.inference.hypgen.generate_hypotheses`
    — the canonical "which hypotheses are still viable" query.
    """
    return frozenset(
        (f.relation_name, f.args)
        for f in generate_hypotheses(kb)
    )


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
