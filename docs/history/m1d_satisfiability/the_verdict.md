# The verdict — enumerate *and* describe, and the qualifier that makes either honest

**Stage:** [S1d.3.3](README.md#s1d33--the-verdict) · **Phase:** [P1d.3](README.md)
**Decided:** 2026-08-26, by the user, on
[S1d.3.2](README.md#s1d32--representations)'s pricing.
**What ships:** a rendering rule on the `Ambiguity` verdict, and `ein solve
--models key`. No engine change: the search, the counters, `verdict.solutions`,
`--json-summary`, `--events` and `:expect` are byte-identical.
**Re-take:** `cargo test … -p ein-cli --test model_set_report` (6 tests, 0.9 s)
and `-p ein-render --test presentation_semantics`.

[Q-M1d.5](open_questions.md#q-m1d5--print-or-describe) asked whether 32
models should be **printed or described**. The answer is *both* — and the half
that was missing turned out to be neither: the engine was printing a model
count it had no right to state.

---

## 1. Q-M1d.5, answered

| | | |
|---|---|---|
| **the count** | `ein solve` enumerates, and now **says so** | an `Ambiguity` reporting `k` carries the exhaustion qualifier a `Solution` has carried since ein.py |
| **the description** | **(b), the determining key**, behind `--models key` | S1d.3.2's recommendation, shipped as *additional output* — 49 lines against 516 on the phase's own case |
| **(a) the envelope** | **does not ship** | the readability veto: it cannot say how many models there are and the arithmetic it invites over-states by 3.11 × 10¹² ([representations §6](representations.md)) |
| **(c) the diagram** | **does not ship** | 355 nodes for 32 models; a diagram wins when *k* is exponential in *n* and here *k* is 32 |
| **(d) the disjunctive store** | **deferred**, three-clause trip-wire | §5 |

The question's own constraint was the test of the answer:

> What is not legitimate is a compact form that only the engine can read.

(b) passes it on the strongest available evidence and (a) fails it, which is
the reversal S1d.3.2 measured. What §2 adds is that the *enumeration* was
failing a weaker test than either: it was not compact, and it was not honest.

---

## 2. The rule (T1d.3.3.2), and the defect writing it down found

The rule, in one table:

| the search | what a report of the model set may claim |
|---|---|
| `exhausted = true` | *these are the models* |
| `exhausted = false` | *these are models **found*** — a further model may exist and may contradict anything derived from these |

### 2.1 The engine did not follow it, and the corpus is full of the case

Before this stage, `ein solve -e examples/saturation/type-exclusivity/colors.ein`
printed:

```
  solutions (k)   5
  verdict         Ambiguous — distinct complete models; the puzzle is under-determined
```

**The file has nine models.** `-m 6` finds them and exhausts; the default cap
stops at depth 5 with a non-empty frontier. Nothing on that surface said so:
`exhausted` is printed by `--stats` and by nothing else, so the reader of a
count had no way to know it was a lower bound.

**Ten corpus entries answer `Ambiguity` under their declared runs, and five of
them do it with `exhausted = false`** — `branching/02`, `06`, `08` and the two
two-object `type-exclusivity` demos.

And the asymmetry was the wrong way round. `Solution` has always qualified
itself — *"(not certified — pass --exhaustive)"* — where the claim it is
hedging is a guess about **uniqueness**. `Ambiguity` qualified nothing, where
the claim is a **number**, and a number short by four is wrong in a way a
missing uniqueness proof is not.

### 2.2 After

```
  solutions (k)   5   (a lower bound — the search did not exhaust)
  verdict         Ambiguous — distinct complete models found; the puzzle is under-determined
```

Two marks, deliberately, because a reader who scans one line must not have to
have read the other; and the rest of the sentence is unchanged because it stays
true either way — five models found is under-determined however deep the search
went.

### 2.3 Three surfaces, because the claim is written in three places

| surface | before | after (`exhausted = false`) |
|---|---|---|
| `render_solution_table` — what `ein solve` prints | `solutions (k) 5` · `… distinct complete models;` | `… 5   (a lower bound …)` · `… models found;` |
| `render_answer` — the one-line headline | `Ambiguous — 5 distinct complete models; the puzzle is under-determined.` | `Ambiguous — at least 5 distinct complete models; the search did not exhaust the lattice.` |
| `trace::linearize` — the `--trace` summary | `Ambiguous — 22 models (showing one).` | `Ambiguous — at least 22 models (showing one); the search did not exhaust.` |

The third is the one no corpus digest covers — all five `Ambiguity` entries
exhaust at the shape sweep's depth, so `trace[no-proof]` renders the other
branch — and it has a unit fixture for exactly that reason.

**`Solution`'s existing hint was left alone, and checked rather than assumed.**
Its wording tells the reader to *pass `--exhaustive`*, which would be wrong
advice to someone who already did. Across the corpus's declared runs, all 29
`Solution` verdicts with `exhausted = false` come from plain `solve`, where
`stop_after = 1` cut the search and the advice is exactly right; under `-e` a
`Solution` is always exhausted. The hint says the true thing in every case it
is printed.

### 2.4 The fixtures

The acceptance asks for one per row, the `exhausted = false` one on a real
multi-model entry. There are three, on three entries:

| fixture | row | entry |
|---|---|---|
| `an_exhausted_search_reports_the_models` | true | `type-exclusivity/colors.ein -e -m 6`, k = 9 |
| `an_unexhausted_search_reports_models_found` | false | the same file at the default cap, k = 5 — and it re-solves at `-m 6` in the same test, so the fixture asserts the count really is short rather than only the wording |
| `an_unexhausted_ambiguity_says_the_count_is_a_lower_bound` | both | `branching/02_one_dead_one_alive.ein` at depth 3, over the headline and the trace summary |

### 2.5 The golden diff, cell by cell

`corpus_shapes.md5` is 8 171 renderings. **Five moved**, and the accounting is
the acceptance evidence:

```
~ examples/branching/04_two_levels.ein::trace[answer]                970L → 970L
~ examples/branching/12_typed_blind_solve.ein::trace[answer]         533L → 533L
~ examples/features/11_expect_ambiguity.ein::trace[answer]           215L → 215L
~ examples/lattice/01_subset_pruned.ein::trace[answer]               192L → 192L
~ examples/lattice/02_genuine_3set_death.ein::trace[answer]          117L → 117L
```

Every one is a `trace[answer]`; every one is an entry that answers `Ambiguity`;
**every line count is unchanged**, because the change is a suffix on one line
and a word on another. No `::solve`, `::lattice`, `::hyp`, `::load` or `::naf`
row moved, which is the same claim as *"the engine did not change"* stated in
the currency the gate keeps.

---

## 3. The key (T1d.3.3.4) — what `--models key` prints

On the phase's own case, `examples/zebra2-minus-15.ein -e -m 3`, whole:

```
  determining key — 4 of 23 varying slots
    22 4-sets determine the model; this one's domains allow fewest
    combinations — 320, of which 32 occur. Every one of the 22 contains
    pet-loc:Horse, pet-loc:Zebra.

    color-loc:Red  nation-loc:Japanese  pet-loc:Horse  pet-loc:Zebra
    -------------  -------------------  -------------  -------------
    House-2        House-3              House-2        House-1
    House-2        House-4              House-2        House-1
    …  (32 rows)
    House-5        House-4              House-1        House-4

    32 rows, one per model found. Add a row's facts to the program and it
    re-solves to that model; the other 19 varying slots follow. A lower bound:
    the search did not exhaust the lattice, so a further model would add a row
    — or share one, which would mean this key is too small.
```

The last paragraph is §2's rule applied to a *description*, and the word it
takes back is **alone**. With the lattice exhausted the sentence reads *"it
re-solves to that model alone"*, and S1d.3.2 verified exactly that on all 32
rows — `k = 1`, `exhausted = true`, the fact set identical, 30 of them without
entering a commitment. Without the proof the word is not available: an unfound
model may **share** a key row, and then the key separates the models *found*
rather than the models.

That is also where the two forms part company, and it decides the phase
acceptance's third bullet:

- **A key row can never be falsified.** Its four values are read off a model
  that exists. A 33rd model can **add** a row or **share** one — the table's
  completeness claim and the key's sufficiency, both of which are printed
  claims and both of which the caveat withdraws.
- **(a)'s core can be.** It is computed by intersecting the models found, and
  intersecting a subset gives a **superset** of the truth, so a 33rd model can
  contradict any of its 312 printed facts.

**(b) fails in its margins; (a) fails in its cells.** Under `exhausted = false`
that is the whole argument, and it is the one S1d.3.2 could not make because it
was pricing forms rather than stating guarantees.

*And the same file prints the other row too* — §3.4 is `zebra2-minus-15` at a
depth that exhausts, where the caveat is gone and the word **alone** comes
back.

### 3.1 It is the census's table, row for row — on every model set the corpus has

The claim that mattered most and is cheapest to check. The shipped Rust form
and [`utils/model_set_census.py --form key`](../../../utils/model_set_census.py)
are **two independent implementations** of the decision-variable rules and the
minimum-hitting-set search, and this is what they were diffed on rather than
eyeballed:

| | |
|---|---|
| **the exact table** | `zebra2-minus-15 -e -m 3`: **all 32 rows identical**, the same 4 columns, the same **22** minimum keys, the same **320** allowed combinations, the same two forced columns |
| **every model set the corpus has** | each of the **11** other entries re-run at the census's own cap: **11 of 11 agree** on `k` *and* on the minimum key size — 2, 1, 8 (declined by budget), 4, 1, 1, 1, 2, 4, 4, 6 |

That is the closest thing the repo still has to an oracle diff. It is not a
gate — the census is a Python script and the engine may not depend on it — but
it is why the key sizes in this record and in
[`model_set_census.md`](model_set_census.md) are one measurement rather than
two.

### 3.2 What it costs and what it saves

| | list | key |
|---|---:|---:|
| `zebra2-minus-15 -e -m 3`, stdout | **516 lines · 17 547 B** | **49 lines · 3 060 B** |
| the key block alone | — | 44 lines · 2 669 B · **78 cols** |
| **the key's own share of the wall** | — | **under 1 %** of a 25 s solve |

The 2 669 B is 163 B over the census's 2 506 because the shipped form spends
its extra bytes on the guarantee sentence, which is the one thing a record
printed inside a document did not have to carry.

### 3.3 The budget, and a fallback that is the enumeration

A minimum determining set is a **minimum hitting set**, which is NP-hard, and
the corpus holds the entry that proves it matters.
`examples/branching/06_lookahead_on.ein` has 42 varying slots and 22 models;
its minimum key is **8**, and `C(42, 8) = 118 030 185` candidate keys is over
the table budget, so the count that answers *"why these"* cannot be taken:

```
  determining key — none within budget
    the smallest determining set is 8 of 42 varying slots, and C(42, 8) =
    118 030 185 candidates is over the budget, so the models are printed
    instead.

  model 1/22
  …
```

**The fallback is (e).** Enumeration was a legitimate winner of the pricing all
along, so declining is a first-class answer rather than an error — exit 0, all
22 models. What finding that out costs is **42 ms** (0.196 s bare against
0.239 s), nearly all of it proving that no 7-set works.

That number is small and it is not the reason for the budget. The reason is the
**shape** of the problem: the search is a minimum hitting set, and the same
search in [the census's Python](../../../utils/model_set_census.py) takes
**12.4 s** on this entry — 300× — because a per-node scan over `pairs ×
variables` is what the obvious implementation does. The shipped one precomputes
the branch table once (which pair is hardest does not depend on the path, so the
candidate list per pair is static), which takes a node from `O(pairs ×
variables)` to `O(pairs)` and is what lets a budget counted in *nodes* mean
anything in seconds. What no implementation changes is that `C(42, 8)` is
118 030 185, and an entry with fifty slots would be worse.

Two guards rather than one, because they answer different questions:

| guard | value | what trips it |
|---|---:|---|
| `KEY_TABLE_BUDGET` — `C(varying, size)` | 4 000 000 | `branching/06`, at 118 030 185 |
| `KEY_NODE_BUDGET` — recursion nodes | 2 000 000 | **nothing in the corpus** |

The node budget is a backstop, and saying so is the honest form: it is there so
that a future entry reports a declined key instead of hanging, and no corpus
entry exercises it today. (The Python census's own budget carries the same
caveat, for the same reason.)

**`list` stays the default** because it is what `ein solve` has always printed
and a default change would move every reader's output, not because the key is
expensive — on the phase's own case it is under 1 % of the solve.

### 3.4 And on the phase's own case the caveat is dischargeable — 2026-08-26

Everything above hedges `zebra2-minus-15` because `solve -e` had never proved
32 was all of them: the depth-5 frontier is non-empty, so the cap stopped the
search and not the lattice ([layer census
§4](layer_census.md#4-zebra2-minus-15-all-five-layers)).
**A deep run taken the same day proves it.** The user ran
`examples/zebra2-minus-15-obligations.ein` at `-m 38`; re-taken here with
`-s` so the claim is asserted rather than inferred:

```
ein solve examples/zebra2-minus-15-obligations.ein -e -j16 -m 38 -s --models key

  solutions (k)   32
  verdict         Ambiguous — distinct complete models; the puzzle is under-determined
  exhausted        true
  enterings        17204592 (alive=17185463 dead_pre=0 dead_post=19129)
  layers_explored  22
  wall             1495695.3 ms
```

| | `-m 5` (`solve -e`) | **`-m 38`** |
|---|---:|---:|
| enterings | 618 076 | **17 204 592** — 27.8× |
| layers explored | 5, cap reached | **22, frontier empty** |
| wall | 416 s | **1 496 s** (24 min 56 s, `-j16`) |
| `exhausted` | false | **true** |
| `k` | 32 | **32** |

**The cap was 38 and the search stopped at 22**, which is the whole claim: the
frontier went empty, so it is the *lattice* that ended and not the budget. `k`
does not move — every model was found at depth 3 and everything after is proof
— which is the milestone's opening measurement carried to its end.

Two things it settles for P1d.3, and one it does not.

- **The rendering rule's two rows are now exercised on the same file.** At
  `-m 3` the key table ends *"32 rows, one per model **found** … A lower bound:
  the search did not exhaust"*; at `-m 38` it ends *"32 rows, one per model.
  Add a row's facts to the program and it re-solves to that model **alone**"*.
  Same puzzle, same 32 rows, different guarantee — and the word that moves is
  the one §3 said would.
- **The phase acceptance's third bullet is met rather than merely respected.**
  A compact description claims completeness only when the search proved it; on
  this puzzle the search now has, at a price of 25 minutes and 17.2 M
  enterings.
- **It does not make the caveat unnecessary.** `colors.ein` at the default cap
  still says 5 for a nine-model file, `branching/02`, `06` and `08` are still
  unexhausted at their declared runs, and nothing about a 25-minute proof is
  available to a reader who did not run one. The rule is for the ordinary case,
  and the ordinary case is `exhausted = false`.

**And the deaths are the number to keep.** The same file at `-m 5` reproduces
the layer census's counters exactly — 618 076 enterings, `dead_post` **19 121**,
`dead_pre` 0 — and at `-m 38` `dead_post` is **19 129**. So layers 6 through 22,
**16 586 516 enterings**, produce **eight** deaths, and `dead_pre` stays 0
throughout: not one candidate dropped by the no-good store before entering.
That is
[S1d.10.1](README.md#s1d101--why-it-does-not-finish)'s
*a layer that kills nothing learns nothing* at full scale, and it belongs to
[P1d.10](README.md#p1d10--exhaustive-search), where the run is recorded.

### 3.5 Additional output, never a replacement — checked three ways

1. **The summaries are byte-identical.** `lattice/02_genuine_3set_death.ein -e`
   run with and without `--models key`, each writing `--json-summary`: the
   files compare equal, so `verdict.solutions` and every counter are untouched.
2. **No shape golden moved for it.** The corpus sweep renders `ModelsForm::List`,
   which is why §2.5's five rows are the *whole* diff — the key form contributes
   nothing to any digest.
3. **The flag is inert where there is no model set.** `Solution`, `Open`,
   `Contradiction` and `Aborted` ignore it, asserted on
   `branching/05_mini_zebra.ein` and `features/12_expect_false.ein` by
   comparing stdout whole.

**`:expect` did not grow a word**, for the reason it did not grow one for
`Open`: its three forms are assertions about **facts**, and *"these are all the
models"* is a claim about a model **set**, which is
[P1d.4](README.md#p1d4--closing-the-model-set)'s subject. `ein test` never
reaches this rendering at all — it reports pass/fail, not a verdict table — so
the flag has no effect there either.

It is deliberately **not** in `corpus.toml`'s `runs`. The corpus prices its
runs — `cost_ms` is a measured claim checked in both directions — and putting a
hitting-set search into the cost of `branching/06`, whose recorded 377 ms it
would more than double, would trade a real measurement for a redundant one.
The surface is owned by `ein-cli/tests/model_set_report.rs`, which is where a
CLI behaviour claim lives.

---

## 4. The semantics (T1d.3.3.3) — what a reported model set is a set *of*

[`ideas.md`](ideas.md) § *Когда fixed point является решением*:

> Но здесь возникает важный вопрос: обязательно ли назначать значение каждому
> возможному факту?
>
> Если действует closed-world completion, все оставшиеся `open` считаются
> отсутствующими. Тогда получаем одну полную модель. Если open-world semantics,
> насыщенный граф может представлять сразу семейство моделей.

**Ein reports neither, and this is the statement of what it does report.**

> **A reported model is a saturated *state*, not an interpretation, and `k`
> counts states.** What `solve` records at a solution node is a KB: a set of
> believed positive facts, a set of believed negatives, and — for every other
> atom — no belief at all. That is [`ideas.md`](ideas.md)'s own three-valued
> `present` / `forbidden` / `open`, and `k` is the number of distinct such
> states, deduped by `state_key`. It is **not** the number of two-valued
> completions, and the engine does not claim it is.

Three consequences, each a number rather than a position:

- **Under closed-world completion each recorded state is exactly one
  interpretation**, and `k` is the model count in the textbook sense.
- **Under an open-world reading each state stands for 2ⁿ**, where *n* is its
  **leftover-open count** — and that number is measured, not hypothetical:
  `zebra2`'s *unique* model leaves **3 678** facts the blind enumerator would
  still propose, so open-world it is 2³⁶⁷⁸ interpretations
  ([census §6](model_set_census.md)).
- **The gap is reportable today.** `EIN_LEFTOVER=1` fills `--json-summary`'s
  `leftover` block with exactly *n*, per recorded state, attributed by
  relation. `leftover = 0` is where the two readings **agree**, and
  `branching/12_typed_blind_solve.ein` is the corpus's example: it closes its
  one relation, so at each of its two models every candidate is already a fact
  or already a stored negative.

### 4.1 Closure is per relation, opt-in, and the language already has it twice

The question reads as one global switch and the engine does not have one. What
it has is a program saying which of *its* relations it means to close:

- **`:expect` — *naming a relation closes it***
  ([grammar § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query)),
  M1c S1c.1.2. If an expectation mentions `pet-loc` at all, the listed
  `pet-loc` facts are that relation's **complete positive extent** in the
  model, and relations it never mentions are unconstrained. Closure is scoped
  to the positives on purpose: *stored negatives are not closed*, so the
  device is "this relation is decided", not "everything is decided".
- **`(__closed__ R)` — the generator may not speculate an `R`-fact**
  ([`domain_contract.md` §3](domain_contract.md), M1d
  S1d.2.2). The same closure said to the *search* rather than to a claim,
  and `std.closure`'s `infer-closure` derives it from `functional ∧ total`.

So the honest answer to *"which semantics"* is: **the one the program asks
for, per relation, and open everywhere it does not ask.** A compact form is
therefore a claim about a family of graphs whose size the program itself sets,
and `leftover` is how big the unasked-for part is.

### 4.2 The two places the answer would change, and neither changes

| | what turns on it | after this stage |
|---|---|---|
| **the closed-and-owing corner** | may *owes-and-cannot-pay* be promoted from `Open` to `(false)`? | **no.** The user's decision of 2026-08-25 stands: `(false)` is a *derived* refutation, and closed-world completion is the inference that would license the promotion. Deferred, with the trip-wire in §5 |
| **the leftover-open count** | is a model with *n* leftover atoms one model or 2ⁿ? | **the engine does not say, and now says that it does not.** `EIN_LEFTOVER` stays a probe, off by default, for the cost reason S1d.3.1 recorded |

### 4.3 The fixture that would break if it changed

`ein-cli/tests/leftover_probe.rs` § 3, unchanged by this stage and now
load-bearing for it: it asserts that `zebra2`'s **single** model leaves more
than a thousand atoms undecided *and is still reported as a model*. Adopting
closed-world completion would leave that assertion true and its meaning
different; adopting an open-world reading of `k` would make it false, because
`k` would not be 1. It is the one test in the tree whose subject is the gap
between the two readings.

---

## 5. What did not ship, and what would reverse it

**(a) the envelope.** Priced free and lossy: 78.2 % of a model, an
over-approximation by 3.11 × 10¹², and a "certain core" of 340 facts that is
two facts of answer and 338 of scaffolding. It stays in
`utils/model_set_census.py --form envelope`. *Trip-wire:* a corpus entry whose
core is mostly *derived* rather than declared — where "these facts hold in
every model" would be a finding rather than an echo of the input.

**(c) the decision diagram.** 355 nodes and 385 edges for 32 models under the
best of five orders, bounded in [24, 737] under every order. *Trip-wire:* a
corpus entry with `k > 10 000` **and** a variable order whose separators are
small — the census's 17-of-23 separator says `zebra2-minus-15` fails the second
clause even if it ever met the first.

**(d) the disjunctive constraint store.** Deferred in the milestone's form,
with [representations §8](representations.md)'s three clauses: a corpus entry
whose model set is **finite**, **too large to enumerate or print**, and **the
question** rather than a by-product. No entry trips it, and the two near
misses fail different clauses. `saturation/type-exclusivity/pets.ein` is the
one to watch, because its *k* grows with the fixture — 9 models at two
objects, 35 at three.

**Closed-world completion.** Deferred with §4's statement as the specification
that survives it. *Trip-wire:* a program that states an obligation, reaches
quiescence owing something no candidate can pay, and wants `(false)` rather
than `Open` — at which point the promotion needs an *inference* that licenses
it, and that inference is the completion. `tests/stdlib/closure/03_closed_and_owing`
is the fixture already banked against that day.

---

## 6. What S1d.3.3 hands forward

**`Contradiction` is where the unqualified count now lives, and one case is
much worse than the ten M1a found.** Three corpus entries answer
`Contradiction` with `exhausted = false` under their declared runs; on
`saturation/type-exclusivity/pets.ein` the word is not merely unproven, it is
**wrong**:

| `-m` | verdict | k |
|---:|---|---:|
| 5 (the default) | `No solution — the constraints are contradictory` | 0 |
| 6, 7, 8 | the same | 0 |
| **10** | `Ambiguous` | **35** |

The ten entries [Q-M1d.6](open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
was opened for had *no* models at any depth — the cap wearing a refutation's
word, but a word that happened to be true. This one has thirty-five. That is
[Q-M1d.1](open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
and [P1d.10](README.md#p1d10--exhaustive-search)'s, by the split the
milestone README already made, and it is a sharper fixture than anything that
question currently has.

**`Open` reports a state count and no corpus entry reaches it unexhausted.**
All twelve `Open` verdicts are `exhausted = true`, so the rule's second row has
no case there. It is structurally reachable — a depth cap can truncate a run
whose recorded states all owe — and if an entry ever produces one, `open states
n` needs §2's treatment for §2's reason. Recorded rather than pre-emptively
rendered, because a qualifier no entry exercises is a qualifier nothing checks.

**The key is a post-processing pass, not a way to avoid the search.** Every
form that survived the pricing reads `verdict.solutions`, so *"model sets
without enumeration"* — the phase's title — is answered **no** by all of them;
only (d) would be a yes, and it is deferred. What `--models key` buys is a
**5.7× smaller printout of an enumeration already paid for**, and the honest
place to say so is here rather than in the flag's help.

**And what the enumeration costs is now measured to its end.** §3.4's `-m 38`
run is the phase's own case exhausted: 17 204 592 enterings for the 32 models
48 745 already found, 0.11 % of them dying, `dead_pre = 0`. The compact form
saves 467 printed lines; it saves nothing of the 16.6 M enterings that produced
the guarantee those lines carry. That gap is
[P1d.10](README.md#p1d10--exhaustive-search)'s whole subject, and this stage
leaves it exactly where it found it — with one more number on it.
