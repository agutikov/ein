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
from dataclasses import dataclass, field, replace
from typing import Literal

from ein_bot.inference.apriori import (
    CanonicalSetId,
    FactId,
    generate_layer,
    layer_1,
)
from ein_bot.inference.canon import state_hash
from ein_bot.inference.commitment import try_commitment_set
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.contradiction import ContradictionDetector
from ein_bot.inference.hypgen import generate_hypotheses
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


@dataclass
class _LatticeLoopState:
    """Mutable accumulator threaded through :func:`_explore_layers`
    for ``entry in ("gaps", "contradictions")``.

    Bundles every lattice-specific local that the per-candidate
    loop mutates so the dozen return sites in the outer function
    can hand a single bag to :func:`_finalise_lattice_verdict`.
    For ``entry == "monotonic"`` the state object is constructed
    but never read.

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
            "monotonic_solve supports SOLVE mode only — use "
            "gaps_solve for GAPS or contradictions_solve for "
            "CONTRADICTIONS (sibling functions in this same "
            "module)",
        )

    # Behaviour-preserving wrapper around the shared
    # _explore_layers helper (S1.5b.21). The early-terminate
    # paths, Phase 3 verdict synthesis, and stats counters all
    # match the pre-refactor monotonic_solve. The helper's
    # `entry="monotonic"` discriminator selects the
    # solution-mode dispatch on every outcome node.
    verdict, stats = _explore_layers(
        root_kb,
        entry="monotonic",
        max_set_size=max_set_size,
        config=config,
        dumper=dumper,
        max_time=max_time,
        max_enterings=max_enterings,
    )
    # The MonotonicStats type-narrow is safe here — entry
    # "monotonic" always returns MonotonicStats.
    assert isinstance(stats, MonotonicStats)
    return verdict, stats


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


# ── Shared core loop — _explore_layers ───────────────────────
#
# S1.5b.21: extracted from the pre-refactor `monotonic_solve`
# body. The `entry` discriminator dispatches outcomes for the
# three public functions (monotonic_solve, gaps_solve,
# contradictions_solve). The shared core is NEVER DUPLICATED
# across the three entries — they're all thin wrappers.
#
# Behaviour-preserving for `entry="monotonic"`. New behaviour
# for `entry="gaps"`: instead of early-terminate on is_solved,
# record into a local `solutions` list and continue the
# search. Phase 3 synthesises `Ambiguity(branches=[Solution
# for each recorded])`. The `entry="contradictions"` branch is
# reserved for S1.5b.23.


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
    free of the lattice counters preserves the
    :func:`monotonic_solve` public surface unchanged.
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
    lattice_stats = _build_lattice_stats(
        stats,
        solutions_found=len(lstate.solutions),
        state_hash_merges=lstate.state_hash_merges,
        elapsed_seconds=elapsed_seconds,
    )
    proof = LatticeProof(
        solutions=tuple(lstate.solutions),
        dead_commitments=tuple(lstate.dead_commitments),
        kb_index=dict(lstate.kb_index) if store_lattice else {},
        alive_at_end=lstate.alive_at_end_tuple,
        learned_nogoods=frozenset(root_kb._nogoods),
        stats=lattice_stats,
    )
    if entry == "gaps":
        branches = tuple(
            Solution(kb=s.kb, trace=s.firings, tree=None)
            for s in lstate.solutions
        )
        return Ambiguity(
            branches=branches, unresolved=(), tree=None, proof=proof,
        )
    # entry == "contradictions"
    cores: frozenset[Fact] = frozenset()
    for d in lstate.dead_commitments:
        cores = cores | d.unsat_core
    return Contradiction(unsat_core=cores, tree=None, proof=proof)


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


def _explore_layers(
    root_kb: KnowledgeBase,
    *,
    entry: Literal["monotonic", "gaps", "contradictions"],
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,
    dumper: MonotonicDumper | LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Verdict, MonotonicStats]:
    """The shared per-candidate loop. See
    `plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md`
    for the per-step contract.

    ``entry`` discriminator picks the outcome dispatch + Phase 3
    verdict synthesis:

    - ``"monotonic"`` — early-terminate on first goal-sat at
      fork or root; Phase 3 returns
      Solution / Ambiguity-frontier / Contradiction per the
      existing trichotomy. Ignores ``store_lattice``.
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

    # Mode for is_solved-style checks. The mode parameter that
    # used to live on monotonic_solve was about verdict shape,
    # which we now dispatch via `entry`. For goal satisfaction
    # the check is always SOLVE-mode semantics.
    mode = Mode.SOLVE

    stats = MonotonicStats()
    t_start = time.perf_counter()
    lstate = _LatticeLoopState()
    # store_lattice is monotonic-irrelevant — short-circuit even
    # if a caller pased it through (defensive; the public
    # ``monotonic_solve`` doesn't expose the flag).
    if entry == "monotonic":
        store_lattice = False

    def _finalise_gaps() -> tuple[Verdict, MonotonicStats]:
        """Build the gaps-side Ambiguity verdict from current
        ``lstate`` + emit the dumper summary via :func:`_finish`.
        Hoisted into a nested function so the dozen gaps return
        sites stay single-line.
        """
        verdict = _finalise_lattice_verdict(
            "gaps",
            root_kb=root_kb,
            lstate=lstate,
            stats=stats,
            elapsed_seconds=time.perf_counter() - t_start,
            store_lattice=store_lattice,
        )
        return _finish(dumper, verdict, stats)

    def _finalise_contradictions() -> tuple[Verdict, MonotonicStats]:
        """Build the contradictions-side Contradiction verdict
        (``unsat_core`` = set union over every recorded
        dead's ``unsat_core``)."""
        verdict = _finalise_lattice_verdict(
            "contradictions",
            root_kb=root_kb,
            lstate=lstate,
            stats=stats,
            elapsed_seconds=time.perf_counter() - t_start,
            store_lattice=store_lattice,
        )
        return _finish(dumper, verdict, stats)

    # ── Phase 1 — Initial saturation + alive ──────────────────
    _ = list(Saturator(root_kb).saturate())
    stats.saturate_count += 1
    if dumper is not None:
        dumper.root_initial(root_kb)
    if ContradictionDetector(root_kb).detect():
        if entry == "monotonic":
            return _finish(dumper, _contradiction(root_kb), stats)
        if entry == "gaps":
            # zero solutions; Ambiguity with empty branches.
            return _finalise_gaps()
        # contradictions: root itself is contradictory — empty
        # ``proof.dead_commitments``, ``verdict.unsat_core`` is
        # the empty frozenset (no commitments were tried).
        return _finalise_contradictions()
    if is_solved(root_kb, mode):
        if entry == "monotonic":
            return _finish(dumper, _solution(root_kb), stats)
        if entry == "gaps":
            # root satisfies trivially; record with empty
            # commitment carrier + return Ambiguity with 1 branch.
            lstate.solutions.append(SolutionRecord(
                commitment=(), kb=root_kb.snapshot(),
                firings=(), layer=0,
            ))
            return _finalise_gaps()
        # contradictions: root is_solved before any commitment.
        # No early return — supersets of singleton hypotheses
        # can still surface deads; fall through to Phase 2.

    alive = _compute_alive(root_kb)
    alive, term = _promote_forced_positives(root_kb, alive, stats, mode)
    if term is not None:
        if entry == "monotonic":
            return _finish(dumper, term, stats)
        if entry == "gaps":
            # if cascade landed Solution, that's one branch;
            # if Contradiction, zero branches (Ambiguity with empty).
            if isinstance(term, Solution):
                lstate.solutions.append(SolutionRecord(
                    commitment=(), kb=root_kb.snapshot(),
                    firings=(), layer=0,
                ))
            return _finalise_gaps()
        # contradictions:
        if isinstance(term, Contradiction):
            # root contradicted by cascade — no deads collected.
            return _finalise_contradictions()
        # Solution: root satisfies after a forced-positive
        # cascade. Don't short-circuit — supersets may still
        # die. Fall through to Phase 2.
    if not alive:
        if entry == "monotonic":
            return _finish(dumper, _contradiction(root_kb), stats)
        if entry == "gaps":
            return _finalise_gaps()
        # contradictions: empty alive + no Phase-2 work to do.
        return _finalise_contradictions()

    a_prev: list[CanonicalSetId] = layer_1(alive)

    # ── Phase 2 — Layer-by-layer iteration ───────────────────
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

        candidates.sort()  # lex; scoring switch in S1.5b.26

        a_layer: list[CanonicalSetId] = []
        for c in candidates:
            # Budget gate — same as pre-refactor monotonic_solve.
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

                # S1.5b.22 state-hash dedup (contradictions) /
                # SetNode storage (gaps). Only for dead-post —
                # dead-pre's "kb" is the unsaturated fork and
                # carries no extra information worth storing.
                # Returns True only on contradictions-side merge;
                # under gaps the merge step is auto-disabled
                # (per-commitment key).
                skip_downstream = False
                if (
                    store_lattice
                    and result.kind == "dead-post"
                    and entry in ("gaps", "contradictions")
                ):
                    skip_downstream = _record_setnode(
                        lstate,
                        entry=entry,  # type: ignore[arg-type]
                        commitment=c,
                        result_kb=result.kb,
                        verdict_label="dead",
                        layer=layer,
                    )
                if skip_downstream:
                    # The earlier arrival in this kb-state slot
                    # already wrote the nogood + writeback AND
                    # appended the DeadCommitment. Skip to next
                    # candidate per S1.5b.22's "entry-side
                    # collection" comment in the spec.
                    continue

                landed = emit_nogood(root_kb, frozenset(c), min_size=1)
                if landed:
                    stats.nogoods_emitted += 1
                else:
                    stats.nogoods_subsumed += 1
                if len(c) == 1:
                    _emit_negated_fact_writeback(root_kb, c[0])

                # S1.5b.23 — contradictions collects every dead
                # commitment with its unsat-core + learned clause.
                if entry == "contradictions":
                    lstate.dead_commitments.append(DeadCommitment(
                        commitment=c,
                        unsat_core=result.unsat_core,
                        learned_clause=frozenset(c),
                        layer=layer,
                        kind=result.kind,
                    ))

                if dumper is not None:
                    dumper.entering(
                        layer, c, result,
                        facts_merged=0,
                        nogood_emitted=landed,
                        nogood_subsumed=not landed,
                    )
                    if (
                        entry == "contradictions"
                        and hasattr(dumper, "dead_recorded")
                    ):
                        dumper.dead_recorded(
                            lstate.dead_commitments[-1],
                        )
                continue

            # Alive.
            stats.enterings_alive += 1
            solved = is_solved(result.kb, mode)

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
                if entry == "monotonic":
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
                            facts_merged=0,
                            nogood_emitted=False,
                            nogood_subsumed=False,
                        )
                        if hasattr(dumper, "solution_recorded"):
                            dumper.solution_recorded(
                                lstate.solutions[-1], layer,
                            )
                    continue  # don't merge; don't append to a_layer
                # entry == "contradictions": no solution recording.
                # Fall through to the alive flow so unconditional
                # facts merge into root and ``c`` lands in
                # ``a_layer`` for next-layer pair generation —
                # supersets of a solved commitment can still die
                # under additional hypotheses.

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
                    if entry == "monotonic":
                        return _finish(
                            dumper, _contradiction(root_kb), stats,
                        )
                    # gaps / contradictions: root contradictory;
                    # stop exploring. Phase 3 synthesises the
                    # entry-specific verdict (gaps→Ambiguity with
                    # whatever solutions were collected;
                    # contradictions→Contradiction with deads).
                    phase_2_done = True
                    break
                alive = _compute_alive(root_kb)
                alive, term = _promote_forced_positives(
                    root_kb, alive, stats, mode,
                )
                if term is not None:
                    if entry == "monotonic":
                        return _finish(dumper, term, stats)
                    if entry == "gaps":
                        if isinstance(term, Solution):
                            if not lstate.root_was_solved:
                                lstate.solutions.append(SolutionRecord(
                                    commitment=c,
                                    kb=root_kb.snapshot(),
                                    firings=(), layer=layer,
                                ))
                                lstate.root_was_solved = True
                                if (
                                    dumper is not None
                                    and hasattr(
                                        dumper, "solution_recorded",
                                    )
                                ):
                                    dumper.solution_recorded(
                                        lstate.solutions[-1], layer,
                                    )
                        # gaps: cascade hit a terminal — exit
                        # Phase 2 either way (Solution: root
                        # satisfies; Contradiction: root
                        # contradictory). Further exploration is
                        # redundant.
                        phase_2_done = True
                        break
                    # entry == "contradictions":
                    if isinstance(term, Contradiction):
                        # root contradicted by cascade — stop.
                        phase_2_done = True
                        break
                    # Solution: continue exploring. Supersets
                    # extending the cascade's now-solved root may
                    # still die under further hypotheses, and we
                    # want to enumerate those deads.

                if is_solved(root_kb, mode):
                    if entry == "monotonic":
                        if dumper is not None:
                            dumper.early_terminate(layer, "is_solved")
                        return _finish(
                            dumper, _solution(root_kb), stats,
                        )
                    if entry == "gaps":
                        # record once (root_was_solved guard)
                        # then terminate Phase 2 — root satisfies,
                        # so every remaining candidate would just
                        # re-confirm via fork-side is_solved.
                        if not lstate.root_was_solved:
                            lstate.solutions.append(SolutionRecord(
                                commitment=c, kb=root_kb.snapshot(),
                                firings=(), layer=layer,
                            ))
                            lstate.root_was_solved = True
                            if (
                                dumper is not None
                                and hasattr(
                                    dumper, "solution_recorded",
                                )
                            ):
                                dumper.solution_recorded(
                                    lstate.solutions[-1], layer,
                                )
                        phase_2_done = True
                        break
                    # entry == "contradictions": root is_solved
                    # is not a stop condition — supersets at
                    # higher layers may still die. Continue.

            a_layer.append(c)

        if dumper is not None:
            dumper.layer_end(layer, root_kb, len(alive), len(a_layer))
        if phase_2_done:
            break
        if not a_layer:
            break
        a_prev = a_layer
        # Capture the surviving size-N frontier when the depth cap
        # is the natural loop terminator. ``alive_at_end`` stays
        # ``()`` on every early-exit path (contradiction, root
        # solved, frontier exhausted).
        if layer == max_set_size:
            lstate.alive_at_end_tuple = tuple(a_layer)

    # ── Phase 3 — Verdict synthesis ──────────────────────────
    if entry == "monotonic":
        # S1.5b.6: singleton dead writebacks may have shrunk
        # `_negated_facts` since the last `_compute_alive` call;
        # refresh so the empty-alive contradiction check below
        # is observed.
        alive = _compute_alive(root_kb)
        if is_solved(root_kb, mode):
            return _finish(dumper, _solution(root_kb), stats)
        if not alive:
            return _finish(dumper, _contradiction(root_kb), stats)
        return _finish(dumper, _ambiguity(root_kb), stats)

    if entry == "gaps":
        # always Ambiguity (mode contract).
        return _finalise_gaps()

    # entry == "contradictions": always Contradiction (mode contract).
    return _finalise_contradictions()


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
