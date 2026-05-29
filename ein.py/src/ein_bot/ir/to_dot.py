"""IR → DOT renderer — S1.1.4 / S1.6.0.

Implements `docs/kernel/ir/03-ein-lang/04_dot_rendering.md`. Forward-only:
reverse parse (`from_dot`) lands in P1.2.

Public entry points:

    to_dot(node, *, rule_mode="a", trace_view="a", levi=False) -> str
        Top-level dispatch. Accepts a single SForm or a tuple of
        top-level SForms. Returns one `digraph` per top-level form,
        joined by blank lines.

    render_ontology / render_facts / render_reasoning /
    render_rule / render_query / render_trace
        Per-form renderers. Each returns a complete `digraph { … }`
        string.

**Compact vs Levi (S1.6.0).** The default view is *compact*
(entity-style): a binary fact `(rel a b)` collapses to one labelled,
relation-coloured arrow `a -> b [label="rel"]`. The canonical
Levi-bipartite view — every relation a list-node (`octagon`) with
role-labelled arrows to its participants — is faithful but unreadable
as a default; it stays available via ``levi=True`` (CLI ``--levi`` /
``EIN_RENDER_LEVI=1``). n-ary facts (arity ≠ 2) render Levi-bipartite
in both modes (DOT has no native hyperedge).

The renderer is *structural*: only graph structure is fixed by the
schema; layout (positions, rank, unspecified styles) is free.
"""
from __future__ import annotations

from collections.abc import Iterable

from ..render.dot_util import (
    EQUALITY_SHAPE,
    GROUND_SHAPE,
    HYPER_SHAPE,
    INSTANCE_SHAPE,
    TYPE_SHAPE,
    VAR_SHAPE,
    WILDCARD_ATTRS,
)
from ..render.dot_util import (
    quote as _quote,
)
from ..render.dot_util import (
    value_label as _value_label,
)
from ..render.palette import hash_color
from ..render.rules import render_rule, render_rules
from .types import (
    Atom,
    Int,
    IRNode,
    KwPair,
    Range,
    SForm,
    String,
    Var,
    Wildcard,
)


def _atom_id(node: Atom | Var | Wildcard) -> str:
    """Return a DOT-safe quoted identifier for an atom-like node."""
    if isinstance(node, Var):
        return _quote(f"?{node.name}")
    if isinstance(node, Wildcard):
        return _quote("_")
    return _quote(node.name)


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


