"""In-tree build backend: copy the shared stdlib into the package (M1a S1a.0.3).

The stdlib lives at repo-root `stdlib/` — one source of truth both
implementations read, because a forked copy would make every parity result
meaningless (`plans/m1a_rust/design/11_shared_assets.md`). But a wheel cannot
reach outside the package, so `ein/stdlib/` still has to exist in a
distribution.

This module resolves that: it is a **PEP 517 in-tree backend** that delegates
every hook to `setuptools.build_meta` and copies `../stdlib/*.ein` into
`src/ein/stdlib/` on the way to a wheel, an sdist or an editable install. The
copy is `.gitignore`d. There is exactly one checked-in stdlib; the packaged one
is a build product, like any other.

`imports._stdlib_root()`'s three-step chain is the other half — `$EIN_STDLIB`,
then a checkout found by walking up for `MANIFEST.sha256`, then this copy — so
an editable install still reads the checkout and a wheel still reads itself.

Wired in `pyproject.toml`:

    [build-system]
    build-backend = "_build"
    backend-path  = ["."]

**Why a backend and not `[tool.setuptools.cmdclass]`.** The hook was a
`build_py` subclass named by a `cmdclass` string until 2026-08-18, and that
never resolved: setuptools looks a `cmdclass` module up under `package-dir`,
which `[tool.setuptools.packages.find] where = ["src"]` fills with
`{"": "src"}` *before* the cmdclass field is expanded — so `_build` was
searched for at `src/_build.py` and `pip install -e ein.py` died with
`ModuleNotFoundError: _build` while still gathering build requirements.
`backend-path` is the mechanism that exists for this: it puts this directory on
the build's import path explicitly, so nothing depends on where the package
tree happens to be rooted.
"""
from __future__ import annotations

import shutil
from pathlib import Path

# Every hook setuptools exposes, re-exported so this module is a complete
# backend; the three build_* ones are then overridden below. The star import is
# deliberate — a hook added to a future setuptools keeps working, where a
# hand-written list would silently not forward it.
from setuptools import build_meta as _setuptools
from setuptools.build_meta import *  # noqa: F403

HERE = Path(__file__).resolve().parent
SHARED = HERE.parent / "stdlib"
PACKAGED = HERE / "src" / "ein" / "stdlib"


def build_wheel(wheel_directory, config_settings=None, metadata_directory=None):
    copy_stdlib()
    return _setuptools.build_wheel(wheel_directory, config_settings, metadata_directory)


def build_sdist(sdist_directory, config_settings=None):
    copy_stdlib()
    return _setuptools.build_sdist(sdist_directory, config_settings)


def build_editable(wheel_directory, config_settings=None, metadata_directory=None):
    copy_stdlib()
    return _setuptools.build_editable(
        wheel_directory, config_settings, metadata_directory
    )


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


__all__ = ["build_editable", "build_sdist", "build_wheel", "copy_stdlib"]
