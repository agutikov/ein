"""Multi-justification provenance + minimal explanation search — S1.21.7.

Covers the two halves of the stage: what the engine *records* (alternative
justifications at the redundant-firing seam, with the fork/snapshot and
ground-out contracts), and what the search *does* with them (a real AND/OR
minimum, not a per-fact greedy pick, terminating on a cyclic graph).
"""
from __future__ import annotations

import pytest

from ein.inference.config import SolverConfig
from ein.inference.contradiction import ContradictionDetector
from ein.inference.explain import (
    ExplanationBudget,
    explain,
    minimal_contradiction_frontier,
)
from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.provenance import Provenance
from ein.kb.store import MAX_ALT_JUSTIFICATIONS, KnowledgeBase

# Two facts, each derivable two ways, clashing. The per-fact best picks are
# (X ← C) at size 1 and (Z ← A,B or D,E) at size 2 — union 3 — but sharing
# {A, B} across BOTH derivations costs only 2. A greedy "smallest derivation
# per fact" search cannot find it; an AND/OR label search can.
SHARED_OPTIMUM = """
(relation A T) (relation B T) (relation C T) (relation D T) (relation E T)
(relation X T) (relation Z T)
(rule x-join () :match (and (A ?o) (B ?o)) :assert (X ?o) :why "xj" :priority 100)
(rule x-chain () :match (C ?o) :assert (X ?o) :why "xc" :priority 110)
(rule z-join () :match (and (A ?o) (B ?o)) :assert (Z ?o) :why "zj" :priority 120)
(rule z-pair () :match (and (D ?o) (E ?o)) :assert (Z ?o) :why "zp" :priority 130)
(rule clash () :match (and (X ?o) (Z ?o)) :assert (false) :why "clash" :priority 300)
(A a :source "(A)") (B a :source "(B)") (C a :source "(C)")
(D a :source "(D)") (E a :source "(E)")
"""

# `mirror` derives its own premise's swap, so once (P x y) is itself DERIVED
# (from the given (S x y)) the two P-facts justify each other: (P x y) ← seed,
# and also ← mirror(P y x); (P y x) ← mirror(P x y). Once re-derivations are
# recorded that is a genuine 2-cycle in the justification graph — the case a
# naive "walk premises until you hit a leaf" search cannot survive. (A `:source`
# fact would NOT cycle: givens are frontier terminals and take no alternatives.)
CYCLIC = """
(relation S T T) (relation P T T) (relation Q T)
(rule seed () :match (S ?a ?b) :assert (P ?a ?b) :why "s" :priority 90)
(rule mirror () :match (P ?a ?b) :assert (P ?b ?a) :why "m" :priority 100)
(rule clash () :match (and (P y x) (Q x)) :assert (false) :why "c" :priority 300)
(S x y :source "(S)") (Q x :source "(Q)")
"""

CONSISTENT = """
(relation R T T)
(R x One :source "(1)")
"""


def _saturated(text: str, **cfg) -> KnowledgeBase:
    kb = KnowledgeBase.from_ir(parse(text))
    if cfg:
        kb.config = SolverConfig(**cfg)
    list(Saturator(kb).saturate())
    return kb


def _names(facts) -> set[str]:
    return {f.relation_name for f in facts}


