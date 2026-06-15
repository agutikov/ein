"""Console entry point.

Subcommands:

    ein ir parse <file>     # parse IR, dump canonical text to stdout
    ein ir lint  <file>     # parse-only; exit non-zero on errors
    ein ir dot   <file>     # render IR as one DOT per top-level form
    ein kb dot   <file>     # render the loaded KB as a unified DOT
                                # (S1.2.4 — per docs/kernel/ir/03-ein-lang/04_dot_rendering.md)
    ein render rules       <file>             # one DOT per rule (S1.6.1)
    ein render rule        <file> --name=N    # a single rule's DOT
    ein render constraints <file>             # constraint-scope DOT
    ein render lattice     <file>             # commitment-lattice DOT (runs a solve)
    ein solve <file> --trace=out.md           # markdown trace (S1.6.4)

All `render`/`dot` commands emit DOT to stdout only; rasterising to SVG
is a shell-script concern (see utils/render_examples.sh). `solve`
writes a self-contained markdown trace with inline `dot` blocks.

Invoked via the ``ein`` console script or ``python -m ein.cli``.
"""
from __future__ import annotations

import argparse
import os
import sys
from collections.abc import Sequence
from pathlib import Path

from .ir import IRParseError, dump_canonical, parse
from .ir import to_dot as ir_to_dot
from .kb import KBLoadError, KnowledgeBase, Layer
from .render import render_constraints, render_lattice, render_rule, render_rules


def _env_truthy(value: str | None) -> bool:
    """True for the usual affirmative env-var spellings."""
    return (value or "").strip().lower() in ("1", "true", "yes", "on")


def _cmd_ir_parse(args: argparse.Namespace) -> int:
    path = Path(args.file)
    nodes = _parse_or_exit(path)
    if nodes is None:
        return 1
    if getattr(args, "resolve", False):
        # D9 — splice `(import …)` inline + drop unreferenced library symbols,
        # emitting a standalone minimal `.ein`. base_dir = the file's dir so
        # file-relative imports resolve; std.* resolves regardless.
        from .kb.from_ir import KBLoadError
        from .kb.imports import resolve_and_minimize
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


_LAYER_BY_NAME = {
    "ontology": Layer.ONTOLOGY,
    "fact": Layer.FACT,
    "facts": Layer.FACT,   # alias — the IR top-level block is `(facts …)`.
    "reasoning": Layer.REASONING,
}


def _cmd_kb_dot(args: argparse.Namespace) -> int:
    """Render the loaded KB as a single unified Graphviz digraph."""
    kb = _load_kb_or_exit(Path(args.file))
    if kb is None:
        return 1
    # Parse --layers spec; default to all.
    if args.layers:
        try:
            layer_set = tuple(
                _LAYER_BY_NAME[name.strip().lower()]
                for name in args.layers.split(",")
            )
        except KeyError as e:
            valid = ",".join(sorted(_LAYER_BY_NAME))
            print(f"unknown layer {e}; pick from {valid}", file=sys.stderr)
            return 2
    else:
        layer_set = (Layer.ONTOLOGY, Layer.FACT, Layer.REASONING)
    sys.stdout.write(kb.to_dot(
        layers=layer_set,
        colour_by=args.colour_by,
        include_types=not args.no_types,
        include_instances=not args.no_instances,
        name=args.graph_name,
    ))
    sys.stdout.write("\n")
    return 0


# ── render subcommand (S1.6.1) — DOT for rules / constraints ──


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
    from .ir import SForm
    return [n for n in nodes
            if isinstance(n, SForm) and n.head.name in ("rule", "hrule")]


def _cmd_render_rules(args: argparse.Namespace) -> int:
    from .ir import Atom, SForm
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
    from .ir import Atom
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
    from .inference.monotonic import contradictions_solve, gaps_solve
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


