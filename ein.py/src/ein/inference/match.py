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

from ein.ir.types import Atom, Int, Var
from ein.kb.entities import Fact
from ein.kb.store import KnowledgeBase

from . import predicates
from .compile import (
    AbsentGuard,
    Guard,
    Join,
    JoinPlan,
    NafGuard,
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


def _candidates(
    step: Scan | Join,
    bindings: dict[str, Any],
    kb: KnowledgeBase,
) -> tuple[Fact, ...]:
    """Facts to try for a Scan/Join step — narrowed by the first bound slot.

    Consults the participation index ``kb._facts_by_rel_slot_val`` keyed on
    the FIRST slot whose value is known: a constant ``Atom`` / ``Int``, or a
    ``Var`` already in ``bindings`` bound to an atomic (str/int) value. The
    returned bucket is a **subset** of the full relation extent (never more
    work) and a **superset** of the facts that match at that slot — the
    caller's :func:`_bind_args` re-checks *every* slot, so the narrowing is
    behaviour-preserving (no false positives, no missed matches). The index
    mirrors :func:`_bind_arg`'s raw ``==`` (it does **not** apply eq-class
    resolution — neither does the unifier), so the two cannot drift.

    Falls back to the full ``kb._facts_by_relation`` extent when no slot is
    bound to an atomic value (the unavoidable base Scan, or a slot bound to a
    nested ``Fact`` / a ``NestedPattern`` slot — neither is keyed).

    S1.8.B-idx (2026-06-14) — the fix for the 60 M ``_bind_args`` calls the
    P1.8a baseline profile attributed to the per-step relation-extent rescan.
    """
    for i, slot in enumerate(step.arg_slots):
        if isinstance(slot, Atom):
            v: Any = slot.name
        elif isinstance(slot, Int):
            v = slot.value
        elif isinstance(slot, Var) and slot.name in bindings:
            v = bindings[slot.name]
            if type(v) is not str and type(v) is not int:
                continue          # nested-Fact binding — not keyed
        else:
            continue              # unbound Var, or NestedPattern slot
        return kb._facts_by_rel_slot_val.get((step.relation, i, v), ())
    return kb._facts_by_relation.get(step.relation, ())


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
        for fact in _candidates(step, bindings, kb):
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

    if isinstance(step, AbsentGuard):
        # Negation-as-failure: parent continues iff sub-plan yields zero.
        #
        # S1.21.8 — this arm no longer fires for a *closure* plan. Top-level
        # guards are lifted out at compile time (`compile.split_naf`) and
        # evaluated on the boundary (`world.World.absent`), so `plan.steps`
        # is purely positive. What still reaches here is a guard **nested
        # inside another guard's sub-plan** — what a `forall` desugars to,
        # `(absent (and G (absent B)))` — which is part of the negative query,
        # not of the closure, and is evaluated as one unit against one world.
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

    S1.8.A13: a rule whose ``:match`` is a top-level ``(or …)`` carries its
    extra disjuncts in ``plan.extra_match_plans``; each runs from a fresh seed,
    so every caller (saturator, lookahead, engine) sees all disjuncts' matches
    without any rule-split. Single-``:match`` rules have no extras (one pass).
    """
    yield from _run_steps(plan.steps, dict(plan.bindings_seed), (), kb)
    for extra_steps in plan.extra_match_plans:
        yield from _run_steps(extra_steps, dict(plan.bindings_seed), (), kb)


def run_guarded(
    plan: JoinPlan,
    kb: KnowledgeBase,
) -> Iterator[tuple[dict[str, Any], tuple[Fact, ...], tuple[NafGuard, ...]]]:
    """Like :func:`run`, but tags each match with **its disjunct's** guards.

    S1.21.8. `run` flattens every disjunct into one stream, which is fine
    while the guards live inside the steps — and wrong once they are lifted
    out, because the caller then has no way to tell which disjunct produced a
    match and therefore which guards must hold for it. Pairing them here is
    what closes D5 structurally rather than by remembering to walk one more
    tuple.
    """
    for steps, guards in plan.disjuncts():
        for bindings, premises in _run_steps(
                steps, dict(plan.bindings_seed), (), kb):
            yield bindings, premises, guards


def run_seeded_guarded(
    plan: JoinPlan,
    fact: Fact,
    kb: KnowledgeBase,
) -> Iterator[tuple[dict[str, Any], tuple[Fact, ...], tuple[NafGuard, ...]]]:
    """:func:`run_seeded`'s guard-tagging twin — see :func:`run_guarded`."""
    for steps, guards in plan.disjuncts():
        for bindings, premises in _seed_steps(
                steps, plan.bindings_seed, fact, kb):
            yield bindings, premises, guards


def _seed_steps(
    steps: tuple[object, ...],
    bindings_seed: dict[str, Any],
    fact: Fact,
    kb: KnowledgeBase,
) -> Iterator[tuple[dict[str, Any], tuple[Fact, ...]]]:
    """Yield matches of ``steps`` in which ``fact`` satisfies one of its
    positive Scan/Join premises (S1.8.B2v D5 semi-naive).

    For each top-level Scan/Join on ``fact``'s relation, bind that step to
    ``fact`` and run the *remaining* steps under those bindings — iterating
    the one new fact at that premise instead of re-scanning the relation's
    whole extent. A relation appearing in several steps (e.g. transitive
    ``(R ?a ?b) ∧ (R ?b ?c)``) is seeded at *each*, since ``fact`` may play
    any role. ``premises`` are rebuilt in the plan's original Scan/Join order
    (``fact`` at its step's position) so provenance is identical to
    :func:`run`.
    """
    for i, step in enumerate(steps):
        if not isinstance(step, (Scan, Join)) or step.relation != fact.relation_name:
            continue
        seed = _bind_args(step.arg_slots, fact.args, dict(bindings_seed))
        if seed is None:
            continue
        rest = steps[:i] + steps[i + 1:]
        prem_pos = sum(
            1 for s in steps[:i] if isinstance(s, (Scan, Join))
        )
        for bindings, rest_prem in _run_steps(rest, seed, (), kb):
            premises = (*rest_prem[:prem_pos], fact, *rest_prem[prem_pos:])
            yield bindings, premises


def run_seeded(
    plan: JoinPlan,
    fact: Fact,
    kb: KnowledgeBase,
) -> Iterator[tuple[dict[str, Any], tuple[Fact, ...]]]:
    """Semi-naive delta match (S1.8.B2v D5): every match of ``plan`` in which
    the newly-derived ``fact`` plays a positive premise. Seeds the primary
    ``plan.steps`` and each ``extra_match_plans`` disjunct. Caller restricts
    this to plans where ``fact``'s relation is a *positive* premise; plans
    with the relation only inside an ``AbsentGuard`` (a ``forall`` that may
    flip) must full-:func:`run` instead — seeding can't observe an absent
    flip."""
    yield from _seed_steps(plan.steps, plan.bindings_seed, fact, kb)
    for extra_steps in plan.extra_match_plans:
        yield from _seed_steps(extra_steps, plan.bindings_seed, fact, kb)


# S1.21.8 — `absents_still_pass` is **gone**, not bypassed.
#
# It was evaluation point E2, the fire-time re-check that closed the
# enqueue/fire NAF race for a queued executor (S1.5a.1, corollary C4). The
# race no longer exists: `(absent …)` premises are lifted out of the closure
# plan at compile time and evaluated on the boundary at positive quiescence
# (`world.World.absent`), so a guard is judged once, against a fixpoint, at
# the moment the firing is admitted — there is no window between the verdict
# and the firing for it to go stale, and `Saturator.naf_dropped` is
# structurally 0.
#
# Deleting it also retires its known gap (D5, P1.21 R4): it walked
# `plan.steps` only, so guards inside S1.8.A13 or-disjuncts got no fire-time
# protection at all. The boundary iterates `plan.disjuncts()`, which pairs
# every disjunct with its own guards.


def run_steps(
    steps: tuple[object, ...],
    bindings: dict[str, Any],
    premises: tuple[Fact, ...],
    kb: KnowledgeBase,
) -> Iterator[tuple[dict[str, Any], tuple[Fact, ...]]]:
    """Public entry to the step driver — the boundary's ``holds`` query.

    :mod:`ein.inference.world` runs guard sub-plans through this rather than
    reaching for the private ``_run_steps``.
    """
    return _run_steps(steps, bindings, premises, kb)


__all__ = [
    "run", "run_guarded", "run_seeded", "run_seeded_guarded", "run_steps",
]
