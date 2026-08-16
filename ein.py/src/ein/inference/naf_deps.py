"""Static NAF dependency map — S1.7.4.

Pure, observability-only analysis: which ``(absent …)`` guards watch a
**rule-derived** relation vs a **declared-only** one (a relation whose
extension is fixed by the puzzle's ``(facts)`` / ``(ontology)``).

Why it matters. **Re-grounded by S1.21.8.** The original rationale was a
soundness one: S1.5a.1 closed the enqueue-vs-fire NAF race with a fire-time
re-check, and this map told the author which of their rules leaned on it (or
on a strictly-lower deriving priority). That re-check is gone — `(absent …)`
is evaluated on the closure/world boundary against a positive fixpoint — so
derived-NAF is no longer a soundness question and priority no longer decides
what is derivable.

What the distinction still buys is **stratification**. NAF over a derived
relation is precisely the shape that can make a rule set non-stratified, and
that is the remaining hazard: the engine answers an unstratifiable program by
picking one model at boundary-admission order rather than reporting that
several exist. NAF over a declared-only relation cannot do that — its watched
extent is fixed by the puzzle. So this map is now the cheap static proxy for
"could this rule set have more than one answer?", the input a real
stratification checker would refine, and it still drives trace explanations
and becomes the soundness instrument if S1.7.7 ever de-hardcodes ``closed``.

**Completeness needs a saturated cache.** Most NAF-bearing rules in the
Zebra family (``adjacent-via-*``, ``typecheck-arg-*``, the elimination /
totality rules) are activated by *derived* facts that do not exist at
load (``(adjacent-via-fwd …)``, ``(total color-loc)``, …). Their
:class:`~ein.inference.compile.JoinPlan` is compiled only once the
saturator's enqueue pass has refreshed the cache (see
:meth:`ein.inference.saturator.Saturator._enqueue_pass`). So pass the
cache of an engine that has run its initial saturation —
:func:`ein.inference.monotonic.solver._phase1_root` is the single
once-per-solve site that does so.

**Negated NAF (Scope B).** ``forall`` / ``total`` / ``domain-elimination``
desugar to ``(absent (… (absent (not (R …)))))``. The literal head there
is ``"not"``, but the watched fact is the *derived* ``(not (R …))``
(produced by ``functional-negative`` / ``co-located-negative`` / …). So a
negated ref ``(R, negated=True)`` is classified derived iff some rule
asserts ``(not (R …))`` — the :func:`_negated_producible` dual of the
positive producible test.
"""
from __future__ import annotations

import warnings
from dataclasses import dataclass

from .compile import (
    JoinPlan,
    asserted_relation,
    naf_relation_refs,
    negated_relation,
)

CacheLike = dict[tuple[str, tuple[str, ...]], JoinPlan]


@dataclass(frozen=True)
class NafDep:
    """One ``(rule, activator)`` pair's NAF dependency classification.

    ``derived`` and ``declared_only`` are sorted tuples of relation
    labels — a negated watch ``(absent (not (R …)))`` is labelled
    ``"(not R)"`` to keep it distinct from the positive ``"R"``. Both
    tuples (and the record order from :func:`compute_naf_map`) are
    stable, so golden tests diff cleanly.
    """
    rule_name: str
    activator_args: tuple[str, ...]
    derived: tuple[str, ...]
    declared_only: tuple[str, ...]


class DerivedNafWarning(UserWarning):
    """A rule's ``(absent …)`` watches a rule-derived relation.

    **What this means changed with S1.21.8.** It used to say "this rule's
    soundness leans on the fire-time NAF re-evaluation (or on the deriving
    rule running at a strictly-lower priority)". That reading is retired
    along with the re-check: `(absent …)` is now evaluated on the
    closure/world boundary against a positive fixpoint, so a derived-NAF rule
    is sound whatever the priorities say.

    What it flags now is **stratification**. NAF over a derived relation is
    exactly the shape that can make a rule set non-stratified, and that is
    the remaining hazard: on a genuinely unstratifiable program (``p ← absent
    q; q ← absent p``) the engine still produces *an* answer, choosing one by
    boundary-admission order, rather than reporting that two models exist.
    Declared-only NAF cannot do that — its watched extent is fixed.

    Advisory, and off by default (``SolverConfig.warn_derived_naf``); a
    dedicated category so callers / the pytest ``filterwarnings=error`` suite
    can target it precisely. A real static stratification checker is the
    proper instrument and remains future work.
    """


def _label(rel: str, negated: bool) -> str:
    return f"(not {rel})" if negated else rel


def _producible(cache: CacheLike) -> frozenset[str]:
    """Relations some compiled plan positively asserts (``(R …)``)."""
    return frozenset(
        r for p in cache.values() if (r := asserted_relation(p)) is not None
    )


def _negated_producible(cache: CacheLike) -> frozenset[str]:
    """Relations ``R`` some compiled plan negates (``(not (R …))``)."""
    return frozenset(
        r for p in cache.values() if (r := negated_relation(p)) is not None
    )


def compute_naf_map(cache: CacheLike) -> list[NafDep]:
    """Static NAF dependency map over a compile ``cache``.

    One :class:`NafDep` per ``(rule, activator)`` plan that carries at
    least one ``AbsentGuard``. Each watched relation (recursively, both
    nesting levels) is classified derived vs declared-only against the
    *same* cache, so the producible set reflects exactly the activators
    that actually exist. Records are sorted by ``(rule_name,
    activator_args)``.
    """
    producible = _producible(cache)
    neg_producible = _negated_producible(cache)
    deps: list[NafDep] = []
    for plan in cache.values():
        refs = naf_relation_refs(plan)
        if not refs:
            continue
        derived: set[str] = set()
        declared: set[str] = set()
        for rel, negated in refs:
            pool = neg_producible if negated else producible
            bucket = derived if rel in pool else declared
            bucket.add(_label(rel, negated))
        deps.append(
            NafDep(
                rule_name=plan.rule_name,
                activator_args=plan.activator_args,
                derived=tuple(sorted(derived)),
                declared_only=tuple(sorted(declared)),
            )
        )
    deps.sort(key=lambda d: (d.rule_name, d.activator_args))
    return deps


def emit_derived_naf_warnings(cache: CacheLike) -> list[NafDep]:
    """Emit a :class:`DerivedNafWarning` per ``(rule, activator)`` with at
    least one derived-NAF dependency. Returns the flagged deps (handy for
    tests). No-op-but-returns-``[]`` when nothing is flagged.
    """
    flagged = [d for d in compute_naf_map(cache) if d.derived]
    for d in flagged:
        act = (
            f" [activator {' '.join(d.activator_args)}]"
            if d.activator_args else ""
        )
        warnings.warn(
            f"rule {d.rule_name!r}{act}: (absent …) watches rule-derived "
            f"relation(s) {', '.join(d.derived)} — the rule set may not be "
            f"stratified, in which case the engine reports one model rather "
            f"than the several that exist. Sound either way (the guard is "
            f"evaluated on the closure/world boundary, S1.21.8). See S1.7.4.",
            DerivedNafWarning,
            stacklevel=2,
        )
    return flagged


__all__ = [
    "DerivedNafWarning",
    "NafDep",
    "compute_naf_map",
    "emit_derived_naf_warnings",
]
