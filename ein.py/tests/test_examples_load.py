"""Every example under `examples/` (outside `broken/`) must load.

Guards a coverage gap that previously let examples rot unnoticed: the demo
glob in `tests/inference/test_demos.py` only walks `examples/saturation/`, and
the other example tests reference specific files by name — so a file elsewhere
(e.g. `examples/branching/`) could break after a config-flag rename or a
loader change and the suite stayed green. Found exactly that in S1.9.E6b:
`10/11_backprop_*` set a config key the engine had renamed (→ `KBLoadError`),
and `features/03_forall` tripped an import-dedup bug. This test load-checks the
whole tree so it can't recur.

`examples/broken/` is the deliberate exception — curated parse/load *failures*,
validated in detail (lint rc != 0, with file:line:col) by
`tests/test_cli.py::test_broken_fixtures`. Here we only assert they do not
*silently* load.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.ir.parser import IRParseError
from ein.kb.from_ir import KBLoadError
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[2]
EXAMPLES = REPO / "examples"

# Every .ein outside broken/ — the well-formed puzzles + fixtures.
LOADABLE = sorted(p for p in EXAMPLES.rglob("*.ein") if "broken" not in p.parts)
BROKEN = sorted((EXAMPLES / "broken").glob("*.ein"))


def _rel(path: Path) -> str:
    return str(path.relative_to(EXAMPLES))


def test_loadable_collection_is_populated():
    """Guard against a *vacuous* pass — a glob that matches nothing would let
    the parametrized test "pass" with zero cases (the very way the rot hid).
    Assert the tree is found, including the branching/ demos."""
    assert len(LOADABLE) >= 40, (
        f"only {len(LOADABLE)} examples found — glob or layout broke?"
    )
    names = {_rel(p) for p in LOADABLE}
    assert "branching/10_kill_cache_on.ein" in names
    assert "branching/11_kill_cache_off.ein" in names


@pytest.mark.parametrize("path", LOADABLE, ids=_rel)
def test_example_loads(path: Path):
    """Each well-formed example loads via `from_file` without raising."""
    KnowledgeBase.from_file(str(path))


@pytest.mark.parametrize("path", BROKEN, ids=_rel)
def test_broken_example_does_not_silently_load(path: Path):
    """`broken/` fixtures must raise on load (detail in test_cli.py)."""
    with pytest.raises((IRParseError, KBLoadError)):
        KnowledgeBase.from_file(str(path))
