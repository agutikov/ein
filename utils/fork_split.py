#!/usr/bin/env python3
"""The fork-entry split — what a run does at root and what it does per entering.

    python3 utils/fork_split.py                    # both `-e` cells
    python3 utils/fork_split.py -k zebra2
    python3 utils/fork_split.py --json out.json
    python3 utils/fork_split.py --events /tmp/z.jsonl        # re-read a stream
    python3 utils/fork_split.py --bin /path/to/ein -- solve examples/zebra.ein -e

The named instrument behind
[baseline.md §9](../plans/m1a_rust/p1a.6_performance/baseline.md#9-the-fork-entry-re-derivation),
promoted from the inline script that section was first written with
(T1a.6.9.1). Re-run it at the end of every P1a.6 stage: S1a.6.8 removed the
compile share of this cost and S1a.6.3 is aimed at the match share, so the
ratio moves under the phase's own work.

**What it measures.** How much of a fork's work is re-derivation. Split the
`--events-level verbose` stream at its `enter` events and the root prefix and
the per-entering suffixes separate exactly.

`commitment::try_commitment_set` forks the *saturated* root. ein.py built a
fresh `Saturator` there — empty `seen` / `fired` / `parked` and `delta = None`,
a FULL enqueue pass — so a fork's first act was to re-derive the parent's whole
deductive closure as `redundant` firings, 94.6 % of them on `zebra -e`. Since
[S1a.6.9](../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
ein.rs **resumes** root's saturation instead
([D3](../plans/m1a_rust/divergences.md)), and this is the instrument that says
by how much: point `--bin` at a `--features fork-delta` build and set
`EIN_FORK_DELTA=0` for the old shape.

**Two attributions the inline script got wrong**, which is why this is a
script and not a `grep`:

- **`enter` closes the block it describes** — `solve.rs` emits it *after*
  `try_commitment_set` returns — so treating it as an opener folds the first
  entering into the root row and leaves a trailing tail that is not an
  entering at all. On `zebra2 -e` that inflated root's firings from 321 to
  810. The block's leading bookkeeping (`hyp` from the previous entering's
  `complete`, a `writeback` and the *root* re-saturation a forced positive
  runs) is split off the same way, and `enter.n_firings` checks the cut: a
  mismatch is reported rather than absorbed.
- **`record_alternative` emits its `alt` lines *before* the `fire` line of the
  firing that produced them**, so reading "the `alt`s after a redundant
  firing" off the raw stream attributes each one to the *previous* firing and
  reports a productive share that does not exist. Every `alt` comes from a
  redundant firing — 5 111 of 5 111 on `zebra2 -e` — because
  `record_alternative` is only reached on the `all_known` path.

What does vary — and what S1a.6.9's invariant argument actually rests on — is
whether that firing's **premises** include a fork-local fact, since those are
the re-derivations a delta-seeded pass still finds. `local` per entering is the
commitment facts plus everything the fork's own productive firings derived, so
the split is computable from the stream alone.
"""
from __future__ import annotations

import argparse
import json
import statistics
import subprocess
import sys
import tempfile
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
EIN = REPO / "ein.rs" / "target" / "release" / "ein"

# The cells baseline.md §9 tabulates. `-e` (exhaustive) because the lattice is
# where enterings are, and a `--stop-after 1` run has too few to average.
CELLS: list[tuple[str, list[str]]] = [
    ("zebra2 -e", ["solve", "examples/zebra2.ein", "-e"]),
    ("zebra -e", ["solve", "examples/zebra.ein", "-e"]),
]

# Per-entering rows, in the order baseline.md §9 prints them.
ROWS = ("fire", "red", "prod", "enqueue", "park", "compile", "quiesce", "alt")


# Events that are never emitted inside a fork's saturation: hypgen's `hyp`,
# the two writebacks, and a learned clause. The last one in a block is
# therefore where the entering starts — everything before it is the *previous*
# entering's `complete`, an inter-layer `compute_alive`, or a forced positive's
# writeback and its **root** re-saturation, whose firings are not a fork's.
BOOKKEEPING = frozenset(("hyp", "writeback", "nogood"))


def tally(block: list[dict], commitment: set[str], into: Counter) -> None:
    """Fold one contiguous run of events into `into`, splitting `alt` by the
    firing it belongs to — which is the *next* `fire` line, not the previous."""
    pending = 0
    local = set(commitment)
    for d in block:
        e = d["e"]
        into[e] += 1
        if e == "alt":
            pending += 1
        elif e == "fire":
            red = bool(d.get("redundant"))
            into["red" if red else "prod"] += 1
            if pending:
                into["alt_red" if red else "alt_prod"] += pending
                if any(p in local for p in d.get("premises", ())):
                    into["alt_local"] += pending
                else:
                    into["alt_root_only"] += pending
                pending = 0
            if not red:
                local.update(d.get("derived", ()))


def record(path: Path) -> dict:
    """Split one `--events-level verbose` stream at its `enter` events."""
    root: Counter = Counter()
    between: Counter = Counter()
    forks: list[Counter] = []
    kinds: Counter = Counter()
    mismatched = 0
    block: list[dict] = []
    first = True
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line:
            continue
        d = json.loads(line)
        if d.get("e") != "enter":
            block.append(d)
            continue
        kinds[d.get("kind", "?")] += 1
        # The entering owns the tail of the block after the last bookkeeping
        # event; the head is root saturation (the first block) or the
        # inter-entering work of the previous one.
        cut = 0
        for i, x in enumerate(block):
            if x["e"] in BOOKKEEPING:
                cut = i + 1
        head, tail = block[:cut], block[cut:]
        n = sum(1 for x in tail if x["e"] == "fire")
        if n != d.get("n_firings", n):
            mismatched += 1
        fork: Counter = Counter()
        tally(tail, set(d.get("commitment", ())), fork)
        forks.append(fork)
        tally(head, set(), root if first else between)
        first = False
        block = []
    # Whatever follows the last `enter` — the final `complete`, the verdict —
    # is not an entering.
    tally(block, set(), between)
    return {"root": root, "between": between, "forks": forks,
            "kinds": kinds, "mismatched": mismatched}


