#!/usr/bin/env python3
"""The solution region of every corpus program the oracle reaches — M1e2
S1e2.1a.3's instrument.

[Q-M1e2.7](../plans/m1e2_correctness/open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)
made the answer to a program a **region** of its hypothesis lattice,
`R = { C : C alive and discharged }`, and the user ruled its sub-questions
for the monotone case on 2026-10-16. A definition ruled from five hand-picked
programs is only as good as the corpus it was not checked against, so this is
that check: every corpus entry that declares a `solve` run, and every file the
wrong-answer register names, through `ein solve FILE --dump-region DIR`.

    utils/region_census.py                    # the tables, to stdout
    utils/region_census.py --json c.json      # + one row per program
    utils/region_census.py -k lattice/07      # one program, every field
    utils/region_census.py --levers           # + R under every config lever
    utils/region_census.py --from c.json      # the tables again, running nothing

**The transport is the oracle's own directory**, `region.json` and `walk.json`
([`region_dump.md`](../docs/kernel/inference/region_dump.md)). Nothing here
saturates or re-derives a node: a census that reconstructs its subject is a
census that can disagree with it. What it adds is arithmetic over the node
array — the count of R in **states** as well as in commitments (k_s and k_c,
ruling (a) × (e)), each state's minimal generators, the program's *shape* as
the cube shows it, and where the walk's `alive₀` is not the oracle's universe.

**One run per program, and it is plain `solve`** — the corpus's declared run,
at default settings. The oracle runs on its own load after the solve and is
config-free in its universe, so which solve precedes it changes only
`walk.json`. `--all` adds the corpus entries that declare **no** `solve`
(`corpus/README.md` § Dropped runs: the run does not ask their question, and
four of them end in the OOM killer), reached through `solve -m 1`, whose walk
stops after layer 1 and whose oracle is the same. A program that does not load
has no region, and the table says which group it is in.

**`--levers` is the config principle as a measurement** (the user's ruling of
2026-10-15: *no config option affects the problem's search lattice*). Every
program the oracle reaches is run again once per lever — the four CLI shadows
the corpus sweeps, the three `EIN_*` search levers, and every `(config …)`
flag flipped in an appended block, [`feature_matrix.py`](feature_matrix.py)'s
cell — and its answer is compared with the default's: the candidates, every
node's status and state class, root's fixpoint, and the facts of every state
in R (`--region-states`, so a lever that moves a model without moving the
shape is still caught). This asks whether a lever changes **R**. Whether it
changes what the *engine reports* is S1e2.1a.6's question, against R.
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
MANIFEST = REPO / "corpus" / "corpus.toml"
REGISTER = REPO / "corpus" / "wrong_answers.toml"
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

# The four levers `corpus.toml` runs both ways, as the CLI spells them.
CLI_LEVERS = (["-L"], ["-K"], ["-y"], ["-o", "score-sum"])

# The environment levers that steer the search and sit outside `SolverConfig`
# (`configuration.md` § 5): the obligations rung's order and its `off`, and the
# tree traversal.
ENV_LEVERS = (("EIN_OBLIGATION_CHOICE", "off"), ("EIN_OBLIGATION_CHOICE", "fail-first"),
              ("EIN_TRAVERSAL", "tree"))

# Every `(config …)` flag, flipped away from its default — the four with a CLI
# shadow too, because a program's own block and a CLI option reach the oracle
# by different routes. The two float weights take an integer (the surface
# lexer has no float literal), and the two inert flags are in on purpose: a
# lever that reaches nothing reaches R least of all, and the census shows it.
CONFIG_LEVERS = {
    "enable-pre-branch-lookahead": "false",
    "enable-lookahead-kill-cache": "false",
    "hypgen-scoring": "most-constrained",
    "hypgen-rel-weight": "2",
    "hypgen-obj-weight": "2",
    "print-alive": "true",
    "warn-derived-naf": "true",
    "candidate-order-seed": "7",
    "lattice-sanity-check": "true",
    "lattice-order": "score-sum",
    "lattice-order-seed": "7",
    "enable-path-nogoods": "false",
    "enable-symmetric-mirror": "false",
    "enable-singleton-writeback": "false",
    "enable-forced-positive": "false",
    "record-alternative-justifications": "false",
    "enable-fail-fast-fork": "false",
}


# ── selection ─────────────────────────────────────────────────────────────


def selection(key: str | None, every: bool) -> list[dict]:
    """Corpus entries that declare `solve` (and under `--all` the rest, solved
    at `-m 1`), then register files that are not one of them. A register
    witness is usually a corpus entry too; the ones under
    `corpus/fuzz_findings/` need not be."""
    entries = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["entry"]
    out: dict[str, dict] = {}
    for e in entries:
        declared = any(r.split()[0] == "solve" for r in e.get("runs", []))
        if declared or every:
            out[e["path"]] = {"path": e["path"], "source": "corpus",
                              "group": e.get("group", ""),
                              "vehicle": [] if declared else ["-m", "1"]}
    reg = tomllib.loads(REGISTER.read_text(encoding="utf-8"))

    def witnesses(node):
        if isinstance(node, dict):
            for k, v in node.items():
                if k == "witness" and isinstance(v, str):
                    yield v
                else:
                    yield from witnesses(v)
        elif isinstance(node, list):
            for v in node:
                yield from witnesses(v)

    for w in witnesses(reg["case"]):
        if w in out:
            out[w]["source"] = "corpus+register"
        else:
            out[w] = {"path": w, "source": "register", "group": "", "vehicle": []}
    rows = sorted(out.values(), key=lambda r: r["path"])
    if key:
        rows = [r for r in rows if key in r["path"]]
    return rows


# ── one run ───────────────────────────────────────────────────────────────


def run_region(args, path: str, extra: list[str], tmp: Path,
               env: dict | None = None) -> tuple[dict | None, dict | None, dict]:
    """`ein solve PATH [extra] --dump-region tmp`; the two files and the run.
    Under `--levers` every node's state is kept too, because a lever that
    leaves R's shape alone can still change what its states hold."""
    argv = [str(args.bin), "solve", path, *extra, "--dump-region", str(tmp / "region"),
            "--json-summary", str(tmp / "summary.json")]
    if args.levers:
        argv.append("--region-states")
    info: dict = {}
    t = time.perf_counter()
    try:
        p = subprocess.run(argv, cwd=REPO, capture_output=True, text=True,
                           timeout=args.timeout, env=env)
        info["exit"] = p.returncode
        if p.returncode not in (0, 1) or not (tmp / "region" / "region.json").exists():
            info["stderr"] = p.stderr.strip().splitlines()[-1:] if p.stderr else []
    except subprocess.TimeoutExpired:
        info["exit"] = None
        info["note"] = f"timeout at {args.timeout:g} s"
    info["wall_s"] = round(time.perf_counter() - t, 3)
    region = walk = None
    if (tmp / "region" / "region.json").exists():
        region = json.loads((tmp / "region" / "region.json").read_text(encoding="utf-8"))
    if (tmp / "region" / "walk.json").exists():
        walk = json.loads((tmp / "region" / "walk.json").read_text(encoding="utf-8"))
    if (tmp / "summary.json").exists():
        info["config"] = json.loads((tmp / "summary.json").read_text(encoding="utf-8")).get("config")
    return region, walk, info


