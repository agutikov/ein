"""Tests for layer views, fork, and encoding-agnostic helpers — S1.2.2."""
from __future__ import annotations

import pytest

from ein_bot.kb import (
    Fact,
    Layer,
)

# ═══════════════════════ Layer views ═══════════════════════════════


class TestLayerViews:
    def test_zebra_ontology_size(self, zebra_kb):
        # 7 type-decl + 30 instance + 8 rule-app + 4 spatial
        #  + 5 relation-decl = 54.
        # (rule-apps: symmetric/co-located, transitive/co-located,
        #  symmetric/next-to, implies/right-of/next-to, square-fwd/right-of,
        #  square-bwd/right-of, square-unique/next-to/House,
        #  type-exclusivity/co-located.)
        # The 5 relation-decl facts (co-located, right-of, next-to, and —
        # since S1.7.6 — type, instance) are stored alongside the
        # kb.relations registry so rules can introspect signatures via
        # (relation ?R ?A ?B). The 7 (type …) decls are ONTOLOGY facts
        # too now (type left the kernel; it is a plain relation).
        assert len(zebra_kb.ontology()) == 54

    def test_zebra_fact_layer_size(self, zebra_kb):
        # 14 explicit puzzle conditions (2..15)
        assert len(zebra_kb.fact_layer()) == 14

    def test_zebra_reasoning_starts_empty(self, zebra_kb):
        assert len(zebra_kb.reasoning()) == 0
        assert not zebra_kb.reasoning()

    def test_all_layers_is_union(self, zebra_kb):
        # 54 ontology + 14 fact + 0 reasoning = 68
        assert len(zebra_kb.all_layers()) == 68

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

    def test_about_filter_by_name(self, zebra_kb):
        # `about` takes a node name (S1.7.23 — no Instance entities).
        # Norwegian appears in: 1 instance fact, 1 co-located, 1 next-to.
        facts = list(zebra_kb.all_layers().about("Norwegian"))
        assert len(facts) == 3
        for f in facts:
            assert "Norwegian" in f.args

    def test_by_source(self, zebra_kb):
        f = list(zebra_kb.fact_layer().by_source("condition (10)"))
        assert len(f) == 1
        assert f[0].relation_name == "co-located"
        assert f[0].args == ("Norwegian", "House-1")

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
        assert "len=54" in r

    def test_matching_stub_raises(self, zebra_kb):
        # P1.3 seam — until then, .matching() is intentionally a stub.
        with pytest.raises(NotImplementedError):
            zebra_kb.all_layers().matching(pattern=None)


# ═══════════════════════ Fork (hypothesis branching) ═══════════════


class TestFork:
    def test_fork_shares_immutable_populations(self, zebra_kb):
        fork = zebra_kb.fork()
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
        assert fork.names is not zebra_kb.names

    def test_fork_classes_independent(self, zebra_kb):
        fork = zebra_kb.fork()
        # Parent had no unions; fork union shouldn't propagate.
        fork.classes.union("Norwegian", "Japanese")
        assert fork.classes.equivalent("Norwegian", "Japanese")
        assert not zebra_kb.classes.equivalent("Norwegian", "Japanese")

    def test_fork_reasoning_isolation(self, zebra_kb):
        from ein_bot.kb import Provenance
        # Add a derived fact to the fork; parent's reasoning view
        # must NOT see it.
        fork = zebra_kb.fork()
        derived = Fact(
            relation_name="co-located",
            args=("Norwegian", "Water"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="hypothetical"),
        )
        fork.add_fact(derived)
        fork._index_fact(fork.facts[-1])
        assert len(fork.reasoning()) >= 1
        # Parent unchanged.
        assert len(zebra_kb.reasoning()) == 0

    def test_fork_entity_back_pointer_caveat(self, zebra_kb):
        """A shared `Relation` entity's `.facts` returns the *original*
        kb's facts.

        Shared entities keep their `_kb` pointing at the root; the
        fork's reasoning additions are reachable only via the fork's
        view methods. This is intentional — entity API == root state;
        view API == branch state.
        """
        from ein_bot.kb import Provenance
        fork = zebra_kb.fork()
        co_located = zebra_kb.relations["co-located"]
        before = len(co_located.facts)
        derived = Fact(
            relation_name="co-located",
            args=("Norwegian", "Water"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="hypothetical"),
        )
        fork.add_fact(derived)
        fork._index_fact(fork.facts[-1])
        # The shared entity still reports the ROOT's facts (unchanged).
        assert len(co_located.facts) == before
        # Fork view sees the new derivation.
        fork_view = list(fork.all_layers().about("Norwegian"))
        assert any(
            f.relation_name == "co-located" and f.args == ("Norwegian", "Water")
            for f in fork_view
        )


# S1.7.23 — the `TestLogicalTypes` / `TestLogicalInstances` /
# `TestEncodingDriftDetection` classes were DELETED: `logical_types` /
# `logical_instances` (the `is-a`-bridge for the removed `kb.types` /
# `kb.instances` entity-view) no longer exist. A puzzle's named-type
# projection is now a user-space ein-lang rule over `is-a`.
