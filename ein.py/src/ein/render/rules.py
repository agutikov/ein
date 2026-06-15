"""Rule → DOT renderer — S1.6.1.

Renders a single `(rule Name (params…) :match … :assert … :why … …)`
as a DOT digraph showing the rule's *pattern → conclusion* shape. Two
modes:

- **sidebyside** (alias ``"a"``, default) — `match` and `assert` as two
  separate clusters; the most readable view for a rule library.
- **overlay** (alias ``"c"``) — match solid + assert dashed on one
  graph; compact, used inline at rule-firing time.

What this renderer gets right that a naive edge-dump does not
(S1.6.1 T1.6.1.1):

- **Clean labels.** Node ids carry a per-panel suffix so the `match`
  and `assert` copies of a variable don't collapse into one shared
  node, but the suffix never reaches the *label* — both panels show
  `?a`. Variables render as diamonds, ground atoms as rectangles,
  wildcards as dashed diamonds (the shape legend).
- **Constraints are not relations.** A guard predicate `(neq ?a ?b)` /
  `(eq ?a ?b)` is computed, not data — it renders as a dotted,
  undirected ``≠`` / ``=`` link (``constraint=false``), never a
  labelled relation arrow.
- **Negation** `(not (R a b))` renders red with a ``¬`` prefix.
- **NAF guards** `(absent (and …))` render as their own
  ``cluster_absent`` sub-graph — inner conjuncts as arrows *inside* the
  cluster, the binder-local variables declared inside it, shared
  variables left outside — so the "no such match exists" reading
  survives instead of being flattened into overlay arrows.

This is the canonical rule renderer; :mod:`ein.ir.to_dot`
delegates `(rule …)` / `(rules …)` rendering here.
"""
from __future__ import annotations

from ..ir.types import Atom, KwPair, SForm
from .dot_util import (
    GROUND_SHAPE,
    HYPER_SHAPE,
    VAR_SHAPE,
    WILDCARD_ATTRS,
    quote,
    value_label,
)
from .palette import hash_color

# Constraint glyphs for the built-in guard predicates.
_PRED_GLYPH = {"neq": "≠", "eq": "="}
_NEG_COLOUR = "#d62728"      # red — negated premises / conclusions
_GUARD_COLOUR = "#888888"    # grey — NAF guard cluster chrome
_CONSTRAINT_COLOUR = "#555555"

# Rule-mode aliases. Default is side-by-side LHS|RHS clusters (most
# readable for rule libraries); compact overlay is opt-in. Legacy
# single-letter names "a"/"c" stay accepted.
_RULE_MODE_ALIASES = {
    "a": "a", "sidebyside": "a", "side-by-side": "a",
    "c": "c", "overlay": "c",
}


# ── Structural-head classification ─────────────────────────────────

def _is_predicate(name: str) -> bool:
    """Whether a head is a built-in guard predicate (`eq` / `neq`).

    Imported lazily: ``ir.to_dot`` imports this module, and
    ``inference.predicates`` transitively imports ``ir`` — a module-level
    import here would close that cycle.
    """
    from ..inference.predicates import is_predicate
    return is_predicate(name)


def _head_name(expr: SForm) -> str | None:
    """The head atom name, or None when the head is a `?var` / `_`."""
    return expr.head.name if isinstance(expr.head, Atom) else None


def _is_guard(expr: SForm) -> bool:
    return _head_name(expr) in ("absent", "forall")


def _positional(expr: SForm) -> list:
    return [a for a in expr.args if not isinstance(a, KwPair)]


# ── Node-occurrence analysis (for absent-cluster scoping) ──────────

