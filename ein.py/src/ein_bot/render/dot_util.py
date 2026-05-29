"""Low-level DOT helpers shared across the renderers — S1.6.1.

`quote`, `value_label`, and the node-shape constants live here so the
per-form IR renderer (:mod:`ein_bot.ir.to_dot`) and the rule renderer
(:mod:`ein_bot.render.rules`) share one copy and one source of truth
for the shape legend, without an import cycle (this module depends only
on :mod:`ein_bot.ir.types`).

Shape legend — `docs/kernel/ir/03-ein-lang/04_dot_rendering.md`.
"""
from __future__ import annotations

from ..ir.types import (
    Atom,
    Int,
    IRNode,
    Keyword,
    Range,
    SForm,
    String,
    Var,
    Wildcard,
)

# ── Shape table (per the §Node-shape legend) ───────────────────────
TYPE_SHAPE = "box"
INSTANCE_SHAPE = "oval"
GROUND_SHAPE = "rectangle"
HYPER_SHAPE = "octagon"
EQUALITY_SHAPE = "doublecircle"
VAR_SHAPE = "diamond"
WILDCARD_ATTRS = "shape=diamond, style=dashed"


def quote(s: str) -> str:
    """Quote a DOT identifier or label, escaping internal quotes."""
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def value_label(node: IRNode) -> str:
    """Human-readable single-line label for a value (e.g. an edge label)."""
    if isinstance(node, Atom):
        return node.name
    if isinstance(node, Var):
        return f"?{node.name}"
    if isinstance(node, Wildcard):
        return "_"
    if isinstance(node, Keyword):
        return f":{node.name}"
    if isinstance(node, String):
        return node.value
    if isinstance(node, Int):
        return str(node.value)
    if isinstance(node, Range):
        high = "*" if node.high is None else str(node.high)
        return f"{node.low}..{high}"
    if isinstance(node, SForm):
        inner = " ".join(value_label(a) for a in node.args)
        head = value_label(node.head)
        return f"({head} {inner})" if inner else f"({head})"
    raise TypeError(f"not a value node: {type(node).__name__}")


__all__ = [
    "EQUALITY_SHAPE",
    "GROUND_SHAPE",
    "HYPER_SHAPE",
    "INSTANCE_SHAPE",
    "TYPE_SHAPE",
    "VAR_SHAPE",
    "WILDCARD_ATTRS",
    "quote",
    "value_label",
]
