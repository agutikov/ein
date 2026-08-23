# P1d.4 — Closing the model set: the claim nothing can state

**Milestone:** [M1d — From saturation to satisfiability](../README.md)
**Estimate:** 1.5 weeks (7 days of stages)
**Depends on:** [P1d.3](../p1d.3_model_sets/README.md) — whether a model set
has a compact form at all decides what a claim about one can even look like;
and [P1d.10](../p1d.10_exhaustive_search/README.md), which is the measurement
of what closing a set costs.
**Created 2026-08-24**, at the user's direction, out of a gap found while
building M1c
[S1c.1.2](../../m1c_external_validation/p1c.1_stdlib_conformance/s1c.1.2_test_form.md).

**Depth: phase README only**, like its two neighbours. See
[§ How deep this plan is](../README.md#how-deep-this-plan-is).

## The gap, exactly

`:expect (or M₁ … M_k)` asserts **the model set is exactly these k**. That is
two claims wearing one coat:

| | the claim | how it is established |
|---|---|---|
| **soundness of the list** | each `Mᵢ` is a model | find it — cheap, local, and the search does it anyway |
| **closure of the set** | there is no `M_{k+1}` | **exhaust the lattice** — global, and the thing that does not finish |

Only the first is affordable, and the second is the one that carries the
meaning. `examples/zebra2-minus-15.ein` is the case: **all 32 models are found
by depth 3**, and depths 4 and 5 exist only to prove there are no more — which
is [the measurement this milestone opens with](../README.md#the-two-halves-of-one-question),
and why `solve -e` on it was killed at thirty minutes.

**And a puzzle cannot state it at all.** `(or A B)` in a `:match` is an
ordinary disjunction over premises — it says *this world satisfies A or B*, and
is satisfied by any world that does. Nothing in the rule language quantifies
over **models**, so "and these are all of them" is not a constraint a program
can carry. The identical s-expression means satisfaction under `:match` and
enumeration-closure under `:expect`, and that asymmetry is the phase's subject
rather than an accident of one keyword's design.

## What M1c already does about it, and why it is not enough

S1c.1.2 shipped the honest half. `:expect` compares distinct models as a
**set** and requires the counts to agree, and — since the same day, after the
hole was found — a search that did not exhaust yields a third outcome:

```
  :expect        NOT CHECKED
    every listed model matches, but the search was not exhausted — k is a
    lower bound, so nothing here is established. Pass --exhaustive.
```

`NOT CHECKED` is not a pass and takes a failing exit code. That closes the
*soundness* hole — no green line for a claim nobody checked — and closes
nothing about the *affordability* one. The result is a form that can state the
answer to `zebra2-minus-15` and cannot verify it on any machine, which is
precisely the pipeline
[M1c's thesis](../../m1c_external_validation/README.md#splitting-them-did-not-split-the-pipeline)
depends on:

> When Clingo enumerates 32 models of `zebra2-minus-15` and Z3's
> blocking-clause loop agrees, that answer is written into the `.ein` file as
> an `:expect`, and from then on `ein test` re-checks it on a machine with no
> external solver installed at all.

**Re-checks it how?** By exhausting a search that does not finish. That
sentence is a debt, and this phase is where it is paid or renegotiated.

## The three questions

1. **May a program require its own model count?** Not "can the engine check
   it" — may a *puzzle* say it. It is a second-order statement (a constraint on
   the set of models, not on a model), and the interesting prior is that no
   language of this family lets a program constrain its own model count:
   ASP's aggregates count within an answer set, not over answer sets, and
   projected model counting is a meta-operation on the program rather than a
   sentence in it. If the answer is *no, and for a reason*, that reason is
   worth writing down once — it is the same boundary
   [Q-M1d.2](../open_questions.md#q-m1d2--where-does-a-requirement-live) draws
   for obligations, one level up.
2. **What can be checked when closure cannot be?** Today: nothing, honestly
   labelled. Candidates, none free: a weaker vocabulary that separates *at
   least these* from *exactly these*, so a test can say the affordable half out
   loud; a **certificate** from outside — [M10](../../m10_external_benchmarks/README.md)'s
   solvers establish the count and `:expect` records who established it, which
   is the sidecar Q-M1c.1 rejected re-entering by the back door; or a bound
   from [P1d.2](../p1d.2_obligations/README.md)'s obligations, where a
   saturated state that owes nothing may know its own model count without
   enumerating.
3. **Does P1d.3's answer change the shape of the claim?** If a model set gets
   a compact description, then `:expect` wants to compare *descriptions* rather
   than enumerate — and that is a cross-milestone edit back into M1c's form,
   not a private decision here.

## Stages

| stage | title | est. |
|---|---|---|
| S1d.4.1 | What closure costs, per corpus entry — which claims are checkable today | 2 d |
| S1d.4.2 | May a program state it? The second-order boundary, and where the neighbours put it | 3 d |
| S1d.4.3 | The vocabulary: what a test says when it can only afford half the claim | 2 d |

**S1d.4.1 is the cheap measurement that sizes the rest**, and it is a sweep the
repo can already run: `exhausted` is in every `--json-summary` and every
`verdict` event, so "which corpus entries could carry a verifiable `:expect`
today" is one pass over the corpus rather than an argument. The expectation is
that the answer is *most of them* — the stdlib fixtures S1c.1.4 is about are
small — and that the exceptions are exactly the puzzles anyone would want to
pin.

## Acceptance for the phase

- **A written answer to "may a puzzle require its own model count"**, with the
  reason, whichever way it goes — this is a language boundary and it should be
  stated once rather than re-litigated per keyword.
- **The `zebra2-minus-15` debt is discharged**: either its 32 models are
  verifiable by something, or M1c's pipeline sentence is rewritten to say what
  is actually checked. A plan that keeps the sentence and cannot honour it is
  worse than one that says less.
- Anything that ships keeps the **guarantee vocabulary** the milestone settles:
  a claim about a model set is marked with what proved it, and `NOT CHECKED`
  stays distinguishable from both pass and fail.
- **No weakening of what S1c.1.2 built.** Relation-closure inside a model is
  local, cheap and decidable by inspection; it is not what this phase is about
  and must not be traded away to make set-closure affordable.

## Risks

- **The affordable answer is the wrong one.** "Let `:expect` say *at least
  these models*" is easy, ships in an afternoon, and quietly turns the form
  back into the per-fact assertion Q-M1c.1 rejected — a check that cannot catch
  a surplus, one level up. If the weaker claim ships it needs a different
  keyword, not a looser reading of the same one.
- **This phase can become model counting.** #SAT and knowledge compilation are
  entire literatures ([`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md)),
  and P1d.3 already carries that risk. Three stages is a decision budget: the
  output is a written answer and possibly a keyword, not a counter.
- **It depends on a phase that may answer "enumerate, and say so".** If P1d.3
  concludes that 32 models are 32 lines, question 3 evaporates and this phase
  shrinks to questions 1 and 2 — which is a legitimate outcome and the reason
  it is sequenced after.

## Cross-links

- [`ein-infer/src/expect.rs`](../../../ein.rs/crates/ein-infer/src/expect.rs) —
  the comparison, and `Outcome::NotChecked`, which is this gap as a value
- [S1c.1.2](../../m1c_external_validation/p1c.1_stdlib_conformance/s1c.1.2_test_form.md)
  — where the form came from, and where the hole was found
- [`examples/features/11_expect_ambiguity.ein`](../../../examples/features/11_expect_ambiguity.ein)
  — the k>1 fixture, which declares `solve -e` and no plain `solve` for exactly
  this reason
- [Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
  — the same question for `k = 0`, and why `:expect (false)` on a non-exhausted
  run is `NOT CHECKED` rather than a position on it
