"""Hypothesis loop driver — S1.5.1 / P1.5.

The outer loop the engine runs when P1.3's saturator stalls
without solving. Generates hypotheses two ways at the candidate
level — pick the *most-constrained* object (proxy for CSP's
smallest-domain heuristic, using fact-count as the proxy), then
enumerate `(relation, slot, filler)` triples the object is
type-compatible with but doesn't yet occupy.

Each hypothesis is wrapped in the **Q40 Option A protocol**: the
fork's REASONING layer gets the hypothesis fact itself plus a
synthetic `(hypothesis <h>)` carrier; on contradiction-detection,
a `(contradiction-under <h>)` is emitted, triggering the
`hypothesis-contradiction` rule shipped in P1.3 (which asserts
`(not h)` for propagation).

This module ships the **single-level** driver: one round of fork,
saturate, detect, propagate. The recursive search tree
(S1.5.2) wraps `try_branch` to build the full proof object;
canonicalisation + dedup + alive-branch termination (S1.5.3)
turns the tree into the minimal proof DAG.

Symmetric relations emit BOTH orderings of `(R obj filler)` and
`(R filler obj)` as separate hypotheses — trades memory (one
extra branch per pair) for time (earlier contradiction detection
when one direction would surface a contradiction the other
wouldn't).

Module path follows Q39 (flat `src/ein_bot/inference/`).
"""
from __future__ import annotations

from collections.abc import Iterator
from dataclasses import dataclass, field
from enum import Enum
from typing import Literal

from ein_bot.ir.types import Atom, Int, Keyword, KwPair, SForm
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase

from .compile import JoinPlan, compile_pattern
from .contradiction import ContradictionDetector
from .firing import Firing
from .match import run as match_run
from .saturator import Saturator

# ── Mode + verdicts ─────────────────────────────────────────────────


class Mode(Enum):
    """What the loop reports at quiescence (idea 03's three task classes)."""
    SOLVE          = "solve"
    GAPS           = "gaps"
    CONTRADICTIONS = "contradictions"


@dataclass(frozen=True)
class Solution:
    """A surviving branch: KB satisfies the query goal (mode-aware).

    `tree` carries the full SearchTree proof object (populated by
    S1.5.2's recursive driver; None for direct construction or
    S1.5.1's single-level driver).
    """
    kb:    KnowledgeBase
    trace: tuple[Firing, ...]
    tree:  SearchTree | None = None


@dataclass(frozen=True)
class Ambiguity:
    """Multiple surviving branches — GAPS mode's normal verdict."""
    branches: tuple[Solution, ...]
    tree:     SearchTree | None = None


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


# ── Hypothesis generation (two-step) ───────────────────────────────


def generate_hypotheses(kb: KnowledgeBase) -> Iterator[Fact]:
    """Yield candidate hypothesis facts in priority order.

    Step 1 — order instances by descending fact-count (most-
    constrained first; deterministic tiebreak by name).
    Step 2 — per instance, enumerate `(R, slot)` in
    `possible(obj) - existing(obj)`, fill the other slot with
    type-compatible instances, prune by `(not …)` in the KB.

    Symmetric R emits both orderings as separate hypotheses.

    Same-call dedup: a fact yielded once (by identity tuple
    `(relation_name, args)`) is not yielded again — both Alice
    and Bob enumerate `(r Alice Bob)` from their respective
    candidate slots, but only the first is yielded.
    """
    if not kb.instances:
        return
    by_count = sorted(
        kb.instances.values(),
        key=lambda o: (
            -len(kb._facts_by_instance.get(o.name, ())),
            o.name,
        ),
    )
    seen: set[tuple[str, tuple]] = set()
    for obj in by_count:
        for h in _hypotheses_for(kb, obj):
            key = (h.relation_name, h.args)
            if key in seen:
                continue
            seen.add(key)
            yield h


def _hypotheses_for(kb: KnowledgeBase, obj) -> Iterator[Fact]:
    existing = {
        (f.relation_name, i)
        for f in kb._facts_by_instance.get(obj.name, ())
        for i, a in enumerate(f.args)
        if a == obj.name
    }
    for rel in kb.relations.values():
        if not rel.signature:
            continue
        for slot_idx, sig_type in enumerate(rel.signature):
            if not _type_compatible(kb, obj.type_name, sig_type):
                continue
            if (rel.name, slot_idx) in existing:
                continue
            yield from _fill_slot(kb, rel, slot_idx, obj)


def _type_compatible(kb: KnowledgeBase, obj_type: str, sig_type: str) -> bool:
    """True iff `obj_type` matches `sig_type` directly or via ancestry.

    Zebra-style: instances declare a specific type (Nationality,
    Color, …); the relation declares an ancestor (Attribute). The
    hypothesis generator needs to walk the type chain to admit such
    instances as fillers.
    """
    if obj_type == sig_type:
        return True
    t = kb.types.get(obj_type)
    if t is None:
        return False
    return any(a.name == sig_type for a in t.ancestors())


