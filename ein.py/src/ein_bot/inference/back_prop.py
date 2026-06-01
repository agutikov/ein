"""Unconditional-death analysis + back-prop write — S1.5.7 T1.5.7.1 / .2.

A hypothesis branch dies *unconditionally* when its contradiction
rests on no speculative fact other than the branch's own hypothesis
``h``: the death proves ``¬h`` in the parent node's context, so
``(not h)`` may be written into the parent KB (T1.5.7.2) — where it
becomes an O(1) ``_negated_facts`` filter entry every sibling and
descendant inherits.

A *conditional* death — one whose contradiction additionally rests on
some **other** hypothesis — is held back from the (irreversible)
parent write by the ``enable_back_prop_unconditional`` gate.

``unsat_core`` membership
-------------------------
:meth:`KnowledgeBase.unsat_core` walks each conflict witness's
derivation DAG to its ``source`` / ``hypothesis`` / un-provenanced
terminals. A dead branch's core therefore *always* contains the
branch's own seeded hypothesis ``h`` (a hypothesis-kind terminal of
the conflict). :func:`is_unconditional_death` is told which
hypothesis is ``h`` via ``own_hypothesis`` and treats it as a benign
terminal — otherwise no death would ever read as unconditional.

The remaining judgement is a transitive premise walk. The shallow
read — *"the core contains no ``kind='hypothesis'`` fact"* — is
unsound: a core fact can be ``kind='rule'`` yet derive, through a
chain of firings, from a hypothesis. The internal walk :func:`_walk`
follows ``Provenance.premises_raw`` (resolved against the KB) until
every chain grounds out at a ``source`` / un-provenanced given, or
any chain reaches a ``hypothesis`` / ``rejected`` terminal that is
not ``own``.
"""
from __future__ import annotations

from contextvars import ContextVar

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import FactId, Provenance
from ein_bot.kb.store import KnowledgeBase

# S1.5a.14 — Ancestor-kb chain for transitive back-prop.
#
# An unconditional death at depth N derives `(not h)` whose
# correctness rests on no hypothesis (own_hypothesis exempted). The
# fact is therefore true at EVERY ancestor level — depth N-1,
# N-2, …, 0 (root) — not just at depth N. Pre-S1.5a.14 the engine
# wrote the negation only to the immediate consume's kb; the
# fix walks the chain and writes to every ancestor too.
#
# `_explore` (solver.py) pushes its kb on entry and pops on exit.
# `back_propagate` reads the chain and mirrors the write to every
# entry except the one it was given. The chain is root-first, so
# chain[0] is root and chain[-1] is the most recent `_explore`'s
# kb (which is also the kb the current consume is operating on).
_kb_chain_ctx: ContextVar[tuple[KnowledgeBase, ...]] = ContextVar(
    "ein_bot_kb_chain", default=(),
)

# S1.5a.17 — Eager root-bubble pass id. None when eager mode is off
# (back-prop behaves as the S1.5a.14 opportunistic write). Set by
# the outer driver to the current pass number; `back_propagate` and
# `_mirror_forced_positive` raise `BubbleAbort(pass_id)` after
# writing a NEW fact when this is not None.
_eager_pass_ctx: ContextVar[int | None] = ContextVar(
    "ein_bot_eager_pass_id", default=None,
)


class BubbleAbort(Exception):  # noqa: N818 — control-flow signal, not an error (cf. StopIteration)
    """Eager-bubble abort signal — S1.5a.17.

    Raised by `back_propagate` and `_mirror_forced_positive` after
    a new fact has been written to root.kb (and ancestor kbs)
    under eager mode. Caught by the outer solve driver, which
    discards the in-flight subtree, re-saturates root.kb, and
    restarts the search.

    Carries the pass id so the catch site can verify it's the
    current pass (stale aborts from a prior pass that somehow
    bubbled through a ContextVar gap would re-raise).
    """
    def __init__(self, pass_id: int) -> None:
        super().__init__(f"S1.5a.17 BubbleAbort(pass_id={pass_id})")
        self.pass_id = pass_id

# Provenance kinds marking a fact as speculative — introduced on a
# hypothesis branch rather than given (`source`) or rule-derived.
# `rejected` (a retracted hypothesis) counts: it is still branch-local.
_SPECULATIVE_KINDS = frozenset({"hypothesis", "rejected"})


def _walk(
    kb: KnowledgeBase, fact: Fact, visited: set[FactId],
    own: FactId | None,
) -> bool:
    """Recursive core: True iff ``fact``'s premise chain touches a
    speculative fact *other than* ``own``.

    ``own`` is the FactId of the branch's own hypothesis — expected
    in its death's witness set (it is what ``¬`` is being proven of,
    not an external assumption), so a speculative fact equal to
    ``own`` is a benign terminal.

    ``visited`` guards provenance cycles and memoises across sibling
    walks: a fact left in ``visited`` was reached on a chain that
    did not (yet) yield a non-``own`` hypothesis, so a revisit
    contributes nothing — return False. Sound because a chain that
    *does* reach one short-circuits every caller above it.
    """
    key: FactId = (fact.relation_name, fact.args)
    if key in visited:
        return False
    visited.add(key)
    prov = fact.provenance
    if prov is None:
        return False                       # un-provenanced — a given
    if prov.kind in _SPECULATIVE_KINDS:
        return key != own                  # hypothesis terminal
    if prov.kind == "rule":
        for rid in prov.premises_raw:
            premise = kb._fact_by_id(*rid)
            if premise is not None and _walk(kb, premise, visited, own):
                return True
        return False
    return False                           # source-kind — a given


