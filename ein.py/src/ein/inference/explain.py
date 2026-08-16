"""Minimal explanation over the AND/OR proof graph — S1.21.7.

Ein's provenance is an **AND/OR graph**, not a tree: a fact is an OR-node
over the derivations the saturator recorded for it
(:meth:`~ein.kb.store.KnowledgeBase.justifications`), and each derivation is
an AND-node over its ``premises_raw``. This module answers the question that
structure exists to answer — *what is the smallest set of given facts that
forces this conclusion?* — by computing an **ATMS label** for each fact: the
subset-minimal environments (sets of frontier facts) from which it follows.

Why this is not a walk. :meth:`~ein.kb.store.KnowledgeBase.unsat_core` walks
one justification per fact, so what it returns is minimal only over the
derivations that happened to be recorded first (P1.21 R3 §E3: flipping two
rules' ``:priority`` flipped the reported core between ``{C, Y}`` and
``{A, B, Y}`` while ``{C, Y}`` still existed). Choosing the best *combination*
of justifications is a different problem — the minimum-axiom-set / ATMS-label
problem, worst-case exponential — so it needs a real search and an explicit
budget, which is what :class:`ExplanationBudget` is.

The algorithm
-------------

A least fixpoint from the frontier upward, in the style of ATMS label
propagation:

- **Terminals.** A ``source`` / ``hypothesis`` / un-provenanced fact is a
  frontier leaf: its label is ``{{itself}}``. A rule-kind provenance with no
  premises is a synthetic engine writeback (``<forced-positive>`` and
  friends — ``docs/kernel/inference/reserved_engine_strings.md``) whose
  contract is that provenance walks ground out on it; its label is
  ``{∅}``, which contributes nothing to any environment, exactly as it
  contributes nothing to today's ``unsat_core``.
- **Interior nodes.** For each justification, fold its premises' labels with
  set union (one environment chosen per premise); the fact's label is the
  subset-minimal elements of the union over all its justifications.
- **Iterate** until no label changes. Labels start empty ("not known
  derivable") and only ever improve, so a **cyclic** justification graph is
  handled by construction — a fact can never ground itself, because at the
  moment its own label is still empty it contributes nothing. That matters:
  once re-derivations are recorded, `(R a b)` and `(R b a)` justify each
  other through the symmetric mirror in any ordinary puzzle.

Completeness caveat (unchanged by this module). The alternatives searched are
only the firings the saturator actually attempted, capped per fact by
``store.MAX_ALT_JUSTIFICATIONS``; "minimal" therefore remains relative to the
rule set and the saturation strategy. :attr:`Explanation.exhausted` reports
whether *this* search ran to completion within its budget — it says nothing
about derivations that were never recorded.
"""
from __future__ import annotations

from collections.abc import Iterable
from dataclasses import dataclass, field

from ein.kb.entities import Fact
from ein.kb.provenance import FactId, Provenance
from ein.kb.store import KnowledgeBase

# ── Budget ─────────────────────────────────────────────────────────


@dataclass(frozen=True)
class ExplanationBudget:
    """Anytime budget for the label search.

    Minimum-cardinality source frontier over an AND/OR graph is worst-case
    exponential (it is why ATMS labels blow up), so every axis along which
    the search can explode is capped. Every cap is *sound* — truncation
    only ever discards environments, and each surviving environment is
    still a real set of derivation leaves — but a truncated search may miss
    a smaller one, which is what :attr:`Explanation.exhausted` reports.

    Attributes:
        max_environments: environments kept per fact label, smallest first.
            The dominant memory/time knob.
        max_rounds: fixpoint iterations before giving up. A converged search
            normally needs about the depth of the derivation graph.
        max_env_size: drop environments larger than this. ``None`` (default)
            keeps all sizes; setting it turns the search into "is there an
            explanation of at most N givens?", which is much cheaper.
        max_facts: refuse to search a premise closure larger than this,
            returning the recorded-derivation frontier instead of hanging.
    """
    max_environments: int = 64
    max_rounds: int = 64
    max_env_size: int | None = None
    max_facts: int = 20_000


