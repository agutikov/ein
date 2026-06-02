"""The gaps/contradictions public-entry type contract (S1.7c.13).

``gaps_solve`` / ``contradictions_solve`` route through
``solver._lattice_public``, which enforces "the verdict is of the
expected type and carries a LatticeProof" with an explicit ``raise``
rather than an ``assert``. F-ENG-14: a bare ``assert`` is stripped under
``python -O``, where a contract slip would degrade to an opaque
``AttributeError`` on ``.proof``. These tests pin the raise, and that it
survives ``-O``.
"""
from __future__ import annotations

import os
import subprocess
import sys
import textwrap

import pytest

from ein_bot.inference.monotonic import solver
from ein_bot.kb.store import KnowledgeBase


def test_gaps_rejects_wrong_verdict_type(monkeypatch):
    # Force the loop to hand back a non-Ambiguity verdict; the post-amble
    # must reject it rather than return it.
    monkeypatch.setattr(solver, "_explore_layers", lambda *a, **k: (object(), None))
    with pytest.raises(TypeError):
        solver.gaps_solve(KnowledgeBase())


def test_contradictions_rejects_wrong_verdict_type(monkeypatch):
    monkeypatch.setattr(solver, "_explore_layers", lambda *a, **k: (object(), None))
    with pytest.raises(TypeError):
        solver.contradictions_solve(KnowledgeBase())


def test_contract_survives_python_O():
    """Same check under ``python -O``: were the contract a bare ``assert``
    it would be stripped and the wrong-typed verdict would slip through.
    The explicit ``raise`` in ``_lattice_public`` holds.
    """
    script = textwrap.dedent(
        """
        import ein_bot.inference.monotonic.solver as s
        from ein_bot.kb.store import KnowledgeBase
        s._explore_layers = lambda *a, **k: (object(), None)
        try:
            s.gaps_solve(KnowledgeBase())
        except TypeError:
            print("RAISED")
        else:
            raise SystemExit("gaps contract did NOT raise under -O")
        """,
    )
    env = {**os.environ, "PYTHONPATH": os.pathsep.join(p for p in sys.path if p)}
    proc = subprocess.run(
        [sys.executable, "-O", "-c", script],
        capture_output=True, text=True, env=env,
    )
    assert proc.returncode == 0, proc.stderr
    assert "RAISED" in proc.stdout, proc.stdout + proc.stderr
