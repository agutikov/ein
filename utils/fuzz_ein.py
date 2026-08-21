#!/usr/bin/env python3
"""The engine fuzzer — generate ein programs, check what one engine can check.

    utils/fuzz_ein.py --iters 200                     # one pass
    utils/fuzz_ein.py --minutes 60 --mode mixed       # a session
    utils/fuzz_ein.py --seed 7 --iters 50             # replay a session
    utils/fuzz_ein.py --replay corpus/fuzz_findings/f-0001.ein

S1a.6.6 built this as a **differential** fuzzer: generate a program, run it on
both engines, and let `ein-conformance` decide whether the two outputs
differed. It found four real parity bugs in its first twenty minutes, on a
surface five phases had signed off. **None of the four was a crash — all four
were wrong answers** — so the arm that found them is the one that cannot be
kept, and this header does not get to claim its predecessor's headline.
It is [accepted loss L1](../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#6-accepted-loss),
the single largest in [P1a.10](../plans/m1a_rust/p1a.10_single_implementation/README.md),
and the rewrite is
[S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md)
T1a.10.4.2.

The generator and the minimiser are untouched. What changed is what a case is
*asked*.

## The properties

Each is named in the finding it produces, so a report says which claim broke
rather than "the fuzzer failed".

| | property | instrument |
|---|---|---|
| `no-crash` | a generated program exits 0, 1 or 2 — never a signal, never a Rust panic | this script, per run |
| `diagnosed` | a refusal says why on stderr | this script, per run |
| `terminates` | every run finishes inside `--timeout`, under budgets that bound the *search* | this script; the timeout **is** the instrument |
| `deterministic` | the same argv twice gives the same exit code and the same bytes | this script, with durations masked |
| `id-order` | the same program under a **permuted interner** answers the same way | `ein-render`'s `id_order_invariance`, pointed at the batch with `EIN_ID_FILES` |

`deterministic` is the **dynamic** counterpart of
`utils/check_hashmap_iteration.py`: the grep finds an iteration whose order
*could* reach an output, and two identical runs find one that *does* — along
with an address or a clock reading that leaked into a rendering. It needs one
normalisation and exactly one: `saturate` prints a phase table, so a `0.05 ms`
is masked before the bytes are compared. That is not a parity cut — it is the
one quantity that is nondeterministic **by construction** — and the fuzzer
deliberately owns no other, because a private idea of "what two outputs are
allowed to differ in" is how a checker drifts away from the gate.

`id-order` is the ledger's property 3 and the strongest of the five: it is the
successor to the `PYTHONHASHSEED` sweep, asked of generated input rather than
of the corpus, and it compares **45 rendering ops** per file rather than what
a CLI prints. It is also the only one that needs `cargo`, so `--no-id-order`
turns it off — explicitly, because a property that skips itself when a tool is
missing is how the workspace ended up reporting 41 passing tests that asserted
nothing ([the ledger §2](../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#2-the-finding--46--of-einrss-own-integration-tests-are-differential)).

## Two properties that are not here, and where they are

- **`dump → parse → dump` is a fixed point.** A *frontend* property, and it
  has an owner with its own generator: `ein-ir/tests/fuzz_properties.rs`,
  which mutates the corpus at the character level and round-trips everything
  that parses. Duplicating it here would need a dumper on the CLI, which was
  removed in P1.11 (`ein ir dump`) and is not coming back for a fuzzer. **The
  division is: that file owns the frontend, this one owns what happens after
  it** — load, compile, saturate, search, render.
- **`--jobs` invariance.** There is no `--jobs` yet;
  [S1a.7.5](../plans/m1a_rust/p1a.7_parallelism/s1a.7.5_jobs_contract.md) is
  where the flag and its contract land, and that is where this row goes.

## What is honestly weaker

All five properties are things a **correct-looking wrong answer satisfies**. A
generated program that loads, terminates, is deterministic and permutation-
invariant can still derive the wrong facts, and nothing here would notice.
That is L1 stated exactly, and its only mitigation is
[P1c.1](../plans/m1c_external_validation/p1c.1_stdlib_conformance/README.md)'s
stated expectations — a stdlib rule whose result is written down.

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
`--max-set-size 2` budgets bound the *search* on top of that. A case that
outlives `--timeout` anyway is the `terminates` finding.

**The D2 exclusion is gone.** The generator used to avoid negative `:assert`
heads and int hypothesis arguments, and the two D2 fixtures were kept out of
the mutation seed set, because those shapes are the divergence ledger's own
accepted entry and four findings in five would have been the answer we already
knew. With one engine there is no divergence to re-find, so the generator's
own restriction is lifted and every corpus file under the size limit is a
mutation seed.

## Modes

- `gen` — pure generation.
- `mutate` — take a corpus file and edit it: drop a form, swap two, rename an
  atom, flip a `:priority`, add a `(not …)` fact, toggle a config lever. Finds
  near-misses on programs that are known to be meaningful.
- `mixed` (default) — both, 60/40.

## Findings

A violation is **minimised** — forms deleted, conjuncts dropped, kw-pairs
removed, while the *same property on the same run* still fails — and then
written to `corpus/fuzz_findings/` with the property, the run, the seed and
the engine's own output. A 400-line generated program is not a bug report; an
8-line one is. Nothing is added to `corpus/corpus.toml` automatically: that is
the growth rule's step, and it happens in the commit that fixes the find or
records it in the ledger, as a `regression` entry with a name.

**One report per distinct cause, across sessions.** A grammar-directed
generator reaches the same shapes over and over: an unfiltered session writes
one answer thirty times. A `no-crash` therefore dedups on the panic's *site
and message* and everything else on the minimised program, and the set is
seeded from the notes already in `fuzz_findings/` — so a recorded find is not
re-filed tomorrow, and `--replay` is how you ask whether it still reproduces.
Measured: the session that found the three below re-runs at **720 cases, 3 600
runs, 0 findings, 67 duplicates suppressed**.

The first sessions after the rewrite found three, all of them questions rather
than fixes and all of them in `fuzz_findings/`: an `(hrule …)` reading `not`
aborts a debug build on a `debug_assert!`; the **unsat core**'s contents move
under a permuted id space; and the goal-binding row the solve table prints
does too — which re-derived, from a different seed, the exact seven forms of a
find that had been filed as a cross-engine divergence in August, and showed it
was never one.

**No manifest is written any more.** The batch used to be handed to the
harness as a throwaway corpus, which is why the group vocabulary had a
`generated` name in it; S1a.10.4 removed the group with the manifest, because
a corpus entry is a file the engine is *permanently* exercised over and these
live for milliseconds.
"""
from __future__ import annotations

