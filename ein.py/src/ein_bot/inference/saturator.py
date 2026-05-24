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
from .compile import JoinPlan
from .engine import Engine, _hashable
from .firing import Firing, build_fact, fire

# Default priority for rules with no :priority kw-pair. Sits between
# eliminate (300) and hypothesis (900) — well-defined, but rarely
# expected to be hit because every shipping rule declares a priority.
_DEFAULT_PRIORITY = 1000


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

    # ── Public API ────────────────────────────────────────────────

    def step(self) -> Firing | None:
        """Pop the highest-priority candidate and apply it.

        Returns the resulting :class:`Firing`, or ``None`` when the
        queue is empty after a fresh enqueue pass.
        """
        if self._needs_enqueue:
            self._enqueue_pass()
            self._needs_enqueue = False
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
        """True iff no firing is currently available.

        P1.5's hypothesis loop calls this to decide when to fork:
        ``while not solved() and not is_stalled(): step()``. A fork
        injects a new hypothesis fact; the caller must clear the
        ``_needs_enqueue`` flag (or call this method, which forces a
        fresh pass) so subsequent matches are picked up.
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
        :class:`ein_bot.inference.contradiction.ContradictionDetector`.
        P1.5's hypothesis loop calls this between branch saturations
        to decide retraction; the Saturator owns the KB reference,
        so callers don't need to construct a detector themselves.

        Returns:
            Tuple of :class:`~ein_bot.inference.contradiction.Contradiction`
            records, empty when the KB is consistent.
        """
        from .contradiction import ContradictionDetector
        return ContradictionDetector(self.kb).detect()

    def solved(self) -> bool:
        """Query-mode predicate. M1 stub returning False.

        Plugged in by P1.5/P1.7 — will compile the `(query …)`
        block's `:goal` pattern and check whether the KB satisfies
        it. Until then, the saturator runs to fixed point regardless
        of the query block.
        """
        return False

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

    def _enqueue_pass(self) -> None:
        """Walk every plan; enqueue any binding not yet seen.

        Re-running this on every step is cheap on Zebra-scale and
        keeps the queue current after each fact write. The ``_seen``
        set absorbs duplicates from earlier passes in O(1).

        Refresh the compile cache first so any activator-shaped
        facts derived since the last pass (e.g. `(functional R)`
        produced by a `(bijective R)` expansion rule) get their
        plans built. `compile_all` is idempotent — already-cached
        (rule, activator) pairs early-return.
        """
        self.engine.compile_all()
        for plan in self.engine.cache.values():
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

    def _apply(
        self,
        plan: JoinPlan,
        bindings: dict[str, Any],
        premises: tuple[Fact, ...],
        key: tuple,
    ) -> Firing | None:
        """Apply one popped candidate; build a Firing (productive or redundant)."""
        if plan.assert_template is None:
            # Defensive — no assert clause, nothing to derive. Mark
            # fired so the queue doesn't churn on it forever.
            self.engine._fired.add(key)
            return None

        # Tentative-build: what would this firing derive? Cheap
        # because build_fact is a pure walk over the assert template.
        tentative = build_fact(plan.assert_template, bindings)
        existing = self.kb._fact_by_id(
            tentative.relation_name, tentative.args,
        )

        # Mark this binding fired regardless of redundancy — the
        # matcher will keep producing it on every pass otherwise.
        self.engine._fired.add(key)

        if existing is not None:
            # Conclusion already known. Record the firing as
            # redundant; the trace renderer will show the engine
            # considered it without re-asserting.
            return Firing(
                rule=plan.rule_name,
                activator=tuple(str(a) for a in plan.activator_args),
                bindings=dict(bindings),
                derived=existing,
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
