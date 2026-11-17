#!/usr/bin/env python3
"""check_tool_mirrors.py — every process tool that copies a list of the engine's still copies it.

    python3 utils/check_tool_mirrors.py

Three tools under `utils/` hold a copy of something the engine defines, and a
copy drifts with no symptom anyone would notice: a new predicate that does
not colour in an editor, a census that files a kernel name as a puzzle's, a
review bundle that never shows a new artefact. The engine's crates read
nothing of the process, so the process holds its own copies to the project,
here, as a static step of the gate. It reads files only: no binary.

1. **The editor grammar**, `vscode-ein/ein.tmLanguage.json`:
   - it parses, claims `source.ein` and the `ein` file type, and has patterns;
   - the names it highlights as declarators are exactly the closed set,
     the first column of `docs/ein/user/reference/reserved-names.md`'s table
     of top-level declarators;
   - its primitives are `ein_core::STRUCTURAL` and the verdict atom `open`
     (reserved, `ein_core::RESERVED`, and no rule-body primitive), and its
     predicates `ein_infer::predicates::names()`, each read out of the source;
   - the four block heads the language does not have (`ontology`, `facts`,
     `reasoning`, `rules`) are highlighted by none of the three scopes.
2. **The natural-language census**, `nl_census.py`: its `KERNEL` is
   `ein_core::RESERVED`, name for name, and it assigns the four registers
   `docs/ein/user/syntax-and-semantics/natural-language.md` § 4 names.
3. **The review bundle's driver**, `review/run.py`: every `ein solve` option
   whose value is a path (`metavar=FILE…` or `DIR`), read off the help
   surface's golden (`ein-cli/tests/golden/help_shape.txt`, which a crate
   test holds to the binary), is one the driver asks for, so the bundle is
   every artefact a run can write.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GRAMMAR = "utils/vscode-ein/ein.tmLanguage.json"
TERMS = "ein.rs/crates/ein-core/src/terms.rs"
PREDICATES = "ein.rs/crates/ein-infer/src/predicates.rs"
RESERVED_PAGE = "docs/ein/user/reference/reserved-names.md"
NL_PAGE = "docs/ein/user/syntax-and-semantics/natural-language.md"
NL_CENSUS = "utils/nl_census.py"
HELP = "ein.rs/crates/ein-cli/tests/golden/help_shape.txt"
DRIVER = "utils/review/run.py"
REMOVED_WRAPPERS = ["ontology", "facts", "reasoning", "rules"]
SCOPES = {
    "declarator": "keyword.control.declarator.ein",
    "primitive": "keyword.control.primitive.ein",
    "predicate": "keyword.operator.predicate.ein",
}


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def rust_strs(body: str, consts: dict[str, str]) -> list[str]:
    """The items of a Rust array literal: string literals, and the names of
    `&str` constants, resolved through `consts`."""
    out = []
    for item in (i.strip() for i in body.split(",")):
        if not item:
            continue
        if item.startswith('"') and item.endswith('"'):
            out.append(item[1:-1])
        elif item in consts:
            out.append(consts[item])
        else:
            raise ValueError(f"an array item that is no string and no known constant: {item!r}")
    return out


def engine_lists() -> tuple[list[str], list[str], list[str]]:
    """`STRUCTURAL` and `RESERVED` out of `ein-core/src/terms.rs`, and
    `predicates::names()` out of `ein-infer/src/predicates.rs`."""
    terms = read(TERMS)
    consts = dict(re.findall(r'^pub const ([A-Z_]+): &str = "([^"]*)";', terms, re.M))

    def array(name: str, text: str) -> list[str]:
        m = re.search(rf"pub const {name}: \[&str; (\d+)\] = \[(.*?)\];", text, re.S)
        if not m:
            raise ValueError(f"{TERMS}: no `pub const {name}: [&str; N] = [...]`")
        items = rust_strs(m.group(2), consts)
        if len(items) != int(m.group(1)):
            raise ValueError(f"{TERMS}: {name} reads as {len(items)} items, declared {m.group(1)}")
        return items

    m = re.search(r"pub fn names\(\) -> \[&'static str; (\d+)\] \{\s*\[(.*?)\]\s*\}",
                  read(PREDICATES), re.S)
    if not m:
        raise ValueError(f"{PREDICATES}: no `pub fn names() -> [&'static str; N]`")
    predicates = rust_strs(m.group(2), consts)
    return array("STRUCTURAL", terms), array("RESERVED", terms), predicates


def lowercase_alternations(regex: str) -> list[str]:
    """Every `(a|b|c)` group of lowercase words in a regex."""
    out = []
    for body in re.findall(r"\(([^()]*)\)", regex):
        words = body.split("|")
        if all(re.fullmatch(r"[a-z][a-z*-]*", w) for w in words):
            out.extend(words)
    return out


def names_for_scope(node, scope: str, found: set[str]) -> None:
    """The names a pattern highlights under `scope`: a head keyword is a
    lowercase alternation in capture group 2 of a `begin` / `match` regex."""
    if isinstance(node, dict):
        caps = node.get("beginCaptures") or node.get("captures") or {}
        if isinstance(caps, dict) and caps.get("2", {}).get("name") == scope:
            found.update(lowercase_alternations(node.get("begin") or node.get("match") or ""))
        for v in node.values():
            names_for_scope(v, scope, found)
    elif isinstance(node, list):
        for v in node:
            names_for_scope(v, scope, found)


def declarators_on_the_page() -> set[str]:
    text = read(RESERVED_PAGE)
    section = text.split("## Top-level declarators", 1)[1].split("\n## ", 1)[0]
    rows = [l for l in section.split("\n") if l.startswith("| `")]
    return {re.match(r"\| `([^`]+)`", l).group(1) for l in rows}


def check_grammar(structural: list[str], reserved: list[str], predicates: list[str]) -> list[str]:
    bad = []
    try:
        g = json.loads(read(GRAMMAR))
    except json.JSONDecodeError as e:
        return [f"{GRAMMAR} is not JSON ({e}): an editor applies no grammar that does not parse"]
    if g.get("scopeName") != "source.ein":
        bad.append(f"{GRAMMAR}: scopeName is {g.get('scopeName')!r}, not source.ein")
    if "ein" not in (g.get("fileTypes") or []):
        bad.append(f"{GRAMMAR} does not claim .ein files")
    if not isinstance(g.get("patterns"), list):
        bad.append(f"{GRAMMAR} has no patterns")
    found = {}
    for key, scope in SCOPES.items():
        found[key] = set()
        names_for_scope(g, scope, found[key])
    page = declarators_on_the_page()
    if len(page) < 8:
        bad.append(f"{RESERVED_PAGE}: only {sorted(page)} read as declarators: the parse stopped")
    if found["declarator"] != page:
        bad.append(f"{GRAMMAR}'s declarators {sorted(found['declarator'])} are not the closed set "
                   f"{RESERVED_PAGE} lists, {sorted(page)}")
    if "open" not in reserved or "open" in structural:
        bad.append("`open` is reserved and no rule-body primitive no longer: the grammar's "
                   "primitives are STRUCTURAL and the verdict atom, which is to be revisited")
    want = set(structural) | {"open"}
    if found["primitive"] != want:
        bad.append(f"{GRAMMAR}'s primitives {sorted(found['primitive'])} are not "
                   f"ein_core::STRUCTURAL and the verdict atom, {sorted(want)}")
    if found["predicate"] != set(predicates):
        bad.append(f"{GRAMMAR}'s predicates {sorted(found['predicate'])} are not "
                   f"predicates::names(), {sorted(predicates)}")
    for key, names in found.items():
        for w in REMOVED_WRAPPERS:
            if w in names:
                bad.append(f"{GRAMMAR} highlights {w!r} as a {key}: the language has no block heads")
    # Non-vacuity: whole names, not prefixes.
    if "rule" not in found["declarator"] or "rules" in found["declarator"]:
        bad.append(f"{GRAMMAR}'s declarators are read by prefix, not by name")
    return bad


def check_nl_census(reserved: list[str]) -> list[str]:
    bad = []
    census = read(NL_CENSUS)
    m = re.search(r"^KERNEL = \{(.*?)\}", census, re.M | re.S)
    if not m:
        return [f"{NL_CENSUS}: no `KERNEL = {{…}}`"]
    kernel = sorted(re.findall(r'"([^"]*)"', m.group(1)))
    if kernel != sorted(reserved):
        bad.append(f"{NL_CENSUS}'s KERNEL {kernel} is not ein_core::RESERVED {sorted(reserved)}")
    page = read(NL_PAGE)
    for reg in ["puzzle", "stdlib", "kernel", "undeclared"]:
        if f"| **{reg}** |" not in page:
            bad.append(f"{NL_PAGE}: no register row {reg}")
        if f'return "{reg}"' not in census:
            bad.append(f"{NL_CENSUS} assigns no register {reg}")
    return bad


def check_review_driver() -> list[str]:
    golden = read(HELP)
    if "COMMAND ein solve\n" not in golden:
        return [f"{HELP} has no `ein solve` parser"]
    solve = golden.split("COMMAND ein solve\n", 1)[1].split("\nCOMMAND ", 1)[0]
    driver = read(DRIVER)
    flags, missing = [], []
    for line in solve.split("\n"):
        if not line.lstrip().startswith("OPTION "):
            continue
        words = line.split()
        if len(words) < 2 or not ("metavar=DIR" in line or "metavar=FILE" in line):
            continue
        flags.append(words[1])
        if f'"{words[1]}"' not in driver:
            missing.append(words[1])
    bad = []
    if len(flags) < 6:
        bad.append(f"found only {flags} in {HELP}: the reader stopped looking")
    if missing:
        bad.append(f"`ein solve` writes artefacts the review bundle never asks for: {missing}. "
                   f"Drive them in {DRIVER}, or the bundle is no longer every artefact")
    return bad


def main() -> int:
    try:
        structural, reserved, predicates = engine_lists()
    except ValueError as e:
        print(f"check_tool_mirrors: {e}", file=sys.stderr)
        return 1
    bad = (check_grammar(structural, reserved, predicates)
           + check_nl_census(reserved)
           + check_review_driver())
    for b in bad:
        print(f"check_tool_mirrors: {b}", file=sys.stderr)
    if bad:
        print(f"check_tool_mirrors: {len(bad)} finding(s)", file=sys.stderr)
        return 1
    print("check_tool_mirrors: the editor grammar, the natural-language census and the review "
          "driver copy what the engine defines")
    return 0


if __name__ == "__main__":
    sys.exit(main())
