# The vocabulary — nothing ships, and the reason is a denominator

**Stage:** [S1d.4.3](s1d.4.3_the_vocabulary.md) · **Phase:** [P1d.4](README.md)
**Decided:** 2026-08-26, by the user. **No keyword.** All tests stay
exhaustive by default, every `:expect` stays closed by default, and there is
**no extra syntax** — not a weaker claim, not a certificate, not a bound.
A claim too slow to check at the runner's depth stays out of the corpus, and
deep exhaustive search is [P1d.10](../p1d.10_exhaustive_search/README.md)'s.
**What shipped instead** is one line on stderr, one word in `corpus.toml`, and
two sentences rewritten in other people's documents.
**Inputs:** [`closure_census.md`](closure_census.md) (the denominator),
[`the_boundary.md`](the_boundary.md) (where the claim may live).

---

## 1. The decision

The phase's three candidates, priced against S1d.4.1's denominator, with the
column the reconnaissance made possible:

| candidate | what it would make checkable | corpus entries that would use it | verdict |
|---|---|---:|---|
| **(a) a weaker keyword** — *at least these* | nothing. `NOT CHECKED` already reports the affordable half | **0** | **declined** |
| **(b) a certificate** — who established the count | nothing the check reads | **0** | **declined** |
| **(c) a bound from obligations** | an upper bound on *k*, unavailable where it matters | **2 of 12** | **declined, and measured** |

Zero in the third column for (a) and (b) is not a forecast. Today the corpus
states **one** closure claim, it holds, and it exhausts
([census §1–2](closure_census.md)); a weaker form has nothing to weaken and a
certificate has nothing to attribute. F9's rule is the one that applies: *a
mechanism inert on the corpus is recorded as inert, with the number.*

### Why (a) is the one worth spelling out

*"These are models, and there may be others"* is the affordable answer and the
phase README named it as the dangerous one. The sharper objection is that it
does not do the thing it looks like it does:

**`NOT CHECKED` already says the affordable half out loud.** It reports every
listed model as matching, states that the search did not exhaust, and takes a
**failing** exit code. A weaker keyword's only *operational* effect is to turn
that red line green. So it is not a vocabulary proposal at all — it is a
proposal to renegotiate an exit code, wearing a grammar slot. Argued as the
former it is a runner question (§3); argued as the latter it is
[Q-M1c.1](../../../docs/history/m1c_external_validation/open_questions.md)'s
rejected per-fact assertion one level up, unable to catch a surplus model the
way its ancestor could not catch a surplus fact.

### Why (b) is a comment with a grammar slot

The test to apply: **would a wrong certificate ever be caught?** No — an
attribution is a string the check does not read. A *sidecar* (a second file the
check consults) is what Q-M1c.1 rejected; an attribution is documentation, and
`.ein` already has comments. If M10 wants provenance recorded, a comment above
the `:expect` carries it at zero cost to the grammar.

---

## 2. (c) measured before it was judged — and it fails twice

The stage required a number rather than an argument. Two were taken.

### It is loose beyond use

The obligation-derived bound is the product of the live candidate-set sizes at
root. On `examples/zebra2-minus-15-obligations.ein`, from the `rung` event's own
generation at `solve -m 1 -e --events`:

| | |
|---|---:|
| owed instances at root | **46** |
| branches the rung proposes, declined, uncovered | 46 · 0 · 4 |
| raw candidates | 230 |
| live candidate facts (`hyp` `emitted`) | **96**, over **23** instances — 9 of size 5, 9 of size 4, 5 of size 3 |
| **Π \|candidates\|** | **1.244 × 10¹⁴** |
| true *k* | **32** |
| over-statement | **3.89 × 10¹²** |

Which is the same order as the two independent over-statements P1d.3 already
measured — `Π \|dom\|` over the models' varying variables, and the arithmetic the
certain core invites, at 3.11 × 10¹²
([`representations.md`](../p1d.3_model_sets/representations.md)). Three
different ways of multiplying candidate sets, three answers within a factor of
two of each other, all of them **twelve orders of magnitude** past the truth.

