#!/usr/bin/env python3
"""From a region to a program — M1e2 S1e2.1a.8, the opposite task.

    region_synth.py --example NAME [--out FILE.ein]      a built-in target, written as a program
    region_synth.py SPEC.json [--out FILE.ein]           a target from a file
    region_synth.py --list                               the built-in targets
    region_synth.py --check FILE.ein…                    the oracle against each file's target
    region_synth.py --solution FILE.ein --out DIR        each line node as a loadable program
    region_synth.py --random N [--seed S] [--max-n K]    N random targets, written and checked

`ein solve --dump-region` computes the region of a program; this writes a
program for a region. A **target** is a status per node of the n-cube over the
candidates `(h A)`, `(h B)`, …: in **R**, **owing**, or **dead**. A spec says it
in intervals:

    {"name": "…", "about": "…", "candidates": "ABCDEFG",
     "R":       [["C", "CDE"], …],    R: the union of these intervals [lo … hi]
     "holes":   [["FA", "FA"], …],    minus these
     "owing":   [["", "AB"], …],      a node outside R in one of these owes; else it is dead
     "implies": [["A", "B"], …]}      optional: (h A) derives (h B)

A node is its letters, `""` for ∅. With `implies`, saturation moves a node to
its closure, so the target is read on the closure: a node is what its
fixpoint is.

**How a target becomes rules.** The dead set and the owing set are two
Boolean functions of the n bits (owing is free on dead nodes, and both are
free on nodes saturation moves). Each is cut into prime implicants
(Quine–McCluskey, then a greedy cover). A dead implicant is a rule asserting
`(false)` whose premises are its literals: `(h X)` for a 1, `(absent (h X))`
for a 0. An owing implicant is an obligation `(open h)` whose positive
literals are its trigger and whose 0-literals are the witnesses that discharge
it: `(absent (and (need Kj ?x) (h ?x)))` with `(need Kj X)` given per 0-literal.
A dead set that is upward-closed has only positive prime implicants, and a
discharged set that is upward-closed only negative owing ones, so **a
monotone target comes out with no site and a target with a hole cannot**:
the static check is tested from the other side.

**The header carries the target.** `;;; region-status:` is one character per
node in mask order (`R`, `o`, `x`), bit i the i-th candidate in text order.
That is what `ein-infer/tests/region.rs` and `--check` compare the oracle
with, node for node. `;;; region-spec:` is the spec it came from.

`--check` runs `ein solve FILE -e --dump-region` and compares every node.
`--solution` writes each node of R's two lines as the program plus that
node's facts: a file that loads and saturates to the line node's state, which
`ein render kb` draws. That is the right answer, written by construction and
not by the engine.

The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`.
"""

from __future__ import annotations

import argparse
import itertools
import json
import os
import random
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
STATUS_CHARS = {"R": "R", "owing": "o", "dead": "x"}


# ── the target ────────────────────────────────────────────────────────────────

def mask_of(word: str, cands: str) -> int:
    m = 0
    for ch in word:
        i = cands.index(ch)
        m |= 1 << i
    return m


def name_of(m: int, cands: str) -> str:
    return "".join(c for i, c in enumerate(cands) if m >> i & 1) or "∅"


def within(m: int, lo: int, hi: int) -> bool:
    return m & lo == lo and m & ~hi == 0


def closure(m: int, implies: list[tuple[int, int]]) -> int:
    while True:
        nxt = m
        for a, b in implies:
            if m & a == a:
                nxt |= b
        if nxt == m:
            return m
        m = nxt


