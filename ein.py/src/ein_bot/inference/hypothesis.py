"""Hypothesis loop driver — S1.5.1 / S1.5.2 / S1.5.3.

Top-level driver for the outer loop the engine runs when P1.3's
saturator stalls without solving. Wraps the lower-level pieces:

- generation:  :mod:`ein_bot.inference.hypgen`
- single-branch test (Q40 protocol):  :func:`try_branch` (here)
- proof object + IR round-trip:  :mod:`ein_bot.inference.search_tree`
- canonical state hashing for dedup:  :mod:`ein_bot.inference.canon`

This module owns the Mode/Verdict types, the Q40 try_branch call,
the recursive descent (`_explore`), the bottom-up verdict
promotion (`_promote_verdicts`), and the top-level `solve` driver.

Module path follows Q39 (flat `src/ein_bot/inference/`); see
[[project-canonical-zebra2]] for the encoding-agnostic story.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Literal

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase

from .canon import state_hash
from .compile import JoinPlan, compile_pattern
from .contradiction import ContradictionDetector
from .firing import Firing
from .hypgen import generate_hypotheses
from .match import run as match_run
from .saturator import Saturator
from .search_tree import BranchId, SearchNode, SearchTree

# ── Mode + verdicts ─────────────────────────────────────────────────


class Mode(Enum):
    """What the loop reports at quiescence (idea 03's three task classes)."""
    SOLVE          = "solve"
    GAPS           = "gaps"
    CONTRADICTIONS = "contradictions"


@dataclass(frozen=True)
class Solution:
    """A surviving branch: KB satisfies the query goal (mode-aware).

    `tree` carries the full SearchTree proof object.
    """
    kb:    KnowledgeBase
    trace: tuple[Firing, ...]
    tree:  SearchTree | None = None


@dataclass(frozen=True)
class Ambiguity:
    """Multiple surviving branches — GAPS mode's normal verdict.

    Under SOLVE mode, also returned when ``solve()`` couldn't pin
    the answer to one distinct KB state: either there are multiple
    solution-leaves OR there are leaves stamped ``open`` (max_depth
    cutoff). ``unresolved`` lists the open leaves so callers can
    decide whether to re-run with a deeper budget.
    """
    branches:   tuple[Solution, ...]
    unresolved: tuple[SearchNode, ...] = ()
    tree:       SearchTree | None = None


@dataclass(frozen=True)
class Contradiction:
    """No surviving branch — the puzzle is unsolvable under the
    given constraints. `unsat_core` is the source-frontier facts
    that jointly produce the conflict; `tree` is the proof-by-
    refutation artefact (every branch dead, with its own unsat-core)."""
    unsat_core: frozenset[Fact] = frozenset()
    tree:       SearchTree | None = None


Verdict = Solution | Ambiguity | Contradiction


# ── BranchResult (intermediate value) ──────────────────────────────


@dataclass(frozen=True)
class BranchResult:
    """One branch's outcome from `try_branch`.

    `kind='alive'` → no contradiction; the saturated KB is available
    for further branching (recursion in S1.5.2).
    `kind='dead'`  → contradiction detected; `unsat_core` records
    the source-frontier facts.
    """
    branch_id:  int
    hypothesis: Fact
    kind:       Literal["alive", "dead"]
    kb:         KnowledgeBase
    firings:    tuple[Firing, ...]
    unsat_core: frozenset[Fact] = field(default_factory=frozenset)

    @classmethod
    def alive(cls, branch_id: int, hypothesis: Fact,
              kb: KnowledgeBase, firings: tuple[Firing, ...]) -> BranchResult:
        return cls(branch_id=branch_id, hypothesis=hypothesis,
                   kind="alive", kb=kb, firings=firings)

    @classmethod
    def dead(cls, branch_id: int, hypothesis: Fact,
             kb: KnowledgeBase, firings: tuple[Firing, ...],
             unsat_core: frozenset[Fact]) -> BranchResult:
        return cls(branch_id=branch_id, hypothesis=hypothesis,
                   kind="dead", kb=kb, firings=firings,
                   unsat_core=unsat_core)

    def is_alive(self) -> bool:
        return self.kind == "alive"


# ── Single-branch test cycle (Q40 protocol) ────────────────────────


def try_branch(
    parent_kb: KnowledgeBase,
    hypothesis: Fact,
    *,
    branch_id: int,
    saturator_steps: int = 10_000,
) -> BranchResult:
    """Fork the parent KB, seed with hypothesis, saturate, detect.

    Implements the Q40 Option A protocol:
      1. Write the hypothesis fact with `kind='hypothesis'`
         provenance to the fork.
      2. Emit synthetic `(hypothesis <h>)` carrier.
      3. Saturate.
      4. If contradiction: emit `(contradiction-under <h>)`,
         re-saturate. The `hypothesis-contradiction` rule (P1.3)
         fires and asserts `(not h)` for the parent to consume.
    """
    fork = parent_kb.fork()

    # Re-stamp hypothesis with branch-specific provenance.
    h_fact = Fact(
        relation_name=hypothesis.relation_name,
        args=hypothesis.args,
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=branch_id),
    )
    h_fact = fork.add_fact(h_fact)
    fork._index_fact(h_fact)

    synth_h = Fact(
        relation_name="hypothesis",
        args=(h_fact,),
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=branch_id),
    )
    synth_h = fork.add_fact(synth_h)
    fork._index_fact(synth_h)

    sat = Saturator(fork)
    firings = list(sat.saturate(max_steps=saturator_steps))

    contradictions = ContradictionDetector(fork).detect()
    if contradictions:
        # Q40 step 5: emit (contradiction-under h) so
        # hypothesis-contradiction rule can fire.
        contra_fact = Fact(
            relation_name="contradiction-under",
            args=(h_fact,),
            layer=Layer.REASONING,
            provenance=Provenance.from_hypothesis(branch=branch_id),
        )
        contra_fact = fork.add_fact(contra_fact)
        fork._index_fact(contra_fact)
        # Q40 step 6: re-saturate; the rule fires and produces (not h).
        more = list(sat.saturate(max_steps=saturator_steps))
        firings.extend(more)
        # Compute unsat-core from the contradicting facts.
        unsat = fork.unsat_core(c.positive for c in contradictions)
        return BranchResult.dead(
            branch_id=branch_id,
            hypothesis=h_fact,
            kb=fork,
            firings=tuple(firings),
            unsat_core=frozenset(unsat),
        )

    return BranchResult.alive(
        branch_id=branch_id,
        hypothesis=h_fact,
        kb=fork,
        firings=tuple(firings),
    )


