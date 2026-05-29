"""Integration tests for KnowledgeBase loading + cross-references.

These exercise the S1.2.1 acceptance criteria against the canonical
examples (zebra.ein and zebra2.ein).
"""
from __future__ import annotations

from ein_bot.kb import (
    Fact,
    Instance,
    KnowledgeBase,
    Layer,
)

# ═══════════════════════ zebra.ein (classic split) ════════════════


class TestZebraCounts:
    """Top-level entity counts against the S1.2.1 acceptance."""

    def test_seven_types(self, zebra_kb):
        # Attribute + 6 leaf types.
        assert len(zebra_kb.types) == 7
        assert set(zebra_kb.types) == {
            "Attribute", "House", "Color", "Nationality",
            "Pet", "Cigarette", "Drink",
        }

    def test_thirty_instances(self, zebra_kb):
        # 5 houses + 5 of each of 5 attribute categories = 30.
        # (The early S1.2.1 draft said 25; off-by-one — see plan
        # acceptance update.)
        assert len(zebra_kb.instances) == 30

    def test_three_declared_relations(self, zebra_kb):
        declared = sorted(
            n for n, r in zebra_kb.relations.items() if r.declared
        )
        assert declared == ["co-located", "next-to", "right-of"]

    def test_open_world_relations_include_rule_names(self, zebra_kb):
        # Property tags (heads of `(symmetric R)` etc.) auto-vivify
        # as open-world relations.
        open_world = sorted(
            n for n, r in zebra_kb.relations.items() if not r.declared
        )
        assert {"symmetric", "transitive", "implies", "square-fwd",
                "square-bwd", "instance"} <= set(open_world)

    def test_eight_rules(self, zebra_kb):
        """S1.3.2 + square-unique addition for corner-house spatial inference."""
        assert len(zebra_kb.rules) == 8
        assert set(zebra_kb.rules) == {
            "symmetric", "transitive", "implies",
            "square-fwd", "square-bwd", "square-unique",
            "type-exclusivity",
            "hypothesis-contradiction",
        }


class TestZebraTypeHierarchy:
    def test_attribute_has_no_parent(self, zebra_kb):
        assert zebra_kb.types["Attribute"].parent is None

    def test_house_has_attribute_parent(self, zebra_kb):
        assert zebra_kb.types["House"].parent is zebra_kb.types["Attribute"]

    def test_attribute_children(self, zebra_kb):
        kids = {t.name for t in zebra_kb.types["Attribute"].children}
        assert kids == {"House", "Color", "Nationality", "Pet", "Cigarette", "Drink"}

    def test_house_has_five_instances(self, zebra_kb):
        names = {i.name for i in zebra_kb.types["House"].instances}
        assert names == {"House-1", "House-2", "House-3", "House-4", "House-5"}

    def test_ancestors_chain(self, zebra_kb):
        # House -> Attribute -> (root)
        chain = [t.name for t in zebra_kb.types["House"].ancestors()]
        assert chain == ["Attribute"]


class TestZebraInstance:
    def test_norwegian_type(self, zebra_kb):
        n = zebra_kb.instances["Norwegian"]
        assert n.type == zebra_kb.types["Nationality"]

    def test_norwegian_facts_include_instance_and_explicit(self, zebra_kb):
        # Per S1.2.1 acceptance: Norwegian.facts includes both the
        # implicit `(instance Norwegian Nationality)` and explicit
        # co-located / next-to facts.
        facts = zebra_kb.instances["Norwegian"].facts
        rels = [(f.relation_name, f.layer) for f in facts]
        assert ("instance", Layer.ONTOLOGY) in rels
        assert ("co-located", Layer.FACT) in rels
        assert ("next-to", Layer.FACT) in rels

    def test_house_1_facts_layer_split(self, zebra_kb):
        f = zebra_kb.instances["House-1"].facts
        layers = {fact.layer for fact in f}
        # Appears in ontology (instance + structural right-of) and in
        # fact layer (condition (10)).
        assert Layer.ONTOLOGY in layers
        assert Layer.FACT in layers


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

    def test_relation_signature_resolution(self, zebra_kb):
        rel = zebra_kb.relations["co-located"]
        assert rel.signature == ("Attribute", "Attribute")
        sig_types = rel.signature_types
        assert len(sig_types) == 2
        assert sig_types[0] is zebra_kb.types["Attribute"]


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
        # Ontology: 30 instance + 8 rule-app + 4 spatial + 3 relation-decl = 45.
        # (rule-apps include square-unique + type-exclusivity activators;
        #  relation-decl facts are co-located, right-of, next-to.)
        # Facts: 14 (conditions 2..15).
        # Total: 59.
        assert len(zebra_kb.facts) == 59

    def test_fact_resolves_relation(self, zebra_kb):
        fs = [f for f in zebra_kb.facts if f.source == "condition (10)"]
        assert len(fs) == 1
        f = fs[0]
        assert f.relation_name == "co-located"
        assert f.relation == zebra_kb.relations["co-located"]
        assert f.layer == Layer.FACT

    def test_fact_arg_entities_resolution(self, zebra_kb):
        fs = [f for f in zebra_kb.facts if f.source == "condition (10)"]
        f = fs[0]
        a, b = f.arg_entities
        assert isinstance(a, Instance)
        assert isinstance(b, Instance)
        assert a.name == "Norwegian"
        assert b.name == "House-1"

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

    def test_zebra2_no_types_or_instances(self, zebra2_kb):
        # All inheritance is expressed via `is-a` facts; no `(type …)`
        # or `(instance …)` declarations.
        assert zebra2_kb.types == {}
        assert zebra2_kb.instances == {}

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
        from ein_bot.ir import parse
        text = """
        (ontology
          (type Foo)
          (instance A Foo))
        (facts
          (mystery-relation A B :source "(1)"))
        """
        kb = KnowledgeBase.from_ir(parse(text))
        assert "mystery-relation" in kb.relations
        rel = kb.relations["mystery-relation"]
        assert rel.declared is False
        assert len(rel.facts) == 1

    def test_undeclared_type_auto_vivifies(self):
        from ein_bot.ir import parse
        text = """
        (ontology
          (instance A NoSuchType))
        """
        kb = KnowledgeBase.from_ir(parse(text))
        assert "NoSuchType" in kb.types
        assert kb.types["NoSuchType"].parent is None
        assert kb.instances["A"].type == kb.types["NoSuchType"]


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
        from ein_bot.kb.store import EqClasses
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
        from ein_bot.ir import parse
        text = """
        (ontology
          (type T)
          (instance A T)
          (relation r T T))
        """
        from ein_bot.kb import Provenance
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
    assert "types=7" in r
    assert "rules=8" in r
    assert "facts=59" in r


