"""Solution-node predicates — P1.7a's domain-agnostic definition of "an answer".

A **solution node** is a saturated KB that is *consistent* and *complete*:

    is_solution_node(kb)  ⟺  consistent(kb) ∧ complete(kb)

- ``complete(kb)``   — **no open hypothesis**: the hypothesis generator
  proposes nothing that isn't already decided (present or refuted).
  Built only on :func:`ein.inference.hypgen.generate_hypotheses` + the
  KB's positive / negated fact sets — **no** ``is-a`` / ``total`` / relation
  signatures (those encoding-specific crutches made the S1.7.3 patch wrong).
- ``consistent(kb)`` — no contradiction (no ``(false)``, no ``X ∧ ¬X``).

This is the signal P1.7a's search records on, replacing the old
``is_solved`` (goal-pattern match) — which accepted a partial dead-end as a
solution (the severe bug S1.7.3 found). See
``M1 P1.7a``.

``open_hypotheses`` is the canonical implementation; ``solver._compute_alive``
delegates here so there is exactly one open-set definition. ``complete``
asks the same generator for its *first* element only — the open set is
never materialised just to be truth-tested (S1.9.E16).
"""
from __future__ import annotations

from ein.inference.contradiction import ContradictionDetector
from ein.inference.hypgen import generate_hypotheses
from ein.kb.provenance import FactId
from ein.kb.store import KnowledgeBase


def open_hypotheses(kb: KnowledgeBase) -> frozenset[FactId]:
    """The open set — viable, not-yet-decided hypotheses.

    :func:`generate_hypotheses` already yields exactly the candidates that
    are neither asserted (``_already_a_fact``) nor refuted (``_negated_facts``)
    nor immediately doomed (lookahead).

    S1.7.24 — **no symmetric canonicalisation.** The kernel keys on
    ``is_symmetric`` nowhere, so ``(R a b)`` and ``(R b a)`` are TWO
    distinct open entries even for a ``(symmetric R)`` relation. Correct
    ``k`` is recovered generically: committing either orientation
    re-derives the other (the user's ``(rule symmetric)``), so both
    branches saturate to the **same** KB and collapse at the
    ``canon.state_key`` solution-node dedup — the pair counts once
    because the *user's* rule established the equivalence, not the kernel.
    """
    return frozenset(
        (f.relation_name, f.args) for f in generate_hypotheses(kb)
    )


def complete(kb: KnowledgeBase) -> bool:
    """No open hypothesis — the generator proposes nothing undecided.

    Short-circuits (S1.9.E16): the question is emptiness, so the first
    candidate the generator yields answers it — building the whole
    frozenset only to truth-test it re-ran the per-candidate filter
    pipeline (the one-step lookahead included) for every *later*
    candidate whose value was already irrelevant. Equivalent to
    ``not open_hypotheses(kb)`` by construction; measured 54 ms of a
    1.7 s zebra2 ``solve(stop_after=1)``, where 8 of 9 ``complete``
    calls are answered by candidate #1.
    """
    return next(generate_hypotheses(kb), None) is None


def consistent(kb: KnowledgeBase) -> bool:
    """No contradiction (no ``(false)``, no same-layer ``X ∧ ¬X``)."""
    return not ContradictionDetector(kb).detect()


def is_solution_node(kb: KnowledgeBase) -> bool:
    """consistent ∧ complete — the P1.7a definition of a solution."""
    return consistent(kb) and complete(kb)


__all__ = [
    "complete",
    "consistent",
    "is_solution_node",
    "open_hypotheses",
]