# ── Mode-aware goal check ──────────────────────────────────────────


def is_solved(kb: KnowledgeBase, mode: Mode) -> bool:
    """Has the KB satisfied the query goal under `mode`?

    SOLVE — exactly one binding satisfies the goal pattern.
    GAPS  — at least one binding satisfies the goal pattern.
    CONTRADICTIONS — never solved (runs to exhaustion).
    """
    if mode is Mode.CONTRADICTIONS:
        return False
    if kb.query is None:
        return False
    goal = _query_value(kb.query, "goal")
    if goal is None:
        return False

    steps = compile_pattern(goal, {})
    plan = JoinPlan(
        rule_name="<query>",
        activator_args=(),
        bindings_seed={},
        steps=tuple(steps),
        assert_template=None,
        why="",
    )
    matches = list(match_run(plan, kb))
    if mode is Mode.SOLVE:
        return len(matches) == 1
    if mode is Mode.GAPS:
        return len(matches) >= 1
    return False


def _query_value(query, kw_name: str):
    """Look up a kw_pair value by keyword name on a Query."""
    for kp in query.kw_pairs:
        if hasattr(kp, "key") and kp.key.name == kw_name:
            return kp.value
    return None


def _mode_from_query(kb: KnowledgeBase) -> Mode | None:
    if kb.query is None:
        return None
    mv = _query_value(kb.query, "mode")
    if mv is None or not hasattr(mv, "name"):
        return None
    try:
        return Mode(mv.name)
    except ValueError:
        return None


# ── Tree builder (internal) ────────────────────────────────────────


