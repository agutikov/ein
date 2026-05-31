#!/usr/bin/env python3
"""S1.7a.1 probe — measure the *solution-node* count `k` read-only.

Applies P1.7a's domain-agnostic solution definition to the EXISTING
engine output, WITHOUT changing any engine code:

    solution_node(kb)  ⟺  consistent(kb) ∧ complete(kb)
        complete(kb)    ≡  not _compute_alive(kb)          (no open hypothesis)
        consistent(kb)  ≡  no contradiction
    k = | { state_hash(node.kb) : node is a solution_node } |   (deduped)

`gaps_solve` records one ``SolutionRecord`` per *goal-satisfying*
commitment. For the zebra2 family the ``:goal`` is existential over
slots that every complete model fills, so the goal-satisfying set is a
superset of the complete set — filtering ``proof.solutions`` by
``is_solution_node`` and deduping by ``state_hash`` therefore yields the
true `k` (incomplete goal-matchers like the partial dead-end drop out).

It also answers S1.7a.1's open questions:
  - OQ1: for each *incomplete* record, is it OPEN (``_compute_alive``
    non-empty) or an all-refuted/contradictory dead-end? (printed)
  - OQ2: does exhaustive ``gaps_solve`` terminate per variant? (timed;
    ``--max-set-size`` / ``--max-time`` bound it, abort reported)
  - the ``k → verdict`` reading (1 unique / >1 ambiguity / 0 contradiction).

Run under PyPy for the full zebra2 family:
    ./bench_solve_monotonic_pypy.sh  -- (peer runner)
    .venv-pypy/bin/python ein.py/demo/probe_solution_nodes.py examples/zebra2.ein
"""
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

from ein_bot.inference.canon import state_hash
from ein_bot.inference.contradiction import ContradictionDetector
from ein_bot.inference.monotonic import (
    BudgetExceededError,
    contradictions_solve,
    gaps_solve,
    solve,
)
from ein_bot.inference.monotonic.solver import _compute_alive
from ein_bot.inference.verdict import Ambiguity, Contradiction, Solution
from ein_bot.ir import parse
from ein_bot.kb import KnowledgeBase

_LOC_RELS = ("color-loc", "nation-loc", "drink-loc", "smoke-loc", "pet-loc")


def _loc_facts(kb: KnowledgeBase) -> list[tuple[str, tuple]]:
    """Positive ``*-loc`` assignments — the puzzle grid cells."""
    out: list[tuple[str, tuple]] = []
    for rel in _LOC_RELS:
        for f in kb._facts_by_relation.get(rel, ()):
            if len(f.args) == 2:
                out.append((rel, f.args))
    return sorted(set(out))


def _classify(rec_kb: KnowledgeBase) -> dict:
    alive = _compute_alive(rec_kb)
    contra = bool(ContradictionDetector(rec_kb).detect())
    return {
        "alive": alive,
        "open": len(alive),
        "consistent": not contra,
        "complete": not alive,
        "is_node": (not alive) and (not contra),
        "hash": state_hash(rec_kb),
        "ncells": len(_loc_facts(rec_kb)),
    }


