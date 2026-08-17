"""Saturation engine with banded priorities — S1.3.3.

Wraps :class:`Engine` (from S1.3.1) in a priority-ordered firing
loop. Fires rules until no rule produces a new fact, in
:attr:`kb.Rule.priority` order (lower fires first), deduplicating
along the way.

Q41 (resolved 2026-05-20) locked the priority scale:

- 100 — propagate band: `symmetric`, `implies`.
- 200 — derive band:    `transitive`, `square-fwd`, `square-bwd`.
- 300 — eliminate band: `type-exclusivity`.
- 900 — hypothesis band: `hypothesis-contradiction` (fires only
  when P1.5's fork/contradict machinery emits the synthetic
  premise facts).

Within-band ordering is FIFO (insertion order). Priorities live
**per-rule** inside the `(rule …)` form — the saturator reads
`kb.Rule.priority`; there is no static-config priority table.

Algorithm:

1. Enqueue every (plan, binding) pair the matcher currently yields,
   stamped with its rule's priority and a monotonic tiebreaker.
2. On `step()`, pop the lowest-priority entry. If the entry's
   binding has been fired already (engine._fired tracks this), drop
   and recurse. If the conclusion fact already exists in the KB,
   yield a :class:`Firing` with ``redundant=True`` and do NOT apply.
   Otherwise apply via :func:`fire` and yield the firing.
3. After a successful (non-redundant) firing, re-run the enqueue
   pass — newly added facts may unlock matches the previous pass
   missed.
4. ``saturate()`` is an iterator yielding firings until the queue
   is empty after an enqueue pass.

Complexity: each step does one `_enqueue_pass` (O(plans x facts))
plus a heap pop. For Zebra-scale KBs (≤ 100 facts, 7 plans) this
is a few hundred operations per step.
"""
from __future__ import annotations

import heapq
from collections.abc import Iterator
from typing import Any

from ein.kb.entities import Fact
from ein.kb.provenance import Provenance
from ein.kb.store import KnowledgeBase

from . import match
from .compile import Join, JoinPlan, NafGuard, Scan
from .config import SolverConfig
from .engine import Engine, _hashable
from .firing import Firing, build_fact, fire
from .world import World

# Default priority for rules with no :priority kw-pair. Sits between
# eliminate (300) and hypothesis (900) — well-defined, but rarely
# expected to be hit because every shipping rule declares a priority.
_DEFAULT_PRIORITY = 1000

# Dunder convention — the kernel-native symmetric mirror trigger. A relation
# marked `(__symmetric__ R)` has its extension closed under arg-swap directly
# by the saturator (no compiled rule / `match.run`) — the perf-opt counterpart
# of the stdlib `symmetric` closure rule. See
# docs/kernel/inference/reserved_engine_strings.md.
SYMMETRIC = "__symmetric__"


def _positive_relations(plan: JoinPlan) -> set[str]:
    """Relations of a plan's TOP-LEVEL positive Scan/Join premises (incl. the
    A13 ``extra_match_plans`` disjuncts) — the **D5-seedable** set: a new fact
    at one of these creates a new binding the matcher can find by seeding *at*
    that fact (:func:`match.run_seeded`) instead of re-scanning the relation's
    whole extent."""
    rels: set[str] = set()
    for step in plan.steps:
        if isinstance(step, (Scan, Join)):
            rels.add(step.relation)
    for extra in plan.extra_match_plans:
        for step in extra:
            if isinstance(step, (Scan, Join)):
                rels.add(step.relation)
    return rels


# S1.21.8 — `_absent_relations` is **gone**, not bypassed.
#
# It drove the absent-flip full-match split: a plan whose delta relation
# appeared inside an `AbsentGuard` had to re-run `match.run` over the whole
# extent, because a `forall`'s nested absent can flip from failing to passing
# as the KB grows and there is no positive premise to seed at (S1.8.B2v D5;
# skipping it was the D2 completeness bug).
#
# With guards lifted out of the closure, a flip is no longer something the
# *matcher* has to notice. NAF-guarded candidates are parked, and every parked
# candidate is re-evaluated against the world at each quiescence — which is
# strictly more complete than the full-match split (it also catches a flip
# with no delta in the watched relation at all) and costs one guard
# evaluation instead of one full re-match.


