"""S1.5.3 canonicalisation: state_hash, dedup, verdict promotion.

Pure-zebra2 fixtures (no kernel `(type)` / `(instance)`).
"""
from __future__ import annotations

from ein_bot.inference.hypothesis import (
    Ambiguity,
    Contradiction,
    Mode,
    SearchTree,
    Solution,
    _promote_verdicts,
    _TreeBuilder,
    solve,
    state_hash,
    try_branch,
)
from ein_bot.ir import dump_canonical, parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


_RULES = """
(rules
  (rule symmetric (?rel)
    :match  (?rel ?a ?b) :assert (?rel ?b ?a)
    :why "s" :priority 100)
  (rule transitive (?rel)
    :match  (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c))
    :assert (?rel ?a ?c)
    :why "t" :priority 200)
  (rule sibling-exclusive (?out)
    :match  (and (is-a ?a ?T) (is-a ?b ?T) (neq ?a ?b))
    :assert (not (?out ?a ?b))
    :why "sib" :priority 300)
  (rule functional (?R)
    :match  (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))
    :assert (false)
    :why "fn" :priority 250))
"""


# ── state_hash determinism ────────────────────────────────────────


def test_state_hash_deterministic_across_calls():
    kb = _kb("""
    (ontology
      (relation is-a T T) (relation r T T)
      (is-a A T) (is-a B T))
    (facts (r A B :source "(1)"))
    """)
    assert state_hash(kb) == state_hash(kb)


def test_state_hash_order_insensitive_across_facts():
    """Two KBs with the same facts but reversed insertion order
    hash to the same value."""
    a = _kb("""
    (ontology (relation is-a T T) (relation r T T))
    (facts
      (r A B :source "(1)")
      (r B C :source "(2)"))
    """)
    b = _kb("""
    (ontology (relation is-a T T) (relation r T T))
    (facts
      (r B C :source "(2)")
      (r A B :source "(1)"))
    """)
    assert state_hash(a) == state_hash(b)


def test_state_hash_preserves_arg_order():
    """`(r A B)` and `(r B A)` are different propositions; their
    KBs must hash differently."""
    a = _kb("""
    (ontology (relation is-a T T) (relation r T T))
    (facts (r A B :source "(1)"))
    """)
    b = _kb("""
    (ontology (relation is-a T T) (relation r T T))
    (facts (r B A :source "(1)"))
    """)
    assert state_hash(a) != state_hash(b)


def test_state_hash_distinguishes_nested_args():
    """`(not (r A B))` vs `(not (r B A))` — the outer-set sort
    doesn't touch the inner args."""
    a = _kb("""
    (ontology (relation is-a T T) (relation r T T))
    (facts (not (r A B) :source "(1)"))
    """)
    b = _kb("""
    (ontology (relation is-a T T) (relation r T T))
    (facts (not (r B A) :source "(1)"))
    """)
    assert state_hash(a) != state_hash(b)


def test_state_hash_ignores_bookkeeping_facts():
    """try_branch adds `(hypothesis h)` / `(contradiction-under h)`
    bookkeeping carriers; state_hash skips them so symmetric
    branches with differing carriers still dedup."""
    kb = _kb("""
    (ontology
      (relation is-a T T) (relation r T T)
      (is-a A T) (is-a B T))
    """)
    before = state_hash(kb)
    result = try_branch(
        kb,
        Fact(relation_name="r", args=("A", "B"), layer=Layer.REASONING),
        branch_id=1,
    )
    # The fork has (r A B) plus the bookkeeping carriers. The
    # propositional content (r A B) is new, so hash differs from
    # `before`; but if we further test the same hypothesis under a
    # different branch_id, the post-fork hash must match.
    again = try_branch(
        kb,
        Fact(relation_name="r", args=("A", "B"), layer=Layer.REASONING),
        branch_id=2,
    )
    assert state_hash(result.kb) == state_hash(again.kb)
    assert state_hash(result.kb) != before


# ── dedup collapses symmetric duplicates ──────────────────────────


def test_dedup_collapses_symmetric_pairs():
    """A puzzle whose answer is reachable via a symmetric pair of
    hypotheses produces ONE solution leaf, not two."""
    kb = _kb(_RULES + """
    (ontology
      (relation is-a T T)
      (relation co-located T T)
      (symmetric         co-located)
      (transitive        co-located)
      (sibling-exclusive co-located)
      (functional        is-a)
      (is-a Color T) (is-a House T)
      (is-a Red Color) (is-a Blue Color)
      (is-a H1  House) (is-a H2  House))
    (facts (co-located Red H1 :source "(1)"))
    (query :mode solve :goal (co-located ?c H2))
    """)
    v = solve(kb, max_depth=3)
    assert isinstance(v, Solution), f"expected Solution, got {type(v).__name__}"
    assert v.tree is not None
    leaves = v.tree.solutions()
    assert len(leaves) == 1, (
        f"expected 1 solution-leaf after dedup, got {len(leaves)}"
    )


# ── verdict promotion ─────────────────────────────────────────────


def test_promote_all_dead_to_dead():
    """Synthesise a 1-level tree with three dead children — root
    inherits ``dead``."""
    from ein_bot.inference.hypothesis import SearchNode
    builder = _TreeBuilder()
    root = builder.alloc()
    c1, c2, c3 = builder.alloc(), builder.alloc(), builder.alloc()
    for c in (c1, c2, c3):
        builder.add(SearchNode(
            id=c, parent=root, hypothesis=None,
            kb_snapshot=None, firings=(),
            verdict="dead", children=(),
        ))
    builder.add(SearchNode(
        id=root, parent=None, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="open", children=(c1, c2, c3),
    ))
    _promote_verdicts(builder, root, Mode.SOLVE)
    assert builder.nodes[root].verdict == "dead"


