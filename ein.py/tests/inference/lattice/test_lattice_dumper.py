"""LatticeDumper file-I/O tests — S1.5b.29 T1.5b.29.6.

Verifies the per-hypothesis audit folder layout under each entry x
flag combination. The layout is **per-layer grouped** (S1.5b.30):

- ``enterings/layer_NN/<C-slug>/`` — one folder per commitment
  tested at that layer, carrying ``outcome.txt`` (alive | dead-pre
  | dead-post | solution) plus the emission artefacts for that
  outcome (``firings.jsonl`` + ``unconditional_facts.jsonl`` for
  every saturated fork; ``kb.ein`` for solutions; ``unsat_core.jsonl``
  + ``learned_clause.json`` for deaths).
- ``kb_index/layer_NN/kb_<i>/`` — under ``store_lattice=True``,
  ordered ids per layer (``kb_0`` … ``kb_n``) rather than
  hash-named folders.
- ``proof_summary.json`` indexes everything; each solution /
  dead entry carries a ``path`` pointing at its ``enterings/``
  folder.
- ``out_dir=None`` produces no on-disk artefacts.

Cross-references:

- User-facing doc: ``docs/kernel/inference/lattice_dump.md``.
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


def _outcome_folders(dump_dir: Path) -> dict[str, list[Path]]:
    """Map outcome → list of ``enterings/`` folders carrying it."""
    by_outcome: dict[str, list[Path]] = {}
    for outcome_file in dump_dir.glob("enterings/layer_*/*/outcome.txt"):
        by_outcome.setdefault(
            outcome_file.read_text().strip(), [],
        ).append(outcome_file.parent)
    return by_outcome


# ── enterings/ solution folders under gaps ────────────────


def test_dumper_solutions_folder_under_gaps(tmp_path: Path):
    """``gaps_solve`` on branching/04 with dumper produces per-layer
    ``enterings/layer_NN/<slug>/`` folders; the ``solution``-outcome
    ones each carry commitment.json + kb.ein + firings.jsonl."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "gaps-dump"
    dumper = LatticeDumper(out_dir=dump_dir)
    gaps_solve(kb, max_set_size=3, dumper=dumper)

    assert (dump_dir / "enterings").is_dir()
    by_outcome = _outcome_folders(dump_dir)
    solution_folders = by_outcome.get("solution", [])
    assert len(solution_folders) >= 1
    for sub in solution_folders:
        assert (sub / "commitment.json").is_file()
        assert (sub / "kb.ein").is_file()
        assert (sub / "firings.jsonl").is_file()
        # commitment.json is a parseable list of FactId dicts.
        with (sub / "commitment.json").open() as fp:
            commit = json.load(fp)
        assert isinstance(commit, list)
        assert len(commit) >= 1  # at least one FactId
    # Per-layer grouping: folders live under layers/layer_NN/, never
    # at the flat enterings/ root.
    assert not list(dump_dir.glob("enterings/co-located*"))


# ── enterings/ dead folders under contradictions ──────────


def test_dumper_dead_folder_under_contradictions(tmp_path: Path):
    """``contradictions_solve`` on branching/04 produces
    ``dead-pre`` / ``dead-post`` outcome folders carrying
    commitment.json + unsat_core.jsonl + learned_clause.json.
    The layer + kind metadata lives in proof_summary, not in
    commitment.json (which is the bare FactId list)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "contra-dump"
    dumper = LatticeDumper(out_dir=dump_dir)
    contradictions_solve(kb, max_set_size=3, dumper=dumper)

    by_outcome = _outcome_folders(dump_dir)
    dead_folders = (
        by_outcome.get("dead-pre", []) + by_outcome.get("dead-post", [])
    )
    assert len(dead_folders) >= 1
    for sub in dead_folders:
        assert (sub / "commitment.json").is_file()
        assert (sub / "unsat_core.jsonl").is_file()
        assert (sub / "learned_clause.json").is_file()
        with (sub / "commitment.json").open() as fp:
            commit = json.load(fp)
        assert isinstance(commit, list)
        assert len(commit) >= 1

    # Layer + kind for every dead commitment is indexed in
    # proof_summary.json.
    with (dump_dir / "proof_summary.json").open() as fp:
        summary = json.load(fp)
    assert summary["dead_commitments"]
    for d in summary["dead_commitments"]:
        assert d["kind"] in ("dead-pre", "dead-post")
        assert "layer" in d
        assert (dump_dir / d["path"]).is_dir()


# ── proof_summary.json indexes everything ─────────────────


def test_dumper_proof_summary_indexes_everything(tmp_path: Path):
    """``proof_summary.json`` is written for both lattice
    entries; every ``path`` it lists resolves to an on-disk
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
    # Every solution listed has its per-layer enterings folder.
    assert summary["solutions"]
    for s in summary["solutions"]:
        assert (dump_dir / s["path"]).is_dir()
        assert s["path"].startswith("enterings/layer_")
    # stats has the LatticeStats field set.
    assert summary["stats"]["solutions_found"] == len(summary["solutions"])


# ── kb_index/ folder under store_lattice ──────────────────


def test_dumper_kb_index_folder_under_store_lattice(tmp_path: Path):
    """``--store-lattice`` produces ``kb_index/layer_NN/kb_<i>/``
    with per-layer ordered ids (one folder per :class:`SetNode`)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "gaps-store"
    dumper = LatticeDumper(out_dir=dump_dir)
    gaps_solve(
        kb, max_set_size=3, store_lattice=True, dumper=dumper,
    )

    kb_index_dir = dump_dir / "kb_index"
    assert kb_index_dir.is_dir()
    # Two-level layout: kb_index/layer_NN/kb_<i>/.
    layer_dirs = sorted(kb_index_dir.iterdir())
    assert layer_dirs
    for layer_dir in layer_dirs:
        assert layer_dir.name.startswith("layer_")
        node_dirs = sorted(layer_dir.iterdir())
        assert node_dirs
        # Ordered ids kb_0 … kb_n within the layer.
        assert [p.name for p in node_dirs] == [
            f"kb_{i}" for i in range(len(node_dirs))
        ]
        for sub in node_dirs:
            assert (sub / "canonical_set.json").is_file()
            assert (sub / "labels.json").is_file()
            assert (sub / "verdict.txt").is_file()
            assert (sub / "state_hash.txt").is_file()
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
    proof_summary, summary). Each ``entering`` record carries
    its ``outcome``, and at least one is a ``solution``."""
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
    assert "proof_summary" in event_names
    assert "summary" in event_names
    # Every entering carries an outcome; the solution outcome
    # (formerly a separate solution_recorded event) is now an
    # entering with outcome="solution".
    enterings = [e for e in events if e["event"] == "entering"]
    assert all("outcome" in e for e in enterings)
    assert any(e["outcome"] == "solution" for e in enterings)
    # Sequence numbers are monotonic.
    seqs = [e["seq"] for e in events]
    assert seqs == sorted(seqs)
