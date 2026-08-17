"""Where `std.*` comes from — the three-step chain (M1a S1a.0.3).

The stdlib is checked in once, at repo-root `stdlib/`, and **both**
implementations read those bytes. That is a correctness requirement, not
tidiness: it is 1 231 lines of ein-lang that `zebra2.ein` imports from, so a
forked copy would make every parity result meaningless — a conformance diff
would report "the engines disagree" when in fact the *programs* differ
([design/11](../../../plans/m1a_rust/design/11_shared_assets.md)).

Resolution order, identical in `ein.py`'s `_stdlib_root` and `ein.rs`'s
`stdlib::resolve`:

1. `$EIN_STDLIB` — always wins; the conformance harness sets it so both
   engines demonstrably read the same bytes;
2. the checkout, found by walking up for a `stdlib/` carrying
   `MANIFEST.sha256`;
3. the packaged copy — the wheel's `ein/stdlib/`, written at build time.

The Rust half of the same contract is `ein-ir`'s `stdlib::tests`.
"""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from ein.kb.from_ir import KBLoadError
from ein.kb.imports import _STDLIB_MARKER, _stdlib_root, stdlib_macro_names
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
SHARED = REPO / "stdlib"


@pytest.fixture(autouse=True)
def _clear_macro_cache():
    """`stdlib_macro_names` caches on the resolved root, so a test that moves
    the root must not read the previous one's answer."""
    from ein.kb.imports import _cached_macro_names
    _cached_macro_names.cache_clear()
    yield
    _cached_macro_names.cache_clear()


def test_the_checkout_is_the_source_of_truth():
    assert _stdlib_root() == SHARED
    assert (SHARED / _STDLIB_MARKER).is_file()
    assert len(list(SHARED.glob("*.ein"))) == 7


def test_the_marker_is_what_identifies_it(tmp_path: Path, monkeypatch):
    """A directory called `stdlib/` proves nothing; the manifest is the
    marker, which is why an empty override does not silently succeed."""
    empty = tmp_path / "stdlib"
    empty.mkdir()
    monkeypatch.setenv("EIN_STDLIB", str(empty))
    assert _stdlib_root() == empty
    with pytest.raises(KBLoadError, match=r"module not found"):
        KnowledgeBase.from_file(str(REPO / "examples" / "zebra2.ein"))


def test_the_override_wins(tmp_path: Path, monkeypatch):
    """`$EIN_STDLIB` beats the checkout even when the checkout is right
    there — the harness relies on it to point both engines at one directory."""
    other = tmp_path / "elsewhere"
    other.mkdir()
    (other / "macro.ein").write_text("(macro only-here (?p) (rel ?p))\n",
                                     encoding="utf-8")
    monkeypatch.setenv("EIN_STDLIB", str(other))
    assert _stdlib_root() == other
    assert stdlib_macro_names() == {"only-here"}


def test_editing_a_module_takes_effect_without_a_rebuild(monkeypatch, tmp_path):
    """The reason a checkout outranks the packaged copy. Exercised through an
    override rather than by editing the real tree, so a failing run cannot
    leave the repo's stdlib modified."""
    root = tmp_path / "stdlib"
    root.mkdir()
    macro = root / "macro.ein"
    macro.write_text("(macro forall (?p) (rel ?p))\n", encoding="utf-8")
    monkeypatch.setenv("EIN_STDLIB", str(root))
    assert stdlib_macro_names() == {"forall"}

    from ein.kb.imports import _cached_macro_names
    macro.write_text("(macro forall (?p) (rel ?p))\n(macro open (?p) (rel ?p))\n",
                     encoding="utf-8")
    _cached_macro_names.cache_clear()
    assert stdlib_macro_names() == {"forall", "open"}


def test_a_packaged_copy_does_not_shadow_the_checkout(tmp_path, monkeypatch):
    """The bug this arrangement is most likely to grow: the build copy carries
    the marker too (it is verbatim), and the walk starts inside the package —
    so without an explicit skip a *stale* build product would outrank the
    checkout it was built from, which is the exact failure the single-source
    rule exists to prevent. Found by building a wheel in a checkout and
    watching resolution move.
    """
    monkeypatch.delenv("EIN_STDLIB", raising=False)
    packaged = Path(__import__("ein").__file__).resolve().parent / "stdlib"
    packaged.mkdir(parents=True, exist_ok=True)
    (packaged / _STDLIB_MARKER).write_text("deadbeef  fake.ein\n", encoding="utf-8")
    try:
        assert _stdlib_root() == SHARED
    finally:
        (packaged / _STDLIB_MARKER).unlink()
        if not any(packaged.iterdir()):
            packaged.rmdir()


def test_the_manifest_matches_the_files():
    """The drift check, in the suite as well as in `utils/stdlib_manifest.py`
    — the packaged and embedded copies are verified against this file, so it
    being right is load-bearing for both."""
    import hashlib

    recorded = {}
    for line in (SHARED / _STDLIB_MARKER).read_text(encoding="utf-8").splitlines():
        if line.strip():
            sha, _, name = line.partition("  ")
            recorded[name.strip()] = sha.strip()
    actual = {p.name: hashlib.sha256(p.read_bytes()).hexdigest()
              for p in SHARED.glob("*.ein")}
    assert recorded == actual, (
        "stdlib drift; if intentional: python3 utils/stdlib_manifest.py --write"
    )


def test_zebra2_still_loads_from_the_checkout():
    """The end-to-end claim: the puzzle that imports from three stdlib modules
    still loads, from the moved location, with nothing on `$EIN_STDLIB`."""
    assert "EIN_STDLIB" not in os.environ
    kb = KnowledgeBase.from_file(str(REPO / "examples" / "zebra2.ein"))
    assert kb.rules and kb.facts
