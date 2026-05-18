"""CLI entrypoint smoke tests."""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.cli import main


def test_legacy_dot_output(conditions_path: Path, capsys: pytest.CaptureFixture[str]):
    rc = main(["legacy", str(conditions_path)])
    assert rc == 0
    out = capsys.readouterr().out
    assert out.startswith("digraph G {")
    assert "House_1" in out


def test_legacy_dump_flag(conditions_path: Path, capsys: pytest.CaptureFixture[str]):
    rc = main(["legacy", str(conditions_path), "--dump"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "House_1" in out
    # Dump format is whitespace-triples, not DOT.
    assert "digraph" not in out


def test_legacy_no_color_flag(conditions_path: Path, capsys: pytest.CaptureFixture[str]):
    rc = main(["legacy", str(conditions_path), "--no-color"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "<font" not in out
    assert '[label="is"]' in out


def test_ir_parse_zebra(capsys: pytest.CaptureFixture[str]):
    """`ein-bot ir parse examples/zebra.ein` exits 0 and emits canonical IR."""
    repo_root = Path(__file__).resolve().parent.parent
    zebra = repo_root / "examples" / "zebra.ein"
    rc = main(["ir", "parse", str(zebra)])
    assert rc == 0
    out = capsys.readouterr().out
    assert out.startswith("(rules") or out.startswith("(ontology")
    assert "(facts" in out


def test_ir_lint_ok(capsys: pytest.CaptureFixture[str]):
    repo_root = Path(__file__).resolve().parent.parent
    zebra = repo_root / "examples" / "zebra.ein"
    rc = main(["ir", "lint", str(zebra)])
    assert rc == 0


def test_ir_lint_failure(tmp_path: Path, capsys: pytest.CaptureFixture[str]):
    bad = tmp_path / "broken.ein"
    bad.write_text("(unknown-head a b)\n")
    rc = main(["ir", "lint", str(bad)])
    assert rc == 1
    err = capsys.readouterr().err
    assert "broken.ein:" in err


@pytest.mark.parametrize("name", [
    "unclosed_paren.ein",
    "keyword_as_value.ein",
    "bare_top_level_atom.ein",
    "instance_in_ontology.ein",
    "rule_missing_params.ein",
])
def test_broken_fixtures(name: str, capsys: pytest.CaptureFixture[str]):
    """Each curated broken fixture lints non-zero with file:line:col in stderr."""
    repo_root = Path(__file__).resolve().parent.parent
    f = repo_root / "examples" / "broken" / name
    assert f.exists(), f"missing fixture: {f}"
    rc = main(["ir", "lint", str(f)])
    assert rc == 1, f"expected lint failure for {name}"
    err = capsys.readouterr().err
    assert name in err, f"error message missing filename: {err!r}"
    # The location prefix is "<file>:<line>:<col>: ..."
    assert ":" in err.split(name, 1)[1].split(":", 1)[0] + ":"
