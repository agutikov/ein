#!/usr/bin/env python3
"""S1a.6.6 — the differential fuzzer: generate ein programs, diff two engines.

The corpus covers what someone wrote a fixture for. This covers the rest of
the input space — the only mechanism that finds parity bugs in shapes no human
authored ([design/01](../plans/m1a_rust/design/01_parity_contract.md) §7).

    utils/fuzz_ein.py --iters 200                    # one pass, both engines
    utils/fuzz_ein.py --minutes 60 --mode mixed      # a session
    utils/fuzz_ein.py --seed 7 --iters 50 --tier T1  # replay a session
    utils/fuzz_ein.py --replay conformance/fuzz_findings/f-0001.ein

## What runs what

**It does not diff anything itself.** `ein-conformance` is the one
implementation of "what the two engines are not required to agree on"
([`ein-parity`](../ein.rs/crates/ein-parity)), and a fuzzer with its own
private idea of a difference would drift from the gate the day it was written.
So a batch is written out as a **corpus**, and the harness runs it:

    generate → conformance/out/fuzz/cases/*.ein
             → conformance/out/fuzz/corpus.toml     (one entry per case)
             → ein-conformance run --corpus … --tier T3
             → minimise every reported cell, write it to conformance/fuzz_findings/

Each batch carries one **canary** — a corpus fixture both engines are known to
solve — in the `positive` group, so the harness's own liveness check applies:
two engines that both failed to start agree on every generated case too.

## The generator

Grammar-directed against `grammar.lark`, over a small universe (3–5 objects,
2–4 relations), biased toward the shapes that stress the ordering-sensitive
paths this port has actually broken on:

| bias | why |
|---|---|
| several `(or …)` disjuncts binding the same variables | the S1.22.0 collision — two disjuncts, one binding key |
| `(absent …)` guards, nested via `forall` / `open` | the NAF boundary's admission order |
| `(__symmetric__ R)` markers | hazard H1 — the native mirror's insertion order |
| `(not (R …))` facts *and* `(absent (R …))` premises | the two negations that look alike |
| mixed `str` / `int` fact arguments | Q-M1a.4 |
| a `(config …)` lever | the flags the corpus exercises only on `solve` |

Asserted heads never construct a nested term, so the derivable set is finite
and a generated program terminates; the per-case `--max-enterings` /
`--max-set-size 2` budgets bound the *search* on top of that.

## Modes

- `gen` — pure generation.
- `mutate` — take a corpus file and edit it: drop a form, swap two, rename an
  atom, flip a `:priority`, add a `(not …)` fact, toggle a config lever. Finds
  near-miss divergences on programs that are known to be meaningful.
- `mixed` (default) — both, 50/50.

## Findings

A reported cell is **minimised** — forms deleted, conjuncts dropped, kw-pairs
removed, while the divergence survives — and written to
`conformance/fuzz_findings/` with a note naming the run, the tier and the
harness's own diff lines. A 400-line generated program is not a bug report; an
8-line one is. Nothing is added to `conformance/corpus.toml` automatically:
that is the growth rule's step, and it happens in the commit that fixes the
find or records it in the ledger.
"""
from __future__ import annotations

import argparse
import json
import random
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
HARNESS = REPO / "ein.rs" / "target" / "release" / "ein-conformance"
EIN_RS = REPO / "ein.rs" / "target" / "release" / "ein"
WORK = REPO / "conformance" / "out" / "fuzz"
FINDINGS = REPO / "conformance" / "fuzz_findings"
# A cell both engines are known to solve — the liveness check's subject.
CANARY = "examples/saturation/symmetric/friends.ein"

# The runs every generated case is exercised under. Deterministic budgets
# only: `--max-time` would put a machine-dependent reason string into
# `summary.json`, which T0 compares.
RUNS = ["solve --max-set-size 2 --max-enterings 300",
        "solve -e --max-set-size 2 --max-enterings 300",
        "saturate"]

LEVERS = [":enable-pre-branch-lookahead", ":enable-lookahead-kill-cache",
          ":enable-path-nogoods", ":enable-symmetric-mirror",
          ":enable-singleton-writeback", ":enable-forced-positive",
          ":enable-fail-fast-fork"]


# ───────────────────────── the generator ─────────────────────────