def _arg_nodes(expr: SForm) -> list[str]:
    """Value-labels of all argument positions in this clause tree.

    Relation / predicate *heads* are edge labels, not nodes, so they
    are excluded. Recurses through `and` / `or` / `not` / `absent` /
    `forall` so a guard's inner argument nodes are surfaced.
    """
    hn = _head_name(expr)
    if hn in ("and", "or", "absent", "forall"):
        out: list[str] = []
        for child in expr.args:
            if isinstance(child, SForm):
                out.extend(_arg_nodes(child))
        return out
    if hn == "not":
        if expr.args and isinstance(expr.args[0], SForm):
            return _arg_nodes(expr.args[0])
        return []
    return [value_label(a) for a in _positional(expr)]


def _ordered_nodes(clauses: list[SForm]) -> list[str]:
    """First-seen order of argument-node labels across `clauses`."""
    seen: list[str] = []
    s: set[str] = set()
    for c in clauses:
        for n in _arg_nodes(c):
            if n not in s:
                s.add(n)
                seen.append(n)
    return seen


def _node_homes(top: list[SForm], guards: list[SForm]) -> dict[str, object]:
    """Assign each node a home: ``"top"`` or a guard index.

    A node lives in a guard cluster iff it appears in exactly one guard
    and nowhere in the top-level clauses (it is the guard's binder-local
    variable). Everything shared stays at the top so its edges cross
    into the cluster rather than pulling the node inside.
    """
    top_nodes = {n for c in top for n in _arg_nodes(c)}
    guard_nodes = [set(_arg_nodes(g)) for g in guards]
    homes: dict[str, object] = {}
    every = set(top_nodes)
    for gs in guard_nodes:
        every |= gs
    for n in every:
        in_guards = [i for i, gs in enumerate(guard_nodes) if n in gs]
        homes[n] = in_guards[0] if (n not in top_nodes and len(in_guards) == 1) else "top"
    return homes


# ── Shapes / ids ───────────────────────────────────────────────────

def _shape_attrs(nodelabel: str) -> str:
    if nodelabel == "_":
        return WILDCARD_ATTRS
    if nodelabel.startswith("?"):
        return f"shape={VAR_SHAPE}"
    return f"shape={GROUND_SHAPE}"


def _nid(nodelabel: str, suffix: str) -> str:
    """Quoted DOT id: clean label + a per-panel disambiguating suffix."""
    return quote(nodelabel + suffix)


# ── The renderer ───────────────────────────────────────────────────

