"""Shared plumbing: the binary, files, markdown, and DOT -> SVG."""

from __future__ import annotations

import concurrent.futures
import json
import os
import re
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]


def ein_bin(override: str | None = None) -> str:
    """`--bin`, then `$EIN_BIN`, then the release build — utils/'s rule."""
    return override or os.environ.get("EIN_BIN") or str(REPO / "ein.rs/target/release/ein")


@dataclass
class Invocation:
    """One process the bundle ran, and everything it left behind."""

    name: str
    argv: list[str]
    env: dict[str, str] = field(default_factory=dict)
    code: int = 0
    stdout: str = ""
    stderr: str = ""

    def shell(self) -> str:
        env = " ".join(f"{k}={v}" for k, v in self.env.items())
        return (env + " " if env else "") + " ".join(_quote(a) for a in self.argv)


def _quote(a: str) -> str:
    return a if re.fullmatch(r"[\w./:=+,@%-]+", a) else "'" + a.replace("'", "'\\''") + "'"


def run(name: str, argv: list[str], env: dict[str, str] | None = None, cwd: Path = REPO,
        timeout: float | None = None) -> Invocation:
    """Run one process from the repo root, capturing everything."""
    env = env or {}
    try:
        p = subprocess.run(argv, cwd=cwd, capture_output=True, text=True,
                           env={**os.environ, **env}, timeout=timeout)
        return Invocation(name, argv, env, p.returncode, p.stdout, p.stderr)
    except subprocess.TimeoutExpired as e:
        out = e.stdout.decode() if isinstance(e.stdout, bytes) else (e.stdout or "")
        return Invocation(name, argv, env, -2, out, f"(killed after {timeout} s)\n")


def write(path: Path, text: str) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")
    return path


