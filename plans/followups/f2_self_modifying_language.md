# F2 — Self-modifying constraint language with LLM ↔ harness loop

**Theme owner:** the user.
**Trigger:** M2 (LLM infra + GBNF for IR) is complete and stable.

## What this is

Per [`docs/ideas/01-self-modifying-constraint-language.md`](../../docs/ideas/01-self-modifying-constraint-language.md):
put the GBNF meta-grammar, the *currently applied* output-constraint
grammar, an explanation that the harness will apply modifications,
and a task on the syntax itself into the LLM's system prompt; then
cycle the LLM's output into its input.

This is **the most ambitious idea in the raw set** (idea 01 §Why
this is the most ambitious idea). It combines GBNF / grammar-
constrained decoding (M2), reflective / meta-circular evaluators
(the artefact under construction is the constraint system itself),
and self-modifying formal languages (with all the risks they
imply — drift into private dialects).

## Why deferred past M2

M2 ships *static* GBNFs — one per task, parameterised by the IR
ontology, but not user-evolvable. F2 introduces:

- **Versioned grammars with rollback** — `G_0, G_1, …`; the harness
  retains old versions so divergent dialects can be compared rather
  than overwritten.
- **A grammar-update DSL** — high-level edits that compile to GBNF
  (not direct GBNF mutation; idea 01 §Constraints).
- **A semantic firewall** — beyond syntax, a typechecker /
  interpreter that ensures the new grammar is *compileable* and
  *interpretable*. Idea 01 calls this the central open question.
- **A bounded problem domain** — pick "language for theorems about
  finite directed graphs" or even "language for Zebra-style
  puzzles" so the loop has something concrete to optimise (idea 01
  §Concrete next step possibilities).

## What promotion would look like

A new milestone `m_followups_self_modifying/` with phases:

- **PSM.1** — version + rollback infrastructure for GBNFs;
  resolves [Q13](../open_questions.md#q13--self-modifying-constraint-language).
- **PSM.2** — grammar-update DSL (a small Lisp dialect that emits
  GBNF diff operations).
- **PSM.3** — semantic firewall: typecheck the proposed grammar
  by attempting to *parse + interpret* a fixed test set under it.
- **PSM.4** — bounded problem domain: pick the target (Zebra
  grammar optimisation, or graph theorems).
- **PSM.5** — observability: log every grammar mutation; diff
  grammars; visualise divergence.
- **PSM.6** — drift detection: alert when the grammar mutates into
  something the M1 engine can no longer load.

## Risks (idea 01 §Constraints / non-negotiables)

- **Private-language failure mode** — multi-agent / self-loop
  systems drift into locally-optimal-but-meaningless dialects.
  Drift detection (PSM.6) + the bounded domain (PSM.4) are
  containment.
- **Semantic erosion** — GBNF only enforces syntax; without a
  semantic firewall the LLM may emit syntactically valid grammars
  that don't compile.

## Prior art / connections

- [Idea 01](../../docs/ideas/01-self-modifying-constraint-language.md) — the full description.
- [`docs/index/01-llm-constrained-generation.md`](../../docs/index/01-llm-constrained-generation.md) — GBNF / parser-guided decoding.
- Existing nearby work: SMT-LIB (clean S-expression IR), PLT Redex,
  K Framework, ACL2, miniKanren.
- **Not** arbitrary direct GBNF mutation — better an immutable
  kernel + high-level update DSL + typechecker.
