"""Linearize a lattice solve into a depth-ordered story — S1.6.4 T1.6.4.0.

The engine emits an *unordered* commitment lattice; the narrative needs
a linear sequence. :func:`linearize` turns a :class:`Verdict` (+ its
:class:`LatticeProof`) into a :class:`Trace`:

- **the spine** — the firings of the primary solution commitment
  (smallest commitment first; ``()`` = the unconditional root
  saturation), each a :class:`TraceStep` carrying its premises, derived
  fact, rendered ``:why``, quoted source sentences, and an inline
  derivation-slice diagram (S1.6.2);
- **reductios** — one per refuted commitment
  (`proof.dead_commitments`, from `contradictions_solve`): "Suppose X.
  Then ⊥," closed by its lifted no-good (T1.6.4.6);
- the closing **lattice DAG** (S1.6.3) + **solution grid** (S1.6.2).

This maps the human `(d, hypothesis)` framing onto `(layer,
commitment-set)`; order need only be *recognisably equivalent* to the
walkthrough (idea-08 §acceptance).
"""
from __future__ import annotations

from dataclasses import dataclass, field

from ..inference.verdict import Solution, Verdict
from ..inference.why import render_why
from ..render.dot_util import fact_label
from ..render.lattice_dag import render_lattice
from ..render.slice import render_slice, render_solution, render_state
from .ast import TraceStep


@dataclass(frozen=True)
class Reductio:
    """A refuted hypothesis — rendered as a foldable `<details>`."""

    summary:        str          # one-line: what was assumed + that it failed
    commitment:     str          # the assumed commitment, rendered
    learned_clause: str          # the lifted no-good
    diagram:        str | None   # the contradiction slice (⊥)


@dataclass
class Trace:
    """A linearized solve, ready for markdown rendering."""

    steps:        list[TraceStep] = field(default_factory=list)
    reductios:    list[Reductio] = field(default_factory=list)
    summary:      str = ""
    commitment:   str = ""        # the primary solution's assumed hypotheses
    solved:       bool = False
    n_solutions:  int = 0
    lattice_dot:  str | None = None
    solution_dot: str | None = None
    full_kb_dot:  str | None = None


def _commitment_label(commitment) -> str:
    if not commitment:
        return "∅ (unconditional)"
    return "{" + ", ".join(fact_label(fid[0], fid[1]) for fid in commitment) + "}"


def _target_entity(derived) -> str | None:
    """The node a derived fact is 'about' — its first argument (T1.6.4.3)."""
    _rel, args = derived
    for a in args:
        if isinstance(a, str):
            return a
    return None


def _step_from_firing(n: int, firing, kb, *, diagrams: bool) -> TraceStep:
    premises = tuple((p.relation_name, p.args) for p in firing.premises)
    derived = (firing.derived.relation_name, firing.derived.args)
    bindings = {k: str(v) for k, v in firing.bindings.items()}
    rule = getattr(kb, "rules", {}).get(firing.rule) if kb is not None else None
    why = render_why(rule.why, firing.bindings) if (rule and rule.why) else ""
    sources = tuple(p.source for p in firing.premises if p.source)
    diagram = render_slice((), (firing,), kb, name=f"step{n}") if diagrams else None
    return TraceStep(
        n=n, rule=firing.rule, premises=premises, derived=derived,
        bindings=bindings, why=why, diagram=diagram,
        section=_target_entity(derived), sources=sources,
    )


def _reductio(dc, kb, *, diagrams: bool) -> Reductio:
    commitment = _commitment_label(dc.commitment)
    clause = ", ".join(sorted(fact_label(fid[0], fid[1]) for fid in dc.learned_clause))
    core_sources = sorted({f.source for f in dc.unsat_core if f.source})
    contradicts = f" — contradicts {', '.join(core_sources)}" if core_sources else ""
    diagram = (
        render_slice(dc.commitment, (), kb,
                     contradiction=(dc.unsat_core, dc.learned_clause),
                     name="reductio")
        if diagrams else None
    )
    return Reductio(
        summary=f"Assumed {commitment}{contradicts} — refuted ({dc.kind})",
        commitment=commitment, learned_clause=clause, diagram=diagram,
    )


def linearize(
    verdict: Verdict, *, diagrams: bool = True, full_kb_snapshots: bool = False,
) -> Trace:
    """Build a :class:`Trace` from a solver verdict (+ its proof)."""
    proof = getattr(verdict, "proof", None)

    # ── Monotonic Solution (no proof): the trace IS solution.trace. ──
    if isinstance(verdict, Solution) and proof is None:
        kb = verdict.kb
        steps = [_step_from_firing(i, f, kb, diagrams=diagrams)
                 for i, f in enumerate(verdict.trace, start=1)]
        return Trace(
            steps=steps, summary=f"Solved in {len(steps)} steps (unconditional).",
            commitment="∅ (unconditional)", solved=True, n_solutions=1,
            solution_dot=render_solution(kb) if diagrams else None,
            full_kb_dot=render_state(kb) if full_kb_snapshots else None,
        )

    solutions = list(proof.solutions) if proof is not None else []
    deads = list(proof.dead_commitments) if proof is not None else []

    # Primary solution = smallest commitment (∅ root sorts first).
    primary = min(solutions, key=lambda r: (len(r.commitment), r.commitment),
                  default=None)
    spine_kb = primary.kb if primary is not None else None

    steps: list[TraceStep] = []
    if primary is not None:
        steps = [_step_from_firing(i, f, spine_kb, diagrams=diagrams)
                 for i, f in enumerate(primary.firings, start=1)]

    reductios = [_reductio(dc, spine_kb, diagrams=diagrams) for dc in deads]

    solved = primary is not None
    commitment = _commitment_label(primary.commitment) if primary else "—"
    if solved:
        summary = (f"Solved in {len(steps)} steps; commitment {commitment}; "
                   f"{len(solutions)} solution(s), {len(reductios)} refuted.")
    else:
        summary = (f"No solution — {len(reductios)} commitments refuted "
                   f"({len(deads)} dead).")

    lattice_dot = None
    if diagrams and proof is not None:
        # Solution sub-DAG when there's a survivor; full lattice for unsat.
        lattice_dot = render_lattice(proof, view="solution" if solved else "full")

    return Trace(
        steps=steps, reductios=reductios, summary=summary, commitment=commitment,
        solved=solved, n_solutions=len(solutions), lattice_dot=lattice_dot,
        solution_dot=(render_solution(spine_kb)
                      if (diagrams and spine_kb is not None) else None),
        full_kb_dot=(render_state(spine_kb)
                     if (full_kb_snapshots and spine_kb is not None) else None),
    )


__all__ = ["Reductio", "Trace", "linearize"]
