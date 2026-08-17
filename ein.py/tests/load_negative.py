"""Shared access to the load-negative corpus — `examples/broken/load/**`.

Not a test module: the helpers the load-negative tests share. The corpus itself
and its contract are documented in
[`examples/broken/load/README.md`](../../examples/broken/load/README.md); the
suite that pins every entry is
[`tests/kb/test_load_negative.py`](kb/test_load_negative.py).

A test that wants one specific case (because it also documents *why* that case
is rejected) calls :func:`load_error`, which loads the fixture, requires a
`KBLoadError`, checks the message against the checked-in `.expected`, and hands
back the message for whatever further assertion the test wants to make. So
re-pointing a test at a fixture cannot weaken it — the anchor is enforced at
every call site, not only in the parametrized sweep.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.kb.from_ir import KBLoadError
from ein.kb.imports import _stdlib_root
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[2]
LOAD_DIR = REPO / "examples" / "broken" / "load"


def fixtures() -> list[Path]:
    """Every load-negative fixture, sorted."""
    return sorted(LOAD_DIR.glob("*.ein"))


def encode(msg: str, path: Path) -> str:
    """Message → `.expected` text: machine-specific paths to placeholders.

    `{FILE}` before `{DIR}`, since the file path contains the directory path.
    """
    return (msg
            .replace(str(_stdlib_root()), "{STDLIB}")
            .replace(str(path), "{FILE}")
            .replace(str(path.parent), "{DIR}"))


def decode(expected: str, path: Path) -> str:
    """`.expected` text → the message this machine should produce."""
    return (expected
            .replace("{STDLIB}", str(_stdlib_root()))
            .replace("{FILE}", str(path))
            .replace("{DIR}", str(path.parent)))


def load_error(name: str) -> str:
    """Load fixture ``name``, assert it fails with its recorded message, and
    return that message.

    Raises through pytest's assertion machinery, so a test body is just
    ``assert "…" in load_error("…")``.
    """
    path = LOAD_DIR / f"{name}.ein"
    assert path.is_file(), f"no load-negative fixture named {name!r}"
    with pytest.raises(KBLoadError) as exc:
        KnowledgeBase.from_file(str(path))
    msg = str(exc.value)
    expected_path = path.with_suffix(".expected")
    assert expected_path.is_file(), f"{name}: missing {expected_path.name}"
    expected = decode(expected_path.read_text(encoding="utf-8").rstrip("\n"), path)
    assert msg == expected, (
        f"{name}: message diverged from {expected_path.name}; if intentional, "
        f"re-run with UPDATE_GOLDEN=1 and review the diff."
    )
    return msg
