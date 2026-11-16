#!/usr/bin/env python3
"""The structure bundle — M1f S1f.10.12, task 1: the hypothesis set's structure, shown.

    utils/structure_bundle.py [--out DIR] [--bin EIN] [--jobs N] [FILE.ein …]   write the bundle
    utils/structure_bundle.py --check [--out DIR]                                 every link, every raw file

What P1f.10's part 1 computed about a program's hypothesis set H, on every
corpus program that searches, put in front of a reader **before** any surface
of `ein` is designed around it. The bundle is a review artefact for one stop;
no tier of the gate runs this script.

**It computes no structure.** Every number on a page is read from a file the
engine wrote, and the page names that file. The inputs, all under the
untracked `build/`:

- `build/structure/census.jsonl` and `census.env`, and per program
  `build/structure/<program>/{pairs,theory,cover,exactly_one,coupling}.json`,
  their `.md` twins, `theory.ein` and `coupling.dot`: the example `pair_layer`'s
  (`utils/README.md`, the measurement set), by
  `utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer --routes --check --readings`;
- `build/structure/prices/` and per program `prices.json`, `prices.md` and
  `prices/<question>.nodes.jsonl`: the example `price_list`'s;
- `build/s1f.10.9/`, `build/s1f.10.6/` and `build/s1f.10.15/`: track H's
  censuses and witnesses (`object_census`, `owed_census`, `rung_census`);
- `build/structure/s14/must.json`: S1f.10.14's T3, what a program would
  have to state where *exactly one* is not derived, and every question of
  `ein solve` before and after on the copy that states it, read off the
  read-outs by `build/structure/s14/must.py` from `probes.sh`'s runs;
- the phase's own Markdown under `plans/…/p1f.10_hypothesis_structure/` and
  `open_questions.md`, rendered with links back to the files.

What the script does itself, each said on the page it is on: it **joins** the
engine's lists by a candidate's text (H listed), it **draws** (the DOT of the
exclusion graph and of the questions' hypergraph is written here from the
engine's JSON; `coupling.dot` is the engine's own and is drawn as it is), and
it makes **two readings by name**, which the owner may not make and which are
marked as a script's: a relation's name off a candidate's text, with what the
program states of it (`bijective` and its halves), as S1f.10.4's
`build/structure/s4/bijections.py` reads it; and S1f.10.9's marks carried from
the names its census lists to the candidates that write them.

It writes `DIR/index.html` and everything else under `DIR/bundle/`, which it
owns and replaces whole; no file of a stage is rewritten. DIR defaults to
`build/structure`. The pages are plain HTML with one stylesheet, light and
dark by `prefers-color-scheme`, and fetch nothing: they work from `file://`.
Two runs over the same inputs write the same bytes.

A missing or malformed input stops the script, non-zero, naming the file:
no page is written with a hole. A DOT Graphviz refuses stops it too, with
Graphviz's message. A picture of more than `DRAW_BOUND` edges is not laid
out: its DOT is kept, the page says so (`.undrawn.txt` beside the DOT), and
the parts of it are drawn instead.

`FILE.ein …` writes the bundle of those programs alone, for trying a change.
`--check` walks every page: a link that does not resolve, a page with a
fragment no element carries, or a table or a picture without its raw file
beside it is an error, exit 1.

The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`: its
`--version` lines are on every page. Graphviz (`dot`, with the layout plugin
that holds `neato`) must be on PATH.
"""

from __future__ import annotations

import concurrent.futures
import html
import json
import os
import re
import shutil
import subprocess
import sys
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PLANS = REPO / "plans/m1f_hypothesis_and_documentation"
PHASE = PLANS / "p1f.10_hypothesis_structure"
QUESTIONS_MD = PLANS / "open_questions.md"

#: A picture of more edges than this is not laid out. The engine's own bound
#: for a DOT view, `ein-render/tests/dot_wellformed.rs`'s `DOT_LAYOUT_BOUND`.
DRAW_BOUND = 1000
#: A picture of more edges than this is also drawn in parts.
SPLIT_BOUND = 150
#: A table of more rows than this is folded.
FOLD_ROWS = 40
#: The marker that says a directory is this script's to replace.
MARKER = ".structure_bundle"

INSTANCES = [
    ("I-Z1", "examples/zebra.ein"),
    ("I-Z2", "examples/zebra2.ein"),
    ("I-Z2M", "examples/zebra2-minus-15.ein"),
    ("I-B06", "examples/branching/06_lookahead_on.ein"),
    ("I-L02", "examples/lattice/02_genuine_3set_death.ein"),
    ("I-SEND", "examples/beyond_arith/send_more_money.ein"),
]
TAG = {path: tag for tag, path in INSTANCES}

#: The price list's six questions: the key of a cell, and the flags of `ein solve`.
ASKED = [("describe", "describe"), ("count", "--count"), ("single", "--single"),
         ("n1", "-n 1"), ("minimal", "--minimal"), ("maximal", "--maximal")]
#: The ways of spending, in the order `prices.md` tables them.
WAYS = ["layer-whole", "layer-by-class", "singles", "denials", "layer-by-need", "death-pairs",
        "death-core-pair", "death-core", "theory", "theory-shared", "cover", "wipe-out",
        "wipe-out-layer", "factor"]
#: The parts a cell's work is split into, with `prices.md`'s column names.
SPLIT = [("in_r", "in R"), ("owing_owed_group", "owing, an owed group"), ("owing_other", "owing otherwise"),
         ("dead_theory", "dead, a pair of the theory"), ("dead_puzzle", "dead, a pair of the puzzle"),
         ("dead_single", "dead, a candidate dead alone"), ("dead_edge", "dead otherwise, inside one edge"),
         ("dead_other", "dead otherwise")]

#: The stages of part 1, in the order they ran, and track H's.
STAGES = [
    ("S1f.10.1", "s1f.10.1_exclusion_census.md"),
    ("S1f.10.10", "s1f.10.10_the_theorys_share.md"),
    ("S1f.10.2", "s1f.10.2_groups.md"),
    ("S1f.10.4", "s1f.10.4_bounded_groups.md"),
    ("S1f.10.11", "s1f.10.11_coupling.md"),
    ("S1f.10.13", "s1f.10.13_the_price_list.md"),
    ("S1f.10.9", "s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md"),
    ("S1f.10.6", "s1f.10.6_obligations_under_hypothesis.md"),
    ("S1f.10.15", "s1f.10.15_the_rung_a_loaded_rule_chooses.md"),
]

CENSUS_COMMANDS = """\
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer -p ein-cli
utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer --routes --check --readings \\
    > build/structure/census.jsonl 2> build/structure/census.env"""
PRICES_COMMANDS = """\
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example price_list
utils/bench_env.sh --require-ac ein.rs/target/release/examples/price_list \\
    > build/structure/prices/verdicts.md 2> build/structure/prices/prices.env"""
MUST_COMMANDS = """\
bash build/structure/s14/probes.sh
python3 build/structure/s14/must.py > build/structure/s14/must.json"""
TRACK_H = [
    ("S1f.10.9", "s1f.10.9", "object_census", "Q-M1f.306",
     "cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example object_census "
     "> build/s1f.10.9/census/rows.tsv"),
    ("S1f.10.6", "s1f.10.6", "owed_census", "Q-M1f.307", "bash build/s1f.10.6/run.sh"),
    ("S1f.10.15", "s1f.10.15", "rung_census", "Q-M1f.308", "bash build/s1f.10.15/run.sh"),
]


class Fail(Exception):
    """An input is missing or malformed, or Graphviz refused: the message names the file."""


class Doc(dict):
    """A JSON object that names its file when a key it is asked for is missing."""

    file: Path = Path("?")

    def __missing__(self, key):
        raise Fail(f"{self.file}: no key {key!r} where one is read (the object has: "
                   f"{', '.join(sorted(map(str, self))) or 'nothing'})")


def read_text(path: Path) -> str:
    try:
        return path.read_text()
    except OSError as e:
        raise Fail(f"{path}: {e.strerror or e}") from None


def parse_json(text: str, path: Path, where: str = ""):
    def hook(d):
        o = Doc(d)
        o.file = path
        return o
    try:
        return json.loads(text, object_hook=hook)
    except json.JSONDecodeError as e:
        raise Fail(f"{path}{where}: malformed JSON: {e}") from None


def load_json(path: Path):
    return parse_json(read_text(path), path)


def load_jsonl(path: Path) -> list:
    rows = []
    for k, line in enumerate(read_text(path).splitlines(), 1):
        if line.strip():
            rows.append(parse_json(line, path, f", line {k}"))
    if not rows:
        raise Fail(f"{path}: no line")
    return rows


def load_tsv(path: Path) -> tuple[list[str], dict[str, dict[str, str]], list[str]]:
    """A census of track H: its header, its rows by `file`, and its `# ` summary lines."""
    lines = read_text(path).splitlines()
    data = [l for l in lines if l and not l.startswith("#")]
    if not data or not data[0].startswith("file\t"):
        raise Fail(f"{path}: no header line starting `file`")
    head = data[0].split("\t")
    rows = {}
    for l in data[1:]:
        cells = l.split("\t")
        rows[cells[0]] = dict(zip(head, cells))
    return head, rows, [l for l in lines if l.startswith("#")]


def need(path: Path) -> Path:
    if not path.exists():
        raise Fail(f"{path}: missing")
    return path


def esc(s) -> str:
    return html.escape(str(s), quote=True)


def rel(target: Path, page_dir: Path) -> str:
    """`target` as a link written on a page in `page_dir`."""
    return os.path.relpath(target, page_dir).replace(os.sep, "/")


def size_of(path: Path) -> str:
    n = path.stat().st_size
    for unit, k in (("MB", 1 << 20), ("KB", 1 << 10)):
        if n >= k:
            return f"{n / k:.1f} {unit}"
    return f"{n} B"


# ── cells and tables ──────────────────────────────────────────────────────────

class C:
    """A table cell: its HTML, and the value its JSON twin holds."""

    __slots__ = ("h", "v", "num")

    def __init__(self, h: str, v, num: bool = False):
        self.h, self.v, self.num = h, v, num


def T(s) -> C:
    """Text."""
    return C("" if s is None else esc(s), s)


def K(s) -> C:
    """A fact, a name or a rule, as the program writes it."""
    return C("" if s is None else f"<code>{esc(s)}</code>", s)


def N(n) -> C:
    """A count."""
    if n is None:
        return C("", None, True)
    if isinstance(n, bool):
        return T("yes" if n else "no")
    if isinstance(n, float):
        return C(esc(f"{n:.6g}"), n, True)
    return C(esc(n), n, True)


def KS(items, sep: str = "<br>") -> C:
    """Several facts."""
    items = list(items)
    return C(sep.join(f"<code>{esc(i)}</code>" for i in items), items)


def H(h: str, v) -> C:
    """HTML as written, with the value the JSON holds for it."""
    return C(h, v)


def W(seconds) -> C:
    """A wall, as its census measured it."""
    if seconds is None:
        return C("", None, True)
    return C(esc(f"{seconds * 1000:.1f} ms" if seconds < 1 else f"{seconds:.2f} s"), seconds, True)


def any_cell(v) -> C:
    """A value of a kind no column was written for: shown whole, never dropped."""
    if v is None:
        return T("")
    if isinstance(v, (bool, int, float)):
        return N(v)
    if isinstance(v, str):
        return K(v) if v.startswith("(") else T(v)
    if isinstance(v, list) and all(isinstance(x, str) for x in v):
        return KS(v)
    if isinstance(v, dict) and all(not isinstance(x, (dict, list)) for x in v.values()):
        return C("<br>".join(f"{esc(k)} {any_cell(x).h}" for k, x in v.items()), v)
    return C(f"<code class=j>{esc(json.dumps(v, ensure_ascii=False))}</code>", v)


def key_values(d: dict, skip=()) -> C:
    """An object as `key value` lines."""
    parts = []
    for k, v in d.items():
        if k in skip:
            continue
        parts.append(f"{esc(k)} {any_cell(v).h}")
    return C("<br>".join(parts), {k: v for k, v in d.items() if k not in skip})


def slug(text: str) -> str:
    """GitHub's anchor of a heading."""
    text = re.sub(r"`|\*|~~", "", text)
    text = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", text)
    text = text.strip().lower()
    text = re.sub(r"[^\w\- ]", "", text)
    return text.replace(" ", "-")


# ── Markdown, as far as the repo's pages and the engine's twins write it ─────

class Markdown:
    """Markdown to HTML: headings, paragraphs, tables, lists, quotes, fences, and the inline
    marks. `source` is the file the text is from, so that a relative link resolves from the
    page; `whole` says the page holds the file whole, so that `#anchor` stays on the page."""

    def __init__(self, source: Path, page_dir: Path, whole: bool, prefix: str = ""):
        self.source, self.page_dir, self.whole, self.prefix = source, page_dir, whole, prefix
        self.seen: Counter = Counter()
        self.headings: list[tuple[int, str, str]] = []

    # inline

    def link(self, url: str) -> str:
        url = url.strip()
        if re.match(r"[a-z][a-z0-9+.-]*:", url):
            return url
        path, _, frag = url.partition("#")
        if not path:
            if self.whole:
                return "#" + self.prefix + frag
            return rel(self.source, self.page_dir) + "#" + frag
        target = (self.source.parent / path).resolve()
        return rel(target, self.page_dir) + ("#" + frag if frag else "")

    @staticmethod
    def _close(s: str, i: int, token: str) -> int:
        """The next `token` at or after `i` that is outside a code span, or -1."""
        n = len(s)
        while i < n:
            if s[i] == "\\":
                i += 2
                continue
            if s[i] == "`":
                j = i
                while j < n and s[j] == "`":
                    j += 1
                end = s.find("`" * (j - i), j)
                i = j if end < 0 else end + (j - i)
                continue
            if s.startswith(token, i):
                return i
            i += 1
        return -1

    @staticmethod
    def _bracket(s: str, i: int, open_: str, close: str) -> int:
        """The bracket that closes the one at `i`, code spans skipped, or -1."""
        depth, n = 0, len(s)
        while i < n:
            c = s[i]
            if c == "\\":
                i += 2
                continue
            if c == "`":
                j = i
                while j < n and s[j] == "`":
                    j += 1
                end = s.find("`" * (j - i), j)
                i = j if end < 0 else end + (j - i)
                continue
            if c == open_:
                depth += 1
            elif c == close:
                depth -= 1
                if depth == 0:
                    return i
            i += 1
        return -1

    def inline(self, s: str) -> str:
        out, i, n = [], 0, len(s)
        while i < n:
            c = s[i]
            if c == "\\" and i + 1 < n and s[i + 1] in "\\`*_{}[]()#+-.!|~<>":
                out.append(esc(s[i + 1]))
                i += 2
                continue
            if c == "`":
                j = i
                while j < n and s[j] == "`":
                    j += 1
                ticks = s[i:j]
                end = s.find(ticks, j)
                while end >= 0 and s.startswith(ticks + "`", end):
                    end = s.find(ticks, end + len(ticks) + 1)
                if end >= 0:
                    code = s[j:end].replace("\n", " ")
                    if len(code) > 1 and code[0] == " " and code[-1] == " ":
                        code = code[1:-1]
                    out.append(f"<code>{esc(code)}</code>")
                    i = end + len(ticks)
                    continue
                out.append(esc(ticks))
                i = j
                continue
            if c == "[" or (c == "!" and s.startswith("![", i)):
                image = c == "!"
                start = i + 1 if image else i
                end = self._bracket(s, start, "[", "]")
                if end >= 0 and end + 1 < n and s[end + 1] == "(":
                    close = self._bracket(s, end + 1, "(", ")")
                    if close >= 0:
                        text, url = s[start + 1:end], s[end + 2:close].split(' "')[0]
                        if image:
                            out.append(f'<img src="{esc(self.link(url))}" alt="{esc(text)}">')
                        else:
                            out.append(f'<a href="{esc(self.link(url))}">{self.inline(text)}</a>')
                        i = close + 1
                        continue
            if s.startswith("**", i):
                end = self._close(s, i + 2, "**")
                if end > i + 2:
                    out.append(f"<strong>{self.inline(s[i + 2:end])}</strong>")
                    i = end + 2
                    continue
            if s.startswith("~~", i):
                end = self._close(s, i + 2, "~~")
                if end > i + 2:
                    out.append(f"<del>{self.inline(s[i + 2:end])}</del>")
                    i = end + 2
                    continue
            if c == "*" and i + 1 < n and not s[i + 1].isspace() and s[i + 1] != "*":
                end = self._close(s, i + 1, "*")
                while end >= 0 and (s[end - 1].isspace() or s.startswith("**", end)):
                    end = self._close(s, end + (2 if s.startswith("**", end) else 1), "*")
                if end > i + 1:
                    out.append(f"<em>{self.inline(s[i + 1:end])}</em>")
                    i = end + 1
                    continue
            if c == "<":
                m = re.match(r"<(https?://[^>\s]+)>", s[i:])
                if m:
                    out.append(f'<a href="{esc(m.group(1))}">{esc(m.group(1))}</a>')
                    i += m.end()
                    continue
            out.append(esc(c))
            i += 1
        return "".join(out)

    # blocks

    @staticmethod
    def _cells(line: str) -> list[str]:
        line = line.strip()
        if line.startswith("|"):
            line = line[1:]
        cells, cur, i, n = [], [], 0, len(line)
        while i < n:
            c = line[i]
            if c == "\\" and i + 1 < n and line[i + 1] == "|":
                cur.append("\\|")
                i += 2
                continue
            if c == "`":
                j = i
                while j < n and line[j] == "`":
                    j += 1
                end = line.find("`" * (j - i), j)
                if end >= 0:
                    cur.append(line[i:end + (j - i)])
                    i = end + (j - i)
                    continue
            if c == "|":
                cells.append("".join(cur).strip())
                cur = []
            else:
                cur.append(c)
            i += 1
        if "".join(cur).strip():
            cells.append("".join(cur).strip())
        return cells

    _ITEM = re.compile(r"^( *)([-*+]|\d+[.)])( +|$)")
    _RULE = re.compile(r"^ {0,3}(-{3,}|\*{3,}|_{3,})\s*$")
    _SEP = re.compile(r"^ *\|?( *:?-{1,}:? *\|)+ *:?-{1,}:? *\|? *$|^ *\| *:?-+:? *\| *$")

    def render(self, text: str) -> str:
        return self.blocks(text.splitlines())

    def blocks(self, lines: list[str]) -> str:
        out, i, n = [], 0, len(lines)
        while i < n:
            line = lines[i]
            if not line.strip():
                i += 1
                continue
            fence = re.match(r"^ {0,3}(`{3,}|~{3,})(.*)$", line)
            if fence:
                mark, j = fence.group(1), i + 1
                body = []
                while j < n and not re.match(rf"^ {{0,3}}{re.escape(mark[0])}{{{len(mark)},}}\s*$", lines[j]):
                    body.append(lines[j])
                    j += 1
                lang = fence.group(2).strip()
                cls = f' class="lang-{esc(lang)}"' if lang else ""
                out.append(f"<pre><code{cls}>{esc(chr(10).join(body))}</code></pre>")
                i = j + 1
                continue
            head = re.match(r"^ {0,3}(#{1,6}) +(.*?)\s*#*\s*$", line)
            if head:
                level, text = len(head.group(1)), head.group(2)
                base = slug(text)
                k = self.seen[base]
                self.seen[base] += 1
                anchor = self.prefix + (base if k == 0 else f"{base}-{k}")
                self.headings.append((level, anchor, text))
                out.append(f'<h{level} id="{esc(anchor)}">{self.inline(text)}</h{level}>')
                i += 1
                continue
            if self._RULE.match(line):
                out.append("<hr>")
                i += 1
                continue
            if line.lstrip().startswith("|") and i + 1 < n and self._SEP.match(lines[i + 1]):
                head_cells, aligns = self._cells(line), []
                for c in self._cells(lines[i + 1]):
                    aligns.append(' class="num"' if c.endswith(":") else "")
                rows, j = [], i + 2
                while j < n and lines[j].lstrip().startswith("|"):
                    rows.append(self._cells(lines[j]))
                    j += 1
                t = ['<div class="scroll"><table><thead><tr>']
                t += [f"<th{aligns[k] if k < len(aligns) else ''}>{self.inline(c)}</th>"
                      for k, c in enumerate(head_cells)]
                t.append("</tr></thead><tbody>")
                for r in rows:
                    t.append("<tr>" + "".join(
                        f"<td{aligns[k] if k < len(aligns) else ''}>{self.inline(c)}</td>"
                        for k, c in enumerate(r)) + "</tr>")
                t.append("</tbody></table></div>")
                out.append("".join(t))
                i = j
                continue
            if line.lstrip().startswith(">"):
                body, j = [], i
                while j < n and lines[j].strip() and (lines[j].lstrip().startswith(">") or body):
                    body.append(re.sub(r"^ {0,3}> ?", "", lines[j]))
                    j += 1
                    if j < n and not lines[j].lstrip().startswith(">") and (
                            self._ITEM.match(lines[j]) or re.match(r"^ {0,3}#", lines[j])):
                        break
                out.append(f"<blockquote>{self.blocks(body)}</blockquote>")
                i = j
                continue
            item = self._ITEM.match(line)
            if item:
                indent = len(item.group(1))
                ordered = item.group(2)[0].isdigit()
                items, j = [], i
                while j < n:
                    m = self._ITEM.match(lines[j])
                    if not m or len(m.group(1)) != indent or m.group(2)[0].isdigit() != ordered:
                        break
                    width = len(m.group(0))
                    body = [lines[j][width:]]
                    j += 1
                    while j < n:
                        l = lines[j]
                        if not l.strip():
                            k = j + 1
                            while k < n and not lines[k].strip():
                                k += 1
                            if k < n and len(lines[k]) - len(lines[k].lstrip()) > indent:
                                body.append("")
                                j += 1
                                continue
                            break
                        lead = len(l) - len(l.lstrip())
                        if lead <= indent and (self._ITEM.match(l) or re.match(r"^ {0,3}(#|`{3}|\|)", l)):
                            break
                        if lead <= indent and body and not body[-1].strip():
                            break
                        body.append(l[min(lead, width):])
                        j += 1
                    items.append(body)
                tag = "ol" if ordered else "ul"
                start = ""
                if ordered and item.group(2)[:-1] != "1":
                    start = f' start="{esc(item.group(2)[:-1])}"'
                parts = []
                for body in items:
                    inner = self.blocks(body)
                    if inner.count("<p>") == 1 and inner.startswith("<p>") and inner.endswith("</p>"):
                        inner = inner[3:-4]
                    parts.append(f"<li>{inner}</li>")
                out.append(f"<{tag}{start}>{''.join(parts)}</{tag}>")
                i = j
                continue
            para, j = [line.strip()], i + 1
            while j < n and lines[j].strip():
                l = lines[j]
                if (re.match(r"^ {0,3}(#{1,6} |`{3}|~{3}|>)", l) or self._ITEM.match(l) or self._RULE.match(l)
                        or (l.lstrip().startswith("|") and j + 1 < n and self._SEP.match(lines[j + 1]))):
                    break
                para.append(l.strip())
                j += 1
            out.append(f"<p>{self.inline(chr(10).join(para))}</p>")
            i = j
        return "\n".join(out)