def test_promote_any_solution_to_solution():
    """One solution among dead siblings ⇒ parent is ``solution``."""
    from ein_bot.inference.hypothesis import SearchNode
    builder = _TreeBuilder()
    root = builder.alloc()
    c1, c2, c3 = builder.alloc(), builder.alloc(), builder.alloc()
    builder.add(SearchNode(
        id=c1, parent=root, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="dead", children=(),
    ))
    builder.add(SearchNode(
        id=c2, parent=root, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="solution", children=(),
    ))
    builder.add(SearchNode(
        id=c3, parent=root, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="dead", children=(),
    ))
    builder.add(SearchNode(
        id=root, parent=None, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="open", children=(c1, c2, c3),
    ))
    _promote_verdicts(builder, root, Mode.SOLVE)
    assert builder.nodes[root].verdict == "solution"


def test_promote_mixed_with_open_to_open():
    """Mixed dead + open (no solution) ⇒ ``open``."""
    from ein_bot.inference.hypothesis import SearchNode
    builder = _TreeBuilder()
    root = builder.alloc()
    c1, c2 = builder.alloc(), builder.alloc()
    builder.add(SearchNode(
        id=c1, parent=root, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="dead", children=(),
    ))
    builder.add(SearchNode(
        id=c2, parent=root, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="open", children=(),
    ))
    builder.add(SearchNode(
        id=root, parent=None, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="open", children=(c1, c2),
    ))
    _promote_verdicts(builder, root, Mode.SOLVE)
    assert builder.nodes[root].verdict == "open"


# ── Ambiguity.unresolved ─────────────────────────────────────────


def test_max_depth_zero_ambiguity_carries_unresolved():
    """Root with no children + verdict=open ⇒ Ambiguity with
    ``unresolved`` listing it."""
    kb = _kb("""
    (ontology
      (relation is-a T T)
      (relation r T T)
      (is-a A T) (is-a B T))
    (query :mode solve :goal (r A B))
    """)
    v = solve(kb, max_depth=0)
    assert isinstance(v, Ambiguity)
    assert len(v.unresolved) == 1
    assert v.unresolved[0].id == v.tree.root


# ── solve() doesn't return early on first-match ──────────────────


def test_solve_does_not_early_return_on_first_match():
    """Baked-in contradiction at root — solve() detects it and
    returns Contradiction, NOT Solution, even though one of the
    facts would trivially match the goal."""
    kb = _kb("""
    (ontology
      (relation is-a T T) (relation r T T)
      (is-a A T) (is-a B T))
    (reasoning
      (r A B)
      (not (r A B)))
    (query :mode solve :goal (r A B))
    """)
    v = solve(kb)
    assert isinstance(v, Contradiction)


# ── IR round-trip survives branch-ref for shared nodes ───────────


def test_ir_round_trip_with_branch_ref():
    """Build a tree with a deduped node manually, round-trip it,
    verify the shared-child structure survives."""
    from ein_bot.inference.hypothesis import SearchNode
    builder = _TreeBuilder()
    root = builder.alloc()
    shared = builder.alloc()    # the shared descendant
    a = builder.alloc()
    b = builder.alloc()
    # shared is reached from both a and b.
    builder.add(SearchNode(
        id=shared, parent=a, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="solution", children=(),
    ))
    builder.add(SearchNode(
        id=a, parent=root, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="solution", children=(shared,),
    ))
    builder.add(SearchNode(
        id=b, parent=root, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="solution", children=(shared,),
    ))
    builder.add(SearchNode(
        id=root, parent=None, hypothesis=None,
        kb_snapshot=None, firings=(),
        verdict="solution", children=(a, b),
    ))
    tree = builder.finalize(root)

    ir = tree.to_ir()
    text = dump_canonical(ir)
    forms = parse(text)
    rec = SearchTree.from_ir(forms[0])

    # Both a and b should still reference `shared` in their children.
    assert shared in rec.nodes[a].children
    assert shared in rec.nodes[b].children
    # `shared` is emitted once; the second visit comes via branch-ref.
    assert shared in rec.nodes


# ── End-to-end: demos exercise the engine through Solution ─────


def test_demo_p2_returns_solution_after_dedup():
    """examples/branching/02_one_dead_one_alive.ein returns
    Solution under dedup (was Ambiguity pre-S1.5.3)."""
    from pathlib import Path
    repo = Path(__file__).resolve().parents[3]
    text = (repo / "examples" / "branching"
            / "02_one_dead_one_alive.ein").read_text()
    kb = KnowledgeBase.from_ir(parse(text))
    v = solve(kb, mode=Mode.SOLVE)
    assert isinstance(v, Solution)


def test_demo_p4_remains_ambiguity():
    """examples/branching/04_two_levels.ein is genuinely
    underspecified — both Blue and Green can be at H3. Dedup
    cannot collapse the two distinct closed KBs."""
    from pathlib import Path
    repo = Path(__file__).resolve().parents[3]
    text = (repo / "examples" / "branching" / "04_two_levels.ein").read_text()
    kb = KnowledgeBase.from_ir(parse(text))
    v = solve(kb, mode=Mode.SOLVE, max_depth=3)
    assert isinstance(v, Ambiguity)
    assert len(v.branches) >= 2