def target(spec: dict) -> tuple[str, list[str], list[tuple[int, int]]]:
    """The candidates, the class of every node, and the implications as masks."""
    cands = spec["candidates"]
    n = len(cands)
    iv = lambda key: [(mask_of(a, cands), mask_of(b, cands)) for a, b in spec.get(key, [])]
    r, holes, owing = iv("R"), iv("holes"), iv("owing")
    implies = [(mask_of(a, cands), mask_of(b, cands)) for a, b in spec.get("implies", [])]
    raw = []
    for m in range(1 << n):
        if any(within(m, lo, hi) for lo, hi in r) and not any(within(m, lo, hi) for lo, hi in holes):
            raw.append("R")
        elif any(within(m, lo, hi) for lo, hi in owing):
            raw.append("owing")
        else:
            raw.append("dead")
    status = [raw[closure(m, implies)] for m in range(1 << n)]
    return cands, status, implies


# ── Quine–McCluskey ───────────────────────────────────────────────────────────

def primes(n: int, on: set[int], dc: set[int]) -> list[tuple[int, int]]:
    """Prime implicants of on ∪ dc, each `(value, care)`: the bits in `care`
    must equal `value`'s."""
    full = (1 << n) - 1
    cur = {(m, full) for m in on | dc}
    out: set[tuple[int, int]] = set()
    while cur:
        used, nxt = set(), set()
        by_care: dict[int, list[tuple[int, int]]] = {}
        for t in cur:
            by_care.setdefault(t[1], []).append(t)
        for care, ts in by_care.items():
            vals = {v for v, _ in ts}
            for v in vals:
                for b in range(n):
                    if care >> b & 1 and not v >> b & 1 and (v | 1 << b) in vals:
                        nxt.add((v, care & ~(1 << b)))
                        used.add((v, care))
                        used.add((v | 1 << b, care))
        out |= cur - used
        cur = nxt
    return sorted(out, key=lambda t: (-bin(t[1] ^ full).count("1"), t[1], t[0]))


def covers(t: tuple[int, int], m: int) -> bool:
    v, care = t
    return m & care == v


def cover(n: int, on: set[int], dc: set[int]) -> list[tuple[int, int]]:
    """A small set of prime implicants covering `on`: essentials, then greedy."""
    ps = primes(n, on, dc)
    left, chosen = set(on), []
    for m in sorted(on):
        hits = [p for p in ps if covers(p, m)]
        if len(hits) == 1 and hits[0] not in chosen:
            chosen.append(hits[0])
    for p in chosen:
        left -= {m for m in left if covers(p, m)}
    while left:
        best = max(ps, key=lambda p: (sum(covers(p, m) for m in left), -bin(p[1]).count("1")))
        chosen.append(best)
        left -= {m for m in left if covers(best, m)}
    return chosen


# ── the program ───────────────────────────────────────────────────────────────

def literals(t: tuple[int, int], cands: str) -> tuple[list[str], list[str]]:
    v, care = t
    pos = [c for i, c in enumerate(cands) if care >> i & 1 and v >> i & 1]
    neg = [c for i, c in enumerate(cands) if care >> i & 1 and not v >> i & 1]
    return pos, neg


def topology(n: int, status: list[str]) -> dict:
    """R's lines, holes and components — the target's own, for the header."""
    r = [s == "R" for s in status]
    rs = [m for m in range(1 << n) if r[m]]
    le = lambda a, b: a & b == a
    mn = [m for m in rs if not any(x != m and le(x, m) for x in rs)]
    mx = [m for m in rs if not any(x != m and le(m, x) for x in rs)]
    holes = [m for m in range(1 << n) if not r[m] and any(le(a, m) for a in rs) and any(le(m, b) for b in rs)]
    seen, comps = set(), []
    for m in rs:
        if m in seen:
            continue
        seen.add(m)
        stack, piece = [m], []
        while stack:
            x = stack.pop()
            piece.append(x)
            for i in range(n):
                y = x ^ 1 << i
                if r[y] and y not in seen:
                    seen.add(y)
                    stack.append(y)
        comps.append(sorted(piece))
    dead = [s == "dead" for s in status]
    mono_dead = all(not dead[m] or dead[m | 1 << i] for m in range(1 << n) for i in range(n))
    # Discharged upward-closed where it is defined: no alive node owes above a
    # node of R (owing is free on a dead node).
    mono_owed = all(not r[m] or status[m | 1 << i] != "owing" for m in range(1 << n) for i in range(n))
    return dict(R=rs, minimal=mn, maximal=mx, holes=holes, components=comps,
                monotone=mono_dead and mono_owed)