import argparse
import json
import os
import random
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))
WORK = REPO / "corpus" / "out" / "fuzz"
CASES = WORK / "cases"
FINDINGS = REPO / "corpus" / "fuzz_findings"

# The runs every generated case is exercised under, as `ein <run>` with the
# file spliced in after the subcommand — `ein_corpus::plan::argv`'s first
# shape, and `render`'s second. Deterministic budgets only: `--max-time` would
# make the answer depend on the machine, and `deterministic` is one of the
# properties.
RUNS = [
    "solve --max-set-size 2 --max-enterings 300",
    "solve -e --max-set-size 2 --max-enterings 300",
    "saturate",
    "render rules",
    "render constraints",
]

# The run name a sweep finding carries, where the other four carry an `ein`
# argv. It is not one: the instrument is `cargo test`, and the finding's note
# says so rather than printing a command that does not exist.
SWEEP = "cargo test -p ein-render --test id_order_invariance (EIN_ID_FILES)"

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
        # rule-driven path, so both get exercised.
        #
        # It used to decide whether integer arguments were allowed as well: a
        # hypothesis whose argument is an int (or a nested fact — see `head`)
        # is **D2**, the ledger's accepted divergence, and while there were two
        # engines a generator that kept producing it reported the known answer
        # instead of a new one. There is one engine, so there is nothing to
        # diverge from and the restriction is lifted (S1a.10.4): int arguments
        # are drawn independently of the hypothesis path.
        self.hrule = rng.random() < 0.4
        self.ints = rng.random() < 0.25                      # Q-M1a.4: int args
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
        """`:assert`. `positive` forbids a negative head.

        Kept as a parameter but no longer passed: it existed to keep `(hrule
        …)` off D2's shape, and with one engine there is no divergence to
        avoid. A caller that wants only positive heads still has it.
        """
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
                 f"  :assert {self.head(bound)}"]
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


