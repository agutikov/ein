#!/usr/bin/env python3
"""T1a.6.9.2 — does the *resumed* fork saturator reach the same fork?

    cargo build --release --features fork-delta --target-dir target-fd
    python3 utils/fork_delta_verify.py
    python3 utils/fork_delta_verify.py -k zebra --json out.json
    python3 utils/fork_delta_verify.py --with-trace     # size the narration too

Runs **one** binary twice over the whole parity corpus — `EIN_FORK_DELTA=0`,
then unset — so the only difference between the arms is `Saturator::new`
against `Saturator::resume`
([S1a.6.9](../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)).
The resumed saturator is the shipping path; a `fork-delta` build compiles in
the way back to the old one, and this script is
[D3](../plans/m1a_rust/divergences.md)'s fixture — the divergence stays
measured, so it cannot silently widen.

The point is to compare artefacts that are **not** firing lists, because the
firing list is what the change is expected to move. Per entering, from
`$EIN_FORK_AUDIT` (`ein_infer::fork_audit`, `fork-delta` only):

- `kind` and `core` — how the entering ended and its unsat core;
- `state` — the fork's fact set at quiescence, **fact by fact**, split alive
  from dead: `enable_fail_fast_fork` stops a *dying* fork at the firing that
  kills it, so two firing orders leave two different partial states by design.
  An **alive** fork always runs to quiescence, so its state is the fixpoint
  claim itself;
- `primary` — the first-recorded justification of each fact, which is what
  `--trace` renders and what `explain` walks;
- `alt-set` / `alt-order` — the rest of the fact's OR-node, distinguished
  because a reordering and a membership change are different findings.

And per run, from the process itself: `stdout` (the verdict, `k`, the models,
the query bindings, and for an unsat puzzle the core — the **answer**),
`summary.json` (**T0 + T1**: the verdict and every counter the engine
publishes, with only the clock normalised out) and the `--dump-states` tree
(the lattice, the no-good clauses). Wall-clock fields and firing counts are normalised out of both and
reported separately — those are the narration, which is
[T1a.6.9.3](../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)'s
number rather than an invariant.

`--no-fail-fast` re-runs each entry with `(config :enable-fail-fast-fork
false)` appended, which is how the *dead* forks' fixpoints get compared too.

Exit code is 1 if a **hard** invariant moved — `kind`, `core`, an alive
fork's state, the entering count, or a normalised stdout / state dump. A
proof-structure move (`primary`, `alt-*`) is reported in full and does not set
the exit code, because it is the finding rather than a failure of the run.
"""
from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from collections import Counter, defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
FD = REPO / "ein.rs" / "target-fd" / "release" / "ein"

# The runs that fork. `saturate` and `render` never call `try_commitment_set`.
SOLVE_RUNS = ("solve", "solve -e", "solve -n 3", "solve -m 2", "solve -p -s",
              "solve --dump-states {out}/states")
# Answer-printing forms the corpus does not declare, added to every entry that
# declares the run they extend. `-p` prints the model, or for an unsat verdict
# the unsat core — which *is* the answer there, and the one artefact a
# `solve`-only run would not have compared.
ANSWER_RUNS = {"solve": ("solve -p", "solve -P", "solve -f"),
               "solve -e": ("solve -e -p",)}
TRACE_RUN = "solve --trace {out}/trace.md"


# Wall-clock and firing counts are *expected* to move. Strip them before
# comparing text, and count the firings separately.
VOLATILE = [
    re.compile(r'"ts_ms":\s*[0-9.]+'),
    re.compile(r'"elapsed_seconds":\s*[0-9.]+'),
    re.compile(r'"wall_seconds":\s*[0-9.]+'),
    re.compile(r"^(\s*wall\s+).*$", re.M),
    re.compile(r'"firings":\s*\d+'),
    re.compile(r'"n_firings":\s*\d+'),
    re.compile(r"^(\s*(?:firings|derivations)\s+)[\d ]+$", re.M),
]
FIRINGS = re.compile(r'"(?:n_)?firings":\s*(\d+)')
# `summary.json` is T0 + T1 — the verdict and **every counter the engine
# publishes**. Only the clock is normalised out of it: a firing count in here
# would be a T1 move, which is the one thing this class exists to catch.
TIME_ONLY = [
    re.compile(r'"elapsed_seconds":\s*[0-9.]+'),
    re.compile(r'"wall_seconds":\s*[0-9.]+'),
    re.compile(r'"[a-z_]*(?:ms|nanos|seconds)":\s*[0-9.]+'),
]