class RootDead(Exception):
    """∅ dead with R non-empty: the oracle builds every node from root's
    fixpoint, which already holds that `(false)`, so every node would be dead.
    Which route a node is built by is P1e2.1b's question (W8)."""


def program(spec: dict) -> str:
    cands, status, implies = target(spec)
    n = len(cands)
    if status[0] == "dead" and "R" in status:
        raise RootDead(f"{spec['name']}: ∅ is dead and R is not empty — every node is built from root's "
                       "fixpoint, which would already hold the (false); make ∅ owing (P1e2.1b, W8)")
    moved = {m for m in range(1 << n) if closure(m, implies) != m}
    dead_on = {m for m in range(1 << n) if status[m] == "dead" and m not in moved}
    dead_dc = set(moved)
    owe_on = {m for m in range(1 << n) if status[m] == "owing" and m not in moved}
    owe_dc = set(moved) | {m for m in range(1 << n) if status[m] == "dead"}
    dead_rules = cover(n, dead_on, dead_dc) if dead_on else []
    owe_rules = cover(n, owe_on, owe_dc) if owe_on else []
    topo = topology(n, status)
    nm = lambda m: "{" + ",".join(name_of(m, cands)) + "}" if m else "∅"
    lst = lambda ms: ", ".join(nm(m) for m in ms) or "none"

    out = [f";;; {spec['name']} — a program written for its region, by utils/region_synth.py",
           ";;; (M1e2 S1e2.1a.8). Generated: edit the spec and regenerate, never this file:",
           f";;;   python3 utils/region_synth.py --example {spec['name']} --out <this file>",
           ";;;"]
    for line in spec.get("about", "").split("\n"):
        out.append(f";;; {line}".rstrip())
    out += [";;;",
            f";;; The cube is over {n} candidates (h {' '.join(cands)}), {1 << n} nodes. The target:",
            f";;;   R           {len(topo['R'])} nodes in {len(topo['components'])} component(s): "
            + "; ".join(f"{len(c)} nodes from {nm(min(c, key=lambda x: (bin(x).count('1'), x)))}" for c in topo["components"]),
            f";;;   minimal     {lst(topo['minimal'])}",
            f";;;   maximal     {lst(topo['maximal'])}",
            f";;;   holes       {lst(topo['holes'])}",
            f";;;   owing       {sum(s == 'owing' for s in status)} nodes; dead {sum(s == 'dead' for s in status)}",
            f";;;   monotone    {'yes' if topo['monotone'] else 'no'} — "
            + ("the static check should find no site" if topo['monotone']
               else "a dead node revived or a debt added by growth needs an `absent` or an obligation trigger, so the static check should flag it"),
            ";;;",
            f";;; {len(dead_rules)} dead rule(s), {len(owe_rules)} obligation rule(s)"
            + (f", {len(implies)} implication(s)" if implies else "") + ", from the target's prime implicants.",
            ";;;",
            ";;; region-spec: " + json.dumps(spec, ensure_ascii=False, separators=(",", ":")),
            ";;; region-status: " + "".join(STATUS_CHARS[s] for s in status),
            "",
            "(relation h T)",
            "(relation slot T)",
            "(relation world T)",
            "(relation need T T)",
            "",
            "(world W)",
            "(" + ") (".join(f"slot {c}" for c in cands) + ")",
            ""]
    needs: list[str] = []
    rules: list[str] = []
    for a, b in spec.get("implies", []):
        prem = " ".join(f"(h {c})" for c in a)
        rules += [f"(rule imp-{a}-{b} ()",
                  f"  :match  {prem if len(a) == 1 else '(and ' + prem + ')'}",
                  f"  :assert {'(h ' + b + ')' if len(b) == 1 else '(and ' + ' '.join(f'(h {c})' for c in b) + ')'}",
                  f"  :why    \"{a} brings {b}\"",
                  "  :priority 200)", ""]
    for i, t in enumerate(dead_rules, 1):
        pos, neg = literals(t, cands)
        prem = [f"(h {c})" for c in pos] + [f"(absent (h {c}))" for c in neg]
        if not pos:
            prem = ["(world ?w)"] + prem
        what = " ".join(pos) or "∅"
        why = f"{what} is dead" + (f" while {' and '.join(neg)} {'is' if len(neg) == 1 else 'are'} missing" if neg else "")
        rules += [f"(rule dead-{i} ()",
                  f"  :match  {prem[0] if len(prem) == 1 else '(and ' + ' '.join(prem) + ')'}",
                  "  :assert (false)",
                  f"  :why    \"{why}\"",
                  "  :priority 300)", ""]
    for j, t in enumerate(owe_rules, 1):
        pos, neg = literals(t, cands)
        key = f"K{j}"
        witnesses = neg or ["Z"]  # no 0-literal: a witness no generator proposes
        needs += [f"(need {key} {c})" for c in witnesses]
        prem = [f"(h {c})" for c in pos] or ["(world ?w)"]
        prem.append(f"(absent (and (need {key} ?x) (h ?x)))")
        what = "with " + " ".join(pos) if pos else "from the start"
        why = (f"{what}: one of {' '.join(neg)} is required" if neg else f"{what}: a debt nothing pays")
        rules += [f"(rule owe-{j} ()",
                  f"  :match  (and {' '.join(prem)})",
                  "  :assert (open h)",
                  f"  :why    \"{why}\"",
                  "  :priority 500)", ""]
    if needs:
        out += [" ".join(needs), ""]
    out += ["(hrule guess () :match (slot ?x) :assert (h ?x) :why \"guess: (h {?x})?\")", ""]
    out += rules
    out += ["(query :goal (h ?x))"]
    return "\n".join(out) + "\n"


