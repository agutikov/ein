"""Commitment-lattice / proof-DAG renderer tests — S1.6.3.

Covers `ein.render.lattice_dag.render_lattice`:

- a 3-commitment proof (1 solution + 2 dead) renders with the right
  verdict colours; dead nodes carry an unsat-core tooltip + a no-good
  back-edge;
- a `state_hash`-collapsed `SetNode` renders as one multilabel node;
- rendering from `lattice_snapshot` is invariant under
  `lattice_order_seed` (the S1.5b.31 shuffle harness);
- both views render valid DOT (Graphviz, skipped if absent);

plus the `trace-view=dag` derivation-DAG de-stub (T1.6.3.3).
"""
from __future__ import annotations

import shutil
import subprocess
import tempfile
from dataclasses import replace
from pathlib import Path

import pytest

from ein.cli import main
from ein.inference.config import SolverConfig
from ein.inference.monotonic import (
    contradictions_solve,
    gaps_solve,
    lattice_snapshot,
)
from ein.inference.monotonic.lattice import (
    DeadCommitment,
    LatticeProof,
    SolutionRecord,
)
from ein.ir import parse, render_trace
from ein.kb import KnowledgeBase
from ein.kb.entities import Fact
from ein.render import render_lattice

REPO = Path(__file__).resolve().parents[3]
BRANCHING = REPO / "examples" / "branching"
LATTICE = REPO / "examples" / "lattice"

_HAVE_DOT = shutil.which("dot") is not None

SOL_GREEN = 'fillcolor="#e8f6e8"'
DEAD_RED = 'fillcolor="#fdeaea"'


