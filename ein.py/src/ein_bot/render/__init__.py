"""ein-bot rendering package — P1.6.

Home for the DOT / markdown renderers that turn the engine's work into
a readable *story* (the P1.6 goal). Stage map:

- :mod:`ein_bot.render.palette` — shared relation-colour palette
  (S1.6.0); reused by the per-form IR renderer
  (:mod:`ein_bot.ir.to_dot`) and the unified KB renderer
  (:mod:`ein_bot.kb.render`).
- ``rules`` / ``constraints`` — static-artefact DOT (S1.6.1).
- ``slice`` — per-hypothesis derivation cones + KB snapshots (S1.6.2).
- ``lattice_dag`` — commitment-lattice / proof-DAG DOT (S1.6.3).

See ``plans/m1_core_graph_reasoning/p1.6_rendering_and_trace/``.
"""
from __future__ import annotations

from .palette import PALETTE, hash_color

__all__ = ["PALETTE", "hash_color"]
