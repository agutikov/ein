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

from ein_bot.kb.entities import Fact
from ein_bot.kb.store import KnowledgeBase

from . import match
from .compile import AbsentGuard, Join, JoinPlan, Scan
from .engine import Engine, _hashable
from .firing import Firing, build_fact, fire

# Default priority for rules with no :priority kw-pair. Sits between
# eliminate (300) and hypothesis (900) — well-defined, but rarely
# expected to be hit because every shipping rule declares a priority.
_DEFAULT_PRIORITY = 1000


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


def _absent_relations(plan: JoinPlan) -> set[str]:
    """Relations appearing inside any (nested) ``AbsentGuard`` of a plan.

    A delta here can FLIP the absent — a ``forall`` desugars to a nested
    absent ``(absent (and G (absent B)))``, so adding a ``B`` fact makes the
    inner absent fail, the outer **pass**, and *enables* a firing (e.g.
    ``domain-elimination`` once the last ``(not (R a b_other))`` exists). But
    there is no positive premise to bind, so a plan whose delta relation lands
    here must **full-match**, not seed (S1.8.B2v D5) — treating it as seedable
    would re-introduce the D2 completeness bug (stalled eliminations →
    lattice blow-up). Note ``_positive_relations`` and ``_absent_relations``
    together are the full re-enqueue trigger set (a relation can be in both)."""
    rels: set[str] = set()

    def walk_absent(steps: tuple) -> None:
        for step in steps:
            if isinstance(step, (Scan, Join)):
                rels.add(step.relation)
            elif isinstance(step, AbsentGuard):
                walk_absent(step.sub_steps)

    def find_absents(steps: tuple) -> None:
        for step in steps:
            if isinstance(step, AbsentGuard):
                walk_absent(step.sub_steps)

    find_absents(plan.steps)
    for extra in plan.extra_match_plans:
        find_absents(extra)
    return rels


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
        # (rule_name, activator_args, hashable-bindings) -> already enqueued
        self._seen: set[tuple[str, tuple[str, ...], frozenset]] = set()
        # heap entries: (priority, tiebreaker, plan, bindings, premises)
        self._queue: list[tuple[int, int, JoinPlan, dict[str, Any], tuple[Fact, ...]]] = []
        self._tiebreaker = 0
        # Optimisation: enqueue_pass is needed only after a productive
        # firing (a new fact was written; new matches may exist) or
        # before the very first step.
        self._needs_enqueue: bool = True
        # Last yielded Firing — kept on the instance so a step-limit
        # raise can quote it without forcing every caller to track it.
        self._last_firing: Firing | None = None
        # Count of firings dropped at dequeue because an AbsentGuard
        # premise that passed at enqueue no longer holds against the
        # current KB (S1.5a.1 fire-time NAF re-eval).
        self.naf_dropped: int = 0
        # S1.8.B2v — incremental (delta-driven) enqueue. After the cold full
        # pass, an enqueue pass processes only the DELTA — the facts the last
        # productive firing derived: positive-premise plans are SEEDED at the
        # new fact (D5 semi-naive, `match.run_seeded`), absent-premise plans
        # (a `forall` that may flip) full-match, and any never-matched plan
        # (reflective) full-matches. `_delta_facts is None` ⇒ a FULL pass
        # (cold start / is_stalled).
        self._delta_facts: list[Fact] | None = None
        self._matched_plan_ids: set[int] = set()
        # relation → plans, split by premise polarity (positive Scan/Join vs
        # inside an AbsentGuard); rebuilt when the compile cache grows. See
        # `_indexes`.
        self._pos_index: dict[str, list[JoinPlan]] | None = None
        self._abs_index: dict[str, list[JoinPlan]] | None = None
        self._index_n: int = -1

    # ── Public API ────────────────────────────────────────────────

    def step(self) -> Firing | None:
        """Pop the highest-priority candidate and apply it.

        Returns the resulting :class:`Firing`, or ``None`` when the
        queue is empty after a fresh enqueue pass.
        """
        if self._needs_enqueue:
            self._enqueue_pass(self._delta_facts)
            self._needs_enqueue = False
            self._delta_facts = None
        while self._queue:
            _priority, _tb, plan, bindings, premises = heapq.heappop(self._queue)
            key = self._binding_key(plan, bindings)

            # Engine._fired is the canonical "already fired" record —
            # we skip if either Saturator's own loop or a prior
            # Engine.step() (from the same Engine instance) has fired
            # this binding.
            if key in self.engine._fired:
                continue

            firing = self._apply(plan, bindings, premises, key)
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
            return firing
        return None

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
        """True iff no firing is currently available (the queue holds
        no unfired binding).

        Forces a fresh enqueue pass first — callers may have written
        facts directly to the KB outside ``step()``'s flow — so any
        newly-matchable binding is picked up before the stalled check.
        """
        # Always force a fresh pass — callers may have written facts
        # directly to the KB outside of step()'s flow (e.g. P1.5).
        self._enqueue_pass()
        self._needs_enqueue = False
        # The queue may contain entries whose bindings have already
        # been fired — filter them out before claiming stalled.
        return not any(
            self._binding_key(p, b) not in self.engine._fired
            for _pr, _tb, p, b, _pr_facts in self._queue
        )

    def contradictions(self) -> tuple:
        """Detect ``(X, (not X))`` same-layer pairs in the current KB.

        Convenience wrapper around
        :class:`ein_bot.inference.contradiction.ContradictionDetector`
        for callers holding a Saturator: it owns the KB reference, so
        they don't need to construct a detector themselves.

        Returns:
            Tuple of :class:`~ein_bot.inference.contradiction.Contradiction`
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

    def _enqueue_pass(self, delta_facts: list[Fact] | None = None) -> None:
        """Enqueue any not-yet-seen binding.

        S1.8.B2v — **delta-driven, semi-naive (D5)**. ``delta_facts is None``
        ⇒ a FULL pass: full-match every plan (the cold first pass;
        ``is_stalled``; any caller that wrote facts outside ``step()``).
        Otherwise a DELTA pass over the facts the last firing derived:

        - a plan never matched yet (e.g. a reflective rule's freshly-compiled
          plan) gets one FULL match — it may match existing facts, not just
          the delta;
        - a plan whose delta relation lands inside an ``AbsentGuard`` FULL-
          matches — a ``forall`` may have flipped and there is no positive
          premise to seed (the D2 completeness case);
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
        pos_index, abs_index = self._indexes()
        full_done: set[int] = set()
        # Never-matched plans (reflective): one full match each.
        for plan in cache.values():
            if id(plan) not in self._matched_plan_ids:
                self._matched_plan_ids.add(id(plan))
                full_done.add(id(plan))
                self._full_match(plan)
        # Absent-premise plans for any delta relation: full-match once
        # (a forall may have flipped — not seedable).
        for rel in {f.relation_name for f in delta_facts}:
            for plan in abs_index.get(rel, ()):
                if id(plan) not in full_done:
                    full_done.add(id(plan))
                    self._full_match(plan)
        # Positive-premise plans: seed each delta fact (semi-naive).
        for fact in delta_facts:
            for plan in pos_index.get(fact.relation_name, ()):
                if id(plan) not in full_done:
                    self._seed_match(plan, fact)

    def _indexes(
        self,
    ) -> tuple[dict[str, list[JoinPlan]], dict[str, list[JoinPlan]]]:
        """(positive, absent) relation → plans indexes, rebuilt when the
        compile cache grows. ``positive`` keys a plan's top-level Scan/Join
        relations (seedable); ``absent`` keys relations inside its
        AbsentGuards (must full-match). Cheap (plan-count * premise-count)."""
        cache = self.engine.cache
        if self._pos_index is None or self._index_n != len(cache):
            pos: dict[str, list[JoinPlan]] = {}
            absent: dict[str, list[JoinPlan]] = {}
            for plan in cache.values():
                for rel in _positive_relations(plan):
                    pos.setdefault(rel, []).append(plan)
                for rel in _absent_relations(plan):
                    absent.setdefault(rel, []).append(plan)
            self._pos_index = pos
            self._abs_index = absent
            self._index_n = len(cache)
        return self._pos_index, self._abs_index

    def _full_match(self, plan: JoinPlan) -> None:
        """Full re-match of a plan (cold pass / new plan / absent-flip)."""
        priority = self._priority_for(plan)
        for bindings, premises in match.run(plan, self.kb):
            self._enqueue_binding(plan, bindings, premises, priority)

    def _seed_match(self, plan: JoinPlan, fact: Fact) -> None:
        """Semi-naive seed (D5): enqueue plan matches in which ``fact`` plays
        a positive premise."""
        priority = self._priority_for(plan)
        for bindings, premises in match.run_seeded(plan, fact, self.kb):
            self._enqueue_binding(plan, bindings, premises, priority)

    def _enqueue_binding(
        self,
        plan: JoinPlan,
        bindings: dict[str, Any],
        premises: tuple[Fact, ...],
        priority: int,
    ) -> None:
        """Dedup a (plan, bindings) match against ``_seen`` / ``_fired`` and
        push it onto the priority queue if fresh."""
        key = self._binding_key(plan, bindings)
        if key in self._seen:
            return
        if key in self.engine._fired:
            self._seen.add(key)
            return
        self._seen.add(key)
        self._tiebreaker += 1
        heapq.heappush(
            self._queue,
            (priority, self._tiebreaker, plan, dict(bindings), premises),
        )

    def _apply(
        self,
        plan: JoinPlan,
        bindings: dict[str, Any],
        premises: tuple[Fact, ...],
        key: tuple,
    ) -> Firing | None:
        """Apply one popped candidate; build a Firing (productive or redundant)."""
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
            return Firing(
                rule=plan.rule_name,
                activator=tuple(str(a) for a in plan.activator_args),
                bindings=dict(bindings),
                derived=tuple(existing),
                premises=premises,
                redundant=True,
            )

        # S1.5a.1 fire-time NAF re-evaluation. An AbsentGuard premise
        # that passed at enqueue time may have been invalidated by a
        # rule that derived the watched fact in the interim. Skip the
        # firing in that case — the binding stays in `_fired` so the
        # queue doesn't churn on it.
        if not match.absents_still_pass(plan, bindings, self.kb):
            self.naf_dropped += 1
            return None

        firing = fire(plan, bindings, premises, self.kb)
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
