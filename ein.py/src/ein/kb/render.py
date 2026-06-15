"""Unified KB → DOT renderer — S1.2.4.

Produces a *single* ``digraph`` covering the entire knowledge base.
Unlike the per-form IR renderer in :mod:`ein.ir.to_dot`, this
renderer **fuses** entity identity across forms: the `Norwegian`
instance node is emitted **once** and participates in:

- its type-edge to ``Nationality`` (ontology layer),
- its `(co-located Norwegian House-1 :source "(10)")` fact edge
  (fact layer),
- any inferred edges (reasoning layer).

This is the visual target from the 2021 prototype's `linked.svg` — types,
instances, and inferred edges merged onto one canvas, not stacked
tiles. See [`docs/kernel/ir/03-ein-lang/04_dot_rendering.md` §Unified
KB graph](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md).

**Schema** (per S1.2.4 T1.2.4.1):

- Each ``Type`` → ``box`` (one node per type).
- Each ``Instance`` → ``oval``.
- Each binary ``Fact(rel, a, b)`` → ``a → b [label=rel ...]`` (one
  direct edge; no Levi node).
- Each n-ary ``Fact`` (arity ≠ 2) → an ``octagon`` Levi-bipartite
  hyperedge node with slot-labelled edges.
- ``Instance → Type`` (instance-of) → ``style=dashed, arrowhead=empty``.
- Layer styling:
  - ONTOLOGY → plain solid.
  - FACT → solid; edge label includes the short ``(N)`` source id.
  - REASONING → dashed; edge label includes the rule name.
- Colour by relation: a stable hash → palette colour, consistent
  across layers so the eye groups by relation.
- Rule-application meta-facts (``(symmetric R)`` etc.) are
  **suppressed** — they're meta, not data.
- ``instance`` / ``type`` schema facts are suppressed in favour of the
  derived instance-of edge (otherwise every instance would have a
  duplicate edge).

**Encoding-agnostic** (presentation reads the inheritance convention
directly — S1.7.23, since the kernel no longer keeps a type/instance
entity-view): :func:`_schema_nodes` scans the puzzle's `is-a` /
`(type …)` / `(instance …)` facts to decide which node atoms are
type-like (box) vs instance-like (oval), and the `is-a` facts are drawn
with the type-edge style in the fact pass.
"""
from __future__ import annotations

import re
from collections.abc import Iterable
from typing import Literal

from ..render.dot_util import fact_key, hashed_id
from ..render.dot_util import quote as _q
from ..render.palette import hash_color as _hash_color
from .entities import Fact, Layer
from .store import KnowledgeBase

# ── Colour helper ─────────────────────────────────────────────────
# The relation-colour palette is shared with the per-form IR renderer
# (S1.6.0) so a relation is drawn the same colour in every view; see
# :mod:`ein.render.palette`.


# ── DOT escaping helpers ──────────────────────────────────────────


def _fact_node_id(f: Fact) -> str:
    """Stable DOT identifier for an octagon hyperedge node.

    Derived deterministically from ``(relation_name, args)`` so two
    runs produce the same id (visual-regression friendly). S1.7c.25 —
    the shared ``f_<md5[:10]>`` identity scheme (``dot_util.hashed_id``).
    """
    return hashed_id("f_", fact_key(f.relation_name, f.args))


def _short_source(source: str | None) -> str | None:
    """Extract the short ``(N)`` from ``"condition (N)"``.

    Returns the original string if no ``(N)`` pattern is found, or
    ``None`` for ``None`` input.
    """
    if not source:
        return None
    m = re.search(r"\(([^)]+)\)", source)
    return f"({m.group(1)})" if m else source


# ── Edge / node emitters ──────────────────────────────────────────


def _schema_nodes(
    kb: KnowledgeBase,
) -> tuple[set[str], set[str], list[tuple[str, str]]]:
    """Derive the (type-names, instance-names, instance-of-edges) the
    renderer draws, straight from the puzzle's inheritance facts.

    S1.7.23 — replaces the deleted `logical_types` / `logical_instances`
    helpers (which read the removed `kb.types` / `kb.instances`
    entity-view). Presentation may know the inheritance convention; it
    is not kernel reasoning. Recognised:

    - ``(is-a Child Parent)`` — Parent is a type, Child a leaf unless it
      is itself a parent elsewhere; the edge Child→Parent is drawn by the
      fact pass (type-edge style), so it is NOT returned here.
    - ``(type X [P])`` — X (and P, if present) are types. No type→parent
      edge is drawn (matching the pre-S1.7.23 renderer, which drew only
      instance-of edges, not the type hierarchy).
    - ``(instance I T)`` — I is an instance, T a type; edge I→T.
    """
    types: set[str] = set()
    children: set[str] = set()
    parents: set[str] = set()
    edges: list[tuple[str, str]] = []

    def _two_strs(args: tuple) -> bool:
        return len(args) >= 2 and isinstance(args[0], str) and isinstance(args[1], str)

    for f in kb.facts:
        rn, args = f.relation_name, f.args
        if rn == "is-a" and _two_strs(args):
            children.add(args[0])
            parents.add(args[1])
            types.add(args[1])
        elif rn == "type" and args and isinstance(args[0], str):
            types.add(args[0])
            if len(args) >= 2 and isinstance(args[1], str):
                types.add(args[1])
        elif rn == "instance" and _two_strs(args):
            children.add(args[0])
            types.add(args[1])
            edges.append((args[0], args[1]))
    instances = (children - parents) - types
    return types, instances, edges


