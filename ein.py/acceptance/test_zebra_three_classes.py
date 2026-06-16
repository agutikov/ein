"""P1.7a acceptance — the corrected M1 gate.

The three idea-03 task classes as **three readings of one sound search**
(``ein.inference.monotonic.solve``), exercised on the canonical
``examples/`` fixtures (no inline puzzles — the fixtures are the spec):

- :file:`examples/zebra2.ein`              → **Solution**, unique (k=1).
- :file:`examples/zebra2-minus-15.ein`     → **Ambiguity** (k>1): drops
  condition (15), leaving the colours under-determined.
- :file:`examples/ein-bugs/zebra2-bad.ein` → **Contradiction** (k=0): an
  injected ``(color-loc Green House-1)`` clashes with the spatial chain.

The hard invariant: **a SAT puzzle never yields Contradiction; an UNSAT
puzzle never yields Solution** (the soundness bug S1.7.3 found — see
``plans/m1_core_graph_reasoning/p1.7a_solution_search_refactor/``).

This suite lives **outside** ``ein.py/tests/`` (the pytest ``testpaths``) on
purpose: it is the slow (~1-2 min each under PyPy), end-to-end acceptance
gate, NOT part of the fast unit suite. ``./run_tests.sh`` runs the unit
suite first and then this folder **as a separate phase** with a live
:class:`ProgressDumper` (pass ``--fast`` to skip it). Run it alone with::

    ./run_tests.sh --acceptance-only        # just this gate, with progress
    .venv-pypy/bin/python -m pytest -s acceptance/   # equivalent, from ein.py/
"""
from __future__ import annotations

import sys
from pathlib import Path

from ein.inference.canon import state_hash
from ein.inference.monotonic import ProgressDumper, solve
from ein.inference.verdict import Ambiguity, Contradiction, Solution
from ein.ir import parse
from ein.kb import KnowledgeBase

REPO = Path(__file__).resolve().parents[2]
EXAMPLES = REPO / "examples"
ZEBRA2 = EXAMPLES / "zebra2.ein"
MINUS_15 = EXAMPLES / "zebra2-minus-15.ein"
BAD = EXAMPLES / "ein-bugs" / "zebra2-bad.ein"

_LOC_RELS = ("color-loc", "nation-loc", "drink-loc", "smoke-loc", "pet-loc")


def _solve(path: Path, label: str, **kw):
    """Run the sound search on a fixture with live progress to stderr.

    The :class:`ProgressDumper` prints layer/entering/solution-node progress
    (visible under ``pytest -s`` — which ``./run_tests.sh`` uses for the
    acceptance phase) so a multi-minute exhaustive solve isn't a silent hang.
    """
    kb = KnowledgeBase.from_ir(parse(path.read_text()))
    dumper = ProgressDumper(label=label, progress_every=10, stream=sys.stderr)
    return solve(kb, dumper=dumper, **kw)


def _has(kb: KnowledgeBase, relation: str, *args: str) -> bool:
    return kb._fact_by_id(relation, tuple(args)) is not None


def _grid_cells(kb: KnowledgeBase) -> int:
    return sum(
        1
        for rel in _LOC_RELS
        for f in kb._facts_by_relation.get(rel, ())
        if len(f.args) == 2
    )


# ── SOLVE — unique model ──────────────────────────────────────────


def test_zebra2_is_unique_solution():
    """zebra2 → exactly one solution node (consistent ∧ complete), the
    full 25/25 model, certified unique by an exhausted search."""
    verdict, stats = _solve(ZEBRA2, "zebra2 (SOLVE/unique)")

    assert isinstance(verdict, Solution), (
        f"zebra2 must be a unique Solution, got {type(verdict).__name__}"
    )
    assert stats.solution_nodes == 1, (
        f"k must be 1 (unique), got {stats.solution_nodes}"
    )
    assert stats.exhausted, "uniqueness requires an exhausted search"

    model = verdict.kb
    assert _grid_cells(model) == 25, "model must fill all 25 grid cells"
    # The canonical Wikipedia answer, read off the model.
    assert _has(model, "drink-loc", "Water", "House-1")
    assert _has(model, "pet-loc", "Zebra", "House-5")
    assert _has(model, "nation-loc", "Norwegian", "House-1")
    assert _has(model, "nation-loc", "Japanese", "House-5")


