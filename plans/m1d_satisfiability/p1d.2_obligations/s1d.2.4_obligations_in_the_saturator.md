# S1d.2.4 — Obligations in the saturator

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 4 days
**Depends on:** [S1d.2.3](s1d.2.3_the_form.md) (the atom loads),
[S1d.2.1](s1d.2.1_property_audit.md) (the disturbance list),
[S1d.2.2](s1d.2.2_domains.md) (the domain contract)

## Context

The report stratum, whole — and nothing else: after this stage a saturated
state can say **what it still owes**, through every surface a fact store is
inspected through, while every verdict word, counter and golden stands.

The mechanism is decided (`obligation_forms.md` § G, § When the obligation
rules run): obligation rules are **not in the saturation agenda**. They are
one pass over the quiescent KB, run once the fixpoint is reached — after
saturation completes, never mixed into it — and their conclusions are
**tallied, never admitted**. The atom is terminal like `(false)`: no rule
matches it, the engine reads it.
An instance is (rule, bindings), and the tally counts the instances that
fire, deduped by instance identity so two rules owing the same slot count
once each and one rule firing twice counts once.

**Discharge is the guard** — revised 2026-08-25 with the form
([S1d.2.3](s1d.2.3_the_form.md) item 3). Under the superseded triple the
engine re-ran `∃b: G ∧ B` per instance, which restated the rule's own
`absent` and could disagree with it. Under `(open ?R)` there is one
statement: the rule fires while the witness is missing and does not fire once
it has arrived. Evaluated at the fixpoint, that is a property of the final
KB, so "undischarged" and "firing" are the same predicate, the tally has no
second query to pay for, and `absents_still_pass` goes back to being about
ordinary negative premises rather than load-bearing for the verdict.

**Where the tally lives**: a field of the search-lattice node, beside
`CommitmentSetResult.kind` — not in the KB, because truth maintenance is the
reason openness cannot be a fact (a stored `open` would survive its own
discharge in a fork).

## Tasks

### Task T1d.2.4.1 — the post-fixpoint pass

Run the obligation rules once per quiescent KB, after saturation reaches its
fixpoint, and keep the tally and the instance list on the lattice node beside
`CommitmentSetResult.kind` — not in the fact store. S1d.2.3 has already kept
these rules out of the agenda, so the check here is the complement: with the
duals loaded and activated, **the saturation loop's firing counts and
selection order are bit-identical** on every corpus entry. S1d.2.1's
disturbance list is what says which bands that claim has to survive.

### Task T1d.2.4.2 — the three surfaces

- **`--events`**: a new `owe` event kind (per undischarged instance at
  quiescence: rule, bindings, rendered `:why`) — the events schema moves, and
  `every_event_kind_the_schema_defines_is_reachable_from_the_corpus` demands
  a corpus entry reaches it, so the fixtures of T1d.2.4.4 are not optional.
- **`--json-summary`**: an `owes` block — count plus instances.
- **the trace**: an "outstanding obligations" section rendering the `:why`s.

[`events.md`](../../../docs/kernel/inference/events.md) and the summary
schema doc move with it.

### Task T1d.2.4.3 — the stdlib duals

`total-owed` / `surjective-owed` in `std.algebra` (or `std.bijection` — the
audit says which file owns totality today), fanned out by `bijective-setup`
gaining two activators; the slots-family dual per S1d.2.1's finding. No
puzzle changes a line — the fan-out reaches every `(bijective R)` declarer.

### Task T1d.2.4.4 — the conformance programs

Per new rule, the P1c.1 pair: one program where it **fires** (an unmet
obligation) and one where it is **loaded, activated and satisfied** — fires
nothing, owes nothing. The stdlib-coverage gate enforces the first half
mechanically; the second is the negative case a guard bug lives in.

**The check channel, decided here** — because `:expect` cannot carry this
claim and will not learn to in this stage. Its three forms are `(model …)`,
`(or (model …) …)` and `(false)`
([`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md)),
each an assertion about *facts*, and an `open` verdict is by construction
never a fact — so no `:expect` can observe an owe count. Meanwhile the gate
demands that every program under `tests/` state one. So:

- each fixture carries an ordinary `:expect (model …)` about the facts it
  derives, which is what satisfies the gate and what pins the rest of its
  behaviour;
- the **owe claim** is checked in-process, by a sibling of
  [`stdlib_coverage.rs`](../../../ein.rs/crates/ein-infer/tests/) — load the
  fixture, run it the way `ein test` does, assert the tally and the instance
  list. That is S1c.1.5's own pattern (in-process, no binary, 0.04 s) and the
  same place the T1d.2.4.5 numbers are asserted.

The alternative — growing `:expect` a word for an open verdict — is
**deliberately not taken here**: it is a verdict-vocabulary change, this
stage moves no verdict word, and
[S1d.2.6](s1d.2.6_verdicts_counters_corpus.md) T3 already routes it to
[P1d.4](../p1d.4_model_set_closure/README.md). It becomes the right answer
the moment a *puzzle author* outside this suite needs to state the claim;
until then a Rust test is the honest channel and costs no grammar.

### Task T1d.2.4.5 — the two numbers

- **The §5 number, from the engine.** `zebra2-minus-15` at root quiescence
  reports **owes = 46** — the number
  [`obligation_forms.md` §5](obligation_forms.md#5-what-this-looks-like-on-zebra2-minus-15)
  measured by hand (23 forward + 23 backward). The engine reproducing the
  hand census is the stage's acceptance in one line.
- **The conservation audit.** A declared `bijective` n×n owes 2n at the
  unwitnessed base; the predicted-vs-emitted diff is a test
  (`ideas.md`'s "obligated facts × arity", the layer-census style of claim).

### Task T1d.2.4.6 — the cost guard

The pass is one sweep per quiescent KB, and a search enters a great many of
them — so this is still the §1 cost shape P1a.6 spent twelve stages buying
back, just bounded better than a band would have bounded it: nothing runs
*inside* the loop, and the projection is resolved once per activator
(S1d.2.3) rather than per firing. Instrument with `ein_core::counters`
(compiled out unless asked), and hold the line: `zebra -e` and `zebra2 -e`
within their P1a.6 baselines. The stage that cannot hold it redesigns the
pass — the obvious lever is running it only where a verdict is read rather
than at every node — not the budget.

## Acceptance

- A saturated state reports outstanding obligations through `--events`,
  `--json-summary` and the trace — the phase README's second acceptance
  bullet, delivered here.
- `ein solve examples/zebra2-minus-15.ein` (root) reports owes = 46; the
  conservation audit passes on every `bijective` declarer in the corpus.
- **Every existing verdict, counter and golden unchanged** — this stage adds
  a report line and nothing else. New goldens only for the new fixtures.
- Timings hold (`zebra -e` 47.5 ms-class, per the F9/P1a.6 discipline);
  the counter instrumentation shows where the tally's time goes.
