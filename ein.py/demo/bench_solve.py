#!/usr/bin/env python3
"""Run the P1.5 hypothesis loop on a .ein file and print a summary.

Usage:

    python ein.py/demo/bench_solve.py <puzzle.ein> [--max-depth N]
                                      [--tree] [--leaves]

Prints, in order:

1. Phase timings — parse, kb-load, solve.
2. Verdict — Solution / Ambiguity / Contradiction (+ count of solution
   branches when there's more than one).
3. Search-tree shape — total nodes, max depth, leaves grouped by
   verdict (solution / dead / open).
4. For each solution leaf, the bindings that match the query's
   ``:goal`` pattern.
5. With ``--leaves``: a one-line summary per leaf (id, parent,
   verdict, the hypothesis fact that seeded it).
6. With ``--tree``: the SearchTree serialised as ``(trace …)`` IR
   (the round-trippable proof object).

Without S1.5.3's canonical-state dedup, a query whose answer can
be reached via a symmetric pair of hypotheses produces TWO alive
branches with the same closed KB — SOLVE mode then returns
``Ambiguity`` rather than ``Solution``. The unique answer is still
visible in the bindings printed under §4.
"""
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO / "ein.py" / "src"))

from ein_bot.inference.hypothesis import (  # noqa: E402
    Ambiguity,
    Contradiction,
    Mode,
    SearchTree,
    Solution,
    solve,
)
from ein_bot.ir import dump_canonical, parse  # noqa: E402
from ein_bot.kb.entities import Fact  # noqa: E402
from ein_bot.kb.store import KnowledgeBase  # noqa: E402


# ── Tree metrics ───────────────────────────────────────────────────


def tree_depth(tree: SearchTree) -> int:
    """Max distance from root to any leaf."""
    cache: dict[int, int] = {}

    def depth(nid: int) -> int:
        if nid in cache:
            return cache[nid]
        node = tree.nodes[nid]
        if not node.children:
            cache[nid] = 0
        else:
            cache[nid] = 1 + max(depth(c) for c in node.children)
        return cache[nid]

    return depth(tree.root)


def leaf_summary(tree: SearchTree) -> dict[str, int]:
    counts = {"solution": 0, "dead": 0, "open": 0}
    for node in tree.nodes.values():
        if not node.children:
            counts[node.verdict] = counts.get(node.verdict, 0) + 1
    return counts


# ── Goal-binding extraction ────────────────────────────────────────


def query_goal_bindings(kb: KnowledgeBase) -> list[dict[str, str]]:
    """Run the query's :goal pattern against `kb`, return the binding rows."""
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
            rows = [dict(b) for b, _premises in match_run(plan, kb)]
            return rows
    return []


def _fact_repr(fact: Fact | None) -> str:
    if fact is None:
        return "—"
    parts: list[str] = []
    for a in fact.args:
        if isinstance(a, Fact):
            parts.append("(" + _fact_repr(a) + ")")
        else:
            parts.append(str(a))
    return f"{fact.relation_name} " + " ".join(parts)


# ── Main ───────────────────────────────────────────────────────────


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("puzzle", type=Path)
    ap.add_argument("--max-depth", type=int, default=4)
    ap.add_argument(
        "--mode",
        choices=[m.value for m in Mode],
        default=None,
        help="override the query's :mode field",
    )
    ap.add_argument("--tree", action="store_true",
                    help="dump the full SearchTree as (trace …) IR")
    ap.add_argument("--leaves", action="store_true",
                    help="print one line per leaf node")
    args = ap.parse_args()

    text = args.puzzle.read_text()
    t0 = time.perf_counter()
    forms = parse(text)
    t1 = time.perf_counter()
    kb = KnowledgeBase.from_ir(forms)
    t2 = time.perf_counter()

    mode = Mode(args.mode) if args.mode else None
    verdict = solve(kb, mode=mode, max_depth=args.max_depth)
    t3 = time.perf_counter()

    print(f"file               {args.puzzle}")
    print(f"  parse            {(t1-t0)*1e3:7.2f} ms")
    print(f"  kb-load          {(t2-t1)*1e3:7.2f} ms")
    print(f"  solve            {(t3-t2)*1e3:7.2f} ms")
    print()

    # ── Verdict ──────────────────────────────────────────────────
    if isinstance(verdict, Solution):
        print("verdict          Solution")
    elif isinstance(verdict, Ambiguity):
        print(f"verdict          Ambiguity ({len(verdict.branches)} solution branches)")
    elif isinstance(verdict, Contradiction):
        print(f"verdict          Contradiction (unsat-core size {len(verdict.unsat_core)})")
    else:
        print(f"verdict          ??? ({type(verdict).__name__})")
    print()

    # ── Tree shape ───────────────────────────────────────────────
    tree = verdict.tree
    if tree is None:
        print("(no tree)")
        return 0

    print(f"tree nodes       {len(tree.nodes)}")
    print(f"tree depth       {tree_depth(tree)}")
    leaves = leaf_summary(tree)
    print(f"leaves           "
          f"solution={leaves['solution']} "
          f"dead={leaves['dead']} "
          f"open={leaves['open']}")
    print()

    # ── Solution bindings ────────────────────────────────────────
    sols = tree.solutions()
    if sols:
        print("solution bindings (from query :goal):")
        seen_keys: set[tuple] = set()
        for s in sols:
            rows = query_goal_bindings(s.kb_snapshot)
            for row in rows:
                key = tuple(sorted(row.items()))
                if key in seen_keys:
                    continue
                seen_keys.add(key)
                fmt = ", ".join(f"{k}={v}" for k, v in row.items())
                print(f"  {fmt}")
        if not seen_keys:
            print("  (none — query goal has no variables?)")
        print()

    # ── Per-leaf summary ────────────────────────────────────────
    if args.leaves:
        print("leaves:")
        for node in tree.nodes.values():
            if node.children:
                continue
            hyp = _fact_repr(node.hypothesis)
            print(f"  b{node.id:<3} parent=b{node.parent if node.parent is not None else '_':<3} "
                  f"verdict={node.verdict:<8} on=({hyp})")
        print()

    # ── Full tree IR ─────────────────────────────────────────────
    if args.tree:
        print("search tree (IR):")
        print(dump_canonical([tree.to_ir()]))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