def probe(path: Path, *, max_set_size: int, max_time: float | None,
          run_contradictions: bool) -> int:
    text = path.read_text()
    print(f"\n=== {path} ===")

    # ── gaps_solve: enumerate goal-satisfying commitments ──────────
    kb = KnowledgeBase.from_ir(parse(text))
    t0 = time.perf_counter()
    aborted = None
    try:
        verdict, stats = gaps_solve(
            kb, max_set_size=max_set_size, max_time=max_time,
        )
        recs = verdict.proof.solutions
    except BudgetExceededError as e:
        aborted, stats, recs = e.reason, e.stats, ()
    gaps_ms = (time.perf_counter() - t0) * 1e3

    print(
        f"gaps_solve: {'ABORTED('+aborted+')' if aborted else 'ok'}  "
        f"solutions_recorded={len(recs)}  "
        f"enterings_total={stats.enterings_total}  "
        f"layers={stats.layers_explored}  wall={gaps_ms:.0f}ms",
    )

    nodes_by_hash: dict[int, dict] = {}
    incomplete: list[dict] = []
    inconsistent = 0
    for rec in recs:
        c = _classify(rec.kb)
        c["commitment"] = rec.commitment
        if c["is_node"]:
            nodes_by_hash.setdefault(c["hash"], {**c, "count": 0})
            nodes_by_hash[c["hash"]]["count"] += 1
        elif not c["consistent"]:
            inconsistent += 1
        else:
            incomplete.append(c)

    k = len(nodes_by_hash)
    reading = "UNIQUE (k=1)" if k == 1 else (
        "AMBIGUITY (k>1)" if k > 1 else "CONTRADICTION / no model (k=0)")
    print(
        f"  solution nodes (consistent ∧ complete, deduped by state_hash): "
        f"k = {k}  →  {reading}",
    )
    print(
        f"    breakdown of {len(recs)} recorded: "
        f"complete&consistent collapsing to {k} state(s); "
        f"incomplete(open)={len(incomplete)}; inconsistent={inconsistent}",
    )
    for h, n in nodes_by_hash.items():
        print(f"    node[{h & 0xffffffff:08x}] x{n['count']:<3d} cells={n['ncells']}/25")
    # OQ1 — are the incomplete records OPEN or all-refuted?
    for c in incomplete[:5]:
        sample = sorted(f"{r}{a}" for (r, a) in list(c["alive"])[:6])
        print(
            f"    INCOMPLETE cells={c['ncells']}/25 open={c['open']} "
            f"consistent={c['consistent']}  open-sample={sample}",
        )

    # ── contradictions_solve: dead cores (the k=0 evidence path) ───
    if run_contradictions:
        kb2 = KnowledgeBase.from_ir(parse(text))
        t1 = time.perf_counter()
        try:
            cv, _cstats = contradictions_solve(
                kb2, max_set_size=max_set_size, max_time=max_time,
            )
            core = cv.unsat_core
            ndead = len(cv.proof.dead_commitments)
            cab = None
        except BudgetExceededError as e:
            cab, core, ndead = e.reason, frozenset(), 0
        c_ms = (time.perf_counter() - t1) * 1e3
        print(
            f"contradictions_solve: {'ABORTED('+cab+')' if cab else 'ok'}  "
            f"dead_commitments={ndead}  unsat_core_size={len(core)}  "
            f"wall={c_ms:.0f}ms",
        )
        for f in sorted(core, key=lambda x: (x.relation_name, x.args))[:6]:
            src = getattr(f.provenance, "source", None)
            print(f"    core: ({f.relation_name} {' '.join(map(str, f.args))})"
                  f"{'  :source ' + repr(src) if src else ''}")
    return k


def run_solve(path: Path, *, max_set_size: int, max_time: float | None,
              stop_after: int | None) -> int:
    """Verify the NEW P1.7a ``solve()`` engine end-to-end on one puzzle."""
    text = path.read_text()
    print(f"\n=== {path}  [solve()] ===")
    kb = KnowledgeBase.from_ir(parse(text))
    t0 = time.perf_counter()
    try:
        verdict, stats = solve(
            kb, stop_after=stop_after, max_set_size=max_set_size,
            max_time=max_time,
        )
    except BudgetExceededError as e:
        print(f"  ABORTED: {e.reason}  (k so far={e.stats.solution_nodes})")
        return -1
    ms = (time.perf_counter() - t0) * 1e3
    vt = type(verdict).__name__
    print(
        f"  verdict={vt}  k={stats.solution_nodes}  exhausted={stats.exhausted}  "
        f"enterings={stats.enterings_total}  layers={stats.layers_explored}  "
        f"wall={ms:.0f}ms",
    )
    if isinstance(verdict, Solution):
        print(f"    model cells={len(_loc_facts(verdict.kb))}/25")
    elif isinstance(verdict, Ambiguity):
        for i, b in enumerate(verdict.branches):
            print(f"    model[{i}] cells={len(_loc_facts(b.kb))}/25")
    elif isinstance(verdict, Contradiction):
        for f in sorted(verdict.unsat_core,
                        key=lambda x: (x.relation_name, x.args))[:6]:
            src = getattr(f.provenance, "source", None)
            print(f"    core: ({f.relation_name} {' '.join(map(str, f.args))})"
                  f"{'  :source ' + repr(src) if src else ''}")
    return stats.solution_nodes


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(
        description="S1.7a.1 probe — measure solution-node count k read-only "
                    "(consistent ∧ complete, deduped by state_hash).",
    )
    ap.add_argument("puzzles", nargs="+", type=Path,
                    help="one or more .ein puzzle files")
    ap.add_argument("--max-set-size", type=int, default=5,
                    help="lattice depth cap for exhaustive gaps search (default 5)")
    ap.add_argument("--max-time", type=float, default=None,
                    help="wall-clock budget in seconds (default: none)")
    ap.add_argument("--no-contradictions", action="store_true",
                    help="skip the contradictions_solve pass")
    ap.add_argument("--solve", action="store_true",
                    help="verify the NEW P1.7a solve() engine instead of the "
                         "read-only gaps-based classification")
    ap.add_argument("--stop-after", type=int, default=None,
                    help="solve(): stop after the first N solution nodes "
                         "(default: exhaust)")
    args = ap.parse_args(argv)
    for p in args.puzzles:
        if args.solve:
            run_solve(p, max_set_size=args.max_set_size,
                      max_time=args.max_time, stop_after=args.stop_after)
        else:
            probe(p, max_set_size=args.max_set_size, max_time=args.max_time,
                  run_contradictions=not args.no_contradictions)
    return 0


if __name__ == "__main__":
    sys.exit(main())
