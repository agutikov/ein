"""``ein solve --json-summary FILE`` — the structured run summary (S1a.0.1).

One JSON object describing what a solve concluded and how much work it did:
the verdict and its model, every :class:`MonotonicStats` counter, the
root-saturation shape, and the resolved config. It exists for the M1a
conformance harness, whose first two parity tiers are exactly this file —
**T0** (the answer is the same) and **T1** (every counter the engine reports
about its own work is the same). See
[`plans/m1a_rust/design/01`](../../../../plans/m1a_rust/design/01_parity_contract.md)
§2 for the tier definitions and the counter list this schema realises.

Three properties the harness depends on:

- **Additive.** The flag writes a file; it never touches stdout, stderr or the
  exit code, so a T3 byte-comparison of the same run is unaffected and one
  invocation serves every tier.
- **Order-free.** Every set-shaped observable (the model, the unsat core, the
  goal-binding rows, the per-relation fact counts) is sorted before it is
  written, so a T0/T1 diff reports semantic differences and leaves iteration
  order to T2/T3, which is where order belongs.
- **Self-describing.** Field order is fixed by construction and `schema`
  carries a version, so a diff of two summaries is readable top-to-bottom.

The `root` block re-derives the root-saturation observables the same way
`--hyp-stats` and `--timing` derive theirs — on a fork, after the solve, so
nothing here can perturb the run it describes. That costs a second root
saturation, which is the right trade: this is a parity mode, never a benchmark
mode, and the harness never times a `--json-summary` run.
"""
from __future__ import annotations

import json
from collections import Counter
from dataclasses import fields as _dc_fields
from pathlib import Path
from typing import Any

from ._factdump import fact_sexpr

SCHEMA = "ein-summary/1"


def _facts(kb: Any) -> list[str]:
    """A KB's facts as sorted s-expressions — the model, as a set."""
    return sorted(fact_sexpr(f) for f in kb.facts)


def _binding_value(v: Any) -> Any:
    """One binding value, as JSON can carry it.

    ``goal_bindings`` returns the **stored** argument, so a slot holding an
    IR ``INT`` is a Python ``int`` and stays a JSON *number* — that is the
    shape ein.rs was taught to match. A slot holding a *nested fact* is a
    :class:`Fact`, which ``json.dumps`` cannot serialise at all: before
    2026-08-20 ``ein solve --json-summary`` **raised** on
    ``(query :goal (r1 ?x ?y))`` over ``(r1 (u0 o1) o2)`` and wrote no
    summary, where ein.rs answered. Render it the way every other fact in
    this file is rendered. Found by
    `S1a.6.6 <../../../../plans/m1a_rust/p1a.6_performance/s1a.6.6_differential_fuzzer.md>`_'s
    differential fuzzer; ``examples/ein-bugs/fact-goal-binding.ein`` is the
    fixture.
    """
    if isinstance(v, (str, int, float, bool)) or v is None:
        return v
    return fact_sexpr(v)


def _bindings(kb: Any) -> list[dict[str, Any]]:
    """`(query :goal …)` binding rows, sorted; each row's keys sorted too."""
    from ..inference.verdict import goal_bindings

    rows = [{k: _binding_value(v) for k, v in sorted(row.items())}
            for row in goal_bindings(kb)]
    return sorted(rows, key=lambda r: sorted((k, str(v)) for k, v in r.items()))


def _verdict_block(verdict: Any) -> dict[str, Any]:
    """The T0 observables: what was proved, and the model(s) that prove it."""
    from ..inference.verdict import Ambiguity, Contradiction, Solution

    block: dict[str, Any] = {}
    if isinstance(verdict, Contradiction):
        block["unsat_core"] = sorted(fact_sexpr(f) for f in verdict.unsat_core)
        block["solutions"] = []
        return block
    block["unsat_core"] = []
    branches = (verdict.branches if isinstance(verdict, Ambiguity)
                else (verdict,) if isinstance(verdict, Solution) else ())
    # Sorted by model, not left in `Ambiguity.branches` order: which of k
    # models is found first is a *traversal* fact, and `--shuffle` reorders it
    # while proving the answer unchanged. Leaving the engine's order here
    # would make T0 report a difference on exactly the runs whose point is
    # that there is none.
    block["solutions"] = sorted(
        ({"facts": _facts(b.kb), "goal_bindings": _bindings(b.kb)}
         for b in branches),
        key=lambda s: s["facts"],
    )
    return block