def _fill_slot(kb: KnowledgeBase, rel, fixed_slot: int, obj) -> Iterator[Fact]:
    """Enumerate type-compatible fillers; emit symmetric duplicates."""
    if len(rel.signature) != 2:
        return     # M1 only handles arity-2 relations
    other_slot = 1 - fixed_slot
    other_type = rel.signature[other_slot]
    symmetric = _is_symmetric(kb, rel.name)

    for filler in kb.instances.values():
        if filler.name == obj.name:
            continue        # skip self-edges
        if not _type_compatible(kb, filler.type_name, other_type):
            continue

        # Build args for the chosen slot assignment.
        args = _build_args(obj.name, fixed_slot, filler.name, other_slot)
        fact = Fact(
            relation_name=rel.name,
            args=args,
            layer=Layer.REASONING,
            provenance=None,    # caller adds Provenance.from_hypothesis later
        )
        if not _is_excluded(kb, fact):
            yield fact

        # Symmetric R: emit the reversed ordering too.
        if symmetric:
            rev_args = _build_args(filler.name, fixed_slot, obj.name, other_slot)
            rev = Fact(
                relation_name=rel.name,
                args=rev_args,
                layer=Layer.REASONING,
                provenance=None,
            )
            if not _is_excluded(kb, rev):
                yield rev


def _build_args(a_name: str, a_slot: int,
                b_name: str, b_slot: int) -> tuple[str, ...]:
    """Place two named values into a 2-tuple at the given slots."""
    args: list[str] = ["", ""]
    args[a_slot] = a_name
    args[b_slot] = b_name
    return tuple(args)


def _is_symmetric(kb: KnowledgeBase, r_name: str) -> bool:
    apps = kb._facts_by_relation.get("symmetric", ())
    return any(f.args == (r_name,) for f in apps)


def _is_excluded(kb: KnowledgeBase, fact: Fact) -> bool:
    """True iff `(not <fact>)` already exists in the KB."""
    for n in kb._facts_by_relation.get("not", ()):
        if not n.args:
            continue
        inner = n.args[0]
        if isinstance(inner, Fact) and (
            inner.relation_name == fact.relation_name
            and inner.args == fact.args
        ):
            return True
    return False


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


# ── Search tree (proof object) ─────────────────────────────────────


BranchId = int


@dataclass(frozen=True)
class SearchNode:
    """One node in the search tree — a fork point + its verdict.

    Each node represents *one branch* of exploration: the KB it
    ran on (post-saturation), the hypothesis fact that seeded it
    (None for the root), and a verdict:

    - ``solution`` — the branch's KB satisfies the query goal
      under the active mode. Constructive part of the proof.
    - ``dead`` — the branch contradicted; ``unsat_core`` carries
      the source-frontier facts that produced the conflict. The
      dead branch is part of the *uniqueness* proof for the
      surviving sibling(s).
    - ``open`` — interior node whose verdict is determined by its
      children. S1.5.3's alive-branch promotion settles these;
      S1.5.2 stamps them and lets the top-level driver inspect
      leaves directly.
    """
    id:          BranchId
    parent:      BranchId | None
    hypothesis:  Fact | None             # None for the root
    kb_snapshot: KnowledgeBase | None    # post-saturation; None after IR round-trip
    firings:     tuple[Firing, ...]
    verdict:     Literal["solution", "dead", "open"]
    children:    tuple[BranchId, ...] = ()
    unsat_core:  frozenset[Fact] = field(default_factory=frozenset)


