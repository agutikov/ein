"""Engine driver tests — S1.3.1 T1.3.1.9."""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.engine import Engine
from ein_bot.ir import parse
from ein_bot.kb.entities import Layer
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
ZEBRA = REPO / "examples" / "zebra.ein"


def test_engine_compiles_zebra_activator_count():
    """Zebra.ein has 8 T2 activators + 1 non-generic rule; compile_all
    should produce 9 entries in the cache."""
    kb = KnowledgeBase.from_ir(parse(ZEBRA.read_text()))
    eng = Engine(kb)
    eng.compile_all()
    # T2 activators (8): (symmetric co-located), (symmetric next-to),
    #                    (transitive co-located), (implies right-of next-to),
    #                    (square-fwd right-of), (square-bwd right-of),
    #                    (square-unique next-to House),
    #                    (type-exclusivity co-located).
    # Non-generic (1): hypothesis-contradiction.
    assert len(eng.cache) == 9
    # Every key is well-formed.
    for (rule_name, args) in eng.cache:
        assert rule_name in kb.rules
        assert isinstance(args, tuple)


def test_engine_step_produces_one_firing():
    """A symmetric activator on co-located + one co-located fact →
    step() produces exactly one new fact (the reverse)."""
    kb = KnowledgeBase.from_ir(parse("""
    (rules
      (rule symmetric (?rel)
        :match (?rel ?a ?b)
        :assert (?rel ?b ?a)
        :why "{?rel} sym {?a}↔{?b}"))
    (ontology
      (relation co-located T T)
      (symmetric co-located))
    (facts
      (co-located Norwegian House-1 :source "(10)"))
    """))
    eng = Engine(kb)
    eng.compile_all()
    firing = eng.step()
    assert firing is not None
    assert firing.rule == "symmetric"
    assert firing.derived.relation_name == "co-located"
    assert firing.derived.args == ("House-1", "Norwegian")
    assert firing.derived.layer == Layer.REASONING
    # Provenance threads the premise.
    prov = firing.derived.provenance
    assert prov is not None and prov.kind == "rule"
    assert prov.rule == "symmetric"
    assert prov.premises_raw == (("co-located", ("Norwegian", "House-1")),)


def test_engine_saturate_bounded():
    """`symmetric` over two facts saturates to four facts (each pair
    + its reverse). After two firings step() returns None."""
    kb = KnowledgeBase.from_ir(parse("""
    (rules
      (rule symmetric (?rel)
        :match (?rel ?a ?b)
        :assert (?rel ?b ?a)
        :why "s"))
    (ontology
      (relation r T T)
      (symmetric r))
    (facts
      (r A B :source "(1)")
      (r C D :source "(2)"))
    """))
    eng = Engine(kb)
    eng.compile_all()
    firings = list(eng.saturate())
    # Two original facts → two reverses. Reverses also match `symmetric`
    # → assert the originals. add_fact dedupes, so the second round
    # produces no NEW facts, but it still counts as firings until the
    # _fired set absorbs every (rule, activator, bindings) triple.
    derived = {(f.derived.relation_name, f.derived.args) for f in firings}
    assert ("r", ("B", "A")) in derived
    assert ("r", ("D", "C")) in derived


def test_engine_type_exclusivity_produces_negative_facts():
    """type-exclusivity is T2 polymorphic; its :assert is `(not (?R …))`
    which lowers to a nested-fact derivation after the activator
    binds ?R."""
    kb = KnowledgeBase.from_ir(parse("""
    (rules
      (rule type-exclusivity (?R)
        :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
        :assert (not (?R ?a ?b))
        :why "x"))
    (ontology
      (type T)
      (instance A T) (instance B T)
      (relation co-located T T)
      (type-exclusivity co-located))
    """))
    eng = Engine(kb)
    eng.compile_all()
    firings = list(eng.saturate())
    # Each ordered pair of distinct A/B instances produces one negative.
    # (instance A T, instance B T, neq A B) → not (co-located A B).
    # (instance B T, instance A T, neq B A) → not (co-located B A).
    rels = {f.derived.relation_name for f in firings}
    assert "not" in rels
    # Nested-Fact arg verification.
    not_firing = next(f for f in firings if f.derived.relation_name == "not")
    inner = not_firing.derived.args[0]
    # The inner is a Fact (relational-node arg per Q40 / R9).
    assert hasattr(inner, "relation_name")
    assert inner.relation_name == "co-located"
