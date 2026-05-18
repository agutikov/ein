"""KnowledgeBase — entity registry + reverse indexes — S1.2.1 T1.2.1.2/3.

The KB owns:

- five registries keyed by name (``types``, ``instances``,
  ``relations``, ``rules``) and one fact list (``facts``);
- six reverse indexes covering the cross-references documented in
  ``plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/s1.2.1_data_model.md``;
- an optional :class:`Query` slot (set when an IR file contains
  ``(query …)``);
- a placeholder :class:`EqClasses` union-find — reserved for F4
  e-graph promotion (T1.2.1.6).

The entity classes expose those cross-references as ``@property``
accessors that delegate here via the entity's ``_kb`` back-pointer;
the indexes are precomputed in :meth:`KnowledgeBase._rebuild_indexes`.

Loading IR is in :mod:`ein_bot.kb.from_ir`; this module is purely
the in-memory store.
"""
from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from typing import Any

from .entities import Fact, Instance, Layer, Relation, Rule, Type, _attach

# ── Equality-class hooks (T1.2.1.6 placeholder) ───────────────────


class EqClasses:
    """A union-find over instance names. M1 placeholder.

    The engine does *not* yet fire equality propagation; this exists
    so that `(= ?a ?b)` rule conclusions can call ``kb.classes.union``
    without surrounding code asking "what union-find?". F4 promotes
    this into an e-graph when the need arrives.
    """
    def __init__(self) -> None:
        self._parent: dict[str, str] = {}

    def find(self, x: str) -> str:
        # Auto-vivify.
        if x not in self._parent:
            self._parent[x] = x
            return x
        # Path-compress.
        root = x
        while self._parent[root] != root:
            root = self._parent[root]
        cur = x
        while self._parent[cur] != root:
            self._parent[cur], cur = root, self._parent[cur]
        return root

    def union(self, a: str, b: str) -> str:
        ra, rb = self.find(a), self.find(b)
        if ra != rb:
            self._parent[rb] = ra
        return ra

    def equivalent(self, a: str, b: str) -> bool:
        return self.find(a) == self.find(b)

    def classes(self) -> dict[str, list[str]]:
        out: dict[str, list[str]] = defaultdict(list)
        for x in self._parent:
            out[self.find(x)].append(x)
        return dict(out)


# ── Query placeholder (T1.2.1.4 step 8) ───────────────────────────


@dataclass(frozen=True)
class Query:
    """A `(query …)` block lifted into a small entity.

    The body keeps the raw `KwPair` tuple — interpretation lives in
    P1.5 (hypothesis loop), which reads ``mode`` and ``goal``.
    """
    kw_pairs: tuple[Any, ...] = ()


# ── KnowledgeBase ──────────────────────────────────────────────────


