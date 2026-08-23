#!/usr/bin/env python3
"""Criterion's estimates as one table — mean, spread, and whether it is stable.

    utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
    python3 utils/criterion_table.py                       # after the run
    python3 utils/criterion_table.py --max-rsd 3 --json out.json

`cargo bench`'s console output is per-bench and scrolls; the numbers stay in
`ein.rs/target/criterion/<group>/<case>/new/estimates.json` and include the
one column a "3x faster" claim needs and the console line buries — the
**standard deviation**. [S1a.6.1](../docs/history/m1a_rust/README.md#s1a61--fresh-profile-and-bench-baseline)
requires every bench under 3 % relative standard deviation on the bench
machine before any result from it is believed, and this is how that is
checked rather than asserted; the exit code is 1 if any bench misses, so it
works in CI as-is.

Reads whatever the last run left, so it is also how a run interrupted halfway
still yields its finished benches.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
CRITERION = REPO / "ein.rs" / "target" / "criterion"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--dir", type=Path, default=CRITERION)
    ap.add_argument("--max-rsd", type=float, default=3.0,
                    help="fail if any bench's relative sd exceeds this %% "
                         "(default 3, S1a.6.1's gate)")
    ap.add_argument("--json", type=Path, default=None)
    args = ap.parse_args()

    if not args.dir.is_dir():
        print(f"no criterion output at {args.dir} — run `cargo bench` first",
              file=sys.stderr)
        return 2

    rows: list[dict] = []
    for est in sorted(args.dir.glob("*/*/new/estimates.json")):
        group, case = est.parts[-4], est.parts[-3]
        e = json.loads(est.read_text(encoding="utf-8"))
        mean = e["mean"]["point_estimate"] / 1e6
        sd = e["std_dev"]["point_estimate"] / 1e6
        ci = e["mean"]["confidence_interval"]
        rows.append({
            "bench": f"{group}/{case}", "mean_ms": mean, "sd_ms": sd,
            "rsd_pct": sd / mean * 100 if mean else 0.0,
            "ci_lo_ms": ci["lower_bound"] / 1e6,
            "ci_hi_ms": ci["upper_bound"] / 1e6,
        })

    print(f"{'bench':<26}{'mean':>13}{'sd':>11}{'rsd':>8}   95 % CI")
    print("─" * 76)
    worst = 0.0
    for r in rows:
        worst = max(worst, r["rsd_pct"])
        flag = "" if r["rsd_pct"] <= args.max_rsd else "  ← unstable"
        print(f"{r['bench']:<26}{fmt(r['mean_ms']):>13}{fmt(r['sd_ms']):>11}"
              f"{r['rsd_pct']:>7.2f}%   [{fmt(r['ci_lo_ms'])}, "
              f"{fmt(r['ci_hi_ms'])}]{flag}")
    print(f"\n{len(rows)} benches, worst relative sd {worst:.2f} % "
          f"(gate {args.max_rsd:.0f} %)")

    if args.json:
        args.json.write_text(json.dumps({"rows": rows}, indent=2) + "\n",
                             encoding="utf-8")
        print(f"artifact: {args.json}", file=sys.stderr)
    return 0 if worst <= args.max_rsd else 1


def fmt(ms: float) -> str:
    """Milliseconds, in whatever unit keeps three significant figures — a
    `fork` at 0.0003 ms and a `solve` at 198 ms are in the same table."""
    if ms >= 1:
        return f"{ms:.2f} ms"
    if ms >= 1e-3:
        return f"{ms * 1e3:.1f} µs"
    return f"{ms * 1e6:.0f} ns"


if __name__ == "__main__":
    raise SystemExit(main())