def _kb(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _parses(dot: str) -> bool:
    return subprocess.run(["dot", "-Tcanon"], input=dot,
                          capture_output=True, text=True).returncode == 0


# ── synthetic 3-commitment proof: 1 solution + 2 dead ──────────────

def _synthetic_proof() -> LatticeProof:
    empty = KnowledgeBase.from_ir(parse(' '))
    sol = SolutionRecord(commitment=(("p", ("a",)),), kb=empty, firings=(), layer=1)
    d1 = DeadCommitment(
        commitment=(("p", ("b",)),),
        unsat_core=frozenset({Fact("p", ("b",)), Fact("q", ("b",))}),
        learned_clause=frozenset({("p", ("b",))}),
        layer=1, kind="dead-post",
    )
    d2 = DeadCommitment(
        commitment=(("p", ("c",)),),
        unsat_core=frozenset({Fact("p", ("c",))}),
        learned_clause=frozenset({("p", ("c",))}),
        layer=1, kind="dead-pre",
    )
    return LatticeProof(solutions=(sol,), dead_commitments=(d1, d2))


def test_verdict_colours_one_solution_two_dead():
    dot = render_lattice(_synthetic_proof(), view="solution")
    assert dot.count(SOL_GREEN) == 1      # one solution → green
    assert dot.count(DEAD_RED) == 2       # two dead → red
    # commitment labels present
    assert "p(a)" in dot and "p(b)" in dot and "p(c)" in dot


def test_dead_nodes_have_unsat_tooltip_and_nogood_backedge():
    dot = render_lattice(_synthetic_proof(), view="solution")
    assert "tooltip=" in dot
    assert "unsat-core" in dot
    # the lifted no-good is a dashed back-edge
    backedges = [ln for ln in dot.splitlines()
                 if "no-good" in ln and "style=dashed" in ln]
    assert len(backedges) == 2            # one per dead commitment


def test_verdict_raises_on_bad_view():
    with pytest.raises(ValueError, match="lattice view"):
        render_lattice(_synthetic_proof(), view="bogus")


def test_render_lattice_unwraps_a_verdict():
    """A Verdict carrying a proof is accepted directly."""
    kb = _kb(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3, store_lattice=True)
    dot = render_lattice(verdict, view="solution")  # not verdict.proof
    assert "digraph lattice" in dot


# ── real gaps proof: frontier matches solutions ────────────────────

def test_gaps_full_solution_frontier_matches_proof():
    kb = _kb(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3, store_lattice=True)
    dot = render_lattice(verdict.proof, view="full")
    sol_nodes = [
        n for n in verdict.proof.kb_index.values() if n.verdict == "solution"
    ]
    # The render shows one green node per satisfying COMMITMENT — the gaps
    # kb_index is per-commitment (no state merge; the documented contract).
    # S1.7.24: branching/04's `co-located` is symmetric, and with the
    # kernel no longer canonicalising symmetric pairs both orientations
    # are explored, so the two models surface as four satisfying
    # commitments → four green nodes.
    assert dot.count(SOL_GREEN) == len(sol_nodes)
    # `proof.solutions` reports distinct MODELS (state_hash-deduped): two.
    assert len(verdict.proof.solutions) == 2
    # …and the green frontier covers exactly those two distinct states.
    assert len({n.state_hash for n in sol_nodes}) == 2


# ── state_hash collapse → one multilabel node ──────────────────────

def test_state_hash_collision_renders_multilabel_node():
    kb = _kb(LATTICE / "03_state_hash_collision.ein")
    verdict, _ = contradictions_solve(kb, max_set_size=3, store_lattice=True)
    dot = render_lattice(verdict.proof, view="full")
    assert "≡ same state" in dot          # ≥2 commitments collapsed


# ── shuffle invariance via the snapshot ────────────────────────────

@pytest.mark.parametrize("fixture", [BRANCHING / "04_two_levels.ein",
                                     LATTICE / "03_state_hash_collision.ein"])
def test_snapshot_render_is_shuffle_invariant(fixture: Path):
    def _snap_dot(seed: int) -> str:
        kb = _kb(fixture)
        cfg = replace(kb.config or SolverConfig(), lattice_order_seed=seed)
        verdict, _ = gaps_solve(kb, max_set_size=3, store_lattice=True, config=cfg)
        return render_lattice(lattice_snapshot(verdict, kb), view="full")
    assert _snap_dot(0) == _snap_dot(3) == _snap_dot(9)


# ── DOT validity ───────────────────────────────────────────────────

@pytest.mark.skipif(not _HAVE_DOT, reason="graphviz `dot` not installed")
def test_lattice_dot_parses():
    assert _parses(render_lattice(_synthetic_proof(), view="solution"))
    kb = _kb(LATTICE / "02_genuine_3set_death.ein")
    verdict, _ = contradictions_solve(kb, max_set_size=3, store_lattice=True)
    assert _parses(render_lattice(verdict.proof, view="full"))
    assert _parses(render_lattice(verdict.proof, view="solution"))


# ── trace-view=dag derivation DAG (T1.6.3.3 de-stub) ───────────────

_TRACE = """
(trace
  (step s1 :rule from-condition :using (c10) :derives (lives-in Norwegian House-1))
  (step s2 :rule exclusivity :using (s1) :derives (not (lives-in Norwegian House-2))))
"""


def test_trace_view_dag_is_derivation_graph():
    (form,) = parse(_TRACE)
    dag = render_trace(form, view="dag")
    # nodes are the derived facts, not step names
    assert "lives-in Norwegian House-1" in dag
    assert 'label="exclusivity"' in dag           # rule labels the edge
    # s2 chains off s1's derived fact (the explanation graph)
    assert '"(lives-in Norwegian House-1)" -> ' in dag


def test_trace_view_dag_differs_from_per_step():
    (form,) = parse(_TRACE)
    assert render_trace(form, view="dag") != render_trace(form, view="per-step")
    # legacy letters still work
    assert render_trace(form, view="c") == render_trace(form, view="dag")
    assert render_trace(form, view="a") == render_trace(form, view="per-step")


def test_trace_view_invalid_raises():
    (form,) = parse("(trace)")
    with pytest.raises(ValueError, match="trace view"):
        render_trace(form, view="nope")


def test_cli_render_lattice(capsys: pytest.CaptureFixture[str]):
    """`render lattice` runs a solve and emits the lattice DOT."""
    rc = main(["render", "lattice", str(BRANCHING / "04_two_levels.ein"),
               "--max-set-size", "3"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "digraph lattice" in out
    assert SOL_GREEN in out                # two_levels has solutions


def test_cli_render_lattice_contradictions(capsys: pytest.CaptureFixture[str]):
    rc = main(["render", "lattice", str(LATTICE / "02_genuine_3set_death.ein"),
               "--mode", "contradictions"])
    assert rc == 0
    assert "no-good" in capsys.readouterr().out


def test_cli_trace_view_dag(capsys: pytest.CaptureFixture[str]):
    with tempfile.NamedTemporaryFile("w", suffix=".ein", delete=False) as fh:
        fh.write(_TRACE)
        path = fh.name
    rc = main(["ir", "dot", path, "--trace-view", "dag"])
    assert rc == 0
    out = capsys.readouterr().out
    assert "digraph trace" in out
    assert "exclusivity" in out