def minimal_generators(masks: list[int]) -> list[int]:
    """The members of a state class with no member one fact smaller.

    Exact for a monotone program, where a class is convex — if g ⊆ C ⊆ s and
    cl(g) = s then cl(C) = s — so a proper subset in the class implies one a
    single fact smaller. On a non-monotone program it reads *locally*
    minimal, and the row says the program is not monotone."""
    members = set(masks)
    out = []
    for m in masks:
        b, smaller = m, False
        while b:
            low = b & -b
            if (m ^ low) in members:
                smaller = True
                break
            b ^= low
        if not smaller:
            out.append(m)
    return out


def analyse(region: dict, walk: dict | None) -> dict:
    """One program's row, from its dump. No saturation, only the node array."""
    row: dict = {"n": region["n"], "limit": region["limit"]}
    mono = region["monotonicity"]
    row["monotone"] = bool(mono["monotone"])
    row["sites"] = len(mono["sites"])
    row["site_rules"] = sorted({s["rule"] for s in mono["sites"]})
    row["obligation_plans"] = (mono.get("plans") or {}).get("obligation", 0)
    if walk:
        u = walk.get("universe") or {}
        proposed, alive0 = set(u.get("proposed") or []), set(u.get("alive0") or [])
        promoted = {p["fact"] if isinstance(p, dict) else p
                    for p in walk.get("promotions") or []}
        row["universe"] = {
            "proposed": len(proposed),
            "alive0": len(alive0),
            "promoted": sorted(promoted),
            "dropped": sorted(proposed - alive0 - promoted),
            "extra": sorted(alive0 - proposed),
        }
        v = walk.get("verdict") or {}
        row["walk"] = {"type": v.get("type"), "k": v.get("k"),
                       "exhausted": v.get("exhausted")}
    if region.get("refused"):
        row["refused"] = True
        return row
    r = region["region"]
    nodes = r["nodes"]
    status = [x["status"] for x in nodes]
    cands = region["candidates"]
    top = sorted(cands)
    in_r = [m for m, s in enumerate(status) if s == "R"]
    classes: dict[int, list[int]] = {}
    for m in in_r:
        classes.setdefault(nodes[m]["class"], []).append(m)
    gens = {c: minimal_generators(ms) for c, ms in classes.items()}
    merged = {c: ms for c, ms in classes.items() if len(ms) > 1}
    dead = sum(1 for s in status if s.startswith("dead"))
    owing_any = sum(1 for s in status if "owing" in s)
    alive_owing = sum(1 for s in status if s == "owing")

    def norm(lines):
        return sorted(tuple(sorted(x)) for x in lines)

    minimal, maximal = r["minimal"], r["maximal"]
    holes = []
    for h in r["minimal_holes"]:
        m = sum(1 << cands.index(f) for f in h)
        x = nodes[m]
        holes.append({
            "node": h,
            "status": x["status"],
            "absent": x["absent_premises"],
            "refuted_by": x["refuted_by"],
            "owed": x["owed"],
            "kind": ("absent" if x["status"].startswith("dead") and x["absent_premises"]
                     else "trigger" if "owing" in x["status"] else "other"),
        })
    sub = r["subcubes"]
    row.update(
        k_c=r["size"],
        k_s=len(classes),
        empty_in_r=status[0] == "R",
        root_dead=bool(r["root_dead"]),
        alive=len(status) - dead,
        alive_owing=alive_owing,
        dead=dead,
        owing_bit=owing_any,
        shape=("both" if dead and owing_any else "restrictions" if dead
               else "obligations" if owing_any else "neither"),
        minimal=len(minimal),
        maximal=len(maximal),
        min_trivial=norm(minimal) == [()],
        max_trivial=norm(maximal) == [tuple(top)],
        lines_meet=bool(minimal) and norm(minimal) == norm(maximal),
        holes=len(r["holes"]),
        minimal_holes=holes,
        violations={k: v["count"] for k, v in r["violations"].items()},
        duality={"minimal": r["duality"]["minimal_holds"],
                 "maximal": r["duality"]["maximal_holds"]},
        components=len(r["components"]),
        point_components=sum(1 for c in r["components"] if c["size"] == 1),
        subcube_dims=None if sub is None else sorted(c["dimension"] for c in sub),
        nogoods=len(r["nogoods"]),
        discharging=len(r["discharging"]),
        local_max=len(r["local_max"]),
        local_min=len(r["local_min"]),
        merged_states=len(merged),
        largest_class=max((len(ms) for ms in classes.values()), default=0),
        several_generators=sum(1 for g in gens.values() if len(g) > 1),
        guess_depth=max((min(bin(g).count("1") for g in gs) for gs in gens.values()),
                        default=None),
        one_max_several_states=len(maximal) == 1 and len(classes) > 1,
        edges={f"{e['from']}->{e['to']}": e["count"] for e in r["edges"]},
    )
    return row


