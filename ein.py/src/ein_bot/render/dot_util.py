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


def fact_label(relation_name: str, args: tuple) -> str:
    """A readable ``rel(a, b, …)`` label for a fact or fact-id.

    Recurses into nested Fact-shaped args (Q40 relational nodes). Used
    by the derivation-slice (:mod:`ein_bot.render.slice`) and the
    lattice DAG (:mod:`ein_bot.render.lattice_dag`).
    """
    parts: list[str] = []
    for a in args:
        if hasattr(a, "relation_name") and hasattr(a, "args"):
            parts.append(fact_label(a.relation_name, a.args))         # nested Fact
        elif (isinstance(a, tuple) and len(a) == 2
              and isinstance(a[0], str) and isinstance(a[1], tuple)):
            parts.append(fact_label(a[0], a[1]))                      # nested FactRef
        else:
            parts.append(str(a))
    inner = ", ".join(parts)
    return f"{relation_name}({inner})" if inner else relation_name


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
    "fact_label",
    "quote",
    "value_label",
]