class TestRecording:
    def test_redundant_firing_is_recorded_not_dropped(self):
        # The seam is `Saturator._apply`'s redundant branch, NOT
        # `store.add_and_index_fact` — a wholly-redundant firing returns
        # before `fire()` ever builds a Provenance.
        kb = _saturated(SHARED_OPTIMUM)
        alts = kb._alt_justifications
        assert {p.rule for p in alts[("X", ("a",))]} == {"x-chain"}
        assert {p.rule for p in alts[("Z", ("a",))]} == {"z-pair"}
        # Primary + alternative together are the fact's OR-node.
        x = kb._fact_by_id("X", ("a",))
        assert {p.rule for p in kb.justifications(x)} == {"x-join", "x-chain"}

    def test_given_facts_take_no_alternatives(self):
        # A `:source` clue is the frontier — what the engine treats as given.
        # Re-deriving one must not turn it into an interior node, or the
        # explanation would stop naming the premise the user supplied.
        kb = _saturated("""
        (relation G T) (relation H T)
        (rule re-derive () :match (H ?o) :assert (G ?o) :why "r" :priority 100)
        (G a :source "(G)") (H a :source "(H)")
        """)
        g = kb._fact_by_id("G", ("a",))
        assert g.provenance.kind == "source"
        assert ("G", ("a",)) not in kb._alt_justifications
        assert len(kb.justifications(g)) == 1

    def test_synthetic_writeback_keeps_its_ground_out_contract(self):
        # `<forced-positive>` & co. are rule-kind with NO premises: their
        # stated contract is that provenance walks ground out on them. An
        # alternative would give the fact a real derivation and silently
        # change every core that passes through it.
        kb = KnowledgeBase.from_ir(parse("(relation W T)"))
        w = Fact(
            relation_name="W", args=("a",), layer=Layer.REASONING,
            provenance=Provenance.from_rule(rule="<forced-positive>"),
        )
        kb.add_and_index_fact(w)
        assert kb.accepts_justification(w, 1) is False
        assert kb.record_justification(w, Provenance.from_rule(
            rule="r", premises_raw=(("A", ("a",)),))) is False
        assert kb._alt_justifications == {}

    def test_alternatives_are_capped_keeping_the_shortest(self):
        kb = KnowledgeBase.from_ir(parse("(relation T2 T)"))
        target = Fact(
            relation_name="T2", args=("a",), layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="primary", premises_raw=(("P", (0,)),)),
        )
        kb.add_and_index_fact(target)
        # Fill past the cap with long justifications, then offer a short one.
        for i in range(MAX_ALT_JUSTIFICATIONS + 10):
            kb.record_justification(target, Provenance.from_rule(
                rule=f"long{i}",
                premises_raw=tuple(("L", (i, j)) for j in range(5))))
        alts = kb._alt_justifications[("T2", ("a",))]
        assert len(alts) == MAX_ALT_JUSTIFICATIONS
        assert kb.accepts_justification(target, 5) is False   # O(1) reject
        assert kb.record_justification(target, Provenance.from_rule(
            rule="short", premises_raw=(("S", ("a",)),))) is True
        alts = kb._alt_justifications[("T2", ("a",))]
        assert len(alts) == MAX_ALT_JUSTIFICATIONS
        assert alts[0].rule == "short"                        # sorted shortest-first
        # Identity is (rule, premises_raw) — bindings are display metadata.
        assert kb.record_justification(target, Provenance.from_rule(
            rule="short", premises_raw=(("S", ("a",)),),
            bindings=(("o", "a"),))) is False

    def test_fork_does_not_leak_justifications_into_root(self):
        # A justification recorded inside a hypothesis fork can name premises
        # root never assumed; sharing the table (as `_nogoods` is shared)
        # would surface a phantom assumption in a root-level unsat core.
        kb = _saturated(SHARED_OPTIMUM)
        fork = kb.fork()
        target = fork._fact_by_id("X", ("a",))
        assert fork.record_justification(target, Provenance.from_rule(
            rule="fork-only", premises_raw=(("A", ("a",)),))) is True
        assert "fork-only" in {p.rule for p in fork.justifications(target)}
        assert "fork-only" not in {p.rule for p in kb.justifications(target)}

    def test_snapshot_isolates_and_rebuild_preserves(self):
        kb = _saturated(SHARED_OPTIMUM)
        snap = kb.snapshot()
        assert snap._alt_justifications == kb._alt_justifications
        snap._alt_justifications.clear()
        assert kb._alt_justifications                    # not aliased
        # The table is history, not a projection of `facts` — a rebuild
        # cannot reconstruct it, so it must survive one.
        before = dict(kb._alt_justifications)
        kb.rebuild_indexes()
        assert kb._alt_justifications == before

    def test_config_gate_turns_recording_off(self):
        kb = _saturated(SHARED_OPTIMUM, record_alternative_justifications=False)
        assert kb._alt_justifications == {}
        assert kb.has_alternative_justifications() is False


