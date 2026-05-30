"""Goal-relevant trace pruning — S1.6.5.

The full firing log is the engine's *complete saturation*: for zebra2,
560 firings of which 517 merely re-derive facts already present
(`Firing.redundant`) and most of the rest are closure bookkeeping
(transitive / includes lifting, negative propagation) a human never
writes down. A human walkthrough is ~20 moves.

:func:`relevant_firings` recovers that human-scale slice by a
**provenance backtrack** from the solution, exactly as one would do by
hand:

1. **Seed** with the solved *assignment* — the positive facts on the
   relations the puzzle solves for (the query goal's relations + its
   `:hrules` targets; the grid).
2. **Backward cone** — keep only firings whose derived fact lies on a
   provenance path (`Firing.premises`) to a seed. A firing that derived
   a fact nothing downstream used is exploratory noise.
3. **Drop `redundant`** re-derivations.
4. **Unconditional vs hypothesis** — a firing is *conditional* iff its
   derivation transitively consumes a commitment (hypothesis) fact; the
   rest is the unconditional `d=0` spine (idea-08's branch-depth).

For zebra2 this is 560 → ~11 load-bearing firings.
"""
from __future__ import annotations

from typing import TYPE_CHECKING

from ..ir.types import Atom, SForm

if TYPE_CHECKING:
    from ..inference.firing import Firing
    from ..kb.store import KnowledgeBase

FactKey = tuple


def _key(fact) -> FactKey:
    return (fact.relation_name, fact.args)


# ── seed: the solved assignment ────────────────────────────────────

def _solution_relations(kb: KnowledgeBase) -> set[str]:
    """Relation names the puzzle solves for — from the query goal +
    its `:hrules` targets. Empty when there is no query."""
    rels: set[str] = set()
    query = getattr(kb, "query", None)
    if query is None:
        return rels
    for kp in getattr(query, "kw_pairs", ()):
        name = getattr(kp.key, "name", None)
        val = kp.value
        if name == "goal":
            _collect_goal_relations(val, rels)
        elif name == "hrules" and isinstance(val, SForm):
            # `(<activator> (R T1 T2) …)` — each triple's head is a relation.
            for triple in val.args:
                if isinstance(triple, SForm) and isinstance(triple.head, Atom):
                    rels.add(triple.head.name)
    return rels


def _collect_goal_relations(form, rels: set[str]) -> None:
    if not isinstance(form, SForm):
        return
    head = form.head.name if isinstance(form.head, Atom) else None
    if head in ("and", "or", "not"):
        for a in form.args:
            _collect_goal_relations(a, rels)
    elif head:
        rels.add(head)


def _seed_keys(kb: KnowledgeBase) -> set[FactKey]:
    rels = _solution_relations(kb)
    if not rels:
        return set()
    return {
        _key(f) for f in kb.facts
        if f.relation_name in rels and len(f.args) == 2
    }


# ── the prune ──────────────────────────────────────────────────────

def relevant_firings(
    firings: tuple[Firing, ...], kb: KnowledgeBase, commitment,
) -> list[tuple[Firing, bool]]:
    """Return ``[(firing, conditional)]`` for the goal-relevant slice.

    ``conditional`` flags firings that depend (transitively) on a
    hypothesis (commitment) fact. Firings keep their original order;
    each derived fact appears once (the first, non-redundant derivation).
    """
    seeds = _seed_keys(kb)

    # Backward provenance cone from the seeds over the firing graph.
    by_derived: dict[FactKey, list[Firing]] = {}
    for f in firings:
        by_derived.setdefault(_key(f.derived), []).append(f)
    needed: set[FactKey] = set()
    stack = list(seeds)
    while stack:
        k = stack.pop()
        if k in needed:
            continue
        needed.add(k)
        for f in by_derived.get(k, []):
            for p in f.premises:
                stack.append(_key(p))

    # Conditional-fact closure: seeded by the commitment, grown forward.
    conditional: set[FactKey] = {tuple(fid) for fid in (commitment or ())}
    for f in firings:
        if any(_key(p) in conditional for p in f.premises):
            conditional.add(_key(f.derived))

    kept: list[tuple[Firing, bool]] = []
    seen: set[FactKey] = set()
    for f in firings:
        if f.redundant:
            continue
        dk = _key(f.derived)
        if dk not in needed or dk in seen:
            continue
        seen.add(dk)
        kept.append((f, dk in conditional))
    return kept


__all__ = ["relevant_firings"]