def md_section(text: str, start: int) -> tuple[str, int]:
    """The section whose heading is line `start`: its lines to the next heading of its level
    or above, fences skipped, and the line after it."""
    lines = text.splitlines()
    level = len(re.match(r"^(#+)", lines[start]).group(1))
    j, fenced = start + 1, False
    while j < len(lines):
        if re.match(r"^ {0,3}(```|~~~)", lines[j]):
            fenced = not fenced
        elif not fenced:
            m = re.match(r"^(#{1,6}) ", lines[j])
            if m and len(m.group(1)) <= level:
                break
        j += 1
    return "\n".join(lines[start:j]).rstrip() + "\n", j


def md_headings(text: str) -> list[tuple[int, int, str]]:
    """Every heading of a Markdown text outside a fence: its line, its level, its text."""
    out, fenced = [], False
    for k, line in enumerate(text.splitlines()):
        if re.match(r"^ {0,3}(```|~~~)", line):
            fenced = not fenced
        elif not fenced:
            m = re.match(r"^(#{1,6}) +(.*?)\s*$", line)
            if m:
                out.append((k, len(m.group(1)), m.group(2)))
    return out


# ── the bundle: pages, tables, pictures ──────────────────────────────────────

CSS = """\
:root { color-scheme: light dark; --bg: #ffffff; --fg: #1f2328; --dim: #59636e; --line: #d1d9e0;
  --soft: #f6f8fa; --link: #0969da; --mark: #fff8c5; --dead: #8c959f; --go: #1a7f37; --no: #cf222e; }
@media (prefers-color-scheme: dark) { :root { --bg: #0d1117; --fg: #e6edf3; --dim: #9198a1;
  --line: #3d444d; --soft: #151b23; --link: #4493f8; --mark: #3a2f00; --dead: #7d8590; --go: #3fb950; --no: #f85149; } }
* { box-sizing: border-box; }
body { background: var(--bg); color: var(--fg); margin: 0 auto; padding: 0 16px 64px; max-width: 1500px;
  font: 14px/1.5 system-ui, -apple-system, "Segoe UI", sans-serif; }
a { color: var(--link); text-decoration: none; } a:hover { text-decoration: underline; }
h1 { font-size: 22px; margin: 18px 0 6px; } h2 { font-size: 18px; margin: 34px 0 8px; padding-top: 10px;
  border-top: 1px solid var(--line); } h3 { font-size: 15px; margin: 22px 0 6px; } h4 { font-size: 14px; margin: 16px 0 4px; }
code, pre { font: 12px/1.45 ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
code { background: var(--soft); border-radius: 3px; padding: 0 3px; white-space: nowrap; }
code.j, p code, li code, blockquote code, summary code { white-space: normal; overflow-wrap: anywhere; }
pre { background: var(--soft); border: 1px solid var(--line); border-radius: 6px; padding: 8px 10px; overflow-x: auto; }
pre code { background: none; padding: 0; white-space: pre; }
table { border-collapse: collapse; margin: 6px 0; } th, td { border: 1px solid var(--line); padding: 2px 7px;
  vertical-align: top; text-align: left; } th { background: var(--soft); font-weight: 600; }
td.num, th.num { text-align: right; font-variant-numeric: tabular-nums; } td.num { white-space: nowrap; }
th { max-width: 15em; } body.wide { max-width: none; }
.scroll { overflow-x: auto; max-width: 100%; }
nav.crumbs { padding: 10px 0 0; color: var(--dim); } nav.toc { columns: 3 260px; margin: 8px 0; }
nav.toc a { display: block; }
.what { color: var(--dim); margin: 2px 0 4px; } .raw { color: var(--dim); font-size: 12px; margin: 0 0 4px; }
.note { border-left: 3px solid var(--line); padding: 2px 10px; margin: 8px 0; color: var(--dim); }
.slot { border: 2px dashed var(--no); padding: 8px 12px; margin: 10px 0; }
.script { border-left: 3px solid var(--no); padding: 2px 10px; margin: 8px 0; }
details { margin: 6px 0; } summary { cursor: pointer; } details.prov { background: var(--soft);
  border: 1px solid var(--line); border-radius: 6px; padding: 6px 10px; }
figure { margin: 8px 0; } figure .paper { background: #ffffff; border: 1px solid var(--line); border-radius: 6px;
  padding: 6px; } figure img { display: block; max-width: 100%; height: auto; margin: 0 auto; }
.dead { color: var(--dead); text-decoration: line-through; } .dim { color: var(--dim); }
.go { color: var(--go); font-weight: 600; } .no { color: var(--no); font-weight: 600; }
td.tag { white-space: nowrap; } .tag { border: 1px solid var(--line); border-radius: 10px; padding: 0 7px; font-size: 12px; white-space: nowrap; }
td.cell { min-width: 9em; } table.grid th { white-space: nowrap; } table.matrix td { text-align: center; padding: 1px 4px; font-size: 11px; }
blockquote { border-left: 3px solid var(--line); margin: 8px 0; padding: 0 12px; color: var(--dim); }
th.sort { cursor: pointer; white-space: nowrap; position: sticky; top: 0; } th.sort:hover { text-decoration: underline; }
th.asc::after { content: " ▲"; } th.desc::after { content: " ▼"; }
table.survey td:first-child, table.survey th:first-child { position: sticky; left: 0; background: var(--bg); z-index: 1; }
table.survey thead th:first-child { z-index: 3; background: var(--soft); } table.survey thead th { z-index: 2; }
.controls label { margin-right: 12px; white-space: nowrap; }
"""


class Bundle:
    """What one run of the script knows: where it writes, what it read once, what it wrote."""

    def __init__(self, out: Path, ein: str, jobs: int):
        self.out = out
        self.root = out / "bundle"
        self.ein = ein
        self.jobs = jobs
        self.dots: list[tuple[Path, Path, str]] = []
        self.pages = 0
        self.tables = 0
        self.pictures = 0
        self.undrawn: list[tuple[Path, str]] = []
        self.version = ""
        self.census_env = ""
        self.prices_env = ""

    def env_block(self, path: Path) -> str:
        """A `bench_env.sh` fingerprint: the block between its two rules."""
        lines, inside, block = read_text(path).splitlines(), False, []
        for l in lines:
            if l.startswith("──"):
                block.append(l)
                if inside:
                    break
                inside = True
            elif inside:
                block.append(l)
        if len(block) < 3:
            raise Fail(f"{path}: no `bench_env.sh` fingerprint")
        return "\n".join(block)

    def provenance(self, page_dir: Path, extra: str = "") -> str:
        """Where a page's data came from: the commands, the binary, the censuses' machine state."""
        def link(p: Path) -> str:
            return f'<a href="{esc(rel(p, page_dir))}"><code>{esc(p.relative_to(REPO))}</code></a>'
        env = dict(re.findall(r"^\s+(\w[\w /]*?)\s{2,}(.*)$", self.census_env, re.M))
        head = (f"the census of {esc(env.get('date', '?'))}, tree at <code>{esc(env.get('git', '?'))}</code>, "
                f"{esc(env.get('power', '?'))}")
        return (
            f'<details class="prov"><summary>Where this page\'s data came from: {head}; '
            f'<code>{esc(self.version.splitlines()[0])}</code></summary>'
            f"<p>Written by <code>python3 utils/structure_bundle.py</code> from files the engine wrote; the script "
            f"computes no structure. {extra}</p>"
            f"<p>The structure (the pair layer, the theory's share, the cover, <em>exactly one</em>, the coupling), "
            f"and the three routes beside it: the example <code>pair_layer</code>, into {link(self.out / 'census.jsonl')} "
            f"and each program's directory, with its machine state in {link(self.out / 'census.env')}:</p>"
            f"<pre><code>{esc(CENSUS_COMMANDS)}</code></pre><pre><code>{esc(self.census_env)}</code></pre>"
            f"<p>The price list: the example <code>price_list</code>, into {link(self.out / 'prices')} and each "
            f"program's <code>prices.json</code>, with {link(self.out / 'prices/prices.env')}:</p>"
            f"<pre><code>{esc(PRICES_COMMANDS)}</code></pre><pre><code>{esc(self.prices_env)}</code></pre>"
            f"<p>Track H's censuses: <code>object_census</code>, <code>owed_census</code> and <code>rung_census</code>, "
            f"under {link(REPO / 'build/s1f.10.9')}, {link(REPO / 'build/s1f.10.6')} and {link(REPO / 'build/s1f.10.15')}, "
            f"each with its <code>run.sh</code> and <code>version.txt</code>.</p>"
            f"<p>The binary, <code>{esc(self.ein)} --version</code> as it stands; the examples are binaries of the same "
            f"workspace, built from the tree each fingerprint names:</p><pre><code>{esc(self.version)}</code></pre>"
            f"</details>")

    def write_page(self, path: Path, title: str, crumbs: list[tuple[str, Path | None]], body: str,
                   script: str = "", wide: bool = False) -> None:
        page_dir = path.parent
        page_dir.mkdir(parents=True, exist_ok=True)
        trail = " / ".join(esc(t) if p is None else f'<a href="{esc(rel(p, page_dir))}">{esc(t)}</a>'
                           for t, p in crumbs)
        text = (
            "<!doctype html>\n<html lang=\"en\"><head><meta charset=\"utf-8\">"
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"
            f"<title>{esc(title)}</title>"
            f'<link rel="stylesheet" href="{esc(rel(self.root / "style.css", page_dir))}"></head>\n'
            f'<body{" class=wide" if wide else ""}>\n'
            f'<nav class="crumbs">{trail}</nav>\n{body}\n{script}</body></html>\n')
        path.write_text(text)
        self.pages += 1


class Page:
    """One page being written: its parts in order, its tables' JSON and its pictures' DOT beside it."""

    def __init__(self, bundle: Bundle, directory: Path):
        self.b = bundle
        self.dir = directory
        self.parts: list[str] = []
        self.toc: list[tuple[int, str, str]] = []
        directory.mkdir(parents=True, exist_ok=True)

    def add(self, html_text: str) -> None:
        self.parts.append(html_text)

    def h(self, level: int, anchor: str, title: str) -> None:
        self.toc.append((level, anchor, title))
        self.parts.append(f'<h{level} id="{esc(anchor)}">{esc(title)}</h{level}>')

    def p(self, html_text: str, cls: str = "") -> None:
        self.parts.append(f'<p class="{cls}">{html_text}</p>' if cls else f"<p>{html_text}</p>")

    def link(self, target: Path, text: str | None = None) -> str:
        label = esc(text) if text is not None else f"<code>{esc(target.name)}</code>"
        return f'<a href="{esc(rel(target, self.dir))}">{label}</a>'

    def raw(self, sources: list[tuple[Path, str]]) -> str:
        """*Where the raw file is*: each engine file a table or a picture was read from, with the key."""
        parts = []
        for path, key in sources:
            need(path)
            where = f" § <code>{esc(key)}</code>" if key else ""
            parts.append(f'<a href="{esc(rel(path, self.dir))}"><code>{esc(path.relative_to(REPO))}</code></a>{where}')
        return "; ".join(parts)

    def table(self, name: str, title: str, what: str, sources: list[tuple[Path, str]], head: list[str],
              rows: list[list[C]], fold: bool | None = None, cls: str = "", level: int = 3,
              row_ids: list[str] | None = None) -> None:
        """A table, its JSON beside it (`tables/<name>.json`, the rows as shown), and its header:
        what it is and where the raw file is."""
        twin = self.dir / "tables" / f"{name}.json"
        twin.parent.mkdir(exist_ok=True)
        twin.write_text(json.dumps({
            "what": re.sub(r"<[^>]+>", "", what),
            "read_from": [{"file": str(p.relative_to(REPO)), "key": k} for p, k in sources],
            "columns": head,
            "rows": [[c.v for c in r] for r in rows],
        }, ensure_ascii=False, indent=1) + "\n")
        self.b.tables += 1
        nums = [all(r[k].num or r[k].v is None for r in rows) and any(r[k].num for r in rows)
                for k in range(len(head))] if rows else [False] * len(head)
        t = [f'<div class="scroll"><table class="{cls}"><thead><tr>']
        t += [f'<th{" class=num" if nums[k] else ""}>{esc(c)}</th>' for k, c in enumerate(head)]
        t.append("</tr></thead><tbody>")
        for j, r in enumerate(rows):
            rid = f' id="{esc(row_ids[j])}"' if row_ids and row_ids[j] else ""
            t.append(f"<tr{rid}>" + "".join(f'<td{" class=num" if nums[k] else ""}>{c.h}</td>'
                                            for k, c in enumerate(r)) + "</tr>")
        t.append("</tbody></table></div>")
        if title:
            self.h(level, name, title)
        self.p(what, "what")
        self.p(f"Raw: {self.raw(sources)}. This table as JSON: {self.link(twin)}. {len(rows)} "
               f"row{'' if len(rows) == 1 else 's'}.", "raw")
        if fold is None:
            fold = len(rows) > FOLD_ROWS
        if not rows:
            self.p("No row: the list is empty in the raw file.", "dim")
        elif fold:
            self.add(f'<details><summary>{len(rows)} rows</summary>{"".join(t)}</details>')
        else:
            self.add("".join(t))

    def picture(self, name: str, title: str, what: str, sources: list[tuple[Path, str]], dot: str, edges: int,
                engine: str, level: int = 3, engine_dot: Path | None = None) -> None:
        """A picture: its DOT beside it, its SVG where it is laid out, and its header. `engine_dot`
        is a DOT the engine wrote, drawn as it is; otherwise the DOT is written here."""
        if engine_dot is not None:
            dot_path = engine_dot
            written = f"Its DOT is the engine's own, {self.link(dot_path)}"
        else:
            dot_path = self.dir / f"{name}.dot"
            dot_path.write_text(dot)
            written = f"Its DOT, written by this script from the raw file: {self.link(dot_path)}"
        svg = self.dir / f"{name}.svg"
        if title:
            self.h(level, name, title)
        self.p(what, "what")
        raw = f"Raw: {self.raw(sources)}. " if sources else ""
        if edges > DRAW_BOUND:
            why = (f"not drawn: {edges} edges, over the bound of {DRAW_BOUND} this script lays out "
                   f"(`DOT_LAYOUT_BOUND` of ein-render/tests/dot_wellformed.rs); the DOT is kept, and its parts "
                   f"are drawn where they are within the bound")
            note = self.dir / f"{name}.undrawn.txt"
            note.write_text(why + "\n")
            self.b.undrawn.append((dot_path, why))
            self.p(f"{raw}{written}. <strong>Not drawn</strong>: {edges} edges, over the bound of {DRAW_BOUND} "
                   f"this script lays out ({self.link(note)}).", "raw")
            return
        self.b.dots.append((dot_path, svg, engine))
        self.b.pictures += 1
        command = " ".join(["dot", f"-K{engine}", *LAYOUT_FLAGS[engine], "-Tsvg"])
        self.p(f"{raw}{written}; laid out by Graphviz, <code>{esc(command)}</code>, into {self.link(svg)}. "
               f"{edges} edge{'' if edges == 1 else 's'}. Shown at the page's width; the picture is a link to the "
               f"SVG at its own size, where an edge and a box say what they are on hover.", "raw")
        self.add(f'<figure><div class="paper"><a href="{esc(svg.name)}">'
                 f'<img src="{esc(svg.name)}" alt="{esc(title or name)}"></a></div></figure>')

    def toc_html(self) -> str:
        return '<nav class="toc">' + "".join(
            f'<a href="#{esc(a)}" style="padding-left:{(l - 2) * 14}px">{esc(t)}</a>' for l, a, t in self.toc
        ) + "</nav>"


def dq(s: str) -> str:
    """A DOT string."""
    return '"' + str(s).replace("\\", "\\\\").replace('"', '\\"') + '"'


def layout_for(nodes: int, edges: int) -> str:
    """The layered layout for a small picture, the force-directed one for a large."""
    return "dot" if edges <= 60 and nodes <= 60 else "neato"


#: What each layout is asked beside the DOT. `neato`, with straight edges, and every DOT written
#: here asks `overlap=scale`: Graphviz 16.0.0 removes overlaps differently from one run to the
#: next under `overlap=false` (its fallback where no prism library is linked), in `fdp` and, on a
#: few graphs, in `neato`; and `fdp` with `splines=true` also moves an edge's label. Under either,
#: two runs of this script would not write the same bytes. The engine's own `coupling.dot` sets no
#: `overlap`, so it is asked for on the command line.
LAYOUT_FLAGS = {"dot": [], "neato": ["-Gsplines=line", "-Goverlap=scale"]}

#: The fill of a candidate's box, by the component of the theory's exclusion graph it is in.
FILLS = ["#ddf4ff", "#fff8c5", "#dafbe1", "#ffebe9", "#fbefff", "#fff1e5", "#e6f6f8", "#f0f0f0"]


def draw(bundle: Bundle) -> None:
    """Every DOT laid out. A refusal fails the script, with Graphviz's message under the DOT's path."""
    if shutil.which("dot") is None:
        raise Fail("no `dot` on PATH: Graphviz draws every picture of the bundle")

    def one(job: tuple[Path, Path, str]):
        dot, svg, engine = job
        try:
            r = subprocess.run(["dot", f"-K{engine}", *LAYOUT_FLAGS[engine], "-Tsvg", "-o", str(svg), str(dot)],
                               capture_output=True, text=True, timeout=900)
        except subprocess.TimeoutExpired:
            return dot, f"`dot -K{engine}` did not end in 900 s", ""
        if r.returncode != 0 or not svg.exists() or svg.stat().st_size == 0:
            return dot, r.stderr.strip() or f"`dot -K{engine}` exited {r.returncode} and wrote no SVG", ""
        return dot, "", r.stderr.strip()

    with concurrent.futures.ThreadPoolExecutor(max_workers=bundle.jobs) as pool:
        results = list(pool.map(one, bundle.dots))
    refused = [(d, e) for d, e, _ in results if e]
    for d, _, warning in results:
        if warning:
            print(f"graphviz, of {d.relative_to(REPO)}:\n  " + warning.replace("\n", "\n  "), file=sys.stderr)
    if refused:
        raise Fail("Graphviz refused:\n" + "\n".join(
            f"{d}\n    " + e.replace("\n", "\n    ") for d, e in refused))


# ── one program ──────────────────────────────────────────────────────────────

def fact_args(text: str) -> list[str]:
    """Every name in an argument position of a fact's text, at any depth: the names
    `object_census` reads a candidate's marks by (`examples/shared/marks.rs`, `arg_symbols`)."""
    out, head_next = [], False
    for t in re.findall(r"\(|\)|[^\s()]+", text):
        if t == "(":
            head_next = True
        elif t != ")":
            if head_next:
                head_next = False
            else:
                out.append(t)
    return out


def relation_of(text: str) -> str:
    """A candidate's relation, read off its text: a reading by name, the script's and not the owner's."""
    heads = re.findall(r"\(\s*([^\s()]+)", text)
    if not heads:
        return text
    return heads[1] if heads[0] == "not" and len(heads) > 1 else heads[0]


PROPERTIES = ("bijective", "functional", "injective", "total", "surjective", "total-owed", "surjective-owed")
MARKS = ("K1", "K2", "R1", "R2", "IN", "IR")
MARK_IS = {
    "K1": "a kernel name the program never writes in an argument position (E-1)",
    "K2": "a kernel name the program also writes in an argument position",
    "R1": "a relation's name the program never writes in an argument position (E-7)",
    "R2": "a relation's name the program also writes in an argument position",
    "IN": "a name only an imported module writes (E-54)",
    "IR": "of a relation the main file does not spell (E-54)",
}


