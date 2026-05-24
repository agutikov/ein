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

from contextvars import ContextVar
from dataclasses import dataclass, field
from enum import Enum
from typing import TYPE_CHECKING, Literal

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase

from .back_prop import back_propagate, is_unconditional_death
from .canon import state_hash
from .closed import emit_closed
from .compile import JoinPlan, compile_pattern
from .config import SolverConfig
from .contradiction import ContradictionDetector
from .firing import Firing
from .hypgen import (
    generate_hypotheses,
    generate_hypotheses_with_stats,
    score_hypothesis,
)
from .match import run as match_run
from .saturator import Saturator
from .search_tree import BranchId, SearchNode, SearchTree

if TYPE_CHECKING:
    from .state_dump import StateDumper

# S1.5a.11 — optional per-solve filesystem snapshotting. When a
# StateDumper is bound here, the solver calls back at lifecycle
# points (see state_dump.StateDumper). Bound by solve() at entry;
# reset on exit. ContextVar so concurrent solves don't trample each
# other (the engine itself isn't thread-safe, but solve() is a
# reasonable boundary for parallel diagnostic runs).
_dumper_ctx: ContextVar[StateDumper | None] = ContextVar(
    "ein_bot_state_dumper", default=None,
)

# Current parent-node id for the consume/descend cycle. _consume
# and _descend set this to the node they are exploring so the
# `node_resaturated` hook (and any future hook needing "current
# scope") can attach events to the right node. `None` denotes the
# root node's children scope.
_current_parent_ctx: ContextVar[int | None] = ContextVar(
    "ein_bot_current_parent_nid", default=None,
)

# Depth of the search-tree node `_explore` is currently building.
# Set on `_explore` entry, reset on exit. Diagnostic / instrumentation
# wrappers (bench_solve --verbose) read this to print branch level
# next to each progress line. Root is depth 0; children of root are
# depth 1; etc.
_current_depth_ctx: ContextVar[int] = ContextVar(
    "ein_bot_current_depth", default=0,
)


def _alloc_node(builder: _TreeBuilder, parent_id: int | None) -> int:
    """`builder.alloc()` + register the node's dir on any bound dumper.

    Pre-registering the dir at alloc time guarantees that any
    children allocated inside the same `_explore` / `_consume` call
    can nest under their parent's already-existing directory.
    """
    nid = builder.alloc()
    d = _dumper_ctx.get()
    if d is not None:
        d.node_alloc(nid, parent_id)
    return nid


def _dump_node(
    nid: int,
    parent_id: int | None,
    hypothesis: Fact | None,
    kb: KnowledgeBase,
    firings: tuple[Firing, ...],
    verdict_kind: str,
    unsat_core: frozenset[Fact] = frozenset(),
) -> None:
    """Forward to the bound StateDumper (no-op if none is bound).

    Centralises the dumper-existence check so the alloc sites in
    `_explore` / `_descend` / `_consume` stay terse. Mirrors the
    fields `SearchNode` carries — call it right after
    `builder.add(SearchNode(...))`.
    """
    d = _dumper_ctx.get()
    if d is not None:
        d.node_dump(nid, parent_id, hypothesis, kb, firings,
                    verdict_kind, unsat_core)

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


# ── ConsumeStats (S1.5.7b T1.5.7b.2) ───────────────────────────────


@dataclass
class ConsumeStats:
    """Per-`solve()` counters for the back-prop `_consume` loop.

    Mutated in place by `_consume`; seeded on the root KB by
    :func:`solve` and inherited by forks via `kb.fork()` (shared by
    reference so all consume invocations across the search write to
    the same instance).

    - ``alive_cached_skips`` — `try_branch` calls *avoided* because
      the candidate was already verified alive against the current
      re-saturation generation (T1.5.7b.1).
    - ``cond_dead_cached_skips`` — `try_branch` calls *avoided*
      because the candidate was already verified conditionally-dead
      against the current re-saturation generation (T1.5.7b.4).
    - ``cache_invalidations`` — number of times the per-`_consume`-call
      cache was cleared because re-saturation derived a non-`(not …)`
      fact. Always 0 under M1 (no rule consumes `(not X)` to derive a
      positive); > 0 only once S1.5.8's `domain-elimination` ships
      (T1.5.7b.5).
    """
    alive_cached_skips:      int = 0
    cond_dead_cached_skips:  int = 0
    cache_invalidations:     int = 0


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
        unsat = fork.unsat_core(c.witness for c in contradictions)
        result = BranchResult.dead(
            branch_id=branch_id,
            hypothesis=h_fact,
            kb=fork,
            firings=tuple(firings),
            unsat_core=frozenset(unsat),
        )
    else:
        result = BranchResult.alive(
            branch_id=branch_id,
            hypothesis=h_fact,
            kb=fork,
            firings=tuple(firings),
        )

    return result


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