class _TreeBuilder:
    """Append-only builder for SearchNodes during recursive descent.

    `state_index` maps a post-saturation `state_hash(kb)` to the
    `BranchId` of the SearchNode that owns that state. S1.5.3's
    dedup short-circuits `_explore` when an already-seen state is
    reached via a new path; the search tree becomes a DAG in
    storage (multiple parents can reference the same child).

    `set_verdict` replaces a node's verdict — frozen-dataclass
    rebuilds — used by `_promote_verdicts` to propagate leaf
    verdicts up through the interior nodes.
    """

    def __init__(self) -> None:
        self._next_id: BranchId = 0
        self.nodes: dict[BranchId, SearchNode] = {}
        self.state_index: dict[int, BranchId] = {}

    def alloc(self) -> BranchId:
        nid = self._next_id
        self._next_id += 1
        return nid

    def peek_id(self) -> BranchId:
        return self._next_id

    def add(self, node: SearchNode) -> None:
        self.nodes[node.id] = node

    def set_verdict(self, nid: BranchId, verdict: str) -> None:
        """Replace a node's verdict (frozen dataclass → rebuild)."""
        prev = self.nodes[nid]
        self.nodes[nid] = SearchNode(
            id=prev.id, parent=prev.parent, hypothesis=prev.hypothesis,
            kb_snapshot=prev.kb_snapshot, firings=prev.firings,
            verdict=verdict, children=prev.children,
            unsat_core=prev.unsat_core,
        )

    def finalize(self, root_id: BranchId) -> SearchTree:
        return SearchTree(root=root_id, nodes=dict(self.nodes))


# ── Recursive descent ──────────────────────────────────────────────


def _explore(
    kb: KnowledgeBase,
    parent_id: BranchId | None,
    hypothesis: Fact | None,
    firings: tuple[Firing, ...],
    depth: int,
    max_depth: int,
    builder: _TreeBuilder,
    mode: Mode,
) -> BranchId:
    """Build the subtree rooted at the given (already-saturated) KB.

    Leaf verdicts: ``dead`` (contradiction), ``solution`` (is_solved
    under mode), ``open`` (depth cap hit). Interior nodes are
    stamped ``open`` and later promoted by ``_promote_verdicts``.

    S1.5.3 dedup: every entry computes `state_hash(kb)` and short-
    circuits to the existing node when the same state has been
    explored via another path. Two branches that saturate to the
    same closed KB share one SearchNode — the tree is a DAG in
    storage.
    """
    sh = state_hash(kb)
    existing = builder.state_index.get(sh)
    if existing is not None:
        return existing

    contradictions = ContradictionDetector(kb).detect()
    if contradictions:
        nid = builder.alloc()
        builder.state_index[sh] = nid
        unsat = kb.unsat_core(c.positive for c in contradictions)
        builder.add(SearchNode(
            id=nid, parent=parent_id, hypothesis=hypothesis,
            kb_snapshot=kb, firings=firings,
            verdict="dead", children=(),
            unsat_core=frozenset(unsat),
        ))
        return nid

    if is_solved(kb, mode):
        nid = builder.alloc()
        builder.state_index[sh] = nid
        builder.add(SearchNode(
            id=nid, parent=parent_id, hypothesis=hypothesis,
            kb_snapshot=kb, firings=firings,
            verdict="solution", children=(),
        ))
        return nid

    if depth >= max_depth:
        nid = builder.alloc()
        builder.state_index[sh] = nid
        builder.add(SearchNode(
            id=nid, parent=parent_id, hypothesis=hypothesis,
            kb_snapshot=kb, firings=firings,
            verdict="open", children=(),
        ))
        return nid

    # Interior: allocate id, then recurse into each child. Register
    # the state mapping BEFORE recursing so a child whose
    # post-saturation state matches this one (rare but possible)
    # can dedup back to us.
    interior_id = builder.alloc()
    builder.state_index[sh] = interior_id
    child_ids: list[BranchId] = []
    seen_children: set[BranchId] = set()
    for h in generate_hypotheses(kb):
        result = try_branch(kb, h, branch_id=builder.peek_id())
        if result.is_alive():
            child_id = _explore(
                result.kb, interior_id, h, result.firings,
                depth + 1, max_depth, builder, mode,
            )
        else:
            # Dead branches are NOT deduped by state-hash: each
            # carries its own contradicting fact pair and its own
            # unsat-core, both of which are part of the proof
            # witnesses. Two distinct hypotheses dying for the same
            # underlying reason are still recorded separately.
            child_id = builder.alloc()
            builder.add(SearchNode(
                id=child_id, parent=interior_id, hypothesis=h,
                kb_snapshot=result.kb, firings=result.firings,
                verdict="dead", children=(),
                unsat_core=result.unsat_core,
            ))
        if child_id not in seen_children:
            seen_children.add(child_id)
            child_ids.append(child_id)

    builder.add(SearchNode(
        id=interior_id, parent=parent_id, hypothesis=hypothesis,
        kb_snapshot=kb, firings=firings,
        verdict="open", children=tuple(child_ids),
    ))
    return interior_id


