#!/usr/bin/env python3
"""S1.7.23 T1.7.23.1 — diagnostic probe for the standing blind-path fixture.

Loads a puzzle, runs the same `emit_closed` + saturate the solver does, and
reports — at the root and after committing one hypothesis — the three signals
that decide whether a node is a *solution node* (P1.7a `solution.py`):

  - open_hypotheses(kb)   — what the blind enumerator still proposes
  - consistent(kb)        — no contradiction
  - complete(kb)          — open set empty

Used to debug why the bijection cascade does (or does not) close to a
complete model. Run under any interpreter (the fixture is small):

    .venv-pypy/bin/python ein.py/demo/probe_blind_fixture.py [puzzle] [--commit R a b]
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "ein.py" / "src"))

from ein.inference.closed import emit_closed
from ein.inference.saturator import Saturator
from ein.inference.solution import complete, consistent, open_hypotheses
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.store import KnowledgeBase

_DEFAULT = (
    Path(__file__).resolve().parents[1]
    / "examples" / "branching" / "12_typed_blind_solve.ein"
)


def _report(label: str, kb: KnowledgeBase) -> None:
    oh = sorted(open_hypotheses(kb))
    print(f"  [{label}]")
    print(f"    consistent = {consistent(kb)}   complete = {complete(kb)}")
    print(f"    open hypotheses ({len(oh)}):")
    for fid in oh:
        print(f"      {fid[0]} {fid[1]}")
    pos = sorted(
        f"{f.relation_name} {f.args}"
        for f in kb.facts
        if f.relation_name not in ("is-a", "relation", "closed")
        and f.layer != Layer.ONTOLOGY
    )
    print(f"    derived/fact non-ontology ({len(pos)}):")
    for line in pos:
        print(f"      {line}")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("puzzle", type=Path, nargs="?", default=_DEFAULT)
    ap.add_argument("--commit", nargs=3, metavar=("R", "A", "B"),
                    help="commit (R A B) as a REASONING fact, then re-saturate")
    args = ap.parse_args(argv)

    kb = KnowledgeBase.from_ir(parse(args.puzzle.read_text()))
    newly = emit_closed(kb)
    print(f"puzzle  {args.puzzle}")
    print(f"emit_closed -> {newly}")
    list(Saturator(kb).saturate(max_steps=10000))
    _report("root (saturated)", kb)

    if args.commit:
        r, a, b = args.commit
        fork = kb.fork()
        fork.add_and_index_fact(
            Fact(relation_name=r, args=(a, b), layer=Layer.REASONING)
        )
        list(Saturator(fork).saturate(max_steps=10000))
        _report(f"after commit ({r} {a} {b})", fork)
    return 0


if __name__ == "__main__":
    sys.exit(main())
