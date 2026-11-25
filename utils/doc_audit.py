#!/usr/bin/env python3
"""Does a documentation page still describe the engine that ships? (M1e S1e.2.2)

`docs/ein/` is the only statement of intent in this repo that is not also
the implementation, so a claim there is checked by `cargo test --workspace` and
by nothing else — which means a page nothing runs is a page nothing checks.
M1a's S1a.10.6 was a doc pass and it missed **every** page of `CD-H1`, for a reason
worth naming: a pass driven by *what the milestone changed* cannot catch a page
describing machinery removed two milestones ago, because nothing in the
milestone touched it.

This is the milestone-independent half. It asks three mechanical questions of
every page, none of which needs to know what changed:

    python3 utils/doc_audit.py                  # all three, as a report
    python3 utils/doc_audit.py --links --check --tracked   # the gate step, decision ids too
    python3 utils/doc_audit.py --inbound --tracked [--target docs/ein/]
    python3 utils/doc_audit.py --identifiers -k ein.rs/06-lattice-dump.md

The default tree is `docs/ein`, which is what the three questions are
*about*. The **links** half is not — a relative link resolves or it does not
wherever it is written — so it takes any number of directories and files, or
`--tracked`: every file `git ls-files` lists of the four kinds it reads.

**links** reads four kinds of referrer, and exits 1 under `--check` on a
target that does not exist or an anchor no heading of the target has, naming
the referrer and its line:

- **`.md`**: every markdown link, relative to the page, a reference-style
  definition (`[label]: target`) too;
- **`.rs`**: every markdown link in a `//`, `///` or `//!` comment line,
  relative to the source file — the `](../../../docs/…)` form a crate cites a
  page by, which `cargo doc` does not resolve. `ein.rs/vendor/` is not read:
  its links are rustdoc's own, relative to the rendered HTML;
- **`.ein`**: every **bare path** in a comment, the text after a `;` outside a
  string;
- **`corpus/corpus.toml`**: every bare path in the file.

A bare path starts at a top-level directory of the repository
(`docs/ein/README.md`), or at a crate's directory, which the repo writes
short for `ein.rs/crates/<crate>/` (`ein-cli/tests/corpus_cli.rs`), and names
a file with an extension or a directory with a trailing `/`: `tests/stdlib/
typing/03` is a prefix, not a path, and `ein-corpus/2` a schema. A path with
`*`, `<` or `{` in it is a pattern and is not read; a missing one git would
ignore (`ein.rs/target/…`, `build/…`) names an output, not a file of the tree,
and is not a finding. A directory argument walks `.md` pages only, so a built
tree such as the review bundle is read as pages and nothing else.

**decision ids**, under `--links --tracked`: every decision id the
**project** cites resolves to exactly one decision file,
`docs/decisions/**/<id>-<slug>.md`, and no id has two. The project is what the
process is not: `plans/`, `utils/`, the gate and CI (`run_tests.sh`,
`.github/`) and `AGENTS.md` are the process, `c/`, `nl2smt/` and `docs/lib/`
are external, and `docs/history/` and `ein.rs/vendor/` are not read. EinAf's
pages, `docs/einaf/`, are read as the project's are, on purpose: a decision
they cite is one of Ein's, and resolves to its file; EinAf's own questions are
answered in its pages, not cited by id. What is read of the rest:

- every project `.md`, whole;
- every `.rs` under `ein.rs/crates/`: its comments (`//`, `///`, `//!`,
  `/* */`, at a line's start or after code) and its string literals, raw and
  byte strings too, since the binary prints some;
- every `.ein`'s comments, the text after a `;` outside a string;
- `corpus/*.toml`, whole.

An id is read in two shapes:

- **`Q-M<milestone>.<n>`**, `Q-M1e2.24`, and every `.n` that continues it in
  a list or a range written short — `Q-M1f.135, .141 and .142`,
  `Q-M1f.315 to .340` — each the same milestone's id (a range's two ends are
  read, not what lies between);
- **a bare `Q<n>`**, the global ledger's numbering, `Q23`, where it is
  written as a decision id. It is not one where it is part of a longer name
  (`Q1.5b.7`, `S1f.5.4#Q1`, `Q23@M2`), and not where the words beside it say
  which other ledger numbered it: the word before it is a possessive (`the
  review's Q2`, `S1f.5.4's Q4`), a followup or milestone id (`F4 Q30`,
  `M1 Q26`), a stage id and a slash (`S1e.1.3 / Q2`) or a file
  (`f4_cross_cutting.md Q40`); it is followed by `of <file>.md`; or it opens a
  quoted reply, `"Q3 - as recommended"`. The second end of a range,
  `Q1–Q10`, is read as its first is. Such a number is a plan's local number,
  not a decision id, and removing it is the plan-reference sweep's.

A finding names the referrer, its line and the id: *no decision file* where
none matches, *N decision files* where more than one does. A file under
`docs/decisions/` that is not a `README.md` and whose name is not
`<id>-<slug>.md` is a finding too, and so is an id two files carry. One
short allow-list, `DECISION_IDS_ALLOWED`, lets an id stand in a file where
re-aiming it would move a golden, each row with why and removed by the stage
that re-aims the engine's own ids; a row whose file no longer cites its id is
a finding.

**inbound** counts, for every file a reference resolves to, the references to
it by referrer kind, with the `.md` ones from `plans/` and `docs/history/`
counted apart: the count a page's move has to rewrite in the commit that
moves it. `--target PREFIX` keeps the targets under a prefix.

Each markdown link resolves, file *and* `#anchor`, with
GitHub's slugification, **and stays inside the repository**: a target that
escapes it is a finding on its own, because `exists()` on one would answer for
the machine and not for the tree (three `../../acva/…` links passed this check
on the author's workstation and failed CI, 2026-09-01). Plus the check this
script exists because nothing had:
a **prose section reference** — ``[`page.md`](page.md) §3d.vii`` — is not a
link, so no anchor checker sees it, and S1e.2.2 found four such numbers cited
six times into a file that has never had any of them.

**identifiers** — every backticked token that looks like code (`EIN_*`,
`foo.rs`, `fn()`, `Type`, `snake_case`, `a::b`) resolved against
`ein.rs/crates/**`. This is the one that found four pages `CD-H1` does not
list, and it is **noisy on purpose**: it reports rather than fails, because
`Human`, `rel_can` and DOT node ids are not identifiers and no rule
distinguishes them from ones that are. A human skims the list; the signal is a
name that *looks* like the engine's and is not there.

**states** — a page of `docs/ein/` is current or it is deleted: none keeps a
banner saying it is superseded or historical. *current* carries no banner, so
what this reports is the pages that **do** carry one, which under `docs/ein/`
should be none.

**The links half is a gate step**, `--links --check --tracked`, the decision
ids with it. The other two report and do not fail: `--check` exits 1 on the
unambiguous half alone (links, prose §-references and decision ids).

What this cannot check, and what `AGENTS.md` asks a reader to do by hand:
run the commands a page shows. Four of S1e.2.2's findings were *invocations* — a CLI line producing
neither artifact it names, `(instnce ?a ?T)` claimed to be a parse error — and
the only instrument for those is a shell.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path
from urllib.parse import unquote

REPO = Path(__file__).resolve().parents[1]
CRATES = REPO / "ein.rs" / "crates"
DEFAULT_TREE = REPO / "docs" / "ein"
#: Third-party source, edited only through a patch: its doc links are
#: rustdoc's, relative to the rendered HTML rather than to the file.
VENDOR = REPO / "ein.rs" / "vendor"
CORPUS_TOML = REPO / "corpus" / "corpus.toml"

TICK = re.compile(r"`([^`\n]+)`")
LINK = re.compile(r"\[([^\]]*)\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
#: A reference-style link's definition, `[label]: target`, at a line's start
#: (up to three spaces in). A target in backticks is code, not a link, and a
#: footnote's `[^n]:` is not a link either.
REF_DEF = re.compile(r"(?m)^[ \t]{0,3}\[([^\]^\n][^\]\n]*)\]:[ \t]*<?([^\s>`][^\s>]*)>?")
#: ``[`page.md`](page.md) §3d.vii`` and ``… § 3a + § 3d.iv`` — a section
#: reference in prose, immediately after a link to the page it is about, and
#: before the next link, whose own label may carry one for a different page.
PROSE_SECTION = re.compile(r"§\s*([0-9][0-9a-z.]*)")
#: A banner that puts the page in a state other than *current*.
STATE_BANNER = re.compile(
    r"^>\s*\*\*(?:Status|Historical|Superseded)[^*]*\*\*", re.MULTILINE
)


# ── the crates, as one searchable blob ──────────────────────────────────────


def haystack() -> tuple[str, set[str]]:
    parts = []
    for pat in ("*.rs", "*.toml"):
        for p in sorted(CRATES.rglob(pat)):
            parts.append(p.read_text(errors="replace"))
    names = {p.name for p in REPO.rglob("*") if ".git/" not in str(p)}
    return "\n".join(parts), names


def classify(tok: str) -> str | None:
    """Which shape of identifier is this, if any? `None` means prose."""
    t = tok.strip()
    if not t:
        return None
    if re.fullmatch(r"EIN_[A-Z0-9_]+", t):
        return "env"
    if re.fullmatch(r"[A-Za-z0-9_]+\.rs", t):
        return "file"
    if re.fullmatch(r"[a-z_][a-z0-9_]*\(\)", t):
        return "fn"
    if re.fullmatch(r"[A-Z][A-Za-z0-9]*", t) and not t.isupper() and len(t) > 3:
        return "type"
    if re.fullmatch(r"[a-z][a-z0-9]*(_[a-z0-9]+)+", t):
        return "snake"
    if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*::[A-Za-z0-9_:]+", t):
        return "path"
    return None


def resolves(tok: str, kind: str, hay: str, files: set[str]) -> bool:
    t = tok.strip()
    if kind == "file":
        return t in files
    needle = t[:-2] if kind == "fn" else t.split("::")[-1] if kind == "path" else t
    return re.search(r"\b" + re.escape(needle) + r"\b", hay) is not None


# ── links and anchors ───────────────────────────────────────────────────────


def slugify(heading: str) -> str:
    """GitHub's heading → fragment, near enough for a repo of ASCII headings."""
    h = re.sub(r"^#+\s*", "", heading.strip())
    h = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", h).replace("`", "")
    h = re.sub(r"[*]", "", h).lower()
    h = re.sub(r"[^\w\s-]", "", h)
    return h.strip().replace(" ", "-")


def anchors_of(path: Path, _cache: dict[Path, set[str]] = {}) -> set[str]:
    if path in _cache:
        return _cache[path]
    out: set[str] = set()
    try:
        text = path.read_text(errors="replace")
    except OSError:
        _cache[path] = out
        return out
    fence = False
    for line in text.splitlines():
        if line.lstrip().startswith("```"):
            fence = not fence
            continue
        if fence:
            continue
        if line.startswith("#"):
            s = base = slugify(line)
            n = 1
            while s in out:
                s, n = f"{base}-{n}", n + 1
            out.add(s)
        m = re.search(r'<a\s+(?:id|name)="([^"]+)"', line)
        if m:
            out.add(m.group(1))
    _cache[path] = out
    return out