def _emit_type_node(name: str) -> str:
    return f"  {_q(name)} [shape=box, label={_q(name)}];"


def _emit_instance_node(name: str) -> str:
    return f"  {_q(name)} [shape=oval, label={_q(name)}];"


def _emit_is_a_edge(child: str, parent: str, *, penwidth: int | None = None) -> str:
    """The dashed empty-arrow type-edge."""
    pw = f", penwidth={penwidth}" if penwidth else ""
    return (
        f"  {_q(child)} -> {_q(parent)} "
        f'[style=dashed, arrowhead=empty, label="is-a"{pw}];'
    )


def _emit_binary_fact(
    fact: Fact, *, colour: str, style: str, label_extra: str | None,
    penwidth: int | None = None,
) -> str:
    src, dst = fact.args
    label_parts = [fact.relation_name]
    if label_extra:
        label_parts.append(label_extra)
    label = " ".join(label_parts)
    attrs = [
        f"label={_q(label)}",
        f'color="{colour}"',
        f'fontcolor="{colour}"',
        f"style={style}",
    ]
    if penwidth:
        attrs.append(f"penwidth={penwidth}")
    return f"  {_q(str(src))} -> {_q(str(dst))} [{', '.join(attrs)}];"


def _emit_hyperedge(
    fact: Fact, *, colour: str, style: str, label_extra: str | None,
    penwidth: int | None = None,
) -> list[str]:
    """A non-binary fact: one octagon node + n labelled edges to args.

    Arity 0 / 1 are also handled this way (no participants for arity 0;
    one labelled edge for arity 1).
    """
    nid = _fact_node_id(fact)
    head_label = f"({fact.relation_name})"
    if label_extra:
        head_label += f"\\n{label_extra}"
    pw = f", penwidth={penwidth}" if penwidth else ""
    lines = [
        f'  {nid} [shape=octagon, label={_q(head_label)}, '
        f'color="{colour}", fontcolor="{colour}", style={style}{pw}];',
    ]
    for i, a in enumerate(fact.args, 1):
        lines.append(
            f"  {nid} -> {_q(str(a))} "
            f'[label="#{i}", color="{colour}", style={style}{pw}];'
        )
    return lines


# ── Main entry point ──────────────────────────────────────────────


_ALL_LAYERS = (Layer.ONTOLOGY, Layer.FACT, Layer.REASONING)


def to_dot(
    kb: KnowledgeBase,
    *,
    layers: Iterable[Layer] = _ALL_LAYERS,
    colour_by: Literal["relation", "layer", "none"] = "relation",
    include_types: bool = True,
    include_instances: bool = True,
    since: KnowledgeBase | None = None,
    name: str = "kb",
) -> str:
    """Render a :class:`KnowledgeBase` as a unified Graphviz digraph.

    Parameters
    ----------
    kb
        The knowledge base to render.
    layers
        Which fact layers to include. Default: all three. Use
        ``layers=(Layer.ONTOLOGY,)`` for a schema-only view, etc.
    colour_by
        Per-relation deterministic colour (default), per-layer
        colour, or no colour.
    include_types
        Emit `Type` nodes + type-edges. Default ``True``.
    include_instances
        Emit `Instance` nodes. Default ``True``.
    since
        A prior KB to diff against (S1.6.2 T1.6.2.3). Facts present here
        but absent from ``since`` are drawn thicker (``penwidth=3``) —
        the transition highlight, "this step added E". Default ``None``
        (no highlight; output byte-identical to the un-``since`` form).
    name
        DOT graph name (default ``"kb"``).

    Returns
    -------
    str
        Complete ``digraph`` string. Pass to ``dot -Tsvg`` for SVG.
    """
    layers_set = set(layers)
    since_keys = (
        {(f.relation_name, f.args) for f in since.facts}
        if since is not None else None
    )
    lines: list[str] = [
        f"digraph {name} {{",
        # `fdp` is the layout engine; this is just a hint via a comment so
        # render_examples.sh picks fdp for kb dot outputs.
        "  // suggested layout: fdp",
        "  rankdir=BT;",
        '  node [fontname="Inter"];',
    ]
    # Type / instance / instance-of nodes from the schema facts.
    lines.extend(_emit_schema_nodes(
        kb, layers_set=layers_set,
        include_types=include_types, include_instances=include_instances,
    ))
    # Facts.
    if kb.facts:
        lines.append("")
        lines.append("  // facts")
    for f in kb.facts:
        if f.layer not in layers_set or _suppress(f, kb):
            continue
        lines.extend(_emit_fact_line(f, colour_by=colour_by, since_keys=since_keys))
    lines.append("}")
    return "\n".join(lines)