class Program:
    """One searching program: its census line and every file the engine wrote of it."""

    FILES = ["pairs.json", "theory.json", "theory.md", "theory.ein", "cover.json", "cover.md",
             "exactly_one.json", "exactly_one.md", "coupling.json", "coupling.md", "coupling.dot",
             "prices.json", "prices.md"]

    def __init__(self, b: Bundle, row: Doc, shared: dict):
        self.b, self.row, self.path = b, row, row["path"]
        self.rel = self.path[:-4]
        self.dir = b.out / self.rel
        for f in self.FILES:
            need(self.dir / f)
        for key, _ in ASKED:
            need(self.dir / "prices" / f"{key}.nodes.jsonl")
        self.pairs = load_json(self.dir / "pairs.json")
        self.theory = load_json(self.dir / "theory.json")
        self.cover = load_json(self.dir / "cover.json")
        self.x1 = load_json(self.dir / "exactly_one.json")
        self.coupling = load_json(self.dir / "coupling.json")
        self.prices = load_json(self.dir / "prices.json")
        self.shared = shared
        self.page_dir = b.root / "p" / self.rel
        self.cands: list[str] = self.pairs["candidates"]
        self.idx = {c: k for k, c in enumerate(self.cands)}
        if len(self.idx) != len(self.cands):
            raise Fail(f"{self.dir / 'pairs.json'}: two candidates of one text")
        cov = self.cover["candidates"]
        if [c["candidate"] for c in cov] != self.cands:
            raise Fail(f"{self.dir / 'cover.json'}: its candidates are not `pairs.json`'s, in its order")
        self.mark = {c["candidate"]: c["at_root"] for c in cov}
        self.groups_of = {c["candidate"]: c["groups"] for c in cov}
        self.questions = self.coupling["questions"]
        groups = self.cover["groups"]
        for g, q in zip(groups, self.questions):
            if q["is"] != "a group" or [m[0] for m in q["members"]] != g["members"]:
                raise Fail(f"{self.dir / 'coupling.json'}: its first questions are not `cover.json`'s groups, in order")
        self.alone_q = {q["members"][0][0]: k for k, q in enumerate(self.questions)
                        if q["is"] != "a group"}
        for e in self.coupling["edges"] + (self.coupling["edges_one_step_back"] or []) + self.coupling["debts"]:
            if "question_ids" not in e:
                raise Fail(f"{self.dir / 'coupling.json'}: an edge names its questions by text alone "
                           f"(no `question_ids`): the census was taken by a `pair_layer` older than the one "
                           f"this bundle reads; re-take it,\n{CENSUS_COMMANDS}")
            if [self.questions[q]["name"] for q in e["question_ids"]] != e["questions"]:
                raise Fail(f"{self.dir / 'coupling.json'}: an edge's `question_ids` do not name its `questions`")

    def f(self, name: str) -> Path:
        return self.dir / name

    def c(self, text: str) -> C:
        """A candidate, struck where it is dead alone at root."""
        mark = self.mark.get(text)
        cls = ' class="dead"' if mark == "dead alone" else ""
        return C(f'<code{cls} title="{esc(mark or "no candidate of H")}">{esc(text)}</code>', text)

    def cs(self, texts, sep: str = "<br>") -> C:
        texts = list(texts)
        return C(sep.join(self.c(t).h for t in texts), texts)

    def g(self, k: int) -> str:
        return f'<a href="#g{k}">g{k}</a>'

    def q(self, k: int, name: bool = True) -> str:
        label = f" <code>{esc(self.questions[k]['name'])}</code>" if name else ""
        return f'<a href="#q{k}">q{k}</a>{label}'

    def questions_of(self, cand: str) -> list[int]:
        return list(self.groups_of[cand]) or ([self.alone_q[cand]] if cand in self.alone_q else [])


def flat(obj, prefix: str = "") -> list[list[C]]:
    """An object of counts as `key | value` rows, a nested object's keys dotted."""
    rows = []
    for k, v in obj.items():
        key = f"{prefix}{k}"
        if isinstance(v, dict) and v and all(not isinstance(x, (list, dict)) or isinstance(x, dict) for x in v.values()):
            rows += flat(v, key + ".")
        else:
            rows.append([K(key), any_cell(v)])
    return rows


def junk_marks(P: Program) -> tuple[dict[str, list[str]], str, dict | None]:
    """S1f.10.9's marks, carried from the names its census lists to the candidates that write
    one: per candidate its sets, a sentence that says how the join stands, and the census row."""
    head, rows, _ = P.shared["h9"]
    row = rows.get(P.path)
    src = REPO / "build/s1f.10.9/census/rows.tsv"
    if row is None:
        return {}, (f"S1f.10.9's census has no row for this program: <code>{esc(src.relative_to(REPO))}</code> "
                    f"does not cover it, and no mark is shown."), None
    if row["n"] != str(len(P.cands)):
        return {}, (f"S1f.10.9's census read {esc(row['n'])} candidates here and this census "
                    f"{len(P.cands)}: another H, and no mark is carried to a candidate."), row
    marks: dict[str, list[str]] = {c: [] for c in P.cands}
    mismatch = []
    for m in MARKS:
        names = set() if row[f"{m}:names"] == "-" else set(row[f"{m}:names"].split(","))
        got = 0
        for c in P.cands:
            hit = relation_of(c) in names if m == "IR" else bool(names & set(fact_args(c)))
            if hit:
                marks[c].append(m)
                got += 1
        if str(got) != row[f"{m}:candidates"]:
            mismatch.append(f"{m}: {got} candidates name one of its names where the census counts "
                            f"{row[f'{m}:candidates']}")
    if mismatch:
        return {}, ("S1f.10.9's census lists the marked names a program and not the marked candidates, and "
                    "carrying its names to this page's candidates does not give its counts (" + "; ".join(map(esc, mismatch))
                    + "): no mark is shown per candidate. The census row is under § Track H.", row)
    marked = sum(1 for v in marks.values() if v)
    return marks, (f"S1f.10.9's census marks {marked} of the {len(P.cands)} candidates (rung "
                   f"<code>{esc(row['rung'])}</code>). The census lists the marked <em>names</em> a program; this "
                   f"column carries each to the candidates that write it in an argument position, and the count "
                   f"of every set is the census's own."), row


def section_h(pg: Page, P: Program) -> None:
    pairs, theory, x1 = P.pairs, P.theory, P.x1
    implies: dict[str, list[str]] = {c: [] for c in P.cands}
    implied_by: dict[str, list[str]] = {c: [] for c in P.cands}
    for a, b in pairs["implies"]:
        implies[a].append(b)
        implied_by[b].append(a)
    cls_of: dict[str, tuple[int, list[str]]] = {}
    for k, cl in enumerate(pairs["equivalent"]):
        for c in cl:
            cls_of[c] = (k, [x for x in cl if x != c])
    theory_n, root_theory_n, puzzle_n, inter_n = Counter(), Counter(), Counter(), Counter()
    for a, b in theory["theory_excluded"]:
        theory_n[a] += 1
        theory_n[b] += 1
    for x in theory["root_excluded"]:
        for c in x["pair"]:
            (root_theory_n if x["of_the_theory"] else puzzle_n)[c] += 1
    for a, b in pairs["interacting"]:
        inter_n[a] += 1
        inter_n[b] += 1
    killed = set(theory["theory_dead_alone"])
    edges_of: dict[int, list[int]] = {}
    for k, e in enumerate(P.coupling["edges"]):
        for q in e["question_ids"]:
            edges_of.setdefault(q, []).append(k)
    back_of: dict[int, list[int]] = {}
    for k, e in enumerate(P.coupling["edges_one_step_back"] or []):
        for q in e["question_ids"]:
            back_of.setdefault(q, []).append(k)
    marks, junk_note, _ = junk_marks(P)
    P.marks = marks
    rows, ids = [], []
    for k, c in enumerate(P.cands):
        gs = P.groups_of[c]
        owed = [g for g in gs if x1["groups"][g]["owed_by"]]
        about = [g for g in gs if x1["groups"][g]["an_unlicensed_instance_is_about_it"]]
        qs = P.questions_of(c)
        es = sorted({e for q in qs for e in edges_of.get(q, [])})
        bs = sorted({e for q in qs for e in back_of.get(q, [])})
        cl = cls_of.get(c)

        def folded(n: int, items: list[str]) -> str:
            if not items:
                return "0"
            return f'<details><summary>{n}</summary>{" ".join(items)}</details>'

        owed_text = ("" if not gs else "yes: " + ", ".join(P.g(g) for g in owed) if owed else "no")
        if about:
            owed_text += "; an unlicensed instance is about " + ", ".join(P.g(g) for g in about)
        rows.append([
            N(k), P.c(c), T(P.mark[c]), T("yes" if c in killed else ""),
            P.cs(implies[c]), P.cs(implied_by[c]),
            H("" if cl is None else f"class {cl[0]}: " + P.cs(cl[1]).h, None if cl is None else {"class": cl[0], "with": cl[1]}),
            N(theory_n[c]), N(root_theory_n[c]), N(puzzle_n[c]), N(inter_n[c]),
            H(", ".join(P.g(g) for g in gs) if gs else "in no group", gs),
            H(owed_text, {"owed": owed, "an_unlicensed_instance_is_about": about}),
            H("<br>".join(P.q(q) for q in qs), qs),
            H(folded(len(es), [f'<a href="#e{e}">e{e}</a>' for e in es]), es),
            H(folded(len(bs), [f'<a href="#b{e}">b{e}</a>' for e in bs]), bs),
            H(" ".join(f'<span class="tag" title="{esc(MARK_IS[m])}">{m}</span>' for m in marks.get(c, [])),
              marks.get(c, []) if marks else None),
        ])
        ids.append(f"h{k}")
    pg.h(2, "h", "H listed")
    pg.table(
        "h", "", "Every candidate of H, in the universe's order, with what root says of it alone, what it implies and "
        "is equivalent to, the pairs it is in, its groups, whether any is owed, its questions and the edges those are "
        "in, and S1f.10.9's mark. <strong>A join, by the candidate's text, of the engine's lists</strong>; nothing is "
        "computed here. A candidate dead alone at root is struck through on every table of this page.",
        [(P.f("pairs.json"), "candidates, implies, equivalent, interacting"), (P.f("cover.json"), "candidates"),
         (P.f("theory.json"), "theory_excluded, root_excluded, theory_dead_alone"),
         (P.f("exactly_one.json"), "groups"), (P.f("coupling.json"), "questions, edges, edges_one_step_back"),
         (REPO / "build/s1f.10.9/census/rows.tsv", P.path)],
        ["#", "candidate", "alone, at root", "the theory kills it alone", "implies", "implied by", "equivalent to",
         "pairs the theory excludes it in (all of H)", "pairs root excludes it in: the theory's", "… the puzzle's",
         "pairs it interacts in", "groups", "is any owed", "questions", "edges through its questions",
         "edges one step back", "S1f.10.9's mark"],
        rows, fold=len(rows) > 150, row_ids=ids)
    pg.p(junk_note + " The sets: " + "; ".join(f"<code>{m}</code>, {esc(t)}" for m, t in MARK_IS.items()) + ".", "note")
    lists = [("at_root", "held by root's own fixpoint"), ("dead_alone", "dead alone"),
             ("against_l1", "a pair alive over a member dead alone (a flagged program's)"),
             ("against_l3", "a pair whose node is not its implying member's (a flagged program's)")]
    rows = [[K(k), T(what), N(len(pairs[k])), any_cell(pairs[k])] for k, what in lists]
    pg.table("h_lists", "The pair layer's other lists", "The lists of `pairs.json` the table above does not hold a "
             "column for whole, and the equivalence classes.".replace("`pairs.json`", "<code>pairs.json</code>"),
             [(P.f("pairs.json"), "at_root, dead_alone, against_l1, against_l3, equivalent")],
             ["list", "what it is", "entries", "the list"],
             rows + [[K(f"equivalent[{k}]"), T("one closure: an equivalence class"), N(len(cl)), P.cs(cl)]
                     for k, cl in enumerate(pairs["equivalent"])], fold=True)
    counts = {k: v for k, v in P.row.items() if k not in ("theory", "routes", "path", "lists")}
    pg.table("h_counts", "The pair layer's counts", "The census line's own names for the pair layer; "
             "<code>pair_layer.rs</code>'s header says what each is.",
             [(P.b.out / "census.jsonl", P.path), (REPO / "ein.rs/crates/ein-infer/examples/pair_layer.rs", "")],
             ["name", "value"], flat(counts), fold=True)


def support_ids(P: Program) -> dict[str, str]:
    """An id for every statement the small KB drops, in the order the files state them."""
    ids: dict[str, str] = {}
    for s in P.theory["stated"]:
        if not s["kept"] and s["fact"] not in ids:
            ids[s["fact"]] = f"s{len(ids) + 1}"
    return ids


def fact_of(named: str) -> str:
    """The fact a support names: the example writes a statement with its `:source` after it."""
    return re.sub(r' :source ".*"$', "", named)


def label_of(ids: list[str]) -> str:
    """Statement ids as an edge's label, six a line."""
    return "\\n".join(" ".join(ids[k:k + 6]) for k in range(0, len(ids), 6))


THEORY_STROKE = 'color="#0969da", penwidth=1.3'
THEORY_DEAD_STROKE = 'color="#0969da", style=dotted'
PUZZLE_STROKE = 'color="#cf222e", style=dashed, fontcolor="#cf222e", fontsize=8'


def exclusion_dot(P: Program, title: str, keys: list[tuple[str, str]], E: dict, sids: dict[str, str],
                  parts: dict[str, int], sides: tuple[set, set] | None = None) -> tuple[str, int, int]:
    """The DOT of some of the excluded pairs: the theory's a solid stroke, the puzzle's a dashed one
    labelled with the statements its support names."""
    used = sorted({c for key in keys for c in key}, key=P.idx.__getitem__)
    lines = ["graph exclusion {",
             f"  graph [label={dq(title)}, labelloc=t, fontsize=11, fontname=\"Helvetica\", overlap=scale"
             + (", rankdir=LR, ranksep=1.6" if sides else "") + "];",
             '  node [shape=box, fontsize=9, fontname="Helvetica", margin="0.06,0.03", height=0.25];',
             '  edge [fontname="Helvetica"];']

    def node(c: str) -> str:
        dead = P.mark[c] == "dead alone"
        part = parts.get(c, -1)
        fill = FILLS[part % len(FILLS)] if part >= 0 else "#ffffff"
        style = f', style="filled,dashed", color="#8c959f", fontcolor="#6e7781"' if dead else ', style=filled'
        where = f"component {part}" if part >= 0 else "in no pair of the theory"
        return f"n{P.idx[c]} [label={dq(c)}{style}, fillcolor={dq(fill)}, tooltip={dq(P.mark[c] + '; ' + where)}];"

    lines += ["  " + node(c) for c in used]
    for a, b in keys:
        e = E[(a, b)]
        if sides and a not in sides[0]:
            a, b = b, a
        if e["theory"]:
            stroke = THEORY_STROKE if e["root"] else THEORY_DEAD_STROKE
            tip = "the theory's" + ("" if e["root"] else "; root's layer does not list it as excluded")
            lines.append(f"  n{P.idx[a]} -- n{P.idx[b]} [{stroke}, tooltip={dq(tip)}];")
        else:
            s = e["support"]
            named = [sids[fact_of(f)] for f in s["dropped"]] if s and s["read"] else []
            label = label_of(named) if named else "?"
            tip = "the puzzle's, by: " + "; ".join(s["dropped"]) if named else "the puzzle's; no support read"
            lines.append(f"  n{P.idx[a]} -- n{P.idx[b]} [{PUZZLE_STROKE}, label={dq(label).replace(chr(92) * 2 + 'n', chr(92) + 'n')}, "
                         f"tooltip={dq(tip)}];")
    lines.append("}")
    return "\n".join(lines) + "\n", len(used), len(keys)


def section_exclusion(pg: Page, P: Program) -> None:
    theory = P.theory
    sids = support_ids(P)
    E: dict[tuple[str, str], dict] = {}

    def key(a: str, b: str) -> tuple[str, str]:
        return (a, b) if P.idx[a] < P.idx[b] else (b, a)

    for a, b in theory["theory_excluded"]:
        E[key(a, b)] = {"theory": True, "root": False, "support": None}
    for x in theory["root_excluded"]:
        k = key(*x["pair"])
        e = E.setdefault(k, {"theory": False, "root": False, "support": None})
        if e["theory"] != x["of_the_theory"]:
            raise Fail(f"{P.f('theory.json')}: {k} is `of_the_theory` {x['of_the_theory']} under `root_excluded` "
                       f"and {'in' if e['theory'] else 'not in'} `theory_excluded`")
        e["root"], e["support"] = True, x.get("support")
        if e["support"] and e["support"]["read"]:
            for fact in e["support"]["dropped"]:
                if fact_of(fact) not in sids:
                    raise Fail(f"{P.f('theory.json')}: the support of {k} names {fact!r} as dropped, "
                               f"which `stated` does not list as dropped")
    keys = sorted(E, key=lambda k: (P.idx[k[0]], P.idx[k[1]]))
    theory_keys = [k for k in keys if E[k]["theory"]]
    puzzle_keys = [k for k in keys if not E[k]["theory"]]
    labelled = sum(1 for k in puzzle_keys if E[k]["support"] and E[k]["support"]["read"])

    pg.h(2, "exclusion", "The exclusion graph")
    by_member = sum(1 for k in theory_keys if not E[k]["root"] and "dead alone" in (P.mark[k[0]], P.mark[k[1]]))
    unlisted = sum(1 for k in theory_keys if not E[k]["root"]) - by_member
    pg.p(f"{len(theory_keys)} pairs the theory excludes alone (of them {sum(1 for k in theory_keys if E[k]['root'])} "
         f"root excludes too, and {by_member} hold a member dead alone at root, "
         f"so that root's node of the pair is dead by that member and is no exclusion there"
         + (f"; {unlisted} more root's layer does not list as excluded though neither member is dead alone" if unlisted else "")
         + f"), and {len(puzzle_keys)} "
         f"pairs root excludes that the theory does not: the puzzle's. <strong>Two strokes</strong>: a pair of the "
         f"theory is a solid blue line, dotted where root's layer does not list it as excluded; a pair of the puzzle is a dashed "
         f"red line labelled with the statements its support names, by the ids of the table "
         f"<a href=\"#statements\">below</a> ({labelled} of {len(puzzle_keys)} have a support read; a support is "
         f"sufficient, not least, and not canonical). A candidate dead alone at root is a grey dashed box. A "
         f"candidate in no pair is not drawn: {len(P.cands) - len({c for k in keys for c in k})} of "
         f"{len(P.cands)}.")
    if P.pairs["domain"]:
        pg.p(esc(P.pairs["domain"]), "note")
    src = [(P.f("theory.json"), "theory_excluded, root_excluded"), (P.f("cover.json"), "components, candidates")]
    parts = {c: k for k, comp in enumerate(P.cover["components"]) for c in comp}
    if not keys:
        pg.p("No pair is excluded, by the theory or at root: there is nothing to draw.", "note")
    else:
        dot, nodes, edges = exclusion_dot(P, f"{P.path}: the excluded pairs", keys, E, sids, parts)
        pg.picture("exclusion", "Every excluded pair", "The theory's pairs and the puzzle's together. A box's "
                   "fill is its connected component of the theory's exclusion graph, white where it is in none.",
                   src, dot, edges,
                   layout_for(nodes, edges))
        if theory_keys and puzzle_keys:
            dot, nodes, edges = exclusion_dot(P, f"{P.path}: the theory's pairs", theory_keys, E, sids, parts)
            pg.picture("exclusion.theory", "The theory's pairs alone", "The stroke of the theory, without the "
                       "puzzle's: the graph a group is a maximal clique of.", src, dot, edges, layout_for(nodes, edges))
            dot, nodes, edges = exclusion_dot(P, f"{P.path}: the puzzle's pairs", puzzle_keys, E, sids, parts)
            pg.picture("exclusion.puzzle", "The puzzle's pairs alone", "The stroke of the puzzle, each pair "
                       "labelled with the statements its support names.", src, dot, edges, layout_for(nodes, edges))
        if len(keys) > SPLIT_BOUND:
            comps = sorted({parts.get(c, -1) for k in keys for c in k})
            pg.h(3, "exclusion.parts", "The same pairs, in parts")
            pg.p(f"The whole graph has {len(keys)} edges, more than {SPLIT_BOUND}, so it is also drawn in parts: the "
                 f"pairs inside each component of the theory's exclusion graph, and the pairs between each two "
                 f"components. <em>Component −</em> is the candidates in no pair of the theory.", "what")
            name = {k: (f"c{k}" if k >= 0 else "c-") for k in comps}
            for a in comps:
                inside = [k for k in keys if parts.get(k[0], -1) == a and parts.get(k[1], -1) == a]
                if inside:
                    dot, nodes, edges = exclusion_dot(P, f"{P.path}: the pairs inside component {name[a][1:]}",
                                                      inside, E, sids, parts)
                    pg.picture(f"exclusion.{name[a]}", f"Inside component {name[a][1:]}",
                               f"{sum(1 for k in inside if E[k]['theory'])} of the theory, "
                               f"{sum(1 for k in inside if not E[k]['theory'])} of the puzzle.", [], dot, edges,
                               layout_for(nodes, edges), level=4)
            for i, a in enumerate(comps):
                for bb in comps[i + 1:]:
                    across = [k for k in keys if {parts.get(k[0], -1), parts.get(k[1], -1)} == {a, bb}]
                    if across:
                        left = {c for k in across for c in k if parts.get(c, -1) == a}
                        right = {c for k in across for c in k if parts.get(c, -1) == bb}
                        dot, nodes, edges = exclusion_dot(
                            P, f"{P.path}: the pairs between components {name[a][1:]} and {name[bb][1:]}",
                            across, E, sids, parts, (left, right))
                        pg.picture(f"exclusion.{name[a]}-{name[bb]}",
                                   f"Between components {name[a][1:]} and {name[bb][1:]}",
                                   f"{len(across)} pairs, every one the puzzle's: the theory excludes no pair "
                                   f"across two of its components.", [], dot, edges,
                                   "dot" if edges <= 250 else "neato", level=4)

    rows = []
    for a, b in keys:
        e = E[(a, b)]
        s = e["support"]
        dead = [c for c in (a, b) if P.mark[c] == "dead alone"]
        rows.append([
            P.c(a), P.c(b), T("the theory" if e["theory"] else "the puzzle"),
            H("yes" if e["root"] else ("no: dead by a member dead alone, " + P.cs(dead, ", ").h if dead
                                       else "no, and neither member is dead alone at root"), e["root"]),
            H(" ".join(f'<a href="#st-{sids[fact_of(f)]}">{sids[fact_of(f)]}</a>' for f in s["dropped"]) if s else "",
              [sids[fact_of(f)] for f in s["dropped"]] if s else None),
            KS(s["dropped"]) if s else T(""), KS(s["kept"]) if s else T(""), KS(s["rules"]) if s else T(""),
            T(s["status"]) if s else T(""),
            key_values(s, skip=("dropped", "kept", "rules", "status", "commitments")) if s else T(""),
        ])
    pg.table("pairs", "Every excluded pair, with its support", "One row a pair of the pictures above. "
             "<em>By</em> is the support's statements the small KB drops, the puzzle's; <em>kept</em> those it "
             "keeps; the rules are the ones that end the derivation. A support is read for a pair root excludes, "
             "so a pair of the theory with a member dead alone at root has none.",
             [(P.f("theory.json"), "theory_excluded, root_excluded[].support")],
             ["a", "b", "of", "root excludes it", "by", "by: the statements", "kept statements", "rules",
              "the support's search", "the check, and the rest"], rows, fold=len(rows) > FOLD_ROWS)
    rows, ids = [], []
    for s in theory["stated"]:
        sid = sids.get(s["fact"]) if not s["kept"] else None
        rows.append([T(sid or ""), K(s["fact"]), T("kept" if s["kept"] else "dropped: the puzzle's"),
                     T(s["clause"]), T(s["source"]), T(s["origin"]), N(s["read_absent"])]
                    + [any_cell(v) for k, v in s.items() if k not in ("fact", "kept", "clause", "source", "origin", "read_absent")])
        ids.append(f"st-{sid}" if sid else "")
    seen = set()
    ids = [i if i and i not in seen and not seen.add(i) else "" for i in ids]
    extra = [k for k in (theory["stated"][0] if theory["stated"] else {}) if k not in
             ("fact", "kept", "clause", "source", "origin", "read_absent")]
    pg.table("statements", "What the program states, and what the small KB keeps of it",
             "Every fact the program's files write, with whether the small KB keeps it and by which clause of "
             "S1f.10.10's rule. A dropped statement has the id the pictures label a puzzle's pair with.",
             [(P.f("theory.json"), "stated"), (P.f("theory.ein"), "")],
             ["id", "fact", "the small KB", "clause", ":source", "origin", "of a relation a rule reads the absence of"]
             + extra, rows, fold=len(rows) > FOLD_ROWS, row_ids=ids)
    share = {k: v for k, v in P.row["theory"].items() if k not in ("cover", "coupling")}
    pg.table("theory_counts", "The theory's share, counted", "The census line's <code>theory</code> object, without "
             "the cover and the coupling: the small KB, its layer, the supports and their check, "
             "<code>theory.ein</code>.", [(P.b.out / "census.jsonl", f"{P.path} § theory")], ["name", "value"],
             flat(share), fold=True)
    rows = [[P.c(a), P.c(b)] for a, b in P.pairs["interacting"]]
    pg.table("interacting", "The pairs that interact", "Two candidates alive alone whose pair is alive and says "
             "more than its two singles do: no exclusion, and a tie of the coupling below.",
             [(P.f("pairs.json"), "interacting")], ["a", "b"], rows, fold=True)


