# Inspirations and divergences

ein-lang takes from several traditions and diverges from each in
specific places. This document records the lineage.

This was [`docs/ir.md` §4](../../../ir.md) before the kernel-
documentation split.

---

| source                                                          | what we took                                                                  | what we diverged on                                                          |
|-----------------------------------------------------------------|-------------------------------------------------------------------------------|------------------------------------------------------------------------------|
| **SMT-LIB 2**                                                   | S-exprs, `;` + `#| … |#` comments, `:keyword` arg markers, atom-heavy semantics | rules in-band (no `(define-fun-rec …)` distinction); no theory annotations; no `(check-sat)` |
| **miniKanren**                                                  | relational atom shape, `?var` convention, conjunctive patterns                 | no first-class `fresh` / unification syntax — the validator handles binding  |
| **AtomSpace**                                                   | explicit `(type …)` / `(instance …)` declarations; atom typing                | no probabilistic truth values; no built-in node weights                       |
| **Datalog / Soufflé**                                           | rule shape `head :- body` as inspiration for `:match` / `:assert` separation  | S-expressions over infix; `:why` templates for human-readable firings        |
| **DPO graph rewriting** ([idea 07](../../../ideas/07-categorical-formulation.md)) | `:match` / `:assert` as before / after sub-graphs                              | not yet expressed as `L ← K → R` span — DPO render mode is opt-in       |

## Why S-expressions

The competing surface for a constraint IR is a richer DSL with infix
operators (Prolog-style, MiniZinc, or even Z3's API). The S-expression
choice — locked by
[Q3](../../../../plans/open_questions.md#q3--surface-ir-syntax) — is
motivated by:

- **Homoiconicity.** Rules are facts about facts; traces are facts
  about derivations. Having one syntactic family for everything means
  rules can match traces, the trace renderer can manipulate IR, and
  M2's NL → IR LLM can emit one grammar instead of several.
- **GBNF lift.** A future M2 GBNF for LLM-constrained decoding is
  a near-mechanical translation from `grammar.lark`. SMT-LIB-style
  syntax is the easiest target for grammar-based constrained
  generation (see [`docs/index/01-llm-constrained-generation.md`](../../../index/01-llm-constrained-generation.md)).
- **Comments + naming.** Hyphenated names and `;` comments make
  hand-authored puzzle files readable; PascalCase types vs
  hyphen-relation names visually distinguish entity kinds without a
  parser hint.

## Why a pattern sub-language inside rules

Some constraint systems (Z3, MiniZinc) keep one universal expression
language; others (Prolog, Datalog) keep two (terms + clauses). ein-
lang is in the latter camp: the **pattern sub-language**
([`02_patterns.md`](02_patterns.md)) is a strict subset of the
fact-form syntax, plus variables.

The split makes the rule-application semantics legible: a rule's
LHS is *only* a pattern, never an expression with side effects or
arbitrary calls. This is what enables the trace fidelity acceptance
([idea 08](../../../ideas/08-human-style-deductive-trace.md)): every
firing has a structurally observable cause.

## What we'd revisit

If M1 acceptance proves the explanation-completeness criterion
durable across the ten rule families, the obvious post-M1 evolution
is the **DPO categorical reading** ([idea 07](../../../ideas/07-categorical-formulation.md))
where rules become `L ← K → R` spans and the engine becomes a graph-
rewriting morphism. The `04_dot_rendering.md` §Rule rendering mode
(b) is the visual anchor for that direction.

The other open evolution is the **headless-list `:using` syntax**
(`:using ((rel a b) (rel c d))`), currently not parseable — see
[`01_grammar.md` §Reasoning](01_grammar.md). A small grammar tweak
unlocks round-trippable engine dumps; we defer to M1 acceptance
needs.

## See also

- [`docs/index/02-solvers-csp-sat-smt.md`](../../../index/02-solvers-csp-sat-smt.md)
  — the SMT-LIB / Datalog / Prolog lineage in detail.
- [`docs/index/05-category-theory.md`](../../../index/05-category-theory.md)
  — DPO and the categorical framing.
- [`docs/index/10-nlp-semantic-parsing.md`](../../../index/10-nlp-semantic-parsing.md)
  — why a homoiconic IR helps with NL → IR.
