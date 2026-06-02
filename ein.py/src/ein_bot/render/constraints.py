"""Constraint-scope → DOT renderer — S1.6.1 T1.6.1.2.

A puzzle's *structural constraints* are the rule-application facts in
the **ontology** layer — the implicit "co-located is symmetric /
color-loc is a bijection" context the solver supplies, as opposed to
the explicit puzzle conditions in `(facts …)`. They are identified
structurally: an ontology fact whose head is **neither a kernel
keyword (`relation` / `type` / `instance`) nor a declared
relation name**. That captures property activators like `bijective`
(matched by the `bijective-*` rules, but not itself a rule name) while
excluding relation data such as `(is-a House Attribute)` — no
hardcoded property list, and no dependence on exact rule names.

This renderer diagrams which relations carry which structural
property, so you can sanity-check that the engine has the constraints
the puzzle requires.

Layout:

- **Nodes** — the relations / types a constraint quantifies over
  (boxes). A relation accumulates its *unary* properties as a badge:
  `color-loc «bijective»`, `next-to «symmetric»`.
- **Edges** — *binary* property facts (`(includes right-of next-to)`,
  `(square-unique next-to House)`) become a labelled edge between the
  two operands, coloured by property (shared palette).
- **Octagons** — any arity-≥3 property fact (none in the current
  examples) renders Levi-bipartite.

The constraint set is derived from the rule registry — **no hardcoded
property list** — so it tracks whatever rules a puzzle declares.
"""
from __future__ import annotations

from collections.abc import Iterable

from ..ir.types import Atom, KwPair, SForm
from .dot_util import GROUND_SHAPE, HYPER_SHAPE, TYPE_SHAPE, quote, value_label
from .palette import hash_color

# Kernel ontology keywords — declarations, not constraint facts.
_KERNEL_ONTOLOGY_HEADS = frozenset({"relation", "type", "instance"})


_NON_FACT_HEADS = frozenset({
    "rule", "hrule", "query", "trace", "config",
    "ontology", "facts", "reasoning",   # deprecated wrappers
})


def _flat_layer(form: SForm) -> str:
    """Layer of a flat fact form (mirrors `kb.from_ir._layer_of`): explicit
    `:layer` wins, else `:rule`/`:using`→reasoning, `:source`→fact, else
    ontology."""
    kw = {a.key.name for a in form.args if isinstance(a, KwPair)}
    for a in form.args:
        if (isinstance(a, KwPair) and a.key.name == "layer"
                and isinstance(a.value, Atom)):
            return a.value.name
    if "rule" in kw or "using" in kw:
        return "reasoning"
    if "source" in kw:
        return "fact"
    return "ontology"


def _is_ontology_form(f: object) -> bool:
    """A flat top-level form belonging to the ontology group (schema +
    property tags + type/is-a enumerations) — i.e. a relation decl or any
    ONTOLOGY-layer fact, but not a rule / query / trace / config / sourced
    (FACT) condition / derived (REASONING) fact."""
    if not (isinstance(f, SForm) and isinstance(f.head, Atom)):
        return False
    if f.head.name in _NON_FACT_HEADS:
        return False
    return _flat_layer(f) == "ontology"


def _declared_relations(ontology: SForm) -> set[str]:
    """Relation names declared via `(relation Name …)` in the ontology."""
    out: set[str] = set()
    for decl in ontology.args:
        if (isinstance(decl, SForm) and isinstance(decl.head, Atom)
                and decl.head.name == "relation" and decl.args
                and isinstance(decl.args[0], (Atom,))):
            out.add(decl.args[0].name)
    return out


def render_constraints(
    forms: SForm | Iterable[SForm], *, name: str = "constraints",
) -> str:
    """Render the structural-constraint scopes of a parsed program.

    ``forms`` is the tuple of top-level forms (as from ``parse``). The
    constraints are the rule-application facts in the `(ontology …)`
    form — heads that are neither kernel keywords nor declared
    relations (see the module docstring).
    """
    forms_l = [forms] if isinstance(forms, SForm) else list(forms)
    ontology = next(
        (f for f in forms_l if isinstance(f, SForm) and f.head.name == "ontology"),
        None,
    )
    if ontology is None:
        # Flat program (P1.7c): synthesise the ontology group from the
        # relation decls + ONTOLOGY-layer facts among the top-level forms.
        ontology = SForm(
            head=Atom(name="ontology"),
            args=tuple(f for f in forms_l if _is_ontology_form(f)),
        )
    declared_rel = _declared_relations(ontology)

    unary: dict[str, list[str]] = {}    # relation → [property, …]
    binary: list[tuple[str, str, str]] = []   # (property, a, b)
    nary: list[tuple[str, list[str]]] = []    # (property, [args])
    relations: list[str] = []           # node labels, first-seen order
    types: set[str] = set()             # binary-edge targets (drawn as types)

    def _note(label: str) -> None:
        if label not in relations:
            relations.append(label)

    if ontology is not None:
        for decl in ontology.args:
            if not isinstance(decl, SForm) or not isinstance(decl.head, Atom):
                continue
            prop = decl.head.name
            if prop in _KERNEL_ONTOLOGY_HEADS or prop in declared_rel:
                continue
            pos = [a for a in decl.args if not isinstance(a, KwPair)]
            labels = [value_label(a) for a in pos]
            if len(pos) == 1:
                unary.setdefault(labels[0], []).append(prop)
                _note(labels[0])
            elif len(pos) == 2:
                binary.append((prop, labels[0], labels[1]))
                _note(labels[0])
                _note(labels[1])
                types.add(labels[1])
            elif len(pos) >= 3:
                nary.append((prop, labels))
                for lbl in labels:
                    _note(lbl)

    lines = [f"digraph {name} {{", "  rankdir=LR;",
             '  node [fontname="Inter"];']
    if not relations:
        lines.append("  // no structural-constraint facts found "
                     "(rule-headed ontology facts)")
        lines.append("}")
        return "\n".join(lines)

    # Nodes — relations as boxes, badged with their unary properties.
    for rel in relations:
        props = unary.get(rel)
        shape = TYPE_SHAPE if (rel in types and rel not in unary) else GROUND_SHAPE
        if props:
            badge = ", ".join(props)
            label = f"{rel}\\n«{badge}»"
        else:
            label = rel
        lines.append(f"  {quote(rel)} [shape={shape}, label={quote(label)}];")

    # Binary properties → labelled, property-coloured edges.
    for prop, a, b in binary:
        colour = hash_color(prop)
        lines.append(f"  {quote(a)} -> {quote(b)} "
                     f'[label={quote(prop)}, color="{colour}", '
                     f'fontcolor="{colour}"];')

    # Arity-≥3 properties → Levi octagon.
    for i, (prop, args) in enumerate(nary, start=1):
        colour = hash_color(prop)
        h = quote(f"c{i}_{prop}")
        lines.append(f'  {h} [shape={HYPER_SHAPE}, label={quote(prop)}, '
                     f'color="{colour}", fontcolor="{colour}"];')
        for j, arg in enumerate(args, start=1):
            lines.append(f'  {h} -> {quote(arg)} [label="{j}", color="{colour}"];')

    lines.append("}")
    return "\n".join(lines)


__all__ = ["render_constraints"]
