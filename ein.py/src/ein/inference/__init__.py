"""Inference engine — pattern matcher, predicate registry, saturation,
lattice solve. Spans P1.3 (matcher + saturator) .. P1.5b (the set-search
lattice solver). The package is flat (S1.3) with one nested subpackage,
:mod:`ein.inference.monotonic`, added when the lattice solver landed.

- :mod:`predicates`    — built-in predicate registry (``eq`` + ``neq``).
- :mod:`compile`       — per-(rule, activator) pattern compiler.
- :mod:`match`         — runtime matcher executing compiled JoinPlans.
- :mod:`firing`        — Firing record + ``:assert`` substitution.
- :mod:`why`           — ``{?var}`` ``:why`` template substitution.
- :mod:`engine`        — compile cache + naive step / saturate.
- :mod:`saturator`     — priority-banded saturation driver (S1.3.3).
- :mod:`contradiction` — ``(X, (not X))`` same-layer pair detector (S1.4.1).
- :mod:`verdict`       — ``Solution`` / ``Ambiguity`` / ``Contradiction``
  + ``goal_bindings`` (the shared verdict surface).
- :mod:`hypgen`        — hypothesis candidate generation + scoring.
- :mod:`monotonic`     — the set-indexed lattice search; its
  :func:`ein.inference.monotonic.solve` is the single public solve entry.

The Python embedding contract over this surface (parse → load → saturate →
solve → read) is documented in `docs/api/` (see ``docs/api/inference.md``).
"""
from __future__ import annotations

from . import predicates

__all__ = ["predicates"]