def stated_of(P: Program, relation: str) -> list[str]:
    """What the program states of a relation by one of the property names: a reading by name."""
    out = []
    for s in P.theory["stated"]:
        f = s["fact"].strip("()").split()
        if len(f) >= 2 and f[1] == relation and f[0].split(".")[-1] in PROPERTIES:
            out.append(s["fact"])
    return out


def mechanisms_of(P: Program, relation: str) -> list[str]:
    """What `cover.json`'s `declared` says of a relation: the engine's reading of the mechanisms at root."""
    d = P.cover["declared"]
    out = []
    for key, what in (("proposed", "an active hrule proposes it"), ("owable", "an obligation of root's can owe it"),
                      ("closed", "a closure takes it out"), ("excluded", "`:no-hypothesis` takes it out"),
                      ("listed", "`:hypothesis-relations` lists it")):
        if relation in (d.get(key) or []):
            out.append(what)
    return out


def section_cover(pg: Page, P: Program) -> None:
    cover, x1 = P.cover, P.x1
    pg.h(2, "cover", "The cover")
    pg.p(f"A group is a maximal set of two or more candidates that pairwise exclude by the theory alone: <em>pick at "
         f"most one</em>. {len(cover['groups'])} groups over {len(P.cands)} candidates, "
         f"{len(cover['in_no_group'])} candidates in no group, {len(cover['components'])} connected components of "
         f"the theory's exclusion graph. The groups are a cover and not a partition: a candidate is in as many as "
         f"hold it.")
    if cover["domain"]:
        pg.p(esc(cover["domain"]), "note")
    rows, ids = [], []
    for g, grp in enumerate(cover["groups"]):
        xg = x1["groups"][g]
        if xg["members"] != grp["members"]:
            raise Fail(f"{P.f('exactly_one.json')}: group {g}'s members are not `cover.json`'s")
        rels = sorted({relation_of(m) for m in grp["members"]})
        declared = []
        for r in rels:
            what = mechanisms_of(P, r) + [f"<code>{esc(f)}</code>" for f in stated_of(P, r)]
            declared.append(f"<code>{esc(r)}</code>: " + ("; ".join(w if w.startswith("<") else esc(w).replace("`", "")
                                                                       for w in what) or "nothing"))
        grid = xg["grid"]
        owed = xg["owed_by"]
        about = xg["an_unlicensed_instance_is_about_it"]
        rules = sorted({P.rule_of[i] for i in owed})
        rows.append([
            H(f"g{g}", g), K(P.questions[g]["name"]), N(len(grp["members"])), N(len(grp["open"])),
            P.cs(grp["members"]),
            H(("<strong>owed</strong>, by " + ", ".join(f"<code>{esc(r)}</code>" for r in rules)) if owed
              else "nothing owes it", bool(owed)),
            H("licensed by L5: " + KS(owed).h if owed else
              ("no licensed instance; an instance L5 does not license is about it: " + KS(about).h if about
               else "no instance, licensed or not"), {"owed_by": owed, "an_unlicensed_instance_is_about_it": about}),
            H("<br>".join(declared), {r: {"mechanisms": mechanisms_of(P, r), "stated": stated_of(P, r)} for r in rels}),
            H("" if grid is None else f'<a href="#grid{grid["grid"]}">grid {grid["grid"]}</a>, family {grid["family"]}',
              grid),
        ])
        ids.append(f"g{g}")
    pg.table("groups", "Every group", "Its members with their marks, the open ones those alive alone at root; "
             "<em>owed</em>, whether its members are exactly the alternatives of an instance root owes; "
             "<em>licensed</em>, whether L5 licenses that instance; <em>declared by</em>, what the program says of "
             "the relation its members are of. <strong>The relation is read off the members' text, and the "
             "statements <code>(bijective R)</code> and its halves by their names: the script's readings</strong>, "
             "as S1f.10.4's <code>bijections.py</code> makes them. The mechanisms are the engine's "
             "(<code>cover.json</code> § <code>declared</code>). A group's name is its question's.",
             [(P.f("cover.json"), "groups, declared"), (P.f("exactly_one.json"), "groups, instances"),
              (P.f("coupling.json"), "questions[].name"), (P.f("theory.json"), "stated")],
             ["group", "its question's name", "members", "open", "the members", "owed", "licensed", "declared by",
              "in a grid"], rows, row_ids=ids)

    grid_of_component = {g["component"]: g for g in x1["grids"]}
    not_grid = {c["component"]: c for c in x1["components_that_are_no_grid"]}
    for g in x1["grids"]:
        if g["grid"] == 0:
            pg.h(3, "grids", "The grids")
            pg.p("A grid is a component of the theory's exclusion graph in which every candidate is in exactly two "
                 "groups, the groups in two families, no two groups of one family sharing a member and two groups "
                 "sharing at most one. Drawn as it is: one family the rows, the other the columns, a member the cell "
                 "its row and its column share. A <strong>bijection</strong> is a grid with as many rows as columns, "
                 "every one of them owed. The <code>relations</code> are read off the members by the example, for the "
                 "reader; the owner reads none.", "what")
        fam = g["families"]
        rows_g, cols_g = fam[0]["groups"], fam[1]["groups"]

        def head(k: int) -> str:
            xg = x1["groups"][k]
            owed = "owed" if xg["owed_by"] else ("not owed; an unlicensed instance is about it"
                                                 if xg["an_unlicensed_instance_is_about_it"] else "not owed")
            return f'{P.g(k)} <code>{esc(P.questions[k]["name"])}</code><br><span class="dim">{owed}</span>'

        body, twin = [], []
        for r in rows_g:
            line, trow = [f"<th>{head(r)}</th>"], []
            for c in cols_g:
                shared = [m for m in cover["groups"][r]["members"] if m in set(cover["groups"][c]["members"])]
                if len(shared) > 1:
                    raise Fail(f"{P.f('exactly_one.json')}: grid {g['grid']}'s groups {r} and {c} share "
                               f"{len(shared)} members, where two groups of a grid share at most one")
                line.append(f'<td class="cell">{P.c(shared[0]).h if shared else ""}</td>')
                trow.append(shared[0] if shared else None)
            body.append("<tr>" + "".join(line) + "</tr>")
            twin.append(trow)
        twin_path = pg.dir / "tables" / f"grid{g['grid']}.json"
        twin_path.parent.mkdir(exist_ok=True)
        twin_path.write_text(json.dumps({
            "what": "a grid as rows and columns: a cell is the member its row's group and its column's group share",
            "read_from": [{"file": str(P.f("exactly_one.json").relative_to(REPO)), "key": f"grids[{g['grid']}]"},
                          {"file": str(P.f("cover.json").relative_to(REPO)), "key": "groups"}],
            "grid": g, "rows": rows_g, "columns": cols_g, "cells": twin}, ensure_ascii=False, indent=1) + "\n")
        pg.b.tables += 1
        kind = "<strong>a bijection</strong>" if g["bijection"] else "no bijection"
        pg.h(4, f"grid{g['grid']}", f"Grid {g['grid']}")
        pg.p(f"{kind}: {esc(g['kind'])}. Component {g['component']}; {g['members']} members, {g['open']} alive "
             f"alone at root; {len(rows_g)} rows ({fam[0]['owed']} owed, an unlicensed instance about "
             f"{fam[0]['an_unlicensed_instance_is_about']}) by {len(cols_g)} columns ({fam[1]['owed']} owed, "
             f"about {fam[1]['an_unlicensed_instance_is_about']}); {g['cells']} cells; relations "
             + ", ".join(f"<code>{esc(r)}</code>" for r in g["relations"]) + ".", "what")
        pg.p(f"Raw: {pg.raw([(P.f('exactly_one.json'), 'grids[%d]' % g['grid']), (P.f('cover.json'), 'groups')])}. "
             f"This grid as JSON: {pg.link(twin_path)}.", "raw")
        pg.add('<div class="scroll"><table class="grid"><thead><tr><th></th>'
               + "".join(f"<th>{head(c)}</th>" for c in cols_g) + "</tr></thead><tbody>" + "".join(body)
               + "</tbody></table></div>")
    if not x1["grids"]:
        pg.h(3, "grids", "The grids")
        pg.p("No component of the theory's exclusion graph is a grid.", "note")
    rows = []
    for k, comp in enumerate(cover["components"]):
        grid, no = grid_of_component.get(k), not_grid.get(k)
        if grid is None and no is None:
            raise Fail(f"{P.f('exactly_one.json')}: component {k} is neither a grid nor a component that is none")
        its = (grid["families"][0]["groups"] + grid["families"][1]["groups"]) if grid else no["groups"]
        rows.append([
            N(k), N(len(comp)),
            H(f'<a href="#grid{grid["grid"]}">grid {grid["grid"]}</a>' if grid else "no grid", bool(grid)),
            T("" if grid else no["why"]),
            any_cell(None if grid else no["detail"]),
            H(", ".join(P.g(x) for x in its), its),
            KS((grid or no)["relations"], ", "), P.cs(comp, " "),
        ])
    pg.table("components", "The components of the theory's exclusion graph",
             "Each is a grid, or is none with the engine's reason.",
             [(P.f("cover.json"), "components"), (P.f("exactly_one.json"), "grids, components_that_are_no_grid")],
             ["component", "members", "is", "why it is no grid", "detail", "its groups", "relations", "the members"],
             rows)
    rows = [[P.c(c), T(P.mark[c]), N(c in set(P.theory["theory_dead_alone"]))] for c in cover["in_no_group"]]
    pg.table("in_no_group", "The candidates in no group", "No pair of the theory holds them: nothing says "
             "<em>at most one</em> of any of them with another.", [(P.f("cover.json"), "in_no_group")],
             ["candidate", "alone, at root", "the theory kills it alone"], rows, fold=len(rows) > FOLD_ROWS)
    rows = [[P.c(a), P.c(b)] for a, b in cover["pairs_alive_at_root"]]
    pg.table("alive_at_root", "The theory's pairs alive at root", "A pair the theory excludes whose node is alive "
             "at root would break <em>at most one</em>; the list is empty on a program the small KB fits.",
             [(P.f("cover.json"), "pairs_alive_at_root")], ["a", "b"], rows)
    cliques = cover["root_cliques"]
    if cliques and cliques["cliques"]:
        sub = Page(pg.b, pg.dir)
        rows = [[N(k), N(len(cl)), P.cs(cl, " ")] for k, cl in enumerate(cliques["cliques"])]
        sub.h(1, "top", f"{P.path}: the maximal cliques of root's exclusion graph")
        sub.table("root_cliques", "", "The maximal cliques of the graph of the pairs <strong>root</strong> excludes: "
                  "the definition of a group as first planned, which the control falsified on <code>zebra2.ein</code>. "
                  "Counted and listed beside the cover, and not called groups.",
                  [(P.f("cover.json"), "root_cliques")], ["#", "members", "the clique"], rows, fold=False)
        pg.b.write_page(pg.dir / "root_cliques.html", f"{P.path}: root's cliques",
                        [("the structure, shown", pg.b.out / "index.html"), (P.path, pg.dir / "index.html"),
                         ("root's cliques", None)], pg.b.provenance(pg.dir) + "\n".join(sub.parts))
        pg.p(f"<strong>Root's cliques</strong>, beside the cover and not groups: {len(cliques['cliques'])} maximal "
             f"cliques of root's own exclusion graph, {cliques['steps']} calls of the enumeration, cut "
             f"{any_cell(cliques['cut']).h or 'nothing'}. Listed on {pg.link(pg.dir / 'root_cliques.html', 'their own page')}.")
    else:
        pg.p("<strong>Root's cliques</strong>: root's exclusion graph has no maximal clique of two or more "
             "(<code>cover.json</code> § <code>root_cliques</code>).")
    counts = {k: v for k, v in P.row["theory"]["cover"].items() if k not in ("exactly_one", "declared", "owed")}
    pg.table("cover_counts", "The cover, counted", "The census line's <code>theory.cover</code> object: sizes as "
             "<code>[size, groups]</code>, membership as <code>[k, candidates in exactly k groups]</code>, the "
             "enumeration's calls against its bound and what the bound cut.",
             [(P.b.out / "census.jsonl", f"{P.path} § theory.cover")], ["name", "value"], flat(counts), fold=True)


def section_exactly_one(pg: Page, P: Program) -> None:
    x1 = P.x1
    pg.h(2, "exactly_one", "Exactly one")
    table = (f"L5's table lists {len(x1['instances'])} instances" if x1["no_table"] is None
             else f"no table is read: {esc(x1['no_table'])}")
    pg.p(f"Root owes {x1['root_owes']} instances; {table}. An <strong>owed group</strong> is a group whose members "
         f"are exactly the alternatives of an instance L5 licenses: every state of R holds exactly one of its open "
         f"members. {len(x1['owed_groups'])} owed groups, {len(x1['groups_nothing_owes'])} groups nothing owes, "
         f"{len(x1['owed_sets_not_a_group'])} owed sets that are not a group (<em>at least one</em>, with no "
         f"<em>at most one</em>), {len(x1['unlicensed'])} instances L5 does not license.")
    if x1["domain"]:
        pg.p(esc(x1["domain"]), "note")
    known = ("instance", "rule", "licensed", "standing", "group", "alternatives", "witnesses",
             "witnesses_denied_at_root", "why", "about_group", "pay_by_the_theory_alone")
    rows, ids = [], []
    for k, i in enumerate(x1["instances"]):
        ws = i["witnesses"]
        if isinstance(ws, list):
            wh = "<br>".join(
                f"{P.c(w['fact']).h} <span class=\"dim\">{esc(w['is'])}</span>"
                + (f"; held at root by {P.cs(w['held_at_root_by'], ', ').h}" if w["held_at_root_by"] else "")
                + (f"; in the theory by {P.cs(w['held_in_the_theory_by'], ', ').h}" if w["held_in_the_theory_by"] else "")
                for w in ws)
        else:
            wh = any_cell(ws).h
        group = i.get("group")
        about = i.get("about_group")
        rows.append([
            N(k), K(i["rule"]), K(i["instance"]),
            H("licensed" if i["licensed"] else "<strong>not licensed</strong>", i["licensed"]), T(i["standing"]),
            H("" if group is None else P.g(group), group),
            P.cs(i["alternatives"]) if "alternatives" in i else T(""),
            H(wh, ws), N(i["witnesses_denied_at_root"]),
            T(i.get("why")),
            H("" if about is None else P.g(about), about),
            P.cs(i["pay_by_the_theory_alone"]) if "pay_by_the_theory_alone" in i else T(""),
            key_values({a: b for a, b in i.items() if a not in known}),
        ])
        ids.append(f"i{k}")
    pg.table("instances", "Every instance root owes", "With its alternatives where L5 licenses it, or L5's reason "
             "where it does not and the group it is <em>about</em>: the group whose members are the candidates whose "
             "own node in the small KB holds one of its witnesses. That is a reading and licenses nothing. The "
             "witnesses are every fact that can witness the instance, with what root says of each and the "
             "candidates whose own node holds it.", [(P.f("exactly_one.json"), "instances")],
             ["#", "rule", "instance", "L5", "standing", "its group", "alternatives", "witnesses",
              "witnesses denied at root", "why not licensed", "about group", "payers by the theory alone",
              "the rest"], rows, fold=len(rows) > FOLD_ROWS, row_ids=ids)
    rows = [
        [T("owed groups"), H(", ".join(P.g(g) for g in x1["owed_groups"]), x1["owed_groups"])],
        [T("groups nothing owes"), H(", ".join(P.g(g) for g in x1["groups_nothing_owes"]), x1["groups_nothing_owes"])],
        [T("owed sets that are not a group (instances)"),
         H(", ".join(f'<a href="#i{k}">i{k}</a>' if isinstance(k, int) else any_cell(k).h
                     for k in x1["owed_sets_not_a_group"]), x1["owed_sets_not_a_group"])],
        [T("instances L5 does not license"),
         H(", ".join(f'<a href="#i{k}">i{k}</a>' for k in x1["unlicensed"]), x1["unlicensed"])],
    ]
    pg.table("exactly_one_lists", "The three kinds, listed", "The groups by whether something owes them, the owed "
             "sets that are no group, and the unlicensed instances, as lists of the tables above.",
             [(P.f("exactly_one.json"), "owed_groups, groups_nothing_owes, owed_sets_not_a_group, unlicensed")],
             ["kind", "which"], rows)
    pg.table("exactly_one_counts", "Exactly one, counted", "The census line's "
             "<code>theory.cover.exactly_one</code> and <code>theory.cover.owed</code> objects.",
             [(P.b.out / "census.jsonl", f"{P.path} § theory.cover.exactly_one, theory.cover.owed")],
             ["name", "value"], flat({"exactly_one": P.row["theory"]["cover"]["exactly_one"],
                                      "owed": P.row["theory"]["cover"]["owed"]}), fold=True)


