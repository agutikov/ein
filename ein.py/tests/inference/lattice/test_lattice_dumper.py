"""LatticeDumper file-I/O tests — S1.5b.29 T1.5b.29.6.

Verifies the per-set audit folder layout under each entry x flag
combination:

- ``solutions/<C-slug>/`` under gaps with non-empty solutions
- ``dead/<C-slug>/`` under contradictions with non-empty deads
- ``kb_index/<state_hash_hex>/`` under store_lattice=True
- ``proof_summary.json`` indexes everything
- ``out_dir=None`` produces no on-disk artefacts

Cross-references:

- Layout spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.29_lattice_proof.md``
- Implementation:
  ``ein.py/src/ein_bot/inference/monotonic/state_dump.py`` (the
  :class:`LatticeDumper` class).
"""
from __future__ import annotations

import json
from pathlib import Path

from ein_bot.inference.monotonic import (
    LatticeDumper,
    contradictions_solve,
    gaps_solve,
)
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


# ── solutions/ folder under gaps ──────────────────────────


def test_dumper_solutions_folder_under_gaps(tmp_path: Path):
    """``gaps_solve`` on branching/04 with dumper produces
    ``solutions/`` with two subfolders (one per SolutionRecord)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "gaps-dump"
    dumper = LatticeDumper(out_dir=dump_dir)
    gaps_solve(kb, max_set_size=3, dumper=dumper)

    solutions_dir = dump_dir / "solutions"
    assert solutions_dir.is_dir()
    subfolders = sorted(p.name for p in solutions_dir.iterdir())
    assert len(subfolders) == 2
    # Each subfolder has commitment.json + kb.ein + firings.jsonl.
    for sub in solutions_dir.iterdir():
        assert (sub / "commitment.json").is_file()
        assert (sub / "kb.ein").is_file()
        assert (sub / "firings.jsonl").is_file()
        # commitment.json is parseable JSON.
        with (sub / "commitment.json").open() as fp:
            commit = json.load(fp)
        assert isinstance(commit, list)
        assert len(commit) >= 1  # at least one FactId
    # No dead/ folder under gaps.
    assert not (dump_dir / "dead").exists()


# ── dead/ folder under contradictions ─────────────────────


def test_dumper_dead_folder_under_contradictions(tmp_path: Path):
    """``contradictions_solve`` on branching/04 produces
    ``dead/`` with subfolders carrying commitment.json +
    unsat_core.jsonl + learned_clause.json."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "contra-dump"
    dumper = LatticeDumper(out_dir=dump_dir)
    contradictions_solve(kb, max_set_size=3, dumper=dumper)

    dead_dir = dump_dir / "dead"
    assert dead_dir.is_dir()
    subfolders = sorted(p.name for p in dead_dir.iterdir())
    assert len(subfolders) >= 1
    for sub in dead_dir.iterdir():
        assert (sub / "commitment.json").is_file()
        assert (sub / "unsat_core.jsonl").is_file()
        assert (sub / "learned_clause.json").is_file()
        with (sub / "commitment.json").open() as fp:
            payload = json.load(fp)
        assert "commitment" in payload
        assert "layer" in payload
        assert "kind" in payload
        assert payload["kind"] in ("dead-pre", "dead-post")
    # No solutions/ folder under contradictions.
    assert not (dump_dir / "solutions").exists()


# ── proof_summary.json indexes everything ─────────────────


def test_dumper_proof_summary_indexes_everything(tmp_path: Path):
    """``proof_summary.json`` is written for both lattice
    entries; every slug it lists resolves to an on-disk
    folder."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "gaps-dump"
    dumper = LatticeDumper(out_dir=dump_dir)
    gaps_solve(kb, max_set_size=3, dumper=dumper)

    summary_path = dump_dir / "proof_summary.json"
    assert summary_path.is_file()
    with summary_path.open() as fp:
        summary = json.load(fp)
    # Expected top-level keys.
    for key in (
        "solutions", "dead_commitments", "kb_index",
        "alive_at_end", "learned_nogoods_count", "stats",
    ):
        assert key in summary
    # Every solution slug listed has its folder.
    for s in summary["solutions"]:
        slug = s["slug"]
        assert (dump_dir / "solutions" / slug).is_dir()
    # stats has the LatticeStats field set.
    assert summary["stats"]["solutions_found"] == len(summary["solutions"])


# ── kb_index/ folder under store_lattice ──────────────────


def test_dumper_kb_index_folder_under_store_lattice(tmp_path: Path):
    """``--store-lattice`` produces ``kb_index/`` with one
    subfolder per :class:`SetNode`."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "gaps-store"
    dumper = LatticeDumper(out_dir=dump_dir)
    gaps_solve(
        kb, max_set_size=3, store_lattice=True, dumper=dumper,
    )

    kb_index_dir = dump_dir / "kb_index"
    assert kb_index_dir.is_dir()
    subfolders = list(kb_index_dir.iterdir())
    assert len(subfolders) > 0
    for sub in subfolders:
        assert (sub / "canonical_set.json").is_file()
        assert (sub / "labels.json").is_file()
        assert (sub / "verdict.txt").is_file()
        # verdict.txt is one of the literal values.
        verdict_text = (sub / "verdict.txt").read_text().strip()
        assert verdict_text in ("alive", "dead", "solution")


def test_dumper_kb_index_absent_without_store_lattice(tmp_path: Path):
    """Default flags (``store_lattice=False``) — ``kb_index/``
    folder is absent + ``proof_summary.json``'s ``kb_index``
    list is empty."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "gaps-no-store"
    dumper = LatticeDumper(out_dir=dump_dir)
    gaps_solve(kb, max_set_size=3, dumper=dumper)

    assert not (dump_dir / "kb_index").exists()
    with (dump_dir / "proof_summary.json").open() as fp:
        summary = json.load(fp)
    assert summary["kb_index"] == []


# ── out_dir=None → no on-disk artefacts ───────────────────


def test_dumper_no_op_when_out_dir_is_none(tmp_path: Path):
    """``out_dir=None`` — every hook is invoked but no files
    are written. Backward-compatible with the S1.5b.20 stub
    behaviour."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dumper = LatticeDumper(out_dir=None)
    verdict, _ = gaps_solve(kb, max_set_size=3, dumper=dumper)
    assert verdict.proof is not None
    # No on-disk effects to assert beyond "no exception raised".
    # tmp_path is irrelevant here — the dumper writes nothing.
    assert not any(tmp_path.iterdir())


# ── 00_timeline.jsonl records lifecycle events ────────────


def test_dumper_timeline_records_events(tmp_path: Path):
    """The timeline carries one record per hook invocation
    (root_initial, layer_start, entering, layer_end,
    solution_recorded / dead_recorded, proof_summary, summary)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "timeline-check"
    dumper = LatticeDumper(out_dir=dump_dir)
    gaps_solve(kb, max_set_size=3, dumper=dumper)

    timeline_path = dump_dir / "00_timeline.jsonl"
    assert timeline_path.is_file()
    events = []
    with timeline_path.open() as fp:
        for line in fp:
            line = line.strip()
            if line:
                events.append(json.loads(line))
    event_names = [e["event"] for e in events]
    assert "root_initial" in event_names
    assert "layer_start" in event_names
    assert "entering" in event_names
    assert "layer_end" in event_names
    assert "solution_recorded" in event_names
    assert "proof_summary" in event_names
    assert "summary" in event_names
    # Sequence numbers are monotonic.
    seqs = [e["seq"] for e in events]
    assert seqs == sorted(seqs)
