"""Solve-result rendering for the CLI ``solve`` table.

**No hardcoded vocabulary.** Every word of English in the output comes from
*puzzle-authored* templates — there is no relation→verb table in this module.
Two template sources drive the text (both reuse the rule ``:why`` engine,
:func:`ein.inference.why.render_why`):

- **per-relation** — ``(relation R T1 T2 … :why "<tmpl>")`` renders ONE fact of
  ``R``, with ``{?1}`` / ``{?2}`` bound to the fact's args *positionally*. This
  drives the table's *rendered query facts* column. A relation **without** a
  ``:why`` template renders as its raw IR s-expression ``(R a b)`` — never any
  invented prose.
- **per-query** — ``(query … :goal-text "<tmpl>")`` renders the headline
  *NL result*, with the goal's own vars (``{?who_water}``, ``{?h_zebra}``, …)
  bound from the solution. Absent, the result line is omitted (the rendered
  facts above already carry the answer).

:func:`render_solution_table` assembles the five fields the CLI prints —
``solutions (k)`` · ``verdict`` · ``query bindings`` · ``rendered query
facts`` · ``NL result`` — for each verdict shape (Solution / Ambiguity /
Contradiction). :func:`render_answer` is the one-line headline used as the
result row (and kept for callers that want just the sentence).
"""
from __future__ import annotations

from ein.inference.canon import state_hash
from ein.inference.verdict import (
    Ambiguity,
    Contradiction,
    Solution,
    Verdict,
    goal_bindings,
    query_value,
)
from ein.inference.why import render_why
from ein.ir.types import Atom, Int, SForm, String, Var

# ── Fact / conjunct rendering ──────────────────────────────────────


def _sexpr(relation_name: str, args) -> str:
    """A fact as a flat IR s-expression — the no-template fallback."""
    inner = " ".join(str(a) for a in args)
    return f"({relation_name} {inner})" if inner else f"({relation_name})"


def _conjuncts(goal) -> list:
    """The conjuncts of a goal — unwrap a top-level ``(and …)``."""
    if (isinstance(goal, SForm) and isinstance(goal.head, Atom)
            and goal.head.name == "and"):
        return list(goal.args)
    return [goal] if goal is not None else []


def _ground(conj, b: dict[str, str]):
    """``(rel, ground_args)`` for one goal conjunct under bindings ``b``.

    Vars resolve to their bound value (an unbound var keeps its ``?name``
    text); Atoms / Ints stay literal. Returns ``None`` for a non-fact
    conjunct (e.g. a nested ``(and …)``).
    """
    if not (isinstance(conj, SForm) and isinstance(conj.head, Atom)):
        return None
    out: list[str] = []
    for a in conj.args:
        if isinstance(a, Var):
            out.append(str(b.get(a.name, f"?{a.name}")))
        elif isinstance(a, Atom):
            out.append(a.name)
        elif isinstance(a, Int):
            out.append(str(a.value))
        else:
            out.append(str(a))
    return conj.head.name, tuple(out)


def _render_fact(kb, relation_name: str, args) -> str:
    """Render one ground fact to text via the relation's ``:why`` template,
    or fall back to its IR s-expression when the relation has no template."""
    rel = kb.relations.get(relation_name) if kb is not None else None
    if rel is not None and rel.why:
        slots = {str(i + 1): str(a) for i, a in enumerate(args)}
        return render_why(rel.why, slots)
    return _sexpr(relation_name, args)


def _goal_text(kb, b: dict[str, str]) -> str | None:
    """The query ``:goal-text`` template rendered under bindings ``b``."""
    if kb is None or kb.query is None:
        return None
    node = query_value(kb.query, "goal-text")
    if not isinstance(node, String):
        return None
    return render_why(node.value, b)


def _query_goal(kb):
    return query_value(kb.query, "goal") if kb is not None and kb.query else None


# ── Headline (the NL-result row) ───────────────────────────────────


def render_answer(verdict: Verdict, *, exhausted: bool = True) -> str:
    """One-line headline for a ``solve()`` verdict.

    Solution → the ``:goal-text`` rendered against the solution bindings
    (or, with no ``:goal-text``, a neutral "Solved." — no invented prose).
    Ambiguity / Contradiction keep their stable wording ("Ambiguous …" /
    "No solution …").
    """
    if isinstance(verdict, Solution):
        rows = goal_bindings(verdict.kb, _query_goal(verdict.kb))
        text = _goal_text(verdict.kb, rows[0]) if rows else None
        if text is None:
            return "Solved."
        if not exhausted:
            text += "  (a solution — pass --exhaustive to certify uniqueness)"
        return text
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