def section_numbers_of(path: Path, _cache: dict[Path, set[str]] = {}) -> set[str]:
    """Every `#### 3d.` / `##### 3c.ii.` style number a page's headings carry."""
    if path in _cache:
        return _cache[path]
    out: set[str] = set()
    try:
        text = path.read_text(errors="replace")
    except OSError:
        _cache[path] = out
        return out
    for line in text.splitlines():
        if not line.startswith("#"):
            continue
        m = re.match(r"#+\s*§?\s*([0-9][0-9a-zA-Z.]*?)\.?\s+\S", line)
        if m:
            out.add(m.group(1).rstrip("."))
    _cache[path] = out
    return out


# ── the referrers: which files, and the text of each that is read ──────────


def kind_of(path: Path) -> str | None:
    """Which of the four kinds of referrer a file is, if any."""
    if path.suffix == ".md":
        return "md"
    if path.suffix == ".rs" and not path.is_relative_to(VENDOR):
        return "rs"
    if path.suffix == ".ein":
        return "ein"
    if path == CORPUS_TOML:
        return "toml"
    return None


def tracked_referrers() -> list[Path]:
    """Every file `git ls-files` lists of the four kinds, resolved, once each
    (`CLAUDE.md` is a link to `AGENTS.md`)."""
    out = subprocess.run(
        ["git", "-C", str(REPO), "ls-files", "-z"],
        capture_output=True, text=True, check=True,
    ).stdout
    pages = set()
    for name in out.split("\0"):
        if not name:
            continue
        path = (REPO / name).resolve()
        if path.is_file() and kind_of(path):
            pages.add(path)
    return sorted(pages)