class Gen:
    """One program, generated. Every choice is `self.r`'s, so a seed replays."""

    def __init__(self, rng: random.Random) -> None:
        self.r = rng
        self.n_obj = rng.randint(3, 5)
        self.objs = [f"o{i}" for i in range(1, self.n_obj + 1)]
        self.bins = [f"r{i}" for i in range(rng.randint(2, 4))]
        self.unis = [f"u{i}" for i in range(rng.randint(0, 2))]
        # An `(hrule …)` switches generation from the blind enumerator to the
        # rule-driven path, so both get exercised. It also decides whether
        # integer arguments are allowed: a hypothesis whose argument is an int
        # (or a nested fact — see `head`) is **D2**, the ledger's own accepted
        # divergence, and a generator that keeps producing it reports the
        # known answer instead of a new one.
        self.hrule = rng.random() < 0.4
        self.ints = not self.hrule and rng.random() < 0.25   # Q-M1a.4: int args
        self.macros = rng.random() < 0.4         # forall / open
        self.algebra = rng.random() < 0.3        # symmetric / transitive rules
        self.forms: list[str] = []

    # — atoms —

    def obj(self) -> str:
        if self.ints and self.r.random() < 0.3:
            return str(self.r.randint(-2, 9))
        return self.r.choice(self.objs)

    def rel(self) -> str:
        return self.r.choice(self.bins)

    # — premises —

    def pattern(self, vars_: list[str], bind: bool) -> tuple[str, list[str]]:
        """One positive pattern and the variables it binds."""
        pick = self.r.random()
        if self.unis and pick < 0.2:
            v = self.var(vars_, bind)
            return f"({self.r.choice(self.unis)} {v})", [v] if v.startswith("?") else []
        if pick < 0.3:
            v = self.var(vars_, bind)
            return f"(is-a {v} T)", [v] if v.startswith("?") else []
        a, b = self.var(vars_, bind), self.var(vars_, bind)
        rel = self.rel()
        if self.r.random() < 0.12 and self.unis:
            # A nested argument — the shape the participation index keys into.
            return (f"({rel} {a} ({self.r.choice(self.unis)} {b}))",
                    [v for v in (a, b) if v.startswith("?")])
        return f"({rel} {a} {b})", [v for v in (a, b) if v.startswith("?")]

    def var(self, vars_: list[str], bind: bool) -> str:
        """Reuse a bound variable, introduce one, or drop in a constant."""
        if vars_ and self.r.random() < 0.55:
            return self.r.choice(vars_)
        if not bind and self.r.random() < 0.2:
            return self.obj()
        nxt = f"?v{len(vars_)}"
        vars_.append(nxt)
        return nxt

    def conjunct(self, bound: list[str]) -> list[str]:
        """A conjunction: positives first, then the guards they bind."""
        out: list[str] = []
        for _ in range(self.r.randint(1, 3)):
            pat, _ = self.pattern(bound, True)
            out.append(pat)
        roll = self.r.random()
        if roll < 0.3 and len(bound) >= 2:
            a, b = self.r.sample(bound, 2)
            out.append(f"(neq {a} {b})")
        elif roll < 0.55:
            guard, _ = self.pattern(list(bound), False)
            if self.macros and self.r.random() < 0.4 and bound:
                inner, _ = self.pattern(list(bound), False)
                out.append(f"(forall {self.r.choice(bound)} {guard} {inner})")
            elif self.r.random() < 0.3:
                out.append(f"(absent (not {guard}))")
            elif self.macros and self.r.random() < 0.3:
                out.append(f"(open {guard})")
            else:
                out.append(f"(absent {guard})")
        elif roll < 0.7:
            neg, _ = self.pattern(list(bound), False)
            out.append(f"(not {neg})")
        return out

    def body(self) -> tuple[str, list[str]]:
        """`:match`, and the variables it leaves bound."""
        if self.r.random() < 0.3:
            # Disjunction, every disjunct binding the SAME variables — the
            # one-binding-key-two-disjuncts shape (S1.22.0).
            bound = [f"?v{i}" for i in range(self.r.randint(1, 2))]
            anchor = (f"({self.rel()} {bound[0]} {bound[1]})" if len(bound) == 2
                      else f"(is-a {bound[0]} T)")
            arms = []
            for _ in range(self.r.randint(2, 3)):
                # The anchor is what makes the disjuncts share a binding key:
                # without it an arm can bind none of what the head reads, and
                # the rule is a compile error rather than the shape wanted.
                arms.append(" ".join([anchor, *self.conjunct(list(bound))]))
            return "(or " + " ".join(f"(and {a})" for a in arms) + ")", bound
        bound: list[str] = []
        parts = self.conjunct(bound)
        if len(parts) == 1:
            return parts[0], bound
        return "(and " + " ".join(parts) + ")", bound

    def head(self, bound: list[str], positive: bool = False) -> str:
        """`:assert`. `positive` forbids a negative head — see D2 in `__init__`."""
        roll = self.r.random()
        if roll < 0.12 and not positive:
            return "(false)"
        if not bound:
            return f"({self.rel()} {self.obj()} {self.obj()})"
        a = self.r.choice(bound)
        b = self.r.choice(bound) if len(bound) > 1 else self.obj()
        if self.unis and roll < 0.3:
            return f"({self.r.choice(self.unis)} {a})"
        inner = f"({self.rel()} {a} {b})"
        return f"(not {inner})" if roll < 0.45 and not positive else inner

    # — forms —

    def rule(self, i: int, kind: str = "rule") -> str:
        """One rule. The name may not *begin* with a reserved word plus `-`:
        `rule-0` is a parse error in both engines (grammar.lark's SYMBOL
        lookahead is `\\b`-anchored), so the rules are `fire-i` / `hyp-i`."""
        name = f"{'hyp' if kind == 'hrule' else 'fire'}-{i}"
        match, bound = self.body()
        params, activator = "()", None
        if kind == "rule" and self.r.random() < 0.15:
            params, activator = "(?P)", f"({name} T)"
        lines = [f"({kind} {name} {params}",
                 f"  :match  {match}",
                 f"  :assert {self.head(bound, positive=kind == 'hrule')}"]
        if self.r.random() < 0.4:
            lines.append(f'  :why    "{name} fired"')
        if self.r.random() < 0.3:
            lines.append(f"  :priority {self.r.choice([10, 100, 200])}")
        out = "\n".join(lines) + ")"
        return out if activator is None else f"{out}\n{activator}"

    def program(self, tag: str) -> str:
        f = self.forms
        f.append(f";;; {tag} — utils/fuzz_ein.py")
        if self.macros:
            f.append("(import std.macro :symbols (forall open))")
        if self.algebra:
            f.append("(import std.algebra :symbols (symmetric transitive))")
        for rel in self.bins:
            why = f' :why "{rel} {{?1}} {{?2}}"' if self.r.random() < 0.3 else ""
            f.append(f"(relation {rel} T T{why})")
        for uni in self.unis:
            f.append(f"(relation {uni} T)")
        f.append("(relation is-a T T)")
        for o in self.objs:
            f.append(f"(is-a {o} T)")
        for i in range(self.r.randint(2, 6)):
            src = f' :source "({i + 1})"' if self.r.random() < 0.4 else ""
            f.append(f"({self.rel()} {self.obj()} {self.obj()}{src})")
        for _ in range(self.r.randint(0, 2)):
            f.append(f"(not ({self.rel()} {self.obj()} {self.obj()}))")
        for uni in self.unis:
            if self.r.random() < 0.5:
                f.append(f"({uni} {self.r.choice(self.objs)})")
        if self.algebra:
            for rel in self.bins:
                if self.r.random() < 0.4:
                    f.append(f"({self.r.choice(['symmetric', 'transitive'])} {rel})")
        for rel in self.bins:                       # hazard H1
            if self.r.random() < 0.25:
                f.append(f"(__symmetric__ {rel})")
        n_rules = self.r.randint(1, 4)
        for i in range(n_rules):
            f.append(self.rule(i))
        if self.hrule:
            f.append(self.rule(0, "hrule"))
        f.append(f"(query :goal ({self.rel()} ?x ?y))")
        if self.r.random() < 0.5:
            lever = self.r.choice(LEVERS)
            f.append(f"(config {lever} {self.r.choice(['true', 'false'])})")
        return "\n".join(f) + "\n"