class KnowledgeBase:
    """The entity registry + reverse indexes.

    Conceptually the underlying form is the canonical graph (every
    Type / Instance / Relation / Rule / Fact is a node); the registry
    dicts and reverse indexes are *cached lookups* into that graph.
    Different views (P1.2 S1.2.2) and the unified DOT rendering
    (P1.2 S1.2.4) read the same indexes.
    """

    def __init__(self) -> None:
        # Registries (entity by name).
        self.types: dict[str, Type] = {}
        self.instances: dict[str, Instance] = {}
        self.relations: dict[str, Relation] = {}
        self.rules: dict[str, Rule] = {}
        # Facts list (layer is a field on each Fact).
        self.facts: list[Fact] = []
        # Query (optional, only when the IR carries a `(query …)`).
        self.query: Query | None = None

        # Equality-class hooks — reserved for F4.
        self.classes: EqClasses = EqClasses()

        # Reverse indexes — populated by `_rebuild_indexes`.
        self._types_by_parent: dict[str, tuple[Type, ...]] = {}
        self._instances_by_type: dict[str, tuple[Instance, ...]] = {}
        self._facts_by_relation: dict[str, tuple[Fact, ...]] = {}
        self._facts_by_instance: dict[str, tuple[Fact, ...]] = {}
        self._rules_by_relation: dict[str, tuple[Rule, ...]] = {}
        self._rules_by_type: dict[str, tuple[Rule, ...]] = {}
        self._rule_apps_by_rule: dict[str, tuple[Fact, ...]] = {}
        self._rule_apps_on_relation: dict[str, tuple[Fact, ...]] = {}

    # ── from_ir convenience ───────────────────────────────────────

    @classmethod
    def from_ir(cls, forms) -> KnowledgeBase:
        """Construct a KB from parsed IR forms.

        Delegates to :func:`ein_bot.kb.from_ir.load`. Importing here
        avoids a hard import-time cycle (`from_ir` imports `store`).
        """
        from .from_ir import load
        return load(forms)

    # ── Registry mutation (used by the loader) ────────────────────

    def add_type(self, t: Type) -> Type:
        """Register a Type (idempotent on name; first-wins for parent)."""
        if t.name in self.types:
            return self.types[t.name]
        _attach(t, self)
        self.types[t.name] = t
        return t

    def add_instance(self, inst: Instance) -> Instance:
        if inst.name in self.instances:
            return self.instances[inst.name]
        _attach(inst, self)
        self.instances[inst.name] = inst
        return inst

    def add_relation(self, rel: Relation) -> Relation:
        """Register a Relation; the *declared* flag wins over open-world.

        If an open-world entry exists, a subsequent ``add_relation``
        call with ``declared=True`` upgrades it in place (replaces the
        registry entry; the previous detached instance becomes
        orphaned but no one was holding it).
        """
        existing = self.relations.get(rel.name)
        if existing is None:
            _attach(rel, self)
            self.relations[rel.name] = rel
            return rel
        if rel.declared and not existing.declared:
            _attach(rel, self)
            self.relations[rel.name] = rel
            return rel
        return existing

    def add_rule(self, rule: Rule) -> Rule:
        if rule.name in self.rules:
            return self.rules[rule.name]
        _attach(rule, self)
        self.rules[rule.name] = rule
        return rule

    def add_fact(self, fact: Fact) -> Fact:
        """Append a Fact; dedupe is by ``(relation_name, args)``.

        Layer is excluded from identity — a single proposition lives
        only once in the KB, and `layer` records its origin (ontology /
        fact / reasoning). If a fact arrives twice with different
        layers, the *first* occurrence wins; this preserves the most
        primitive declaration. Duplicate-only-by-source/by-rule are
        collapsed; provenance details (S1.2.3) attach to the canonical
        Fact.
        """
        _attach(fact, self)
        for existing in self.facts:
            if (existing.relation_name == fact.relation_name
                    and existing.args == fact.args):
                return existing
        self.facts.append(fact)
        return fact

    # ── Index rebuild ─────────────────────────────────────────────

    def rebuild_indexes(self) -> None:
        """Recompute all reverse indexes from current registries + facts.

        Cheap enough on Zebra-scale (single-digit milliseconds);
        called once after batch ingest. Incremental maintenance for
        single-fact additions in the reasoning layer is provided by
        :meth:`_index_fact` so a saturation loop need not rebuild.
        """
        # types_by_parent
        types_by_parent: dict[str, list[Type]] = defaultdict(list)
        for t in self.types.values():
            if t.parent_name is not None:
                types_by_parent[t.parent_name].append(t)
        self._types_by_parent = {
            p: tuple(sorted(ts, key=lambda x: x.name))
            for p, ts in types_by_parent.items()
        }

        # instances_by_type
        instances_by_type: dict[str, list[Instance]] = defaultdict(list)
        for inst in self.instances.values():
            instances_by_type[inst.type_name].append(inst)
        self._instances_by_type = {
            t: tuple(insts) for t, insts in instances_by_type.items()
        }

        # facts indexes
        self._facts_by_relation = {}
        self._facts_by_instance = {}
        self._rule_apps_by_rule = {}
        self._rule_apps_on_relation = {}
        fbr: dict[str, list[Fact]] = defaultdict(list)
        fbi: dict[str, list[Fact]] = defaultdict(list)
        rabr: dict[str, list[Fact]] = defaultdict(list)
        raor: dict[str, list[Fact]] = defaultdict(list)
        for fact in self.facts:
            fbr[fact.relation_name].append(fact)
            for a in fact.args:
                if isinstance(a, str) and a in self.instances:
                    fbi[a].append(fact)
            if fact.relation_name in self.rules:
                rabr[fact.relation_name].append(fact)
                # Each arg that is a known Relation is a target of the
                # rule application.
                for a in fact.args:
                    if isinstance(a, str) and a in self.relations:
                        raor[a].append(fact)
        self._facts_by_relation = {k: tuple(v) for k, v in fbr.items()}
        self._facts_by_instance = {k: tuple(v) for k, v in fbi.items()}
        self._rule_apps_by_rule = {k: tuple(v) for k, v in rabr.items()}
        self._rule_apps_on_relation = {k: tuple(v) for k, v in raor.items()}

        # rules_by_relation (named in pattern + via property fact)
        rbr: dict[str, set[str]] = defaultdict(set)
        for rule in self.rules.values():
            for p in (rule.match, rule.assert_):
                if p is None:
                    continue
                for rn in p.relation_names:
                    if rn in self.relations:
                        rbr[rn].add(rule.name)
        # Add the property-fact side: a property fact `(symmetric R)`
        # is a rule application of `symmetric` on `R`, so the rule's
        # name (the fact's head) is implicitly a rule on `R`. The
        # `symmetric` *Rule* doesn't name `R` in its body — `?rel`
        # binds to `R` via the property fact.
        for rel_name, app_facts in self._rule_apps_on_relation.items():
            for f in app_facts:
                rbr[rel_name].add(f.relation_name)
        self._rules_by_relation = {
            rel: tuple(self.rules[n] for n in sorted(names) if n in self.rules)
            for rel, names in rbr.items()
        }

        # rules_by_type
        rbt: dict[str, set[str]] = defaultdict(set)
        for rule in self.rules.values():
            for p in (rule.match, rule.assert_):
                if p is None:
                    continue
                for tn in p.type_names:
                    if tn in self.types:
                        rbt[tn].add(rule.name)
        self._rules_by_type = {
            t: tuple(self.rules[n] for n in sorted(names) if n in self.rules)
            for t, names in rbt.items()
        }

    def _index_fact(self, fact: Fact) -> None:
        """Incrementally append a single fact to the indexes.

        Used by the reasoning-layer mutations (P1.3 / P1.5) so a
        saturation step needn't call :meth:`rebuild_indexes`.
        """
        rn = fact.relation_name
        self._facts_by_relation = {
            **self._facts_by_relation,
            rn: (*self._facts_by_relation.get(rn, ()), fact),
        }
        for a in fact.args:
            if isinstance(a, str) and a in self.instances:
                self._facts_by_instance = {
                    **self._facts_by_instance,
                    a: (*self._facts_by_instance.get(a, ()), fact),
                }
        if rn in self.rules:
            self._rule_apps_by_rule = {
                **self._rule_apps_by_rule,
                rn: (*self._rule_apps_by_rule.get(rn, ()), fact),
            }
            for a in fact.args:
                if isinstance(a, str) and a in self.relations:
                    self._rule_apps_on_relation = {
                        **self._rule_apps_on_relation,
                        a: (*self._rule_apps_on_relation.get(a, ()), fact),
                    }

    # ── Convenience accessors ─────────────────────────────────────

    def facts_in_layer(self, layer: Layer) -> tuple[Fact, ...]:
        """All facts in a given layer."""
        return tuple(f for f in self.facts if f.layer == layer)

    def __len__(self) -> int:
        """Total node count: types + instances + relations + rules + facts."""
        return (
            len(self.types) + len(self.instances) + len(self.relations)
            + len(self.rules) + len(self.facts)
        )

    def __repr__(self) -> str:
        return (
            f"<KnowledgeBase types={len(self.types)} instances={len(self.instances)} "
            f"relations={len(self.relations)} rules={len(self.rules)} "
            f"facts={len(self.facts)}>"
        )


__all__ = ["EqClasses", "KnowledgeBase", "Query"]