# ── checking ──────────────────────────────────────────────────────────────────

def header(path: Path) -> tuple[dict, str]:
    spec = status = None
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith(";;; region-spec: "):
            spec = json.loads(line.split(": ", 1)[1])
        elif line.startswith(";;; region-status: "):
            status = line.split(": ", 1)[1].strip()
    if spec is None or status is None:
        raise SystemExit(f"{path}: no region-spec / region-status header")
    return spec, status


def ein_bin(arg: str | None) -> str:
    return arg or os.environ.get("EIN_BIN") or str(REPO / "ein.rs/target/release/ein")


def oracle(path: Path, bin: str) -> dict:
    with tempfile.TemporaryDirectory(prefix="region-synth-") as d:
        p = subprocess.run([bin, "solve", str(path), "-e", "--dump-region", d], capture_output=True, text=True)
        try:
            return json.loads((Path(d) / "region.json").read_text())
        except FileNotFoundError:
            raise SystemExit(f"{path}: no region.json — {p.stderr.strip()}")


def check(path: Path, bin: str) -> list[str]:
    """Every node's class, the oracle against the header; and the static check
    against the target's monotonicity."""
    spec, want = header(path)
    region = oracle(path, bin)
    cands = region["candidates"]
    expect = [f"(h {c})" for c in spec["candidates"]]
    if cands != expect:
        return [f"candidates {cands} ≠ {expect}"]
    got = "".join({"R": "R", "owing": "o", "dead": "x", "dead+owing": "x"}[x["status"]]
                  for x in region["region"]["nodes"])
    bad = [f"{name_of(m, spec['candidates'])}: want {w}, oracle {g}"
           for m, (w, g) in enumerate(zip(want, got)) if w != g]
    topo = topology(len(spec["candidates"]), [{"R": "R", "o": "owing", "x": "dead"}[c] for c in want])
    mono = topo["monotone"]
    static = region["monotonicity"]["monotone"]
    if mono != static:
        bad.append(f"the target is {'' if mono else 'not '}monotone and the static check says "
                   f"{'monotone' if static else 'not monotone'}")
    return bad


