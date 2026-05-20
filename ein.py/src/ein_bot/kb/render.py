"""Unified KB → DOT renderer — S1.2.4.

Produces a *single* ``digraph`` covering the entire knowledge base.
Unlike the per-form IR renderer in :mod:`ein_bot.ir.to_dot`, this
renderer **fuses** entity identity across forms: the `Norwegian`
instance node is emitted **once** and participates in:

- its type-edge to ``Nationality`` (ontology layer),
- its `(co-located Norwegian House_1 :source "(10)")` fact edge
  (fact layer),
- any inferred edges (reasoning layer).

This is the visual target from the 2021 PoC's `linked.svg` — types,
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
- ``instance`` kernel facts are suppressed in favour of the
  type-edge derived from ``Instance.type_name`` (otherwise every
  instance would have a duplicate edge).

**Encoding-agnostic**: zebra2.ein has empty `kb.types` /
`kb.instances` — the renderer falls back to :func:`logical_types`
and :func:`logical_instances` to decide which node atoms are
type-like (box) vs instance-like (oval), and renders the explicit
`is-a` facts with the type-edge style.
"""
from __future__ import annotations

import hashlib
import re
from collections.abc import Iterable
from typing import Literal

from .entities import Fact, Layer
from .store import KnowledgeBase
from .views import instance_name, logical_instances, logical_types, type_name

# ── Palette + colour helper ───────────────────────────────────────


# Distinct, mid-saturation colours readable on both light and dark
# backgrounds. From d3.schemeCategory10 with a couple of swaps for
# legibility on print.
_PALETTE = (
    "#1f77b4",  # blue
    "#ff7f0e",  # orange
    "#2ca02c",  # green
    "#d62728",  # red
    "#9467bd",  # purple
    "#8c564b",  # brown
    "#e377c2",  # pink
    "#7f7f7f",  # gray
    "#bcbd22",  # olive
    "#17becf",  # cyan
)


def _hash_color(name: str) -> str:
    """Stable colour per relation name — deterministic across runs."""
    h = hashlib.sha1(name.encode("utf-8")).hexdigest()
    return _PALETTE[int(h, 16) % len(_PALETTE)]


# ── DOT escaping helpers ──────────────────────────────────────────


def _q(s: str) -> str:
    """Quote a DOT identifier or label."""
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def _fact_node_id(f: Fact) -> str:
    """Stable DOT identifier for an octagon hyperedge node.

    Derived deterministically from ``(relation_name, args)`` so two
    runs produce the same id (visual-regression friendly).
    """
    key = f"{f.relation_name}|" + ",".join(str(a) for a in f.args)
    return "f_" + hashlib.md5(key.encode("utf-8")).hexdigest()[:10]


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


def _emit_type_node(name: str) -> str:
    return f"  {_q(name)} [shape=box, label={_q(name)}];"


def _emit_instance_node(name: str) -> str:
    return f"  {_q(name)} [shape=oval, label={_q(name)}];"


def _emit_is_a_edge(child: str, parent: str) -> str:
    """The dashed empty-arrow type-edge."""
    return (
        f"  {_q(child)} -> {_q(parent)} "
        f'[style=dashed, arrowhead=empty, label="is-a"];'
    )


