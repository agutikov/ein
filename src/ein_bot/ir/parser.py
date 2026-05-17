"""IR parser — thin Lark wrapper.

S1.1.1: returns the raw Lark `Tree` so the grammar can be exercised
end-to-end without committing to an AST shape. S1.1.2 will introduce
a typed `IRNode` (frozen dataclasses) and a `dump()` round-trip.
"""
from importlib.resources import files

from lark import Lark, Tree

_GRAMMAR = (files("ein_bot.ir") / "grammar.lark").read_text(encoding="utf-8")
_parser = Lark(_GRAMMAR, parser="earley", start="start")


def parse(text: str) -> Tree:
    """Parse IR source text into a Lark parse tree.

    The Earley parser is used during grammar bring-up so ambiguity
    surfaces early; S1.1.2 may switch to LALR(1) once the grammar
    stabilises.
    """
    return _parser.parse(text)
