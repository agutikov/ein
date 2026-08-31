#!/usr/bin/env python3
"""Does `tests/stdlib/` actually kill a defect in the rule it tests? — M1e S1e.3.6's instrument.

`tests/README.md` recorded **50 of 51** from M1c S1c.1.4 to M1e S1e.3.6, with
one named survivor. The number was taken **by hand**: one deliberate defect per
rule *family*, injected into a copy of `stdlib/` and run past `ein test
tests/`. Every one of the five test idioms that README describes was added
*because* a mutant survived without it — so the sweep is not decoration, it is
what built the suite. And it was the only census in the repo with no `utils/`
script, which means the score rots silently as rules are added: a rule with no
negative case lowers it and nothing says so.

**This is exhaustive where the hand sweep was one-per-family**, so the two
numbers are not the same measurement: 217 mutants against 51, and 157 killed
(2026-08-31). The survivors are banked as a *set* in
`tests/mutation_survivors.txt`, and 48 of the 60 are in `slots.ein` — the
module with the most parameterised, direction-sensitive rules and the fewest
programs per rule.

    utils/stdlib_mutants.py                  # the table, to stdout
    utils/stdlib_mutants.py --json m.json    # + the machine copy
    utils/stdlib_mutants.py --check          # exit 1 if the survivor set moved
    utils/stdlib_mutants.py --bless          # re-bank tests/mutation_survivors.txt
    utils/stdlib_mutants.py -k slot-adjacent # one rule's mutants
    utils/stdlib_mutants.py --keep DIR       # leave the mutant trees behind

## What a mutation is

Four families, mechanical, applied to the **s-expressions** of a `(rule …)`
rather than to its text — a regex over `.ein` cannot tell a `neq` in a `:match`
from the word in a `:why` string:

| family | what it does | what it is a defect of |
|---|---|---|
| `drop-neq` | delete one `(neq ?x ?y)` conjunct | a rule that no longer excludes the degenerate case — the `transitive` two-cycle shape |
| `swap-premise` | exchange the two arguments of one binary premise | direction: a converse read as its own inverse, which is the family the recorded survivor is in |
| `drop-absent` | delete one `(absent …)` conjunct | a uniqueness guard that stops being asked, so the rule fires where more than one witness exists |
| `swap-assert` | exchange the two arguments of the `:assert` head | the conclusion pointing the wrong way |

A mutant is **killed** when `ein test tests/` exits non-zero on a stdlib with
that one change, and **survives** when the whole suite still passes. A survivor
is not automatically a hole: it can be a mutation the rule is genuinely
insensitive to (an argument-symmetric premise). What it is always is a
*question*, and the point of the script is that the questions are enumerated
rather than remembered.

## Why it is not the gate

`ein-infer/tests/stdlib_coverage.rs` is the gate and asks a cheaper question —
every rule is *activated* by a program here. This asks whether the activation
would notice a defect, which costs one process per mutant. On this tree that is
about a hundred runs of a 0.04 s suite: a minute, and a nightly's business
rather than a per-commit one.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DEFAULT_BIN = ROOT / "ein.rs/target/release/ein"

# ── A very small s-expression reader ────────────────────────────────
#
# Enough for `.ein`: parens, atoms, `"strings"`, `;` comments. It keeps each
# node's byte span, because a mutation is a *splice* into the original text and
# re-printing the tree would reformat the whole file.


@dataclass
class Node:
    """One s-expression, with the byte span it occupied."""

    head: str | None  # the first atom of a list, or None
    items: list["Node"]
    start: int
    end: int
    atom: str | None = None

    def is_list(self) -> bool:
        return self.atom is None


def read_forms(text: str) -> list[Node]:
    """Every top-level form of `text`, with spans."""
    i, n = 0, len(text)
    out: list[Node] = []

    def skip_trivia(i: int) -> int:
        while i < n:
            if text[i] in " \t\r\n":
                i += 1
            elif text[i] == ";":
                while i < n and text[i] != "\n":
                    i += 1
            else:
                break
        return i

    def read(i: int) -> tuple[Node, int]:
        i = skip_trivia(i)
        start = i
        if text[i] == "(":
            i += 1
            items: list[Node] = []
            while True:
                i = skip_trivia(i)
                if i >= n:
                    raise ValueError(f"unclosed form at {start}")
                if text[i] == ")":
                    i += 1
                    break
                child, i = read(i)
                items.append(child)
            head = items[0].atom if items and items[0].atom else None
            return Node(head, items, start, i), i
        if text[i] == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            i += 1
            return Node(None, [], start, i, atom=text[start:i]), i
        while i < n and text[i] not in " \t\r\n()":
            i += 1
        return Node(None, [], start, i, atom=text[start:i]), i

    while True:
        i = skip_trivia(i)
        if i >= n:
            return out
        form, i = read(i)
        out.append(form)


# ── The mutation vocabulary ─────────────────────────────────────────


@dataclass
class Mutation:
    module: str
    rule: str
    family: str
    detail: str
    #: `(start, end, replacement)` in the module's own text.
    splice: tuple[int, int, str]


def conjuncts(node: Node) -> list[Node]:
    """The premises of a `:match`, flattening `(and …)` and `(or …)`."""
    if node.is_list() and node.head in ("and", "or"):
        out: list[Node] = []
        for c in node.items[1:]:
            out.extend(conjuncts(c))
        return out
    return [node]


def clause(rule: Node, keyword: str) -> Node | None:
    """A rule's `:match` / `:assert` body."""
    items = rule.items
    for i, it in enumerate(items):
        if it.atom == keyword and i + 1 < len(items):
            return items[i + 1]
    return None


