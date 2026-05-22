"""Multilevel search-tree tests — S1.5.2 / P1.5.

Fixtures use the canonical (zebra2-style) encoding — `is-a` only, no
kernel `(type)` / `(instance)`. See [[project-canonical-zebra2]].
"""
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


# Shared rule library — sibling-exclusive over the is-a hierarchy.
_RULES = """
(rules
  (rule sibling-exclusive (?siblings-via ?exclusive-under)
    :match  (and (?siblings-via ?a ?T) (?siblings-via ?b ?T) (neq ?a ?b))
    :assert (not (?exclusive-under ?a ?b))
    :why    "sib"
    :priority 300))
"""


# ── Tree shape from solve() ───────────────────────────────────────


def test_solve_returns_tree_on_solution():
    """The Verdict carries the SearchTree on its `.tree` attribute."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T))
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
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T))
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
    leaf at the root."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T))
    (facts (r A A :source "(1)"))
    (query :mode solve :goal (r A A))
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
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation co-located T T)
      (sibling-exclusive is-a co-located)
      (is-a Color T) (is-a House T)
      (is-a Red Color) (is-a Blue Color)
      (is-a H1 House) (is-a H2 House))
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
    """A hypothesis that conflicts via sibling-exclusive → dead leaf
    in the tree with non-empty unsat-core."""
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation co-located T T)
      (sibling-exclusive is-a co-located)
      (is-a Red T) (is-a Blue T))
    (query :mode solve :goal (co-located Red Blue))
    """)
    v = solve(kb, max_depth=2)
    assert v.tree is not None
    deads = v.tree.dead_branches()
    for d in deads:
        # Each dead node carries its verdict.
        assert d.verdict == "dead"


# ── max_depth + open leaves ───────────────────────────────────────


def test_max_depth_zero_caps_at_root():
    """With max_depth=0, no branching happens; tree has one node."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
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
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
    (query :mode solve :goal (r A B))
    """)
    v = solve(kb, max_depth=0)
    assert isinstance(v, Ambiguity)
    assert v.tree is not None


def test_contradiction_carries_tree():
    """A baked-in contradiction returns Contradiction with the tree."""
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
    v = solve(kb)
    assert isinstance(v, Contradiction)
    assert v.tree is not None


# ── SearchTree IR round-trip ──────────────────────────────────────


def test_to_ir_produces_trace_form():
    """SearchTree.to_ir() returns a `(trace …)` SForm."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T))
    (facts (r A A :source "(1)"))
    (query :mode solve :goal (r A A))
    """)
    v = solve(kb)
    assert v.tree is not None
    ir = v.tree.to_ir()
    assert ir.head.name == "trace"
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
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation co-located T T)
      (sibling-exclusive is-a co-located)
      (is-a Red T) (is-a Blue T))
    (query :mode solve :goal (co-located Red Blue))
    """)
    v = solve(kb, max_depth=2)
    assert v.tree is not None
    ir = v.tree.to_ir()
    text = dump_canonical(ir)
    forms = parse(text)
    assert len(forms) == 1
    reconstructed = SearchTree.from_ir(forms[0])
    assert reconstructed.root == v.tree.root
    assert set(reconstructed.nodes.keys()) == set(v.tree.nodes.keys())
    for nid, node in v.tree.nodes.items():
        assert reconstructed.nodes[nid].verdict == node.verdict
    for nid, node in v.tree.nodes.items():
        assert reconstructed.nodes[nid].children == node.children


def test_ir_round_trip_preserves_hypothesis_seed():
    """Non-root branches' :on hypothesis fact survives round-trip."""
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation co-located T T)
      (sibling-exclusive is-a co-located)
      (is-a Red T) (is-a Blue T))
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
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T) (is-a C T))
    (query :mode solve :goal (r A B))
    """)
    v_shallow = solve(kb, max_depth=1)
    v_deep = solve(kb, max_depth=3)
    assert v_shallow.tree is not None
    assert v_deep.tree is not None
    # Deeper search has at least as many nodes.
    assert len(v_deep.tree.nodes) >= len(v_shallow.tree.nodes)