class TestSearch:
    def test_finds_a_shared_optimum_a_greedy_search_would_miss(self):
        kb = _saturated(SHARED_OPTIMUM)
        result = minimal_contradiction_frontier(kb)
        assert result.exhausted is True
        assert _names(result.frontier) == {"A", "B"}      # 2, not the greedy 3
        # Sanity: the greedy per-fact picks really are worse.
        assert len(result.frontier) == 2

    def test_terminates_and_is_sound_on_a_cyclic_justification_graph(self):
        kb = _saturated(CYCLIC)
        # The cycle is real: (P x y) and (P y x) justify each other.
        assert {p.rule for p in kb._alt_justifications[("P", ("x", "y"))]} \
            == {"mirror"}
        result = minimal_contradiction_frontier(kb)
        assert result.exhausted is True
        # Grounds out in the givens, never in itself.
        assert _names(result.frontier) == {"S", "Q"}
        assert all(f.provenance.kind == "source" for f in result.frontier)

    def test_empty_on_a_consistent_kb(self):
        result = minimal_contradiction_frontier(_saturated(CONSISTENT))
        assert result.frontier == frozenset()
        assert result.target is None

    def test_explaining_an_underivable_target_is_empty_not_wrong(self):
        kb = _saturated(CONSISTENT)
        orphan = Fact(relation_name="R", args=("nope", "nope"))
        assert explain(kb, [orphan]).frontier == frozenset()

    def test_a_given_explains_itself(self):
        kb = _saturated(CONSISTENT)
        given = kb._fact_by_id("R", ("x", "One"))
        assert explain(kb, [given]).frontier == frozenset({given})

    @pytest.mark.parametrize("budget", [
        ExplanationBudget(max_rounds=1),
        ExplanationBudget(max_environments=1),
        ExplanationBudget(max_facts=2),
    ])
    def test_every_budget_cap_stays_sound_and_reports_itself(self, budget):
        kb = _saturated(SHARED_OPTIMUM)
        witnesses = [c.witness for c in ContradictionDetector(kb).detect()]
        envelope = kb.unsat_core(witnesses, all_justifications=True)
        result = minimal_contradiction_frontier(kb, budget=budget)
        # Sound under every cap: only ever a subset of the OR-aware closure.
        assert result.frontier <= envelope
        # And it never silently claims a truncated answer is the minimum.
        assert result.exhausted is False or len(result.frontier) == 2

    def test_max_env_size_asks_a_decision_question(self):
        kb = _saturated(SHARED_OPTIMUM)
        assert len(minimal_contradiction_frontier(
            kb, budget=ExplanationBudget(max_env_size=2)).frontier) == 2
        # No explanation of size <= 1 exists, so the search finds none.
        assert minimal_contradiction_frontier(
            kb, budget=ExplanationBudget(max_env_size=1)).frontier == frozenset()

    def test_result_is_stable_across_repeated_runs(self):
        # Deterministic tie-breaks: no set-iteration or dict order leaks in.
        kb = _saturated(SHARED_OPTIMUM)
        seen = {frozenset(minimal_contradiction_frontier(kb).frontier)
                for _ in range(5)}
        assert len(seen) == 1


class TestAndOrDag:
    def test_derivation_dag_defaults_to_one_derivation_per_fact(self):
        kb = _saturated(SHARED_OPTIMUM)
        dag = kb.derivation_dag(kb._fact_by_id("X", ("a",)))
        assert dag.is_or_graph is False
        assert [n.relation_name for n in dag.and_nodes[0][1]] == ["A", "B"]
        assert "diamond" not in dag.to_dot()

    def test_or_dag_keeps_the_and_grouping_a_flat_edge_set_loses(self):
        kb = _saturated(SHARED_OPTIMUM)
        dag = kb.derivation_dag(
            kb._fact_by_id("X", ("a",)), all_justifications=True)
        assert dag.is_or_graph is True
        groups = {tuple(sorted(p.relation_name for p in prem))
                  for _c, prem in dag.and_nodes}
        assert groups == {("A", "B"), ("C",)}
        # The flat edge set alone cannot say which in-edges form one proof.
        assert len({p.relation_name for p, _c in dag.edges}) == 3
        assert "diamond" in dag.to_dot()
