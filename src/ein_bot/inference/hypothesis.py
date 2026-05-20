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
    """A surviving branch: KB satisfies the query goal (mode-aware)."""
    kb:    KnowledgeBase
    trace: tuple[Firing, ...]


@dataclass(frozen=True)
class Ambiguity:
    """Multiple surviving branches — GAPS mode's normal verdict."""
    branches: tuple[Solution, ...]


@dataclass(frozen=True)
class Contradiction:
    """No surviving branch — the puzzle is unsolvable under the
    given constraints. `unsat_core` is the source-frontier facts
    that jointly produce the conflict."""
    unsat_core: frozenset[Fact] = frozenset()


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
        for slot_idx, type_name in enumerate(rel.signature):
            if type_name != obj.type_name:
                continue
            if (rel.name, slot_idx) in existing:
                continue
            yield from _fill_slot(kb, rel, slot_idx, obj)


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
        if filler.type_name != other_type:
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


# ── Single-level solve driver (S1.5.1 scope) ───────────────────────


def solve(
    kb: KnowledgeBase,
    *,
    mode: Mode | None = None,
    max_depth: int = 6,        # plumbed for S1.5.2's recursion
) -> Verdict:
    """One-level hypothesis loop: saturate, try each hypothesis once.

    S1.5.2 wraps this in a recursive descent (the full proof-object
    search tree). S1.5.3 adds canonicalisation + dedup + alive-
    branch termination.

    `max_depth` is accepted but ignored at this stage — S1.5.2's
    recursion respects it. Passing it now keeps the signature
    stable across stages.
    """
    _ = max_depth
    mode = mode or _mode_from_query(kb) or Mode.SOLVE

    sat = Saturator(kb)
    firings = list(sat.saturate(max_steps=10_000))

    contradictions = ContradictionDetector(kb).detect()
    if contradictions:
        unsat = kb.unsat_core(c.positive for c in contradictions)
        return Contradiction(unsat_core=frozenset(unsat))

    if is_solved(kb, mode):
        return Solution(kb=kb, trace=tuple(firings))

    if mode is Mode.CONTRADICTIONS:
        # At single-level: no contradictions surfaced, no branches
        # explored — exhaustion is trivially "nothing".
        return Contradiction(unsat_core=frozenset())

    survivors: list[Solution] = []
    branch_id = 0
    for h in generate_hypotheses(kb):
        branch_id += 1
        result = try_branch(kb, h, branch_id=branch_id)
        if not result.is_alive():
            continue
        if is_solved(result.kb, mode):
            survivors.append(Solution(kb=result.kb, trace=result.firings))

    if mode is Mode.SOLVE:
        if len(survivors) == 1:
            return survivors[0]
        if len(survivors) == 0:
            return Contradiction(unsat_core=frozenset())
        return Ambiguity(branches=tuple(survivors))

    # GAPS: report all survivors.
    return Ambiguity(branches=tuple(survivors))


__all__ = [
    "Ambiguity",
    "BranchResult",
    "Contradiction",
    "Mode",
    "Solution",
    "Verdict",
    "generate_hypotheses",
    "is_solved",
    "solve",
    "try_branch",
]
