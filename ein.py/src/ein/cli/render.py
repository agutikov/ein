"""``ein render`` — DOT views of rules / constraints / the search lattice (S1.6.1)."""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

from ..render import render_constraints, render_lattice, render_rule, render_rules
from ._common import _load_kb_or_exit, _parse_or_exit, _rule_forms


def _cmd_render_rules(args: argparse.Namespace) -> int:
    from ..ir import Atom, SForm
    nodes = _parse_or_exit(Path(args.file))
    if nodes is None:
        return 1
    rule_forms = _rule_forms(nodes)
    if not rule_forms:
        print(f"no rule forms in {args.file}", file=sys.stderr)
        return 1
    # Render the rule library as one group (one digraph per rule).
    dot = render_rules(
        SForm(head=Atom(name="rules"), args=tuple(rule_forms)),
        mode=args.rule_mode,
    )
    sys.stdout.write(dot)
    sys.stdout.write("\n")
    return 0


def _cmd_render_rule(args: argparse.Namespace) -> int:
    from ..ir import Atom
    nodes = _parse_or_exit(Path(args.file))
    if nodes is None:
        return 1
    for r in _rule_forms(nodes):
        if (r.args and isinstance(r.args[0], Atom)
                and r.args[0].name == args.name):
            sys.stdout.write(render_rule(r, mode=args.rule_mode))
            sys.stdout.write("\n")
            return 0
    print(f"no rule named {args.name!r} in {args.file}", file=sys.stderr)
    return 1


def _cmd_render_constraints(args: argparse.Namespace) -> int:
    nodes = _parse_or_exit(Path(args.file))
    if nodes is None:
        return 1
    sys.stdout.write(render_constraints(nodes))
    sys.stdout.write("\n")
    return 0


def _cmd_render_lattice(args: argparse.Namespace) -> int:
    """Run a proof-producing solve and render its commitment lattice.

    Unlike the static `render` views, this *runs the engine* — the
    lattice DAG comes from a `LatticeProof`. ``gaps_solve`` /
    ``contradictions_solve`` populate the proof; the full view needs
    ``store_lattice`` (on by default).
    """
    kb = _load_kb_or_exit(Path(args.file))
    if kb is None:
        return 1
    from ..inference.monotonic import contradictions_solve, gaps_solve
    solve = contradictions_solve if args.mode == "contradictions" else gaps_solve
    verdict, _ = solve(
        kb, max_set_size=args.max_set_size,
        store_lattice=not args.no_store_lattice,
    )
    proof = getattr(verdict, "proof", None)
    if proof is None:
        print(f"{args.mode} produced no LatticeProof for {args.file}",
              file=sys.stderr)
        return 1
    sys.stdout.write(render_lattice(proof, view=args.view))
    sys.stdout.write("\n")
    return 0


def add_parser(sub) -> None:
    # ── render subcommand (S1.6.1) — DOT only, no SVG ──
    render_cmd = sub.add_parser(
        "render", help="DOT for rules / constraints (S1.6.1)",
    )
    render_sub = render_cmd.add_subparsers(dest="render_cmd", required=True)

    rule_mode_opts = dict(
        choices=["sidebyside", "overlay"], default="sidebyside",
        help="rule rendering: 'sidebyside' LHS|RHS clusters (default) "
             "or 'overlay' (LHS solid + RHS dashed)",
    )

    r_rules = render_sub.add_parser("rules", help="one DOT digraph per rule")
    r_rules.add_argument("file")
    r_rules.add_argument("--rule-mode", **rule_mode_opts)
    r_rules.set_defaults(func=_cmd_render_rules)

    r_rule = render_sub.add_parser("rule", help="a single rule's DOT, by name")
    r_rule.add_argument("file")
    r_rule.add_argument("--name", required=True, help="rule name to render")
    r_rule.add_argument("--rule-mode", **rule_mode_opts)
    r_rule.set_defaults(func=_cmd_render_rule)

    r_con = render_sub.add_parser(
        "constraints", help="constraint-scope DOT (structural properties)",
    )
    r_con.add_argument("file")
    r_con.set_defaults(func=_cmd_render_constraints)

    r_lat = render_sub.add_parser(
        "lattice",
        help="commitment-lattice / proof-DAG DOT (runs a solve, S1.6.3)",
    )
    r_lat.add_argument("file")
    r_lat.add_argument("--mode", choices=["gaps", "contradictions"], default="gaps",
                       help="proof-producing solve to run (default: gaps)")
    r_lat.add_argument("--view", choices=["full", "solution"], default="full",
                       help="'full' every commitment (default) or 'solution' frontier")
    r_lat.add_argument("--max-set-size", type=int, default=3,
                       help="commitment-set depth cap (default 3)")
    r_lat.add_argument("--no-store-lattice", action="store_true",
                       help="don't store the per-commitment lattice (full view "
                            "then falls back to the solution frontier)")
    r_lat.set_defaults(func=_cmd_render_lattice)
