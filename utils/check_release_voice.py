#!/usr/bin/env python3
"""The project speaks as a released system, and names nothing of the process.

The project is `README.md`, `ein.rs/` with its tests, `stdlib/`, `examples/`,
`tests/`, `corpus/` and `docs/` but `docs/lib/`. The process is the plans, the
scripts, the gate and CI and the agents' guide; it reads the project, which it
sees in many states, and nothing in the project reads, links or names it. A
released system's documentation states what is true of it now. A line that
names the work which produced it reads only while the process keeps what it
points at, and a plan is deleted when it is finished. This check finds those
lines wherever the project keeps prose:

- every `.md` file of the project;
- every comment of a crate's `.rs` file, a whole line or the end of one, doc
  comments and plain ones, block comments too, of every crate under
  `ein.rs/crates/`, the six that ship and the two dev-only ones, their tests,
  examples and benches included; a whole line's `//` comment of the grammar's
  sources, `ein.l` and `ein.y`; and a `#` comment of every `.toml` under
  `ein.rs/` (the workspace's, each crate's, `deny.toml`, the toolchain's);
- every comment of a `.ein` file of the project (the text after a `;`
  outside a string, read by `doc_audit.py`'s reader);
- `corpus/corpus.toml` and `corpus/regressions.toml`, whole.

A line fails on any of

- an id of a milestone, phase, stage or task, the letter forms too, as
  `M2`, `M1e2`, `P2.3`, `S2.6.4`, `S1e2.1c.5`, `T2.6.4.1` are;
- an id a plan files its work under: a finding (`E-33`, `K-82`, `L-7`), a
  followup (`F21`) or a review's (`CD-H1`, `TE-L3`);
- a case of the wrong-answer register, `W62`, or one of its rows,
  `W8/features-07`: the project names a regression by its name in
  `corpus/regressions.toml`, and an open wrong answer is the process's;
- a path into `plans/` or `utils/`, written bare or relative
  (`../../plans/m2_nl_to_ir/`), or either folder named alone, and the gate's,
  CI's and the agents' files: `run_tests.sh`, `.github/`, `AGENTS.md`,
  `CLAUDE.md`;
- an ISO date, `YYYY-MM-DD`: in this scope a date dates a development event.

**A decision id passes**: `Q-M1f.304` names a decision, which a page or a
comment may cite. A line carrying `release-voice-ok: <reason>` passes too, the
reason required, as `determinism-ok:` is in `check_hashmap_iteration.py`: a
test's list of the strings no output may contain, say. There is no baseline:
the check fails on any line it finds.

    python3 utils/check_release_voice.py               # exit 1 on any finding
    python3 utils/check_release_voice.py --list        # every failing line, file:line
    python3 utils/check_release_voice.py --list README.md ein.rs/crates/ein-ir
"""
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPO / "utils"))
from doc_audit import RS_CHAR, RS_RAW, ein_comment_text  # noqa: E402

#: What is not the project: the process (the plans, the scripts, the gate and
#: CI, the agents' guide), what is external to both, and third-party source.
NOT_PROJECT = (
    "plans/", "utils/", ".github/", "run_tests.sh", "AGENTS.md", "CLAUDE.md",
    "c/", "nl2smt/", "docs/lib/", "ein.rs/vendor/",
)
#: The project's data read whole, beside its `.md` files.
WHOLE = ("corpus/corpus.toml", "corpus/regressions.toml")


#: A milestone number: `1`, `10`, `1f`, `1e2`. An id is not preceded by a word
#: character or a `-`, which is what lets a decision id's `Q-M…` through.
_N = r"\d+(?:[a-z]\d*)?"
_PART = r"\.\d+[a-z]?"
PATTERNS: tuple[tuple[str, re.Pattern], ...] = (
    ("milestone", re.compile(r"(?<![\w-])M" + _N + r"(?![\w])")),
    ("phase", re.compile(r"(?<![\w-])P" + _N + _PART + r"(?![\w])")),
    ("stage", re.compile(r"(?<![\w-])S" + _N + _PART + r"(?:" + _PART + r")*(?![\w])")),
    ("task", re.compile(r"(?<![\w-])T" + _N + _PART + r"(?:" + _PART + r")+(?![\w])")),
    ("finding", re.compile(r"(?<![\w-])[EKL]-\d+(?![\w])")),
    ("followup", re.compile(r"(?<![\w-])F\d{1,2}(?![\w])")),
    ("review", re.compile(r"(?<![\w-])[A-Z]{2}-[HML]\d+(?![\w])")),
    ("case", re.compile(r"(?<![\w-])W-?\d{1,3}(?:/[\w-]+(?:\.[\w-]+)*)?(?![\w])")),
    ("plans path", re.compile(r"(?<![\w./-])(?:\.\./)*plans/")),
    ("process path", re.compile(
        r"(?<![\w./-])(?:\.\./)*(?:utils/|run_tests\.sh(?![\w])|\.github/"
        r"|(?:AGENTS|CLAUDE)\.md(?![\w]))")),
    ("date", re.compile(r"(?<!\d)20\d\d-[01]\d-[0-3]\d(?!\d)")),
)
ESCAPE = re.compile(r"release-voice-ok:\s*\S")


