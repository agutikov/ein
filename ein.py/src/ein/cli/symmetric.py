#!/usr/bin/env python3
"""Wall-clock benchmark: `__symmetric__` kernel mirror vs the stdlib `symmetric`
rule (Phase 2b).

A relation marked `(__symmetric__ R)` is closed under arg-swap by the saturator
directly — no compiled rule, no `match.run` (`inference/saturator.py`). This is
the perf-opt counterpart of `std.algebra`'s `symmetric` closure rule. The bench
builds a symmetric-heavy synthetic puzzle (a chain of N one-way `knows` edges),
saturates it both ways, asserts the two closures are IDENTICAL, and reports the
wall-clock ratio of the saturation loop.

Run from repo root:

    python ein.py/demo/bench_symmetric.py [--edges N] [--repeats R]

Defaults: --edges 800 --repeats 5 (reports the min over repeats — most stable).

Informational, not a CI gate. There is no real symmetric-heavy puzzle yet
(zebra2 does not use the generic `symmetric` closure), so this synthetic
workload is what justifies the kernel opt: it isolates the per-mirror cost the
native path skips (the JoinPlan + matcher the rule pays).
"""
from __future__ import annotations

import argparse
import sys
import time

from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.store import KnowledgeBase

_STDLIB = "(import std.algebra :symbols (symmetric))\n(symmetric knows)\n"
_NATIVE = "(__symmetric__ knows)\n"


def _puzzle(n_edges: int, *, native: bool) -> str:
    """A chain of `n_edges` one-way `knows` edges over `n_edges + 1` nodes,
    marked symmetric one of the two ways."""
    header = _NATIVE if native else _STDLIB
    edges = "".join(f"(knows n{i} n{i + 1})\n" for i in range(n_edges))
    return header + "(relation knows T T)\n" + edges


def _time_saturate(src: str) -> tuple[float, frozenset]:
    """Saturate to fixpoint; return (saturation wall-seconds, knows-extension).

    Construction + compile happen BEFORE the timed region, so the measurement
    isolates the saturation loop (where the mirror / rule firing happens)."""
    kb = KnowledgeBase.from_ir(parse(src))
    sat = Saturator(kb)
    t0 = time.perf_counter()
    list(sat.saturate(max_steps=10_000_000))
    dt = time.perf_counter() - t0
    knows = frozenset(f.args for f in kb._facts_by_relation.get("knows", ()))
    return dt, knows


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(
        description="Bench __symmetric__ kernel mirror vs the stdlib symmetric rule.",
    )
    ap.add_argument("--edges", type=int, default=800,
                    help="number of one-way chain edges to mirror (default 800)")
    ap.add_argument("--repeats", type=int, default=5,
                    help="timed repeats; the min is reported (default 5)")
    args = ap.parse_args(argv)

    stdlib_src = _puzzle(args.edges, native=False)
    native_src = _puzzle(args.edges, native=True)

    # Parity first: the whole point is that they compute the same closure.
    _, stdlib_ext = _time_saturate(stdlib_src)
    _, native_ext = _time_saturate(native_src)
    if stdlib_ext != native_ext:
        print("PARITY FAILED — closures differ; aborting bench", file=sys.stderr)
        return 1

    stdlib_t = min(_time_saturate(stdlib_src)[0] for _ in range(args.repeats))
    native_t = min(_time_saturate(native_src)[0] for _ in range(args.repeats))

    print(f"edges={args.edges}  repeats={args.repeats}  "
          f"closure-size={len(native_ext)}  parity=OK")
    print(f"  stdlib  (rule symmetric)   {stdlib_t * 1000:8.1f} ms")
    print(f"  native  (__symmetric__)    {native_t * 1000:8.1f} ms")
    if native_t > 0:
        print(f"  speedup                    {stdlib_t / native_t:8.2f}x")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
