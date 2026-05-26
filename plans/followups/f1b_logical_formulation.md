# F1b — Logical formulation

**Theme owner:** the user (research interest).
**Trigger:** when the kernel + rule library are stable enough that
the question "what fragment of FOL does ein cover?" has a concrete
answer. Sibling to [F1](f1_categorical_formulation.md) — both ask
"what is the formal substrate?", but from different angles.

## What this is

Can we express **first-order logic** in ein, and how? The kernel
already ships much of the syntactic shape — relations, variables,
`(not …)`, `(absent …)`, `(and …)`, `(or …)`, the `forall` macro
(S1.5.9), the universal closed-world `closed` activator — so the
question isn't "is FOL embeddable" but "which fragment is *native*
and which would be derived sugar?".

Reference points to compare against:

- [Rules of inference — propositional calculus](https://en.wikipedia.org/wiki/List_of_rules_of_inference#Rules_for_propositional_calculus)
  — modus ponens, modus tollens, disjunction elimination,
  conjunction introduction, etc. Which of these are direct
  rule-graph rewrites in ein and which need machinery the kernel
  doesn't have?
- [Relation (mathematics)](https://en.wikipedia.org/wiki/Relation_(mathematics))
  — n-ary relations, the algebra of subsets of cartesian products.
  ein's `(relation R T1 T2 …)` declaration is exactly this; what's
  *missing* is the second-order vocabulary (relations as objects
  one quantifies over).
- [Relation algebra](https://en.wikipedia.org/wiki/Relation_algebra)
  — composition, converse, identity, union, intersection,
  complement. The 2026-05-22 stdlib work (`converse` rule, the
  `imply` family, `symmetric` ⟺ `converse R R`) is *literally*
  relation algebra encoded as activators; F1b asks how far this
  can go.
- [Equivalence relation](https://en.wikipedia.org/wiki/Equivalence_relation)
  — reflexive + symmetric + transitive. ein's `co-located` (when
  used in zebra) is exactly an equivalence relation; the property
  facts that mark it as such are activators for the kernel rules.


TODO: Relation algebra questions:
- stdlib rules for rel properties from list: functional, coreflexive, etc...
  - converse - what? rule?
- properties detection - are rules able to detect properties from facts?


## Why deferred

Like F1, the formal reading is post-hoc — pinning down "ein is
FOL ∩ X" only becomes useful when the engine + rule set are stable
enough that the answer doesn't drift each milestone. M1 ships
operationally; F1b formalises after.

The trigger for promotion: when the rule library (stdlib + custom)
is large enough that *characterising* it formally pays back the
characterisation effort. A leading indicator is a question like
"can ein decide formula X?" that the current docs can't answer
without re-deriving the semantics from scratch.

## What promotion would look like

A new milestone `m_followups_logical/` with phases:

- **PFL.1** — pin the fragment: which of `∀`, `∃`, `¬`, `∧`, `∨`,
  `→`, `↔`, `=`, `≠` are native; which are macros; which are
  uninhabited. State the answer as a one-line BNF over the kernel
  forms.
- **PFL.2** — encode the propositional-calculus rules of inference
  as ein activators / rules; check completeness against the Wikipedia
  table.
- **PFL.3** — relation-algebra operations: declare `compose`,
  `intersect`, `union`, `complement` as activator families;
  prove (or disprove) closure under composition for the rule set.
- **PFL.4** — identify the **first-order fragment ein cannot
  decide** without an SMT backend (cf. [M3](../m3_smt_integration/README.md));
  document the boundary.

## Prior art / connections

- [F1 — Categorical formulation](f1_categorical_formulation.md) —
  the other formal-substrate followup. F1 asks "is the engine a
  category?", F1b asks "is the engine a fragment of FOL?". Both
  are post-hoc readings; both feed each other (FOL ⊂ topos
  internal language; relation algebra is one categorical structure).
- [F5 — Rules as data](f5_rules_as_data.md) — if rules-on-rules
  works, the rules of inference become *expressible* in ein
  itself; F1b becomes the proof obligation for that expressibility.
- [S1.5.9 macros](../m1_core_graph_reasoning/p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md)
  — the `forall` / `open` macros are the syntactic precursors to
  the FOL fragment F1b would characterise.
- [P1.8 Theme A — stdlib](../m1_core_graph_reasoning/p1.8_ein_lang_modules/README.md)
  — the `converse` rule, the `imply` family, `symmetric ⟺ converse R R`
  are the relation-algebra encodings F1b §PFL.3 would systematise.
- [docs/index/03 — theorem-proving / formal methods](../../docs/index/03-theorem-proving-formal-methods.md)
  — external tech: Coq, Lean, Isabelle/HOL — the formalisations
  whose ein equivalent F1b would document.
- [docs/index/02 — solvers / CSP / SAT / SMT](../../docs/index/02-solvers-csp-sat-smt.md)
  — the boundary case PFL.4 names.
