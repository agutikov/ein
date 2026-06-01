"""Static NAF dependency map — S1.7.4.

Pure, observability-only analysis: which ``(absent …)`` guards watch a
**rule-derived** relation vs a **declared-only** one (a relation whose
extension is fixed by the puzzle's ``(facts)`` / ``(ontology)``).

Why it matters. S1.5a.1 closed the enqueue-vs-fire NAF race at runtime —
:func:`ein_bot.inference.match.absents_still_pass` re-evaluates every
``AbsentGuard`` at fire time, so a rule whose NAF watches a derived
relation is *sound* regardless. What the engine doesn't tell the author
is **which** of their rules rely on that re-eval (or on a strictly-lower
deriving priority). An ``(absent (R …))`` over a declared-only ``R``
behaves identically under enqueue-time and fire-time semantics; one over
a derived ``R`` is only sound because of the re-eval. The distinction
drives trace explanations and priority tuning, and becomes the soundness
instrument if S1.7.7 ever de-hardcodes ``closed``.

**Completeness needs a saturated cache.** Most NAF-bearing rules in the
Zebra family (``adjacent-via-*``, ``typecheck-arg-*``, the elimination /
totality rules) are activated by *derived* facts that do not exist at
load (``(adjacent-via-fwd …)``, ``(total color-loc)``, …). Their
:class:`~ein_bot.inference.compile.JoinPlan` is compiled only once the
saturator's enqueue pass has refreshed the cache (see
:meth:`ein_bot.inference.saturator.Saturator._enqueue_pass`). So pass the
cache of an engine that has run its initial saturation —
:func:`ein_bot.inference.monotonic.solver._phase1_root` is the single
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

    Its soundness leans on the S1.5a.1 fire-time NAF re-evaluation (or
    on the deriving rule running at a strictly-lower priority). Advisory
    — emitted only when ``SolverConfig.warn_derived_naf`` is set. A
    dedicated category so callers / the pytest ``filterwarnings=error``
    suite can target it precisely.
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
            f"relation(s) {', '.join(d.derived)} — soundness relies on the "
            f"fire-time NAF re-evaluation (S1.5a.1). See S1.7.4.",
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