# ── Candidate selection — T1.5.4.8 (Topic D) ──────────────────────


def _candidate_sort_key(fact: Fact, kb: KnowledgeBase) -> tuple:
    """Total-order key for hypothesis-candidate iteration.

    Components, in order of dominance:

    1. ``-score_hypothesis(fact, kb)`` — higher score wins, by tried-
       first convention. S1.5a.1a leaves the score constant at 0
       (S1.5a.7 will fill it); the unary negation keeps the sort
       ascending so the same tuple-key shape composes when the
       score is real.
    2. ``fact.args`` — tuple of strings (or nested Facts under Q40,
       sorted lexicographically by repr-shape).
    3. ``fact.relation_name`` — final tiebreaker so two facts with
       identical args under different relations get a stable order.

    The point of this key is **determinism**: it is content-based, so
    a Python process's randomised ``hash(str)`` never reaches the
    iteration order. See `plans/.../s1.5a.1a_branch_order_determinism.md`.
    """
    return (-score_hypothesis(fact, kb), fact.args, fact.relation_name)


def _candidates_for(kb: KnowledgeBase) -> list[Fact]:
    """Pick the per-branch hypothesis candidates, ordered deterministically.

    Default path (``cfg.enable_alive_inherit=True``): the alive
    set lives on ``kb.alive`` (seeded once at the root by
    :func:`solve` and inherited through forks). Prune it against
    the current KB (re-applying the candidate-level filters that
    might have flipped on path-introduced facts) and iterate.

    Fallback path (``cfg.enable_alive_inherit=False``): per-branch
    ``generate_hypotheses(kb)`` — the pre-``40b8dd4`` shape.
    Useful escape hatch for puzzles whose rule library violates
    the M1 invariant (no rule-created relations / objects).

    Both paths run the result through :func:`_candidate_sort_key`
    so the visit order is process-independent (S1.5a.1a). With the
    S1.5a.1a stub score the effective order is
    ``(args, relation_name)``; the score primary key reserves the
    slot for S1.5a.7's real scoring without a downstream code edit.
    """
    cfg: SolverConfig = kb.config or SolverConfig()
    if not cfg.enable_alive_inherit or kb.alive is None:
        raw = list(generate_hypotheses(kb))
        return sorted(raw, key=lambda f: _candidate_sort_key(f, kb))
    pruned = _prune_alive(kb.alive, kb)
    if cfg.print_alive:
        import sys
        print(
            f"  [alive] inherited={len(kb.alive)} "
            f"pruned={len(kb.alive) - len(pruned)} "
            f"emit={len(pruned)}",
            file=sys.stderr,
        )
    # Mutate kb.alive in place so siblings/descendants of this
    # state pick up the pruned form without re-pruning.
    kb.alive = pruned
    return sorted(pruned, key=lambda f: _candidate_sort_key(f, kb))


def _prune_alive(alive: frozenset, kb: KnowledgeBase) -> frozenset:
    """Drop alive candidates inadmissible at this state.

    Re-applies the per-candidate filters from
    :func:`~ein_bot.inference.hypgen._apply_filters` against the
    current KB:

    - **negated_fact** — ``(rel, args) in kb._negated_facts`` (the
      Tier-A check; fires when an ancestor or this state's
      saturation derived ``(not h)``).
    - **fact_already_exists** — ``kb._fact_by_id(rel, args)`` returns
      a hit (the path added the fact directly; speculating it
      again would be a no-op + state-hash collision).

    Returns a new frozenset; never mutates ``alive``.
    """
    out = []
    for h in alive:
        key = (h.relation_name, h.args)
        if key in kb._negated_facts:
            continue
        if kb._fact_by_id(h.relation_name, h.args) is not None:
            continue
        out.append(h)
    return frozenset(out)


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
    depth_token = _current_depth_ctx.set(depth)
    try:
        return _explore_inner(
            kb, parent_id, hypothesis, firings, depth, max_depth,
            builder, mode,
        )
    finally:
        _current_depth_ctx.reset(depth_token)