# The hard invariants — a move in one of these is a failure of the idea, not a
# finding about the narration.
HARD = ("enterings", "kind", "core", "state-alive", "stdout", "summary", "dump")


def normalise(text: str) -> str:
    for r in VOLATILE:
        text = r.sub("~", text)
    return text


def clock_only(text: str) -> str:
    for r in TIME_ONLY:
        text = r.sub("~", text)
    return text


def arm(binary: Path, argv: list[str], out: Path, delta: bool,
        audit: Path) -> dict:
    """One process. Returns everything the comparison reads."""
    out.mkdir(parents=True, exist_ok=True)
    env = {"EIN_FORK_AUDIT": str(audit), "PATH": "/usr/bin:/bin"}
    if not delta:
        # The way back to the pre-S1a.6.9 fresh fork saturator.
        env["EIN_FORK_DELTA"] = "0"
    cmd = [str(binary)] + [t.replace("{out}", str(out)) for t in argv]
    # `--json-summary` on every solve cell, exactly as `plan.rs` does: it is
    # what T0 and T1 read, and comparing it is the difference between "the
    # answers agree" and "the engines did the same search".
    cmd += ["--json-summary", str(out / "summary.json")]
    p = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True, env=env)
    tree, firings, summary = {}, 0, ""
    for f in sorted(out.rglob("*")):
        if f.is_file():
            raw = f.read_text(errors="replace")
            firings += sum(int(m) for m in FIRINGS.findall(raw))
            if f.name == "summary.json":
                summary = clock_only(raw)
                continue
            tree[str(f.relative_to(out))] = normalise(raw)
    return {"code": p.returncode, "stdout": normalise(p.stdout),
            "stderr": p.stderr, "tree": tree, "audit": audit,
            "firings": firings, "summary": summary}


def compare(a: dict, b: dict, ex: dict[str, list[str]]) -> Counter:
    """Classify every way the two arms differ. `ex` collects one example each."""
    n: Counter = Counter()

    def hit(cls: str, detail: str) -> None:
        n[cls] += 1
        ex.setdefault(cls, []).append(detail)

    if a["code"] != b["code"]:
        hit("stdout", f"exit code {a['code']} → {b['code']}")
    if a["stdout"] != b["stdout"]:
        hit("stdout", "stdout text")
    if a["summary"] != b["summary"]:
        import difflib
        d = [l for l in difflib.unified_diff(
            a["summary"].splitlines(), b["summary"].splitlines(), n=0)
            if l[:1] in "+-" and l[:3] not in ("---", "+++")]
        hit("summary", "summary.json (T0+T1): " + " ".join(d[:6]))
    if a["tree"].keys() != b["tree"].keys():
        hit("dump", "the dump file set")
    else:
        for k in a["tree"]:
            if a["tree"][k] != b["tree"][k]:
                hit("dump", f"dump {k}")
    # Streamed, one entering at a time: an exhaustive solve of the bigger
    # saturation fixtures records hundreds of thousands of enterings, each
    # carrying its whole fact set and every justification of every fact, and
    # holding two of those lists in memory is how this script first died.
    n_a = n_b = 0
    for i, (x, y) in enumerate(records(a["audit"], b["audit"])):
        if x is None or y is None:
            n_a += x is not None
            n_b += y is not None
            continue
        n_a += 1
        n_b += 1
        where = f"entering {i} {' '.join(x['commitment']) or '<root>'}"
        if x["commitment"] != y["commitment"]:
            hit("enterings", f"{where}: a different commitment")
            continue
        if x["kind"] != y["kind"]:
            hit("kind", f"{where}: {x['kind']} → {y['kind']}")
        if x["core"] != y["core"]:
            hit("core", f"{where} ({x['kind']}): {x['core']} → {y['core']}")
        if x["state"] != y["state"]:
            gone = sorted(set(x["state"]) - set(y["state"]))
            new = sorted(set(y["state"]) - set(x["state"]))
            cls = "state-alive" if x["kind"] == "alive" else "state-dead-partial"
            hit(cls, f"{where} ({x['kind']}): −{len(gone)} +{len(new)} "
                     f"{gone[:2]} {new[:2]}")
            continue
        # By fact, not by position: the two arms intern in different orders,
        # so only the rendered key is common ground.
        jb_of = dict(y["just"])
        for f, ja in x["just"]:
            jb = jb_of[f]
            if ja == jb:
                continue
            if ja[:1] != jb[:1]:
                hit("primary", f"{where}: {f}\n         was {ja[0][:110]}"
                               f"\n         now {jb[0][:110]}")
            elif set(ja) != set(jb):
                hit("alt-set", f"{where}: {f} "
                               f"−{sorted(set(ja) - set(jb))[:1]} "
                               f"+{sorted(set(jb) - set(ja))[:1]}")
            else:
                hit("alt-order", f"{where}: {f} ({len(ja)} justifications)")
    if n_a != n_b:
        hit("enterings", f"{n_a} → {n_b}")
    return n


