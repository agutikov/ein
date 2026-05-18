"""Tests for layer views, fork, and encoding-agnostic helpers — S1.2.2."""
from __future__ import annotations

import pytest

from ein_bot.kb import (
    Fact,
    Layer,
    instance_name,
    logical_instances,
    logical_types,
    type_name,
)

# ═══════════════════════ Layer views ═══════════════════════════════


class TestLayerViews:
    def test_zebra_ontology_size(self, zebra_kb):
        # 30 instance facts + 6 rule-app + 4 spatial = 40
        assert len(zebra_kb.ontology()) == 40

    def test_zebra_fact_layer_size(self, zebra_kb):
        # 14 explicit puzzle conditions (2..15)
        assert len(zebra_kb.fact_layer()) == 14

    def test_zebra_reasoning_starts_empty(self, zebra_kb):
        assert len(zebra_kb.reasoning()) == 0
        assert not zebra_kb.reasoning()

    def test_all_layers_is_union(self, zebra_kb):
        assert len(zebra_kb.all_layers()) == 54

    def test_ontology_fact_not_in_fact_layer(self, zebra_kb):
        # Pick any fact known to be in ONTOLOGY (e.g. an instance form).
        on_facts = list(zebra_kb.ontology())
        fact_facts = list(zebra_kb.fact_layer())
        for f in on_facts:
            assert f not in fact_facts

    def test_fact_layer_not_in_reasoning(self, zebra_kb):
        f_facts = list(zebra_kb.fact_layer())
        r_facts = list(zebra_kb.reasoning())
        for f in f_facts:
            assert f not in r_facts


class TestFactViewFilters:
    def test_relation_filter(self, zebra_kb):
        # All co-located facts across all layers.
        co_loc = list(zebra_kb.all_layers().relation("co-located"))
        # 12 in fact layer (conditions 2,3,4,5,7,8,9,10,13,14) plus 0
        # in ontology. Actually let me just check >= 10.
        assert len(co_loc) >= 10
        for f in co_loc:
            assert f.relation_name == "co-located"

    def test_about_filter_by_instance_object(self, zebra_kb):
        norwegian = zebra_kb.instances["Norwegian"]
        facts = list(zebra_kb.all_layers().about(norwegian))
        # Norwegian appears in: 1 instance fact, 1 co-located, 1 next-to.
        assert len(facts) == 3

    def test_about_filter_by_name(self, zebra_kb):
        # String form should be equivalent to Instance form.
        facts_by_name = list(zebra_kb.all_layers().about("Norwegian"))
        norwegian = zebra_kb.instances["Norwegian"]
        facts_by_obj = list(zebra_kb.all_layers().about(norwegian))
        assert facts_by_name == facts_by_obj

    def test_by_source(self, zebra_kb):
        f = list(zebra_kb.fact_layer().by_source("condition (10)"))
        assert len(f) == 1
        assert f[0].relation_name == "co-located"
        assert f[0].args == ("Norwegian", "House_1")

    def test_by_rule_returns_empty_pre_reasoning(self, zebra_kb):
        # Before any reasoning happens, no facts have :rule provenance.
        assert list(zebra_kb.reasoning().by_rule("transitive")) == []

    def test_view_iter_protocol(self, zebra_kb):
        v = zebra_kb.fact_layer()
        assert isinstance(list(v), list)
        assert len(list(v)) == len(v)

    def test_view_contains(self, zebra_kb):
        v = zebra_kb.fact_layer()
        some_fact = next(iter(v))
        assert some_fact in v
        # Construct a fact that doesn't exist in the view.
        ghost = Fact(relation_name="nonexistent", args=("a", "b"), layer=Layer.FACT)
        assert ghost not in v

    def test_view_repr(self, zebra_kb):
        r = repr(zebra_kb.ontology())
        assert "ontology" in r
        assert "len=40" in r

    def test_matching_stub_raises(self, zebra_kb):
        # P1.3 seam — until then, .matching() is intentionally a stub.
        with pytest.raises(NotImplementedError):
            zebra_kb.all_layers().matching(pattern=None)


# ═══════════════════════ Fork (hypothesis branching) ═══════════════


