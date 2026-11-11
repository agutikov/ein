# S1g.2.2 — Valuations as hypotheses: the truth table as R; tautology, satisfiability, equivalence and entailment as questions about R

**Phase:** [P1g.2](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.2.1](s1g.2.1_formulas_and_evaluation.md).
**Pulls in:** [idea 03](../../ideas/03-three-task-classes.md)'s three task classes;
S1f.6.17's applications 2 to 4 (a circuit's truth table, satisfiability, equivalence), which this stage
restates for formulas.
**Stops for the user:** its first task, a page of the notions below on real formulas, before the module's
shape is fixed.

## What it asks

*"what are hypothesis in this case?"* (the user, 2026-11-11). For the semantics, the plan's answer: **a
hypothesis is one letter's value, and the region R is the set of valuations.** Then Ein's answer shape, as it
is, states each classical notion. The stage shows that on programs and finds where the fit is not exact.

## The construction

Three declarations turn S1g.2.1's theory into a truth table (they are in
[S1g.1.1's sketch](../p1g.1_survey_and_design/s1g.1.1_what_ein_represents_today.md#the-sketch-for-rows-2-to-4)):
an **hrule** that proposes each letter's two values, a **restriction** that no letter has both, and an
**obligation** that every letter has one. Then:

- **H** is 2n candidates for n letters; a state is alive when no letter has two values and discharged when
  each has one, so **R is the 2ⁿ valuations**, each a minimal and a maximal state.
- **Root owes** n instances, so ∅ is not in R: the least guess is n candidates.
- A query over all of R groups the states by the formula's value: **the projection classes are the truth
  table**, and a tautology has one class.

Probe 1 (2026-11-11): Peirce's law, two letters, k_s 4, 1 class `?v = 1`, 11 saturations.

A requirement is one more rule, here as a sketch to be replaced by computed programs (it is probe 1b's, which
loaded and answered k_s 0 on the release binary at `8860d616`):

```lisp
;; sketch: added to S1g.1.1's sketch, in place of its (eval …), (solve …) and (query …).
;; A formula that must hold kills each valuation that falsifies it.
(relation must-hold Formula :why "{?1} is required to be true")
(rule required ()
  :match  (and (must-hold ?f) (value-of ?f 0))
  :assert (false)
  :why    "{?f} is required and has the value 0")

(eval (And p (Not p)))
(must-hold (And p (Not p)))
(solve :hrules (guess (p 0) (p 1) (q 0) (q 1))
       :expect (region (owing (model (val q 0)) (model (val q 1)))))
```

## The notions, each to be shown

| the notion | the program | what the read-out says | to be settled |
|---|---|---|---|
| truth table | a query `(value-of F ?v)` over all of R | classes, each with its states | whether the table reads well when the classes are two and the states many; `--models key` on the letters |
| tautology | the same | one class, `?v = 1` | how a program *claims* it ([Q-M1g.4](../open_questions.md#q-m1g4--how-a-program-claims-a-tautology)) |
| satisfiable | `(must-hold F)` | k_s ≥ 1; R lists the satisfying valuations | `--single` and `-n 1` as *find one*, at what cost against describing all of R |
| unsatisfiable | `(must-hold F)` | k_s 0. Probe 1b: *empty: every alive node owes*, 2 candidates dead alone | what N and its cores hold: for `(And p (Not p))`, the two dead sets `{(val p 0)}` and `{(val p 1)}`. Whether the cores read as a refutation, a resolution-style proof, or neither |
| equivalent | `(must-hold (Not (Iff F G)))` | k_s 0 | the same as unsatisfiable; or one class of a query on both values |
| entails | `(must-hold G)` for each G of Γ, and `(must-hold (Not F))` | k_s 0 | with Γ empty this is validity again: three ways to say one thing, and the page picks one to recommend |
| contingent | none of the above | two classes | idea 03's *gaps* |

**Idea 03's three classes, read here:** *solve* is a formula with exactly one satisfying valuation,
*gaps* a contingent one, *contradictions* an unsatisfiable one, and the read-out already tells them apart.

## Tasks

1. **T1g.2.2.1 — the page, shown to the user.** Each notion on three or four formulas of the textbook kind
   (excluded middle, Peirce's law, De Morgan's two, a contingent one, an unsatisfiable one, an entailment
   with two premises), each with its program, its read-out in full, its `--dump-answer`, and for each empty R
   its cores. With it, Q-M1g.4's two ways of claiming a tautology, side by side.
2. **T1g.2.2.2 — without the hrule.** The same programs with the hrule and the `:hrules` clause deleted, so
   the obligations rung proposes the values from the obligation alone, as `zebra2-obligations.ein` does. Same
   R, or the reason it is not.
3. **T1g.2.2.3 — the oracle.** `ein oracle` on every program within its limit; the same R. A disagreement is
   a register row.
4. **T1g.2.2.4 — the Rust check's new rows:** the tautology verdict, the satisfying set and each equivalence,
   for every formula up to the bound, read from `--json-summary`.
5. **T1g.2.2.5 — what ships:** the three declarations as a module or as a documented idiom, as ruled; a
   program per notion with an `:expect`.

## Acceptance

- Each notion has a program whose claim `ein test` holds, and the page shows its read-out.
- R agrees with the oracle on every program within the limit, and with the Rust check on every formula up to
  the bound.
- The page says in plain words what an empty R's `owing` rows and N's cores mean for an unsatisfiable formula.
- The user has ruled on the page before the module's shape is fixed.

## What it may find

- **That *exactly one value* wants more than two rules.** S1f.6.17 expects *"an input's exactly one value
  wants a typed obligation"*; if it found one, this stage uses it.
- **That the cores are not a proof a reader accepts.** A core names the facts that suffice for the
  contradiction, here a letter's value and the requirement; whether that is an explanation is
  [idea 08](../../ideas/08-human-style-deductive-trace.md)'s question, and F22 owns making cores canonical.
- **That a satisfying valuation which leaves a letter open would be a smaller answer.** A formula that does
  not depend on `q` is satisfied whatever `q` is, and R still splits on it. A partial valuation as a state is
  three-valued logic, which [S1g.2.4](s1g.2.4_applications_and_scale.md) takes up.