def blank(m: re.Match) -> str:
    """A match replaced by spaces, its newlines kept, so line numbers hold."""
    return re.sub(r"[^\n]", " ", m.group(0))


#: A full-line Rust comment: `//`, `///` or `//!`, and the text after it.
RS_COMMENT = re.compile(r"^\s*//[/!]?(.*)$")


def rust_comment_text(text: str) -> str:
    """The text of every full-line comment, one line for each source line and
    blank where the line is code, with fenced code blocks blanked: a link in a
    doc comment's example is code."""
    lines = []
    for line in text.splitlines():
        m = RS_COMMENT.match(line)
        lines.append(m.group(1) if m else "")
    body = "\n".join(lines)
    return re.sub(r"(?ms)^\s*```.*?^\s*```", blank, body)


def ein_comment_text(text: str) -> str:
    """The text after every `;` that is not inside a string, one line for each
    source line and blank where the line holds no comment."""
    lines, in_string = [], False
    for line in text.splitlines():
        comment, i = "", 0
        while i < len(line):
            c = line[i]
            if in_string:
                if c == "\\":
                    i += 1
                elif c == '"':
                    in_string = False
            elif c == '"':
                in_string = True
            elif c == ";":
                comment = line[i:]
                break
            i += 1
        lines.append(comment)
    return "\n".join(lines)