def solution(path: Path, out: Path, bin: str) -> list[Path]:
    """Each node on R's two lines, as the program plus its facts."""
    spec, want = header(path)
    cands = spec["candidates"]
    topo = topology(len(cands), [{"R": "R", "o": "owing", "x": "dead"}[c] for c in want])
    text = path.read_text(encoding="utf-8")
    written = []
    for line in ("minimal", "maximal"):
        for m in topo[line]:
            name = name_of(m, cands).replace("∅", "empty")
            f = out / line / f"{name}.ein"
            f.parent.mkdir(parents=True, exist_ok=True)
            facts = " ".join(f"(h {c})" for i, c in enumerate(cands) if m >> i & 1)
            f.write_text(f";;; {spec['name']}: the {line}-line node {{{','.join(name_of(m, cands))}}} — the program plus its facts.\n"
                         ";;; Loads and saturates to that node's state (utils/region_synth.py --solution).\n\n"
                         + "\n".join(l for l in text.splitlines() if not l.startswith(";;; region-"))
                         + ("\n\n;; the node\n" + facts + "\n" if facts else "\n"), encoding="utf-8")
            written.append(f)
    return written


# ── the built-in targets ──────────────────────────────────────────────────────

EXAMPLES: dict[str, dict] = {
    "three_components": {
        "name": "three_components",
        "about": ("The user's shape of 2026-10-15: R in three separate components — one\n"
                  "monotone (a 2-dimensional sub-cube over C), one with a one-node hole (as\n"
                  "ein-bugs/naf-upward-closure.ein has it), and one with a bigger hole. Each\n"
                  "component is picked by one of C, F, G, which exclude each other; a node\n"
                  "holding none of them owes one."),
        "candidates": "ABCDEFG",
        "R": [["C", "CDE"], ["F", "ABF"], ["G", "ABDG"]],
        "holes": [["AF", "AF"], ["AG", "ABG"]],
        "owing": [["", "ABDE"]],
    },
    "trivial_line": {
        "name": "trivial_line",
        "about": ("Q-M1e2.7 (f): obligations and nothing else. Root owes one of A, B; past that\n"
                  "nothing is refuted, so the maximal line is the whole candidate set, n facts\n"
                  "down — the line the user called trivial — and the minimal line is {A}, {B}."),
        "candidates": "ABCDE",
        "R": [["A", "ABCDE"], ["B", "ABCDE"]],
        "owing": [["", "CDE"]],
    },
    "one_model_per_component": {
        "name": "one_model_per_component",
        "about": ("Q-M1e2.7 (i): a well-posed puzzle's shape. Exactly one of A, B, C, D, any\n"
                  "two exclude each other, none owes: R is four isolated nodes, each its own\n"
                  "component and each on both lines. An Ambiguity of four, component by\n"
                  "component."),
        "candidates": "ABCD",
        "R": [["A", "A"], ["B", "B"], ["C", "C"], ["D", "D"]],
        "owing": [["", ""]],
    },
    "merged_states": {
        "name": "merged_states",
        "about": ("Q-M1e2.7 (e): saturation moves nodes. A brings B and C brings D, so a\n"
                  "commitment and its closure are one state; R is everything holding A or C\n"
                  "and not both, so its minimal line {A}, {C} is two transient nodes whose\n"
                  "saturation lands on {A,B} and {C,D}."),
        "candidates": "ABCDE",
        "implies": [["A", "B"], ["C", "D"]],
        "R": [["A", "ABDE"], ["C", "BCDE"]],
        "owing": [["", "BDE"]],
    },
    "trigger_hole": {
        "name": "trigger_hole",
        "about": ("A hole made by an obligation, not by an absent (lattice/04's mechanism):\n"
                  "B triggers a debt only C pays, so {A,B} owes between {A} and {A,B,C}.\n"
                  "Beside it, a dead node revived — {A,D} dies without E — so the region\n"
                  "has a hole of each kind. Root owes A."),
        "candidates": "ABCDE",
        "R": [["A", "ABCDE"]],
        "holes": [["AB", "ABDE"], ["AD", "ABCD"]],
        "owing": [["", "BCDE"], ["AB", "ABDE"]],
    },
}


