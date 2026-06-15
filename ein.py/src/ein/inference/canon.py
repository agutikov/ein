"""Canonical state hashing — S1.5.3 T1.5.3.1.

Order-insensitive hash of a `KnowledgeBase`'s propositional fact
list, used by the lattice search's state-index dedup. Two
commitment-set branches that saturate to the same closed KB collapse
to one lattice node; the search space is a DAG, not a tree.
"""
from __future__ import annotations

from ein.kb.entities import Fact
from ein.kb.store import KnowledgeBase


def state_hash(kb: KnowledgeBase) -> int:
    """Order-insensitive hash of the KB's *propositional* fact list.

    Hashes ONLY the facts (ontology, rules, and query are constant
    across branches). The per-branch trace is intentionally the
    *dedup target* — different proof paths leading to the same
    closed KB should collapse to one lattice node.

    Args order **inside** each fact is preserved: `(right-of A B)`
    and `(right-of B A)` are different facts with different
    semantics. Only the OUTER list of facts is sorted to
    canonicalise the set.

    Nested-Fact args (Q40) hash recursively via `_hashable_args`.
    """
    return hash(tuple(sorted(
        (f.layer.value, f.relation_name, _hashable_args(f.args))
        for f in kb.facts
    )))


def _hashable_args(args) -> tuple:
    return tuple(
        (a.relation_name, _hashable_args(a.args))
        if isinstance(a, Fact)
        else a
        for a in args
    )


__all__ = ["state_hash"]
