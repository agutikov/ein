"""Solver configuration — T1.5.4.4 (S1.5.4 ship).

`SolverConfig` is the typed view over the IR `(config …)` head and
the kwarg accepted by :func:`ein_bot.inference.solver.solve`.

Resolution precedence (highest wins):

1. explicit ``solve(kb, config=…)`` kwarg;
2. ``kb.config`` parsed from the IR `(config …)` block;
3. ``SolverConfig()`` defaults (the "vanilla" engine — every M1
   pruning gate that lands without an explicit per-stage opt-out
   is enabled by default; experimental gates default off).

Each field maps 1:1 to a `:kebab-flag` in the IR. The loader at
:func:`ein_bot.kb.from_ir.load` translates `(config :foo true)`
into `SolverConfig(foo=True)` and stashes the result on
``kb.config``.

The mapping table is the source of truth for the `(config …)`
schema; adding a new flag means adding a field here and a row in
:func:`from_kw_pairs`.
"""
from __future__ import annotations

from collections.abc import Iterable
from dataclasses import dataclass, fields
from typing import Any


@dataclass(frozen=True)
class SolverConfig:
    """Solver-level knobs gating each S1.5.4-and-later pruning tier.

    Defaults follow the per-task ship-stage call recorded in
    [s1.5.4_hypgen_improvements.md](../../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.4_hypgen_improvements.md):

    - ``enable_alive_inherit`` (default **True**) — Topic D, the
      inherit-alive-set mechanism (commit ``40b8dd4`` + T1.5.4.8
      polish). Setting to ``False`` reverts to per-branch
      ``generate_hypotheses(kb)`` — the pre-``40b8dd4`` shape.
      Escape hatch for puzzles whose rule library violates the M1
      invariant (no rule-created relations / objects).
    - ``enable_pre_branch_negated`` (default **True**) — Topic B
      Tier A, the O(1) ``_negated_facts`` filter inside
      ``generate_hypotheses``. Setting to ``False`` lets every
      generated hypothesis fork even when the parent KB already
      derived its negation; useful for measuring the filter's win.
    - ``enable_pre_branch_lookahead`` (default **True** once S1.5.6
      ships; **False** until then) — Topic B Tier B, the
      ``_dies_immediately(kb, h)`` one-step rule simulator. Skipped
      cleanly by the engine when the implementation isn't present.
    - ``enable_back_prop_unconditional`` (default **True**, flipped
      from off on 2026-05-23 once T1.5.7.2 + .5 + .6 shipped) —
      Topic C from S1.5.7. The consume loop sweeps candidates,
      back-propagates ``(not h)`` for unconditional deaths into the
      parent KB, re-saturates, and re-evaluates. ``False`` reverts
      to the static pre-S1.5.7 descent — keep as an escape hatch if
      a puzzle's rule library produces nested-Fact hypotheses or
      otherwise breaks the back-prop preconditions.
    - ``enable_auto_closure`` (default **False**) — S1.5.5's
      ``infer-closure-from-functional`` rule firing during
      saturation. Off until the auto-inference is verified to not
      over-fire on rule libraries that don't expect it.
    - ``enable_eager_root_bubble`` (default **False**) — S1.5a.17.
      Flips back-prop from opportunistic write to eager
      abort-and-restart. Any unconditional bubble (positive or
      negative, at any depth) raises ``BubbleAbort``, the in-flight
      subtree is discarded, root.kb is re-saturated, and the outer
      loop continues with the next root candidate. Promoted-dead
      root children also synthesise ``(not h)`` between passes so
      the outer loop terminates by fixpoint. Default off until the
      S1.5a.16 shuffle-invariance harness ratifies the order-sensitivity
      change.
    - ``print_alive`` (default **False**) — diagnostic from
      T1.5.4.8.b. When True, every ``_explore`` entry logs the
      inherited alive-set size and the per-filter prune counts.
    """
    enable_alive_inherit:            bool = True
    enable_pre_branch_negated:       bool = True
    enable_pre_branch_lookahead:     bool = True
    enable_back_prop_unconditional:  bool = True
    enable_auto_closure:             bool = False
    enable_eager_root_bubble:        bool = False
    print_alive:                     bool = False

    @classmethod
    def from_kw_pairs(cls, kw_pairs: Iterable[Any]) -> SolverConfig:
        """Build a SolverConfig from a (parsed) `(config …)` body.

        ``kw_pairs`` is the ``SForm.args`` tuple of the
        ``config_form`` — each element is a :class:`KwPair`. Keyword
        names use kebab-case (matching the IR convention); the
        translator below maps them to snake_case field names.
        Unknown keys raise :class:`ValueError` so puzzle authors
        find typos at load time, not at silent-default time.

        Boolean values accept ``true`` / ``false`` (SYMBOL atoms),
        the strings ``"true"`` / ``"false"``, or Python ``bool``.
        Anything else raises.
        """
        known_fields = {f.name for f in fields(cls)}
        out: dict[str, Any] = {}
        for kp in kw_pairs:
            key = getattr(kp, "key", None)
            value = getattr(kp, "value", None)
            if key is None or value is None:
                raise ValueError(
                    f"(config …) body expects kw_pairs, got {kp!r}",
                )
            key_name = getattr(key, "name", str(key))
            field_name = key_name.replace("-", "_")
            if field_name not in known_fields:
                raise ValueError(
                    f"unknown config flag :{key_name} "
                    f"(expected one of: "
                    f"{', '.join(sorted(n.replace('_', '-') for n in known_fields))})",
                )
            out[field_name] = _coerce_bool(field_name, value)
        return cls(**out)


def _coerce_bool(field_name: str, value: Any) -> bool:
    """Coerce a SolverConfig field value to bool.

    Accepts: Python ``bool``; SYMBOL/Atom ``true``/``false``; the
    strings ``"true"``/``"false"``. Raises ValueError otherwise.
    """
    if isinstance(value, bool):
        return value
    raw = getattr(value, "name", value)
    if isinstance(raw, str) and raw.lower() in ("true", "false"):
        return raw.lower() == "true"
    raise ValueError(
        f"config flag :{field_name.replace('_', '-')} expects "
        f"true/false, got {value!r}",
    )


__all__ = ["SolverConfig"]
