"""Integration tests for KnowledgeBase loading + cross-references.

These exercise the S1.2.1 acceptance criteria against the canonical
examples (zebra.ein and zebra2.ein).
"""
from __future__ import annotations

from ein.kb import (
    Fact,
    KnowledgeBase,
    Layer,
)

# ═══════════════════════ zebra.ein (classic split) ════════════════


class TestZebraCounts:
    """Top-level entity counts against the S1.2.1 acceptance.

    S1.7.23 — the `test_seven_types` / `test_thirty_instances` cases were
    DELETED with the `kb.types` / `kb.instances` entity-view; the
    inheritance forest is now just `(type …)` / `(instance …)` facts in
    the fact list (counted by `TestZebraFact.test_fact_count`).
    """

    def test_five_declared_relations(self, zebra_kb):
        # S1.7.6: `type` / `instance` are now ordinary DECLARED relations
        # (was kernel forms) alongside the three domain relations.
        declared = sorted(
            n for n, r in zebra_kb.relations.items() if r.declared
        )
        assert declared == ["co-located", "instance", "next-to", "right-of", "type"]

    def test_open_world_relations_include_rule_names(self, zebra_kb):
        # Property tags (heads of `(symmetric R)` etc.) auto-vivify
        # as open-world relations. (`instance` is no longer here — it is
        # a declared relation since S1.7.6.)
        open_world = sorted(
            n for n, r in zebra_kb.relations.items() if not r.declared
        )
        assert {"symmetric", "transitive", "implies", "square-fwd",
                "square-bwd"} <= set(open_world)
        assert "instance" not in open_world

    def test_eight_rules(self, zebra_kb):
        """S1.3.2 + square-unique addition for corner-house spatial inference."""
        assert len(zebra_kb.rules) == 8
        assert set(zebra_kb.rules) == {
            "symmetric", "transitive", "implies",
            "square-fwd", "square-bwd", "square-unique",
            "type-exclusivity",
            "hypothesis-contradiction",
        }


# S1.7.23 — `TestZebraTypeHierarchy` (Type.parent/children/instances/
# ancestors) and `TestZebraInstance` (Instance.type/.facts) were DELETED
# with the `Type` / `Instance` entity classes. The inheritance forest is
# `is-a` / `(type …)` / `(instance …)` facts; a puzzle that wants a
# typed view computes it with a user-space ein-lang rule.


class TestZebraRelation:
    def test_right_of_rules(self, zebra_kb):
        names = {r.name for r in zebra_kb.relations["right-of"].rules}
        assert names == {"implies", "square-fwd", "square-bwd"}

    def test_next_to_rules(self, zebra_kb):
        # Acceptance: rules whose name is the head of a property fact
        # involving next-to, OR whose pattern names it.
        # Property facts: `(symmetric next-to)`, `(implies right-of next-to)`.
        names = {r.name for r in zebra_kb.relations["next-to"].rules}
        # `square-unique` joined the next-to rule list via the
        # `(square-unique next-to House)` activator fact (S1.3.2+).
        assert names == {"symmetric", "implies", "square-unique"}

    def test_co_located_in_type_exclusivity(self, zebra_kb):
        # type-exclusivity body asserts `(not (co-located ?a ?b))` —
        # so it names co-located by literal head.
        names = {r.name for r in zebra_kb.relations["co-located"].rules}
        assert "type-exclusivity" in names

    def test_right_of_properties(self, zebra_kb):
        props = zebra_kb.relations["right-of"].properties
        heads = sorted(f.relation_name for f in props)
        # `(implies right-of next-to)`, `(square-fwd right-of)`,
        # `(square-bwd right-of)`.
        assert heads == ["implies", "square-bwd", "square-fwd"]

    def test_relation_signature_is_opaque_names(self, zebra_kb):
        # S1.7.23 — `signature` is opaque type-name atoms; there is no
        # `signature_types` → `Type` resolution (no Type entities).
        rel = zebra_kb.relations["co-located"]
        assert rel.signature == ("Attribute", "Attribute")


