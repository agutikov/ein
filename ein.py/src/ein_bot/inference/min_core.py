"""Minimal unsat core — S1.9.E19.

`minimal_unsat_core(kb)` returns the **smallest single-contradiction source
frontier**: the tightest set of given/assumed facts that — via one real
derivation — produce a contradiction. It is the readable explanation for a
trace ("these N clues conflict") versus the larger set
:meth:`KnowledgeBase.unsat_core` returns.

Why this shape. `unsat_core(witnesses)` unions the source frontier of *every*
contradiction witness. When one cause propagates it fans out into many
witnesses, so the union over-states the conflict — e.g. `zebra2-bad`'s one
injected fact produces **123** witnesses whose frontiers union to **38** facts,
yet each witness is a complete contradiction on its own with a **1-5** fact
frontier, the smallest being exactly the injected culprit. The smallest
single-witness frontier is therefore both minimal and the most legible.

Why provenance, not re-saturation. The textbook MUS minimiser — drop a fact,
re-saturate, keep it dropped if still ⊥ — is **unsound under NAF here**: removing
a fact can flip an `(absent P)` premise and *fabricate* a contradiction that the
full KB never had, so "the subset still contradicts" need not mean "the subset
reproduces the real contradiction." Measured on `zebra2-bad`, that minimiser
degenerates to a structural-declaration residue (relation/property decls whose
removal merely breaks saturation), not the conflict. A single witness's frontier
is instead a *real* derivation's leaves — sound by construction, NAF-safe, and
cheap (provenance walks via E6's `walk_premises`, no saturation).
"""
from __future__ import annotations

from collections.abc import Iterable

from ein_bot.inference.contradiction import ContradictionDetector
from ein_bot.kb.entities import Fact
from ein_bot.kb.store import KnowledgeBase


def minimal_unsat_core(
    kb: KnowledgeBase, witnesses: Iterable[Fact] | None = None,
) -> frozenset[Fact]:
    """Smallest single-witness source frontier of ``kb``'s contradictions.

    ``witnesses`` defaults to the witness fact of every
    :class:`~ein_bot.inference.contradiction.Contradiction` the detector finds.
    Returns ``frozenset()`` when there is no contradiction. Each candidate
    frontier is ``kb.unsat_core([w])`` — one witness's real derivation leaves —
    so the result is a sound, NAF-safe explanation, never a re-saturated guess.
    """
    if witnesses is None:
        witnesses = [c.witness for c in ContradictionDetector(kb).detect()]
    best: frozenset[Fact] | None = None
    for w in witnesses:
        frontier = frozenset(kb.unsat_core([w]))
        if best is None or len(frontier) < len(best):
            best = frontier
    return best if best is not None else frozenset()


__all__ = ["minimal_unsat_core"]
