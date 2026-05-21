"""Hypothesis generation — the two-step "pick object, pick relation"
enumerator that produces candidate Facts for the search tree.

Step 1 — order *instance-like* objects (graph leaves of `is-a` /
`instance`, plus the kernel `kb.instances` view as a fallback) by
descending fact-participation; ties broken by name.
Step 2 — per object, enumerate `(relation, slot)` pairs the object
doesn't already occupy, fill the other slot with type-compatible
instance-like objects, prune by `(not …)` exclusions and emit both
orderings for symmetric relations.

Encoding-agnostic across zebra-original (kernel `(instance N T)`)
and zebra2 (`is-a` leaves) — see [[project-canonical-zebra2]] and
docs/kernel/ir/01-ein-graph/03_ein_model.md §6.
"""
from __future__ import annotations

from collections.abc import Iterator

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

# Inheritance-relation names the generator recognises when walking
# ancestry. Both legacy kernel `instance` and canonical zebra2 `is-a`
# are treated equivalently — the type-compat walk follows whichever
# the puzzle uses (or both, if a compatibility layer is loaded).
INHERITANCE_RELATIONS: tuple[str, ...] = ("is-a", "instance")


def generate_hypotheses(kb: KnowledgeBase) -> Iterator[Fact]:
    """Yield candidate hypothesis facts in priority order.

    Same-call dedup: a fact yielded once (by identity tuple
    `(relation_name, args)`) is not yielded again — both Alice and
    Bob enumerate `(r Alice Bob)` from their respective candidate
    slots, but only the first is yielded.
    """
    objects = list(_instance_like_objects(kb))
    if not objects:
        return
    by_count = sorted(
        objects,
        key=lambda nr: (
            -(len(nr.as_head) + len(nr.as_arg)),
            nr.name,
        ),
    )
    seen: set[tuple[str, tuple]] = set()
    for obj_ref in by_count:
        for h in _hypotheses_for(kb, obj_ref):
            key = (h.relation_name, h.args)
            if key in seen:
                continue
            seen.add(key)
            yield h


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


def _hypotheses_for(kb: KnowledgeBase, obj_ref) -> Iterator[Fact]:
    # S1.5.4b: Filter B ("slot already used" — skip (R, slot_idx) if
    # `obj_ref` already sits there in some fact) is INTENTIONALLY
    # removed. The principled architecture: let user rules express
    # the functional constraint via (functional R) / (sibling-
    # exclusive …), let those rules' :assert (not h) populate
    # kb._negated_facts, let Filter C (T1.5.4.7's `negated_fact`
    # filter) drop h in O(1). Filter B was only sound for
    # functional-on-slot relations — broke open-world multi-image
    # relations like `friends-with`. See
    # plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.4b-fix-filter-slot-already-used.md
    for rel in kb.relations.values():
        if not rel.signature:
            continue
        for slot_idx, sig_type in enumerate(rel.signature):
            if not _type_compatible(kb, obj_ref.name, sig_type):
                continue
            yield from _fill_slot(kb, rel, slot_idx, obj_ref)


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


def _fill_slot(kb: KnowledgeBase, rel,
               fixed_slot: int, obj_ref) -> Iterator[Fact]:
    """Enumerate type-compatible fillers; emit symmetric duplicates."""
    if len(rel.signature) != 2:
        return     # M1 only handles arity-2 relations
    other_slot = 1 - fixed_slot
    other_type = rel.signature[other_slot]
    symmetric = _is_symmetric(kb, rel.name)

    for filler in _instance_like_objects(kb):
        if filler.name == obj_ref.name:
            continue        # skip self-edges
        if not _type_compatible(kb, filler.name, other_type):
            continue

        # Build args for the chosen slot assignment.
        args = _build_args(obj_ref.name, fixed_slot, filler.name, other_slot)
        fact = Fact(
            relation_name=rel.name,
            args=args,
            layer=Layer.REASONING,
            provenance=None,    # caller adds Provenance.from_hypothesis later
        )
        if not _is_excluded(kb, fact):
            yield fact

        # Symmetric R: emit the reversed ordering too.
        if symmetric:
            rev_args = _build_args(filler.name, fixed_slot, obj_ref.name, other_slot)
            rev = Fact(
                relation_name=rel.name,
                args=rev_args,
                layer=Layer.REASONING,
                provenance=None,
            )
            if not _is_excluded(kb, rev):
                yield rev


def _build_args(a_name: str, a_slot: int,
                b_name: str, b_slot: int) -> tuple[str, ...]:
    """Place two named values into a 2-tuple at the given slots."""
    args: list[str] = ["", ""]
    args[a_slot] = a_name
    args[b_slot] = b_name
    return tuple(args)


def _is_symmetric(kb: KnowledgeBase, r_name: str) -> bool:
    apps = kb._facts_by_relation.get("symmetric", ())
    return any(f.args == (r_name,) for f in apps)


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


__all__ = [
    "INHERITANCE_RELATIONS",
    "generate_hypotheses",
]
