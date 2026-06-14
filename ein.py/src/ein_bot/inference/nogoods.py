"""Path-condition no-good clause learning — S1.5a.18.

CDCL flavour applied to the hypothesis search: every dead
commitment's path condition (chain of ancestor hypotheses + own
hyp) becomes a learned clause stored at ``root.kb._nogoods``. A
prospective commitment set that is a superset of any learned
clause is filtered **pre-fork** by
:func:`ein_bot.inference.apriori.filter_candidate` (the set-search
engine's downward-closure prune) — no fork, no saturation, no own
clause emitted.

Clause representation: ``frozenset[tuple[str, tuple]]`` — set of
FactIds. The implicit "and" is set semantics; the implicit "not
(and ...)" is the learned constraint.

Subsumption: stored clauses are kept minimal. On emit, if any
existing clause is a subset of the new clause, the new clause is
subsumed and dropped; otherwise existing clauses that are strict
supersets of the new clause are removed and the new clause is
added.

Single-element clauses (size < 2) are owned by
:func:`back_prop.back_propagate` (which writes ``(not h)`` into
``_negated_facts``); :func:`emit_nogood`'s ``min_size`` parameter
defaults to ``2`` to preserve that split. The set-indexed
engines (monotonic / lattice — Q1.5b.5.c) pass ``min_size=1``
to let singleton clauses land, since the in-layer filter relies
on ``root._nogoods`` for cross-layer pruning before the next
``_compute_alive`` recomputes ``alive``.

Eager-mode composition (S1.5a.17): on a successful add (clause
was novel and non-subsumed), :func:`emit_nogood` bumps
``root_kb._pass_bubbled`` and raises :class:`BubbleAbort` if
``_eager_pass_ctx`` is set — same control flow as
``back_propagate``'s eager-mode raise. The outer driver discards
the in-flight subtree and re-enters with the tightened clause
set.
"""
from __future__ import annotations

from ein_bot.kb.provenance import FactId
from ein_bot.kb.store import KnowledgeBase

from .back_prop import (
    BubbleAbort,
    _bump_pass_bubbled,
    _eager_pass_ctx,
)

# Clause = frozenset of FactIds — order-insensitive set of
# hypothesis (relation_name, args) tuples whose joint commitment
# is provably inconsistent at root.
Clause = frozenset[FactId]


def emit_nogood(
    root_kb: KnowledgeBase,
    clause: Clause,
    *,
    min_size: int = 2,
) -> bool:
    """Insert ``clause`` into ``root_kb._nogoods`` with subsumption.

    Returns True iff a new clause was actually added; False if the
    clause is subsumed by an existing one (or is below ``min_size``).

    Subsumption rules:
    - If any existing ``C ⊆ clause``, return False (existing is
      stronger; new clause adds no information).
    - Else remove every existing ``C' ⊇ clause`` (new clause
      subsumes them), insert ``clause``, return True.

    ``min_size`` (default 2) preserves the
    "size-1 clauses are back-prop's domain" split — the
    ``(not h)`` writeback already filters those candidates via
    ``_negated_facts`` before any nogood check would fire.
    Set-indexed engines (monotonic / lattice — Q1.5b.5.c) pass
    ``min_size=1`` so layer-1 singleton deaths land too,
    because the `apriori.filter_candidate` subset check runs
    against ``root._nogoods`` before ``alive`` is recomputed.

    Under eager mode (``_eager_pass_ctx`` set) and a True return,
    bumps ``root_kb._pass_bubbled`` and raises :class:`BubbleAbort`.
    """
    if len(clause) < min_size:
        return False

    nogoods = root_kb._nogoods
    # Subsumed by an existing stronger clause?
    for c in nogoods:
        if c.issubset(clause):
            return False

    # Remove any existing clauses this one subsumes.
    to_remove = [c for c in nogoods if clause.issubset(c)]
    for c in to_remove:
        nogoods.discard(c)
    nogoods.add(clause)

    eager_pass = _eager_pass_ctx.get()
    if eager_pass is not None:
        _bump_pass_bubbled(root_kb, 1)
        raise BubbleAbort(pass_id=eager_pass)
    return True


__all__ = [
    "Clause",
    "FactId",
    "emit_nogood",
]
