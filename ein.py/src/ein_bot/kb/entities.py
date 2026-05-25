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
from typing import TYPE_CHECKING, Literal

from ein_bot.ir.types import IRNode, Loc

if TYPE_CHECKING:
    from .pattern import Pattern
    from .provenance import Provenance
    from .store import KnowledgeBase


NameCategory = Literal["object", "relation", "rule"]

# Kernel meta-relations — names hardcoded as `category="relation"`.
# `relation` and `rule` are the two true kernel forms; `instance` and
# `type` are listed here only until the proto-library lands and they
# get registered through `(relation instance T T)` / `(relation type T)`
# — owed by T1.7.2.5.d (plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/
# s1.7.2_dynamic_vs_hardcoded.md). See [[project-canonical-zebra2]].
KERNEL_META_RELATIONS = frozenset({"relation", "rule", "instance", "type"})


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
    """A relation declaration — `(relation Name T1 T2 …)`.

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

    Args admit three shapes — matching the kernel ein model's
    *named* vs *relational* node duality
    (``docs/kernel/ir/01-ein-graph/03_ein_model.md`` §3):

    - ``str`` — a named node (name of an Instance / Relation / Type).
    - ``int`` — a numeric literal.
    - ``Fact`` — a **relational node** embedded as an argument
      (e.g. ``(hypothesis (co-located Norwegian House-2))``). The
      nested ``Fact`` participates in identity recursively: two outer
      facts are equal iff their ``(relation_name, args)`` tuples
      compare equal element-wise, with nested ``Fact`` instances
      cascading via their own ``__eq__``.

    Resolution to a typed `Instance` / `Relation` happens through the
    KB at access time via :attr:`arg_entities`. Nested ``Fact`` args
    are returned as-is (they're already entities).
    """
    relation_name: str
    args: tuple[str | int | Fact, ...]
    layer: Layer = field(default=Layer.FACT, compare=False, hash=False)
    provenance: Provenance | None = field(default=None, compare=False, hash=False, repr=False)
    raw: IRNode | None = field(default=None, compare=False, hash=False, repr=False)
    loc: Loc | None = field(default=None, compare=False, hash=False, repr=False)
    _kb: KnowledgeBase | None = field(default=None, compare=False, hash=False, repr=False)

    # ── Backward-compat shorthand properties ──────────────────────
    #
    # The pre-S1.2.3 Fact had flat `source` / `rule_name` / `using`
    # fields; these read through to the Provenance object so existing
    # callers (FactView.by_source, test_store, …) keep working.

    @property
    def source(self) -> str | None:
        """The `:source` sentence iff provenance is kind='source'."""
        if self.provenance is None or self.provenance.kind != "source":
            return None
        return self.provenance.source

    @property
    def rule_name(self) -> str | None:
        """The rule name iff provenance is kind='rule'."""
        if self.provenance is None or self.provenance.kind != "rule":
            return None
        return self.provenance.rule

    @property
    def using(self) -> tuple[tuple[str, tuple[str | int, ...]], ...]:
        """The raw fact-id premises (rel, args), iff kind='rule'."""
        if self.provenance is None or self.provenance.kind != "rule":
            return ()
        return self.provenance.premises_raw

    @property
    def premises(self) -> tuple[Fact, ...]:
        """Resolved premise facts (rule-kind only). Empty otherwise."""
        if (self.provenance is None
                or self.provenance.kind != "rule"
                or self._kb is None):
            return ()
        out: list[Fact] = []
        for rid in self.provenance.premises_raw:
            f = self._kb._fact_by_id(*rid)
            if f is not None:
                out.append(f)
        return tuple(out)

    @property
    def relation(self) -> Relation | None:
        if self._kb is None:
            return None
        return self._kb.relations.get(self.relation_name)

    @property
    def arg_entities(self) -> tuple[Instance | Relation | Type | Fact | str | int, ...]:
        """Resolve each arg to its KB entity, or leave as raw string/int.

        Nested ``Fact`` args are returned as-is (they're already
        entity-shaped). String / int args fall back to the raw value
        when no matching entity is found, so the loader stays
        tolerant of open-world references.
        """
        if self._kb is None:
            return self.args
        out: list = []
        for a in self.args:
            if isinstance(a, Fact):
                out.append(a)
                continue
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


# ── Global names index ────────────────────────────────────────────


@dataclass(frozen=True)
class NameRef:
    """A globally-unique name in the graph + its participation set.

    Per `docs/kernel/ir/01-ein-graph/03_ein_model.md` §2, every distinct
    name across the KB refers to the same node. This entity records
    the **participation** of one such name: every fact in which it
    appears as the head, and every fact in which it appears as a
    direct string argument.

    Encoding-agnostic: works the same for zebra-original (kernel
    `(type)` / `(instance)`) and zebra2 (`is-a` as an ordinary
    declared relation). Consumers that need to iterate "the
    instance-like objects" can filter by ``category == "object"``;
    the leaf-vs-internal distinction is then derived from
    ``as_head`` / ``as_arg`` of the chosen inheritance relation.

    Nested-Fact args (Q40) are NOT counted: the nested Fact is its
    own entry in `kb.facts`, so its args show up via that entry's
    ``as_arg``. The outer fact's ``args`` tuple is walked for direct
    string values only.

    `category`:
      - ``"relation"`` — declared via ``(relation N …)`` OR a kernel
        meta-head in `KERNEL_META_RELATIONS`.
      - ``"rule"`` — declared via ``(rule N …)``.
      - ``"object"`` — every other name (instances, types, attributes,
        anchors).
    """
    name:     str
    category: NameCategory
    as_head:  tuple[Fact, ...] = ()
    as_arg:   tuple[Fact, ...] = ()


__all__ = [
    "KERNEL_META_RELATIONS",
    "Fact",
    "Instance",
    "Layer",
    "NameCategory",
    "NameRef",
    "Relation",
    "Rule",
    "Type",
    "_attach",
    "_detach",
]
