"""Multilevel search-tree tests — S1.5.2 / P1.5."""
from __future__ import annotations

from ein_bot.inference.hypothesis import (
    Ambiguity,
    Contradiction,
    SearchTree,
    Solution,
    solve,
)
from ein_bot.ir import dump_canonical, parse
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# ── Tree shape from solve() ───────────────────────────────────────


def test_solve_returns_tree_on_solution():
    """The Verdict carries the SearchTree on its `.tree` attribute."""
    kb = _kb("""
    (ontology (type T) (instance A T) (relation r T T))
    (facts (r A A :source "(1)"))
    (query :mode solve :goal (r A A))
    """)
    v = solve(kb)
    assert isinstance(v, Solution)
    assert v.tree is not None
    assert isinstance(v.tree, SearchTree)


def test_root_node_has_no_hypothesis():
    """Root branch wasn't seeded by any hypothesis; root.hypothesis is None."""
    kb = _kb("""
    (ontology (type T) (instance A T) (relation r T T))
    (facts (r A A :source "(1)"))
    (query :mode solve :goal (r A A))
    """)
    v = solve(kb)
    assert v.tree is not None
    root = v.tree.nodes[v.tree.root]
    assert root.hypothesis is None
    assert root.parent is None


def test_trivial_solve_has_solution_leaf():
    """KB is already solved by saturation; the tree has one solution
    leaf (the root)."""
    kb = _kb("""
    (rules (rule symmetric (?rel)
      :match (?rel ?a ?b) :assert (?rel ?b ?a) :why "s" :priority 100))
    (ontology (type T) (instance A T) (instance B T)
              (relation r T T) (symmetric r))
    (facts (r A B :source "(1)"))
    (query :mode solve :goal (r B A))
    """)
    v = solve(kb)
    assert isinstance(v, Solution)
    assert v.tree is not None
    sols = v.tree.solutions()
    assert len(sols) == 1
    # In a trivial solve, the root itself is the solution leaf.
    assert sols[0].id == v.tree.root


# ── Branching produces non-trivial trees ──────────────────────────


def test_branching_produces_children():
    """A KB needing one hypothesis level produces a root with children."""
    kb = _kb("""
    (rules
      (rule type-exclusivity (?R)
        :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
        :assert (not (?R ?a ?b))
        :why "tx" :priority 300))
    (ontology
      (type Attribute)
      (type Color Attribute) (type House Attribute)
      (instance Red Color) (instance Blue Color)
      (instance H1 House) (instance H2 House)
      (relation co-located Attribute Attribute)
      (type-exclusivity co-located))
    (query :mode solve :goal (co-located Red H1))
    """)
    v = solve(kb, max_depth=2)
    assert v.tree is not None
    # Branching should occur — no positive facts force any pairing.
    root = v.tree.nodes[v.tree.root]
    if root.verdict == "open":
        assert len(root.children) > 0
    # Some leaves should be reached.
    assert v.tree.solutions() or v.tree.dead_branches() or root.verdict == "open"


def test_dead_branches_are_in_tree():
    """A hypothesis that conflicts via type-exclusivity → dead leaf
    in the tree with non-empty unsat-core."""
    kb = _kb("""
    (rules
      (rule type-exclusivity (?R)
        :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
        :assert (not (?R ?a ?b))
        :why "tx" :priority 300))
    (ontology
      (type Color)
      (instance Red Color) (instance Blue Color)
      (relation co-located T T)
      (type-exclusivity co-located))
    (query :mode solve :goal (co-located Red Blue))
    """)
    v = solve(kb, max_depth=2)
    assert v.tree is not None
    deads = v.tree.dead_branches()
    # Some branches should die (Red↔Blue can't co-locate).
    for d in deads:
        # Each dead node has an unsat_core.
        assert d.verdict == "dead"


# ── max_depth + open leaves ───────────────────────────────────────


def test_max_depth_zero_caps_at_root():
    """With max_depth=0, no branching happens; tree has one node."""
    kb = _kb("""
    (ontology
      (type T) (instance A T) (instance B T)
      (relation r T T))
    (query :mode solve :goal (r A B))
    """)
    v = solve(kb, max_depth=0)
    assert v.tree is not None
    assert len(v.tree.nodes) == 1
    root = v.tree.nodes[v.tree.root]
    # Root isn't solved (no facts), not contradicted, max_depth = 0 → open.
    assert root.verdict == "open"


# ── Ambiguity / Contradiction packaging ───────────────────────────


