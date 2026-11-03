#!/usr/bin/env python3
"""check_test_scratch.py — the tests left nothing in their `TMPDIR`.

    python3 utils/check_test_scratch.py DIR

M1e2 S1e2.3.5's check, run by `./run_tests.sh` and by the per-commit
workflow. Both run `cargo test` with `TMPDIR` set to DIR, a fresh empty
directory under `build/`, and run this after the tests pass. Every entry
still in DIR is scratch a test wrote and did not remove. Each one is named,
with the source files under `ein.rs/crates/` and `utils/` that spell its
prefix; then DIR is removed and the exit is 1. An empty DIR is removed, and
the exit is 0.

One directory, not an assertion per test: a test cannot see what another
binary leaves, and a forgotten directory is by definition one that no test
knew it was writing. Before this check, 1 098 `/tmp/ein-*` directories had
piled up, eleven more per gate run (S1e2.3.5's sweep, 2026-11-03).
"""

from __future__ import annotations

import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def writers(name: str) -> list[str]:
    """The source files that spell `name`'s prefix as a string literal.

    `ein-kb-semantics-reexport-1234-0` is
    `Scratch::new(&format!("ein-kb-semantics-{name}"))` in process 1234. The
    prefix is what comes before the first all-digit component, the pid. A
    test's tag is not in its source, so the prefix is shortened one component
    at a time until some source spells it: as a format string that adds a tag
    or the pid, `"prefix-{`, else as `tempfile`'s `"prefix-"`, else as a
    whole literal, `"prefix"`.
    """
    parts = re.sub(r"-\d+(?:[-.].*)?$", "", name).split("-")
    texts = {p: p.read_text(encoding="utf-8", errors="replace")
             for pattern in ("ein.rs/crates/**/*.rs", "utils/**/*.py")
             for p in sorted(ROOT.glob(pattern))
             if p.resolve() != Path(__file__).resolve()}
    for k in range(len(parts), 1, -1):
        prefix = "-".join(parts[:k])
        for needle in (f'"{prefix}-{{', f'"{prefix}-"', f'"{prefix}"'):
            found = [str(p.relative_to(ROOT)) for p, text in texts.items() if needle in text]
            if found:
                return found
    return []


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: python3 utils/check_test_scratch.py DIR", file=sys.stderr)
        return 2
    tmp = Path(sys.argv[1])
    if not tmp.is_dir():
        print(f"error: {tmp} is not a directory: the tests did not run under it",
              file=sys.stderr)
        return 2
    left = sorted(tmp.iterdir())
    for entry in left:
        files = sum(1 for p in entry.rglob("*") if p.is_file()) if entry.is_dir() else 1
        who = writers(entry.name)
        print(f"  {entry.name}  ({files} file{'' if files == 1 else 's'}): "
              + (", ".join(who) if who else "no source under ein.rs/crates or utils spells it"),
              file=sys.stderr)
    shutil.rmtree(tmp, ignore_errors=True)
    if left:
        print(f"error: the tests left {len(left)} entr{'y' if len(left) == 1 else 'ies'} "
              f"in their TMPDIR, listed above with the file that writes each.\n"
              f"       Make scratch with ein_corpus::Scratch, which removes itself "
              f"on drop.", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