**And the 23 are the same 23**, which is what makes the comparison a
cross-check rather than a coincidence: five attribute relations over five
objects is 25 slots, the clues fix two, and
[`model_set_census.md` §2](../p1d.3_model_sets/model_set_census.md) reports
exactly `vars 23, fixed 2` for this file. The coupling it measured is why the
product is hopeless: those 23 are **one** component, K₂₃ minus five edges, so
multiplying candidate-set sizes assumes an independence that is absent on every
edge but five.

### It is unavailable where a model set exists

Worse than loose, and the number nobody had asked for. Of the **12 corpus
entries with a model set** ([census §3](closure_census.md)), how many state an
obligation at all?

| | entries |
|---|---:|
| `declared = 0` — no obligation stated, so no bound exists | **10** |
| `declared = 2`, `owed = 46` — the two `zebra2-minus-15` twins | 2 |

So (c) is not a bound that is too weak; it is a bound that **does not exist on
ten of the twelve entries it would be for**. `branching/*`, `lattice/*`,
`features/11` and `saturation/type-exclusivity/*` state no requirement, and a
mechanism that reads requirements has nothing to read.

### And it is the wrong shape even when it is tight

`(or M₁ … M_k)` asserts a **set**, not a cardinality. An upper bound of 32 does
not establish that the 32 listed are the 32 that exist — it cannot distinguish
the right 32 from a wrong 32.

### The useful direction turns out to be already shipped

The stage asked the other half honestly: *does any state know its own `k` from
its obligations without searching?* **Yes — 58 of the corpus's 59 claims do**,
and it is not a new mechanism. They enter **zero commitments**: saturation
reaches a state the generator calls complete and the tally calls discharged,
and the verdict is read off it. The engine's `complete(kb)` plus S1d.2.6's
discharge test *is* (c)'s useful half, shipped at
[S1d.2.4](../p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md)–[S1d.2.6](../p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md).
The one claim that needs a search is the one claim about a *set*.

---

## 3. The exit code and the stream, separated from the keyword

T1d.4.3.2's discipline: *whether `NOT CHECKED` should take a failing exit code
is a question about **runners**, not about `:expect`'s grammar, and conflating
them is how (a) ships by accident.*

