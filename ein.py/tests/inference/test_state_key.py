"""Canonical ``state_key`` model identity — P1.21 R1.

Pins the identity contract of :func:`ein.inference.canon.state_key`
(the replacement for the retired ``state_hash`` int):

- **layer-free** — model identity is the extensional fact set
  ``M = {(R, args)}``; the layer a proposition was installed at never
  splits (or merges) states;
- **discriminating** — distinct closed models (zebra2 vs
  zebra2-minus-15) get distinct keys, and nested-Fact args (Q40)
  participate recursively;
- **collision-proof** — the verdict-deciding solution-node dedup keys
  on the canonical representation itself, so even a *total* digest
  collapse (every state hashing to one bucket) cannot flip
  ``Ambiguity`` into ``Solution``;
- **deterministic** — two independent parse+saturate runs of one
  fixture produce identical keys.

Cross-references: report
``plans/m1_core_graph_reasoning/p1.21_review_response/reports/r1_state_identity.md``;
implementation ``ein.py/src/ein/inference/canon.py`` +
``monotonic/_helpers._record_node``.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.inference import canon
from ein.inference.canon import state_digest, state_key
from ein.inference.monotonic import _helpers as helpers_mod
from ein.inference.monotonic import solve
from ein.inference.saturator import Saturator
from ein.inference.verdict import Ambiguity
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
EXAMPLES = REPO / "examples"
BRANCHING = EXAMPLES / "branching"


def _saturated(path: Path) -> KnowledgeBase:
    kb = KnowledgeBase.from_ir(parse(path.read_text()))
    _ = list(Saturator(kb).saturate())
    return kb


def _kb_with(*facts: Fact) -> KnowledgeBase:
    kb = KnowledgeBase()
    for f in facts:
        kb.add_and_index_fact(f)
    return kb


# ── Layer-free identity ──────────────────────────────────────


def test_same_facts_different_layers_same_key():
    """Two KBs holding the same ``(R, args)`` propositions installed at
    different :class:`Layer`s have EQUAL state keys — layer is not part
    of fact identity anywhere in the KB (``Fact.__eq__``, ``add_fact``
    dedup), and the state key says the same (P1.21 R1 §2)."""
    kb_fact = _kb_with(
        Fact("color-loc", ("Green", "House-1"), layer=Layer.FACT),
        Fact("pet-loc", ("Zebra", "House-5"), layer=Layer.ONTOLOGY),
    )
    kb_reasoning = _kb_with(
        Fact("color-loc", ("Green", "House-1"), layer=Layer.REASONING),
        Fact("pet-loc", ("Zebra", "House-5"), layer=Layer.REASONING),
    )
    assert state_key(kb_fact) == state_key(kb_reasoning)


def test_nested_fact_arg_layer_is_also_excluded():
    """A nested-Fact arg's layer is excluded exactly like a top-level
    fact's — the canonical form is internally consistent."""
    kb_a = _kb_with(
        Fact("not", (Fact("p", ("a",), layer=Layer.FACT),)),
    )
    kb_b = _kb_with(
        Fact("not", (Fact("p", ("a",), layer=Layer.REASONING),)),
    )
    assert state_key(kb_a) == state_key(kb_b)


# ── Distinct models ⇒ distinct keys ──────────────────────────


@pytest.fixture(scope="module")
def zebra2_keys():
    """Two INDEPENDENT parse+saturate runs of zebra2 — reused by the
    determinism and distinctness tests below."""
    return (
        state_key(_saturated(EXAMPLES / "zebra2.ein")),
        state_key(_saturated(EXAMPLES / "zebra2.ein")),
    )


def test_state_key_deterministic_across_runs(zebra2_keys):
    """Two independent parse+saturate runs of one fixture produce
    identical keys (and hence identical display digests)."""
    a, b = zebra2_keys
    assert a == b
    assert state_digest(a) == state_digest(b)


def test_distinct_closures_have_distinct_keys(zebra2_keys):
    """The zebra2 and zebra2-minus-15 root closures are different
    states — their keys differ."""
    k_minus = state_key(_saturated(EXAMPLES / "zebra2-minus-15.ein"))
    assert zebra2_keys[0] != k_minus


def test_nested_fact_args_recurse_into_identity():
    """Nested-Fact args (Q40) participate in the key recursively: two
    KBs differing only inside a nested fact get distinct keys."""
    kb_a = _kb_with(Fact("not", (Fact("p", ("a",)),)))
    kb_b = _kb_with(Fact("not", (Fact("p", ("b",)),)))
    assert state_key(kb_a) != state_key(kb_b)


# ── Identity survives a total digest collapse ────────────────


class _Collide(tuple):
    """A StateKey wrapper whose hash is constant — every state lands in
    ONE dict bucket, forcing the dedup to fall back to tuple equality."""

    def __hash__(self) -> int:
        return 0


def test_forced_digest_collision_keeps_models_distinct(monkeypatch):
    """E2E: with every state key hashing to the same bucket, the
    solution-node dedup still counts branching/04's two distinct models
    — ``k == 2`` and the verdict stays :class:`Ambiguity`. Under the
    retired hash-as-identity scheme this exact scenario silently
    collapsed to a falsely-certified ``Solution`` (REVIEW_M1-01 §1)."""

    def colliding_state_key(kb):
        return _Collide(canon.state_key(kb))

    monkeypatch.setattr(helpers_mod, "state_key", colliding_state_key)

    kb = KnowledgeBase.from_ir(
        parse((BRANCHING / "04_two_levels.ein").read_text()),
    )
    verdict, stats = solve(kb, stop_after=None, max_set_size=3)
    assert isinstance(verdict, Ambiguity)
    assert stats.solution_nodes == 2
    assert len(verdict.branches) == 2
