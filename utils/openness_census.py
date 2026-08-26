#!/usr/bin/env python3
"""What every corpus entry owes, and whether it is judged by discharge —
S1d.2.6's instrument.

[S1d.2.4](../docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator)
made a quiescent state able to say what it owes; this asks the same question of
the **whole corpus**, because S1d.2.6's scope rule is a claim about it: *a
program that states no obligation keeps today's verdict*. A rule stated that
way is only as good as the count behind it, and the count is this file.

    utils/openness_census.py                  # the table, to stdout
    utils/openness_census.py --json c.json    # + the machine copy
    utils/openness_census.py --scope          # just the scope-rule partition
    utils/openness_census.py -k zebra2        # one entry, with its instances

**Three numbers per entry, and the third is the one nothing reported before.**
`declared` is how many obligation rules the program states — the scope-rule
signal, and *not* inferable from the other two, since `owes = 0` is equally
true of a debt paid and of a debt never stated. `root` is what the initial
fixpoint owes. `models` is what each recorded model owes, which is the
closed-and-owing corner as a number: a state the generator calls complete and
the tally calls unfinished.

**One run per entry, and it is `solve`** — not `solve -e`. Openness is a
property of a *state*, and every state this asks about is reached on both
paths; the exhaustive path costs the corpus four minutes to answer the same
question. `--exhaustive` takes it anyway, for the entries where the model set
is the subject.

The transport is `--json-summary`'s `owes` block, whose `declared` field
S1d.2.6 added for exactly this. Nothing here re-derives a tally from the event
stream: a census that reconstructs its own subject is a census that can
disagree with the engine, and T1d.2.4.5 already proved the engine's tally
against a hand count.

Argv follows `ein-corpus/src/plan.rs`, mirrored the way
[`layer_census.py`](layer_census.py) and [`corpus_cost.py`](corpus_cost.py)
mirror it, and for the same reason.
"""
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
MANIFEST = REPO / "corpus" / "corpus.toml"
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

# The verdict words a run can end in, in the order the table groups them.
WORDS = ("Solution", "Ambiguity", "Open", "Contradiction", "Aborted")


def measure(path: str, args, out: Path) -> dict:
    """One `solve` of one entry, reduced to its openness row."""
    argv = [str(args.bin), "solve", path, "--json-summary", str(out)]
    if args.exhaustive:
        argv.append("-e")
    row: dict = {"path": path}
    if out.exists():
        out.unlink()
    try:
        p = subprocess.run(argv, capture_output=True, text=True, timeout=args.timeout)
        row["exit"] = p.returncode
    except subprocess.TimeoutExpired:
        row["exit"] = None
        row["note"] = f"timeout at {args.timeout}s"
        return row
    if not out.exists():
        # A load error, or a budget abort with `on_budget = abort`: there is no
        # fixpoint, so there is nothing this census can say about it.
        row["note"] = "no summary"
        return row
    d = json.loads(out.read_text(encoding="utf-8"))
    v = d.get("verdict") or {}
    owes = d.get("owes") or {}
    root = owes.get("root") or {}
    models = owes.get("models") or []
    row.update(
        verdict=v.get("type"),
        k=v.get("k"),
        exhausted=v.get("exhausted"),
        declared=owes.get("declared", 0),
        root_owes=root.get("total", 0),
        root_by_relation=root.get("by_relation") or {},
        model_owes=[m.get("total", 0) for m in models],
        instances=[
            {"rule": i.get("rule"), "relation": i.get("relation"), "why": i.get("why")}
            for i in (root.get("instances") or [])
        ],
    )
    return row


def all_runs(entry: dict) -> list[str]:
    """The run names an entry declares, levers folded in — `plan.rs`'s view."""
    return [r.split()[0] for r in entry.get("runs", [])]


def classify(row: dict) -> str:
    """The scope rule's partition, which is the census's whole point.

    `out-of-scope` is the majority and the claim: those entries are judged by
    exhaustion because nothing told them what they owe. Of the rest, `owing`
    is where a verdict word can move and `discharged` is where it cannot.
    """
    if row.get("note") == "no solve run declared":
        return "not-run"
    if row.get("verdict") is None:
        return "unmeasured"
    if not row.get("declared"):
        return "out-of-scope"
    owed = row.get("root_owes", 0) + sum(row.get("model_owes") or [])
    return "owing" if owed else "discharged"


def sweep(entries: list[dict], args) -> list[dict]:
    rows = []
    with tempfile.TemporaryDirectory(prefix="openness-") as td:
        out = Path(td) / "summary.json"
        for i, e in enumerate(entries, 1):
            if args.key and args.key not in e["path"]:
                continue
            # **A run the manifest does not declare is not run here either.**
            # Four entries drop `solve` because it does not terminate on them
            # (`features/04_open` and the three `square-unique` demos: an open
            # hypothesis space, and the run ends in the OOM killer rather than
            # a verdict). Timing them out would put four rows in the census
            # that say nothing about openness and one thing about patience.
            if "solve" not in all_runs(e):
                row = {"path": e["path"], "note": "no solve run declared"}
            else:
                row = measure(e["path"], args, out)
            row["group"] = e.get("group", "")
            row["class"] = classify(row)
            rows.append(row)
            if not args.quiet:
                print(f"  [{i}/{len(entries)}] {e['path']}", file=sys.stderr)
    return rows