class TestZebraRule:
    def test_symmetric_applications(self, zebra_kb):
        sym = zebra_kb.rules["symmetric"]
        apps = sorted(f.args for f in sym.applications)
        assert apps == [("co-located",), ("next-to",)]

    def test_implies_applications(self, zebra_kb):
        imp = zebra_kb.rules["implies"]
        apps = [f.args for f in imp.applications]
        assert apps == [("right-of", "next-to")]

    def test_type_exclusivity_has_one_application(self, zebra_kb):
        # T2 rule: activator `(type-exclusivity co-located)` is the
        # single property fact authorising the rule for co-located.
        rule = zebra_kb.rules["type-exclusivity"]
        assert len(rule.applications) == 1
        app = rule.applications[0]
        assert app.relation_name == "type-exclusivity"
        assert app.args == ("co-located",)

    def test_type_exclusivity_mentions_co_located(self, zebra_kb):
        # ?R appears only in :assert (not (?R ?a ?b)), so the structural
        # relation_names list doesn't include "co-located" — but the
        # `applications` activator + the cross-reference via
        # `kb.relations["co-located"].rules` does.
        co_loc_rules = {r.name for r in zebra_kb.relations["co-located"].rules}
        assert "type-exclusivity" in co_loc_rules

    def test_rule_has_pattern_objects(self, zebra_kb):
        sym = zebra_kb.rules["symmetric"]
        assert sym.match is not None
        assert sym.assert_ is not None
        # The match `(?rel ?a ?b)` has three vars and no literal
        # relation head (?rel is a Var, not an Atom).
        assert set(sym.match.variables) == {"rel", "a", "b"}
        assert sym.match.relation_names == ()


class TestZebraFact:
    def test_fact_count(self, zebra_kb):
        # Ontology: 7 type-decl + 30 instance + 8 rule-app + 4 spatial
        #  + 5 relation-decl = 54. (S1.7.6: type/instance are plain
        #  relations now — the 7 (type …) decls are ONTOLOGY facts, and
        #  relation-decls are co-located, right-of, next-to, type, instance.)
        # Facts: 14 (conditions 2..15).
        # Total: 68.
        assert len(zebra_kb.facts) == 68

    def test_fact_resolves_relation(self, zebra_kb):
        fs = [f for f in zebra_kb.facts if f.source == "condition (10)"]
        assert len(fs) == 1
        f = fs[0]
        assert f.relation_name == "co-located"
        assert f.relation == zebra_kb.relations["co-located"]
        assert f.layer == Layer.FACT

    def test_fact_arg_entities_resolution(self, zebra_kb):
        # S1.7.23 — object-name args resolve to raw strings (no
        # `Instance` entities); only Relation-name args resolve to a
        # `Relation`. `(co-located Norwegian House-1)` → both strings.
        fs = [f for f in zebra_kb.facts if f.source == "condition (10)"]
        f = fs[0]
        a, b = f.arg_entities
        assert a == "Norwegian"
        assert b == "House-1"

    def test_property_fact_is_rule_application(self, zebra_kb):
        # `(symmetric co-located)` has head=symmetric (a rule).
        f = next(
            fact for fact in zebra_kb.facts
            if fact.relation_name == "symmetric" and fact.args == ("co-located",)
        )
        assert f.is_rule_application is True
        assert f.applied_rule == zebra_kb.rules["symmetric"]

    def test_non_rule_fact_is_not_rule_application(self, zebra_kb):
        f = next(
            fact for fact in zebra_kb.facts
            if fact.relation_name == "co-located"
        )
        assert f.is_rule_application is False
        assert f.applied_rule is None

    def test_facts_in_layer_filter(self, zebra_kb):
        # 14 explicit condition facts; the rest are ontology.
        fact_layer = zebra_kb.facts_in_layer(Layer.FACT)
        assert len(fact_layer) == 14
        reasoning = zebra_kb.facts_in_layer(Layer.REASONING)
        assert reasoning == ()


# ═══════════════════════ zebra2.ein (unified is-a) ═════════════════


