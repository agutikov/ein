"""The load-negative corpus — `examples/broken/load/**` (M1a S1a.0.1 T1a.0.1.2).

Every `KBLoadError` the loader can raise from a *parseable* file is a fixture
there: a `.ein` that fails to load, beside a `.expected` holding the exact
message. Before this, those cases lived as inline source strings inside a dozen
unrelated test modules under `pytest.raises(..., match=<regex>)`, which pinned a
fragment of each message and nothing else.

Two reasons the files matter more than the regexes:

1. **ein.rs has to reproduce the text.** The Rust port's P1a.2 gate is
   byte-identical load errors
   ([`plans/m1a_rust/design/01`](../../../plans/m1a_rust/design/01_parity_contract.md)
   §2, T3), and a regex fragment is not something a second implementation can
   be held to. A checked-in message is.
2. **It makes the loader's error surface visible as data.** This module is the
   count; [`examples/broken/load/README.md`](../../../examples/broken/load/README.md)
   records the format, the placeholders, and which loader messages have no
   fixture and why.

Refresh after an intentional message change with `UPDATE_GOLDEN=1 pytest
tests/kb/test_load_negative.py`, and read the diff.
"""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from tests.load_negative import decode, encode, fixtures, load_error

FIXTURES = fixtures()
REPO = Path(__file__).resolve().parents[3]


def test_corpus_is_populated():
    """Guard against a vacuous pass — a glob that matched nothing would let
    the parametrized tests below "pass" with zero cases."""
    assert len(FIXTURES) >= 29, f"only {len(FIXTURES)} load-negative fixtures"
    names = {p.stem for p in FIXTURES}
    # One from each of the loader's two error-raising modules.
    assert "relation_duplicate" in names          # kb/from_ir.py
    assert "import_module_not_found" in names     # kb/imports.py


@pytest.mark.parametrize("path", FIXTURES, ids=lambda p: p.stem)
def test_fixture_message_matches_expected(path: Path):
    """Each fixture raises `KBLoadError` with exactly its recorded message."""
    expected_path = path.with_suffix(".expected")
    if os.environ.get("UPDATE_GOLDEN") or not expected_path.exists():
        from ein.kb.from_ir import KBLoadError
        from ein.kb.store import KnowledgeBase
        with pytest.raises(KBLoadError) as exc:
            KnowledgeBase.from_file(str(path))
        expected_path.write_text(encode(str(exc.value), path) + "\n",
                                 encoding="utf-8")
        if not os.environ.get("UPDATE_GOLDEN"):
            pytest.skip(f"{path.stem}: expected message captured; re-run.")
    load_error(path.stem)


@pytest.mark.parametrize("path", FIXTURES, ids=lambda p: p.stem)
def test_expected_holds_no_absolute_path(path: Path):
    """A `.expected` carrying a machine-specific path would pass here and fail
    everywhere else, so the placeholders are enforced rather than trusted."""
    text = path.with_suffix(".expected").read_text(encoding="utf-8")
    assert str(REPO) not in text, "use {FILE} / {DIR} / {STDLIB}"


@pytest.mark.parametrize("name", [
    "import_cycle", "import_cycle_b", "import_module_not_found",
    "import_relative_not_found", "macro_arity_mismatch",
])
def test_placeholders_round_trip(name: str):
    """`encode` ∘ `decode` is the identity on the five fixtures whose message
    is machine-specific at all — the two ends of the import cycle, the two
    module-not-found paths, and the one message carrying a real `Loc`."""
    path = FIXTURES[0].parent / f"{name}.ein"
    raw = path.with_suffix(".expected").read_text(encoding="utf-8").rstrip("\n")
    assert "{" in raw, f"{name}: expected a placeholder"
    assert encode(decode(raw, path), path) == raw