def print_scope(rows: list[dict]) -> None:
    """The scope-rule partition — the acceptance bullet, as a table."""
    buckets: dict[str, list[dict]] = {}
    for r in rows:
        buckets.setdefault(r["class"], []).append(r)
    print("\n## The scope rule\n")
    print(f"{'class':14} {'entries':>7}  what it means")
    print(f"{'-'*14} {'-'*7}  {'-'*54}")
    meaning = {
        "out-of-scope": "states no obligation — judged by exhaustion, word unchanged",
        "discharged": "states one and owes nothing — satisfied by discharge",
        "owing": "states one and owes something — where a word can move",
        "unmeasured": "no fixpoint to read (a load error, by design)",
        "not-run": "the manifest declares no `solve` run — an open hypothesis space",
    }
    for c in ("out-of-scope", "discharged", "owing", "unmeasured", "not-run"):
        if c in buckets:
            print(f"{c:14} {len(buckets[c]):>7}  {meaning[c]}")
    print(f"{'total':14} {len(rows):>7}")
    for c in ("owing", "discharged"):
        if not buckets.get(c):
            continue
        print(f"\n### {c}\n")
        print(f"{'entry':56} {'verdict':13} {'decl':>4} {'root':>5} {'models':>8}")
        for r in sorted(buckets[c], key=lambda r: r["path"]):
            m = r.get("model_owes") or []
            ms = ",".join(str(x) for x in m) if m else "-"
            print(f"{r['path']:56} {str(r.get('verdict')):13} "
                  f"{r.get('declared', 0):>4} {r.get('root_owes', 0):>5} {ms:>8}")


def print_words(rows: list[dict]) -> None:
    """Verdict words by scope — the invariant the acceptance bullet asserts."""
    print("\n## Verdict words, by scope\n")
    seen = sorted({r.get("verdict") for r in rows if r.get("verdict")},
                  key=lambda w: (WORDS.index(w) if w in WORDS else 99, w))
    print(f"{'scope':14} " + " ".join(f"{w:>13}" for w in seen))
    print(f"{'-'*14} " + " ".join("-" * 13 for _ in seen))
    for c in ("out-of-scope", "discharged", "owing"):
        sub = [r for r in rows if r["class"] == c]
        if not sub:
            continue
        cells = [sum(1 for r in sub if r.get("verdict") == w) for w in seen]
        print(f"{c:14} " + " ".join(f"{n:>13}" for n in cells))


def print_entry(rows: list[dict]) -> None:
    """One entry, with the debts spelled out — `-k`'s output."""
    for r in rows:
        print(f"\n## {r['path']}\n")
        print(f"  verdict    {r.get('verdict')}  k={r.get('k')}  "
              f"exhausted={r.get('exhausted')}")
        print(f"  declared   {r.get('declared', 0)} obligation rule(s)")
        print(f"  root owes  {r.get('root_owes', 0)}  {r.get('root_by_relation') or ''}")
        print(f"  models owe {r.get('model_owes') or '-'}")
        for i in r.get("instances") or []:
            rel = i.get("relation") or "(open)"
            print(f"    {i.get('rule'):24} {rel:16} {i.get('why', '')}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", default=EIN, type=Path,
                    help=f"the ein binary (default $EIN_BIN or {EIN})")
    ap.add_argument("--json", type=Path, help="also write the rows as JSON")
    ap.add_argument("-k", "--key", help="only entries whose path contains this")
    ap.add_argument("-e", "--exhaustive", action="store_true",
                    help="solve -e rather than solve (the model set is the subject)")
    ap.add_argument("--scope", action="store_true",
                    help="print only the scope-rule partition")
    ap.add_argument("--timeout", type=float, default=60.0,
                    help="seconds per entry (default 60)")
    ap.add_argument("-q", "--quiet", action="store_true", help="no progress lines")
    args = ap.parse_args()

    if not Path(args.bin).exists():
        print(f"no engine at {args.bin} — run ./build.sh, "
              f"or name one with --bin / $EIN_BIN", file=sys.stderr)
        return 2

    entries = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["entry"]
    rows = sweep(entries, args)
    if not rows:
        print("no entries matched", file=sys.stderr)
        return 2

    if args.key:
        print_entry(rows)
    else:
        print_scope(rows)
        if not args.scope:
            print_words(rows)
    if args.json:
        args.json.write_text(json.dumps(rows, indent=1), encoding="utf-8")
        print(f"\nwrote {args.json}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
