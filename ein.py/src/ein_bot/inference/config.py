"""Solver configuration — T1.5.4.4 (S1.5.4 ship).

`SolverConfig` is the typed view over the IR `(config …)` head and
the kwarg accepted by :func:`ein_bot.inference.monotonic.solve`
+ :func:`ein_bot.inference.monotonic.gaps_solve` +
:func:`ein_bot.inference.monotonic.contradictions_solve`.

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
    - ``enable_lookahead_kill_cache`` (default **True**) — when the
      one-step lookahead (``enable_pre_branch_lookahead``) kills a
      candidate, cache it as a ``(not h)`` REASONING fact so subsequent
      enumerations skip ``h`` via the O(1) ``_negated_facts`` filter
      instead of re-running the lookahead's match. ``False`` re-runs the
      lookahead each time. (Renamed 2026-06-15 from
      ``enable_back_prop_unconditional``.)
    - ``hypgen_scoring`` (default ``"popularity"``, flipped from
      ``"most-constrained"`` 2026-05-25 once S1.5a.7 measurement
      ratified the popularity heuristic) — S1.5a.7. Values:
      ``"popularity"`` (Idea 1 — weighted sum of fact-popularity
      at relation + object level; per-branch via kb fact-indexes),
      ``"most-constrained"`` (the prior default — score 0,
      content-based tiebreaker dominates; kept as escape hatch
      for puzzles where popularity hurts),
      ``"branch-info"`` / ``"popularity+branch-info"`` reserved
      for a future stage (raise NotImplementedError today —
      requires post-saturation signal not cheaply available
      pre-fork). Scoring is recomputed per branch since
      ``score_hypothesis(fact, kb)`` reads ``kb._facts_by_*``.
    - ``hypgen_rel_weight`` (default 1.0) — popularity-mode
      coefficient for the relation's fact-count.
    - ``hypgen_obj_weight`` (default 1.0) — popularity-mode
      coefficient for each of the two object args' fact-counts.
    - ``print_alive`` (default **False**) — diagnostic from
      T1.5.4.8.b. When True, every ``_explore`` entry logs the
      inherited alive-set size and the per-filter prune counts.
    - ``warn_derived_naf`` (default **False**) — S1.7.4
      observability. When True, the root saturation in
      :func:`ein_bot.inference.monotonic.solver._phase1_root` emits a
      :class:`ein_bot.inference.naf_deps.DerivedNafWarning` for every
      rule whose ``(absent …)`` guard watches a rule-derived relation
      (so its soundness leans on the S1.5a.1 fire-time re-eval).
      Default **off**, not on as the stage doc first proposed: the
      suite runs under ``filterwarnings=["error"]``, and while
      ``closed`` stays hardcoded the NAF is sound regardless so the
      warning is pure advisory. Promote to default-on when
      [S1.7.7](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.7_kernel_purity_analysis.md)
      de-hardcodes closure and the map becomes load-bearing. The map
      itself is always available via
      :meth:`ein_bot.inference.engine.Engine.naf_dependency_map`.
    - ``candidate_order_seed`` (default **-1**) — T1.5a.2a.2 /
      S1.5a.16. Negative means the default S1.5a.1a content-sort
      (``_candidate_sort_key`` — deterministic, branch order
      replayable across processes). A non-negative integer applies
      ``random.Random(seed-mixed-with-content)`` to the sorted list
      in ``_candidates_for`` so the branch order is a deterministic
      permutation of the default, *different per branch point*.
      Used by ``tests/inference/test_shuffle_invariance.py`` to
      probe whether the depth-D output (alive / dead endpoints +
      unsat-core union) is path-independent. Puzzle authors won't
      normally set it.
    """
    enable_alive_inherit:            bool = True
    enable_pre_branch_negated:       bool = True
    enable_pre_branch_lookahead:     bool = True
    enable_lookahead_kill_cache:     bool = True
    # `enable_auto_closure` was a declared-but-inert placeholder for an
    # `infer-closure-from-functional` rule. P1.8 S1.8.A6 ships that rule as
    # `std.closure`, opt-in by IMPORT (no kernel↔stdlib rule-name coupling),
    # so the flag is retired.
    hypgen_scoring:                  str = "popularity"
    hypgen_rel_weight:               float = 1.0
    hypgen_obj_weight:               float = 1.0
    print_alive:                     bool = False
    warn_derived_naf:                bool = False
    candidate_order_seed:            int = -1
    # S1.5b.27 — saturation-commutativity sanity check.
    # When True, ``_explore_layers`` runs
    # :func:`ein_bot.inference.monotonic.sanity.check_commutativity`
    # for every alive size-``k>=2`` commitment, verifying every
    # ``(k-1)``-subset parent path produces the same
    # post-saturation kb. Off by default — costs ``k+1``
    # saturations per checked commitment. Release-regression
    # only.
    lattice_sanity_check:            bool = False
    # S1.5b.26 — within-layer candidate ordering. ``"lex"``
    # is canonical-tuple sort (deterministic + the shipping
    # default — preserves regression baselines).
    # ``"score-sum"`` is the per-set score from S1.5a.7's
    # :func:`ein_bot.inference.hypgen.score_hypothesis` summed
    # over the set's elements (descending; tiebreak lex).
    # The actual differentiation depends on
    # :attr:`hypgen_scoring` — under the default
    # ``"most-constrained"`` every score is 0.0 and
    # score-sum collapses to lex. Set
    # ``hypgen_scoring="popularity"`` alongside to surface
    # informed ordering. The deviation from the
    # ``s1.5b.26_lattice_scoring.md`` spec's default
    # (``"score-sum"``) is intentional: the regression
    # baselines under the monotonic loop + lattice tests were
    # recorded under lex; switching default would force
    # re-baselining for no engine-correctness gain.
    lattice_order:                   str = "lex"
    # S1.5b.31 — within-layer shuffle seed. When set, the
    # solver applies a per-layer ``random.Random(seed).shuffle``
    # to ``candidates`` AFTER :func:`order_candidates` so the
    # shuffle invariance harness can probe traversal-order
    # dependence. One ``random.Random`` is created per solve;
    # its state advances across layers, so different layers
    # get different (but deterministic-given-seed) shuffles.
    # ``None`` (default) disables — every candidate list is
    # consumed in the ``lattice_order`` order without
    # randomisation.
    lattice_order_seed:              int | None = None

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
        field_by_name = {f.name: f for f in fields(cls)}
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
            if field_name not in field_by_name:
                raise ValueError(
                    f"unknown config flag :{key_name} "
                    f"(expected one of: "
                    f"{', '.join(sorted(n.replace('_', '-') for n in field_by_name))})",
                )
            out[field_name] = _coerce(field_by_name[field_name], value)
        return cls(**out)