def binary_premises(node: Node) -> list[Node]:
    """Every two-argument relational premise, `(not …)` unwrapped."""
    out: list[Node] = []
    for c in conjuncts(node):
        inner = c.items[1] if c.is_list() and c.head == "not" and len(c.items) == 2 else c
        if inner.is_list() and inner.head not in ("and", "or", "not", "absent", "neq", "eq") \
                and len(inner.items) == 3:
            out.append(inner)
    return out


def mutations_of(module: str, text: str) -> list[Mutation]:
    out: list[Mutation] = []
    for form in read_forms(text):
        if not form.is_list() or form.head != "rule":
            continue
        name = form.items[1].atom if len(form.items) > 1 else "?"
        match_ = clause(form, ":match")
        assert_ = clause(form, ":assert")
        if match_ is not None:
            for c in conjuncts(match_):
                if c.is_list() and c.head == "neq":
                    out.append(Mutation(module, name, "drop-neq",
                                        text[c.start:c.end], (c.start, c.end, "")))
                if c.is_list() and c.head == "absent":
                    out.append(Mutation(module, name, "drop-absent",
                                        text[c.start:c.end][:60], (c.start, c.end, "")))
            for p in binary_premises(match_):
                a, b = p.items[1], p.items[2]
                swapped = (
                    text[p.start:a.start]
                    + text[b.start:b.end]
                    + text[a.end:b.start]
                    + text[a.start:a.end]
                    + text[b.end:p.end]
                )
                out.append(Mutation(module, name, "swap-premise",
                                    text[p.start:p.end], (p.start, p.end, swapped)))
        if assert_ is not None:
            for p in binary_premises(assert_):
                a, b = p.items[1], p.items[2]
                swapped = (
                    text[p.start:a.start]
                    + text[b.start:b.end]
                    + text[a.end:b.start]
                    + text[a.start:a.end]
                    + text[b.end:p.end]
                )
                out.append(Mutation(module, name, "swap-assert",
                                    text[p.start:p.end], (p.start, p.end, swapped)))
    return out


# ── The sweep ───────────────────────────────────────────────────────


@dataclass
class Result:
    mutation: Mutation
    killed: bool
    code: int
    line: str = ""


def key(m: Mutation) -> str:
    """A mutation's identity — module, rule, family and the text it changed."""
    return f"{m.module}::{m.rule}::{m.family}::{' '.join(m.detail.split())}"


def run_suite(binary: Path, stdlib: Path, targets: list[str]) -> tuple[int, str]:
    env = dict(os.environ, EIN_STDLIB=str(stdlib))
    p = subprocess.run(
        [str(binary), "test", *targets, "-q"],
        cwd=ROOT, env=env, capture_output=True, text=True,
    )
    last = (p.stdout.strip().splitlines() or [""])[-1]
    return p.returncode, last


