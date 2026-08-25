# S1d.4.3 — The vocabulary: what a test says when it can only afford half the claim

**Phase:** [P1d.4](README.md) (Closing the model set)
**Estimate:** 2 days
**Depends on:** [S1d.4.1](s1d.4.1_what_closure_costs.md) (the denominator) and
[S1d.4.2](s1d.4.2_the_second_order_boundary.md) (whether the claim belongs at
the meta level at all). Reads [P1d.3](../p1d.3_model_sets/README.md)'s answer
if it has landed, and does not wait for it.

## Context

The phase's decision stage. If [S1d.4.2](s1d.4.2_the_second_order_boundary.md)
answers *no — a program may not state its own model count*, then `:expect` is
the right home for the claim and the only question left is what it may say when
it cannot verify what it said.

**Three candidates, none free**, from
[Q-M1d.7](../open_questions.md#q-m1d7--may-a-program-require-its-own-model-count)
and the phase README:

- **(a) a weaker vocabulary** — separate *at least these* from *exactly these*,
  so a test can say the affordable half out loud;
- **(b) a certificate** — [M10](../../m10_external_benchmarks/README.md)'s
  solvers establish the count and `:expect` records who established it;
- **(c) a bound from obligations** — a state that owes nothing may know its own
  model count without enumerating.

And one constraint that outranks all three, from the phase's own Risks:

> **The affordable answer is the wrong one.** "Let `:expect` say *at least
> these models*" is easy, ships in an afternoon, and quietly turns the form
> back into the per-fact assertion Q-M1c.1 rejected — a check that cannot catch
> a surplus, one level up. If the weaker claim ships it needs a different
> keyword, not a looser reading of the same one.

**Two things from S1d.4.1's reconnaissance change how these should be
weighed.** The set-closure form is used **twice in the corpus**, both feature
demos, and `Outcome::NotChecked` **never fires on a corpus entry**. So the
vocabulary being chosen is for a case with two instances and no live failures —
which argues for the smallest thing that discharges the debt, and against
anything that grows the grammar.

**And the debt turns out to be smaller than stated.**
[M1c's thesis](../../../docs/history/m1c_external_validation/README.md#splitting-them-did-not-split-the-pipeline)
says `zebra2-minus-15`'s answer "is written into the `.ein` file as an
`:expect`". It is not — the file carries no expectation at all, and writing one
would be ~512 lines on a 539-line file and then come back `NOT CHECKED`. So
"discharge the debt" has a third option the phase README did not list:
**write the sentence M1c should have written**, which needs no keyword.

## The candidates, and what each has to answer

### (a) A weaker keyword

*"These are models"* without *"and there are no others"*. Cheap, and the
hazard is named above: it must be a **different keyword**, because the same
one read loosely is Q-M1c.1's rejected per-fact assertion wearing the coat of a
model claim.

The question it must answer is what it is *for*. `NOT CHECKED` already reports
the affordable half honestly, and it takes a failing exit code because a green
line for an unchecked claim is what the whole form exists to prevent. A keyword
whose only effect is to turn that red line green is not a vocabulary
improvement — it is the exit code being renegotiated, and it should be argued
as that or not at all.

### (b) A certificate

`:expect` records **who established the count** — Clingo, Z3's blocking-clause
loop, a person — and `ein test` re-checks the affordable half while carrying
the attribution for the half it cannot.

This is the shape that matches M1c's pipeline sentence most literally, and its
hazard is also named: it is *"the sidecar
[Q-M1c.1](../../../docs/history/m1c_external_validation/open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects)
rejected re-entering by the back door"*. The distinction to draw, and it is
drawable: a **sidecar** is a second file the check reads, and an **attribution**
is a string inside the expectation the check does not read. If the certificate
never changes what is verified, it is documentation with a grammar slot, and
whether that is worth a grammar slot is this stage's call.

The sharp version of the question: **would a wrong certificate ever be
caught?** If not, it is a comment, and `.ein` already has comments.

### (c) A bound from obligations

The one that would be new rather than notational, and the one
[P1d.3](../p1d.3_model_sets/README.md) has just made harder. The hope: a
saturated state's obligations bound the model count from above — 46 owed
instances with candidate sets is a constraint on how many completions exist, and
that bound needs no search.

**P1d.3's reconnaissance is the reason this needs measuring before it is
believed.** The 32 models of `zebra2-minus-15` do not factor at any
granularity — one coupling component over all 23 varying decision variables —
so the product of candidate-set sizes is not a model count but a wildly loose
over-approximation (10¹³ against 32). A bound is still a bound, and a useless
bound is still useless: the stage's task is to compute it once and see which it
is.

**A bound is also the wrong shape for the claim** even when it is tight. `(or
M₁ … M_k)` asserts a set, not a cardinality; an upper bound of 32 does not
establish that the 32 listed are the 32 that exist. Where it *would* help is
the other direction — a state whose obligations admit exactly one completion
knows `k = 1` without searching — and whether any corpus entry is in that
position is a number this stage can get from
[`openness_census.md`](../p1d.2_obligations/openness_census.md)'s 12 discharged
entries.

## Tasks

### Task T1d.4.3.1 — (a), (b), (c) priced against S1d.4.1's denominator

One table, the columns being what each candidate costs (grammar, loader,
`expect.rs`, docs), what it makes checkable that is not checkable today, and
**how many corpus entries would use it**. The last column is the one the
reconnaissance makes possible and it is expected to be small; a candidate that
scores well on the first two and 0 on the third is a candidate for a followup.

### Task T1d.4.3.2 — the exit-code question, separated from the keyword question

`NOT CHECKED` takes a failing code. Whether that is right is a question about
**runners**, not about `:expect`'s grammar, and conflating them is how (a)
ships by accident. State the two separately, and note that
[`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md) is where
the answer would be normative if it moved.

The reason it is worth separating: a fixture that is *expected* to be
not-checked has no way to say so today, and that — not the vocabulary — is what
[T1d.4.1.4](s1d.4.1_what_closure_costs.md) ran into.

### Task T1d.4.3.3 — (c) measured before it is judged

Compute the obligation-derived bound on `zebra2-minus-15` and on the seven
small multi-model entries, and compare it to the true count. One number per
entry, and the likely answer is *"loose beyond use"* — which is a result, not a
failure, and F9's rule applies: a mechanism inert on the corpus is recorded as
inert, with the number.

Then the other direction: of
[the 12 discharged entries](../p1d.2_obligations/openness_census.md), does any
know `k` from its obligations without searching? If one does, that is the
useful half of (c) and it is worth more than the bound.

### Task T1d.4.3.4 — the M1c debt, discharged

The phase acceptance's second bullet, and the reconnaissance gives it a third
option:

| option | what it costs | what it leaves |
|---|---|---|
| write the `:expect (or …)` | ~512 lines on a 539-line file | `NOT CHECKED`, and a corpus entry that fails `ein test examples/` |
| ship a vocabulary that makes it checkable | (a), (b) or (c) | a keyword for a case with two instances |
| **rewrite M1c's sentence** | one paragraph | an honest pipeline claim and no grammar change |

The third is not a cop-out — the phase README says so itself: *"A plan that
keeps the sentence and cannot honour it is worse than one that says less."*
What it must not do is quietly drop the sentence: whatever replaces it says
what `ein test` actually re-checks with no external solver installed, and
[M10](../../m10_external_benchmarks/README.md) is told, because the pipeline is
its claim as much as M1c's.

### Task T1d.4.3.5 — if a keyword ships

Grammar, loader refusal, `expect.rs` outcome, a fixture that fires it and a
fixture that does not — the shape
[S1c.1.2](../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
built and [`tests/README.md`](../../../tests/README.md)'s five idioms require.
And the negative that matters most: **a fixture proving the new keyword cannot
be used to make an unchecked claim look checked**, which is the Risk section's
whole worry in one file.

### Task T1d.4.3.6 — the phase ledger

The closing record in [the phase README](README.md), in the form
[P1d.2's](../p1d.2_obligations/README.md) took: decisions and where, stages with
their numbers, the census, and every deferral carrying the specification that
survives it and a trip-wire that is a property of a corpus entry.

**And one hand-off that must be explicit**: whatever this phase decides about
the closure claim is what
[P1d.3](../p1d.3_model_sets/README.md)'s compact form — if one ships — has to
be comparable under. `:expect` comparing *descriptions* rather than enumerating
is the phase README's third question, and it is a cross-milestone edit back
into M1c's form rather than a private decision here.

## Acceptance

- The three candidates are priced in one table, including **how many corpus
  entries would use each**, and a candidate scoring 0 there is named as a
  followup rather than shipped.
- The exit-code question and the keyword question are answered **separately**,
  and the "expected to be not-checked" gap is either closed or recorded.
- (c) carries a measured bound against the true count, and F9's inert-with-a-
  number treatment if it is loose.
- **The `zebra2-minus-15` debt is discharged by one of the three options**, and
  if it is the third, M1c's sentence is actually rewritten and M10 is told.
- If a keyword ships: it is a **new** keyword, `NOT CHECKED` stays
  distinguishable from both pass and fail, and a fixture proves the new form
  cannot launder an unchecked claim.
- **Nothing S1c.1.2 built is weakened** — relation-closure inside a model is
  local, cheap and decidable by inspection, and is not this phase's to trade.
- The phase ledger is written, and the P1d.3 hand-off is in it.
