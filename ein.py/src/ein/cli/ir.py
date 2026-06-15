"""``ein ir`` — IR utilities: parse / lint / dot."""
from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path

from ..ir import dump_canonical
from ..ir import to_dot as ir_to_dot
from ._common import _env_truthy, _parse_or_exit


def _cmd_ir_parse(args: argparse.Namespace) -> int:
    path = Path(args.file)
    nodes = _parse_or_exit(path)
    if nodes is None:
        return 1
    if getattr(args, "resolve", False):
        # D9 — splice `(import …)` inline + drop unreferenced library symbols,
        # emitting a standalone minimal `.ein`. base_dir = the file's dir so
        # file-relative imports resolve; std.* resolves regardless.
        from ..kb.from_ir import KBLoadError
        from ..kb.imports import resolve_and_minimize
        try:
            nodes = resolve_and_minimize(nodes, base_dir=path.parent)
        except KBLoadError as e:
            print(f"resolve error: {e}", file=sys.stderr)
            return 1
    sys.stdout.write(dump_canonical(nodes))
    return 0


def _cmd_ir_lint(args: argparse.Namespace) -> int:
    if _parse_or_exit(Path(args.file)) is None:
        return 1
    return 0


def _cmd_ir_dot(args: argparse.Namespace) -> int:
    nodes = _parse_or_exit(Path(args.file))
    if nodes is None:
        return 1
    levi = args.levi or _env_truthy(os.environ.get("EIN_RENDER_LEVI"))
    sys.stdout.write(ir_to_dot(nodes, rule_mode=args.rule_mode,
                               trace_view=args.trace_view, levi=levi))
    sys.stdout.write("\n")
    return 0


def add_parser(sub) -> None:
    ir = sub.add_parser("ir", help="IR utilities")
    ir_sub = ir.add_subparsers(dest="ir_cmd", required=True)

    ir_parse = ir_sub.add_parser("parse", help="parse and re-dump canonical IR")
    ir_parse.add_argument("file")
    ir_parse.add_argument(
        "--resolve", action="store_true",
        help="splice (import …) inline and tree-shake unreferenced library "
             "symbols, emitting a standalone minimal .ein (P1.8 D9)")
    ir_parse.set_defaults(func=_cmd_ir_parse)

    ir_lint = ir_sub.add_parser("lint", help="parse-only check; non-zero on error")
    ir_lint.add_argument("file")
    ir_lint.set_defaults(func=_cmd_ir_lint)

    ir_dot = ir_sub.add_parser(
        "dot",
        help="render parsed IR as DOT (per docs/kernel/ir/03-ein-lang/04_dot_rendering.md)",
    )
    ir_dot.add_argument("file")
    ir_dot.add_argument(
        "--rule-mode", choices=["sidebyside", "overlay"], default="sidebyside",
        help="rule rendering: 'sidebyside' LHS|RHS clusters, LR (default); "
             "'overlay' LHS solid + RHS dashed",
    )
    ir_dot.add_argument(
        "--trace-view",
        choices=["per-step", "aggregate", "dag", "a", "b", "c"], default="per-step",
        help="trace view: 'per-step' (default), 'aggregate', or 'dag' "
             "(derivation DAG); legacy a/b/c accepted",
    )
    ir_dot.add_argument(
        "--levi", action="store_true",
        help="Levi-bipartite hyperedge view (default: compact entity-style); "
             "also enabled by EIN_RENDER_LEVI=1",
    )
    ir_dot.set_defaults(func=_cmd_ir_dot)
