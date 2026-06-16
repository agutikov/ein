"""Shared fact-dump helpers for the ``ein solve --print-final-*`` options.

Render a solution / final-state KB slice as canonical s-expressions — one dump
per solution branch, or the unsat-core facts when there is no model.
"""
from __future__ import annotations

from typing import Any

from ein.kb.store import KnowledgeBase


def fact_sexpr(arg: Any) -> str:
    """Render a Fact arg as an s-expression, recursing into nested
    Fact args (e.g. for ``(not (co-located Blue Green))``)."""
    from ein.kb.entities import Fact

    if isinstance(arg, Fact):
        inner = " ".join(fact_sexpr(a) for a in arg.args)
        return f"({arg.relation_name} {inner})" if inner else f"({arg.relation_name})"
    return str(arg)


def hypothesis_target_relations(kb: KnowledgeBase) -> set[str]:
    """Relations a query ``:hrules`` clause targets — the hypothesis
    commitments (e.g. zebra2's five ``*-loc``). Generic: the atoms named in
    the ``:hrules`` activators that are *declared relations*, so type/object
    atoms (``House``, ``Color``) drop out position-independently. Empty when
    the puzzle has no query/hrules."""
    if kb is None or kb.query is None:
        return set()
    from ein.ir import Atom, SForm

    def _atoms(node: Any, out: set[str]) -> set[str]:
        if isinstance(node, Atom):
            out.add(node.name)
        elif isinstance(node, SForm):
            if isinstance(node.head, Atom):
                out.add(node.head.name)
            for a in node.args:
                _atoms(a, out)
        return out

    names: set[str] = set()
    for kp in kb.query.kw_pairs:
        if kp.key.name == "hrules":
            _atoms(kp.value, names)
    return names & set(kb.relations)


def print_final_state(
    kb: KnowledgeBase, *, mode: str = "all",
    targets: set[str] | None = None,
) -> None:
    """Dump a slice of a solution kb's facts in canonical order.

    Three modes (the three ``--print-final-*`` flags):

    - ``all`` — the full REASONING layer: the propositional residue of the
      solve.
    - ``positive`` — ``all`` with the ``(not …)`` facts dropped too: the
      positive residue.
    - ``hfacts`` — only the positive facts whose relation is a query
      ``:hrules`` target (``targets``), across *every* layer so the given
      conditions count too — the hypothesis commitments (e.g. zebra2's 25
      ``*-loc``). Negatives have relation_name ``not`` ∉ targets, so they drop
      out for free.
    """
    from ein.kb.entities import Layer

    if mode == "hfacts":
        targets = targets or set()
        facts = [f for f in kb.facts if f.relation_name in targets]
        label = f"positive hypothesis-relation facts; :hrules {sorted(targets)}"
    else:
        facts = [
            f for f in kb.facts
            if f.layer == Layer.REASONING
            and not (mode == "positive" and f.relation_name == "not")
        ]
        label = (
            "REASONING layer, (not …) omitted" if mode == "positive"
            else "REASONING layer"
        )
    facts.sort(key=lambda f: (
        f.relation_name,
        tuple(fact_sexpr(a) for a in f.args),
    ))
    print(f"final-state facts ({label}; {len(facts)} facts):")
    for f in facts:
        args_str = " ".join(fact_sexpr(a) for a in f.args)
        print(f"  ({f.relation_name} {args_str})")