def _stats_block(stats: Any) -> dict[str, Any]:
    """Every `MonotonicStats` field, in declaration order (base counters
    first — `_BaseStats` leads, so the order matches `summary.json`'s)."""
    return {f.name: getattr(stats, f.name) for f in _dc_fields(stats)}


def _root_block(kb: Any) -> dict[str, Any]:
    """Root-saturation observables: how big the fixpoint is, how many plans
    it took, what the boundary did, and what hypgen would enumerate from it.

    Computed on a fork, so the caller's KB is untouched. The saturator's NAF
    counters live per-`Saturator` and are never aggregated into `stats`, so
    the root instance is the one well-defined place to read them; `naf_dropped`
    is structurally 0 since S1.21.8, and anything else means the boundary was
    rebuilt wrong.
    """
    from ..inference.closed import emit_closed
    from ..inference.engine import Engine
    from ..inference.hypgen import generate_hypotheses_with_stats
    from ..inference.saturator import Saturator

    root = kb.fork()
    emit_closed(root)
    sat = Saturator(root)
    list(sat.saturate())
    _hyps, hstats = generate_hypotheses_with_stats(root)

    engine = Engine(kb.fork())
    engine.compile_all()

    by_rel: Counter = Counter(f.relation_name for f in root.facts)
    return {
        "facts": len(root.facts),
        "facts_by_relation": {r: by_rel[r] for r in sorted(by_rel)},
        "plans": len(engine.cache),
        "saturator": {
            "naf_rounds":   sat.naf_rounds,
            "naf_admitted": sat.naf_admitted,
            "naf_retired":  sat.naf_retired,
            "naf_dropped":  sat.naf_dropped,
        },
        "hypgen": {
            "raw":     hstats.raw,
            "emitted": hstats.emitted,
            "filtered": {k: hstats.filtered[k] for k in sorted(hstats.filtered)},
            "pre_candidate": {
                k: hstats.pre_candidate[k] for k in sorted(hstats.pre_candidate)
            },
        },
    }


def _config_block(config: Any) -> dict[str, Any]:
    """The resolved `SolverConfig`, kebab-cased, in declaration order — the
    same field order `--dump-config` prints."""
    return {f.name.replace("_", "-"): getattr(config, f.name)
            for f in _dc_fields(config)}


def build(*, verdict: Any, stats: Any, kb: Any, config: Any,
          source: str) -> dict[str, Any]:
    """The summary object for a completed solve."""
    return {
        "schema": SCHEMA,
        "source": source,
        "verdict": {
            "type": type(verdict).__name__,
            "k": stats.solution_nodes,
            "exhausted": stats.exhausted,
            **_verdict_block(verdict),
        },
        "stats": _stats_block(stats),
        "root": _root_block(kb),
        "config": _config_block(config),
    }


def build_aborted(*, reason: str, stats: Any, kb: Any, config: Any,
                  source: str) -> dict[str, Any]:
    """The summary for a run cut short by `--max-time` / `--max-enterings`.

    `stats` is partial by construction and `exhausted` is False. Note that a
    `--max-time` abort is not a parity observable — *which* enterings finished
    before the clock ran out is machine-speed-dependent — so the harness's run
    matrix uses `--max-enterings`, which is.
    """
    return {
        "schema": SCHEMA,
        "source": source,
        "verdict": {
            "type": "Aborted",
            "k": stats.solution_nodes,
            "exhausted": stats.exhausted,
            "reason": reason,
            "unsat_core": [],
            "solutions": [],
        },
        "stats": _stats_block(stats),
        "root": _root_block(kb),
        "config": _config_block(config),
    }


def write(path: str | Path, summary: dict[str, Any]) -> None:
    """Write the summary as pretty JSON with a trailing newline."""
    Path(path).write_text(
        json.dumps(summary, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


__all__ = ["SCHEMA", "build", "build_aborted", "write"]
