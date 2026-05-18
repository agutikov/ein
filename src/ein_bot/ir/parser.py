"""IR parser — Lark grammar + typed-AST Transformer.

The Lark grammar in `grammar.lark` is the source of truth (the M2.P2.3
GBNF lift is derived from it). This module wires the grammar to the
typed `IRNode` AST defined in `types.py` via the Transformer in
`ast.py`.

Public API:

    parse(text, *, filename=None) -> tuple[SForm, ...]
        Returns the top-level forms.

    parse_tree(text) -> lark.Tree
        Returns the raw Lark parse tree (escape hatch, used by tests
        that exercise the grammar without committing to the AST shape).

Errors are wrapped in `IRParseError` carrying `file:line:col`.
"""
from __future__ import annotations

from importlib.resources import files

from lark import Lark, Tree
from lark.exceptions import UnexpectedInput

from .ast import to_ast
from .types import SForm

_GRAMMAR = (files("ein_bot.ir") / "grammar.lark").read_text(encoding="utf-8")
_parser = Lark(
    _GRAMMAR,
    parser="earley",
    start="start",
    propagate_positions=True,
)


class IRParseError(SyntaxError):
    """Raised on malformed IR. Message is `file:line:col: <detail>`."""


def parse_tree(text: str) -> Tree:
    """Return the raw Lark parse tree — escape hatch for grammar tests."""
    return _parser.parse(text)


def parse(text: str, *, filename: str | None = None) -> tuple[SForm, ...]:
    """Parse IR source into a tuple of typed top-level `SForm`s.

    Raises `IRParseError` with `file:line:col` on any syntactic
    failure. The optional `filename` is recorded in `Loc` and in
    error messages.
    """
    try:
        tree = _parser.parse(text)
    except UnexpectedInput as e:
        loc = f"{filename or '<string>'}:{e.line}:{e.column}"
        context = e.get_context(text).rstrip("\n")
        raise IRParseError(f"{loc}: unexpected input\n{context}") from e
    return to_ast(tree, filename=filename)