DEFAULT_BUDGET = ExplanationBudget()


# ── Result ─────────────────────────────────────────────────────────


@dataclass(frozen=True)
class Explanation:
    """A minimal-cardinality set of given facts that forces the target.

    Attributes:
        frontier: the explanation — ``source`` / ``hypothesis`` / given
            facts whose joint truth derives the target through *some*
            recorded derivation. Empty when the target is not derivable
            from the frontier at all (or when there is no target).
        target: the fact explained — for a contradiction search, the
            witness whose explanation won.
        exhausted: True iff no budget cap was hit anywhere in the search,
            i.e. :attr:`frontier` is a true minimum **over the recorded
            derivations**. False means "sound, but possibly not smallest".
        rounds: fixpoint iterations actually run.
        facts_considered: size of the premise closure the search covered.
    """
    frontier: frozenset[Fact] = frozenset()
    target: Fact | None = None
    exhausted: bool = True
    rounds: int = 0
    facts_considered: int = 0

    def __len__(self) -> int:
        return len(self.frontier)

    def __iter__(self):
        return iter(self.frontier)


# ── Internals ──────────────────────────────────────────────────────

# A label is a tuple of environments (frozensets of frontier FactIds),
# kept subset-minimal and sorted smallest-first.
Env = frozenset
Label = tuple


@dataclass
class _Graph:
    """The AND/OR premise closure of a set of targets."""
    index: dict[FactId, Fact] = field(default_factory=dict)
    # fid -> justifications (each an AND-node over premise fids)
    just: dict[FactId, tuple[tuple[FactId, ...], ...]] = field(default_factory=dict)
    # fid -> the facts whose justifications name it (propagation edges)
    consumers: dict[FactId, set[FactId]] = field(default_factory=dict)
    # terminals with a fixed label
    seed: dict[FactId, tuple] = field(default_factory=dict)
    # fid -> a stable integer, assigned once in repr order. FactIds are not
    # orderable (an arg may be a nested `Fact`, which has no `__lt__`), and
    # `repr` in an inner loop is far too slow, so every tie-break and every
    # deterministic iteration in the search goes through this.
    rank: dict[FactId, int] = field(default_factory=dict)
    truncated: bool = False

    def key(self, env: Env) -> tuple:
        """Deterministic sort key for an environment: size, then contents."""
        return (len(env), tuple(sorted(self.rank.get(x, -1) for x in env)))


def _is_frontier(prov: Provenance | None) -> bool:
    """The derivation frontier — what the engine treats as *given*.

    Identical to :meth:`KnowledgeBase.unsat_core`'s predicate, so an
    environment this module returns is drawn from exactly the same
    population as the recorded-derivation core.
    """
    return prov is None or prov.kind in ("source", "hypothesis")


def _build_graph(
    kb: KnowledgeBase, targets: Iterable[Fact], budget: ExplanationBudget,
) -> _Graph:
    """Collect the AND/OR premise closure of ``targets``.

    One pass over ``kb.facts`` builds a FactId index (``_fact_by_id`` is an
    O(deg) scan, and this walk would call it once per premise), then a
    worklist expands every justification of every reached fact.
    """
    g = _Graph()
    g.index = {}
    for f in kb.facts:
        g.index.setdefault((f.relation_name, f.args), f)

    stack: list[FactId] = []
    for t in targets:
        tid: FactId = (t.relation_name, t.args)
        if tid not in g.just and tid not in g.seed:
            stack.append(tid)
            g.just[tid] = ()
    while stack:
        fid = stack.pop()
        fact = g.index.get(fid)
        if fact is None:
            # Dangling premise id — the same case `walk_premises` skips.
            # Ground it out (contributes nothing) rather than killing the
            # justification that named it, so parity with the recorded walk
            # is preserved.
            g.seed[fid] = (Env(),)
            g.just.pop(fid, None)
            continue
        if _is_frontier(fact.provenance):
            g.seed[fid] = (Env((fid,)),)
            g.just.pop(fid, None)
            continue
        provs = kb.justifications(fact)
        ands: list[tuple[FactId, ...]] = []
        for p in provs:
            if p.kind != "rule":
                continue
            if not p.premises_raw:
                ands = []                       # synthetic ground-out wins
                break
            ands.append(tuple(p.premises_raw))
        if not ands:
            g.seed[fid] = (Env(),)
            g.just.pop(fid, None)
            continue
        g.just[fid] = tuple(ands)
        for and_node in ands:
            for pid in and_node:
                g.consumers.setdefault(pid, set()).add(fid)
                if pid not in g.just and pid not in g.seed:
                    g.just[pid] = ()
                    stack.append(pid)
        if len(g.just) + len(g.seed) > budget.max_facts:
            g.truncated = True
            break
    g.rank = {
        fid: i for i, fid in enumerate(
            sorted({*g.just, *g.seed}, key=repr))
    }
    return g