def is_unconditional_death(
    kb: KnowledgeBase,
    unsat_core: frozenset[Fact],
    *,
    own_hypothesis: Fact | None = None,
) -> bool:
    """True iff a dead branch's contradiction rests on no hypothesis
    beyond the branch's own.

    ``unsat_core`` is the conflict's source-frontier
    (``BranchResult.unsat_core``, from ``kb.unsat_core``).
    ``own_hypothesis`` is the branch's seeded hypothesis — always
    present in its own core, and exempted from the count (see the
    module docstring). The death is unconditional iff no *other*
    core fact's premise chain reaches a speculative terminal — then
    ``¬own_hypothesis`` holds in the parent's context and
    ``(not h)`` may be back-propagated.

    An empty ``unsat_core`` returns False (treated conditional): a
    real contradiction always grounds out at a non-empty frontier,
    so an empty one signals an analysis gap — and back-prop,
    irreversible against the parent KB, must not fire on an
    unattributable death.
    """
    if not unsat_core:
        return False
    own: FactId | None = (
        (own_hypothesis.relation_name, own_hypothesis.args)
        if own_hypothesis is not None else None
    )
    visited: set[FactId] = set()
    return not any(_walk(kb, f, visited, own) for f in unsat_core)


# S1.7.24 — `is_symmetric_relation` was DELETED with the on-death
# symmetric mirror: back-prop no longer writes `(not (R b a))` when
# `(R a b)` dies. A symmetric counterpart's death is recovered
# generically — committing it re-derives the dead orientation (the
# user's `(rule symmetric)`) and hits the same ⊥ one branch later — so
# the kernel imposes no `(symmetric R)` semantics here.


def _write_negation(
    kb: KnowledgeBase, hypothesis: Fact,
    unsat_core: frozenset[Fact], rule_name: str,
) -> Fact:
    """Write one ``(not hypothesis)`` fact with the given provenance
    rule name; idempotent — a pre-existing ``(not h)`` is returned
    untouched."""
    existing = kb._fact_by_id("not", (hypothesis,))
    if existing is not None:
        return existing
    not_fact = Fact(
        relation_name="not",
        args=(hypothesis,),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule=rule_name,
            premises_raw=tuple(
                (f.relation_name, f.args) for f in unsat_core
            ),
        ),
    )
    stored = kb.add_and_index_fact(not_fact)
    return stored


def _bubble_to_ancestors(
    ancestors: tuple[KnowledgeBase, ...],
    fact: Fact,
    unsat_core: frozenset[Fact],
    bubbled_name: str,
) -> int:
    """Write ``(not fact)`` into every ancestor kb; return the number of
    ancestors that did not already carry it (the new-write delta).

    Shared by the primary-hypothesis and symmetric-mirror bubbles in
    :func:`back_propagate` (S1.5a.14).
    """
    new = 0
    for anc_kb in ancestors:
        if anc_kb._fact_by_id("not", (fact,)) is None:
            new += 1
        _write_negation(anc_kb, fact, unsat_core, bubbled_name)
    return new


