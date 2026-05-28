#!/usr/bin/env python3
"""Run the unified set-search engine's lattice entries on a .ein file.

Both :func:`gaps_solve` and :func:`contradictions_solve` live
in :mod:`ein_bot.inference.monotonic` alongside
:func:`monotonic_solve` — one unified engine, three sibling
public functions. This CLI is a convenience dispatcher for the
two non-monotonic entries; :mod:`bench_monotonic` covers the
solution-mode entry.

- ``--gaps`` → :func:`gaps_solve` — collects every satisfying
  commitment; verdict is always :class:`Ambiguity`.
- ``--contradictions`` → :func:`contradictions_solve` —
  collects every dead commitment + builds the refutation
  map; verdict is always :class:`Contradiction`.

Both entries accept the orthogonal ``--store-lattice`` flag
which opts into per-SetNode storage (state-hash dedup MERGE
under ``contradictions_solve``; SetNodes built but not
merged under ``gaps_solve`` per the GAPS contract).

**Skeleton stage — S1.5b.20.** Both entries currently raise
:class:`NotImplementedError`; this CLI exists so S1.5b.21
(gaps_solve backbone) and S1.5b.23 (contradictions_solve
backbone) can drop into a finished surface. The bench's
output shape mirrors :mod:`bench_monotonic`'s; S1.5b.21+
wires the real per-entry prints.
"""
from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

# Make `from ein_bot.…` resolve when running from a checkout.
sys.path.insert(
    0, str(Path(__file__).resolve().parents[1] / "src"),
)

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.monotonic import (
    LatticeDumper,
    contradictions_solve,
    gaps_solve,
)
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _build_argparser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(
        description=__doc__.split("\n", 1)[0],
    )
    ap.add_argument(
        "puzzle", type=Path,
        help="path to .ein puzzle file",
    )
    # Mutually-exclusive entry selection — required.
    entry = ap.add_mutually_exclusive_group(required=True)
    entry.add_argument(
        "--gaps", action="store_true",
        help="gaps_solve: enumerate every satisfying commitment "
             "(verdict: Ambiguity)",
    )
    entry.add_argument(
        "--contradictions", action="store_true",
        help="contradictions_solve: build the refutation map "
             "(verdict: Contradiction)",
    )
    # Orthogonal storage flag.
    ap.add_argument(
        "--store-lattice", action="store_true",
        help="opt-in: build per-SetNode kb_index storage. "
             "Under contradictions_solve enables state-hash "
             "dedup merge; under gaps_solve the SetNodes are "
             "built but not merged (GAPS contract).",
    )
    ap.add_argument(
        "--max-set-size", type=int, default=5,
        help="largest commitment size to enumerate (default: 5)",
    )
    ap.add_argument(
        "--dump-states", type=Path, default=None,
        help="if set, write a per-set audit dump to this "
             "directory (S1.5b.29 fills the real layout; "
             "skeleton stage is a no-op).",
    )
    ap.add_argument(
        "--max-time", type=float, default=None,
        help="abort after N wall-clock seconds",
    )
    ap.add_argument(
        "--max-enterings", type=int, default=None,
        help="abort after N try_commitment_set calls",
    )
    return ap


def main(argv: list[str] | None = None) -> int:
    args = _build_argparser().parse_args(argv)
    text = args.puzzle.read_text()
    kb = KnowledgeBase.from_ir(parse(text))

    config = kb.config or SolverConfig()
    dumper = (
        LatticeDumper(out_dir=args.dump_states)
        if args.dump_states is not None else None
    )

    entry_fn = gaps_solve if args.gaps else contradictions_solve
    entry_name = "gaps_solve" if args.gaps else "contradictions_solve"

    t0 = time.perf_counter()
    try:
        verdict, _stats = entry_fn(
            kb,
            max_set_size=args.max_set_size,
            config=config,
            store_lattice=args.store_lattice,
            dumper=dumper,
            max_time=args.max_time,
            max_enterings=args.max_enterings,
        )
    except NotImplementedError as e:
        # S1.5b.20 skeleton: both entries raise. Print cleanly,
        # exit 2 to signal "not implemented yet" rather than
        # genuine error. S1.5b.21 (gaps) and S1.5b.23 (contra)
        # land the backbones.
        print(f"** error: {e} **", file=sys.stderr)
        return 2
    elapsed = time.perf_counter() - t0

    print(f"file              {args.puzzle}")
    print(f"entry             {entry_name}")
    print(f"store_lattice     {args.store_lattice}")
    print(f"verdict           {type(verdict).__name__}")
    # Per-entry verdict-shape printing lands in S1.5b.21+
    # (gaps: enumerate branches; contradictions: print
    # unsat_core size + dead_commitments count).
    print()
    print("stats")
    print(f"  wall             {elapsed * 1000:.1f} ms")
    if dumper is not None and args.dump_states is not None:
        print(f"dump              {args.dump_states}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
