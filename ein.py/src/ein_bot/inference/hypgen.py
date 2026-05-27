"""Hypothesis generation — the two-step "pick object, pick relation"
enumerator that produces candidate Facts for the search tree.

Step 1 — order *instance-like* objects (graph leaves of `is-a` /
`instance`, plus the kernel `kb.instances` view as a fallback) by
descending fact-participation; ties broken by name.
Step 2 — per object, enumerate `(relation, slot)` pairs, fill the
other slot with type-compatible instance-like objects, prune by
the named filter pipeline (see :class:`HypGenStats`), and emit
both orderings for symmetric relations.

T1.5.4.7 — the per-filter counter refactor. User observation
2026-05-21: *"I think we can't 'not generate' some hypothesis
directly, we can only filter, so count raw generated hypothesis
and filtered by every condition."* :func:`generate_hypotheses`
is the iterator API (existing call sites unchanged);
:func:`generate_hypotheses_with_stats` materialises and returns
the counter dataclass alongside.

Encoding-agnostic across zebra-original (kernel `(instance N T)`)
and zebra2 (`is-a` leaves) — see [[project-canonical-zebra2]] and
docs/kernel/ir/01-ein-graph/03_ein_model.md §6.
"""
from __future__ import annotations

from collections import defaultdict
from collections.abc import Iterator
from dataclasses import dataclass, field

from ein_bot.ir.types import Atom, SForm
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

from .tree.back_prop import back_propagate
from .config import SolverConfig
from .hrule import Hrules
from .lookahead import Lookahead

# Inheritance-relation names the generator recognises when walking
# ancestry. Both legacy kernel `instance` and canonical zebra2 `is-a`
# are treated equivalently — the type-compat walk follows whichever
# the puzzle uses (or both, if a compatibility layer is loaded).
INHERITANCE_RELATIONS: tuple[str, ...] = ("is-a", "instance")


# ── Stats dataclass — T1.5.4.7 ─────────────────────────────────────


@dataclass
class HypGenStats:
    """Per-filter counters for one ``generate_hypotheses`` call.

    Counts are split into three groups:

    - **pre_candidate**: structural skips at the relation/slot level,
      before any single candidate fact is constructed. Today:
      ``closed_relation`` (relation skipped because `(closed R)`),
      ``relation_not_whitelisted`` (relation absent from the
      `(config :hypothesis-relations …)` whitelist),
      ``type_incompatible_slot`` (slot skipped because the focal
      object's type doesn't match the slot's signature),
      ``self_edge`` (filler equals focal object).
    - **raw**: number of constructed candidate facts entering the
      per-candidate filter pipeline. Equals
      ``emitted + sum(filtered.values())``.
    - **filtered**: per-named-filter drop counts at the candidate
      level. Today: ``negated_fact`` (Tier-A
      ``_negated_facts`` membership), ``fact_already_exists``
      (S1.5.4b narrower replacement for Filter B),
      ``lookahead_killed`` (S1.5.6 one-step lookahead — the
      candidate dies against the saturated KB in a single rule
      firing), ``seen_in_call`` (same-call duplicate from another
      focal object's perspective).
    - **emitted**: facts actually yielded to the caller.

    Invariant: ``raw == emitted + sum(filtered.values())``.

    Adding a new candidate-level filter means bumping a new
    ``filtered`` key and adding the filter step in
    :func:`_apply_filters`. Pre-candidate skips bump
    ``pre_candidate`` directly at the skip site.
    """
    raw: int = 0
    emitted: int = 0
    filtered: dict[str, int] = field(default_factory=lambda: defaultdict(int))
    pre_candidate: dict[str, int] = field(default_factory=lambda: defaultdict(int))

    def total_filtered(self) -> int:
        return sum(self.filtered.values())

    def as_report_lines(self) -> list[str]:
        """Human-readable per-line summary for `bench_solve --hyp-stats`."""
        lines = [
            f"  raw                {self.raw}",
            f"  emitted            {self.emitted}",
        ]
        for k in sorted(self.filtered):
            lines.append(f"  filtered.{k:18s} {self.filtered[k]}")
        for k in sorted(self.pre_candidate):
            lines.append(f"  pre.{k:23s} {self.pre_candidate[k]}")
        return lines