def root_facts(args, path: str, env: dict | None = None) -> tuple:
    """Root's fixpoint, `sat(program)`, as `ein saturate --dump` lists it.

    `region.json`'s `state` is a node's facts *beyond root's*, so a lever that
    changes root — `enable-symmetric-mirror` on a program whose only node is
    ∅ — would move every model and no field of the dump. Read with the same
    `(config …)` the cell carries."""
    p = subprocess.run([str(args.bin), "saturate", path, "--dump"], cwd=REPO,
                       capture_output=True, text=True, timeout=args.timeout, env=env)
    out, on = [], False
    for line in p.stdout.splitlines():
        if "SATURATED KB DUMP" in line:
            on = True
        elif on and line.startswith("  (") and line.endswith(")"):
            fact = line.strip()
            if " :" in fact:                    # `:rule …` provenance, not the fact
                fact = fact[:fact.index(" :")] + ")"
            out.append(fact)
    return tuple(sorted(out))


def signature(region: dict, root: tuple = ()) -> tuple | None:
    """What the answer is, and nothing about how it was explained: the
    coordinates, every node's status and state class, root's fixpoint, and
    the facts of every state in R."""
    if region is None or region.get("refused"):
        return None
    nodes = region["region"]["nodes"]
    return (tuple(region["candidates"]),
            tuple(x["status"] for x in nodes),
            tuple(x["class"] for x in nodes),
            root,
            tuple(tuple(sorted(x.get("state") or [])) for x in nodes if x["status"] == "R"))