def seed_corpus(limit_bytes: int = 6000) -> list[Path]:
    """The small corpus files worth mutating — big ones are slow, not clever.

    Every one of them, since S1a.10.4. Two were excluded — the D2 fixtures
    `examples/ein-bugs/{mixed-type,nested-fact}-hypothesis.ein` — because a
    mutant of one is still a D2 reproducer and seeding from them meant finding
    the ledger's own entry over and over. There is no second engine to diverge
    from, so they are ordinary seeds again.
    """
    out = []
    for root in ("examples", "stdlib"):
        for p in sorted((REPO / root).rglob("*.ein")):
            if p.stat().st_size <= limit_bytes:
                out.append(p)
    return out

# ───────────────────────── the properties ─────────────────────────

# `ein` exits 0 (answered), 1 (refused, with a diagnostic) or 2 (aborted on a
# budget). Anything else — 101 from a Rust panic, or a negative code from a
# signal — is the `no-crash` finding.
OK_CODES = (0, 1, 2)
PANIC = re.compile(r"panicked at|RUST_BACKTRACE", re.I)

# `saturate` and `--timing` print a phase table. A duration is the one thing in
# an engine's output that is nondeterministic by construction, so it is masked
# before two runs are compared byte for byte — and it is the *only* thing this
# script masks.
DURATION = re.compile(r"\d+\.\d+ ms")

# `panicked at crates/ein-infer/src/hrule.rs:113:13:` and the line after it.
# Two panics at the same site with the same message are one finding, however
# many programs reach it — see `report`'s dedup.
PANIC_SITE = re.compile(r"panicked at ([^\n]+)\n([^\n]*)")


def masked(text: str) -> str:
    return DURATION.sub("<ms>", text)

PROPERTIES = {
    "no-crash": "exits 0, 1 or 2 — no panic, no signal",
    "diagnosed": "a refusal says why on stderr",
    "terminates": "finishes inside the timeout",
    "deterministic": "the same argv twice gives the same exit code and bytes",
    "id-order": "the same answer under a permuted interner",
}


def argv_for(run: str, case: Path) -> list[str]:
    """`ein <run>` with the file spliced in — `plan::argv`'s two shapes."""
    toks = run.split()
    if toks[0] == "render":
        return [str(EIN), toks[0], toks[1], str(case), *toks[2:]]
    return [str(EIN), toks[0], str(case), *toks[1:]]


def run_bounded(argv: list[str], timeout: float) -> tuple[int, str, str]:
    """(exit code, stdout, stderr). Exit `-2` for a run the timeout killed.

    `-2` is what `ein-cli/tests/corpus_cli.rs` records for the same thing, so
    a fuzz finding and a corpus cell name a non-termination the same way.
    """
    try:
        proc = subprocess.run(argv, cwd=REPO, capture_output=True, text=True,
                              errors="replace", timeout=timeout)
    except subprocess.TimeoutExpired as e:
        out = e.stdout or b""
        err = e.stderr or b""
        decode = lambda b: b.decode("utf-8", "replace") if isinstance(b, bytes) else b
        return -2, decode(out), decode(err)
    return proc.returncode, proc.stdout, proc.stderr