class TestZebra2:
    """The unified `is-a` model uses no `(type …)` or `(instance …)`."""

    def test_zebra2_has_no_type_instance_facts(self, zebra2_kb):
        # All inheritance is expressed via `is-a` facts; no `(type …)`
        # or `(instance …)` facts. (S1.7.23 — there are no kb.types /
        # kb.instances registries to check; assert over the fact list.)
        rels = {f.relation_name for f in zebra2_kb.facts}
        assert "type" not in rels
        assert "instance" not in rels
        assert "is-a" in rels

    def test_zebra2_relations_include_is_a(self, zebra2_kb):
        declared = {n for n, r in zebra2_kb.relations.items() if r.declared}
        assert "is-a" in declared

    def test_zebra2_facts_count_nonzero(self, zebra2_kb):
        # ≥ 30 is-a facts (subtype + leaf) plus rule-apps plus
        # spatial plus conditions. Just smoke-check non-empty.
        assert len(zebra2_kb.facts) > 30


# ═══════════════════════ Open-world / robustness ═══════════════════


class TestOpenWorld:
    def test_undeclared_relation_auto_vivifies(self):
        from ein.ir import parse
        text = """
        (type Foo)
        (instance A Foo)
        (mystery-relation A B :source "(1)")
        """
        kb = KnowledgeBase.from_ir(parse(text))
        assert "mystery-relation" in kb.relations
        rel = kb.relations["mystery-relation"]
        assert rel.declared is False
        assert len(rel.facts) == 1

# S1.7.23 — `test_undeclared_type_auto_vivifies` was DELETED: there is no
# `kb.types` registry to auto-vivify into. An `(instance A NoSuchType)`
# fact is just a fact; `NoSuchType` is an ordinary node name.


class TestQueryLoading:
    def test_zebra_query_present(self, zebra_kb):
        assert zebra_kb.query is not None
        # kw_pairs is a tuple of KwPair objects.
        keys = sorted(kw.key.name for kw in zebra_kb.query.kw_pairs)
        assert keys == ["goal", "mode"]


# ═══════════════════════ EqClasses placeholder ═════════════════════


class TestEqClasses:
    def test_union_find_basic(self, zebra_kb):
        c = zebra_kb.classes
        # Two instances start in their own classes.
        a = c.find("Norwegian")
        b = c.find("Japanese")
        assert a != b
        c.union("Norwegian", "Japanese")
        assert c.equivalent("Norwegian", "Japanese")

    def test_classes_dict(self):
        from ein.kb.store import EqClasses
        c = EqClasses()
        c.union("a", "b")
        c.union("c", "d")
        groups = c.classes()
        assert len(groups) == 2
        # Each group contains its members.
        for members in groups.values():
            assert len(members) == 2


# ═══════════════════════ Index maintenance ═════════════════════════


class TestIncrementalIndex:
    def test_index_fact_appends(self):
        from ein.ir import parse
        text = """
        (type T)
        (instance A T)
        (relation r T T)
        """
        from ein.kb import Provenance
        kb = KnowledgeBase.from_ir(parse(text))
        # Add a reasoning-layer fact incrementally.
        f = Fact(
            relation_name="r", args=("A", "B"), layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="my-rule"),
        )
        kb.add_fact(f)
        kb._index_fact(kb.facts[-1])
        assert kb.relations["r"].facts == (kb.facts[-1],)
        assert any(fact.layer == Layer.REASONING for fact in kb.facts)


# ═══════════════════════ Sanity ═══════════════════════════════════


def test_kb_repr_summary(zebra_kb):
    r = repr(zebra_kb)
    assert "rules=8" in r
    assert "facts=68" in r


def test_kb_len_is_node_total(zebra_kb):
    # S1.7.23 — relations + rules + facts (no types / instances).
    expected = (
        len(zebra_kb.relations) + len(zebra_kb.rules) + len(zebra_kb.facts)
    )
    assert len(zebra_kb) == expected


# ═══════════════════════ Nested-fact args (Q40) ════════════════════


