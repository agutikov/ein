"""Console entry point.

Subcommands:

    ein render rules       <file>             # one DOT per rule (S1.6.1)
    ein render rule        <file> --name=N    # a single rule's DOT
    ein render constraints <file>             # constraint-scope DOT
    ein render lattice     <file>             # commitment-lattice DOT (runs a solve)
    ein saturate <file>                       # saturate to fixpoint + timing/state dump
    ein solve    <file>                       # solve: print the solution(s) / unsat core
    ein solve    <file> --exhaustive          # certify unique / ambiguous / unsat
    ein solve    <file> --trace=out.md        # + markdown derivation trace (to a file)

The IR/KB inspection subcommands (``ir parse|lint|dot``, ``kb dot``) were
removed, and the ``profile`` / ``symmetric`` engine-runners were moved to
standalone scripts under ``utils/`` (``utils/profile_solve.py``,
``utils/symmetric_bench.py``), leaving the three operational commands:
``render``, ``saturate``, ``solve``. (The earlier ``search`` / ``lattice``
runners were merged into ``solve`` — 2026-06-16.)

All `render` commands emit DOT to stdout only; rasterising to SVG is a
shell-script concern (see utils/render_examples.sh). `solve` writes a
self-contained markdown trace with inline `dot` blocks via `--trace`.

Invoked via the ``ein`` console script or ``python -m ein.cli``.
"""
from __future__ import annotations

import argparse
import importlib
import sys
from collections.abc import Sequence

from . import render, solve

# Engine-runner subcommand whose implementation lives in a sibling module
# exposing ``main(argv) -> int``. Dispatch is intercepted in :func:`main`
# *before* argparse (so the module's own argparse owns ``saturate --help`` and
# its flag parsing). Registered as a bare subparser below purely so
# ``ein --help`` lists it (argparse never actually parses it).
_DELEGATED: dict[str, str] = {
    "saturate": "saturate to a least fixpoint + phase timings / state dump",
}


def _run_delegated(name: str, rest: Sequence[str]) -> int:
    mod = importlib.import_module(f"ein.cli.{name}")
    return mod.main(list(rest))


def _add_delegated(sub) -> None:
    for name, help_text in _DELEGATED.items():
        sub.add_parser(name, help=help_text, add_help=False)


def _build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="ein",
        description="Graph-based relation algebra solver for Zebra-style logic puzzles.",
    )
    sub = p.add_subparsers(dest="cmd", required=True)
    render.add_parser(sub)
    solve.add_parser(sub)
    _add_delegated(sub)
    return p


def main(argv: Sequence[str] | None = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    # Intercept the engine-runner subcommand before argparse — it owns its own
    # flag parsing via the delegated module's main(argv).
    if argv and argv[0] in _DELEGATED:
        return _run_delegated(argv[0], argv[1:])
    parser = _build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
