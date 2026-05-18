"""Entity types of the knowledge base — S1.2.1 T1.2.1.1.

These are *typed views* over the canonical graph store; the data
model itself is the graph (per the framing locked in 8e2ef71).
Each entity carries a `_kb` back-pointer used by the cross-reference
properties (`Type.instances`, `Relation.rules`, …); the back-pointer
is set by `KnowledgeBase` after instance construction via
`object.__setattr__` so the dataclasses can stay frozen.

Identity:
    Type, Instance, Relation, Rule  — by `name` (a str).
    Fact                            — by `(relation_name, args)`.

`_kb` is excluded from `__eq__` / `__hash__` / `__repr__`; two
entities of the same kind with the same name are equal across KBs.

Cross-reference fields (`Type.instances`, `Relation.rules`, etc.) live
on the entity as ``@property`` accessors that delegate to the KB —
they return empty tuples when the entity is detached (no `_kb`).

The Pattern entity (the `:match` / `:assert` clauses) lives in
:mod:`ein_bot.kb.pattern`.
"""
from __future__ import annotations

from collections.abc import Iterator
from dataclasses import dataclass, field
from enum import Enum
from typing import TYPE_CHECKING

from ein_bot.ir.types import IRNode, Loc

if TYPE_CHECKING:
    from .pattern import Pattern
    from .store import KnowledgeBase


# ── Layer enum ─────────────────────────────────────────────────────


class Layer(Enum):
    """Three knowledge populations the KB stratifies facts into.

    ONTOLOGY  — implicit assumptions: schema, instance enumeration,
                rule-application meta-facts, structural facts derived
                from background context.
    FACT      — explicit problem statements: the puzzle's numbered
                conditions, each annotated with ``:source "(N)"``.
    REASONING — derived facts the engine produces at runtime
                (rule firings, hypotheses, contradictions).
    """
    ONTOLOGY = "ontology"
    FACT = "fact"
    REASONING = "reasoning"


# ── Helpers ────────────────────────────────────────────────────────
#
# `loc` and `_kb` are excluded from `__eq__` / `__hash__` / `__repr__`
# via `field(...)` defaults — they're metadata, not identity. Inlined
# below per RUF009 (no function-call defaults).


# ── Entities ───────────────────────────────────────────────────────


@dataclass(frozen=True)
class Type:
    """A type node in the inheritance forest.

    Roots have ``parent_name is None``; everything else inherits from
    a parent type. Multi-typing is out of scope for M1 (M1 Q23) —
    each Type has at most one direct parent.
    """
    name: str
    parent_name: str | None = None
    loc: Loc | None = field(default=None, compare=False, hash=False, repr=False)
    _kb: KnowledgeBase | None = field(default=None, compare=False, hash=False, repr=False)

    @property
    def parent(self) -> Type | None:
        if self._kb is None or self.parent_name is None:
            return None
        return self._kb.types.get(self.parent_name)

    @property
    def children(self) -> tuple[Type, ...]:
        """Direct subtypes."""
        if self._kb is None:
            return ()
        return self._kb._types_by_parent.get(self.name, ())

    def ancestors(self) -> Iterator[Type]:
        """Walk the parent chain (excluding self), nearest first."""
        cur = self.parent
        while cur is not None:
            yield cur
            cur = cur.parent

    @property
    def instances(self) -> tuple[Instance, ...]:
        """Direct instances declared `(instance _ ThisType)`."""
        if self._kb is None:
            return ()
        return self._kb._instances_by_type.get(self.name, ())

    @property
    def rules(self) -> tuple[Rule, ...]:
        """Rules whose `:match` / `:assert` patterns name this type."""
        if self._kb is None:
            return ()
        return self._kb._rules_by_type.get(self.name, ())


