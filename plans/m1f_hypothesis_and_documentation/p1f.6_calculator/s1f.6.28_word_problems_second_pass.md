# S1f.6.28 — Word problems, second pass: a reading is always chosen, and *what is not listed is zero*

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's replies of that day. Not started. It continues
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
