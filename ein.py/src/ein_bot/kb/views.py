"""Layer views + encoding-agnostic helpers — S1.2.2.

Three things live here:

1. :class:`FactView` — a read-only filtered window over a sequence of
   facts. Used by `kb.ontology()` / `kb.fact_layer()` /
   `kb.reasoning()` / `kb.all_layers()` to expose layer-scoped queries
   without rebuilding the underlying KB. (The `FACT` layer accessor
   is named `fact_layer()`, not `facts()`, to avoid shadowing the
   `kb.facts: list[Fact]` registry attribute from S1.2.1.)

2. :func:`logical_types` — encoding-agnostic answer to "what are the
   type-like nodes in this KB?". Unions `kb.types.values()` with the
   right-hand sides of `is-a` facts so downstream code (renderer,
   trace planner, hypothesis branching) doesn't have to know whether
   the puzzle was encoded with classic `(type …)` / `(instance …)`
   declarations or with the unified `(is-a Child Parent)` form.
   The IR encoding decision is deferred to P1.7 S1.7.2 T1.7.2.5
   (memory: project — IR encoding choice deferred); this helper is
   the bridge.

3. :func:`logical_instances` — the dual: what are the leaf
   instance-like nodes? For classic encoding that's
   `kb.instances.values()`. For unified-is-a, that's the leaves of
   the is-a forest — nodes that appear as the left-hand side of an
   is-a fact but not the right-hand side of any other.

Pattern-based filtering (`view.matching(pattern)`) is deferred to
P1.3 where the matcher lives; the seam is recorded here in the
:meth:`FactView.matching` stub.
"""
from __future__ import annotations

from collections.abc import Iterator
from dataclasses import dataclass
from typing import TYPE_CHECKING

from .entities import Fact, Instance, Type

if TYPE_CHECKING:
    from .store import KnowledgeBase


# ── FactView ───────────────────────────────────────────────────────


@dataclass(frozen=True)
class FactView:
    """A read-only window over a tuple of facts.

    Constructed by `KnowledgeBase.ontology()` etc.; the `_kb` field is
    used so view methods can resolve names to entities (e.g.
    ``view.about(Instance(...))`` accepts an Instance OR its name str).

    The view is *eager* — `_facts` is materialised at construction
    time. Iteration order is the order in which facts were ingested
    (i.e. the IR file order, plus any reasoning-layer additions in
    insertion order).
    """
    _facts: tuple[Fact, ...]
    _kb: KnowledgeBase
    _label: str = "all"

    # ── Sequence protocol ─────────────────────────────────────────

    def __iter__(self) -> Iterator[Fact]:
        return iter(self._facts)

    def __len__(self) -> int:
        return len(self._facts)

    def __contains__(self, fact: Fact) -> bool:
        return fact in self._facts

    def __bool__(self) -> bool:
        return bool(self._facts)

    def __repr__(self) -> str:
        return f"<FactView {self._label!r} len={len(self._facts)}>"

    # ── Filters (return Iterator, lazy) ───────────────────────────

    def relation(self, name: str) -> Iterator[Fact]:
        """Facts in this view whose head relation is `name`."""
        for f in self._facts:
            if f.relation_name == name:
                yield f

    def about(self, target: Instance | str) -> Iterator[Fact]:
        """Facts in this view mentioning the given instance (any arg position).

        Accepts either an :class:`Instance` object or its name string.
        """
        name = target.name if isinstance(target, Instance) else target
        for f in self._facts:
            if name in f.args:
                yield f

    def by_source(self, source: str) -> Iterator[Fact]:
        """Facts whose `:source` annotation matches.

        Useful in the FACT view to find "condition (10)" etc.
        """
        for f in self._facts:
            if f.source == source:
                yield f

    def by_rule(self, rule_name: str) -> Iterator[Fact]:
        """Facts whose `:rule` provenance equals `rule_name`.

        Useful in the REASONING view to find all firings of a
        specific rule.
        """
        for f in self._facts:
            if f.rule_name == rule_name:
                yield f

    # ── Pattern matcher hook (P1.3) ───────────────────────────────

    def matching(self, pattern):  # pragma: no cover — P1.3 seam
        """Facts matching a :class:`Pattern`. Implemented in P1.3."""
        raise NotImplementedError(
            "Pattern-based filtering lives in P1.3 (rule matcher)."
        )


# ── Encoding-agnostic "logical type" / "logical instance" views ────


def logical_types(kb: KnowledgeBase) -> tuple[Type | str, ...]:
    """Return the type-like nodes regardless of IR encoding.

    For zebra.ein (classic) this returns the seven `Type` entities
    declared via `(type …)`.

    For zebra2.ein (unified is-a) `kb.types` is empty; this helper
    returns the right-hand sides of `(is-a Child Parent)` facts —
    e.g. `Attribute`, `Nationality`, `House`, …. These are returned
    as **names** (strings), because no `Type` entity has been built
    for them; promotion to typed entities is a P1.3 / P1.7 question.

    The result mixes `Type` and `str` so callers can use the same
    `.name`-style attribute via :func:`type_name`.
    """
    if kb.types:
        return tuple(kb.types.values())
    seen: dict[str, None] = {}
    for f in kb.facts:
        if f.relation_name == "is-a" and len(f.args) >= 2:
            parent = f.args[1]
            if isinstance(parent, str) and parent not in seen:
                seen[parent] = None
    return tuple(seen)


def logical_instances(kb: KnowledgeBase) -> tuple[Instance | str, ...]:
    """Return the leaf-like nodes regardless of IR encoding.

    For classic encoding: `kb.instances.values()`.

    For unified is-a: leaves of the is-a forest — names that appear
    as ``args[0]`` of an `is-a` fact and never as ``args[1]``.
    """
    if kb.instances:
        return tuple(kb.instances.values())
    children: dict[str, None] = {}
    parents: set[str] = set()
    for f in kb.facts:
        if f.relation_name == "is-a" and len(f.args) >= 2:
            child = f.args[0]
            parent = f.args[1]
            if isinstance(child, str):
                children[child] = None
            if isinstance(parent, str):
                parents.add(parent)
    return tuple(name for name in children if name not in parents)


def type_name(t: Type | str) -> str:
    """Best-effort name accessor for the logical-types union return."""
    return t.name if isinstance(t, Type) else t


def instance_name(i: Instance | str) -> str:
    """Best-effort name accessor for the logical-instances union return."""
    return i.name if isinstance(i, Instance) else i


__all__ = [
    "FactView",
    "instance_name",
    "logical_instances",
    "logical_types",
    "type_name",
]
