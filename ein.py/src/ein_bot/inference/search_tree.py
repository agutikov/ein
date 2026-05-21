"""Search tree as proof object — S1.5.2 plus S1.5.3's DAG support.

The hypothesis loop's *output*: a tree (DAG after dedup) of
SearchNodes, one per visited KB state, recording the proof shape:

- ``solution`` leaves — the distinct goal-satisfying KB states;
- ``dead`` leaves — branches contradicted; their `unsat_core` is a
  constructive witness of the surviving sibling's uniqueness;
- ``open`` leaves — max-depth cutoffs;
- interior nodes — verdict promoted by `_promote_verdicts`
  (S1.5.3) so the root carries the overall verdict.

Per [S1.5.0 §F](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.0_review.md#f):
**the tree IS the proof**. Dead branches are first-class artefacts;
the trace renderer (P1.6) serialises the whole tree.

This module owns the `SearchNode` / `SearchTree` dataclasses and
the `(trace …)` IR round-trip (`to_ir` / `from_ir`). The recursive
descent that *builds* the tree lives in `hypothesis.py`.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal

from ein_bot.ir.types import Atom, Int, Keyword, KwPair, SForm
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

from .firing import Firing

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
      children, OR a leaf hit by `max_depth`.

    Under S1.5.3 dedup the tree is a DAG in storage — multiple
    parents can point to the same SearchNode. The IR round-trip
    handles this with `(branch-ref bN)` events.
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

    `solutions` / `dead_branches` / `open_branches` return LEAVES
    only (post-S1.5.3 dedup, interior verdicts are inferred from
    leaves via verdict promotion).
    """
    root:  BranchId
    nodes: dict[BranchId, SearchNode]

    def solutions(self) -> tuple[SearchNode, ...]:
        """*Deepest* solution markers — the distinct answer states.

        A node is a solution-endpoint iff its verdict is ``solution``
        AND none of its direct children have verdict ``solution``.
        Catches both:
          - true leaves where `generate_hypotheses` returned nothing
            and `is_solved` was true;
          - interior nodes whose children all turned out dead but
            whose own state matches the goal (the "no consistent
            extension, but the state itself is the answer" case from
            `_promote_verdicts`).

        Excludes interior nodes whose ``solution`` verdict is purely
        propagated from a deeper descendant — those aren't endpoints,
        just path markers.
        """
        return tuple(
            n for n in self.nodes.values()
            if n.verdict == "solution"
            and not any(
                self.nodes[c].verdict == "solution"
                for c in n.children
            )
        )

    def dead_branches(self) -> tuple[SearchNode, ...]:
        """Leaf nodes that contradicted — the uniqueness witnesses."""
        return tuple(n for n in self.nodes.values()
                     if n.verdict == "dead" and not n.children)

    def open_branches(self) -> tuple[SearchNode, ...]:
        """Leaf nodes that hit ``max_depth`` without resolving."""
        return tuple(n for n in self.nodes.values()
                     if n.verdict == "open" and not n.children)

    # ── IR round-trip ─────────────────────────────────────────────

    def to_ir(self) -> SForm:
        """Serialise as a ``(trace …)`` SForm — DFS-ordered events.

        Each node becomes a `branch-open` + `branch-close` pair;
        recursive children are emitted between them. Verdicts go
        on `branch-close` as ``:verdict <atom>``. The hypothesis
        seed (when present) goes on `branch-open` as ``:on``.

        DAG-shared nodes (S1.5.3 dedup): the first visit emits the
        full subtree; subsequent visits emit `(branch-ref bN)`.

        KB snapshots, firings, and unsat-core fact references are
        NOT round-tripped — the IR encodes the *tree shape* + the
        verdicts + the hypothesis seeds; per-branch state is the
        trace renderer's (P1.6) territory.
        """
        events: list[SForm] = []
        emitted: set[BranchId] = set()
        self._emit(self.root, events, emitted)
        return SForm(head=Atom(name="trace"), args=tuple(events))

    def _emit(self, nid: BranchId, events: list[SForm],
              emitted: set[BranchId]) -> None:
        if nid in emitted:
            # DAG-shared node: emit a single branch-ref instead of
            # repeating the subtree (S1.5.3 dedup may have multiple
            # parents pointing to the same node).
            events.append(SForm(
                head=Atom(name="branch-ref"),
                args=(Atom(name=_branch_name(nid)),),
            ))
            return
        emitted.add(nid)
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
            self._emit(child_id, events, emitted)
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

        `kb_snapshot` and `firings` are NOT reconstructed (the IR
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
            elif head == "branch-ref":
                # Shared DAG node — append the existing nid to the
                # current parent's children list without creating a
                # new SearchNode entry.
                if not stack:
                    raise ValueError("branch-ref outside of any branch")
                ref_nid = _parse_branch_id(event.args[0])
                stack[-1][1].append(ref_nid)
            # other events (step, contradiction, symmetry-class) ignored.

        if root_id is None:
            raise ValueError("empty trace — no root branch")
        if stack:
            raise ValueError("unbalanced branch-open / close")
        return cls(root=root_id, nodes=nodes)


# ── Internal helpers (also used by hypothesis.py) ─────────────────


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


__all__ = [
    "BranchId",
    "SearchNode",
    "SearchTree",
]
