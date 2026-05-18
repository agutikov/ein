"""IR → DOT renderer — S1.1.4.

Implements `docs/ir.md` §6 (Rendering schema). Forward-only: reverse
parse (`from_dot`) lands in P1.2.

Public entry points:

    to_dot(node, *, rule_mode="c", trace_view="a") -> str
        Top-level dispatch. Accepts a single SForm or a tuple of
        top-level SForms. Returns one `digraph` per top-level form,
        joined by blank lines.

    render_ontology / render_facts / render_reasoning /
    render_rule / render_query / render_trace
        Per-form renderers. Each returns a complete `digraph { … }`
        string.

The renderer is *structural*: only graph structure is fixed by the
schema; layout (positions, rank, unspecified styles) is free.
"""
from __future__ import annotations

from collections.abc import Iterable

from .types import (
    Atom, IRNode, Int, Keyword, KwPair, Range, SForm, String, Var,
    Wildcard,
)

# ── Shape table (per docs/ir.md §6 node-shape legend) ──────────────
TYPE_SHAPE      = "box"
INSTANCE_SHAPE  = "oval"
GROUND_SHAPE    = "rectangle"
HYPER_SHAPE     = "octagon"
EQUALITY_SHAPE  = "doublecircle"
VAR_SHAPE       = "diamond"
WILDCARD_ATTRS  = 'shape=diamond, style=dashed'


def _quote(s: str) -> str:
    """Quote a DOT identifier or label, escaping internal quotes."""
    return '"' + s.replace('\\', '\\\\').replace('"', '\\"') + '"'


def _atom_id(node: Atom | Var | Wildcard) -> str:
    """Return a DOT-safe quoted identifier for an atom-like node."""
    if isinstance(node, Var):
        return _quote(f"?{node.name}")
    if isinstance(node, Wildcard):
        return _quote("_")
    return _quote(node.name)


def _value_label(node: IRNode) -> str:
    """Human-readable single-line label for a value (used as edge labels)."""
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
        inner = " ".join(_value_label(a) for a in node.args)
        head = _value_label(node.head)
        return f"({head} {inner})" if inner else f"({head})"
    raise TypeError(f"not a value node: {type(node).__name__}")


# ── Builder — accumulates node decls + edges for one digraph ───────

class _Builder:
    """Accumulates DOT node declarations and edges for a single digraph.

    Node ids are stored as their *quoted* DOT form (`"Norwegian"`).
    Re-declaring an id with the same attrs is a no-op; with different
    attrs, the LATER decl overrides — useful when a name first
    appears as a generic ground atom and is later refined (e.g., the
    ontology declares it a type or instance).
    """

    def __init__(self, name: str) -> None:
        self.name = name
        self._nodes: dict[str, str] = {}        # id → attr string
        self._edges: list[str] = []
        self._hcount = 0

    def node(self, node_id: str, attrs: str | None = None) -> None:
        if attrs is not None or node_id not in self._nodes:
            self._nodes[node_id] = attrs or ""

    def edge(self, src: str, dst: str, attrs: str | None = None) -> None:
        line = f"  {src} -> {dst}"
        if attrs:
            line += f" [{attrs}]"
        line += ";"
        self._edges.append(line)

    def fresh_h(self, label: str) -> str:
        """Mint a fresh hyperedge node id and declare it."""
        self._hcount += 1
        node_id = _quote(f"h_{self._hcount}_{label}")
        self.node(node_id, f'shape={HYPER_SHAPE}, label="{label}"')
        return node_id

    def build(self) -> str:
        node_lines = []
        for node_id, attrs in self._nodes.items():
            if attrs:
                node_lines.append(f"  {node_id} [{attrs}];")
            else:
                node_lines.append(f"  {node_id};")
        body = "\n".join(node_lines + self._edges)
        return f"digraph {self.name} {{\n{body}\n}}"


# ── Form renderers ─────────────────────────────────────────────────

def _atom_arg_attrs(node: IRNode) -> str:
    """Shape attrs for an atom-like arg appearing in a fact / pattern."""
    if isinstance(node, Var):
        return f"shape={VAR_SHAPE}"
    if isinstance(node, Wildcard):
        return WILDCARD_ATTRS
    if isinstance(node, Atom):
        return f"shape={GROUND_SHAPE}"
    return f"shape={GROUND_SHAPE}"


