"""Runtime matcher — S1.3.1 T1.3.1.4.

Executes a :class:`JoinPlan` against a :class:`KnowledgeBase` and
yields ``(bindings, premises)`` tuples — one per successful match.

``bindings`` is a ``dict[str, str | int | Fact]`` mapping each
variable name to its bound value. ``premises`` is the tuple of
:class:`Fact` instances the Scan/Join steps consumed, in the order
they were consumed. The firing module reads both to build the
derived :class:`Fact` and its :class:`Provenance`.

Unification (``_bind_args``) is recursive:

- Atomic slot vs atomic arg — equality on the resolved literal.
- ``Var`` slot — bind on first encounter; on subsequent encounters,
  must match the existing binding.
- ``NestedPattern`` slot vs ``Fact`` arg (Q40 Option A) — relation
  names equal AND args unify pointwise (recursively).
"""
from __future__ import annotations

from collections.abc import Iterator
from typing import Any

from ein_bot.ir.types import Atom, Int, Var
from ein_bot.kb.entities import Fact
from ein_bot.kb.store import KnowledgeBase

from . import predicates
from .compile import (
    Guard,
    Join,
    JoinPlan,
    NegativeGuard,
    NestedPattern,
    Scan,
)

# ── Unification ────────────────────────────────────────────────────


def _bind_arg(
    slot: object,
    arg: Any,
    bindings: dict[str, Any],
) -> dict[str, Any] | None:
    """Unify a slot against a fact argument under current bindings.

    Returns the (possibly extended) bindings dict on success, or
    None on failure. Always returns a new dict on success to keep
    callers safe from aliasing.
    """
    if isinstance(slot, Var):
        if slot.name in bindings:
            return bindings if bindings[slot.name] == arg else None
        return {**bindings, slot.name: arg}
    if isinstance(slot, Atom):
        return bindings if slot.name == arg else None
    if isinstance(slot, Int):
        return bindings if slot.value == arg else None
    if isinstance(slot, NestedPattern):
        if not isinstance(arg, Fact):
            return None
        if arg.relation_name != slot.relation:
            return None
        if len(arg.args) != len(slot.arg_slots):
            return None
        b: dict[str, Any] | None = bindings
        for s, a in zip(slot.arg_slots, arg.args, strict=True):
            b = _bind_arg(s, a, b)
            if b is None:
                return None
        return b
    # Unknown slot type - treat as opaque literal compared by equality.
    return bindings if slot == arg else None


def _bind_args(
    slots: tuple[object, ...],
    args: tuple[Any, ...],
    bindings: dict[str, Any],
) -> dict[str, Any] | None:
    """Unify a tuple of slots against a tuple of args, in order."""
    if len(slots) != len(args):
        return None
    b: dict[str, Any] | None = bindings
    for s, a in zip(slots, args, strict=True):
        b = _bind_arg(s, a, b)
        if b is None:
            return None
    return b


# ── Plan execution ─────────────────────────────────────────────────


def _run_steps(
    steps: tuple[object, ...],
    bindings: dict[str, Any],
    premises: tuple[Fact, ...],
    kb: KnowledgeBase,
) -> Iterator[tuple[dict[str, Any], tuple[Fact, ...]]]:
    """Recursive driver. Yields (bindings, premises) on every success."""
    if not steps:
        yield bindings, premises
        return
    step, *rest_list = steps
    rest = tuple(rest_list)

    if isinstance(step, (Scan, Join)):
        for fact in kb._facts_by_relation.get(step.relation, ()):
            new_b = _bind_args(step.arg_slots, fact.args, bindings)
            if new_b is not None:
                yield from _run_steps(rest, new_b, (*premises, fact), kb)
        return

    if isinstance(step, Guard):
        fn = predicates.get(step.predicate)
        if fn is None:
            return
        if fn(bindings, step.args):
            yield from _run_steps(rest, bindings, premises, kb)
        return

    if isinstance(step, NegativeGuard):
        # Negation-as-failure: parent continues iff sub-plan yields zero.
        any_match = False
        for _ in _run_steps(step.sub_steps, bindings, premises, kb):
            any_match = True
            break
        if not any_match:
            yield from _run_steps(rest, bindings, premises, kb)
        return

    # Unknown step type — skip (defensive).
    yield from _run_steps(rest, bindings, premises, kb)


def run(
    plan: JoinPlan,
    kb: KnowledgeBase,
) -> Iterator[tuple[dict[str, Any], tuple[Fact, ...]]]:
    """Execute `plan` against `kb`. Yields one (bindings, premises) per match.

    The seeded bindings from the activator binding are merged into
    every emitted result so the asserter has uniform access to all
    bound names (rule params + body vars).
    """
    seed: dict[str, Any] = dict(plan.bindings_seed)
    yield from _run_steps(plan.steps, seed, (), kb)


__all__ = ["run"]
