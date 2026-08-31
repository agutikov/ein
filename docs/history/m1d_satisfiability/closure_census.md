# The closure census — who claims a model set, and what claiming one costs

**Stage:** [S1d.4.1](README.md#s1d41--what-closure-costs) · **Phase:** [P1d.4](README.md)
**Taken:** 2026-08-26, over the three corpus roots (197 files, 124 queries) and
all 197 `corpus.toml` entries.
**Instrument:** [`utils/closure_census.py`](../../../utils/closure_census.py),
reading **`ein test --json-report`** — the read-out this stage added, one row
per `(query …)` of a selection — and `solve -e --json-summary` for the
counterfactual.
**Re-take:** `utils/closure_census.py --long --json c.json` — **2 min 55 s**,
of which 2 min 51 s is the two `zebra2-minus-15` entries: 60 s of budget each,
then 26 s each at the `-m 3` the ladder drops to.
**Machine:** i9-14900HX, governor `powersave`, turbo on, `git fadc520` plus
this stage's tree — the read-out is part of what is measured
(`utils/bench_env.sh`).

The fourth census after
[`layer_census.md`](layer_census.md),
[`openness_census.md`](openness_census.md) and
[`model_set_census.md`](model_set_census.md), and the first
whose subject is the **claim**: not the search, not the program, not the
answer, but the sentence a file writes about its own answer.

---

## 0. The reconnaissance was wrong, and how it was wrong is the method

[T1d.4.1.1](README.md#s1d41--what-closure-costs) asked for a census *parsed from the
loaded program rather than grepped*, "because `:expect` is a query keyword and
a grep cannot tell a keyword from a comment about one", and added — in
parentheses — *"this file's own reconnaissance grepped, and says so"*.

It was right to. Parsed, the corpus's closure claims are **not two but one**:

| shape | grepped, 2026-08-25 | parsed, of the programs that load |
|---|---:|---:|
| `(model …)` | 40 | **38** |
| `(false)` | 20 | 20 |
| **`(or …)`** | **2** | **1** |
| files / claims | 62 | **59** |

`examples/features/10_expect.ein` is the file that moved. Its `:expect` is a
`(model …)`; the `(or …)` a grep finds in it is **line 12 of its header
comment**, documenting the form. So the phase's subject — *the model set is
exactly these k* — is written **once in the whole repository**, and the one
instance is `examples/features/11_expect_ambiguity.ein`: two models, four
facts, `exhausted = true`, checked in 0.11 ms.

The mistake and its correction are both pinned by
`ein-cli/tests/test_cli.rs::the_shape_comes_from_the_program_and_not_from_the_text`.

**The other three are the other half of the same correction**, and they go the
other way: `expect_is_a_pattern`, `expect_omits_the_goal` and
`expect_unknown_relation` are `examples/broken/load/` fixtures that carry the
token, are all `(model …)` in shape, and **never become programs**. So the
grep's 62 is 59 claims plus three refusals, and only the 59 are counted here.
Putting the loader's own negatives in the numerator of *"what fraction of the
corpus claims a model set"* would be counting the wrong thing twice.

---

## 1. Who states a claim — T1d.4.1.1

One row per `(query …)`, from one `ein test examples tests stdlib
--json-report` in **0.04 s**.

| root | files | no query | refused | queries | claims | `model` | `or` | `false` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `examples/` | 134 | 25 | 41 | 68 | **3** | 1 | **1** | 1 |
| `tests/` | 56 | 0 | 0 | 56 | **56** | 37 | 0 | 19 |
| `stdlib/` | 7 | 7 | 0 | 0 | 0 | 0 | 0 | 0 |
| **total** | **197** | **32** | **41** | **124** | **59** | **38** | **1** | **20** |

**The denominator is the point.** 59 of 124 queries state a claim about their
own answer — 48 % — and **1 of 124** states a claim about a model *set*. The
usage is not thin at the edges; it is concentrated: 56 of the 59 are
`tests/stdlib/`, where an `:expect` is the fixture's whole purpose, and the
three under `examples/` are the three files M1c wrote to demonstrate the form.

`stdlib/` contributes seven files and zero queries, which is the stdlib being
a library. `examples/` contributes 41 refusals, which is `broken/` being
broken.

---

## 2. Is the claim checkable today — T1d.4.1.2

Under `ein test`'s regime: exhausting, `-m 5`, one job. Every one of the 59
gets a row (`--long`); the roll-up:

| outcome | claims | |
|---|---:|---|
| **held** | **59** | |
| FAILED | 0 | |
| **NOT CHECKED** | **0** | ← the column the phase is about |
| ERROR | 0 | |

and `exhausted = true` on **59 of 59**. So the reconnaissance's prediction
holds exactly: *the answer is most of them*, and it is in fact all of them.

**Which is not the same as "closure is affordable", and §4 is why.** The
column is empty because the entries whose claim would not be checkable have
never written one — not because writing one would have worked.

Two details of the 59 worth having:

- **Twelve claims hold against a verdict whose `k` is 0.** They are the twelve
  [S1d.2.6](README.md#s1d26--verdicts-counters-corpus) moved to
  `Open`, and their `(model …)` claims are unchanged, because all three
  `:expect` forms are assertions about **facts** and the facts an open state
  reached are the facts it reached. The report carries `verdict`, `k` and
  `solution_nodes` separately, so the split is visible in a row rather than
  inferable from a word.
- **Fifty-eight of the 59 enter no commitment at all**, and the fifty-ninth
  (`features/11_expect_ambiguity`) enters ten across two layers. The corpus's
  claims are checked by *saturation*; the one claim that needs a search is the
  one claim about a set.

---

## 3. What a claim would cost to **write** — T1d.4.1.3

The phase README names one cost, the verify cost. This is the other, and on
the entries that motivate the phase it is the one nobody had counted.

*Naming a relation closes it*, so an `:expect` must list the complete extent of
every relation its `:goal` asks about, in **every** model. The arithmetic is
`facts = Σ_models |{positive facts of the goal's relations}|`.

**`facts` is the measurement; `lines` is `facts + k + 1` and is a
convention** — one fact per line, one `(model` per disjunct, one `:expect (or`
to open. The corpus's one `(or …)` packs two facts per line, because two fit;
at fifteen facts per model nothing fits, so the convention is the readable
rendering rather than the observed one. Every ratio below moves with it and
none of the orderings do.

| entry | *k* | facts/model | facts | lines | file | ×file | exhausted |
|---|---:|---:|---:|---:|---:|---:|---|
| `zebra2-minus-15` | 32 | 15 | 480 | **513** | 534 | **0.96** | ✘ |
| `zebra2-minus-15-obligations` | 32 | 15 | 480 | 513 | 539 | 0.95 | ✘ |
| **`branching/06_lookahead_on`** | 22 | — | 384 | **407** | **95** | **4.28** | ✘ |
| `branching/08_hypothesis_relation_whitelist` | 8 | — | 100 | 109 | 73 | 1.49 | ✘ |
| `branching/02_one_dead_one_alive` | 4 | — | 36 | 41 | 64 | 0.64 | ✘ |
| `branching/04_two_levels` | 2 | 6 | 12 | 15 | 81 | 0.19 | ✔ |
| `saturation/type-exclusivity/colors` | 5 | — | 8 | 14 | 35 | 0.40 | ✘ |
| `saturation/type-exclusivity/nationalities` | 5 | — | 8 | 14 | 36 | 0.39 | ✘ |
| `branching/12_typed_blind_solve` | 2 | 3 | 6 | 9 | 168 | 0.05 | ✔ |
| `lattice/01_subset_pruned` | 2 | 3 | 6 | 9 | 82 | 0.11 | ✔ |
| `features/11_expect_ambiguity` | 2 | 2 | 4 | 7 | 58 | 0.12 | ✔ |
| `lattice/02_genuine_3set_death` | 3 | — | 2 | 6 | 72 | 0.08 | ✔ |

Twelve entries have a model set at this depth; the other 71 that answer with
models have exactly one, and their claim is a `(model …)` costing **5.5 facts**
on average. **Every ✘ row is a floor**, because a capped model set is a subset
and the facts it would take to list one are therefore a lower bound —
S1d.3.3's rule about what a *count* may claim, applied to what a *claim* would
cost.

**The phase README's "roughly double the file" is right about the zebra and
wrong about the corpus.** 513 lines on 534 is 0.96× — the *mildest* ratio of
any multi-model entry above 4 models. The worst is `branching/06_lookahead_on`
at **4.28×**: 407 lines of expectation on a 95-line file. Widen the depth to
where the model sets are complete
([`model_set_census.md` §2](model_set_census.md)) and the
worst is worse still — `saturation/type-exclusivity/pets.ein` at `-m 10` has 35
models and 120 facts, **156 lines on a 36-line file, 4.33×**, and is *still*
not exhausted.

So the write cost is not a property of the puzzle's difficulty. It is
`k × |goal extent| / |file|`, and it is worst where a **small demo has a large
model set** — which is exactly where a compact representation would help and
exactly where P1d.3 measured that one is unavailable:
[`representations.md`](representations.md) prices
`branching/06`'s determining key out at `C(42, 8) = 118 030 185` candidate
keys, so `ein solve --models key` declines on it and **prints the models**.
**On the corpus's worst write-cost entry both forms fail, and they fail for
unrelated reasons.**

### 3b. The formula, checked against the claims that exist

A write-cost formula only ever applied where it cannot be checked is a formula
nobody has tested. Applied to the 38 `(model …)` claims that *do* exist and
compared with what those files list:

| | claims | |
|---|---:|---|
| predicted **==** listed positives | **17** | the claim names exactly the goal's relations |
| predicted **<** listed positives | 21 | the claim names more relations than the goal does |
| predicted **>** listed positives | **0** | — |

Never over-charges, which is the direction that matters: the counterfactual in
§3 is a **lower** bound on what someone would actually have to type, because a
real expectation tends to pin more than the goal demands.

### 3c. What it would cost to **verify** — borrowed, not re-taken

[P1d.10](README.md#p1d10--exhaustive-search)'s numbers, per
[T1d.4.1.3](README.md#s1d41--what-closure-costs):

| run | enterings | wall | `exhausted` |
|---|---:|---:|---|
| `solve -e` (depth 5, `-j1`) | 618 076 | 416 s | **false** |
| `solve -e -m 38 -j16` | 17 204 592 | 24 min 56 s | **true** |

**The zebra's closure claim is verifiable, and `ein test` still would not
verify it.** `ein test` runs at `-m 5` and `jobs: 1` — there is no `--jobs` on
that subcommand at all — so the run a claim would actually be checked by is the
first row, and the first row is `NOT CHECKED`. Passing `-m 38` to `ein test`
buys the second row's proof at the second row's price, single-threaded: at
depth 5's measured **673 µs per entering** the whole lattice extrapolates to
**≈ 3 h 13 m**, which is arithmetic on two measurements and is offered as
arithmetic, not as a measurement.

---

## 4. The counterfactual `NOT CHECKED` set

§2's empty column asks to be read as *"every claim in the corpus is
checkable"*. It is not that. It is *"no entry that would fail to check has
written a claim"* — and this is that set, taken under the same regime.

| | entries | |
|---|---:|---|
| `exhausted = true` | **111** | a closure claim here is checkable |
| **`exhausted = false`** | **8** | a closure claim here comes back **NOT CHECKED** |
| over the census's budget | 2 | this script stopped, not the runner — unmeasured here |
| no declared `solve` run | 29 | the manifest declines it — an open hypothesis space |
| no fixpoint | 47 | a load error, by design |
| **total** | **197** | |

121 entries reach a fixpoint, which is
[`openness_census.md`](openness_census.md)'s 121, and
**eight of them cannot certify a model set at the depth `ein test` runs at**:

| entry | verdict | *k* | wall |
|---|---|---:|---:|
| `branching/02_one_dead_one_alive` | Ambiguity | 4 | 0.02 s |
| `branching/06_lookahead_on` | Ambiguity | 22 | 0.21 s |
| `branching/07_lookahead_off` | Contradiction | 0 | 0.28 s |
| `branching/08_hypothesis_relation_whitelist` | Ambiguity | 8 | 0.04 s |
| `features/01_not_and_absent` | Contradiction | 0 | 1.74 s |
| `saturation/type-exclusivity/colors` | Ambiguity | 5 | 0.01 s |
| `saturation/type-exclusivity/nationalities` | Ambiguity | 5 | 0.01 s |
| **`saturation/type-exclusivity/pets`** | **Contradiction** | **0** | 0.03 s |

**The two `zebra2-minus-15` entries are the ninth and tenth, and this census
does not measure them.** Their regime run outlives the 60 s budget, so the
ladder drops to `-m 3` and finds all 32 models in 25.7 s — and a ladder row
proves nothing about the regime, because `exhausted = false` at `-m 3` is
consistent with `true` at `-m 5`. Only exhaustion travels upward. What settles
them is [P1d.10](layer_census.md), which measured
depth 5 directly: 618 076 enterings, 416 s, **`exhausted = false`**. So the
honest total is **ten**, eight of them measured here and two borrowed.

**Eight of the ten are cheap.** Seven cost under a third of a second and report
a `k` that is simply short — `colors` says 5 where the file has 9
([S1d.3.3](README.md#s1d33--the-verdict)'s finding, which is why
that count now prints its own qualifier); `features/01` is 1.7 s. The
affordability problem the phase is named after is one entry's, and its twin's.

**`pets.ein` is the sharp one, and it is where `NOT CHECKED` earns its
keep.** At `-m 5` it reports `Contradiction, k = 0`; at `-m 10` it has **35
models**. A closure claim written on that file would be compared against
**zero** models and would be *wrong* by 35 — and it comes back `NOT CHECKED`
rather than `FAILED`, because
[`expect.rs`](../../../ein.rs/crates/ein-infer/src/expect.rs) declines to
refute a claim on the strength of a search that stopped. The one thing standing
between the corpus and a false red on that file is `exhausted`. It is also
[Q-M1d.1](open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
fixture, which is the same fact from the other end.

---

## 5. The `NOT CHECKED` corpus gap, decided — T1d.4.1.4

The stage's framing was that the mechanism *"never fires on a corpus entry …
exercised only by `test_cli.rs` and `expect_semantics.rs` on constructed inputs
with a `-m` cap"*. Measured, that is one claim too many:

- **It does fire on a corpus file, in the gate, today.**
  `test_cli.rs::test_exhausts_where_solve_stops_at_one` runs plain `ein solve`
  on `examples/features/11_expect_ambiguity.ein` — a real fixture, **no
  constructed input and no `-m` cap** — and asserts exit 1 and `NOT CHECKED`.
  Plain `solve` stops at `-n 1`, which is all the truncation a claim about a
  *set* needs.
- **What it does not have is a manifest cell.** That entry's `runs` column
  deliberately omits plain `solve` (`corpus.toml`: *"**No plain `solve`**: it
  stops at the first model and reports k=1, which is correct and is not this
  fixture's question"*), so no cell of the 622-cell sweep produces the outcome.

So the gap is one line of manifest wide — and **it cannot be closed by writing
that line**, which is this task's finding. `ein solve` prints the `:expect`
verdict on **stdout** and exits 1 with an empty stderr, while
`corpus_cli.rs::every_refusal_carries_a_diagnostic` requires a non-zero exit to
say why on **stderr**. Declaring the cell turns a green sweep red on an
unrelated invariant.

**The decision: record it, and make the collision fail loudly when it is
fixed.** The four options and their prices:

| option | price |
|---|---|
| a fixture *expected* to be not-checked | a runner concept `ein test` does not have and has said it will not grow ("Not a test framework") |
| a synthetic `tests/` entry that hits the cap | `NOT CHECKED` takes exit 1, so `ein test tests/` goes red by design |
| write `zebra2-minus-15`'s claim and let it come back NOT CHECKED | 513 lines, and `ein test examples/` red by design |
| **declare `solve` on `11_expect_ambiguity`** | one word — and `every_refusal_carries_a_diagnostic` |

The fourth is the cheapest by an order of magnitude and is blocked on a
question this stage may not answer: is a false claim a **refusal**, which
belongs on stderr, or a **result**, which belongs under the solution table?
That is a surface decision about `:expect`, and
[S1d.4.3](README.md#s1d43--the-vocabulary) is the only stage in this phase licensed
to make one — S1d.4.1's own acceptance says *nothing about `:expect` changes in
this stage*.

What ships instead is a **trip-wire**:
`expect_cli.rs::not_checked_is_reachable_from_a_corpus_file_with_no_cap` pins
the corpus witness *and* asserts the empty stderr, with the failure message
naming the consequence. The day the diagnosis moves streams, that test goes red
and the manifest cell becomes available in the same breath.

---

## 6. What this leaves the rest of the phase

- **[S1d.4.2](README.md#s1d42--the-second-order-boundary)** — untouched by these
  numbers. Whether a *puzzle* may require its own model count is a language
  question, and it is asked of a language whose tests ask it once.
- **[S1d.4.3](README.md#s1d43--the-vocabulary)** — priced. A vocabulary for
  set-closure would be a vocabulary for **one** existing instance and, at
  today's depth cap, **ten** places where it could not be checked. Against
  that: the write cost is 4.3× the file at its worst and 0.05× at its
  cheapest, so any weaker form that says *at least these* buys nothing on
  size — the disjuncts are the size, not the guarantee word. Two decisions land
  in this stage rather than beside it: the stream a failing claim is reported
  on (§5), and whether M1c's pipeline sentence is rewritten given §3c — the
  claim *is* verifiable, at `-m 38`, and not by the run `ein test` performs.
- **The phase acceptance** — *"the `zebra2-minus-15` debt is discharged: either
  its 32 models are verifiable by something, or M1c's pipeline sentence is
  rewritten"* — now has a third reading the phase README did not have, because
  P1d.10 exhausted the lattice on 2026-08-26: the models are verifiable, the
  file still carries no claim, and what `ein test` would do with one is `NOT
  CHECKED`. Discharging it is a change to a default, a flag, or a sentence, and
  none of the three is a vocabulary.

---

## Re-taking it

```sh
utils/closure_census.py --long --json c.json      # everything above, 2 min 55 s
utils/closure_census.py --no-solve --long         # tables 1-2 only, 0.05 s
utils/closure_census.py -k zebra2                 # one entry's counterfactual
ein test examples tests stdlib --json-report r.json   # the transport, alone
```

The two tables that cost nothing are the two that answer T1d.4.1.1 and
T1d.4.1.2, so the usage census is re-takable inside a coffee break's coffee.
