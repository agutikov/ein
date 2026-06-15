# Ideas

The user's own ideas, intuitions, and open questions — distinct from
the assistant's elaborations or the external systems catalogued in
`docs/index/`.

Each file records:
- the **idea** in 1–3 sentences,
- the user's own framing (preserved),
- open questions the user raised,
- pointers to the relevant index files for *context*, not answers.

These are seeds for further work, not specifications.

## Files

1. [Self-modifying constraint language with LLM ↔ harness loop](01-self-modifying-constraint-language.md)
   The LLM emits content + grammar updates; an external harness applies
   the updates, recompiles the GBNF, loops the output back into input.
   The most original idea in the set.

2. [Graph IR as formal computation substrate](02-graph-as-formal-substrate.md)
   Intuition: graphs are already formal systems, so why translate to
   SMT — maybe compute directly on the graph.

3. [Three task classes on a constraint graph](03-three-task-classes.md)
   (a) solve the complete problem, (b) find gaps in an incomplete one,
   (c) find contradictions.

4. [NLP → graph IR → solver pipeline](04-nlp-to-graph-to-solver-pipeline.md)
   Direct AST → SMT loses too much; an intermediate graph-shaped
   semantic IR is the right hinge.

5. [Zebra-puzzle graph reasoner (5-year-old Ein design)](05-zebra-puzzle-graph-reasoner.md)
   Typed constraint graph + constraints + triangle/square inference +
   hypothesis generation / testing / multilevel branching + ambiguity
   detection.

6. [Completeness of inference rules](06-inference-rules-completeness.md)
   Are triangle + square enough? If so for Zebra, what other rules
   exist and for which task classes?

7. [Categorical formulation of the puzzle](07-categorical-formulation.md)
   Triangle inference = composition. Can the whole problem be expressed
   in CT — what are objects, morphisms, categories, functors?

8. [Solver as reproducer of human deductive trace](08-human-style-deductive-trace.md)
   Goal isn't "find an answer" — it's reproducing the step-by-step
   elimination / contradiction-by-cases reasoning a human writes out.

9. [Puzzles beyond Zebra](09-puzzles-beyond-zebra.md)
   A catalogue of classical logic puzzles (Knights & Knaves, muddy
   children, hat puzzles, Tower of Hanoi, paradoxes, …) and what each
   one stresses that Zebra does not — the *capability map* the engine
   should grow into. Companion benchmarks file:
   [`docs/index/12`](../index/12-llm-and-reasoning-benchmarks.md).

10. [Generic self-modification — three rungs](10-generic-self-modification.md)
    Three rungs of a ladder: grammar (F2), rules (F5), harness (F6).
    Why the M1 kernel's shape is chosen so these followups could be
    added without rework. The unifying view of [F2](../../plans/followups/f2_self_modifying_language.md),
    [F5](../../plans/followups/f5_rules_as_data.md),
    [F6](../../plans/followups/f6_modify_own_harness.md).

## Cross-cutting observations

These are not standalone ideas but recurring framings worth keeping
visible:

- **"Graphs are everywhere"** — most of the systems discussed (CT,
  e-graphs, SAT, CSP, abstract interpretation, theorem provers, …)
  either are graphs or hide graphs internally. Picked up and extended
  in [06-graphs-rewrite-systems.md](../index/06-graphs-rewrite-systems.md).

- **Surprising convergence with current research** — the architecture
  parallels modern constrained-reasoning frameworks (Const-o-T, GCR,
  CRANE, SGR). Not a *new* idea, but it validates the older ones and
  locates them on the current research map. See
  [01-llm-constrained-generation.md](../index/01-llm-constrained-generation.md).

## How to use these files

- These are *yours*. Treat them as a project notebook.
- When working on `Ein`, the natural starting points are
  ideas 5 (current state) and 4 (next step: NLP → graph).
- The bolder long-arc ideas are 1 (self-modifying language) and 7
  (categorical formulation); they require sustained design work.
- Idea 8 (human-readable trace) is the *acceptance criterion* that
  distinguishes this project from "yet another SMT wrapper".
