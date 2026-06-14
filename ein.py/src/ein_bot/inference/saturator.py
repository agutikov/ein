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


def _scan_relations(plan: JoinPlan) -> set[str]:
    """**Every** relation in a plan's premises — positive Scan/Join AND
    inside any (nested) ``AbsentGuard`` — the S1.8.B2v re-enqueue trigger
    set (incl. the A13 ``extra_match_plans`` disjuncts).

    A plan's match result can change only if a fact changes at one of these
    relations. Two ways a delta creates a *new* binding:

    - a **positive** Scan/Join premise gains a fact (the obvious case); and
    - a fact lands inside an ``AbsentGuard`` and flips it. A bare
      ``(absent P)`` only *blocks*, but a ``forall`` desugars to a NESTED
      absent ``(absent (and G (absent B)))`` — adding a ``B`` fact makes the
      inner absent fail, the outer absent **pass**, and so *enables* a
      firing (e.g. ``domain-elimination`` fires once the last
      ``(not (R a b_other))`` exists). So absent relations are triggers too,
      at **every nesting depth**. (Collecting only the top-level positive
      premises was the completeness bug that stalled the zebra2 lattice —
      eliminations stopped firing, hypotheses never pruned.)
    """
    rels: set[str] = set()

    def walk(steps: tuple) -> None:
        for step in steps:
            if isinstance(step, (Scan, Join)):
                rels.add(step.relation)
            elif isinstance(step, AbsentGuard):
                walk(step.sub_steps)

    walk(plan.steps)
    for extra in plan.extra_match_plans:
        walk(extra)
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
        # S1.8.B2v — incremental (delta-driven) enqueue. After the cold
        # full pass, an enqueue pass re-matches ONLY the plans whose
        # positive premises reference a relation in the last productive
        # firing's derived facts (the "delta"), plus any plan never yet
        # matched (e.g. a reflective rule's freshly-compiled plan).
        # `_delta_relations is None` ⇒ a FULL pass (cold start / is_stalled).
        self._delta_relations: set[str] | None = None
        self._matched_plan_ids: set[int] = set()
        # relation → plans (positive-premise index), rebuilt when the
        # compile cache grows; see `_relation_index`.
        self._rel_index: dict[str, list[JoinPlan]] | None = None
        self._rel_index_n: int = -1

    # ── Public API ────────────────────────────────────────────────

    def step(self) -> Firing | None:
        """Pop the highest-priority candidate and apply it.

        Returns the resulting :class:`Firing`, or ``None`` when the
        queue is empty after a fresh enqueue pass.
        """
        if self._needs_enqueue:
            self._enqueue_pass(self._delta_relations)
            self._needs_enqueue = False
            self._delta_relations = None
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
                # S1.8.B2v — the next enqueue pass need only re-match plans
                # whose positive premises reference a relation this firing
                # just derived (accumulated until that pass consumes it).
                rels = {d.relation_name for d in firing.derived}
                if self._delta_relations is None:
                    self._delta_relations = rels
                else:
                    self._delta_relations |= rels
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

    def _enqueue_pass(self, delta_relations: set[str] | None = None) -> None:
        """Enqueue any not-yet-seen binding.

        S1.8.B2v — **delta-driven**. ``delta_relations is None`` ⇒ a FULL
        pass over every plan (the cold first pass; ``is_stalled``; any
        caller that wrote facts outside ``step()``). Otherwise a DELTA
        pass: re-match only the plans whose positive premises reference a
        relation in ``delta_relations`` (via :meth:`_relation_index`), plus
        any plan never matched yet. Soundness: a new binding requires a new
        fact at a Scan/Join premise, so a plan none of whose positive-
        premise relations changed cannot gain one; ``AbsentGuard`` deltas
        only *block* (re-checked at fire time by
        :func:`match.absents_still_pass`). This turns the per-firing
        re-enqueue from a full-KB re-match into a handful of plans — the
        940-full-re-matches → ~run-count fix the S1.8.B2v measurement sized.

        Refresh the compile cache first so any activator-shaped facts
        derived since the last pass (e.g. ``(functional R)`` from a
        ``(bijective R)`` expansion) get their plans built; ``compile_all``
        is idempotent. This per-pass refresh IS **reflective rule-
        implication** (S1.8.A9 / F5 rung 2): a derived ``(symmetric foo)``
        becomes an activator for the generic rule next pass — and its
        freshly-compiled plan is force-matched here as a "never matched
        yet" plan regardless of the delta. Pinned by
        ``tests/inference/test_reflective_rule.py``.
        """
        self.engine.compile_all()
        cache = self.engine.cache
        # Plans never matched yet (e.g. just compiled for a derived
        # activator) always get one full match, regardless of the delta.
        targets: list[JoinPlan] = [
            p for p in cache.values() if id(p) not in self._matched_plan_ids
        ]
        if delta_relations is None:
            targets = list(cache.values())
        else:
            chosen = {id(p) for p in targets}
            index = self._relation_index()
            for rel in delta_relations:
                for plan in index.get(rel, ()):
                    if id(plan) not in chosen:
                        chosen.add(id(plan))
                        targets.append(plan)
        for plan in targets:
            self._matched_plan_ids.add(id(plan))
            priority = self._priority_for(plan)
            for bindings, premises in match.run(plan, self.kb):
                key = self._binding_key(plan, bindings)
                if key in self._seen:
                    continue
                if key in self.engine._fired:
                    self._seen.add(key)
                    continue
                self._seen.add(key)
                self._tiebreaker += 1
                heapq.heappush(
                    self._queue,
                    (priority, self._tiebreaker, plan, dict(bindings), premises),
                )

    def _relation_index(self) -> dict[str, list[JoinPlan]]:
        """relation → plans with that relation in a positive premise.

        Rebuilt when the compile cache grows (reflective rules add plans);
        cheap (about plan-count * premise-count). Used only by delta passes.
        """
        cache = self.engine.cache
        if self._rel_index is None or self._rel_index_n != len(cache):
            index: dict[str, list[JoinPlan]] = {}
            for plan in cache.values():
                for rel in _scan_relations(plan):
                    index.setdefault(rel, []).append(plan)
            self._rel_index = index
            self._rel_index_n = len(cache)
        return self._rel_index

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
