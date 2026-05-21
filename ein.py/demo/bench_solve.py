#!/usr/bin/env python3
"""Run the P1.5 hypothesis loop on a .ein file and print a summary.

Usage:

    python ein.py/demo/bench_solve.py <puzzle.ein> [--max-depth N]
                                      [--tree] [--leaves]
                                      [--verbose] [--max-nodes N]
                                      [--max-time S] [--progress-every K]
                                      [--hyp-stats]

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

Debug logging (S1.5.4-motivated diagnostics for branching blowup):

- ``--verbose`` / ``-v`` — emit a one-line progress report on stderr
  every ``--progress-every`` branches (default 100): cumulative
  branch count, elapsed wall-clock, branches-per-second, the
  hypothesis relation, and whether the last branch lived or died.
- ``--max-nodes N`` — abort the search if the branch counter
  exceeds ``N``. Default unset (no cap). When triggered, the
  partial tree built so far is *not* available (the abort raises
  out of `_explore` before `builder.finalize` runs); the script
  still reports the cumulative counters.
- ``--max-time S`` — abort if more than ``S`` seconds have elapsed.
- ``--hyp-stats`` — at the end (or on abort), dump the
  per-relation breakdown of generated hypotheses. Diagnoses which
  relation is producing the bulk of the candidates (typically
  ``is-a`` on a puzzle that hasn't declared ``(closed is-a)``;
  see plans/.../s1.5.4_hypgen_improvements.md).
"""
from __future__ import annotations

import argparse
import sys
import time
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO / "ein.py" / "src"))

from ein_bot.inference import hypothesis as _hyp_mod  # noqa: E402
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

# ── Debug / instrumentation state ──────────────────────────────────


class BudgetExceededError(RuntimeError):
    """Raised by the wrapped `try_branch` when --max-nodes/--max-time hits."""


class _DebugState:
    """Per-run counters + limits captured for the monkey-patched wrappers."""
    def __init__(self) -> None:
        self.branches: int = 0
        self.alive: int = 0
        self.dead: int = 0
        self.hyps_gen_total: int = 0
        self.hyps_by_rel: Counter[str] = Counter()
        self.saturate_time_total: float = 0.0
        self.start_time: float = 0.0
        self.last_progress: float = 0.0
        self.verbose: bool = False
        self.progress_every: int = 100
        self.max_nodes: int | None = None
        self.max_time: float | None = None
        self.aborted: str | None = None


_dbg = _DebugState()


def _install_instrumentation(verbose: bool, progress_every: int,
                             max_nodes: int | None,
                             max_time: float | None) -> None:
    """Monkey-patch `generate_hypotheses` + `try_branch` to count and log.

    The wrappers replace module-level names so `_explore`'s unqualified
    references (`generate_hypotheses(kb)` / `try_branch(...)`) resolve to
    them via the module's globals lookup at call time.
    """
    _dbg.verbose = verbose
    _dbg.progress_every = progress_every
    _dbg.max_nodes = max_nodes
    _dbg.max_time = max_time
    _dbg.start_time = time.perf_counter()
    _dbg.last_progress = _dbg.start_time

    orig_gen = _hyp_mod.generate_hypotheses
    orig_try = _hyp_mod.try_branch

    def wrapped_gen(kb):
        for h in orig_gen(kb):
            _dbg.hyps_gen_total += 1
            _dbg.hyps_by_rel[h.relation_name] += 1
            yield h

    def wrapped_try_branch(parent_kb, hypothesis, *, branch_id,
                           saturator_steps=10_000):
        _dbg.branches += 1
        now = time.perf_counter()
        if _dbg.max_nodes is not None and _dbg.branches > _dbg.max_nodes:
            _dbg.aborted = f"max-nodes ({_dbg.max_nodes}) exceeded"
            raise BudgetExceededError(_dbg.aborted)
        if _dbg.max_time is not None and (now - _dbg.start_time) > _dbg.max_time:
            _dbg.aborted = f"max-time ({_dbg.max_time:.1f}s) exceeded"
            raise BudgetExceededError(_dbg.aborted)

        t = now
        res = orig_try(parent_kb, hypothesis, branch_id=branch_id,
                       saturator_steps=saturator_steps)
        dt = time.perf_counter() - t
        _dbg.saturate_time_total += dt
        if res.is_alive():
            _dbg.alive += 1
        else:
            _dbg.dead += 1

        if _dbg.verbose and _dbg.branches % _dbg.progress_every == 0:
            elapsed = time.perf_counter() - _dbg.start_time
            since_last = time.perf_counter() - _dbg.last_progress
            _dbg.last_progress = time.perf_counter()
            recent_rate = (
                _dbg.progress_every / since_last if since_last > 0 else 0
            )
            overall_rate = _dbg.branches / elapsed if elapsed > 0 else 0
            print(
                f"  [progress] b={_dbg.branches:>6d}  "
                f"elapsed={elapsed:>6.1f}s  "
                f"rate(recent/avg)={recent_rate:>5.0f}/{overall_rate:.0f} br/s  "
                f"alive={_dbg.alive} dead={_dbg.dead}  "
                f"last=({hypothesis.relation_name}",
                end="", file=sys.stderr,
            )
            args_repr = " ".join(
                a if isinstance(a, str) else "<fact>"
                for a in hypothesis.args
            )
            print(f" {args_repr}) {dt*1e3:.1f}ms "
                  f"{'alive' if res.is_alive() else 'dead'}",
                  file=sys.stderr)
        return res

    _hyp_mod.generate_hypotheses = wrapped_gen
    _hyp_mod.try_branch = wrapped_try_branch


