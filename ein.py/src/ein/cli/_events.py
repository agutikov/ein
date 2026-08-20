"""``--events`` CLI plumbing — shared by `solve` and `saturate` (S1a.0.2).

The flag pair, the argv/config-bearing `run` event, and the `load` and
`verdict` events that bracket a run. The per-site instrumentation lives in the
engine modules; this is only the seam that turns it on.

Schema: [`docs/kernel/inference/events.md`](../../../../docs/kernel/inference/events.md).
"""
from __future__ import annotations

import sys
from dataclasses import fields as _dc_fields
from typing import Any

from .. import events


def add_arguments(p: Any) -> None:
    """Add `--events` / `--events-level` to a subcommand's parser."""
    p.add_argument("--events", default=None, metavar="FILE.jsonl",
                   help="record the engine's step-by-step event log to FILE "
                        "(one JSON object per line). Off by default and "
                        "additive: stdout, stderr and the exit code are "
                        "unchanged. Feeds the M1a conformance harness's T2 "
                        "parity tier.")
    p.add_argument("--events-level", choices=("normal", "verbose"),
                   default="normal",
                   help="(--events) `verbose` adds redundant firings and "
                        "pre-candidate hypothesis skips — ~6x the volume, and "
                        "what T2 comparisons run at, since a dropped "
                        "redundant firing is exactly what a port loses.")


def start(args: Any, *, config: Any = None) -> None:
    """Open the log, if `--events` was given, and emit the `run` event."""
    path = getattr(args, "events", None)
    if not path:
        return
    fields: dict[str, Any] = {
        "impl": "ein.py",
        "file": str(args.file),
        "argv": sys.argv[1:],
    }
    if config is not None:
        fields["config"] = {
            f.name.replace("_", "-"): getattr(config, f.name)
            for f in _dc_fields(config)
        }
    events.open_log(path, level=getattr(args, "events_level", "normal"),
                    **fields)


def load(kb: Any) -> None:
    """The `load` event: what the loader built, in registry order."""
    if not events.ON:
        return
    events.emit(
        "load",
        relations=len(kb.relations), rules=len(kb.rules),
        hrules=len(kb.hrules), macros=len(kb.macros), facts=len(kb.facts),
        relation_names=list(kb.relations), rule_names=list(kb.rules),
    )


def verdict(v: Any, stats: Any) -> None:
    """The closing `verdict` event: the answer plus every counter."""
    if not events.ON:
        return
    from ..inference.verdict import Ambiguity, Contradiction, Solution
    branches = (v.branches if isinstance(v, Ambiguity)
                else (v,) if isinstance(v, Solution) else ())
    events.emit(
        "verdict", type=type(v).__name__,
        k=stats.solution_nodes, exhausted=stats.exhausted,
        counters={f.name: getattr(stats, f.name) for f in _dc_fields(stats)},
        core=(sorted(events.fact(f) for f in v.unsat_core)
              if isinstance(v, Contradiction) else []),
        models=sorted(sorted(events.fact(f) for f in b.kb.facts)
                      for b in branches),
    )


def finish() -> None:
    events.close_log()


__all__ = ["add_arguments", "finish", "load", "start", "verdict"]