def random_spec(rng: random.Random, n: int, i: int) -> dict:
    cands = "ABCDEFGH"[:n]
    word = lambda m: "".join(c for b, c in enumerate(cands) if m >> b & 1)
    def iv():
        lo = rng.randrange(1 << n)
        hi = lo | rng.randrange(1 << n)
        return [word(lo), word(hi)]
    # ∅ is always owing — the one target the oracle's fork route cannot build
    # (`RootDead`). Half the targets have implications, so saturation moves
    # nodes and the target is read on closures.
    spec = {"name": f"random_{i}", "about": f"random target {i}", "candidates": cands,
            "R": [iv() for _ in range(rng.randint(1, 3))],
            "holes": [iv() for _ in range(rng.randint(0, 2))],
            "owing": [["", ""]] + [iv() for _ in range(rng.randint(0, 2))]}
    if n > 1 and rng.random() < 0.5:
        pairs = [(a, b) for a in cands for b in cands if a != b]
        spec["implies"] = [list(p) for p in rng.sample(pairs, rng.randint(1, min(3, len(pairs))))]
    return spec


# ── main ──────────────────────────────────────────────────────────────────────

def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("spec", nargs="*")
    ap.add_argument("--example")
    ap.add_argument("--list", action="store_true")
    ap.add_argument("--out")
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--solution")
    ap.add_argument("--random", type=int)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--max-n", type=int, default=5)
    ap.add_argument("--bin")
    a = ap.parse_args(argv)
    bin = ein_bin(a.bin)
    if a.list:
        for k, v in EXAMPLES.items():
            print(f"{k:28s} n={len(v['candidates'])}  {v['about'].splitlines()[0]}")
        return 0
    if a.check:
        bad_files = 0
        for f in a.spec:
            bad = check(Path(f), bin)
            print(f"{f}: {'ok' if not bad else f'{len(bad)} mismatch(es)'}")
            for b in bad[:20]:
                print(f"  {b}")
            bad_files += bool(bad)
        return 1 if bad_files else 0
    if a.solution:
        for f in solution(Path(a.solution), Path(a.out or "."), bin):
            print(f)
        return 0
    if a.random:
        rng = random.Random(a.seed)
        fails = 0
        with tempfile.TemporaryDirectory(prefix="region-synth-random-") as d:
            for i in range(a.random):
                spec = random_spec(rng, rng.randint(1, a.max_n), i)
                f = Path(d) / f"{spec['name']}.ein"
                f.write_text(program(spec), encoding="utf-8")
                bad = check(f, bin)
                if bad:
                    fails += 1
                    print(f"{spec['name']}: {json.dumps(spec)}")
                    for b in bad[:10]:
                        print(f"  {b}")
        print(f"random: {a.random} targets, seed {a.seed}, {fails} failed")
        return 1 if fails else 0
    if a.example:
        if a.example not in EXAMPLES:
            print(f"no example {a.example!r}; --list", file=sys.stderr)
            return 2
        spec = EXAMPLES[a.example]
    elif a.spec:
        spec = json.loads(Path(a.spec[0]).read_text(encoding="utf-8"))
    else:
        ap.print_help()
        return 2
    try:
        text = program(spec)
    except RootDead as e:
        print(f"error: {e}", file=sys.stderr)
        return 2
    if a.out:
        Path(a.out).parent.mkdir(parents=True, exist_ok=True)
        Path(a.out).write_text(text, encoding="utf-8")
    else:
        sys.stdout.write(text)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