def _emit_schema_nodes(
    kb: KnowledgeBase,
    *,
    layers_set: set[Layer],
    include_types: bool,
    include_instances: bool,
) -> list[str]:
    """The type / instance / instance-of DOT lines (S1.7c.26).

    S1.7.23 — read the puzzle's `is-a` / `(type …)` / `(instance …)` facts
    directly (presentation knows the convention; the kernel no longer keeps
    a type/instance entity-view). Instance-of edges are emitted only when
    ONTOLOGY is requested; the `is-a` *facts* draw their own type-edge in
    the per-fact pass.
    """
    schema_types, schema_insts, instanceof_edges = _schema_nodes(kb)
    type_set = schema_types if include_types else set()
    type_names = sorted(type_set)
    # Skip ovals when a name is already a type box (avoid double-render).
    inst_names = sorted(schema_insts - type_set) if include_instances else []

    out: list[str] = []
    if type_names:
        out.append("  // types")
        out.extend(_emit_type_node(name_) for name_ in type_names)
    if inst_names:
        out.append("  // instances")
        out.extend(_emit_instance_node(name_) for name_ in inst_names)
    if Layer.ONTOLOGY in layers_set and include_types and include_instances:
        for child, parent in instanceof_edges:
            if parent in type_set:
                out.append("")
                out.append(_emit_is_a_edge(child, parent))
    return out


def _emit_fact_line(
    f: Fact,
    *,
    colour_by: Literal["relation", "layer", "none"],
    since_keys: set | None,
) -> list[str]:
    """The DOT line(s) for one fact: `is-a` → type-edge style, binary →
    coloured arrow, else hyperedge (S1.7c.26). The caller filters by layer
    and suppression. ``since_keys`` drives the S1.6.2 transition highlight
    (facts new since a prior KB drawn at ``penwidth=3``)."""
    colour = _pick_colour(f, colour_by)
    style = _pick_style(f.layer)
    label_extra = _label_extra(f)
    penwidth = (
        3 if since_keys is not None
        and (f.relation_name, f.args) not in since_keys
        else None
    )
    if f.relation_name == "is-a" and len(f.args) == 2:
        child, parent = f.args
        return [_emit_is_a_edge(str(child), str(parent), penwidth=penwidth)]
    if len(f.args) == 2:
        return [_emit_binary_fact(
            f, colour=colour, style=style, label_extra=label_extra,
            penwidth=penwidth,
        )]
    return list(_emit_hyperedge(
        f, colour=colour, style=style, label_extra=label_extra,
        penwidth=penwidth,
    ))


# ── Per-fact decision helpers ─────────────────────────────────────


def _suppress(f: Fact, kb: KnowledgeBase) -> bool:
    """Return True iff this fact should NOT appear in the unified view.

    Three classes of suppression:
    1. Rule-application meta-facts — head matches a Rule name
       (`(symmetric co-located)` etc.). Pure meta; not data.
    2. `instance` / `type` schema facts — already rendered via the
       derived entity nodes (type boxes, instance ovals) and the
       instance-of / parent edges. Since S1.7.6 these are ordinary
       facts (not kernel declarators); suppress them here so the
       schema isn't drawn twice (entity node + octagon).
    3. `not`-headed facts whose arg structure is collapsed by the
       loader — the inner proposition is lost, so we can't render
       them faithfully. M1 punt; revisit when the loader preserves
       nested SForm args in `Fact.raw`.
    """
    if f.relation_name in kb.rules:
        return True
    if f.relation_name in ("instance", "type"):
        return True
    if f.relation_name == "not":
        # Negative facts — punt for M1 (see docstring).
        return True
    return False


def _pick_colour(f: Fact, colour_by: str) -> str:
    if colour_by == "relation":
        return _hash_color(f.relation_name)
    if colour_by == "layer":
        return {
            Layer.ONTOLOGY: "#444444",
            Layer.FACT: "#000000",
            Layer.REASONING: "#1f77b4",
        }[f.layer]
    return "#000000"


def _pick_style(layer: Layer) -> str:
    if layer == Layer.REASONING:
        return "dashed"
    return "solid"


def _label_extra(f: Fact) -> str | None:
    """The bit after the relation name on the edge label.

    Fact-layer: `(N)` short source id.
    Reasoning-layer: `by <rule-name>`.
    Ontology-layer: nothing.
    """
    if f.layer == Layer.FACT and f.source:
        return _short_source(f.source)
    if f.layer == Layer.REASONING and f.rule_name:
        return f"by {f.rule_name}"
    return None


__all__ = ["to_dot"]
