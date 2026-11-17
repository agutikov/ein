#!/usr/bin/env python3
"""The pages of `docs/ein/` keep the documentation's rules: names, diagrams, formulas, languages.

Each rule is a decision under `docs/decisions/docs/` (Q-M1f.367 to .371), and
this is what holds the pages to them. It reads every tracked `.md` under
`docs/ein/`, and for rules 1 and 6 those under `docs/decisions/` too, and
reports each finding as `file:line: rule N: what`.

| rule | what fails | where |
|---|---|---|
| 1 | a fenced `dot` block that Graphviz refuses (`dot -Tsvg`, exit ≠ 0) | `docs/ein/`, `docs/decisions/` |
| 2 | the numbering: an entry of `user/`, `reasoning/`, `ein.rs/` or `overview/`, at any depth, that is not `README.md`, not a `diagrams/` folder and not `NN-kebab-name`; a folder whose numbers are not `01..n`, each once | the four folders |
| 3 | a page of the reasoning that speaks an implementation's language: a fenced block that is not `dot` or `plantuml`; a link that resolves outside `docs/`; inline code that is a file path, a Rust path, a crate or test name, or a command line | `docs/ein/reasoning/` |
| 4 | a formula in a block: a fenced block with no language or `text` that holds a mathematical operator no program output holds | `docs/ein/` |
| 5 | a diagram drawn in characters: a fenced block that is not `dot` or `plantuml` and holds box-drawing corners, junctions or verticals, an arrow of dashes (`-->`, `<--`), a `+--` corner, or, in a block with no language or `text`, a line that is only a connector (`|`, `v`, `^`) or one that chains two or more arrows (`a ← b ← c`) | `docs/ein/` |
| 6 | with `--plantuml`: a fenced `plantuml` block with no `@start…` line, or one PlantUML does not draw (no SVG, or the words *Syntax error* in it) | `docs/ein/`, `docs/decisions/` |

    python3 utils/check_doc_blocks.py                # the gate's form: rules 1 to 5, exit 1 on a finding
    python3 utils/check_doc_blocks.py --plantuml     # rule 6 as well (the dev host's: needs a PlantUML that knows @startebnf)
    python3 utils/check_doc_blocks.py --list         # every finding of rules 1 to 5, exit 0
    python3 utils/check_doc_blocks.py --list --rule 3 --rule 4 docs/ein/reasoning
    python3 utils/check_doc_blocks.py --counts       # findings per rule and per perspective, exit 0

**The heuristics, stated.**

*Rule 3, inline code.* A span between single backticks fails when it is

- a file path: it ends in a source or data extension (`.rs`, `.ein`, `.einb`,
  `.py`, `.sh`, `.toml`, `.json`, `.y`, `.l`, `.txt`, `.md5`, `.lock`), or it
  starts with a directory of the repository (`ein.rs/`, `examples/`,
  `tests/`, `stdlib/`, `corpus/`); a page's name, `08-monotonicity.md`, is not
  one, and neither is `ein.rs` alone, the implementation's name;
- a Rust path: it holds `::`;
- a crate name: `ein-core`, `ein_infer` and the six others, alone or leading
  a path;
- a test, function or field name: one lower-case identifier of three or more
  words joined by `_` (Ein's own names join words with `-`), with or without
  `()`;
- a command line: `ein` and a subcommand, with or without arguments, `ein`
  and an option, or a line that starts `cargo `, `python3 ` or `./`.

*Rules 4 and 5, what is output.* A block is program output, and neither rule
reads it, when its page is written whole from the binary's runs (one of its
first twelve lines says `The page is generated`, or starts `<!-- generated`),
when the block stands inside a region a test compares (between `<!-- name …`
and `<!-- /name -->`, whatever the name: `generated`, `transcript`, `rows`,
`state-file`), or when the line before it is `<!-- output -->`, which a page
writes only before what `ein` printed.

*Rule 4, the operators.* `⟺ ⇔ ⟸ ∀ ∃ ∄ ⊨ ⊢ ∨ ∈ ∉ ⊆ ⊂ ⊇ ⊃ ∪ ∩ ≤ ≠ ≡ ↦ ⊥ ∖ ⇒ ∑ ∏
∘ ≜ √`: the mathematical operators that stand in no golden under
`ein.rs/crates/*/tests/golden/` and nowhere on the generated read-out page,
which is where what `ein solve` and its trace print is banked. Those do hold
`⟹ ∧ ¬ ≥ ∅ → ← ⊤ ×` (a trace prints a rule's `:why`, and the standard
library's say `⟹` and `∧`), so they are not in the set. Three surfaces write
operators of the set and are not text a page shows in a `text` block: a rule's
graph labels its quantifiers `∀` and `∄`, the region's drawing and
`region.md` write `≠ ⊂ ⇒ ≡`, and a slice's drawing `⊥`. A page that quotes
such a line marks the block `<!-- output -->`.

**What it does not catch.**

- Rule 3: an option written alone (`--minimal`); a relation, a keyword or a
  form of Ein written inline (`(false)`, `:match`), which a page of the
  reasoning should state as mathematics; a type's or a module's name with no
  `::` (`Kb`, `saturator`); a link to another site; a table whose cells name
  tests in words; an algorithm told in prose.
- Rule 4: a formula spelt only in the operators a read-out also prints
  (`∧ ¬ → ⟹ ≥ ×`) or in ASCII (`<=>`, `|=`), and a formula in a block of
  another language or in a paragraph.
- Rule 5: a drawing with one arrow a line, or made only of letters, dashes
  and indentation; a drawing a page marks as output.
- Rule 1 checks that Graphviz lays the block out, not that it draws what the
  text says. Rule 6 is PlantUML's syntax check, which a block of an unknown
  diagram type passes as the type it guesses.
"""
from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]