def measure(args, entry: dict, root: Path) -> dict:
    row = dict(entry)
    tmp = Path(tempfile.mkdtemp(dir=root))
    try:
        region, walk, info = run_region(args, entry["path"], entry["vehicle"], tmp)
        config = info.pop("config", None)
        row.update(info)
        if region is None:
            return row
        row.update(analyse(region, walk))
        if args.levers and not region.get("refused"):
            base = signature(region, root_facts(args, entry["path"]))
            row["levers"] = levers(args, entry["path"], base, config, root)
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
    return row


def levers(args, path: str, base: tuple, config: dict | None, root: Path) -> dict:
    """R under every lever, against R at default settings: `same`, `moved`,
    or why there is nothing to compare."""
    out: dict[str, str] = {}
    env = dict(os.environ)
    env["EIN_STDLIB"] = str(REPO / "stdlib")      # config cells live outside
    for lever in CLI_LEVERS:
        tmp = Path(tempfile.mkdtemp(dir=root))
        try:
            region, _, info = run_region(args, path, list(lever), tmp)
            # A CLI shadow reaches the solve and the oracle's search-side
            # options; root's fixpoint is the program's, so it is base's.
            out[" ".join(lever)] = compare(base, region, info, base[3])
        finally:
            shutil.rmtree(tmp, ignore_errors=True)
    for name, value in ENV_LEVERS:
        tmp = Path(tempfile.mkdtemp(dir=root))
        try:
            region, _, info = run_region(args, path, [], tmp, {**os.environ, name: value})
            out[f"{name}={value}"] = compare(base, region, info, base[3])
        finally:
            shutil.rmtree(tmp, ignore_errors=True)
    src = (REPO / path).read_text(encoding="utf-8")
    for flag, value in CONFIG_LEVERS.items():
        if config is None:
            out[flag] = "no resolved config"
            continue
        tmp = Path(tempfile.mkdtemp(dir=root))
        try:
            cell = tmp / Path(path).name
            cell.write_text(src + config_block(config, flag, value), encoding="utf-8")
            region, _, info = run_region(args, str(cell), [], tmp / "out", env)
            out[flag] = compare(base, region, info, root_facts(args, str(cell), env))
        finally:
            shutil.rmtree(tmp, ignore_errors=True)
    return out


