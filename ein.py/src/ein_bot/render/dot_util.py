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


def esc(s: str) -> str:
    """Escape DOT-special characters (backslash, double-quote) in a label
    or identifier body — *without* the surrounding quotes."""
    return s.replace("\\", "\\\\").replace('"', '\\"')


def quote(s: str) -> str:
    """Quote a DOT identifier or label, escaping internal specials."""
    return '"' + esc(s) + '"'


def multiline(*parts: str) -> str:
    r"""A quoted DOT label with ``\n``-separated, escaped non-empty lines."""
    return '"' + "\\n".join(esc(p) for p in parts if p) + '"'


def hashed_id(prefix: str, seed: str, *, quoted: bool = False) -> str:
    """Content-addressed DOT node id: ``prefix`` + ``md5(seed)[:10]`` hex.

    The single identity scheme behind the fact octagons (``prefix="f_"``)
    and lattice cells (``prefix="n_"``) — S1.7c.25 (F-KB-8) collapses four
    hand-rolled copies (``kb/render``, ``kb/provenance``, ``render/slice``,
    ``render/lattice_dag``) onto this one ``[:10]`` definition. The caller
    owns ``seed`` construction: the flat ``rel|arg,arg`` key (see
    :func:`fact_key`) and ``slice``'s *recursive* key are deliberately NOT
    merged — only this hash+prefix tail is shared. ``quoted=True`` wraps
    the id in DOT quotes (``render/slice`` / ``lattice_dag`` emit quoted
    ids; ``kb/render`` and ``kb/provenance`` emit bare)."""
    import hashlib
    nid = prefix + hashlib.md5(seed.encode("utf-8")).hexdigest()[:10]
    return quote(nid) if quoted else nid


def digraph_open(name: str, *, rankdir: str | None = None,
                 node_defaults: str | None = None) -> list[str]:
    """The opening lines of a ``digraph`` — returned as a list to seed the
    caller's ``lines`` (S1.7c.25 shares this across the LR-family
    renderers ``render/slice``, ``render/lattice_dag``, ``render/constraints``):

        digraph_open("slice", rankdir="LR", node_defaults='fontname="Inter"')
        -> ['digraph slice {', '  rankdir=LR;', '  node [fontname="Inter"];']

    Each keyword's line is omitted when ``None``. ``name`` is interpolated
    bare — every current emitter does (the graph names in use need no
    quoting). The other emitters' preambles (``kb/render``'s interleaved
    fdp comment, ``provenance``/``rules``'s bespoke headers, ``_Builder``'s
    inline ``{``) diverge too much to route here byte-identically."""
    out = [f"digraph {name} {{"]
    if rankdir is not None:
        out.append(f"  rankdir={rankdir};")
    if node_defaults is not None:
        out.append(f"  node [{node_defaults}];")
    return out


def fact_key(relation_name: str, args: tuple) -> str:
    """Flat content key ``f"{rel}|" + ",".join(str(a) …)`` for a fact id.

    The ``kb/render`` + ``kb/provenance`` form — NOT recursive (nested
    Fact args stringify via ``str(a)``). ``render/slice`` builds its key
    recursively and keeps its own ``_key`` (the recursion is load-bearing
    for its node ids)."""
    return f"{relation_name}|" + ",".join(str(a) for a in args)


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
    "digraph_open",
    "esc",
    "fact_key",
    "fact_label",
    "hashed_id",
    "multiline",
    "quote",
    "value_label",
]
