#!/usr/bin/env python3
"""The Python half of the M1a benchmark set (S1a.0.4 T1a.0.4.5).

`ein.rs` measures itself with `criterion`; this produces **the same
measurement set** from `ein.py`, so `plans/m1a_rust/design/README.md`
§ Measured is refreshed by one command per implementation and the two columns
are comparable rather than merely adjacent.

    python3 utils/bench_baseline.py                 # the default set
    python3 utils/bench_baseline.py --json out.json # + the raw artifact
    python3 utils/bench_baseline.py -k parse        # one bench
    .venv-pypy/bin/python utils/bench_baseline.py   # the PyPy column

Every bench runs in **this** process, timed with `perf_counter`, best-of-N
after a warm-up — not in a subprocess like `utils/feature_matrix.py`, because
what is being compared here is engine work, not process start-up, and PyPy
needs the warm-up to be measuring its JIT rather than its interpreter.

The set mirrors [design/12](../plans/m1a_rust/design/12_toolchain_and_layout.md)
§4's bench table one for one. Where a name means something slightly different
in Python it says so in its docstring; where it cannot be measured at all it is
skipped with a reason rather than silently dropped.
"""
from __future__ import annotations

import argparse
import json
import os
import statistics
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
# `EIN_SRC` points the benches at a *different* checkout — how a before/after
# measurement is taken without stashing the tree under test (`git worktree add`
# a revision, then set this to its `ein.py/src`). The corpus paths still come
# from this checkout, so both sides read the same `.ein` bytes.
sys.path.insert(0, os.environ.get("EIN_SRC", str(REPO / "ein.py" / "src")))

EXAMPLES = REPO / "examples"
ZEBRA2 = EXAMPLES / "zebra2.ein"
ZEBRA = EXAMPLES / "zebra.ein"


# ── the harness ────────────────────────────────────────────────────


class Result:
    def __init__(self, name: str, samples: list[float], note: str = "") -> None:
        self.name = name
        self.samples = samples
        self.note = note

    @property
    def best(self) -> float:
        return min(self.samples)

    @property
    def median(self) -> float:
        return statistics.median(self.samples)

    def as_dict(self) -> dict:
        return {"bench": self.name, "best_ms": round(self.best * 1e3, 3),
                "median_ms": round(self.median * 1e3, 3),
                "runs": len(self.samples), "note": self.note}


def measure(name: str, thunk, *, runs: int, warmup: int, note: str = "") -> Result:
    for _ in range(warmup):
        thunk()
    samples = []
    for _ in range(runs):
        t0 = time.perf_counter()
        thunk()
        samples.append(time.perf_counter() - t0)
    return Result(name, samples, note)


# ── the benches ────────────────────────────────────────────────────


def _parse_thunk(paths: list[Path]):
    from ein.ir import parse
    texts = [(p.read_text(encoding="utf-8"), str(p)) for p in paths]

    def run():
        for text, name in texts:
            parse(text, filename=name)
    return run


def _load_thunk(path: Path):
    """Parse + import resolution + macro expansion + index build — the whole
    `from_file`, which is what a user waits for before anything reasons."""
    from ein.kb.store import KnowledgeBase

    def run():
        KnowledgeBase.from_file(str(path))
    return run


def _saturated_root(path: Path):
    from ein.inference.closed import emit_closed
    from ein.inference.saturator import Saturator
    from ein.kb.store import KnowledgeBase
    kb = KnowledgeBase.from_file(str(path))
    emit_closed(kb)
    list(Saturator(kb).saturate())
    return kb


def _saturate_root_thunk(path: Path):
    from ein.inference.closed import emit_closed
    from ein.inference.saturator import Saturator
    from ein.kb.store import KnowledgeBase
    kb0 = KnowledgeBase.from_file(str(path))

    def run():
        kb = kb0.fork()
        emit_closed(kb)
        list(Saturator(kb).saturate())
    return run