#: The banked survivor set — one `module::rule::family::detail` per line.
#:
#: A **set**, not a score, for the reason a floor over a growing corpus is
#: [TE-M2](../plans/m1e_review_processing/p1e.3_medium/s1e.3.6_tests.md): a
#: number decays while the thing it guards grows, and a set says *which*. A new
#: survivor fails `--check`; a banked one that has since been killed also
#: fails, because that is an improvement worth banking rather than losing.
#:
#: `slot-adjacent-bwd-neg`'s exchanged structure premise — `tests/README.md`'s
#: recorded survivor since M1c — is **not** on it: M1e S1e.3.6 T6 added
#: `tests/stdlib/slots/13_adjacent_bwd_neg_direction.ein`, which needs the
#: exclusion the exchanged rule stops deriving, and it killed two siblings on
#: the way.
BASELINE = ROOT / "tests/mutation_survivors.txt"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", type=Path, default=Path(os.environ.get("EIN_BIN", DEFAULT_BIN)))
    ap.add_argument("--json", type=Path)
    ap.add_argument("--check", action="store_true",
                    help="exit 1 if the survivor set is not the banked one")
    ap.add_argument("--bless", action="store_true",
                    help="rewrite tests/mutation_survivors.txt from this run")
    ap.add_argument("-k", "--filter", help="only rules whose name contains this")
    ap.add_argument("--keep", type=Path, help="leave the mutant trees in this directory")
    ap.add_argument("--targets", nargs="*", default=["tests/"],
                    help="what to run (default: tests/)")
    args = ap.parse_args()

    if not args.bin.is_file():
        print(f"no binary at {args.bin} — build with ./build.sh", file=sys.stderr)
        return 2

    src = ROOT / "stdlib"
    modules = {p.name: p.read_text() for p in sorted(src.glob("*.ein"))}
    muts: list[Mutation] = []
    for name, text in modules.items():
        muts.extend(mutations_of(name, text))
    # A splice that reproduces its own source is not a mutant: `(?R ?a ?a)`
    # swapped is `(?R ?a ?a)`, and counting it as *killed* or as *survived*
    # would both be wrong — there is nothing there to detect.
    muts = [m for m in muts if m.splice[2] != modules[m.module][m.splice[0]:m.splice[1]]]
    if args.filter:
        muts = [m for m in muts if args.filter in m.rule]

    # The control: the unmutated tree must pass, or every "killed" below is a
    # statement about the suite being broken rather than about the mutation.
    code, line = run_suite(args.bin, src, args.targets)
    if code != 0:
        print(f"the suite fails on an unmutated stdlib ({code}): {line}", file=sys.stderr)
        return 2

    workdir = Path(args.keep) if args.keep else Path(tempfile.mkdtemp(prefix="ein-mutants-"))
    workdir.mkdir(parents=True, exist_ok=True)
    results: list[Result] = []
    for i, m in enumerate(muts):
        tree = workdir / f"m{i:03d}"
        if tree.exists():
            shutil.rmtree(tree)
        shutil.copytree(src, tree)
        start, end, repl = m.splice
        text = modules[m.module]
        (tree / m.module).write_text(text[:start] + repl + text[end:])
        code, line = run_suite(args.bin, tree, args.targets)
        results.append(Result(m, code != 0, code, line))
        if not args.keep:
            shutil.rmtree(tree)
    if not args.keep:
        shutil.rmtree(workdir, ignore_errors=True)

    killed = sum(1 for r in results if r.killed)
    print(f"stdlib mutants: {killed} of {len(results)} killed by `ein test {' '.join(args.targets)}`\n")
    by_family: dict[str, list[int]] = {}
    for r in results:
        f = by_family.setdefault(r.mutation.family, [0, 0])
        f[0] += int(r.killed)
        f[1] += 1
    print(f"  {'family':<14} {'killed':>7} {'of':>5}")
    for family, (k, n) in sorted(by_family.items()):
        print(f"  {family:<14} {k:>7} {n:>5}")

    survivors = [r for r in results if not r.killed]
    banked = (
        {
            l.strip()
            for l in BASELINE.read_text().splitlines()
            if l.strip() and not l.startswith("#")
        }
        if BASELINE.is_file()
        else set()
    )
    now = {key(r.mutation) for r in survivors}
    if survivors:
        print(f"\n  survivors ({len(survivors)}):")
        for k in sorted(now):
            print(f"    {k}{'' if k in banked else '   <- NEW'}")
    for k in sorted(banked - now):
        print(f"    {k}   <- killed since the baseline")

    if args.json:
        args.json.write_text(json.dumps({
            "killed": killed,
            "total": len(results),
            "by_family": {f: {"killed": k, "total": n} for f, (k, n) in by_family.items()},
            "survivors": [
                {"module": r.mutation.module, "rule": r.mutation.rule,
                 "family": r.mutation.family, "detail": r.mutation.detail,
                 "banked": key(r.mutation) in banked}
                for r in survivors
            ],
        }, indent=2) + "\n")

    if args.bless:
        BASELINE.write_text(
            "# Mutants of `stdlib/` that `ein test tests/` does not kill.\n"
            "# Written by `utils/stdlib_mutants.py --bless`; read by `--check`.\n"
            "#\n"
            "# A set rather than a score: a new line is a rule whose tests stopped\n"
            "# noticing a defect in it, and a removed line is a fixture that started.\n"
            + "".join(f"{k}\n" for k in sorted(now))
        )
        print(f"\nbanked {len(now)} survivor(s) to {BASELINE.relative_to(ROOT)}")

    if args.check:
        new = sorted(now - banked)
        gone = sorted(banked - now)
        for k in new:
            print(f"survivor not in the baseline: {k}", file=sys.stderr)
        for k in gone:
            print(f"banked survivor now killed: {k}", file=sys.stderr)
        if new or gone:
            print(
                "re-bank with `utils/stdlib_mutants.py --bless` once each line "
                "above is understood",
                file=sys.stderr,
            )
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
