"""Shared helpers for the CLI subcommand modules.

Parse / KB-load sentinels reused by the ``render`` subcommand (and split out
of the former monolithic ``cli.py`` in P1.11 S1.11.4). The ``ir`` / ``kb``
inspection subcommands that also used these were removed; ``solve`` carries
its own phase-split loader (``_timed_load``).
"""
from __future__ import annotations

import sys
from pathlib import Path

from ..ir import IRParseError, parse
from ..kb import KBLoadError, KnowledgeBase


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
    the ``render lattice`` handler carries.
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