def totals(r: dict) -> dict:
    forks, root = r["forks"], r["root"]
    tot: Counter = Counter()
    for f in forks:
        tot.update(f)
    fire = tot["fire"] or 1
    return {
        "enterings": len(forks),
        "kinds": dict(r["kinds"]),
        "mismatched": r["mismatched"],
        "between": dict(r["between"]),
        "fork": dict(tot),
        "root": dict(root),
        "redundant_pct": tot["red"] / fire * 100,
        "per_entering": {
            k: (statistics.mean([f[k] for f in forks]) if forks else 0.0) for k in ROWS
        },
    }


def group(n: int | float) -> str:
    if isinstance(n, float) and n != int(n):
        return f"{n:,.1f}".replace(",", " ")
    return f"{int(n):,}".replace(",", " ")


def report(cells: list[tuple[str, dict]]) -> None:
    w = 16
    print(f"\n{'':<14}" + "".join(f"{c[0]:>{w}}" for c in cells))
    print("─" * (14 + w * len(cells)))

    def row(label: str, fn) -> None:
        print(f"{label:<14}" + "".join(f"{fn(c[1]):>{w}}" for c in cells))

    row("enterings", lambda t: group(t["enterings"]))
    row("alive / dead", lambda t: f"{t['kinds'].get('alive', 0)} / "
        f"{t['kinds'].get('dead-pre', 0) + t['kinds'].get('dead-post', 0)}")
    row("fork firings", lambda t: group(t["fork"].get("fire", 0)))
    row("  redundant", lambda t: f"{group(t['fork'].get('red', 0))} "
        f"({t['redundant_pct']:.1f}%)")
    row("  productive", lambda t: group(t["fork"].get("prod", 0)))
    row("fork enqueues", lambda t: group(t["fork"].get("enqueue", 0)))
    row("fork compiles", lambda t: group(t["fork"].get("compile", 0)))
    if any(t["mismatched"] for _, t in cells):
        row("! n_firings", lambda t: f"{t['mismatched']} block(s) cut wrong")

    print("\n── per entering, and the root for scale ──")
    print("   (`root` is root saturation + phase-1 hypgen; `between` is the\n"
          "    inter-layer work — `compute_alive`, forced positives and their\n"
          "    *root* re-saturations — which belongs to neither)")
    print(f"{'':<14}" + "".join(f"{r:>10}" for r in ROWS))
    for name, t in cells:
        print(f"{name + ' root':<14}"
              + "".join(f"{group(t['root'].get(r, 0)):>10}" for r in ROWS))
        print(f"{name + ' betw':<14}"
              + "".join(f"{group(t['between'].get(r, 0)):>10}" for r in ROWS))
        print(f"{name + ' fork':<14}"
              + "".join(f"{group(t['per_entering'][r]):>10}" for r in ROWS))

    print("\n── `alt`: every one of them comes from a redundant firing ──")
    print(f"{'':<14}{'total':>10}{'in forks':>10}{'own firing':>12}"
          f"{'premises':>10}{'root-only':>11}")
    print(f"{'':<14}{'':>10}{'':>10}{'redundant':>12}{'local':>10}{'':>11}")
    for name, t in cells:
        f = t["fork"]
        total = f.get("alt", 0) + t["root"].get("alt", 0)
        print(f"{name:<14}{group(total):>10}{group(f.get('alt', 0)):>10}"
              f"{group(f.get('alt_red', 0)):>12}{group(f.get('alt_local', 0)):>10}"
              f"{group(f.get('alt_root_only', 0)):>11}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR")
    ap.add_argument("--bin", type=Path, default=EIN, help=f"default {EIN}")
    ap.add_argument("--events", type=Path, default=None,
                    help="read this stream instead of running anything")
    ap.add_argument("--json", type=Path, default=None)
    ap.add_argument("cmd", nargs=argparse.REMAINDER,
                    help="an explicit ein command after `--`")
    args = ap.parse_args()

    cmd = [c for c in args.cmd if c != "--"]
    if args.events:
        cells = [(args.events.name, totals(record(args.events)))]
    else:
        work = CELLS if not cmd else [(" ".join(cmd), cmd)]
        tmp = Path(tempfile.mkdtemp(prefix="ein-fork-split-"))
        cells = []
        for name, argv in work:
            if args.only and args.only not in name:
                continue
            out = tmp / (name.replace(" ", "_").replace("/", "_") + ".jsonl")
            print(f"… {name}", file=sys.stderr, flush=True)
            r = subprocess.run(
                [str(args.bin), *argv, "--events", str(out),
                 "--events-level", "verbose"],
                cwd=REPO, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                text=True)
            if r.returncode != 0:
                print(r.stderr, file=sys.stderr)
                return 1
            cells.append((name, totals(record(out))))
    if not cells:
        print("nothing selected", file=sys.stderr)
        return 1

    report(cells)
    if args.json:
        args.json.write_text(json.dumps(
            {"bin": str(args.bin), "cells": [{"cell": n, **t} for n, t in cells]},
            indent=2) + "\n", encoding="utf-8")
        print(f"\nartifact: {args.json}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