@dataclass(frozen=True)
class Instance:
    """A leaf node — an instance of exactly one type.

    Created from `(instance Name TypeName)` ontology forms.
    """
    name: str
    type_name: str
    loc: Loc | None = field(default=None, compare=False, hash=False, repr=False)
    _kb: KnowledgeBase | None = field(default=None, compare=False, hash=False, repr=False)

    @property
    def type(self) -> Type | None:
        if self._kb is None:
            return None
        return self._kb.types.get(self.type_name)

    @property
    def facts(self) -> tuple[Fact, ...]:
        """All facts mentioning this instance (any argument position)."""
        if self._kb is None:
            return ()
        return self._kb._facts_by_instance.get(self.name, ())


@dataclass(frozen=True)
class Relation:
    """A relation declaration — `(relation Name (T1 T2 …))`.

    `signature` holds the argument-position types **by name**; resolve
    via :attr:`signature_types` to get the corresponding `Type`
    entities (filtered to those known to the KB).

    Note: a Relation entity is also created on the fly for "open-world"
    relations — heads of facts that have no `(relation …)` declaration.
    The user-defined property tags (``symmetric``, ``transitive``,
    ``square-fwd``, …) in zebra.ein/zebra2.ein are this case: they
    appear as fact heads (``(symmetric co-located)``) without explicit
    relation declarations, because the same atom is the *name of a
    rule*. The loader auto-creates a Relation for them so they can
    participate in the cross-reference indexes; whether such a relation
    is "real" or merely a rule-application carrier is determined by
    inspecting :attr:`rule` (set iff the head matches a Rule name).
    """
    name: str
    signature: tuple[str, ...] = ()
    declared: bool = True
    loc: Loc | None = field(default=None, compare=False, hash=False, repr=False)
    _kb: KnowledgeBase | None = field(default=None, compare=False, hash=False, repr=False)

    @property
    def signature_types(self) -> tuple[Type, ...]:
        """Argument-position types as `Type` entities (known to the KB)."""
        if self._kb is None:
            return ()
        return tuple(
            self._kb.types[n] for n in self.signature if n in self._kb.types
        )

    @property
    def facts(self) -> tuple[Fact, ...]:
        """All facts whose head is this relation's name (any layer)."""
        if self._kb is None:
            return ()
        return self._kb._facts_by_relation.get(self.name, ())

    @property
    def properties(self) -> tuple[Fact, ...]:
        """Rule-application facts targeting this relation.

        I.e. facts whose head matches a Rule name AND whose argument
        list includes this relation's name. Example: in zebra.ein the
        fact ``(symmetric co-located)`` is a property of `co-located`
        — head ``symmetric`` is a rule name, and ``co-located`` is the
        target relation.
        """
        if self._kb is None:
            return ()
        return self._kb._rule_apps_on_relation.get(self.name, ())

    @property
    def rules(self) -> tuple[Rule, ...]:
        """Rules whose patterns name this relation, OR whose name is
        the head of a property fact on this relation.
        """
        if self._kb is None:
            return ()
        return self._kb._rules_by_relation.get(self.name, ())

    @property
    def rule(self) -> Rule | None:
        """If this relation's name matches a Rule, that Rule.

        Non-None for the user-defined property tags (``symmetric``,
        ``transitive``, …) whose head is a generic property carrier.
        """
        if self._kb is None:
            return None
        return self._kb.rules.get(self.name)