def back_propagate(
    kb: KnowledgeBase, hypothesis: Fact, unsat_core: frozenset[Fact],
    *,
    rule_name: str = "<back-prop-unconditional>",
) -> Fact:
    """Write ``(not hypothesis)`` into ``kb`` *and* every ancestor kb
    on an unconditional death.

    The negation is a REASONING-layer ``(not <hypothesis>)`` fact
    with rule-provenance citing the ``unsat_core`` frontier — so its
    derivation walks back to the same given facts the death rested
    on. Indexing it updates ``kb._negated_facts``, so every
    subsequent ``_candidates_for`` / ``_prune_alive`` drops
    ``hypothesis`` in O(1), and ``kb.fork()`` carries the exclusion
    to descendant branches.

    **Transitive bubble-up (S1.5a.14).** Because the death is
    unconditional — its premise chain reaches no speculative fact —
    ``(not h)`` is provably true in *every* ancestor's context, not
    just the immediate parent. The :data:`_kb_chain_ctx` ContextVar
    holds the chain (root first, current consume's kb last) so this
    function can mirror the write into every ancestor. Ancestor
    writes carry the rule-name suffix ``-bubbled`` so traces show
    which writes were lifted from a deeper level.

    ``rule_name`` distinguishes the back-prop's origin in traces and
    audits (T1.5.7.2 ``<back-prop-unconditional>`` for consume-loop
    kills, T1.5.7.4 ``<lookahead-dies-immediately>`` for S1.5.6
    lookahead kills).

    S1.7.24 — no symmetric mirror: a dead ``(R a b)`` does NOT
    proactively write ``(not (R b a))``. The counterpart's death is
    recovered generically (re-derivation hits the same ⊥); the kernel
    keys on ``is_symmetric`` nowhere.

    Idempotent: a pre-existing ``(not h)`` at any level is returned
    untouched.

    Returns the *primary* stored ``(not …)`` Fact in ``kb``.

    **Eager mode (S1.5a.17).** When :data:`_eager_pass_ctx` is set
    (the outer driver bound a pass id), this function raises
    :class:`BubbleAbort` after writing if at least one *new* fact
    was added (idempotent re-writes don't trigger). The chain's
    root kb (``chain[0]``) gets its ``_pass_bubbled`` counter
    incremented by the number of new writes so the outer driver
    can detect fixpoint by counter delta.
    """
    primary_existed = kb._fact_by_id("not", (hypothesis,)) is not None
    primary = _write_negation(kb, hypothesis, unsat_core, rule_name)

    # Bubble (not h) up to EVERY ancestor kb (S1.5a.14). The
    # death's premise chain reaches no hypothesis other than the
    # branch's own, so ``(not h)`` is provably true at every level.
    # Pruning at every level is essential for combinatorial sanity
    # — without it, depth-6 puzzles with 60 alive hyps face 60^6
    # potential branchings even when most are dead at the parent.
    #
    # Mutating ancestor kbs invalidates two caches the ancestors
    # may be relying on mid-flight:
    #   1. `builder.state_index` keyed by `state_hash(ancestor.kb)`
    #      — now stale (different fact set → different hash). Future
    #      _explore calls on equivalent states compute the *new*
    #      hash, so dedup just gives false negatives (slower, not
    #      wrong); we accept it.
    #   2. Per-`_consume` `verdict_at` — caches alive/cond-dead
    #      verdicts for already-tried hypotheses. If (not h) lands
    #      and h is cached "alive" at the ancestor's level, the
    #      ancestor would recurse on a now-doomed branch. Cleared
    #      via the `_consume_caches_ctx` chain (see solver.py).
    chain = _kb_chain_ctx.get()
    ancestors = tuple(ak for ak in chain if ak is not kb)
    bubbled_name = rule_name + "-bubbled"
    new_writes = 0 if primary_existed else 1
    new_writes += _bubble_to_ancestors(
        ancestors, hypothesis, unsat_core, bubbled_name,
    )

    # Invalidate ancestor verdict_at caches so any stale "alive"
    # entry for the now-dead hypothesis is re-classified on the
    # next sweep. Cache invalidation is done at every ancestor
    # whose kb received the bubbled write.
    if ancestors:
        _clear_ancestor_verdict_caches()

    # S1.5a.17 — eager-bubble abort. Bump the root counter and
    # raise after the writes are durable so the outer driver sees
    # the new state on re-entry.
    eager_pass = _eager_pass_ctx.get()
    if eager_pass is not None and new_writes > 0:
        root_kb = chain[0] if chain else kb
        _bump_pass_bubbled(root_kb, new_writes)
        raise BubbleAbort(pass_id=eager_pass)
    return primary


def _bump_pass_bubbled(root_kb: KnowledgeBase, n: int) -> None:
    """Increment root_kb._pass_bubbled by n (initialise if absent).

    The counter is a thin attribute on root.kb that the outer
    driver in `solver.py` reads to decide fixpoint between passes.
    Always set, even when eager mode is off, so post-solve
    inspection finds zero instead of AttributeError.
    """
    cur = getattr(root_kb, "_pass_bubbled", 0)
    root_kb._pass_bubbled = cur + n


# Verdict-at chain. Each `_consume` pushes its own verdict_at dict
# onto this contextvar on entry and pops on exit. `back_propagate`
# clears every ancestor entry on a bubble so the next sweep
# re-classifies hypotheses against the now-tighter kb.
_consume_caches_ctx: ContextVar[tuple[dict, ...]] = ContextVar(
    "ein_bot_consume_caches", default=(),
)


def _clear_ancestor_verdict_caches() -> None:
    """Clear every ancestor `_consume`'s verdict_at cache.

    Called after `back_propagate` writes bubbled negatives. The
    chain holds verdict_at dicts in root-first order with the
    current `_consume`'s dict last; we clear all but the last
    (the current sweep will re-evaluate on its own).
    """
    caches = _consume_caches_ctx.get()
    if len(caches) <= 1:
        return
    for v in caches[:-1]:
        v.clear()


__all__ = [
    "BubbleAbort",
    "_bump_pass_bubbled",
    "_consume_caches_ctx",
    "_eager_pass_ctx",
    "_kb_chain_ctx",
    "_write_negation",
    "back_propagate",
    "is_unconditional_death",
]
