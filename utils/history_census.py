#!/usr/bin/env python3
"""How much of the tree speaks of its development: the history census.

Per tree, over the files `git ls-files` lists, it counts

- **files**: the tree's tracked files of its kind (`.md` for a docs tree, `.rs`
  for the crates, `.ein` for the programs), and **lines**, their newlines;
- **carries an id**: lines with a milestone, phase, stage or task id, by
  `check_release_voice.py`'s patterns, whole lines (a `.rs` file's code and
  strings too), a decision id (`Q-M…`) not counted;
- **refers into `plans/` or `docs/history/`**: lines with a markdown link that
  resolves there, from the file's directory, or a path written into either,
  bare or relative (`../../plans/…`, `docs/history/m1a_rust/…`);
- **dates**: lines with an ISO date, `YYYY-MM-DD`;

each as lines and as the files that have one. The summary line is the
milestone-id column over every tree but `docs/history/`, which is the history.
Then the crates' comment links into `docs/history/`, by sub-folder, and into
`plans/`; the tracked names outside `plans/` and `docs/history/` that carry an
id; and the release-voice check's own count over its scope, which fails on
any line.

    python3 utils/history_census.py          # the tables, as markdown
"""
from __future__ import annotations

import re
import sys
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPO / "utils"))
from check_release_voice import PATTERNS, counts, scope, tracked  # noqa: E402
from doc_audit import LINK, rust_comment_text  # noqa: E402

ID_KINDS = ("milestone", "phase", "stage", "task")
IDS = [pat for kind, pat in PATTERNS if kind in ID_KINDS]
DATE = dict(PATTERNS)["date"]
ESCAPE = re.compile(r"release-voice-ok:\s*\S")
HISTORY_PATH = re.compile(r"(?<![\w.-])(?:\.\./)*(?:plans|docs/history)/(?=[\w.])")
PLANS = REPO / "plans"
HISTORY = REPO / "docs" / "history"


def md_in(prefix: str):
    return lambda r: r.startswith(prefix) and r.endswith(".md")


ROWS = (
    ("`docs/ein`", md_in("docs/ein/")),
    ("`docs/history`", md_in("docs/history/")),
    ("`docs/lib`", md_in("docs/lib/")),
    ("`ein.rs/crates/**/*.rs`", lambda r: r.startswith("ein.rs/crates/") and r.endswith(".rs")),
    ("`stdlib` + `examples` + `tests` `.ein`",
     lambda r: r.startswith(("stdlib/", "examples/", "tests/")) and r.endswith(".ein")),
    ("`corpus/corpus.toml`", lambda r: r == "corpus/corpus.toml"),
    ("`examples` + `stdlib` + `utils` `README.md`",
     lambda r: r in ("examples/README.md", "stdlib/README.md", "utils/README.md")),
    ("`AGENTS.md`", lambda r: r == "AGENTS.md"),
    ("`README.md`", lambda r: r == "README.md"),
)


def resolves_into_history(src: Path, target: str) -> bool:
    if target.startswith(("http://", "https://", "mailto:", "#")):
        return False
    dest = (src.parent / target.split("#", 1)[0]).resolve()
    return dest.is_relative_to(PLANS) or dest.is_relative_to(HISTORY)


def line_marks(src: Path, line: str) -> tuple[bool, bool, bool]:
    """Does the line carry an id, refer into the history, carry a date?"""
    has_id = any(p.search(line) for p in IDS)
    refers = bool(HISTORY_PATH.search(line)) or any(
        resolves_into_history(src, m.group(2)) for m in LINK.finditer(line)
    )
    return has_id, refers, bool(DATE.search(line))


def census_rows(files: list[str]) -> list[tuple[str, list[int]]]:
    out = []
    for name, keep in ROWS:
        n_files = n_lines = 0
        lines = [0, 0, 0]
        hit_files = [0, 0, 0]
        for rel in files:
            if not keep(rel):
                continue
            path = REPO / rel
            if not path.is_file():
                continue
            text = path.read_text(errors="replace")
            n_files += 1
            n_lines += text.count("\n")
            seen = [False, False, False]
            for line in text.splitlines():
                for i, mark in enumerate(line_marks(path, line)):
                    if mark:
                        lines[i] += 1
                        seen[i] = True
            for i in range(3):
                hit_files[i] += seen[i]
        out.append((name, [n_files, n_lines, *lines, *hit_files]))
    return out