def _explore_inner(
    kb: KnowledgeBase,
    parent_id: BranchId | None,
    hypothesis: Fact | None,
    firings: tuple[Firing, ...],
    depth: int,
    max_depth: int,
    builder: _TreeBuilder,
    mode: Mode,
) -> BranchId:
    """Body of `_explore` (factored out for the depth-ctx try/finally
    wrapper). See :func:`_explore` for the contract.
    """
    sh = state_hash(kb)
    existing = builder.state_index.get(sh)
    if existing is not None:
        return existing

    contradictions = ContradictionDetector(kb).detect()
    if contradictions:
        nid = _alloc_node(builder, parent_id)
        builder.state_index[sh] = nid
        unsat = kb.unsat_core(c.witness for c in contradictions)
        builder.add(SearchNode(
            id=nid, parent=parent_id, hypothesis=hypothesis,
            kb_snapshot=kb, firings=firings,
            verdict="dead", children=(),
            unsat_core=frozenset(unsat),
        ))
        _dump_node(nid, parent_id, hypothesis, kb, firings,
                   "dead", frozenset(unsat))
        return nid

    # NOTE — no early `is_solved` exit. A state that matches the
    # goal but still has alive hypotheses is NOT a solution leaf
    # (S1.5.0 §F: "the complete exploration tree is the proof";
    # user reaffirmed 2026-05-21). The is_solved check happens at
    # the genuine leaf — when `generate_hypotheses` produces no
    # candidates — and at verdict-promotion for interior nodes
    # whose children all turn out dead.

    if depth >= max_depth:
        nid = _alloc_node(builder, parent_id)
        builder.state_index[sh] = nid
        builder.add(SearchNode(
            id=nid, parent=parent_id, hypothesis=hypothesis,
            kb_snapshot=kb, firings=firings,
            verdict="open", children=(),
        ))
        _dump_node(nid, parent_id, hypothesis, kb, firings, "open")
        return nid

    # Interior or no-hypothesis leaf — allocate id, then enumerate
    # hypotheses. If none, this is a leaf and the verdict is
    # determined by `is_solved` (true) or "open" (state matched the
    # goal but the puzzle isn't maximally constrained under the
    # current rule set).
    nid = _alloc_node(builder, parent_id)
    builder.state_index[sh] = nid

    # S1.5.7 T1.5.7.6 — the candidate descent. With back-prop off
    # `_descend` runs the static pre-S1.5.7 loop, byte for byte;
    # with it on, `_consume` runs the iterative back-prop loop and
    # may extend this node's firings with re-saturation passes — a
    # forced move folds in here rather than spending a tree level.
    cfg: SolverConfig = kb.config or SolverConfig()
    if cfg.enable_back_prop_unconditional:
        child_ids, node_firings = _consume(
            kb, nid, firings, depth, max_depth, builder, mode,
        )
    else:
        child_ids = _descend(kb, nid, depth, max_depth, builder, mode)
        node_firings = firings

    if not child_ids:
        # No more hypotheses available — this is a true leaf. The
        # state is maximally constrained under the current rule set.
        leaf_verdict = "solution" if is_solved(kb, mode) else "open"
        builder.add(SearchNode(
            id=nid, parent=parent_id, hypothesis=hypothesis,
            kb_snapshot=kb, firings=node_firings,
            verdict=leaf_verdict, children=(),
        ))
        _dump_node(nid, parent_id, hypothesis, kb, node_firings,
                   leaf_verdict)
        return nid

    # Interior: stamp "open" pre-promotion. _promote_verdicts will
    # resolve it from the children + this state's own is_solved
    # status (the "all-dead-AND-goal-matched ⇒ solution-endpoint"
    # case).
    builder.add(SearchNode(
        id=nid, parent=parent_id, hypothesis=hypothesis,
        kb_snapshot=kb, firings=node_firings,
        verdict="open", children=tuple(child_ids),
    ))
    _dump_node(nid, parent_id, hypothesis, kb, node_firings, "open")
    return nid


