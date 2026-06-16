# F1 — Categorical formulation

**Theme owner:** the user (research interest).
**Trigger:** M1 stabilises with a fixed rule set + IR — *then*
the post-hoc categorical reading becomes possible.

## What this is

Per [`docs/ideas/07-categorical-formulation.md`](../../docs/ideas/07-categorical-formulation.md):
the triangle rule is *literally* categorical composition. Once we
notice that, the natural question is whether the whole engine has
a clean CT presentation — and what (if anything) the formal
viewpoint *operationally* buys.

Three plausible readings (idea 07 §The mapping):

- **A — graph as free category**: objects = entities, morphisms =
  relations + paths.
- **B — schema view**: objects = types, morphisms = typed relations;
  each puzzle is a functor into `FinSet`.
- **C — sketch / commuting diagrams**: a puzzle is the *search for
  a functor* satisfying commuting-diagram constraints.

## Why deferred

The CT view is pragmatically *a meta-language*, not a runtime
substrate (idea 07 §Pragmatic note). M1's engine doesn't need it.
Writing it down now would either be premature speculation (no
implementation to formalise) or duplicative with what the engine
already does.

The trigger for promotion: when M1 ships and the rule set + IR
are stable, the CT formulation becomes a *concrete* design-time
sanity check — does every rule correspond to a clean DPO rewrite?
Are the three layers (ontology / fact / reasoning) really a
pushout? Etc.

## What promotion would look like

A new milestone `m_followups_categorical/` with phases:

- **PCF.1** — pin a CT reading (A vs B vs C); resolves
  [Q12](../open_questions.md#q12--ct-reading).
- **PCF.2** — formalise the rule set as DPO rewrites
  (idea 07 §Where category theory becomes genuinely heavy).
- **PCF.3** — investigate whether automatic detection of rule-set
  equivalence (functorial isomorphism) buys anything (idea 07
  §Open questions Q2).
- **PCF.4** — Catlab.jl prototype as a *cross-check oracle* —
  not a runtime; just a "the engine's rule set translates to
  this CT sketch" verifier.

## Prior art / connections

- [Idea 07](../../docs/ideas/07-categorical-formulation.md) — the
  question and the three readings.
- [`docs/lib/05-category-theory.md`](../../docs/lib/05-category-theory.md) — CT primitives, DPO/SPO, Catlab.jl.
- [`docs/lib/06-graphs-rewrite-systems.md`](../../docs/lib/06-graphs-rewrite-systems.md) — DPO/SPO graph rewriting.