def config_block(config: dict, flag: str, value: str) -> str:
    """The program's own resolved config with one flag changed — one block,
    because the loader keeps the **last** `(config …)` whole rather than
    merging ([`feature_matrix.py`](feature_matrix.py)'s cell)."""
    pairs = []
    for k, v in config.items():
        k = k.replace("_", "-")
        if k == flag:
            lit = value
        elif v is None:
            continue
        elif isinstance(v, bool):
            lit = "true" if v else "false"
        elif isinstance(v, float):
            if not v.is_integer():
                continue          # no float literal; the default stands
            lit = str(int(v))
        else:
            lit = str(v)
        pairs.append(f"  :{k} {lit}")
    if not any(p.split()[0] == f":{flag}" for p in pairs):
        pairs.append(f"  :{flag} {value}")
    return ("\n;;; utils/region_census.py lever cell: the resolved config, one flag changed\n"
            "(config\n" + "\n".join(pairs) + ")\n")


def compare(base: tuple, region: dict | None, info: dict, root: tuple) -> str:
    if region is None:
        return f"no region (exit {info.get('exit')})"
    sig = signature(region, root)
    if sig is None:
        return "refused"
    if sig == base:
        return "same"
    if sig[0] != base[0]:
        return "moved: candidates"
    what = []
    st = sum(1 for a, b in zip(sig[1], base[1]) if a != b)
    cl = sum(1 for a, b in zip(sig[2], base[2]) if a != b)
    if st:
        what.append(f"{st} statuses")
    if cl:
        what.append(f"{cl} classes")
    if sig[3] != base[3]:
        gone, new = set(base[3]) - set(sig[3]), set(sig[3]) - set(base[3])
        what.append(f"root's fixpoint (−{len(gone)} +{len(new)} facts)")
    if not st and sig[4] != base[4]:
        what.append(f"{sum(1 for a, b in zip(sig[4], base[4]) if a != b)} states in R")
    return "moved: " + ", ".join(what)


# ── tables ────────────────────────────────────────────────────────────────


def section(title: str) -> None:
    print(f"\n## {title}\n")


def table(head: list[str], rows: list[list]) -> None:
    print("| " + " | ".join(head) + " |")
    print("|" + "|".join("---" for _ in head) + "|")
    for r in rows:
        print("| " + " | ".join(str(c) for c in r) + " |")