def _descend(
    kb: KnowledgeBase,
    nid: BranchId,
    depth: int,
    max_depth: int,
    builder: _TreeBuilder,
    mode: Mode,
    *,
    exclude_keys: frozenset[tuple[str, tuple]] = frozenset(),
) -> list[BranchId]:
    """Enumerate hypotheses once; recurse alive, record dead.

    The static (non-back-prop) descent — the pre-S1.5.7 ``_explore``
    candidate loop extracted verbatim, so the
    ``enable_back_prop_unconditional=False`` path stays byte-identical
    (S1.5.7 T1.5.7.6.d). ``nid`` is the parent of every child.

    ``exclude_keys`` (S1.5.7b T1.5.7b.4) is the set of
    ``(relation_name, args)`` keys for candidates already turned into
    dead SearchNodes by `_consume`'s conditional-dead handling — they
    are skipped here to avoid double-allocation. Empty default keeps
    the flag-off path byte-identical.
    """
    child_ids: list[BranchId] = []
    seen_children: set[BranchId] = set()
    candidates = _candidates_for(kb)
    parent_token = _current_parent_ctx.set(nid)
    try:
        for h in candidates:
            if exclude_keys and (h.relation_name, h.args) in exclude_keys:
                continue
            result = try_branch(kb, h, branch_id=builder.peek_id())
            if result.is_alive():
                child_id = _explore(
                    result.kb, nid, h, result.firings,
                    depth + 1, max_depth, builder, mode,
                )
            else:
                # Dead branches are NOT deduped by state-hash: each
                # carries its own contradicting fact pair and its own
                # unsat-core, both of which are part of the proof
                # witnesses. Two distinct hypotheses dying for the same
                # underlying reason are still recorded separately.
                child_id = _alloc_node(builder, nid)
                builder.add(SearchNode(
                    id=child_id, parent=nid, hypothesis=h,
                    kb_snapshot=result.kb, firings=result.firings,
                    verdict="dead", children=(),
                    unsat_core=result.unsat_core,
                ))
                _dump_node(child_id, nid, h, result.kb, result.firings,
                           "dead", result.unsat_core)
            if child_id not in seen_children:
                seen_children.add(child_id)
                child_ids.append(child_id)
    finally:
        _current_parent_ctx.reset(parent_token)
    return child_ids


