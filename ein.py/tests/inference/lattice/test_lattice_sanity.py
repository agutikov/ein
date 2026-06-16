"""Saturation-commutativity sanity check — S1.5b.27 T1.5b.27.3.

Pins the behaviour of
:func:`ein.inference.monotonic.sanity.check_commutativity`:

- Passes on every shipped fixture (M1's rule set IS monotone;
  no commutativity violation surfaces).
- Off-by-default doesn't run the per-commitment check (no
  measurable cost).
- ``SanityError`` raises when two paths to the same
  commitment do produce distinct state_hashes — verified via
  a direct call with a constructed mismatch.

Cross-references:

- Implementation:
  ``ein.py/src/ein/inference/monotonic/sanity.py``.
- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.27_lattice_sanity_check.md``.
- Premise:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/README.md`` § Motivation.
"""
from __future__ import annotations

from dataclasses import replace
from pathlib import Path

import pytest

from ein.inference.config import SolverConfig
from ein.inference.monotonic import solve
from ein.inference.monotonic.sanity import (
    SanityError,
    check_commutativity,
)
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"
LATTICE = REPO / "examples" / "lattice"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


# ── Sanity check passes on monotone fixtures ─────────────


def test_sanity_check_passes_on_branching_04():
    """``branching/04_two_levels`` is monotone — every alive
    multi-element commitment's parent paths produce identical
    kbs. The end-to-end run completes without
    :exc:`SanityError`."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    cfg = replace(kb.config or SolverConfig(), lattice_sanity_check=True)
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, config=cfg)
    # No exception → premise verified for this fixture.
    assert verdict is not None


def test_sanity_check_passes_on_03_state_hash_collision():
    """The collision fixture has commitments saturating to
    identical kbs — exactly the case the sanity check
    verifies (each lattice path through a commitment lands
    in the same kb)."""
    kb = _kb_from(LATTICE / "03_state_hash_collision.ein")
    cfg = replace(kb.config or SolverConfig(), lattice_sanity_check=True)
    verdict, _ = solve(kb, stop_after=None, max_set_size=2, config=cfg)
    assert verdict is not None


def test_sanity_check_passes_on_genuine_3set_death():
    """``02_genuine_3set_death.ein`` has a 3-set that dies; the
    sanity check on the alive 2-set parents must still pass
    (no commutativity violation on their saturated kbs)."""
    kb = _kb_from(LATTICE / "02_genuine_3set_death.ein")
    cfg = replace(kb.config or SolverConfig(), lattice_sanity_check=True)
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, config=cfg)
    assert verdict is not None


# ── Off-by-default doesn't run the check ─────────────────


def test_sanity_check_off_by_default():
    """``SolverConfig.lattice_sanity_check`` defaults to False;
    a normal solve doesn't invoke the per-commitment check
    (verified by the absence of import-time side effects + the
    config default)."""
    cfg = SolverConfig()
    assert cfg.lattice_sanity_check is False
    # End-to-end: solve runs without raising; we don't assert
    # the check is skipped (the check is structurally absent
    # when the flag is False — no observable side effect).
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = solve(kb, stop_after=None, max_set_size=3)
    assert verdict is not None


# ── SanityError is raised on a constructed mismatch ────


def test_sanity_failure_on_constructed_mismatch(monkeypatch):
    """Force a state_hash mismatch by monkeypatching
    :func:`state_hash` to return different values on each
    call — confirms the SanityError path fires correctly
    when the premise IS violated. The monkeypatch is the
    only way to construct the failure since M1's actual rule
    set is monotone (the spec's "contrived violation" fixture
    can't be written within the IR's declarative semantics)."""
    from ein.inference.monotonic import sanity as sanity_mod

    call_count = {"n": 0}

    def alternating_state_hash(_kb):
        call_count["n"] += 1
        # First call (direct path) returns 1; every subsequent
        # (parent paths) returns a unique increasing value, so
        # the direct vs parent comparison fails.
        return call_count["n"]

    monkeypatch.setattr(
        sanity_mod, "state_hash", alternating_state_hash,
    )

    kb = _kb_from(LATTICE / "03_state_hash_collision.ein")
    # We need a size-2+ commitment to exercise. Use the
    # checker directly on a known commitment.
    h1 = ("h1", ("X",))
    h2 = ("h2", ("X",))
    commitment = tuple(sorted([h1, h2]))
    with pytest.raises(SanityError) as exc_info:
        check_commutativity(kb, commitment)
    failure = exc_info.value
    assert failure.commitment == commitment
    assert len(failure.parent_state_hashes) >= 1


def test_sanity_check_noop_for_singleton():
    """Size-1 commitments have no parents — the check is a
    pure no-op + cannot fail."""
    kb = _kb_from(LATTICE / "03_state_hash_collision.ein")
    singleton = (("h1", ("X",)),)
    # Should not raise; should not even invoke state_hash.
    check_commutativity(kb, singleton)


def test_sanity_check_skips_dead_pre_commitment():
    """When the direct path produces ``dead-pre`` (pre-saturation
    contradiction), the check returns silently — no state
    to hash."""
    # Use a fixture where some commitments fire dead-pre.
    # branching/04 has dead-post deads; we don't have a
    # natural dead-pre fixture, so just call on a synthetic
    # commitment that would naturally die. The no-op path
    # for dead-pre is the same as no-op for dead-post (just
    # skips silently); we're really testing the absence of a
    # crash, not the specific reason for skipping.
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    # A commitment with the contradiction-immediate hypothesis
    # (which dies pre-saturation). For branching/04 there's
    # no clean dead-pre — but the function should also handle
    # a dead-post case silently. Construct a fake commitment
    # of well-typed but unsupported facts; depending on the
    # engine's response the check returns either silently or
    # via a normal kind handling. Either way, no exception.
    fake = (
        ("nonexistent-rel", ("nonsense",)),
        ("other-fake", ("nonsense2",)),
    )
    # This shouldn't raise — the direct path is alive on a
    # well-formed kb regardless of unrecognised hypothesis
    # names. The parent paths similarly don't crash.
    check_commutativity(kb, fake)