EDGE_STROKES = {
    # kind, whether the node gave it, whether every debt of it is licensed
    "pairs": ('color="#0969da"', "solid blue: a conflict or a derivation the pair layer's ties give"),
    "node": ('color="#1f2328", penwidth=1.4', "solid black: a conflict or a derivation the whole node gives"),
    "debt": ('color="#8250df", style=dashed', "dashed violet: a debt, of instances L5 licenses"),
    "debt-unlicensed": ('color="#8250df", style=dotted, penwidth=1.6',
                        "dotted violet: a debt, of an instance L5 does not license"),
    "both": ('color="#bc4c00", penwidth=1.4', "solid orange: a conflict or a derivation, and a debt"),
}


def edge_stroke(P: Program, e: Doc) -> str:
    licensed = {d["instance"]: d["licensed"] for d in P.coupling["debts"]}
    if e["kind"] == "a debt":
        return "debt" if all(licensed[i] for i in e["debts"]) else "debt-unlicensed"
    if e["kind"] != "a conflict or a derivation":
        return "both"
    return "node" if any(e["node"].values()) else "pairs"


def coupling_dot(P: Program, title: str, edges: list[tuple[str, Doc]]) -> tuple[str, int, int]:
    """The questions' hypergraph over some edges: a question a box, an edge of two a line, an
    edge of more a point joined to each of its questions, each under the id its table has."""
    used = sorted({q for _, e in edges for q in e["question_ids"]})
    lines = ["graph coupling {",
             f'  graph [label={dq(title)}, labelloc=t, fontsize=11, fontname="Helvetica", overlap=scale];',
             '  node [shape=box, fontsize=9, fontname="Helvetica", margin="0.06,0.03", height=0.25];',
             '  edge [fontname="Helvetica", fontsize=8];']
    for q in used:
        qq = P.questions[q]
        label = f"q{q} {qq['name']}\\n{qq['open']} open of {len(qq['members'])}"
        style = ""
        if qq["is"] != "a group":
            style += ", style=rounded"
        if qq["open"] == 0:
            style += ', color="#8c959f", fontcolor="#8c959f"'
        lines.append(f"  q{q} [label={dq(label).replace(chr(92) * 2 + 'n', chr(92) + 'n')}{style}];")
    n = 0
    for eid, e in edges:
        stroke = EDGE_STROKES[edge_stroke(P, e)][0]
        qs = e["question_ids"]
        tip = dq(f"{eid}: {e['kind']}")
        if len(qs) == 2:
            lines.append(f"  q{qs[0]} -- q{qs[1]} [label={dq(eid)}, {stroke}, tooltip={tip}];")
            n += 1
        else:
            lines.append(f"  {eid} [shape=point, width=0.09, xlabel={dq(eid)}, fontsize=8, {stroke.split(', style')[0]}, tooltip={tip}];")
            for q in qs:
                lines.append(f"  {eid} -- q{q} [{stroke}];")
            n += len(qs)
    lines.append("}")
    return "\n".join(lines) + "\n", len(used), n


def edge_rows(P: Program, edges: list[Doc], prefix: str) -> tuple[list[list[C]], list[str]]:
    licensed = {d["instance"]: d["licensed"] for d in P.coupling["debts"]}
    known = ("questions", "question_ids", "kind", "pairs", "node", "debts", "stated")
    rows, ids = [], []
    for k, e in enumerate(edges):
        p, nd = e["pairs"], e["node"]
        debts = "<br>".join(f"<code>{esc(i)}</code> <span class=\"dim\">{'licensed' if licensed[i] else 'not licensed'}</span>"
                            for i in e["debts"])
        rows.append([
            H(f"{prefix}{k}", f"{prefix}{k}"), T(e["kind"]), N(len(e["question_ids"])),
            H("<br>".join(P.q(q) for q in e["question_ids"]), e["question_ids"]),
            N(p["excludes"]), N(p["implies"]), N(p["interacts"]),
            N(nd["conflicts"]), N(nd["derivations"]), N(nd["implications"]),
            H(debts, [{"instance": i, "licensed": licensed[i]} for i in e["debts"]]),
            H("<br>".join(f"<code>{esc(s)}</code> ×{n}" for s, n in e["stated"]), e["stated"]),
            key_values({a: b for a, b in e.items() if a not in known}),
        ])
        ids.append(f"{prefix}{k}")
    return rows, ids


EDGE_HEAD = ["edge", "kind", "arity", "its questions", "pairs: excludes", "implies", "interacts",
             "node: conflicts", "derivations", "implications", "the debts", "the statements most named", "the rest"]


def hypergraph(pg: Page, P: Program, name: str, title: str, what: str, edges: list[Doc], prefix: str, key: str) -> None:
    """One reading's edges: the table, the picture, its parts where it is large, and the matrix of the edges of two."""
    src = [(P.f("coupling.json"), key)]
    rows, ids = edge_rows(P, edges, prefix)
    pg.h(3, name, title)
    pg.p(what, "what")
    labelled = [(f"{prefix}{k}", e) for k, e in enumerate(edges)]
    if not edges:
        pg.p("No edge: no two questions bear on each other under this reading.", "note")
    else:
        dot, nodes, n = coupling_dot(P, f"{P.path}: {title.lower()}", labelled)
        pg.picture(f"{name}.all", "", "Every edge. " + " ".join(
            f"<em>{esc(t)}</em>." for _, t in EDGE_STROKES.values()) + " A grey box is a question with no open "
            "member, a rounded one a candidate in no group. An edge's label is its row below.",
            src, dot, n, layout_for(nodes, n), level=4)
        if n > SPLIT_BOUND:
            big = [(i, e) for i, e in labelled if len(e["question_ids"]) > 2]
            two = [(i, e) for i, e in labelled if len(e["question_ids"]) == 2]
            if big and two:
                dot, nodes, m = coupling_dot(P, f"{P.path}: {title.lower()}, of three questions or more", big)
                pg.picture(f"{name}.three", "", f"The same, the {len(big)} edges of three questions or more alone.",
                           [], dot, m, layout_for(nodes, m), level=4)
                dot, nodes, m = coupling_dot(P, f"{P.path}: {title.lower()}, of two questions", two)
                pg.picture(f"{name}.two", "", f"The same, the {len(two)} edges of two questions alone.", [], dot, m,
                           layout_for(nodes, m), level=4)
            comps = {}
            for c_k, comp in enumerate(P.coupling["components"]):
                for c in comp:
                    for q in P.questions_of(c):
                        comps.setdefault(q, c_k)
            by_comp: dict[int, list] = {}
            for i, e in labelled:
                ks = {comps.get(q, -1) for q in e["question_ids"]}
                by_comp.setdefault(min(ks) if len(ks) == 1 else -2, []).append((i, e))
            if len([k for k in by_comp if k >= 0]) > 1:
                for c_k in sorted(k for k in by_comp if k >= 0):
                    dot, nodes, m = coupling_dot(P, f"{P.path}: {title.lower()}, component {c_k}", by_comp[c_k])
                    pg.picture(f"{name}.component{c_k}", "", f"The same, the {len(by_comp[c_k])} edges inside "
                               f"component {c_k} (of the components over every edge).", [], dot, m,
                               layout_for(nodes, m), level=4)
    pg.table(name, "", "One row an edge of the picture.", src, EDGE_HEAD, rows, fold=len(rows) > FOLD_ROWS,
             row_ids=ids, level=4)
    two = [(k, e) for k, e in enumerate(edges) if len(e["question_ids"]) == 2]
    qs = sorted({q for _, e in two for q in e["question_ids"]})
    if two and len(qs) <= 64:
        at = {tuple(e["question_ids"]): (k, e) for k, e in two}
        body = []
        for a in qs:
            line = [f'<th><a href="#q{a}">q{a}</a></th>']
            for b in qs:
                hit = at.get((a, b)) or at.get((b, a))
                if hit is None:
                    line.append("<td></td>")
                else:
                    k, e = hit
                    p, nd = e["pairs"], e["node"]
                    text = "".join(f"{letter}{v}" for letter, v in (
                        ("x", p["excludes"]), ("i", p["implies"]), ("n", p["interacts"]),
                        ("C", nd["conflicts"]), ("D", nd["derivations"]), ("I", nd["implications"]),
                        ("o", len(e["debts"]))) if v)
                    line.append(f'<td><a href="#{prefix}{k}" title="{prefix}{k}">{esc(text) or "·"}</a></td>')
            body.append("<tr>" + "".join(line) + "</tr>")
        pg.p(f"The {len(two)} edges of two questions as a matrix, a question a row and a column: a cell is its edge, "
             f"written as the counts of its row above, <code>x</code> excludes, <code>i</code> implies, <code>n</code> "
             f"interacts of the pair layer's ties, <code>C</code> conflicts, <code>D</code> derivations, <code>I</code> "
             f"implications of the node, <code>o</code> debts. The same rows as the table, and the legible form of "
             f"them where the picture is dense.", "what")
        pg.add('<details><summary>the matrix, ' + f"{len(qs)} by {len(qs)}" + '</summary><div class="scroll">'
               '<table class="matrix"><thead><tr><th></th>'
               + "".join(f'<th><a href="#q{b}">q{b}</a></th>' for b in qs) + "</tr></thead><tbody>" + "".join(body)
               + "</tbody></table></div></details>")
    elif two:
        pg.p(f"The edges of two questions are over {len(qs)} questions, more than the 64 a matrix is written for "
             f"here: the table above is their only form.", "note")


def section_coupling(pg: Page, P: Program) -> None:
    cp = P.coupling
    counts = cp["counts"]
    pg.h(2, "coupling", "The hypergraph of questions")
    pg.p(f"A node is a <strong>question</strong>: a group, <em>which of these, if any</em>, or a candidate in no "
         f"group, <em>this one, or not</em>. An edge is two or more questions that bear on each other: some conflict "
         f"or some derivation needs a member of each of them (a <em>tie</em>), or some instance root owes has payers "
         f"in each of them (a <em>debt</em>, licensed by L5 or not). {len(cp['questions'])} questions, "
         f"{len(cp['ties'])} ties of the pair layer, {len(cp['edges'])} edges under the reading taken, "
         f"<code>{esc(cp['reading'])}</code>, "
         + (f"{len(cp['edges_one_step_back'])} one application back" if cp["edges_one_step_back"] is not None
            else "no reading one application back") +
         f"; {len(cp['components'])} components over every edge and {len(cp['conflict_components'])} over the "
         f"conflicts' and the derivations' alone; {len(cp['debts'])} debts that are edges.")
    for note in (cp["domain"], counts["node_stopped"] and f"The whole node was not read: {counts['node_stopped']}."):
        if note:
            pg.p(esc(note), "note")
    if not cp["complete"]:
        pg.p("The coupling was <strong>not read whole</strong> (<code>coupling.json</code> § <code>complete</code>).", "note")
    rows, ids = [], []
    n_edges, n_back = Counter(), Counter()
    for e in cp["edges"]:
        n_edges.update(e["question_ids"])
    for e in cp["edges_one_step_back"] or []:
        n_back.update(e["question_ids"])
    for k, q in enumerate(cp["questions"]):
        rows.append([H(f"q{k}", k), T(q["is"]), K(q["name"]), N(len(q["members"])), N(q["open"]),
                     P.cs([m[0] for m in q["members"]]), N(n_edges[k]), N(n_back[k]),
                     H(P.g(k) if q["is"] == "a group" else "", k if q["is"] == "a group" else None)])
        ids.append(f"q{k}")
    pg.table("questions", "The questions", "A question's place is its id: the groups in the cover's order, then the "
             "candidates in no group. <strong>A name is no identity</strong>: two questions of one program can share a "
             "text, and an edge names its questions by place (<code>question_ids</code>).",
             [(P.f("coupling.json"), "questions")],
             ["question", "is", "name", "members", "open at root", "the members", "edges it is in",
              "edges one step back", "its group"], rows, fold=len(rows) > FOLD_ROWS, row_ids=ids)
    pg.picture("coupling.engine", "The engine's own picture", "<code>coupling.dot</code> as the example wrote it, "
               "under the reading taken: an edge of two questions a line, an edge of more a point joined to each, a "
               "debt dashed; a label is the number of ties and ways the edge covers.", [(P.f("coupling.json"), "edges")],
               "", sum(1 for l in read_text(P.f("coupling.dot")).splitlines() if " -- " in l),
               layout_for(len(cp["questions"]), sum(1 for l in read_text(P.f("coupling.dot")).splitlines() if " -- " in l)),
               engine_dot=P.f("coupling.dot"))
    hypergraph(pg, P, "edges", "The edges of the whole node",
               "The reading taken: the pair layer's ties, and one node with every candidate alive alone committed, "
               "followed back to the candidates. Each edge with its kind and its reason: the ties and the ways it "
               "covers, its debts, and the puzzle's statements its derivations name most.", cp["edges"], "e", "edges")
    if cp["edges_one_step_back"] is not None:
        hypergraph(pg, P, "edges_back", "The edges one application back",
                   "Read off the same node, each derived premise followed back one application: the finer edges "
                   "the whole node's union of labels absorbs.", cp["edges_one_step_back"], "b", "edges_one_step_back")
    else:
        pg.h(3, "edges_back", "The edges one application back")
        pg.p("The file holds none (<code>edges_one_step_back</code> is <code>null</code>).", "note")
    rows = [[K(d["rule"]), K(d["instance"]), H("licensed" if d["licensed"] else "<strong>not licensed</strong>", d["licensed"]),
             P.cs(d["payers"]), H("<br>".join(P.q(q) for q in d["question_ids"]), d["question_ids"]),
             key_values({a: b for a, b in d.items() if a not in ("rule", "instance", "licensed", "payers", "questions", "question_ids")})]
            for d in cp["debts"]]
    pg.table("debts", "The debts that are edges", "Every instance root owes whose payers lie in two questions or "
             "more. An unlicensed one's payers are those a single candidate's node shows.",
             [(P.f("coupling.json"), "debts, debts_across_conflict_components, unlicensed_instances")],
             ["rule", "instance", "L5", "payers", "their questions", "the rest"], rows, fold=len(rows) > FOLD_ROWS)
    pg.p(f"Debts across two components of the conflicts' and the derivations' edges: "
         f"{any_cell(cp['debts_across_conflict_components']).h or 'none'}. Instances L5 does not license, in all: "
         f"{any_cell(cp['unlicensed_instances']).h}.", "raw")
    rows = []
    for which, comps in (("over every edge", cp["components"]),
                         ("over the conflicts' and the derivations' edges alone", cp["conflict_components"])):
        for k, comp in enumerate(comps):
            qs = sorted({q for c in comp for q in P.questions_of(c)})
            rows.append([T(which), N(k), N(len(comp)), P.cs(comp, " "),
                         H(" ".join(P.q(q, name=False) for q in qs), qs)])
    pg.table("coupling_components", "The components, both sets", "Of the candidates alive alone at root: two are "
             "together where they share a question or an edge holds a question of each. R is the product of its "
             "parts over the first set; the second stands beside.",
             [(P.f("coupling.json"), "components, conflict_components")],
             ["set", "component", "candidates", "the candidates", "their questions"], rows, fold=len(rows) > FOLD_ROWS)
    rows = [[T(t["kind"]), P.c(t["pair"][0]), P.c(t["pair"][1]),
             KS(t["stated"]) if t["stated"] is not None else T("")] for t in cp["ties"]]
    pg.table("ties", "The ties of the pair layer", "Two candidates alive alone, in no question together, that root "
             "excludes (and the theory does not), one of which implies the other, or that interact. An excluding "
             "tie names the statements its support rests on; a tie that implies or interacts has no support.",
             [(P.f("coupling.json"), "ties")], ["kind", "a", "b", "the statements"], rows, fold=True)
    pg.table("committed", "The candidates the whole node commits", "Those alive alone at root.",
             [(P.f("coupling.json"), "committed")], ["candidates"], [[P.cs(cp["committed"], " ")]], fold=True)
    pg.table("coupling_counts", "The coupling, counted", "The census line's <code>theory.coupling</code> object, "
             "the three other readings' counts under <code>readings</code>; <code>by_arity</code> is "
             "<code>[arity, edges, of the pairs, of the node, of a debt]</code>.",
             [(P.b.out / "census.jsonl", f"{P.path} § theory.coupling"), (P.f("coupling.json"), "counts")],
             ["name", "value"], flat(P.row["theory"]["coupling"]), fold=True)


def bijections(P: Program) -> dict:
    """Every bijection, found or declared: S1f.10.4's T2, as `build/structure/s4/bijections.py`
    reads it, of one program. *Found* is the owner's (`exactly_one.json`'s `bijection`).
    *Declared* is a reading by name, which the owner may not make: the program states
    `(bijective R)`, under any module prefix."""
    declared, props = set(), {}
    for s in P.theory["stated"]:
        f = s["fact"].strip("()").split()
        if len(f) < 2:
            continue
        name = f[0].split(".")[-1]
        if name == "bijective" and len(f) == 2:
            declared.add(f[1])
        if name in PROPERTIES[1:]:
            props.setdefault(f[1], set()).add(name)
    cands: dict[str, list[Doc]] = {}
    for c in P.cover["candidates"]:
        cands.setdefault(c["candidate"].strip("()").split()[0], []).append(c)
    grids = P.x1["grids"]
    in_grid = {}
    for g in grids:
        for k in g["families"][0]["groups"]:
            for m in P.x1["groups"][k]["members"]:
                in_grid[m] = g["grid"]
    rows = []
    for r in sorted(declared):
        mine = cands.get(r, [])
        if not mine:
            rows.append((r, "declared, and nothing of it left in H", 0, [], "root decides the whole relation"))
            continue
        gs = sorted({in_grid[c["candidate"]] for c in mine if c["candidate"] in in_grid})
        loose = [c for c in mine if c["candidate"] not in in_grid]
        if not loose and all(grids[k]["bijection"] for k in gs):
            rows.append((r, "found and declared", len(mine), gs, ""))
            continue
        if loose:
            n = Counter(len(c["groups"]) for c in loose)
            why = ("a candidate is in more than two groups" if max(n) > 2 else
                   "no pair of its candidates is excluded by the theory" if set(n) == {0} else
                   "a candidate is in fewer than two groups")
        else:
            owed = [tuple(f["owed"] for f in grids[k]["families"]) for k in gs]
            about = sum(f["an_unlicensed_instance_is_about"] for k in gs for f in grids[k]["families"])
            why = ("its grids are found and nothing owes a group of them" if not any(any(o) for o in owed) else
                   "its grids are found and one family of each is owed"
                   + (", the other only by instances L5 does not license" if about else ""))
        rows.append((r, "declared and not found", len(mine), gs, why))
    found = []
    for g in grids:
        rels = set(g["relations"])
        states = sorted(set().union(*(props.get(r, set()) for r in rels)))
        if g["bijection"]:
            found.append((g["grid"], g["relations"], "found and declared" if rels & declared
                          else "found and declared nowhere", states))
        else:
            found.append((g["grid"], g["relations"], "a grid and no bijection: " + g["kind"], states))
    return {"declared": rows, "grids": found,
            "counts": Counter([r[1] for r in rows] + [f[2] for f in found if f[2] == "found and declared nowhere"])}