@dataclass(frozen=True)
class Rule:
    """A rewrite rule — `(rule Name (?p1 ?p2 …) :match … :assert … …)`.

    `match` / `assert_` are :class:`Pattern` objects holding the
    structural view of the clauses (variables bound, relations named,
    types touched); the matching semantics lives in P1.3.
    """
    name: str
    params: tuple[str, ...] = ()
    match: Pattern | None = None
    assert_: Pattern | None = None
    why: str = ""
    priority: int | None = None
    loc: Loc | None = field(default=None, compare=False, hash=False, repr=False)
    _kb: KnowledgeBase | None = field(default=None, compare=False, hash=False, repr=False)

    @property
    def relations(self) -> tuple[Relation, ...]:
        """Relations mentioned by name in `match` or `assert_`."""
        if self._kb is None or self.match is None:
            return ()
        names: set[str] = set()
        for p in (self.match, self.assert_):
            if p is not None:
                names.update(p.relation_names)
        return tuple(
            self._kb.relations[n] for n in sorted(names)
            if n in self._kb.relations
        )

    @property
    def types(self) -> tuple[Type, ...]:
        """Types touched by `(instance ?_ T)` patterns inside this rule."""
        if self._kb is None or self.match is None:
            return ()
        names: set[str] = set()
        for p in (self.match, self.assert_):
            if p is not None:
                names.update(p.type_names)
        return tuple(
            self._kb.types[n] for n in sorted(names) if n in self._kb.types
        )

    @property
    def applications(self) -> tuple[Fact, ...]:
        """Property-application facts whose head is this rule's name.

        Example: for the ``symmetric`` rule, applications are
        ``(symmetric co-located)`` and ``(symmetric next-to)``.
        """
        if self._kb is None:
            return ()
        return self._kb._rule_apps_by_rule.get(self.name, ())


@dataclass(frozen=True)
class Fact:
    """A fact: a hyperedge node in the canonical graph.

    Identity is `(relation_name, args)`; `layer`, `loc`, and the
    raw kw-pairs are not part of identity (two facts with the same
    head + args but different layers / sources still count as the
    "same" fact for de-duplication; the loader keeps the first
    occurrence's metadata).

    `args` are stored as a tuple of strings (names of instances /
    relations / types — depending on context) or :class:`int`.
    Resolution to a typed `Instance` / `Relation` happens through the
    KB at access time via :attr:`arg_entities`.
    """
    relation_name: str
    args: tuple[str | int, ...]
    layer: Layer = field(default=Layer.FACT, compare=False, hash=False)
    source: str | None = field(default=None, compare=False, hash=False, repr=False)
    rule_name: str | None = field(default=None, compare=False, hash=False, repr=False)
    using: tuple[str, ...] = field(default=(), compare=False, hash=False, repr=False)
    raw: IRNode | None = field(default=None, compare=False, hash=False, repr=False)
    loc: Loc | None = field(default=None, compare=False, hash=False, repr=False)
    _kb: KnowledgeBase | None = field(default=None, compare=False, hash=False, repr=False)

    @property
    def relation(self) -> Relation | None:
        if self._kb is None:
            return None
        return self._kb.relations.get(self.relation_name)

    @property
    def arg_entities(self) -> tuple[Instance | Relation | Type | str | int, ...]:
        """Resolve each arg to its KB entity, or leave as raw string/int.

        Falls back to the raw arg when no matching entity is found
        (lets the loader stay tolerant of open-world references).
        """
        if self._kb is None:
            return self.args
        out: list = []
        for a in self.args:
            if isinstance(a, int):
                out.append(a)
                continue
            if a in self._kb.instances:
                out.append(self._kb.instances[a])
            elif a in self._kb.relations:
                out.append(self._kb.relations[a])
            elif a in self._kb.types:
                out.append(self._kb.types[a])
            else:
                out.append(a)
        return tuple(out)

    @property
    def is_rule_application(self) -> bool:
        """True iff the fact's head matches a declared Rule's name."""
        if self._kb is None:
            return False
        return self.relation_name in self._kb.rules

    @property
    def applied_rule(self) -> Rule | None:
        """If this is a rule-application fact, the Rule it applies."""
        if not self.is_rule_application:
            return None
        return self._kb.rules.get(self.relation_name)


# ── Attach / detach helpers ────────────────────────────────────────


def _attach(entity, kb: KnowledgeBase) -> None:
    """Set the `_kb` back-pointer on a frozen dataclass.

    Uses ``object.__setattr__`` to bypass the frozen constraint. The
    field is excluded from compare/hash so identity is unaffected.
    """
    object.__setattr__(entity, "_kb", kb)


def _detach(entity) -> None:
    object.__setattr__(entity, "_kb", None)


__all__ = [
    "Fact",
    "Instance",
    "Layer",
    "Relation",
    "Rule",
    "Type",
    "_attach",
    "_detach",
]
