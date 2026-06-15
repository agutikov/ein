"""Console entry point.

Subcommands:

    ein ir parse <file>     # parse IR, dump canonical text to stdout
    ein ir lint  <file>     # parse-only; exit non-zero on errors
    ein ir dot   <file>     # render IR as one DOT per top-level form
    ein kb dot   <file>     # render the loaded KB as a unified DOT
    ein render rules       <file>             # one DOT per rule (S1.6.1)
    ein render rule        <file> --name=N    # a single rule's DOT
    ein render constraints <file>             # constraint-scope DOT
    ein render lattice     <file>             # commitment-lattice DOT (runs a solve)
    ein solve <file> --trace=out.md           # markdown trace (S1.6.4)

Engine-runner subcommands (promoted from the former ``demo/`` scripts in
P1.11 S1.11.3; each delegates to its module's ``main(argv)``):

    ein saturate  <file>            # saturate to fixpoint + timing/state dump
    ein search    <file>            # sound set-search solve() + engine stats
    ein lattice   <file> --gaps     # gaps / contradictions lattice search + stats
    ein profile   <file>            # cProfile a solve() run
    ein symmetric                   # symmetric-closure micro-benchmark

All `render`/`dot` commands emit DOT to stdout only; rasterising to SVG
is a shell-script concern (see utils/render_examples.sh). `solve`
writes a self-contained markdown trace with inline `dot` blocks.

Invoked via the ``ein`` console script or ``python -m ein.cli``.
"""
from __future__ import annotations

import argparse
import importlib
import sys
from collections.abc import Sequence

from . import ir, kb, render, solve

# Engine-runner subcommands whose implementation lives in a sibling module
# exposing ``main(argv) -> int``. Dispatch is intercepted in :func:`main`
# *before* argparse (so the module's own argparse owns ``<cmd> --help`` and
# flag parsing, keeping the promoted scripts' surface byte-identical — P1.11).
# They are still registered as bare subparsers below purely so ``ein --help``
# lists them; argparse never actually parses them (the intercept catches the
# name first, sidestepping argparse REMAINDER's leading-``-`` mishandling).
_DELEGATED: dict[str, str] = {
    "saturate": "saturate to a least fixpoint + phase timings / state dump",
    "search": "sound set-search solve() + engine stats (the ein.inference.monotonic entry)",
    "lattice": "gaps / contradictions lattice search + stats",
    "profile": "cProfile a solve() run (perf diagnostic)",
    "symmetric": "symmetric-closure micro-benchmark (synthetic fixture)",
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
    ir.add_parser(sub)
    kb.add_parser(sub)
    render.add_parser(sub)
    solve.add_parser(sub)
    _add_delegated(sub)
    return p


def main(argv: Sequence[str] | None = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    # Intercept engine-runner subcommands before argparse — they own their
    # own flag parsing via the delegated module's main(argv).
    if argv and argv[0] in _DELEGATED:
        return _run_delegated(argv[0], argv[1:])
    parser = _build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
