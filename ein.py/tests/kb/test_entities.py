"""Unit tests for kb.entities — frozen dataclasses + back-pointer access.

These tests exercise the entity API *standalone* (no IR involved) so
they pin the structural contract independently of the loader. The
loader-driven cross-reference tests live in `test_store.py`.
"""
from __future__ import annotations

from ein.kb import (
    Fact,
    KnowledgeBase,
    Layer,
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


def test_fact_identity_by_relation_and_args_not_layer_or_provenance():
    from ein.kb import Provenance
    a = Fact(
        relation_name="co-located", args=("A", "B"),
        layer=Layer.FACT,
        provenance=Provenance.from_source("(2)"),
    )
    b = Fact(relation_name="co-located", args=("A", "B"), layer=Layer.ONTOLOGY)
    assert a == b


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
    f = Fact(relation_name="co-located", args=("A", "B"), layer=Layer.FACT)
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


# ── Layer enum coverage ────────────────────────────────────────────


def test_layer_values_distinct():
    assert {Layer.ONTOLOGY, Layer.FACT, Layer.REASONING} == set(Layer)
    # str roundtrip
    assert Layer.ONTOLOGY.value == "ontology"
    assert Layer.FACT.value == "fact"
    assert Layer.REASONING.value == "reasoning"


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
    assert p.type_names == ()


def test_pattern_instance_is_a_generic_relation():
    # S1.7.6: `instance` is no longer a kernel meta-primitive — it is an
    # ordinary relation. So `(instance ?a House)` in a pattern registers
    # `instance` in `relation_names` (like any relation head) and no
    # longer plucks `House` into `type_names` (which is now vestigial).
    from ein.ir import parse
    forms = parse(
        '(rule r () :match (and (instance ?a House) (co-located ?a ?b))'
        ' :assert (instance ?b Nationality) :why "")'
    )
    rule = forms[0]  # P1.7c: a flat top-level (rule …) form
    match_node = next(kw.value for kw in rule.args if hasattr(kw, "key") and kw.key.name == "match")
    p = Pattern.from_ir(match_node)
    assert "instance" in p.relation_names
    assert "co-located" in p.relation_names
    assert "House" not in p.type_names
    assert p.type_names == ()
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


def test_nested_fact_layer_excluded_from_identity():
    # As with non-nested facts: layer/provenance is metadata, not part
    # of identity. Two outer facts with nested facts in different
    # layers are still equal.
    from ein.kb import Provenance
    inner_fact = Fact(
        relation_name="co-located", args=("Norwegian", "House-2"),
        layer=Layer.FACT,
    )
    inner_reasoning = Fact(
        relation_name="co-located", args=("Norwegian", "House-2"),
        layer=Layer.REASONING,
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
