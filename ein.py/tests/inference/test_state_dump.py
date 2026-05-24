"""StateDumper smoke tests — S1.5a.11 T1.5a.11.

Asserts the dumper produces the documented directory structure on
a small branching demo, that summary.json carries the expected
top-level keys, and that the per-branch verdict.json round-trips
through json.loads.

The dumper is diagnostic — there's no semantic invariant the tests
pin beyond "produces files at the expected paths without raising".
"""
from __future__ import annotations

import json
from pathlib import Path

import pytest

from ein_bot.inference.solver import solve
from ein_bot.inference.state_dump import StateDumper
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
DEMO = REPO / "examples" / "branching" / "03_five_hyps_one_alive.ein"


@pytest.fixture()
def dump_dir(tmp_path: Path) -> Path:
    return tmp_path / "dump"


def test_dumper_produces_root_phase_files(dump_dir: Path):
    """Three root-phase artefacts at the documented paths."""
    kb = KnowledgeBase.from_ir(parse(DEMO.read_text()))
    solve(kb, max_depth=2, dumper=StateDumper(out_dir=dump_dir))

    assert (dump_dir / "00_root_initial.ein").exists()
    assert (dump_dir / "01_root_saturated.ein").exists()
    assert (dump_dir / "01_root_saturated" / "stats.json").exists()
    assert (dump_dir / "01_root_saturated" / "firings.jsonl").exists()
    assert (dump_dir / "02_root_hyps.ein").exists()
    assert (dump_dir / "02_root_hyps" / "hyp_stats.json").exists()


def test_dumper_produces_branch_files(dump_dir: Path):
    """Each visited branch produces its core 4-file kit."""
    kb = KnowledgeBase.from_ir(parse(DEMO.read_text()))
    solve(kb, max_depth=2, dumper=StateDumper(out_dir=dump_dir))

    branches = dump_dir / "branches"
    assert branches.exists()
    bdirs = sorted(branches.iterdir())
    assert bdirs, "no branch directories created"
    for bdir in bdirs:
        for fname in ("hypothesis.ein", "post_sat.ein",
                      "firings.jsonl", "verdict.json"):
            assert (bdir / fname).exists(), (
                f"branch {bdir.name} missing {fname}"
            )


def test_dumper_nests_subbranches(dump_dir: Path):
    """A branch with children must contain a `branches/` subfolder
    with one entry per child SearchNode."""
    demo = REPO / "examples" / "branching" / "04_two_levels.ein"
    kb = KnowledgeBase.from_ir(parse(demo.read_text()))
    solve(kb, max_depth=3, dumper=StateDumper(out_dir=dump_dir))

    # At least one direct child of root must itself have grandchildren.
    nested = False
    for child in (dump_dir / "branches").iterdir():
        if (child / "branches").exists() and any((child / "branches").iterdir()):
            nested = True
            break
    assert nested, (
        "expected at least one branch with a non-empty branches/ "
        "subfolder; demo 04_two_levels descends two levels"
    )


def test_dumper_summary_json(dump_dir: Path):
    """summary.json has the expected top-level keys + a parseable verdict."""
    kb = KnowledgeBase.from_ir(parse(DEMO.read_text()))
    solve(kb, max_depth=2, dumper=StateDumper(out_dir=dump_dir))

    summary = json.loads((dump_dir / "summary.json").read_text())
    assert {"verdict", "leaves", "tree_nodes",
            "branches_dumped", "elapsed_seconds", "config"} <= set(summary)
    assert summary["verdict"] in ("Solution", "Ambiguity", "Contradiction")
    assert summary["tree_nodes"] >= 2  # root + at least one branch
    assert {"solution", "dead", "open"} <= set(summary["leaves"])


def test_dumper_verdict_json_parseable(dump_dir: Path):
    """Every branch's verdict.json is parseable + has the documented shape."""
    kb = KnowledgeBase.from_ir(parse(DEMO.read_text()))
    solve(kb, max_depth=2, dumper=StateDumper(out_dir=dump_dir))

    def _walk(branches_dir: Path) -> int:
        seen = 0
        if not branches_dir.exists():
            return 0
        for bdir in branches_dir.iterdir():
            v = json.loads((bdir / "verdict.json").read_text())
            assert v["kind"] in ("solution", "open", "dead"), v["kind"]
            assert v["hypothesis"]["relation"]
            assert isinstance(v["hypothesis"]["args"], list)
            assert isinstance(v["firings"], int)
            assert isinstance(v["unsat_core"], list)
            seen += 1
            seen += _walk(bdir / "branches")
        return seen

    assert _walk(dump_dir / "branches") >= 1


def test_solve_without_dumper_unaffected():
    """Passing dumper=None (the default) is byte-identical to omitting it."""
    text = DEMO.read_text()

    kb_a = KnowledgeBase.from_ir(parse(text))
    v_a = solve(kb_a, max_depth=2)

    kb_b = KnowledgeBase.from_ir(parse(text))
    v_b = solve(kb_b, max_depth=2, dumper=None)

    assert type(v_a) is type(v_b)
    assert len(v_a.tree.nodes) == len(v_b.tree.nodes)
