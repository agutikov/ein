"""Contradiction detector — S1.4.1 / P1.4.

Scans the KB for ``(X, (not X))`` pairs in the same layer, emitting
:class:`Contradiction` records the hypothesis loop (P1.5) and trace
renderer (P1.6) consume.

A pair encodes a **branch failure** under M1's append-only KB
model: there's no way to retract ``X`` once asserted, so the only
resolution is for whatever caused the conflict (typically a
hypothesis fork) to be unwound. P1.4 ships the *detector*; P1.5
will ship the *unwinder*.

The detector is a pure scan over the existing fact set — no new
entity kinds, no indexes, no incremental machinery. The
Saturator's append-only ``_index_fact`` already keeps
``_facts_by_relation`` current; this module just reads it.

Same-layer vs cross-layer:

- A REASONING-layer ``(not X)`` derived from ``type-exclusivity``
  is the *expected* output of saturation. If the matching
  REASONING-layer positive ``X`` is also present (e.g. asserted
  speculatively by a hypothesis), the pair is a branch failure.
- A cross-layer pair (FACT-layer X + REASONING-layer ``(not X)``)
  is **not** a contradiction in this design — layer separation is
  the engine's way of marking "different epistemic statuses".
  Both layers being co-present is by construction (the matcher
  consults them jointly), and a FACT-layer positive shouldn't
  cause a derived negative to flag it as broken. P1.5 will
  introduce branch-scoped layers; the cross-source case becomes
  meaningful there.
"""
from __future__ import annotations

from collections.abc import Iterator
from dataclasses import dataclass

from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

# ── Contradiction record ──────────────────────────────────────────


@dataclass(frozen=True)
class Contradiction:
    """A ``(X, (not X))`` pair found in the same layer.

    Both facts carry their own :class:`~ein_bot.kb.provenance.Provenance`
    — the trace renderer (P1.6) reads them to explain *why* each
    fact was asserted, and the hypothesis loop (P1.5) reads
    ``positive.provenance`` to decide which branch produced the
    conflict.

    Identity is by the pair plus the layer; same pair across two
    layers (impossible in the same KB, since :meth:`KnowledgeBase.add_fact`
    dedupes globally) would compare as distinct. The dataclass is
    frozen so it can live in sets if a caller wants to dedupe a
    multi-pass scan.
    """
    positive: Fact         # the X fact
    negative: Fact         # the wrapping (not X) fact;
                           # negative.args[0] is the inner positive Fact
    layer: Layer           # the shared layer (REASONING / FACT / ONTOLOGY)


# ── Detector ──────────────────────────────────────────────────────


class ContradictionDetector:
    """Scanner over ``kb.facts`` looking for ``(X, (not X))`` pairs.

    Construct once per KB; call :meth:`detect` whenever a pair
    snapshot is needed (typically: after a saturation cycle, before
    a hypothesis-loop branch decision). The detector keeps no
    state between calls — repeated scans are independent.

    Complexity: O(|`not`-headed facts|) per scan. On Zebra
    (~120 negatives after saturation), each iteration does one
    O(deg(rel)) lookup in :attr:`KnowledgeBase._facts_by_relation`
    — bounded by the number of facts with that relation name.
    Sub-millisecond on Zebra-scale.
    """

    def __init__(self, kb: KnowledgeBase) -> None:
        self.kb = kb

    # ── Public API ────────────────────────────────────────────────

    def detect(self) -> tuple[Contradiction, ...]:
        """All ``(X, (not X))`` same-layer pairs across the KB."""
        return tuple(self._iter())

    def detect_layer(self, layer: Layer) -> tuple[Contradiction, ...]:
        """Pairs scoped to a single layer."""
        return tuple(c for c in self._iter() if c.layer is layer)

    def has_contradiction(self) -> bool:
        """Short-circuit yes/no — stops on the first pair found.

        Faster than ``bool(detect())`` when the KB has many
        negative facts and only the existence-question matters
        (e.g. P1.5 deciding whether to retract a branch).
        """
        for _ in self._iter():
            return True
        return False

    # ── Iterator (single source of truth) ─────────────────────────

    def _iter(self) -> Iterator[Contradiction]:
        """Yield each contradiction once. Used by all three public
        methods to keep the algorithm in one place.
        """
        not_facts = self.kb._facts_by_relation.get("not", ())
        for negative in not_facts:
            if not negative.args:
                continue  # malformed; skip defensively
            inner = negative.args[0]
            if not isinstance(inner, Fact):
                # Q40 / R9 widening: the inner of a `(not …)` is
                # expected to be a Fact. If it's a string/int,
                # there's no positive proposition to match against
                # — the engine's loader / matcher should never
                # produce this shape, but we tolerate it.
                continue
            positive = self.kb._fact_by_id(inner.relation_name, inner.args)
            if positive is None:
                continue
            if positive.layer is not negative.layer:
                # Cross-layer pair — by design, NOT a contradiction.
                # See module docstring.
                continue
            yield Contradiction(
                positive=positive,
                negative=negative,
                layer=positive.layer,
            )


__all__ = ["Contradiction", "ContradictionDetector"]
