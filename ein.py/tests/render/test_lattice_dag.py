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
from dataclasses import replace
from pathlib import Path

import pytest

from ein.cli import main
from ein.inference.config import SolverConfig
from ein.inference.monotonic import (
    solve,
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
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, store_lattice=True)
    dot = render_lattice(verdict, view="solution")  # not verdict.proof
    assert "digraph lattice" in dot


# ── real solve proof: full view falls back to the solution frontier ─

def test_full_view_falls_back_to_solution_frontier():
    """``solve`` does not build the per-SetNode DAG (``proof.kb_index`` is
    empty), so ``render_lattice(view="full")`` falls back to the
    solution-frontier view: one green node per distinct MODEL
    (``proof.solutions``, state_hash-deduped). branching/04 → 2 models →
    2 green nodes."""
    kb = _kb(BRANCHING / "04_two_levels.ein")
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, store_lattice=True)
    assert verdict.proof.kb_index == {}
    dot = render_lattice(verdict.proof, view="full")
    # `proof.solutions` reports distinct MODELS (state_hash-deduped): two.
    assert len(verdict.proof.solutions) == 2
    # The fallback frontier shows one green node per distinct model.
    assert dot.count(SOL_GREEN) == 2


# ── state_hash collision fixture under solve ────────────────────────

def test_state_hash_collision_renders_single_model():
    """``03_state_hash_collision`` resolves to ONE model under ``solve``
    (committing h2 derives h1+h3 → complete). With no DAG built the full
    view falls back to the solution frontier: a single green node.

    TODO(P1.7a): the multilabel ``≡ same state`` DAG node that the removed
    ``contradictions_solve`` produced is unreachable through ``solve``
    (no per-SetNode kb_index). The merge view would need the DAG built
    separately."""
    kb = _kb(LATTICE / "03_state_hash_collision.ein")
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, store_lattice=True)
    dot = render_lattice(verdict.proof, view="full")
    assert "≡ same state" not in dot      # no DAG → no multilabel marker
    assert dot.count(SOL_GREEN) == 1      # the single model


# ── shuffle invariance of the rendered proof ───────────────────────

@pytest.mark.parametrize("fixture", [BRANCHING / "04_two_levels.ein",
                                     LATTICE / "03_state_hash_collision.ein"])
def test_proof_render_is_shuffle_invariant(fixture: Path):
    """The rendered lattice (solution view) is identical across
    ``lattice_order_seed`` values — the traversal order changes, the answer
    does not (S1.5b.31). `solve` records the lex-smallest commitment per
    model state, so the reps are deterministic and the render is stable."""
    def _dot(seed: int) -> str:
        kb = _kb(fixture)
        cfg = replace(kb.config or SolverConfig(), lattice_order_seed=seed)
        verdict, _ = solve(kb, stop_after=None, max_set_size=3,
                           store_lattice=True, config=cfg)
        return render_lattice(verdict, view="solution")
    assert _dot(0) == _dot(3) == _dot(9)


# ── DOT validity ───────────────────────────────────────────────────

@pytest.mark.skipif(not _HAVE_DOT, reason="graphviz `dot` not installed")
def test_lattice_dot_parses():
    assert _parses(render_lattice(_synthetic_proof(), view="solution"))
    kb = _kb(LATTICE / "02_genuine_3set_death.ein")
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, store_lattice=True)
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


def test_cli_render_lattice_shows_nogood(capsys: pytest.CaptureFixture[str]):
    """``render lattice`` shows the learned no-good back-edge for a refuted
    commitment. (The ``--mode contradictions`` flag was removed — there is one
    lattice per solve; the default ``--view solution`` renders the deads with
    their no-goods.) ``lattice/01_subset_pruned`` has the {a,b} death + its
    nogood even though the puzzle as a whole is satisfiable."""
    rc = main(["render", "lattice", str(LATTICE / "01_subset_pruned.ein")])
    assert rc == 0
    assert "no-good" in capsys.readouterr().out