# ───────────────────────── the mutator ─────────────────────────

def split_forms(text: str) -> list[str]:
    """Top-level forms, comments attached to the form that follows them."""
    forms, depth, cur, in_str = [], 0, [], False
    i = 0
    while i < len(text):
        c = text[i]
        if in_str:
            cur.append(c)
            if c == "\\" and i + 1 < len(text):
                cur.append(text[i + 1])
                i += 2
                continue
            in_str = c != '"'
        elif c == ";" and depth == 0:
            end = text.find("\n", i)
            end = len(text) if end < 0 else end + 1
            cur.append(text[i:end])
            i = end
            continue
        elif c == ";":
            end = text.find("\n", i)
            end = len(text) if end < 0 else end + 1
            cur.append(text[i:end])
            i = end
            continue
        else:
            cur.append(c)
            if c == '"':
                in_str = True
            elif c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    forms.append("".join(cur).strip())
                    cur = []
        i += 1
    tail = "".join(cur).strip()
    if tail:
        forms.append(tail)
    return forms


def mutate(text: str, rng: random.Random) -> str:
    """One to three edits to a program that is known to mean something."""
    forms = split_forms(text)
    if not forms:
        return text
    for _ in range(rng.randint(1, 3)):
        pick = rng.random()
        if pick < 0.25 and len(forms) > 3:
            forms.pop(rng.randrange(len(forms)))
        elif pick < 0.4 and len(forms) > 3:
            i, j = rng.sample(range(len(forms)), 2)
            forms[i], forms[j] = forms[j], forms[i]
        elif pick < 0.55:
            atoms = sorted(set(re.findall(r"(?<=[( ])([A-Za-z][A-Za-z0-9_*-]*)", "\n".join(forms))))
            if atoms:
                a = rng.choice(atoms)
                forms = [re.sub(rf"(?<=[( ]){re.escape(a)}(?=[ )])", f"{a}x", f)
                         for f in forms]
        elif pick < 0.7:
            i = rng.randrange(len(forms))
            forms[i] = re.sub(r":priority \d+", f":priority {rng.choice([1, 50, 999])}",
                              forms[i])
        elif pick < 0.85:
            rels = sorted(set(re.findall(r"\(relation ([A-Za-z][\w*-]*)", text)))
            if rels:
                forms.append(f"(not ({rng.choice(rels)} m1 m2))")
        else:
            forms.append(f"(config {rng.choice(LEVERS)} "
                         f"{rng.choice(['true', 'false'])})")
    return "\n".join(forms) + "\n"


