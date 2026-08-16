"""Smallest contradiction frontier — S1.9.E19, renamed per R3, OR-aware per S1.21.7.

`smallest_contradiction_frontier(kb)` returns the smallest source frontier
that forces a contradiction: over the contradiction witnesses the detector
finds, the smallest set of given/hypothesis leaves from which **one** witness
follows. Sound by construction and NAF-safe — the result is a real
derivation's leaves, never a re-saturated guess.

It is still **not** a subset-minimal MUS: no proper subset is checked for
satisfiability, so a smaller *logically* sufficient set may exist that no
recorded derivation exhibits.

What S1.21.7 changed. It used to be minimal only over *recorded* derivations
— the KB kept one justification per fact (first derivation wins), so a
shorter derivation that fired later was invisible and the answer moved with
rule-firing order (R3 §E3: `{C, Y}` or `{A, B, Y}` for the same KB, decided
by `:priority`). Re-derivations are now recorded as alternative
justifications (``store.record_justification``), making the proof structure
an AND/OR graph, and this function searches it
(:mod:`ein.inference.explain`) instead of walking one justification per fact.
The result no longer depends on firing order.

Two caveats survive, both structural:

- **Recorded, not all, derivations.** Alternatives are only the firings the
  saturator attempted, capped per fact at ``store.MAX_ALT_JUSTIFICATIONS``.
  "Minimal" stays relative to the rule set and the saturation strategy.
- **Budgeted.** Minimum-cardinality frontier over an AND/OR graph is the
  minimum-axiom-set / ATMS-label problem, worst-case exponential. The search
  runs under an :class:`~ein.inference.explain.ExplanationBudget`; call
  :func:`~ein.inference.explain.minimal_contradiction_frontier` directly to
  read :attr:`~ein.inference.explain.Explanation.exhausted`, which reports
  whether the budget was hit.

Why this shape. `unsat_core(witnesses)` unions the source frontier of *every*
contradiction witness. When one cause propagates it fans out into many
witnesses, so the union over-states the conflict — e.g. `zebra2-bad`'s one
injected fact produces **123** witnesses whose frontiers union to **38**
facts, yet each witness is a complete contradiction on its own with a **1-5**
fact frontier, the smallest being exactly the injected culprit. The smallest
single-witness frontier is therefore the most legible explanation the
provenance supports.

Why provenance, not re-saturation. The textbook MUS minimiser — drop a fact,
re-saturate, keep it dropped if still ⊥ — is **unsound under NAF here**: removing
a fact can flip an `(absent P)` premise and *fabricate* a contradiction that the
full KB never had, so "the subset still contradicts" need not mean "the subset
reproduces the real contradiction" (corollary C3 of the normative NAF
semantics, ``docs/kernel/inference/absent_semantics.md``). Measured on
`zebra2-bad`, that minimiser
degenerates to a structural-declaration residue (relation/property decls whose
removal merely breaks saturation), not the conflict. A single witness's frontier
is instead a *real* derivation's leaves — sound by construction, NAF-safe, and
cheap (provenance only, no saturation).
"""
from __future__ import annotations

from collections.abc import Iterable

from ein.kb.entities import Fact
from ein.kb.store import KnowledgeBase

from .explain import ExplanationBudget, minimal_contradiction_frontier


def smallest_contradiction_frontier(
    kb: KnowledgeBase,
    witnesses: Iterable[Fact] | None = None,
    *,
    budget: ExplanationBudget | None = None,
) -> frozenset[Fact]:
    """Smallest source frontier of ``kb``'s contradictions.

    ``witnesses`` defaults to the witness fact of every
    :class:`~ein.inference.contradiction.Contradiction` the detector finds.
    Returns ``frozenset()`` when there is no contradiction.

    The frontier is drawn from exactly the population
    :meth:`~ein.kb.store.KnowledgeBase.unsat_core` returns (``source`` /
    ``hypothesis`` / un-provenanced givens), so the result is always a subset
    of the union core. Searches every recorded derivation (S1.21.7), so it is
    independent of rule-firing order — see the module docstring for the two
    caveats that survive.
    """
    return minimal_contradiction_frontier(
        kb, witnesses, budget=budget).frontier


__all__ = ["smallest_contradiction_frontier"]