def test_kb_len_is_node_total(zebra_kb):
    # types + instances + relations + rules + facts
    expected = (
        len(zebra_kb.types) + len(zebra_kb.instances)
        + len(zebra_kb.relations) + len(zebra_kb.rules)
        + len(zebra_kb.facts)
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
        from ein_bot.ir import parse
        forms = parse(
            "(facts "
            "  (hypothesis (co-located Norwegian House-2)))"
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
        from ein_bot.ir import parse
        src = "(facts (hypothesis (co-located Norwegian House-2)))"
        kb1 = KnowledgeBase.from_ir(parse(src))
        kb2 = KnowledgeBase.from_ir(parse(src))
        f1 = next(f for f in kb1.facts if f.relation_name == "hypothesis")
        f2 = next(f for f in kb2.facts if f.relation_name == "hypothesis")
        assert f1 == f2
        assert hash(f1) == hash(f2)
        assert f1.args[0] == f2.args[0]

    def test_two_levels_of_nesting_through_loader(self):
        from ein_bot.ir import parse
        forms = parse(
            "(facts "
            "  (contradiction-under "
            "    (hypothesis (co-located Norwegian House-2))))"
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
        from ein_bot.ir import parse
        from ein_bot.kb import Provenance
        text = """
        (ontology
          (type T)
          (instance a T)
          (instance b T)
          (relation r T T))
        (facts (r a b :source "(1)"))
        """
        kb = KnowledgeBase.from_ir(parse(text))
        snap = kb.snapshot()
        n_facts_at_snapshot = len(snap.facts)

        # Mutate source: add a new fact + a nogood + a
        # committed-hypothesis marker.
        new_fact = Fact(
            relation_name="r", args=("b", "a"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="post-snapshot"),
        )
        kb.add_fact(new_fact)
        kb._index_fact(kb.facts[-1])
        kb._nogoods.add(frozenset({("h", ("x",))}))
        kb.committed_hypotheses.add(("h", ("x",)))

        # Snapshot's mutable state unchanged.
        assert len(snap.facts) == n_facts_at_snapshot
        assert len(kb.facts) == n_facts_at_snapshot + 1
        # Snapshot's _nogoods + committed_hypotheses are decoupled.
        assert snap._nogoods == set()
        assert snap.committed_hypotheses == set()
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
        from ein_bot.ir import parse
        from ein_bot.kb import Provenance
        text = """
        (ontology
          (type T)
          (instance a T) (instance b T)
          (relation p T T)
          (relation q T T))
        (facts (p a b :source "(1)"))
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
        """Registries (types / instances / relations / rules) are
        shared by reference — mutation of these on the source IS
        visible on the snapshot, by design. This documents the
        contract."""
        from ein_bot.ir import parse
        text = """
        (ontology
          (type T)
          (instance a T)
          (relation r T T))
        """
        kb = KnowledgeBase.from_ir(parse(text))
        snap = kb.snapshot()
        assert snap.types is kb.types
        assert snap.instances is kb.instances
        assert snap.relations is kb.relations
        assert snap.rules is kb.rules
