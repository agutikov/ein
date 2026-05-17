"""CLI entrypoint smoke tests."""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.cli import main


def test_dot_output(conditions_path: Path, capsys: pytest.CaptureFixture[str]):
    rc = main([str(conditions_path)])
    assert rc == 0
    out = capsys.readouterr().out
    assert out.startswith("digraph G {")
    assert "House_1" in out


def test_dump_flag(conditions_path: Path, capsys: pytest.CaptureFixture[str]):
    rc = main([str(conditions_path), "--dump"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "House_1" in out
    # Dump format is whitespace-triples, not DOT.
    assert "digraph" not in out


def test_no_color_flag(conditions_path: Path, capsys: pytest.CaptureFixture[str]):
    rc = main([str(conditions_path), "--no-color"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "<font" not in out
    assert '[label="is"]' in out