def _emit_fact(b: _Builder, fact: SForm, *, derived: bool = False) -> None:
    """Emit one fact as a Levi-bipartite hyperedge.

    Reserved-word facts (`=`, `instance`, `not`) get specialised
    encodings; everything else uses the generic Levi-bipartite shape.
    """
    head = fact.head.name
    positional = tuple(a for a in fact.args if not isinstance(a, KwPair))
    kwpairs = tuple(a for a in fact.args if isinstance(a, KwPair))

    # Equality fact → double-circle equality class
    if head == "=" and len(positional) == 2:
        a, c = positional
        eq_id = _quote(f"eq_{_value_label(a)}_{_value_label(c)}")
        b.node(eq_id, f"shape={EQUALITY_SHAPE}, label=\"=\"")
        _emit_atom(b, a)
        _emit_atom(b, c)
        b.edge(eq_id, _atom_id_for_value(a))
        b.edge(eq_id, _atom_id_for_value(c))
        return

    # Instance fact → dashed instance-of edge (UML-style)
    if head == "instance" and len(positional) == 2:
        ent, typ = positional
        if isinstance(ent, (Atom, Var, Wildcard)):
            b.node(_atom_id(ent), f"shape={INSTANCE_SHAPE}")
        if isinstance(typ, (Atom, Var, Wildcard)):
            b.node(_atom_id(typ), f"shape={TYPE_SHAPE}")
        b.edge(_atom_id_for_value(ent), _atom_id_for_value(typ),
               'style=dashed, arrowhead=empty, label="instance-of"')
        return

    # Negative fact → recurse into the wrapped expression, mark dashed
    if head == "not" and len(positional) == 1 and isinstance(positional[0], SForm):
        _emit_fact(b, positional[0], derived=True)  # dashed via 'derived'
        return

    # Generic n-ary relation → Levi-bipartite octagon
    style = ", style=dashed" if derived else ""
    h_id = b.fresh_h(head)
    for i, arg in enumerate(positional, start=1):
        _emit_atom(b, arg)
        b.edge(h_id, _atom_id_for_value(arg), f'label="{i}"{style}')
    # Suppress unused-kwpairs warning — provenance is implicit on the
    # hyperedge node; not rendered visually here.
    _ = kwpairs


def _atom_id_for_value(node: IRNode) -> str:
    """Resolve an atom-like or SForm value to a DOT node id.

    For nested SForm values (rare — e.g. `(= (color House_1) Red)`),
    recursively introduce a hyperedge node.
    """
    if isinstance(node, (Atom, Var, Wildcard)):
        return _atom_id(node)
    if isinstance(node, String):
        return _quote(node.value)
    if isinstance(node, Int):
        return _quote(str(node.value))
    if isinstance(node, Range):
        return _quote(_value_label(node))
    if isinstance(node, SForm):
        return _quote(_value_label(node))
    raise TypeError(f"cannot use as DOT node id: {type(node).__name__}")


def _emit_atom(b: _Builder, node: IRNode) -> None:
    """Ensure an atom-like node is declared with appropriate shape."""
    if isinstance(node, (Atom, Var, Wildcard)):
        b.node(_atom_id(node), _atom_arg_attrs(node))


def render_ontology(form: SForm) -> str:
    """Render `(ontology …)` per docs/ir.md §6 'Ontology — UML-ish'."""
    b = _Builder("ontology")
    for decl in form.args:
        if not isinstance(decl, SForm):
            continue
        head = decl.head.name
        if head == "type":
            # (type Name [Parent])
            args = [a for a in decl.args if isinstance(a, (Atom, Var, Wildcard))]
            if not args:
                continue
            name = args[0]
            b.node(_atom_id(name), f"shape={TYPE_SHAPE}")
            if len(args) >= 2:
                parent = args[1]
                b.node(_atom_id(parent), f"shape={TYPE_SHAPE}")
                b.edge(_atom_id(name), _atom_id(parent),
                       "style=dashed, arrowhead=empty")
        elif head == "relation":
            # (relation Name (T1 T2 …) [kw…])
            args = decl.args
            if not args or not isinstance(args[0], (Atom, Var)):
                continue
            rel_name = _value_label(args[0])
            sig = args[1] if len(args) > 1 and isinstance(args[1], SForm) else None
            if sig is not None and len(sig.args) >= 2:
                src = sig.args[0]
                dst = sig.args[1]
                if isinstance(src, (Atom, Var, Wildcard)):
                    b.node(_atom_id(src), f"shape={TYPE_SHAPE}")
                if isinstance(dst, (Atom, Var, Wildcard)):
                    b.node(_atom_id(dst), f"shape={TYPE_SHAPE}")
                b.edge(_atom_id_for_value(src), _atom_id_for_value(dst),
                       f'label="{rel_name}", style=dashed')
        elif head == "a-priori":
            args = decl.args
            if not args:
                continue
            ap_name = _value_label(args[0])
            sig = args[1] if len(args) > 1 and isinstance(args[1], SForm) else None
            if sig is not None and len(sig.args) >= 2:
                src, dst = sig.args[0], sig.args[1]
                if isinstance(src, (Atom, Var, Wildcard)):
                    b.node(_atom_id(src), f"shape={TYPE_SHAPE}")
                if isinstance(dst, (Atom, Var, Wildcard)):
                    b.node(_atom_id(dst), f"shape={TYPE_SHAPE}")
                b.edge(_atom_id_for_value(src), _atom_id_for_value(dst),
                       f'label="{ap_name}", style=dashed, penwidth=2')
        else:
            # Implicit fact inside ontology — Levi-bipartite.
            _emit_fact(b, decl)
    return b.build()