# ── Public API ─────────────────────────────────────────────────────


def generate_hypotheses(kb: KnowledgeBase) -> Iterator[Fact]:
    """Yield candidate hypothesis facts in priority order.

    Iterator API: discards the stats. For per-filter counters, use
    :func:`generate_hypotheses_with_stats` (returns
    ``(list[Fact], HypGenStats)``).

    Same-call dedup: a fact yielded once (by identity tuple
    ``(relation_name, args)``) is not yielded again — both Alice
    and Bob enumerate ``(r Alice Bob)`` from their respective
    candidate slots, but only the first is yielded.
    """
    yield from _generate(kb, HypGenStats())


def generate_hypotheses_with_stats(
    kb: KnowledgeBase,
) -> tuple[list[Fact], HypGenStats]:
    """Materialised + counted: returns ``(facts, stats)`` tuple.

    Used by ``bench_solve --hyp-stats`` and by the SearchTree's
    root-metadata stash (T1.5.4.7.d). The list is the same content
    :func:`generate_hypotheses` would yield, in the same order.
    """
    stats = HypGenStats()
    facts = list(_generate(kb, stats))
    return facts, stats


# ── Internal generator + filter pipeline ───────────────────────────


def _generate(kb: KnowledgeBase, stats: HypGenStats) -> Iterator[Fact]:
    cfg: SolverConfig = kb.config or SolverConfig()
    # S1.5.6 T1.5.6.2 — one-step lookahead, gated by config. Built
    # once per call (compiles the rule plans); reused per candidate.
    lookahead = Lookahead(kb) if cfg.enable_pre_branch_lookahead else None
    # S1.5.6b T1.5.6b.1 — relation whitelist from the (query …)
    # block. None ⇒ no restriction.
    allowed = _query_relations(kb)
    seen: set[tuple[str, tuple]] = set()

    def _emit(fact: Fact) -> Iterator[Fact]:
        stats.raw += 1
        if _apply_filters(kb, fact, seen, stats, lookahead, cfg):
            stats.emitted += 1
            yield fact

    # S1.5.6b — generation is rule-driven when the puzzle declares
    # any `(hrule …)`; otherwise the blind combinatorial enumerator
    # runs. The two never both run: hrule presence *is* the switch.
    if kb.hrules:
        for fact in Hrules(kb).candidates(kb):
            yield from _emit(fact)
    else:
        by_count = sorted(
            _instance_like_objects(kb),
            key=lambda nr: (
                -(len(nr.as_head) + len(nr.as_arg)),
                nr.name,
            ),
        )
        for obj_ref in by_count:
            for fact in _raw_candidates(kb, obj_ref, stats, allowed):
                yield from _emit(fact)


def _apply_filters(
    kb: KnowledgeBase,
    fact: Fact,
    seen: set[tuple[str, tuple]],
    stats: HypGenStats,
    lookahead: Lookahead | None,
    cfg: SolverConfig,
) -> bool:
    """Run the candidate-level filter pipeline; return True iff kept.

    Filter order matters only for the counter attribution — the
    first filter to drop a candidate gets the bump; later filters
    aren't asked. Order chosen so the cheapest checks run first.
    """
    # negated_fact (Tier A; O(1)).
    if _is_excluded(kb, fact):
        stats.filtered["negated_fact"] += 1
        return False
    # fact_already_exists (S1.5.4b narrower replacement for Filter B; O(1)).
    if _already_a_fact(kb, fact):
        stats.filtered["fact_already_exists"] += 1
        return False
    # lookahead_killed (S1.5.6 Tier B; one rule step, no fork). Runs
    # last of the per-candidate checks — it is the costliest.
    if lookahead is not None and lookahead.dies_immediately(kb, fact):
        stats.filtered["lookahead_killed"] += 1
        # T1.5.7.4 — a lookahead kill consults no ancestor hypothesis
        # (the simulator runs one rule step against the already-
        # saturated parent KB), so the death is unconditional by
        # construction. Cache it as a back-propped `(not h)` so
        # subsequent enumerations / branches inherit the exclusion in
        # O(1) instead of re-running the lookahead's match.
        if cfg.enable_back_prop_unconditional:
            back_propagate(kb, fact, frozenset(),
                           rule_name="<lookahead-dies-immediately>")
        return False
    # seen_in_call dedup (stateful across the call).
    key = (fact.relation_name, fact.args)
    if key in seen:
        stats.filtered["seen_in_call"] += 1
        return False
    seen.add(key)
    return True


