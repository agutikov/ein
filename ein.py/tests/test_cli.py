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


# ── one broken file, three entry points ────────────────────────────
#
# Building the load-negative corpus (M1a S1a.0.1) found the three commands
# disagreeing about how to report a file that parses and then fails to load:
# `solve` printed `kb load error: …`, `saturate` raised through to a
# traceback, and `render` said "no rule forms". Two of those are now the same
# message; the third is correct as it stands and says why.

BROKEN_LOAD = REPO / "examples" / "broken" / "load" / "relation_duplicate.ein"


@pytest.mark.parametrize("cmd", [
    ["solve", str(BROKEN_LOAD)],
    ["saturate", str(BROKEN_LOAD)],
])
def test_a_load_error_is_reported_not_raised(cmd, capsys):
    """`solve` and `saturate` both take the KB path, so both must diagnose a
    load failure the same way: one line on stderr, exit 1, no traceback."""
    assert main(cmd) == 1
    err = capsys.readouterr().err
    assert "kb load error: duplicate relation 'opaque'" in err
    assert "Traceback" not in err


def test_render_views_the_ir_not_the_kb(capsys):
    """`render` is the exception, and deliberately: its views render the
    *parsed IR*, never the KB, so a file that would fail to load still has
    rules to draw — or, here, does not. Reporting a load error it never
    triggered would be the inconsistency."""
    assert main(["render", "rules", str(BROKEN_LOAD)]) == 1
    err = capsys.readouterr().err
    assert "no rule forms" in err
    assert "kb load error" not in err


def test_a_parse_error_from_saturate_names_the_file(capsys):
    """…and `saturate`'s parse errors carry the file name, like `solve`'s.
    It parsed without `filename=`, so every location read `<string>:l:c`."""
    broken = REPO / "examples" / "broken" / "unclosed_paren.ein"
    assert main(["saturate", str(broken)]) == 1
    assert str(broken) in capsys.readouterr().err
