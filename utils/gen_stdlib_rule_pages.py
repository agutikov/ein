#!/usr/bin/env python3
"""gen_stdlib_rule_pages.py — the standard library's rules, as its reference shows them.

    python3 utils/gen_stdlib_rule_pages.py [--check] [--bin ein.rs/target/gate/ein]
    python3 utils/gen_stdlib_rule_pages.py --missing     # the rules no page shows, one a line

The pages under `docs/ein/user/04-reference/04-stdlib/` show a rule of the
standard library as Ein and as the engine's drawing of it. Each showing is a
region of a page, and the text between its two markers is written here and
never by hand, as is the drawing's source, a file of the folder's `diagrams/`:

    <!-- rule std.algebra transitive -->
    ```lisp
    (rule transitive (?rel) …)        the rule as its module writes it
    ```

    ![The rule transitive of std.algebra, as the engine draws it](diagrams/03-std.algebra.transitive.svg)
    <!-- /rule -->

    diagrams/03-std.algebra.transitive.dot      what `ein render rules FILE` writes for the rule

The Ein is cut from the module's file under `stdlib/`, from `(rule NAME` to its
closing parenthesis. The drawing's source is the digraph `ein render rules
FILE` prints for the rule, byte for byte, in the canonical form, every pattern
an octagon whose first arrow is at its relation. A rule whose canonical
drawing has more than `MAX_NODES` nodes is drawn folded, the digraph `ein
render rules --fold FILE` prints, each pattern of three arrows an arrow and
each of two a loop, where that one has no more, and the image's text says so;
one whose folded drawing has more too is shown as Ein alone and has no file. A
closing rule names every relation and rule of its module, each in a pattern of
two arrows, so it is drawn folded where its module is small.

**A source's name** is `NN-MODULE.RULE.dot`: `NN` the number of the page that
shows the rule, `MODULE` the module's dotted name, `RULE` the rule's name as
written. A rule's name holds letters, digits and `-` and no dot, so the name
after `NN-` splits at its last dot into the module and the rule, which is how
`--check` reads a file back. No hand-written diagram of the folder starts
`NN-std.`.

**The pictures** are `render_diagrams.py`'s: a run that writes calls it on the
folder, so every source written here has its `.svg` drawn and stamped. A source
that is no longer wanted is removed with its picture.

What a page says around a region, the heading, the sentence, the order, is the
page's own and is not touched.

`--check` writes nothing and exits 1 where a region or a source differs from
what would be written, where a rule of a module `stdlib/MANIFEST.sha256` lists
has no region or more than one, where a region names a module or a rule that
does not exist, and where the folder holds the source of a rule's drawing that
no region shows: a rule that is gone, renamed, or moved to another page. So a
rule added to the library fails the check until a page shows it, and one edited
fails it until this script is run. That each source has a fresh picture is
`render_diagrams.py --check`'s, a rule of `check_doc_blocks.py`.

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
DIAGRAMS = PAGES / "diagrams"

#: A drawing of more nodes than this is not shown.
MAX_NODES = 120

OPEN = re.compile(r"^<!-- rule (std\.[\w.-]+) (\S+) -->$")
CLOSE = "<!-- /rule -->"
#: What a module's and a rule's name may hold to be part of a file's name.
MODULE = re.compile(r"std(?:\.[a-z0-9]+(?:-[a-z0-9]+)*)+")
RULE = re.compile(r"[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*")
#: A source this script wrote: `NN-MODULE.RULE.dot`.
SOURCE = re.compile(r"\d\d-std\..*\.dot")


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


def drawings_of(ein: str, path: Path, fold: bool = False) -> dict[str, str]:
    """Rule name → the digraph `ein render rules` prints for it, `--fold` where `fold`."""
    argv = [ein, "render", "rules", *(["--fold"] if fold else []), str(path)]
    r = subprocess.run(argv, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"gen_stdlib_rule_pages: `{' '.join(argv[1:])}` exits {r.returncode}:\n{r.stderr}")
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


def drawn(dot: str) -> bool:
    return nodes(dot) <= MAX_NODES


def chosen(canonical: str, folded: str) -> tuple[str, bool]:
    """The drawing a page shows of a rule, and whether it is the folded one: the
    canonical where it is within the bound, else the folded one. A rule neither
    is within it is shown as Ein alone, `drawn` false of the folded one."""
    if drawn(canonical):
        return canonical, False
    return folded, True


def stem(page: Path, module: str, rule: str) -> str:
    """The name, less its extension, of the source of `rule`'s drawing on `page`."""
    return f"{page.name[:2]}-{module}.{rule}"