def _minimise(
    envs: Iterable[Env], g: _Graph, budget: ExplanationBudget,
) -> tuple[Label, bool]:
    """Subset-minimal environments, smallest first, capped.

    Returns ``(label, truncated)``. Discarding a superset is exact — a
    superset can never be a minimum-cardinality answer, nor lead to one,
    since union is monotone — so only the cap loses information.
    """
    ordered = sorted(set(envs), key=g.key)
    out: list[Env] = []
    for e in ordered:
        if budget.max_env_size is not None and len(e) > budget.max_env_size:
            continue
        if any(m <= e for m in out):
            continue                            # superset of a kept one
        out.append(e)
        if len(out) >= budget.max_environments:
            return tuple(out), len(out) < len(ordered)
    return tuple(out), False


def _fold(
    and_node: tuple[FactId, ...],
    labels: dict[FactId, Label],
    g: _Graph,
    budget: ExplanationBudget,
) -> tuple[list[Env], bool]:
    """Environments of one justification: union one env per premise.

    A beam fold — after each premise the working set is re-minimised and
    capped — so the cost is bounded by ``max_environments`` per premise
    instead of by the full cross-product over all of them.
    """
    envs: list[Env] = [Env()]
    truncated = False
    for pid in and_node:
        plabel = labels.get(pid, ())
        if not plabel:
            return [], False                    # premise not derivable yet
        merged = [e | pe for e in envs for pe in plabel]
        envs_t, cut = _minimise(merged, g, budget)
        truncated = truncated or cut
        envs = list(envs_t)
        if not envs:
            return [], truncated
    return envs, truncated


def _propagate(
    g: _Graph, budget: ExplanationBudget,
) -> tuple[dict[FactId, Label], int, bool]:
    """Least-fixpoint label propagation. Returns (labels, rounds, exhausted).

    Labels start empty ("no known derivation") and grow, so the fixpoint is
    reached from below — which is what makes a cyclic justification graph
    safe: a fact contributes to its own consumers only once it has a label
    that did not come through them.
    """
    labels: dict[FactId, Label] = dict(g.seed)
    truncated = g.truncated
    # Seed the worklist with the consumers of every terminal; a fact with no
    # derivable premise simply never acquires a label.
    dirty: set[FactId] = set()
    for fid in g.seed:
        dirty |= g.consumers.get(fid, set())
    rounds = 0
    while dirty and rounds < budget.max_rounds:
        rounds += 1
        wave, dirty = dirty, set()
        for fid in sorted(wave, key=lambda k: g.rank.get(k, -1)):
            ands = g.just.get(fid)
            if not ands:
                continue
            candidates: list[Env] = []
            for and_node in ands:
                envs, cut = _fold(and_node, labels, g, budget)
                truncated = truncated or cut
                candidates.extend(envs)
            if not candidates:
                continue
            new, cut = _minimise(candidates, g, budget)
            truncated = truncated or cut
            if new != labels.get(fid, ()):
                labels[fid] = new
                dirty |= g.consumers.get(fid, set())
    return labels, rounds, not (truncated or bool(dirty))


# ── Public API ─────────────────────────────────────────────────────


