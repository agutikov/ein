#!/usr/bin/env python3
"""Code, comments and docs speak as a released system, not as a development log.

A released system's documentation states what is true of it now. A line that
names the work which produced it — a milestone, phase, stage or task id, a path
into `plans/`, the date a ruling was made or a measurement taken — reads only
while the plan it points at exists, and plans are deleted when finished. This
check finds those lines where the released tree keeps its prose:

- every `.md` under `docs/ein/` and `docs/decisions/`;
- every doc comment (`///`, `//!`) of a shipped crate: `ein-core`, `ein-ir`,
  `ein-infer`, `ein-einb`, `ein-render`, `ein-cli`, their tests, examples and
  benches included;
- every comment of a tracked `.ein` file outside `plans/` and `docs/history/`
  (the text after a `;` outside a string, read by `doc_audit.py`'s reader);
- `corpus/corpus.toml`, whole;
- `README.md`; not `AGENTS.md` (and so `CLAUDE.md`), which guides the
  development and is part of it, as `plans/` and `utils/` are;
- this script, which holds itself to the rule it states.

A line fails on any of

- an id of a milestone, phase, stage or task, the letter forms too:
  `M2`, `M1e2`, `P2.3`, `S2.6.4`, `S1e2.1c.5`, `T2.6.4.1`;  # release-voice-ok: what fails
- a path into `plans/`, written bare or relative (`../../plans/m2_nl_to_ir/`);  # release-voice-ok: what fails
  the folder named alone, `plans/`, passes;
- an ISO date, `YYYY-MM-DD`: in this scope a date dates a development event.

**A decision id passes**: `Q-M1f.304` names a decision, which a page or a  # release-voice-ok: the form that passes
comment may cite. A line carrying `release-voice-ok: <reason>` passes too,
the reason required, as `determinism-ok:` is in `check_hashmap_iteration.py`.

**It is a ratchet.** The tree does not pass it yet, so it reads a banked
baseline, `utils/release_voice_baseline.txt`, of each file's count of failing
lines, and fails where a count rises or a file it does not list has one. A
count that falls is reported, and the commit that lowers it re-banks the
baseline (`--bless`); when every count is 0 the baseline is empty and goes.

    python3 utils/check_release_voice.py               # exit 1 on a rise or a new file
    python3 utils/check_release_voice.py --list        # every failing line, file:line
    python3 utils/check_release_voice.py --list README.md ein.rs/crates/ein-ir
    python3 utils/check_release_voice.py --bless       # re-bank the baseline
"""
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPO / "utils"))
from doc_audit import ein_comment_text  # noqa: E402

BASELINE = REPO / "utils" / "release_voice_baseline.txt"
SELF = Path(__file__).resolve()

#: The crates that ship; `ein-corpus` and `ein-narration` are dev-only.
SHIPPED = ("ein-core", "ein-ir", "ein-infer", "ein-einb", "ein-render", "ein-cli")


#: A milestone number: `1`, `10`, `1f`, `1e2`. An id is not preceded by a word
#: character or a `-`, which is what lets a decision id's `Q-M…` through.
_N = r"\d+(?:[a-z]\d*)?"
_PART = r"\.\d+[a-z]?"
PATTERNS: tuple[tuple[str, re.Pattern], ...] = (
    ("milestone", re.compile(r"(?<![\w-])M" + _N + r"(?![\w])")),
    ("phase", re.compile(r"(?<![\w-])P" + _N + _PART + r"(?![\w])")),
    ("stage", re.compile(r"(?<![\w-])S" + _N + _PART + r"(?:" + _PART + r")*(?![\w])")),
    ("task", re.compile(r"(?<![\w-])T" + _N + _PART + r"(?:" + _PART + r")+(?![\w])")),
    ("plans path", re.compile(r"(?<![\w.-])(?:\.\./)*plans/(?=[\w.])")),
    ("date", re.compile(r"(?<!\d)20\d\d-[01]\d-[0-3]\d(?!\d)")),
)
ESCAPE = re.compile(r"release-voice-ok:\s*\S")

#: A full-line doc comment of a Rust source file.
DOC_COMMENT = re.compile(r"^\s*//[/!]")


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
    if rel.startswith(("docs/ein/", "docs/decisions/")) and rel.endswith(".md"):
        return True
    if rel.endswith(".rs") and any(rel.startswith(f"ein.rs/crates/{c}/") for c in SHIPPED):
        return True
    if rel.endswith(".ein") and not rel.startswith(("plans/", "docs/history/")):
        return True
    return rel in ("corpus/corpus.toml", "README.md")


