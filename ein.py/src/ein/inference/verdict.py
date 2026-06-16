"""Verdict types + Mode + is_solved — the shared surface for the
single inference engine entry (:func:`solve`).

Migrated 2026-05-29 out of ``inference.tree.solver`` (P1.5b). The
tree-side ``SearchTree`` / ``SearchNode`` types and their ``tree`` /
``unresolved`` verdict fields were dropped; the NL renderer consumes
``proof: LatticeProof`` instead.

What survives:

- :class:`Mode` — the three task classes from idea 03
  (SOLVE / GAPS / CONTRADICTIONS).
- :class:`Solution`, :class:`Ambiguity`, :class:`Contradiction` —
  the three verdict shapes (the three *answers* to one problem, not
  three problem statements). Each carries an optional
  ``proof: LatticeProof | None`` that :func:`solve` attaches when
  called with ``store_lattice=True``.
- :data:`Verdict` — the union type.
- :func:`is_solved` — goal-pattern check against a kb (mode-aware:
  SOLVE expects exactly one binding, GAPS expects at least one,
  CONTRADICTIONS never satisfies).
- :func:`query_value` — small Query accessor used by the engine
  (goal projection, the CLI answer path).
"""
from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import TYPE_CHECKING

from ein.kb.entities import Fact
from ein.kb.store import KnowledgeBase

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

    ``proof`` is the optional :class:`LatticeProof` attached by
    :func:`solve` when called with ``store_lattice=True``; it is None
    on the default fast path.
    """

    kb:    KnowledgeBase
    trace: tuple[Firing, ...]
    proof: LatticeProof | None = None


@dataclass(frozen=True)
class Ambiguity:
    """Multiple surviving branches — the verdict :func:`solve` returns
    when it finds ``k > 1`` distinct solution nodes (a genuine
    ambiguity / gap).

    ``proof`` is the optional :class:`LatticeProof` attached by
    :func:`solve` when called with ``store_lattice=True`` (its
    ``proof.solutions`` is the gaps view).
    """

    branches: tuple[Solution, ...]
    proof:    LatticeProof | None = None


@dataclass(frozen=True)
class Contradiction:
    """No surviving branch — the puzzle is unsolvable under the
    given constraints. ``unsat_core`` is the source-frontier facts
    that jointly produce the conflict; ``proof`` is the optional
    :class:`LatticeProof` attached by :func:`solve` when called with
    ``store_lattice=True`` (its ``proof.dead_commitments`` +
    ``unsat_core`` are the contradictions view).
    """

    unsat_core: frozenset[Fact] = frozenset()
    proof:      LatticeProof | None = None


@dataclass(frozen=True)
class Aborted:
    """Search cut short by a budget (``max_enterings`` / ``max_time``) before it
    completed — **not** a proven verdict (S1.9.E17). ``reason`` is the abort
    message; ``stats`` is the partial run (``stats.solution_nodes`` is a lower
    bound, ``stats.exhausted`` is False). Distinct from :class:`Contradiction`:
    ``solution_nodes == 0`` here means *unexplored*, not *proven unsatisfiable*.

    Returned by :func:`solve` only when called with ``on_budget="verdict"``;
    the default ``on_budget="raise"`` keeps raising ``BudgetExceededError``
    instead. Kept **out** of the :data:`Verdict` union so exhaustive verdict
    handling is unaffected — an opt-in caller matches ``Aborted`` explicitly.
    """

    reason: str
    stats:  object = None     # MonotonicStats — `object` dodges the import cycle


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
        assert_templates=(),
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
    "Aborted",
    "Ambiguity",
    "Contradiction",
    "Mode",
    "Solution",
    "Verdict",
    "goal_bindings",
    "is_solved",
    "query_value",
]
