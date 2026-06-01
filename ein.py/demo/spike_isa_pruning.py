#!/usr/bin/env python3
"""S1.7.7 T1.7.7.1 — de-risk spike: is the `is-a` type-pruning load-bearing?

Measures what happens to the zebra2 solve when hypgen's
**type-compatibility filter is neutralised** — i.e. when
:func:`ein_bot.inference.hypgen._type_compatible` is forced to return
``True`` so the generator stops restricting candidate fillers to
type-compatible instances (the two sites: `_raw_candidates` slot-check +
`_fill_slot` filler-check). Everything else is untouched — in particular
`_instance_like_objects` still selects the `is-a` leaves as the candidate
object set (that is a *different* `is-a` use, not the type filter).

It runs the same puzzle twice — **baseline** (pruning on) and
**neutralised** (pruning off) — under one budget cap, and reports the
three numbers the spike gates on:

  1. does it still solve?            (verdict + deduped solution count k)
  2. wall-time delta                 (note PyPy JIT-warmup caveat below)
  3. hypothesis-count delta          (root-hyp raw/emitted + enterings_total)

If the neutralised run hits the entering/time budget it **aborts** — that
abort *is* the "blows up" result (an irreducible search optimisation).

Run it under PyPy (CPython is too slow for full zebra2):

    ./bench_isa_spike_pypy.sh                       # wrapper, repo root
    .venv-pypy/bin/python ein.py/demo/spike_isa_pruning.py   # direct

See plans/.../p1.7_bootstrapping_zebra/s1.7.8_isa.md (§Decision) and
s1.7.7_kernel_purity_analysis.md (T1.7.7.1).
"""
from __future__ import annotations

import argparse
import sys
import time
from collections import Counter
from dataclasses import dataclass
from pathlib import Path

# Make `from ein_bot.…` resolve from a checkout (peer of bench_monotonic.py).
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))

import ein_bot.inference.hypgen as hypgen
from ein_bot.inference.closed import emit_closed
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.hypgen import generate_hypotheses_with_stats
from ein_bot.inference.monotonic.solver import BudgetExceededError, solve
from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

_DEFAULT_PUZZLE = (
    Path(__file__).resolve().parents[2] / "examples" / "zebra2.ein"
)


@dataclass
class RunResult:
    label: str
    pruning: bool
    # root-hyp preview (post-saturation, what hypgen enumerates at root)
    root_raw: int
    root_emitted: int
    root_by_rel: Counter
    # pre-candidate skips — the DIRECT evidence of what type-pruning does
    # (type_incompatible_slot / type_incompatible_filler). With pruning
    # OFF these must drop to 0; raw must rise by the same amount.
    root_type_skips: int
    n_objects: int
    # solve outcome
    verdict: str
    k: int
    exhausted: bool
    enterings_total: int
    enterings_alive: int
    enterings_dead_pre: int
    enterings_dead_post: int
    nogoods_emitted: int
    wall_ms: float
    aborted_reason: str | None


def _root_hyp_preview(kb: KnowledgeBase) -> tuple[int, int, Counter, int, int]:
    """Saturate a fork and report what hypgen would enumerate at root.

    The raw/emitted counts reflect the *current* (possibly patched)
    `_type_compatible`, so this is the headline hypothesis-count signal.
    Also returns the type-incompat pre-candidate skip total (the direct
    pruning evidence) and the instance-like object count.
    """
    preview = kb.fork()
    emit_closed(preview)
    list(Saturator(preview).saturate())
    n_objects = sum(1 for _ in hypgen._instance_like_objects(preview))
    facts, stats = generate_hypotheses_with_stats(preview)
    by_rel: Counter = Counter(f.relation_name for f in facts)
    type_skips = (
        stats.pre_candidate.get("type_incompatible_slot", 0)
        + stats.pre_candidate.get("type_incompatible_filler", 0)
    )
    return stats.raw, stats.emitted, by_rel, type_skips, n_objects


def _run(
    label: str,
    puzzle: Path,
    *,
    pruning: bool,
    stop_after: int | None,
    max_set_size: int,
    max_time: float | None,
    max_enterings: int | None,
) -> RunResult:
    """Solve `puzzle` once. When ``pruning`` is False, monkeypatch
    `hypgen._type_compatible` to a no-op for the duration of the run."""
    kb = KnowledgeBase.from_ir(parse(puzzle.read_text()))
    config = kb.config or SolverConfig()

    original = hypgen._type_compatible
    if not pruning:
        hypgen._type_compatible = lambda _kb, _obj, _sig: True
    try:
        root_raw, root_emitted, root_by_rel, root_type_skips, n_objects = (
            _root_hyp_preview(kb)
        )

        aborted_reason: str | None = None
        verdict = None
        t0 = time.perf_counter()
        try:
            verdict, stats = solve(
                kb,
                stop_after=stop_after,
                max_set_size=max_set_size,
                config=config,
                max_time=max_time,
                max_enterings=max_enterings,
            )
        except BudgetExceededError as e:
            aborted_reason = e.reason
            stats = e.stats
        wall_ms = (time.perf_counter() - t0) * 1000.0
    finally:
        hypgen._type_compatible = original

    return RunResult(
        label=label,
        pruning=pruning,
        root_raw=root_raw,
        root_emitted=root_emitted,
        root_by_rel=root_by_rel,
        root_type_skips=root_type_skips,
        n_objects=n_objects,
        verdict=type(verdict).__name__ if verdict is not None else "—",
        k=stats.solution_nodes,
        exhausted=stats.exhausted,
        enterings_total=stats.enterings_total,
        enterings_alive=stats.enterings_alive,
        enterings_dead_pre=stats.enterings_dead_pre,
        enterings_dead_post=stats.enterings_dead_post,
        nogoods_emitted=stats.nogoods_emitted,
        wall_ms=wall_ms,
        aborted_reason=aborted_reason,
    )


