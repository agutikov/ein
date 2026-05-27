"""Monotonic engine main loop — S1.5b.5 backbone + S1.5b.6 CDCL.

Single ``KnowledgeBase`` instance (root); commitment sets entered
via :func:`ein_bot.inference.commitment.try_commitment_set`;
unconditional facts merged into root; re-saturate + recompute
alive after each merge (Option A cadence — Q1.5b.2.a); terminate
on :func:`is_solved` (``root.kb``) or layer exhaustion.

SOLVE mode only (Q1.5b.7). CDCL nogoods integrated via
:func:`ein_bot.inference.nogoods.emit_nogood` — every dead
entering emits ``frozenset(C)`` (singletons included; see
Q1.5b.5.c). Singleton deaths additionally write ``(not h)`` into
``root_kb._negated_facts`` so the next ``_compute_alive`` drops
``h`` from ``alive`` (mirrors ``tree/back_prop._write_negation``
without its ContextVar coupling). No dumper (S1.5b.7).
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
    alive, term = _promote_forced_positives(root_kb, alive, stats, mode)
    if term is not None:
        return term, stats
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
                nogoods=root_kb._nogoods,
            )

        candidates.sort()  # lex; scoring switch lands in monotonic followups

        a_layer: list[CanonicalSetId] = []
        for c in candidates:
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
                if emit_nogood(root_kb, frozenset(c), min_size=1):
                    stats.nogoods_emitted += 1
                else:
                    stats.nogoods_subsumed += 1
                # Singleton dead: writeback (not h) so the next
                # _compute_alive drops h, mirroring back_prop's
                # _write_negation in the tree engine (no ContextVar
                # coupling here — the monotonic loop has no chain).
                if len(c) == 1:
                    _emit_negated_fact_writeback(root_kb, c[0])
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
                # Forced-positive promotion (S1.5b.5b): if alive
                # shrinks to a singleton, that hypothesis is forced.
                alive, term = _promote_forced_positives(
                    root_kb, alive, stats, mode,
                )
                if term is not None:
                    return term, stats

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
    # S1.5b.6: singleton dead writebacks may have shrunk
    # `_negated_facts` since the last `_compute_alive` call (which
    # only fires on alive enterings with merged facts). Refresh
    # ``alive`` so the empty-alive contradiction check below is
    # observed when every layer-1 singleton died.
    alive = _compute_alive(root_kb)
    if is_solved(root_kb, mode):
        return _solution(root_kb), stats
    if not alive:
        return _contradiction(root_kb), stats
    return _ambiguity(root_kb), stats


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
