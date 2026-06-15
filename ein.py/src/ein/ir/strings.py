"""S-expression string-literal escaping — the single source of truth for
IR + trace serialisation (S1.7c.32).

:mod:`ein.ir.dump` and :mod:`ein.trace.ast` both render Python
strings as ein string literals. Before this module each had its own
escaper, and the two trace sites escaped only ``\\`` and ``"`` — dropping
``\n`` / ``\t`` / ``\r`` on the way out, so a trace whose text carried a
newline did not round-trip (the parser's lexer unescapes the full set).
One function now serves all three sites.
"""
from __future__ import annotations


def escape_string_literal(s: str) -> str:
    r"""Render ``s`` as an ein S-expression string literal: the full escape
    set (``\`` ``"`` ``\n`` ``\t`` ``\r``) applied to the body in that
    order, wrapped in double quotes.

    The parser's lexer unescapes the same set, so
    ``parse(escape_string_literal(s))`` recovers ``s`` exactly. For a string
    with no control characters the result is identical to the old
    ``\\``/``"``-only escaping (the extra replacements are no-ops).
    """
    body = (
        s.replace("\\", "\\\\")
        .replace('"', '\\"')
        .replace("\n", "\\n")
        .replace("\t", "\\t")
        .replace("\r", "\\r")
    )
    return f'"{body}"'