def _raw_candidates(
    kb: KnowledgeBase, obj_ref, stats: HypGenStats,
    allowed: frozenset[str] | None,
) -> Iterator[Fact]:
    """Enumerate every type-compatible non-self-edge candidate fact.

    Pre-candidate skips (closed_relation, relation_not_whitelisted,
    type_incompatible_slot, self_edge) increment
    ``stats.pre_candidate``; the per-candidate filters fire
    downstream in :func:`_apply_filters`. ``allowed`` is the
    S1.5.6b relation whitelist (``None`` ⇒ no restriction).
    """
    for rel in kb.relations.values():
        if not rel.signature:
            continue
        if _is_closed(kb, rel.name):
            # T1.5.4.1 — `(closed R)` activator. The whole
            # relation contributes zero candidates.
            stats.pre_candidate["closed_relation"] += 1
            continue
        if allowed is not None and rel.name not in allowed:
            # S1.5.6b T1.5.6b.1 — relation not on the whitelist.
            stats.pre_candidate["relation_not_whitelisted"] += 1
            continue
        for slot_idx, sig_type in enumerate(rel.signature):
            if not _type_compatible(kb, obj_ref.name, sig_type):
                stats.pre_candidate["type_incompatible_slot"] += 1
                continue
            yield from _fill_slot(kb, rel, slot_idx, obj_ref, stats)


def _fill_slot(
    kb: KnowledgeBase,
    rel,
    fixed_slot: int,
    obj_ref,
    stats: HypGenStats,
) -> Iterator[Fact]:
    """Enumerate type-compatible fillers; emit symmetric duplicates.

    S1.5.4b: Filter B ("slot already used" — skip (R, slot_idx)
    when ``obj_ref`` already sits there) is INTENTIONALLY removed.
    Its narrower replacement (``fact_already_exists``) lives in
    :func:`_apply_filters`.
    """
    if len(rel.signature) != 2:
        return     # M1 only handles arity-2 relations
    other_slot = 1 - fixed_slot
    other_type = rel.signature[other_slot]
    symmetric = _is_symmetric(kb, rel.name)

    for filler in _instance_like_objects(kb):
        if filler.name == obj_ref.name:
            stats.pre_candidate["self_edge"] += 1
            continue
        if not _type_compatible(kb, filler.name, other_type):
            stats.pre_candidate["type_incompatible_filler"] += 1
            continue

        args = _build_args(obj_ref.name, fixed_slot, filler.name, other_slot)
        yield Fact(
            relation_name=rel.name,
            args=args,
            layer=Layer.REASONING,
            provenance=None,
        )
        if symmetric:
            rev_args = _build_args(filler.name, fixed_slot,
                                   obj_ref.name, other_slot)
            yield Fact(
                relation_name=rel.name,
                args=rev_args,
                layer=Layer.REASONING,
                provenance=None,
            )


# ── Per-candidate predicates ───────────────────────────────────────


def _already_a_fact(kb: KnowledgeBase, fact: Fact) -> bool:
    """True iff a fact with `(fact.relation_name, fact.args)` is
    already in the KB. Independent of layer / provenance —
    speculating a known fact is a no-op + a state-hash collision."""
    return kb._fact_by_id(fact.relation_name, fact.args) is not None


def _is_excluded(kb: KnowledgeBase, fact: Fact) -> bool:
    """True iff `(not <fact>)` already exists in the KB.

    O(1) lookup via the kb's `_negated_facts` set — built in
    `rebuild_indexes` and maintained incrementally in `_index_fact`.
    The set IS the dead-hypothesis cache (S1.5.4 T1.5.4.3): every
    `(not h)` derived during saturation, asserted by a rule, or
    back-propagated from a dying branch lands here and stops the
    generator from re-emitting `h` on future levels.
    """
    return (fact.relation_name, fact.args) in kb._negated_facts


def _is_symmetric(kb: KnowledgeBase, r_name: str) -> bool:
    apps = kb._facts_by_relation.get("symmetric", ())
    return any(f.args == (r_name,) for f in apps)


