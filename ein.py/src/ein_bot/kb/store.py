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
from collections.abc import Iterable
from dataclasses import dataclass
from typing import TYPE_CHECKING, Any

from .entities import (
    KERNEL_META_RELATIONS,
    Fact,
    Instance,
    Layer,
    NameRef,
    Relation,
    Rule,
    Type,
    _attach,
)

if TYPE_CHECKING:
    from .provenance import DerivationDAG
    from .views import FactView

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
        # Hypothesis rules (S1.5.6b) — kept separate from `rules`:
        # they generate candidate hypotheses for hypgen and are
        # never fired by the saturator. Loaded from `(hrule …)`.
        self.hrules: dict[str, Rule] = {}
        # Facts list (layer is a field on each Fact).
        self.facts: list[Fact] = []
        # Query (optional, only when the IR carries a `(query …)`).
        self.query: Query | None = None
        # SolverConfig (optional, only when the IR carries a `(config
        # …)` head; T1.5.4.4). Typed as `Any` here to avoid a hard
        # import cycle (config lives under inference/, which imports
        # store). `solve()` resolves precedence as: kwarg > kb.config
        # > SolverConfig().
        self.config: Any | None = None
        # Alive-hypothesis set — T1.5.4.8 Topic D ship. None until
        # `solve()` populates it at the root via
        # `generate_hypotheses_with_stats(root_kb)`. Forks inherit
        # by reference; `_explore` re-prunes per path and stashes
        # the new frozenset back here before recursing. Each entry
        # is a Fact with provenance=None — try_branch re-stamps
        # branch-specific provenance.
        self.alive: Any | None = None

        # S1.5.7b ConsumeStats — per-solve counters for the back-prop
        # consume loop's `try_branch`-skip cache. None until
        # `solve()` allocates an instance on the root KB; forks share
        # by reference so every `_consume` invocation across the
        # search mutates the same counter. Typed as `Any` to mirror
        # `config`/`alive` and avoid the inference → kb import cycle.
        self.consume_stats: Any | None = None

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

        # Global names index — encoding-agnostic node participation
        # set per docs/kernel/ir/01-ein-graph/03_ein_model.md §2.
        # Keyed by name; the NameRef's `category` discriminates
        # `object` / `relation` / `rule`.
        self.names: dict[str, NameRef] = {}

        # Negated-fact index — for each `(not <inner>)` fact, the
        # `(inner.relation_name, inner.args)` tuple. Lets the
        # hypothesis-generator's Tier-A exclusion check land in O(1)
        # instead of an O(|not-facts|) scan over `_facts_by_relation`.
        self._negated_facts: set[tuple[str, tuple]] = set()

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

    def add_hrule(self, rule: Rule) -> Rule:
        """Register a hypothesis rule (S1.5.6b) into ``hrules``.

        A hrule is a :class:`Rule` by shape; it lives in its own
        registry so the engine / saturator — which walk ``rules`` —
        never fire it. ``hypgen`` is its only consumer. Rule and
        hrule names share one name-space — the loader rejects a
        duplicate, so ``hrules`` is keyed by name like ``rules``.
        """
        if rule.name in self.hrules:
            return self.hrules[rule.name]
        _attach(rule, self)
        self.hrules[rule.name] = rule
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
        self._negated_facts = set()
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
            if fact.relation_name == "not" and fact.args:
                inner = fact.args[0]
                if isinstance(inner, Fact):
                    self._negated_facts.add(
                        (inner.relation_name, inner.args)
                    )
        self._facts_by_relation = {k: tuple(v) for k, v in fbr.items()}
        self._facts_by_instance = {k: tuple(v) for k, v in fbi.items()}
        self._rule_apps_by_rule = {k: tuple(v) for k, v in rabr.items()}
        self._rule_apps_on_relation = {k: tuple(v) for k, v in raor.items()}

        # Global names index — every distinct name that appears
        # anywhere in the KB. Built from `kb.facts` (head + direct
        # string args) plus the registry keys (so a relation
        # declared with no facts yet still shows up).
        head_lists: dict[str, list[Fact]] = defaultdict(list)
        arg_lists: dict[str, list[Fact]] = defaultdict(list)
        for fact in self.facts:
            head_lists[fact.relation_name].append(fact)
            for a in fact.args:
                if isinstance(a, str):
                    arg_lists[a].append(fact)
        all_names: set[str] = (
            set(head_lists)
            | set(arg_lists)
            | set(self.relations)
            | set(self.rules)
            | set(self.types)
            | set(self.instances)
        )
        self.names = {
            n: NameRef(
                name=n,
                category=self._categorise_name(n),
                as_head=tuple(head_lists.get(n, ())),
                as_arg=tuple(arg_lists.get(n, ())),
            )
            for n in all_names
        }

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

    def _categorise_name(self, name: str) -> str:
        """Map a name to its `NameRef.category`.

        `relation` / `rule` are the two true kernel forms; `instance`
        / `type` are temporarily hardcoded as `category="relation"`
        until the proto-library lands and declares them via
        `(relation instance T T)` / `(relation type T)` — see
        plans/.../proto_library.md and [[project-canonical-zebra2]].
        """
        if name in KERNEL_META_RELATIONS:
            return "relation"
        if name in self.relations:
            return "relation"
        if name in self.rules:
            return "rule"
        return "object"

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

        # Negated-fact index — O(1) Tier-A exclusion lookup for the
        # hypothesis generator.
        if rn == "not" and fact.args:
            inner = fact.args[0]
            if isinstance(inner, Fact):
                self._negated_facts.add(
                    (inner.relation_name, inner.args)
                )

        # Names index — append to head + arg sets, creating fresh
        # NameRefs (frozen dataclasses) so the dict-value identity
        # shifts on update.
        def _bump_head(name: str) -> None:
            prev = self.names.get(name)
            if prev is None:
                self.names = {**self.names, name: NameRef(
                    name=name, category=self._categorise_name(name),
                    as_head=(fact,), as_arg=(),
                )}
            else:
                self.names = {**self.names, name: NameRef(
                    name=prev.name, category=prev.category,
                    as_head=(*prev.as_head, fact),
                    as_arg=prev.as_arg,
                )}

        def _bump_arg(name: str) -> None:
            prev = self.names.get(name)
            if prev is None:
                self.names = {**self.names, name: NameRef(
                    name=name, category=self._categorise_name(name),
                    as_head=(), as_arg=(fact,),
                )}
            else:
                self.names = {**self.names, name: NameRef(
                    name=prev.name, category=prev.category,
                    as_head=prev.as_head,
                    as_arg=(*prev.as_arg, fact),
                )}

        _bump_head(rn)
        for a in fact.args:
            if isinstance(a, str):
                _bump_arg(a)

    # ── Convenience accessors ─────────────────────────────────────

    def facts_in_layer(self, layer: Layer) -> tuple[Fact, ...]:
        """All facts in a given layer."""
        return tuple(f for f in self.facts if f.layer == layer)

    # ── Layer views — S1.2.2 T1.2.2.2 ─────────────────────────────

    def ontology(self) -> FactView:
        """Read-only view of `ONTOLOGY`-layer facts."""
        from .views import FactView
        return FactView(self.facts_in_layer(Layer.ONTOLOGY), self, "ontology")

    def fact_layer(self) -> FactView:
        """Read-only view of `FACT`-layer facts.

        Named `fact_layer` (not `facts`) to avoid collision with the
        registry attribute :attr:`facts`.
        """
        from .views import FactView
        return FactView(self.facts_in_layer(Layer.FACT), self, "fact")

    def reasoning(self) -> FactView:
        """Read-only view of `REASONING`-layer facts."""
        from .views import FactView
        return FactView(self.facts_in_layer(Layer.REASONING), self, "reasoning")

    def all_layers(self) -> FactView:
        """Read-only view of every fact across layers."""
        from .views import FactView
        return FactView(tuple(self.facts), self, "all")

    # ── Provenance + derivation DAG — S1.2.3 ──────────────────────

    def _fact_by_id(
        self, relation_name: str, args: tuple,
    ) -> Fact | None:
        """Look up a fact by its identity tuple — O(deg(relation))."""
        for f in self._facts_by_relation.get(relation_name, ()):
            if f.args == args:
                return f
        return None

    def derivation_dag(self, fact: Fact) -> DerivationDAG:
        """Build the derivation DAG rooted at `fact`.

        BFS over ``fact.provenance.premises_raw``, resolving each id
        to a Fact via :meth:`_fact_by_id` and recursing into
        ``rule``-kind facts only. ``source``- and ``hypothesis``-kind
        facts terminate the recursion.

        Cycles are broken at re-visit (the revisited fact appears as a
        node but is not re-expanded).
        """
        from .provenance import build_derivation_dag
        return build_derivation_dag(fact, self._fact_by_id)

    # ── Unified DOT rendering — S1.2.4 ────────────────────────────

    def to_dot(self, **kwargs) -> str:
        """Render the KB as a unified Graphviz ``digraph`` string.

        Delegates to :func:`ein_bot.kb.render.to_dot`. Keyword args
        forwarded — see that function for ``layers`` / ``colour_by`` /
        ``include_types`` / ``include_instances`` / ``name``.
        """
        from .render import to_dot
        return to_dot(self, **kwargs)

    def unsat_core(self, conflicting: Iterable[Fact]) -> set[Fact]:
        """Minimal source-kind frontier across a set of conflicting facts.

        For each conflicting fact, walk its derivation DAG and
        accumulate the source-kind terminals. The union is the
        minimal set of *given* facts that, jointly, derive the
        conflict; it's what the *contradictions* task class returns
        to the user (idea 03).
        """
        core: set[Fact] = set()
        for f in conflicting:
            dag = self.derivation_dag(f)
            core.update(dag.sources)
        return core

    # ── Fork — S1.2.2 T1.2.2.3 ────────────────────────────────────

    def fork(self) -> KnowledgeBase:
        """Create a branch for hypothesis exploration.

        Shares the immutable populations (`types`, `instances`,
        `relations`, `rules`, `query`) **by reference**. The
        :attr:`facts` list and the reverse indexes are shallow-copied
        so the fork can append :class:`Layer.REASONING` facts and
        rebuild its own incremental indexes without leaking into the
        parent.

        Caveat about entity back-pointers: shared entities keep their
        ``_kb`` pointing at the **original** KB. This means
        ``norwegian.facts`` returns the original KB's facts, NOT the
        fork's view. For fork-scoped queries use
        ``fork.all_layers().about(norwegian)`` or the explicit
        indexes on the fork (``fork._facts_by_instance[name]``). This
        is intentional: hypothesis branches rarely introduce new
        entities, only new derived facts; the entity API tells you
        the root state, the fork view tells you the branch state.

        Cost: O(|facts|) for the shallow copies — bounded by Zebra-
        scale at ~50-200 facts. The plan's "O(|reasoning|)" target is
        relaxed to "O(|facts|) with O(1) per fact" because the indexes
        partition by relation/instance, and shallow-copying a dict
        is constant per entry; if hypothesis branching ever becomes a
        hot path (P1.5 profiling), revisit with a copy-on-write
        wrapper.
        """
        new = KnowledgeBase()
        # Share immutable populations by reference.
        new.types = self.types
        new.instances = self.instances
        new.relations = self.relations
        new.rules = self.rules
        new.hrules = self.hrules
        new.query = self.query
        # Equality classes: fork its own state (the parent's classes
        # remain reachable via the union-find roots that were created
        # before the fork).
        new.classes = EqClasses()
        new.classes._parent = dict(self.classes._parent)
        # Facts list: copy so appends to the fork don't touch parent.
        new.facts = list(self.facts)
        # Reverse indexes: shallow-copy the dicts (values stay shared;
        # mutations replace whole entries via dict-merge in
        # `_index_fact`).
        new._types_by_parent = dict(self._types_by_parent)
        new._instances_by_type = dict(self._instances_by_type)
        new._facts_by_relation = dict(self._facts_by_relation)
        new._facts_by_instance = dict(self._facts_by_instance)
        new._rules_by_relation = dict(self._rules_by_relation)
        new._rules_by_type = dict(self._rules_by_type)
        new._rule_apps_by_rule = dict(self._rule_apps_by_rule)
        new._rule_apps_on_relation = dict(self._rule_apps_on_relation)
        new.names = dict(self.names)
        new._negated_facts = set(self._negated_facts)
        # T1.5.4.4 / T1.5.4.8 carry-over: configs and alive sets are
        # immutable references; the fork inherits them as-is and
        # `_explore` swaps in a pruned alive set before recursing.
        # S1.5.7b: consume_stats is a *shared* mutable counter — all
        # forks point at the same instance so every `_consume` call
        # accumulates into the same per-solve total.
        new.config = self.config
        new.alive = self.alive
        new.consume_stats = self.consume_stats
        return new

    # ── Dunder ────────────────────────────────────────────────────

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
