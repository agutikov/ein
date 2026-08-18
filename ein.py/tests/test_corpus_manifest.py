"""The parity corpus is complete — `conformance/corpus.toml` (S1a.0.1).

The manifest lists every `.ein` file the two implementations are compared on.
Its value depends entirely on being exhaustive: an unlisted file is a hole the
harness reports as green. So the bijection between the tree and the manifest is
checked here, and again on the Rust side
(`ein.rs/crates/ein-conformance`) — both suites, because either one alone can
be the suite nobody ran.

This is the mechanical version of the rule
[`examples/README.md`](../../examples/README.md) already states in prose. Format
and the group vocabulary: [`conformance/README.md`](../../conformance/README.md).
"""
from __future__ import annotations

from pathlib import Path

import pytest
import tomllib

REPO = Path(__file__).resolve().parents[2]
MANIFEST = REPO / "conformance" / "corpus.toml"

SCHEMA = "ein-corpus/1"
GROUPS = {"positive", "parse-negative", "load-negative", "stdlib",
          "golden", "generated", "crash-parity"}


def _stdlib_dir() -> Path:
    """The one checked-in stdlib — repo-root `stdlib/` since S1a.0.3.

    Deliberately NOT `Path(ein.__file__).parent / "stdlib"`: that is the
    build-time copy `_build.py` writes into the package, a git-ignored build
    product the corpus does not list and must not be asked to. Reading it here
    made this check fail the moment an editable install started working again
    (2026-08-18) — seven `.ein` files "missing" from the manifest that are the
    same seven it already lists under `stdlib/`.
    """
    return REPO / "stdlib"


def _tracked() -> set[str]:
    """Every `.ein` the corpus must cover, repo-root-relative."""
    out = {str(p.relative_to(REPO)) for p in (REPO / "examples").rglob("*.ein")}
    out |= {str(p.relative_to(REPO)) for p in _stdlib_dir().glob("*.ein")}
    return out


@pytest.fixture(scope="module")
def manifest() -> dict:
    return tomllib.loads(MANIFEST.read_text(encoding="utf-8"))


def test_schema_is_the_expected_version(manifest: dict):
    assert manifest["schema"] == SCHEMA


def test_every_ein_file_has_an_entry(manifest: dict):
    """The completeness check. A new fixture without a manifest entry fails
    here, in the same commit that adds it."""
    listed = {e["path"] for e in manifest["entry"]}
    missing = sorted(_tracked() - listed)
    assert not missing, (
        f"{len(missing)} .ein file(s) with no conformance/corpus.toml entry: "
        f"{missing}"
    )


def test_every_entry_names_a_real_file(manifest: dict):
    """…and the other direction: a stale entry (a file renamed or deleted
    without updating the manifest) is just as much a hole, because the runner
    would report it as a cell that produced nothing on both sides."""
    stale = sorted(e["path"] for e in manifest["entry"]
                   if not (REPO / e["path"]).is_file())
    assert not stale, f"entries naming files that do not exist: {stale}"


def test_paths_are_unique(manifest: dict):
    paths = [e["path"] for e in manifest["entry"]]
    dupes = sorted({p for p in paths if paths.count(p) > 1})
    assert not dupes, f"duplicate entries: {dupes}"


def test_groups_are_from_the_vocabulary(manifest: dict):
    unknown = sorted({e["group"] for e in manifest["entry"]} - GROUPS)
    assert not unknown, f"unknown group(s): {unknown}"


def test_every_entry_has_at_least_one_run(manifest: dict):
    """An entry with no runs is listed but never executed — the exact hole the
    completeness check exists to close, one level down."""
    empty = sorted(e["path"] for e in manifest["entry"] if not e.get("runs"))
    assert not empty, f"entries with no runs: {empty}"


def test_negatives_are_grouped_by_where_they_fail(manifest: dict):
    """`broken/*.ein` fail at parse; `broken/load/*.ein` fail at load. The
    split is what lets P1a.1 gate on one and P1a.2 on the other.

    A file that *loads and then crashes the engine* is neither, and it is not
    in `broken/`: it is a well-formed input the engine mishandles, so it lives
    with the other bug-repro puzzles and carries the `crash-parity` group,
    which compares exit code and exception class rather than output.
    """
    by_path = {e["path"]: e["group"] for e in manifest["entry"]}
    for path, group in by_path.items():
        if path.startswith("examples/broken/load/"):
            assert group == "load-negative", path
        elif path.startswith("examples/broken/"):
            assert group == "parse-negative", path
        elif path.startswith("examples/"):
            assert group in ("positive", "crash-parity"), path


def test_the_load_negative_group_matches_the_fixture_directory(manifest: dict):
    """Cross-check against the other half of S1a.0.1: every load-negative
    fixture is a corpus entry, and every load-negative entry has its
    `.expected` beside it."""
    from tests.load_negative import fixtures

    listed = {e["path"] for e in manifest["entry"] if e["group"] == "load-negative"}
    on_disk = {str(p.relative_to(REPO)) for p in fixtures()}
    assert listed == on_disk
    for path in sorted(listed):
        assert (REPO / path).with_suffix(".expected").is_file(), path
