"""Inference engine — pattern matcher, predicate registry, saturation.

P1.3 (S1.3.1 .. S1.3.3). Module layout per Q39 (resolved 2026-05-20):
flat ``src/ein_bot/inference/`` — no nested ``rules/inference/``
until a second non-engine file appears.

- :mod:`predicates` — built-in predicate registry (``eq`` + ``neq``).
- :mod:`compile`    — per-(rule, activator) pattern compiler.
- :mod:`match`      — runtime matcher executing compiled JoinPlans.
- :mod:`firing`     — Firing record + ``:assert`` substitution.
- :mod:`why`        — ``{?var}`` ``:why`` template substitution.
- :mod:`engine`     — driver: compile_all, step, saturate (S1.3.3).
"""
from __future__ import annotations

from . import predicates

__all__ = ["predicates"]