# Fixtures that exist *because* the two engines differ (the D2 shapes). A
# mutant of one is still a D2 reproducer, so seeding from them means finding
# the ledger's own entry over and over instead of something new.
KNOWN_DIVERGENT = ("examples/ein-bugs/mixed-type-hypothesis.ein",
                   "examples/ein-bugs/nested-fact-hypothesis.ein")


def seed_corpus(limit_bytes: int = 6000) -> list[Path]:
    """The small corpus files worth mutating — big ones are slow, not clever."""
    out = []
    for root in ("examples", "stdlib"):
        for p in sorted((REPO / root).rglob("*.ein")):
            rel = p.relative_to(REPO).as_posix()
            if p.stat().st_size <= limit_bytes and rel not in KNOWN_DIVERGENT:
                out.append(p)
    return out


# ───────────────────────── the runner ─────────────────────────

def write_corpus(cases: list[tuple[str, Path]], path: Path,
                 canary: str = CANARY) -> None:
    """One entry per case, plus the canary that keeps liveness honest."""
    lines = ['schema = "ein-corpus/1"', ""]
    if canary:
        lines += ["[[entry]]", f'path   = "{canary}"', 'group  = "positive"',
                  'runs   = ["solve"]', ""]
    for group, case in cases:
        rel = case.relative_to(REPO).as_posix()
        runs = RUNS if group == "generated" else ["solve"]
        lines += ["[[entry]]", f'path   = "{rel}"', f'group  = "{group}"',
                  "runs   = [" + ", ".join(f'"{r}"' for r in runs) + "]", ""]
    path.write_text("\n".join(lines), encoding="utf-8")


PARSE_ERROR = re.compile(r"unexpected input|expected |unterminated|unexpected end",
                         re.I)


def classify(case: Path) -> tuple[str, bool]:
    """(corpus group, did it load) — one cheap ein.rs probe per case.

    `--max-enterings 0` stops at the first commitment, so this costs a parse,
    a load and a root saturation. Exit 0 or 2 means the program is a program;
    exit 1 is the frontend rejecting it (a `*-negative` group, where what the
    two engines have to agree on is the *message*) or the compiler rejecting a
    rule, which is a program the corpus still wants compared.
    """
    proc = subprocess.run([str(EIN_RS), "solve", str(case), "--max-enterings", "0"],
                          cwd=REPO, capture_output=True, text=True, timeout=60)
    if proc.returncode in (0, 2):
        return "generated", True
    err = proc.stderr
    if PARSE_ERROR.search(err):
        return "parse-negative", False
    if "kb load error" in err or "load error" in err:
        return "load-negative", False
    return "generated", False        # compile / saturate: a program that ran