def findings_of(line: str) -> list[tuple[str, str]]:
    """What in one line fails the voice, as (kind, text); empty if it passes."""
    if ESCAPE.search(line):
        return []
    return [(kind, m.group(0)) for kind, pat in PATTERNS for m in pat.finditer(line)]


# ── the scope ───────────────────────────────────────────────────────────────


def tracked() -> list[str]:
    out = subprocess.run(
        ["git", "-C", str(REPO), "ls-files", "-z"], capture_output=True, text=True, check=True
    ).stdout
    return [p for p in out.split("\0") if p]


def in_scope(rel: str) -> bool:
    if rel.startswith(NOT_PROJECT):
        return False
    if rel.endswith((".md", ".ein")) or rel in WHOLE:
        return True
    if rel.startswith("ein.rs/crates/") and rel.endswith((".rs", ".l", ".y")):
        return True
    return rel.startswith("ein.rs/") and rel.endswith(".toml")


def rust_comments(text: str) -> str:
    """Every comment of a Rust source, all else blanked to spaces, its
    newlines kept, so a line number is the file's: a comment at a line's start
    or after code, a block comment nested; a string or a char literal is
    code."""
    out: list[str] = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        keep = False
        if text.startswith("//", i):
            j, keep = text.find("\n", i), True
            j = n if j < 0 else j
        elif text.startswith("/*", i):
            depth, j, keep = 1, i + 2, True
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
        elif c in "bcr" and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")) \
                and (m := RS_RAW.match(text, i)):
            close = '"' + m.group(1)
            j = text.find(close, m.end())
            j = n if j < 0 else j + len(close)
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j = min(j + 1, n)
        else:
            m = RS_CHAR.match(text, i) if c == "'" else None
            j = m.end() if m else i + 1
        out.append(text[i:j] if keep else re.sub(r"[^\n]", " ", text[i:j]))
        i = j
    return "".join(out)


def read_lines(path: Path) -> list[str]:
    """The text of each line that is read, `""` for one that is not, so that a
    finding's line number is the file's."""
    text = path.read_text(errors="replace")
    if path.suffix == ".rs":
        return rust_comments(text).split("\n")
    if path.suffix in (".l", ".y"):
        return [line if line.lstrip().startswith("//") else "" for line in text.splitlines()]
    if path.suffix == ".toml" and path.relative_to(REPO).as_posix() not in WHOLE:
        return [line[line.index("#"):] if "#" in line else "" for line in text.splitlines()]
    if path.suffix == ".ein":
        return ein_comment_text(text).splitlines()
    return text.splitlines()


def scope() -> list[Path]:
    files = {(REPO / rel) for rel in tracked() if in_scope(rel)}
    return sorted(p for p in files if p.is_file() and not p.is_symlink())


def scan(path: Path) -> list[tuple[int, str, str, str]]:
    """Every failing line of one file, as (line, kind, text, the line)."""
    out = []
    for i, line in enumerate(read_lines(path), start=1):
        for kind, text in findings_of(line):
            out.append((i, kind, text, line))
    return out


def counts(files: list[Path]) -> tuple[dict[str, int], dict[str, list]]:
    """Per file, its count of failing lines, and its findings."""
    per: dict[str, int] = {}
    hits: dict[str, list] = {}
    for path in files:
        found = scan(path)
        if found:
            rel = path.relative_to(REPO).as_posix()
            hits[rel] = found
            per[rel] = len({line for line, *_ in found})
    return per, hits


# ── main ────────────────────────────────────────────────────────────────────


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--list", nargs="*", metavar="PATH",
                    help="print every failing line, of the files under each PATH (default all)")
    args = ap.parse_args()

    files = scope()
    per, hits = counts(files)
    total = sum(per.values())

    if args.list is not None:
        keep = [a.rstrip("/") for a in args.list]
        for rel in sorted(hits):
            if keep and not any(rel == k or rel.startswith(k + "/") for k in keep):
                continue
            for line, kind, text, whole in hits[rel]:
                print(f"{rel}:{line}: {kind} `{text}` — {whole.strip()[:120]}")
        return 0

    if per:
        print("release voice: lines of the project that speak of the development or name the "
              "process:", file=sys.stderr)
        for rel, n in sorted(per.items()):
            print(f"  {rel}: {n} line(s)", file=sys.stderr)
        print(f"\n  {total} line(s) in {len(per)} file(s); "
              "python3 utils/check_release_voice.py --list PATH prints them", file=sys.stderr)
        print("\nState the behaviour, name the regression or cite a decision id, instead of\n"
              "the plan, the id, the path or the date; a line that has to keep one carries\n"
              "`release-voice-ok: <why>`.", file=sys.stderr)
        return 1
    print(f"release voice ok — {len(files)} file(s) read, none speaks of the development")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