def crate_links(files: list[str]) -> tuple[Counter, int, int]:
    """Markdown links in the crates' comments that resolve into
    `docs/history/<sub>/`, per sub-folder, those into `m1a_rust/design/`, and
    those into `plans/`."""
    per: Counter = Counter()
    design = plans = 0
    for rel in files:
        if not (rel.startswith("ein.rs/crates/") and rel.endswith(".rs")):
            continue
        src = REPO / rel
        for m in LINK.finditer(rust_comment_text(src.read_text(errors="replace"))):
            target = m.group(2)
            if target.startswith(("http://", "https://", "#")):
                continue
            dest = (src.parent / target.split("#", 1)[0]).resolve()
            if dest.is_relative_to(HISTORY):
                parts = dest.relative_to(HISTORY).parts
                per[parts[0] if len(parts) > 1 else "(README.md)"] += 1
                design += parts[:2] == ("m1a_rust", "design")
            elif dest.is_relative_to(PLANS):
                plans += 1
    return per, design, plans


NAME_ID = re.compile(r"(?i)(?<![a-z0-9])[mpst]\d+[a-z]?\d*[._-]\d+")
FN_ID = re.compile(r"\bfn\s+(\w*?(?<![a-z0-9])[mpst]\d+[a-z]\d*_\d+\w*)")


def names_with_ids(files: list[str]) -> list[str]:
    out = []
    for rel in files:
        if rel.startswith(("plans/", "docs/history/")):
            continue
        if NAME_ID.search(Path(rel).name):
            out.append(rel)
        if rel.endswith(".rs"):
            for m in FN_ID.finditer((REPO / rel).read_text(errors="replace")):
                out.append(f"{rel}: fn {m.group(1)}")
    return out


def num(n: int) -> str:
    return f"{n:,}".replace(",", " ")


def files_(n: int) -> str:
    return f"{n} file" if n == 1 else f"{n} files"


def main() -> int:
    files = tracked()

    print("| tree | files | lines | carries an id | refers into `plans/` or `docs/history/` | dates |")
    print("|---|---:|---:|---:|---:|---:|")
    total_lines = total_files = 0
    for name, (n_files, n_lines, l_id, l_ref, l_date, f_id, f_ref, f_date) in census_rows(files):
        print(f"| {name} | {num(n_files)} | {num(n_lines)} | {num(l_id)} lines, {files_(f_id)} "
              f"| {num(l_ref)} lines, {files_(f_ref)} | {num(l_date)} lines, {files_(f_date)} |")
        if name != "`docs/history`":
            total_lines += l_id
            total_files += f_id
    print(f"\n**{num(total_lines)} lines across {total_files} files carry a milestone, phase, "
          "stage or task id**, every tree but `docs/history`.")

    per, design, plans = crate_links(files)
    print(f"\nThe crates' comment links into `docs/history/`: **{sum(per.values())}** — "
          + ", ".join(f"{n} into `{sub}`" for sub, n in per.most_common())
          + f"; {design} of them into `m1a_rust/design/`. Into `plans/`: {plans}.")

    names = names_with_ids(files)
    print(f"\nTracked names outside `plans/` and `docs/history/` that carry an id: {len(names)}"
          + ("" if not names else ":"))
    for n in names:
        print(f"- `{n}`")

    voice, _ = counts(scope())
    by: Counter = Counter()
    for rel, n in voice.items():
        if rel.startswith("docs/ein/"):
            by["`docs/ein/` pages"] += n
        elif rel.endswith(".rs"):
            by["crates' comments"] += n
        elif rel.endswith(".ein"):
            by["`.ein` comments"] += n
        else:
            by[f"`{rel}`"] += n
    print(f"\nThe release-voice check over its scope: **{num(sum(voice.values()))} "
          f"lines in {len(voice)} files** — "
          + ", ".join(f"{k} {num(n)}" for k, n in by.most_common()) + ".")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