def check_run(case: Path, run: str, timeout: float) -> tuple[str, str] | None:
    """The four per-process properties, for one run. `(property, detail)` or None."""
    code, out, err = run_bounded(argv_for(run, case), timeout)
    if code == -2:
        return ("terminates", f"still running after {timeout:g}s")
    if code not in OK_CODES or PANIC.search(err):
        return ("no-crash", f"exit {code}\n{err.strip()[-1200:]}")
    if code == 1 and not err.strip():
        return ("diagnosed", "exit 1 with nothing on stderr")
    code2, out2, err2 = run_bounded(argv_for(run, case), timeout)
    if (code2, masked(out2), masked(err2)) != (code, masked(out), masked(err)):
        return ("deterministic",
                f"run 1: exit {code}\n{first_difference(masked(out), masked(out2))}\n"
                f"stderr: {first_difference(masked(err), masked(err2))}")
    return None


def first_difference(a: str, b: str) -> str:
    """The first line the two runs disagree on — a diff nobody has to read."""
    for i, (x, y) in enumerate(zip(a.splitlines(), b.splitlines())):
        if x != y:
            return f"  line {i + 1}\n    run 1: {x}\n    run 2: {y}"
    if a == b:
        return "  (identical)"
    return (f"  same {min(len(a.splitlines()), len(b.splitlines()))} lines, "
            f"then {len(a.splitlines())} vs {len(b.splitlines())}")


def check_case(case: Path, timeout: float) -> list[tuple[str, str, str]]:
    """Every run of one case: `(property, run, detail)` for each violation."""
    out = []
    for run in RUNS:
        hit = check_run(case, run, timeout)
        if hit:
            out.append((hit[0], run, hit[1]))
    return out


def id_order(directory: Path, seeds: int) -> tuple[str | None, str]:
    """`(property, report)` — the permuted-interner sweep over `directory`.

    `None` when every file is invariant. Otherwise the property that broke,
    which is **not always `id-order`**: the sweep is a `cargo test`, so it is a
    *debug* build, and a `debug_assert!` the release binary compiles out fires
    here as a panic. That is a `no-crash` finding reached through this
    instrument, and calling it an ordering bug would send its reader looking
    for a permutation that has nothing to do with it.

    This shells out to `cargo test`, which is the point: the sweep is
    `ein-render/tests/id_order_invariance.rs` and a second copy of it here
    would be a second opinion about what an observable is. `EIN_ID_FILES` is
    the seam it grew for this caller.
    """
    env = dict(os.environ, EIN_ID_FILES=str(directory), EIN_ID_SEEDS=str(seeds))
    proc = subprocess.run(
        ["cargo", "test", "--manifest-path", str(REPO / "ein.rs" / "Cargo.toml"),
         "-q", "-p", "ein-render", "--test", "id_order_invariance"],
        cwd=REPO, env=env, capture_output=True, text=True, errors="replace")
    report = proc.stdout + proc.stderr
    if proc.returncode == 0:
        return None, report
    if PANIC.search(report) and "pairs move when the ids do" not in report:
        return "no-crash", report
    return "id-order", report


# ───────────────────────── minimisation ─────────────────────────

def minimise(text: str, case: Path, fails) -> str:
    """Delete forms, then kw-pairs, then conjuncts, while `fails(text)` holds.

    Unchanged from the differential version except for the predicate, which
    was "the harness reports this cell" and is now "the same property on the
    same run still fails". The caller writes the file; `fails` reads it.
    """
    best = text
    changed = True
    while changed:
        changed = False
        forms = split_forms(best)
        for i in range(len(forms) - 1, -1, -1):
            trial = "\n".join(forms[:i] + forms[i + 1:]) + "\n"
            if trial.strip() and fails(trial):
                best, changed = trial, True
                break
        if changed:
            continue
        for pat in (r"\s*:why\s+\"[^\"]*\"", r"\s*:priority \d+", r"\s*:source \"[^\"]*\""):
            trial = re.sub(pat, "", best)
            if trial != best and fails(trial):
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
                if fails(trial):
                    best, changed = trial, True
                    break
            if changed:
                break
    # Leave the *world* at the minimum, not just the file: the last `fails`
    # call was a rejected trial, so for the sweep predicate the scratch
    # directory it reads still holds that trial. Re-running the predicate on
    # `best` is what makes the re-judge below describe the program that got
    # saved. (Reported as "test result: ok" beside a finding, once.)
    fails(best)
    case.write_text(best, encoding="utf-8")
    return best


