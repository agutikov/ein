"""LatticeDumper file-I/O tests — S1.5b.29 (P1.7a refit, 2026-06-16).

Verifies the per-hypothesis audit folder layout produced by
:func:`solve` (exhaustive) with a :class:`LatticeDumper`. The dumper
receives the same ``entering`` lifecycle callbacks under ``solve`` as it
did under the removed lattice entries, so the ``enterings/`` folders
(``outcome.txt`` + the per-outcome artefacts) are unchanged.

What changed (P1.7a): ``solve`` does NOT build the per-SetNode DAG, so
``proof.kb_index`` is empty and the dumper therefore does NOT materialise
the ``kb_index/`` folder (it is written only when ``proof.kb_index`` is
populated). And without ``store_lattice`` ``solve`` attaches no proof at
all, so ``proof_summary.json`` is written only under
``store_lattice=True``.

Layout (per-layer grouped):

- ``enterings/layer_NN/<C-slug>/`` — one folder per commitment tested at
  that layer, carrying ``outcome.txt`` (alive | dead-pre | dead-post |
  solution) plus the outcome's emission artefacts.
- ``proof_summary.json`` indexes everything (under ``store_lattice``).
- ``out_dir=None`` produces no on-disk artefacts.

Cross-references:

- User-facing doc: ``docs/kernel/inference/lattice_dump.md``.
- Implementation:
  ``ein.py/src/ein/inference/monotonic/state_dump.py`` (the
  :class:`LatticeDumper` class).
"""
from __future__ import annotations

import json
from pathlib import Path

from ein.inference.monotonic import (
    LatticeDumper,
    solve,
)
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _solve(kb: KnowledgeBase, *, dumper, **kw):
    return solve(kb, stop_after=None, store_lattice=True, dumper=dumper, **kw)


def _outcome_folders(dump_dir: Path) -> dict[str, list[Path]]:
    """Map outcome → list of ``enterings/`` folders carrying it."""
    by_outcome: dict[str, list[Path]] = {}
    for outcome_file in dump_dir.glob("enterings/layer_*/*/outcome.txt"):
        by_outcome.setdefault(
            outcome_file.read_text().strip(), [],
        ).append(outcome_file.parent)
    return by_outcome


# ── enterings/ solution folders ───────────────────────────


def test_dumper_solution_folders(tmp_path: Path):
    """``solve`` on branching/04 (k=2 → Ambiguity) with a dumper produces
    per-layer ``enterings/layer_NN/<slug>/`` folders; the
    ``solution``-outcome ones each carry commitment.json + kb.ein +
    firings.jsonl."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "solve-dump"
    dumper = LatticeDumper(out_dir=dump_dir)
    _solve(kb, max_set_size=3, dumper=dumper)

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


# ── enterings/ dead folders ───────────────────────────────


def test_dumper_dead_folders(tmp_path: Path):
    """``solve`` on branching/04 records the deads explored along the way;
    their ``dead-pre`` / ``dead-post`` outcome folders carry
    commitment.json + unsat_core.jsonl + learned_clause.json. The layer +
    kind metadata lives in proof_summary."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "dead-dump"
    dumper = LatticeDumper(out_dir=dump_dir)
    _solve(kb, max_set_size=3, dumper=dumper)

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

    # Layer + kind for every (non-root) dead commitment is indexed in
    # proof_summary.json.
    with (dump_dir / "proof_summary.json").open() as fp:
        summary = json.load(fp)
    assert summary["dead_commitments"]
    for d in summary["dead_commitments"]:
        assert d["kind"] in ("dead-pre", "dead-post")
        assert "layer" in d
        # br04 has no root dead, so every dead has an enterings/ folder.
        assert (dump_dir / d["path"]).is_dir()


# ── proof_summary.json indexes everything ─────────────────


def test_dumper_proof_summary_indexes_everything(tmp_path: Path):
    """``proof_summary.json`` is written under ``store_lattice``; every
    solution ``path`` it lists resolves to an on-disk folder."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "summary-dump"
    dumper = LatticeDumper(out_dir=dump_dir)
    _solve(kb, max_set_size=3, dumper=dumper)

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


# ── kb_index/ folder is NOT produced by solve ─────────────


def test_dumper_kb_index_folder_absent_for_solve(tmp_path: Path):
    """``solve`` does not build the per-SetNode DAG (``proof.kb_index`` is
    empty), so the dumper writes NO ``kb_index/`` folder and the
    ``proof_summary.json`` ``kb_index`` list is empty — even under
    ``store_lattice=True``. The solution / dead enterings are still
    dumped.

    TODO(P1.7a): the per-SetNode ``kb_index/layer_NN/kb_<i>/`` dump (the
    DAG audit view) is unreachable through ``solve`` by design. If a
    bench wants that view it must build the DAG separately; this test
    pins the new (no-DAG) behaviour rather than the removed contract.
    """
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "solve-store"
    dumper = LatticeDumper(out_dir=dump_dir)
    _solve(kb, max_set_size=3, dumper=dumper)

    # No kb_index/ DAG folder.
    assert not (dump_dir / "kb_index").exists()
    # …but the enterings were still dumped.
    assert (dump_dir / "enterings").is_dir()
    with (dump_dir / "proof_summary.json").open() as fp:
        summary = json.load(fp)
    assert summary["kb_index"] == []


def test_dumper_no_proof_summary_without_store_lattice(tmp_path: Path):
    """Without ``store_lattice`` ``solve`` attaches no proof, so
    ``proof_summary.json`` is not written (the dumper's ``proof_summary``
    hook is skipped when ``verdict.proof is None``). The per-commitment
    ``enterings/`` folders are still produced from the live ``entering``
    callbacks."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "solve-no-store"
    dumper = LatticeDumper(out_dir=dump_dir)
    solve(kb, stop_after=None, max_set_size=3, dumper=dumper)

    assert not (dump_dir / "proof_summary.json").exists()
    assert not (dump_dir / "kb_index").exists()
    # enterings still captured.
    assert (dump_dir / "enterings").is_dir()


# ── out_dir=None → no on-disk artefacts ───────────────────


def test_dumper_no_op_when_out_dir_is_none(tmp_path: Path):
    """``out_dir=None`` — every hook is invoked but no files are written.
    Under ``store_lattice`` the verdict still carries its proof."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dumper = LatticeDumper(out_dir=None)
    verdict, _ = _solve(kb, max_set_size=3, dumper=dumper)
    assert verdict.proof is not None
    # No on-disk effects to assert beyond "no exception raised".
    assert not any(tmp_path.iterdir())


# ── 00_timeline.jsonl records lifecycle events ────────────


def test_dumper_timeline_records_events(tmp_path: Path):
    """The timeline carries one record per hook invocation (root_initial,
    layer_start, entering, layer_end, proof_summary, summary). Each
    ``entering`` record carries its ``outcome``, and at least one is a
    ``solution``."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    dump_dir = tmp_path / "timeline-check"
    dumper = LatticeDumper(out_dir=dump_dir)
    _solve(kb, max_set_size=3, dumper=dumper)

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
    # Every entering carries an outcome; at least one is a solution.
    enterings = [e for e in events if e["event"] == "entering"]
    assert all("outcome" in e for e in enterings)
    assert any(e["outcome"] == "solution" for e in enterings)
    # Sequence numbers are monotonic.
    seqs = [e["seq"] for e in events]
    assert seqs == sorted(seqs)