DOCS = "docs/ein/"
DECISIONS = "docs/decisions/"
REASONING = "docs/ein/reasoning/"
#: The folders whose entries are numbered, at every depth.
NUMBERED = ("docs/ein/user", "docs/ein/reasoning", "docs/ein/ein.rs", "docs/ein/overview")
#: What a numbered folder holds without a number.
UNNUMBERED = ("README.md", "diagrams")
NAME = re.compile(r"(\d\d)-[a-z0-9]+(?:[.-][a-z0-9]+)*")

ALL_RULES = (1, 2, 3, 4, 5)

FENCE = re.compile(r"^ {0,3}(`{3,}|~{3,})([^`]*)$")
DIAGRAMS = ("dot", "plantuml")

#: Rule 4: the operators no run of `ein` prints (the docstring says how the set was taken).
FORMULA = "⟺⇔⟸∀∃∄⊨⊢∨∈∉⊆⊂⊇⊃∪∩≤≠≡↦⊥∖⇒∑∏∘≜√"
#: Rule 5: corners, junctions and verticals of the box-drawing block, and arrow heads; the
#: engine rules a line with `─ ━ ┈ ╌` and marks one with `▶`, which are not here.
BOX = "┌┐└┘├┤┬┴┼│┃┏┓┗┛┣┫┳┻╋╭╮╯╰║╔╗╚╝╠╣╦╩╬▼▲◀►◄"
ART = re.compile(r"-{2,}>|<-{2,}|\+-{2,}|-{2,}\+")
CONNECTOR = re.compile(r"^[\s|v^]*[|v^][\s|v^]*$")
ARROW = re.compile(r"[←→↔⟶⟵]|(?<![-<])->|<-(?![->])")