def _match_hot_thunk(path: Path):
    """`match.run` over the saturated root, every plan once. The matcher is
    46 % of self time in the CPython profile, so this is the bench the register
    machine ([design/05](../plans/m1a_rust/design/05_matcher.md)) has to move."""
    from ein.inference import match
    from ein.inference.engine import Engine
    kb = _saturated_root(path)
    engine = Engine(kb)
    engine.compile_all()
    plans = list(engine.cache.values())

    def run():
        for plan in plans:
            for _ in match.run_guarded(plan, kb):
                pass
    return run


def _boundary_thunk(path: Path):
    """One `_admit_from_boundary` round against the saturated world. 72 % of
    the exhaustive profile's cumulative time sits under this call."""
    from ein.inference.closed import emit_closed
    from ein.inference.saturator import Saturator
    from ein.kb.store import KnowledgeBase
    kb0 = KnowledgeBase.from_file(str(path))

    def run():
        kb = kb0.fork()
        emit_closed(kb)
        sat = Saturator(kb)
        list(sat.saturate())
        sat._admit_from_boundary()
    return run


def _fork_thunk(path: Path):
    """Fork + first delta write. Already 0.003 s / 206 calls in the profile —
    it is here not because it is slow but because
    [P1a.7](../plans/m1a_rust/p1a.7_parallelism/README.md) needs hundreds of
    thousands of them, and that is a different question."""
    from ein.kb.entities import Fact
    kb = _saturated_root(path)

    def run():
        fork = kb.fork()
        fork.add_and_index_fact(Fact(relation_name="__bench__", args=("a", "b")))
    return run


def _solve_thunk(path: Path, *, stop_after: int | None):
    from ein.inference.monotonic import solve
    from ein.kb.store import KnowledgeBase

    def run():
        solve(KnowledgeBase.from_file(str(path)), stop_after=stop_after)
    return run


def _stdlib_paths() -> list[Path]:
    for d in (REPO / "stdlib", REPO / "ein.py/src/ein/stdlib"):
        if d.is_dir():
            return sorted(d.glob("*.ein"))
    return []


# name -> (builder, runs, warmup)
BENCHES: dict[str, tuple] = {
    "parse":             (lambda: _parse_thunk([ZEBRA2, ZEBRA, *_stdlib_paths()]), 5, 2),
    "load":              (lambda: _load_thunk(ZEBRA2), 5, 2),
    "saturate_root":     (lambda: _saturate_root_thunk(ZEBRA2), 5, 2),
    "match_hot":         (lambda: _match_hot_thunk(ZEBRA2), 5, 2),
    "boundary":          (lambda: _boundary_thunk(ZEBRA2), 3, 1),
    "fork":              (lambda: _fork_thunk(ZEBRA2), 20, 5),
    "solve_fast":        (lambda: _solve_thunk(ZEBRA2, stop_after=1), 3, 1),
    "solve_exhaustive":  (lambda: _solve_thunk(ZEBRA2, stop_after=None), 3, 1),
}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR",
                    help="run only benches whose name contains SUBSTR")
    ap.add_argument("--json", type=Path, default=None, metavar="FILE",
                    help="also write the raw samples to FILE")
    ap.add_argument("--runs", type=int, default=None,
                    help="override every bench's run count (1 for a smoke test)")
    args = ap.parse_args()

    impl = f"{sys.implementation.name} {sys.version.split()[0]}"
    import ein
    print(f"bench_baseline — {impl} @ {Path(ein.__file__).parent}\n",
          file=sys.stderr)

    results: list[Result] = []
    for name, (build, runs, warmup) in BENCHES.items():
        if args.only and args.only not in name:
            continue
        if args.runs is not None:
            runs, warmup = args.runs, 0
        print(f"… {name}", file=sys.stderr, flush=True)
        results.append(measure(name, build(), runs=runs, warmup=warmup))

    print(f"\n{'bench':<20}{'best':>12}{'median':>12}{'runs':>6}")
    print("─" * 50)
    for r in results:
        print(f"{r.name:<20}{r.best * 1e3:>10.1f}ms{r.median * 1e3:>10.1f}ms"
              f"{len(r.samples):>6}")
    if args.json:
        args.json.write_text(json.dumps(
            {"impl": impl, "results": [r.as_dict() for r in results]},
            indent=2) + "\n", encoding="utf-8")
        print(f"\nartifact: {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
