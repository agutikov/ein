# P2.9 — The formal account (Stage M)

**Estimate:** 1.5 weeks; phase README only. Written to stage depth when
[M5](../../m5_presentation/README.md) fixes the venue, because the depth of
the formal treatment — a definition and a proposition, or a section with
proofs — depends on who will read it.
**Depends on:** nothing to *start* — the definitions below can be written
against the shipped kernel today — and the **Level C gate** to *finish*,
because § M's last line, the central question stated as a probability, has
to name the feedback functions the ablations actually ran.
**Blocks:** [M5](../../m5_presentation/README.md) § 4 *Ein* and § 5
*Autoformalization*, which are this phase written for a reader; Level D's
*formal treatment*.
**Research plan:** [`EinAf.md` § Stage M](../EinAf.md#stage-m--formalize-the-conceptual-model).

---

## What the plan asks for

*Describe Ein mathematically, independently of the Rust implementation.* The
plan gives the skeleton and this phase fills it against the engine that
exists, which is the part that makes it work rather than decoration:

| the plan's object | what it is on this kernel | where the kernel pins it |
|---|---|---|
| `K = (O, R, F, Γ, P)` — objects, relations, facts, rules / constraints, provenance | the typed hypergraph: `Value` / `FactId`, relations with signatures, the layered KB, the AND/OR provenance DAG | [`01-ein-graph/`](../../../docs/kernel/ir/01-ein-graph/), [`02-data-model/`](../../../docs/kernel/ir/02-data-model/) |
| `T_Γ` monotone, `K* = lfp(T_Γ)` | saturation — semi-naive, to a fixpoint; **with a seam**: `(absent P)` is judged at the closure / world boundary, so `T_Γ` is monotone on the positive fragment and the NAF step is a separate, stratified operator | [`architecture_and_algorithms.md`](../../../docs/kernel/inference/architecture_and_algorithms.md) O2, O3; [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md) — the part the plan's one-line `T_Γ` does not have and the account must |
| hypotheses / commitments; the model space `{M ⊇ K* : M ⊨ Γ}` | the commitment lattice: layer *k* the size-*k* commitment sets over `alive`, each saturated, killed by `(false)` or a no-good; a model is a complete, consistent leaf, identified by its sorted fact list | [design/07](../../../docs/history/m1a_rust/design/07_search_layer.md); [`defined_behaviour.md` § 2.4](../../../docs/kernel/defined_behaviour.md) |
| `|𝓜| = 0 / 1 / > 1` → contradiction / unique / ambiguity | the verdict, read off `k` — **with `exhausted`**: `|𝓜|` is known only when the lattice is exhausted, and the engine's fourth state (`Aborted`, and the `-m`-cap case [Q-M1d.6](../../../docs/history/m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false) names) is a lower bound on `|𝓜|`, not a value | [`verdict.rs`](../../../ein.rs/crates/ein-infer/src/verdict.rs); [P2.1 § The sixth outcome](../p2.1_kernel_as_instrumentation/README.md#the-sixth-outcome-today) |
| `A_θ(x) → T`; `E(T) → (v, d)`; `Tᵢ₊₁ = A_θ(x, Tᵢ, E(Tᵢ))` | the formalizer, the feedback object, the loop | [P2.2](../p2.2_formalizer/README.md), [S2.1.2](../p2.1_kernel_as_instrumentation/s2.1.2_feedback_object.md), [P2.4](../p2.4_loop/README.md) |
| `P(faithful(Tₙ) ∧ correct(Tₙ))` as a function of `E` | the milestone's dependent variable, with `E` ranging over F0–F8 | [S2.5.3](../p2.5_harness/s2.5.3_metrics.md), [P2.6](../p2.6_ablations/README.md) |

Three things the account has to say that the plan's sketch does not, because
the kernel does them and a reader who checks will find them:

1. **Negation as failure is not in `T_Γ`.** The least fixpoint is of the
   positive rules; `(absent P)` is evaluated at the closure boundary of a
   world, and the account states the stratification and what it costs
   (the engine's `DerivedNafWarning` is the runtime trace of the case the
   stratification does not cover).
2. **The search is a lattice, not a tree.** Commitment sets ordered by
   cardinality, Apriori-generated, pruned by no-goods and downward closure —
   [F9](../../followups/f9_e_catalog.md)'s ledger is the evidence that this
   matters (*every branch-count optimisation failed, the one cost
   optimisation worked, because the search is a complete cardinality-BFS*).
   An account that draws a DPLL tree describes a different engine.
3. **Lower bounds are refutations.** `total` / `surjective` are implemented
   as *every candidate excluded ⇒ dead*, never as *one candidate owed* —
   [M1d § what the note says](../../../docs/history/m1d_satisfiability/README.md).
   The account states the constraint fragment the engine decides and the one
   it enumerates, which is [F1b](../../followups/f1b_logical_formulation.md)'s
   question answered for the paper.

## Stages — the shape, for when it is written

- **S2.9.1** — the kernel: the definitions in the table, the two operators
  (positive saturation, the NAF step), the lattice, the verdict with
  `exhausted`; each definition followed by the sentence that says which
  page of [`docs/kernel/`](../../../docs/kernel/README.md) is its
  specification and which test holds it.
- **S2.9.2** — the loop: `A_θ`, `E`, the iteration, the two fixed points
  ([Q-M2.4](../open_questions.md#q-m24--is-the-fixed-point-syntactic-or-semantic) —
  syntactic equality of `Tᵢ` and equality of `K*ᵢ`), termination, and the
  central question as a probability over the feedback functions that ran.
- **S2.9.3** — the reading against the categorical and logical notes:
  [F1](../../followups/f1_categorical_formulation.md) (is the fixpoint a
  colimit; composition as the triangle rule) and
  [F1b](../../followups/f1b_logical_formulation.md) (which FOL fragment),
  [F12](../../followups/f12_rules_and_relations/ideas.md) (rules as
  relation-valued operators, properties as closure conditions `R ⋆ R ⪯ R`);
  what of them the account adopts, and what it leaves as future work with
  the reason.

## Acceptance

- Every symbol in the plan's § M has a definition that names the kernel
  page that specifies it and the test that holds it — the account is
  **checkable against the engine**, not beside it.
- The three divergences from the sketch above are stated as such.
- The central question is written with the actual set of feedback
  functions, and the ablation tables are cited as its values.

## Connections

- [`EinAf.md` § Stage M](../EinAf.md#stage-m--formalize-the-conceptual-model).
- [`docs/kernel/`](../../../docs/kernel/README.md) — the specification the
  account formalises; [`glossary.md`](../../../docs/kernel/glossary.md).
- [F1](../../followups/f1_categorical_formulation.md), [F1b](../../followups/f1b_logical_formulation.md),
  [F12](../../followups/f12_rules_and_relations/ideas.md), [F15](../../followups/f15_math_formulae/ideas.md)
  (rules as formulae, the algebraic signature of a theory — the notation
  the account can borrow).
- [M5](../../m5_presentation/README.md) § 4–5 — the consumer.
