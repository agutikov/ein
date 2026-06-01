"""Layer views — S1.2.2.

:class:`FactView` — a read-only filtered window over a sequence of facts.
Used by `kb.ontology()` / `kb.fact_layer()` / `kb.reasoning()` /
`kb.all_layers()` to expose layer-scoped queries without rebuilding the
underlying KB. (The `FACT` layer accessor is named `fact_layer()`, not
`facts()`, to avoid shadowing the `kb.facts: list[Fact]` registry
attribute from S1.2.1.)

S1.7.23 — the `logical_types` / `logical_instances` / `type_name` /
`instance_name` helpers were DELETED. They were the encoding-agnostic
`is-a`-bridge for the now-removed `kb.types` / `kb.instances` type-system
entity-view; a puzzle that wants a named-type projection computes it with
an ein-lang rule over its own inheritance relation, and the renderer reads
`is-a` facts directly.

Pattern-based filtering (`view.matching(pattern)`) is deferred to
P1.3 where the matcher lives; the seam is recorded here in the
:meth:`FactView.matching` stub.
"""
from __future__ import annotations

from collections.abc import Iterator
from dataclasses import dataclass
from typing import TYPE_CHECKING

from .entities import Fact

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

    def about(self, target: str) -> Iterator[Fact]:
        """Facts in this view mentioning the given node name (any arg
        position)."""
        for f in self._facts:
            if target in f.args:
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


__all__ = [
    "FactView",
]
