# examples/word_problems/

Elementary-school word problems as Ein programs (M1f P1f.6,
[S1f.6.20](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md)). The
calculator, [`std.arith`](../../stdlib/arith.ein), is a library among other relations here and not the subject: a
problem states what its sentences say as facts, a theory of what the words mean turns them into requests to the
calculator, and the theory reads the answers back as facts about the people. The engine knows no arithmetic and
no word.

These are **examples, not a stdlib module** (the user's reply 34, *"examples for now"*). The four theory modules
live in this directory and the problems import them by file name. The facts of each problem were written by
hand; reading them off the English is [M2](../../plans/m2_nl_to_ir/README.md)'s.

They were banked on 2026-11-12, on the user's instruction (*"bank word problems into examples/word_problems with
recommended variants"*), from the prototype that
[`word_problems.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md) reports, and
rebuilt on 2026-11-13 by
[S1f.6.28](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md) on
the user's replies of 2026-11-12: **a reading is always chosen** (reply 31), and ***what is not listed is zero***
is an assumption a problem may adopt (reply 32). The other forms here, the gap, the assumption, the order of
events and the backwards plan, are the ones picked to bank the problems, and in the user's words **not only and
not final** ([§ The banked forms](#the-banked-forms-not-only-and-not-final)).

## Running one

```sh
ein solve examples/word_problems/boxes12.ein                  # the read-out: each query's answer as a sentence
ein test  examples/word_problems/                             # every claim, as a status code
ein solve examples/word_problems/boxes12.ein -G --trace T.md  # the reasoning, a step a rule, each with its :why
ein saturate examples/word_problems/boxes12.ein --dump        # root's state: the plan (needs …) and the requests
```

A number prints as the calculator writes it, `(n z 3)` for 3 and `(n (n z 1) 2)` for 12, and a quantity as the
nested fact it is, `(amount Bob apples t1)` for *how many apples Bob has at t1*.

## The problems

Every answer below is the file's own `:expect`, which `ein test examples/word_problems/` holds and the corpus
sweep banks per commit.

| file | the problem | the answer | search |
|---|---|---|---|
| [`bob1.ein`](bob1.ein) | Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have? | 3 | none |
| [`gap.ein`](gap.ein) | Bob has 5 apples. He gave 2 to Alice. How many apples do they have together? | Bob's 3 is known; *together* has no answer; the gap is named: how many apples Alice had at `t0` is not stated | none |
| [`gap_fact.ein`](gap_fact.ein) | the same, with *Alice has 4 apples* | Bob 3, Alice 6, together 9 | none |
| [`gap_assume.ein`](gap_assume.ein) | the same, with the assumption written: Alice had none | 5, and a fact that says the answer rests on the assumption | none |
| [`contra.ein`](contra.ein) | Bob has 5 apples. He gave 7 to Alice. How many apples does Bob have? | the problem contradicts itself: root is dead, R is empty, and the unsat core names the two sentences, the question and the model's assumption | none |
| [`kinds.ein`](kinds.ein) | Bob has 5 apples, Alice has 3 pears. How many apples do they have together? | no answer: how many apples Alice has is not stated | none |
| [`kinds_assume.ein`](kinds_assume.ein) | the same, assuming the problem lists everything each has, written as a fact | 5, resting on the assumption | none |
| [`kinds_listed.ein`](kinds_listed.ein) | the same, with [`listed_is_all.ein`](listed_is_all.ein) imported: *what is not listed is zero*, adopted for `has` | 5, and `rests-on` names what it rests on: Alice's apples, assumed zero | none |
| [`kinds_full.ein`](kinds_full.ein) | Bob has 5 apples and 2 pears. Alice has 3 apples and 4 pears. How many apples together? | 8; the pears are never wanted | none |
| [`more.ein`](more.ein) | Bob has 5 apples. Alice has 12 apples. Who has more apples, and by how many? | Alice, by 7 | none |
| [`pencils.ein`](pencils.ein) | A school has 12 boxes of pencils, 144 in each. 250 go to the office. The rest is shared equally among 31 classes. How many does each class get, and how many are left over? | 47 each, 21 left over (1728, then 1478) | none |
| [`boxes12.ein`](boxes12.ein) | Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need? | 4, in two states, one a reading of *need* each: nothing is left over, so the readings agree. **Two solutions and one answer**; neither state's trace has a step, the answer being root's, *whatever need means* | 2 candidates, a reading each; root owes one |
| [`boxes13.ein`](boxes13.ein) | Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need? | **two states and two answers**: 4 boxes and 1 apple unpacked, if *need* is read as whole-only; 5 boxes, the last part full, if as pack-all | 2 candidates, a reading each; root owes one |
| [`unused.ein`](unused.ein) | Bob has 5 apples. Alice has 7 apples. Carol has 9 apples and a box that holds 3. Bob gave 2 apples to Alice. How many apples does Bob have? | 3; Alice's 7, Carol's 9 and the box are never wanted | none |
| [`bystander.ein`](bystander.ein) | Bob has 5 apples. Carol has 9 apples. Bob gave 2 apples to Alice. How many apples does Carol have? | 9, unchanged | none |
| [`before.ein`](before.ein) | Bob gave Alice 2 apples. Now Bob has 5. How many apples did Bob have before? | **a witness, with today's answer: none**, and the question's quantity named as not stated. **The right answer is 7.** The theory reads *gave* in one direction of time and has no rule from an earlier moment to a later one; M1h's P1h.1 owns it, and its claim moves to 7 when that lands | none |

*search: none* is the read-out's `universe` row, *no candidate: R is {∅} or empty, decided by root alone*. Every
problem here is search-free but the two that ask what is *needed*: a reading of *need* is always chosen, so their
R is the readings, and root, which chose none, owes one (the `owed` row under `∅` names it).

## The model's vocabulary

**A problem writes** (relations of [`words.ein`](words.ein)), each fact with the sentence it encodes as its
`:source`:

| the fact | the sentence |
|---|---|
| `(has WHO KIND NUMBER MOMENT)` | *Bob has 5 apples.* |
| `(gave WHO WHOM KIND NUMBER BEFORE AFTER)` | *He gave 2 to Alice.* The event names the moment before it and the moment after |
| `(holds CONTAINER KIND NUMBER)` | *A box holds 3.* |
| `(packed-in WHO KIND CONTAINER MOMENT)` | *12 boxes of pencils*: the pencils are in the boxes, each full |
| `(there-are KIND NUMBER)` | *31 classes* |
| `(asked QUANTITY)` | the question |
| `(assumed QUANTITY NUMBER)` | an assumption, which is not a sentence of the problem |
| `(reads-as need READING)` | *Every apple must be packed.* (a relation of [`readings.ein`](readings.ein); no banked problem writes it) |
| `(adopted listed-is-all has)` | *what is not listed is zero*, adopted for `has`: a fact of [`listed_is_all.ein`](listed_is_all.ein), which a problem adopts by importing it |

A kind is written in the plural, `apples`, `boxes`, since a `:why` cannot inflect. A moment is a name, `t0`,
`t1`, or `now` where the problem has no event.

**A quantity** is a nested fact: `(amount WHO KIND MOMENT)`, `(together WHO WHO KIND MOMENT)`, `(apart WHO WHO
KIND MOMENT)` (by how many two differ), `(capacity CONTAINER KIND)`, `(needed CONTAINER KIND QUANTITY)`, `(over
QUANTITY)` (left unpacked), `(number-of KIND)`, `(each-of QUANTITY KIND)` and `(rest-of QUANTITY KIND)` (a
sharing's quotient and remainder).

**The theory derives:**

- **the plan**, backwards from the question: `(wanted Q)`, and `(needs Q Q')`, *to know Q one must know Q'*.
  Nothing is computed that no question needs, which is why an unused sentence is silent
  ([`unused.ein`](unused.ein)) and why a contradiction is found only where a question reaches it;
- **the computation**, forwards from the stated numbers: a request to the calculator, `(add …)`, `(sub …)`,
  `(mul …)`, `(div …)` or `(cmp …)`, and its answer read back as `(value Q NUMBER)`. A word is two rules, one
  that asks and one that reads, and the order of the operations is the derivation's: the second request is
  derived from the first one's answer;
- `(has-more WHO WHOM KIND MOMENT)` and `(same-amount …)` for a comparison, `(left-over Q FILLED OVER)` where a
  division for *need* leaves a remainder, and the flag `(gave-more-than-had WHO KIND HAD GAVE)`, from which a
  second rule derives `(false)`;
- **what the model leaves out**, stated once: the fact `(model-counts-only)`, whose `:source` says that things
  are counted by number alone and nobody gives away what they do not have. The rules whose conclusion depends
  on it take it as a premise, so a trace and an unsat core cite it where it is used.

[`gaps.ein`](gaps.ein) adds, by `absent`, `(not-stated Q)` for a quantity that is wanted, has no value and needs
nothing; `(unanswered QUESTION Q)` for a question that rests on one; and `(rests-on QUESTION Q NUMBER)` for a
question computed from an assumption. [`readings.ein`](readings.ein) adds the readings of *need* as facts,
`(reading-of need whole-only)` and `(reading-of need pack-all)`, the hrule `read-need` that proposes `(reads-as
need R)` for every question of how many containers are needed, a rule a reading that answers under it, and the
obligation `reading-owed`: a state with such a question and no reading **owes** one, so root is not in R and
every state of R names the reading its answer rests on. [`listed_is_all.ein`](listed_is_all.ein) is an
assumption, not a theory of words: by `absent` over `has`, for every holder and kind the `has` sentences name at
a moment with no amount listed, `(assumed (amount WHO KIND MOMENT) z)`, resting on `(adopted listed-is-all has)`.
It does not hold of every relation: in the Zebra puzzle an empty house is one whose owner is not known yet. A
problem that adopts it states that `has` is no hypothesis relation, `(solve :no-hypothesis (has))`.

Each module has one entry rule, as `std.arith` has `arith`: `(import words :symbols (words))`, `(import gaps
:symbols (gaps))`, `(import readings :symbols (readings))`, `(import listed_is_all :symbols (listed-is-all))`. A
problem that imports `readings` also writes `(solve :hrules (read-need))`, which a module cannot write for it.

## The banked forms, not only and not final

Each thing the investigation tried in several forms is banked in one. The user, 2026-11-12, on how a missing
quantity is detected (reply 28): *"I want already written word problems banked as is, so pick recommended, and
note that is is not only and not final."* So each form below is **the one picked to bank the problems, not the
only one and not the final one**: the subject is
[M1h's](../../plans/m1h_beyond_word_problems/README.md), which takes each up again. The section numbers are
[`word_problems.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md)'s, which has
every other form with its run.

| the choice | banked | the reply | the other forms, on the page |
|---|---|---|---|
| **the gap**, a missing quantity (§ 3) | (b) of 28, a derived fact by `absent`, in a module of its own: `gaps.ein`, `gap.ein` | 28: *"pick recommended … not only and not final"* | (a), an obligation, which names the gap and answers nothing; (c), the unknown as a candidate from a stated list |
| **the assumption** (§ 3.5) | (a) of 29, a marked fact, `assumed`, with `rests-on` beside the answer: written by hand (`gap_assume.ein`, `kinds_assume.ein`) or derived by `listed_is_all.ein` (`kinds_listed.ein`) | 29: *"for this stage (a) is OK, return back to investigations of this in M1h"* | the assumption as a hypothesis, alone, with (a) and with (b); the last leaves a stale fact (F21) |
| **the order of events** (§ 1) | named moments, `t0`, `t1`, and an event naming the moment before it and after it | 26: *"pick the simplest that works - leave what is implemented already"* | events as nested facts; a chain of numbered sentences |
| **the plan** (§ 2.1) | backwards from the question, then forwards from the stated numbers | 27 (a) | (the only one built) |
| **a word with two readings** (§ 4) | the readings as hypotheses, a reading **always owed**: `readings.ein`, `boxes12.ein`, `boxes13.ein` | 31: *"reading always chosen, it can't be not chosen"*; round 2: *"yes"* | owed only where something is left over (flagged by the static check); never owed, with the state of no reading in R (banked until 2026-11-13); both answers derived; a named gap and no answer |
| **two kinds** (§ 6.2) | *Alice has 3 pears* does not say she has no apples: a gap (`kinds.ein`), unless the problem adopts *what is not listed is zero* (`kinds_listed.ein`) | 32: *"what is not listed is zero - good idea, it could be a reusable universal assumption"* | (none built) |
| the model's boundary (§ 5) | a fact with a `:source`, a premise of the rules that depend on it | 33, and round 2: *"OK"*; M1h's slicing takes it up | (the query's `:goal-text` is in `boxes12.ein` as well) |
| more readings of a word (§ 8) | none added: *together* over everyone and *gave all* are not here | 35: *"Take only needed right now"* | — |

Against the page's files: `readings.ein` is its `readings_always` with the obligation's `:why` written for the
read-out's `owed` row; `boxes12.ein` and `boxes13.ein` are its `boxes12_always` and `boxes13_always` with a region
claim and, in `boxes12.ein`, the model's query; `listed_is_all.ein`, `kinds_listed.ein` and `before.ein` were
written by S1f.6.28 and are not on the page.

What the program says of **why it divided and did not subtract** is in the trace of each state, as far as the
rules go, and no further: a theory of solutions is M1h's P1h.4 and M2's
([S1f.6.28's record](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md#t1f6283--why-divided-and-not-subtracted-written-down-done)).