def process_predicate(case: Path, prop: str, run: str, timeout: float):
    """`fails(text)` for one of the four per-process properties."""
    def fails(text: str) -> bool:
        case.write_text(text, encoding="utf-8")
        hit = check_run(case, run, timeout)
        return bool(hit) and hit[0] == prop
    return fails


def id_order_predicate(case: Path, work: Path, seeds: int, prop: str):
    """`fails(text)` for a sweep finding: one file, its own directory, one sweep."""
    solo = work / "min-id"
    def fails(text: str) -> bool:
        shutil.rmtree(solo, ignore_errors=True)
        solo.mkdir(parents=True, exist_ok=True)
        case.write_text(text, encoding="utf-8")
        (solo / case.name).write_text(text, encoding="utf-8")
        return id_order(solo, seeds)[0] == prop
    return fails


def write_finding(name: str, prop: str, run: str, small: str, detail: str,
                  origin: str, seed: int, mode: str, forms_before: int) -> Path:
    """The `.ein` and the `.md` beside it."""
    FINDINGS.mkdir(parents=True, exist_ok=True)
    out = FINDINGS / f"{name}.ein"
    out.write_text(small, encoding="utf-8")
    (FINDINGS / f"{name}.md").write_text(
        f"# {name}\n\n"
        f"- found: {time.strftime('%Y-%m-%d %H:%M:%S')}\n"
        f"- property: **{prop}** — {PROPERTIES[prop]}\n"
        + (f"- reached by: `{run}`\n" if run == SWEEP
           else f"- run: `ein {run} {name}.ein`\n")
        + f"- seed: {seed}, mode: {mode}\n"
        f"- minimised: {forms_before} → {len(split_forms(small))} forms\n"
        f"- from: `{origin}`\n\n"
        f"```\n{small}```\n\n## What the engine did\n\n"
        f"```\n{detail.strip()[-4000:]}\n```\n",
        encoding="utf-8")
    return out


# ───────────────────────── the session ─────────────────────────

def known_findings() -> set[tuple[str, str]]:
    """The dedup keys of the findings already in `fuzz_findings/`."""
    out: set[tuple[str, str]] = set()
    for note in sorted(FINDINGS.glob("*.md")):
        text = note.read_text(encoding="utf-8", errors="replace")
        prop = re.search(r"^- property: \*\*([a-z-]+)\*\*", text, re.M)
        if not prop:
            continue
        site = PANIC_SITE.search(text) if prop.group(1) == "no-crash" else None
        body = re.search(r"```\n(.*?)```", text, re.S)
        out.add((prop.group(1),
                 site.group(0) if site else " ".join((body.group(1) if body else "").split())))
    return out


