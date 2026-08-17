"""Unit tests for kb.entities — frozen dataclasses + back-pointer access.

These tests exercise the entity API *standalone* (no IR involved) so
they pin the structural contract independently of the loader. The
loader-driven cross-reference tests live in `test_store.py`.
"""
from __future__ import annotations

from ein.kb import (
    Fact,
    KnowledgeBase,
    Pattern,
    Relation,
    Rule,
)
from ein.kb.entities import _attach, _detach

# ── Identity & equality ────────────────────────────────────────────


# S1.7.23 — the `Type` / `Instance` identity tests were DELETED with
# those entity classes (the kernel keeps no type-system entity-view).


def test_relation_identity_by_name_and_signature():
    a = Relation(name="co-located", signature=("Attribute", "Attribute"))
    b = Relation(name="co-located", signature=("Attribute", "Attribute"))
    assert a == b


def test_fact_identity_by_relation_and_args_not_provenance():
    from ein.kb import Provenance
    a = Fact(
        relation_name="co-located", args=("A", "B"),
        provenance=Provenance.from_source("(2)"),
    )
    b = Fact(relation_name="co-located", args=("A", "B"))
    assert a == b


def test_origin_predicates_read_provenance():
    """S1.22.1b — `is_given` / `is_derived` replace the `Layer` enum.

    Three cases, matching what the three layers used to record: an
    authored condition, an engine derivation, and a background
    assumption (no annotation, or a source-kind record with no source).
    """
    from ein.kb import Provenance
    given = Fact(
        relation_name="co-located", args=("A", "B"),
        provenance=Provenance.from_source("(2)"),
    )
    assert (given.is_given, given.is_derived) == (True, False)

    for prov in (Provenance.from_rule(rule="transitive"),
                 Provenance.from_hypothesis(branch=1),
                 Provenance.rejected(branch=1)):
        derived = Fact(relation_name="co-located", args=("A", "B"),
                       provenance=prov)
        assert (derived.is_given, derived.is_derived) == (False, True)

    for prov in (None, Provenance.from_source(None)):
        background = Fact(relation_name="is-a", args=("Red", "Color"),
                          provenance=prov)
        assert (background.is_given, background.is_derived) == (False, False)


def test_rule_identity_by_name_only():
    a = Rule(name="symmetric", params=("rel",))
    b = Rule(name="symmetric", params=("rel",))
    assert a == b


# ── Detached entities return empty cross-refs ──────────────────────


def test_detached_relation_has_no_rules_or_facts():
    rel = Relation(name="co-located", signature=("Attribute", "Attribute"))
    assert rel.facts == ()
    assert rel.rules == ()
    assert rel.properties == ()


def test_detached_rule_has_no_applications():
    r = Rule(name="symmetric", params=("rel",))
    assert r.applications == ()
    assert r.relations == ()


def test_detached_fact_args_pass_through():
    f = Fact(relation_name="co-located", args=("A", "B"))
    assert f.relation is None
    assert f.arg_entities == ("A", "B")
    assert f.is_rule_application is False
    assert f.applied_rule is None


# ── _attach / _detach mechanics ────────────────────────────────────


def test_attach_does_not_break_equality():
    kb = KnowledgeBase()
    a = Relation(name="co-located")
    b = Relation(name="co-located")
    _attach(a, kb)
    # b is detached; eq is by data, not by _kb.
    assert a == b
    assert hash(a) == hash(b)


def test_attach_can_be_undone():
    kb = KnowledgeBase()
    t = Relation(name="co-located")
    _attach(t, kb)
    assert t._kb is kb
    _detach(t)
    assert t._kb is None


def test_attach_does_not_affect_repr_compare_hash():
    # _kb is excluded from compare/hash/repr.
    kb = KnowledgeBase()
    a = Relation(name="co-located", signature=("A", "B"))
    b = Relation(name="co-located", signature=("A", "B"))
    _attach(a, kb)
    assert repr(a) == repr(b)
    assert hash(a) == hash(b)


# S1.22.1b — `test_layer_values_distinct` was DELETED with the `Layer`
# enum. A fact's origin is its `Provenance`; `Fact.is_given` /
# `Fact.is_derived` are the derived predicates presentation reads, covered
# by `test_origin_predicates_read_provenance` above.


# ── Pattern structural extraction ──────────────────────────────────


def test_pattern_extracts_variables():
    from ein.ir import parse
    # Use a simple match clause: (co-located ?a ?b)
    forms = parse(
        '(rule r () :match (co-located ?a ?b) :assert (co-located ?b ?a) :why "x")'
    )
    rule = forms[0]  # P1.7c: a flat top-level (rule …) form
    match_node = next(
        kw.value for kw in rule.args
        if hasattr(kw, "key") and kw.key.name == "match"
    )
    p = Pattern.from_ir(match_node)
    assert p.variables == ("a", "b")
    assert p.relation_names == ("co-located",)