def tops() -> tuple[set[str], set[str]]:
    """The repository's top-level directories, and the crates' directories,
    which the repo writes short for `ein.rs/crates/<crate>/`."""
    top = {p.name for p in REPO.iterdir() if p.is_dir() and p.name != ".git"}
    crates = {p.name for p in CRATES.iterdir() if p.is_dir()} if CRATES.is_dir() else set()
    return top, crates


def bare_path_pattern() -> re.Pattern:
    top, crates = tops()
    heads = "|".join(re.escape(h) for h in sorted(top | crates, key=len, reverse=True))
    return re.compile(r"(?<![\w./\-#~$@])(?:" + heads + r")/[\w.\-/*<>{}]*")


def bare_paths(text: str) -> list[tuple[int, str, Path]]:
    """Every bare path in ``text``: its line, as written, and where it lands.

    A path names a file with an extension or a directory with a trailing `/`;
    one with a pattern character in it is not read."""
    pat = bare_path_pattern()
    _, crates = tops()
    found = []
    for m in pat.finditer(text):
        written = m.group(0).rstrip(".,:;")
        if any(c in written for c in "*<>{}"):
            continue
        last = written.rstrip("/").rsplit("/", 1)[-1]
        if not written.endswith("/") and "." not in last:
            continue
        head = written.split("/", 1)[0]
        base = CRATES if head in crates and not (REPO / head).is_dir() else REPO
        line = text[: m.start()].count("\n") + 1
        found.append((line, written, (base / written).resolve()))
    return found


def git_ignored(paths: list[Path]) -> set[Path]:
    """The ones git would ignore: outputs of a build or a run, not files of
    the tree, so their absence on a fresh checkout is not a finding."""
    if not paths:
        return set()
    try:
        out = subprocess.run(
            ["git", "-C", str(REPO), "check-ignore", "--stdin", "-z"],
            input="\0".join(str(p) for p in paths), capture_output=True, text=True,
        ).stdout
    except OSError:
        return set()
    return {Path(p) for p in out.split("\0") if p}


#: One reference: the referrer, its line, the target as written, where it
#: resolves (`None` for an anchor in the referrer itself) and the referrer kind.
Ref = tuple[Path, int, str, Path | None, str]