def _print_hyp_stats() -> None:
    print()
    print("hypothesis-generation stats")
    print(f"  branches taken         {_dbg.branches}")
    print(f"  branches alive         {_dbg.alive}")
    print(f"  branches dead          {_dbg.dead}")
    print(f"  hypotheses generated   {_dbg.hyps_gen_total}")
    print(f"  saturate-time (sum)    {_dbg.saturate_time_total*1e3:.0f} ms")
    if _dbg.branches > 0:
        avg = _dbg.saturate_time_total / _dbg.branches * 1e3
        print(f"  saturate-time per br   {avg:.2f} ms")
    if _dbg.hyps_by_rel:
        print("  hypotheses by relation:")
        for rel, n in _dbg.hyps_by_rel.most_common():
            pct = 100.0 * n / max(1, _dbg.hyps_gen_total)
            print(f"    {rel:<24s} {n:>6d}  ({pct:>5.1f}%)")


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
    """Headline counts as seen by the engine — solution endpoints
    (deepest verdict=solution markers, may be interior nodes whose
    children all turned dead) + dead-leaves + open-leaves."""
    return {
        "solution": len(tree.solutions()),
        "dead":     len(tree.dead_branches()),
        "open":     len(tree.open_branches()),
    }


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


def _hashable_args(args) -> tuple:
    """Recursively unwrap a fact's args into a hashable / sortable shape."""
    return tuple(
        (a.relation_name, _hashable_args(a.args))
        if isinstance(a, Fact)
        else a
        for a in args
    )


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
    ap.add_argument("--verbose", "-v", action="store_true",
                    help="emit per-branch progress lines to stderr")
    ap.add_argument("--progress-every", type=int, default=100,
                    help="emit a progress line every N branches (default 100)")
    ap.add_argument("--max-nodes", type=int, default=None,
                    help="abort the search after N branches (default: no cap)")
    ap.add_argument("--max-time", type=float, default=None,
                    help="abort the search after S wall-clock seconds")
    ap.add_argument("--hyp-stats", action="store_true",
                    help="print per-relation hypothesis-generation breakdown")
    ap.add_argument("--solution-facts", action="store_true",
                    help="for each solution leaf, dump its propositional "
                         "(REASONING-layer, non-bookkeeping) facts + state_hash "
                         "— useful for confirming why supposedly-duplicate "
                         "solutions weren't deduped")
    args = ap.parse_args()

    text = args.puzzle.read_text()
    t0 = time.perf_counter()
    forms = parse(text)
    t1 = time.perf_counter()
    kb = KnowledgeBase.from_ir(forms)
    t2 = time.perf_counter()

    print(f"file               {args.puzzle}")
    print(f"  parse            {(t1-t0)*1e3:7.2f} ms")
    print(f"  kb-load          {(t2-t1)*1e3:7.2f} ms")
    # ── Initial root-hypothesis preview (cheap, useful diagnostic) ──
    # Snapshot how many candidates the root would enumerate, broken
    # down by relation. Drives the decision to add `(closed R)` /
    # restrict signatures / cap depth before kicking off solve().
    root_hyps: Counter[str] = Counter()
    for h in _hyp_mod.generate_hypotheses(kb):
        root_hyps[h.relation_name] += 1
    if root_hyps:
        total_root = sum(root_hyps.values())
        print(f"  root hyps        {total_root} candidates "
              f"across {len(root_hyps)} relations")
        if args.verbose or args.hyp_stats:
            for rel, n in root_hyps.most_common():
                pct = 100.0 * n / total_root
                print(f"    {rel:<24s} {n:>6d}  ({pct:>5.1f}%)")

    _install_instrumentation(
        verbose=args.verbose,
        progress_every=args.progress_every,
        max_nodes=args.max_nodes,
        max_time=args.max_time,
    )

    mode = Mode(args.mode) if args.mode else None
    try:
        verdict = solve(kb, mode=mode, max_depth=args.max_depth)
        t3 = time.perf_counter()
    except BudgetExceededError:
        t3 = time.perf_counter()
        print()
        print(f"** ABORTED: {_dbg.aborted} **")
        print(f"  branches before abort  {_dbg.branches}")
        print(f"  elapsed                {(t3-t2)*1e3:7.2f} ms")
        if args.hyp_stats or args.verbose:
            _print_hyp_stats()
        return 2

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

    # ── Per-solution-leaf fact dump ─────────────────────────────
    if args.solution_facts:
        from ein_bot.inference.canon import BOOKKEEPING_HEADS, state_hash
        from ein_bot.kb.entities import Layer
        sols = tree.solutions()
        print(f"solution-leaf facts (REASONING layer, "
              f"bookkeeping {sorted(BOOKKEEPING_HEADS)} omitted):")
        for s in sols:
            sh = state_hash(s.kb_snapshot) if s.kb_snapshot else None
            print(f"  b{s.id}  hash={sh}  on=({_fact_repr(s.hypothesis)})")
            if s.kb_snapshot is None:
                print("    (no kb_snapshot)")
                continue
            facts = [
                f for f in s.kb_snapshot.facts
                if f.layer == Layer.REASONING
                and f.relation_name not in BOOKKEEPING_HEADS
            ]
            facts.sort(key=lambda f: (f.relation_name, _hashable_args(f.args)))
            for f in facts:
                print(f"    ({_fact_repr(f)})")
            print()

    # ── Per-endpoint / leaf summary ─────────────────────────────
    if args.leaves:
        print("endpoints + leaves (solutions first, then dead, then open):")
        groups = (
            ("solution", tree.solutions()),
            ("dead",     tree.dead_branches()),
            ("open",     tree.open_branches()),
        )
        for label, nodes in groups:
            for node in nodes:
                hyp = _fact_repr(node.hypothesis)
                parent_str = (
                    f"b{node.parent}" if node.parent is not None else "_"
                )
                # Solution endpoints can be interior (all-dead children
                # + own state goal-matched) — flag this so the shape is
                # readable.
                shape = "leaf" if not node.children else f"+{len(node.children)} dead-children"
                print(f"  b{node.id:<3} parent={parent_str:<4} "
                      f"verdict={label:<8} {shape:<22} on=({hyp})")
        print()

    # ── Full tree IR ─────────────────────────────────────────────
    if args.tree:
        print("search tree (IR):")
        print(dump_canonical([tree.to_ir()]))

    # ── Hypothesis-generation stats trailer ──────────────────────
    if args.hyp_stats:
        _print_hyp_stats()

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