**The exit code does not move.** `NOT CHECKED` is not a pass; a green line for
a claim nobody checked is what the whole form exists to prevent
([`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md)). It
stays 1 under both `solve` and `test`, and it stays distinguishable from both
pass and fail in the report.

**The stream did move, and it is the one thing this stage shipped in the
engine.** [S1d.4.1](closure_census.md) found that `ein solve` printed the
`:expect` verdict on stdout and exited 1 with an **empty stderr** — the exact
shape `corpus_cli::every_refusal_carries_a_diagnostic` exists to forbid, and
the reason `NOT CHECKED` had no manifest cell. The question it posed —
*is a false claim a refusal or a result?* — has an answer that keeps both:

| | stream | why |
|---|---|---|
| the `:expect` block (label, disagreements, the derivation behind a surplus fact, the models projected through the `:goal`) | **stdout**, unchanged | it is what the run *found*. A false claim is a **result** and its report belongs under the table it is about |
| `<file>: :expect NOT CHECKED — expected Ambiguity with k = 2, got Solution with k = 1` | **stderr**, one line | an exit 1 nobody can diagnose from a pipeline is a defect whichever stream the detail is on |

**And the cell it unblocked is banked.**
`examples/features/11_expect_ambiguity.ein` now declares plain `solve` in its
`runs`, `corpus_exits.txt` gains **one line — `1 … :: solve`** — and
`Outcome::NotChecked` has the corpus witness
[T1d.4.1.4](s1d.4.1_what_closure_costs.md) went looking for. One word of
manifest, one line of golden, one line of stderr.

**The "expected to be not-checked" gap is recorded, not closed.** A fixture that
*should* come back unchecked still has no way to say so, and `ein test` will not
grow one — *"There is no setup, teardown, fixture, tag, skip or
parameterisation, and there will not be. If a rule needs a framework to be
tested, the interesting finding is about the rule."* Under the corpus policy in
§4 the gap has no instances, which is why it stays a note rather than a
followup.

---

## 4. The corpus policy, and the gate that already enforces it

The user's rule, in the form the repo can check: **a claim that cannot be
checked at the runner's own depth does not go in the corpus.**

It needs no mechanism, because it has one. `NOT CHECKED` takes exit 1 and
`ein test` is in `cargo test`, so **a claim that does not check cannot be added
without turning the gate red.** The policy is not a convention somebody has to
remember; it is the failure mode of adding one.

Its current cost is **zero entries**, measured: 59 of 59 claims hold and all 59
exhaust. Its *forward* cost is the ten entries whose claim would come back
unchecked if written ([census §4](closure_census.md)) — of which the sharp one
is `saturation/type-exclusivity/pets.ein`, `Contradiction k = 0` at `-m 5` and
**35 models** at `-m 10`. None of the ten carries a claim, and under this policy
none of them gets one until the search that would check it is affordable.

Which is the hand-off: **[P1d.10](../p1d.10_exhaustive_search/README.md) is
what would move the policy**, by making the deep exhaustive search cheap enough
that the depth stops being the constraint. That is the phase boundary the user
drew, and it is the right one — the vocabulary was never the thing standing in
the way.

---

## 5. The `zebra2-minus-15` debt, discharged

The phase acceptance: *either its 32 models are verifiable by something, or
M1c's pipeline sentence is rewritten. A plan that keeps the sentence and cannot
honour it is worse than one that says less.*

Both halves turned out to be true, and the sentence is rewritten because the
first half is not what the sentence claimed:

| | |
|---|---|
| **are the 32 verifiable?** | **yes**, and measured: `-m 38 -j16` ends the lattice at depth 22 — 17 204 592 enterings, `k = 32`, `exhausted = true`, 24 min 56 s ([the milestone's opening measurement](../README.md#the-two-halves-of-one-question)) |
| **does `ein test` verify them?** | **no.** It exhausts at `-m 5`, where the answer is `Ambiguity k = 32, exhausted = false` → `NOT CHECKED` |
| **can the file ask for the depth?** | **no, by design** — a depth is a budget and budgets are not sentences in a program ([the boundary](the_boundary.md) § 4 corollary 2) |

So M1c's *"from then on `ein test` re-checks it on a machine with no external
solver installed at all"* is narrowed and widened in one paragraph, now written
into
[M1c § Splitting them did not split the pipeline](../../../docs/history/m1c_external_validation/README.md#splitting-them-did-not-split-the-pipeline):
the honest form of the promise is **`ein test <file> -m 38`**, a command
somebody runs; and the pipeline the sentence describes *does* run every commit,
on the 59 claims that `tests/stdlib/` states, in 0.04 s with no solver
installed.

**And [M10](../../m10_external_benchmarks/README.md) is told**, because the
pipeline is its claim as much as M1c's: its acceptance bullet now reads *"…at a
depth that reaches it"* — an encoding whose `:expect` cannot be checked at
`-m 5` is checked in with the `-m` that checks it, or it is not checked in.
`zebra2-minus-15` still carries **no** `:expect`, and under §4's policy that is
the correct state for it: 513 lines of expectation that come back `NOT CHECKED`
would be the debt written down rather than paid.

---

## 6. What was not weakened, and the hand-off to P1d.3's form

- **Nothing S1c.1.2 built moved.** Relation-closure inside a model is local,
  cheap and decidable by inspection; `(or …)` still compares as a set and still
  requires the counts to agree; `(false)`, `(model …)` and `(or …)` are still
  peers with the verdict implied. `expect.rs` is byte-identical.
- **`NOT CHECKED` stays distinguishable from both pass and fail** — on stdout as
  a third label, in `ein test`'s tally as a third counter, in `Came`'s dominance
  order, and now on stderr as its own line.
- **The P1d.3 hand-off, and it is short.** [S1d.3.3](../p1d.3_model_sets/the_verdict.md)
  shipped `--models key` as *additional output* rather than as a change to what
  is recorded, so there is no compact form for `:expect` to compare *against*:
  the key is a rendering of a model set the engine already has, and a claim is
  still compared model by model. The phase README's third question — *does
  P1d.3's answer change the shape of the claim?* — therefore closes **no**, and
  the cross-milestone edit back into M1c's form it anticipated is not needed.
  Should a future stage make the key a *recorded* artefact rather than a
  rendering, this is the paragraph that has to be revisited.