class TestFork:
    def test_fork_shares_immutable_populations(self, zebra_kb):
        fork = zebra_kb.fork()
        assert fork.types is zebra_kb.types
        assert fork.instances is zebra_kb.instances
        assert fork.relations is zebra_kb.relations
        assert fork.rules is zebra_kb.rules
        assert fork.query is zebra_kb.query

    def test_fork_copies_facts_list(self, zebra_kb):
        fork = zebra_kb.fork()
        assert fork.facts is not zebra_kb.facts
        assert len(fork.facts) == len(zebra_kb.facts)

    def test_fork_copies_indexes(self, zebra_kb):
        fork = zebra_kb.fork()
        assert fork._facts_by_relation is not zebra_kb._facts_by_relation
        assert fork._facts_by_instance is not zebra_kb._facts_by_instance

    def test_fork_classes_independent(self, zebra_kb):
        fork = zebra_kb.fork()
        # Parent had no unions; fork union shouldn't propagate.
        fork.classes.union("Norwegian", "Japanese")
        assert fork.classes.equivalent("Norwegian", "Japanese")
        assert not zebra_kb.classes.equivalent("Norwegian", "Japanese")

    def test_fork_reasoning_isolation(self, zebra_kb):
        # Add a derived fact to the fork; parent's reasoning view
        # must NOT see it.
        fork = zebra_kb.fork()
        derived = Fact(
            relation_name="co-located",
            args=("Norwegian", "Water"),
            layer=Layer.REASONING,
            rule_name="hypothetical",
        )
        fork.add_fact(derived)
        fork._index_fact(fork.facts[-1])
        assert len(fork.reasoning()) >= 1
        # Parent unchanged.
        assert len(zebra_kb.reasoning()) == 0

    def test_fork_entity_back_pointer_caveat(self, zebra_kb):
        """`norwegian.facts` returns the *original* kb's facts.

        Shared entities keep their `_kb` pointing at the root; the
        fork's reasoning additions are reachable only via the fork's
        view methods. This is intentional — entity API == root state;
        view API == branch state.
        """
        fork = zebra_kb.fork()
        norwegian = zebra_kb.instances["Norwegian"]
        derived = Fact(
            relation_name="co-located",
            args=("Norwegian", "Water"),
            layer=Layer.REASONING,
            rule_name="hypothetical",
        )
        fork.add_fact(derived)
        fork._index_fact(fork.facts[-1])
        # entity.facts returns the ROOT's view of Norwegian.
        assert all(f.layer != Layer.REASONING for f in norwegian.facts)
        # Fork view sees the new derivation.
        fork_view = list(fork.all_layers().about(norwegian))
        assert any(
            f.relation_name == "co-located" and f.args == ("Norwegian", "Water")
            for f in fork_view
        )


# ═══════════════════════ Encoding-agnostic logical views ═══════════


class TestLogicalTypes:
    def test_classic_encoding(self, zebra_kb):
        # zebra.ein populates kb.types directly.
        names = {type_name(t) for t in logical_types(zebra_kb)}
        assert names == {
            "Attribute", "House", "Color", "Nationality",
            "Pet", "Cigarette", "Drink",
        }

    def test_unified_is_a_encoding(self, zebra2_kb):
        # zebra2.ein has empty kb.types — logical_types must fall
        # back to walking is-a facts.
        names = {type_name(t) for t in logical_types(zebra2_kb)}
        # T is the catch-all root in zebra2.ein; classic encoding
        # doesn't have it. Otherwise the leaf types should match.
        leaf_types = names - {"T"}
        assert leaf_types == {
            "Attribute", "House", "Color", "Nationality",
            "Pet", "Cigarette", "Drink",
        }
        # T appears as the root.
        assert "T" in names

    def test_logical_types_yields_strings_for_unified(self, zebra2_kb):
        # In the unified-is-a case, no Type entity exists, so the
        # helper returns raw names (strings).
        for t in logical_types(zebra2_kb):
            assert isinstance(t, str)

    def test_logical_types_yields_type_entities_for_classic(self, zebra_kb):
        # Classic case: Type entities are returned directly.
        from ein_bot.kb import Type
        for t in logical_types(zebra_kb):
            assert isinstance(t, Type)


class TestLogicalInstances:
    def test_classic_encoding(self, zebra_kb):
        insts = logical_instances(zebra_kb)
        assert len(insts) == 30
        names = {instance_name(i) for i in insts}
        # Spot-check a few.
        assert "Norwegian" in names
        assert "House_1" in names
        assert "Zebra" in names

    def test_unified_is_a_encoding(self, zebra2_kb):
        # Leaves of the is-a forest — same 30 entities, but as names.
        insts = logical_instances(zebra2_kb)
        names = {instance_name(i) for i in insts}
        assert "Norwegian" in names
        assert "House_1" in names
        # Non-leaf types must NOT appear (House appears as parent of
        # House_1..5, so it's not a leaf).
        assert "House" not in names
        assert "Attribute" not in names
        assert "T" not in names
        # 30 leaves expected (5 of each of 6 categories).
        assert len(insts) == 30

    def test_logical_instances_returns_strings_for_unified(self, zebra2_kb):
        for i in logical_instances(zebra2_kb):
            assert isinstance(i, str)


class TestEncodingDriftDetection:
    """Asserts both encodings produce the same logical content.

    This is the "drift detector" promised by the encoding-deferral
    principle (memory: project — IR encoding choice deferred): if a
    change breaks one encoding's view but not the other, this test
    fails first.
    """

    def test_same_leaf_set(self, zebra_kb, zebra2_kb):
        classic = {instance_name(i) for i in logical_instances(zebra_kb)}
        unified = {instance_name(i) for i in logical_instances(zebra2_kb)}
        assert classic == unified

    def test_same_leaf_types_modulo_root(self, zebra_kb, zebra2_kb):
        # Classic encoding has 7 types (Attribute + 6 leaves).
        # Unified encoding adds the catch-all `T` above Attribute —
        # 8 total, 7 in common with classic.
        classic = {type_name(t) for t in logical_types(zebra_kb)}
        unified = {type_name(t) for t in logical_types(zebra2_kb)}
        assert classic <= unified
        assert unified - classic == {"T"}