def check_links(
    pages: list[Path], roots: tuple[Path, ...] = (), refs: list[Ref] | None = None
) -> list[tuple[Path, int, str, str]]:
    """Every broken reference of ``pages``, as (referrer, line, target, why).
    ``refs``, when given, collects every reference that resolves."""
    bad: list[tuple[Path, int, str, str]] = []
    missing_bare: list[tuple[Path, int, str, Path]] = []
    for page in pages:
        kind = kind_of(page) or "md"
        raw = page.read_text(errors="replace")
        if kind in ("ein", "toml"):
            text = ein_comment_text(raw) if kind == "ein" else raw
            for line, written, dest in bare_paths(text):
                if dest.exists():
                    if refs is not None:
                        refs.append((page, line, written, dest, kind))
                elif not dest.is_relative_to(REPO):
                    bad.append((page, line, written, "outside the repo"))
                else:
                    missing_bare.append((page, line, written, dest))
            continue
        if kind == "rs":
            text = rust_comment_text(raw)
        else:
            # A `[x](y)` inside a fenced code block is text, not a link — no
            # viewer renders it as one. Blanked line for line, so a finding's
            # line number is still the page's. The review bundle quotes whole
            # programs in fences, and a program's comments may carry links of
            # their own.
            text = re.sub(r"(?ms)^```.*?^```", blank, raw)
        # A page outside the repository — the review bundle, built into a
        # temporary directory by the gate's smoke step — may also link inside
        # the tree it was checked as part of. Its links into the repo are held
        # to the rule below like any other.
        own = next((r for r in roots if page.is_relative_to(r) and not r.is_relative_to(REPO)), None)
        found = [(m, False) for m in LINK.finditer(text)]
        found += [(m, True) for m in REF_DEF.finditer(text)]
        for m, is_def in sorted(found, key=lambda pair: pair[0].start()):
            target = m.group(2)
            if target.startswith(("http://", "https://", "mailto:")):
                continue
            # An anchor in a Rust comment is rustdoc's, into the rendered page.
            if kind == "rs" and target.startswith("#"):
                continue
            # Two shapes that are `[...](...)` and are not links, both found by
            # M1e S1e.3.8's sweep over `plans/`: a **placeholder** in the stage
            # skeleton (`[<idea>](../../ideas/<file>.md)`) and **notation** that
            # happens to be bracketed — `F_r[R,S,T](x)`. A real relative link
            # names a file, so it carries a `.` or a `/`; nothing else does.
            if "<" in target or ">" in target:
                continue
            if not target.startswith("#") and "/" not in target and "." not in target:
                continue
            line = text[: m.start()].count("\n") + 1
            if target.startswith("#"):
                if unquote(target[1:]) not in anchors_of(page):
                    bad.append((page, line, target, "anchor missing in self"))
                continue
            fpart, _, frag = target.partition("#")
            dest = (page.parent / unquote(fpart)).resolve()
            # A target outside the repository is a finding wherever it points
            # and whatever this machine happens to have beside the checkout:
            # `exists()` would otherwise be an answer about the machine rather
            # than about the tree, so the same commit passes here and fails in
            # CI. Three `../../acva/…` links did exactly that on 2026-09-01 —
            # they resolve on the author's workstation, where the sibling repo
            # sits next to this one, and nowhere else. The fix at those sites
            # is the repo's standing one: a reference that cannot be a link is
            # still fine as `code`.
            if not dest.is_relative_to(REPO) and not (own and dest.is_relative_to(own)):
                bad.append((page, line, target, "outside the repo"))
                continue
            if not dest.exists():
                bad.append((page, line, target, "file missing"))
                continue
            if refs is not None:
                refs.append((page, line, target, dest, kind))
            if frag and dest.suffix == ".md":
                if unquote(frag) not in anchors_of(dest):
                    bad.append((page, line, target, "anchor missing"))
                continue
            # No fragment: the reference is a prose "§x.y", in the link's own
            # label if it has one and otherwise in the text right after it.
            # Both are cut at the first `|` so a table row's next cell — and
            # the next row's link — cannot claim it.
            if dest.suffix == ".md" and dest != page and not is_def:
                have = section_numbers_of(dest)
                if not have:
                    continue
                label = m.group(1)
                scope = label if "§" in label else text[m.end() : m.end() + 100]
                # Cut at the first `|` so a table row's next cell cannot claim
                # the reference — and at the first `[`, because the next
                # *link's* own label can (`[design/07](…); [`x.md` § 2.4](…)`
                # gave design/07 a §2.4 it never had: M1e S1e.3.8).
                scope = scope.split("|")[0].split("\n\n")[0].split("[")[0]
                for sec in PROSE_SECTION.findall(scope):
                    sec = sec.rstrip(".")
                    if sec not in have:
                        bad.append(
                            (page, line, f"{fpart} §{sec}", "prose § not a heading")
                        )
    ignored = git_ignored([d for _, _, _, d in missing_bare])
    for page, line, written, dest in missing_bare:
        if dest not in ignored:
            bad.append((page, line, written, "file missing"))
    bad.sort(key=lambda b: (str(b[0]), b[1]))
    return bad


# ── decision ids ────────────────────────────────────────────────────────────

DECISIONS = REPO / "docs" / "decisions"
#: What is not the project, and so not read for decision ids: the process
#: (the plans, the scripts, the gate and CI, the agents' guide), what is
#: external to both, and third-party source.
NOT_PROJECT = (
    "plans/", "utils/", ".github/", "run_tests.sh", "AGENTS.md", "CLAUDE.md",
    "c/", "nl2smt/", "docs/lib/", "ein.rs/vendor/",
)
#: A decision file's name: its id, then the slug of its title.
DECISION_FILE = re.compile(r"^(Q-M\d+(?:[a-z]\d*)?\.\d+|Q\d+)-[a-z0-9][a-z0-9-]*\.md$")
#: A milestone-scoped id, and the `.n` that continue it written short, in a
#: list or a range: `Q-M1f.135, .141 and .142`, `Q-M1f.315 to .340`.
QM_ID = re.compile(
    r"(?<![\w-])Q-M(\d+(?:[a-z]\d*)?)\.(\d+)(?!\d)"
    r"((?:\s*(?:,|and|or|to|–|-|/)\s*\.\d+(?!\d))*)"
)
#: A bare `Q<n>`, the global ledger's numbering, not inside a longer name:
#: `Q1.5b.7`, `S1f.5.4#Q1`, `Q23@M2` and `Q-M1f.6` hold none.
BARE_Q = re.compile(r"(?<![\w#@./\\$-])Q(\d+)(?![\w@#]|\.\d)")
#: The word before a bare `Q<n>` that names the ledger which numbered it.
LEDGER_BEFORE = re.compile(
    r"(?:\b(?:review's|milestone's|F\d+|M\d+(?:[a-z]\d*)?"
    r"|[SPT]\d+(?:[a-z]\d*)?(?:\.\d+[a-z]?)+(?:'s)?(?:\s+census)?)"
    r"|[\w.-]+\.md)\s*(?:/\s*)?[`*\[(]*$"
)
#: …or the words after it: `Q1 of malformed_numbers.md`, `"Q3 - as recommended"`.
LEDGER_AFTER = re.compile(r"^`?\s+of\s+`?[\w./-]+\.md|^\s+-\s")
#: A range's first end, right before its second: `Q1–Q10`, `` `Q1`–`Q10` ``.
RANGE_BEFORE = re.compile(r"Q\d+`?\s*[–-]\s*`?$")