def render_facts(form: SForm, *, derived: bool = False) -> str:
    """Render `(facts …)` or `(reasoning …)` — Levi-bipartite hyperedges."""
    name = "reasoning" if derived else "facts"
    b = _Builder(name)
    for fact in form.args:
        if isinstance(fact, SForm):
            _emit_fact(b, fact, derived=derived)
    return b.build()


def render_reasoning(form: SForm) -> str:
    """Render `(reasoning …)` — derived facts shown with dashed edges."""
    return render_facts(form, derived=True)


def render_query(form: SForm) -> str:
    """Render `(query …)` — keyword args as a compact node."""
    b = _Builder("query")
    label_parts = []
    for arg in form.args:
        if isinstance(arg, KwPair):
            label_parts.append(f":{arg.key.name} {_value_label(arg.value)}")
    label = "\\n".join(label_parts) if label_parts else "query"
    b.node(_quote("query"), f'shape=note, label="{label}"')
    return b.build()


def render_rule(rule_sform: SForm, *, mode: str = "c") -> str:
    """Render a single `(rule Name params :match … :assert … :why … …)`.

    Modes:
      - "a" : side-by-side LHS|RHS clusters (default for rule libraries)
      - "c" : overlay — LHS solid, RHS additions dashed (default for traces)
    """
    if mode not in ("a", "c"):
        raise ValueError(f"unknown rule mode: {mode!r} (expected 'a' or 'c')")

    # Extract rule fields. rule_decl args: [name, params, kw_pair...]
    rule_name = "anon"
    match_expr: SForm | None = None
    assert_expr: SForm | None = None
    for i, arg in enumerate(rule_sform.args):
        if i == 0 and isinstance(arg, Atom):
            rule_name = arg.name
        elif isinstance(arg, KwPair):
            if arg.key.name == "match" and isinstance(arg.value, SForm):
                match_expr = arg.value
            elif arg.key.name == "assert" and isinstance(arg.value, SForm):
                assert_expr = arg.value

    safe_name = rule_name.replace("-", "_").replace(" ", "_")
    graph_name = f"rule_{safe_name}_{'lhs_rhs' if mode == 'a' else 'overlay'}"

    if mode == "a":
        return _render_rule_lhs_rhs(graph_name, match_expr, assert_expr)
    return _render_rule_overlay(graph_name, match_expr, assert_expr)


def _render_rule_lhs_rhs(graph_name: str, match: SForm | None,
                         assert_: SForm | None) -> str:
    """Mode (a) — side-by-side `cluster_lhs` / `cluster_rhs` blocks."""
    parts = [f"digraph {graph_name} {{"]
    parts.append('  subgraph cluster_lhs { label="match";')
    if match is not None:
        for line in _render_pattern_edges(match, suffix="_l"):
            parts.append(f"    {line}")
    parts.append("  }")
    parts.append('  subgraph cluster_rhs { label="assert";')
    if assert_ is not None:
        for line in _render_pattern_edges(assert_, suffix="_r"):
            parts.append(f"    {line}")
    parts.append("  }")
    parts.append("}")
    return "\n".join(parts)


def _render_rule_overlay(graph_name: str, match: SForm | None,
                         assert_: SForm | None) -> str:
    """Mode (c) — LHS solid, RHS additions dashed (overlay)."""
    parts = [f"digraph {graph_name} {{"]
    if match is not None:
        for line in _render_pattern_edges(match, suffix=""):
            parts.append(f"  {line}")
    if assert_ is not None:
        for line in _render_pattern_edges(assert_, suffix="",
                                          extra_attrs="style=dashed"):
            parts.append(f"  {line}")
    parts.append("}")
    return "\n".join(parts)