def _is_closed(kb: KnowledgeBase, r_name: str) -> bool:
    """True iff `(closed R)` is asserted in the KB (any layer).

    T1.5.4.1 — a closed relation contributes zero hypotheses; the
    puzzle author has fully populated it. Either authored directly
    in `(ontology …)` or asserted by a rule firing (e.g. the
    S1.5.5 `infer-closure-from-functional` rule once it ships).
    The hypothesis generator does not care which path produced the
    fact — the index lookup is the same.
    """
    apps = kb._facts_by_relation.get("closed", ())
    return any(f.args == (r_name,) for f in apps)


def _query_relations(kb: KnowledgeBase) -> frozenset[str] | None:
    """The `:hypothesis-relations` whitelist from the `(query …)` block.

    S1.5.6b T1.5.6b.1 — a query-scoped hint restricting which
    relations the enumerator builds candidates for. Returns
    ``None`` (no restriction) when the query block is absent or
    carries no `:hypothesis-relations` keyword.
    """
    q = kb.query
    if q is None:
        return None
    for kp in q.kw_pairs:
        key = getattr(kp, "key", None)
        if key is not None and getattr(key, "name", None) == "hypothesis-relations":
            return frozenset(_coerce_relation_names(kp.value)) or None
    return None


def _coerce_relation_names(value: object) -> tuple[str, ...]:
    """Relation names from a `:hypothesis-relations` value — a bare
    SYMBOL (one relation) or a `(r1 r2 …)` list."""
    if isinstance(value, Atom):
        return (value.name,)
    if isinstance(value, SForm):
        return tuple(
            p.name for p in (value.head, *value.args) if isinstance(p, Atom)
        )
    return ()


# ── Type-system walks ──────────────────────────────────────────────


def _type_compatible(kb: KnowledgeBase, obj_name: str, sig_type: str) -> bool:
    """True iff `obj_name` is `sig_type` or has it as a transitive ancestor.

    Walks both `is-a` / `instance` Facts and the kernel `Type.parent`
    chain. The convention atom `T` is treated as a universal top
    (compatible with anything) so an unrooted ontology still
    type-checks against `(relation R T T)` signatures.
    """
    if sig_type == obj_name or sig_type == "T":
        return True
    return sig_type in _ancestor_names(kb, obj_name)


def _ancestor_names(kb: KnowledgeBase, name: str) -> set[str]:
    """Transitive ancestor set under `is-a` / `instance` + kernel `Type`."""
    visited: set[str] = set()
    stack: list[str] = [name]
    while stack:
        n = stack.pop()
        for rel_name in INHERITANCE_RELATIONS:
            for f in kb._facts_by_relation.get(rel_name, ()):
                if (len(f.args) >= 2
                        and isinstance(f.args[0], str)
                        and f.args[0] == n
                        and isinstance(f.args[1], str)
                        and f.args[1] not in visited):
                    visited.add(f.args[1])
                    stack.append(f.args[1])
        t = kb.types.get(n)
        if t is not None and t.parent_name and t.parent_name not in visited:
            visited.add(t.parent_name)
            stack.append(t.parent_name)
    return visited


