"""Inference engine driver — S1.3.1 T1.3.1.7.

``Engine`` orchestrates compile + match + fire. M1 ships the
*per-(rule, activator) compile cache* and a ``step()`` that runs one
firing; the saturation loop with banded priorities (Q41) lands in
S1.3.3.

The cache key is ``(rule_name, activator_args)`` where
``activator_args`` is the activator fact's arguments. Activator-less
rules (parameter-less, e.g. ``type-exclusivity``) use the key
``(rule_name, ())``.
"""
from __future__ import annotations

from collections.abc import Iterator
from typing import Any

from ein_bot.kb.entities import Fact, Rule
from ein_bot.kb.store import KnowledgeBase

from . import match
from .compile import JoinPlan, compile_rule
from .firing import Firing, fire

CacheKey = tuple[str, tuple[str, ...]]


class Engine:
    """Inference engine attached to a :class:`KnowledgeBase`.

    Compile is lazy: :meth:`compile_all` populates the cache up
    front (called once after KB load); :meth:`compile_for` compiles
    on demand for a single (rule, activator) pair.
    """

    def __init__(self, kb: KnowledgeBase) -> None:
        self.kb = kb
        self._cache: dict[CacheKey, JoinPlan] = {}
        # Track which (rule, activator, bindings-hash) combinations
        # have already fired; the saturation loop reads this to avoid
        # re-firing the same firing endlessly. Bindings-hash is the
        # frozenset of binding items so dict order doesn't matter.
        self._fired: set[tuple[str, tuple[str, ...], frozenset]] = set()

    # ── Compile ───────────────────────────────────────────────────

    def _activators_for(self, rule: Rule) -> tuple[Fact | None, ...]:
        """Activator facts authorising `rule` to apply.

        - Parameter-less rules (``rule.params == ()``) have *one*
          implicit activator — None. They apply once over the KB.
        - Parameterised rules consult ``rule.applications`` (the
          property-application facts whose head matches the rule's
          name).
        """
        if not rule.params:
            return (None,)
        return tuple(rule.applications)

    def compile_for(
        self, rule: Rule, activator: Fact | None,
    ) -> JoinPlan:
        """Compile (and cache) one (rule, activator) pair."""
        key: CacheKey = (
            rule.name,
            tuple(str(a) for a in (activator.args if activator else ())),
        )
        if key in self._cache:
            return self._cache[key]
        plan = compile_rule(rule, activator)
        self._cache[key] = plan
        return plan

    def compile_all(self) -> None:
        """Walk ``kb.rules`` x ``activators``; cache one JoinPlan per pair."""
        for rule in self.kb.rules.values():
            for activator in self._activators_for(rule):
                self.compile_for(rule, activator)

    @property
    def cache(self) -> dict[CacheKey, JoinPlan]:
        """The compile cache — exposed for tests + tracing."""
        return self._cache

    # ── Firing ────────────────────────────────────────────────────

    def _iter_pending(self) -> Iterator[tuple[JoinPlan, dict[str, Any], tuple[Fact, ...]]]:
        """Yield every match that hasn't fired yet (any rule).

        Each match is the matcher's (bindings, premises) plus the
        plan it came from. The saturation loop in S1.3.3 will turn
        this into a banded-priority queue; for S1.3.1 we just yield
        in insertion order of `self._cache`.
        """
        for plan in self._cache.values():
            for bindings, premises in match.run(plan, self.kb):
                key = (
                    plan.rule_name,
                    plan.activator_args,
                    frozenset((k, _hashable(v)) for k, v in bindings.items()),
                )
                if key in self._fired:
                    continue
                yield plan, bindings, premises

    def step(self) -> Firing | None:
        """Run one firing — the first match found across all plans.

        Returns the :class:`Firing` record, or None when no plan
        produces a new (unfired) match. The full saturation loop
        with banded priorities lives in S1.3.3.
        """
        for plan, bindings, premises in self._iter_pending():
            firing = fire(plan, bindings, premises, self.kb)
            if firing is None:
                continue
            # Mark this binding fired so step() makes progress.
            key = (
                plan.rule_name,
                plan.activator_args,
                frozenset((k, _hashable(v)) for k, v in bindings.items()),
            )
            self._fired.add(key)
            return firing
        return None

    def saturate(self, max_steps: int = 10_000) -> Iterator[Firing]:
        """Run firings until no plan produces a new match.

        Bounded by ``max_steps`` as a runaway-saturation guard.
        Yields each firing in the order step() produced it. The
        priority-banded version lands in S1.3.3.
        """
        for _ in range(max_steps):
            f = self.step()
            if f is None:
                return
            yield f


def _hashable(v: Any) -> Any:
    """Make a value hashable for the ``_fired`` set.

    Facts are nominally hashable (frozen dataclass on
    (relation_name, args)); tuples/strings/ints already are; only
    pathological values need conversion. This is a defensive shim.
    """
    if isinstance(v, list):
        return tuple(_hashable(x) for x in v)
    if isinstance(v, dict):
        return frozenset((k, _hashable(val)) for k, val in v.items())
    return v


__all__ = ["Engine"]
