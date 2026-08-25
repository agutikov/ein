# S1d.4.1 — What closure costs, per corpus entry

**Phase:** [P1d.4](README.md) (Closing the model set)
**Estimate:** 2 days
**Depends on:** nothing — `exhausted` is already in every `--json-summary` and
every `verdict` event, which is why this is the stage that sizes the rest.

## Context

The phase README predicted what this sweep would find:

> The expectation is that the answer is *most of them* — the stdlib fixtures
> S1c.1.4 is about are small — and that the exceptions are exactly the puzzles
> anyone would want to pin.

**A reconnaissance taken 2026-08-25 confirms it and then says something
sharper.** Four numbers, from `ein test` over the three corpus roots and a
grep over the expectation shapes:

| | |
|---|---:|
| `.ein` files carrying an `:expect` | **62** — 56 under `tests/`, 3 under `examples/`, 3 broken-fixture refusals |
| of those, expectations that **held** | **59 of 59** checked; 0 FAILED, **0 not checked** |
| expectation shapes | `(model …)` **40** · `(false)` **20** · `(or …)` **2** |
| the two `(or …)` users | `features/10_expect.ein`, `features/11_expect_ambiguity.ein` |

**So the claim this phase exists to make affordable is written twice in the
whole corpus, and both are feature demos.** The only real one —
`11_expect_ambiguity` — is `k = 2, exhausted = true`: a two-model toy that
closes in milliseconds. The set-closure form is not a thing the corpus strains
against; it is a thing the corpus has never used in anger.

**And the debt is not merely unverifiable — it is unwritten.**
[M1c's thesis](../../../docs/history/m1c_external_validation/README.md#splitting-them-did-not-split-the-pipeline)
says the answer to `zebra2-minus-15` "is written into the `.ein` file as an
`:expect`, and from then on `ein test` re-checks it". **`examples/zebra2-minus-15.ein`
carries no `:expect` at all**, and neither does its obligations twin. The
sentence describes a workflow nobody has run, which is a different problem from
the one the phase README states and a cheaper one to fix.

**What it would cost to write** is measurable and was measured. The query's
goal names three relations — `drink-loc`, `nation-loc`, `pet-loc` — and *naming
a relation closes it*, so an `:expect (or …)` must list every one of their facts
in all 32 models: **15 positive facts per model × 32 = 480**, about **512 lines
of expectation on a 539-line file.** It would roughly double the puzzle, and
then come back `NOT CHECKED`.

**One gap the sweep found that nothing asked for.** `Outcome::NotChecked` — the
value that makes the whole hole honest — **never fires on a corpus entry**. It
is exercised, but only by `test_cli.rs` and `expect_semantics.rs` on
constructed inputs with a `-m` cap. A mechanism whose only witnesses are
synthetic is a mechanism the corpus cannot notice rotting, and the repo's own
rule for that case is
[F9](../../followups/f9_e_catalog.md)'s: *record it as inert, with the number*.

## Tasks

### Task T1d.4.1.1 — the usage census

Every `.ein` file under `examples/`, `tests/` and `stdlib/`: does it carry an
`:expect`, which of the three shapes, and — for `(or …)` — how many models it
lists. Parsed from the loaded program rather than grepped, because
`:expect` is a query keyword and a grep cannot tell a keyword from a comment
about one (this file's own reconnaissance grepped, and says so).

The output is one table, and its point is the denominator: **what fraction of
the corpus makes a closure claim at all.**

### Task T1d.4.1.2 — verifiability today

For each expectation-carrying entry, under `ein test`'s regime (exhausting, no
`-n`): the outcome and `exhausted`. The reconnaissance says 59/59 held and
nothing was `NOT CHECKED`; this re-takes it as a table with the depth each
entry reached, so *"which claims are checkable today"* is answered per entry
rather than in aggregate.

**The interesting column is the one that is empty**: entries whose expectation
was not checked. If it stays empty, the phase's affordability problem does not
exist anywhere the corpus can see it, and [S1d.4.3](s1d.4.3_the_vocabulary.md)
is choosing a vocabulary for a case with no instances — which is a legitimate
thing to know before choosing one.

### Task T1d.4.1.3 — the counterfactual, for the entries that motivate the phase

The census above measures claims that *exist*. This measures the ones that do
not, on the entries anyone would want to pin — `zebra2-minus-15`, its
obligations twin, and the seven small multi-model entries
[P1d.3's reconnaissance](../p1d.3_model_sets/README.md) found. Two costs, and
they are independent:

| cost | how it is measured |
|---|---|
| **to write** | goal relations × models × facts, under relation-closure — 512 lines for `zebra2-minus-15` |
| **to verify** | the wall and the depth at which `exhausted = true` is reached, or the statement that it is not reached |

The second is [P1d.10](../p1d.10_exhaustive_search/README.md)'s subject and
this stage borrows its numbers rather than re-taking them: `solve -e` on
`zebra2-minus-15` is 618 076 enterings and 416 s and still `exhausted = false`
([layer census §4](../p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers)).

**Both costs matter and the phase README only names the second.** A claim that
is affordable to check and doubles the file it lives in is not obviously worth
writing, and that is a finding this stage can deliver without deciding
anything.

### Task T1d.4.1.4 — the `NOT CHECKED` corpus gap

The mechanism has no corpus witness. Decide, with the measurement in hand,
whether to give it one — and the option worth weighing is the sharp one:
**write `zebra2-minus-15`'s `:expect (or …)` and let it come back `NOT
CHECKED`**, so the debt is visible in the gate instead of in prose.

Its consequence is what makes it a decision rather than a task: `NOT CHECKED`
takes a failing exit code, so such an entry breaks `ein test examples/` by
design. The choices are a fixture that is *expected* to be not-checked (which
needs a runner concept that does not exist), a smaller synthetic entry that
reaches the cap cheaply, or recording the gap and leaving it — with the reason.
**This stage does not have to close it; it has to stop it being invisible.**

### Task T1d.4.1.5 — the measurement, banked

`closure_census.md` beside this file. It carries the three tables, and it is
the input [S1d.4.3](s1d.4.3_the_vocabulary.md) prices vocabularies against —
in particular the denominator, because a vocabulary for a case with two
instances is priced differently from one with fifty.

## Acceptance

- The usage census is parsed, not grepped, and gives the fraction of the corpus
  that makes a closure claim.
- **Every expectation-carrying entry has a row** saying whether its claim is
  checkable today and at what depth — including the empty column, reported as
  empty rather than omitted.
- The counterfactual carries **both** costs for the entries that motivate the
  phase, and states the write cost in lines of `.ein`.
- The `NOT CHECKED` gap is decided or explicitly recorded as open, with the
  exit-code consequence stated.
- `closure_census.md` banked and re-takable, the milestone's third census after
  [`layer_census.md`](../p1d.10_exhaustive_search/layer_census.md) and
  [`openness_census.md`](../p1d.2_obligations/openness_census.md).
- **Nothing about `:expect` changes in this stage.** It measures; S1d.4.3 is
  the only stage that may move a keyword.
