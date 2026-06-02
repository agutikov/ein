"""Verdict types + Mode + is_solved — the shared surface across the
inference engine entries (``solve``, ``gaps_solve``,
``contradictions_solve``).

Migrated 2026-05-29 out of ``inference.tree.solver`` as part of the
tree-solver removal that closes P1.5b. The tree-side ``SearchTree``
/ ``SearchNode`` types — and their ``tree`` / ``unresolved`` fields
on the verdict classes — were dropped in the move; the lattice
engines never populated them and the P1.6 NL renderer consumes
``proof: LatticeProof`` instead.

What survives:

- :class:`Mode` — the three task classes from idea 03
  (SOLVE / GAPS / CONTRADICTIONS).
- :class:`Solution`, :class:`Ambiguity`, :class:`Contradiction` —
  the three verdict shapes. Each carries an optional
  ``proof: LatticeProof | None`` that gaps_solve /
  contradictions_solve attach.
- :data:`Verdict` — the union type.
- :func:`is_solved` — goal-pattern check against a kb (mode-aware:
  SOLVE expects exactly one binding, GAPS expects at least one,
  CONTRADICTIONS never satisfies).
- :func:`query_value` — small Query accessor used by the engine
  (goal projection, the CLI answer path).

The tree-side ``.tree`` / ``.unresolved`` verdict fields were dropped in
that move; the lattice engines never populated them.
"""
from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import TYPE_CHECKING

from ein_bot.kb.entities import Fact
from ein_bot.kb.store import KnowledgeBase

from .compile import JoinPlan, compile_pattern
from .firing import Firing
from .match import run as match_run

if TYPE_CHECKING:
    # Forward-only — runtime import would form a cycle
    # (monotonic.lattice → inference.verdict via Verdict types).
    from .monotonic.lattice import LatticeProof


# ── Mode ──────────────────────────────────────────────────────────


class Mode(Enum):
    """What the loop reports at quiescence (idea 03's three task classes)."""
    SOLVE          = "solve"
    GAPS           = "gaps"
    CONTRADICTIONS = "contradictions"


# ── Verdicts ──────────────────────────────────────────────────────


@dataclass(frozen=True)
class Solution:
    """A surviving branch: KB satisfies the query goal (mode-aware).

    ``proof`` is the optional :class:`LatticeProof` attached by the
    set-search engine's lattice entries
    (:func:`gaps_solve`, :func:`contradictions_solve`).
    :func:`solve` leaves it None.
    """

    kb:    KnowledgeBase
    trace: tuple[Firing, ...]
    proof: LatticeProof | None = None


@dataclass(frozen=True)
class Ambiguity:
    """Multiple surviving branches — GAPS mode's normal verdict.

    Also returned by :func:`solve` when it finds ``k > 1`` distinct
    solution nodes (a genuine ambiguity / gap).
    ``proof`` is the optional :class:`LatticeProof` returned by
    :func:`gaps_solve`.
    """

    branches: tuple[Solution, ...]
    proof:    LatticeProof | None = None


@dataclass(frozen=True)
class Contradiction:
    """No surviving branch — the puzzle is unsolvable under the
    given constraints. ``unsat_core`` is the source-frontier facts
    that jointly produce the conflict; ``proof`` is the optional
    :class:`LatticeProof` returned by :func:`contradictions_solve`.
    """

    unsat_core: frozenset[Fact] = frozenset()
    proof:      LatticeProof | None = None


Verdict = Solution | Ambiguity | Contradiction


# ── Goal check ────────────────────────────────────────────────────


def goal_bindings(kb: KnowledgeBase, goal=None) -> list[dict[str, str]]:
    """Run the query ``:goal`` pattern against ``kb``; return the binding
    rows (``var -> value``).

    Same matcher machinery :func:`is_solved` uses to *count* matches — here
    the rows are returned so callers can project an answer (the CLI
    ``--mode=solve`` path, P1.7a S1.7a.6). ``goal`` defaults to the kb's own
    ``(query :goal …)``; pass an explicit goal pattern to project a different
    question (e.g. ``(nation-loc ?who <house>)``) over a solved model. Values
    are bare strings (fact args are stored unwrapped).
    """
    if goal is None:
        if kb.query is None:
            return []
        goal = query_value(kb.query, "goal")
    if goal is None:
        return []
    plan = JoinPlan(
        rule_name="<query>",
        activator_args=(),
        bindings_seed={},
        steps=tuple(compile_pattern(goal, {})),
        assert_template=None,
        why="",
    )
    return [dict(b) for b, _premises in match_run(plan, kb)]


def is_solved(kb: KnowledgeBase, mode: Mode) -> bool:
    """Has the KB satisfied the query goal under ``mode``?

    SOLVE — exactly one binding satisfies the goal pattern.
    GAPS  — at least one binding satisfies the goal pattern.
    CONTRADICTIONS — never solved (runs to exhaustion).
    """
    if mode is Mode.CONTRADICTIONS:
        return False
    n = len(goal_bindings(kb))
    if mode is Mode.SOLVE:
        return n == 1
    if mode is Mode.GAPS:
        return n >= 1
    return False


def query_value(query, kw_name: str):
    """Look up a kw_pair value by keyword name on a Query."""
    for kp in query.kw_pairs:
        if hasattr(kp, "key") and kp.key.name == kw_name:
            return kp.value
    return None


__all__ = [
    "Ambiguity",
    "Contradiction",
    "Mode",
    "Solution",
    "Verdict",
    "goal_bindings",
    "is_solved",
    "query_value",
]