def _render_pattern_edges(expr: SForm, *, suffix: str,
                          extra_attrs: str = "") -> list[str]:
    """Flatten a pattern expression into DOT edge lines.

    Patterns are `(and …)` / `(or …)` / `(not …)` / a single relation
    pattern. Each binary relation `(rel a b)` becomes one edge
    `a -> b [label="rel"]`; n-ary patterns are rendered Levi-bipartite.
    """
    out: list[str] = []
    # Combinator heads are always Atoms (and / or / not are reserved
    # kernel primitives); only relation-pattern heads can be Var or
    # Wildcard.
    head_name = expr.head.name if isinstance(expr.head, Atom) else None
    if head_name == "and":
        for child in expr.args:
            if isinstance(child, SForm):
                out.extend(_render_pattern_edges(child, suffix=suffix,
                                                 extra_attrs=extra_attrs))
        return out
    if head_name == "or":
        for child in expr.args:
            if isinstance(child, SForm):
                or_attrs = f"{extra_attrs}, color=blue" if extra_attrs else "color=blue"
                out.extend(_render_pattern_edges(child, suffix=suffix,
                                                 extra_attrs=or_attrs))
        return out
    if head_name == "not" and expr.args and isinstance(expr.args[0], SForm):
        not_attrs = f"{extra_attrs}, color=red" if extra_attrs else "color=red"
        inner = _render_pattern_edges(expr.args[0], suffix=suffix,
                                      extra_attrs=not_attrs)
        return inner
    # A relation pattern: head is the relation name (or ?var/wildcard),
    # positional args are the participants.
    head_label = _value_label(expr.head)
    positional = [a for a in expr.args if not isinstance(a, KwPair)]
    if len(positional) == 2:
        a, b = positional
        a_id = _quote(_value_label(a) + suffix)
        b_id = _quote(_value_label(b) + suffix)
        attrs = f'label="{head_label}"'
        if extra_attrs:
            attrs += f", {extra_attrs}"
        out.append(f"{a_id} -> {b_id} [{attrs}];")
    elif positional:
        # n-ary: introduce one hyperedge node
        h_id = _quote(f"h_{head_label}{suffix}")
        out.append(f'{h_id} [shape={HYPER_SHAPE}, label="{head_label}"];')
        for i, arg in enumerate(positional, start=1):
            t_id = _quote(_value_label(arg) + suffix)
            attrs = f'label="{i}"'
            if extra_attrs:
                attrs += f", {extra_attrs}"
            out.append(f"{h_id} -> {t_id} [{attrs}];")
    return out


def render_trace(form: SForm, *, view: str = "a") -> str:
    """Render `(trace …)` — currently only mode (a) per-step is implemented.

    Mode (a) emits a single digraph showing all step nodes connected
    by `:using` references. Modes (b) aggregate and (c) derivation-DAG
    are P1.6 territory; this v0 just labels them and falls through.
    """
    if view not in ("a", "b", "c"):
        raise ValueError(f"unknown trace view: {view!r}")
    b = _Builder("trace")
    for ev in form.args:
        if not isinstance(ev, SForm):
            continue
        kind = ev.head.name
        # First positional arg is the step name (a SYMBOL).
        step_name = None
        for arg in ev.args:
            if isinstance(arg, Atom):
                step_name = arg.name
                break
        if step_name is None:
            continue
        step_id = _quote(step_name)
        shape = "box" if kind == "step" else "ellipse"
        b.node(step_id, f"shape={shape}, label=\"{kind}: {step_name}\"")
        # Extract :using premises for edge connections.
        for arg in ev.args:
            if isinstance(arg, KwPair) and arg.key.name == "using":
                if isinstance(arg.value, SForm):
                    head = arg.value.head
                    if isinstance(head, Atom):
                        b.node(_atom_id(head), "shape=rectangle")
                        b.edge(_atom_id(head), step_id, 'style=dashed')
                    for premise in arg.value.args:
                        if isinstance(premise, Atom):
                            b.node(_atom_id(premise), "shape=rectangle")
                            b.edge(_atom_id(premise), step_id,
                                   'style=dashed')
    return b.build()


# ── Top-level dispatch ─────────────────────────────────────────────

def to_dot(node: IRNode | Iterable[IRNode], *, rule_mode: str = "c",
           trace_view: str = "a") -> str:
    """Render an IR node (or tuple of top-level forms) to DOT.

    Returns one `digraph { … }` per top-level form, joined by blank
    lines. For a `(rules …)` form, each child rule becomes its own
    digraph (a rule library is multiple graphs).
    """
    if isinstance(node, SForm):
        head = node.head.name
        if head == "ontology":
            return render_ontology(node)
        if head == "facts":
            return render_facts(node, derived=False)
        if head == "reasoning":
            return render_reasoning(node)
        if head == "rules":
            chunks = [render_rule(r, mode=rule_mode)
                      for r in node.args if isinstance(r, SForm)]
            return "\n\n".join(chunks)
        if head == "rule":
            return render_rule(node, mode=rule_mode)
        if head == "query":
            return render_query(node)
        if head == "trace":
            return render_trace(node, view=trace_view)
        raise ValueError(f"unknown top-level form: {head}")
    # Tuple/iterable of top-level forms
    chunks = [to_dot(f, rule_mode=rule_mode, trace_view=trace_view)
              for f in node]
    return "\n\n".join(chunks)


__all__ = [
    "to_dot",
    "render_ontology", "render_facts", "render_reasoning",
    "render_rule", "render_query", "render_trace",
]
