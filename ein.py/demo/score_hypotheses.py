#!/usr/bin/env python
"""Per-hypothesis scoring analysis — S1.9 (E9/E12 measurement).

For each layer-1 alive singleton, fork the *initially-saturated* root in
isolation (no other hypothesis's firings), add just that one hypothesis,
saturate, and measure the cascade:

  - fire  : total firings triggered
  - pos   : positive facts derived
  - neg   : negative `(not …)` facts derived (domain eliminations)
  - done  : ✓ if the commitment cascades to a complete∧consistent solution

Then tabulate each candidate-ordering SCORE next to the measured outcome:

  - pop   : the engine's existing pre-fork heuristic (`hypgen.score_hypothesis`,
            popularity mode). The other existing mode, "most-constrained", is
            degenerate (constant 0 → lex tiebreak), so it isn't a column.
  - LCV   : E9 least-constraining-value — prefer FEW negatives (-neg).
  - info  : E12 informativeness — prefer the largest cascade (here: pos).

Finally, report the rank of the first COMPLETING hypothesis under each
ordering — ≈ the enterings the fast `solve(stop_after=1)` would pay under that
order (the lattice explores layer-1 in that order until the first solution).
Lower rank = the scoring surfaces a completer sooner = fewer saturations.

The scores LCV/info here are computed from the *measured* (post-fork) firings:
this is a diagnostic to learn which signal predicts completers, before deciding
whether it can be estimated cheaply pre-fork.

Usage:  demo/score_hypotheses.py [PUZZLE.ein]      (default: examples/zebra2.ein)
"""
from __future__ import annotations

import argparse
import sys
from dataclasses import replace
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))

from ein.inference import primitives
from ein.inference.commitment import try_commitment_set
from ein.inference.config import SolverConfig
from ein.inference.hypgen import score_hypothesis
from ein.inference.saturator import Saturator
from ein.inference.solution import is_solution_node, open_hypotheses
from ein.kb.entities import Fact, Layer
from ein.kb.provenance import FactId, Provenance
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[2]
DEFAULT_PUZZLE = REPO / "examples" / "zebra2.ein"


def _measure(root: KnowledgeBase, fid: FactId) -> dict:
    """Fork the saturated root in isolation, add `fid`, saturate, measure."""
    result = try_commitment_set(root, (fid,))
    pos = neg = false = 0
    for firing in result.firings:
        for d in firing.derived:
            if d.relation_name == primitives.FALSE:
                false += 1
            elif d.relation_name == primitives.NOT:
                neg += 1
            else:
                pos += 1
    completes = result.kind == "alive" and is_solution_node(result.kb)
    return {
        "fid": fid, "kind": result.kind, "fire": len(result.firings),
        "pos": pos, "neg": neg, "false": false, "completes": completes,
    }


def _first_completer_rank(rows: list[dict], key) -> int | None:
    """Position (1-based) of the first completing hyp under `key` (with a
    canonical-FactId tiebreak for determinism)."""
    for i, r in enumerate(sorted(rows, key=lambda r: (key(r), r["fid"])), 1):
        if r["completes"]:
            return i
    return None


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("puzzle", nargs="?", type=Path, default=DEFAULT_PUZZLE,
                    help=f"path to .ein puzzle (default: {DEFAULT_PUZZLE})")
    args = ap.parse_args()

    kb = KnowledgeBase.from_file(str(args.puzzle))
    list(Saturator(kb).saturate())                     # initial root saturation
    # Pristine isolation: kill-cache OFF so `open_hypotheses` doesn't mutate the
    # root with cached negatives; popularity ON so the `pop` column is real.
    kb.config = replace(kb.config or SolverConfig(),
                        hypgen_scoring="popularity",
                        enable_lookahead_kill_cache=False)

    alive = sorted(open_hypotheses(kb))
    rows = []
    for fid in alive:
        m = _measure(kb, fid)
        h = Fact(relation_name=fid[0], args=fid[1], layer=Layer.REASONING,
                 provenance=Provenance.from_hypothesis(branch=0))
        m["pop"] = score_hypothesis(h, kb)
        m["lcv"] = -m["neg"]        # E9: fewer eliminations = less constraining
        m["info"] = m["pos"]        # E12: more positive deductions = more cascade
        rows.append(m)

    print(f"# {args.puzzle.name}: {len(alive)} layer-1 alive singletons "
          f"(each forked from the saturated root, tested in isolation)\n")
    hdr = (f"{'hypothesis':31} {'kind':9} {'done':4} {'fire':>5} {'pos':>4} "
           f"{'neg':>4} {'pop':>7} {'LCV':>5} {'info':>5}")
    print(hdr)
    print("-" * len(hdr))
    for r in sorted(rows, key=lambda r: (-r["fire"], r["fid"])):
        fid = r["fid"]
        name = f"{fid[0]} {' '.join(map(str, fid[1]))}"[:31]
        done = "✓" if r["completes"] else ""
        print(f"{name:31} {r['kind']:9} {done:4} {r['fire']:>5} {r['pos']:>4} "
              f"{r['neg']:>4} {r['pop']:>7.2f} {r['lcv']:>5} {r['info']:>5}")

    completers = sum(r["completes"] for r in rows)
    print(f"\n# completers (cascade to a full solution): {completers}/{len(rows)}")
    print("# rank of the FIRST completer under each ordering "
          "(≈ enterings for solve(stop_after=1); lower = better):")
    print(f"   lex (canonical — current default) : "
          f"{_first_completer_rank(rows, lambda r: r['fid'])}")
    print(f"   popularity (desc)                 : "
          f"{_first_completer_rank(rows, lambda r: -r['pop'])}")
    print(f"   LCV  = fewest negatives (E9)      : "
          f"{_first_completer_rank(rows, lambda r: r['neg'])}")
    print(f"   info = most positives  (E12)      : "
          f"{_first_completer_rank(rows, lambda r: -r['pos'])}")
    print(f"   info = most firings    (E12-raw)  : "
          f"{_first_completer_rank(rows, lambda r: -r['fire'])}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
