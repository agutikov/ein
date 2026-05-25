"""Tests for per-fact provenance + derivation DAG — S1.2.3."""
from __future__ import annotations

from ein_bot.ir import parse
from ein_bot.kb import (
    DerivationDAG,
    Fact,
    KnowledgeBase,
    Layer,
    Provenance,
)
from ein_bot.kb.provenance import detect_provenance_cycles

# ── Helpers ────────────────────────────────────────────────────────


def _tiny_kb() -> KnowledgeBase:
    """Three-node ontology suitable for triangle-composition tests."""
    text = """
    (ontology
      (type T)
      (instance A T) (instance B T) (instance C T) (instance D T)
      (relation r T T))
    (facts
      (r A B :source "(1)")
      (r B C :source "(2)")
      (r C D :source "(3)"))
    """
    return KnowledgeBase.from_ir(parse(text))


def _add_derived(
    kb: KnowledgeBase,
    relation_name: str,
    args: tuple,
    rule: str,
    premises: tuple,
) -> Fact:
    """Add a REASONING-layer fact with rule-kind provenance."""
    f = Fact(
        relation_name=relation_name, args=args,
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule=rule, premises_raw=premises,
        ),
    )
    kb.add_fact(f)
    kb._index_fact(kb.facts[-1])
    return kb.facts[-1]


# ═══════════════════════ Provenance dataclass ══════════════════════


class TestProvenanceConstruction:
    def test_from_source(self):
        p = Provenance.from_source(source="(10)")
        assert p.kind == "source"
        assert p.source == "(10)"
        assert p.rule is None
        assert p.premises_raw == ()

    def test_from_rule(self):
        p = Provenance.from_rule(
            rule="transitive",
            premises_raw=(("r", ("a", "b")), ("r", ("b", "c"))),
            bindings=(("x", "a"), ("y", "c")),
        )
        assert p.kind == "rule"
        assert p.rule == "transitive"
        assert len(p.premises_raw) == 2
        assert p.bindings == (("x", "a"), ("y", "c"))

    def test_from_hypothesis(self):
        p = Provenance.from_hypothesis(branch=3)
        assert p.kind == "hypothesis"
        assert p.branch == 3
        assert p.rule is None

    def test_rejected(self):
        p = Provenance.rejected(branch=5)
        assert p.kind == "rejected"
        assert p.branch == 5

    def test_provenance_equality(self):
        a = Provenance.from_source(source="(2)")
        b = Provenance.from_source(source="(2)")
        assert a == b
        assert hash(a) == hash(b)

    def test_provenance_loc_excluded_from_eq(self):
        from ein_bot.ir.types import Loc
        a = Provenance.from_source(source="(2)", loc=Loc("f", 1, 1))
        b = Provenance.from_source(source="(2)", loc=Loc("g", 99, 99))
        assert a == b


# ═══════════════════════ Fact backward-compat properties ═══════════


class TestFactProperties:
    def test_source_property(self, zebra_kb):
        f = next(
            x for x in zebra_kb.facts
            if x.relation_name == "co-located" and x.args == ("Norwegian", "House-1")
        )
        assert f.source == "condition (10)"
        assert f.rule_name is None
        assert f.using == ()

    def test_rule_name_property_for_rule_kind(self):
        kb = _tiny_kb()
        derived = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        assert derived.source is None
        assert derived.rule_name == "triangle"
        assert derived.using == (("r", ("A", "B")), ("r", ("B", "C")))

    def test_premises_resolved(self):
        kb = _tiny_kb()
        derived = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        premises = derived.premises
        assert len(premises) == 2
        assert all(isinstance(p, Fact) for p in premises)
        assert premises[0].args == ("A", "B")
        assert premises[1].args == ("B", "C")

    def test_premises_empty_for_source_kind(self, zebra_kb):
        f = next(
            x for x in zebra_kb.facts
            if x.source == "condition (10)"
        )
        assert f.premises == ()


# ═══════════════════════ derivation_dag ════════════════════════════


