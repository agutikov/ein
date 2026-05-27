"""Monotonic set-search engine.

Converges to the first met solution if any. Single
KnowledgeBase instance (root); commitment sets entered via the
common :func:`ein_bot.inference.commitment.try_commitment_set`
primitive; unconditional facts merged into root; terminates on
``is_solved(root.kb)`` becoming True.

SOLVE mode only. See [P1.5b README] for the design rationale
and the equivalence claim against the lattice engine.
"""
from ein_bot.inference.monotonic.solver import monotonic_solve

__all__ = ["monotonic_solve"]
