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


def size_of(path: Path) -> str:
    """`N lines` for a file, `N files` for a directory."""
    if path.is_dir():
        n = sum(1 for p in path.rglob("*") if p.is_file())
        return f"{n} files"
    if not path.exists():
        return "absent"
    text = read(path)
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
    """A stream of `digraph NAME { … }` blocks, as `(name, block)` pairs."""
    out, cur, name = [], [], None
    for line in text.splitlines():
        m = re.match(r"^(?:strict\s+)?(?:di)?graph\s+(\S+)?\s*\{", line)
        if m and name is None:
            name, cur = (m.group(1) or "graph").strip('"'), [line]
            continue
        if name is not None:
            cur.append(line)
            if line.strip() == "}":
                out.append((name, "\n".join(cur) + "\n"))
                name, cur = None, []
    return out


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


def render_svgs(jobs: list[SvgJob]) -> list[SvgResult]:
    """Every DOT to SVG, in parallel. A refusal is kept — the DOT is written
    beside the SVG path as `.dot`, the error is returned — never dropped,
    because a DOT Graphviz refuses is a `dot_wellformed.rs` gap."""
    def one(j: SvgJob) -> SvgResult:
        j.svg.parent.mkdir(parents=True, exist_ok=True)
        j.svg.with_suffix(".dot").write_text(j.dot, encoding="utf-8")
        p = subprocess.run([layout_for(j.dot), "-Tsvg", "-o", str(j.svg)],
                           input=j.dot, capture_output=True, text=True)
        ok = p.returncode == 0 and j.svg.exists() and j.svg.stat().st_size > 0
        return SvgResult(j.svg, ok, p.stderr.strip())
    with concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count() or 4) as pool:
        return list(pool.map(one, jobs))


def image(svg: SvgResult, page: Path, alt: str) -> list[str]:
    """An SVG reference, its DOT one link away; a refusal as its text."""
    dot = svg.svg.with_suffix(".dot")
    if svg.ok:
        return [f"![{alt}]({rel(svg.svg, page)})", "",
                f"<sub>[DOT]({rel(dot, page)})</sub>", ""]
    return [f"**Graphviz refused this graph** ([DOT]({rel(dot, page)})):", "",
            "```text", svg.error or "(no message)", "```", ""]
