"""Runtime matcher tests — S1.3.1 T1.3.1.9."""
from __future__ import annotations

from ein_bot.inference import match
from ein_bot.inference.compile import compile_rule
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def test_single_scan_binds_binary_fact():
    kb = _kb("""
    (rules
      (rule mirror (?r)
        :match (?r ?a ?b)
        :assert (?r ?b ?a)
        :why "m"))
    (ontology
      (relation co-located T T)
      (mirror co-located))
    (facts
      (co-located Norwegian House_1 :source "(1)"))
    """)
    plan = compile_rule(kb.rules["mirror"], kb._facts_by_relation["mirror"][0])
    results = list(match.run(plan, kb))
    assert len(results) == 1
    bindings, premises = results[0]
    assert bindings["r"] == "co-located"
    assert bindings["a"] == "Norwegian"
    assert bindings["b"] == "House_1"
    assert premises[0].relation_name == "co-located"


def test_join_with_shared_var_chains_facts():
    kb = _kb("""
    (rules
      (rule transitive (?rel)
        :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
        :assert (?rel ?a ?c)
        :why "t"))
    (ontology
      (relation r T T)
      (transitive r))
    (facts
      (r A B :source "(1)")
      (r B C :source "(2)")
      (r C D :source "(3)"))
    """)
    plan = compile_rule(
        kb.rules["transitive"],
        kb._facts_by_relation["transitive"][0],
    )
    results = list(match.run(plan, kb))
    # Expected matches: A-B-C, B-C-D (each chain of two adjacent edges
    # with neq enforced).
    chains = sorted(
        (b["a"], b["b"], b["c"]) for b, _ in results
    )
    assert chains == [("A", "B", "C"), ("B", "C", "D")]


def test_neq_guard_prunes_self_loops():
    kb = _kb("""
    (rules
      (rule self (?rel)
        :match (and (?rel ?a ?a))
        :assert (?rel ?a ?a)
        :why "s"))
    (ontology (relation r T T) (self r))
    (facts (r X X :source "(1)") (r Y Z :source "(2)"))
    """)
    # No neq guard here — test the `?a ?a` repeated-var unification.
    plan = compile_rule(
        kb.rules["self"], kb._facts_by_relation["self"][0],
    )
    results = list(match.run(plan, kb))
    # Only (r X X) matches — (r Y Z) fails the repeated-var unification.
    assert len(results) == 1
    assert results[0][0]["a"] == "X"


def test_neq_guard_in_transitive_prunes_2_cycles():
    kb = _kb("""
    (rules
      (rule transitive (?rel)
        :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
        :assert (?rel ?a ?c)
        :why "t"))
    (ontology (relation r T T) (transitive r))
    (facts (r A B :source "(1)") (r B A :source "(2)"))
    """)
    plan = compile_rule(
        kb.rules["transitive"],
        kb._facts_by_relation["transitive"][0],
    )
    results = list(match.run(plan, kb))
    # A-B-A and B-A-B are pruned by neq; no other chains exist.
    assert results == []


def test_negation_as_failure_succeeds_when_inner_empty():
    kb = _kb("""
    (rules
      (rule guarded (?r)
        :match (and (?r ?a ?b) (not (other ?a ?b)))
        :assert (ok ?a ?b)
        :why "g"))
    (ontology
      (relation r T T) (relation other T T)
      (guarded r))
    (facts (r X Y :source "(1)"))
    """)
    plan = compile_rule(
        kb.rules["guarded"], kb._facts_by_relation["guarded"][0],
    )
    results = list(match.run(plan, kb))
    assert len(results) == 1
    assert results[0][0]["a"] == "X" and results[0][0]["b"] == "Y"


def test_negation_as_failure_fails_when_inner_matches():
    kb = _kb("""
    (rules
      (rule guarded (?r)
        :match (and (?r ?a ?b) (not (other ?a ?b)))
        :assert (ok ?a ?b)
        :why "g"))
    (ontology
      (relation r T T) (relation other T T)
      (guarded r))
    (facts (r X Y :source "(1)") (other X Y :source "(2)"))
    """)
    plan = compile_rule(
        kb.rules["guarded"], kb._facts_by_relation["guarded"][0],
    )
    results = list(match.run(plan, kb))
    assert results == []


def test_nested_fact_pattern_unifies_against_relational_arg():
    """Q40 — match `(hypothesis (co-located ?a ?b))` against a
    synthetic fact whose first arg is a nested co-located Fact."""
    kb = _kb("""
    (rules
      (rule trap ()
        :match (hypothesis (co-located ?a ?b))
        :assert (caught ?a ?b)
        :why "h"))
    (ontology (relation co-located T T) (relation hypothesis T))
    """)
    # Synthesise the nested fact manually (P1.5 will do this).
    inner = Fact(
        relation_name="co-located",
        args=("Norwegian", "House_2"),
        layer=Layer.REASONING,
        provenance=Provenance.from_source(source=None),
    )
    outer = Fact(
        relation_name="hypothesis",
        args=(inner,),
        layer=Layer.REASONING,
        provenance=Provenance.from_source(source=None),
    )
    kb.add_fact(inner)
    kb.add_fact(outer)
    kb.rebuild_indexes()

    plan = compile_rule(kb.rules["trap"], None)
    results = list(match.run(plan, kb))
    assert len(results) == 1
    bindings, _ = results[0]
    assert bindings["a"] == "Norwegian"
    assert bindings["b"] == "House_2"


def test_no_match_when_relation_absent():
    kb = _kb("""
    (rules
      (rule sym (?rel)
        :match (?rel ?a ?b)
        :assert (?rel ?b ?a)
        :why "s"))
    (ontology (relation r T T) (sym r))
    """)
    plan = compile_rule(
        kb.rules["sym"], kb._facts_by_relation["sym"][0],
    )
    # No (r …) facts → no matches.
    assert list(match.run(plan, kb)) == []
