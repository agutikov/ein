"""Acceptance — one engine, three consistent answers.

The engine is a single sound process: :func:`ein.inference.monotonic.solve`
explores the commitment lattice once, records every solution node + every
refuted commitment, and reads the verdict from the count ``k`` via
:func:`verdict_of` — ``k = 0 / 1 / >1`` → ``Contradiction / Solution /
Ambiguity``. These are three *answers* to one problem (unsat / unique /
under-determined), selected by the **input**, never by which function is
called. With ``store_lattice=True`` the verdict carries a sound
:class:`LatticeProof` whose ``solutions`` (the gaps view) and
``dead_commitments`` / ``unsat_core`` (the contradictions view) are just
readings of that one result.

This suite pins the matrix and, crucially, that the three readings AGREE for a
given input:

    fixture                       verdict        gaps view (proof.solutions)   contradictions view
    ----------------------------  -------------  ----------------------------  --------------------
    zebra2.ein          (k = 1)   Solution       exactly 1 distinct model      empty unsat core
    zebra2-minus-15.ein (k >= 2)  Ambiguity      >= 2 distinct models          empty unsat core
    ein-bugs/zebra2-bad (k = 0)   Contradiction  0 models                      core names culprit

History (2026-06-16): three *separate* entries used to disagree on the same
input — ``ein search`` (sound ``solve``) said Solution while ``ein lattice
--gaps`` said Ambiguity (a fabricated 2nd model) and ``ein lattice
--contradictions`` said Contradiction (an 81-fact "core" for a satisfiable
puzzle), because the verdict tracked the *function called*, not the input. The
buggy ``gaps_solve`` / ``contradictions_solve`` entries were removed; this test
is the regression guard that the surviving single engine stays consistent.

Lives in ``acceptance/`` (the slow, serial, end-to-end gate — minutes under
PyPy), outside the fast pytest ``testpaths``.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.inference.canon import state_hash
from ein.inference.monotonic import solve
from ein.inference.verdict import Ambiguity, Contradiction, Solution
from ein.ir import parse
from ein.kb import KnowledgeBase

REPO = Path(__file__).resolve().parents[2]
EXAMPLES = REPO / "examples"

# (id, fixture relpath, expected verdict type, solve stop_after, culprit fact in core)
#
# stop_after=None exhausts the lattice — needed to *certify* the unique (k=1)
# and unsat (k=0) cases. For the ambiguous fixture, 2 distinct nodes already
# prove Ambiguity, so we stop at 2 (full exhaustion of an ambiguous puzzle is
# needlessly slow).
CASES = [
    ("zebra2", "zebra2.ein", Solution, None, None),
    ("minus15", "zebra2-minus-15.ein", Ambiguity, 2, None),
    ("bad", "ein-bugs/zebra2-bad.ein", Contradiction, None,
     ("color-loc", ("Green", "House-1"))),
]
_IDS = [c[0] for c in CASES]


def _solve(rel: str, stop_after):
    kb = KnowledgeBase.from_ir(parse((EXAMPLES / rel).read_text()))
    return solve(kb, stop_after=stop_after, max_set_size=5, store_lattice=True)


def _distinct_models(proof) -> int:
    """Number of distinct (state_hash-deduped) complete models the proof
    witnesses — the gaps view."""
    return len({state_hash(r.kb) for r in proof.solutions})


# ── Per-fixture: verdict + both proof views ───────────────────────


@pytest.mark.parametrize("cid,rel,expected,stop,core_fact", CASES, ids=_IDS)
def test_one_engine_classifies(cid, rel, expected, stop, core_fact):
    """``solve`` reads the verdict from ``k`` — the right answer per input."""
    verdict, stats = _solve(rel, stop)
    assert isinstance(verdict, expected), (
        f"solve({rel}) → {type(verdict).__name__}, expected {expected.__name__} "
        f"(k={stats.solution_nodes})"
    )
    # Certifying unique (k=1) / unsat (k=0) requires an exhausted search;
    # Ambiguity proven by stop_after=2 needn't exhaust.
    if expected is not Ambiguity:
        assert stats.exhausted, "certifying unique/unsat requires an exhausted search"


@pytest.mark.parametrize("cid,rel,expected,stop,core_fact", CASES, ids=_IDS)
def test_gaps_view_matches_verdict(cid, rel, expected, stop, core_fact):
    """The gaps view (``proof.solutions``) must agree with the verdict: a
    Solution witnesses exactly one distinct model, an Ambiguity two-or-more, a
    Contradiction none."""
    verdict, _ = _solve(rel, stop)
    n = _distinct_models(verdict.proof)
    if expected is Solution:
        assert n == 1, f"unique puzzle must witness exactly 1 model, got {n}"
    elif expected is Ambiguity:
        assert n >= 2, f"ambiguous puzzle must witness >= 2 models, got {n}"
    else:
        assert n == 0, f"unsat puzzle must witness 0 models, got {n}"


@pytest.mark.parametrize("cid,rel,expected,stop,core_fact", CASES, ids=_IDS)
def test_contradictions_view_matches_verdict(cid, rel, expected, stop, core_fact):
    """The contradictions view (``unsat_core``) must be non-empty and name the
    culprit *iff* the puzzle is truly unsat — and empty for any satisfiable
    puzzle (no contradiction to report)."""
    verdict, _ = _solve(rel, stop)
    core = getattr(verdict, "unsat_core", frozenset())
    if expected is Contradiction:
        assert core, "an UNSAT puzzle must yield a non-empty unsat core"
        ids = {(f.relation_name, f.args) for f in core}
        assert core_fact in ids, f"unsat core must name the culprit {core_fact}"
    else:
        assert not core, (
            f"a SATISFIABLE puzzle must report NO contradiction, but the unsat "
            f"core has {len(core)} fact(s)"
        )