@dataclass(frozen=True)
class SearchTree:
    """Immutable view of a completed search. Built by ``solve()``.

    Per S1.5.0 §F: the tree IS the proof. Dead branches are
    first-class artefacts witnessing uniqueness; the trace
    renderer (P1.6) serialises the whole tree.
    """
    root:  BranchId
    nodes: dict[BranchId, SearchNode]

    def solutions(self) -> tuple[SearchNode, ...]:
        return tuple(n for n in self.nodes.values()
                     if n.verdict == "solution")

    def dead_branches(self) -> tuple[SearchNode, ...]:
        return tuple(n for n in self.nodes.values()
                     if n.verdict == "dead")

    def open_branches(self) -> tuple[SearchNode, ...]:
        """Interior nodes whose verdict S1.5.3 hasn't promoted, OR
        leaf nodes that hit ``max_depth``."""
        return tuple(n for n in self.nodes.values()
                     if n.verdict == "open")

    # ── IR round-trip ─────────────────────────────────────────────

    def to_ir(self) -> SForm:
        """Serialise as a ``(trace …)`` SForm — DFS-ordered events.

        Each node becomes a `branch-open` + `branch-close` pair;
        recursive children are emitted between them. Verdicts go
        on `branch-close` as ``:verdict <atom>``. The hypothesis
        seed (when present) goes on `branch-open` as ``:on``.

        KB snapshots, firings, and unsat-core fact references are
        NOT round-tripped — the IR encodes the *tree shape* + the
        verdicts + the hypothesis seeds; per-branch state is the
        trace renderer's (P1.6) territory.
        """
        events: list[SForm] = []
        self._emit(self.root, events)
        return SForm(head=Atom(name="trace"), args=tuple(events))

    def _emit(self, nid: BranchId, events: list[SForm]) -> None:
        node = self.nodes[nid]
        open_args: list = [Atom(name=_branch_name(nid))]
        if node.hypothesis is not None:
            open_args.append(KwPair(
                key=Keyword(name="on"),
                value=_fact_to_sform(node.hypothesis),
            ))
        events.append(SForm(head=Atom(name="branch-open"),
                            args=tuple(open_args)))
        for child_id in node.children:
            self._emit(child_id, events)
        close_args = [
            Atom(name=_branch_name(nid)),
            KwPair(key=Keyword(name="verdict"),
                   value=Atom(name=node.verdict)),
        ]
        events.append(SForm(head=Atom(name="branch-close"),
                            args=tuple(close_args)))

    @classmethod
    def from_ir(cls, trace: SForm) -> SearchTree:
        """Reconstruct a SearchTree from a ``(trace …)`` SForm.

        kb_snapshot and firings are NOT reconstructed (the IR
        doesn't carry them); use the live tree from a fresh
        ``solve()`` if you need them.
        """
        if not (isinstance(trace, SForm)
                and isinstance(trace.head, Atom)
                and trace.head.name == "trace"):
            raise ValueError("expected (trace …) SForm")
        nodes: dict[BranchId, SearchNode] = {}
        stack: list[tuple[BranchId, list[BranchId]]] = []
        root_id: BranchId | None = None

        for event in trace.args:
            if not isinstance(event, SForm) or not isinstance(event.head, Atom):
                continue
            head = event.head.name
            if head == "branch-open":
                nid = _parse_branch_id(event.args[0])
                hyp = _parse_on_kw(event.args)
                parent = stack[-1][0] if stack else None
                if root_id is None:
                    root_id = nid
                if stack:
                    stack[-1][1].append(nid)
                # Store partial node now; finalize on branch-close.
                nodes[nid] = SearchNode(
                    id=nid, parent=parent, hypothesis=hyp,
                    kb_snapshot=None, firings=(),
                    verdict="open", children=(),
                )
                stack.append((nid, []))
            elif head == "branch-close":
                if not stack:
                    raise ValueError("unmatched branch-close")
                nid, children_ids = stack.pop()
                close_nid = _parse_branch_id(event.args[0])
                if close_nid != nid:
                    raise ValueError(
                        f"branch-close {close_nid} doesn't match open {nid}"
                    )
                verdict = _parse_verdict_kw(event.args)
                prev = nodes[nid]
                nodes[nid] = SearchNode(
                    id=prev.id, parent=prev.parent,
                    hypothesis=prev.hypothesis,
                    kb_snapshot=None, firings=(),
                    verdict=verdict, children=tuple(children_ids),
                )
            # other events (step, contradiction, symmetry-class) ignored.

        if root_id is None:
            raise ValueError("empty trace — no root branch")
        if stack:
            raise ValueError("unbalanced branch-open / close")
        return cls(root=root_id, nodes=nodes)


def _branch_name(nid: BranchId) -> str:
    return f"b{nid}"


def _fact_to_sform(fact: Fact) -> SForm:
    """Convert a Fact to an IR SForm — recursive for nested args."""
    arg_nodes = []
    for a in fact.args:
        if isinstance(a, Fact):
            arg_nodes.append(_fact_to_sform(a))
        elif isinstance(a, int):
            arg_nodes.append(Int(value=a))
        else:
            arg_nodes.append(Atom(name=str(a)))
    return SForm(head=Atom(name=fact.relation_name), args=tuple(arg_nodes))


def _sform_to_fact(node: SForm) -> Fact:
    """Reverse of `_fact_to_sform` — recursive for nested SForms."""
    if not isinstance(node.head, Atom):
        raise ValueError(f"expected Atom head, got {type(node.head).__name__}")
    args: list = []
    for a in node.args:
        if isinstance(a, Atom):
            args.append(a.name)
        elif isinstance(a, Int):
            args.append(a.value)
        elif isinstance(a, SForm) and isinstance(a.head, Atom):
            args.append(_sform_to_fact(a))
    return Fact(
        relation_name=node.head.name,
        args=tuple(args),
        layer=Layer.REASONING,
    )


