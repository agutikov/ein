"""Run the monotonic set-search engine on a .ein file.

Mirrors :mod:`bench_solve`'s CLI shape but reaches into
``inference/monotonic/`` instead of ``inference/tree/``. Output
is one-shot: verdict + goal bindings (if Solution) + per-run
stats + elapsed wall. With ``--dump-states DIR``, produce a
minimal monotonic dump (root snapshot per layer +
``00_timeline.jsonl``) — currently a no-op (S1.5b.7).
"""
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

# Make `from ein_bot.…` resolve when running from a checkout.
sys.path.insert(
    0, str(Path(__file__).resolve().parents[1] / "src"),
)

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.monotonic.solver import monotonic_solve
from ein_bot.inference.monotonic.state_dump import MonotonicDumper
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _build_argparser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(
        description=__doc__.split("\n", 1)[0],
    )
    ap.add_argument("puzzle", type=Path,
                    help="path to .ein puzzle file")
    ap.add_argument("--max-set-size", type=int, default=5,
                    help="largest commitment size to enumerate "
                         "(default: 5)")
    ap.add_argument("--dump-states", type=Path, default=None,
                    help="if set, write a minimal monotonic dump "
                         "to this directory")
    ap.add_argument("--max-time", type=float, default=None,
                    help="abort after N seconds")
    ap.add_argument("--verbose", "-v", action="store_true",
                    help="per-layer progress")
    return ap


def _query_goal_bindings(kb: KnowledgeBase) -> list[dict[str, str]]:
    """Run the query's ``:goal`` pattern against ``kb``; return
    binding rows. Mirrors bench_solve.query_goal_bindings."""
    from ein_bot.inference.compile import JoinPlan, compile_pattern
    from ein_bot.inference.match import run as match_run

    if kb is None or kb.query is None:
        return []
    for kp in kb.query.kw_pairs:
        if kp.key.name == "goal":
            steps = compile_pattern(kp.value, {})
            plan = JoinPlan(
                rule_name="<query>",
                activator_args=(),
                bindings_seed={},
                steps=tuple(steps),
                assert_template=None,
                why="",
            )
            return [dict(b) for b, _premises in match_run(plan, kb)]
    return []


def main(argv: list[str] | None = None) -> int:
    args = _build_argparser().parse_args(argv)
    text = args.puzzle.read_text()
    kb = KnowledgeBase.from_ir(parse(text))

    config = SolverConfig()  # default; flags can extend later

    dumper = (
        MonotonicDumper(out_dir=args.dump_states)
        if args.dump_states is not None else None
    )

    t0 = time.perf_counter()
    verdict, stats = monotonic_solve(
        kb,
        max_set_size=args.max_set_size,
        config=config,
        dumper=dumper,
    )
    elapsed = time.perf_counter() - t0

    print(f"file              {args.puzzle}")
    print(f"verdict           {type(verdict).__name__}")

    sol_kb = getattr(verdict, "kb", None)
    if sol_kb is None and getattr(verdict, "branches", None):
        sol_kb = verdict.branches[0].kb
    if sol_kb is not None:
        rows = _query_goal_bindings(sol_kb)
        if rows:
            print("goal bindings (from query :goal):")
            for row in rows:
                pairs = ", ".join(
                    f"{k}={v}" for k, v in sorted(row.items())
                )
                print(f"  {pairs}")

    print()
    print("stats")
    print(f"  enterings_total    {stats.enterings_total}")
    print(f"  enterings_alive    {stats.enterings_alive}")
    print(f"  enterings_dead_pre  {stats.enterings_dead_pre}")
    print(f"  enterings_dead_post {stats.enterings_dead_post}")
    print(f"  facts_merged       {stats.facts_merged}")
    print(f"  forced_positives   {stats.forced_positives}")
    print(f"  saturate_count     {stats.saturate_count}")
    print(f"  layers_explored    {stats.layers_explored}")
    print(f"  nogoods_emitted    {stats.nogoods_emitted}")
    print(f"  nogoods_subsumed   {stats.nogoods_subsumed}")
    print(f"  wall               {elapsed * 1000:.1f} ms")
    if dumper is not None:
        print(f"dump              {args.dump_states}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
