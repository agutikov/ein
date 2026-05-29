"""ein-bot trace package — P1.6 S1.6.4.

Turns a solver `Verdict` (+ its `LatticeProof`) into a markdown
narrative — the project's main human-facing output, what makes ein-bot
*not just a solver* (idea 08).

- :mod:`ein_bot.trace.ast` — `TraceStep`, the trace-as-IR AST
  (round-trips through the P1.1 parser as a `(trace …)` form).
- :mod:`ein_bot.trace.linearize` — `linearize(verdict)`: the unordered
  commitment lattice → a depth-ordered list of `TraceStep`s + reductio
  branches (T1.6.4.0).
- :mod:`ein_bot.trace.render` — `render_markdown(trace, …)`: the
  markdown narrative with inline `dot` derivation slices (S1.6.2), a
  foldable reductio per refuted hypothesis, and a closing
  lattice-DAG (S1.6.3) + solution grid.
"""
from __future__ import annotations

from .ast import TraceStep, parse_trace_steps, trace_to_ir
from .linearize import Reductio, Trace, linearize
from .render import render_markdown

__all__ = [
    "Reductio",
    "Trace",
    "TraceStep",
    "linearize",
    "parse_trace_steps",
    "render_markdown",
    "trace_to_ir",
]
