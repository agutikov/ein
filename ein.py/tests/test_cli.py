"""CLI entrypoint smoke tests."""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.cli import main


def test_ir_parse_zebra(capsys: pytest.CaptureFixture[str]):
    """`ein ir parse examples/zebra.ein` exits 0 and emits canonical IR."""
    repo_root = Path(__file__).resolve().parents[2]
    zebra = repo_root / "examples" / "zebra.ein"
    rc = main(["ir", "parse", str(zebra)])
    assert rc == 0
    out = capsys.readouterr().out
    # P1.7c: flat forms — zebra.ein opens with its first rule (broken
    # multi-line by the pretty-printer), and the puzzle facts/relations
    # are top-level forms (no block wrappers).
    assert out.startswith("(rule")
    assert "(co-located" in out
    assert "(query" in out


def test_ir_parse_resolve_zebra2(capsys: pytest.CaptureFixture[str]):
    """`ir parse --resolve examples/zebra2.ein` splices `(import std.macro …)`
    inline and emits a standalone file: no `(import …)` remains, the `forall`
    macro is present (zebra2 uses it), and there is no `open` macro (unused —
    tree-shaken)."""
    repo_root = Path(__file__).resolve().parents[2]
    zebra2 = repo_root / "examples" / "zebra2.ein"
    rc = main(["ir", "parse", "--resolve", str(zebra2)])
    assert rc == 0
    out = capsys.readouterr().out
    assert "(import " not in out
    assert "(macro forall " in out
    assert "(macro open " not in out          # unused → dropped


def test_ir_lint_ok(capsys: pytest.CaptureFixture[str]):
    repo_root = Path(__file__).resolve().parents[2]
    zebra = repo_root / "examples" / "zebra.ein"
    rc = main(["ir", "lint", str(zebra)])
    assert rc == 0


def test_ir_lint_failure(tmp_path: Path, capsys: pytest.CaptureFixture[str]):
    # Post-P1.7c `(unknown-head a b)` is a valid FLAT fact, so it lints
    # clean. A genuine lint failure now needs a real malformed form — a
    # top-level kernel primitive (`and` is shape-pinned, never a fact head).
    bad = tmp_path / "broken.ein"
    bad.write_text("(and a b)\n")
    rc = main(["ir", "lint", str(bad)])
    assert rc == 1
    err = capsys.readouterr().err
    assert "broken.ein:" in err


def test_ir_dot_zebra(capsys: pytest.CaptureFixture[str]):
    """`ein ir dot examples/zebra.ein` exits 0 and emits non-empty DOT."""
    repo_root = Path(__file__).resolve().parents[2]
    zebra = repo_root / "examples" / "zebra.ein"
    rc = main(["ir", "dot", str(zebra)])
    assert rc == 0
    out = capsys.readouterr().out
    assert "digraph" in out
    # At least the three signature shapes should appear in zebra:
    assert "shape=octagon" in out
    assert "shape=oval" in out
    assert "shape=box" in out


def test_ir_dot_failure(tmp_path: Path, capsys: pytest.CaptureFixture[str]):
    bad = tmp_path / "broken.ein"
    bad.write_text("(unclosed\n")
    rc = main(["ir", "dot", str(bad)])
    assert rc == 1
    err = capsys.readouterr().err
    assert "broken.ein:" in err


def _ir_dot_out(capsys, *extra: str) -> str:
    repo_root = Path(__file__).resolve().parents[2]
    zebra = repo_root / "examples" / "zebra.ein"
    rc = main(["ir", "dot", str(zebra), *extra])
    assert rc == 0
    return capsys.readouterr().out


def test_ir_dot_levi_collapses_binaries(capsys: pytest.CaptureFixture[str]):
    """Compact (default) draws binary facts as single arrows; --levi
    expands every binary into a Levi octagon, so it has strictly more
    octagons than the compact view."""
    compact = _ir_dot_out(capsys)
    levi = _ir_dot_out(capsys, "--levi")
    assert levi.count("octagon") > compact.count("octagon")


def test_ir_dot_env_levi_matches_flag(
    capsys: pytest.CaptureFixture[str], monkeypatch: pytest.MonkeyPatch,
):
    """EIN_RENDER_LEVI=1 is equivalent to passing --levi."""
    flag = _ir_dot_out(capsys, "--levi")
    monkeypatch.setenv("EIN_RENDER_LEVI", "1")
    env = _ir_dot_out(capsys)
    assert env == flag


def test_ir_dot_rule_mode_overlay(capsys: pytest.CaptureFixture[str]):
    """--rule-mode=overlay yields the dashed-RHS overlay (no clusters)."""
    out = _ir_dot_out(capsys, "--rule-mode", "overlay")
    assert "cluster_lhs" not in out
    assert "style=dashed" in out


@pytest.mark.parametrize("name", [
    "unclosed_paren.ein",
    "keyword_as_value.ein",
    "bare_top_level_atom.ein",
    # `instance_in_ontology.ein` retired in S1.7.6: it tested parse-time
    # rejection of `(instance Norwegian)` (arity-1), but `instance` is no
    # longer a reserved declarator with pinned arity — it parses as an
    # ordinary generic fact, so there is no lint failure to assert.
    "rule_missing_params.ein",
])
def test_broken_fixtures(name: str, capsys: pytest.CaptureFixture[str]):
    """Each curated broken fixture lints non-zero with file:line:col in stderr."""
    repo_root = Path(__file__).resolve().parents[2]
    f = repo_root / "examples" / "broken" / name
    assert f.exists(), f"missing fixture: {f}"
    rc = main(["ir", "lint", str(f)])
    assert rc == 1, f"expected lint failure for {name}"
    err = capsys.readouterr().err
    assert name in err, f"error message missing filename: {err!r}"
    # The location prefix is "<file>:<line>:<col>: ..."
    assert ":" in err.split(name, 1)[1].split(":", 1)[0] + ":"
