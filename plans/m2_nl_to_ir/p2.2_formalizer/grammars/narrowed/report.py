#!/usr/bin/env python3
"""report.py — the narrowed root's speeds, from `speed/record.jsonl.gz`, read as S2.2.3's `../report.py --speeds`
reads its own: per request, the generated tokens and the stop, tokens a second (the generated tokens over the
generation's wall time, the prompt's evaluation apart; the median of three calls, and the three), the grammar's
sampler built in (the envelope's `build_ms`, the median), the tokens whose first choice the grammar refused under
`chosen-first`, whether a `chosen-first` call generated the `every-token` call's tokens, and whether the narrowed
root's generated the root before it's. Standard library only.

    python3 report.py
"""

import gzip
import json
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent


def main() -> None:
    names = (HERE / "speed" / "order.txt").read_text().split()
    by = {}
    with gzip.open(HERE / "speed" / "record.jsonl.gz", "rt") as f:
        for name, line in zip(names, f):
            by.setdefault(name, []).append(json.loads(line))
    toks = {n: rs[0]["generation"]["envelope"]["generation"]["tokens"] for n, rs in by.items()
            if rs[0].get("generation")}
    print("| request | tokens | stop | tokens a second (median of 3) | the three | grammar built in, ms (median) | "
          "chosen again | the same tokens as every-token | the same tokens as the root before |")
    print("|---|---:|---|---:|---|---:|---:|---|---|")
    for name, rs in by.items():
        gens = [r["generation"] for r in rs if r.get("generation")]
        if len(gens) != len(rs):
            print(f"| `{name}` | a call failed |")
            continue
        tps = [len(g["envelope"]["generation"]["tokens"]) / (g["envelope"]["generation"]["generation_ms"] / 1e3)
               for g in gens]
        e = gens[0]["envelope"]
        gr = e.get("grammar") or {}
        builds = [g["envelope"]["grammar"].get("build_ms", 0) for g in gens if g["envelope"].get("grammar")]
        same_check = same_root = ""
        if name.endswith(".chosen-first"):
            same_check = "yes" if toks.get(name.replace(".chosen-first", ".every-token")) == toks[name] else "**no**"
        if ".root." in name:
            same_root = "yes" if toks.get(name.replace(".root.", ".whole.")) == toks[name] else "**no**"
        built = f"{statistics.median(builds):.2f}" if builds else "—"
        print(f"| `{name}` | {len(e['generation']['tokens'])} | {e['generation']['stop']} | "
              f"{statistics.median(tps):.1f} | {', '.join(f'{t:.1f}' for t in tps)} | {built} | "
              f"{gr.get('chosen_again', '—')} | {same_check} | {same_root} |")


if __name__ == "__main__":
    main()
