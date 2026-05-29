"""MonotonicDumper tests — S1.5b.7 T1.5b.7.3.

Covers:

1. **Files produced.** A run with ``dumper=MonotonicDumper(tmp_path)``
   leaves ``00_root_initial.ein``, ``00_timeline.jsonl``,
   ``layers/layer_NN_pre.ein`` + ``layer_NN_post.ein``, and
   ``summary.json``.
2. **Timeline event order.** The JSONL records the lifecycle as
   ``root_initial → layer_start → entering* → layer_end → ... →
   summary`` with monotonic ``seq``.
3. **Entering records carry nogood info.** A fixture with a known
   layer-1 dead-post entering shows up in the timeline with
   ``kind="dead-post"`` and ``nogood_emitted=True``.
4. **Backbone parity.** Running with ``dumper=None`` produces the
   same verdict + stats as a default run — the hooks have no
   semantic effect.
"""
from __future__ import annotations

import json
from pathlib import Path

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.monotonic import monotonic_solve
from ein_bot.inference.monotonic.state_dump import MonotonicDumper
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# Same `(paint Blue ?)` kill fixture as test_monotonic_cdcl —
# guarantees at least one layer-1 dead-post entering when the
# CDCL path is exercised under `_NO_LOOKAHEAD`.
SINGLETON_FIXTURE = """
(rules
  (rule forbid-paint-Blue ()
    :match  (paint Blue ?y)
    :assert (false)
    :why    "Blue can't paint anything"
    :priority 100))
(ontology
  (relation paint T T)
  ; declared but never asserted — keeps the goal unreachable so
  ; the fork-side is_solved check doesn't short-circuit the run
  ; before layer_end + the dead-post entering's nogood event fire.
  (relation never T)
  (is-a Thing T)
  (is-a Red Thing) (is-a Blue Thing))
(facts)
(query :mode solve
       :goal  (never ?x)
       :hypothesis-relations paint)
"""

_NO_LOOKAHEAD = SolverConfig(
    enable_pre_branch_lookahead=False,
    enable_back_prop_unconditional=False,
)


# ── 1) Files produced ─────────────────────────────────────────────


def test_run_produces_expected_files(tmp_path: Path) -> None:
    kb = _kb(SINGLETON_FIXTURE)
    dumper = MonotonicDumper(out_dir=tmp_path)
    monotonic_solve(
        kb, max_set_size=2, config=_NO_LOOKAHEAD, dumper=dumper,
    )

    assert (tmp_path / "00_root_initial.ein").is_file()
    assert (tmp_path / "00_timeline.jsonl").is_file()
    assert (tmp_path / "summary.json").is_file()
    layers = tmp_path / "layers"
    assert (layers / "layer_01_pre.ein").is_file()
    assert (layers / "layer_01_post.ein").is_file()

    # summary.json carries the verdict + stats fields.
    summary = json.loads((tmp_path / "summary.json").read_text())
    assert summary["verdict"] in ("Solution", "Ambiguity", "Contradiction")
    assert "elapsed_seconds" in summary
    assert "stats" in summary
    assert "nogoods_emitted" in summary["stats"]


# ── 2) Timeline event order ───────────────────────────────────────


def _read_timeline(p: Path) -> list[dict]:
    return [
        json.loads(line)
        for line in p.read_text().splitlines() if line.strip()
    ]


def test_timeline_event_order(tmp_path: Path) -> None:
    kb = _kb(SINGLETON_FIXTURE)
    dumper = MonotonicDumper(out_dir=tmp_path)
    monotonic_solve(
        kb, max_set_size=2, config=_NO_LOOKAHEAD, dumper=dumper,
    )

    events = _read_timeline(tmp_path / "00_timeline.jsonl")
    assert events, "timeline must not be empty"

    # Monotonic seq, monotonic ts_ms (ts may tie on fast events).
    seqs = [e["seq"] for e in events]
    assert seqs == sorted(seqs)
    assert seqs == list(range(len(events)))
    assert all(e["ts_ms"] >= 0 for e in events)

    # First event is root_initial; last is summary.
    assert events[0]["event"] == "root_initial"
    assert events[-1]["event"] == "summary"

    # Between them, every layer_start precedes its layer_end and
    # every entering's layer matches an enclosing layer_start.
    kinds = [e["event"] for e in events]
    assert "layer_start" in kinds
    # Verify ordering: layer_start(1) → ... → layer_end(1).
    start_idx = kinds.index("layer_start")
    end_idx = kinds.index("layer_end")
    assert start_idx < end_idx
    enterings_in_layer_1 = [
        e for e in events[start_idx + 1:end_idx]
        if e["event"] == "entering"
    ]
    assert enterings_in_layer_1, (
        "fixture is supposed to produce layer-1 enterings"
    )
    assert all(e["layer"] == 1 for e in enterings_in_layer_1)


# ── 3) Entering records carry nogood info ─────────────────────────