def section_declared(pg: Page, P: Program) -> None:
    pg.h(2, "declared", "Beside what the program declares")
    d = P.cover["declared"]
    rows = [[K(k), any_cell(v)] for k, v in d.items() if k != "groups"] + flat({"groups": d["groups"]})
    pg.table("mechanisms", "The mechanisms, as the engine reads them at root", "The relations the program's active "
             "hrules propose, its obligations can owe and its closures and lists take out "
             "(<code>ein_infer::mechanisms::read</code> at root's fixpoint), and the groups of each.",
             [(P.f("cover.json"), "declared")], ["name", "value"], rows)
    b = bijections(P)
    P.bijections = b
    theirs = P.b.out / "s4/bijections.py"
    record = PHASE / "s1f.10.4_bounded_groups.md"
    cited = (pg.link(theirs) if theirs.exists() else "<code>build/structure/s4/bijections.py</code>") + (
        f' (<a href="{esc(rel(record, pg.dir))}#t2-every-bijection-found-or-declared">S1f.10.4 § T2</a>)')
    pg.add('<div class="script">The two tables below are <strong>a script\'s reading by name</strong>, S1f.10.4\'s '
           f'T2 as {cited} makes it: <em>declared</em> is the program stating '
           "<code>(bijective R)</code>, under any module prefix, and a relation is read off a candidate's text. "
           "<em>Found</em> is the owner's: a grid with as many groups in each family, every one owed.</div>")
    rows = [[K(r), T(status), N(n), H(", ".join(f'<a href="#grid{g}">grid {g}</a>' for g in gs), gs), T(why)]
            for r, status, n, gs, why in b["declared"]]
    pg.table("declared_bijections", "Every relation declared bijective", "Set against the grids its candidates "
             "are members of: found and declared, declared and not found with what broke it, or declared with "
             "nothing left in H.", [(P.f("theory.json"), "stated"), (P.f("cover.json"), "candidates"),
                                    (P.f("exactly_one.json"), "grids, groups")],
             ["relation", "is", "its candidates in H", "the grids over them", "what broke it"], rows)
    rows = [[H(f'<a href="#grid{g}">grid {g}</a>', g), KS(rels, ", "), T(status), KS(states, ", ")]
            for g, rels, status, states in b["grids"]]
    pg.table("found_bijections", "Every grid found", "A bijection found and declared, one found and declared "
             "nowhere, or a grid that is no bijection, with what the program states of its relations by the "
             "names <code>functional</code>, <code>injective</code>, <code>total</code>, <code>surjective</code>, "
             "<code>total-owed</code>, <code>surjective-owed</code>.",
             [(P.f("exactly_one.json"), "grids"), (P.f("theory.json"), "stated")],
             ["grid", "its relations", "is", "stated of them, by name"], rows)
    x1 = P.x1
    unlicensed_about = [g for g, xg in enumerate(x1["groups"]) if xg["an_unlicensed_instance_is_about_it"]]
    rows = [
        [T("found and owed: a group that is a licensed instance's alternatives"), N(len(x1["owed_groups"])),
         H(", ".join(P.g(g) for g in x1["owed_groups"]), x1["owed_groups"])],
        [T("found, and nothing owes it: at most one, and no must"), N(len(x1["groups_nothing_owes"])),
         H(", ".join(P.g(g) for g in x1["groups_nothing_owes"]), x1["groups_nothing_owes"])],
        [T("… of them, an instance L5 does not license is about it"), N(len(unlicensed_about)),
         H(", ".join(P.g(g) for g in unlicensed_about), unlicensed_about)],
        [T("owed, and its alternatives are no group: at least one, and no at most one"),
         N(len(x1["owed_sets_not_a_group"])), any_cell(x1["owed_sets_not_a_group"])],
        [T("owed by an instance L5 does not license"), N(len(x1["unlicensed"])),
         H(", ".join(f'<a href="#i{k}">i{k}</a>' for k in x1["unlicensed"]), x1["unlicensed"])],
    ]
    pg.table("declared_owed", "The groups found, beside the debts stated", "The engine's own two lists set against "
             "each other: what the cover finds and what root owes.",
             [(P.f("exactly_one.json"), "owed_groups, groups_nothing_owes, groups, owed_sets_not_a_group, unlicensed")],
             ["", "how many", "which"], rows)


def verdict_word(v: Doc) -> str:
    word = "go" if v["go"] else "no"
    if v["bound"]:
        word += ", as a bound"
    if v["estimate"]:
        word += ", an estimate"
    return word


def section_cost(pg: Page, P: Program) -> None:
    row, st = P.row, P.prices["structure"]
    th, cp = row["theory"], row["theory"]["coupling"]
    pg.h(2, "cost", "What it cost")
    pg.p("In the read-out's units, saturations and facts beyond root, and the walls the censuses measured on AC, "
         "one P-core (the fingerprints are at the top of the page). A wall is this machine's; every other number "
         "is the program's.")
    lay, small, sup = st["root_layer"], th["small"], th["supports"]
    node = cp["node"] or {}
    rows = [
        [T("root's pair layer, whole"), N(row["saturations"]), N(row["facts"]), W(None),
         T(f"{row['dead_alone']} dead alone, {row['pairs']['excluded']} pairs excluded")],
        [T("… root's node and the singles"), N(lay["singles"]["saturations"]), N(lay["singles"]["facts"]), W(None),
         T("the price list's split of the layer")],
        [T("… the pairs"), N(lay["pairs"]["saturations"]), N(lay["pairs"]["facts"]), W(None), T("")],
        [T("… the layer read a second time, node by node (the price list's)"), N(lay["saturations"]), N(lay["facts"]),
         W(lay["wall"]), T("")],
        [T("the small KB's pair layer"), N(small["saturations"]), N(small["facts"]), W(None),
         T(f"in its own root's terms; {small['pairs']['excluded']} pairs excluded, {small['dead_alone']} killed alone")],
        [T("the supports of the pairs root excludes"), N(sup["work"]["supports"]), N(None), W(None),
         T(f"one more saturation a pair, and a search; {sup['cut']} cut at the search's budget")],
        [T("the supports' check"), N(sup["work"]["check_saturations"]), N(sup["work"]["check_facts"]), W(None),
         T(f"{sup['work']['check_roots']} check programs")],
        [T("the cover, beside the small KB's layer"), N(st["cover"]["saturations"]), N(0), W(None),
         T(f"{st['cover']['groups']} groups, {st['cover']['steps']} calls of the enumeration")],
        [T("what root owes, beside the cover"), N(st["owed"]["saturations"]), N(0), W(None),
         T(f"{st['owed']['instances']} instances, {st['owed']['licensed']} licensed, {st['owed']['owed_groups']} "
           f"owed groups, {st['owed']['grids']} grids")],
        [T("the coupling's whole node, beside both layers"), N(node.get("saturations")), N(node.get("facts")), W(None),
         T(f"{node.get('applications')} applications, {node.get('steps')} derivation steps, {node.get('rounds')} rounds"
           if node else f"not read: {cp['node_stopped']}")],
        [T("the coupling's whole second reading, both pair layers included"), N(None), N(None), W(cp["wall"]), T("")],
    ]
    for reading, r in cp.get("readings", {}).items():
        rows.append([T(f"… the same under the reading `{reading}`"), N(None), N(None), W(r.get("wall")),
                     T(r.get("error", ""))])
    rows.append([T("the price list's reading of the structure, once"), N(None), N(None), W(st["wall"]), T("")])
    rows.append([T("the census line's whole reading"), N(None), N(None), W(row["wall"]),
                 T("everything above but the price list's: both layers, the supports and their check, the cover, "
                   "exactly one, the files written, and the coupling under its four readings")])
    for name, key in (("a describe", "describe"), ("`--single`", "single"), ("`--maximal`", "maximal")):
        r = row["routes"][key]
        how = "finished" if r["finished"] else f"stopped: {json.dumps(r['stopped'])}"
        if r["verdict"]:
            how += f"; {r['verdict']}"
        rows.append([H(f"<strong>beside it: {esc(name).replace('`', '')}</strong>", f"beside it: {name}"),
                     N(r["saturations"]), N(r["facts"]), W(r["wall"]), T(how)])
    pg.table("cost_objects", "Each object, beside a describe and --single", "What each object of the structure "
             "cost to compute, and what the program's own questions cost at the same budget.",
             [(P.b.out / "census.jsonl", f"{P.path} § saturations, facts, wall, theory.small, theory.supports, "
               "theory.coupling, routes"), (P.f("prices.json"), "structure")],
             ["object", "saturations", "facts beyond root", "wall", ""], rows)

    cells = P.prices["cells"]
    rows = []
    for key, flags in ASKED:
        c = cells[key]
        split = c["split"]
        rows.append([K(flags), T(c["route"]), N(c["work"]["saturations"]), N(c["work"]["facts"]),
                     T(c["verdict"]), T("exhausted" if c["exhausted"] else "not exhausted"),
                     any_cell(c["stopped"]), W(c["wall"]), W(c["wall_watched"])]
                    + [N(split[p][0]) for p, _ in SPLIT] + [N(split[p][1]) for p, _ in SPLIT]
                    + [N(c["same_work_watched"]), N(c["states_listed"]), any_cell(c["ceiling"]),
                       H(pg.link(P.f("prices") / f"{key}.nodes.jsonl", "the nodes"), f"prices/{key}.nodes.jsonl")])
    pg.table("cost_cells", "The six questions, and the split of each one's work",
             "The price list's cells for this program: each question asked plain and watched, and its saturations "
             "and facts split by the structure. <em>Dead otherwise</em> is the last two parts together. The nodes "
             "of each run, one line a node, are linked raw and are not rendered.",
             [(P.f("prices.json"), "cells"), (P.f("prices.md"), "")],
             ["question", "route", "saturations", "facts", "verdict", "", "stopped", "wall", "wall, watched"]
             + [f"saturations: {t}" for _, t in SPLIT] + [f"facts: {t}" for _, t in SPLIT]
             + ["watched run the same", "states listed", "what a best order could not go under", "every node"], rows)

    verdicts = P.shared["verdicts"]["verdicts"]
    rows = []
    for key, flags in ASKED:
        c = cells[key]
        for w in WAYS:
            way = c["ways"][w]
            v = verdicts[key][w]
            higher = [u for u in ("saturations", "facts")
                      if any(l["program"] == P.path for l in v[u]["losers"])]
            delta = way["with"]["saturations"] - c["work"]["saturations"] if way["applies"] else None
            rows.append([
                K(flags), K(w), N(way["applies"]), N(c["work"]["saturations"]),
                N(way["added"]["saturations"]), N(way["removed"]["saturations"]), N(way["with"]["saturations"]),
                N(delta), N(c["work"]["facts"]), N(way["added"]["facts"]), N(way["removed"]["facts"]),
                N(way["with"]["facts"]),
                H("; ".join(f"{esc(o)} {x['saturations']}" for o, x in way["by"].items()), way["by"]),
                H("; ".join(f"{esc(o)} {x}" for o, x in way["counts"].items()), way["counts"]),
                N(way["exact"]), T(way["note"]),
                H(f'<span class="{"go" if v["go"] else "no"}">{esc(verdict_word(v))}</span>', verdict_word(v)),
                T("go" if v["saturations"]["go"] else "no"), T("go" if v["facts"]["go"] else "no"),
                T("higher by more than a tenth in " + " and ".join(higher) if higher else ""),
            ])
    pg.table("cost_ways", "Each way of spending the structure, on this program",
             "The price list's simulation on the nodes the routes visited, the route's order held: what a way would "
             "add (the structure it pays for), what it would remove, and the cell with it. The verdict is the "
             "corpus's, for the question and the way, by the rule written before the table; the last column says "
             "whether this program is one of its losers. No way is applied to a program the static check flags.",
             [(P.f("prices.json"), "cells[].ways"), (P.b.out / "prices/verdicts.json", "verdicts"),
              (P.b.out / "prices/verdicts.md", "")],
             ["question", "way", "applies", "saturations", "added", "removed", "with the way", "with − without",
              "facts", "added", "removed", "with the way", "added, by object (saturations)", "counts", "exact",
              "note", "the corpus's verdict", "… in saturations", "… in facts", "this program"], rows, fold=True)
    rows = []
    for key, flags in ASKED:
        for k, v in cells[key]["detail"].items():
            rows.append([K(flags), K(k), any_cell(v)])
        rows.append([K(flags), K("texts_no_candidate_has"), N(cells[key]["texts_no_candidate_has"])])
    rows.append([T("the program"), K("denials_read"), any_cell(P.prices["denials_read"])])
    pg.table("cost_detail", "Beside the split", "The counts each cell carries beside its split, under the price "
             "list's own names (<code>examples/shared/prices.rs</code>'s header says what each is).",
             [(P.f("prices.json"), "cells[].detail, denials_read")], ["question", "name", "value"], rows, fold=True)


def section_track_h(pg: Page, P: Program) -> None:
    pg.h(2, "track_h", "Track H's census rows")
    pg.p("What the three stages of track H read of this program, each row whole under its census's own column "
         "names. They were taken on 2026-11-15, before this census; a row's <code>n</code> is the H its census read.")
    for stage, d, example, question, _ in TRACK_H:
        head, rows, _ = P.shared[d]
        src = REPO / "build" / d / "census/rows.tsv"
        row = rows.get(P.path)
        if row is None:
            pg.h(3, f"h_{d}", f"{stage}, {example}")
            pg.p(f"<code>{esc(src.relative_to(REPO))}</code> has no row for this program.", "note")
            continue
        pg.table(f"h_{d.replace('.', '_')}", f"{stage}, {example} ({question})", f"This program's row of {stage}'s "
                 f"census; the example's header says what each column is.",
                 [(src, P.path), (REPO / f"ein.rs/crates/ein-infer/examples/{example}.rs", "")],
                 ["column", "value"], [[K(k), T(row.get(k, ""))] for k in head], fold=True)


def must_rows(entry: Doc) -> list[list[C]]:
    """One program's questions, before and after the lines that state the *must*."""
    def side(c: Doc) -> C:
        stop = " <strong>stopped</strong>" if c["stopped"] else ""
        return H(f'{c["saturations"]:,} · {c["facts"]:,}{stop}'.replace(",", " "),
                 [c["saturations"], c["facts"], c["stopped"]])
    return [[K(c["question"]), side(c["before"]), side(c["after"]), T(c["before"]["answer"]),
             T(c["after"]["answer"])] for c in entry["cells"]]


def ticks(text: str) -> str:
    """`text` escaped, a span in backticks set as code, as the stage's Markdown writes it."""
    return re.sub(r"`([^`]+)`", r"<code>\1</code>", esc(text))


def must_entry(b: Bundle, path: str) -> Doc | None:
    return next((e for e in b.must["programs"] if e["path"] == path), None)


def section_must(pg: Page, P: Program) -> None:
    """S1f.10.14's T3 of this program, where it is one of the programs the stage concerns."""
    e = must_entry(P.b, P.path)
    if e is None:
        return
    src = P.b.out / "s14/must.json"
    pg.h(2, "must", "What it would have to state")
    pg.p(f"S1f.10.14's T3, advice to the program's author and nothing the engine does: "
         f"{e['groups']} groups nothing owes here, <em>{esc(e['kind'])}</em>. What the program would have to state "
         f"for <em>exactly one</em> of them: {ticks(e['states'])}. On the copy that states it, "
         f"<strong>{ticks(e['r'])}</strong>. The lines, appended or edited in place "
         f"(<code>{esc(e['copy'])}</code>):")
    pg.add(f"<pre>{esc(chr(10).join(e['lines']))}</pre>")
    pg.table("must_questions", "Every question, before and after",
             "Each question of <code>ein solve</code> asked of the program and of the copy, saturations · facts "
             "beyond root, and what the read-out answered. A cell the budget stopped is marked.",
             [(src, P.path)], ["question", "before", "after", "the answer before", "the answer after"],
             must_rows(e), level=3)


def must_page(b: Bundle, built: set[str]) -> None:
    """The whole of S1f.10.14's T3, one table a program and a summary over them."""
    out = b.root / "must.html"
    pg = Page(b, b.root)
    src = b.out / "s14/must.json"
    m = b.must
    rows = []
    for e in m["programs"]:
        link = (f'<a href="{esc(rel(b.root / "p" / e["path"][:-4] / "index.html", b.root))}#must">'
                f'<code>{esc(e["path"])}</code></a>' if e["path"] in built else f'<code>{esc(e["path"])}</code>')
        cells = {c["question"]: c for c in e["cells"]}
        rows.append([H(link, e["path"]), T(e["tag"]), N(e["groups"]), T(e["kind"]), H(ticks(e["states"]), e["states"]),
                     H(ticks(e["r"]), e["r"])] + [
            H(f'{cells[q]["before"]["saturations"]:,} → {cells[q]["after"]["saturations"]:,}'.replace(",", " ")
              + (" <strong>stopped</strong>" if cells[q]["before"]["stopped"] or cells[q]["after"]["stopped"] else ""),
              [cells[q]["before"]["saturations"], cells[q]["after"]["saturations"]])
            for q in m["questions"]])
    pg.table("must", "Every program, every question", "Per program S1f.10.4 named where <em>exactly one</em> is not "
             "derived: how many groups nothing owes and why, what the program would have to state, whether the copy "
             "that states it has the same R, and each question's saturations before → after. Each program's page has "
             "its lines and the answers.", [(src, "programs")],
             ["program", "tag", "groups nothing owes", "why", "what it would have to state", "R"] + m["questions"],
             rows, level=2)
    for e in m["programs"]:
        pg.table(f'must_{slug(e["path"])}', e["path"], f"<code>{esc(e['path'])}</code>: {ticks(e['r'])}.",
                 [(src, e["path"])], ["question", "before", "after", "the answer before", "the answer after"],
                 must_rows(e), level=3)
    stage = PHASE / "s1f.10.14_exactly_one_spent.md"
    body = (f"<h1>Where <em>exactly one</em> is missing: what a program would have to state</h1>"
            f"<p>S1f.10.14's T3, advice to a program's author: for each program S1f.10.4 named, the lines that would "
            f"state the <em>must</em> and every question asked before and after, by <code>ein solve</code> on both. "
            f"Nothing here is synthesised by the engine. The record: "
            f'<a href="{esc(rel(stage, b.root))}">S1f.10.14</a>. Taken {esc(m["taken"])} by '
            f"<code>{esc(m['binary'])}</code>, with</p><pre>{esc(MUST_COMMANDS)}</pre>"
            + b.provenance(b.root) + pg.toc_html() + "\n".join(pg.parts))
    b.write_page(out, "Exactly one, missing", [("the structure, shown", b.out / "index.html"),
                                               ("exactly one, missing", None)], body)


def section_files(pg: Page, P: Program) -> None:
    pg.h(2, "files", "Every file")
    rows = []
    engine = {"theory.md": "theory", "cover.md": "cover", "exactly_one.md": "exactly_one",
              "coupling.md": "coupling", "prices.md": "prices"}
    whats = {
        "pairs.json": "the pair layer's lists, as the program writes its facts",
        "theory.json": "the theory's share: what is stated, what the small KB keeps, each pair root excludes with its support",
        "theory.md": "the same as tables", "theory.ein": "the small KB as a program",
        "cover.json": "the cover: groups, components, root's cliques, what the program declares",
        "cover.md": "the same as tables",
        "exactly_one.json": "every instance with every witness, every group with who owes it, the grids",
        "exactly_one.md": "the same as tables",
        "coupling.json": "the questions, the ties, the edges under the reading taken and one step back, the debts",
        "coupling.md": "the same as tables", "coupling.dot": "the questions' hypergraph, the engine's picture",
        "prices.json": "the six questions' cells, split, with each way of spending",
        "prices.md": "the same as tables",
    }
    for f in Program.FILES:
        path = P.f(f)
        twin = f'<a href="engine/{engine[f]}.html">as a page</a>' if f in engine else ""
        rows.append([H(pg.link(path), f), T(size_of(path)), T(whats[f]), H(twin, f"engine/{engine[f]}.html" if f in engine else None)])
    for key, flags in ASKED:
        path = P.f("prices") / f"{key}.nodes.jsonl"
        rows.append([H(pg.link(path, f"prices/{key}.nodes.jsonl"), f"prices/{key}.nodes.jsonl"), T(size_of(path)),
                     H(f"every node <code>{esc(flags)}</code> visited: commitment, what it is, its facts, its part, "
                       f"its core. <strong>Not rendered</strong>, one line a node", None), T("")])
    rows.append([H(pg.link(REPO / P.path), P.path), T(size_of(REPO / P.path)), T("the program"), T("")])
    pg.table("file_list", "", "Every file the engine wrote of this program, raw, and the engine's own Markdown "
             "twins rendered as pages.", [(P.dir, "")], ["file", "size", "what it is", "rendered"], rows)


def engine_pages(b: Bundle, P: Program) -> None:
    """The engine's own Markdown twins, rendered: nothing of them is left to this script's tables."""
    for name in ("theory", "cover", "exactly_one", "coupling", "prices"):
        src = P.f(f"{name}.md")
        out = P.page_dir / "engine" / f"{name}.html"
        md = Markdown(src, out.parent, whole=True)
        body = md.render(read_text(src))
        note = (f'<p class="raw">The engine\'s own tables, <a href="{esc(rel(src, out.parent))}"><code>'
                f'{esc(src.relative_to(REPO))}</code></a>, rendered as they are written; the raw lists are '
                f'<a href="{esc(rel(src.with_suffix(".json"), out.parent))}"><code>{esc(name)}.json</code></a>.</p>')
        b.write_page(out, f"{P.path}: {name}.md",
                     [("the structure, shown", b.out / "index.html"), (P.path, P.page_dir / "index.html"),
                      (f"{name}.md", None)], b.provenance(out.parent) + note + body)


def program_page(b: Bundle, P: Program, prev: str | None, nxt: str | None) -> None:
    pg = Page(b, P.page_dir)
    P.rule_of = {i["instance"]: i["rule"] for i in P.x1["instances"]}
    row = P.row
    tag = TAG.get(P.path)
    section_h(pg, P)
    section_exclusion(pg, P)
    section_cover(pg, P)
    section_exactly_one(pg, P)
    section_coupling(pg, P)
    section_declared(pg, P)
    section_cost(pg, P)
    section_track_h(pg, P)
    section_must(pg, P)
    section_files(pg, P)
    engine_pages(b, P)

    def other(path: str | None, word: str) -> str:
        if path is None:
            return ""
        return f'{word}: <a href="{esc(rel(b.root / "p" / path[:-4] / "index.html", P.page_dir))}"><code>{esc(path)}</code></a>'

    cov, x1c, cpc = row["theory"]["cover"], row["theory"]["cover"]["exactly_one"], row["theory"]["coupling"]
    head = [
        f'<h1>{f"<span class=tag>{esc(tag)}</span> " if tag else ""}<code>{esc(P.path)}</code></h1>',
        f'<p>{pg.link(REPO / P.path, "the program")} · {" · ".join(x for x in (other(prev, "previous"), other(nxt, "next")) if x)}'
        f' · <a href="{esc(rel(b.root / "survey.html", P.page_dir))}">the survey</a></p>',
        f"<p>{row['n']} candidates, built by the rung <code>{esc(row['theory']['rung'])}</code>; the static check "
        f"calls the program {'monotone' if row['monotone'] else '<strong>flagged</strong>, not monotone'}. "
        f"{row['dead_alone']} dead alone, {row['pairs']['excluded']} pairs root excludes "
        f"({row['theory']['root_excluded_of_theory']} the theory's), {row['theory']['small']['pairs']['excluded']} pairs "
        f"the theory excludes over all of H; {cov['groups']} groups, {x1c['owed_groups']} owed, "
        f"{len(P.x1['grids'])} grids, {x1c['bijections']} bijections; {cpc['questions']} questions, "
        f"{cpc['edges']['all']} edges, {cpc['components']} components.</p>",
    ]
    if row["domain"]:
        head.append(f'<p class="note">{esc(row["domain"])}</p>')
    head.append(b.provenance(P.page_dir))
    head.append(pg.toc_html())
    b.write_page(P.page_dir / "index.html", f"{tag + ' ' if tag else ''}{P.path}",
                 [("the structure, shown", b.out / "index.html"), (P.path, None)],
                 "\n".join(head) + "\n" + "\n".join(pg.parts))