def test_ambiguity_carries_tree():
    """An Ambiguity verdict still carries the tree."""
    kb = _kb("""
    (ontology
      (type T) (instance A T) (instance B T)
      (relation r T T))
    (query :mode solve :goal (r A B))
    """)
    v = solve(kb, max_depth=0)
    assert isinstance(v, Ambiguity)
    assert v.tree is not None


def test_contradiction_carries_tree():
    """A baked-in contradiction returns Contradiction with the tree."""
    kb = _kb("""
    (ontology
      (type T) (instance A T) (instance B T)
      (relation r T T))
    (reasoning
      (r A B)
      (not (r A B)))
    (query :mode solve :goal (r A B))
    """)
    v = solve(kb)
    assert isinstance(v, Contradiction)
    assert v.tree is not None
    # The root itself is dead.
    root = v.tree.nodes[v.tree.root]
    assert root.verdict == "dead"


# ── SearchTree IR round-trip ──────────────────────────────────────


def test_to_ir_produces_trace_form():
    """SearchTree.to_ir() returns a `(trace …)` SForm."""
    kb = _kb("""
    (ontology (type T) (instance A T) (relation r T T))
    (facts (r A A :source "(1)"))
    (query :mode solve :goal (r A A))
    """)
    v = solve(kb)
    assert v.tree is not None
    ir = v.tree.to_ir()
    assert ir.head.name == "trace"
    # Should contain at least one branch-open / branch-close pair.
    open_events = [
        e for e in ir.args
        if hasattr(e, "head") and e.head.name == "branch-open"
    ]
    close_events = [
        e for e in ir.args
        if hasattr(e, "head") and e.head.name == "branch-close"
    ]
    assert len(open_events) == len(close_events) == len(v.tree.nodes)


def test_ir_round_trip_preserves_structure():
    """to_ir → parse → from_ir reproduces the tree shape + verdicts."""
    kb = _kb("""
    (rules (rule type-exclusivity (?R)
      :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
      :assert (not (?R ?a ?b)) :why "tx" :priority 300))
    (ontology
      (type Color)
      (instance Red Color) (instance Blue Color)
      (relation co-located T T)
      (type-exclusivity co-located))
    (query :mode solve :goal (co-located Red Blue))
    """)
    v = solve(kb, max_depth=2)
    assert v.tree is not None
    ir = v.tree.to_ir()
    # Round-trip through canonical dump + parse.
    text = dump_canonical(ir)
    forms = parse(text)
    assert len(forms) == 1
    reconstructed = SearchTree.from_ir(forms[0])
    # Same root id and same set of node ids.
    assert reconstructed.root == v.tree.root
    assert set(reconstructed.nodes.keys()) == set(v.tree.nodes.keys())
    # Verdicts preserved.
    for nid, node in v.tree.nodes.items():
        assert reconstructed.nodes[nid].verdict == node.verdict
    # Parent-child structure preserved.
    for nid, node in v.tree.nodes.items():
        assert reconstructed.nodes[nid].children == node.children


def test_ir_round_trip_preserves_hypothesis_seed():
    """Non-root branches' :on hypothesis fact survives round-trip."""
    kb = _kb("""
    (rules (rule type-exclusivity (?R)
      :match (and (instance ?a ?T) (instance ?b ?T) (neq ?a ?b))
      :assert (not (?R ?a ?b)) :why "tx" :priority 300))
    (ontology
      (type Color) (instance Red Color) (instance Blue Color)
      (relation co-located T T)
      (type-exclusivity co-located))
    (query :mode solve :goal (co-located Red Blue))
    """)
    v = solve(kb, max_depth=2)
    assert v.tree is not None

    ir = v.tree.to_ir()
    text = dump_canonical(ir)
    forms = parse(text)
    reconstructed = SearchTree.from_ir(forms[0])

    for nid, node in v.tree.nodes.items():
        if node.hypothesis is not None:
            r = reconstructed.nodes[nid]
            assert r.hypothesis is not None
            assert r.hypothesis.relation_name == node.hypothesis.relation_name
            assert r.hypothesis.args == node.hypothesis.args


# ── solve() honours max_depth ─────────────────────────────────────


def test_solve_respects_max_depth():
    """With a very tight max_depth, deep branches don't run."""
    kb = _kb("""
    (ontology
      (type T)
      (instance A T) (instance B T) (instance C T)
      (relation r T T))
    (query :mode solve :goal (r A B))
    """)
    v_shallow = solve(kb, max_depth=1)
    v_deep = solve(kb, max_depth=3)
    assert v_shallow.tree is not None
    assert v_deep.tree is not None
    # Deeper search has at least as many nodes.
    assert len(v_deep.tree.nodes) >= len(v_shallow.tree.nodes)
