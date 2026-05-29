"""Markdown trace renderer — S1.6.4 T1.6.4.2 / .3.

Threads a linearized :class:`~ein_bot.trace.linearize.Trace` into a
single self-contained markdown narrative: a numbered step per rule
firing (name, English ``:why``, premises with their quoted source
sentences, and an inline `dot` derivation slice), refuted hypotheses
folded into `<details>` reductio sections, and a closing lattice-DAG +
solution grid. Every diagram is an inline fenced `dot` block — no SVG
(the Python layer emits DOT; rasterising is a shell concern).

``mode="reorder"`` clusters the steps by the entity they are about
(`## About <X>`) — a presentation pass over the same steps (T1.6.4.3).
"""
from __future__ import annotations

from .ast import TraceStep
from .linearize import Reductio, Trace


def _dot_block(dot: str | None) -> list[str]:
    if not dot:
        return []
    return ["```dot", dot, "```", ""]


def _premises_line(step: TraceStep) -> str:
    labels = step.premise_labels()
    if labels:
        base = "Premises: " + ", ".join(f"`{p}`" for p in labels)
        if step.sources:                       # source-sentence quoting (T1.6.4.5)
            base += " — from " + ", ".join(step.sources)
        return base
    if step.sources:                           # no derived premise — the source is given
        return "Premises: from " + ", ".join(step.sources)
    return "Premises: —"


def _render_step(step: TraceStep, *, diagrams: bool) -> list[str]:
    out = [f"## Step {step.n} — `{step.rule}`", ""]
    if step.why:
        out += [f"> {step.why}", ""]
    out += [_premises_line(step), "",
            f"Derives `{step.derived_label}`.", ""]
    if diagrams:
        out += _dot_block(step.diagram)
    return out


def _render_reductio(r: Reductio, *, diagrams: bool) -> list[str]:
    out = ["<details>", f"<summary>{r.summary}</summary>", ""]
    out += [f"Assumed **{r.commitment}**; the branch derives ⊥.", ""]
    if r.learned_clause:
        out += [f"Lifted no-good: `{r.learned_clause}`.", ""]
    if diagrams:
        out += _dot_block(r.diagram)
    out += ["</details>", ""]
    return out


def render_markdown(
    trace: Trace, *, mode: str = "engine", diagrams: bool = True,
) -> str:
    """Render a :class:`Trace` as a markdown string.

    ``mode`` is ``"engine"`` (numbered engine order, default) or
    ``"reorder"`` (clustered by target entity).
    """
    if mode not in ("engine", "reorder"):
        raise ValueError(f"unknown trace mode: {mode!r} (expected 'engine' or 'reorder')")

    lines = ["# Solution trace", "", f"> {trace.summary}", ""]
    if trace.commitment and trace.commitment not in ("∅ (unconditional)", "—"):
        lines += [f"Assuming **{trace.commitment}**.", ""]

    if not trace.steps:
        lines += ["_(no surviving derivation — see the refuted branches below.)_", ""]
    elif mode == "reorder":
        lines += _render_reordered(trace, diagrams=diagrams)
    else:
        for step in trace.steps:
            lines += _render_step(step, diagrams=diagrams)

    if trace.reductios:
        lines += ["## Refuted hypotheses", ""]
        for r in trace.reductios:
            lines += _render_reductio(r, diagrams=diagrams)

    if diagrams and trace.lattice_dot:
        lines += ["## Commitment lattice", "", *_dot_block(trace.lattice_dot)]
    if diagrams and trace.solution_dot:
        lines += ["## Solution", "", *_dot_block(trace.solution_dot)]
    if trace.full_kb_dot:
        lines += ["## Full KB (final state)", "", *_dot_block(trace.full_kb_dot)]

    return "\n".join(lines).rstrip() + "\n"


def _render_reordered(trace: Trace, *, diagrams: bool) -> list[str]:
    """Stable-partition the steps by their target entity (T1.6.4.3).

    A presentation pass: every step is emitted exactly once, keeping its
    engine step number and the within-cluster order — so the reordered
    trace has the *same set of steps* as engine order (the acceptance
    criterion), grouped under `## About <entity>` headings. Clusters
    appear in first-seen order; steps with no entity fall under "About
    (misc)".
    """
    order: list[str] = []
    by_entity: dict[str, list[TraceStep]] = {}
    for step in trace.steps:
        key = step.section or "(misc)"
        if key not in by_entity:
            by_entity[key] = []
            order.append(key)
        by_entity[key].append(step)

    out: list[str] = []
    for entity in order:
        out += [f"## About {entity}", ""]
        for step in by_entity[entity]:
            why = f" — {step.why}" if step.why else ""
            out += [f"**Step {step.n}** · `{step.rule}` → `{step.derived_label}`{why}", ""]
            out.append(_premises_line(step))
            out.append("")
            if diagrams:
                out += _dot_block(step.diagram)
    return out


__all__ = ["render_markdown"]