def _parse_branch_id(node) -> BranchId:
    if not isinstance(node, Atom):
        raise ValueError(f"expected Atom branch id, got {type(node).__name__}")
    name = node.name
    if not name.startswith("b") or not name[1:].isdigit():
        raise ValueError(f"branch id must be `b<int>`, got {name!r}")
    return int(name[1:])


def _parse_on_kw(args) -> Fact | None:
    for a in args:
        if isinstance(a, KwPair) and a.key.name == "on":
            value = a.value
            if isinstance(value, SForm) and isinstance(value.head, Atom):
                return _sform_to_fact(value)
    return None


def _parse_verdict_kw(args) -> str:
    for a in args:
        if isinstance(a, KwPair) and a.key.name == "verdict":
            value = a.value
            if isinstance(value, Atom):
                return value.name
    return "open"


# ── Tree builder (internal) ────────────────────────────────────────


class _TreeBuilder:
    """Append-only builder for SearchNodes during recursive descent."""

    def __init__(self) -> None:
        self._next_id: BranchId = 0
        self.nodes: dict[BranchId, SearchNode] = {}

    def alloc(self) -> BranchId:
        nid = self._next_id
        self._next_id += 1
        return nid

    def peek_id(self) -> BranchId:
        return self._next_id

    def add(self, node: SearchNode) -> None:
        self.nodes[node.id] = node

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
    stamped ``open``; S1.5.3 will promote them via the alive-branch
    protocol.
    """
    contradictions = ContradictionDetector(kb).detect()
    if contradictions:
        nid = builder.alloc()
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
        builder.add(SearchNode(
            id=nid, parent=parent_id, hypothesis=hypothesis,
            kb_snapshot=kb, firings=firings,
            verdict="solution", children=(),
        ))
        return nid

    if depth >= max_depth:
        nid = builder.alloc()
        builder.add(SearchNode(
            id=nid, parent=parent_id, hypothesis=hypothesis,
            kb_snapshot=kb, firings=firings,
            verdict="open", children=(),
        ))
        return nid

    # Interior: allocate id, then recurse into each child.
    interior_id = builder.alloc()
    child_ids: list[BranchId] = []
    for h in generate_hypotheses(kb):
        result = try_branch(kb, h, branch_id=builder.peek_id())
        if result.is_alive():
            child_id = _explore(
                result.kb, interior_id, h, result.firings,
                depth + 1, max_depth, builder, mode,
            )
        else:
            child_id = builder.alloc()
            builder.add(SearchNode(
                id=child_id, parent=interior_id, hypothesis=h,
                kb_snapshot=result.kb, firings=result.firings,
                verdict="dead", children=(),
                unsat_core=result.unsat_core,
            ))
        child_ids.append(child_id)

    builder.add(SearchNode(
        id=interior_id, parent=parent_id, hypothesis=hypothesis,
        kb_snapshot=kb, firings=firings,
        verdict="open", children=tuple(child_ids),
    ))
    return interior_id


# ── Top-level solve driver (S1.5.2) ────────────────────────────────


def solve(
    kb: KnowledgeBase,
    *,
    mode: Mode | None = None,
    max_depth: int = 6,
) -> Verdict:
    """Multilevel hypothesis search — saturate, generate, recurse.

    Builds the full SearchTree (proof object). Returns:

    - ``Solution`` if exactly one branch reaches a ``solution``
      leaf under SOLVE mode (all surviving leaves under GAPS).
    - ``Contradiction`` if zero branches survive (the dead-branch
      unsat-cores certify unsolvability).
    - ``Ambiguity`` if multiple solution-leaves exist under SOLVE,
      or any open-leaves exist (max_depth hit).

    The returned Verdict carries the SearchTree on its ``tree``
    attribute. P1.6 reads it to render the proof.

    S1.5.3 adds canonical-state-hash dedup + alive-branch verdict
    promotion (interior nodes' verdicts inherit from descendants).
    """
    mode = mode or _mode_from_query(kb) or Mode.SOLVE

    # Initial saturation on the root KB.
    sat = Saturator(kb)
    root_firings = tuple(sat.saturate(max_steps=10_000))

    builder = _TreeBuilder()
    root_id = _explore(kb, None, None, root_firings,
                       depth=0, max_depth=max_depth,
                       builder=builder, mode=mode)
    tree = builder.finalize(root_id)

    solutions = tree.solutions()
    dead = tree.dead_branches()
    opens = tree.open_branches()

    if mode is Mode.SOLVE:
        if len(solutions) == 1:
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
            tree=tree,
        )

    if mode is Mode.GAPS:
        return Ambiguity(
            branches=tuple(
                Solution(kb=s.kb_snapshot, trace=s.firings, tree=tree)
                for s in solutions
            ),
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
    "try_branch",
]
