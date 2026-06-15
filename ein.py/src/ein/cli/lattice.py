#!/usr/bin/env python3
"""Run the unified set-search engine's lattice entries on a .ein file.

Both :func:`gaps_solve` and :func:`contradictions_solve` live
in :mod:`ein.inference.monotonic` alongside
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
from dataclasses import replace
from pathlib import Path

from ein.inference.config import SolverConfig
from ein.inference.monotonic import (
    LatticeDumper,
    ProgressDumper,
    contradictions_solve,
    gaps_solve,
)
from ein.ir import parse
from ein.kb.store import KnowledgeBase


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
        help="if set, write a per-set audit dump to this directory "
             "(LatticeDumper; or, under --verbose, the ProgressDumper "
             "timeline + per-layer snapshots).",
    )
    ap.add_argument(
        "--max-time", type=float, default=None,
        help="abort after N wall-clock seconds",
    )
    ap.add_argument(
        "--max-enterings", type=int, default=None,
        help="abort after N try_commitment_set calls",
    )
    ap.add_argument(
        "--lattice-sanity-check", action="store_true",
        help="S1.5b.27 release regression: for every alive "
             "size-k>=2 commitment, verify every (k-1)-subset "
             "parent path saturates to the same kb. Off by "
             "default — costs k+1 saturations per checked "
             "commitment.",
    )
    ap.add_argument(
        "--lattice-order", choices=("lex", "score-sum"),
        default=None,
        help="within-layer candidate ordering (S1.5b.26). "
             "'lex' (default) is canonical-tuple sort — "
             "deterministic regression baseline. 'score-sum' "
             "uses S1.5a.7 hypgen scoring summed per-set; "
             "informed only when --hypgen-scoring is set to "
             "an informed mode like 'popularity'.",
    )
    # ── Hypothesis-order shuffle (S1.5b.31) ──
    ap.add_argument(
        "--shuffle", action="store_true",
        help="shuffle the within-layer commitment-candidate (hypothesis) "
             "order before exploring — a per-layer random.Random(--seed)."
             "shuffle applied on top of --lattice-order. The verdict is "
             "shuffle-invariant (same answer, different traversal); use it "
             "to probe order-dependence of the search.",
    )
    ap.add_argument(
        "--seed", type=int, default=0,
        help="RNG seed for --shuffle (default 0); ignored without --shuffle.",
    )
    # ── Verbose progress (parity with `ein search`) ──
    ap.add_argument(
        "--verbose", "-v", action="store_true",
        help="stream per-layer + per-entering progress to stderr "
             "(mirrors `ein search --verbose`).",
    )
    ap.add_argument(
        "--progress-every", type=int, default=10,
        help="under --verbose, log every N-th entering (default 10; "
             "solution nodes + layer boundaries always log).",
    )
    return ap


def main(argv: list[str] | None = None) -> int:
    args = _build_argparser().parse_args(argv)
    text = args.puzzle.read_text()
    kb = KnowledgeBase.from_ir(parse(text))

    config = kb.config or SolverConfig()
    replacements: dict = {}
    if args.lattice_sanity_check:
        replacements["lattice_sanity_check"] = True
    if args.lattice_order is not None:
        replacements["lattice_order"] = args.lattice_order
    if args.shuffle:
        # A non-None lattice_order_seed makes the solver apply a per-layer
        # random.Random(seed).shuffle to the candidate order (after
        # --lattice-order); verdict-invariant (S1.5b.31).
        replacements["lattice_order_seed"] = args.seed
    if replacements:
        config = replace(config, **replacements)

    # --verbose streams progress via ProgressDumper (the canonical live
    # emitter, shared with `ein search` / the acceptance runner); with
    # --dump-states it also writes the filesystem timeline. Without
    # --verbose, --dump-states keeps the LatticeDumper per-set audit dump.
    if args.verbose:
        dumper = ProgressDumper(
            progress_every=args.progress_every,
            out_dir=args.dump_states,
        )
    elif args.dump_states is not None:
        dumper = LatticeDumper(out_dir=args.dump_states)
    else:
        dumper = None

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

    # Per-entry verdict-shape printing — gaps enumerates
    # branches, contradictions prints unsat_core size +
    # dead count. ``verdict.proof`` is non-None for both
    # lattice entries (S1.5b.22).
    proof = getattr(verdict, "proof", None)
    if args.gaps:
        branches = getattr(verdict, "branches", ())
        print(f"branches          {len(branches)}")
        if proof is not None:
            print(f"solutions         {len(proof.solutions)}")
    else:
        # contradictions
        unsat_core = getattr(verdict, "unsat_core", frozenset())
        print(f"unsat_core_size   {len(unsat_core)}")
        if proof is not None:
            print(f"dead_commitments  {len(proof.dead_commitments)}")
    print()

    print("stats")
    print(f"  wall             {elapsed * 1000:.1f} ms")
    if proof is not None:
        s = proof.stats
        print(f"  enterings        {s.enterings_total} "
              f"(alive={s.enterings_alive} "
              f"dead_pre={s.enterings_dead_pre} "
              f"dead_post={s.enterings_dead_post})")
        print(f"  layers_explored  {s.layers_explored}")
        print(f"  saturate_count   {s.saturate_count}")
        print(f"  facts_merged     {s.facts_merged}")
        print(f"  forced_positives {s.forced_positives}")
        print(f"  nogoods          emitted={s.nogoods_emitted} "
              f"subsumed={s.nogoods_subsumed}")
        if args.store_lattice:
            print(f"  kb_index         {len(proof.kb_index)} nodes")
            print(f"  state_hash_merges {s.state_hash_merges}")
        if proof.alive_at_end:
            print(f"  alive_at_end     {len(proof.alive_at_end)} "
                  "(depth cap hit)")
    if dumper is not None and args.dump_states is not None:
        print(f"dump              {args.dump_states}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
