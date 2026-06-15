"""Predicate registry tests — S1.3.1 T1.3.1.9."""
from __future__ import annotations

from ein.inference import predicates
from ein.ir.types import Atom, Int, Var


def test_registry_contains_eq_and_neq_only():
    assert set(predicates.names()) == {"eq", "neq"}


def test_is_predicate():
    assert predicates.is_predicate("eq")
    assert predicates.is_predicate("neq")
    assert not predicates.is_predicate("foo")
    assert not predicates.is_predicate("co-located")
    assert not predicates.is_predicate("not")  # `not` is structural


def test_get_returns_callable_or_none():
    assert predicates.get("eq") is not None
    assert predicates.get("neq") is not None
    assert predicates.get("missing") is None


def test_eq_resolves_vars():
    fn = predicates.get("eq")
    assert fn is not None
    bindings = {"a": "Norwegian", "b": "Norwegian"}
    assert fn(bindings, (Var("a"), Var("b"))) is True
    bindings = {"a": "Norwegian", "b": "Spaniard"}
    assert fn(bindings, (Var("a"), Var("b"))) is False


def test_neq_resolves_vars():
    fn = predicates.get("neq")
    assert fn is not None
    bindings = {"a": "Norwegian", "b": "Spaniard"}
    assert fn(bindings, (Var("a"), Var("b"))) is True
    bindings = {"a": "Norwegian", "b": "Norwegian"}
    assert fn(bindings, (Var("a"), Var("b"))) is False


def test_eq_resolves_literal_atom_and_int():
    fn = predicates.get("eq")
    assert fn is not None
    # Atom literal vs bound var resolving to its name.
    bindings = {"x": "Red"}
    assert fn(bindings, (Var("x"), Atom("Red"))) is True
    assert fn(bindings, (Var("x"), Atom("Green"))) is False
    # Int literal vs bound int.
    bindings = {"n": 5}
    assert fn(bindings, (Var("n"), Int(5))) is True
    assert fn(bindings, (Var("n"), Int(6))) is False


def test_register_extends_registry():
    """Followup-hook: register a new predicate; verify lookup; clean up."""
    def always_true(b, args):
        return True
    predicates.register("always-true", always_true)
    try:
        assert predicates.is_predicate("always-true")
        assert predicates.get("always-true") is always_true
    finally:
        # Restore the registry to its M1-shipped state.
        del predicates._REGISTRY["always-true"]
    assert not predicates.is_predicate("always-true")


def test_loader_does_not_auto_vivify_predicates():
    """The kb loader must skip predicate names when auto-vivifying."""
    from ein.ir import parse
    from ein.kb.store import KnowledgeBase

    # A rule that uses `(neq ?a ?b)` positionally. The KB loader sees
    # the rule body but NOT a top-level `(neq …)` fact, so `neq`
    # should never appear in kb.relations.
    text = """
    (rule asym (?rel)
      :match (and (?rel ?a ?b) (neq ?a ?b))
      :assert (not (?rel ?b ?a))
      :why "asym")
    (relation r T T) (asym r)
    """
    kb = KnowledgeBase.from_ir(parse(text))
    assert "neq" not in kb.relations
    assert "eq" not in kb.relations
    # The user's relation `r` is registered.
    assert "r" in kb.relations
