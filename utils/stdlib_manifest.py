#!/usr/bin/env python3
"""`stdlib/MANIFEST.sha256` — generate or verify (M1a S1a.0.3, narrowed
at [S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md)).

The stdlib is 1 231 lines of ein-lang across seven modules, three of which
`zebra2.ein` imports. It is *not* test data: it is program text the engine
reads at run time, and it is checked in **once**, at repo-root `stdlib/`.

The manifest had three readers, and has two.

- **the checkout** — a directory is the stdlib because it carries this file.
  `ein_ir::stdlib::MARKER` is the constant, and the walk-up in
  `ein_ir::stdlib::resolve` is what makes a source tree beat an installed
  copy without a hardcoded path. Only the *presence* of the file matters here.
- **the binary** — the `include_dir!`-embedded tree is digested and compared
  against this manifest by `ein-ir`'s `the_embedded_copy_matches_the_manifest`
  / `the_embedded_copy_has_no_extra_modules`. That pair is the real check, and
  it is not stale-able: `include_dir!` registers each file as a build
  dependency, so editing a module rebuilds the crate and the test compares the
  *new* bytes. (Verified by breaking it: append a comment to `algebra.ein` and
  `cargo test -p ein-ir --lib stdlib` goes red without any other change.)
- ~~**the wheel**~~ — a build-time copy landed in `ein/src/ein/stdlib/`, and
  CI compared it against the manifest so a stale copy could not ship. There is
  no Python package and therefore no second copy; `--against DIR` went with
  it, along with the fallback that let `stdlib_dir()` answer with
  `ein.py/src/ein/stdlib`.

So what is left here is the half `cargo test` cannot do: **writing** the
manifest. Nothing in the workspace regenerates it — a test that rewrote the
file it checks would check nothing — so a stdlib edit is two steps, and the
Rust test is what fails if you take only the first.

Usage::

    python3 utils/stdlib_manifest.py            # verify (exit 1 on drift)
    python3 utils/stdlib_manifest.py --write    # regenerate

Verify is kept as well, for two reasons: it names the drift *per module*
where the Rust assertion names the first one it reaches, and it answers in
milliseconds without a toolchain, which is why it is the per-commit tier's
first step rather than something you wait for a build to tell you.

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
    """The stdlib root. One location, named rather than searched for.

    It was a search — repo-root `stdlib/`, else `ein.py/src/ein/stdlib` — and
    a fallback like that turns "the stdlib is not where it should be" into
    "checked a different directory, all fine", which is the same defect
    S1a.10.3 took out of the corpus completeness check.
    """
    root = REPO / "stdlib"
    if not root.is_dir():
        raise SystemExit(f"no stdlib directory at {root}")
    return root


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


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--write", action="store_true",
                    help="regenerate the manifest instead of verifying it")
    args = ap.parse_args()

    root = stdlib_dir()
    if args.write:
        (root / MANIFEST_NAME).write_text(build(root), encoding="utf-8")
        n = len(list(root.glob("*.ein")))
        print(f"wrote {root / MANIFEST_NAME} ({n} modules)")
        return 0

    problems = verify(root)
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