def _cmd_solve(args: argparse.Namespace) -> int:
    """Solve a puzzle (S1.6.4 / S1.7a.6).

    ``--mode=solve`` runs the sound search (:func:`solve`) and prints the
    answer in English (with ``--trace`` it also writes the markdown trace).
    ``--mode=gaps`` / ``--mode=contradictions`` run the proof-producing
    entries (``store_lattice`` so the lattice DAG renders), linearize the
    lattice into a story, and write the markdown (to ``--trace`` or stdout).
    """
    kb = _load_kb_or_exit(Path(args.file))
    if kb is None:
        return 1

    # P1.7a S1.7a.6 — sound solve() + English answer line.
    if args.mode == "solve":
        from .inference.monotonic import solve as sound_solve
        from .trace import linearize, render_answer, render_markdown
        verdict, stats = sound_solve(
            kb, stop_after=None if args.exhaustive else 1,
            max_set_size=args.max_set_size,
        )
        print(render_answer(verdict, exhausted=stats.exhausted))
        if args.trace and args.trace != "-":
            md = render_markdown(
                linearize(verdict, diagrams=not args.no_diagrams,
                          full_kb_snapshots=args.full_kb_snapshots,
                          relevant=args.relevant),
                mode="reorder" if args.reorder else "engine",
                diagrams=not args.no_diagrams,
            )
            Path(args.trace).write_text(md, encoding="utf-8")
            print(f"wrote {args.trace}", file=sys.stderr)
        return 0

    from .inference.monotonic import contradictions_solve, gaps_solve
    from .trace import linearize, render_markdown
    solve = contradictions_solve if args.mode == "contradictions" else gaps_solve
    verdict, _ = solve(kb, max_set_size=args.max_set_size, store_lattice=True)
    trace = linearize(
        verdict, diagrams=not args.no_diagrams,
        full_kb_snapshots=args.full_kb_snapshots, relevant=args.relevant,
    )
    md = render_markdown(
        trace, mode="reorder" if args.reorder else "engine",
        diagrams=not args.no_diagrams,
    )
    if args.trace and args.trace != "-":
        Path(args.trace).write_text(md, encoding="utf-8")
        print(f"wrote {args.trace} ({len(trace.steps)} steps, "
              f"{len(trace.reductios)} refuted)", file=sys.stderr)
    else:
        sys.stdout.write(md)
    return 0


def _add_ir_parser(sub) -> None:
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


def _add_kb_parser(sub) -> None:
    kb_cmd = sub.add_parser("kb", help="Knowledge-base utilities")
    kb_sub = kb_cmd.add_subparsers(dest="kb_cmd", required=True)

    kb_dot = kb_sub.add_parser(
        "dot",
        help="render the loaded KB as a unified DOT graph (S1.2.4)",
    )
    kb_dot.add_argument("file")
    kb_dot.add_argument(
        "--layers",
        default=None,
        help="comma-separated layer subset (ontology,facts,reasoning); default: all",
    )
    kb_dot.add_argument(
        "--colour-by",
        choices=["relation", "layer", "none"],
        default="relation",
        help="colour edges by relation (default), by layer, or none",
    )
    kb_dot.add_argument(
        "--no-types",
        action="store_true",
        help="omit Type nodes (boxes)",
    )
    kb_dot.add_argument(
        "--no-instances",
        action="store_true",
        help="omit Instance nodes (ovals)",
    )
    kb_dot.add_argument(
        "--graph-name",
        default="kb",
        help="DOT graph name (default 'kb')",
    )
    kb_dot.set_defaults(func=_cmd_kb_dot)


def _add_render_parser(sub) -> None:
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


def _add_solve_parser(sub) -> None:
    # ── solve subcommand (S1.6.4) — markdown trace ──
    solve_cmd = sub.add_parser(
        "solve", help="solve a puzzle and write its markdown trace (S1.6.4)",
    )
    solve_cmd.add_argument("file")
    solve_cmd.add_argument("--trace", default=None, metavar="OUT.md",
                           help="write the markdown trace here (default: stdout; "
                                "'-' for stdout)")
    solve_cmd.add_argument("--mode", choices=["solve", "gaps", "contradictions"],
                           default="gaps",
                           help="solve = sound search + English answer (S1.7a.6); "
                                "gaps/contradictions = proof-producing markdown "
                                "trace (default: gaps)")
    solve_cmd.add_argument("--exhaustive", action="store_true",
                           help="--mode=solve: exhaust the lattice to certify "
                                "unique/ambiguous/unsat (slower) rather than "
                                "stopping at the first solution")
    solve_cmd.add_argument("--max-set-size", type=int, default=3,
                           help="commitment-set depth cap (default 3)")
    solve_cmd.add_argument("--no-diagrams", action="store_true",
                           help="suppress all inline dot blocks")
    solve_cmd.add_argument("--full-kb-snapshots", action="store_true",
                           help="append a whole-KB snapshot of the final state")
    solve_cmd.add_argument("--reorder", action="store_true",
                           help="cluster steps by target entity instead of engine order")
    solve_cmd.add_argument("--relevant", action="store_true",
                           help="prune to the goal-relevant slice (drop redundant + "
                                "provenance backtrack from the solution; S1.6.5)")
    solve_cmd.set_defaults(func=_cmd_solve)


def _build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="ein",
        description="Graph-based reasoner for Zebra-style puzzles.",
    )
    sub = p.add_subparsers(dest="cmd", required=True)
    _add_ir_parser(sub)
    _add_kb_parser(sub)
    _add_render_parser(sub)
    _add_solve_parser(sub)
    return p


def main(argv: Sequence[str] | None = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