# ── The five-field table ───────────────────────────────────────────


def _rule(width: int = 62) -> str:
    return "─" * width


def _two_col(
    rows: list[tuple[str, str]], *, indent: str = "    ",
    header: tuple[str, str] | None = None,
) -> list[str]:
    """Left-aligned two-column block; col-1 width fits the widest entry
    (and the header label, so an optional ``header`` row stays aligned)."""
    if not rows:
        return []
    w = max(len(a) for a, _ in rows)
    out: list[str] = []
    if header is not None:
        w = max(w, len(header[0]))
        out.append(f"{indent}{header[0]:<{w}}  {header[1]}".rstrip())
    out += [f"{indent}{a:<{w}}  {b}".rstrip() for a, b in rows]
    return out


def _solution_block(kb, *, header: str = "") -> list[str]:
    """The bindings + rendered-facts + result sections for one model."""
    goal = _query_goal(kb)
    rows = goal_bindings(kb, goal)
    out: list[str] = []
    if header:
        out.append(f"  {header}")
    if not rows:
        out.append("    (no query goal to project)")
        return out
    b = rows[0]

    # query bindings — sorted by var name for deterministic output
    out.append("  query bindings")
    out += _two_col([(f"?{k}", f"= {v}") for k, v in sorted(b.items())])
    out.append("")

    # rendered query facts: ground each conjunct, render via relation :why
    grounds = [g for c in _conjuncts(goal) if (g := _ground(c, b)) is not None]
    fact_rows = [(_sexpr(rel, args), _render_fact(kb, rel, args))
                 for rel, args in grounds]
    out += _two_col(fact_rows, header=("query facts", "rendered"))
    out.append("")

    # NL result — from :goal-text, or omitted (facts above carry the answer)
    text = _goal_text(kb, b)
    out.append("  result")
    if text is None:
        out.append("    (query has no :goal-text template)")
    else:
        out.append(f"    {text}")
    return out


def render_solution_table(
    verdict: Verdict, stats, *, exhausted: bool = True, source: str | None = None,
) -> str:
    """The full ``solve`` table: solutions (k), verdict, and — per verdict
    shape — the query bindings, rendered query facts, and NL result.

    All text is rendered from puzzle data (per-relation ``:why`` +
    per-query ``:goal-text``); this function contributes only the field
    labels and layout, never domain vocabulary.
    """
    k = getattr(stats, "solution_nodes", None)
    lines: list[str] = []
    title = f"solve · {source}" if source else "solve"
    lines.append(title)
    lines.append(_rule())

    if isinstance(verdict, Solution):
        cert = "" if exhausted else "   (not certified — pass --exhaustive)"
        lines.append(f"  solutions (k)   {k if k is not None else 1}{cert}")
        lines.append("  verdict         Solution")
        lines.append("")
        lines += _solution_block(verdict.kb)

    elif isinstance(verdict, Ambiguity):
        kk = (len({state_hash(b.kb) for b in verdict.branches})
              or len(verdict.branches))
        lines.append(f"  solutions (k)   {kk}")
        lines.append("  verdict         Ambiguous — distinct complete models; "
                     "the puzzle is under-determined")
        for i, branch in enumerate(verdict.branches, 1):
            lines.append("")
            lines += _solution_block(branch.kb, header=f"model {i}/{len(verdict.branches)}")

    elif isinstance(verdict, Contradiction):
        lines.append("  solutions (k)   0")
        lines.append("  verdict         No solution — the constraints are "
                     "contradictory")
        core = sorted(verdict.unsat_core,
                      key=lambda f: (f.relation_name, tuple(map(str, f.args))))
        # Source frontier — the given conditions (`:source`) that jointly
        # force the conflict. The human-meaningful "which inputs clash"; the
        # raw fact list below is the full core.
        srcs = sorted({s for f in core
                       if (s := getattr(f.provenance, "source", None))})
        if srcs:
            lines.append(f"  conflicting sources: {', '.join(srcs)}")
        lines.append("")
        lines.append(f"  unsat core ({len(core)} facts)")
        # Render each core fact through its relation :why (or IR fallback).
        kb = core[0]._kb if core else None
        lines += _two_col([
            (_sexpr(f.relation_name, f.args),
             _render_fact(kb or f._kb, f.relation_name, f.args))
            for f in core
        ])
    else:
        lines.append(f"  {render_answer(verdict, exhausted=exhausted)}")

    return "\n".join(lines)


__all__ = ["render_answer", "render_solution_table"]