def read_lines(path: Path) -> list[str]:
    """The text of each line that is read, `""` for one that is not, so that a
    finding's line number is the file's."""
    text = path.read_text(errors="replace")
    if path.suffix == ".rs":
        return [line if DOC_COMMENT.match(line) else "" for line in text.splitlines()]
    if path.suffix == ".ein":
        return ein_comment_text(text).splitlines()
    return text.splitlines()


def scope() -> list[Path]:
    files = {(REPO / rel).resolve() for rel in tracked() if in_scope(rel)}
    files = {p for p in files if p.is_file()}
    files.add(SELF)
    return sorted(files)


def scan(path: Path) -> list[tuple[int, str, str, str]]:
    """Every failing line of one file, as (line, kind, text, the line)."""
    out = []
    for i, line in enumerate(read_lines(path), start=1):
        for kind, text in findings_of(line):
            out.append((i, kind, text, line))
    return out


def counts(files: list[Path]) -> tuple[dict[str, int], dict[str, list]]:
    per: dict[str, int] = {}
    hits: dict[str, list] = {}
    for path in files:
        found = scan(path)
        if found:
            rel = path.relative_to(REPO).as_posix()
            hits[rel] = found
            per[rel] = len({line for line, *_ in found})
    return per, hits


# ── the baseline ────────────────────────────────────────────────────────────

HEADER = """\
# The release-voice check's banked baseline: per file, the lines that still
# carry a milestone, phase, stage or task id, a path into plans/ or a date.
# Written by `python3 utils/check_release_voice.py --bless`; read by the
# check, which fails where a file's count rises or a file not listed here
# has one. A count only falls: the commit that lowers it re-banks this file,
# and when every count is 0 the file is deleted with the baseline's reader.
"""


def read_baseline() -> dict[str, int] | None:
    if not BASELINE.exists():
        return None
    out = {}
    for line in BASELINE.read_text().splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        n, path = line.split(None, 1)
        out[path.strip()] = int(n)
    return out


def write_baseline(per: dict[str, int]) -> None:
    body = "".join(f"{n:>5}  {path}\n" for path, n in sorted(per.items()))
    BASELINE.write_text(HEADER + body)


# ── main ────────────────────────────────────────────────────────────────────


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--list", nargs="*", metavar="PATH",
                    help="print every failing line, of the files under each PATH (default all)")
    ap.add_argument("--bless", action="store_true", help="re-bank the baseline from the tree")
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

    if args.bless:
        old = read_baseline()
        write_baseline(per)
        print(f"wrote {BASELINE.relative_to(REPO)}: {total} line(s) in {len(per)} file(s)")
        # A bless that raises a count is a regression banked: say which.
        for p in sorted(per) if old is not None else []:
            if per[p] > old.get(p, 0):
                print(f"  rose: {p} {old.get(p, 0)} → {per[p]}")
        return 0

    base = read_baseline()
    if base is None:
        print(f"no {BASELINE.relative_to(REPO)}: write it with --bless", file=sys.stderr)
        return 2
    rose = [(p, base[p], n) for p, n in sorted(per.items()) if p in base and n > base[p]]
    new = [(p, n) for p, n in sorted(per.items()) if p not in base]
    fell = [(p, b, per.get(p, 0)) for p, b in sorted(base.items()) if per.get(p, 0) < b]

    if rose or new:
        print("release voice: lines that speak of the development, above the baseline:",
              file=sys.stderr)
        for p, b, n in rose:
            print(f"  {p}: {n} line(s), the baseline {b}", file=sys.stderr)
        for p, n in new:
            print(f"  {p}: {n} line(s), and not in the baseline", file=sys.stderr)
        print(f"\n  python3 utils/check_release_voice.py --list {' '.join(p for p, *_ in rose + new)}",
              file=sys.stderr)
        print("\nState the behaviour, or cite a decision id, instead of the plan, the id or\n"
              "the date; a line that has to keep one carries `release-voice-ok: <why>`.",
              file=sys.stderr)
        return 1
    print(f"release voice ok — {len(files)} file(s) read, {total} line(s) in {len(per)} "
          f"file(s), none above {BASELINE.relative_to(REPO)}")
    if fell:
        print(f"  {len(fell)} file(s) below the baseline; lower it with --bless:")
        for p, b, n in fell:
            print(f"    {p}: {b} → {n}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