class TestDerivationDAG:
    def test_single_hop_dag(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        dag = kb.derivation_dag(ac)
        assert isinstance(dag, DerivationDAG)
        assert dag.root == ac
        assert len(dag.nodes) == 3
        assert len(dag.edges) == 2
        # All edges point at ac as the conclusion.
        for _premise, conclusion in dag.edges:
            assert conclusion == ac

    def test_multi_hop_dag(self):
        # A→B, B→C, C→D in source layer; derive A→C (triangle from A-B,
        # B-C) then A→D (triangle from A-C, C-D). derivation_dag(A→D)
        # walks back to four source facts via A→C.
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        ad = _add_derived(
            kb, "r", ("A", "D"), "triangle",
            premises=(("r", ("A", "C")), ("r", ("C", "D"))),
        )
        dag = kb.derivation_dag(ad)
        # Nodes: ad (rule), ac (rule), three sources (A-B, B-C, C-D).
        # That's 5 in total.
        assert len(dag.nodes) == 5
        # Sources frontier: A-B, B-C, C-D.
        src_args = {s.args for s in dag.sources}
        assert src_args == {("A", "B"), ("B", "C"), ("C", "D")}
        assert ac in dag.nodes
        # ad has rule-kind provenance and is NOT a source.
        assert ad not in dag.sources

    def test_dag_for_source_fact_is_singleton(self, zebra_kb):
        f = next(
            x for x in zebra_kb.facts
            if x.source == "condition (10)"
        )
        dag = zebra_kb.derivation_dag(f)
        assert len(dag.nodes) == 1
        assert dag.edges == ()
        assert dag.sources == (f,)

    def test_dag_root_field(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        dag = kb.derivation_dag(ac)
        assert dag.root == ac

    def test_dag_breaks_cycles(self):
        # Construct an artificial cyclic provenance (bypassing the
        # load-time validator) and verify derivation_dag terminates.
        kb = _tiny_kb()
        # r(A,B) cites r(B,A); r(B,A) cites r(A,B).
        ab_loop = Fact(
            relation_name="r", args=("A", "B_alt"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="self", premises_raw=(("r", ("A", "B_alt")),),
            ),
        )
        kb.add_fact(ab_loop)
        kb._index_fact(kb.facts[-1])
        dag = kb.derivation_dag(kb.facts[-1])
        # Cycle was broken; result is finite.
        assert len(dag.nodes) <= 2

    def test_dag_iter_and_len(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        dag = kb.derivation_dag(ac)
        assert len(dag) == 3
        assert list(dag) == list(dag.nodes)


# ═══════════════════════ DerivationDAG.to_dot ═════════════════════


class TestDerivationDAGtoDot:
    def test_to_dot_returns_digraph(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        dot = kb.derivation_dag(ac).to_dot()
        assert dot.startswith("digraph derivation {")
        assert dot.endswith("}")

    def test_to_dot_marks_sources_as_ellipse(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        dot = kb.derivation_dag(ac).to_dot()
        assert "shape=ellipse" in dot

    def test_to_dot_marks_rule_derivation_as_box(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        dot = kb.derivation_dag(ac).to_dot()
        assert "shape=box" in dot
        # Rule label embedded.
        assert "triangle" in dot

    def test_to_dot_includes_compact_form(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        dot = kb.derivation_dag(ac).to_dot()
        assert "(r A C)" in dot
        assert "(r A B)" in dot


# ═══════════════════════ unsat_core ════════════════════════════════


class TestUnsatCore:
    def test_unsat_core_single_conflict(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        core = kb.unsat_core([ac])
        core_keys = {(f.relation_name, f.args) for f in core}
        assert core_keys == {("r", ("A", "B")), ("r", ("B", "C"))}

    def test_unsat_core_multiple_conflicts(self):
        kb = _tiny_kb()
        ac = _add_derived(
            kb, "r", ("A", "C"), "triangle",
            premises=(("r", ("A", "B")), ("r", ("B", "C"))),
        )
        bd = _add_derived(
            kb, "r", ("B", "D"), "triangle",
            premises=(("r", ("B", "C")), ("r", ("C", "D"))),
        )
        core = kb.unsat_core([ac, bd])
        core_keys = {(f.relation_name, f.args) for f in core}
        # Union of both derivation chains: A-B, B-C, C-D.
        assert core_keys == {
            ("r", ("A", "B")), ("r", ("B", "C")), ("r", ("C", "D")),
        }

    def test_unsat_core_with_source_fact_is_itself(self, zebra_kb):
        f = next(
            x for x in zebra_kb.facts
            if x.source == "condition (10)"
        )
        core = zebra_kb.unsat_core([f])
        assert core == {f}


# ═══════════════════════ Cycle detection ═══════════════════════════


class TestCycleDetection:
    def test_detect_two_node_cycle(self):
        kb = _tiny_kb()
        # Forge a two-node cycle directly.
        f_ab = Fact(
            relation_name="r", args=("A_c", "B_c"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="R1", premises_raw=(("r", ("B_c", "A_c")),),
            ),
        )
        f_ba = Fact(
            relation_name="r", args=("B_c", "A_c"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="R2", premises_raw=(("r", ("A_c", "B_c")),),
            ),
        )
        kb.add_fact(f_ab)
        kb.add_fact(f_ba)
        kb._index_fact(kb.facts[-2])
        kb._index_fact(kb.facts[-1])
        cycles = detect_provenance_cycles(kb.facts, kb._fact_by_id)
        assert len(cycles) >= 1
        # The cycle includes both nodes.
        flat = {step for cyc in cycles for step in cyc}
        assert ("r", ("A_c", "B_c")) in flat or ("r", ("B_c", "A_c")) in flat

    def test_no_cycle_on_acyclic_kb(self, zebra_kb):
        # The Zebra KB has no rule-kind facts yet, so no cycles.
        cycles = detect_provenance_cycles(
            zebra_kb.facts, zebra_kb._fact_by_id,
        )
        assert cycles == []


# ═══════════════════════ Loader cycle rejection ════════════════════


class TestLoaderRejectsCycles:
    """The load-time validator catches user-authored cycles.

    The IR grammar doesn't currently round-trip `:using` (S1.2.3
    T1.2.3.4 deferred to a P1.1 follow-up); for now the cycle test
    forges Facts directly and verifies the validator sees them.
    """

    def test_cycle_via_direct_construction_then_revalidation(self):
        # Build a KB that's cycle-free at load, then manually add a
        # cycle, then call the same validator the loader uses.
        kb = _tiny_kb()
        f_ab = Fact(
            relation_name="r", args=("A2", "B2"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="R1", premises_raw=(("r", ("B2", "A2")),),
            ),
        )
        f_ba = Fact(
            relation_name="r", args=("B2", "A2"),
            layer=Layer.REASONING,
            provenance=Provenance.from_rule(
                rule="R2", premises_raw=(("r", ("A2", "B2")),),
            ),
        )
        kb.add_fact(f_ab)
        kb.add_fact(f_ba)
        kb._index_fact(kb.facts[-2])
        kb._index_fact(kb.facts[-1])
        cycles = detect_provenance_cycles(kb.facts, kb._fact_by_id)
        assert cycles != []
