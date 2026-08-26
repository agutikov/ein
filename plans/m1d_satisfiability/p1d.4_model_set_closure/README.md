# P1d.4 — Closing the model set: the claim nothing can state

**Milestone:** [M1d — From saturation to satisfiability](../README.md)
**Estimate:** 1.5 weeks (7 days of stages)
**Depends on:** [P1d.3](../p1d.3_model_sets/README.md) — whether a model set
has a compact form at all decides what a claim about one can even look like;
and [P1d.10](../p1d.10_exhaustive_search/README.md), which is the measurement
of what closing a set costs.
**Created 2026-08-24**, at the user's direction, out of a gap found while
building M1c
[S1c.1.2](../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects).

**Depth: stage files, written 2026-08-25** — three of them, and like
[P1d.3](../p1d.3_model_sets/README.md) the phase changed shape before it
started, because a reconnaissance measured what it had assumed. See
[§ How deep this plan is](../README.md#how-deep-this-plan-is) and
§ What the corpus says, below.
**[S1d.4.1](s1d.4.1_what_closure_costs.md) is done, 2026-08-26** — the
reconnaissance below is **superseded** by
[`closure_census.md`](closure_census.md), which is parsed rather than grepped
and disagrees with it: the closure claim is written **once**, not twice, and
the write cost's worst case is 4.28× a file rather than the zebra's 0.96×. What
made the census possible is `ein test --json-report`, a read-out the stage
added because `:expect` had no machine-readable surface at all.

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
[M1c's thesis](../../../docs/history/m1c_external_validation/README.md#splitting-them-did-not-split-the-pipeline)
depends on:

> When Clingo enumerates 32 models of `zebra2-minus-15` and Z3's
> blocking-clause loop agrees, that answer is written into the `.ein` file as
> an `:expect`, and from then on `ein test` re-checks it on a machine with no
> external solver installed at all.

**Re-checks it how?** By exhausting a search that does not finish. That
sentence is a debt, and this phase is where it is paid or renegotiated.

## What the corpus says — measured 2026-08-25, **re-taken parsed 2026-08-26**

Four numbers, from `ein test` over the three roots and the expectation shapes
of every `.ein` file that carries one:

| | grepped, 2026-08-25 | **parsed, 2026-08-26** |
|---|---:|---:|
| files carrying an `:expect` | 62 — 56 `tests/`, 3 `examples/`, 3 broken-fixture refusals | **59** programs that load; the three refusals make no claim, because a claim is a property of a *program* |
| expectations checked, and their outcome | **59 held**, 0 FAILED, **0 not checked** | unchanged, and `exhausted` on 59 of 59 |
| shapes | `(model …)` **40** · `(false)` **20** · `(or …)` **2** | `(model …)` **38** · `(false)` **20** · **`(or …)` 1** |
| the `(or …)` users | `features/10_expect.ein`, `features/11_expect_ambiguity.ein` | **`features/11_expect_ambiguity.ein`, alone** |

**The claim this phase exists to make affordable is written once**, and the
correction is the phase's own method arriving early: `10_expect.ein`'s
`:expect` is a `(model …)`, and the `(or …)` a grep finds in it is **line 12 of
its header comment**, documenting the form. The one real instance —
`11_expect_ambiguity` — is `k = 2, exhausted = true`, a two-model toy that
closes in 0.11 ms. Set-closure is not a form the corpus strains against; it is
one the corpus has never used in anger, and a vocabulary chosen for it is a
vocabulary for **one** instance.

The denominator the census adds: **59 of 124 queries** state a claim at all,
and **1 of 124** states one about a set
([`closure_census.md` §1](closure_census.md)).

**The debt is not merely unverifiable — it is unwritten.**
`examples/zebra2-minus-15.ein` **carries no `:expect` at all**, and neither does
its obligations twin. M1c's sentence describes a workflow nobody has run, which
is a different problem from the one § The gap states and a cheaper one to fix.

**And writing it would roughly double the file.** The query's goal names
`drink-loc`, `nation-loc` and `pet-loc`, and *naming a relation closes it*, so
an `:expect (or …)` must list all three relations' facts in all 32 models — 15
positive facts per model × 32 = **480**, **513 lines on a 534-line file** — and
then come back `NOT CHECKED`. So the cost of *writing* the claim is a second
cost the phase README does not mention, and on the one entry that motivates the
phase it is the larger of the two.

> **And on the corpus it is not the largest instance of itself.** 0.96× is the
> *mildest* ratio of any entry with more than four models.
> `branching/06_lookahead_on` would take **407 lines on a 95-line file —
> 4.28×** — and `saturation/type-exclusivity/pets.ein`, at the depth that finds
> its 35 models, **4.33× on 36 lines**. The write cost is `k × |goal extent| /
> |file|`, so it is worst where a *small demo* has a *large* model set — which
> is where [S1d.3.2](../p1d.3_model_sets/representations.md) already priced the
> compact form out. On `branching/06` the enumeration costs 4.28× the file
> **and** `--models key` declines. Both forms fail on the same entry, for
> unrelated reasons.

**One gap nothing asked for**: `Outcome::NotChecked` — the value that makes the
hole honest — **never fires on a corpus entry**. It is exercised only by
`test_cli.rs` and `expect_semantics.rs` on constructed inputs with a `-m` cap.
A mechanism whose only witnesses are synthetic is one the corpus cannot notice
rotting, and [T1d.4.1.4](s1d.4.1_what_closure_costs.md) is where that is
decided rather than left invisible.

> **Half of that is wrong, and the half that is left is one manifest line.**
> `test_cli.rs::test_exhausts_where_solve_stops_at_one` runs plain `ein solve`
> on `examples/features/11_expect_ambiguity.ein` — a corpus file, no
> constructed input, no `-m` cap — and asserts `NOT CHECKED`. What has no
> witness is a *manifest cell*, because that entry's `runs` column omits plain
> `solve` on purpose. Declaring it is one word and **fails a different gate**:
> `solve` prints the `:expect` verdict on stdout and exits 1 with an empty
> stderr, and `corpus_cli.rs::every_refusal_carries_a_diagnostic` requires a
> non-zero exit to say why on stderr. Whether a false claim is a *refusal* or a
> *result* is a surface question and [S1d.4.3](s1d.4.3_the_vocabulary.md)'s;
> what S1d.4.1 shipped is the trip-wire that goes red the day it moves
> ([`closure_census.md` §5](closure_census.md)).

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

| stage | title | est. | |
|---|---|---|---|
| [S1d.4.1](s1d.4.1_what_closure_costs.md) | What closure costs, per corpus entry — which claims are checkable today | 2 d | **done 2026-08-26** — [`closure_census.md`](closure_census.md) |
| [S1d.4.2](s1d.4.2_the_second_order_boundary.md) | May a program state it? The second-order boundary, and where the neighbours put it | 3 d | |
| [S1d.4.3](s1d.4.3_the_vocabulary.md) | The vocabulary: what a test says when it can only afford half the claim | 2 d | |

**S1d.4.1 is the cheap measurement that sizes the rest**, and the
reconnaissance above is its first hour. Its prediction held — the answer *is*
most of them, 59 of 59 — and what it adds is the **denominator**: the
exceptions are not entries whose claim is too expensive to check, they are
entries that never wrote a claim. So the stage's second half is the
counterfactual — what a closure claim would cost to *write* and to *verify* on
the entries that motivate the phase — and its third is the `NOT CHECKED` gap.

**S1d.4.2 has a precedent to apply rather than a question to open.**
[Q-M1d.2](../open_questions.md#q-m1d2--where-does-a-requirement-live) asked
where a *requirement* lives and answered *(c) a rule shape asserting a reserved
verdict atom*, which works because a requirement is a sentence about **one**
world. The test here is the same one, one level up: is there a rule shape that
expresses model-set closure? The expected answer is no, and the *reason* — a
rule that read the search's state would make derivation depend on the
traversal, which [S1a.7.0's invariant](../../../docs/history/m1a_rust/README.md)
forbids — is the boundary, and is what settles every second-order keyword
anyone proposes next rather than only this one.

**S1d.4.3 inherits a third option for the debt** that this README's § The gap
does not list, because the reconnaissance found it: M1c's sentence can be
**rewritten**. The phase acceptance already licenses it — *"A plan that keeps
the sentence and cannot honour it is worse than one that says less"* — and it
costs one paragraph against a keyword for two instances.

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

- [`closure_census.md`](closure_census.md) — **S1d.4.1's measurement**: who
  claims a model set (1 query of 124), whether the claim is checkable (59 of
  59), what one would cost to write (0.96× the file on the puzzle, 4.28× on a
  feature demo) and where it could not be checked (10 of 121). Re-taken by
  [`utils/closure_census.py`](../../../utils/closure_census.py) in 2 min 55 s
- [`ein-infer/src/expect.rs`](../../../ein.rs/crates/ein-infer/src/expect.rs) —
  the comparison, and `Outcome::NotChecked`, which is this gap as a value
- [S1c.1.2](../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
  — where the form came from, and where the hole was found
- [`examples/features/11_expect_ambiguity.ein`](../../../examples/features/11_expect_ambiguity.ein)
  — the k>1 fixture, which declares `solve -e` and no plain `solve` for exactly
  this reason
- [Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
  — the same question for `k = 0`, and why `:expect (false)` on a non-exhausted
  run is `NOT CHECKED` rather than a position on it
