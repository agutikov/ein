#!/usr/bin/env python3
"""S1a.7.0 — how often would a speculated entering have been wrong?

    cargo build --release --features spec-audit --target-dir target-sa
    python3 utils/spec_audit.py
    python3 utils/spec_audit.py -k zebra --json out.json
    python3 utils/spec_audit.py --no-fail-fast     # compare dead fixpoints too

[design/08](../plans/m1a_rust/design/08_parallelism.md) §2 evaluates a whole
layer's enterings against `R0` — root as it stood when the layer opened — and
then commits them in candidate order, validating each against the write set `W`
that the earlier commits produced. Three cases; only the third costs anything,
and [Q-M1a.7](../plans/m1a_rust/open_questions.md) asks how often it fires.
The phase's acceptance says "≤ a few percent".

That is measurable **before any of it is built**, and this measures it: one
process per run, the sequential engine, and beside every entering the same
entering re-run against `R0` (`ein_infer::spec_audit`, `spec-audit` builds
only). Where the two agree the speculation would have stood as computed; where
they disagree, something would have had to correct it.

Reported per run and summed over the corpus:

- **case 1 / 2 / 3** — the classification design/08 §2 defines. Case 1 is also
  the instrument's control: both arms fork the same root, so a difference there
  is a nondeterminism and not a finding about `W`.
- **kind** — `alive` / `dead-pre` / `dead-post`. A move here moves
  `enterings_alive` and `enterings_dead_*`, which are T1 counters.
- **core** — the entering's unsat core, which reaches the `enter` event and,
  through `union_dead_cores`, an unsat verdict's printed core.
- **state (alive)** — an alive fork runs to quiescence, so its fact set is the
  fixpoint claim itself and any difference is semantic. A **dead** fork under
  `enable_fail_fast_fork` stops at the firing that killed it, so its state is a
  firing-order-dependent prefix; `--no-fail-fast` is what compares those
  fixpoints instead.
- **past W** — a case-3 fork inherits `W` from root and a speculative one does
  not, so every case-3 entering differs by at least `W` itself. This counts the
  enterings that differ by something *derived from* a mid-layer write, which is
  the work a continuation would have to do.

Exit code is 1 when any `kind` moved: that is the number the phase's contract
is written against.
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
import tomllib
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
SA = REPO / "ein.rs" / "target-sa" / "release" / "ein"

# The runs that fork. `saturate` and `render` never call `try_commitment_set`,
# and `-e` is where a layer is large enough for the question to mean anything.
SOLVE_RUNS = ("solve", "solve -e")


def variant_no_fail_fast(path: Path, tmp: Path) -> Path:
    """The same entry with fail-fast off, written **beside** the original so a
    relative `(import …)` still resolves."""
    src = REPO / path
    dst = src.with_suffix(".ffoff.ein")
    dst.write_text(src.read_text() + "\n(config :enable-fail-fast-fork false)\n")
    return dst


def run(binary: Path, argv: list[str], audit: Path, timeout: float) -> list[dict]:
    env = {"EIN_SPEC_AUDIT": str(audit), "PATH": "/usr/bin:/bin"}
    try:
        subprocess.run([str(binary), *argv], cwd=REPO, capture_output=True,
                       text=True, env=env, timeout=timeout)
    except subprocess.TimeoutExpired:
        pass  # a partial audit is still a measurement of what it reached
    if not audit.exists():
        return []
    rows = []
    for line in audit.open(encoding="utf-8"):
        try:
            rows.append(json.loads(line))
        except json.JSONDecodeError:
            break  # a timeout can cut the last line in half
    return rows


def tally(rows: list[dict], t: Counter) -> None:
    for r in rows:
        if "error" in r:
            t["error"] += 1
            continue
        t["enterings"] += 1
        t[f"case{r['case']}"] += 1
        alive = r["kind"] == "alive"
        t["alive" if alive else "dead"] += 1
        if not r["same_kind"]:
            t["kind_moved"] += 1
        if not r["same_core"]:
            t["core_moved"] += 1
        if not r["same_state"]:
            t["state_moved"] += 1
            if alive:
                t["state_moved_alive"] += 1
        if r["n_derived_only_seq"]:
            t["past_w"] += 1
        if r["n_only_spec"]:
            t["spec_extra"] += 1
        if r["n_firings"] != r["spec_n_firings"]:
            t["firings_moved"] += 1
        t["w_max"] = max(t["w_max"], r["w"])
        # The control: case 1 forks the same root in both arms.
        if r["case"] == 1 and not (r["same_kind"] and r["same_core"]
                                   and r["same_state"]):
            t["control_broke"] += 1


def pct(n: int, d: int) -> str:
    return f"{100 * n / d:5.1f}%" if d else "    — "


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR")
    ap.add_argument("--bin", type=Path, default=SA)
    ap.add_argument("--skip-slow", action="store_true")
    ap.add_argument("--no-fail-fast", action="store_true",
                    help="append (config :enable-fail-fast-fork false), so a "
                         "dead fork's fixpoint is compared rather than its "
                         "fail-fast prefix")
    ap.add_argument("--timeout", type=float, default=60.0)
    ap.add_argument("--max-report", type=int, default=4)
    ap.add_argument("--json", type=Path, default=None)
    args = ap.parse_args()

    if not args.bin.exists():
        print(f"missing {args.bin} — cargo build --release --features "
              f"spec-audit --target-dir target-sa", file=sys.stderr)
        return 2

    corpus = tomllib.load(open(REPO / "corpus" / "corpus.toml", "rb"))
    # `regression` was `positive` + `crash-parity` until S1a.10.3 regrouped
    # `examples/ein-bugs/` as one directory; five of its ten entries were in
    # this selection before and are again. Two of the rest are refused, which
    # costs nothing here — a run that writes no audit rows contributes none.
    entries = [e for e in corpus["entry"]
               if e["group"] in ("positive", "stdlib", "regression")
               and not (args.only and args.only not in e["path"])
               and not (args.skip_slow and e.get("slow"))]

    tmp = Path(tempfile.mkdtemp(prefix="ein-spec-audit-"))
    variants: list[Path] = []
    work: list[tuple[str, list[str]]] = []
    for e in entries:
        path = e["path"]
        if args.no_fail_fast:
            v = variant_no_fail_fast(Path(path), tmp)
            variants.append(v)
            path = str(v.relative_to(REPO))
        for r in e.get("runs", []):
            if r in SOLVE_RUNS:
                work.append((path, r.split() + [path]))

    totals: Counter = Counter()
    per_run: list[tuple[str, Counter]] = []
    examples: list[dict] = []
    try:
        for i, (path, argv) in enumerate(work):
            label = f"{' '.join(argv[:-1])} {path}"
            print(f"\r… {i + 1}/{len(work)} {label[:70]:<70}", end="",
                  file=sys.stderr, flush=True)
            rows = run(args.bin, argv, tmp / f"{i}.jsonl", args.timeout)
            t: Counter = Counter()
            tally(rows, t)
            if t["enterings"]:
                per_run.append((label, t))
                totals.update({k: v for k, v in t.items() if k != "w_max"})
                totals["w_max"] = max(totals["w_max"], t["w_max"])
            for r in rows:
                if len(examples) < args.max_report and not r.get("same", True) \
                        and not r.get("same_kind", True):
                    examples.append({"run": label, **r})
    finally:
        for v in variants:
            v.unlink(missing_ok=True)

    ff = "off" if args.no_fail_fast else "on"
    n = totals["enterings"]
    print(f"\r{' ' * 78}\r{len(per_run)} run(s) over {len(entries)} corpus "
          f"entries, fail-fast {ff}\n{n} enterings speculated and compared, "
          f"max |W| = {totals['w_max']}\n")
    print(f"  case 1  (W empty, the control)  {totals['case1']:7}  "
          f"{pct(totals['case1'], n)}")
    print(f"  case 2  (c meets ¬W)            {totals['case2']:7}  "
          f"{pct(totals['case2'], n)}")
    print(f"  case 3  (W disjoint from c)     {totals['case3']:7}  "
          f"{pct(totals['case3'], n)}   ← the re-validation rate")
    print()
    print(f"  kind moved                     {totals['kind_moved']:7}  "
          f"{pct(totals['kind_moved'], n)}   ← T1 counters")
    print(f"  core moved                     {totals['core_moved']:7}  "
          f"{pct(totals['core_moved'], n)}")
    print(f"  state moved (alive forks)      {totals['state_moved_alive']:7}  "
          f"{pct(totals['state_moved_alive'], totals['alive'])} of "
          f"{totals['alive']} alive")
    print(f"  state moved past W             {totals['past_w']:7}  "
          f"{pct(totals['past_w'], n)}")
    print(f"  firing count moved             {totals['firings_moved']:7}  "
          f"{pct(totals['firings_moved'], n)}")
    if totals["control_broke"]:
        print(f"\n  ** control broke on {totals['control_broke']} case-1 "
              f"enterings — the instrument, not the finding **")
    if totals["error"]:
        print(f"  ({totals['error']} speculative arms errored)")

    worst = sorted(per_run, key=lambda kv: -kv[1]["kind_moved"])[:8]
    if worst and worst[0][1]["kind_moved"]:
        print("\n  where the kind moved most")
        for label, t in worst:
            if not t["kind_moved"]:
                break
            print(f"    {t['kind_moved']:5} / {t['enterings']:5}  {label}")
    for e in examples:
        print(f"\n  ── {e['run']}  layer {e['layer']} entering {e['i']}  "
              f"(case {e['case']}, |W| = {e['w']})")
        print(f"     commitment       {e['commitment']}")
        print(f"     sequential       {e['kind']}, {e['n_facts']} facts, "
              f"{e['n_firings']} firings")
        print(f"     speculative      {e['spec_kind']}, {e['spec_n_facts']} "
              f"facts, {e['spec_n_firings']} firings")
        print(f"     derived only seq {e['derived_only_seq']}")

    if args.json:
        args.json.write_text(json.dumps(
            {"fail_fast": ff, "runs": len(per_run), "entries": len(entries),
             "totals": dict(totals),
             "per_run": [{"run": k, **dict(v)} for k, v in per_run],
             "examples": examples}, indent=2) + "\n")
    return 1 if totals["kind_moved"] else 0


if __name__ == "__main__":
    sys.exit(main())