# ── the survey ───────────────────────────────────────────────────────────────

def at(obj, path: str):
    """`a.b.c` of a census line, or None where a step of it is missing."""
    for k in path.split("."):
        if not isinstance(obj, dict) or k not in obj or obj[k] is None:
            return None
        obj = obj[k]
    return obj


def arity_text(by_arity) -> str | None:
    if by_arity is None:
        return None
    return " ".join(f"{a[0]}:{a[1]}" for a in by_arity)


def survey_columns() -> list[tuple[str, str, object]]:
    """Every column of the survey: its group, its name, and where a census line holds it."""
    def route(key: str, field: str):
        return lambda r, e: at(r, f"routes.{key}.{field}")

    cols: list[tuple[str, str, object]] = [
        ("its kind", "static check", lambda r, e: None if "monotone" not in r else ("monotone" if r["monotone"] else "flagged")),
        ("its kind", "rung", "theory.rung"),
        ("H", "candidates", "n"), ("H", "held at root", "at_root"), ("H", "dead alone", "dead_alone"),
        ("H", "implications", "implications"), ("H", "equivalence classes", "equivalence_classes"),
        ("H", "candidates in a class", "equivalent_candidates"),
        ("pairs at root", "pairs", "pairs.all"), ("pairs at root", "dead by a member (L1)", "pairs.dead_member"),
        ("pairs at root", "implied (L3)", "pairs.implied"), ("pairs at root", "saturated", "pairs.saturated"),
        ("pairs at root", "excluded", "pairs.excluded"), ("pairs at root", "interacting", "pairs.interacting"),
        ("pairs at root", "candidates in an excluded pair", "in_excluded"),
        ("the theory's share", "facts stated", "theory.stated"), ("the theory's share", "dropped", "theory.dropped"),
        ("the theory's share", "the theory kills alone", "theory.small.dead_alone"),
        ("the theory's share", "excluded pairs: the theory's, over all of H", "theory.small.pairs.excluded"),
        ("the theory's share", "… a member dead alone at root", "theory.theory_over_dead_member"),
        ("the theory's share", "root's excluded pairs: the theory's", "theory.root_excluded_of_theory"),
        ("the theory's share", "root's excluded pairs: the puzzle's",
         lambda r, e: None if at(r, "theory.root_excluded") is None
         else r["theory"]["root_excluded"] - r["theory"]["root_excluded_of_theory"]),
        ("the theory's share", "supports cut at the search's budget", "theory.supports.cut"),
        ("the cover", "groups", "theory.cover.groups"),
        ("the cover", "the most groups a candidate is in", "theory.cover.most"),
        ("the cover", "candidates in no group", "theory.cover.in_no_group.all"),
        ("the cover", "components", "theory.cover.components"),
        ("the cover", "root's cliques", "theory.cover.root_cliques.cliques"),
        ("exactly one", "root owes", "theory.cover.exactly_one.root_owes"),
        ("exactly one", "instances in L5's table", "theory.cover.exactly_one.table"),
        ("exactly one", "owed groups", "theory.cover.exactly_one.owed_groups"),
        ("exactly one", "groups nothing owes", "theory.cover.exactly_one.groups_nothing_owes.all"),
        ("exactly one", "owed sets not a group", "theory.cover.exactly_one.owed_sets_not_a_group"),
        ("exactly one", "unlicensed instances", "theory.cover.exactly_one.unlicensed.all"),
        ("exactly one", "… about a group", "theory.cover.exactly_one.unlicensed.about_a_group"),
        ("exactly one", "grids", lambda r, e: None if at(r, "theory.cover.exactly_one.grids") is None
         else len(r["theory"]["cover"]["exactly_one"]["grids"])),
        ("exactly one", "bijections", "theory.cover.exactly_one.bijections"),
        ("beside what is declared (by name, the script's)", "relations declared bijective", lambda r, e: e.get("declared")),
        ("beside what is declared (by name, the script's)", "found and declared", lambda r, e: e.get("found and declared")),
        ("beside what is declared (by name, the script's)", "declared and not found", lambda r, e: e.get("declared and not found")),
        ("beside what is declared (by name, the script's)", "declared, nothing left in H",
         lambda r, e: e.get("declared, and nothing of it left in H")),
        ("beside what is declared (by name, the script's)", "found and declared nowhere",
         lambda r, e: e.get("found and declared nowhere")),
        ("the coupling", "questions", "theory.coupling.questions"),
        ("the coupling", "… a candidate in no group", "theory.coupling.alone"),
        ("the coupling", "ties", "theory.coupling.ties.all"), ("the coupling", "ties: excludes", "theory.coupling.ties.excludes"),
        ("the coupling", "ties: implies", "theory.coupling.ties.implies"),
        ("the coupling", "ties: interacts", "theory.coupling.ties.interacts"),
        ("the coupling", "edges", "theory.coupling.edges.all"),
        ("the coupling", "edges of the pairs", "theory.coupling.edges.of_the_pairs"),
        ("the coupling", "edges of the node", "theory.coupling.edges.of_the_node"),
        ("the coupling", "edges of a debt", "theory.coupling.edges.of_a_debt"),
        ("the coupling", "edges of a debt alone", "theory.coupling.edges.of_a_debt_alone"),
        ("the coupling", "edges of three questions or more", "theory.coupling.edges.of_three_or_more"),
        ("the coupling", "the largest edge", "theory.coupling.edges.largest"),
        ("the coupling", "edges by arity (arity:edges)", lambda r, e: arity_text(at(r, "theory.coupling.edges.by_arity"))),
        ("the coupling", "one step back: edges", "theory.coupling.one_step.all"),
        ("the coupling", "one step back: of three or more", "theory.coupling.one_step.of_three_or_more"),
        ("the coupling", "one step back: the largest", "theory.coupling.one_step.largest"),
        ("the coupling", "one step back: by arity", lambda r, e: arity_text(at(r, "theory.coupling.one_step.by_arity"))),
        ("the coupling", "components", "theory.coupling.components"),
        ("the coupling", "the largest component", "theory.coupling.largest_component"),
        ("the coupling", "components of the conflicts alone", "theory.coupling.conflict_components"),
        ("the coupling", "debts that are edges", "theory.coupling.debts"),
        ("the coupling", "… unlicensed", "theory.coupling.debts_unlicensed"),
        ("what it cost", "root's layer: saturations", "saturations"), ("what it cost", "root's layer: facts", "facts"),
        ("what it cost", "root's layer read node by node (the price list's): wall, s",
         lambda r, e: at(e, "structure.root_layer.wall")),
        ("what it cost", "the small KB's layer: saturations", "theory.small.saturations"),
        ("what it cost", "the small KB's layer: facts", "theory.small.facts"),
        ("what it cost", "supports read", "theory.supports.work.supports"),
        ("what it cost", "the check: saturations", "theory.supports.work.check_saturations"),
        ("what it cost", "the coupling's node: saturations", "theory.coupling.node.saturations"),
        ("what it cost", "the coupling's node: facts", "theory.coupling.node.facts"),
        ("what it cost", "the coupling's node: applications", "theory.coupling.node.applications"),
        ("what it cost", "the coupling's second reading: wall, s", "theory.coupling.wall"),
        ("what it cost", "the census line's whole reading: wall, s", "wall"),
    ]
    for key, name in (("describe", "describe"), ("single", "--single"), ("maximal", "--maximal")):
        cols += [("the routes (the census's)", f"{name}: saturations", route(key, "saturations")),
                 ("the routes (the census's)", f"{name}: facts", route(key, "facts")),
                 ("the routes (the census's)", f"{name}: wall, s", route(key, "wall")),
                 ("the routes (the census's)", f"{name}: finished", route(key, "finished"))]
    cols.append(("the routes (the census's)", "--single: verdict", route("single", "verdict")))
    for key, flags in ASKED:
        group = f"the price list: {flags}"
        cols.append((group, f"{flags}: saturations", lambda r, e, key=key: at(e, f"cells.{key}.work.saturations")))
        cols.append((group, f"{flags}: facts", lambda r, e, key=key: at(e, f"cells.{key}.work.facts")))
        for w in WAYS:
            def delta(r, e, key=key, w=w):
                way = at(e, f"cells.{key}.ways.{w}")
                if way is None or not way["applies"]:
                    return None
                return way["with"]["saturations"] - e["cells"][key]["work"]["saturations"]
            cols.append((group, f"{flags}, with {w}: saturations more or fewer", delta))
    return cols


SURVEY_JS = """
<script>
(function () {
  var table = document.getElementById('survey'), body = table.tBodies[0];
  var heads = table.tHead.rows[table.tHead.rows.length - 1].cells;
  function value(row, k) { var t = row.cells[k].textContent.trim(); return t; }
  function sortBy(k, th) {
    var rows = Array.prototype.slice.call(body.rows), desc = th.classList.contains('asc');
    var numeric = rows.every(function (r) { var v = value(r, k); return v === '' || !isNaN(parseFloat(v)) && isFinite(v); });
    rows.sort(function (a, b) {
      var x = value(a, k), y = value(b, k);
      if (x === '' || y === '') return (x === '') - (y === '');
      var c = numeric ? parseFloat(x) - parseFloat(y) : x.localeCompare(y);
      return desc ? -c : c;
    });
    for (var i = 0; i < heads.length; i++) heads[i].classList.remove('asc', 'desc');
    th.classList.add(desc ? 'desc' : 'asc');
    rows.forEach(function (r) { body.appendChild(r); });
  }
  for (var i = 0; i < heads.length; i++) (function (k) {
    heads[k].addEventListener('click', function () { sortBy(k, heads[k]); });
  })(i);
  var boxes = document.querySelectorAll('.controls input[data-group]');
  for (var j = 0; j < boxes.length; j++) boxes[j].addEventListener('change', function () {
    table.classList.toggle('hide-' + this.getAttribute('data-group'), !this.checked);
  });
  var only = document.getElementById('only-searching');
  only.addEventListener('change', function () { table.classList.toggle('only-searching', only.checked); });
})();
</script>
"""


def survey_page(b: Bundle, census: list[Doc], extras: dict[str, dict], built: set[str]) -> dict:
    cols = survey_columns()
    groups = []
    for g, _, _ in cols:
        if g not in groups:
            groups.append(g)
    head = ["program", "tag", "searches"] + [name for _, name, _ in cols] + ["the engine's refusal"]
    data = []
    body = []
    for r in census:
        path = r["path"]
        e = extras.get(path, {})
        if "error" in r:
            searches = "refused at load"
        elif r["searches"]:
            searches = "yes"
        else:
            searches = "no, and no page"
        values = []
        for _, _, where in cols:
            v = at(r, where) if isinstance(where, str) else where(r, e)
            values.append(v)
        data.append([path, TAG.get(path, ""), searches] + values + [r.get("error")])
        link = (f'<a href="{esc(rel(b.root / "p" / path[:-4] / "index.html", b.root))}"><code>{esc(path)}</code></a>'
                if path in built else f"<code>{esc(path)}</code>")
        cells = [f"<td>{link}</td>", f'<td class="tag">{esc(TAG.get(path, ""))}</td>',
                 f'<td class="tag">{esc(searches)}</td>']
        for (g, _, _), v in zip(cols, values):
            k = groups.index(g)
            if v is None:
                text = ""
            elif isinstance(v, bool):
                text = "yes" if v else "no"
            elif isinstance(v, float):
                text = f"{v:.3f}"
            else:
                text = str(v)
            num = isinstance(v, (int, float)) and not isinstance(v, bool)
            cells.append(f'<td class="g{k}{" num" if num else ""}">{esc(text)}</td>')
        cells.append(f'<td class="refusal">{esc(r.get("error", ""))}</td>')
        body.append(f'<tr class="{"s" if r.get("searches") else "ns"}">' + "".join(cells) + "</tr>")
    (b.root / "survey.json").write_text(json.dumps({
        "what": "one row a corpus file that loads or is refused; every count is the census line's or the price "
                "list's, and the five columns `beside what is declared` are the script's reading by name",
        "read_from": ["build/structure/census.jsonl", "build/structure/<program>/prices.json",
                      "build/structure/<program>/{theory,cover,exactly_one}.json"],
        "columns": head, "groups": ["the program"] * 3 + [g for g, _, _ in cols] + ["the refusal"], "rows": data,
    }, ensure_ascii=False, indent=0) + "\n")
    with open(b.root / "survey.tsv", "w") as f:
        f.write("\t".join(head) + "\n")
        for row in data:
            f.write("\t".join("" if v is None else str(v).replace("\t", " ").replace("\n", " ") for v in row) + "\n")
    b.tables += 1
    spans = "".join(f'<th colspan="{sum(1 for g, _, _ in cols if g == grp)}" class="g{k}">{esc(grp)}</th>'
                    for k, grp in enumerate(groups))
    names = "".join(f'<th class="sort g{groups.index(g)}">{esc(n)}</th>' for g, n, _ in cols)
    css = "".join(f"table.hide-g{k} .g{k} {{ display: none; }} " for k in range(len(groups)))
    css += "table.only-searching tr.ns { display: none; } td.refusal { min-width: 60em; }"
    controls = "".join(f'<label><input type="checkbox" checked data-group="g{k}"> {esc(g)}</label>'
                       for k, g in enumerate(groups))
    n_search = sum(1 for r in census if r.get("searches"))
    n_refused = sum(1 for r in census if "error" in r)
    body_html = (
        f"<h1>The survey: every corpus program, every count a column</h1>"
        f"<p>{len(census)} corpus files and register witnesses: {n_search} search and have a page, "
        f"{len(census) - n_search - n_refused} load and do not search (a row, and no page), and {n_refused} the "
        f"engine refuses at load. <strong>Click a column's name to sort by it</strong>, again to reverse; an empty "
        f"cell sorts last. The price list's columns are, for each question, the cell's saturations and facts and, for "
        f"each way of spending, how many saturations more (positive) or fewer (negative) the cell is with the way, "
        f"empty where the way does not apply.</p>"
        f'<p class="raw">Raw: <a href="{esc(rel(b.out / "census.jsonl", b.root))}"><code>build/structure/census.jsonl'
        f'</code></a>, one line a program, and each program\'s <code>prices.json</code>. This table as '
        f'<a href="survey.json"><code>survey.json</code></a> and <a href="survey.tsv"><code>survey.tsv</code></a>. '
        f"{len(data)} rows, {len(head)} columns.</p>"
        + b.provenance(b.root)
        + f'<p class="controls">Show: <label><input type="checkbox" id="only-searching"> only the programs that '
          f"search</label> {controls}</p>"
        + f"<style>{css}</style>"
        + '<div class="scroll" style="max-height:88vh;overflow:auto"><table id="survey" class="survey"><thead>'
        + f'<tr><th colspan="3">the program</th>{spans}<th>the refusal</th></tr>'
        + f'<tr><th class="sort">program</th><th class="sort">tag</th><th class="sort">searches</th>{names}'
          f'<th class="sort">the engine\'s refusal</th></tr>'
        + "</thead><tbody>" + "\n".join(body) + "</tbody></table></div>")
    b.write_page(b.root / "survey.html", "The survey", [("the structure, shown", b.out / "index.html"), ("the survey", None)],
                 body_html, SURVEY_JS, wide=True)
    totals = Counter()
    for e in extras.values():
        for k in ("declared", "found and declared", "declared and not found",
                  "declared, and nothing of it left in H", "found and declared nowhere"):
            totals[k] += e.get(k, 0)
    return dict(totals)


# ── the phase's own pages ────────────────────────────────────────────────────

def source_note(path: Path, page_dir: Path, anchor: str = "") -> str:
    href = rel(path, page_dir) + (f"#{anchor}" if anchor else "")
    return (f'<p class="raw">Rendered from <a href="{esc(href)}"><code>{esc(path.relative_to(REPO))}</code></a>'
            f'{" § <code>#" + esc(anchor) + "</code>" if anchor else ""}, as it is written; a link of it goes to the '
            f"file it names.</p>")


def whole_page(b: Bundle, out: Path, title: str, crumb: str, sources: list[tuple[Path, str]], lead: str = "") -> None:
    """A page that holds one or more Markdown files whole."""
    parts = [f"<h1>{esc(title)}</h1>", lead, b.provenance(out.parent)]
    for k, (src, prefix) in enumerate(sources):
        md = Markdown(src, out.parent, whole=True, prefix=prefix)
        parts.append(source_note(src, out.parent))
        parts.append(md.render(read_text(src)))
        if k + 1 < len(sources):
            parts.append("<hr>")
    b.write_page(out, title, [("the structure, shown", b.out / "index.html"), (crumb, None)], "\n".join(parts))


def defaults_page(b: Bundle) -> int:
    out = b.root / "defaults.html"
    parts = ["<h1>Every default part 1 took</h1>",
             "<p>Each stage record's <em>Defaults taken</em> section, <strong>verbatim</strong>, under its stage, "
             "with the file and the anchor it is from; and the two entries of <code>open_questions.md</code> that "
             "list the defaults of the re-plan and of part 1. Nothing is paraphrased: the rendered text is the "
             "section's, and its Markdown is folded under it. Each is the user's to revise.</p>",
             b.provenance(out.parent)]
    toc, n = [], 0
    for stage, name in STAGES:
        src = need(PHASE / name)
        text = read_text(src)
        heads = md_headings(text)
        title = heads[0][2] if heads else name
        seen: Counter = Counter()
        found = []
        for line, level, htext in heads:
            base = slug(htext)
            anchor = base if seen[base] == 0 else f"{base}-{seen[base]}"
            seen[base] += 1
            if re.search(r"defaults? taken", htext, re.I):
                found.append((line, htext, anchor))
        sid = "d-" + stage.lower().replace(".", "-")
        toc.append((sid, f"{stage}: {len(found)} section{'' if len(found) == 1 else 's'}"))
        parts.append(f'<h2 id="{sid}">{Markdown(src, out.parent, False).inline(title)}</h2>')
        parts.append(f'<p class="raw"><a href="{esc(rel(src, out.parent))}"><code>{esc(src.relative_to(REPO))}</code></a></p>')
        if not found:
            settle = [(line, htext) for line, _, htext in heads if "had to settle" in htext]
            if stage in ("S1f.10.9", "S1f.10.6", "S1f.10.15"):
                parts.append("<p class=\"note\">No section of this record is headed <em>Defaults taken</em>: the stage "
                             "stopped at its question, and its default is named there and not acted on. The question "
                             f'is on <a href="questions.html">the questions\' page</a>.</p>')
            else:
                links = ", ".join(f'<a href="{esc(rel(src, out.parent))}#{esc(slug(h))}">§ {esc(h)}</a>' for _, h in settle)
                parts.append("<p class=\"note\">No section of this record is headed <em>Defaults taken</em>. What its "
                             f"definition settled without a ruling is under {links or 'its record'}.</p>")
            continue
        for line, htext, anchor in found:
            section, _ = md_section(text, line)
            n += 1
            md = Markdown(src, out.parent, whole=False, prefix=f"{sid}-")
            parts.append(source_note(src, out.parent, anchor))
            parts.append(md.render(section))
            parts.append(f"<details><summary>the section's Markdown, verbatim</summary><pre><code>{esc(section)}</code></pre></details>")
    qtext = read_text(need(QUESTIONS_MD))
    for qid in ("Q-M1f.305", "Q-M1f.310"):
        hit = [(line, htext) for line, _, htext in md_headings(qtext) if htext.startswith(qid + " ")]
        sid = "d-" + qid.lower().replace(".", "")
        if not hit:
            raise Fail(f"{QUESTIONS_MD}: no heading of {qid}")
        line, htext = hit[0]
        section, _ = md_section(qtext, line)
        toc.append((sid, qid))
        parts.append(f'<h2 id="{sid}">{esc(qid)}, in <code>open_questions.md</code></h2>')
        parts.append(source_note(QUESTIONS_MD, out.parent, slug(htext)))
        parts.append(Markdown(QUESTIONS_MD, out.parent, whole=False, prefix=f"{sid}-").render(section))
        parts.append(f"<details><summary>the entry's Markdown, verbatim</summary><pre><code>{esc(section)}</code></pre></details>")
        n += 1
    nav = '<nav class="toc">' + "".join(f'<a href="#{a}">{esc(t)}</a>' for a, t in toc) + "</nav>"
    parts.insert(3, nav)
    b.write_page(out, "Every default part 1 took", [("the structure, shown", b.out / "index.html"), ("the defaults", None)],
                 "\n".join(parts))
    return n