# ── GAPS — multiple models ────────────────────────────────────────


def test_minus15_is_ambiguous():
    """zebra2 minus condition (15) → ≥ 2 distinct complete models.

    Finding two distinct solution nodes is *definitive* ambiguity — no need
    to exhaust — so ``stop_after=2`` keeps it bounded.
    """
    verdict, stats = _solve(MINUS_15, "minus-15 (GAPS)", stop_after=2)

    assert isinstance(verdict, Ambiguity), (
        f"minus-15 must be Ambiguity, got {type(verdict).__name__}"
    )
    assert stats.solution_nodes >= 2, (
        f"ambiguity needs ≥2 models, got k={stats.solution_nodes}"
    )
    assert len(verdict.branches) >= 2
    # The branches must be genuinely distinct complete models.
    hashes = {state_hash(b.kb) for b in verdict.branches}
    assert len(hashes) >= 2, "the models must be distinct (different state)"
    for b in verdict.branches:
        assert _grid_cells(b.kb) == 25, "each model must be complete (25/25)"


# ── CONTRADICTIONS — no model ─────────────────────────────────────


def test_bad_is_contradiction_with_injected_fact_in_core():
    """zebra2-bad → no solution node; the unsat core names the injected
    fact. (The core is the sound source-frontier — broader than a minimal
    MUS; MUS minimisation is a future refinement.)"""
    verdict, stats = _solve(BAD, "zebra2-bad (CONTRADICTIONS)")

    assert isinstance(verdict, Contradiction), (
        f"zebra2-bad must be a Contradiction, got {type(verdict).__name__}"
    )
    assert stats.solution_nodes == 0, (
        f"k must be 0 (no model), got {stats.solution_nodes}"
    )
    assert stats.exhausted, "UNSAT requires the search to have run to dead/exhausted"

    core = {(f.relation_name, f.args) for f in verdict.unsat_core}
    assert ("color-loc", ("Green", "House-1")) in core, (
        "the injected fact must be in the unsat core"
    )
    # And the injected fact carries its provenance into the core.
    injected = next(
        f for f in verdict.unsat_core
        if (f.relation_name, f.args) == ("color-loc", ("Green", "House-1"))
    )
    assert getattr(injected.provenance, "source", None) == "injected contradiction"


# ── The hard soundness invariant ──────────────────────────────────


def test_sat_never_contradiction_unsat_never_solution():
    """The invariant S1.7.3's bug violated, both directions."""
    # SAT puzzle, fast existence check — must NOT be a Contradiction.
    v_sat, _ = _solve(ZEBRA2, "invariant: SAT↛⊥", stop_after=1)
    assert not isinstance(v_sat, Contradiction), (
        "a satisfiable puzzle must never yield Contradiction"
    )
    assert isinstance(v_sat, Solution)
    assert _grid_cells(v_sat.kb) == 25

    # UNSAT puzzle — must NOT be a Solution / Ambiguity.
    v_unsat, _ = _solve(BAD, "invariant: UNSAT↛Solution")
    assert isinstance(v_unsat, Contradiction), (
        "an unsatisfiable puzzle must never yield Solution/Ambiguity"
    )


# ── CLI answer path (S1.7a.6) — acceptance criterion #1 ───────────


def test_cli_solve_emits_answer_in_words(capsys):
    """`ein solve zebra2.ein` exits 0 and prints the canonical answer in
    English (who, projected from the model)."""
    from ein.cli import main

    rc = main(["solve", str(ZEBRA2)])
    assert rc == 0
    out = capsys.readouterr().out.lower()
    # who + what, both questions answered.
    assert "norwegian" in out and "water" in out
    assert "japanese" in out and "zebra" in out


def test_cli_solve_contradiction_reports_no_solution(capsys):
    """`ein solve` on the unsat fixture reports no solution + names the
    injected fact in the core (exit 0 — the tool classified it correctly)."""
    from ein.cli import main

    rc = main(["solve", str(BAD)])
    assert rc == 0
    out = capsys.readouterr().out.lower()
    assert "no solution" in out and "contradict" in out
    assert "injected contradiction" in out
