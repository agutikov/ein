# examples/word_problems/

Elementary-school word problems as Ein programs (M1f P1f.6,
[S1f.6.20](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md)). The
calculator, [`std.arith`](../../stdlib/arith.ein), is a library among other relations here and not the subject: a
problem states what its sentences say as facts, a theory of what the words mean turns them into requests to the
calculator, and the theory reads the answers back as facts about the people. The engine knows no arithmetic and
no word.

These are **examples, not a stdlib module**. The three theory modules live in this directory and the problems
import them by file name. The facts of each problem were written by hand; reading them off the English is
[M2](../../plans/m2_nl_to_ir/README.md)'s.

They were banked on 2026-11-12, on the user's instruction (*"bank word problems into examples/word_problems with
recommended variants"*), from the prototype that
[`word_problems.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md) reports.
That page has every variant that was tried, each with its run, and twelve questions for the user.
**Those questions are open.** What is here is the variant the page recommends for each, banked as an example; it
is not a ruling on any of them ([§ Which variants](#which-variants-and-what-is-not-here)).

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
| [`kinds_assume.ein`](kinds_assume.ein) | the same, assuming the problem lists everything each has | 5, resting on the assumption | none |
| [`kinds_full.ein`](kinds_full.ein) | Bob has 5 apples and 2 pears. Alice has 3 apples and 4 pears. How many apples together? | 8; the pears are never wanted | none |
| [`more.ein`](more.ein) | Bob has 5 apples. Alice has 12 apples. Who has more apples, and by how many? | Alice, by 7 | none |
| [`pencils.ein`](pencils.ein) | A school has 12 boxes of pencils, 144 in each. 250 go to the office. The rest is shared equally among 31 classes. How many does each class get, and how many are left over? | 47 each, 21 left over (1728, then 1478) | none |
| [`boxes12.ein`](boxes12.ein) | Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need? | 4; nothing is left over, so the readings of *need* agree and none is proposed | none |
| [`unused.ein`](unused.ein) | Bob has 5 apples. Alice has 7 apples. Carol has 9 apples and a box that holds 3. Bob gave 2 apples to Alice. How many apples does Bob have? | 3; Alice's 7, Carol's 9 and the box are never wanted | none |
| [`bystander.ein`](bystander.ein) | Bob has 5 apples. Carol has 9 apples. Bob gave 2 apples to Alice. How many apples does Carol have? | 9, unchanged | none |

*search: none* is the read-out's `universe` row, *no candidate: R is {∅} or empty, decided by root alone*. Every
problem here is search-free. **The remainder variant is not here** ([§ What the gate could not
hold](#what-the-gate-could-not-hold)).

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
need R)` where a division leaves something over, a rule a reading that answers under it, and
`(reading-matters need Q FILLED OVER)`, which names the gap in the state where no reading is supposed.

Each module has one entry rule, as `std.arith` has `arith`: `(import words :symbols (words))`, `(import gaps
:symbols (gaps))`, `(import readings :symbols (readings))`. A problem that imports `readings` also writes
`(solve :hrules (read-need))`, which a module cannot write for it.

## Which variants, and what is not here

One variant of each thing the page tried is banked. The section numbers are
[`word_problems.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md)'s.

| the choice | banked | on the page only |
|---|---|---|
| how a problem is written (§ 1) | form 1, named moments | form 2, events as nested facts; form 3, a chain of numbered sentences |
| the direction of the theory (§ 2.1) | backwards from the question, then forwards | (the only one built) |
| a missing quantity (§ 3) | (b), a derived fact by `absent`, in a module of its own: `gaps.ein`, `gap.ein` | (a), an obligation, which names the gap and answers nothing; (c), the unknown as a candidate from a stated list |
| an assumption (§ 3.5) | a marked fact, `assumed`, with `rests-on` beside the answer: `gap_assume.ein` | the assumption as a hypothesis, alone, with (a) and with (b); the last leaves a stale fact (F21) |
| a missing reading (§ 4) | readings as hypotheses with nothing owed, the page's `readings_plain`, here `readings.ein`; its 13-apple problems are not banked (below) | `readings` (owed where it matters, flagged by the static check), `readings_always`, both answers derived, a named gap and no answer |
| two kinds (§ 6.2) | *Alice has 3 pears* does not say she has no apples: a gap | (no alternative built) |
| the model's boundary (§ 5) | a fact with a `:source`, a premise of the rules that depend on it | (the query's `:goal-text` is in `boxes12.ein` as well) |

Two things differ from the page's files, and neither is an answer:

- **`readings.ein` is the page's `readings_plain.ein`**, under the name the page proposes for the module. Its
  import of `words` lists what it reads; the prototype's also listed `wanted`, which it read nowhere.
- **`boxes12.ein` is the page's `boxes12` and `boxes12_plain` in one file**: the two-step problem with
  `readings` imported, and the model's query. It fires one rule more at root than the page's `boxes12`, the entry
  rule of `readings`.

Every banked answer was re-taken on the release binary and the stdlib of the commit that banked these files, and
none differs from the page's (`ein test examples/word_problems/`, 2026-11-12).

## What the gate could not hold

**The remainder variant, 13 apples in boxes of 3, is not banked**, nor is its twin with the reading stated
(the page's `boxes13_plain` and `boxes13_stated`, § 4). Both answer as the page reports on today's engine: three
states, no answer with no reading supposed, 4 boxes under `whole-only` and 5 under `pack-all`; and one state, 5
boxes, with *Every apple must be packed* (`ein solve` on each, 2026-11-12). What cannot hold them is a test of
the gate, not the engine: `ein-infer/tests/region.rs`'s `fact_grain_never_calls_a_violated_cube_monotone` runs a
prototype static analysis over every corpus program, and on a program where a hypothesis leads to a `value`
fact it does not end, since the rules of `words` turn any value into a request to the calculator and the
analysis then follows every shape of every circuit. It is E-33 of the phase's
[`findings.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md), and what to do about
it is the user's
([Q-M1f.198](../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f198--the-remainder-variant-of-the-word-problems-and-the-gate)).

`readings.ein` is banked all the same: it is the module the page recommends, `boxes12.ein` imports it, and a
problem written beside these files with a remainder runs under `ein solve` and `ein test` as the page shows:

```lisp
(import words :symbols (words))
(import readings :symbols (readings))
(has Bob apples (n z 5) now)
(has Alice apples (n z 8) now)
(holds boxes apples (n z 3))
(asked (needed boxes apples (together Bob Alice apples now)))
(solve :hrules (read-need))
(query :goal (and (asked ?q) (value ?q ?n)) :goal-text "{?q}: {?n}")
```
