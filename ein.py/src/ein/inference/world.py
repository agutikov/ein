"""The closure/worlds boundary — S1.21.8.

A :class:`World` is the object `absent_semantics.md` describes and the
engine previously only implied: a **saturated** knowledge base together with
the commitment that produced it. Every negation-as-failure query goes through
it, and nothing else does.

Why this exists
---------------

`(absent P)` is not a ground atom. It is a *query against a world* — the
normative reading in
``docs/kernel/inference/absent_semantics.md`` is ``W ⊭ ∃x̄.Pθ``, where ``W``
is a saturated world. Before S1.21.8 the engine evaluated it three times, all
*inside* the closure and all against a transient mid-saturation KB: at
enqueue-time matching, again at fire time to close the enqueue/fire race, and
implicitly through the semi-naive enqueue's absent-flip full-match. The world
a guard actually queried was therefore "the KB as of this dequeue", which is
not a world at all — soundness rested on a monotone-growth argument plus
priority-band discipline (leak **L1** of the R6 seam census).

Now the closure is purely positive and runs to quiescence; guards are
evaluated **here**, against that fixpoint, and admissions re-enter the
closure. `W ⊭ ∃x̄.Pθ` becomes literal, and the priority-band discipline drops
from load-bearing to advisory.

What a World is *not*
---------------------

It is not a snapshot and it does not own the KB — it is a read-only view
taken at a quiescence point. Saturating past that point invalidates it; the
saturator builds a fresh one per round rather than mutating one. Guard
evaluation is read-only by construction (`_run_steps` never writes).
"""
from __future__ import annotations

from collections.abc import Iterable
from typing import Any

from ein.kb.entities import Fact
from ein.kb.provenance import FactId
from ein.kb.store import KnowledgeBase

from . import match
from .compile import Join, NafGuard, NestedPattern, Scan

# What a passing guard is recorded as: the relation queried and the argument
# pattern it was queried with, `None` marking a position left free. Hashable,
# so it can ride on a frozen `Provenance` (S1.21.8 negative provenance).
NafRef = tuple[str, tuple[object, ...]]


def project(bindings: dict[str, Any], scope: frozenset[str]) -> dict[str, Any]:
    """Restrict ``bindings`` to ``scope`` — the guard's original context.

    See :class:`~ein.inference.compile.NafGuard`: a guard lifted to the
    boundary must not silently gain the bindings of premises that followed
    it, or `(and (absent (P ?x)) (Q ?x))` would quietly become
    `(and (Q ?x) (absent (P ?x)))`.
    """
    if not scope:
        return {}
    return {k: v for k, v in bindings.items() if k in scope}


class World:
    """A saturated KB plus its commitment — the boundary NAF is evaluated on.

    ``commitment`` is the hypothesis set this world assumes (empty at root).
    It is carried because a world is *branch-relative*: `absent(P)` means
    "P does not follow from the givens **and this commitment**", which is why
    a guard verdict taken in one branch says nothing about another.
    """

    __slots__ = ("commitment", "kb")

    def __init__(
        self, kb: KnowledgeBase, commitment: tuple[FactId, ...] = (),
    ) -> None:
        self.kb = kb
        self.commitment = commitment

    # ── The two boundary questions ────────────────────────────────

    def holds(
        self, steps: tuple[object, ...], bindings: dict[str, Any] | None = None,
    ) -> bool:
        """``W ⊨ ∃x̄. steps θ`` — does some match of ``steps`` exist here?"""
        for _ in match.run_steps(steps, dict(bindings or {}), (), self.kb):
            return True
        return False

    def absent(self, guard: NafGuard, bindings: dict[str, Any]) -> bool:
        """``W ⊭ ∃x̄. Pθ`` — negation as failure, at the boundary.

        The guard's sub-plan is run under the bindings *projected to its
        scope*; the guard passes iff it yields nothing. Nested guards inside
        the sub-plan (a ``forall``'s ``(absent (and G (absent B)))``) are
        evaluated as part of that one query, against this same world.
        """
        return not self.holds(guard.sub_steps, project(bindings, guard.scope))

    def admits(
        self, guards: Iterable[NafGuard], bindings: dict[str, Any],
    ) -> bool:
        """True iff every guard passes in this world."""
        return self.first_failing(guards, bindings) is None

    def first_failing(
        self, guards: Iterable[NafGuard], bindings: dict[str, Any],
    ) -> NafGuard | None:
        """The first guard that does *not* pass here, or ``None`` if all do.

        Callers need the guard, not just the verdict: an anti-monotone one
        (:attr:`~ein.inference.compile.NafGuard.monotone`) that fails has
        failed permanently — the KB only grows, so its query keeps matching —
        which lets the saturator retire the candidate instead of re-asking it
        at every future quiescence.
        """
        for g in guards:
            if not self.absent(g, bindings):
                return g
        return None

    # ── Negative provenance ───────────────────────────────────────

    def negative_premises(
        self, guards: Iterable[NafGuard], bindings: dict[str, Any],
    ) -> tuple[NafRef, ...]:
        """What a firing admitted here depended on *not* holding.

        The missing half of `Deps(Y)`, the union of `PositiveDeps(Y)` and `NegativeDeps(Y)`
        (REVIEW_M1-01 §2). Positive provenance records the facts a firing
        consumed; without this, a firing's dependence on an absence is
        invisible to every provenance walk — the root cause behind the
        retired `unconditional_facts` extraction (corollary C2) and the
        NAF-unsoundness of deletion-based core minimisation (C3).

        One :data:`NafRef` per relation pattern the guard queried, with the
        guard's projected bindings substituted in and ``None`` left where the
        query ranged free. Nested guards contribute their patterns too: the
        whole query is what had to fail.
        """
        out: list[NafRef] = []
        for g in guards:
            _collect_refs(g.sub_steps, project(bindings, g.scope), out)
        return tuple(dict.fromkeys(out))


def _collect_refs(
    steps: tuple[object, ...], bindings: dict[str, Any], out: list[NafRef],
) -> None:
    """Ground a guard sub-plan's relation patterns against ``bindings``."""
    for st in steps:
        if isinstance(st, (Scan, Join)):
            out.append((st.relation, tuple(
                _ground(slot, bindings) for slot in st.arg_slots)))
        elif hasattr(st, "sub_steps"):          # nested AbsentGuard
            _collect_refs(st.sub_steps, bindings, out)


def _ground(slot: object, bindings: dict[str, Any]) -> object:
    """A slot's value under ``bindings``; ``None`` when still free."""
    name = getattr(slot, "name", None)
    if name is not None and not isinstance(slot, NestedPattern):
        # `Var` (bound → its value, free → None) or `Atom` (a literal).
        if type(slot).__name__ == "Var":
            v = bindings.get(name)
            return v if isinstance(v, (str, int)) else None
        return name
    value = getattr(slot, "value", None)
    if value is not None:
        return value                            # Int literal
    if isinstance(slot, NestedPattern):
        return (slot.relation, tuple(
            _ground(s, bindings) for s in slot.arg_slots))
    return None


def root_world(kb: KnowledgeBase) -> World:
    """The commitment-free world — what root saturation quiesces to."""
    return World(kb)


def fact_ids(facts: Iterable[Fact]) -> tuple[FactId, ...]:
    """`(relation_name, args)` ids — for building a commitment tuple."""
    return tuple((f.relation_name, f.args) for f in facts)


__all__ = ["NafRef", "World", "fact_ids", "project", "root_world"]