def region(source: str, dot: str, folded: bool, page: Path, module: str, rule: str) -> list[str]:
    lines = ["```lisp", *source.splitlines(), "```"]
    if drawn(dot):
        how = "as the engine draws it folded, each pattern of three arrows one labelled arrow and each of two a loop" if folded \
            else "as the engine draws it"
        lines += ["", f"![The rule {rule} of {module}, {how}]"
                      f"(diagrams/{stem(page, module, rule)}.svg)"]
    return lines


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--check", action="store_true", help="write nothing; exit 1 on a difference")
    ap.add_argument("--missing", action="store_true", help="print the rules no page shows")
    ap.add_argument("--bin", default=os.environ.get("EIN_BIN", str(ROOT / "ein.rs/target/release/ein")))
    args = ap.parse_args()

    mods = modules()
    #: (module, rule) → the rule as written, its digraph, and whether it is folded.
    want: dict[tuple[str, str], tuple[str, str, bool]] = {}
    for name, path in mods.items():
        rules = rules_of(path.read_text())
        if not rules:
            continue
        digraphs = drawings_of(args.bin, path)
        folded = drawings_of(args.bin, path, fold=True)
        if set(digraphs) != set(rules) or set(folded) != set(rules):
            sys.exit(f"gen_stdlib_rule_pages: {name}: the rules read off the file and the digraphs "
                     f"`ein render rules` prints differ: {sorted(set(digraphs) ^ set(rules))}")
        if not MODULE.fullmatch(name):
            sys.exit(f"gen_stdlib_rule_pages: the module's name `{name}` cannot be part of a file's name")
        for rule, source in rules.items():
            if not RULE.fullmatch(rule):
                sys.exit(f"gen_stdlib_rule_pages: {name}: the rule's name `{rule}` cannot be part of "
                         "a file's name (letters, digits and `-`)")
            want[(name, rule)] = (source, *chosen(digraphs[rule], folded[rule]))

    seen: dict[tuple[str, str], list[str]] = {}
    problems: list[str] = []
    stale = 0
    #: The sources to have: file name → its text.
    sources: dict[str, str] = {}
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
                source, dot, fold = want[key]
                body = region(source, dot, fold, page, *key)
                if drawn(dot):
                    sources[stem(page, *key) + ".dot"] = dot.rstrip("\n") + "\n"
                if lines[i + 1 : end] != body:
                    stale += 1
                    if args.check:
                        problems.append(f"{rel}:{i + 1}: the region of {key[0]}'s `{key[1]}` is not what "
                                        f"the module and the engine write")
                out += [lines[i], *body, CLOSE]
            i = end + 1
        if not args.check and not args.missing and out != lines:
            page.write_text("\n".join(out) + "\n")

    # The sources: each as the engine draws its rule, and none left of a rule no region shows.
    lower: dict[str, str] = {}
    for name in sources:
        if lower.setdefault(name.lower(), name) != name:
            sys.exit(f"gen_stdlib_rule_pages: `{name}` and `{lower[name.lower()]}` differ only in case")
    on_disk = sorted(p.name for p in DIAGRAMS.glob("*.dot") if SOURCE.fullmatch(p.name))
    drew = 0
    for name, text in sources.items():
        file = DIAGRAMS / name
        if file.is_file() and file.read_text() == text:
            continue
        drew += 1
        if args.check:
            problems.append(f"{file.relative_to(ROOT)}: "
                            + ("is not the engine's drawing of its rule" if file.is_file()
                               else "is not there: the source of a drawing a page shows"))
        elif not args.missing:
            DIAGRAMS.mkdir(exist_ok=True)
            file.write_text(text)
    for name in on_disk:
        if name in sources:
            continue
        drew += 1
        module, _, rule = name[3 : -len(".dot")].rpartition(".")
        if args.check:
            problems.append(f"{(DIAGRAMS / name).relative_to(ROOT)}: no region of page {name[:2]} shows "
                            f"`{rule}` of {module}: the source of a rule that is gone, renamed or moved")
        elif not args.missing:
            (DIAGRAMS / name).unlink()
            (DIAGRAMS / name).with_suffix(".svg").unlink(missing_ok=True)

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
    verb = ("are as written" if args.check or not (stale or drew)
            else f"written ({stale} region(s) and {drew} source(s) changed)")
    canonical = sum(1 for _, dot, fold in want.values() if drawn(dot) and not fold)
    folds = sum(1 for _, dot, fold in want.values() if drawn(dot) and fold)
    print(f"gen_stdlib_rule_pages: {len(want)} rules of {len(mods)} modules ({canonical} drawn canonical, "
          f"{folds} folded, {len(want) - canonical - folds} as Ein only), their regions and the "
          f"{len(sources)} sources of their drawings {verb}", flush=True)
    if not args.check:
        # The pictures are drawn from the sources, by the script that draws every diagram.
        r = subprocess.run([sys.executable, str(Path(__file__).resolve().parent / "render_diagrams.py"),
                            str(PAGES)])
        if r.returncode != 0:
            print("gen_stdlib_rule_pages: the pictures were not all drawn: run "
                  "`python3 utils/render_diagrams.py`", file=sys.stderr)
            return r.returncode
    return 0


if __name__ == "__main__":
    sys.exit(main())