def report(rows: list[dict]) -> None:
    computed = [r for r in rows if "k_c" in r]
    refused = [r for r in rows if r.get("refused")]
    none = [r for r in rows if "n" not in r]
    mono = [r for r in computed if r["monotone"]]
    flagged = [r for r in computed if not r["monotone"]]

    section("Reach")
    bins = {"≤ 16": 0, "17–19": 0, "20–31": 0, "≥ 32": 0}
    for r in refused:
        n = r["n"]
        bins["17–19" if n <= 19 else "20–31" if n <= 31 else "≥ 32"] += 1
    bins["≤ 16"] = len(computed)
    table(["", "programs"], [
        ["selected", len(rows)],
        ["… from the corpus only / the register only / both",
         " / ".join(str(sum(1 for r in rows if r["source"] == s))
                    for s in ("corpus", "register", "corpus+register"))],
        ["region computed (n ≤ 16)", len(computed)],
        ["… monotone by the static check", len(mono)],
        ["… flagged non-monotone", len(flagged)],
        ["refused by the limit", len(refused)],
        ["no region at all", len(none)],
    ])
    print()
    table(["n", "programs"], [[k, v] for k, v in bins.items()])
    if none:
        print()
        why: dict[str, list[str]] = {}
        for r in none:
            reason = r.get("note") or f"exit {r.get('exit')}, group {r.get('group') or '—'}"
            why.setdefault(reason, []).append(r["path"])
        table(["why there is no region", "programs", "e.g."],
              [[k, len(v), v[0]] for k, v in sorted(why.items(), key=lambda kv: -len(kv[1]))])
    late = [r for r in rows if r.get("vehicle")]
    if late:
        print(f"\n{len(late)} of the programs declare no `solve` and were reached "
              f"through `solve -m 1` (`--all`); {sum(1 for r in late if 'k_c' in r)} of them "
              f"have a region.")

    # A cube with no candidate is one node, root, so R is {∅} or nothing and
    # every count below is trivially true of it. They are most of the corpus
    # (the saturation fixtures and `tests/stdlib/`), so they get a column of
    # their own rather than diluting the programs that have a lattice at all.
    head = "monotone n > 0 / monotone n = 0 / flagged"

    def split(pred) -> str:
        return " / ".join(str(sum(1 for r in g if pred(r))) for g in (
            [r for r in mono if r["n"] > 0], [r for r in mono if r["n"] == 0], flagged))

    section("R — size, lines, shape")
    table(["", head], [
        ["programs", split(lambda r: True)],
        ["R empty", split(lambda r: r["k_c"] == 0)],
        ["… root dead (no alive node)", split(lambda r: r["k_c"] == 0 and r["root_dead"])],
        ["… every alive node owes", split(lambda r: r["k_c"] == 0 and not r["root_dead"])],
        ["R non-empty", split(lambda r: r["k_c"] > 0)],
        ["∅ ∈ R", split(lambda r: r["empty_in_r"])],
        ["|R| = 1", split(lambda r: r["k_c"] == 1)],
        ["k_s = 1", split(lambda r: r["k_s"] == 1)],
        ["k_c ≠ k_s", split(lambda r: r["k_c"] != r["k_s"])],
        ["one maximal node, several states", split(lambda r: r["one_max_several_states"])],
        ["the lines meet (min = max, R non-empty)", split(lambda r: r["lines_meet"])],
        ["minimal line trivial, {∅}", split(lambda r: r["min_trivial"])],
        ["maximal line trivial, {⊤}", split(lambda r: r["max_trivial"])],
    ])
    print()
    shapes = ("neither", "restrictions", "obligations", "both")
    m1 = [r for r in mono if r["n"] > 0]
    table(["shape in the cube (monotone, n > 0)", "programs", "R non-empty", "min = {∅}",
           "max = {⊤}", "flagged"], [
        [s,
         sum(1 for r in m1 if r["shape"] == s),
         sum(1 for r in m1 if r["shape"] == s and r["k_c"] > 0),
         sum(1 for r in m1 if r["shape"] == s and r["min_trivial"]),
         sum(1 for r in m1 if r["shape"] == s and r["max_trivial"]),
         sum(1 for r in flagged if r["shape"] == s)]
        for s in shapes])

    def held(shape: str, key: str) -> str:
        sub = [r for r in mono if r["shape"] == shape and r["k_c"] > 0 and r["n"] > 0]
        return f"{sum(1 for r in sub if r[key])} of {len(sub)}"

    print("\nRuling (f) on the monotone cubes with R non-empty and n > 0: "
          f"restrictions only ⇒ min = {{∅}} holds on {held('restrictions', 'min_trivial')}; "
          f"obligations only ⇒ max = {{⊤}} holds on {held('obligations', 'max_trivial')}.")

    section("Components and sub-cubes (non-empty R, n > 0)")
    ne = [r for r in computed if r["k_c"] > 0 and r["n"] > 0]
    comp: dict[int, int] = {}
    for r in ne:
        comp[r["components"]] = comp.get(r["components"], 0) + 1
    table(["components", "programs", "every component a point"],
          [[c, k, sum(1 for r in ne if r["components"] == c and r["point_components"] == c)]
           for c, k in sorted(comp.items())])
    print()
    dims: dict[int, int] = {}
    unbounded = 0
    for r in ne:
        if r["subcube_dims"] is None:
            unbounded += 1
            continue
        dims[max(r["subcube_dims"])] = dims.get(max(r["subcube_dims"]), 0) + 1
    table(["largest sub-cube dimension", "programs"], [[d, k] for d, k in sorted(dims.items())]
          + ([["not computed (work bound)", unbounded]] if unbounded else []))

    section("Holes, upward closure, the duality")
    holed = [r for r in computed if r["holes"]]
    viol = [r for r in computed if any(r["violations"].values())]
    table(["", head], [
        ["has a hole", split(lambda r: r["holes"] > 0)],
        ["dead not upward-closed", split(lambda r: r["violations"]["dead_not_upward_closed"] > 0)],
        ["discharged not upward-closed",
         split(lambda r: r["violations"]["discharged_not_upward_closed"] > 0)],
        ["duality fails (either side)",
         split(lambda r: not (r["duality"]["minimal"] and r["duality"]["maximal"]))],
        ["… without a hole",
         split(lambda r: r["holes"] == 0 and not (r["duality"]["minimal"] and r["duality"]["maximal"]))],
        ["local test ≠ cube (local max or min)", split(lambda r: r["local_max"] or r["local_min"])],
    ])
    if holed or viol:
        print()
        table(["program", "monotone", "holes", "minimal holes, and why"],
              [[r["path"], r["monotone"], r["holes"],
                "; ".join(f"{{{', '.join(h['node'])}}} {h['kind']}: "
                          + (", ".join(h["absent"]) if h["kind"] == "absent"
                             else "; ".join(h["owed"]) if h["kind"] == "trigger" else h["status"])
                          for h in r["minimal_holes"][:3])
                + (" …" if len(r["minimal_holes"]) > 3 else "")]
               for r in sorted({r["path"]: r for r in holed + viol}.values(),
                               key=lambda r: r["path"])])

    section("Commitments against states (k_c against k_s)")
    diff = sorted((r for r in computed if r["k_c"] != r["k_s"]), key=lambda r: -r["k_c"])
    table(["program", "monotone", "k_c", "k_s", "states merged", "largest class",
           "states with several minimal generators", "guess depth"],
          [[r["path"], r["monotone"], r["k_c"], r["k_s"], r["merged_states"],
            r["largest_class"], r["several_generators"], r["guess_depth"]] for r in diff])

    section("The universe — the oracle's candidates against the walk's alive₀")
    uni = [r for r in rows if "universe" in r]
    moved = [r for r in uni if r["universe"]["dropped"] or r["universe"]["promoted"]
             or r["universe"]["extra"]]
    table(["", "programs"], [
        ["walk written", len(uni)],
        ["alive₀ = the config-free list", len(uni) - len(moved)],
        ["alive₀ differs", len(moved)],
        ["… a candidate promoted into root (forced positive)",
         sum(1 for r in moved if r["universe"]["promoted"])],
        ["… a candidate dropped before the search (lookahead, root negative)",
         sum(1 for r in moved if r["universe"]["dropped"])],
        ["… alive₀ holds a fact the config-free list does not",
         sum(1 for r in moved if r["universe"]["extra"])],
    ])

    section("Route 1's reach — alive nodes and their dead border (N)")
    tot_nodes = sum(2 ** r["n"] for r in mono)
    tot_r1 = sum(r["alive"] + r["nogoods"] for r in mono)
    print(f"Over the {len(mono)} monotone cubes: alive + |N| = {tot_r1} of {tot_nodes} "
          f"nodes ({100 * tot_r1 / max(tot_nodes, 1):.1f} %).\n")
    top = sorted(mono, key=lambda r: -(r["alive"] + r["nogoods"]))[:8]
    table(["program", "n", "alive", "|N|", "of 2ⁿ"],
          [[r["path"], r["n"], r["alive"], r["nogoods"], 2 ** r["n"]] for r in top])

    if any("levers" in r for r in rows):
        section("The config principle — R under every lever")
        names = ([" ".join(l) for l in CLI_LEVERS] + [f"{n}={v}" for n, v in ENV_LEVERS]
                 + list(CONFIG_LEVERS))
        lv = [r for r in rows if "levers" in r]
        out = []
        for name in names:
            cells = [r["levers"].get(name, "") for r in lv]
            same = sum(1 for c in cells if c == "same")
            moved_r = [r["path"] for r in lv if r["levers"].get(name, "").startswith("moved")]
            other = len(cells) - same - len(moved_r)
            out.append([f"`{name}`", same, len(moved_r), other,
                        ", ".join(moved_r[:4]) + (" …" if len(moved_r) > 4 else "")])
        table(["lever", "same R", "R moved", "no region", "where R moved"], out)
        odd = [(r["path"], k, v) for r in lv for k, v in r["levers"].items()
               if v != "same"]
        if odd:
            print()
            table(["program", "lever", "what"], [[p, f"`{k}`", v] for p, k, v in odd])