#: Decision ids let stand where they are, (file, id) -> why. Each is an id in
#: text the engine produces and a golden digests, so that re-aiming it moves the
#: golden: whoever re-aims the engine's own ids removes the row with the
#: golden's re-bank (none is left today). A row whose id its file no longer
#: cites is a finding, so the list cannot outlive what it excuses.
DECISION_IDS_ALLOWED: dict[tuple[str, str], str] = {}

#: Rust: a char literal (so that `'"'` opens no string), and a raw string's
#: opening, `r"`, `r#"`, `br#"`.
RS_CHAR = re.compile(r"'(?:[^'\\\n]|\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]{1,6}\}|.))'")
RS_RAW = re.compile(r'(?:b|c)?r(#*)"')


def rust_comments_and_strings(text: str) -> str:
    """Every comment and every string literal of a Rust source, all else
    blanked to spaces, its newlines kept, so a line number is the file's.
    Comments at a line's start or after code, block comments nested; strings
    plain, byte and raw."""
    out: list[str] = []
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
        elif text.startswith("/*", i):
            depth, j = 1, i + 2
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
            out.append(re.sub(r"[^\n]", " ", text[i:j]))
            i = j
            continue
        out.append(text[i:j])
        i = j
    return "".join(out)


def project_referrers() -> list[Path]:
    """Every tracked file of the project that may cite a decision: a `.md`, a
    crate's `.rs`, a `.ein` and `corpus/*.toml`."""
    out = subprocess.run(
        ["git", "-C", str(REPO), "ls-files", "-z"], capture_output=True, text=True, check=True
    ).stdout
    files = set()
    for rel in out.split("\0"):
        if not rel or rel.startswith(NOT_PROJECT):
            continue
        if (
            rel.endswith((".md", ".ein"))
            or (rel.endswith(".rs") and rel.startswith("ein.rs/crates/"))
            or (rel.startswith("corpus/") and rel.endswith(".toml") and rel.count("/") == 1)
        ):
            path = REPO / rel
            if path.is_file() and not path.is_symlink():
                files.add(path)
    return sorted(files)


def decision_files() -> tuple[dict[str, list[Path]], list[Path]]:
    """Every decision file by its id, and the files under `docs/decisions/`
    that are neither a `README.md` nor named `<id>-<slug>.md`."""
    by_id: dict[str, list[Path]] = {}
    misnamed = []
    for path in sorted(DECISIONS.rglob("*.md")) if DECISIONS.is_dir() else []:
        if path.name == "README.md":
            continue
        m = DECISION_FILE.match(path.name)
        if m:
            by_id.setdefault(m.group(1), []).append(path)
        else:
            misnamed.append(path)
    return by_id, misnamed


def bare_q_is_decision(line: str, m: re.Match) -> bool:
    """Whether a bare `Q<n>` is written as a decision id, or names the ledger
    that numbered it (see the module's docstring)."""
    before = line[: m.start()]
    if RANGE_BEFORE.search(before):
        first = list(re.finditer(r"Q\d+", before))[-1]
        return bare_q_is_decision(line, first)
    return not (LEDGER_BEFORE.search(before) or LEDGER_AFTER.match(line[m.end():]))


def decision_ids_of(text: str) -> list[tuple[int, str]]:
    """Every decision id in ``text``, as (line, id), in order."""
    found = []
    for ln, line in enumerate(text.split("\n"), start=1):
        for m in QM_ID.finditer(line):
            ms = m.group(1)
            found.append((m.start(), ln, f"Q-M{ms}.{m.group(2)}"))
            for c in re.finditer(r"\.(\d+)", m.group(3)):
                found.append((m.start(3) + c.start(), ln, f"Q-M{ms}.{c.group(1)}"))
        for m in BARE_Q.finditer(line):
            if bare_q_is_decision(line, m):
                found.append((m.start(), ln, f"Q{m.group(1)}"))
    found.sort(key=lambda f: (f[1], f[0]))
    return [(ln, qid) for _, ln, qid in found]


