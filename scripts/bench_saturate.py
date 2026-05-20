#!/usr/bin/env python3
"""Quick wall-clock benchmark of the Saturator on examples/zebra.ein.

Informational (S1.3.3 acceptance) — not a CI gate. Run from repo
root:

    python scripts/bench_saturate.py [example.ein]

Reports:
- IR parse time
- KB load time
- Engine compile_all time
- Saturator.saturate() time + firing counts (productive vs redundant)

Target on a laptop: < 200 ms wall-clock for the Zebra initial
saturation. Treat numbers as informational; performance work
proper is a P1.7+ concern when we have the trace renderer to
profile end-to-end.
"""
from __future__ import annotations

import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO / "src"))

from ein_bot.inference.engine import Engine  # noqa: E402
from ein_bot.inference.saturator import Saturator  # noqa: E402
from ein_bot.ir import parse  # noqa: E402
from ein_bot.kb.store import KnowledgeBase  # noqa: E402


def bench(path: Path) -> None:
    src = path.read_text()
    print(f"input:    {path}")
    print(f"          {len(src)} chars")

    t0 = time.perf_counter()
    forms = parse(src)
    t1 = time.perf_counter()
    print(f"parse:    {(t1 - t0) * 1000:8.2f} ms  ({len(forms)} top-level forms)")

    t0 = time.perf_counter()
    kb = KnowledgeBase.from_ir(forms)
    t1 = time.perf_counter()
    print(
        f"kb load:  {(t1 - t0) * 1000:8.2f} ms  "
        f"({len(kb.types)} types, {len(kb.instances)} instances, "
        f"{len(kb.relations)} relations, {len(kb.rules)} rules, "
        f"{len(kb.facts)} facts)"
    )

    eng = Engine(kb)
    t0 = time.perf_counter()
    eng.compile_all()
    t1 = time.perf_counter()
    print(
        f"compile:  {(t1 - t0) * 1000:8.2f} ms  ({len(eng.cache)} cache entries)"
    )

    sat = Saturator(kb, engine=eng)
    t0 = time.perf_counter()
    firings = list(sat.saturate())
    t1 = time.perf_counter()
    productive = sum(1 for f in firings if not f.redundant)
    redundant = sum(1 for f in firings if f.redundant)
    print(
        f"saturate: {(t1 - t0) * 1000:8.2f} ms  "
        f"({len(firings)} firings: {productive} productive, "
        f"{redundant} redundant)"
    )
    print(f"          {sum(1 for _ in kb.reasoning())} REASONING-layer facts")


if __name__ == "__main__":
    target = Path(sys.argv[1]) if len(sys.argv) > 1 else REPO / "examples" / "zebra.ein"
    if not target.exists():
        print(f"error: {target} not found", file=sys.stderr)
        sys.exit(1)
    bench(target)
