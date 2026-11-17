#!/usr/bin/env python3
"""gen_stdlib_rule_pages.py — the standard library's rules, as its reference shows them.

    python3 utils/gen_stdlib_rule_pages.py [--check] [--bin ein.rs/target/gate/ein]
    python3 utils/gen_stdlib_rule_pages.py --missing     # the rules no page shows, one a line

The pages under `docs/ein/user/04-reference/04-stdlib/` show a rule of the
standard library as Ein and as the engine's drawing of it. Each showing is a
region of a page, and the text between its two markers is written here and
never by hand:

    <!-- rule std.algebra transitive -->
    ```lisp
    (rule transitive (?rel) …)        the rule as its module writes it
    ```

    ```dot
    digraph rule_transitive_lhs_rhs { … }      what `ein render rules FILE` writes for it
    ```
    <!-- /rule -->

The Ein is cut from the module's file under `stdlib/`, from `(rule NAME` to its
closing parenthesis. The drawing is the digraph `ein render rules FILE` prints
for the rule, byte for byte. A rule whose drawing has more than `MAX_NODES`
nodes is shown as Ein alone: a closing rule names every relation and rule of
its module, and its drawing is a few hundred octagons nobody reads.

What a page says around a region, the heading, the sentence, the order, is the
page's own and is not touched.

`--check` writes nothing and exits 1 where a region differs from what would be
written, where a rule of a module `stdlib/MANIFEST.sha256` lists has no region
or more than one, and where a region names a module or a rule that does not
exist. So a rule added to the library fails the check until a page shows it,
and one edited fails it until this script is run.

The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STDLIB = ROOT / "stdlib"
PAGES = ROOT / "docs/ein/user/04-reference/04-stdlib"

#: A drawing of more nodes than this is not shown.
MAX_NODES = 120

OPEN = re.compile(r"^<!-- rule (std\.[\w.-]+) (\S+) -->$")
CLOSE = "<!-- /rule -->"


def modules() -> dict[str, Path]:
    """`std.x.y` → its file, for every module the manifest lists."""
    out = {}
    for line in (STDLIB / "MANIFEST.sha256").read_text().splitlines():
        if not line.strip():
            continue
        rel = line.split()[-1]
        out["std." + rel[: -len(".ein")].replace("/", ".")] = STDLIB / rel
    return out


def rules_of(text: str) -> dict[str, str]:
    """Every top-level `(rule NAME …)` of a module's text, in its order, as written."""
    out: dict[str, str] = {}
    i, n, depth, start = 0, len(text), 0, None
    while i < n:
        c = text[i]
        if c == ";":
            while i < n and text[i] != "\n":
                i += 1
            continue
        if c == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif c == "(":
            if depth == 0:
                start = i
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0 and start is not None:
                form = text[start : i + 1]
                m = re.match(r"\(rule\s+(\S+)", form)
                if m:
                    out[m.group(1)] = form
        i += 1
    return out


def drawings_of(ein: str, path: Path) -> dict[str, str]:
    """Rule name → the digraph `ein render rules` prints for it."""
    r = subprocess.run([ein, "render", "rules", str(path)], capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"gen_stdlib_rule_pages: `ein render rules {path}` exits {r.returncode}:\n{r.stderr}")
    out = {}
    for part in re.split(r"(?m)^(?=digraph )", r.stdout):
        if not part.strip():
            continue
        m = re.search(r'^  label="([^"]*)"; labelloc=t;$', part, re.M)
        if not m:
            sys.exit(f"gen_stdlib_rule_pages: a digraph of {path} has no title:\n{part[:200]}")
        out[m.group(1)] = part.rstrip("\n")
    return out


def nodes(dot: str) -> int:
    return sum(1 for l in dot.splitlines() if re.match(r'\s+"[^"]*" \[', l))


def region(source: str, dot: str) -> list[str]:
    lines = ["```lisp", *source.splitlines(), "```"]
    if nodes(dot) <= MAX_NODES:
        lines += ["", "```dot", *dot.splitlines(), "```"]
    return lines


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--check", action="store_true", help="write nothing; exit 1 on a difference")
    ap.add_argument("--missing", action="store_true", help="print the rules no page shows")
    ap.add_argument("--bin", default=os.environ.get("EIN_BIN", str(ROOT / "ein.rs/target/release/ein")))
    args = ap.parse_args()

    mods = modules()
    want: dict[tuple[str, str], list[str]] = {}
    for name, path in mods.items():
        rules = rules_of(path.read_text())
        if not rules:
            continue
        drawn = drawings_of(args.bin, path)
        if set(drawn) != set(rules):
            sys.exit(f"gen_stdlib_rule_pages: {name}: the rules read off the file and the digraphs "
                     f"`ein render rules` prints differ: {sorted(set(drawn) ^ set(rules))}")
        for rule, source in rules.items():
            want[(name, rule)] = region(source, drawn[rule])

    seen: dict[tuple[str, str], list[str]] = {}
    problems: list[str] = []
    stale = 0
    for page in sorted(PAGES.glob("*.md")):
        rel = page.relative_to(ROOT)
        lines = page.read_text().splitlines()
        out, i = [], 0
        while i < len(lines):
            m = OPEN.match(lines[i])
            if not m:
                out.append(lines[i])
                i += 1
                continue
            key = (m.group(1), m.group(2))
            try:
                end = lines.index(CLOSE, i)
            except ValueError:
                sys.exit(f"{rel}:{i + 1}: a rule's region is not closed by `{CLOSE}`")
            seen.setdefault(key, []).append(f"{rel}:{i + 1}")
            if key not in want:
                problems.append(f"{rel}:{i + 1}: no rule `{key[1]}` in {key[0]}")
                out += lines[i : end + 1]
            else:
                if lines[i + 1 : end] != want[key]:
                    stale += 1
                    if args.check:
                        problems.append(f"{rel}:{i + 1}: the region of {key[0]}'s `{key[1]}` is not what "
                                        f"the module and the engine write")
                out += [lines[i], *want[key], CLOSE]
            i = end + 1
        if not args.check and not args.missing and out != lines:
            page.write_text("\n".join(out) + "\n")

    missing = [k for k in want if k not in seen]
    if args.missing:
        for mod, rule in missing:
            print(f"{mod} {rule}")
        return 0
    for mod, rule in missing:
        problems.append(f"{mod}: no page shows the rule `{rule}`")
    for key, places in seen.items():
        if len(places) > 1:
            problems.append(f"{key[0]}: the rule `{key[1]}` is shown {len(places)} times: {', '.join(places)}")
    for p in problems:
        print(f"gen_stdlib_rule_pages: {p}", file=sys.stderr)
    if problems:
        return 1
    verb = "are as written" if args.check or not stale else f"written ({stale} changed)"
    print(f"gen_stdlib_rule_pages: {len(want)} rules of {len(mods)} modules, their regions {verb}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
