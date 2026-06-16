# M2b — Presentation + paper

**Estimate:** TBD.
**Status:** **placeholder** — slotted between M2 and M3, or after
M3 depending on which results read more cleanly.
**Depends on:** [M2](../m2_nl_to_ir/README.md) — needs the NL → IR
pipeline working on at least one benchmark suite to have a
concrete result to write up.
**Blocks:** nothing on the M1-M3 critical path; this is the
"share what we built" milestone.

## Goal

Write up the Ein system for an external audience. Frame the
contributions, compare to the systems catalogued in
[`docs/lib/`](../../docs/lib/), measure on shared benchmarks,
and document the directions worth pursuing next.

The intended outputs:

- A **paper** — workshop / venue TBD; could be a technical
  report first.
- A **talk / presentation** — slide deck plus a worked demo
  (Zebra + an NL-fed puzzle).
- A **website / landing page** — README + a one-pager that
  surfaces the engine + ideas to readers who don't want to clone
  the repo.

## Tracks

### Track A — comparison to prior art

Use the existing catalogue under [`docs/lib/`](../../docs/lib/)
(12 thematic files, 2026-05-18 catalogue) as the structured
comparison axis:

- **CSP / SAT / SMT solvers** ([`02`](../../docs/lib/02-solvers-csp-sat-smt.md))
  — Z3, MiniZinc, OR-tools. How does Ein's graph + rules
  approach compare on a Zebra-class puzzle: encoding effort,
  solve time, trace quality, explainability?
- **Theorem provers / proof assistants** ([`03`](../../docs/lib/03-theorem-proving-formal-methods.md))
  — Coq, Lean, Isabelle. Ein is rule-driven but not a
  proof assistant; characterise the difference.
- **Graphs + rewrite systems** ([`06`](../../docs/lib/06-graphs-rewrite-systems.md))
  — Catlab.jl, GP 2, etc. Graph-rewriting is the closest
  formal relative; ein's twist is the typed-hypergraph + the
  three-layer structure + the human-style trace.
- **Cognitive architectures / neuro-symbolic** ([`09`](../../docs/lib/09-cognitive-architectures-neurosymbolic.md))
  — ATMS, SOAR, ACT-R, and modern neuro-symbolic stacks. Ein's
  hypothesis loop is ATMS-adjacent; the human-trace target is
  a deliberate departure.
- **LLM + reasoning benchmarks** ([`12`](../../docs/lib/12-llm-and-reasoning-benchmarks.md))
  — ProofWriter, FOLIO, BIG-Bench, etc. M2's NL → IR pipeline
  is the bridge into these suites.

For each: a short "what they do / what we do / where we differ"
section, plus head-to-head numbers where applicable.

### Track B — benchmarks

Concrete measurement tracks:

- **Zebra family** — the canonical puzzle + the variants under
  [`docs/ideas/09-puzzles-beyond-zebra.md`](../../docs/ideas/09-puzzles-beyond-zebra.md).
  Ein's home turf; should solve cleanly.
- **NL → IR end-to-end** — feed problem text via M2's pipeline,
  solve with M1's engine, report accuracy + ambiguity + time.
- **Existing reasoning benchmarks** ([`docs/lib/12`](../../docs/lib/12-llm-and-reasoning-benchmarks.md))
  — at least one external benchmark to anchor against published
  numbers.

Mode coverage: report both `solve` and `prove` results (cf.
[P1.9 modes idea](../m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/README.md)),
plus the `gaps` / `contradictions` task classes
([Idea 03](../../docs/ideas/03-three-task-classes.md)).

### Track C — results write-up

For each benchmark / comparison:

- Methodology (what was measured, on what hardware, with what
  config).
- Headline numbers.
- Where Ein wins (likely: trace quality, encoding ergonomics,
  human-style explanations) and where it loses (likely: raw solve
  time vs CSP solvers on adversarial puzzles).
- Threat-to-validity discussion.

### Track D — growth directions

Codify the open followups as "future work" for the paper:

- The three self-modification rungs
  ([F2](../followups/f2_self_modifying_language.md) /
  [F5](../followups/f5_rules_as_data.md) /
  [F6](../followups/f6_modify_own_harness.md)).
- Categorical formulation ([F1](../followups/f1_categorical_formulation.md))
  and the FOL / relation-algebra angle ([F1b](../followups/f1b_logical_formulation.md)).
- Rule induction ([F7](../followups/f7_rule_induction.md)).
- The umbrella [`docs/ideas/10-generic-self-modification.md`](../../docs/ideas/10-generic-self-modification.md).

Each gets a paragraph in §Future Work that's honest about the
known unknowns rather than aspirational.

## Out of scope

- Implementation work driven *by* the paper. The paper writes
  about what's already shipped; if a benchmark needs a missing
  feature, that's a milestone-back-pressure signal, not paper work.
- Multiple papers / venues; M2b is one write-up cycle.

## Acceptance (sketch)

- A draft technical report covering tracks A-D, with at least
  one head-to-head comparison and one shared-benchmark number.
- A 20-30 minute talk deck.
- A README / landing-page update that links the report.

## Open questions

- **Venue** — workshop (where?), conference, arXiv tech report
  only?
- **Author list** — solo or with collaborators?
- **Open-source story** — what's the license posture / community
  story by the time we're writing? (Repo already LICENSE'd; the
  paper choice may shape outreach.)
- **Reproducibility artefact** — what does a reader need to
  reproduce numbers? Probably a tagged repo commit + a benchmark
  driver script.

## Cross-links

- [M1 — core graph reasoning](../m1_core_graph_reasoning/README.md),
  [M2 — NL → IR](../m2_nl_to_ir/README.md),
  [M3 — SMT integration](../m3_smt_integration/README.md) — the
  results the write-up reports on.
- [docs/lib/](../../docs/lib/) — the structured comparison
  axis (12 thematic files + knowledge graph).
- [docs/ideas/](../../docs/ideas/) — the user's framing of the
  contributions; the paper's "what we built" leans on this.
- [docs/ideas/09 — puzzles beyond Zebra](../../docs/ideas/09-puzzles-beyond-zebra.md)
  — the benchmark menu for Track B.
- [docs/lib/12 — LLM and reasoning benchmarks](../../docs/lib/12-llm-and-reasoning-benchmarks.md)
  — external benchmarks Track B anchors against.