def print_one(rows: list[dict]) -> None:
    for r in rows:
        print(json.dumps(r, indent=1, ensure_ascii=False))


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", default=EIN, type=Path,
                    help=f"the ein binary (default $EIN_BIN or {EIN})")
    ap.add_argument("--json", type=Path, help="also write the rows as JSON")
    ap.add_argument("-k", "--key", help="only programs whose path contains this; "
                                        "prints every field")
    ap.add_argument("--from", dest="from_json", type=Path, metavar="ROWS.json",
                    help="print the tables from a --json file of an earlier run, "
                         "running nothing")
    ap.add_argument("--all", action="store_true",
                    help="also the corpus entries that declare no solve run (at -m 1)")
    ap.add_argument("--levers", action="store_true",
                    help="also compute R under every lever and compare it")
    ap.add_argument("--timeout", type=float, default=120.0,
                    help="seconds per run (default 120)")
    ap.add_argument("-j", "--jobs", type=int, default=max(1, (os.cpu_count() or 2) // 2),
                    help="programs run at once (default: half the cores)")
    ap.add_argument("-q", "--quiet", action="store_true", help="no progress lines")
    args = ap.parse_args()

    if not args.from_json and not Path(args.bin).exists():
        print(f"no engine at {args.bin} — run ./build.sh, "
              f"or name one with --bin / $EIN_BIN", file=sys.stderr)
        return 2
    t0 = time.perf_counter()
    if args.from_json:
        rows = json.loads(args.from_json.read_text(encoding="utf-8"))
    else:
        entries = selection(args.key, args.all)
        if not entries:
            print("no programs matched", file=sys.stderr)
            return 2
        rows = []
        with tempfile.TemporaryDirectory(prefix="region-census-") as td, \
                ThreadPoolExecutor(max_workers=args.jobs) as pool:
            futs = {pool.submit(measure, args, e, Path(td)): e for e in entries}
            for i, f in enumerate(as_completed(futs), 1):
                rows.append(f.result())
                if not args.quiet:
                    print(f"  [{i}/{len(entries)}] {futs[f]['path']}", file=sys.stderr)
        rows.sort(key=lambda r: r["path"])
    wall = time.perf_counter() - t0

    if args.key:
        print_one(rows)
    else:
        rev = subprocess.run(["git", "rev-parse", "--short", "HEAD"], cwd=REPO,
                             capture_output=True, text=True).stdout.strip()
        if args.from_json:
            print(f"# Region census — re-printed from {args.from_json.name}, {len(rows)} programs")
        else:
            print(f"# Region census — {time.strftime('%Y-%m-%d')} at `{rev}`, "
                  f"{len(rows)} programs, {wall:.0f} s on {args.jobs} jobs")
        report(rows)
    if args.json:
        args.json.write_text(json.dumps(rows, indent=1, ensure_ascii=False) + "\n",
                             encoding="utf-8")
        print(f"\nwrote {args.json}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
