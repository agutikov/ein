"""Every `CompileError` says exactly what its `.expected` says — S1a.3.1.

The four shapes `compile.py` refuses are authoring errors a rule author reads,
so their text is a surface, not an implementation detail. The corpus is
[`examples/broken/compile/`](../../../examples/broken/compile/README.md); the
Rust port is held to the same four messages by
`ein.rs/crates/ein-infer/tests/compile_parity.rs`, which compares against the
live oracle *and* against these same files.

All four used to be a silent ``return []``. The fixtures' own comments record
what each silently did; what this module pins is that they now say so.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.inference.compile import CompileError, compile_rule
from ein.inference.engine import Engine
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
COMPILE_DIR = REPO / "examples" / "broken" / "compile"


def fixtures() -> list[Path]:
    return sorted(COMPILE_DIR.glob("*.ein"))


def _compile_all_unfiltered(kb: KnowledgeBase) -> None:
    """Compile every (rule, rule-application-fact) pair, arity filter **off**.

    `Engine._activators_for` drops a mismatched activator before the compiler
    sees it (S1.22.0), which is why the arity `CompileError` is unreachable
    through the engine and why this walk exists: it is what a direct caller of
    `compile_rule` is.
    """
    for rule in kb.rules.values():
        if not rule.params:
            activators = (None,)
        else:
            activators = tuple(kb._rule_apps_by_rule.get(rule.name, ()))
        for activator in activators:
            compile_rule(rule, activator)


@pytest.mark.parametrize("path", fixtures(), ids=lambda p: p.stem)
def test_the_message_is_its_expected_file(path: Path):
    expected = path.with_suffix(".expected")
    assert expected.is_file(), f"{path.name} has no .expected beside it"
    kb = KnowledgeBase.from_ir(parse(path.read_text(encoding="utf-8"), filename=str(path)),
                               base_dir=path.parent)
    with pytest.raises(CompileError) as excinfo:
        _compile_all_unfiltered(kb)
    assert str(excinfo.value) == expected.read_text(encoding="utf-8").rstrip("\n")


def test_the_directory_and_the_expected_files_agree():
    """No fixture without its message, and no message without its fixture."""
    eins = {p.stem for p in fixtures()}
    expecteds = {p.stem for p in COMPILE_DIR.glob("*.expected")}
    assert eins == expecteds
    assert len(eins) == 4, "one fixture per CompileError branch"


def test_the_arity_filter_is_what_keeps_the_fourth_unreachable():
    """`activator_arity.ein` compiles cleanly through the *engine*.

    Its `CompileError` is the second of S1.22.0's two guards; the first — the
    arity filter — is the one the engine actually relies on, so the rule simply
    never runs and the file is an ordinary `positive` corpus entry. If this
    ever raises, the filter regressed and the fixture stopped testing what it
    is for.
    """
    path = COMPILE_DIR / "activator_arity.ein"
    kb = KnowledgeBase.from_ir(parse(path.read_text(encoding="utf-8"), filename=str(path)),
                               base_dir=path.parent)
    engine = Engine(kb)
    engine.compile_all()          # no CompileError
    assert not engine.cache, (
        "the arity-mismatched activator authorised a plan — `pairwise` is the "
        "file's only rule, so an empty cache is the whole assertion"
    )