def _consume(
    kb: KnowledgeBase,
    nid: BranchId,
    firings: tuple[Firing, ...],
    depth: int,
    max_depth: int,
    builder: _TreeBuilder,
    mode: Mode,
) -> tuple[list[BranchId], tuple[Firing, ...]]:
    """Iterative back-prop consume loop — S1.5.7 T1.5.7.2 / .5 / .6
    + S1.5.7b T1.5.7b.1 / .4 (verdict cache).

    Each pass sweeps every still-unverified candidate with
    ``try_branch``. The verdict cache (``verdict_at``) records each
    candidate's last-known outcome so subsequent passes skip
    candidates whose verdict is stable across re-saturation — under
    the M1 rule library re-saturation cannot turn an alive or
    conditionally-dead verdict into anything else, since it only
    propagates more ``(not …)`` facts (the
    :meth:`Firing.derives_positive` predicate is universally False
    under M1; ``True`` only once S1.5.8's ``domain-elimination``
    ships, at which point the cache is cleared).

    Per-candidate outcomes:

    - **alive** ⇒ cache the verdict; the candidate is left for
      ``_descend``'s recursion on the fixpoint pass.
    - **dead, unconditional** (``is_unconditional_death`` walks back
      to source/rule leaves only) ⇒ ``back_propagate`` writes
      ``(not h)`` into the parent KB (T1.5.7.2), making the
      candidate disappear from ``_candidates_for`` permanently. The
      dead SearchNode is allocated after the sweep completes.
    - **dead, conditional** (unsat-core's transitive walk reaches an
      ancestor-hypothesis fact) ⇒ ``back_propagate`` would be
      unsound (``h`` may be alive against a different sibling at a
      different ancestor context). Instead, the SearchNode is
      allocated inline now, the verdict cached, and the candidate
      key added to ``cond_dead_keys`` so ``_descend`` skips it via
      its ``exclude_keys`` arg (T1.5.7b.4).

    If any sibling died unconditionally this pass the parent ``kb``
    is re-saturated once (T1.5.7.5) — a back-propped ``(not h)`` is
    only an O(1) filter entry until re-saturation makes it a premise
    the rule engine can chain from — and the loop repeats over the
    shrunk candidate set. Each progressing pass back-props ≥ 1
    ``(not h)``, so the candidate set strictly shrinks and the loop
    terminates.

    When a pass produces no unconditional deaths the still-pending
    candidates are a genuine disjunction (or already-verified
    alive); the loop hands off to ``_descend`` (which re-runs
    ``try_branch`` inline so branch ids land in allocation order —
    flag-off byte-identical behaviour when no back-prop ever fires).

    Returns ``(child_ids, node_firings)``; ``node_firings`` is the
    passed-in ``firings`` extended with every re-saturation pass, so
    a forced move re-saturation derives folds into *this* node's
    trace rather than spending a tree level (T1.5.7.6).

    NOTE — under the M1 rule set re-saturation cannot itself produce
    a *contradiction* (no rule concludes a clash from a ``(not …)``
    premise). The re-saturation-derived-contradiction case —
    S1.5.8's ``domain-elimination`` all-excluded shape — is owned by
    S1.5.8 (T1.5.8.3); revisit this loop when that rule ships.
    """
    child_ids: list[BranchId] = []
    seen_children: set[BranchId] = set()
    node_firings = firings

    # S1.5.7b T1.5.7b.1/.4 — per-`_consume`-call verdict cache.
    # verdict_at[key] = (verdict, resat_gen) — verdict ∈ {"alive",
    # "cond-dead"}. `cond_dead_keys` is the projection used to filter
    # `_descend`'s candidate iteration. `resat_gen` is informational
    # (stats) and a marker for the invalidation guard.
    verdict_at: dict[tuple[str, tuple], tuple[str, int]] = {}
    cond_dead_keys: set[tuple[str, tuple]] = set()
    resat_gen = 0
    stats = kb.consume_stats

    # S1.5a.11 dumper bookkeeping: emit one node_resaturated event
    # per consume cycle (covering all dead children in that sweep
    # + their cumulative negatives + the re-sat firings).
    _dumper = _dumper_ctx.get()

    def _add(cid: BranchId) -> None:
        if cid not in seen_children:
            seen_children.add(cid)
            child_ids.append(cid)

    parent_token = _current_parent_ctx.set(nid)
    try:
        while True:
            candidates = _candidates_for(kb)
            if not candidates:
                break

            # Apply the cache: skip candidates whose verdict is
            # already known; only the unknown set reaches
            # `try_branch`.
            to_check: list[Fact] = []
            for h in candidates:
                key = (h.relation_name, h.args)
                cached = verdict_at.get(key)
                if cached is None:
                    to_check.append(h)
                    continue
                if stats is not None:
                    if cached[0] == "alive":
                        stats.alive_cached_skips += 1
                    else:
                        stats.cond_dead_cached_skips += 1

            if not to_check:
                # Every still-pending candidate is cached. Hand off
                # to _descend, excluding cond-deads we've already
                # allocated nodes for.
                for cid in _descend(
                    kb, nid, depth, max_depth, builder, mode,
                    exclude_keys=frozenset(cond_dead_keys),
                ):
                    _add(cid)
                break

            # Sweep — classify each unverified candidate.
            unconditional: list[tuple[Fact, BranchResult]] = []
            facts_before_sweep = len(kb.facts)
            for h in to_check:
                result = try_branch(kb, h, branch_id=builder.peek_id())
                key = (h.relation_name, h.args)
                if result.is_alive():
                    verdict_at[key] = ("alive", resat_gen)
                elif is_unconditional_death(
                        result.kb, result.unsat_core,
                        own_hypothesis=result.hypothesis):
                    back_propagate(kb, h, result.unsat_core)
                    unconditional.append((h, result))
                    # Unconditional-dead is permanently cached via
                    # `_negated_facts`; no verdict_at entry needed
                    # since the candidate won't reappear in
                    # `_candidates_for`.
                else:
                    # Conditional dead — record the SearchNode now
                    # so `_descend` (and subsequent sweeps) don't
                    # re-try it.
                    cid = _alloc_node(builder, nid)
                    builder.add(SearchNode(
                        id=cid, parent=nid, hypothesis=h,
                        kb_snapshot=result.kb, firings=result.firings,
                        verdict="dead", children=(),
                        unsat_core=result.unsat_core,
                    ))
                    _dump_node(cid, nid, h, result.kb,
                               result.firings, "dead", result.unsat_core)
                    _add(cid)
                    verdict_at[key] = ("cond-dead", resat_gen)
                    cond_dead_keys.add(key)

            if not unconditional:
                # Fixpoint — no sibling died unconditionally. Hand
                # off to the static descent over the still-pending
                # set.
                for cid in _descend(
                    kb, nid, depth, max_depth, builder, mode,
                    exclude_keys=frozenset(cond_dead_keys),
                ):
                    _add(cid)
                break

            # Record the unconditionally-dead branches now — `(not h)`
            # has been back-propped, so they will not be re-swept
            # next pass.
            dead_for_resat: list[tuple[int, Fact]] = []
            for h, result in unconditional:
                cid = _alloc_node(builder, nid)
                builder.add(SearchNode(
                    id=cid, parent=nid, hypothesis=h,
                    kb_snapshot=result.kb, firings=result.firings,
                    verdict="dead", children=(),
                    unsat_core=result.unsat_core,
                ))
                _dump_node(cid, nid, h, result.kb, result.firings,
                           "dead", result.unsat_core)
                _add(cid)
                dead_for_resat.append((cid, h))

            # Re-saturate the parent so the back-propped negatives
            # are premises rule firings can chain from (T1.5.7.5).
            new_firings = tuple(
                Saturator(kb).saturate(max_steps=10_000),
            )
            node_firings = node_firings + new_firings
            resat_gen += 1

            if _dumper is not None:
                # The "new facts" between the sweep start and now
                # include the back-prop negatives + their re-sat
                # cascades. The dumper's _node_dirs maps `nid` to a
                # filesystem path (root is keyed 0).
                added_facts = list(kb.facts[facts_before_sweep:])
                _dumper.node_resaturated(
                    nid=nid,
                    cycle=resat_gen,
                    kb=kb,
                    new_firings=new_firings,
                    dead_children=dead_for_resat,
                    neg_facts_added=added_facts,
                )

            # S1.5.7b invalidation guard — `Firing.derives_positive`
            # is always False under M1.
            if any(f.derives_positive() for f in new_firings):
                verdict_at.clear()
                if stats is not None:
                    stats.cache_invalidations += 1
    finally:
        _current_parent_ctx.reset(parent_token)

    # Re-index under the post-re-saturation state hash so a later
    # branch reaching the settled state dedups here (T1.5.7.5c);
    # `setdefault` keeps any prior owner of that hash.
    builder.state_index.setdefault(state_hash(kb), nid)
    return child_ids, node_firings