def records(pa: Path, pb: Path):
    """Both audit files, one entering at a time, padded with `None`."""
    fa = pa.open(encoding="utf-8") if pa.exists() else None
    fb = pb.open(encoding="utf-8") if pb.exists() else None
    try:
        while True:
            la = fa.readline() if fa else ""
            lb = fb.readline() if fb else ""
            if not la and not lb:
                return
            yield (json.loads(la) if la.strip() else None,
                   json.loads(lb) if lb.strip() else None)
    finally:
        if fa:
            fa.close()
        if fb:
            fb.close()


def variant_no_fail_fast(path: Path, tmp: Path) -> Path:
    """The same entry with fail-fast off, written **beside** the original so a
    relative `(import …)` still resolves."""
    src = REPO / path
    dst = src.with_suffix(".ffoff.ein")
    dst.write_text(src.read_text() + "\n(config :enable-fail-fast-fork false)\n")
    return dst


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR")
    ap.add_argument("--bin", type=Path, default=FD)
    ap.add_argument("--skip-slow", action="store_true")
    ap.add_argument("--no-fail-fast", action="store_true",
                    help="append (config :enable-fail-fast-fork false), so the "
                         "dead forks' fixpoints are compared too")
    ap.add_argument("--with-trace", action="store_true",
                    help="also report how far the narration moved")
    ap.add_argument("--max-report", type=int, default=4)
    ap.add_argument("--json", type=Path, default=None)
    args = ap.parse_args()

    if not args.bin.exists():
        print(f"missing {args.bin} — cargo build --release --features "
              f"fork-delta --target-dir target-fd", file=sys.stderr)
        return 2

    corpus = tomllib.load(open(REPO / "conformance" / "corpus.toml", "rb"))
    entries = [e for e in corpus["entry"]
               if e["group"] in ("positive", "stdlib")
               and not (args.only and args.only not in e["path"])
               and not (args.skip_slow and e.get("slow"))]

    tmp = Path(tempfile.mkdtemp(prefix="ein-fork-delta-"))
    variants: list[Path] = []
    work: list[tuple[str, list[str]]] = []
    try:
        for e in entries:
            path = e["path"]
            if args.no_fail_fast:
                v = variant_no_fail_fast(Path(path), tmp)
                variants.append(v)
                path = str(v.relative_to(REPO))
            runs = list(SOLVE_RUNS) + ([TRACE_RUN] if args.with_trace else [])
            declared = e.get("runs", [])
            for r in declared:
                if r in runs:
                    work.append((path, r.split() + [path]))
                for extra in ANSWER_RUNS.get(r, ()):
                    work.append((path, extra.split() + [path]))

        totals: Counter = Counter()
        per_entry: dict[str, Counter] = defaultdict(Counter)
        ex: dict[str, list[str]] = {}
        enterings = checked = 0
        fire_a = fire_b = trace_a = trace_b = 0
        for i, (path, argv) in enumerate(work):
            label = f"{' '.join(argv[:-1])} {path}"
            print(f"\r… {i + 1}/{len(work)} {label[:70]:<70}", end="",
                  file=sys.stderr, flush=True)
            a = arm(args.bin, argv, tmp / f"{i}a", False, tmp / f"{i}a.jsonl")
            b = arm(args.bin, argv, tmp / f"{i}b", True, tmp / f"{i}b.jsonl")
            enterings += sum(1 for _ in a["audit"].open(encoding="utf-8")) \
                if a["audit"].exists() else 0
            checked += 1
            if "--trace" in argv:
                trace_a += sum(len(v.splitlines()) for v in a["tree"].values())
                trace_b += sum(len(v.splitlines()) for v in b["tree"].values())
                continue
            fire_a += a["firings"]
            fire_b += b["firings"]
            n = compare(a, b, ex)
            totals.update(n)
            for cls, v in n.items():
                per_entry[path][cls] += v
            # Reclaim as we go: the audit of one exhaustive run of the bigger
            # fixtures is hundreds of megabytes.
            a["audit"].unlink(missing_ok=True)
            b["audit"].unlink(missing_ok=True)
            shutil.rmtree(tmp / f"{i}a", ignore_errors=True)
            shutil.rmtree(tmp / f"{i}b", ignore_errors=True)
    finally:
        for v in variants:
            v.unlink(missing_ok=True)

    ff = "off" if args.no_fail_fast else "on (the shipping default)"
    print(f"\r{' ' * 78}\r{checked} run(s) over {len(entries)} corpus "
          f"entries, fail-fast {ff};\n{enterings} enterings compared "
          f"fact by fact, justification by justification")
    if fire_a:
        print(f"\nnarration (not an invariant): {fire_a} → {fire_b} firings "
              f"across the dumps ({(fire_b / fire_a - 1) * 100:+.1f} %)")
    if args.with_trace and trace_a:
        print(f"trace lines: {trace_a} → {trace_b} "
              f"({(trace_b / trace_a - 1) * 100:+.1f} %)")

    hard = sum(totals[c] for c in HARD)
    print(f"\n  {'class':<20}{'moves':>8}   {'':<8}")
    for cls in (*HARD, "state-dead-partial", "primary", "alt-set", "alt-order"):
        mark = "HARD" if cls in HARD else "proof"
        print(f"  {cls:<20}{totals[cls]:>8}   {mark}")
    for cls in (*HARD, "state-dead-partial", "primary", "alt-set", "alt-order"):
        if not totals[cls]:
            continue
        print(f"\n  ── {cls} ──")
        for line in ex[cls][: args.max_report]:
            print(f"     {line}")
        if totals[cls] > args.max_report:
            print(f"     … {totals[cls] - args.max_report} more")
    worst = sorted(per_entry.items(), key=lambda kv: -sum(kv[1].values()))
    if worst and sum(worst[0][1].values()):
        print("\n  ── by corpus entry ──")
        for path, n in worst[:6]:
            if not sum(n.values()):
                break
            print(f"     {path:<52}"
                  + " ".join(f"{k}={v}" for k, v in sorted(n.items())))

    if not hard:
        print("\n  every hard invariant holds: the entering count, each "
              "entering's kind and\n  unsat core, every alive fork's fixpoint "
              "fact by fact, stdout and the\n  state dumps once the firing "
              "counts are normalised out")

    if args.json:
        args.json.write_text(json.dumps({
            "bin": str(args.bin), "fail_fast": not args.no_fail_fast,
            "runs": checked, "entries": len(entries), "enterings": enterings,
            "firings": [fire_a, fire_b], "trace_lines": [trace_a, trace_b],
            "moves": dict(totals),
            "per_entry": {k: dict(v) for k, v in sorted(per_entry.items())},
            "examples": ex,
        }, indent=2) + "\n", encoding="utf-8")
        print(f"\nartifact: {args.json}", file=sys.stderr)
    shutil.rmtree(tmp, ignore_errors=True)
    return 1 if hard else 0


if __name__ == "__main__":
    raise SystemExit(main())
