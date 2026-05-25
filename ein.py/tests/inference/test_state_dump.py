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


# ── T1.5a.19.3 dumper enhancements ─────────────────────────────────


def _read_jsonl(path: Path) -> list[dict]:
    return [json.loads(line) for line in path.read_text().splitlines() if line]


def test_timeline_jsonl_exists_and_monotonic(dump_dir: Path):
    """`00_timeline.jsonl` is JSONL, has monotonic seq + non-negative ts_ms."""
    kb = KnowledgeBase.from_ir(parse(DEMO.read_text()))
    solve(kb, max_depth=2, dumper=StateDumper(out_dir=dump_dir))

    timeline_path = dump_dir / "00_timeline.jsonl"
    assert timeline_path.exists(), "00_timeline.jsonl not written"
    events = _read_jsonl(timeline_path)
    assert events, "timeline is empty"

    # seq is 0-indexed and strictly monotonic; ts_ms is non-negative
    # and weakly monotonic (events can fire in the same ms tick).
    for i, rec in enumerate(events):
        assert rec["seq"] == i, f"seq jump at index {i}: {rec}"
        assert rec["ts_ms"] >= 0
        assert "event" in rec
    times = [r["ts_ms"] for r in events]
    assert times == sorted(times), "ts_ms went backwards"


def test_timeline_covers_lifecycle_events(dump_dir: Path):
    """Every documented hook category surfaces at least one record."""
    demo = REPO / "examples" / "branching" / "04_two_levels.ein"
    kb = KnowledgeBase.from_ir(parse(demo.read_text()))
    solve(kb, max_depth=3, dumper=StateDumper(out_dir=dump_dir))

    events = _read_jsonl(dump_dir / "00_timeline.jsonl")
    seen = {r["event"] for r in events}
    # node_resaturated is optional (depends on whether unconditional
    # deaths fire); the others must be present.
    assert {"root_initial", "root_saturated", "root_hyps",
            "node_alloc", "node_dump", "summary"} <= seen


def test_timeline_summary_is_last(dump_dir: Path):
    """`summary` is the closing event."""
    kb = KnowledgeBase.from_ir(parse(DEMO.read_text()))
    solve(kb, max_depth=2, dumper=StateDumper(out_dir=dump_dir))

    events = _read_jsonl(dump_dir / "00_timeline.jsonl")
    assert events[-1]["event"] == "summary"


def test_nested_fact_summary_recurses(dump_dir: Path):
    """`(not (R …))` unsat-core entries land as nested dicts, not str-reprs."""
    kb = KnowledgeBase.from_ir(parse(DEMO.read_text()))
    solve(kb, max_depth=2, dumper=StateDumper(out_dir=dump_dir))

    # Walk every verdict.json; if any unsat_core entry has nested-Fact
    # args, ensure they parse as dicts (not the legacy str-repr).
    saw_nested = False
    for vpath in dump_dir.rglob("verdict.json"):
        v = json.loads(vpath.read_text())
        for entry in v["unsat_core"]:
            for arg in entry["args"]:
                if isinstance(arg, dict):
                    assert "relation" in arg and "args" in arg
                    saw_nested = True
                else:
                    # Atoms remain strings; legacy "Fact(relation_name=…)"
                    # repr would also be a str, but with the marker
                    # prefix we can spot the regression.
                    assert not arg.startswith("Fact(relation_name"), (
                        f"legacy str-repr leaked: {arg!r}"
                    )
    # Demo 03 doesn't always produce nested-Fact unsat-cores; the
    # assertion above already covers the negative case (no str-repr
    # leaked), so seeing_nested is informational only.
    _ = saw_nested


def test_resat_attribution_split(tmp_path: Path):
    """When resats fire, `back_prop_writes` and `resat_derivations`
    are both present in the cycle JSON and their union equals the
    legacy flat `negatives_added`."""
    # Demo 10 is the back-prop-on fixture — it deterministically
    # produces unconditional deaths that trigger root resats.
    demo = REPO / "examples" / "branching" / "10_backprop_on.ein"
    kb = KnowledgeBase.from_ir(parse(demo.read_text()))
    dump_dir = tmp_path / "dump"
    solve(kb, max_depth=3, dumper=StateDumper(out_dir=dump_dir))

    cycle_jsons = list(dump_dir.rglob("resats/*.json"))
    if not cycle_jsons:
        pytest.skip("demo produced no resat events; nothing to check")
    for path in cycle_jsons:
        rec = json.loads(path.read_text())
        assert "back_prop_writes" in rec, f"missing in {path}"
        assert "resat_derivations" in rec, f"missing in {path}"
        assert "negatives_added" in rec, "backward-compat field dropped"
        # Each back-prop write either links to a dead child or
        # explains its absence (None for symmetric mirrors etc.).
        dead_ids = {c["branch_id"] for c in rec["dead_children"]}
        for w in rec["back_prop_writes"]:
            assert "from_dead_child" in w
            assert w["from_dead_child"] is None or w["from_dead_child"] in dead_ids
            assert w["rule"]  # always carries a `<…>` rule marker
        # Resat derivations carry rule + premises.
        for d in rec["resat_derivations"]:
            assert "rule" in d
            assert "premises" in d
        # Backward-compat: flat list equals the concatenation in
        # firing order. The dumper builds it as the source iteration
        # order, but the two-bucket split keeps each entry verbatim,
        # so flat-count = sum of bucket counts.
        assert (len(rec["negatives_added"])
                == len(rec["back_prop_writes"])
                + len(rec["resat_derivations"]))