class _RuleRenderer:
    """Accumulates the lines for one rule's DOT, panel by panel."""

    def __init__(self) -> None:
        self._hcount = 0

    def _fresh_hyper(self, label: str, suffix: str) -> str:
        self._hcount += 1
        return quote(f"h{self._hcount}_{label}{suffix}")

    # — one clause → lines (relations, constraints, negation, nested) —

    def _clause_lines(self, clause: SForm, *, suffix: str, dashed: bool,
                      negative: bool) -> list[str]:
        hn = _head_name(clause)
        if hn in ("and", "or"):
            out: list[str] = []
            for child in clause.args:
                if isinstance(child, SForm):
                    out.extend(self._clause_lines(
                        child, suffix=suffix, dashed=dashed, negative=negative))
            return out
        if hn == "not":
            inner = clause.args[0] if clause.args else None
            if isinstance(inner, SForm):
                return self._clause_lines(inner, suffix=suffix, dashed=dashed,
                                          negative=True)
            return []
        if hn is not None and _is_predicate(hn):
            return [self._constraint_line(clause, suffix=suffix)]
        # An absent/forall reached here (nested inside another clause) —
        # render its body inline as forbidden (negative); the top-level
        # case is handled as a cluster by `panel`.
        if hn in ("absent", "forall"):
            out = []
            for child in clause.args:
                if isinstance(child, SForm):
                    out.extend(self._clause_lines(child, suffix=suffix,
                                                  dashed=dashed, negative=True))
            return out
        return self._relation_lines(clause, suffix=suffix, dashed=dashed,
                                    negative=negative)

    def _relation_lines(self, clause: SForm, *, suffix: str, dashed: bool,
                        negative: bool) -> list[str]:
        head_label = value_label(clause.head)
        pos = _positional(clause)
        colour = _NEG_COLOUR if negative else hash_color(head_label.lstrip("?"))
        label = ("¬" + head_label) if negative else head_label
        attrs = [f"label={quote(label)}", f'color="{colour}"',
                 f'fontcolor="{colour}"']
        if dashed:
            attrs.append("style=dashed")
        attr_s = ", ".join(attrs)
        if len(pos) == 2:
            a, b = pos
            return [f"{_nid(value_label(a), suffix)} -> "
                    f"{_nid(value_label(b), suffix)} [{attr_s}];"]
        # n-ary (or arity 0/1): a Levi octagon list-node + role edges.
        h = self._fresh_hyper(head_label.lstrip("?"), suffix)
        lines = [f'{h} [shape={HYPER_SHAPE}, label={quote(label)}, '
                 f'color="{colour}", fontcolor="{colour}"];']
        edge_style = ", style=dashed" if dashed else ""
        for i, arg in enumerate(pos, start=1):
            lines.append(f'{h} -> {_nid(value_label(arg), suffix)} '
                         f'[label="{i}", color="{colour}"{edge_style}];')
        return lines

    def _constraint_line(self, clause: SForm, *, suffix: str) -> str:
        glyph = _PRED_GLYPH.get(clause.head.name, clause.head.name)
        pos = _positional(clause)
        if len(pos) != 2:  # defensive — eq/neq are binary
            return (f"// constraint {value_label(clause)}")
        a, b = pos
        return (f"{_nid(value_label(a), suffix)} -> "
                f"{_nid(value_label(b), suffix)} "
                f'[label="{glyph}", dir=none, style=dotted, '
                f'color="{_CONSTRAINT_COLOUR}", '
                f'fontcolor="{_CONSTRAINT_COLOUR}", constraint=false];')

    # — one pattern (match or assert) → (node decls, body lines) —

    def panel(self, pattern: SForm | None, *, suffix: str, dashed: bool,
              ) -> tuple[dict[str, str], list[str]]:
        """Render one pattern into (node-decl map, ordered body lines).

        Top-home nodes are returned in the decl map (the caller emits
        them at panel scope); guard-local nodes are declared inside
        their `cluster_absent` block within the body lines.
        """
        decls: dict[str, str] = {}
        body: list[str] = []
        if pattern is None:
            return decls, body
        hn = _head_name(pattern)
        clauses = ([c for c in pattern.args if isinstance(c, SForm)]
                   if hn == "and" else [pattern])
        top = [c for c in clauses if not _is_guard(c)]
        guards = [c for c in clauses if _is_guard(c)]
        homes = _node_homes(top, guards)

        # Panel-scope node declarations (shared / top-home nodes).
        for nl in _ordered_nodes(clauses):
            if homes.get(nl) == "top":
                decls.setdefault(_nid(nl, suffix),
                                 f"label={quote(nl)}, {_shape_attrs(nl)}")

        # Top-level clause lines.
        for c in top:
            body.extend(self._clause_lines(c, suffix=suffix, dashed=dashed,
                                           negative=False))

        # Each guard → its own cluster (local decls + inner lines).
        for gi, guard in enumerate(guards):
            kind = _head_name(guard) or "absent"
            cid = f"cluster_{kind}_{suffix.strip('_') or 'o'}_{gi}"
            glyph = "∄" if kind == "absent" else "∀"
            block = [
                f"subgraph {cid} {{",
                f'  label="{kind} ({glyph})"; style="dashed,rounded"; '
                f'color="{_GUARD_COLOUR}"; fontcolor="{_GUARD_COLOUR}";',
            ]
            for nl in _ordered_nodes([guard]):
                if homes.get(nl) == gi:
                    block.append(f"  {_nid(nl, suffix)} "
                                 f"[label={quote(nl)}, {_shape_attrs(nl)}];")
            for child in guard.args:
                if isinstance(child, SForm):
                    block.extend(
                        "  " + ln for ln in self._clause_lines(
                            child, suffix=suffix, dashed=dashed, negative=False))
            block.append("}")
            body.extend(block)
        return decls, body


