"""Runtime matcher tests — S1.3.1 T1.3.1.9."""
from __future__ import annotations

from ein.inference import match
from ein.inference.compile import compile_rule
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.provenance import Provenance
from ein.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def test_single_scan_binds_binary_fact():
    kb = _kb("""
    (rule mirror (?r)
      :match (?r ?a ?b)
      :assert (?r ?b ?a)
      :why "m")
    (relation co-located T T)
    (mirror co-located)
    (co-located Norwegian House-1 :source "(1)")
    """)
    plan = compile_rule(kb.rules["mirror"], kb._facts_by_relation["mirror"][0])
    results = list(match.run(plan, kb))
    assert len(results) == 1
    bindings, premises = results[0]
    assert bindings["r"] == "co-located"
    assert bindings["a"] == "Norwegian"
    assert bindings["b"] == "House-1"
    assert premises[0].relation_name == "co-located"


def test_join_with_shared_var_chains_facts():
    kb = _kb("""
    (rule transitive (?rel)
      :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
      :assert (?rel ?a ?c)
      :why "t")
    (relation r T T)
    (transitive r)
    (r A B :source "(1)")
    (r B C :source "(2)")
    (r C D :source "(3)")
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
    (rule self (?rel)
      :match (and (?rel ?a ?a))
      :assert (?rel ?a ?a)
      :why "s")
    (relation r T T) (self r)
    (r X X :source "(1)") (r Y Z :source "(2)")
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
    (rule transitive (?rel)
      :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
      :assert (?rel ?a ?c)
      :why "t")
    (relation r T T) (transitive r)
    (r A B :source "(1)") (r B A :source "(2)")
    """)
    plan = compile_rule(
        kb.rules["transitive"],
        kb._facts_by_relation["transitive"][0],
    )
    results = list(match.run(plan, kb))
    # A-B-A and B-A-B are pruned by neq; no other chains exist.
    assert results == []


def test_absent_succeeds_when_inner_empty():
    """`(absent P)` is the explicit NAF (S1.5.8c K-Δ.2):
    the premise passes when no fact matches P."""
    kb = _kb("""
    (rule guarded (?r)
      :match (and (?r ?a ?b) (absent (other ?a ?b)))
      :assert (ok ?a ?b)
      :why "g")
    (relation r T T) (relation other T T)
    (guarded r)
    (r X Y :source "(1)")
    """)
    plan = compile_rule(
        kb.rules["guarded"], kb._facts_by_relation["guarded"][0],
    )
    results = list(match.run(plan, kb))
    assert len(results) == 1
    assert results[0][0]["a"] == "X" and results[0][0]["b"] == "Y"


def test_absent_fails_when_inner_matches():
    """`(absent P)` fails when a fact matching P is in the KB."""
    kb = _kb("""
    (rule guarded (?r)
      :match (and (?r ?a ?b) (absent (other ?a ?b)))
      :assert (ok ?a ?b)
      :why "g")
    (relation r T T) (relation other T T)
    (guarded r)
    (r X Y :source "(1)") (other X Y :source "(2)")
    """)
    plan = compile_rule(
        kb.rules["guarded"], kb._facts_by_relation["guarded"][0],
    )
    results = list(match.run(plan, kb))
    assert results == []


def test_not_premise_matches_stored_neg_fact():
    """`(not P)` in :match (post S1.5.8c K-Δ.1) matches a STORED
    ``(not P)`` fact — uniform with how any other fact pattern
    matches its head's storage."""
    kb = _kb("""
    (rule see-neg (?r)
      :match (and (?r ?a ?b) (not (other ?a ?b)))
      :assert (saw-neg ?a ?b)
      :why "stored neg seen")
    (relation r T T) (relation other T T)
    (see-neg r)
    (r X Y :source "(1)") (not (other X Y) :source "(2)")
    """)
    plan = compile_rule(
        kb.rules["see-neg"], kb._facts_by_relation["see-neg"][0],
    )
    results = list(match.run(plan, kb))
    assert len(results) == 1
    assert results[0][0]["a"] == "X" and results[0][0]["b"] == "Y"


def test_not_premise_does_not_match_without_stored_neg():
    """`(not P)` no longer means NAF (S1.5.8c K-Δ.1): with only
    the positive `(other X Y)` in the KB and no stored
    `(not (other X Y))`, the (not …) pattern matches nothing."""
    kb = _kb("""
    (rule see-neg (?r)
      :match (and (?r ?a ?b) (not (other ?a ?b)))
      :assert (saw-neg ?a ?b)
      :why "stored neg seen")
    (relation r T T) (relation other T T)
    (see-neg r)
    (r X Y :source "(1)") (other X Y :source "(2)")
    """)
    plan = compile_rule(
        kb.rules["see-neg"], kb._facts_by_relation["see-neg"][0],
    )
    results = list(match.run(plan, kb))
    # No stored (not (other X Y)) fact → no match.
    assert results == []


def test_nested_fact_pattern_unifies_against_relational_arg():
    """Q40 — match `(hypothesis (co-located ?a ?b))` against a
    synthetic fact whose first arg is a nested co-located Fact."""
    kb = _kb("""
    (rule trap ()
      :match (hypothesis (co-located ?a ?b))
      :assert (caught ?a ?b)
      :why "h")
    (relation co-located T T) (relation hypothesis T)
    """)
    # Synthesise the nested fact manually (P1.5 will do this).
    inner = Fact(
        relation_name="co-located",
        args=("Norwegian", "House-2"),
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
    assert bindings["b"] == "House-2"


def test_no_match_when_relation_absent():
    kb = _kb("""
    (rule sym (?rel)
      :match (?rel ?a ?b)
      :assert (?rel ?b ?a)
      :why "s")
    (relation r T T) (sym r)
    """)
    plan = compile_rule(
        kb.rules["sym"], kb._facts_by_relation["sym"][0],
    )
    # No (r …) facts → no matches.
    assert list(match.run(plan, kb)) == []