def _promote_verdicts(builder: _TreeBuilder, root_id: BranchId,
                      mode: Mode) -> None:
    """Bottom-up walk; interior nodes inherit verdicts from descendants.

    Priority for interior nodes (when at least one child exists):
      - any descendant ``solution`` ⇒ ``solution`` (propagation);
      - else any descendant ``open`` ⇒ ``open``;
      - else (all children dead): this node IS a solution-endpoint
        iff its own KB satisfies the goal. Otherwise ``dead``.

    The "all-dead-AND-goal-matched ⇒ solution" case is what
    distinguishes a true terminal state (no extension is consistent,
    but the state itself answers the query) from a true failure
    (state doesn't answer the query and can't be extended).

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
            elif (node.kb_snapshot is not None
                  and is_solved(node.kb_snapshot, mode)):
                # All children dead + goal matched at this state ⇒
                # the state IS a solution endpoint.
                cache[nid] = "solution"
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
    config: SolverConfig | None = None,
    dumper: StateDumper | None = None,
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

    Config resolution (T1.5.4.4): the effective `SolverConfig` is
    ``config`` (kwarg, highest precedence) or ``kb.config`` (parsed
    from an IR `(config …)` block) or ``SolverConfig()`` (defaults).
    The resolved config is stashed back on ``kb.config`` so
    downstream code (saturator, hypgen, _explore) can read it
    without threading.
    """
    mode = mode or _mode_from_query(kb) or Mode.SOLVE
    effective_config = config or kb.config or SolverConfig()
    kb.config = effective_config
    # S1.5.7b — fresh ConsumeStats on the root; forks inherit by
    # reference so every `_consume` invocation across the search
    # accumulates into the same instance. Allocated even when
    # back-prop is off so post-solve inspection finds zero counters
    # instead of `None`.
    kb.consume_stats = ConsumeStats()

    # S1.5a.11 — bind the dumper for the duration of this solve().
    # Hook points throughout the file read `_dumper_ctx.get()`.
    dumper_token = _dumper_ctx.set(dumper)
    try:
        if dumper is not None:
            dumper.root_initial(kb)

        # Emit `(closed R)` for every relation no rule can positively
        # conclude — before saturation, so the hypothesis generator
        # sees the facts. Replaces hand-written `(closed …)` declarations.
        emit_closed(kb)

        # Initial saturation on the root KB.
        sat = Saturator(kb)
        root_firings = tuple(sat.saturate(max_steps=10_000))
        if dumper is not None:
            dumper.root_saturated(kb, root_firings, sat.naf_dropped)

        # T1.5.4.8 Topic D — compute the alive set once at b0; every
        # descendant inherits it through `kb.fork()`. Skip the seed when
        # the inherit flag is off; `_candidates_for` then falls back to
        # per-branch `generate_hypotheses(kb)`.
        if effective_config.enable_alive_inherit:
            root_alive, root_stats = generate_hypotheses_with_stats(kb)
            kb.alive = frozenset(root_alive)
            if dumper is not None:
                dumper.root_hyps(list(root_alive), root_stats)
            if effective_config.print_alive:
                import sys
                print(
                    f"[root alive] {len(kb.alive)} candidates "
                    f"(raw={root_stats.raw}, "
                    f"filtered={root_stats.total_filtered()})",
                    file=sys.stderr,
                )

        builder = _TreeBuilder()
        root_id = _explore(kb, None, None, root_firings,
                           depth=0, max_depth=max_depth,
                           builder=builder, mode=mode)
        _promote_verdicts(builder, root_id, mode)
        tree = builder.finalize(root_id)

        solutions = tree.solutions()
        dead = tree.dead_branches()
        opens = tree.open_branches()

        verdict: Verdict
        if mode is Mode.SOLVE:
            if len(solutions) == 1 and not opens:
                s = solutions[0]
                verdict = Solution(
                    kb=s.kb_snapshot, trace=s.firings, tree=tree,
                )
            elif len(solutions) == 0 and not opens:
                unsat = frozenset().union(
                    *(d.unsat_core for d in dead), frozenset(),
                )
                verdict = Contradiction(unsat_core=unsat, tree=tree)
            else:
                verdict = Ambiguity(
                    branches=tuple(
                        Solution(kb=s.kb_snapshot, trace=s.firings, tree=tree)
                        for s in solutions
                    ),
                    unresolved=opens,
                    tree=tree,
                )
        elif mode is Mode.GAPS:
            verdict = Ambiguity(
                branches=tuple(
                    Solution(kb=s.kb_snapshot, trace=s.firings, tree=tree)
                    for s in solutions
                ),
                unresolved=opens,
                tree=tree,
            )
        else:
            # CONTRADICTIONS mode — report dead-branch cores.
            unsat = frozenset().union(
                *(d.unsat_core for d in dead), frozenset(),
            )
            verdict = Contradiction(unsat_core=unsat, tree=tree)

        if dumper is not None:
            dumper.summary(verdict, tree, effective_config)
        return verdict
    finally:
        _dumper_ctx.reset(dumper_token)


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