DIFF_LINE = re.compile(r"^  (\S+\.ein) :: (.+)$", re.M)
TRACEBACK = "Traceback (most recent call last)"


def slug(name: str) -> str:
    """`plan::slug`, in Python — how the harness names a cell's directory."""
    out: list[str] = []
    for c in name:
        if c.isascii() and (c.isalnum() or c in "-."):
            out.append(c)
        elif not out or out[-1] != "_":
            out.append("_")
    return "".join(out).strip("_")


def crashed(out: Path, path: str, run: str) -> str | None:
    """Which side, if either, died with a Python traceback (Q-M1a.14)."""
    for side in ("a", "b"):
        err = out / slug(path) / slug(run) / side / "stderr.txt"
        if err.exists() and TRACEBACK in err.read_text(errors="replace"):
            return side
    return None


def run_harness(corpus: Path, out: Path, impl_a: str, impl_b: str, tier: str,
                jobs: int, timeout: int) -> tuple[int, list[tuple[str, str]], str]:
    """(exit code, [(path, run)…], the report) for one batch."""
    proc = subprocess.run(
        [str(HARNESS), "run", "--corpus", str(corpus), "--repo", str(REPO),
         "--out", str(out), "--impl-a", impl_a, "--impl-b", impl_b,
         "--tier", tier, "--jobs", str(jobs), "--timeout", str(timeout),
         "--env", f"PYTHONPATH={REPO / 'ein.py' / 'src'}"],
        cwd=REPO, capture_output=True, text=True)
    report = proc.stdout + proc.stderr
    cells = DIFF_LINE.findall(proc.stdout)
    return proc.returncode, cells, report


# ───────────────────────── minimisation ─────────────────────────

def still_diverges(text: str, case: Path, ctx: dict,
                   force_group: str | None = None) -> bool:
    """Write `text` to `case` and ask the harness whether **it** still differs.

    The cell has to be the case's own. The batch corpus carries a canary, and
    a canary that is itself diverging (a broken engine, a corpus regression)
    would otherwise answer "yes" for every trial — which minimises any input
    down to the first form that still parses.
    """
    case.write_text(text, encoding="utf-8")
    group = force_group or classify(case)[0]
    corpus = ctx["work"] / "min-corpus.toml"
    write_corpus([(group, case)], corpus, ctx["canary"])
    _code, cells, _ = run_harness(corpus, ctx["work"] / "min-run", ctx["a"], ctx["b"],
                                  ctx["tier"], 2, ctx["timeout"])
    mine = case.resolve().relative_to(REPO).as_posix()
    return any(path == mine for path, _run in cells)


def recheck(case: Path, ctx: dict,
            force_group: str | None = None) -> tuple[str, str | None, bool]:
    """(the harness's report, the side that crashed) for `case` as it stands.

    Run after minimisation, because the batch's report describes the input the
    fuzzer *generated* and the note is about the one it saved.
    """
    group = force_group or classify(case)[0]
    corpus = ctx["work"] / "min-corpus.toml"
    write_corpus([(group, case)], corpus, ctx["canary"])
    out = ctx["work"] / "min-run"
    _code, cells, report = run_harness(corpus, out, ctx["a"], ctx["b"],
                                       ctx["tier"], 2, ctx["timeout"])
    mine = case.resolve().relative_to(REPO).as_posix()
    ours = [(path, run) for path, run in cells if path == mine]
    side = next((crashed(out, path, run) for path, run in ours), None)
    return report, side, bool(ours)


