#!/usr/bin/env python3
"""`perf` attribution for ein.rs — the Rust half of `utils/profile_solve.py`.

    python3 utils/profile_ein_rs.py solve examples/zebra2.ein -e
    python3 utils/profile_ein_rs.py --repeat 5 --top 25 solve examples/zebra.ein -e
    python3 utils/profile_ein_rs.py --keep /path/perf.data solve examples/zebra2.ein

Two tables, and the second is the one that compares:

**Self time by symbol** — `perf`'s flat profile, the leaf frame of every
sample. Directly comparable to cProfile's `tottime` column.

**Self time by subsystem** — the same samples bucketed into
`profile_solve.py`'s eight categories, so the Python and Rust profiles can be
read side by side. Bucketing is by the **innermost enclosing engine frame**,
not by the leaf, and the difference is the whole reason the table is
trustworthy:

- `FactStore::get` is called by the matcher *and* by the saturator. A leaf-only
  rule has to pick one and is wrong about the other half.
- `malloc` / `memcpy` / `Vec::push` are leaves under everything. cProfile hides
  that cost inside the Python caller's `tottime`, so to stay comparable this
  walks *through* non-engine frames rather than bucketing them: allocator time
  lands on whoever asked for the memory.
- A stack with no engine frame at all is process start-up — `ld.so` resolving
  relocations, mostly — and gets its own row instead of silently inflating one
  of the engine's.

`other (engine)` is therefore a **check on the needle list**, not a category: it
counts samples inside an `ein_*` module that no needle claims, and a large value
means the mapping is stale rather than that the engine is mysterious.

Inline expansion is deliberately **off**. `perf script --inline` prints inlined
frames as bare names (`try_candidate`, `unify`), discarding exactly the module
qualification the bucketing needs; with `--no-inline` every frame arrives as
`<ein_infer::match_::Matcher>::unify`. The cost is that a symbol here is a
*surviving* function and may account for several source ones.

Unwinding is **LBR** (`--call-graph lbr`), and that choice is load-bearing.
`fp` loses the stack the moment a sample lands inside glibc, which is compiled
without frame pointers: 18 % of an exhaustive `zebra2` profile arrived as
`[libc.so.6]` with no caller, i.e. exactly the allocator cost
[S1a.6.2](../plans/m1a_rust/p1a.6_performance/s1a.6.2_memory_layout.md) needs
attributed. `dwarf,8192` truncated at two frames on this tree. LBR recovers
`malloc ← Vec::from_iter ← compile::plan_key ← Engine::compile_for ← …` in full,
and its known weakness — stale branch history in the *outer* frames — does not
touch the innermost-engine-frame rule this script uses. `--call-graph fp` is
still there for a non-Intel machine, where the `unattributed` row is the
honest report of what it cost.

It builds `--profile profiling` (see `ein.rs/Cargo.toml`) rather than `release`,
and reports the wall-clock of both, because a profile taken on a binary that
runs at a different speed from the shipped one is measuring a different
program.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
from collections import Counter, defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
EIN_RS = REPO / "ein.rs"

# Mirrors `utils/profile_solve.py::_SUBSYSTEMS` — same names, same order, same
# first-match-wins rule, needles rewritten as Rust paths. Where a Python module
# became a differently-named Rust one the mapping is noted; where ein.rs has a
# module Python does not, it joins the bucket its caller belongs to.
_SUBSYSTEMS: list[tuple[str, tuple[str, ...]]] = [
    # `store.py:fork` + `_copy_fact_indexes_into` → the COW layer machinery.
    ("fork/copy",     ("Kb::fork", "Kb::layers", "kb::Layer::", "::snapshot")),
    # `saturator.py`, `compile.py`, `firing.py` + the three store writes.
    ("saturate",      ("ein_infer::saturator", "ein_infer::compile",
                       "ein_infer::firing", "ein_infer::plan",
                       "ein_infer::engine", "add_and_index_fact",
                       "record_justification", "accepts_justification")),
    # `match.py`, `resolve.py` → `match_.rs` (argument resolution is compiled
    # into the register machine; there is no separate resolve module).
    ("match/bind",    ("ein_infer::match_",)),
    ("contradiction", ("ein_infer::contradiction", "ein_infer::nogoods",
                       "ein_infer::naf_deps", "ein_infer::sanity")),
    # `hypgen.py`, `lookahead.py`, `commitment.py` + `hrule.rs`, which is the
    # hypothesis-rule half `hypgen.py` keeps inline.
    ("hypgen/branch", ("ein_infer::hypgen", "ein_infer::lookahead",
                       "ein_infer::commitment", "ein_infer::hrule")),
    # `solver.py:_compute_alive`, `closed.py`, `solution.py` → `verdict.rs`.
    ("alive/closed",  ("ein_infer::closed", "ein_infer::verdict",
                       "compute_alive", "ein_infer::solve")),
    ("canon/key",     ("ein_infer::canon", "state_key")),
    ("apriori/elim",  ("ein_infer::apriori", "ein_infer::predicates",
                       "ein_infer::explain")),
    # Not a Python bucket: the frontend is another phase's row, and without it
    # parse and load would land in `other (engine)` and read as unattributed
    # engine time. `ein_render` joins it — a `render` workload is all frontend
    # and presentation, and neither is what the eight engine buckets are for.
    ("frontend/load", ("ein_ir::", "ein_render::", "ein_cli::")),
]

# Which frames the walk-outward rule may *stop* on. A frame outside this set is
# transparent — `malloc`, `memcpy`, `Vec::push`, `FactStore::get`, `Terms::
# intern_text` — so its cost lands on the engine function that asked for it,
# which is where cProfile's `tottime` puts the equivalent C-level work. That is
# also why `ein_core::` is *not* here: the data model is a leaf under everything,
# and attributing it to itself would answer "what is slow" with "the KB".
_STOP_ON = ("ein_infer::", "ein_ir::", "ein_render::", "ein_cli::")

# `<addr> <symbol>[+0xNN] (<dso>)`. Anchored on the **last** parenthesised group
# and greedy up to it: a non-greedy symbol group stops at the first ` (` and
# truncates `HashMap<BindingKey, (usize, usize), FxBuildHasher>>::insert` — which
# it did, silently, until the tuple showed up in a profile.
_FRAME = re.compile(r"^\s+[0-9a-f]+\s+(.*)\s+\(([^()]*)\)\s*$")
_OFFSET = re.compile(r"\+0x[0-9a-f]+$")
# `>::` as well as `::`, so `<ein_core::kb::Kb>::n_facts_of` is one path and not
# two — the qualified name is what identifies the work that got inlined here.
_ENGINE_PATH = re.compile(
    r"ein_(?:infer|core|ir|render|cli)(?:(?:::|>::)[A-Za-z0-9_]+)+")


def normalise(sym: str) -> str:
    """`<ein_infer::match_::Matcher>::run::<closure#0>` → `ein_infer::match_::
    Matcher::run`.

    Bucketing on the raw symbol is wrong in both directions, and both were
    observed on the first run of this script: a `core::iter` adapter
    monomorphised over `ein_core::kb::Layer` matched a `fork/copy` needle
    (`::layer`) although no fork was involved, and a matcher entry point
    monomorphised over a saturator closure matched `saturate` before
    `match/bind` because the needle order put it first. What a frame *is* is its
    own path; what its generic arguments mention is somebody else's.
    """
    i = 0
    head = ""
    # A leading `<Type as Trait>` / `<Type>` owner: keep the type, drop the rest.
    if sym.startswith("<"):
        depth, i = 1, 1
        owner: list[str] = []
        while i < len(sym) and depth:
            c = sym[i]
            if c == "<":
                depth += 1
            elif c == ">":
                depth -= 1
                if depth == 0:
                    break
            elif depth == 1:
                owner.append(c)
            i += 1
        # `Vec<T> as SpecFromIterNested<…>` → the impl's own type, not the trait.
        head = "".join(owner).split(" as ")[0]
        i += 1
        if sym[i:i + 2] == "::":
            i += 2
    # The rest, with every generic argument group removed.
    tail: list[str] = []
    depth = 0
    for c in sym[i:]:
        if c == "<":
            depth += 1
        elif c == ">":
            depth = max(0, depth - 1)
        elif depth == 0:
            tail.append(c)
    parts = [x for x in (head, "".join(tail)) if x]
    return "::".join(parts).replace("::::", "::").strip(": ")


def display(sym: str) -> str:
    """A flat-table label. The optimiser produces symbols like a 900-character
    `Chain<Map<Iter<Arc<Layer>>…>>::fold`, whose *owner* is `core::iter` and
    whose actual work is the engine closure monomorphised into it; printing the
    raw symbol buries the finding, and printing only the owner deletes it. So:
    the normalised path, plus the innermost engine path mentioned anywhere in
    it when that is not already the owner."""
    path = normalise(sym)
    if path.startswith(("ein_", "[")):
        return path
    hits = list(_ENGINE_PATH.finditer(sym))
    if hits:
        # Prefer a path rustc marked as a closure: in a monomorphised iterator
        # adapter those are exactly the engine bodies that got inlined into it,
        # while the bare mentions are usually just element *types* (`FactId`,
        # `ProvId`) and name nothing that runs. Then by how often it recurs
        # through the type, then by length.
        counts = Counter(h.group(0) for h in hits)

        def score(h: re.Match[str]) -> tuple[int, int, int]:
            closure = sym[h.end():h.end() + 10] == "::{closure"
            return (int(closure), counts[h.group(0)], len(h.group(0)))

        via = max(hits, key=score).group(0).replace(">::", "::")
        return f"{path} ⟨{via}⟩"
    return path


def bucket_of(path: str) -> str | None:
    for name, needles in _SUBSYSTEMS:
        if any(nd in path for nd in needles):
            return name
    return None


def parse_script(text: str, cum_needles: tuple[str, ...] = ()) -> dict:
    """Self time by leaf symbol and by subsystem, plus the two cumulative views.

    One unit per sample; `perf record -F` makes samples equal-weight, so a count
    *is* a time share and no cycle counting is needed.

    **Cumulative** is per-sample presence, deduplicated: a stack that passes
    through the saturator twice counts once. That makes the column comparable to
    cProfile's `cumtime` — "how much of the run happens under this" — and it is
    the only column that can answer the question S1a.6.1 asks out loud, "is the
    boundary still 72 %?", because self time cannot see a caller that spends all
    its time in a callee.
    """
    leaves: Counter = Counter()
    subs: Counter = Counter()
    cum_subs: Counter = Counter()
    cum_needle: Counter = Counter()
    callers: dict[str, Counter] = defaultdict(Counter)
    stack: list[str] = []

    def flush() -> None:
        if not stack:
            return
        leaf = stack[0]
        leaves[leaf] += 1
        for frame in stack:
            path = normalise(frame)
            b = bucket_of(path)
            if b is not None:
                subs[b] += 1
                break
            if path.startswith(_STOP_ON):
                # An engine frame no needle names — the mapping is stale, and
                # this row says so rather than hiding it.
                subs["other (engine)"] += 1
                break
        else:
            # Nothing to stop on: `ld.so`, libc, the runtime's start-up, or a
            # stack whose engine frames the unwinder lost.
            subs["unattributed"] += 1
        if len(stack) > 1:
            callers[leaf][stack[1]] += 1
        paths = [normalise(f) for f in stack]
        for name in {b for p in paths if (b := bucket_of(p)) is not None}:
            cum_subs[name] += 1
        for nd in cum_needles:
            if any(nd in p for p in paths):
                cum_needle[nd] += 1

    for line in text.splitlines():
        if not line.strip():
            flush()
            stack = []
            continue
        if not line[0].isspace():
            # A new sample header without a blank line before it.
            flush()
            stack = []
            continue
        m = _FRAME.match(line)
        if m:
            sym = _OFFSET.sub("", m.group(1).strip())
            dso = m.group(2).strip()
            if sym == "[unknown]":
                # Name it by its object so the flat table stays readable: an
                # unsymbolised `libc` frame is a fact, `[unknown]` is not.
                sym = f"[{Path(dso).name}]"
            stack.append(sym)
    flush()
    return {"leaves": leaves, "subs": subs, "cum_subs": cum_subs,
            "cum_needle": cum_needle, "callers": callers}


def time_it(argv: list[str], runs: int) -> tuple[float, float]:
    samples = []
    for _ in range(runs):
        t0 = time.perf_counter()
        subprocess.run(argv, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                       check=True)
        samples.append(time.perf_counter() - t0)
    return min(samples), statistics.median(samples)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--freq", type=int, default=4999, help="perf -F (default 4999)")
    ap.add_argument("--call-graph", default="lbr", metavar="MODE",
                    help="perf --call-graph (default lbr; fp / dwarf,N)")
    ap.add_argument("--repeat", type=int, default=1,
                    help="run the workload N times under one perf record, for "
                         "samples (default 1)")
    ap.add_argument("--top", type=int, default=20, help="symbols to show (default 20)")
    ap.add_argument("--cum-of", action="append", metavar="SUBSTR",
                    help="also report the share of samples whose stack passes "
                         "through a frame matching SUBSTR (repeatable)")
    ap.add_argument("--callers", default=None, metavar="SUBSTR",
                    help="also break the callers of matching symbols down")
    ap.add_argument("--json", type=Path, default=None)
    ap.add_argument("--keep", type=Path, default=None, help="keep perf.data here")
    ap.add_argument("--no-build", action="store_true")
    ap.add_argument("cmd", nargs=argparse.REMAINDER, help="ein arguments")
    args = ap.parse_args()
    if not args.cmd:
        ap.error("give an ein command, e.g. solve examples/zebra2.ein -e")

    if not args.no_build:
        subprocess.run(["cargo", "build", "--profile", "profiling"], cwd=EIN_RS,
                       check=True, stdout=subprocess.DEVNULL)
    prof = EIN_RS / "target" / "profiling" / "ein"
    rel = EIN_RS / "target" / "release" / "ein"
    if not prof.exists():
        print(f"missing {prof}", file=sys.stderr)
        return 1

    env = dict(os.environ)
    env.pop("EIN_STDLIB", None)
    cmd = [str(prof), *args.cmd]

    # The frame-pointer tax, measured rather than assumed.
    if rel.exists():
        r_best, _ = time_it([str(rel), *args.cmd], 3)
        p_best, _ = time_it(cmd, 3)
        print(f"binary  release {r_best * 1e3:.1f} ms   profiling "
              f"{p_best * 1e3:.1f} ms   ({(p_best / r_best - 1) * 100:+.1f} % "
              f"for the line tables — codegen is identical, and this line is "
              f"how that stays checked)", file=sys.stderr)

    tmp = Path(tempfile.mkdtemp(prefix="ein-perf-"))
    data = tmp / "perf.data"
    # `--repeat` under one `perf record`: a shell loop, so process start-up is
    # amortised and the sample count scales with N.
    inner = " ".join([f"'{c}'" for c in cmd])
    script = f"for i in $(seq {args.repeat}); do {inner} >/dev/null; done"
    rec = subprocess.run(
        ["perf", "record", "-q", "-F", str(args.freq),
         "--call-graph", args.call_graph, "-o", str(data), "--", "sh", "-c", script],
        env=env, stderr=subprocess.PIPE, text=True)
    if rec.returncode != 0 or not data.exists():
        print(rec.stderr, file=sys.stderr)
        return 1
    out = subprocess.run(["perf", "script", "-i", str(data), "--no-inline",
                          "-F", "comm,ip,sym,dso"],
                         capture_output=True, text=True, check=True)
    prof = parse_script(out.stdout, tuple(args.cum_of or ()))
    leaves, subs, callers = prof["leaves"], prof["subs"], prof["callers"]
    cum_subs, cum_needle = prof["cum_subs"], prof["cum_needle"]
    total = sum(subs.values()) or 1

    print(f"\n── self time by symbol ({sum(leaves.values())} samples, "
          f"{args.freq} Hz x {args.repeat} run(s)) ──")
    print(f"  {'%':>6s}  symbol")
    for sym, n in leaves.most_common(args.top):
        print(f"  {n / total * 100:>5.1f}%  {display(sym)}")

    print("\n── by subsystem: self (innermost enclosing frame) and cumulative ──")
    print(f"  {'subsystem':<16s}{'self':>7s}{'samples':>9s}{'cum':>8s}")
    for name, _ in [*_SUBSYSTEMS, ("other (engine)", ()),
                    ("unattributed", ())]:
        n = subs.get(name, 0)
        c = cum_subs.get(name, 0)
        if n or c:
            print(f"  {name:<16s}{n / total * 100:>6.1f}%{n:>9d}"
                  f"{c / total * 100:>7.1f}%")

    if cum_needle or args.cum_of:
        print("\n── cumulative share of samples whose stack passes through ──")
        for nd in args.cum_of or []:
            print(f"  {cum_needle.get(nd, 0) / total * 100:>5.1f}%  {nd}")

    if args.callers:
        print(f"\n── callers of symbols matching {args.callers!r} ──")
        for sym, n in leaves.most_common():
            if args.callers not in sym:
                continue
            print(f"  {display(sym)}  ({n / total * 100:.1f}%)")
            for caller, cn in callers[sym].most_common(6):
                print(f"      {cn / n * 100:>5.1f}%  ← {display(caller)}")

    if args.json:
        args.json.write_text(json.dumps({
            "cmd": cmd[1:], "freq": args.freq, "repeat": args.repeat,
            "samples": sum(leaves.values()),
            "symbols": [[display(k), v] for k, v in leaves.most_common(200)],
            "subsystems": dict(subs), "subsystems_cum": dict(cum_subs),
            "cum_of": dict(cum_needle),
            "callers": {display(k): {display(c): n for c, n in v.items()}
                        for k, v in callers.items() if leaves[k] / total > 0.005},
        }, indent=2) + "\n", encoding="utf-8")
        print(f"\nartifact: {args.json}", file=sys.stderr)
    if args.keep:
        shutil.copy(data, args.keep)
        print(f"perf.data: {args.keep}", file=sys.stderr)
    shutil.rmtree(tmp, ignore_errors=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