def listing(root: Path, page_dir: Path, depth: int = 3) -> str:
    """A directory as nested lists of links. An oracle's dump is linked as a directory and not walked."""
    def walk(d: Path, level: int) -> str:
        items = []
        for p in sorted(d.iterdir(), key=lambda p: p.name):
            link = f'<a href="{esc(rel(p, page_dir))}"><code>{esc(p.name)}{"/" if p.is_dir() else ""}</code></a>'
            if p.is_dir():
                files = sum(1 for _ in p.rglob("*") if _.is_file())
                if p.name.endswith(".oracle") or level >= depth:
                    items.append(f'<li>{link} <span class="dim">{files} files</span></li>')
                else:
                    items.append(f'<li>{link} <span class="dim">{files} files</span>{walk(p, level + 1)}</li>')
            else:
                items.append(f'<li>{link} <span class="dim">{size_of(p)}</span></li>')
        return "<ul>" + "".join(items) + "</ul>"
    return walk(need(root), 1)


def questions_page(b: Bundle) -> None:
    out = b.root / "questions.html"
    qtext = read_text(need(QUESTIONS_MD))
    heads = md_headings(qtext)

    def entry(qid: str) -> tuple[str, str] | None:
        hit = [(line, htext) for line, _, htext in heads if htext.startswith(qid + " ")]
        if not hit:
            return None
        section, _ = md_section(qtext, hit[0][0])
        return hit[0][1], section

    stage = need(PHASE / "s1f.10.12_the_structure_shown.md")
    parts = ["<h1>The four questions' material</h1>",
             "<p>The phase's one stop puts four questions to the user. This page holds what each rests on: the entry "
             "of <code>open_questions.md</code>, rendered as it is written, and the witnesses and censuses on disk. "
             "No default of any of them is acted on before the reply.</p>", b.provenance(out.parent)]
    parts.append('<h2 id="surface">1. The surface: how <code>ein</code> shows a program\'s structure</h2>')
    slot = (f'<div class="slot"><strong>SLOT, the main session\'s to write: the surface question.</strong> This bundle '
            f"designs no surface and proposes none. The options (a) to (d) are in the stage file, "
            f'<a href="{esc(rel(stage, out.parent))}#t1f10122--the-stop"><code>{esc(stage.relative_to(REPO))}</code> '
            f"§ T1f.10.12.2</a>; what would be shown is what this bundle shows: "
            f'<a href="{esc(rel(b.out / "index.html", out.parent))}">the index</a>, '
            f'<a href="survey.html">the survey</a>, and a page a program.</div>')
    parts.append(slot)
    q309 = entry("Q-M1f.309")
    if q309 is not None:
        parts.append("<p class=\"note\">The entry as <code>open_questions.md</code> holds it when this bundle was "
                     "written, the main session's text and not this script's:</p>")
        parts.append(source_note(QUESTIONS_MD, out.parent, slug(q309[0])))
        parts.append(Markdown(QUESTIONS_MD, out.parent, whole=False, prefix="q309-").render(q309[1]))
    else:
        parts.append("<p class=\"note\"><code>open_questions.md</code> holds no entry for it yet.</p>")
    for k, (stage_id, d, example, qid, command) in enumerate(TRACK_H, 2):
        got = entry(qid)
        if got is None:
            raise Fail(f"{QUESTIONS_MD}: no heading of {qid}")
        htext, section = got
        record = need(PHASE / dict(STAGES)[stage_id])
        root = need(REPO / "build" / d)
        sid = qid.lower().replace(".", "")
        parts.append(f'<h2 id="{sid}">{k}. {Markdown(QUESTIONS_MD, out.parent, False).inline(htext)}</h2>')
        parts.append(f'<p>The stage: <a href="{esc(rel(record, out.parent))}"><code>{esc(record.relative_to(REPO))}'
                     f"</code></a>, {esc(stage_id)}. Its census is the example <code>{esc(example)}</code>'s, by "
                     f"<code>{esc(command)}</code>.</p>")
        parts.append(source_note(QUESTIONS_MD, out.parent, slug(htext)))
        parts.append(f"<details open><summary>the entry</summary>"
                     f"{Markdown(QUESTIONS_MD, out.parent, whole=False, prefix=sid + '-').render(section)}</details>")
        parts.append(f'<h3 id="{sid}-witnesses">Its witnesses and its census: <code>build/{esc(d)}/</code></h3>')
        version = root / "version.txt"
        if version.exists():
            parts.append(f"<p class=\"raw\">The binary they were taken with, <code>build/{esc(d)}/version.txt</code>:</p>"
                         f"<pre><code>{esc(read_text(version))}</code></pre>")
        run = root / "run.txt"
        if run.exists():
            parts.append(f'<details><summary><code>run.txt</code>, what <code>run.sh</code> printed</summary>'
                         f"<pre><code>{esc(read_text(run))}</code></pre></details>")
        parts.append(listing(root, out.parent))
    b.write_page(out, "The four questions' material",
                 [("the structure, shown", b.out / "index.html"), ("the questions", None)], "\n".join(parts))


def index_page(b: Bundle, census: list[Doc], built: list[str], n_defaults: int, declared_totals: dict) -> None:
    out = b.out / "index.html"
    by_path = {r["path"]: r for r in census}

    def page_of(path: str) -> str:
        return rel(b.root / "p" / path[:-4] / "index.html", out.parent)

    def row_of(path: str) -> str:
        r = by_path[path]
        cov, cp = r["theory"]["cover"], r["theory"]["coupling"]
        x = cov["exactly_one"]
        route = r["routes"]
        cells = [
            f'<td class="tag">{esc(TAG.get(path, ""))}</td>', f'<td><a href="{esc(page_of(path))}"><code>{esc(path)}</code></a></td>',
            f'<td>{"m" if r["monotone"] else "f"}</td>',
        ] + [f'<td class="num">{esc(v)}</td>' for v in (
            r["n"], r["dead_alone"], r["theory"]["small"]["pairs"]["excluded"], r["pairs"]["excluded"],
            r["theory"]["root_excluded"] - r["theory"]["root_excluded_of_theory"], cov["groups"], x["owed_groups"],
            len(x["grids"]), x["bijections"], cp["questions"], cp["edges"]["all"], cp["edges"]["of_three_or_more"],
            cp["components"], r["saturations"], route["describe"]["saturations"], route["single"]["saturations"])]
        return "<tr>" + "".join(cells) + "</tr>"

    head = ("<tr><th>tag</th><th>program</th><th>static check</th>" + "".join(
        f'<th class="num">{esc(h)}</th>' for h in (
            "candidates", "dead alone", "pairs the theory excludes", "pairs root excludes", "… the puzzle's",
            "groups", "owed", "grids", "bijections", "questions", "edges", "… of three or more", "components",
            "root's layer, saturations", "a describe", "--single")) + "</tr>")
    instances = [p for _, p in INSTANCES if p in built]
    stage = PHASE / "s1f.10.12_the_structure_shown.md"
    n_refused = sum(1 for r in census if "error" in r)
    n_search = sum(1 for r in census if r.get("searches"))
    dirs = [(d, what) for d, what in (
        ("prices", "S1f.10.13: the price list's tables, verdicts, losers and its jq programs"),
        ("control", "S1f.10.2's stopped run: the control's probe and script"),
        ("s2", "S1f.10.2: the cover's scripts"), ("s4", "S1f.10.4: its scripts, bijections.py, and the probes of T3"),
        ("s10", "S1f.10.10: the alternatives' censuses, the binary check"),
        ("s11", "S1f.10.11: the instances under every reading, the census before it"),
        ("s12", "S1f.10.12: the census as it was before this stage re-took it, and every file of it"),
    ) if (b.out / d).is_dir()]
    parts = [
        "<h1>The structure of the hypothesis set, shown</h1>",
        f"<p>What P1f.10's part 1 computed about a program's hypothesis set H, on real programs, by the engine, before "
        f"any surface of <code>ein</code> is designed around it: "
        f'<a href="{esc(rel(stage, out.parent))}">S1f.10.12</a>\'s bundle, for the phase\'s one stop '
        f'(<a href="{esc(rel(PHASE / "README.md", out.parent))}">the phase</a>). '
        f"<strong>Every artefact is here, raw and rendered</strong>: a page's tables are the engine's files, each with "
        f"its JSON beside it and the raw file named; its pictures are SVG with their DOT beside them. Nothing is "
        f"curated, and no number is this script's.</p>",
        b.provenance(out.parent),
        '<h2 id="pages">The pages</h2><ul>'
        f'<li><a href="bundle/survey.html"><strong>The survey</strong></a>: one row a corpus program, every count a '
        f"column, sortable by any; with <a href=\"bundle/survey.json\"><code>survey.json</code></a> and "
        f"<a href=\"bundle/survey.tsv\"><code>survey.tsv</code></a>.</li>"
        f'<li><a href="bundle/prices.html"><strong>The price list</strong></a>: the verdict table and the losers, '
        f'and <a href="bundle/prices_all.html">every table of it over every program</a>.</li>'
        f'<li><a href="bundle/defaults.html"><strong>Every default part 1 took</strong></a>: {n_defaults} sections, '
        f"verbatim, under their stages.</li>"
        f'<li><a href="bundle/must.html"><strong>Where <em>exactly one</em> is missing</strong></a>: S1f.10.14\'s T3, '
        f"what each program S1f.10.4 named would have to state, and its questions before and after.</li>"
        f'<li><a href="bundle/findings.html"><strong>The findings</strong></a>: <code>findings.md</code> whole.</li>'
        f'<li><a href="bundle/questions.html"><strong>The four questions\' material</strong></a>: the surface '
        f"(a slot), and Q-M1f.306, .307 and .308 with their witnesses.</li></ul>",
        '<h2 id="instances">The six instances</h2>',
        "<p>The phase's instances, first: the two encodings of the Zebra puzzle, the under-determined one, the blind "
        "enumerator's, the conflict of three, and SEND + MORE = MONEY.</p>",
        f'<div class="scroll"><table><thead>{head}</thead><tbody>{"".join(row_of(p) for p in instances)}</tbody></table></div>',
        f'<h2 id="programs">Every corpus program that searches</h2>',
        f"<p>{len(built)} pages, in the census's order. {len(census)} corpus files and register witnesses were read: "
        f"{n_search} search, {len(census) - n_search - n_refused} load and do not search and {n_refused} the engine "
        f"refuses; those have a row of <a href=\"bundle/survey.html\">the survey</a> and no page. <em>m</em> is "
        f"monotone by the static check, <em>f</em> flagged.</p>",
        f'<div class="scroll"><table><thead>{head}</thead><tbody>{"".join(row_of(p) for p in built)}</tbody></table></div>',
        '<h2 id="declared">Found and declared, over the corpus</h2>',
        "<p>S1f.10.4's T2 summed over the pages, by the script's reading by name (each page's § Beside what the "
        "program declares): relations declared bijective "
        f"{declared_totals.get('declared', 0)}; found and declared {declared_totals.get('found and declared', 0)}; "
        f"declared and not found {declared_totals.get('declared and not found', 0)}; declared with nothing left in H "
        f"{declared_totals.get('declared, and nothing of it left in H', 0)}; found and declared nowhere "
        f"{declared_totals.get('found and declared nowhere', 0)}.</p>",
        '<h2 id="raw">The stages\' own directories</h2>',
        "<p>Beside each program's directory, the scripts, probes and intermediate censuses the stages left, as they "
        "left them:</p><ul>"
        + "".join(f'<li><a href="{esc(d)}/"><code>build/structure/{esc(d)}/</code></a>: {esc(what)}</li>' for d, what in dirs)
        + f'<li><a href="census.jsonl"><code>census.jsonl</code></a> ({size_of(b.out / "census.jsonl")}) and '
          f'<a href="census.env"><code>census.env</code></a>: the census, one line a program.</li></ul>',
    ]
    b.write_page(out, "The structure of the hypothesis set, shown", [("the structure, shown", None)], "\n".join(parts))


# ── the check ────────────────────────────────────────────────────────────────

def check(out: Path) -> int:
    """Every page of the bundle: each link and image resolves, each fragment names an element,
    each table has its JSON and each picture its DOT. Prints what fails; 1 if anything does."""
    root = out / "bundle"
    pages = [out / "index.html"] + sorted(root.rglob("*.html"))
    if not (out / "index.html").exists() or not (root / MARKER).exists():
        print(f"error: {out}: no bundle here (run utils/structure_bundle.py first)", file=sys.stderr)
        return 1
    ids: dict[Path, set[str]] = {}
    texts: dict[Path, str] = {}

    def ids_of(page: Path) -> set[str]:
        if page not in ids:
            text = texts.get(page) or page.read_text()
            ids[page] = set(html.unescape(m) for m in re.findall(r'\sid="([^"]*)"', text))
        return ids[page]

    errors, links, outside = [], 0, 0
    for page in pages:
        text = page.read_text()
        texts[page] = text
        for m in re.finditer(r'<(?:a|link)\s[^>]*?href="([^"]*)"|<img\s[^>]*?src="([^"]*)"', text):
            url = html.unescape(m.group(1) if m.group(1) is not None else m.group(2))
            if re.match(r"[a-z][a-z0-9+.-]*:", url):
                outside += 1
                continue
            links += 1
            path, _, frag = url.partition("#")
            target = page if not path else (page.parent / path).resolve()
            if not target.exists():
                errors.append(f"{page}: `{url}` names no file")
                continue
            if frag and target.suffix == ".html" and (target == out / "index.html" or root in target.parents):
                if frag not in ids_of(target):
                    errors.append(f"{page}: `{url}`: no element of {target.name} has the id `{frag}`")
        texts.pop(page, None)
    svgs = dots = undrawn = twins = 0
    for svg in sorted(root.rglob("*.svg")):
        svgs += 1
        if svg.stat().st_size == 0:
            errors.append(f"{svg}: empty")
        if not svg.with_suffix(".dot").exists() and svg.name != "coupling.engine.svg":
            errors.append(f"{svg}: no DOT beside it")
    for dot in sorted(root.rglob("*.dot")):
        dots += 1
        if dot.with_suffix(".undrawn.txt").exists():
            undrawn += 1
        elif not dot.with_suffix(".svg").exists():
            errors.append(f"{dot}: neither drawn nor said to be undrawn")
    for twin in sorted(root.rglob("*.json")):
        twins += 1
        try:
            json.loads(twin.read_text())
        except json.JSONDecodeError as e:
            errors.append(f"{twin}: malformed JSON: {e}")
    for e in errors:
        print(f"error: {e}", file=sys.stderr)
    print(f"{len(pages)} pages, {links} links and images resolved against the tree ({outside} to another site, not "
          f"followed), {svgs} pictures, {dots} DOT files written here ({undrawn} kept undrawn), {twins} JSON files; "
          f"{len(errors)} error{'' if len(errors) == 1 else 's'}")
    return 1 if errors else 0


# ── the run ──────────────────────────────────────────────────────────────────

def take(args: list[str], flag: str) -> str | None:
    if flag not in args:
        return None
    i = args.index(flag)
    if i + 1 >= len(args):
        raise Fail(f"{flag} takes a value")
    value = args[i + 1]
    del args[i:i + 2]
    return value


def build(out: Path, ein: str, jobs: int, only: list[str]) -> int:
    b = Bundle(out, ein, jobs)
    census = load_jsonl(need(out / "census.jsonl"))
    b.census_env = b.env_block(need(out / "census.env"))
    b.prices_env = b.env_block(need(out / "prices/prices.env"))
    for f in ("prices/verdicts.md", "prices/prices.md", "prices/losers_named.md"):
        need(out / f)
    if not Path(ein).exists():
        raise Fail(f"{ein}: no such binary (build it, `./build.sh --engine`, or name one with --bin or $EIN_BIN)")
    try:
        version = subprocess.run([ein, "--version"], capture_output=True, text=True, timeout=60)
    except OSError as e:
        raise Fail(f"{ein}: {e}") from None
    if version.returncode != 0 or not version.stdout.strip():
        raise Fail(f"{ein} --version: exit {version.returncode}: {version.stderr.strip()}")
    b.version = version.stdout.strip()
    shared = {"verdicts": load_json(need(out / "prices/verdicts.json"))}
    b.must = load_json(need(out / "s14/must.json"))
    for e in b.must["programs"]:
        for c in e["cells"]:
            for side in ("before", "after"):
                need(REPO / c[side]["file"])
    for _, d, _, _, _ in TRACK_H:
        shared["h9" if d == "s1f.10.9" else d] = load_tsv(need(REPO / "build" / d / "census/rows.tsv"))
        shared[d] = shared["h9" if d == "s1f.10.9" else d]

    searching = [r for r in census if r.get("searches")]
    for r in census:
        if not re.fullmatch(r"[A-Za-z0-9_./-]+\.ein", r["path"]):
            raise Fail(f"{out / 'census.jsonl'}: a path a link cannot be written for as it is: {r['path']!r}")
    if only:
        wanted = {str(Path(p).resolve().relative_to(REPO)) if Path(p).is_absolute() else p for p in only}
        missing = wanted - {r["path"] for r in searching}
        if missing:
            raise Fail(f"{out / 'census.jsonl'}: no searching program {', '.join(sorted(missing))}")
        searching = [r for r in searching if r["path"] in wanted]

    if b.root.exists():
        if not (b.root / MARKER).exists():
            raise Fail(f"{b.root}: exists and is not this script's (no {MARKER} in it): it is not replaced")
        shutil.rmtree(b.root)
    b.root.mkdir(parents=True)
    (b.root / MARKER).write_text("written by utils/structure_bundle.py, which replaces this directory whole\n")
    (b.root / "style.css").write_text(CSS)

    extras: dict[str, dict] = {}
    paths = [r["path"] for r in searching]
    for k, r in enumerate(searching):
        P = Program(b, r, shared)
        try:
            program_page(b, P, paths[k - 1] if k else None, paths[k + 1] if k + 1 < len(paths) else None)
        except (KeyError, TypeError, IndexError, AttributeError, ValueError) as e:
            tb = e.__traceback__
            while tb.tb_next:
                tb = tb.tb_next
            raise Fail(f"{P.dir}: malformed input, {type(e).__name__}: {e} (read at "
                       f"structure_bundle.py:{tb.tb_lineno})") from e
        extra = {"cells": P.prices["cells"], "structure": P.prices["structure"],
                 "declared": len(P.bijections["declared"])}
        extra.update(P.bijections["counts"])
        extras[r["path"]] = extra
        if sys.stderr.isatty():
            print(f"\r{k + 1}/{len(searching)} programs", end="\n" if k + 1 == len(searching) else "",
                  file=sys.stderr, flush=True)

    totals = survey_page(b, census, extras, set(paths))
    whole_page(b, b.root / "prices.html", "The price list", "the price list",
               [(out / "prices/verdicts.md", "v-"), (out / "prices/losers_named.md", "l-")],
               f'<p>S1f.10.13\'s verdicts, as the example <code>price_list</code> printed them, and the losers of each '
               f'verdict named. Every table of the price list over every program is on '
               f'<a href="prices_all.html">its own page</a>; the verdicts raw are '
               f'<a href="{esc(rel(out / "prices/verdicts.json", b.root))}"><code>verdicts.json</code></a>, and a line '
               f'a program <a href="{esc(rel(out / "prices/prices.jsonl", b.root))}"><code>prices.jsonl</code></a>. '
               f'The record, with the rule written before the table: '
               f'<a href="{esc(rel(PHASE / "s1f.10.13_the_price_list.md", b.root))}#record-2026-11-16">S1f.10.13</a>.</p>')
    whole_page(b, b.root / "prices_all.html", "The price list, every table", "the price list, whole",
               [(out / "prices/prices.md", "")])
    whole_page(b, b.root / "findings.html", "The findings", "the findings", [(need(PHASE / "findings.md"), "")],
               "<p>What the phase found outside its stages: K-1 to K-13 of tracks S and R, and track H's H9-n, H6-n "
               "and H15-n.</p>")
    must_page(b, set(paths))
    n_defaults = defaults_page(b)
    questions_page(b)
    index_page(b, census, paths, n_defaults, totals)
    draw(b)
    (b.root / "bundle.json").write_text(json.dumps({
        "pages": b.pages, "tables": b.tables, "pictures": b.pictures,
        "undrawn": [{"dot": str(d.relative_to(REPO)), "why": why} for d, why in b.undrawn],
        "programs": paths, "declared_totals": totals,
    }, ensure_ascii=False, indent=1) + "\n")
    print(f"{out / 'index.html'}: {b.pages} pages ({len(paths)} programs), {b.tables} tables each with its JSON, "
          f"{b.pictures} pictures drawn, {len(b.undrawn)} kept as DOT and not drawn")
    return 0


def main(argv: list[str]) -> int:
    args = list(argv)
    if "-h" in args or "--help" in args:
        print(__doc__)
        return 0
    try:
        out = Path(take(args, "--out") or REPO / "build/structure").resolve()
        ein = take(args, "--bin") or os.environ.get("EIN_BIN") or str(REPO / "ein.rs/target/release/ein")
        jobs = int(take(args, "--jobs") or min(16, os.cpu_count() or 1))
        if "--check" in args:
            return check(out)
        unknown = [a for a in args if a.startswith("-")]
        if unknown:
            raise Fail(f"unknown option {unknown[0]} (see --help)")
        return build(out, ein, jobs, args)
    except Fail as e:
        print(f"error: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