def check_decisions(
    pages: list[Path],
) -> tuple[list[tuple[Path, int, str, str]], int, int]:
    """Every decision id ``pages`` cite that does not resolve to exactly one
    decision file, as (referrer, line, id, why), with the findings of
    `docs/decisions/` itself and of the allow-list; how many citations were
    read; and how many the allow-list let stand."""
    by_id, misnamed = decision_files()
    allowed_seen: set[tuple[str, str]] = set()
    allowed = 0
    bad: list[tuple[Path, int, str, str]] = []
    for path in misnamed:
        bad.append((path, 0, path.name, "not <id>-<slug>.md"))
    for qid, paths in sorted(by_id.items()):
        if len(paths) > 1:
            for path in paths:
                bad.append((path, 0, qid, f"{len(paths)} decision files"))
    read = 0
    for page in pages:
        raw = page.read_text(errors="replace")
        if page.suffix == ".rs":
            text = rust_comments_and_strings(raw)
        elif page.suffix == ".ein":
            text = ein_comment_text(raw)
        else:
            text = raw
        rel = page.relative_to(REPO).as_posix() if page.is_relative_to(REPO) else str(page)
        for line, qid in decision_ids_of(text):
            read += 1
            n = len(by_id.get(qid, []))
            if n != 1 and (rel, qid) in DECISION_IDS_ALLOWED:
                allowed_seen.add((rel, qid))
                allowed += 1
                continue
            if n == 0:
                bad.append((page, line, qid, "no decision file"))
            elif n > 1:
                bad.append((page, line, qid, f"{n} decision files"))
    read_rels = {p.relative_to(REPO).as_posix() for p in pages if p.is_relative_to(REPO)}
    for rel, qid in sorted(DECISION_IDS_ALLOWED):
        if rel in read_rels and (rel, qid) not in allowed_seen:
            bad.append((REPO / rel, 0, qid, "allowed, not cited"))
    bad.sort(key=lambda b: (str(b[0]), b[1]))
    return bad, read, allowed


def inbound(refs: list[Ref], prefixes: list[str]) -> list[tuple[str, Counter]]:
    """Per target, the references to it by referrer kind: `md` (with `plans`
    and `history` counting the `.md` ones from `plans/` and `docs/history/`
    again), `rs`, `ein` and `toml`. A reference of a page to itself is not
    counted."""
    per: dict[str, Counter] = {}
    for page, _, _, dest, kind in refs:
        if dest is None or dest == page or not dest.is_relative_to(REPO):
            continue
        key = str(dest.relative_to(REPO)) + ("/" if dest.is_dir() else "")
        if prefixes and not any(key.startswith(p) for p in prefixes):
            continue
        c = per.setdefault(key, Counter())
        c[kind] += 1
        if kind == "md":
            src = page.relative_to(REPO) if page.is_relative_to(REPO) else page
            if str(src).startswith("plans/"):
                c["plans"] += 1
            elif str(src).startswith("docs/history/"):
                c["history"] += 1
    return sorted(per.items(), key=lambda kv: (-sum(kv[1][k] for k in ("md", "rs", "ein", "toml")), kv[0]))


