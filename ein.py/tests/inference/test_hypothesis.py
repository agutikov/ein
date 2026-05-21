"""Hypothesis loop tests — S1.5.1 / P1.5.

Fixtures use the canonical (zebra2-style) encoding: `is-a` is a normal
declared relation; no kernel `(type)` or `(instance)` forms. The
hypothesis generator picks instance-like objects from `kb.names`
(graph-derived), so this test surface mirrors the production target
(see [[project-canonical-zebra2]]).
"""
from __future__ import annotations

from ein_bot.inference.hypothesis import (
    Contradiction,
    Mode,
    Solution,
    generate_hypotheses,
    is_solved,
    solve,
    try_branch,
)
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# Canonical rule library — symmetric / transitive / sibling-exclusive.
# Each fixture pulls in only what it exercises, but for test stability
# we always include the three core rules; activators select which
# relations they apply to.
_RULES = """
(rules
  (rule symmetric (?rel)
    :match  (?rel ?a ?b)
    :assert (?rel ?b ?a)
    :why    "s"
    :priority 100)
  (rule transitive (?rel)
    :match  (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
    :assert (?rel ?a ?c)
    :why    "t"
    :priority 200)
  (rule sibling-exclusive (?out)
    :match  (and (is-a ?a ?T) (is-a ?b ?T) (neq ?a ?b))
    :assert (not (?out ?a ?b))
    :why    "sib"
    :priority 300)
  (rule single-parent (?rel)
    :match  (and (?rel ?a ?b) (?rel ?a ?c) (neq ?b ?c))
    :assert (not (?rel ?a ?c))
    :why    "1p"
    :priority 250))
"""


# ── Object selection ──────────────────────────────────────────────


def test_object_selection_picks_max_fact_count():
    """Generator yields hypotheses for the most-constrained leaf
    first. With one leaf involved in 3 facts and others in 0 (apart
    from their is-a edges), the first hypothesis fact mentions the
    busy one."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r1 T T) (relation r2 T T) (relation r3 T T)
      (is-a Anchor T) (is-a Other T) (is-a Far T))
    (facts
      (r1 Anchor Other :source "(1)")
      (r2 Anchor Other :source "(2)")
      (r3 Anchor Other :source "(3)"))
    """)
    hyps = list(generate_hypotheses(kb))
    assert hyps, "expected at least one hypothesis"
    assert any(a == "Anchor" for a in hyps[0].args), (
        f"first hypothesis args were {hyps[0].args!r}; "
        "expected Anchor to appear"
    )


# ── Candidate enumeration ─────────────────────────────────────────


def test_excludes_negated_candidates():
    """If (not (r A B)) is already in the KB, the gen skips
    `(r A B)` as a hypothesis candidate."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
    (facts
      (not (r A B) :source "(1)"))
    """)
    hyps = list(generate_hypotheses(kb))
    assert not any(
        h.relation_name == "r" and h.args == ("A", "B")
        for h in hyps
    ), "expected (r A B) to be excluded by the (not …) fact"


def test_skips_already_existing_facts():
    """Slots the object already occupies are skipped — `existing(obj)`
    in the two-step framework."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a Alice T) (is-a Bob T))
    (facts
      (r Alice Bob :source "(1)"))
    """)
    hyps = list(generate_hypotheses(kb))
    # (r Alice Bob) is already in the KB — gen should not re-emit it.
    assert not any(
        h.relation_name == "r" and h.args == ("Alice", "Bob")
        for h in hyps
    )


# ── Symmetric R: emit both orderings ──────────────────────────────