class TestNestedFactArgsThroughLoader:
    """Loader builds nested ``Fact`` instances for nested SForm args.

    Kernel-aligned per ``docs/kernel/ir/01-ein-graph/03_ein_model.md``
    §3 — relational nodes (parenthesised forms) are a first-class
    flavour of node and can appear as arguments to other facts.
    Q40 Option A relies on this for the synthetic
    ``(hypothesis (co-located Norwegian House-2))`` facts emitted
    at fork time.
    """

    def test_loader_constructs_nested_fact_in_args(self):
        from ein.ir import parse
        forms = parse(
            '(hypothesis (co-located Norwegian House-2) :layer fact)'
        )
        kb = KnowledgeBase.from_ir(forms)
        # The outer fact is registered (top-level).
        outer = next(f for f in kb.facts if f.relation_name == "hypothesis")
        # Its first arg is a Fact, not a string.
        assert len(outer.args) == 1
        assert isinstance(outer.args[0], Fact)
        inner = outer.args[0]
        assert inner.relation_name == "co-located"
        assert inner.args == ("Norwegian", "House-2")

    def test_nested_fact_equality_across_loads(self):
        """Two separate loads with identical IR produce equal nested facts."""
        from ein.ir import parse
        src = '(hypothesis (co-located Norwegian House-2) :layer fact)'
        kb1 = KnowledgeBase.from_ir(parse(src))
        kb2 = KnowledgeBase.from_ir(parse(src))
        f1 = next(f for f in kb1.facts if f.relation_name == "hypothesis")
        f2 = next(f for f in kb2.facts if f.relation_name == "hypothesis")
        assert f1 == f2
        assert hash(f1) == hash(f2)
        assert f1.args[0] == f2.args[0]

    def test_two_levels_of_nesting_through_loader(self):
        from ein.ir import parse
        forms = parse(
            '(contradiction-under     (hypothesis (co-located Norwegian House-2)) :layer fact)'
        )
        kb = KnowledgeBase.from_ir(forms)
        outer = next(f for f in kb.facts if f.relation_name == "contradiction-under")
        assert isinstance(outer.args[0], Fact)
        mid = outer.args[0]
        assert mid.relation_name == "hypothesis"
        assert isinstance(mid.args[0], Fact)
        innermost = mid.args[0]
        assert innermost.relation_name == "co-located"
        assert innermost.args == ("Norwegian", "House-2")


# ═══════════════════════ Snapshot — S1.5b.22 T1.5b.22.2 ════════════


