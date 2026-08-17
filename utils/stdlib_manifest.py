#!/usr/bin/env python3
"""`stdlib/MANIFEST.sha256` — generate or verify (M1a S1a.0.3).

The stdlib is the single source of truth both implementations read, and it is
*not* test data: 1 231 lines of ein-lang across seven modules, three of which
`zebra2.ein` imports. A forked copy would make every parity result meaningless
— a T2 diff would report "the engines disagree" when in fact the *programs*
differ.

The manifest is what makes a fork detectable. It is checked in beside the
modules and serves three readers:

- **the checkout** — `imports._stdlib_root()` confirms a directory *is* the
  stdlib by finding this file, which is how a source tree wins over an
  installed copy without a hardcoded path;
- **the wheel** — a build-time copy lands in `ein/stdlib/`, and CI compares it
  against the manifest so a stale copy cannot ship;
- **ein.rs** — the `include_dir!`-embedded tree is digested at build time and
  compared against the manifest in a test.

Usage::

    python3 utils/stdlib_manifest.py            # verify (exit 1 on drift)
    python3 utils/stdlib_manifest.py --write    # regenerate

Format: one `<sha256>  <name>` line per module, sorted by name — `sha256sum`'s
own format, so `sha256sum -c MANIFEST.sha256` works from inside `stdlib/`.
"""
from __future__ import annotations

import argparse
import hashlib
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANIFEST_NAME = "MANIFEST.sha256"


def stdlib_dir() -> Path:
    """The stdlib root: repo-root `stdlib/`, or the pre-S1a.0.3 location."""
    for candidate in (REPO / "stdlib", REPO / "ein.py/src/ein/stdlib"):
        if candidate.is_dir():
            return candidate
    raise SystemExit("no stdlib directory found")


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build(root: Path) -> str:
    """The manifest text for ``root``. Covers `*.ein` only — the README moves
    with the modules but is prose, and hashing it would make a typo fix look
    like a semantic change."""
    return "".join(
        f"{digest(p)}  {p.name}\n" for p in sorted(root.glob("*.ein"))
    )


def parse(text: str) -> dict[str, str]:
    out: dict[str, str] = {}
    for line in text.splitlines():
        if not line.strip():
            continue
        sha, _, name = line.partition("  ")
        out[name.strip()] = sha.strip()
    return out


def verify(root: Path) -> list[str]:
    """Differences between the manifest and the files, as report lines."""
    manifest = root / MANIFEST_NAME
    if not manifest.is_file():
        return [f"missing {manifest}"]
    recorded = parse(manifest.read_text(encoding="utf-8"))
    actual = parse(build(root))
    problems = []
    for name in sorted(set(recorded) | set(actual)):
        if name not in actual:
            problems.append(f"{name}: in the manifest, not on disk")
        elif name not in recorded:
            problems.append(f"{name}: on disk, not in the manifest")
        elif recorded[name] != actual[name]:
            problems.append(f"{name}: content changed "
                            f"({recorded[name][:12]}… → {actual[name][:12]}…)")
    return problems


def compare(root: Path, other: Path) -> list[str]:
    """Differences between two stdlib trees — the drift check that keeps a
    packaged or embedded copy honest against the source of truth."""
    a, b = parse(build(root)), parse(build(other))
    problems = []
    for name in sorted(set(a) | set(b)):
        if a.get(name) != b.get(name):
            problems.append(f"{name}: {root.name} and {other.name} differ")
    return problems


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--write", action="store_true",
                    help="regenerate the manifest instead of verifying it")
    ap.add_argument("--against", type=Path, default=None, metavar="DIR",
                    help="compare the stdlib against another copy of it "
                         "(a built wheel's `ein/stdlib/`, say)")
    args = ap.parse_args()

    root = stdlib_dir()
    if args.write:
        (root / MANIFEST_NAME).write_text(build(root), encoding="utf-8")
        n = len(list(root.glob("*.ein")))
        print(f"wrote {root / MANIFEST_NAME} ({n} modules)")
        return 0

    problems = verify(root)
    if args.against is not None:
        problems += compare(root, args.against)
    if problems:
        print(f"stdlib drift ({root}):", file=sys.stderr)
        for p in problems:
            print(f"  {p}", file=sys.stderr)
        print("\nif intentional: python3 utils/stdlib_manifest.py --write",
              file=sys.stderr)
        return 1
    print(f"stdlib ok — {len(parse(build(root)))} modules match {MANIFEST_NAME}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