def test_symmetric_r_emits_both_orderings():
    """(symmetric R) activator → generator yields both (R A B) and
    (R B A) as separate hypotheses."""
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation friend T T)
      (symmetric friend)
      (is-a Alice T) (is-a Bob T))
    """)
    hyps = list(generate_hypotheses(kb))
    pairs = {(h.relation_name, h.args) for h in hyps}
    assert ("friend", ("Alice", "Bob")) in pairs
    assert ("friend", ("Bob", "Alice")) in pairs


def test_asymmetric_r_emits_one_ordering_per_object():
    """No (symmetric R) activator → the generator still emits both
    orderings because each object enumerates its own slot, but the
    dedup prevents duplicates within a single call."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a Alice T) (is-a Bob T))
    """)
    hyps = list(generate_hypotheses(kb))
    pairs = {(h.relation_name, h.args) for h in hyps}
    # Both orderings appear (because slot enumeration generates them
    # from different objects); dedup just prevents YIELDING the same
    # exact (rel, args) twice in one call.
    assert ("r", ("Alice", "Bob")) in pairs
    assert ("r", ("Bob", "Alice")) in pairs
    # The pair count should equal the unique (rel, args) count.
    assert len(hyps) == len(pairs)


# ── try_branch — single-branch test cycle ─────────────────────────


def test_try_branch_alive_no_contradiction():
    """A hypothesis that doesn't conflict with any KB fact returns
    BranchResult.alive."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a Alice T) (is-a Bob T))
    """)
    hyp = Fact(
        relation_name="r",
        args=("Alice", "Bob"),
        layer=Layer.REASONING,
    )
    result = try_branch(kb, hyp, branch_id=1)
    assert result.is_alive()
    assert result.hypothesis.relation_name == "r"
    assert result.hypothesis.args == ("Alice", "Bob")


def test_try_branch_dead_via_sibling_exclusive():
    """A hypothesis (co-located Alice Bob) where Alice and Bob share a
    parent under is-a triggers sibling-exclusive → contradiction →
    branch dies. Q40 synthetic facts and the (not h) derivation are
    in the fork's REASONING layer."""
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation co-located T T)
      (sibling-exclusive co-located)
      (is-a Alice T) (is-a Bob T))
    """)
    hyp = Fact(
        relation_name="co-located",
        args=("Alice", "Bob"),
        layer=Layer.REASONING,
    )
    result = try_branch(kb, hyp, branch_id=42)
    assert not result.is_alive(), "expected branch to die"
    assert result.unsat_core, "expected non-empty unsat-core"


def test_try_branch_q40_protocol_emits_synthetic_facts():
    """Verify Q40 step 2 emits (hypothesis <h>) in the fork."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
    """)
    hyp = Fact(
        relation_name="r",
        args=("A", "B"),
        layer=Layer.REASONING,
    )
    result = try_branch(kb, hyp, branch_id=7)
    synthetic = [
        f for f in result.kb.facts
        if f.relation_name == "hypothesis"
    ]
    assert len(synthetic) == 1
    inner = synthetic[0].args[0]
    assert isinstance(inner, Fact)
    assert inner.relation_name == "r"
    assert inner.args == ("A", "B")


# ── is_solved per mode ────────────────────────────────────────────


def test_is_solved_solve_mode_exact_one_match():
    """SOLVE mode: exactly one binding for the goal pattern."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
    (facts (r A B :source "(1)"))
    (query :mode solve :goal (r A B))
    """)
    assert is_solved(kb, Mode.SOLVE)


def test_is_solved_solve_mode_zero_matches():
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
    (query :mode solve :goal (r A B))
    """)
    assert not is_solved(kb, Mode.SOLVE)


def test_is_solved_gaps_mode_at_least_one():
    """GAPS: ≥ 1 match passes."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
    (facts (r A B :source "(1)"))
    (query :mode gaps :goal (r ?a ?b))
    """)
    assert is_solved(kb, Mode.GAPS)


def test_is_solved_contradictions_mode_always_false():
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T))
    (facts (r A A :source "(1)"))
    (query :mode contradictions :goal (r A A))
    """)
    assert not is_solved(kb, Mode.CONTRADICTIONS)


# ── solve() top-level driver ──────────────────────────────────────


def test_solve_trivial_already_solved():
    """The goal matches at root, but the search still explores all
    alive hypotheses (S1.5.0 §F — complete exploration tree is the
    proof). With `(single-parent is-a)` the alternate-parent
    hypotheses die, the search terminates, and the verdict promotes
    to Solution."""
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation r T T)
      (symmetric r)
      (single-parent is-a)
      (is-a A T) (is-a B T))
    (facts (r A B :source "(1)"))
    (query :mode solve :goal (r B A))
    """)
    result = solve(kb)
    assert isinstance(result, Solution)


def test_solve_picks_surviving_hypothesis():
    """The goal `(co-located Red H1)` matches at root. With
    `(single-parent is-a)` the alternate-parent is-a hypotheses
    die immediately; with `(sibling-exclusive co-located)` the
    cross-type co-located hypotheses die; the verdict promotes
    to Solution after exhaustive exploration confirms uniqueness."""
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation co-located T T)
      (sibling-exclusive co-located)
      (single-parent     is-a)
      (is-a Color T) (is-a House T)
      (is-a Red Color) (is-a Blue Color)
      (is-a H1 House))
    (facts
      (co-located Red H1 :source "(1)"))
    (query :mode solve :goal (co-located Red H1))
    """)
    result = solve(kb)
    assert isinstance(result, Solution)


def test_solve_returns_contradiction_on_baked_in_conflict():
    """Inject both a positive and its negative directly; solve()
    returns Contradiction."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
    (reasoning
      (r A B)
      (not (r A B)))
    (query :mode solve :goal (r A B))
    """)
    result = solve(kb)
    assert isinstance(result, Contradiction)


def test_solve_mode_defaults_from_query():
    """If no `mode=` kwarg is passed, solve() reads :mode from the query."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T))
    (facts (r A A :source "(1)"))
    (query :mode solve :goal (r A A))
    """)
    result = solve(kb)
    assert isinstance(result, Solution)