EXT = r"(?:rs|ein|einb|py|sh|toml|json|y|l|txt|md5|lock)"
CRATES = r"ein[-_](?:core|ir|infer|einb|render|cli|corpus|narration)"
INLINE = (
    ("a file path", re.compile(r"(?!ein\.rs$)[\w./*{}<>-]*[\w*}>]\." + EXT + r"(?::\d+)?")),
    ("a file path", re.compile(r"(?:ein\.rs|examples|tests|stdlib|corpus)/\S*")),
    ("a Rust path", re.compile(r"\S*::\S*")),
    ("a crate name", re.compile(CRATES + r"(?:[/.:]\S*)?")),
    ("a test, function or field name", re.compile(r"[a-z][a-z0-9]*(?:_[a-z0-9]+){2,}(?:\(\))?")),
    ("a command line", re.compile(
        r"ein (?:oracle|render|saturate|show|solve|test|kb)(?:\s.*)?|ein --\S.*"
        r"|(?:cargo|python3) \S.*|\./\S.*")),
)
CODE_SPAN = re.compile(r"(?<!`)`([^`\n]+)`(?!`)")
LINK = re.compile(r"\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
MARK_OPEN = re.compile(r"^<!--\s*([a-z][\w-]*)\b")
MARK_CLOSE = re.compile(r"^<!--\s*/([a-z][\w-]*)\s*-->")


class Block:
    """One fenced block: the line of its opening fence (1-based), its
    language, its body's lines, and whether the page marks it as output."""

    def __init__(self, line: int, lang: str, output: bool):
        self.line, self.lang, self.output = line, lang, output
        self.body: list[str] = []


def tracked(prefixes: tuple[str, ...]) -> list[str]:
    out = subprocess.run(["git", "-C", str(REPO), "ls-files", "-z", "--", *prefixes],
                         capture_output=True, text=True, check=True).stdout
    return sorted(p for p in out.split("\0") if p)


def parse(text: str) -> tuple[list[Block], list[tuple[int, str]]]:
    """The fenced blocks of a page, and its prose lines as (line, text),
    a fence's own lines and a block's body left out."""
    lines = text.split("\n")
    whole = any(l.startswith("<!-- generated") or "The page is generated" in l for l in lines[:12])
    blocks: list[Block] = []
    prose: list[tuple[int, str]] = []
    region: str | None = None
    cur: Block | None = None
    fence = ""
    last = ""  # the last non-blank prose line
    for i, line in enumerate(lines, start=1):
        m = FENCE.match(line)
        if cur is not None:
            if m and m.group(1)[0] == fence[0] and len(m.group(1)) >= len(fence) \
                    and not m.group(2).strip():
                blocks.append(cur)
                cur = None
                last = ""  # a marker speaks for the one block under it
            else:
                cur.body.append(line)
            continue
        if m:
            fence = m.group(1)
            lang = m.group(2).strip().split()[0] if m.group(2).strip() else ""
            marked = last.strip() == "<!-- output -->"
            cur = Block(i, lang, whole or region is not None or marked)
            continue
        if (c := MARK_CLOSE.match(line)) and c.group(1) == region:
            region = None
        elif region is None and (o := MARK_OPEN.match(line)) and o.group(1) != "output":
            # A region only when the page closes it; a lone comment marks nothing.
            name = o.group(1)
            if any(MARK_CLOSE.match(l) and MARK_CLOSE.match(l).group(1) == name
                   for l in lines[i:]):
                region = name
        prose.append((i, line))
        if line.strip():
            last = line
    if cur is not None:
        blocks.append(cur)
    return blocks, prose


# ── the rules ───────────────────────────────────────────────────────────────

Finding = tuple[str, int, int, str]  # (file, line, rule, what)


def run_blocks(jobs: list[tuple[str, Block]], cmd: list[str],
               verdict) -> list[tuple[str, Block, str]]:
    """Each block's body through `cmd` on stdin, in parallel; `verdict` reads
    (exit code, stdout, stderr) and answers what is wrong, or None."""
    def one(job: tuple[str, Block]):
        rel, block = job
        p = subprocess.run(cmd, input="\n".join(block.body) + "\n", capture_output=True, text=True)
        return rel, block, verdict(p.returncode, p.stdout, p.stderr)
    with ThreadPoolExecutor(max_workers=os.cpu_count() or 1) as pool:
        return [r for r in pool.map(one, jobs) if r[2] is not None]


def rule1(pages: dict[str, list[Block]]) -> list[Finding]:
    jobs = [(rel, b) for rel, blocks in pages.items() for b in blocks if b.lang == "dot"]

    def verdict(rc: int, _out: str, err: str):
        if rc == 0:
            return None
        first = next((l for l in err.splitlines() if l.strip()), "no message")
        return f"Graphviz refuses this `dot` block (exit {rc}): {first.strip()[:160]}"
    return [(rel, b.line, 1, what) for rel, b, what in run_blocks(jobs, ["dot", "-Tsvg"], verdict)]


def rule2(files: list[str]) -> list[Finding]:
    """`files` is every tracked file under `docs/ein/`."""
    out: list[Finding] = []
    entries: dict[str, set[str]] = {}
    for rel in files:
        for root in NUMBERED:
            if not rel.startswith(root + "/"):
                continue
            parts = rel.split("/")
            for depth in range(len(root.split("/")), len(parts)):
                folder = "/".join(parts[:depth])
                if any(p == "diagrams" for p in parts[:depth]):
                    break
                entries.setdefault(folder, set()).add(parts[depth])
    for folder in sorted(entries):
        numbers: dict[str, list[str]] = {}
        for name in sorted(entries[folder]):
            if name in UNNUMBERED:
                continue
            m = NAME.fullmatch(name)
            if not m:
                out.append((f"{folder}/{name}", 0, 2,
                            "not `README.md`, not `diagrams/` and not `NN-kebab-name`"))
                continue
            numbers.setdefault(m.group(1), []).append(name)
        for n, names in sorted(numbers.items()):
            if len(names) > 1:
                out.append((folder, 0, 2, f"the number {n} names {len(names)} entries: "
                            + ", ".join(names)))
        want = [f"{i:02d}" for i in range(1, len(numbers) + 1)]
        if sorted(numbers) != want:
            out.append((folder, 0, 2, "its numbers are " + " ".join(sorted(numbers))
                        + f", not 01 to {len(numbers):02d} with no gap"))
    return out


def rule3(rel: str, blocks: list[Block], prose: list[tuple[int, str]]) -> list[Finding]:
    if not rel.startswith(REASONING):
        return []
    out: list[Finding] = []
    for b in blocks:
        if b.lang not in DIAGRAMS:
            out.append((rel, b.line, 3, f"a fenced `{b.lang or '(no language)'}` block: the "
                        "reasoning's blocks are `dot` and `plantuml`"))
    here = os.path.dirname(rel)
    for i, line in prose:
        for m in LINK.finditer(line):
            target = m.group(1).split("#")[0]
            if not target or re.match(r"[a-z][a-z0-9+.-]*:", target):
                continue
            res = os.path.normpath(os.path.join(here, target))
            if not (res == "docs" or res.startswith("docs/")):
                out.append((rel, i, 3, f"a link out of `docs/`: `{m.group(1)}`"))
        for m in CODE_SPAN.finditer(line):
            span = m.group(1).strip()
            for kind, pat in INLINE:
                if pat.fullmatch(span):
                    out.append((rel, i, 3, f"{kind} in inline code: `{span}`"))
                    break
    return out


def rule4(rel: str, blocks: list[Block]) -> list[Finding]:
    out: list[Finding] = []
    for b in blocks:
        if b.output or b.lang not in ("", "text"):
            continue
        for k, line in enumerate(b.body, start=1):
            found = sorted({c for c in line if c in FORMULA})
            if found:
                out.append((rel, b.line + k, 4, "a formula in a "
                            f"`{b.lang or '(no language)'}` block (`{' '.join(found)}`): "
                            "write it as TeX, between `$$` lines"))
                break
    return out


def rule5(rel: str, blocks: list[Block]) -> list[Finding]:
    out: list[Finding] = []
    for b in blocks:
        if b.output or b.lang in DIAGRAMS:
            continue
        for k, line in enumerate(b.body, start=1):
            box = sorted({c for c in line if c in BOX})
            art = ART.search(line)
            what = None
            if box:
                what = f"box-drawing `{' '.join(box)}`"
            elif art:
                what = f"`{art.group(0)}`"
            elif CONNECTOR.match(line) and b.lang in ("", "text"):
                what = "a line that is only a connector"
            elif len(ARROW.findall(line)) >= 2 and b.lang in ("", "text"):
                what = "a line that chains arrows"
            if what:
                out.append((rel, b.line + k, 5, f"a diagram drawn in characters ({what}) in a "
                            f"`{b.lang or '(no language)'}` block: draw it as a `dot` or a "
                            "`plantuml` block, or mark what `ein` printed with "
                            "`<!-- output -->` on the line before"))
                break
    return out


def rule6(pages: dict[str, list[Block]]) -> list[Finding]:
    out: list[Finding] = []
    jobs = []
    for rel, blocks in pages.items():
        for b in blocks:
            if b.lang != "plantuml":
                continue
            first = next((l.strip() for l in b.body if l.strip()), "")
            if not first.startswith("@start"):
                out.append((rel, b.line, 6, "a `plantuml` block that does not open with `@start…`"))
            else:
                jobs.append((rel, b))

    # `plantuml -syntax` passes a block the renderer then draws as the words "Syntax error!" (an
    # EBNF exception with a group on its right, an empty option), so the block is drawn and the
    # drawing read.
    def verdict(rc: int, stdout: str, err: str):
        if rc != 0 or "<svg" not in stdout:
            return "PlantUML draws nothing for this block" + (f": {err.strip()[:160]}" if err.strip() else "")
        if "Syntax error" in stdout:
            return "PlantUML draws this block as a syntax error"
        return None
    out += [(rel, b.line, 6, what)
            for rel, b, what in run_blocks(jobs, ["plantuml", "-tsvg", "-pipe"], verdict)]
    return out


def perspective(rel: str) -> str:
    parts = rel.split("/")
    if rel.startswith(DOCS) and len(parts) > 3:
        return parts[2] + "/"
    return "docs/ein/ (top)" if rel.startswith(DOCS) else "docs/decisions/"


# ── main ────────────────────────────────────────────────────────────────────


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--all", action="store_true", help="rules 1 to 5, which is the default; kept so an older command line still runs")
    ap.add_argument("--plantuml", action="store_true",
                    help="rule 6 as well: every `plantuml` block drawn by `plantuml -tsvg`")
    ap.add_argument("--rule", type=int, action="append", choices=range(1, 7), metavar="N",
                    help="only rule N (repeatable)")
    ap.add_argument("--list", action="store_true",
                    help="print every finding of rules 1 to 5 (or of each --rule) and exit 0")
    ap.add_argument("--counts", action="store_true",
                    help="findings per rule and per perspective, and exit 0")
    ap.add_argument("paths", nargs="*", metavar="PATH",
                    help="only the pages under each PATH (rule 2 always reads the whole tree)")
    args = ap.parse_args()

    if args.rule:
        rules = tuple(sorted(set(args.rule)))
    else:
        rules = ALL_RULES + ((6,) if args.plantuml else ())

    for rule, tool in ((1, "dot"), (6, "plantuml")):
        if rule in rules and shutil.which(tool) is None:
            print(f"error: no `{tool}` on PATH — rule {rule} did not run.", file=sys.stderr)
            return 127

    files = tracked((DOCS, DECISIONS))
    keep = [a.rstrip("/") for a in args.paths]
    pages: dict[str, list[Block]] = {}
    proses: dict[str, list[tuple[int, str]]] = {}
    for rel in files:
        if not rel.endswith(".md"):
            continue
        if keep and not any(rel == k or rel.startswith(k + "/") for k in keep):
            continue
        path = REPO / rel
        if not path.is_file() or path.is_symlink():
            continue
        pages[rel], proses[rel] = parse(path.read_text(encoding="utf-8"))

    found: list[Finding] = []
    if 1 in rules:
        found += rule1(pages)
    if 2 in rules:
        found += rule2([f for f in files if f.startswith(DOCS)])
    in_docs = {rel: b for rel, b in pages.items() if rel.startswith(DOCS)}
    for rel, blocks in in_docs.items():
        if 3 in rules:
            found += rule3(rel, blocks, proses[rel])
        if 4 in rules:
            found += rule4(rel, blocks)
        if 5 in rules:
            found += rule5(rel, blocks)
    if 6 in rules:
        found += rule6(pages)
    found.sort()

    n_blocks = sum(len(b) for b in pages.values())
    n_dot = sum(1 for bs in pages.values() for b in bs if b.lang == "dot")
    n_puml = sum(1 for bs in pages.values() for b in bs if b.lang == "plantuml")

    if args.counts:
        table: dict[tuple[int, str], int] = {}
        for rel, _line, rule, _what in found:
            table[(rule, perspective(rel))] = table.get((rule, perspective(rel)), 0) + 1
        for rule in rules:
            row = {p: n for (r, p), n in table.items() if r == rule}
            total = sum(row.values())
            parts = ", ".join(f"{p} {n}" for p, n in sorted(row.items()))
            print(f"rule {rule}: {total}" + (f" ({parts})" if parts else ""))
        return 0

    if args.list:
        for rel, line, rule, what in found:
            print(f"{rel}:{line}: rule {rule}: {what}" if line else f"{rel}: rule {rule}: {what}")
        return 0

    said = "rules " + ", ".join(str(r) for r in rules)
    if found:
        print(f"doc blocks: {len(found)} finding(s) of {said}:", file=sys.stderr)
        for rel, line, rule, what in found:
            print(f"  {rel}:{line}: rule {rule}: {what}" if line
                  else f"  {rel}: rule {rule}: {what}", file=sys.stderr)
        return 1
    print(f"doc blocks ok — {len(pages)} page(s), {n_blocks} fenced block(s), {n_dot} `dot`"
          + (f", {n_puml} `plantuml`" if 6 in rules else "") + f"; {said} hold")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
