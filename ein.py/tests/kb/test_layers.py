"""Tests for the fact view, fork, and encoding-agnostic helpers — S1.2.2.

S1.22.1b — the `TestLayerViews` class was DELETED with the `Layer` enum:
it pinned the ONTOLOGY / FACT / REASONING partition of `kb.facts` and the
three accessors that returned it. What a fact's origin is now lives on its
`Provenance`, and `FactView.by_source` / `by_rule` select over that.
"""
from __future__ import annotations

import pytest

from ein.kb import (
    Fact,
)

# ═══════════════════════ Fact view filters ═════════════════════════


class TestFactViewFilters:
    def test_relation_filter(self, zebra_kb):
        co_loc = list(zebra_kb.all_facts().relation("co-located"))
        # 12 authored conditions (2,3,4,5,7,8,9,10,13,14); check >= 10.
        assert len(co_loc) >= 10
        for f in co_loc:
            assert f.relation_name == "co-located"

    def test_about_filter_by_name(self, zebra_kb):
        # `about` takes a node name (S1.7.23 — no Instance entities).
        # Norwegian appears in: 1 instance fact, 1 co-located, 1 next-to.
        facts = list(zebra_kb.all_facts().about("Norwegian"))
        assert len(facts) == 3
        for f in facts:
            assert "Norwegian" in f.args

    def test_by_source(self, zebra_kb):
        f = list(zebra_kb.all_facts().by_source("condition (10)"))
        assert len(f) == 1
        assert f[0].relation_name == "co-located"
        assert f[0].args == ("Norwegian", "House-1")

    def test_by_rule_returns_empty_pre_reasoning(self, zebra_kb):
        # Before any reasoning happens, no facts have :rule provenance.
        assert list(zebra_kb.all_facts().by_rule("transitive")) == []

    def test_view_iter_protocol(self, zebra_kb):
        v = zebra_kb.all_facts()
        assert isinstance(list(v), list)
        assert len(list(v)) == len(v)

    def test_view_contains(self, zebra_kb):
        v = zebra_kb.all_facts()
        some_fact = next(iter(v))
        assert some_fact in v
        # Construct a fact that doesn't exist in the view.
        ghost = Fact(relation_name="nonexistent", args=("a", "b"))
        assert ghost not in v

    def test_view_repr(self, zebra_kb):
        # 5 relation-decl + 7 type + 30 instance + 6 property-application
        # + 4 condition-(1) spatial = 52 background, + 14 authored
        # conditions (2)-(15) = 66.
        r = repr(zebra_kb.all_facts())
        assert "all" in r
        assert "len=66" in r

    def test_matching_stub_raises(self, zebra_kb):
        # P1.3 seam — until then, .matching() is intentionally a stub.
        with pytest.raises(NotImplementedError):
            zebra_kb.all_facts().matching(pattern=None)


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

    def test_fork_derivation_isolation(self, zebra_kb):
        from ein.kb import Provenance
        # Add a derived fact to the fork; the parent must NOT see it.
        fork = zebra_kb.fork()
        derived = Fact(
            relation_name="co-located",
            args=("Norwegian", "Water"),
            provenance=Provenance.from_rule(rule="hypothetical"),
        )
        fork.add_fact(derived)
        fork._index_fact(fork.facts[-1])
        assert len(list(fork.all_facts().by_rule("hypothetical"))) == 1
        # Parent unchanged.
        assert list(zebra_kb.all_facts().by_rule("hypothetical")) == []

    def test_fork_entity_back_pointer_caveat(self, zebra_kb):
        """A shared `Relation` entity's `.facts` returns the *original*
        kb's facts.

        Shared entities keep their `_kb` pointing at the root; the
        fork's derived additions are reachable only via the fork's
        view methods. This is intentional — entity API == root state;
        view API == branch state.
        """
        from ein.kb import Provenance
        fork = zebra_kb.fork()
        co_located = zebra_kb.relations["co-located"]
        before = len(co_located.facts)
        derived = Fact(
            relation_name="co-located",
            args=("Norwegian", "Water"),
            provenance=Provenance.from_rule(rule="hypothetical"),
        )
        fork.add_fact(derived)
        fork._index_fact(fork.facts[-1])
        # The shared entity still reports the ROOT's facts (unchanged).
        assert len(co_located.facts) == before
        # Fork view sees the new derivation.
        fork_view = list(fork.all_facts().about("Norwegian"))
        assert any(
            f.relation_name == "co-located" and f.args == ("Norwegian", "Water")
            for f in fork_view
        )


# S1.7.23 — the `TestLogicalTypes` / `TestLogicalInstances` /
# `TestEncodingDriftDetection` classes were DELETED: `logical_types` /
# `logical_instances` (the `is-a`-bridge for the removed `kb.types` /
# `kb.instances` entity-view) no longer exist. A puzzle's named-type
# projection is now a user-space ein-lang rule over `is-a`.