# ── Field extraction ───────────────────────────────────────────────

def _extract(rule_sform: SForm) -> tuple[str, SForm | None, SForm | None]:
    """Return (name, match-expr, assert-expr) from a `(rule …)` form."""
    name = "anon"
    match_expr: SForm | None = None
    assert_expr: SForm | None = None
    for i, arg in enumerate(rule_sform.args):
        if i == 0 and isinstance(arg, Atom):
            name = arg.name
        elif isinstance(arg, KwPair):
            if arg.key.name == "match" and isinstance(arg.value, SForm):
                match_expr = arg.value
            elif arg.key.name == "assert" and isinstance(arg.value, SForm):
                assert_expr = arg.value
    return name, match_expr, assert_expr


# ── Public entry points ────────────────────────────────────────────

def render_rule(rule_sform: SForm, *, mode: str = "a") -> str:
    """Render one `(rule …)` form as a DOT digraph string.

    ``mode``: ``"sidebyside"`` / ``"a"`` (default) or ``"overlay"`` /
    ``"c"``. Legacy single-letter names are accepted.
    """
    canon = _RULE_MODE_ALIASES.get(mode)
    if canon is None:
        raise ValueError(
            f"unknown rule mode: {mode!r} "
            f"(expected 'sidebyside'/'a' or 'overlay'/'c')"
        )
    name, match_expr, assert_expr = _extract(rule_sform)
    safe = name.replace("-", "_").replace(" ", "_")
    if canon == "a":
        return _render_sidebyside(safe, name, match_expr, assert_expr)
    return _render_overlay(safe, name, match_expr, assert_expr)


def _render_sidebyside(safe: str, name: str, match: SForm | None,
                       assert_: SForm | None) -> str:
    r = _RuleRenderer()
    m_decls, m_body = r.panel(match, suffix="_L", dashed=False)
    a_decls, a_body = r.panel(assert_, suffix="_R", dashed=True)
    out = [f"digraph rule_{safe}_lhs_rhs {{",
           "  rankdir=TB;",
           f"  label={quote(name)}; labelloc=t;",
           '  subgraph cluster_lhs { label="match";']
    out += [f"    {ln}" for ln in _decl_lines(m_decls)]
    out += [f"    {ln}" for ln in m_body]
    out.append("  }")
    out.append('  subgraph cluster_rhs { label="assert";')
    out += [f"    {ln}" for ln in _decl_lines(a_decls)]
    out += [f"    {ln}" for ln in a_body]
    out.append("  }")
    out.append("}")
    return "\n".join(out)


def _render_overlay(safe: str, name: str, match: SForm | None,
                    assert_: SForm | None) -> str:
    r = _RuleRenderer()
    m_decls, m_body = r.panel(match, suffix="", dashed=False)
    a_decls, a_body = r.panel(assert_, suffix="", dashed=True)
    merged = dict(m_decls)
    for k, v in a_decls.items():
        merged.setdefault(k, v)
    out = [f"digraph rule_{safe}_overlay {{",
           f"  label={quote(name)}; labelloc=t;"]
    out += [f"  {ln}" for ln in _decl_lines(merged)]
    out += [f"  {ln}" for ln in m_body]
    out += [f"  {ln}" for ln in a_body]
    out.append("}")
    return "\n".join(out)


def _decl_lines(decls: dict[str, str]) -> list[str]:
    return [f"{nid} [{attrs}];" for nid, attrs in decls.items()]


def render_rules(rules_sform: SForm, *, mode: str = "a") -> str:
    """Render a `(rules …)` library — one digraph per child rule."""
    chunks = [render_rule(r, mode=mode)
              for r in rules_sform.args if isinstance(r, SForm)]
    return "\n\n".join(chunks)


__all__ = ["render_rule", "render_rules"]
