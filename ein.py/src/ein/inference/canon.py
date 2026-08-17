"""Canonical state identity — S1.5.3 T1.5.3.1, reworked in P1.21 R1.

Order-insensitive **canonical representation** of a `KnowledgeBase`'s
propositional fact list, used by the lattice search's state-index
dedup. Two commitment-set branches that saturate to the same closed KB
collapse to one lattice node; the search space is a DAG, not a tree.

The guarantee (P1.21 R1): **identity is the canonical representation
itself, never a hash of it.** :func:`state_key` returns the sorted
tuple of canonical facts and every identity site (solution-node dedup,
snapshot equality, sanity comparison, progress counting) keys dicts /
sets on that tuple directly — Python's dict falls back to tuple
equality on hash collision, so correctness is independent of hash
quality. Any hash of the key (:func:`state_digest`) is an
accelerator / display form only — a digest is **never** identity.
"""
from __future__ import annotations

from ein.kb.entities import Fact
from ein.kb.store import KnowledgeBase

# One fact in canonical form: ``(relation_name, hashable_args)`` —
# metadata-free, matching fact identity everywhere else in the KB
# (`Fact.__eq__`/`__hash__`, `add_fact` dedup, `_is_new_relative_to`).
CanonicalFact = tuple[str, tuple]

# One KB state in canonical form: the sorted tuple of its canonical
# facts. The extensional model identity ``M = {(R, args)}``.
StateKey = tuple[CanonicalFact, ...]


def state_key(kb: KnowledgeBase) -> StateKey:
    """Order-insensitive canonical key of the KB's *propositional* facts.

    Keys ONLY the facts (ontology, rules, and query are constant
    across branches). The per-branch trace is intentionally the
    *dedup target* — different proof paths leading to the same
    closed KB should collapse to one lattice node.

    Args order **inside** each fact is preserved: `(right-of A B)`
    and `(right-of B A)` are different facts with different
    semantics. Only the OUTER list of facts is sorted (``key=repr``
    — a total order even when arg slots mix ``str``/``int``) to
    canonicalise the set.

    Provenance is **excluded** — fact identity is
    ``(relation_name, args)`` everywhere in the KB (``entities.Fact``,
    ``store.add_fact``, ``commitment._is_new_relative_to``), so the
    state key says the same: two branches that reached the same
    propositions by different routes are the same node. Nested-Fact
    args (Q40) recurse via `_hashable_args`.
    """
    return tuple(sorted(
        ((f.relation_name, _hashable_args(f.args)) for f in kb.facts),
        key=repr,
    ))


def state_digest(state: KnowledgeBase | StateKey) -> int:
    """Hash of a :func:`state_key` — **display/logging only** (hex ids
    in lattice dumps, :class:`SanityError` messages). Never use a
    digest as identity: distinct states may share a digest."""
    if isinstance(state, KnowledgeBase):
        state = state_key(state)
    return hash(state)


def _hashable_args(args) -> tuple:
    return tuple(
        (a.relation_name, _hashable_args(a.args))
        if isinstance(a, Fact)
        else a
        for a in args
    )


__all__ = ["CanonicalFact", "StateKey", "state_digest", "state_key"]
