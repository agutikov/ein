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

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

from .config import SolverConfig
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
    objects = list(_instance_like_objects(kb))
    if not objects:
        return
    # S1.5.6 T1.5.6.2 — one-step lookahead, gated by config. Built
    # once per call (compiles the rule plans); reused per candidate.
    cfg: SolverConfig = kb.config or SolverConfig()
    lookahead = Lookahead(kb) if cfg.enable_pre_branch_lookahead else None
    by_count = sorted(
        objects,
        key=lambda nr: (
            -(len(nr.as_head) + len(nr.as_arg)),
            nr.name,
        ),
    )
    seen: set[tuple[str, tuple]] = set()
    for obj_ref in by_count:
        for fact in _raw_candidates(kb, obj_ref, stats):
            stats.raw += 1
            kept = _apply_filters(kb, fact, seen, stats, lookahead)
            if kept:
                stats.emitted += 1
                yield fact


def _apply_filters(
    kb: KnowledgeBase,
    fact: Fact,
    seen: set[tuple[str, tuple]],
    stats: HypGenStats,
    lookahead: Lookahead | None,
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
) -> Iterator[Fact]:
    """Enumerate every type-compatible non-self-edge candidate fact.

    Pre-candidate skips (closed_relation, type_incompatible_slot,
    self_edge) increment ``stats.pre_candidate``; the per-candidate
    filters fire downstream in :func:`_apply_filters`.
    """
    for rel in kb.relations.values():
        if not rel.signature:
            continue
        if _is_closed(kb, rel.name):
            # T1.5.4.1 — `(closed R)` activator. The whole
            # relation contributes zero candidates.
            stats.pre_candidate["closed_relation"] += 1
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
]
