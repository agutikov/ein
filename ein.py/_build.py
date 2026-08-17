"""Build hook: copy the shared stdlib into the package (M1a S1a.0.3).

The stdlib lives at repo-root `stdlib/` — one source of truth both
implementations read, because a forked copy would make every parity result
meaningless (`plans/m1a_rust/design/11_shared_assets.md`). But a wheel cannot
reach outside the package, so `ein/stdlib/` still has to exist in a
distribution.

This hook resolves that: `build_py` copies `../stdlib/*.ein` into
`src/ein/stdlib/` on the way to a wheel or sdist, and the copy is
`.gitignore`d. There is exactly one checked-in stdlib; the packaged one is a
build product, like any other.

`imports._stdlib_root()`'s three-step chain is the other half — `$EIN_STDLIB`,
then a checkout found by walking up for `MANIFEST.sha256`, then this copy — so
an editable install still reads the checkout and a wheel still reads itself.

Wired in `pyproject.toml`:

    [tool.setuptools.cmdclass]
    build_py = "_build.build_py"
"""
from __future__ import annotations

import shutil
from pathlib import Path

from setuptools.command.build_py import build_py as _build_py

HERE = Path(__file__).resolve().parent
SHARED = HERE.parent / "stdlib"
PACKAGED = HERE / "src" / "ein" / "stdlib"


class build_py(_build_py):  # noqa: N801 — setuptools dispatches on the name
    """`build_py`, plus the stdlib copy."""

    def run(self) -> None:
        copy_stdlib()
        super().run()


def copy_stdlib() -> None:
    """Refresh `src/ein/stdlib/` from repo-root `stdlib/`.

    A no-op when the shared tree is absent — building from an sdist, where the
    packaged copy is already the only one there is and there is nothing above
    it to copy from.
    """
    if not SHARED.is_dir():
        return
    PACKAGED.mkdir(parents=True, exist_ok=True)
    wanted = set()
    for src in (*SHARED.glob("*.ein"), SHARED / "README.md",
                SHARED / "MANIFEST.sha256"):
        if src.is_file():
            shutil.copy2(src, PACKAGED / src.name)
            wanted.add(src.name)
    # Drop anything the shared tree no longer has: a stale module left behind
    # would ship, and would be exactly the forked copy this arrangement exists
    # to prevent.
    for stale in PACKAGED.iterdir():
        if stale.is_file() and stale.name not in wanted:
            stale.unlink()


__all__ = ["build_py", "copy_stdlib"]