def test_entering_records_include_nogood_info(tmp_path: Path) -> None:
    kb = _kb(SINGLETON_FIXTURE)
    dumper = MonotonicDumper(out_dir=tmp_path)
    monotonic_solve(
        kb, max_set_size=2, config=_NO_LOOKAHEAD, dumper=dumper,
    )
    events = _read_timeline(tmp_path / "00_timeline.jsonl")

    enterings = [e for e in events if e["event"] == "entering"]
    dead_post = [e for e in enterings if e["kind"] == "dead-post"]
    assert dead_post, "fixture should produce a dead-post entering"
    assert any(e["nogood_emitted"] is True for e in dead_post)

    # The dead entering's commitment is the (paint Blue Red) singleton.
    dying = dead_post[0]
    assert dying["commitment"] == [
        {"relation": "paint", "args": ["Blue", "Red"]}
    ]
    assert dying["facts_merged"] == 0


# ── 4) Backbone parity (dumper=None vs dumper=set) ────────────────


_TRIVIAL_FIXTURE = """
(rules
  (rule sym-r ()
    :match (r ?x ?y) :assert (r ?y ?x)
    :why "symmetric r" :priority 100))
(ontology
  (type T)
  (relation r T T)
  (instance a T) (instance b T))
(facts (r a b :source "(1)"))
(query :mode solve :goal (r b ?x))
"""


def test_backbone_parity_with_and_without_dumper(tmp_path: Path) -> None:
    kb_no_dumper = _kb(_TRIVIAL_FIXTURE)
    v_none, s_none = monotonic_solve(kb_no_dumper, max_set_size=1)

    kb_with_dumper = _kb(_TRIVIAL_FIXTURE)
    dumper = MonotonicDumper(out_dir=tmp_path)
    v_dump, s_dump = monotonic_solve(
        kb_with_dumper, max_set_size=1, dumper=dumper,
    )

    assert type(v_none) is type(v_dump)
    assert s_none == s_dump


def test_dumper_out_dir_none_writes_no_files(
    tmp_path: Path,
) -> None:
    """``MonotonicDumper(out_dir=None)`` is a pure hook target —
    the lifecycle methods all fire but no filesystem writes
    happen. Used by `bench_monotonic --verbose` (no `--dump-states`)
    via the ``_VerboseDumper`` subclass.
    """
    kb = _kb(SINGLETON_FIXTURE)
    dumper = MonotonicDumper(out_dir=None)
    # Hooks fire — sanity-call all six manually to exercise the
    # None-guards in the writer methods. We use the kb itself
    # as a stand-in for the per-hook kb argument.
    monotonic_solve(
        kb, max_set_size=2, config=_NO_LOOKAHEAD, dumper=dumper,
    )
    # tmp_path is unused — assert the file system stays empty.
    assert list(tmp_path.iterdir()) == []


def test_dumper_records_fork_side_early_terminate(
    tmp_path: Path,
) -> None:
    """When the engine terminates via fork-side ``is_solved``
    (S1.5b.9), the dumper emits ``entering`` + ``early_terminate``
    events but NOT ``layer_end`` (we returned mid-layer).
    ``summary`` still fires via the ``_finish`` exit hook.
    """
    from ein_bot.inference.verdict import Solution
    repo = Path(__file__).resolve().parents[4]
    text = (repo / "examples" / "branching" / "05_mini_zebra.ein").read_text()
    kb = _kb(text)
    dumper = MonotonicDumper(out_dir=tmp_path)
    verdict, _stats = monotonic_solve(
        kb, max_set_size=3, dumper=dumper,
    )
    assert isinstance(verdict, Solution)
    events = _read_timeline(tmp_path / "00_timeline.jsonl")
    kinds = [e["event"] for e in events]
    # The dump should show: root_initial → layer_start →
    # entering* → early_terminate → summary, with no layer_end.
    assert "early_terminate" in kinds
    et = next(e for e in events if e["event"] == "early_terminate")
    assert et["reason"] == "is_solved_at_fork"
    assert kinds[-1] == "summary"


def test_dumper_summary_records_contradiction_verdict(
    tmp_path: Path,
) -> None:
    """Phase-1 root contradiction reaches `_finish` and the
    summary still lands. Guards against an early `return` that
    skips the summary hook.
    """
    kb = _kb("""
    (rules
      (rule always-false ()
        :match (trigger ?x)
        :assert (false)
        :why "always" :priority 100))
    (ontology
      (type T)
      (relation trigger T)
      (instance a T))
    (facts (trigger a :source "(1)"))
    (query :mode solve :goal (trigger ?x))
    """)
    dumper = MonotonicDumper(out_dir=tmp_path)
    monotonic_solve(kb, max_set_size=1, dumper=dumper)
    summary = json.loads((tmp_path / "summary.json").read_text())
    assert summary["verdict"] == "Contradiction"
