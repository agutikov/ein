"""CLI entrypoint smoke tests — the ``ein`` console-script surface.

After the CLI restructure the operational subcommands are ``render`` /
``saturate`` / ``solve``; ``ir`` / ``kb`` were removed and ``profile`` /
``symmetric`` moved to standalone ``utils/`` scripts. The broken-fixture parse
check (formerly ``ein ir lint examples/broken/*.ein``) now exercises ``parse()``
directly — the same IRParseError, minus the removed CLI wrapper.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.cli import main
from ein.ir import IRParseError, parse

REPO = Path(__file__).resolve().parents[2]


def test_help_lists_operational_subcommands(capsys: pytest.CaptureFixture[str]):
    """``ein --help`` lists exactly render / solve / saturate."""
    with pytest.raises(SystemExit) as exc:
        main(["--help"])
    assert exc.value.code == 0
    assert "{render,solve,saturate}" in capsys.readouterr().out


@pytest.mark.parametrize("argv", [
    ["ir", "parse", "x.ein"],   # removed
    ["kb", "dot", "x.ein"],     # removed
    ["profile", "x.ein"],       # moved to utils/profile_solve.py
    ["symmetric"],              # moved to utils/symmetric_bench.py
])
def test_removed_and_moved_subcommands_rejected(argv: list[str]):
    """``ir`` / ``kb`` (removed) and ``profile`` / ``symmetric`` (moved to
    utils/) are no longer accepted by the ``ein`` dispatcher."""
    with pytest.raises(SystemExit) as exc:
        main(argv)
    assert exc.value.code != 0


@pytest.mark.parametrize("name", [
    "unclosed_paren.ein",
    "keyword_as_value.ein",
    "bare_top_level_atom.ein",
    # `instance_in_ontology.ein` retired in S1.7.6 — `instance` is no longer a
    # reserved declarator with pinned arity, so it parses as a generic fact.
    "rule_missing_params.ein",
])
def test_broken_fixtures_fail_to_parse(name: str):
    """Each curated ``examples/broken/`` fixture raises ``IRParseError`` with a
    ``file:line:col`` location (was ``ein ir lint``; the ``ir`` subcommand is
    gone, so this exercises ``parse()`` directly)."""
    f = REPO / "examples" / "broken" / name
    assert f.exists(), f"missing fixture: {f}"
    with pytest.raises(IRParseError) as exc:
        parse(f.read_text(encoding="utf-8"), filename=name)
    msg = str(exc.value)
    # "<file>:<line>:<col>: <detail>" — line/col are -1 for an EOF/unclosed
    # error, so assert the located-prefix shape, not specific numbers.
    assert msg.startswith(f"{name}:"), msg
    assert msg[len(name) + 1:].count(":") >= 2, msg   # line + col fields present