def minimise(text: str, case: Path, ctx: dict) -> str:
    """Delete forms, then conjuncts, then kw-pairs, while the divergence holds."""
    best = text
    changed = True
    while changed:
        changed = False
        forms = split_forms(best)
        for i in range(len(forms) - 1, -1, -1):
            trial = "\n".join(forms[:i] + forms[i + 1:]) + "\n"
            if trial.strip() and still_diverges(trial, case, ctx):
                best, changed = trial, True
                break
        if changed:
            continue
        for pat in (r"\s*:why\s+\"[^\"]*\"", r"\s*:priority \d+", r"\s*:source \"[^\"]*\""):
            trial = re.sub(pat, "", best)
            if trial != best and still_diverges(trial, case, ctx):
                best, changed = trial, True
        for m in list(re.finditer(r"\(and ((?:[^()]|\([^()]*\))+)\)", best)):
            parts = re.findall(r"\([^()]*(?:\([^()]*\))?[^()]*\)", m.group(1))
            if len(parts) < 2:
                continue
            for drop in range(len(parts)):
                kept = parts[:drop] + parts[drop + 1:]
                inner = " ".join(kept)
                repl = inner if len(kept) == 1 else f"(and {inner})"
                trial = best[:m.start()] + repl + best[m.end():]
                if still_diverges(trial, case, ctx):
                    best, changed = trial, True
                    break
            if changed:
                break
    still_diverges(best, case, ctx)      # leave the file at the minimum
    return best


