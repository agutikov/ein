"""Measure how much of a zebra2 trace is 'required for the solution'.

Prototype for the S1.6.5 trace-pruning question: the full firing log is
huge (saturation closure + eliminations); a human walkthrough is ~20
moves. This quantifies the gap and the reduction a goal-relevant
*provenance backtrack* (+ redundancy / closure collapse) would buy.

Run under PyPy:  .venv-pypy/bin/python ein.py/demo/trace_relevance.py
"""
from __future__ import annotations

from collections import Counter
from pathlib import Path

from ein.inference.monotonic import solve
from ein.ir import parse
from ein.kb import KnowledgeBase

REPO = Path(__file__).resolve().parents[1]


def _key(f) -> tuple:
    return (f.relation_name, f.args)


def main() -> None:
    kb = KnowledgeBase.from_ir(parse((REPO / "examples" / "zebra2.ein").read_text()))
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, store_lattice=True)
    primary = min(verdict.proof.solutions,
                  key=lambda r: (len(r.commitment), r.commitment))
    firings = primary.firings
    skb = primary.kb

    total = len(firings)
    redundant = sum(1 for f in firings if f.redundant)
    negative = sum(1 for f in firings if f.derived.relation_name == "not")
    print(f"primary commitment : {primary.commitment}")
    print(f"total firings      : {total}")
    print(f"  redundant        : {redundant}")
    print(f"  negative (¬)     : {negative}")
    print(f"  positive         : {total - negative}")
    print("by rule            :")
    for rule, n in Counter(f.rule for f in firings).most_common():
        print(f"    {n:4d}  {rule}")

    # ── Goal-relevant backward slice (provenance backtrack). ──
    # Answer facts = the solved positive *-loc grid (the assignment).
    answer = [f for f in skb.facts
              if f.relation_name.endswith("-loc") and len(f.args) == 2]
    required: set[tuple] = set()
    for af in answer:
        try:
            dag = skb.derivation_dag(af)
        except Exception:
            continue
        for node in dag.nodes:
            required.add(_key(node))

    cone = [f for f in firings if _key(f.derived) in required]
    cone_no_redundant = [f for f in cone if not f.redundant]
    print(f"\nanswer (grid) facts: {len(answer)}")
    print(f"goal-cone firings  : {len(cone)}  ({100*len(cone)//max(total,1)}% of {total})")
    print(f"  minus redundant  : {len(cone_no_redundant)}")
    print("goal-cone by rule  :")
    for rule, n in Counter(f.rule for f in cone_no_redundant).most_common():
        print(f"    {n:4d}  {rule}")


if __name__ == "__main__":
    main()
