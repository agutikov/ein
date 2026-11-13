# S1f.6.28 — Word problems, second pass: a reading is always chosen, and *what is not listed is zero*

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's replies of that day. **Running 2026-11-13:** T1 to T3 done
([the record](#record-2026-11-13)). It continues
[S1f.6.20](s1f.6.20_word_problems.md), whose file is that stage's record; the examples it changes are
[`examples/word_problems/`](../../../examples/word_problems/README.md), banked at `f3fe6ff0`.
**Depends on:** [S1f.6.26](s1f.6.26_what_root_owes.md), the read-out row for what root owes, which an owed
reading is read through; and E-33 resolved (T1 below). Independent of the stdlib layout; the layout move
rewrites its imports afterwards.

## The rulings

The user's replies of 2026-11-12 to the questions of [`questions.md`](questions.md) named:

**[28](questions.md#q28), how a missing quantity is detected:**

> see my answer in question 24; in general - need to discuss, analyze, needs a separate stage. But I want
> already written word problems banked as is, so pick recommended, and note that is is not only and not
> final.

**[29](questions.md#q29), how an assumption is written and reported:**

> again - for this stage (a) is OK, return back to investigations of this in M1h

**[31](questions.md#q31), a word with two readings:**

> reading always chosen, it can't be not chosen: result depends on formula, formula depends on selected
> arith operations, selection depend on reading. The synthesized program must be able answer question about
> the solution, like "why you divided those numbers? why not 3/12, not 3+12, not 12-3, and so on" btw there
> could be the case when answer is correct, but solution is not, right? And finally the solution could be
> correct, but reasoning not - this case is M2 and Ein theory of itself or theory of solutions - so out of
> the scope here.

**[32](questions.md#q32), is *Alice has 3 pears* also *Alice has no apples*:**

> what is not listed is zero - good idea, it could be a reusable universal assumption.

**[35](questions.md#q35), which further readings the theory should hold:**

> Take only needed right now. There are many words and they can have many different meaning in different
> contexts. We don't enumerate them all, at least now.

Round 2 turns 31 and 32 into builds: [item 31](questions_round2.md#q31), *the examples are rebuilt to
"always owed"*; and [item 32](questions_round2.md#q32), *built that way, with the apples-and-pears problem
in both forms*. The user's replies there, committed as `dc88520b`:

**Round 2, item 31:** *"yes"*.

**Round 2, item 32:**

> Yes, relation arguments are marked with some objects that can be treated as types. So can add a rule that
> assigns zero to every unknown and deduce (has Alice apples 0). It doesn't work in zebra for example - if
> nobody lives in the house means we yet don't know who lives there.

So the module is confirmed, with two things added by the reply: **how** (a relation's arguments are typed,
so a rule can range over *every person and every kind of thing* and assign zero where no amount is listed,
deriving `(has Alice apples 0)` outright); and **where it does not hold** (in the Zebra puzzle an empty
slot means *not known yet*, not *nobody*). The second is why it is a module a problem imports and not the
word theory's meaning: the assumption is adopted per problem, and per relation.

## What it changes

**1. A reading is always owed.** The banked `readings.ein` holds the two readings of *need* as hypotheses
with nothing owed (question 31's option (a), *never owed*). It becomes option (b), *always owed*: R holds
only the readings. The consequence round 2 names, and the stage banks as its headline:

> with 12 apples and boxes of 3 both readings answer 4, so R is **two states with one answer**: two
> solutions, one of which may be the wrong reasoning with the right number. The query's projection shows
> one class, 4; the states show the two readings. That is the honest output under your ruling, and it is
> the first corpus witness of *answer correct, solution not necessarily*.

So `boxes12.ein` becomes two states and one class, and its claim says both.

**2. The 13-apple variant comes back.** *13 apples, a box holds 3: how many boxes do they need?*, 4 if only
full boxes count and 5 if every apple is packed. It was left out of the banking because
`fact_grain_never_calls_a_violated_cube_monotone` (`ein-infer/tests/region.rs`) does not end on a program
where a supposed reading leads to a calculator request (E-33 of [`findings.md`](findings.md), Q-M1f.198).
Round 2: *the stage that rebuilds the readings has to resolve that first, without changing what the test
means.*

**3. `listed_is_all.ein`,** the assumption *what is not listed is zero* as a module of its own that a
problem imports when it adopts it, and that says which relation it is adopted for: with it, *Alice has 3 pears* gives *Alice has 0 apples*, and the answer
carries a `rests-on` fact naming the assumption (reply 29 (a)). It needs `absent` over `has`, so it is kept
apart from `words.ein`, which has none, as `gaps.ein` is. Without the import the amount stays a gap, as
banked. The apples-and-pears problem is banked in both forms.

**4. The README says what the banked forms are.** In the user's words: *"not only and not final"*.
`examples/word_problems/README.md` states, for the gap (`absent`, option (b) of 28), the assumption (a
marked fact, option (a) of 29), the order of events (named moments, [reply 26](questions.md#q26): *"pick
the simplest that works - leave what is implemented already"*) and the backwards plan (reply 27 (a)), that
each is the form picked to bank the problems and that the subject is
[M1h's](../../m1h_beyond_word_problems/README.md).

**5. No new word.** Reply 35: *together* over everyone and *gave all* are not added. A word is added only
where one of the problems above needs it, and the record names each.

## Tasks

1. **T1f.6.28.1 — E-33, resolved first.** The fact-grain prototype (`ein-infer/examples/shared/fact_grain.rs`)
   is a test-side static analysis that no route reads; its closure over *shapes* follows every circuit of
   `std.arith` once a supposed reading leads to a `value`. The task reads why it terminates only by its
   widening, and makes the test end on such a program **with its meaning unchanged**: the test still says
   that the fact grain never calls a violated cube monotone, over every corpus file, the two word-problem
   programs among them. Ways to look at, none chosen before reading: the widening applied to the
   continuation tags sooner; the closure's work shared between shapes it re-derives; a bound that makes the
   analysis answer *not monotone*, the safe side, where it gives up. **Not acceptable:** skipping the two
   programs, or any corpus program, by name or by size. If no change keeps the meaning and ends in the
   gate's time, the stage stops and shows the user, and the 13-apple variant stays out.
2. **T1f.6.28.2 — the readings rebuilt, shown before they are banked.** `readings.ein` owing a reading;
   `boxes12` and the 13-apple problem under `ein solve`, `--single`, `--minimal`, `--maximal`, `ein test`
   and `ein oracle`, with the read-out (S1f.6.26's row names the owed reading) and the trace of each state.
   Every answer is also computed apart from the engine, as the banking's were. The table is the record's
   first section.
3. **T1f.6.28.3 — *why divided, and not subtracted*:** what the trace of each state says of the choice of
   operation, as far as the rules go (*need* with *a box holds* means division), written down on the two
   programs; and what it does not say. Nothing is built for it: a theory of solutions is
   [M1h's P1h.4](../../m1h_beyond_word_problems/README.md#p1h4--the-formula-and-the-calculation) and M2's.
4. **T1f.6.28.4 — `listed_is_all.ein`,** written as the user's reply has it: over the typed arguments of
   `has`, a rule that derives a zero amount for every holder and kind with none listed, the derived fact
   marked as resting on the assumption; with one sentence in the module's header on the Zebra case, where
   the same rule would be wrong. And the two forms of the apples-and-pears problem. The record shows what the
   static check and *forbid* say of a program that imports it, since it reads `absent` over `has`; the
   problems state their lattice so that `has` is no hypothesis relation.
5. **T1f.6.28.5 — banked:** each new or changed program a corpus entry with its claim; `alive_invariant`'s
   list where a rule concludes a name; the goldens expected to move named before blessing
   (`corpus_exits.txt`, `corpus_shapes.md5`, and the changed programs' own).
   **And one witness, banked with today's answer and no fix** (added 2026-11-12, on the user's question of
   that day):

   > what is just simply change the order: t1 Bob gave Alice 2 apples; t2 Bob has 5 apples; How many apples
   > Bob had at t0 (before t1); would the same theory infer plus from gave reverted in time? Or even reverted
   > plus directly with reverse arithmetic?

   *Bob gave Alice 2 apples. Now Bob has 5. How many did Bob have before?* The giving is from `t0` to `t1`,
   the `has` is at `t1`, and the question asks the amount at `t0`. On the banked theory the question has no
   row and `gaps` names it, `not stated: (amount Bob apples t0)` (`ein solve` on a scratch copy of
   `words.ein` and `gaps.ein` with the three facts, 2026-11-12): the word rules read *gave* in one direction
   of time, *the amount after is the amount before less what was given*, and no rule plans from an earlier
   moment to a later one, so no request reaches the calculator. It is banked as `before.ein`, its claim the
   named gap, with a header that says the right answer is 7 and that the file is a witness: the theory reads
   a word as an instruction and not as a relation between three quantities. **Nothing is built for it
   here.** The two ways to answer it, a second set of rules for each word reverted in time, or the event
   stated once as a relation with whichever quantity is unknown asked for, are
   [M1h's P1h.1](../../m1h_beyond_word_problems/README.md#p1h1--temporal-and-causal-theories)'s opening
   question, and the second meets [M1g's P1g.9](../../m1g_logic/p1g.9_one_equation_three_methods/README.md).
   When either lands, the file's claim moves to 7 in that commit.
6. **T1f.6.28.6 — the README and the pages:** `examples/word_problems/README.md` as in 4 above;
   [`word_problems.md`](word_problems.md) gets a closing section that records the replies and points here,
   and is otherwise left as the investigation's page; `examples/README.md`; `AGENTS.md`; E-33 closed in
   [`findings.md`](findings.md), Q-M1f.198 marked answered.

## Acceptance

- Every problem with a word of two readings owes the reading: no state of R has an answer with no reading
  chosen.
- `boxes12` is two states and one projection class, held by its claim; the 13-apple problem is two states
  and two classes, 4 and 5, held by its claim.
- `fact_grain_never_calls_a_violated_cube_monotone` ends on the whole corpus, with the two programs in it,
  and says what it said.
- With `listed_is_all` imported the apples-and-pears problem answers and names the assumption it rests on;
  without it the amount is a gap.
- The README says the banked forms are *"not only and not final"*.
- `before.ein` is banked, claims the named gap, and says in its header that the answer is 7 and who owns
  it.

## What it must not do

- **Not move the theory into the stdlib** ([reply 34](questions.md#q34): *"examples for now - (a)"*).
- **Not add an expression layer** (reply 10: P1f.8, now M1g's P1g.10).
- **Not redesign the gap or the assumption** (replies 28 and 29: M1h), nor the order of events (reply 26:
  M1h's first phase).
- **Not change what the fact-grain test means.**
- **Not enumerate words** (reply 35).

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. A changed example is shown with its
computed answers before it is banked (T2). Config never changes an answer; every answer exits 0. The gate is
`./run_tests.sh`, then `./run_tests.sh --nightly` after the code commit (T1 changes a test's helper under
`ein.rs/`); `utils/ci_container.sh` is not needed.

## Record, 2026-11-13

Every run below is on the release binary or the test profile of the commit its paragraph names, on this machine.
The scratch programs are under `build/s1f.6.28/`: `p/` holds E-33's two witnesses and the probes of T1, `p2/` the
programs of T2 to T5 as they were run before they were banked.

### T1f.6.28.1 — E-33, resolved, done

**Why the closure did not end.** `fact_grain::classify` closes the candidates under the saturation plans over
*shapes*, a fact with some arguments *any*; a plan whose even-depth read unifies with a shape adds its conclusions,
with the variables that read bound and *any* for the rest; a shape is kept if no shape of its relation subsumes it;
nesting past four levels is widened to *any*. In the word theory a supposed reading concludes `(value (needed _ _
_) _)`, since the reading rule's other premise, `left-over`, is not the read that unified; `holds-means-divide`
reads `(value ?q ?a)` for any `?q` and concludes `(div _ _)`; and a division with an operand *any* sends the
closure through every shape of `std.arith`'s circuits, whose steps carry their operands and their continuation as
nested facts: `(sub-walk (div-trial (quotient _ _) (multiples (n z 0) (n (n z _) _) …) …) …)`. The widening bounds
the nesting and not the breadth, which is the product of what each argument of a tag can be: *any*, a digit, or a
number of up to four levels. A program of eleven lines shows it with the word theory gone, one hrule, one rule from
the supposed fact to `(div ?v (n z 3))` with `?v` unbound (`build/s1f.6.28/p/gives_up.ein`). On E-33's witness
(`build/s1f.6.28/p/boxes13_request_under_reading.ein`, the closure of `fact_grain.rs` at `fc92f075` with a progress
line, release build, `build/s1f.6.28/t1-probe-long.err`): the closure had not ended when it was stopped after 1 312
s, at 137 731 shapes and 141 743 steps, its work list still 384 long and still yielding new shapes: 9 403 shapes at
2 s, 60 547 at 200 s, 97 656 at 603 s, 122 218 at 1 006 s. Five relations hold nearly all of it, `turn`,
`sub-walk`, `strip`, `done` and `short`, 44 676, 27 602, 27 236, 27 234 and 9 271 of them at the stop; each
insertion scans its relation's list twice, so a step costs more as the list grows, 12 552 steps in the first two
seconds and about sixty a second after ten minutes.

**What reads `Δ̂`.** The verdict reads it through the relation-grain sites and nothing else: `FactGrain::sites`
is `Monotonicity::sites` mapped, `is_monotone` is *no site survives*, and `complete` is about the activator
sites. **A program the relation-grain check does not flag has no site**, so its fact-grain verdict is *monotone*
and *complete* whatever `Δ̂` holds, and the closure ran on it to be thrown away. E-33's two programs are such
programs: `readings` with nothing owed has no `absent` and no obligation, so `D` holds `reads-as` and what it
derives, and nothing reads `D` at a non-monotone polarity. So is every program this stage banks: the always-owed
obligation of T2 reads `wanted` and `reading-of`, neither in `D`, and `reads-as` inside its `absent`, at odd
depth, the polarity an obligation's trigger may read; `listed_is_all` of T4 reads `absent` over `has` on
programs with no candidate; `before.ein` has none either.

**The change** (`ein-infer/examples/shared/fact_grain.rs`, a test-side prototype no route reads):

1. **The closure runs only where a site reads it.** With no relation-grain site `classify` returns *monotone*,
   *complete* and `shapes: None` at once.
2. **On a flagged program it gives up at `MAX_SHAPES`, 10 000 shapes added, on the safe side:** `Δ̂` is taken to
   be every fact, which over-approximates `Δ` as the closure does, so every site is one, its witness the relation
   it reads with every argument *any*, an activator site makes it incomplete, and the verdict is *not monotone*
   (`FactGrain::gave_up`). 10 000 is a hundred times the largest closure a flagged corpus program computes.
3. **`fact_grain_never_calls_a_violated_cube_monotone` fails on a program where the closure gives up,** naming
   it and E-33. A give-up would otherwise turn the program into one fact grain calls not monotone, whose cube the
   test does not read: the test would check less and say nothing. So the bound never decides a verdict the test
   reads; it turns a run that does not end into a failure that names the program.

`site_census` prints `shapes` as `null` where the closure did not run, `gave_up` beside it, and *the closure gave
up* on its program line. A new test, `fact_grain_closure_runs_where_a_site_reads_it_and_gives_up_on_the_safe_side`,
holds both halves on the eleven-line program: unflagged, no `Δ̂`, and its cube read; with an `absent` over the
quotient added, flagged, the closure gives up, the site's witness `(quot-of _ _ _ _)`, *not monotone*.

**Why what the test means is unchanged.** The test says: on every corpus file and register witness, where fact
grain calls the program monotone, the oracle's cube has no violation (and none where relation grain does), on at
least 140 cubes. (1) moves no verdict, by construction, and measured: every file's relation-grain verdict, its
fact-grain verdict, its completeness and every site's witness text, the closure at `fc92f075` against this tree's,
**identical on all 743 files** of the selection (522 that load and saturate, 48 of them flagged;
`build/s1f.6.28/t1-v-old.txt` and `t1-v-new.txt`, `diff` empty, 2026-11-13). (2) moves none on the corpus: no
corpus closure comes near the bound, the largest a flagged program computes being 100 shapes
(`examples/saturation/square-unique/terminus.ein`) and the largest any program computed 383
(`examples/beyond_arith/equation_two_unknowns.ein`, unflagged, now not computed;
`build/s1f.6.28/t1-corpus-sweep.txt`, 0.18 s for the 743 closures together at `fc92f075`); and (3) makes a
future one fail. The cubes the test reads: **437 at `fc92f075` and 437 on this tree**, 24.13 s and 24.09 s
(`cargo test -p ein-infer --test region fact_grain_never -- --nocapture` with a count printed for the run and
taken out after, 2026-11-13).

**On E-33's programs** (`cargo test -p ein-infer --test region fact_grain`, 2026-11-13, each with the files
copied into `examples/word_problems/` for the run and removed after):

| in the corpus's directory | at `fc92f075` | this tree |
|---|---|---|
| nothing added | passes, 24.13 s | 3 passed, 23.37 s |
| E-33's witnesses, `boxes13_request_under_reading.ein` and `readings_request_under_reading.ein` (readings never owed, unflagged) | did not end in 13 263 s (E-33) | **3 passed, 23.55 s** |
| the page's `boxes13`, readings owed where something is left over, flagged by the obligation's trigger over `left-over` | not tried: its closure is E-33's, past 26 000 shapes in 30 s on the probe | **fails in 24.67 s**: *…the fact-grain closure gave up past 10000 shapes (E-33), so its verdict is the safe one and no cube of it is read here* |

**The three ways the stage file named**, as read: *the widening applied to the continuation tags sooner* changes
`Δ̂` on every program and so can move a verdict the test reads, and the antichain would still be a product of
what each argument can be; *the closure's work shared* is not where the time goes, the antichain's own size is;
*a bound that makes the analysis answer not monotone where it gives up* is (2), taken with (3) so that it never
narrows the test. (1) is none of the three: a verdict that reads nothing of `Δ̂` does not compute it.

**What stays open**, the user's ([Q-M1f.225](../open_questions.md#q-m1f225--what-the-fact-grain-test-does-where-the-closure-gives-up),
defaulted): a flagged program on which a supposed fact leads to a calculator request with an operand unbound
fails the test, as the page's `boxes13` does above. No program of the corpus is one, and none this stage banks.

**Run on this tree** (2026-11-13): `cargo test -p ein-infer --test region` **alone**, as `AGENTS.md` has it for
E-33, 29 passed, 41.18 s (S1f.6.27 recorded 39.76 s with one test fewer); `cargo fmt --all --check`; `cargo
clippy -p ein-infer --all-targets -- -D warnings`; `site_census` on three files, its new fields read back. The
wider sweeps run with T5, which adds the programs.

### T1f.6.28.2 — the readings rebuilt, shown before they are banked, done

**`readings.ein`, always owed** (`build/s1f.6.28/p2/readings.ein`, the page's `readings_always` of
[`word_problems.md` § 4.3](word_problems.md#43-the-three-ways-to-write-readings-as-hypotheses)). Against the
module as banked at `f3fe6ff0` (the page's `readings_plain`, nothing owed):

- **an obligation, `reading-owed`**: where a question asks how many containers are needed, `(wanted (needed C K
  Q))`, and no reading the theory holds is supposed, the state owes `(open reads-as)`. Its `:why` is what the
  read-out's `owed` row prints ([S1f.6.26](s1f.6.26_what_root_owes.md)): *a reading of need is owed: the question
  asks how many boxes are needed for [(together Bob Alice apples now)], and the problem does not say what need
  means*;
- **the hrule proposes a reading for every such question**, `(wanted (needed …))`, where it proposed one only
  where a division leaves something over;
- **`need-matters` and `reading-matters` are gone**: they named the gap in the state with no reading, which is
  no longer in R; the `owed` row names it now;
- the import of `words` gains `wanted`, which the obligation and the hrule read. The rules of each reading,
  `one-reading` and the entry rule's closures are as they were; the entry rule names `reading-owed` and no
  longer `need-matters`.

**The table** (`build/s1f.6.28/t2/run.sh` then `python3 build/s1f.6.28/t2/table.py`, the release binary, whose
engine `41806bcd` leaves as `fc92f075` built it, 2026-11-13; every output under `build/s1f.6.28/t2/out/<program>/`). *Apart from the engine* is the
script's own computation: the problem's arithmetic in Python, and R by the readings' meaning over the four
subsets of the two candidates, with two readings at once a contradiction and no reading a debt.

| | `boxes12`: 5 + 7 apples, boxes of 3 | `boxes13`: 5 + 8 apples, boxes of 3 | `boxes13_stated` (scratch): the same, *Every apple must be packed.* |
|---|---|---|---|
| apart from the engine | 12 = 3 × 4 + 0; R = {pack-all}, {whole-only}; each answers 4: **two states, one class** | 13 = 3 × 4 + 1; whole-only 4 and 1 over, pack-all ⌈13 ÷ 3⌉ = 5 and 0 over; R = {pack-all}, {whole-only}: **two states, two classes** | the stated reading is root's: R = {∅}, 5 |
| `ein solve` (describe) | 2 states, `∅ not in R — root owes 1 instance`, its `owed` row as above; minimal and maximal lines 2 states each; `N` 1 minimal dead set, the two readings together; monotone; 2 candidates, 0 dead alone; query 1: **1 class over 2 states, 4**; query 2: 2 classes, pack-all and whole-only; 4 saturations · 5 facts beyond root | 2 states, the same rows; query 1: **2 classes, 4 (state 2, whole-only) and 5 (state 1, pack-all)**; query 2: each with its reading and what is left over, 1 and z; 4 saturations · 45 facts beyond root | 1 state, ∅, root owes nothing; `N` 1 minimal dead set, whole-only, dead alone by `one-reading` against root's pack-all; 5 boxes, z over; 2 saturations |
| `--single` | `Ambiguity — R holds two states or more`, the two states; 3 saturations | `Ambiguity`, the two states; 3 saturations | `Solution — R is one state`; 2 saturations |
| `--minimal` | `2 states — all of them`; 3 saturations | `2 states — all of them`; 3 saturations | `1 state — all of them`; 1 saturation |
| `--maximal` | `2 states — all of them`; 4 saturations | `2 states — all of them`; 4 saturations | `1 state — all of them`; 2 saturations |
| `ein test -v` | 3 claims held: the region claim (state route, k_s = 2), query 1 (*describe: all of R, 1 class over 2 states*), query 2 (2 classes) | 3 held: the region claim, query 1 (2 classes over 2 states), query 2 | 2 held |
| `ein oracle` | 2 states, `E` 2 minimal discharging sets, the same `owed` row | 2 states, the same | 1 state |
| root's firings (`ein saturate`) | 145, as banked | 146 | 161 |
| exit, every run above | 0 | 0 | 0 |

**Every answer agrees with the computation apart from the engine**, and every mode with describe and the oracle
(`--count`: 2 states, 2 commitments, on both). `boxes12` is the first corpus witness of *answer correct, solution
not necessarily*, as round 2 put it: two solutions, one class. The read-out names the debt that took root out of
R, and the `(region …)` names each state's reading, `from {(reads-as need pack-all)}`. `boxes13_stated` shows that a
reading the problem states closes the debt as a sentence closes a gap; it stays in scratch, the stage banking the
13-apple problem the user wrote and no twin of it (reply 35, *"take only needed right now"*).

**The trace of each state** (`ein solve FILE --trace T.md`, `build/s1f.6.28/t2/out/<program>/trace.md`):

| | root | state 1, pack-all | state 2, whole-only |
|---|---|---|---|
| `boxes12` | 145 steps, ten of the word theory: `asked-is-wanted`, `need-plan`, `together-plan`, `holds-states`, `has-states` twice, `together-means-sum`, `together-is-the-sum`, `holds-means-divide`, `need-exact` | **0 steps** | **0 steps** |
| `boxes13` | 146 steps, the same ten with `need-left-over` in place of `need-exact` | 15 steps: `need-pack-all-plan`, the calculator's 4 + 1 in thirteen, `need-pack-all` | 1 step: `need-whole-only` |

### T1f.6.28.3 — *why divided, and not subtracted*, written down, done

Nothing is built; this section reads the traces above, which are the program's own account of the choice.

**What the trace says.**

- **Which operation, and what in the problem called for it.** Each operation is one rule's request, and its step
  names the rule, its `:why` and its premises. On both programs: step 5, `need-plan`, *how many boxes means a
  division: first how many apples there are to pack, and how many one of the boxes holds*, from `(wanted (needed
  boxes apples …))`; step 25, `holds-means-divide`, *one of the boxes holds (n z 3), so the number of boxes is a
  division: (n (n z 1) 3) divided by (n z 3)*, from the question's quantity, the sum's value and the capacity.
  *Need* with *a box holds* means division because those two premises are the rule's, and that is all the
  trace can say of why: it is the rule's reading of the words.
- **Why 13 ÷ 3 and not 3 ÷ 13.** The rule writes `(div ?a ?m)`, the quantity to pack first and the capacity
  second, and the step shows the bindings: `?a` the value of `(together Bob Alice apples now)`, `?m` the value of
  `(capacity boxes apples)`. The order of the operands is the rule's.
- **Why the sum first.** The division's premise is the sum's answer, `(value (together …) 13)`, which step 24
  derives; so the order of the operations is the derivation's (steps 10 to 24, then 25), and no fact states it.
- **What each reading adds.** On `boxes13` the states differ in the step that answers: whole-only's one step,
  *4 boxes are filled, and 1 apple stays unpacked*; pack-all's plan, *4 boxes do not hold everything, since 1
  apple is left over; one more does, and need asks for the least number, so add 1 to 4*, the addition, and the
  answer. The second operation of pack-all, + 1, is explained by the reading and by nothing else.
- **On `boxes12` the reading changes nothing in the derivation.** Both states' traces are empty: the answer is
  root's, `need-exact`, *12 divided by 3 = 4 and nothing is left over: 4 boxes, each full, whatever need means*.
  The two solutions of R have one derivation, and the reading each supposes is a commitment no rule read. That is
  what *answer correct, solution not necessarily* comes to on this theory: the program says the reading did not
  matter, in the `:why` of the step that answered.

**What it does not say.**

- **Why not 3 + 13, or 13 − 3.** No rule of `words` concludes `add` or `sub` from `needed`, so no such step
  exists; a trace lists what fired and not what could have and did not. That the theory has no other reading of
  *how many boxes* is read off the theory (`ein render rules examples/word_problems/words.ein`: `needed` is
  read by `need-plan`, `holds-means-divide`, `need-exact` and `need-left-over`, and by nothing that asks for a
  sum or a difference), not off the run.
- **That the division is right.** The rule is the theory's decision about what the words mean, valid under
  `(model-counts-only)`, which `need-exact` takes as a premise and the trace cites (*a box holds any 3
  apples*). Nothing judges whether a solution is correct beside its answer: the engine has no wrong solution,
  only facts that follow. A solution correct with its reasoning wrong is the user's M2 case, out of scope.
- **The operations as alternatives.** The search here is over readings, `(reads-as need R)`, under rules that
  stay fixed (the user's distinction of 2026-11-11); an operation is never a hypothesis, so no state of R is
  *the subtraction*, dead or with another answer, that the read-out could show beside the division. *Why not
  subtract* answered by a state is a search over formulas, a theory of solutions:
  [M1h's P1h.4](../../m1h_beyond_word_problems/README.md#p1h4--the-formula-and-the-calculation) and M2's.
