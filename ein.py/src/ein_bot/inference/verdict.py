"""Verdict types + Mode + is_solved — the shared surface across the
inference engine entries (``monotonic_solve``, ``gaps_solve``,
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
- :func:`_query_value` / :func:`_mode_from_query` — small Query
  accessors used by the engine + by is_solved.

What was dropped (with the tree solver):

- ``Solution.tree`` / ``Ambiguity.tree`` / ``Contradiction.tree``
  (held a ``SearchTree | None`` produced by the tree solver).
- ``Ambiguity.unresolved`` (held ``tuple[SearchNode, ...]`` of
  depth-cap open leaves — a tree-search concept).

The lattice engines never populated these fields; their migration
from tree.solver leaves them out by design.
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
    set-search engine's non-monotonic entries
    (:func:`gaps_solve`, :func:`contradictions_solve`).
    :func:`monotonic_solve` leaves it None.
    """

    kb:    KnowledgeBase
    trace: tuple[Firing, ...]
    proof: LatticeProof | None = None


@dataclass(frozen=True)
class Ambiguity:
    """Multiple surviving branches — GAPS mode's normal verdict.

    Also returned by :func:`monotonic_solve` when the depth cap is
    reached with non-empty alive set and no surviving solution.
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


def is_solved(kb: KnowledgeBase, mode: Mode) -> bool:
    """Has the KB satisfied the query goal under ``mode``?

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


__all__ = [
    "Ambiguity",
    "Contradiction",
    "Mode",
    "Solution",
    "Verdict",
    "is_solved",
]