def score_hypothesis(fact: Fact, kb: KnowledgeBase) -> float:
    """Ordering score for a hypothesis fact — higher means tried first.

    Dispatches on :attr:`SolverConfig.hypgen_scoring`:

    - ``"most-constrained"`` (default): returns ``0`` — the sort
      falls through to the content-based tiebreaker keys in
      ``solver._candidate_sort_key``, preserving the
      [S1.5a.1a](../../../plans/m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.1a_branch_order_determinism.md)
      determinism property.
    - ``"popularity"`` (S1.5a.7 Idea 1): weighted fact-popularity
      sum. Higher-popularity relations + objects score higher;
      the intuition is "candidates touching the heavily-referenced
      parts of the fact graph are more likely to either fire
      useful rules or be quickly contradicted". Per-branch by
      construction — reads ``kb._facts_by_relation`` and
      ``kb._facts_by_instance`` which carry the branch's own
      facts (including back-propped negatives).
    - ``"branch-info"`` / ``"popularity+branch-info"``: reserved
      for a future stage; raise ``NotImplementedError`` to surface
      misconfigurations at first call. Branch-info ordering
      requires a post-saturation signal (did the branch derive
      new positives?) which isn't cheaply available pre-fork —
      designing a proxy is its own measurement task and lives
      with [S1.5.7b stable-alive
      caching](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.7b_consume_loop_stable_alive.md)
      as the natural integration point.

    Returns ``float`` — caller sort key uses ``-score_hypothesis(...)``
    so larger scores win regardless of int/float.
    """
    cfg = kb.config
    mode = (
        getattr(cfg, "hypgen_scoring", "most-constrained")
        if cfg is not None else "most-constrained"
    )
    if mode == "most-constrained":
        return 0.0
    if mode == "popularity":
        return _score_popularity(fact, kb, cfg)
    if mode in ("branch-info", "popularity+branch-info"):
        raise NotImplementedError(
            f"hypgen-scoring={mode!r} is reserved for a follow-up "
            f"stage; today only 'most-constrained' and 'popularity' "
            f"are wired. See "
            f"plans/m1_core_graph_reasoning/p1.5a_zebra_solution/"
            f"s1.5a.7_hypgen_scoring_branch_info.md § T1.5a.7.3.",
        )
    raise ValueError(
        f"unknown hypgen-scoring mode: {mode!r} "
        f"(expected 'most-constrained' or 'popularity')",
    )


def _score_popularity(
    fact: Fact, kb: KnowledgeBase, cfg,
) -> float:
    """Weighted fact-popularity score — S1.5a.7 Idea 1.

    ``score = rel_weight * count(R) + obj_weight * Σ count(arg_i)``

    Where:
    - ``count(R)`` is the length of ``kb._facts_by_relation[R]``
      (number of facts using this relation in the current branch).
    - ``count(arg_i)`` for a string-valued arg is the length of
      ``kb.names[arg_i].as_arg`` (the encoding-agnostic
      every-name-that-appears-anywhere index — works under both
      the kernel ``(instance …)`` encoding and the zebra2
      ``(is-a X Y)`` relational encoding without gating on
      ``kb.instances``).

    Non-string args (nested Facts, ints) contribute 0 — they don't
    have a name to index by.

    Weights default to 1.0/1.0; tune via
    :attr:`SolverConfig.hypgen_rel_weight` and
    :attr:`SolverConfig.hypgen_obj_weight`.
    """
    rel_count = len(kb._facts_by_relation.get(fact.relation_name, ()))
    obj_count_sum = 0
    for a in fact.args:
        if isinstance(a, str):
            nref = kb.names.get(a)
            if nref is not None:
                obj_count_sum += len(nref.as_arg)
    rel_w = getattr(cfg, "hypgen_rel_weight", 1.0)
    obj_w = getattr(cfg, "hypgen_obj_weight", 1.0)
    return rel_w * rel_count + obj_w * obj_count_sum


def _instance_like_objects(kb: KnowledgeBase) -> Iterator:
    """Yield NameRefs that look like inheritance-relation leaves.

    A name is "instance-like" if it appears at slot 0 of an
    inheritance edge (`is-a` or `instance`) and never at slot 1.
    Kernel `kb.instances` is unioned in for zebra-original encodings
    that don't materialise `is-a` facts.
    """
    at_slot0: set[str] = set()
    at_slot1: set[str] = set()
    for rel_name in INHERITANCE_RELATIONS:
        for f in kb._facts_by_relation.get(rel_name, ()):
            if len(f.args) >= 2:
                if isinstance(f.args[0], str):
                    at_slot0.add(f.args[0])
                if isinstance(f.args[1], str):
                    at_slot1.add(f.args[1])
    leaves = at_slot0 - at_slot1
    leaves |= set(kb.instances)
    for name in leaves:
        ref = kb.names.get(name)
        if ref is not None and ref.category == "object":
            yield ref


def _build_args(a_name: str, a_slot: int,
                b_name: str, b_slot: int) -> tuple[str, ...]:
    """Place two named values into a 2-tuple at the given slots."""
    args: list[str] = ["", ""]
    args[a_slot] = a_name
    args[b_slot] = b_name
    return tuple(args)


__all__ = [
    "INHERITANCE_RELATIONS",
    "HypGenStats",
    "generate_hypotheses",
    "generate_hypotheses_with_stats",
    "score_hypothesis",
]