def explain(
    kb: KnowledgeBase,
    targets: Iterable[Fact],
    *,
    budget: ExplanationBudget | None = None,
) -> Explanation:
    """Smallest recorded-derivation frontier that forces *some* target.

    Each target is explained independently (they are alternatives, not a
    conjunction — the caller asking "why is this KB contradictory?" wants
    the single most legible witness, not the union over all of them), and
    the smallest explanation across targets wins.

    Ties break deterministically on the sorted fact repr, so the result does
    not depend on set iteration order, dict order, or — the whole point of
    S1.21.7 — the order in which the rules happened to fire.
    """
    budget = budget or DEFAULT_BUDGET
    targets = list(targets)
    if not targets:
        return Explanation()
    g = _build_graph(kb, targets, budget)
    labels, rounds, exhausted = _propagate(g, budget)

    best: tuple[tuple, Fact, Env] | None = None
    for t in targets:
        tid: FactId = (t.relation_name, t.args)
        label = labels.get(tid, ())
        if not label:
            continue
        env = label[0]                          # smallest, already sorted
        key = (*g.key(env), g.rank.get(tid, -1))
        if best is None or key < best[0]:
            best = (key, t, env)
    considered = len(g.just) + len(g.seed)
    if best is None:
        if g.truncated:
            # The closure blew the `max_facts` valve before any target got a
            # label. Degrade to the pre-S1.21.7 answer — the smallest
            # *recorded-primary* derivation frontier — rather than returning
            # nothing or, worse, passing derived facts off as givens. Sound
            # and cheap; flagged not-exhausted so the caller knows the
            # minimality search never ran.
            return _recorded_fallback(kb, targets, rounds, considered)
        return Explanation(
            exhausted=exhausted, rounds=rounds, facts_considered=considered,
        )
    _key, target, env = best
    frontier = frozenset(
        f for fid in env if (f := g.index.get(fid)) is not None
    )
    return Explanation(
        frontier=frontier, target=target, exhausted=exhausted,
        rounds=rounds, facts_considered=considered,
    )


def _recorded_fallback(
    kb: KnowledgeBase, targets: list[Fact], rounds: int, considered: int,
) -> Explanation:
    """Pre-S1.21.7 answer: smallest single-target recorded-primary frontier.

    The graceful degradation when the AND/OR search cannot run. Always
    sound — it is a real derivation's leaves — just order-dependent, which
    is what ``exhausted=False`` is telling the caller.
    """
    best: tuple[int, str, Fact, frozenset[Fact]] | None = None
    for t in targets:
        core = frozenset(kb.unsat_core([t]))
        if not core:
            continue
        key = (len(core), " ".join(sorted(repr(f) for f in core)))
        if best is None or key < (best[0], best[1]):
            best = (key[0], key[1], t, core)
    if best is None:
        return Explanation(
            exhausted=False, rounds=rounds, facts_considered=considered)
    return Explanation(
        frontier=best[3], target=best[2], exhausted=False,
        rounds=rounds, facts_considered=considered,
    )


def minimal_contradiction_frontier(
    kb: KnowledgeBase,
    witnesses: Iterable[Fact] | None = None,
    *,
    budget: ExplanationBudget | None = None,
) -> Explanation:
    """Smallest explanation of any contradiction in ``kb``.

    ``witnesses`` defaults to the witness fact of every
    :class:`~ein.inference.contradiction.Contradiction` the detector finds;
    an empty/consistent KB yields an empty :class:`Explanation`.

    Unlike
    :func:`~ein.inference.frontier.smallest_contradiction_frontier`'s
    pre-S1.21.7 behaviour, this searches *across* recorded derivations, so
    the answer no longer depends on which derivation the dedup seam happened
    to keep.
    """
    if witnesses is None:
        from .contradiction import ContradictionDetector
        witnesses = [c.witness for c in ContradictionDetector(kb).detect()]
    return explain(kb, witnesses, budget=budget)


__all__ = [
    "DEFAULT_BUDGET",
    "Explanation",
    "ExplanationBudget",
    "explain",
    "minimal_contradiction_frontier",
]
