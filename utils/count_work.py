#!/usr/bin/env python3
"""What ein.py *did* — the Python half of the T1a.6.1.3 work comparison.

    python3 utils/count_work.py                       # all four cells
    python3 utils/count_work.py -k 'zebra2 fast'
    python3 utils/count_work.py --json out.json

Prints the **same field names** as `ein_core::counters::Counters`, so the two
tables diff directly:

    cargo run --release --features counters -p ein-infer --example counter_cost

Wall-clock says how fast; these say *whether the two engines are doing the same
thing*. A 26x speed-up with `unify_slot` matching to three significant figures
is a port result. The same speed-up with a third of the unifications is a
different search wearing the same verdict — which the parity harness would not
necessarily catch, because T1 compares counters the engine chose to publish and
these are not among them.

Counted by **wrapping**, not by cProfile. `ncalls` cannot separate a nested
`_bind_args` from a top-level one and cannot see how many facts `_candidates`
*returned* — and the candidate count is the one the index work is judged by. The
wrappers cost time (a Python-level call around a 6 M-call function is not free)
and nothing here reports time, so that is the right trade. Nothing in
`ein.py/src/` is edited: the wrappers are installed at runtime, on the module
attributes, and removed when the process exits with them.

The mapping, ein.rs field ← ein.py site:

| field | ein.py |
|---|---|
| `unify_slot` | `match._bind_arg` |
| `candidates` | `match._bind_args` — one call per candidate actually tried |
| `unify` | *no row*: ein.rs splits per-premise (`unify`) from per-argument |
|   | (`unify_slot`) and recurses into the first for a nested pattern, while |
|   | `_bind_arg` handles nesting inline. `unify_slot` is the comparable pair |
| `walk` | `match._run_steps` |
| `plan_run` | `match.run` + `match.run_guarded` |
| `binding_key` | `saturator.Saturator._binding_key` |
| `plan_compile` | `compile.compile_rule` |
| `fact_insert` | `store.KnowledgeBase.add_and_index_fact` |
| `guard_query` | `world.World.absent` — one call per guard, which is what the |
|   | Rust counter bumps inside `first_failing`'s loop |
| `watch_stamp` | `saturator.Saturator._watch_stamp` |
| `watch_stamp_rel` | extent sizes it returned, summed |
| `fork` | `store.KnowledgeBase.fork` |
| `prov_node` | `provenance.walk_premises` — *calls*, not nodes; it is a |
|   | generator, and a solve never drains one |

Two Python-only rows are printed after the table because they have no ein.rs
counterpart *by design*:

- **`candidates_offered`** — the facts `_candidates` returned, summed. ein.py
  materialises the whole bucket as a tuple; ein.rs iterates `facts_with` lazily
  and a join abandoned after three candidates costs three. So `offered` is
  systematically larger than `candidates`, and the gap is the work laziness
  saves rather than a divergence.
- **`candidates_calls`** — how many buckets that took, i.e. the average bucket
  size, which is what the participation index is judged by.
"""
from __future__ import annotations

import argparse
import functools
import json
import os
import sys
import time
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
sys.path.insert(0, os.environ.get("EIN_SRC", str(REPO / "ein.py" / "src")))

# Declaration order of `ein_core::counters::Counters`, so the two tables line up
# row for row without anyone having to sort them.
FIELDS = [
    "unify_slot", "unify", "candidates", "walk", "plan_run", "binding_key",
    "plan_compile", "fact_insert", "guard_query", "watch_stamp",
    "watch_stamp_rel", "fork", "prov_node",
]

COUNTS: Counter = Counter()
# How many module attributes each wrapper replaced — printed with `-v`, because
# "the wrapper was installed" is the assumption every number here rests on.
BOUND: dict[str, int] = {}


def _count_fn(mod, name: str, key: str, *, sum_len: str | None = None):
    """Wrap a module-level function **everywhere it is bound**.

    Replacing the attribute on its defining module is enough for a caller in
    that same module — the lookup is a module-global at call time — and *not*
    enough for `from .compile import compile_rule`, which binds the function
    object into the importer's namespace at import time. That distinction cost
    this script a wrong number: `plan_compile` read 180 against ein.rs's
    17 430, an apparent 97× gap that was really `engine.py` holding its own
    reference and never seeing the wrapper. Both implementations recompile.

    So: rebind every `ein.*` module attribute that *is* the original object.
    """
    orig = getattr(mod, name)

    @functools.wraps(orig)
    def wrapper(*a, **kw):
        COUNTS[key] += 1
        out = orig(*a, **kw)
        if sum_len is not None:
            COUNTS[sum_len] += len(out)
        return out

    patched = 0
    for m in list(sys.modules.values()):
        if m is None or not getattr(m, "__name__", "").startswith("ein"):
            continue
        for attr, val in list(vars(m).items()):
            if val is orig:
                setattr(m, attr, wrapper)
                patched += 1
    if not patched:                    # the defining module at least
        setattr(mod, name, wrapper)
    BOUND[f"{mod.__name__}.{name}"] = patched


