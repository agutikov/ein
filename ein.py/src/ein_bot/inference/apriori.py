"""Apriori-style layer-(k+1) candidate generation — S1.5b.2.

Pure set-arithmetic. Generates the size-``(k+1)`` candidate sets
from a size-``k`` alive frontier via the textbook Apriori
prefix-join, then filters each candidate against the current
alive hypothesis set + the current `_nogoods` store.

Used by both `inference/monotonic/solver.py` (S1.5b.5) and
`inference/lattice/solver.py` (S1.5b.21). No kb inspection, no
saturator invocation, no engine-specific state — the engine
calls `generate_layer(A_prev, alive=..., nogoods=...)` once
per BFS layer and gets back the survivors in canonical order.

Type compatibility: ``FactId`` here is the same shape as
:data:`ein_bot.inference.nogoods.FactId` (``tuple[str, tuple]``),
so apriori candidates and nogood clauses compose without
conversion. See Q1.5b.3 (lattice node representation —
canonical tuple) and Q1.5b.5 (refutation semantics — unified
with `_nogoods` store).

Semantic peer: :func:`ein_bot.inference.nogoods.matches_any_nogood`
performs the same subset check, but against a single prospective
hypothesis being added to a path-set; here :func:`filter_candidate`
checks an already-assembled set against every clause.
"""
from __future__ import annotations

from collections.abc import Iterable

# (relation_name, args) tuple — matches `nogoods.FactId` shape so
# the two modules compose without conversion. See Q1.5b.3.
FactId = tuple[str, tuple]

# Canonically-ordered tuple of FactIds. Sort key is the natural
# tuple order on (relation_name, args). A CanonicalSetId is the
# lattice's node identity.
CanonicalSetId = tuple[FactId, ...]


def canonicalise(elements: Iterable[FactId]) -> CanonicalSetId:
    """Sort + dedup an iterable of fact-ids into a CanonicalSetId."""
    return tuple(sorted(set(elements)))


def apriori_prefix_join(
    a_prev: Iterable[CanonicalSetId],
) -> Iterable[CanonicalSetId]:
    """Textbook Apriori-gen prefix-join.

    For each pair ``(s, t)`` in ``a_prev x a_prev`` that agree on
    their first ``|s|-1`` elements with ``s[-1] < t[-1]``, emit
    ``s[:-1] + (s[-1], t[-1])``.

    Yields each layer-(k+1) candidate of size ``|s|+1`` exactly
    once. Caller is responsible for filtering against the current
    `_nogoods` + alive set (see :func:`filter_candidate`).
    """
    sorted_prev = sorted(a_prev)
    for i, s in enumerate(sorted_prev):
        prefix = s[:-1]
        for t in sorted_prev[i + 1:]:
            if t[:-1] != prefix:
                break  # sorted — no further matches share the prefix
            if s[-1] < t[-1]:
                yield (*prefix, s[-1], t[-1])


def filter_candidate(
    candidate: CanonicalSetId,
    *,
    alive: frozenset[FactId],
    nogoods: Iterable[frozenset[FactId]],
) -> bool:
    """True iff ``candidate`` should be explored.

    Drops ``candidate`` if:

      - any element is not in the current alive set (covers
        single-element negatives back-propagated since
        ``a_prev`` was computed);
      - any nogood clause is a subset of the candidate (covers
        multi-element conditional deaths whose clauses propagated
        up from earlier layers).

    The "every (k-1)-subset ∈ a_prev" check is covered by
    :func:`apriori_prefix_join`'s construction — caller does not
    re-verify here.
    """
    if not all(h in alive for h in candidate):
        return False
    candidate_set = frozenset(candidate)
    for clause in nogoods:
        if clause.issubset(candidate_set):
            return False
    return True


def generate_layer(
    a_prev: Iterable[CanonicalSetId],
    *,
    alive: frozenset[FactId],
    nogoods: Iterable[frozenset[FactId]],
) -> list[CanonicalSetId]:
    """Convenience: prefix-join + per-candidate filter.

    Returns the surviving layer-(k+1) candidates in canonical
    (sorted) order. Order is deterministic; the engine's
    within-layer scoring (Q1.5b.9) re-sorts before iteration if
    a non-default policy is configured.
    """
    return [
        c for c in apriori_prefix_join(a_prev)
        if filter_candidate(c, alive=alive, nogoods=nogoods)
    ]


def layer_1(alive: frozenset[FactId]) -> list[CanonicalSetId]:
    """Layer-1 enumeration: every singleton from ``alive``,
    sorted. Equivalent to what :func:`apriori_prefix_join` would
    produce on an ``A_0 = {()}`` (the empty set), but explicit
    for clarity at the BFS entry point.
    """
    return [(h,) for h in sorted(alive)]


__all__ = [
    "CanonicalSetId",
    "FactId",
    "apriori_prefix_join",
    "canonicalise",
    "filter_candidate",
    "generate_layer",
    "layer_1",
]