def test_pattern_membership_head_is_an_ordinary_relation():
    # The kernel builds no type-system entity-view: membership is stated by
    # an ordinary relation (`is-a`), and a pattern treats its head exactly
    # like any other relation head — it lands in `relation_names`, and the
    # second argument is NOT plucked out as a type name.
    from ein.ir import parse
    forms = parse(
        '(rule r () :match (and (is-a ?a House) (co-located ?a ?b))'
        ' :assert (is-a ?b Nationality) :why "")'
    )
    rule = forms[0]  # P1.7c: a flat top-level (rule …) form
    match_node = next(kw.value for kw in rule.args if hasattr(kw, "key") and kw.key.name == "match")
    p = Pattern.from_ir(match_node)
    assert p.relation_names == ("is-a", "co-located")
    assert set(p.variables) == {"a", "b"}


def test_pattern_dedupes_variables():
    from ein.ir import parse
    forms = parse(
        '(rule r () :match (and (co-located ?a ?b) (co-located ?b ?a)) :assert (= ?a ?b) :why "")'
    )
    rule = forms[0]  # P1.7c: a flat top-level (rule …) form
    match_node = next(kw.value for kw in rule.args if hasattr(kw, "key") and kw.key.name == "match")
    p = Pattern.from_ir(match_node)
    assert p.variables == ("a", "b")
    assert "co-located" in p.relation_names


def test_pattern_iter_yields_variables():
    from ein.ir import parse
    forms = parse('(rule r () :match (rel ?x ?y ?z) :assert (rel ?z ?y ?x) :why "")')
    rule = forms[0]  # P1.7c: a flat top-level (rule …) form
    match_node = next(kw.value for kw in rule.args if hasattr(kw, "key") and kw.key.name == "match")
    p = Pattern.from_ir(match_node)
    assert list(p) == ["x", "y", "z"]


# ── Nested-fact args — relational nodes (Q40 Option A) ────────────


def test_fact_args_admit_nested_fact():
    inner = Fact(relation_name="co-located", args=("Norwegian", "House-2"))
    outer = Fact(relation_name="hypothesis", args=(inner,))
    assert outer.args == (inner,)
    assert isinstance(outer.args[0], Fact)


def test_nested_fact_equality_propagates():
    a_inner = Fact(relation_name="co-located", args=("Norwegian", "House-2"))
    b_inner = Fact(relation_name="co-located", args=("Norwegian", "House-2"))
    a = Fact(relation_name="hypothesis", args=(a_inner,))
    b = Fact(relation_name="hypothesis", args=(b_inner,))
    assert a == b
    assert hash(a) == hash(b)


def test_nested_fact_distinct_inner_makes_outer_unequal():
    a = Fact(
        relation_name="hypothesis",
        args=(Fact(relation_name="co-located", args=("Norwegian", "House-2")),),
    )
    b = Fact(
        relation_name="hypothesis",
        args=(Fact(relation_name="co-located", args=("Norwegian", "House-3")),),
    )
    assert a != b
    assert hash(a) != hash(b)


def test_nested_fact_provenance_excluded_from_identity():
    # As with non-nested facts: provenance is metadata, not part of
    # identity. Two outer facts whose nested facts came from different
    # places are still equal.
    from ein.kb import Provenance
    inner_fact = Fact(
        relation_name="co-located", args=("Norwegian", "House-2"),
    )
    inner_reasoning = Fact(
        relation_name="co-located", args=("Norwegian", "House-2"),
        provenance=Provenance.from_hypothesis(branch=42),
    )
    a = Fact(relation_name="hypothesis", args=(inner_fact,))
    b = Fact(relation_name="hypothesis", args=(inner_reasoning,))
    assert a == b
    assert hash(a) == hash(b)


def test_nested_fact_arg_entities_returns_fact_as_is():
    inner = Fact(relation_name="co-located", args=("Norwegian", "House-2"))
    outer = Fact(relation_name="hypothesis", args=(inner,))
    # Detached entities still produce arg_entities via the str/int
    # passthrough; a Fact arg is returned as-is.
    ents = outer.arg_entities
    assert ents == (inner,)
    assert isinstance(ents[0], Fact)


def test_nested_fact_two_levels_deep():
    # (?outer (?mid (?inner a b))) — chain of three relational nodes.
    innermost = Fact(relation_name="co-located", args=("Norwegian", "House-2"))
    mid = Fact(relation_name="hypothesis", args=(innermost,))
    outer = Fact(relation_name="contradiction-under", args=(mid,))
    # Identity tuple cascades:
    twin = Fact(
        relation_name="contradiction-under",
        args=(Fact(relation_name="hypothesis",
                   args=(Fact(relation_name="co-located",
                              args=("Norwegian", "House-2")),)),),
    )
    assert outer == twin
    assert hash(outer) == hash(twin)


def test_nested_fact_set_membership():
    # Sets need hashing; nested-fact Facts must work in sets.
    f1 = Fact(
        relation_name="hypothesis",
        args=(Fact(relation_name="co-located", args=("Norwegian", "House-2")),),
    )
    f2 = Fact(
        relation_name="hypothesis",
        args=(Fact(relation_name="co-located", args=("Norwegian", "House-2")),),
    )
    s = {f1}
    assert f2 in s
