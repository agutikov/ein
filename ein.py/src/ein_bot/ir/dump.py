"""Canonical IR printer — S1.1.2 T1.1.2.4.

Two entry points:

- `dump(nodes)` / `dump_canonical(nodes)` — pretty multi-line.  A
  form is rendered on one line if its compact form fits in the
  column budget (default 80); otherwise its arguments are broken to
  separate indented lines.

- `dump_compact(node)` — single-line; useful for log messages and
  trace fragments.

Round-trip target: `parse(dump_canonical(parse(text))) == parse(text)`
modulo `Loc` (excluded from `__eq__` on every IR node).
"""
from __future__ import annotations

from collections.abc import Iterable

from .strings import escape_string_literal
from .types import (
    Atom,
    Int,
    IRNode,
    Keyword,
    KwPair,
    Range,
    SForm,
    String,
    Var,
    Wildcard,
)

_INDENT = "  "
_DEFAULT_WIDTH = 80


def _is_headless(form: SForm) -> bool:
    """True for synthetic-head atoms (`@params`, `@empty`)."""
    return isinstance(form.head, Atom) and form.head.name.startswith("@")


def _atom_text(node: Atom | Var | Wildcard) -> str:
    if isinstance(node, Atom):
        return node.name
    if isinstance(node, Var):
        return f"?{node.name}"
    return "_"


def _string_text(node: String) -> str:
    return escape_string_literal(node.value)


def _range_text(node: Range) -> str:
    high = "*" if node.high is None else str(node.high)
    return f"{node.low}..{high}"


def _compact(node: IRNode) -> str:
    """Single-line rendering of any IR node."""
    if isinstance(node, (Atom, Var, Wildcard)):
        return _atom_text(node)
    if isinstance(node, Keyword):
        return f":{node.name}"
    if isinstance(node, String):
        return _string_text(node)
    if isinstance(node, Int):
        return str(node.value)
    if isinstance(node, Range):
        return _range_text(node)
    if isinstance(node, KwPair):
        return f":{node.key.name} {_compact(node.value)}"
    if isinstance(node, SForm):
        inner = " ".join(_compact(a) for a in node.args)
        if _is_headless(node):
            return f"({inner})" if inner else "()"
        head = _atom_text(node.head)
        return f"({head} {inner})" if inner else f"({head})"
    raise TypeError(f"not an IRNode: {type(node).__name__}")


def _pretty(node: IRNode, indent: int, width: int) -> str:
    """Multi-line render: try compact; if too wide, break SForm args."""
    compact = _compact(node)
    cur = indent * len(_INDENT)
    if cur + len(compact) <= width or not isinstance(node, SForm):
        return compact

    if not node.args:
        return compact  # `(head)` always fits or has no breaking room

    head = "" if _is_headless(node) else _atom_text(node.head)
    open_line = f"({head}" if head else "("
    pad = _INDENT * (indent + 1)
    rendered_args = [_pretty(a, indent + 1, width) for a in node.args]
    body = "\n".join(f"{pad}{a}" for a in rendered_args)
    return f"{open_line}\n{body})"


def dump_compact(node: IRNode) -> str:
    """Single-line render."""
    return _compact(node)


def dump_canonical(
    nodes: IRNode | Iterable[IRNode], *, width: int = _DEFAULT_WIDTH,
) -> str:
    """Pretty render. Accepts a single node or an iterable of nodes.

    Top-level forms are separated by a blank line.
    """
    if isinstance(nodes, (Atom, Var, Wildcard, Keyword, String, Int,
                          Range, KwPair, SForm)):
        return _pretty(nodes, 0, width)
    chunks = [_pretty(n, 0, width) for n in nodes]
    return "\n\n".join(chunks) + ("\n" if chunks else "")


# Convenience alias.
dump = dump_canonical


__all__ = ["dump", "dump_canonical", "dump_compact"]