# ── main ────────────────────────────────────────────────────────────────────


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument(
        "tree",
        nargs="*",
        help="directories to walk for .md pages, and single files of the four "
        "kinds; default docs/ein, or nothing beside --tracked",
    )
    ap.add_argument("--links", action="store_true")
    ap.add_argument("--identifiers", action="store_true")
    ap.add_argument("--states", action="store_true")
    ap.add_argument(
        "--tracked",
        action="store_true",
        help="read every tracked .md, .rs (but ein.rs/vendor/), .ein and corpus/corpus.toml",
    )
    ap.add_argument(
        "--inbound",
        action="store_true",
        help="print, per target file, the references that resolve to it, by referrer kind",
    )
    ap.add_argument(
        "--target",
        action="append",
        default=[],
        metavar="PREFIX",
        help="with --inbound: only the targets under this repo-relative prefix (repeatable)",
    )
    ap.add_argument("-k", "--only", help="substring of a page path")
    ap.add_argument("--check", action="store_true", help="exit 1 on a broken link")
    args = ap.parse_args()
    if not (args.links or args.identifiers or args.states or args.inbound):
        args.links = args.identifiers = args.states = True
    if not args.tree and not args.tracked:
        args.tree = [str(DEFAULT_TREE)]

    # `resolve()`, because `rel()` below subtracts REPO and a relative argv
    # would not be under it.
    pages: list[Path] = tracked_referrers() if args.tracked else []
    roots = tuple(Path(a).resolve() for a in args.tree)
    for arg in args.tree:
        root = Path(arg).resolve()
        pages.extend([root] if root.is_file() else root.rglob("*.md"))
    pages = sorted(set(pages))
    if args.only:
        pages = [p for p in pages if args.only in str(p)]
    # The decision half reads the project's files, which `-k` may narrow to
    # some the link half does not read (`corpus/regressions.toml`).
    cited: list[Path] = []
    if args.links and args.tracked:
        cited = project_referrers()
        if args.only:
            cited = [p for p in cited if args.only in str(p)]
    if not pages and not cited:
        print(f"no pages under {' '.join(args.tree)}", file=sys.stderr)
        return 2
    rel = lambda p: p.relative_to(REPO) if p.is_relative_to(REPO) else p  # noqa: E731

    failures = 0

    if args.links or args.inbound:
        refs: list[Ref] = []
        bad = check_links(pages, roots, refs)
    if args.links:
        kinds = Counter(kind_of(p) or "md" for p in pages)
        read = ", ".join(f"{kinds[k]} {k}" for k in ("md", "rs", "ein", "toml") if kinds[k])
        print(f"── links ── {len(pages)} files ({read}), {len(bad)} finding(s)")
        cur = None
        for page, line, target, why in bad:
            if page != cur:
                print(f"   {rel(page)}")
                cur = page
            print(f"     :{line:<5} {why:<22} {target}")
        failures += len(bad)

    if args.links and args.tracked:
        dbad, read, allowed = check_decisions(cited)
        print(
            f"── decision ids ── {read} cited in {len(cited)} project files, "
            f"{allowed} let stand by DECISION_IDS_ALLOWED, {len(dbad)} finding(s)"
        )
        cur = None
        for page, line, qid, why in dbad:
            if page != cur:
                print(f"   {rel(page)}")
                cur = page
            print(f"     :{line:<5} {why:<22} {qid}")
        failures += len(dbad)

    if args.inbound:
        rows = inbound(refs, args.target)
        print("── inbound ── references that resolve, per target, by referrer kind")
        print("   (md counts every .md referrer; plans and history count those of")
        print("   them under plans/ and docs/history/ again; a page's links to itself")
        print("   are not counted)")
        print(f"   {'all':>5} {'md':>5} {'plans':>5} {'hist':>5} {'rs':>5} {'ein':>5} {'toml':>5}  target")
        for key, c in rows:
            total = sum(c[k] for k in ("md", "rs", "ein", "toml"))
            print(
                f"   {total:>5} {c['md']:>5} {c['plans']:>5} {c['history']:>5} "
                f"{c['rs']:>5} {c['ein']:>5} {c['toml']:>5}  {key}"
            )
        print(f"   {len(rows)} target(s)")

    pages = [p for p in pages if p.suffix == ".md"]

    if args.identifiers:
        hay, files = haystack()
        print(f"\n── identifiers ── against {CRATES.relative_to(REPO)} (report only)")
        total = 0
        for page in pages:
            text = page.read_text(errors="replace")
            seen: dict[str, tuple[str, int]] = {}
            for m in TICK.finditer(text):
                tok = m.group(1)
                kind = classify(tok)
                if kind and tok not in seen:
                    seen[tok] = (kind, text[: m.start()].count("\n") + 1)
            missing = [
                (t, k, ln)
                for t, (k, ln) in sorted(seen.items())
                if not resolves(t, k, hay, files)
            ]
            if missing:
                print(f"   {rel(page)}  ({len(missing)})")
                for tok, kind, line in missing:
                    print(f"     :{line:<5} [{kind}] {tok}")
                total += len(missing)
        print(f"   {total} unresolved token(s) — skim, do not count")

    if args.states:
        print("\n── states ──")
        marked = []
        for p in pages:
            hits = STATE_BANNER.findall(p.read_text(errors="replace"))
            if hits:
                marked.append((p, len(hits)))
        print(f"   {len(pages) - len(marked)} page(s) carry no banner at all")
        print(f"   {len(marked)} carry one, and this check cannot tell a")
        print("   whole-page banner from one scoping a single section:")
        for p, n in marked:
            print(f"     {n}× {rel(p)}")
        print("   a page of docs/ein/ keeps none: it is current or it is deleted")

    return 1 if (args.check and failures) else 0


if __name__ == "__main__":
    sys.exit(main())
