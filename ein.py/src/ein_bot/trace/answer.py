"""English answer rendering for the sound ``solve()`` verdict (P1.7a S1.7a.6).

Turns a `solve()` :class:`~ein_bot.inference.verdict.Verdict` into a one-line
natural-language answer for the CLI ``solve --mode=solve`` path. The query
``:goal`` gives the *where* (house vars); the *who* is projected by reading
``nation-loc`` at each bound house (the S1.7.5 who-vs-where note). Thin and
puzzle-shaped on purpose: the relation→verb vocab is the only domain-specific
piece — the full `:ask` NL surface is M2.
"""
from __future__ import annotations

from ein_bot.inference.canon import state_hash
from ein_bot.inference.verdict import (
    Ambiguity,
    Contradiction,
    Solution,
    Verdict,
    _query_value,
    goal_bindings,
)
from ein_bot.ir.types import Atom, SForm, Var

#: Render ``(R anchor house)`` as "the <who> <verb> the <anchor>".
_VERB = {
    "drink-loc": "drinks",
    "pet-loc": "keeps",
    "smoke-loc": "smokes",
    "nation-loc": "is",
}


def _humanise(name: str) -> str:
    return name.replace("_", " ").lower()


def _conjuncts(goal) -> list:
    if (isinstance(goal, SForm) and isinstance(goal.head, Atom)
            and goal.head.name == "and"):
        return list(goal.args)
    return [goal]


def _nation_at(kb, house: str) -> str | None:
    for f in kb._facts_by_relation.get("nation-loc", ()):
        if len(f.args) == 2 and f.args[1] == house:
            return f.args[0]
    return None


def _solution_answer(kb, *, exhausted: bool) -> str:
    goal = _query_value(kb.query, "goal") if kb.query is not None else None
    rows = goal_bindings(kb, goal)
    if goal is None or not rows:
        return "Solved (no query goal to project)."
    b = rows[0]
    parts: list[str] = []
    for conj in _conjuncts(goal):
        if not (isinstance(conj, SForm) and isinstance(conj.head, Atom)
                and len(conj.args) == 2):
            continue
        rel = conj.head.name
        anchor_node, house_node = conj.args
        anchor = anchor_node.name if isinstance(anchor_node, Atom) else None
        hvar = house_node.name if isinstance(house_node, Var) else None
        house = b.get(hvar) if hvar else None
        if anchor is None or house is None:
            continue
        who = _nation_at(kb, house)
        verb = _VERB.get(rel)
        if who and verb:
            parts.append(f"the {who} {verb} the {_humanise(anchor)}")
        else:
            parts.append(f"{_humanise(anchor)} is in {house}")
    if not parts:
        return "Solved."
    sentence = "; ".join(parts)
    sentence = sentence[0].upper() + sentence[1:] + "."
    if not exhausted:
        sentence += "  (a solution — pass --exhaustive to certify uniqueness)"
    return sentence


def render_answer(verdict: Verdict, *, exhausted: bool = True) -> str:
    """One-line English answer for a `solve()` verdict."""
    if isinstance(verdict, Solution):
        return _solution_answer(verdict.kb, exhausted=exhausted)
    if isinstance(verdict, Ambiguity):
        k = (len({state_hash(b.kb) for b in verdict.branches})
             or len(verdict.branches))
        return (f"Ambiguous — {k} distinct complete models; the puzzle is "
                f"under-determined.")
    if isinstance(verdict, Contradiction):
        srcs = sorted({
            s for f in verdict.unsat_core
            if (s := getattr(f.provenance, "source", None))
        })
        core = ", ".join(srcs) if srcs else f"{len(verdict.unsat_core)} facts"
        return ("No solution — the constraints are contradictory "
                f"(unsat core: {core}).")
    return f"Unexpected verdict: {type(verdict).__name__}"


__all__ = ["render_answer"]
