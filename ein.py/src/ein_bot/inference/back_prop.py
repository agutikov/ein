"""Unconditional-death analysis for back-propagation — S1.5.7 T1.5.7.1.

A hypothesis branch dies *unconditionally* when its contradiction
rests on no speculative fact: the same clash is reachable from the
parent KB's own (given + rule-derived) facts, with the branch's
hypothesis ``h`` playing no part. An unconditional death licenses
back-propagating ``(not h)`` into the parent (T1.5.7.2) — where it
becomes an O(1) ``_negated_facts`` filter entry every sibling and
descendant inherits.

A *conditional* death — one whose contradiction depends on the
branch's own hypothesis (or an ancestor's) — must NOT back-propagate:
``(not h)`` would be a claim true only on this path, and writing it
into the parent permanently and wrongly excludes a valid hypothesis.

The judgement is a transitive premise walk. The shallow read — *"the
unsat-core contains no ``kind='hypothesis'`` fact"* — is unsound: an
unsat-core fact can be ``kind='rule'`` yet derive, through a chain of
firings, from a hypothesis (its own provenance is ``'rule'``, but its
premises are not). :func:`reaches_hypothesis` follows
``Provenance.premises_raw`` — resolving each id against the KB —
until every chain grounds out at a ``source``-kind / un-provenanced
given, or any chain reaches a ``hypothesis`` / ``rejected`` terminal.

T1.5.7.6 reuses :func:`reaches_hypothesis` to classify a fact derived
by parent re-saturation as a *forced deduction* (no hypothesis in its
support — fold into the current node) vs a hypothesis-dependent one.
"""
from __future__ import annotations

from ein_bot.kb.entities import Fact
from ein_bot.kb.provenance import FactId
from ein_bot.kb.store import KnowledgeBase

# Provenance kinds marking a fact as speculative — introduced on a
# hypothesis branch rather than given (`source`) or rule-derived.
# `rejected` (a retracted hypothesis) counts: it is still branch-local,
# so a death resting on it is conditional. Erring this way is the safe
# direction — it withholds an irreversible back-prop write.
_SPECULATIVE_KINDS = frozenset({"hypothesis", "rejected"})


def _walk(kb: KnowledgeBase, fact: Fact, visited: set[FactId]) -> bool:
    """Recursive core: True iff ``fact``'s premise chain touches a
    speculative fact.

    ``visited`` guards provenance cycles and memoises across sibling
    walks. The memoisation is sound because a chain that *does* reach
    a hypothesis short-circuits every caller above it: a fact is left
    in ``visited`` only by a walk that has not yielded a hypothesis
    through it, so a later revisit genuinely contributes nothing —
    return False.
    """
    key: FactId = (fact.relation_name, fact.args)
    if key in visited:
        return False
    visited.add(key)
    prov = fact.provenance
    if prov is None:
        return False                       # un-provenanced — a given
    if prov.kind in _SPECULATIVE_KINDS:
        return True                        # hypothesis terminal
    if prov.kind == "rule":
        for rid in prov.premises_raw:
            premise = kb._fact_by_id(*rid)
            if premise is not None and _walk(kb, premise, visited):
                return True
        return False
    return False                           # source-kind — a given


def reaches_hypothesis(kb: KnowledgeBase, fact: Fact) -> bool:
    """True iff ``fact`` transitively depends on a hypothesis-introduced
    fact.

    A ``rule``-kind premise id absent from ``kb`` is skipped — it
    cannot contribute a hypothesis terminal, so an unresolvable chain
    reads as hypothesis-free. Callers must therefore pass a KB in which
    the derivation chain is fully present (a ``try_branch`` fork is
    one).
    """
    return _walk(kb, fact, set())


def is_unconditional_death(
    kb: KnowledgeBase, unsat_core: frozenset[Fact],
) -> bool:
    """True iff a dead branch's contradiction rests on no hypothesis.

    ``unsat_core`` is the source-frontier of the contradiction (the
    ``BranchResult.dead`` field, produced by ``kb.unsat_core``). The
    death is unconditional iff *no* frontier fact's premise chain
    reaches a ``hypothesis`` / ``rejected`` terminal — then the same
    contradiction recurs from the parent's facts alone and ``(not h)``
    may be back-propagated.

    An empty ``unsat_core`` returns False (treated conditional): a
    real contradiction always grounds out at a non-empty frontier, so
    an empty one signals an analysis gap — and back-prop, irreversible
    against the parent KB, must not fire on an unattributable death.
    """
    if not unsat_core:
        return False
    visited: set[FactId] = set()
    return not any(_walk(kb, f, visited) for f in unsat_core)


__all__ = ["is_unconditional_death", "reaches_hypothesis"]