def read(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except (FileNotFoundError, UnicodeDecodeError):
        return ""


def load_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (FileNotFoundError, json.JSONDecodeError):
        return None


def jsonl(path: Path) -> list[dict]:
    out = []
    for line in read(path).splitlines():
        if line.strip():
            try:
                out.append(json.loads(line))
            except json.JSONDecodeError:
                out.append({"e": "<unparsed>", "line": line})
    return out


def rel(target: Path, page: Path) -> str:
    """A link from `page` to `target`, relative, with forward slashes."""
    return os.path.relpath(target, page.parent).replace(os.sep, "/")


def doc(path: str, page: Path) -> str:
    """A link from `page` to a file in the repo."""
    return rel(REPO / path, page)


def rebase(text: str, source: str, page: Path) -> str:
    """Markdown quoted from the repo file `source`, its relative links
    re-aimed so they resolve from `page` instead."""
    def fix(m: re.Match) -> str:
        label, target = m.group(1), m.group(2)
        if target.startswith(("http://", "https://", "mailto:", "#")):
            return m.group(0)
        path, _, frag = target.partition("#")
        dest = os.path.normpath(os.path.join(os.path.dirname(source), path))
        return f"[{label}]({doc(dest, page)}{'#' + frag if frag else ''})"
    return re.sub(r"\[([^\]]*)\]\(([^)\s]+)\)", fix, text)


def warning_words(monotonicity: dict | None) -> str:
    """What the detector found on a flagged program's cube (M1e2 S1e2.1b.5), as
    the read-out's warning says it: the status, by the sites its violations are
    attributed to, and the state. Empty where the block has no status: a
    monotone program, a refused cube, or a dump from before the stage."""
    m = monotonicity or {}
    st = m.get("status") or {}
    bit = st.get("bit")
    if not bit:
        return ""
    by = st.get("by") or []
    out = f": status {bit}" + (f" (by {', '.join(by)})" if by else "")
    k = (m.get("state") or {}).get("stale_nodes")
    if k == 0:
        out += " · no stale fact"
    elif k is not None:
        out += f" · {k} node{'s hold' if k != 1 else ' holds'} a stale fact"
    return out


def size_of(path: Path) -> str:
    """`N lines` for a file, `N files` for a directory."""
    if path.is_dir():
        n = sum(1 for p in path.rglob("*") if p.is_file())
        return f"{n} files"
    if not path.exists():
        return "absent"
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        # A binary artefact (a `.einb` a command's run wrote, M1f S1f.5.19):
        # its size, where its lines would say 0.
        return f"{path.stat().st_size} bytes"
    return f"{text.count(chr(10))} lines"


def esc(s) -> str:
    """A table cell."""
    return str(s).replace("|", "\\|").replace("\n", " ")


def table(head: list[str], rows: list[list]) -> list[str]:
    out = ["| " + " | ".join(esc(h) for h in head) + " |", "|" + "---|" * len(head)]
    out += ["| " + " | ".join(esc(c) for c in r) + " |" for r in rows]
    return out


def header(title: str, what: str, flag: str, defined: list[tuple[str, str]],
           raw: list[Path], page: Path, extra: str = "") -> list[str]:
    """Every page opens the same way: what it is, which flag wrote it, which
    kernel page defines it, its raw file and its size."""
    links = ", ".join(f"[{t}]({doc(p, page)})" for t, p in defined) or "—"
    raws = ", ".join(f"[`{r.name}`]({rel(r, page)}) ({size_of(r)})" if r.exists()
                     else f"`{r.name}` (not written)" for r in raw) or "—"
    lines = [f"# {title}", "",
             f"> **What it is:** {what}  ",
             f"> **Written by:** {flag}  ",
             f"> **Defined in:** {links}  ",
             f"> **Raw:** {raws}  "]
    if extra:
        lines.append(f"> {extra}")
    return lines + [""]


def details(summary: str, body: list[str]) -> list[str]:
    return ["<details>", f"<summary>{summary}</summary>", "", *body, "", "</details>", ""]


# ── DOT ───────────────────────────────────────────────────────────────────────

def split_digraphs(text: str) -> list[tuple[str, str]]:
    """A stream of `digraph NAME { … }` blocks, as `(name, block)` pairs. A
    block ends where its braces balance, counted outside quoted strings and
    comments: until M1f S1f.5.19 it ended at the first line that read `}`,
    which is a `subgraph cluster_…`'s close in every rule view, so every
    `render rules` and `render program` rule the bundle drew was cut there and
    refused by Graphviz (654 graphs over the bundle's 145 cases of
    2026-11-09, each case's `consistency.md` counting them)."""
    out, cur, name, depth = [], [], None, 0
    for line in text.splitlines():
        if name is None:
            m = re.match(r"^(?:strict\s+)?(?:di)?graph\s+(\S+)?\s*\{", line)
            if not m:
                continue
            name, cur, depth = (m.group(1) or "graph").strip('"'), [line], _braces(line)
        else:
            cur.append(line)
            depth += _braces(line)
        if depth <= 0:
            out.append((name, "\n".join(cur) + "\n"))
            name, cur = None, []
    return out


def _braces(line: str) -> int:
    """`{` less `}` on a DOT line, outside a quoted string and a `//` or `#`
    comment."""
    if line.lstrip().startswith("#"):
        return 0
    n, quoted, i = 0, False, 0
    while i < len(line):
        c = line[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c == "/" and line[i + 1:i + 2] == "/":
            break
        elif c == "{":
            n += 1
        elif c == "}":
            n -= 1
        i += 1
    return n


def layout_for(dot: str) -> str:
    """The engine a renderer suggests in its `// suggested layout:` comment."""
    m = re.search(r"//\s*suggested layout:\s*(\w+)", dot)
    return m.group(1) if m and shutil.which(m.group(1)) else "dot"


@dataclass
class SvgJob:
    dot: str
    svg: Path


@dataclass
class SvgResult:
    svg: Path
    ok: bool
    error: str = ""


# Beside a DOT the bundle keeps: why it has no SVG. `.refused.txt` holds what
# Graphviz printed when it refused the graph (`render_svgs`), `.undrawn.txt`
# why the bundle did not ask it (`keep_undrawn`). `refused_dots` reads both.
REFUSED_NOTE = ".refused.txt"
UNDRAWN_NOTE = ".undrawn.txt"


def render_svgs(jobs: list[SvgJob]) -> list[SvgResult]:
    """Every DOT to SVG, in parallel. A refusal is kept — the DOT is written
    beside the SVG path as `.dot`, Graphviz's stderr beside it as
    `.refused.txt` (M1f S1f.5.29), the error is returned — never dropped,
    because a DOT Graphviz refuses is a `dot_wellformed.rs` gap, or a gap in
    the Graphviz that drew it: on Ubuntu 26.04 `fdp` is found and has no
    layout plugin behind it unless `libgvplugin-neato-layout8` is installed."""
    def one(j: SvgJob) -> SvgResult:
        j.svg.parent.mkdir(parents=True, exist_ok=True)
        j.svg.with_suffix(".dot").write_text(j.dot, encoding="utf-8")
        note = j.svg.with_suffix(REFUSED_NOTE)
        j.svg.unlink(missing_ok=True)
        note.unlink(missing_ok=True)
        p = subprocess.run([layout_for(j.dot), "-Tsvg", "-o", str(j.svg)],
                           input=j.dot, capture_output=True, text=True)
        ok = p.returncode == 0 and j.svg.exists() and j.svg.stat().st_size > 0
        if not ok:
            note.write_text(f"{layout_for(j.dot)} -Tsvg exit {p.returncode}\n{p.stderr.strip() or '(no message)'}\n",
                            encoding="utf-8")
        return SvgResult(j.svg, ok, p.stderr.strip())
    with concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count() or 4) as pool:
        return list(pool.map(one, jobs))


def keep_undrawn(dot_path: Path, dot: str, why: str) -> Path:
    """A DOT the bundle keeps and does not lay out, by design (a lattice past
    `DAG_NODES_DRAWN`), said so beside it, so `refused_dots` tells it from a
    refusal by what the bundle wrote and not by its size or its path."""
    write(dot_path, dot)
    write(dot_path.with_suffix(UNDRAWN_NOTE), why.rstrip() + "\n")
    return dot_path


def refused_dots(root: Path) -> list[tuple[Path, str]]:
    """Every DOT under `root` that Graphviz refused, with what it printed
    (M1f S1f.5.29): one with a `.refused.txt` beside it, or with no SVG and
    no `.undrawn.txt`, which no writer should leave. The smoke and the whole
    bundle fail on one; the nightly's bundle drew no KB view from 2026-11-06
    to S1f.5.29 and exited 0, because a refusal was a line on its page."""
    out = []
    for d in sorted(root.rglob("*.dot")):
        note = d.with_suffix(REFUSED_NOTE)
        if note.exists():
            out.append((d, note.read_text(encoding="utf-8").strip()))
        elif not d.with_suffix(".svg").exists() and not d.with_suffix(UNDRAWN_NOTE).exists():
            out.append((d, "(no SVG beside it, and nothing says why)"))
    return out


def image(svg: SvgResult, page: Path, alt: str) -> list[str]:
    """An SVG reference, its DOT one link away; a refusal as its text."""
    dot = svg.svg.with_suffix(".dot")
    if svg.ok:
        return [f"![{alt}]({rel(svg.svg, page)})", "",
                f"<sub>[DOT]({rel(dot, page)})</sub>", ""]
    return [f"**Graphviz refused this graph** ([DOT]({rel(dot, page)})):", "",
            "```text", svg.error or "(no message)", "```", ""]