def _fmt(r: RunResult) -> list[str]:
    status = (
        f"ABORTED ({r.aborted_reason})" if r.aborted_reason
        else f"{r.verdict} k={r.k} exhausted={str(r.exhausted).lower()}"
    )
    return [
        f"  {r.label} (pruning {'ON' if r.pruning else 'OFF'})",
        f"    outcome            {status}",
        f"    root hyps          raw={r.root_raw}  emitted={r.root_emitted}"
        f"  ({len(r.root_by_rel)} relations, {r.n_objects} objects)",
        f"    type-incompat skips {r.root_type_skips}"
        + ("   <-- pruning did nothing" if r.root_type_skips == 0 else ""),
        f"    enterings_total    {r.enterings_total}",
        f"    enterings          alive={r.enterings_alive}"
        f" dead-pre={r.enterings_dead_pre} dead-post={r.enterings_dead_post}",
        f"    nogoods_emitted    {r.nogoods_emitted}",
        f"    wall               {r.wall_ms:.0f} ms",
    ]


def _delta(base: RunResult, neut: RunResult) -> list[str]:
    def x(a: int, b: int) -> str:
        if a == 0:
            return f"+{b - a}"
        return f"{b - a:+d}  ({b / a:.1f}x)"

    solves = "yes" if neut.aborted_reason is None and neut.k >= 1 else "NO"
    same_k = (
        neut.aborted_reason is None and base.k == neut.k
    )
    return [
        "DELTA (neutralised vs baseline)",
        f"  still solves?        {solves}"
        + ("" if neut.aborted_reason is None
           else "  — hit budget, search blew up"),
        f"  same answer (k)?     {'yes' if same_k else 'no'}"
        f"   (baseline k={base.k}, neutralised k={neut.k})",
        f"  root-hyp emitted     {x(base.root_emitted, neut.root_emitted)}",
        f"  root-hyp raw         {x(base.root_raw, neut.root_raw)}",
        f"  enterings_total      {x(base.enterings_total, neut.enterings_total)}",
        f"  wall                 {neut.wall_ms - base.wall_ms:+.0f} ms"
        "   (PyPy JIT warmup makes the first run slower — read enterings)",
    ]


def _build_argparser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(
        description=__doc__.split("\n", 1)[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    ap.add_argument("puzzle", type=Path, nargs="?", default=_DEFAULT_PUZZLE,
                    help=f"path to .ein puzzle (default: {_DEFAULT_PUZZLE})")
    ap.add_argument("--max-set-size", type=int, default=5,
                    help="largest commitment size to enumerate (default 5)")
    ap.add_argument("--exhaustive", action="store_true",
                    help="exhaust the lattice (stop_after=None) so k is exact"
                         " — default stop_after=1 (the sound fast path / gate)")
    ap.add_argument("--max-time", type=float, default=180.0,
                    help="per-run wall budget in seconds (default 180); the "
                         "neutralised run aborts here if it blows up")
    ap.add_argument("--max-enterings", type=int, default=200_000,
                    help="per-run entering budget (default 200000); abort = "
                         "blow-up")
    ap.add_argument("--warmup", action="store_true",
                    help="run baseline once untimed first to prime the PyPy "
                         "JIT before the measured runs (fairer wall numbers)")
    return ap


def main(argv: list[str] | None = None) -> int:
    args = _build_argparser().parse_args(argv)
    stop_after = None if args.exhaustive else 1
    common = dict(
        stop_after=stop_after,
        max_set_size=args.max_set_size,
        max_time=args.max_time,
        max_enterings=args.max_enterings,
    )

    print(f"puzzle            {args.puzzle}")
    print(f"stop_after        {stop_after}   max_set_size {args.max_set_size}")
    print(f"budget            max_time={args.max_time}s "
          f"max_enterings={args.max_enterings}")
    print(f"python            {sys.implementation.name} "
          f"{sys.version.split()[0]}")
    print()

    if args.warmup:
        _run("warmup", args.puzzle, pruning=True, **common)

    baseline = _run("baseline", args.puzzle, pruning=True, **common)
    neutralised = _run("neutralised", args.puzzle, pruning=False, **common)

    for line in _fmt(baseline):
        print(line)
    print()
    for line in _fmt(neutralised):
        print(line)
    print()
    for line in _delta(baseline, neutralised):
        print(line)
    return 0


if __name__ == "__main__":
    sys.exit(main())