def _emit_binary_fact(
    fact: Fact, *, colour: str, style: str, label_extra: str | None,
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
    return f"  {_q(str(src))} -> {_q(str(dst))} [{', '.join(attrs)}];"


def _emit_hyperedge(
    fact: Fact, *, colour: str, style: str, label_extra: str | None,
) -> list[str]:
    """A non-binary fact: one octagon node + n labelled edges to args.

    Arity 0 / 1 are also handled this way (no participants for arity 0;
    one labelled edge for arity 1).
    """
    nid = _fact_node_id(fact)
    head_label = f"({fact.relation_name})"
    if label_extra:
        head_label += f"\\n{label_extra}"
    lines = [
        f'  {nid} [shape=octagon, label={_q(head_label)}, '
        f'color="{colour}", fontcolor="{colour}", style={style}];',
    ]
    for i, a in enumerate(fact.args, 1):
        lines.append(
            f"  {nid} -> {_q(str(a))} "
            f'[label="#{i}", color="{colour}", style={style}];'
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
    name
        DOT graph name (default ``"kb"``).

    Returns
    -------
    str
        Complete ``digraph`` string. Pass to ``dot -Tsvg`` for SVG.
    """
    layers_set = set(layers)
    lines: list[str] = []
    lines.append(f"digraph {name} {{")
    # `fdp` is the layout engine; this is just a hint via a comment so
    # render_examples.sh picks fdp for kb dot outputs.
    lines.append("  // suggested layout: fdp")
    lines.append("  rankdir=BT;")
    lines.append('  node [fontname="Inter"];')

    # ── 1. Resolve "logical" types and instances ──
    # `logical_types(kb)` / `logical_instances(kb)` are encoding-
    # agnostic (per S1.2.2): zebra.ein returns the declared entities;
    # zebra2.ein returns names harvested from `is-a` facts.
    if include_types:
        type_names = sorted({type_name(t) for t in logical_types(kb)})
    else:
        type_names = []
    if include_instances:
        inst_names = sorted({instance_name(i) for i in logical_instances(kb)})
    else:
        inst_names = []
    # In zebra2 there are no actual Instance entities — but the leaf
    # names are returned by logical_instances as raw strings. To avoid
    # rendering the same name as both box and oval we skip ovals when
    # a name is already declared as a type.
    type_set = set(type_names)
    inst_names = [n for n in inst_names if n not in type_set]

    # ── 2. Emit type / instance nodes ──
    if type_names:
        lines.append("  // types")
        for name_ in type_names:
            lines.append(_emit_type_node(name_))
    if inst_names:
        lines.append("  // instances")
        for name_ in inst_names:
            lines.append(_emit_instance_node(name_))

    # ── 3. Emit instance-of edges (only when ONTOLOGY is requested) ──
    # zebra.ein: from kb.instances[name].type_name.
    # zebra2.ein: from `is-a` facts in the ontology layer (rendered
    # below via the fact pass with special-cased styling).
    if Layer.ONTOLOGY in layers_set and include_types and include_instances:
        for inst in kb.instances.values():
            if inst.type_name and inst.type_name in type_set:
                lines.append("")
                lines.append(_emit_is_a_edge(inst.name, inst.type_name))

    # ── 4. Emit facts ──
    if kb.facts:
        lines.append("")
        lines.append("  // facts")
    for f in kb.facts:
        if f.layer not in layers_set:
            continue
        if _suppress(f, kb):
            continue
        colour = _pick_colour(f, colour_by)
        style = _pick_style(f.layer)
        label_extra = _label_extra(f)

        # `is-a` facts (zebra2 case): render with the type-edge style,
        # not the regular relation-coloured arrow.
        if f.relation_name == "is-a" and len(f.args) == 2:
            child, parent = f.args
            lines.append(_emit_is_a_edge(str(child), str(parent)))
            continue

        if len(f.args) == 2:
            lines.append(_emit_binary_fact(
                f, colour=colour, style=style, label_extra=label_extra,
            ))
        else:
            lines.extend(_emit_hyperedge(
                f, colour=colour, style=style, label_extra=label_extra,
            ))

    lines.append("}")
    return "\n".join(lines)


# ── Per-fact decision helpers ─────────────────────────────────────


def _suppress(f: Fact, kb: KnowledgeBase) -> bool:
    """Return True iff this fact should NOT appear in the unified view.

    Three classes of suppression:
    1. Rule-application meta-facts — head matches a Rule name
       (`(symmetric co-located)` etc.). Pure meta; not data.
    2. `instance` kernel facts — already rendered as type-edges via
       `Instance.type_name`. Avoid the duplicate edge.
    3. `not`-headed facts whose arg structure is collapsed by the
       loader — the inner proposition is lost, so we can't render
       them faithfully. M1 punt; revisit when the loader preserves
       nested SForm args in `Fact.raw`.
    """
    if f.relation_name in kb.rules:
        return True
    if f.relation_name == "instance":
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