# ───────────────────────── the session ─────────────────────────

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--iters", type=int, default=200, help="cases (default 200)")
    ap.add_argument("--minutes", type=float, default=None,
                    help="run until this many minutes have passed instead")
    ap.add_argument("--batch", type=int, default=25, help="cases per harness run")
    ap.add_argument("--seed", type=int, default=1, help="the stream (replayable)")
    ap.add_argument("--mode", choices=("gen", "mutate", "mixed"), default="mixed")
    ap.add_argument("--tier", default="T3", help="T0…T3 (default T3)")
    ap.add_argument("--jobs", type=int, default=0, help="0 = the harness's default")
    ap.add_argument("--timeout", type=int, default=120, help="per-cell seconds")
    ap.add_argument("--impl-a", default="python3 -m ein.cli")
    ap.add_argument("--impl-b", default=str(EIN_RS))
    ap.add_argument("--replay", type=Path, default=None,
                    help="re-check and minimise one saved case, then exit")
    ap.add_argument("--keep", action="store_true", help="keep the generated cases")
    ap.add_argument("--canary", default=CANARY, metavar="PATH",
                    help="the known-good corpus entry each batch carries, so "
                         "the harness's liveness check applies; empty disables "
                         "it (only a self-test against a deliberately broken "
                         "engine wants that)")
    args = ap.parse_args()

    if not HARNESS.exists():
        sys.exit(f"{HARNESS} — build it: cargo build --release -p ein-conformance")
    cases_dir = WORK / "cases"
    shutil.rmtree(cases_dir, ignore_errors=True)
    cases_dir.mkdir(parents=True, exist_ok=True)
    FINDINGS.mkdir(parents=True, exist_ok=True)
    jobs = args.jobs or max(1, (__import__("os").cpu_count() or 4) - 2)
    ctx = {"work": WORK, "a": args.impl_a, "b": args.impl_b, "tier": args.tier,
           "timeout": args.timeout, "canary": args.canary}

    if args.replay:
        case = cases_dir / args.replay.name
        text = args.replay.read_text(encoding="utf-8")
        if not still_diverges(text, case, ctx):
            print(f"{args.replay}: no divergence at {args.tier} today")
            return 0
        print(f"{args.replay}: still diverges — minimising")
        print(minimise(text, case, ctx))
        return 1

    rng = random.Random(args.seed)
    seeds = seed_corpus()
    started = time.time()
    stats = {"cases": 0, "loaded": 0, "negative": 0, "diffs": 0, "findings": 0,
             "crashes": 0, "crash_parity_ok": 0, "batches": 0}
    candidates: list[str] = []
    findings: list[str] = []
    n = 0
    while True:
        if args.minutes is not None:
            if (time.time() - started) / 60 >= args.minutes:
                break
        elif n >= args.iters:
            break
        batch: list[tuple[str, Path]] = []
        for _ in range(args.batch):
            n += 1
            tag = f"case {n} (seed {args.seed})"
            if args.mode == "gen" or (args.mode == "mixed" and rng.random() < 0.6):
                text = Gen(random.Random(rng.getrandbits(63))).program(tag)
            else:
                src = rng.choice(seeds)
                text = (f";;; {tag} — mutated from {src.relative_to(REPO)}\n"
                        + mutate(src.read_text(encoding="utf-8"),
                                 random.Random(rng.getrandbits(63))))
            case = cases_dir / f"c{n:06d}.ein"
            case.write_text(text, encoding="utf-8")
            group, loaded = classify(case)
            stats["cases"] += 1
            stats["loaded" if loaded else "negative"] += 1
            batch.append((group, case))

        corpus = WORK / "corpus.toml"
        write_corpus(batch, corpus, args.canary)
        code, cells, report = run_harness(corpus, WORK / "run", args.impl_a,
                                          args.impl_b, args.tier, jobs, args.timeout)
        stats["batches"] += 1
        if code == 2:
            print(report)
            sys.exit("harness liveness check failed — the fuzzer proves nothing")
        if args.canary and any(path == args.canary for path, _ in cells):
            print(report)
            sys.exit(f"the canary ({args.canary}) diverged. That is a corpus-level "
                     "parity failure, not a fuzz finding — fix it before "
                     "fuzzing, because every minimisation would follow it.")
        seen: set[str] = set()
        for path, run in cells:
            stats["diffs"] += 1
            if path in seen:
                continue
            seen.add(path)
            src = REPO / path
            text = src.read_text(encoding="utf-8")
            stem = f"{int(started)}-{len(findings) + 1:03d}"
            small = minimise(text, cases_dir / f"x-{stem}.ein", ctx)
            # A Python traceback is Q-M1a.14's category, not a parity result:
            # classify it separately so the two do not drown each other — and
            # classify the *minimum*, which is what the note shows.
            small_case = cases_dir / f"x-{stem}.ein"
            report, side, _ = recheck(small_case, ctx)
            kind = "crash" if side else "diff"
            if side:
                # T1a.6.6.4: an input that makes ein.py raise belongs to the
                # `crash-parity` group, where the comparison is the exit code
                # and the exception class rather than the traceback ein.rs
                # does not have. Judge it there before calling it a find — a
                # case that passes under those rules is a corpus *candidate*,
                # not a divergence, and reporting it as one is how a fuzzer
                # trains its reader to ignore it.
                cp_report, _, cp_diff = recheck(small_case, ctx, "crash-parity")
                if not cp_diff:
                    stats["crash_parity_ok"] += 1
                    candidates.append(small_case.read_text(encoding="utf-8"))
                    print(f"  ~ crash-parity candidate (agrees on class and "
                          f"exit code): {path}", file=sys.stderr)
                    continue
                report, stats["crashes"] = cp_report, stats["crashes"] + 1
            name = f"{kind[0]}-{stem}"
            out = FINDINGS / f"{name}.ein"
            out.write_text(small, encoding="utf-8")
            (FINDINGS / f"{name}.md").write_text(
                f"# {name}\n\n"
                f"- found: {time.strftime('%Y-%m-%d %H:%M:%S')}\n"
                f"- kind: **{kind}**"
                + (f" — implementation {side} died with a Python traceback "
                   f"(Q-M1a.14: a `crash-parity` corpus entry, not a T1 bug)"
                   if side else "") + "\n"
                f"- seed: {args.seed}, mode: {args.mode}, tier: {args.tier}\n"
                f"- run: `{run}`\n"
                f"- minimised: {len(split_forms(text))} → "
                f"{len(split_forms(small))} forms\n"
                f"- from: `{path}`\n\n"
                f"```\n{small}```\n\n## The harness's diff, on the minimum\n\n"
                "```\n" + "\n".join(
                    l for l in report.splitlines()
                    if ".ein ::" in l or l.startswith("      ")) + "\n```\n",
                encoding="utf-8")
            findings.append(name)
            stats["findings"] += 1
            print(f"  ✗ {kind} {name}: {path} :: {run}", file=sys.stderr)
        el = time.time() - started
        print(f"[{el / 60:5.1f} min] {stats['cases']:6d} cases  "
              f"{stats['loaded']} load / {stats['negative']} reject  "
              f"{stats['findings']} findings  "
              f"({stats['cases'] / max(el, 1) * 60:.0f} cases/min)",
              file=sys.stderr, flush=True)

    el = time.time() - started
    rate = stats["loaded"] / max(stats["cases"], 1) * 100
    if candidates:
        (FINDINGS / "crash-parity-candidates.txt").write_text(
            "\n;;; ─────────────\n".join(candidates), encoding="utf-8")
    print(json.dumps({**stats, "seconds": round(el, 1), "load_rate_pct": round(rate, 1),
                      "seed": args.seed, "mode": args.mode, "tier": args.tier,
                      "findings_written": findings}, indent=2))
    if not args.keep:
        shutil.rmtree(cases_dir, ignore_errors=True)
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