def _emit_fact(b: _Builder, fact: SForm, *, derived: bool = False,
               levi: bool = False) -> None:
    """Emit one fact.

    Reserved-word facts (`=`, `instance`, `not`) get specialised
    encodings. A generic *binary* relation collapses to one labelled
    arrow in the default *compact* view, or — under ``levi=True`` —
    keeps the canonical Levi-bipartite octagon. n-ary relations
    (arity ≠ 2) are always Levi-bipartite (DOT has no native hyperedge).
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
        _emit_fact(b, positional[0], derived=True, levi=levi)  # dashed
        return

    # Compact (default): a binary relation is one relation-coloured
    # arrow — the entity-style view. Levi keeps the octagon (below).
    if not levi and len(positional) == 2:
        a, c = positional
        _emit_atom(b, a)
        _emit_atom(b, c)
        colour = hash_color(head)
        style = "dashed" if derived else "solid"
        b.edge(_atom_id_for_value(a), _atom_id_for_value(c),
               f'label="{head}", color="{colour}", '
               f'fontcolor="{colour}", style={style}')
        _ = kwpairs
        return

    # Levi-bipartite octagon: every n-ary relation, plus binary under
    # ``levi=True`` — one list-node, role-labelled arrows to each arg.
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

    For nested SForm values (rare — e.g. `(= (color House-1) Red)`),
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


def render_ontology(form: SForm, *, levi: bool = False) -> str:
    """Render `(ontology …)` — UML-ish type/instance/relation schema.

    Type / relation / a-priori declarations render identically in both
    modes (they are schema, already drawn as direct labelled edges);
    only implicit facts inside the block honour ``levi``.
    """
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
            # (relation Name T1 T2 … [kw…]) — flat args post-R10.
            args = decl.args
            if not args or not isinstance(args[0], (Atom, Var)):
                continue
            rel_name = _value_label(args[0])
            sig = [a for a in args[1:] if isinstance(a, (Atom, Var, Wildcard))]
            if len(sig) >= 2:
                src, dst = sig[0], sig[1]
                b.node(_atom_id(src), f"shape={TYPE_SHAPE}")
                b.node(_atom_id(dst), f"shape={TYPE_SHAPE}")
                b.edge(_atom_id_for_value(src), _atom_id_for_value(dst),
                       f'label="{rel_name}", style=dashed')
        elif head == "a-priori":
            # (a-priori Name T1 T2 … [kw…]) — flat args post-R10.
            args = decl.args
            if not args:
                continue
            ap_name = _value_label(args[0])
            sig = [a for a in args[1:] if isinstance(a, (Atom, Var, Wildcard))]
            if len(sig) >= 2:
                src, dst = sig[0], sig[1]
                b.node(_atom_id(src), f"shape={TYPE_SHAPE}")
                b.node(_atom_id(dst), f"shape={TYPE_SHAPE}")
                b.edge(_atom_id_for_value(src), _atom_id_for_value(dst),
                       f'label="{ap_name}", style=dashed, penwidth=2')
        else:
            # Implicit fact inside ontology — compact or Levi.
            _emit_fact(b, decl, levi=levi)
    return b.build()


def render_facts(form: SForm, *, derived: bool = False,
                 levi: bool = False) -> str:
    """Render `(facts …)` or `(reasoning …)`.

    Compact by default (binary facts as labelled arrows); pass
    ``levi=True`` for the Levi-bipartite hyperedge view.
    """
    name = "reasoning" if derived else "facts"
    b = _Builder(name)
    for fact in form.args:
        if isinstance(fact, SForm):
            _emit_fact(b, fact, derived=derived, levi=levi)
    return b.build()


def render_reasoning(form: SForm, *, levi: bool = False) -> str:
    """Render `(reasoning …)` — derived facts shown with dashed edges."""
    return render_facts(form, derived=True, levi=levi)


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


# Trace-view aliases — friendly names alongside the legacy letters.
_TRACE_VIEW_ALIASES = {
    "a": "a", "per-step": "a",
    "b": "b", "aggregate": "b",
    "c": "c", "dag": "c",
}


def _trace_premises(using: SForm) -> list[Atom]:
    """Premise refs in a `:using` clause.

    `(and c10 c15)` → [c10, c15] (the `and` head is the combinator);
    `(c10)` → [c10] (the head IS the premise).
    """
    head = using.head
    if isinstance(head, Atom) and head.name == "and":
        return [a for a in using.args if isinstance(a, Atom)]
    out = [head] if isinstance(head, Atom) else []
    out += [a for a in using.args if isinstance(a, Atom)]
    return out


def render_trace(form: SForm, *, view: str = "a") -> str:
    """Render `(trace …)`.

    - **(a) per-step** (default) / **(b) aggregate** — a single digraph
      of step nodes linked by `:using` references.
    - **(c) derivation DAG** (alias ``dag``) — the *explanation graph*:
      one node per derived fact (`:derives`), each linked back to its
      `:using` premises (chaining through earlier steps), the edge
      labelled by the firing `:rule`.
    """
    canon = _TRACE_VIEW_ALIASES.get(view)
    if canon is None:
        raise ValueError(
            f"unknown trace view: {view!r} "
            f"(expected per-step/a, aggregate/b, or dag/c)"
        )
    if canon == "c":
        return _render_trace_dag(form)

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
                    for premise in _trace_premises(arg.value):
                        b.node(_atom_id(premise), "shape=rectangle")
                        b.edge(_atom_id(premise), step_id, 'style=dashed')
    return b.build()


def _render_trace_dag(form: SForm) -> str:
    """View (c) — derived-fact nodes linked to their `:using` premises."""
    b = _Builder("trace")
    derived_by_step: dict[str, str] = {}   # step name → its derived-fact node id
    for ev in form.args:
        if not isinstance(ev, SForm):
            continue
        step_name = next((a.name for a in ev.args if isinstance(a, Atom)), None)
        if step_name is None:
            continue
        rule = None
        derives: SForm | None = None
        using: SForm | None = None
        for arg in ev.args:
            if isinstance(arg, KwPair):
                if arg.key.name == "rule":
                    rule = _value_label(arg.value)
                elif arg.key.name == "derives" and isinstance(arg.value, SForm):
                    derives = arg.value
                elif arg.key.name == "using" and isinstance(arg.value, SForm):
                    using = arg.value
        # Node: the derived fact (falls back to the step name).
        if derives is not None:
            dlabel = _value_label(derives)
            dnode = _quote(dlabel)
            b.node(dnode, f"shape=box, style=bold, label={_quote(dlabel)}")
        else:
            dnode = _quote(step_name)
            b.node(dnode, f"shape=box, label={_quote(step_name)}")
        derived_by_step[step_name] = dnode
        edge_attrs = f'label="{rule}"' if rule else None
        if using is not None:
            for premise in _trace_premises(using):
                # Chain to a prior step's derived fact when the ref names one.
                pid = derived_by_step.get(premise.name, _atom_id(premise))
                if premise.name not in derived_by_step:
                    b.node(_atom_id(premise), "shape=rectangle")
                b.edge(pid, dnode, edge_attrs)
    return b.build()


# ── Top-level dispatch ─────────────────────────────────────────────

def to_dot(node: IRNode | Iterable[IRNode], *, rule_mode: str = "a",
           trace_view: str = "a", levi: bool = False) -> str:
    """Render an IR node (or tuple of top-level forms) to DOT.

    Returns one `digraph { … }` per top-level form, joined by blank
    lines. For a `(rules …)` form, each child rule becomes its own
    digraph (a rule library is multiple graphs).

    ``levi=True`` selects the Levi-bipartite view for ontology / facts
    / reasoning forms (default: compact). ``rule_mode`` defaults to the
    side-by-side LHS|RHS view; pass ``"overlay"`` for the compact
    overlay (S1.6.0).
    """
    if isinstance(node, SForm):
        head = node.head.name
        if head == "ontology":
            return render_ontology(node, levi=levi)
        if head == "facts":
            return render_facts(node, derived=False, levi=levi)
        if head == "reasoning":
            return render_reasoning(node, levi=levi)
        if head == "rules":
            return render_rules(node, mode=rule_mode)
        if head == "rule":
            return render_rule(node, mode=rule_mode)
        if head == "query":
            return render_query(node)
        if head == "trace":
            return render_trace(node, view=trace_view)
        if head == "config":
            # Solver knobs — no graph structure to render.
            return ""
        raise ValueError(f"unknown top-level form: {head}")
    # Tuple/iterable of top-level forms
    chunks = [to_dot(f, rule_mode=rule_mode, trace_view=trace_view, levi=levi)
              for f in node]
    return "\n\n".join(c for c in chunks if c)


__all__ = [
    "render_facts",
    "render_ontology",
    "render_query",
    "render_reasoning",
    "render_rule",
    "render_rules",
    "render_trace",
    "to_dot",
]