class Saturator:
    """Priority-banded saturation driver over a :class:`KnowledgeBase`.

    Construct with `Saturator(kb)` (auto-builds the engine) or
    `Saturator(kb, engine)` (provided engine, must already have its
    compile cache populated or be ready to populate it).
    """

    def __init__(
        self, kb: KnowledgeBase, engine: Engine | None = None,
    ) -> None:
        self.kb = kb
        self.engine = engine if engine is not None else Engine(kb)
        if not self.engine.cache:
            self.engine.compile_all()
        # (binding_key, guards) -> already enqueued. S1.22.0 — the guards are
        # part of the key, not decoration: see `_enqueue_binding`.
        self._seen: set[
            tuple[tuple[str, tuple[str, ...], frozenset],
                  tuple[NafGuard, ...]]
        ] = set()
        # heap entries: (priority, tiebreaker, plan, bindings, premises,
        # naf_guards). Guards ride along so `_apply` can record them as the
        # firing's negative premises; a queued entry's guards have already
        # passed on the boundary (S1.21.8).
        self._queue: list[
            tuple[int, int, JoinPlan, dict[str, Any], tuple[Fact, ...],
                  tuple[NafGuard, ...]]
        ] = []
        self._tiebreaker = 0
        # Optimisation: enqueue_pass is needed only after a productive
        # firing (a new fact was written; new matches may exist) or
        # before the very first step.
        self._needs_enqueue: bool = True
        # Last yielded Firing — kept on the instance so a step-limit
        # raise can quote it without forcing every caller to track it.
        self._last_firing: Firing | None = None
        # S1.21.8 — structurally 0, kept as an observable so the old
        # measurement gate still has something to assert. It counted firings
        # dropped at dequeue because an AbsentGuard that passed at enqueue no
        # longer held at fire time (S1.5a.1). There is no enqueue/fire NAF
        # race any more: a guard is evaluated once, on the boundary, at the
        # moment the candidate is admitted.
        self.naf_dropped: int = 0
        # S1.21.8 two-phase saturation. Candidates whose disjunct carries
        # `(absent …)` guards never enter `_queue` directly — they wait here
        # until the positive closure quiesces, then are judged against that
        # fixpoint (the world W). Entries: (priority, tiebreaker, plan,
        # bindings, premises, guards). A candidate that fails stays parked:
        # a `forall`'s nested absent can flip from failing to *passing* as
        # the KB grows, so "failed once" is not "failed forever".
        self._parked: list[
            tuple[int, int, JoinPlan, dict[str, Any], tuple[Fact, ...],
                  tuple[NafGuard, ...]]
        ] = []
        # tiebreaker -> the watch stamp at that candidate's last (failed)
        # boundary judgement. See `_watch_stamp`: an unchanged stamp means the
        # verdict cannot have moved, so the query is not re-run.
        self._park_stamp: dict[int, tuple[int, ...]] = {}
        # Rounds of the outer (boundary) loop, and admissions made — the
        # observable that says the two-phase loop actually alternated.
        self.naf_rounds: int = 0
        self.naf_admitted: int = 0
        # Candidates retired at the boundary because an anti-monotone guard
        # failed — permanently unfirable, so never re-judged.
        self.naf_retired: int = 0
        # S1.8.B2v — incremental (delta-driven) enqueue. After the cold full
        # pass, an enqueue pass processes only the DELTA — the facts the last
        # productive firing derived: positive-premise plans are SEEDED at the
        # new fact (D5 semi-naive, `match.run_seeded`), absent-premise plans
        # (a `forall` that may flip) full-match, and any never-matched plan
        # (reflective) full-matches. `_delta_facts is None` ⇒ a FULL pass
        # (cold start / is_stalled).
        self._delta_facts: list[Fact] | None = None
        self._matched_plan_ids: set[int] = set()
        # relation → plans with that relation as a positive Scan/Join premise;
        # rebuilt when the compile cache grows. See `_indexes`. (S1.21.8
        # removed the `absent`-polarity twin — guards are off the match path.)
        self._pos_index: dict[str, list[JoinPlan]] | None = None
        self._index_n: int = -1
        # `__symmetric__` native mirror (kernel opt). Marked relations (cached,
        # refreshed when the marker count changes), and the queue of source
        # facts whose arg-swap mirror is still pending.
        self._sym_rels: frozenset[str] | None = None
        self._sym_n: int = -1
        self._mirror_queue: list[Fact] = []
        self._mirror_seeded: bool = False
        # S1.20.I2 — native mirror gate (default on); read once from the
        # resolved config on the kb (`solve` sets `kb.config`).
        cfg = kb.config or SolverConfig()
        self._mirror_enabled: bool = cfg.enable_symmetric_mirror
        # S1.21.7 — record re-derivations as alternative justifications
        # (default on). Read once: this is consulted on the redundant-firing
        # path, the highest-volume path in the engine.
        self._record_alternatives: bool = cfg.record_alternative_justifications

    # ── Public API ────────────────────────────────────────────────

    def step(self) -> Firing | None:
        """Produce the next firing, or ``None`` at the two-phase fixpoint.

        S1.21.8 — the loop alternates two phases:

        1. **Closure (inner).** Purely positive plans fire to quiescence.
           No negation is consulted here at all.
        2. **Boundary (outer).** At quiescence, every parked NAF-guarded
           candidate is judged against that fixpoint — the world ``W`` — and
           the ones whose guards pass are admitted back into phase 1.

        Repeat until a quiescence admits nothing. Every `(absent …)` verdict
        is therefore taken against a saturated world, which is what makes
        `absent_semantics.md`'s ``W ⊭ ∃x̄.Pθ`` reading literal rather than
        approximated by fire-time re-checking.
        """
        while True:
            firing = self._closure_step()
            if firing is not None:
                return firing
            # Positive quiescence — the boundary gets to speak.
            if not self._admit_from_boundary():
                return None

    def _closure_step(self) -> Firing | None:
        """One purely-positive firing, or ``None`` at closure quiescence."""
        if self._needs_enqueue:
            self._enqueue_pass(self._delta_facts)
            self._needs_enqueue = False
            self._delta_facts = None
        # `__symmetric__` native mirror — produced before rule firings so the
        # closure is available to them. A no-op when no relation is marked.
        mirror = self._next_mirror_firing() if self._mirror_enabled else None
        if mirror is not None:
            return mirror
        while self._queue:
            _priority, _tb, plan, bindings, premises, guards = heapq.heappop(
                self._queue)
            key = self._binding_key(plan, bindings)

            # Engine._fired is the canonical "already fired" record —
            # we skip if either Saturator's own loop or a prior
            # Engine.step() (from the same Engine instance) has fired
            # this binding.
            if key in self.engine._fired:
                continue

            firing = self._apply(plan, bindings, premises, key, guards)
            if firing is None:
                continue
            # A productive (non-redundant) firing wrote a new fact;
            # the next step needs a fresh enqueue pass to pick up
            # downstream matches. Redundant firings change nothing.
            if not firing.redundant:
                self._needs_enqueue = True
                # S1.8.B2v — the next enqueue pass processes only the facts
                # this firing just derived (accumulated until consumed).
                if self._delta_facts is None:
                    self._delta_facts = list(firing.derived)
                else:
                    self._delta_facts.extend(firing.derived)
                # A rule-derived marked fact needs its mirror too.
                if self._mirror_enabled:
                    self._enqueue_mirror_sources(firing.derived)
            return firing
        return None

    def _admit_from_boundary(self) -> int:
        """Judge parked candidates against the quiesced world; admit at most one.

        Returns the number admitted (0 or 1). Parked candidates are examined
        in the engine's own (priority, FIFO) order and the **first** whose
        guards pass is admitted; the rest stay parked and are re-judged after
        the closure quiesces again.

        Why one and not the batch. Admitting a whole round's worth would let
        one admission invalidate another's guard *after* that guard's verdict
        was taken — which is the enqueue/fire race this stage exists to
        remove, merely relocated. It shows up on the classic unstratifiable
        program ``p ← absent q; q ← absent p``: both guards pass against the
        empty world, so a batch admission derives **both**, and ``{p, q}`` is
        not a model of that program under any reading. Admitting one closes
        the window completely — the queue is empty at quiescence, so the
        admitted candidate fires immediately, against exactly the world its
        guard was judged against. Nothing can go stale, which is why there is
        no fire-time re-check to reinstate and ``naf_dropped`` is 0.

        On a *stratified* program the choice is immaterial: no admission can
        invalidate another's guard, so one-at-a-time and batch agree — and
        both agree regardless of the rules' priorities, which is what demotes
        the priority-band discipline from load-bearing to advisory.

        Failures stay parked. A `forall`'s nested absent flips from failing
        to passing as the KB grows (``(absent (and G (absent B)))``: adding a
        ``B`` makes the inner fail and the outer pass), so a parked candidate
        is a standing question, not a settled one.
        """
        if not self._parked:
            return 0
        world = World(self.kb)
        self.naf_rounds += 1
        rejected: list = []
        admitted = 0
        while self._parked:
            entry = heapq.heappop(self._parked)
            _priority, tb, plan, bindings, _premises, guards = entry
            if self._binding_key(plan, bindings) in self.engine._fired:
                self._park_stamp.pop(tb, None)
                continue                     # fired by another route
            # Invalidation: a guard's verdict can only move if one of the
            # relations its query reads has grown. Most parked candidates are
            # waiting on something that did not change this round, and
            # re-running their queries is what makes a naive boundary
            # quadratic (zebra2 root: 460 parked, 40 rounds).
            stamp = self._watch_stamp(guards)
            if self._park_stamp.get(tb) == stamp:
                rejected.append(entry)
                continue
            failing = world.first_failing(guards, bindings)
            if failing is None:
                self._park_stamp.pop(tb, None)
                heapq.heappush(self._queue, entry)
                self.naf_admitted += 1
                admitted = 1
                break
            if failing.monotone:
                # Anti-monotone guard, and it found a match: the KB only
                # grows, so it will keep finding one. This candidate is dead,
                # not waiting — retire it rather than re-asking every round.
                self._park_stamp.pop(tb, None)
                self.naf_retired += 1
                continue
            self._park_stamp[tb] = stamp
            rejected.append(entry)
        for entry in rejected:
            heapq.heappush(self._parked, entry)
        return admitted

    def _watch_stamp(self, guards: tuple[NafGuard, ...]) -> tuple[int, ...]:
        """Extent sizes of every relation ``guards`` read, in a stable order.

        Equal stamps ⇒ every watched relation holds exactly the facts it did
        last time this candidate was judged ⇒ the guard sub-plan's match set
        is unchanged ⇒ so is its verdict. Sizes suffice because the KB grows
        monotonically within a saturation run: a relation cannot lose a fact,
        so equal size means equal extent.
        """
        fbr = self.kb._facts_by_relation
        return tuple(
            len(fbr.get(rel, ()))
            for g in guards for rel in sorted(g.watched)
        )

    def saturate(
        self, *, max_steps: int | None = None,
    ) -> Iterator[Firing]:
        """Run firings until the queue is empty after an enqueue pass.

        Yields each :class:`Firing` (productive + redundant) so the
        trace renderer (P1.6) can stream per-step DOT snapshots.

        Args:
            max_steps: hard cap on the number of step() calls. None
                (the default) means run to fixed point; a positive
                integer raises :class:`SaturatorStepLimitError` after
                ``max_steps`` firings without saturation. Use for
                runaway-debugging on unfamiliar inputs.
        """
        i = 0
        while True:
            if max_steps is not None and i >= max_steps:
                raise SaturatorStepLimitError(
                    f"saturator hit max_steps={max_steps} without reaching "
                    f"fixed point — last firing was {self._last_firing!r}; "
                    f"see Saturator._last_firing for the runaway candidate."
                )
            f = self.step()
            if f is None:
                return
            self._last_firing = f
            i += 1
            yield f

    def is_stalled(self) -> bool:
        """True iff no firing is currently available — at the *two-phase*
        fixpoint, not merely at closure quiescence.

        Forces a fresh enqueue pass first — callers may have written
        facts directly to the KB outside ``step()``'s flow — so any
        newly-matchable binding is picked up before the stalled check.

        S1.21.8: a parked NAF candidate whose guards now pass is a firing
        that is available, so the check has to consult the boundary too.
        Answering from the positive queue alone would report "stalled" while
        a `forall` was one round away from admitting.
        """
        # Always force a fresh pass — callers may have written facts
        # directly to the KB outside of step()'s flow (e.g. P1.5).
        self._enqueue_pass()
        self._needs_enqueue = False
        # The queue may contain entries whose bindings have already
        # been fired — filter them out before claiming stalled.
        if self._mirror_enabled and self._has_pending_mirror():
            return False
        if any(self._binding_key(p, b) not in self.engine._fired
               for _pr, _tb, p, b, _pf, _g in self._queue):
            return False
        return not self._admit_from_boundary()

    def contradictions(self) -> tuple:
        """Detect ``(X, (not X))`` same-layer pairs in the current KB.

        Convenience wrapper around
        :class:`ein.inference.contradiction.ContradictionDetector`
        for callers holding a Saturator: it owns the KB reference, so
        they don't need to construct a detector themselves.

        Returns:
            Tuple of :class:`~ein.inference.contradiction.Contradiction`
            records, empty when the KB is consistent.
        """
        from .contradiction import ContradictionDetector
        return ContradictionDetector(self.kb).detect()

    # ── Internals ─────────────────────────────────────────────────

    def _binding_key(
        self, plan: JoinPlan, bindings: dict[str, Any],
    ) -> tuple[str, tuple[str, ...], frozenset]:
        """Canonical key identifying a (rule, activator, bindings) triple."""
        return (
            plan.rule_name,
            plan.activator_args,
            frozenset((k, _hashable(v)) for k, v in bindings.items()),
        )

    def _priority_for(self, plan: JoinPlan) -> int:
        rule = self.kb.rules.get(plan.rule_name)
        if rule is None or rule.priority is None:
            return _DEFAULT_PRIORITY
        return rule.priority

    # ── `__symmetric__` native mirror (kernel optimization) ───────────

    def _symmetric_rels(self) -> frozenset[str]:
        """Relations marked `(__symmetric__ R)`; cached, refreshed when the
        marker-fact count changes (a rule may derive `(__symmetric__ R)`)."""
        facts = self.kb._facts_by_relation.get(SYMMETRIC, ())
        if self._sym_rels is None or len(facts) != self._sym_n:
            self._sym_rels = frozenset(f.args[0] for f in facts if f.args)
            self._sym_n = len(facts)
        return self._sym_rels

    def _enqueue_mirror_sources(self, facts: tuple[Fact, ...]) -> None:
        """Queue any marked-relation fact in `facts` for arg-swap mirroring."""
        sym = self._symmetric_rels()
        if not sym:
            return
        for f in facts:
            if f.relation_name in sym:
                self._mirror_queue.append(f)

    def _has_pending_mirror(self) -> bool:
        """True iff some marked-relation edge `(R a b)` (a≠b) lacks its mirror
        `(R b a)` — non-mutating, so `is_stalled` can consult it. Cheap when no
        relation is marked (the common case)."""
        for r in self._symmetric_rels():
            for f in self.kb._facts_by_relation.get(r, ()):
                if (len(f.args) == 2 and f.args[0] != f.args[1]
                        and self.kb._fact_by_id(r, (f.args[1], f.args[0])) is None):
                    return True
        return False

    def _next_mirror_firing(self) -> Firing | None:
        """One native `(__symmetric__ R)` mirror, or None.

        A marked R's extension is closed under arg-swap: rather than compiling
        the stdlib `symmetric` rule and running `match.run` each pass, pop a
        source `(R a b)` and write `(R b a)` directly — skipping self-loops
        (a=b) and already-present mirrors. The mirror feeds the delta so rules
        re-enqueue against it. This is the kernel optimization — it skips the
        JoinPlan / matcher the rule pays per mirror.

        Cold-seeds from the existing marked-relation extents on first call;
        thereafter `_enqueue_mirror_sources` queues each productive firing's
        derived marked facts.
        """
        if not self._mirror_seeded:
            self._mirror_seeded = True
            for r in self._symmetric_rels():
                self._mirror_queue.extend(self.kb._facts_by_relation.get(r, ()))
        sym = self._symmetric_rels()
        while self._mirror_queue:
            src = self._mirror_queue.pop()
            if src.relation_name not in sym or len(src.args) != 2:
                continue
            a, b = src.args
            if a == b:
                continue
            mirror = self.kb._fact_by_id(src.relation_name, (b, a))
            if mirror is not None:
                # S1.21.7 — the mirror already exists (typically because the
                # stdlib `symmetric` rule, or the mirror of the mirror, got
                # there first). Arg-swap is a real second derivation of it, so
                # record it rather than dropping it. This is what makes the
                # justification graph genuinely cyclic — `(R a b)` and
                # `(R b a)` justify each other — which the explanation search
                # handles by construction (a least fixpoint from the sources
                # up never grounds a fact in itself).
                if self._record_alternatives and self.kb.accepts_justification(
                        mirror, 1):
                    self.kb.record_justification(mirror, Provenance.from_rule(
                        rule=SYMMETRIC,
                        premises_raw=((src.relation_name, src.args),),
                        bindings=(),
                    ))
                continue
            prov = Provenance.from_rule(
                rule=SYMMETRIC,
                premises_raw=((src.relation_name, src.args),),
                bindings=(),
            )
            stored = self.kb.add_and_index_fact(Fact(
                relation_name=src.relation_name, args=(b, a),
                provenance=prov,
            ))
            self._needs_enqueue = True
            if self._delta_facts is None:
                self._delta_facts = [stored]
            else:
                self._delta_facts.append(stored)
            self._enqueue_mirror_sources((stored,))
            return Firing(
                rule=SYMMETRIC,
                activator=(src.relation_name,),
                bindings={"a": a, "b": b},
                derived=(stored,),
                premises=(src,),
            )
        return None

    def _enqueue_pass(self, delta_facts: list[Fact] | None = None) -> None:
        """Enqueue any not-yet-seen binding.

        S1.8.B2v — **delta-driven, semi-naive (D5)**. ``delta_facts is None``
        ⇒ a FULL pass: full-match every plan (the cold first pass;
        ``is_stalled``; any caller that wrote facts outside ``step()``).
        Otherwise a DELTA pass over the facts the last firing derived:

        - a plan never matched yet (e.g. a reflective rule's freshly-compiled
          plan) gets one FULL match — it may match existing facts, not just
          the delta;
        - otherwise the plan is SEEDED at each delta fact
          (:func:`match.run_seeded`) — the matcher iterates the one new fact
          at its premise instead of re-scanning the relation's whole extent
          (the win the D5 caller-split sized: 91% of matcher output was
          re-discovery a full re-match would re-compute).

        ``compile_all`` first so derived activators get plans (reflective
        rule-implication, S1.8.A9; pinned by ``test_reflective_rule.py``).
        """
        self.engine.compile_all()
        cache = self.engine.cache
        if delta_facts is None:
            for plan in cache.values():
                self._matched_plan_ids.add(id(plan))
                self._full_match(plan)
            return
        pos_index = self._indexes()
        full_done: set[int] = set()
        # Never-matched plans (reflective): one full match each.
        for plan in cache.values():
            if id(plan) not in self._matched_plan_ids:
                self._matched_plan_ids.add(id(plan))
                full_done.add(id(plan))
                self._full_match(plan)
        # Positive-premise plans: seed each delta fact (semi-naive).
        for fact in delta_facts:
            for plan in pos_index.get(fact.relation_name, ()):
                if id(plan) not in full_done:
                    self._seed_match(plan, fact)

    def _indexes(self) -> dict[str, list[JoinPlan]]:
        """relation → plans index, rebuilt when the compile cache grows.

        Keys a plan's top-level Scan/Join relations — the seedable ones.
        S1.21.8 dropped the companion `absent` index: guards no longer
        participate in matching, so no relation can force a full re-match.
        Cheap (plan-count * premise-count)."""
        cache = self.engine.cache
        if self._pos_index is None or self._index_n != len(cache):
            pos: dict[str, list[JoinPlan]] = {}
            for plan in cache.values():
                for rel in _positive_relations(plan):
                    pos.setdefault(rel, []).append(plan)
            self._pos_index = pos
            self._index_n = len(cache)
        return self._pos_index

    def _full_match(self, plan: JoinPlan) -> None:
        """Full re-match of a plan (cold pass / new plan)."""
        priority = self._priority_for(plan)
        for bindings, premises, guards in match.run_guarded(plan, self.kb):
            self._enqueue_binding(plan, bindings, premises, priority, guards)

    def _seed_match(self, plan: JoinPlan, fact: Fact) -> None:
        """Semi-naive seed (D5): enqueue plan matches in which ``fact`` plays
        a positive premise."""
        priority = self._priority_for(plan)
        for bindings, premises, guards in match.run_seeded_guarded(
                plan, fact, self.kb):
            self._enqueue_binding(plan, bindings, premises, priority, guards)

    def _enqueue_binding(
        self,
        plan: JoinPlan,
        bindings: dict[str, Any],
        premises: tuple[Fact, ...],
        priority: int,
        guards: tuple[NafGuard, ...] = (),
    ) -> None:
        """Dedup a (plan, bindings) match and route it to queue or park.

        S1.21.8 — a match whose disjunct carries `(absent …)` guards does NOT
        enter the firing queue. It is *parked* for the boundary, because the
        closure must be allowed to reach a fixpoint before any negation is
        consulted; enqueuing it here would be asking "is P absent?" of a
        half-built world, which is the placement the stage removes.

        S1.22.0 — the dedup key is ``(binding_key, guards)``, not
        ``binding_key`` alone. :meth:`_binding_key` is
        ``(rule, activator, bindings)`` and carries no disjunct index, while
        ``naf_guards`` is **per disjunct**: two `(or …)` disjuncts that
        produce the same bindings under *different* guards collided, so only
        the first was ever admitted. A disjunct whose guards would pass was
        masked by one whose guards failed — and permanently, because a failing
        monotone guard retires its candidate. `(or …)` became order-dependent:
        reproduced with `(or (and (a ?x) (absent (block ?x)))
        (and (a ?x) (absent (other ?x))))`, which derived its conclusion in
        one disjunct order and not the other.

        Keying on the guards is exact rather than a proxy for the index: same
        bindings *and* same guards really is the same candidate, so genuine
        duplicates still collapse. ``engine._fired`` stays guard-free — once
        the conclusion is derived, every other disjunct for those bindings is
        redundant, which is what it already meant.
        """
        key = self._binding_key(plan, bindings)
        seen_key = (key, guards)
        if seen_key in self._seen:
            return
        if key in self.engine._fired:
            self._seen.add(seen_key)
            return
        self._seen.add(seen_key)
        self._tiebreaker += 1
        entry = (priority, self._tiebreaker, plan, dict(bindings), premises,
                 guards)
        heapq.heappush(self._parked if guards else self._queue, entry)

    def _record_alternative(
        self,
        plan: JoinPlan,
        bindings: dict[str, Any],
        premises: tuple[Fact, ...],
        existing: list[Fact | None],
        guards: tuple[NafGuard, ...] = (),
    ) -> None:
        """Record a redundant firing as an ALTERNATIVE justification (S1.21.7).

        One `Provenance` per application, shared by every conclusion — the
        same contract :func:`firing.fire` uses for a productive firing, so a
        justification reads identically whether it won the race or not, with
        one deliberate exception: ``bindings`` is left empty. This is the
        highest-volume path in the engine (~194k redundant firings on an
        exhaustive `zebra2`), stringifying every binding on it is the single
        most expensive part of recording, and ``bindings`` is display
        metadata that no consumer of an *alternative* reads — the explanation
        search reads ``premises_raw``, and the trace renders the primary.

        Built only once at least one conclusion can still take an
        alternative, so a hot fact whose list is already full of shorter
        derivations costs one O(1) check and nothing else.
        """
        n = len(premises)
        if not n:
            return
        targets = [
            f for f in existing
            if f is not None and self.kb.accepts_justification(f, n)
        ]
        if not targets:
            return
        prov = Provenance.from_rule(
            rule=plan.rule_name,
            premises_raw=tuple((p.relation_name, p.args) for p in premises),
            # S1.21.8 — an alternative justification carries its OWN negative
            # premises. Provenance is per derivation (S1.21.7), so a
            # re-derivation admitted through the boundary depends on what
            # *its* guards found missing; inheriting the primary's, or
            # recording none, would misreport the alternative's dependencies.
            absent_premises=(
                World(self.kb).negative_premises(guards, bindings)
                if guards else ()
            ),
        )
        for fact in targets:
            self.kb.record_justification(fact, prov)

    def _apply(
        self,
        plan: JoinPlan,
        bindings: dict[str, Any],
        premises: tuple[Fact, ...],
        key: tuple,
        guards: tuple[NafGuard, ...] = (),
    ) -> Firing | None:
        """Apply one popped candidate; build a Firing (productive or redundant).

        ``guards`` are the boundary conditions this candidate was admitted
        under. They are **not** re-evaluated here — that was the fire-time
        re-check S1.21.8 deleted — but they are recorded as the firing's
        *negative* premises, so a provenance walk can see what the conclusion
        depended on not holding.
        """
        if not plan.assert_templates:
            # Defensive — no assert clause, nothing to derive. Mark
            # fired so the queue doesn't churn on it forever.
            self.engine._fired.add(key)
            return None

        # Tentative-build every conclusion (S1.8.A13: a `:assert (and …)` has
        # several). Cheap — build_fact is a pure walk over each template.
        tentative = [build_fact(t, bindings) for t in plan.assert_templates]
        existing = [
            self.kb._fact_by_id(t.relation_name, t.args) for t in tentative
        ]

        # Mark this binding fired regardless of redundancy — the
        # matcher will keep producing it on every pass otherwise.
        self.engine._fired.add(key)

        if all(e is not None for e in existing):
            # Every conclusion already known → redundant (a partially-novel
            # multi-assert is productive and falls through to `fire`). The trace
            # renderer shows the firing was considered without re-asserting.
            #
            # S1.21.7 — this is the real dedup seam. Because we return here,
            # BEFORE `fire()`, a wholly-redundant re-derivation never builds a
            # Provenance and never reaches `store.add_and_index_fact` (an
            # exhaustive zebra2 solve: ~194k redundant firings vs 8 store-level
            # dedup hits). So the alternative justification has to be recorded
            # here — it is exactly the derivation `Fact.provenance`'s
            # first-derivation-wins rule would otherwise drop.
            if self._record_alternatives:
                self._record_alternative(
                    plan, bindings, premises, existing, guards)
            return Firing(
                rule=plan.rule_name,
                activator=tuple(str(a) for a in plan.activator_args),
                bindings=dict(bindings),
                derived=tuple(existing),
                premises=premises,
                redundant=True,
            )

        absent_premises = (
            World(self.kb).negative_premises(guards, bindings)
            if guards else ()
        )
        firing = fire(
            plan, bindings, premises, self.kb,
            absent_premises=absent_premises,
        )
        # `fire()` already calls kb.add_fact + _index_fact; the new
        # fact participates in subsequent enqueue passes.
        return firing


class SaturatorStepLimitError(RuntimeError):
    """Raised when Saturator.saturate(max_steps=N) exhausts the budget.

    Caller inspects the Saturator's ``kb`` and ``_last_firing`` to
    identify the runaway rule (typically a transitive closure or a
    join that overgenerates).
    """


__all__ = ["Saturator", "SaturatorStepLimitError"]