def _flag(field) -> str:
    """Kebab-cased IR flag name for an error message."""
    return field.name.replace("_", "-")


def _unwrap(value: Any) -> Any:
    """Strip an IR wrapper node to its Python value.

    ``Int`` / ``String`` carry ``.value``; ``Atom`` (a SYMBOL like
    ``true`` / ``lex``) carries ``.name``; a raw Python primitive — or a
    node with neither (e.g. ``Range``) — passes through unchanged.
    """
    if hasattr(value, "value"):
        return value.value
    if hasattr(value, "name"):
        return value.name
    return value


def _coerce_bool(field, value: Any) -> bool:
    if isinstance(value, bool):
        return value
    raw = _unwrap(value)
    if isinstance(raw, bool):
        return raw
    if isinstance(raw, str) and raw.lower() in ("true", "false"):
        return raw.lower() == "true"
    raise ValueError(
        f"config flag :{_flag(field)} expects true/false, got {value!r}",
    )


def _coerce_int(field, value: Any) -> int:
    raw = _unwrap(value)
    if isinstance(value, bool) or isinstance(raw, bool):
        raise ValueError(
            f"config flag :{_flag(field)} expects an integer, got bool {value!r}",
        )
    try:
        return int(raw)
    except (TypeError, ValueError) as e:
        raise ValueError(
            f"config flag :{_flag(field)} expects an integer, got {value!r}",
        ) from e


def _coerce_float(field, value: Any) -> float:
    raw = _unwrap(value)
    try:
        return float(raw)
    except (TypeError, ValueError) as e:
        raise ValueError(
            f"config flag :{_flag(field)} expects a number, got {value!r}",
        ) from e


def _coerce_str(field, value: Any) -> str:
    raw = _unwrap(value)
    if isinstance(raw, str):
        return raw
    raise ValueError(
        f"config flag :{_flag(field)} expects a string, got {value!r}",
    )


_COERCERS = {
    "bool": _coerce_bool,
    "int": _coerce_int,
    "float": _coerce_float,
    "str": _coerce_str,
}


def _coerce(field, value: Any) -> Any:
    """Type-dispatch coercer for SolverConfig fields.

    ``field.type`` is a *string* (the module uses ``from __future__ import
    annotations``), so the base type is its first ``|``-separated token —
    ``"int | None"`` coerces as ``int``. IR values arrive wrapped (``Atom``
    with ``.name``; ``Int`` / ``String`` with ``.value``) or as raw Python
    primitives; the per-type coercer unwraps them. A type outside
    ``{bool, int, float, str}`` raises with the kebab-cased flag name.
    """
    base = field.type.split("|")[0].strip()
    coercer = _COERCERS.get(base)
    if coercer is None:
        raise ValueError(
            f"config flag :{_flag(field)} has unsupported type {field.type!r}",
        )
    return coercer(field, value)


__all__ = ["SolverConfig"]
