"""Monotonic dumper — minimal per-layer root snapshots.

Stub — implementation in S1.5b.7 (after the backbone is
running so we know what's worth dumping).
"""
from __future__ import annotations

from pathlib import Path

from ein_bot.kb.store import KnowledgeBase


class MonotonicDumper:
    """Dumps root snapshots per layer + a ``00_timeline.jsonl``
    of every entering's outcome.

    Stub — see S1.5b.7.
    """

    def __init__(self, out_dir: Path) -> None:
        self.out_dir = out_dir
        self.out_dir.mkdir(parents=True, exist_ok=True)

    def root_initial(self, kb: KnowledgeBase) -> None:
        raise NotImplementedError("S1.5b.7")

    def layer_start(self, layer: int) -> None:
        raise NotImplementedError("S1.5b.7")

    def entering(self, commitment, result) -> None:
        raise NotImplementedError("S1.5b.7")

    def layer_end(self, layer: int, kb: KnowledgeBase) -> None:
        raise NotImplementedError("S1.5b.7")

    def summary(self, verdict) -> None:
        raise NotImplementedError("S1.5b.7")
