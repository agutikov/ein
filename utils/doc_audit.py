#!/usr/bin/env python3
"""Does a documentation page still describe the engine that ships? (M1e S1e.2.2)

`docs/kernel/` is the only statement of intent in this repo that is not also
the implementation, so a claim there is checked by `cargo test --workspace` and
by nothing else — which means a page nothing runs is a page nothing checks.
[S1a.10.6] was a doc pass and it missed **every** page of `CD-H1`, for a reason
worth naming: a pass driven by *what the milestone changed* cannot catch a page
describing machinery removed two milestones ago, because nothing in the
milestone touched it.

This is the milestone-independent half. It asks three mechanical questions of
every page, none of which needs to know what changed:

    python3 utils/doc_audit.py                  # all three, as a report
    python3 utils/doc_audit.py --links --check  # exit 1 on a broken link
    python3 utils/doc_audit.py --identifiers -k inference/lattice_dump.md

**links** — every relative markdown link resolves, file *and* `#anchor`, with
GitHub's slugification. Plus the check this script exists because nothing had:
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

**states** — every page declares which of the three states it is in
(`docs/kernel/README.md` § Which pages to trust). *current* is the default and
carries no banner, so what this reports is the pages that **do** claim to be
superseded, for comparison against that README's table.

**Not a gate.** `--check` exits 1 on the unambiguous half (links and prose
§-references) so a pre-commit hook *could* run it; whether any of this belongs
in CI is `DO-M2`'s question, and a markdown-link checker is its obvious
candidate. What lives here is the instrument.

What this cannot check, and what the checklist in `docs/kernel/README.md`
§ Keeping this true asks a reader to do by hand: run the commands a page
shows. Four of S1e.2.2's findings were *invocations* — a CLI line producing
neither artifact it names, `(instnce ?a ?T)` claimed to be a parse error — and
the only instrument for those is a shell.

[S1a.10.6]: docs/history/m1a_rust/README.md#s1a106--the-docs-after-the-oracle
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from urllib.parse import unquote

REPO = Path(__file__).resolve().parents[1]
CRATES = REPO / "ein.rs" / "crates"
DEFAULT_TREE = REPO / "docs" / "kernel"

TICK = re.compile(r"`([^`\n]+)`")
LINK = re.compile(r"\[([^\]]*)\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
#: ``[`page.md`](page.md) §3d.vii`` and ``… § 3a + § 3d.iv`` — a section
#: reference in prose, immediately after a link to the page it is about.
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


def check_links(pages: list[Path]) -> list[tuple[Path, int, str, str]]:
    bad = []
    for page in pages:
        text = page.read_text(errors="replace")
        for m in LINK.finditer(text):
            target = m.group(2)
            if target.startswith(("http://", "https://", "mailto:")):
                continue
            line = text[: m.start()].count("\n") + 1
            if target.startswith("#"):
                if unquote(target[1:]) not in anchors_of(page):
                    bad.append((page, line, target, "anchor missing in self"))
                continue
            fpart, _, frag = target.partition("#")
            dest = (page.parent / unquote(fpart)).resolve()
            if not dest.exists():
                bad.append((page, line, target, "file missing"))
                continue
            if frag and dest.suffix == ".md":
                if unquote(frag) not in anchors_of(dest):
                    bad.append((page, line, target, "anchor missing"))
                continue
            # No fragment: the reference is a prose "§x.y", in the link's own
            # label if it has one and otherwise in the text right after it.
            # Both are cut at the first `|` so a table row's next cell — and
            # the next row's link — cannot claim it.
            if dest.suffix == ".md" and dest != page:
                have = section_numbers_of(dest)
                if not have:
                    continue
                label = m.group(1)
                scope = label if "§" in label else text[m.end() : m.end() + 100]
                scope = scope.split("|")[0].split("\n\n")[0]
                for sec in PROSE_SECTION.findall(scope):
                    sec = sec.rstrip(".")
                    if sec not in have:
                        bad.append(
                            (page, line, f"{fpart} §{sec}", "prose § not a heading")
                        )
    return bad


# ── main ────────────────────────────────────────────────────────────────────


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("tree", nargs="?", default=str(DEFAULT_TREE))
    ap.add_argument("--links", action="store_true")
    ap.add_argument("--identifiers", action="store_true")
    ap.add_argument("--states", action="store_true")
    ap.add_argument("-k", "--only", help="substring of a page path")
    ap.add_argument("--check", action="store_true", help="exit 1 on a broken link")
    args = ap.parse_args()
    if not (args.links or args.identifiers or args.states):
        args.links = args.identifiers = args.states = True

    pages = sorted(Path(args.tree).rglob("*.md"))
    if args.only:
        pages = [p for p in pages if args.only in str(p)]
    if not pages:
        print(f"no pages under {args.tree}", file=sys.stderr)
        return 2
    rel = lambda p: p.relative_to(REPO)  # noqa: E731

    failures = 0

    if args.links:
        bad = check_links(pages)
        print(f"── links ── {len(pages)} pages, {len(bad)} finding(s)")
        cur = None
        for page, line, target, why in bad:
            if page != cur:
                print(f"   {rel(page)}")
                cur = page
            print(f"     :{line:<5} {why:<22} {target}")
        failures += len(bad)

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
        print("   compare against docs/kernel/README.md § Which pages to trust,")
        print("   which names the whole-page ones and there are three")

    return 1 if (args.check and failures) else 0


if __name__ == "__main__":
    sys.exit(main())