def _promote_verdicts(builder: _TreeBuilder, root_id: BranchId) -> None:
    """Bottom-up walk; interior nodes inherit verdicts from descendants.

    Priority: any descendant ``solution`` ⇒ ``solution``; else any
    descendant ``open`` ⇒ ``open``; else all ``dead`` ⇒ ``dead``.
    Memoised per node so DAG-shared subtrees are walked once.
    """
    cache: dict[BranchId, str] = {}
    in_progress: set[BranchId] = set()

    def walk(nid: BranchId) -> str:
        if nid in cache:
            return cache[nid]
        if nid in in_progress:
            # Cycle should be impossible (each branch strictly extends
            # its parent's facts, so state-hashes can't collide along
            # an ancestor chain). Be defensive anyway.
            return "open"
        in_progress.add(nid)
        node = builder.nodes[nid]
        if not node.children:
            cache[nid] = node.verdict
        else:
            child_verdicts = [walk(c) for c in node.children]
            if any(v == "solution" for v in child_verdicts):
                cache[nid] = "solution"
            elif any(v == "open" for v in child_verdicts):
                cache[nid] = "open"
            else:
                cache[nid] = "dead"
        in_progress.discard(nid)
        if cache[nid] != node.verdict:
            builder.set_verdict(nid, cache[nid])
        return cache[nid]

    walk(root_id)


# ── Top-level solve driver ─────────────────────────────────────────


def solve(
    kb: KnowledgeBase,
    *,
    mode: Mode | None = None,
    max_depth: int = 6,
) -> Verdict:
    """Multilevel hypothesis search — saturate, generate, recurse.

    Builds the full SearchTree (proof object) and runs S1.5.3's
    verdict-promotion pass before extracting the verdict. Returns:

    - ``Solution`` if exactly one solution-leaf survives and there
      are no open leaves (under SOLVE mode).
    - ``Contradiction`` if zero solution-leaves survive and there
      are no open leaves (the dead-branch unsat-cores certify
      unsolvability).
    - ``Ambiguity`` if multiple solution-leaves exist OR any open
      leaves remain (max_depth cutoff). The latter case populates
      ``Ambiguity.unresolved`` so callers can decide to deepen.

    The returned Verdict carries the SearchTree on its ``tree``
    attribute. P1.6 reads it to render the proof.
    """
    mode = mode or _mode_from_query(kb) or Mode.SOLVE

    # Initial saturation on the root KB.
    sat = Saturator(kb)
    root_firings = tuple(sat.saturate(max_steps=10_000))

    builder = _TreeBuilder()
    root_id = _explore(kb, None, None, root_firings,
                       depth=0, max_depth=max_depth,
                       builder=builder, mode=mode)
    _promote_verdicts(builder, root_id)
    tree = builder.finalize(root_id)

    solutions = tree.solutions()
    dead = tree.dead_branches()
    opens = tree.open_branches()

    if mode is Mode.SOLVE:
        if len(solutions) == 1 and not opens:
            s = solutions[0]
            return Solution(kb=s.kb_snapshot, trace=s.firings, tree=tree)
        if len(solutions) == 0 and not opens:
            unsat = frozenset().union(
                *(d.unsat_core for d in dead), frozenset(),
            )
            return Contradiction(unsat_core=unsat, tree=tree)
        # Multiple solutions, or unresolved open leaves → ambiguity.
        return Ambiguity(
            branches=tuple(
                Solution(kb=s.kb_snapshot, trace=s.firings, tree=tree)
                for s in solutions
            ),
            unresolved=opens,
            tree=tree,
        )

    if mode is Mode.GAPS:
        return Ambiguity(
            branches=tuple(
                Solution(kb=s.kb_snapshot, trace=s.firings, tree=tree)
                for s in solutions
            ),
            unresolved=opens,
            tree=tree,
        )

    # CONTRADICTIONS mode — report dead-branch cores.
    unsat = frozenset().union(*(d.unsat_core for d in dead), frozenset())
    return Contradiction(unsat_core=unsat, tree=tree)


__all__ = [
    "Ambiguity",
    "BranchId",
    "BranchResult",
    "Contradiction",
    "Mode",
    "SearchNode",
    "SearchTree",
    "Solution",
    "Verdict",
    "generate_hypotheses",
    "is_solved",
    "solve",
    "state_hash",
    "try_branch",
]
