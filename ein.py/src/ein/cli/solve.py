"""``ein solve`` — solve a puzzle and write its markdown trace (S1.6.4)."""
from __future__ import annotations

import argparse
from pathlib import Path

from ._common import _load_kb_or_exit


def _cmd_solve(args: argparse.Namespace) -> int:
    """Solve a puzzle (S1.6.4 / S1.7a.6).

    ``--mode=solve`` runs the sound search (:func:`solve`) and prints the
    answer in English (with ``--trace`` it also writes the markdown trace).
    ``--mode=gaps`` / ``--mode=contradictions`` run the proof-producing
    entries (``store_lattice`` so the lattice DAG renders), linearize the
    lattice into a story, and write the markdown (to ``--trace`` or stdout).
    """
    import sys

    kb = _load_kb_or_exit(Path(args.file))
    if kb is None:
        return 1

    # P1.7a S1.7a.6 — sound solve() + English answer line.
    if args.mode == "solve":
        from ..inference.monotonic import solve as sound_solve
        from ..trace import linearize, render_answer, render_markdown
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

    from ..inference.monotonic import contradictions_solve, gaps_solve
    from ..trace import linearize, render_markdown
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


def add_parser(sub) -> None:
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
