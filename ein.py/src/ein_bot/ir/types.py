"""Typed AST for the IR — S1.1.2 T1.1.2.1.

The Lark grammar in `grammar.lark` defines the surface syntax; this
module gives the parse result a stable, walkable shape. Every node is
a frozen dataclass with an optional `Loc`; `Loc` is absent on nodes
synthesised by the engine (rule conclusions, trace fabrications, …).

`Loc` is excluded from `__eq__` / `__hash__` via `field(compare=False)`
so two structurally-identical trees from different sources compare
equal. The round-trip property `parse(dump(parse(x))) == parse(x)`
relies on this.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Union


@dataclass(frozen=True)
class Loc:
    file: str
    line: int
    col: int


def _loc():
    """Sentinel for the loc field: excluded from compare/hash/repr."""
    return field(default=None, compare=False, hash=False, repr=False)


@dataclass(frozen=True)
class Atom:
    name: str
    loc: Loc | None = _loc()


@dataclass(frozen=True)
class Var:
    """A pattern variable `?name`. The `?` is not stored."""
    name: str
    loc: Loc | None = _loc()


@dataclass(frozen=True)
class Keyword:
    """A `:name` keyword, always appearing as the key of a KwPair."""
    name: str
    loc: Loc | None = _loc()


@dataclass(frozen=True)
class Wildcard:
    loc: Loc | None = _loc()


@dataclass(frozen=True)
class String:
    """A double-quoted string. `value` is the unescaped Python str."""
    value: str
    loc: Loc | None = _loc()


@dataclass(frozen=True)
class Int:
    value: int
    loc: Loc | None = _loc()


@dataclass(frozen=True)
class Range:
    """A `low..high` token; `high is None` for the `*` upper bound."""
    low: int
    high: int | None
    loc: Loc | None = _loc()


IRNode = Union[
    "Atom", "Var", "Keyword", "Wildcard", "String", "Int", "Range",
    "KwPair", "SForm",
]


@dataclass(frozen=True)
class KwPair:
    """A `:key value` pair inside a list."""
    key: Keyword
    value: IRNode
    loc: Loc | None = _loc()


@dataclass(frozen=True)
class SForm:
    """A parenthesised form `(head args...)`.

    `head` is an `Atom` for normal forms (including the `=` atom), but
    can be `Var` or `Wildcard` inside pattern interiors — e.g.
    `(?rel ?a ?b)` or `(_ ?a ?b)`. Synthetic atoms whose name starts
    with `@` (e.g. `@params`, `@empty`) mark headless parens in the
    grammar (`rule_params`, empty `()`); the dumper emits them
    without a head.
    """
    head: Atom | Var | Wildcard
    args: tuple[IRNode, ...]
    loc: Loc | None = _loc()

    def leading_symbol(self) -> str | None:
        """The first positional :class:`Atom` arg's name — e.g. a `(step s3
        …)` record id — or ``None`` if the form has no leading symbol.

        Generic SForm accessor shared by the trace DOT renderers
        (:mod:`ein_bot.ir.to_dot`) and the trace AST parser
        (:mod:`ein_bot.trace.ast`) so the step-field extraction lives once
        (S1.7c.28).
        """
        return next((a.name for a in self.args if isinstance(a, Atom)), None)

    def kw_map(self) -> dict[str, IRNode]:
        """The ``{keyword: value}`` map over this form's :class:`KwPair`
        args. Last wins on a repeated key — the same outcome as the
        positional ``if key == …`` scans this replaces (S1.7c.28)."""
        return {a.key.name: a.value for a in self.args if isinstance(a, KwPair)}


__all__ = [
    "Atom",
    "IRNode",
    "Int",
    "Keyword",
    "KwPair",
    "Loc",
    "Range",
    "SForm",
    "String",
    "Var",
    "Wildcard",
]