class TestKBSnapshot:
    """:meth:`KnowledgeBase.snapshot` deep-copies the mutable state
    so a satisfying-branch kb returned from :func:`gaps_solve` is
    stable under later root mutations."""

    def test_kb_snapshot_isolation(self):
        from ein.ir import parse
        from ein.kb import Provenance
        text = """
        (type T)
        (instance a T)
        (instance b T)
        (relation r T T)
        (r a b :source "(1)")
        """
        kb = KnowledgeBase.from_ir(parse(text))
        snap = kb.snapshot()
        n_facts_at_snapshot = len(snap.facts)

        # Mutate source: add a new fact + a nogood.
        new_fact = Fact(
            relation_name="r", args=("b", "a"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="post-snapshot"),
        )
        kb.add_fact(new_fact)
        kb._index_fact(kb.facts[-1])
        kb._nogoods.add(frozenset({("h", ("x",))}))

        # Snapshot's mutable state unchanged.
        assert len(snap.facts) == n_facts_at_snapshot
        assert len(kb.facts) == n_facts_at_snapshot + 1
        # Snapshot's _nogoods is decoupled.
        assert snap._nogoods == set()
        assert kb._nogoods != snap._nogoods  # source mutated; snap didn't
        # The reverse-indexes on the snapshot reflect only the
        # snapshot-time facts.
        snap_r_facts = snap._facts_by_relation.get("r", ())
        # The fact (r a b) is in the original FACT layer; ensure it's
        # there but (r b a) is NOT.
        snap_r_args = {f.args for f in snap_r_facts}
        assert ("a", "b") in snap_r_args
        assert ("b", "a") not in snap_r_args

    def test_kb_snapshot_preserves_derivation_dag(self):
        """A snapshot's :meth:`derivation_dag` walks the same chain
        the source had at snapshot time, even after the source
        mutates."""
        from ein.ir import parse
        from ein.kb import Provenance
        text = """
        (type T)
        (instance a T) (instance b T)
        (relation p T T)
        (relation q T T)
        (p a b :source "(1)")
        """
        kb = KnowledgeBase.from_ir(parse(text))
        # Add a derived fact whose provenance points at the source.
        p_fact = kb._fact_by_id("p", ("a", "b"))
        assert p_fact is not None
        derived = Fact(
            relation_name="q", args=("a", "b"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="p-to-q",
                premises_raw=(("p", ("a", "b")),),
            ),
        )
        kb.add_fact(derived)
        kb._index_fact(kb.facts[-1])

        snap = kb.snapshot()
        # Snapshot dag walks one premise.
        snap_q = snap._fact_by_id("q", ("a", "b"))
        assert snap_q is not None
        snap_dag = snap.derivation_dag(snap_q)
        snap_source_relations = {
            (f.relation_name, f.args) for f in snap_dag.sources
        }
        assert ("p", ("a", "b")) in snap_source_relations

        # Mutate the source's q.provenance via adding *another* premise
        # is hard without re-instantiating; instead, drop the source's
        # `p` fact's derivation by clearing facts. Snapshot's dag must
        # still walk the same chain.
        kb.facts.clear()
        kb.rebuild_indexes()
        snap_dag2 = snap.derivation_dag(snap_q)
        snap_source_relations2 = {
            (f.relation_name, f.args) for f in snap_dag2.sources
        }
        assert snap_source_relations2 == snap_source_relations

    def test_kb_snapshot_shares_immutable_registries(self):
        """`relations` / `rules` are shared by reference — mutation of
        these on the source IS visible on the snapshot, by design.
        (S1.7.23 — there are no `types` / `instances` registries.)"""
        from ein.ir import parse
        text = """
        (type T)
        (instance a T)
        (relation r T T)
        """
        kb = KnowledgeBase.from_ir(parse(text))
        snap = kb.snapshot()
        assert snap.relations is kb.relations
        assert snap.rules is kb.rules

    def test_kb_snapshot_indexes_match_rebuild(self):
        """S1.7c.21 — ``snapshot`` shallow-copies the in-place-maintained
        indexes instead of calling ``rebuild_indexes``; the index *contents*
        must equal a full rebuild from the copied ``facts`` list, including a
        fact added incrementally via ``_index_fact`` after load.

        (The ``names`` key *order* legitimately differs — the shallow copy
        keeps deterministic insertion order, a rebuild uses set-iteration
        order, which is per-process random anyway; only contents are
        semantic, so this asserts ``==`` not key order.)
        """
        from ein.ir import parse
        from ein.kb import Provenance
        text = """
        (type T)
        (instance a T) (instance b T)
        (relation r T T)
        (relation s T T)
        (r a b :source "(1)")
        """
        kb = KnowledgeBase.from_ir(parse(text))
        # Exercise the incremental index path so the snapshot copies a
        # post-`_index_fact` state, not just a freshly-rebuilt one.
        derived = Fact(
            relation_name="s", args=("b", "a"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="r-to-s"),
        )
        kb.add_fact(derived)
        kb._index_fact(kb.facts[-1])

        snap = kb.snapshot()
        # Oracle: a second snapshot whose indexes are fully rebuilt.
        oracle = kb.snapshot()
        oracle.rebuild_indexes()

        assert snap._facts_by_relation == oracle._facts_by_relation
        assert snap._rule_apps_by_rule == oracle._rule_apps_by_rule
        assert snap._rule_apps_on_relation == oracle._rule_apps_on_relation
        assert snap._negated_facts == oracle._negated_facts
        assert snap._rules_by_relation == oracle._rules_by_relation
        assert snap.names == oracle.names

    def test_fork_snapshot_index_copy_contract(self):
        """S1.7c.22 — `fork` and `snapshot` share one index-copy contract
        (`_copy_fact_indexes_into`): the 5 fact-derived indexes are
        independent shallow copies that agree with the source by value;
        `_rules_by_relation` is shared by reference."""
        from ein.ir import parse
        text = """
        (type T)
        (instance a T) (instance b T)
        (relation r T T)
        (r a b :source "(1)")
        """
        kb = KnowledgeBase.from_ir(parse(text))
        for copy in (kb.fork(), kb.snapshot()):
            # Same contents by value …
            assert copy._facts_by_relation == kb._facts_by_relation
            assert copy._rule_apps_by_rule == kb._rule_apps_by_rule
            assert copy._rule_apps_on_relation == kb._rule_apps_on_relation
            assert copy.names == kb.names
            assert copy._negated_facts == kb._negated_facts
            # … `_rules_by_relation` shared by reference, the rest copied.
            assert copy._rules_by_relation is kb._rules_by_relation
            assert copy._facts_by_relation is not kb._facts_by_relation
            assert copy.names is not kb.names
            assert copy._negated_facts is not kb._negated_facts
