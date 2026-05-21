"""Canonical state hashing — S1.5.3 T1.5.3.1.

Order-insensitive hash of a `KnowledgeBase`'s propositional fact
list, used by the hypothesis loop's state-index dedup. Two branches
that saturate to the same closed KB collapse to one SearchNode;
the search tree becomes a DAG in storage.

Excluded from the hash:
  - bookkeeping facts whose head is in `BOOKKEEPING_HEADS` — the
    Q40 `(hypothesis …)` / `(contradiction-under …)` carriers
    that try_branch adds to mark "which branch is this". They
    reference the branch-specific hypothesis, so without this
    exclusion two symmetric branches with identical post-
    saturation propositional content would still hash differently.
"""
from __future__ import annotations

from ein_bot.kb.entities import Fact
from ein_bot.kb.store import KnowledgeBase

BOOKKEEPING_HEADS: frozenset[str] = frozenset({
    "hypothesis",
    "contradiction-under",
})


def state_hash(kb: KnowledgeBase) -> int:
    """Order-insensitive hash of the KB's *propositional* fact list.

    Hashes ONLY the facts (ontology, rules, and query are constant
    across branches). The per-branch trace is intentionally the
    *dedup target* — different proof paths leading to the same
    closed KB should collapse to one search-tree node.

    Args order **inside** each fact is preserved: `(right-of A B)`
    and `(right-of B A)` are different facts with different
    semantics. Only the OUTER list of facts is sorted to
    canonicalise the set.

    Nested-Fact args (Q40) hash recursively via `_hashable_args`.
    """
    return hash(tuple(sorted(
        (f.layer.value, f.relation_name, _hashable_args(f.args))
        for f in kb.facts
        if f.relation_name not in BOOKKEEPING_HEADS
    )))


def _hashable_args(args) -> tuple:
    return tuple(
        (a.relation_name, _hashable_args(a.args))
        if isinstance(a, Fact)
        else a
        for a in args
    )


__all__ = ["BOOKKEEPING_HEADS", "state_hash"]
