#!/usr/bin/env python3
"""No iteration over a hash map at an observable site (M1a T1a.0.4.3).

Rust's default `HashMap` uses a per-process randomised `RandomState`, so
iterating one makes the *same binary on the same input* produce different
output between runs. That is the one class of nondeterminism a parity harness
cannot report usefully: the diff moves every time you look at it, and the
first suspicion always falls on the other implementation.

`design/12` §2 answers it with `FxHashMap` everywhere — deterministic hashing,
and the right choice here anyway since none of this is adversary-facing input.
But `FxHashMap`'s iteration order, while stable run-to-run, is still an
artefact of hash values and insertion history rather than of the data, where
the observables this port had to reproduce came from *insertion-ordered*
`dict`s and explicit `sorted()`
([design/02](../plans/m1a_rust/design/02_determinism_and_order.md) §2). So the
rule is stronger than "don't use RandomState": **do not iterate a hash map at
all** where the order can reach an output.

The dynamic half of the question is two tests and a fuzzer:
`ein-render/tests/id_order_invariance.rs` runs the corpus twice under
permuted ids, and `utils/fuzz_ein.py` points it at generated input. This grep
finds an order that *could* leak; those find one that *does*.

    python3 utils/check_hashmap_iteration.py            # exit 1 on a finding
    python3 utils/check_hashmap_iteration.py --list     # show the allow-list

The check is a grep, deliberately. `design/02` §9 allows for upgrading it to a
`dylint` rule if the grep proves noisy — but a grep needs no toolchain, runs in
CI in milliseconds, and the escape hatch is one comment:

    // determinism-ok: <reason>
    for (k, v) in map.iter() { … }

An annotation without a reason does not count. The point is not to record that
someone saw the warning; it is to record *why the order cannot be observed*,
which is the thing a later reader needs and the thing that goes stale.
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
CRATES = REPO / "ein.rs" / "crates"

#: Iterating one of these is what the rule is about. `HashSet`/`FxHashSet` too
#: — a set is a map whose values are `()`.
MAP_TYPES = ("HashMap", "FxHashMap", "BTreeMap", "HashSet", "FxHashSet")
#: `BTreeMap`/`BTreeSet` iterate in key order, which is deterministic and
#: content-determined — exactly what the rule wants. They are listed above only
#: so that a binding's *type* can be recognised, never flagged.
ORDERED = ("BTreeMap", "BTreeSet")

ITER = re.compile(r"\b(\w+)\.(iter|iter_mut|keys|values|values_mut|into_iter)\(\)")
ANNOTATION = re.compile(r"//\s*determinism-ok:\s*(\S.*)")
#: `let x: FxHashMap<…>` / `let x = FxHashMap::default()` / a struct field.
BINDING = re.compile(r"\b(\w+)\s*:\s*(?:&\s*)?(" + "|".join(MAP_TYPES) + r")\b")
CONSTRUCT = re.compile(r"\b(\w+)\s*=\s*(" + "|".join(MAP_TYPES) + r")::")


class Finding:
    def __init__(self, path: Path, line_no: int, name: str, line: str) -> None:
        self.path, self.line_no, self.name, self.line = path, line_no, name, line

    def __str__(self) -> str:
        rel = self.path.relative_to(REPO)
        return f"{rel}:{self.line_no}: iterates `{self.name}` — {self.line.strip()}"


def bindings(text: str) -> list[tuple[int, str, str]]:
    """Every `name: Type` / `name = Type::…` in the file, as
    `(line_no, name, type)` in source order.

    Resolution is *nearest preceding binding wins*, which approximates Rust
    scoping closely enough that two functions can each have a local called
    `m` of different types without the second inheriting the first's verdict.

    What it still cannot see is a map reached through a method call on another
    type (`self.cache().iter()`). That is the grep's ceiling and the reason
    `design/02` §9 leaves the door open to a `dylint` rule — but the common
    shape by far is a local or a field declared right there.
    """
    found: list[tuple[int, str, str]] = []
    for i, line in enumerate(text.splitlines(), start=1):
        for match in list(BINDING.finditer(line)) + list(CONSTRUCT.finditer(line)):
            found.append((i, match.group(1), match.group(2)))
    return found


def type_at(binds: list[tuple[int, str, str]], name: str, line_no: int) -> str | None:
    """The type `name` most recently bound to, at or before ``line_no``."""
    best = None
    for i, n, ty in binds:
        if n == name and i <= line_no:
            best = ty
    return best


def scan(path: Path) -> tuple[list[Finding], list[str]]:
    text = path.read_text(encoding="utf-8")
    binds = bindings(text)
    findings: list[Finding] = []
    allowed: list[str] = []
    lines = text.splitlines()
    for i, line in enumerate(lines, start=1):
        annotation = ANNOTATION.search(line)
        if annotation:
            allowed.append(f"{path.relative_to(REPO)}:{i}: {annotation.group(1)}")
            continue
        for match in ITER.finditer(line):
            ty = type_at(binds, match.group(1), i)
            if ty is None or ty in ORDERED:
                continue
            # An annotation on the preceding line covers this one.
            prev = lines[i - 2] if i >= 2 else ""
            if ANNOTATION.search(prev):
                continue
            findings.append(Finding(path, i, match.group(1), line))
    return findings, allowed


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--list", action="store_true",
                    help="print the allow-list instead of checking")
    args = ap.parse_args()

    if not CRATES.is_dir():
        print(f"no {CRATES.relative_to(REPO)} — nothing to check")
        return 0

    findings: list[Finding] = []
    allowed: list[str] = []
    n_files = 0
    for path in sorted(CRATES.rglob("*.rs")):
        n_files += 1
        f, a = scan(path)
        findings += f
        allowed += a

    if args.list:
        print(f"{len(allowed)} annotated site(s):")
        for a in allowed:
            print(f"  {a}")
        return 0

    if findings:
        print(f"{len(findings)} hash-map iteration(s) at a possibly observable "
              f"site:", file=sys.stderr)
        for f in findings:
            print(f"  {f}", file=sys.stderr)
        print("\nEither sort the keys, use an insertion-ordered structure, or "
              "annotate the line:\n    // determinism-ok: <why the order "
              "cannot be observed>", file=sys.stderr)
        return 1
    print(f"determinism lint ok — {n_files} file(s), {len(allowed)} annotated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
