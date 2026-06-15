"""Shared helpers for the CLI subcommand modules.

Parse / KB-load sentinels and small utilities reused across
``ir`` / ``kb`` / ``render`` / ``solve``. Split out of the former
monolithic ``cli.py`` in P1.11 (S1.11.4) so each subcommand module
imports from one place rather than from a sibling command.
"""
from __future__ import annotations

import sys
from pathlib import Path

from ..ir import IRParseError, parse
from ..kb import KBLoadError, KnowledgeBase, Layer

_LAYER_BY_NAME = {
    "ontology": Layer.ONTOLOGY,
    "fact": Layer.FACT,
    "facts": Layer.FACT,   # alias — the IR top-level block is `(facts …)`.
    "reasoning": Layer.REASONING,
}


def _env_truthy(value: str | None) -> bool:
    """True for the usual affirmative env-var spellings."""
    return (value or "").strip().lower() in ("1", "true", "yes", "on")


def _parse_or_exit(path: Path):
    """Parse a file, printing a parse error to stderr and returning None."""
    try:
        return parse(path.read_text(encoding="utf-8"), filename=str(path))
    except IRParseError as e:
        print(e, file=sys.stderr)
        return None


def _load_kb_or_exit(path: Path):
    """Parse + build a :class:`KnowledgeBase`, or print the failure and
    return None.

    Mirrors :func:`_parse_or_exit`'s sentinel convention — return None on
    error and let the caller ``return 1``; it does *not* call ``sys.exit``.
    Collapses the parse (IRParseError) + KB-build (KBLoadError) bail-out
    the ``kb dot`` / ``render lattice`` / ``solve`` handlers each carried
    verbatim.
    """
    nodes = _parse_or_exit(path)
    if nodes is None:
        return None
    try:
        # base_dir = the puzzle's directory so file-relative `(import …)`
        # forms resolve against it (S1.8.A3); `std.*` resolves regardless.
        return KnowledgeBase.from_ir(nodes, base_dir=path.parent)
    except KBLoadError as e:
        print(f"kb load error: {e}", file=sys.stderr)
        return None


def _rule_forms(nodes):
    """All flat `(rule …)` / `(hrule …)` declarations among parsed nodes
    (P1.7c — the `(rules …)` block wrapper is gone)."""
    from ..ir import SForm
    return [n for n in nodes
            if isinstance(n, SForm) and n.head.name in ("rule", "hrule")]