def generate(n: int, args, rng: random.Random, seeds: list[Path]) -> tuple[str, str]:
    """One case's text, and where it came from."""
    tag = f"case {n} (seed {args.seed})"
    if args.mode == "gen" or (args.mode == "mixed" and rng.random() < 0.6):
        return Gen(random.Random(rng.getrandbits(63))).program(tag), "generated"
    src = rng.choice(seeds)
    origin = src.relative_to(REPO).as_posix()
    return (f";;; {tag} — mutated from {origin}\n"
            + mutate(src.read_text(encoding="utf-8"),
                     random.Random(rng.getrandbits(63))), origin)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--iters", type=int, default=200, help="cases (default 200)")
    ap.add_argument("--minutes", type=float, default=None,
                    help="run until this many minutes have passed instead")
    ap.add_argument("--batch", type=int, default=25,
                    help="cases per id-order sweep (default 25)")
    ap.add_argument("--seed", type=int, default=1, help="the stream (replayable)")
    ap.add_argument("--mode", choices=("gen", "mutate", "mixed"), default="mixed")
    ap.add_argument("--timeout", type=float, default=60.0,
                    help="per-run seconds; the `terminates` property's instrument")
    ap.add_argument("--id-seeds", type=int, default=1, metavar="N",
                    help="interner permutations per file (EIN_ID_SEEDS)")
    ap.add_argument("--no-id-order", action="store_true",
                    help="skip the permuted-interner property (it needs cargo)")
    ap.add_argument("--replay", type=Path, default=None,
                    help="re-check and minimise one saved case, then exit")
    ap.add_argument("--keep", action="store_true", help="keep the generated cases")
    args = ap.parse_args()

    if not EIN.exists():
        sys.exit(f"{EIN} does not exist — build it with "
                 f"`cargo build --release -p ein-cli`, or name one with $EIN_BIN")
    want_id_order = not args.no_id_order
    if want_id_order and shutil.which("cargo") is None:
        sys.exit("the `id-order` property needs cargo, which is not on PATH. "
                 "Pass --no-id-order to run the other four on purpose — a "
                 "property that silently skips itself is not a property.")

    shutil.rmtree(CASES, ignore_errors=True)
    CASES.mkdir(parents=True, exist_ok=True)
    rng = random.Random(args.seed)
    seeds = seed_corpus()
    started = time.time()
    stats = {"cases": 0, "runs": 0, "batches": 0, "findings": 0,
             "duplicates": 0, "by_property": {p: 0 for p in PROPERTIES}}
    findings: list[str] = []
    # Seeded from what is already recorded, so a known cause is not re-filed
    # every session. `fuzz_findings/` is small and curated on purpose — a find
    # there is either awaiting a fix or accepted with a note — and `--replay`
    # is how you ask whether one still reproduces.
    seen: set[tuple[str, str]] = known_findings()

    def report(prop: str, run: str, detail: str, case: Path, origin: str,
               text: str) -> None:
        stamp = f"{int(started)}-{len(findings) + 1:03d}"
        work_case = CASES / f"x-{stamp}.ein"
        work_case.write_text(text, encoding="utf-8")
        if run == SWEEP:
            fails = id_order_predicate(work_case, WORK, args.id_seeds, prop)
        else:
            fails = process_predicate(work_case, prop, run, args.timeout)
        small = minimise(text, work_case, fails)
        # One report per distinct *cause*. A grammar-directed generator
        # reaches the same shapes over and over, and three programs that abort
        # at one `debug_assert!` are one finding written three times — so a
        # **`no-crash`** dedups on the panic's site and message, and everything
        # else on the minimised program. The distinction matters both ways: an
        # `id-order` report is a `cargo test` failure and therefore *also*
        # carries a `panicked at`, the sweep's own assertion, which is nearly
        # the same line for every ordering bug there is.
        site = PANIC_SITE.search(detail) if prop == "no-crash" else None
        key = (prop, site.group(0) if site else " ".join(small.split()))
        if key in seen:
            stats["duplicates"] += 1
            return
        seen.add(key)
        # Re-judge the *minimum*: the detail above describes the program the
        # fuzzer generated, and the note is about the one it saved.
        if run == SWEEP:
            fresh = id_order(WORK / "min-id", args.id_seeds)[1]
        else:
            hit = check_run(work_case, run, args.timeout)
            fresh = hit[1] if hit else detail
        name = f"{prop}-{stamp}"
        write_finding(name, prop, run, small, fresh, origin, args.seed,
                      args.mode, len(split_forms(text)))
        findings.append(name)
        stats["findings"] += 1
        stats["by_property"][prop] += 1
        print(f"  ✗ {prop}: {origin} :: {run} → {FINDINGS / (name + '.ein')}",
              file=sys.stderr)

    if args.replay:
        case = CASES / args.replay.name
        text = args.replay.read_text(encoding="utf-8")
        case.write_text(text, encoding="utf-8")
        bad = check_case(case, args.timeout)
        if want_id_order:
            solo = WORK / "replay"
            shutil.rmtree(solo, ignore_errors=True)
            solo.mkdir(parents=True, exist_ok=True)
            (solo / case.name).write_text(text, encoding="utf-8")
            prop, out = id_order(solo, args.id_seeds)
            if prop:
                bad.append((prop, SWEEP, out))
        if not bad:
            print(f"{args.replay}: every property holds today")
            return 0
        # Printed, not written: the saved case is already a finding, and a
        # replay that filed a second copy of it would grow `fuzz_findings/`
        # every time someone checked whether a find still reproduces.
        for prop, run, detail in bad:
            print(f"{args.replay}: **{prop}** fails on `{run}`", file=sys.stderr)
            if run == SWEEP:
                fails = id_order_predicate(case, WORK, args.id_seeds, prop)
            else:
                fails = process_predicate(case, prop, run, args.timeout)
            print(minimise(text, case, fails))
            print(detail.strip()[-1500:], file=sys.stderr)
        return 1

    n = 0
    while True:
        if args.minutes is not None:
            if (time.time() - started) / 60 >= args.minutes:
                break
        elif n >= args.iters:
            break

        batch_dir = WORK / "batch"
        shutil.rmtree(batch_dir, ignore_errors=True)
        batch_dir.mkdir(parents=True, exist_ok=True)
        batch: list[tuple[Path, str, str]] = []
        for _ in range(args.batch):
            n += 1
            text, origin = generate(n, args, rng, seeds)
            case = CASES / f"c{n:06d}.ein"
            case.write_text(text, encoding="utf-8")
            stats["cases"] += 1
            for prop, run, detail in check_case(case, args.timeout):
                report(prop, run, detail, case, origin, text)
            stats["runs"] += len(RUNS)
            batch.append((case, origin, text))
            # Only a program the engine accepts has an id space to permute; a
            # refusal is the same refusal under any interner, and handing the
            # sweep a directory of them would make its own "did anything move"
            # check vacuous.
            code, _, _ = run_bounded([str(EIN), "solve", str(case),
                                      "--max-enterings", "0"], args.timeout)
            if code in (0, 2):
                shutil.copy(case, batch_dir / case.name)

        stats["batches"] += 1
        if want_id_order and any(batch_dir.iterdir()):
            prop, out = id_order(batch_dir, args.id_seeds)
            if prop:
                # Attribute it: re-sweep one file at a time. The batch report
                # does name the file, but a per-file sweep is what the
                # minimiser needs anyway and it costs one pass over ~25 files.
                solo = WORK / "attribute"
                attributed = 0
                for case, origin, text in batch:
                    if not (batch_dir / case.name).exists():
                        continue
                    shutil.rmtree(solo, ignore_errors=True)
                    solo.mkdir(parents=True, exist_ok=True)
                    shutil.copy(case, solo / case.name)
                    prop1, out1 = id_order(solo, args.id_seeds)
                    if prop1:
                        attributed += 1
                        report(prop1, SWEEP, out1, case, origin, text)
                if not attributed:
                    # The sweep is per-file, so this should not happen; if it
                    # does, the batch is the finding and saying so is better
                    # than swallowing it.
                    print("  ! the batch sweep failed but no single file in it "
                          "does — the batch report follows\n" + out[-2000:],
                          file=sys.stderr)
                    stats["findings"] += 1
                    findings.append(f"{prop}-batch-{stats['batches']}")

        el = time.time() - started
        print(f"[{el / 60:5.1f} min] {stats['cases']:6d} cases  "
              f"{stats['runs']} runs  {stats['findings']} findings  "
              f"({stats['cases'] / max(el, 1) * 60:.0f} cases/min)",
              file=sys.stderr, flush=True)

    el = time.time() - started
    print(json.dumps({**stats, "seconds": round(el, 1), "seed": args.seed,
                      "mode": args.mode, "timeout_s": args.timeout,
                      "id_order": want_id_order,
                      "findings_written": findings}, indent=2))
    if not args.keep:
        shutil.rmtree(CASES, ignore_errors=True)
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