def _count_method(cls, name: str, key: str, *, sum_len: str | None = None):
    orig = getattr(cls, name)

    @functools.wraps(orig)
    def wrapper(self, *a, **kw):
        COUNTS[key] += 1
        out = orig(self, *a, **kw)
        if sum_len is not None:
            COUNTS[sum_len] += len(out)
        return out

    setattr(cls, name, wrapper)


def install() -> None:
    # Import the whole engine first: the sweep in `_count_fn` can only rebind a
    # reference that already exists, so a module imported *later* would bind the
    # unwrapped original and quietly drop its calls from the count.
    import ein.cli.solve  # noqa: F401  — pulls in the engine and the renderers
    from ein.inference import compile as compile_mod
    from ein.inference import match as match_mod
    from ein.inference import world as world_mod
    from ein.inference.monotonic import solver  # noqa: F401
    from ein.inference.saturator import Saturator
    from ein.kb import provenance
    from ein.kb.store import KnowledgeBase

    _count_fn(match_mod, "_bind_arg", "unify_slot")
    # One `_bind_args` per candidate *tried* — ein.rs's `try_candidate`. The
    # bucket `_candidates` returned is counted separately: ein.py builds all of
    # it, ein.rs consumes as much of it as the join needs.
    _count_fn(match_mod, "_bind_args", "candidates")
    _count_fn(match_mod, "_candidates", "candidates_calls",
              sum_len="candidates_offered")
    _count_fn(match_mod, "_run_steps", "walk")
    _count_fn(match_mod, "run", "plan_run")
    _count_fn(match_mod, "run_guarded", "plan_run")
    _count_fn(compile_mod, "compile_rule", "plan_compile")
    _count_fn(provenance, "walk_premises", "prov_node")
    _count_method(Saturator, "_binding_key", "binding_key")
    _count_method(Saturator, "_watch_stamp", "watch_stamp", sum_len="watch_stamp_rel")
    _count_method(KnowledgeBase, "add_and_index_fact", "fact_insert")
    _count_method(KnowledgeBase, "fork", "fork")
    _count_method(world_mod.World, "absent", "guard_query")


def cell(path: Path, stop_after: int | None) -> tuple[Counter, int, str]:
    from ein.inference.monotonic import solve
    from ein.kb.store import KnowledgeBase

    kb = KnowledgeBase.from_file(str(path))
    # Reset after the load: the row is the solve, and the frontend's counts are
    # another phase's number.
    COUNTS.clear()
    # `solve` returns `(verdict, stats)`.
    verdict, stats = solve(kb, stop_after=stop_after)
    counts = Counter(COUNTS)
    return counts, getattr(stats, "enterings_total", 0), type(verdict).__name__


def group(n: int) -> str:
    s = str(n)
    return " ".join(
        [s[max(0, i - 3):i] for i in range(len(s) % 3 or 3, len(s) + 1, 3)]
    ) if len(s) > 3 else s


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR")
    ap.add_argument("-v", "--verbose", action="store_true",
                    help="report how many bindings each wrapper replaced")
    ap.add_argument("--json", type=Path, default=None)
    args = ap.parse_args()

    install()
    if args.verbose:
        for site, n in BOUND.items():
            print(f"  wrapped {site}: {n} binding(s)", file=sys.stderr)
    cells: list[tuple[str, Counter, int, str, float]] = []
    for name in ["zebra2", "zebra"]:
        for label, stop_after in [("fast", 1), ("exhaustive", None)]:
            title = f"{name} {label}"
            if args.only and args.only not in title:
                continue
            print(f"… {title}", file=sys.stderr, flush=True)
            t0 = time.perf_counter()
            counts, enterings, verdict = cell(
                REPO / "examples" / f"{name}.ein", stop_after)
            cells.append((title, counts, enterings, verdict,
                          time.perf_counter() - t0))

    width = 20
    print(f"\n{'counter':<18}" + "".join(f"{c[0]:>{width}}" for c in cells))
    print("─" * (18 + width * len(cells)))
    for f in FIELDS:
        cellv = ["—" if f == "unify" else group(c[1][f]) for c in cells]
        print(f"{f:<18}" + "".join(f"{v:>{width}}" for v in cellv))
    print(f"{'(enterings)':<18}" + "".join(f"{group(c[2]):>{width}}" for c in cells))
    print(f"{'(verdict)':<18}" + "".join(f"{c[3]:>{width}}" for c in cells))
    print(f"{'(wrapped wall s)':<18}" + "".join(f"{c[4]:>{width}.1f}" for c in cells))
    # Not a Counters field, but the one number that says whether the index is
    # doing its job: buckets returned per call.
    for extra in ("candidates_offered", "candidates_calls"):
        print(f"{extra:<18}" + "".join(
            f"{group(c[1][extra]):>{width}}" for c in cells))

    if args.json:
        args.json.write_text(json.dumps({
            "impl": f"{sys.implementation.name} {sys.version.split()[0]}",
            "cells": [{"cell": t, "enterings": e, "verdict": v,
                       "wrapped_wall_s": round(w, 2),
                       **{f: c[f] for f in FIELDS if f != "unify"},
                       "candidates_offered": c["candidates_offered"],
                       "candidates_calls": c["candidates_calls"]}
                      for t, c, e, v, w in cells],
        }, indent=2) + "\n", encoding="utf-8")
        print(f"\nartifact: {args.json}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
