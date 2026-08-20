"""The corpus is complete — `corpus/corpus.toml` (S1a.0.1).

The manifest lists every `.ein` file the engine is exercised over. Its value
depends entirely on being exhaustive: an unlisted file is a hole every sweep
reports as green. So the bijection between the tree and the manifest is checked
here, and again on the Rust side (`ein.rs/crates/ein-corpus/src/manifest.rs`).

The Rust twin owns all nine of these claims since T1a.10.1.1, so this file is a
second copy of a settled contract rather than half of one — it goes with the
suite at S1a.10.5.

This is the mechanical version of the rule
[`examples/README.md`](../../examples/README.md) already states in prose. Format
and the group vocabulary: [`corpus/README.md`](../../corpus/README.md).
"""
from __future__ import annotations

from pathlib import Path

import pytest
import tomllib

REPO = Path(__file__).resolve().parents[2]
MANIFEST = REPO / "corpus" / "corpus.toml"

SCHEMA = "ein-corpus/2"
GROUPS = {"positive", "stdlib", "parse-negative", "load-negative",
          "compile-negative", "regression", "generated"}


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
        f"{len(missing)} .ein file(s) with no corpus/corpus.toml entry: "
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
    """A group is a directory: `broken/*.ein` fail at parse,
    `broken/load/*.ein` at load, `broken/compile/*.ein` at compile — the split
    is what lets P1a.1 gate on one, P1a.2 on the next and P1a.3 on the third —
    and `ein-bugs/*.ein` are the bug-repro puzzles, which fail at no fixed
    point because some of them no longer fail at all.

    That last group was `crash-parity` until S1a.10.3, and its membership rule
    was "ein.py raises an unhandled exception here" — neither a directory nor
    a fact about the language.
    """
    by_path = {e["path"]: e["group"] for e in manifest["entry"]}
    for path, group in by_path.items():
        if path.startswith("examples/broken/load/"):
            assert group == "load-negative", path
        elif path.startswith("examples/broken/compile/"):
            # S1a.3.1 — they parse and load, then the compiler refuses; the
            # message is pinned byte-for-byte by the `.expected` files beside
            # them. `activator_arity` is the exception: the S1.22.0 arity
            # filter makes its error unreachable through the engine, so its
            # run succeeds and it is an ordinary `positive`.
            assert group in ("compile-negative", "positive"), path
        elif path.startswith("examples/broken/"):
            assert group == "parse-negative", path
        elif path.startswith("examples/ein-bugs/"):
            assert group == "regression", path
        elif path.startswith("examples/"):
            assert group == "positive", path


def test_every_compile_negative_fixture_has_its_expected(manifest: dict):
    """The other half of S1a.3.1: every `examples/broken/compile/*.ein` is a
    corpus entry and has its `.expected` beside it, and nothing else claims to
    be one."""
    d = REPO / "examples" / "broken" / "compile"
    eins = sorted(p.stem for p in d.glob("*.ein"))
    assert eins, f"no fixtures in {d}"
    assert eins == sorted(p.stem for p in d.glob("*.expected"))
    listed = {e["path"] for e in manifest["entry"]}
    for stem in eins:
        assert f"examples/broken/compile/{stem}.ein" in listed


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
