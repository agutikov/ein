#!/usr/bin/env python3
"""S1.5b.32 T32.2/T32.3 measurement harness.

Domain-elimination saturation rule (pathway A) vs explicit hypothesis
exploration (pathway B), measured across the unified engine's three
public entries (solve / gaps_solve / contradictions_solve)
over the examples/domain_elim/*.ein fixtures.

Run::

    python3 demo/s1_5b_32_measure.py

Reproduces every number in
``docs/kernel/inference/domain_elim_vs_hypothesis.md``. Self-contained:
all cases run from this one script (no inline one-liners). Sections:

  A. Pure-saturation audit (T32.3) — does pathway A's rule fire inside
     a plain saturate(), and what derives the answer fact?
  B. Default-config table — monotonic / gaps / contradictions with the
     shipped config (pre-branch lookahead ON).
  C. Lookahead sweep — the same exhaustive lattice runs with
     enable_pre_branch_lookahead toggled, isolating the real lever.
  D. Provenance audit — in each solved monotonic KB, what mechanism
     actually produced (color-loc Blue H1)?

The three fixtures hold the hypothesis generator constant
(`:hrules (guess (color-loc Color House))`) and vary only the rule
library:

  ab        — elimination rules ON  + negative-completion ON  (A + B)
  b_only    — elimination rules OFF + negative-completion ON   (no A rule)
  b_branch  — elimination rules OFF + negative-completion OFF  (forces forks)
"""
from __future__ import annotations

import sys
from dataclasses import replace
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.monotonic import (
    contradictions_solve,
    gaps_solve,
    solve,
)
from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

EX = Path(__file__).resolve().parents[2] / "examples" / "domain_elim"
FIXTURES = ["ab", "b_only", "b_branch"]
ANSWER = ("color-loc", ["Blue", "H1"])


# ── helpers ──────────────────────────────────────────────────────


def fresh(name: str) -> KnowledgeBase:
    """A fresh KB per call — solving mutates the root in place."""
    return KnowledgeBase.from_ir(parse((EX / f"{name}.ein").read_text()))


def answer_provenance(kb) -> str:
    if kb is None:
        return "—"
    rel, args = ANSWER
    for f in kb.facts:
        if f.relation_name == rel and [str(a) for a in f.args] == args:
            p = f.provenance
            if p is None:
                return "(none)"
            if p.kind == "rule":
                return f"rule:{p.rule}"
            if p.kind == "hypothesis":
                return f"hypothesis(branch={p.branch})"
            return p.kind
    return "ABSENT"


def stats_of(verdict, fallback):
    proof = getattr(verdict, "proof", None)
    st = proof.stats if proof is not None else fallback
    sets = len(proof.kb_index) if proof is not None else None
    return st, sets


def table(headers, rows):
    w = [max(len(str(c)) for c in [headers[i]] + [r[i] for r in rows])
         for i in range(len(headers))]
    def line(cells):
        return "  ".join(str(c).ljust(w[i]) for i, c in enumerate(cells))
    print(line(headers))
    print(line(["-" * x for x in w]))
    for r in rows:
        print(line(r))


# ── A. pure-saturation audit (T32.3) ─────────────────────────────


def section_saturation_audit():
    print("\n== A. Pure-saturation audit (does pathway A fire inside saturate()?) ==\n")
    rows = []
    for name in FIXTURES:
        kb = fresh(name)
        firings = list(Saturator(kb).saturate())
        by_rule: dict[str, int] = {}
        for f in firings:
            by_rule[f.rule] = by_rule.get(f.rule, 0) + 1
        elim = sum(by_rule.get(r, 0)
                   for r in ("domain-elimination", "range-elimination"))
        neg = sum(by_rule.get(r, 0)
                  for r in ("functional-negative", "injective-negative"))
        rows.append([name, len(firings), neg, elim, answer_provenance(kb)])
    table(["fixture", "firings", "neg-compl", "elim(A)", "answer@root-saturation"],
          rows)


# ── B. default-config table ──────────────────────────────────────


def run_entry(name, entry, *, config=None, store_lattice=False):
    if entry == "monotonic":
        v, s = solve(fresh(name), stop_after=1, max_set_size=5, config=config)
    else:
        fn = gaps_solve if entry == "gaps" else contradictions_solve
        v, s = fn(fresh(name), max_set_size=5, config=config,
                  store_lattice=store_lattice)
    st, sets = stats_of(v, s)
    return {
        "verdict": type(v).__name__,
        "ent": st.enterings_total,
        "dead": st.enterings_dead_pre + st.enterings_dead_post,
        "fp": getattr(st, "forced_positives", "—"),
        "sat": st.saturate_count,
        "nogoods": getattr(st, "nogoods_emitted", "—"),
        "sets": "—" if sets is None else sets,
        "prov": answer_provenance(getattr(v, "kb", None)),
    }


def section_default_table():
    print("\n== B. Default config (shipped: pre-branch lookahead ON) ==\n")
    rows = []
    for name in FIXTURES:
        for entry in ("monotonic", "gaps", "contradictions"):
            r = run_entry(name, entry, store_lattice=(entry != "monotonic"))
            rows.append([name, entry, r["verdict"], r["ent"], r["dead"],
                         r["fp"], r["sat"], r["nogoods"], r["sets"], r["prov"]])
    table(["fixture", "engine", "verdict", "ent", "dead", "fp", "sat",
           "nogoods", "sets", "answer-prov"], rows)


# ── C. lookahead sweep (the real lever) ──────────────────────────


def section_lookahead_sweep():
    print("\n== C. pre-branch-lookahead ON vs OFF (exhaustive lattice) ==\n")
    rows = []
    for la in (True, False):
        for name in FIXTURES:
            base = fresh(name).config or SolverConfig()
            cfg = replace(base, enable_pre_branch_lookahead=la)
            r = run_entry(name, "contradictions", config=cfg, store_lattice=True)
            rows.append([f"lookahead={la}", name, r["verdict"], r["ent"],
                         r["dead"], r["fp"], r["sat"], r["nogoods"], r["sets"]])
    table(["config", "fixture", "verdict", "ent", "dead", "fp", "sat",
           "nogoods", "sets"], rows)


# ── D. provenance audit ──────────────────────────────────────────


def section_provenance():
    print("\n== D. Monotonic solved-KB: what produced (color-loc Blue H1)? ==\n")
    rows = []
    for name in FIXTURES:
        v, _ = solve(fresh(name), stop_after=1, max_set_size=5)
        rows.append([name, type(v).__name__,
                     answer_provenance(getattr(v, "kb", None))])
    table(["fixture", "verdict", "answer-provenance"], rows)


def main():
    print("S1.5b.32 — domain-elimination (A) vs hypothesis exploration (B)")
    section_saturation_audit()
    section_default_table()
    section_lookahead_sweep()
    section_provenance()


if __name__ == "__main__":
    main()
