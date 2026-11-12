# S1f.6.20, T1 — word problems on a prototype: the examples, and the stop

**Stage:** [S1f.6.20](s1f.6.20_word_problems.md) of [P1f.6](README.md). **Task:** T1, the prototype and this page. It
stops here for the user: nothing ships, no engine source, stdlib, corpus or test changed.
**Taken:** 2026-11-11.

This page shows a prototype theory of what the words of an elementary-school problem mean, written in Ein over the
calculator [`std.arith`](../../../stdlib/arith.ein), and every problem of the stage's table run on it. It is also
the first instance of M2's
[reasoning as a program](../../m2_nl_to_ir/README.md#the-reasoning-as-a-program-the-users-idea-2026-11-11): for
each problem the English sentences and the Ein facts are side by side, and the facts were written by hand.

**What ran.** The release binary as built at 19:17 on 2026-11-11 (`ein 0.1.0`, `ein-events/1`, features `einb,
parallel, snmalloc`), copied to `build/s1f.6.20/tools/ein`, with the stdlib of HEAD `a621dc75` extracted beside it
(`git archive HEAD stdlib`, `stdlib sha256:50a500e1…a040` by `ein --version`). The check the task names holds:
HEAD's `examples/arith/div.ein` answers 5105, remainder 20. Every program on this page ran as
`EIN_STDLIB=build/s1f.6.20/tools/stdlib build/s1f.6.20/tools/ein …`.

**Where it is banked (2026-11-12).** On the user's instruction of that day, *"bank word problems into
examples/word_problems with recommended variants"*, the theory and most of the problems are in the tree, under
[`examples/word_problems/`](../../../examples/word_problems/README.md): the modules
[`words.ein`](../../../examples/word_problems/words.ein), [`gaps.ein`](../../../examples/word_problems/gaps.ein)
and [`readings.ein`](../../../examples/word_problems/readings.ein), which is this page's `readings_plain`, and
thirteen problems, each a corpus entry with its claim. The banked name of each file of this page:

| on this page | banked as |
|---|---|
| `p/words.ein`, `p/gaps.ein` | `words.ein`, `gaps.ein` |
| `p/readings_plain.ein` | `readings.ein` |
| `p/bob1`, `contra`, `kinds`, `kinds_assume`, `kinds_full`, `pencils`, `more`, `unused`, `bystander`, `gap_fact`, `gap_assume` | the same names |
| `p/gap_b.ein` | `gap.ein` |
| `p/boxes12.ein` and `p/boxes12_plain.ein` | `boxes12.ein`, one file: `readings` imported, and the model's query |
| `p/boxes13_plain.ein`, `p/boxes13_stated.ein` | **not banked, though recommended**: a test of the gate does not end on a program where a reading is supposed and an answer follows from it ([E-33](findings.md), [Q-M1f.198](../open_questions.md#q-m1f198--the-remainder-variant-of-the-word-problems-and-the-gate)). Both answer as § 4 reports on the banking commit's binary |
| `gap_a`, `gap_c`, `gap_assume_h`, `_ha`, `_hb`, `boxes12_readings`, `boxes12_always`, `boxes13` (owed), `boxes13_always`, `boxes13_both`, `boxes13_gap`, `readings`, `readings_always`, `readings_both`, `readings_gap`, `q1/form_events`, `q1/form_chain`, every `witness/` file | not banked: the variants this page does not recommend, and the witnesses. They are on this page and under `build/s1f.6.20/` |

**Banking is not a ruling.** The twelve questions of [§ 11](#11-the-questions-for-the-user) are open; the banked
files take the recommendation of each as an example to read and run. Every banked answer was re-taken on the
release binary and stdlib of the banking commit and is the one this page reports
([the stage's record](s1f.6.20_word_problems.md#t2-record-2026-11-12-the-prototype-banked)).

**Where the prototype is.** `build/s1f.6.20/` is gitignored, so this page carries the theory in full
([§ 12](#12-the-prototype-in-full)) and each problem's facts. There: `p/` holds the modules and the problems,
`q1/` the two other forms of § 1, `witness/` the programs that failed or were refused, and `out/<name>/` every
artefact of a run: `solve.txt` (the read-out in full, and the exit code), `solve_H.txt` (with `-H`), `saturate.txt`
(`ein saturate --dump`), `test.txt` (`ein test -v`), `trace.md`, `trace_G.md` (`--trace -G`), `trace_G_relevant.md`
(`-G -l`), `events.jsonl`, `summary.json`, `answer/` (`--dump-answer`) and `pupil.md` (the word theory's steps
picked out of the trace, § 2). `build/s1f.6.20/run.sh p <name>` re-takes one problem, `table.py` the table of § 7,
and `gen_page.py` wrote this page from those files: **every answer, read-out line and trace step below is read
from a run's output, none is typed.**

**Every number on this page** is from those runs, on 2026-11-11, by the command named beside it.

## 0. The problems and their answers

29 programs, each with an `:expect` that `ein test` holds (`out/<name>/test.txt`:
every line *held*, no `FAILED`, no `NOT CHECKED`). The answer column is the read-out's own sentences, each a
query's `:goal-text` filled from the state. A number prints as the calculator writes it, `(n z 3)` for 3 and
`(n (n z 1) 2)` for 12 ([E-14](findings.md)); a quantity prints as the nested fact it is, `(amount Bob apples t1)`
for *how many apples Bob has at t1*.

| file | the problem | what the run answers | search-free |
|---|---|---|---|
| `p/bob1.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have? | (amount Bob apples t1): (n z 3) | yes |
| `p/gap_a.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples do they have together? Mechanism (a): the gap is an obligation, the state owes. | no answer: root owes — the problem owes [(amount Alice apples t0)] a value: no sentence states it | yes |
| `p/gap_b.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples do they have together? Mechanism (b): the gap is a derived fact, by `absent`. | known: (amount Bob apples t1): (n z 3)<br>known: (amount Bob apples t0): (n z 5)<br>no answer to [(together Bob Alice apples t1)]: the problem does not state [(amount Alice apples t0)] | yes |
| `p/gap_c.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples do they have together? Mechanism (c2): the unknown as a candidate, from a list of numbers the problem file states, proposed for each wanted amount. | (together Bob Alice apples t1): (n z 3)<br>(together Bob Alice apples t1): (n z 4)<br>(together Bob Alice apples t1): (n z 5)<br>(together Bob Alice apples t1): (n z 6)<br>(together Bob Alice apples t1): (n z 7) | no |
| `p/gap_fact.ein` | Bob has 5 apples. Alice has 4 apples. He gave 2 to Alice. How many apples does Bob have? How many does Alice have? How many apples do they have together? | (amount Bob apples t1): (n z 3)<br>(amount Alice apples t1): (n z 6)<br>(together Bob Alice apples t1): (n z 9) | yes |
| `p/gap_assume.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples do they have together? Assume that Alice had none. The assumption is a fact with a marker of its own, `assumed`. | (together Bob Alice apples t1): (n z 5)<br>the answer to [(together Bob Alice apples t1)] rests on the assumption that [(amount Alice apples t0)] is z | yes |
| `p/gap_assume_h.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples do they have together? It may be supposed that Alice had none. The assumption is a hypothesis: a candidate the search may commit to. | (together Bob Alice apples t1): (n z 5)<br>known: (amount Bob apples t1): (n z 3)<br>known: (amount Bob apples t0): (n z 5)<br>known: (amount Alice apples t1): (n z 2)<br>known: (together Bob Alice apples t1): (n z 5)<br>known: (amount Alice apples t0): z | no |
| `p/gap_assume_hb.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples do they have together? It may be supposed that Alice had none. The assumption is a hypothesis, and the gap is a derived fact by `absent`: the two together make the program non-monotone. | (together Bob Alice apples t1): (n z 5)<br>no answer to [(together Bob Alice apples t1)]: the problem does not state [(amount Alice apples t0)] | no |
| `p/contra.ein` | Bob has 5 apples. He gave 7 to Alice. How many apples does Bob have? | no answer: the problem contradicts itself (root is dead) | yes |
| `p/kinds.ein` | Bob has 5 apples, Alice has 3 pears. How many apples do they have together? | known: (amount Bob apples now): (n z 5)<br>no answer to [(together Bob Alice apples now)]: the problem does not state [(amount Alice apples now)] | yes |
| `p/kinds_assume.ein` | Bob has 5 apples, Alice has 3 pears. How many apples do they have together? Assume that the problem lists everything each has: Alice has no apples. | (together Bob Alice apples now): (n z 5)<br>the answer to [(together Bob Alice apples now)] rests on the assumption that [(amount Alice apples now)] is z | yes |
| `p/kinds_full.ein` | Bob has 5 apples and 2 pears. Alice has 3 apples and 4 pears. How many apples do they have together? | (together Bob Alice apples now): (n z 8)<br>(amount Alice apples now): (n z 3)<br>(amount Bob apples now): (n z 5) | yes |
| `p/pencils.ein` | A school has 12 boxes of pencils, 144 pencils in each. 250 pencils go to the office. The rest is shared equally among 31 classes. How many pencils does each class get, and how many are left over? | (rest-of (amount School pencils t1) classes): (n (n z 2) 1)<br>(each-of (amount School pencils t1) classes): (n (n z 4) 7)<br>(amount School pencils t1): (n (n (n (n z 1) 4) 7) 8)<br>(amount School pencils t0): (n (n (n (n z 1) 7) 2) 8)<br>(capacity cartons pencils): (n (n (n z 1) 4) 4)<br>(amount School cartons t0): (n (n z 1) 2)<br>(number-of classes): (n (n z 3) 1) | yes |
| `p/more.ein` | Bob has 5 apples. Alice has 12 apples. Who has more apples, and by how many? | Alice has (n z 7) more apples than Bob | yes |
| `p/boxes12.ein` | Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need? | (needed boxes apples (together Bob Alice apples now)): (n z 4)<br>this answer holds in a model where things are counted by number alone: a box holds any 3 apples | yes |
| `p/boxes12_readings.ein` | Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need? 12 apples, boxes of 3, with the readings of *need* imported: nothing is left over, so no reading is proposed and none is owed. | (needed boxes apples (together Bob Alice apples now)): (n z 4) | yes |
| `p/boxes12_always.ein` | Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need? 12 apples, boxes of 3, and a reading of *need* is proposed whatever the remainder: the readings agree. | (needed boxes apples (together Bob Alice apples now)): (n z 4)<br>need is read as pack-all<br>need is read as whole-only | no |
| `p/boxes13.ein` | Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need? 13 apples, boxes of 3: the word *need* has two readings, and the problem does not choose. The readings are hypotheses. | (needed boxes apples (together Bob Alice apples now)): (n z 4)<br>(needed boxes apples (together Bob Alice apples now)): (n z 5)<br>with need read as pack-all: (n z 5) boxes, and z apples left unpacked<br>with need read as whole-only: (n z 4) boxes, and (n z 1) apples left unpacked | no |
| `p/boxes13_always.ein` | Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need? 13 apples, boxes of 3: the word *need* has two readings, and the problem does not choose. The readings are hypotheses, proposed and owed for every *need* question (`readings_always`). | (needed boxes apples (together Bob Alice apples now)): (n z 4)<br>(needed boxes apples (together Bob Alice apples now)): (n z 5)<br>with need read as pack-all: (n z 5) boxes, and z apples left unpacked<br>with need read as whole-only: (n z 4) boxes, and (n z 1) apples left unpacked | no |
| `p/boxes13_plain.ein` | Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need? 13 apples, boxes of 3: the word *need* has two readings, and the problem does not choose. The readings are hypotheses, and nothing is owed (`readings_plain`): the state with no reading is in R and has no answer. | (needed boxes apples (together Bob Alice apples now)): (n z 4)<br>(needed boxes apples (together Bob Alice apples now)): (n z 5)<br>what need means decides [(needed boxes apples (together Bob Alice apples now))]: (n z 4) filled, (n z 1) left over | no |
| `p/boxes13_stated.ein` | Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need? Every apple must be packed. 13 apples, boxes of 3, and the problem says which reading of *need* it means. | with need read as pack-all: (n z 5) boxes, and z apples left unpacked | no |
| `p/boxes13_both.ein` | Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need? 13 apples, boxes of 3: the word *need* has two readings, and the problem does not choose. Both answers are derived, each marked with its reading (`readings_both`); nothing is guessed. | with need read as pack-all: (n z 5) boxes, and z apples left unpacked<br>with need read as whole-only: (n z 4) boxes, and (n z 1) apples left unpacked | yes |
| `p/boxes13_gap.ein` | Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need? 13 apples, boxes of 3: the word *need* has two readings, and the problem does not choose. The gap is named and no answer is given (`readings_gap`); nothing is guessed. | no answer to [(needed boxes apples (together Bob Alice apples now))]: (n z 4) are filled and (n z 1) left over, and the problem does not say whether need is read as pack-all<br>no answer to [(needed boxes apples (together Bob Alice apples now))]: (n z 4) are filled and (n z 1) left over, and the problem does not say whether need is read as whole-only | yes |
| `p/unused.ein` | Bob has 5 apples. Alice has 7 apples. Carol has 9 apples and a box that holds 3. Bob gave 2 apples to Alice. How many apples does Bob have? | (amount Bob apples t1): (n z 3)<br>(amount Bob apples t0): (n z 5) | yes |
| `p/bystander.ein` | Bob has 5 apples. Carol has 9 apples. Bob gave 2 apples to Alice. How many apples does Carol have? | (amount Carol apples t1): (n z 9) | yes |
| `q1/form_events.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have? Form 2 — an event is a nested fact, and a moment is the history that led to it: `start`, or `(after EVENT MOMENT)`. No moment is named and no order is stated: the order is the nesting. Self-contained: its three rules are the part of the word theory this problem needs, written for this form. | Bob has (n z 3) apples | yes |
| `q1/form_chain.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have? Form 3 — a chain: the sentences are numbered in the order of the text, each says one thing, and what holds is kept per sentence. Self-contained: its rules are the part of the word theory this problem needs, written for this form. | Bob has (n z 3) apples | yes |
| `p/gap_assume_ha.ein` | Bob has 5 apples. He gave 2 to Alice. How many apples do they have together? It may be supposed that Alice had none. The assumption is a hypothesis, and the gap is an obligation: root owes. | (together Bob Alice apples t1): (n z 5)<br>known: (amount Alice apples t1): (n z 2)<br>known: (amount Bob apples t1): (n z 3)<br>known: (amount Bob apples t0): (n z 5)<br>known: (together Bob Alice apples t1): (n z 5)<br>known: (amount Alice apples t0): z | no |
| `p/boxes12_plain.ein` | Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need? 12 apples, boxes of 3, with `readings_plain` imported: nothing is left over, so no reading is proposed and none is owed. | (needed boxes apples (together Bob Alice apples now)): (n z 4) | yes |

*search-free* is: H has no candidate and R is decided by root alone (the read-out's `universe` row, § 7).

## 1. How a problem is written

A problem has states and events in an order: *has*, then *gave*, then *how many now*. A fact carries no keyword of
its own for that, so the order is written in the facts. The same problem, *Bob has 5 apples. He gave 2 to Alice.
How many apples does Bob have?*, in three forms, each run:

**Form 1 — named moments; an event names the moment before it and the moment after.** This is the form the
prototype theory uses (`p/bob1.ein`).

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(has Bob apples (n z 5) t0)` |
| He gave 2 to Alice. | `(gave Bob Alice apples (n z 2) t0 t1)` |
| How many apples does Bob have? | `(asked (amount Bob apples t1))` |

```text
  query             (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 3) · ?q = (amount Bob apples t1)
                        “(amount Bob apples t1): (n z 3)”
```

**Form 2 — an event is a nested fact, and a moment is the history that led to it** (`q1/form_events.ein`, its
two rules written for this form): `start`, or `(after EVENT MOMENT)`. No moment is named and no order is stated.

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(has Bob apples (n z 5) start)` |
| He gave 2 to Alice. | `(happened (after (gave Bob Alice apples (n z 2)) start))` |
| How many apples does Bob have? | `(asked-amount Bob apples (after (gave Bob Alice apples (n z 2)) start))` |

```text
  query             (and (asked-amount ?p ?k ?t) (has ?p ?k ?n ?t))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?k = apples · ?n = (n z 3) · ?p = Bob · ?t = (after (gave Bob Alice apples (n z 2)) start)
                        “Bob has (n z 3) apples”
```

**Form 3 — a chain: the sentences are numbered in the order of the text** (`q1/form_chain.ein`, its four rules
written for this form): `(next s1 s2)`, `(says s2 (gave Bob Alice apples (n z 2)))`, and what holds is kept per
sentence, `(has-after s2 Bob apples (n z 3))`.

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(says s1 (has Bob apples (n z 5)))` |
| He gave 2 to Alice. | `(says s2 (gave Bob Alice apples (n z 2)))` |
| How many apples does Bob have? | `(says s3 (asks-amount Bob apples))` |

```text
  query             (answer ?p ?k ?n)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?k = apples · ?n = (n z 3) · ?p = Bob
                        “Bob has (n z 3) apples”
```

| | form 1, named moments | form 2, nested events | form 3, a chain of sentences |
|---|---|---|---|
| the answer, as the read-out's sentence | (amount Bob apples t1): (n z 3) | Bob has (n z 3) apples | Bob has (n z 3) apples |
| root firings (`ein saturate`) | 20 | 16 | 18 |
| a sentence is | one flat fact with its `:source` | a moment term; the event's sentence needs a carrier fact, `(happened …)` | one `(says sN …)` fact, exactly one per sentence |
| the order is | two names on the event, `t0 t1` | the nesting | `(next s1 s2)`, the text's own order |
| the question names its moment by | a name, `t1` | **repeating the whole history**, `(after (gave Bob Alice apples (n z 2)) start)` | its place in the text |
| a state that is stated late (*Alice had 4 before*) | `(has Alice apples (n z 4) t0)`, wherever it is written | a fact at `start`, wherever it is written | needs the tense read: sentence order is not time order |
| a rule that carries what did not change | one rule per event kind (`bystander-keeps`) | one rule per event kind | one rule per **sentence** kind: every sentence passes the state on |
| a join on a nested fact ([E-3](findings.md)) | on names | on the history term, which grows with the story | on names |

**Recommendation: form 1.** The facts stay flat, one sentence is one fact, a quantity is named by a person, a kind
and a moment, and the order costs two names on each event. Form 3 is the closest to the text, which is what M2
would produce first, and it is the form to translate *from*: its `(says sN …)` facts map to form 1 by naming a
moment at each event sentence. Form 2 needs no names and states no order, but every question and every answer
repeats the history, and two stories that share a prefix share it only as text.

## 2. The words' meaning as rules, and the plan explained

### 2.1 The shape of the theory

The theory, `p/words.ein`, works **backwards from the question and then forwards from the stated numbers**:

- **The plan.** `(asked Q)` makes the quantity Q *wanted*. A wanted quantity makes the quantities it is computed
  from wanted, and each such step is a stored fact, `(needs Q Q')`: *to know Q one must know Q'*. Which
  quantities those are is what the word means.
- **The computation.** Where the quantities a step needs have a value, the word's rule states a request to the
  calculator, `(add …)`, `(sub …)`, `(mul …)`, `(div …)` or `(cmp …)`, and a second rule reads the calculator's
  answer back as `(value Q N)`.

So a number is computed only if a question needs it, and the order of the operations is nowhere written: the
second request is derived from the first one's answer. A quantity is a nested fact of an ordinary relation,
`(amount Bob apples t1)`, `(together Bob Alice apples now)`, `(needed boxes apples (together Bob Alice apples
now))`; the value rule of 2026-11-11 allows it.

| the word | its rules, each with its `:why` |
|---|---|
| *has* | `has-states`: *the problem states it: {?p} has {?v} {?k} at {?t}* |
| *gave … to*, for the giver | `gave-plan-giver`: *{?p} gave {?k} away between {?t} and {?u}: what {?p} has at {?u} comes from what {?p} had at {?t}*<br>`gave-means-less`: *gave away means less: {?p} had {?a} and gave {?x}, so take {?x} from {?a}*<br>`giver-has-the-difference`: *{?a} - {?x} = {?d}: {?p} has {?d} {?k} at {?u}*<br>`giver-gave-too-many`: *{?p} had {?a} {?k} and gave {?x}: {?a} is short of {?x} by {?w}*<br>`gave-more-than-had-is-false`: *nobody gives away what they do not have: {?p} cannot give {?x} {?k} out of {?a}* |
| *gave … to*, for the receiver | `gave-plan-receiver`: *{?p} was given {?k} between {?t} and {?u}: what {?p} has at {?u} comes from what {?p} had at {?t}*<br>`was-given-means-more`: *was given means more: {?p} had {?a} and got {?x}, so add {?x} to {?a}*<br>`receiver-has-the-sum`: *{?a} + {?x} = {?s}: {?p} has {?s} {?k} at {?u}* |
| *gave … to*, for anyone else | `bystander-plan`: *{?p} neither gave nor got {?k} between {?t} and {?u}: what {?p} has is unchanged*<br>`bystander-keeps`: *nothing changed for {?p} between {?t} and {?u}: still {?a} {?k}* |
| *together* | `together-plan`: *together means a sum: first what {?p} has and what {?q} has, of {?k} alone, at {?t}*<br>`together-means-sum`: *together means a sum: {?p} has {?a} and {?q} has {?b}, so add them*<br>`together-is-the-sum`: *{?a} + {?b} = {?s}: together {?p} and {?q} have {?s} {?k}* |
| *who has more, and by how much* | `apart-plan`: *who has more means a comparison: first what {?p} has and what {?q} has*<br>`more-means-compare`: *who has more: compare {?a} with {?b}*<br>`first-has-more`: *{?a} is more than {?b}: {?p} has more; by how much means take {?b} from {?a}*<br>`second-has-more`: *{?a} is less than {?b}: {?q} has more; by how much means take {?a} from {?b}*<br>`neither-has-more`: *{?a} and {?b} are equal: neither has more*<br>`more-by-first`: *{?a} - {?b} = {?d}: {?p} has {?d} more {?k} than {?q}*<br>`more-by-second`: *{?b} - {?a} = {?d}: {?q} has {?d} more {?k} than {?p}* |
| *holds* | `holds-states`: *the problem states it: one of the {?c} holds {?m} {?k}* |
| *N boxes of M* | `packed-plan`: *the {?k} of {?p} are in {?c}: first how many {?c}, and how many {?k} one holds*<br>`of-means-times`: *{?a} {?c} of {?m} {?k} each means a product: multiply {?a} by {?m}*<br>`packed-is-the-product`: *{?a} x {?m} = {?w}: {?p} has {?w} {?k} at {?t}* |
| *there are*, *shared equally among* | `there-are-states`: *the problem states it: there are {?m} {?g}*<br>`rest-plan`: *what is left over is known once the sharing is done*<br>`share-plan`: *shared equally means a division: first how many there are to share, and among how many {?g}*<br>`shared-means-divide`: *shared equally means a division: {?a} among {?m} {?g}*<br>`each-gets-the-quotient`: *{?a} divided by {?m} = {?f}, remainder {?r}: each of the {?g} gets {?f}, and {?r} are left* |
| *need* | `need-plan`: *how many {?c} means a division: first how many {?k} there are to pack, and how many one of the {?c} holds*<br>`holds-means-divide`: *one of the {?c} holds {?m}, so the number of {?c} is a division: {?a} divided by {?m}*<br>`need-exact`: *{?a} divided by {?m} = {?f} and nothing is left over: {?f} {?c}, each full, whatever need means*<br>`need-left-over`: *{?a} divided by {?m} = {?f}, remainder {?r}: {?f} {?c} are filled and {?r} {?k} are left over, so the answer depends on what need means* |
| the question, and an assumption | `asked-is-wanted`: *the question asks [{?q}]*<br>`assumed-gives-value`: *the problem does not state [{?q}]; it is ASSUMED to be {?v}* |

Every rule and relation carries a `:why`; the full text is in [§ 12](#12-the-prototype-in-full).

### 2.2 The two-step problem, in full

*Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need?* (`p/boxes12.ein`)

| the sentence | the fact |
|---|---|
| Bob has 5 apples, | `(has Bob apples (n z 5) now)` |
| Alice has 7. | `(has Alice apples (n z 7) now)` |
| A box holds 3. | `(holds boxes apples (n z 3))` |
| How many boxes do they need? | `(asked (needed boxes apples (together Bob Alice apples now)))` |

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 4) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “(needed boxes apples (together Bob Alice apples now)): (n z 4)”
  as :expect        (model (asked (needed boxes apples (together Bob Alice apples now)))
                           (value (needed boxes apples (together Bob Alice apples now)) (n z 4)))
  :expect           holds
```

**The pupil's steps, read off the trace.** `ein solve boxes12.ein -G --trace out/boxes12/trace_G.md` writes
139 steps. `build/s1f.6.20/pupil.py` keeps the steps whose rule is declared in the prototype's
files and prints each one's `:why` and its first conclusion, **as the trace has them**:

**Root — 139 steps**

1. **`asked-is-wanted`** (step 2): the question asks [(needed boxes apples (together Bob Alice apples now))]  
   → `wanted(needed(boxes, apples, together(Bob, Alice, apples, now)))` — wanted: (needed boxes apples (together Bob Alice apples now))
2. **`need-plan`** (step 4): how many boxes means a division: first how many apples there are to pack, and how many one of the boxes holds  
   → `wanted(together(Bob, Alice, apples, now))` — wanted: (together Bob Alice apples now)
3. **`together-plan`** (step 5): together means a sum: first what Bob has and what Alice has, of apples alone, at now  
   → `wanted(amount(Bob, apples, now))` — wanted: (amount Bob apples now)
4. **`holds-states`** (step 6): the problem states it: one of the boxes holds (n z 3) apples  
   → `value(capacity(boxes, apples), n(z, 3))` — (capacity boxes apples): (n z 3)
5. **`has-states`** (step 7): the problem states it: Bob has (n z 5) apples at now  
   → `value(amount(Bob, apples, now), n(z, 5))` — (amount Bob apples now): (n z 5)
6. **`has-states`** (step 8): the problem states it: Alice has (n z 7) apples at now  
   → `value(amount(Alice, apples, now), n(z, 7))` — (amount Alice apples now): (n z 7)
7. **`together-means-sum`** (step 9): together means a sum: Bob has (n z 5) and Alice has (n z 7), so add them  
   → `add(n(z, 5), n(z, 7))` — asked: (n z 5) + (n z 7)
8. **`together-is-the-sum`** (step 21): (n z 5) + (n z 7) = (n (n z 1) 2): together Bob and Alice have (n (n z 1) 2) apples  
   → `value(together(Bob, Alice, apples, now), n(n(z, 1), 2))` — (together Bob Alice apples now): (n (n z 1) 2)
9. **`holds-means-divide`** (step 22): one of the boxes holds (n z 3), so the number of boxes is a division: (n (n z 1) 2) divided by (n z 3)  
   → `div(n(n(z, 1), 2), n(z, 3))` — asked: (n (n z 1) 2) divided by (n z 3)
10. **`need-exact`** (step 139): (n (n z 1) 2) divided by (n z 3) = (n z 4) and nothing is left over: (n z 4) boxes, each full, whatever need means  
   → `value(needed(boxes, apples, together(Bob, Alice, apples, now)), n(z, 4))` — (needed boxes apples (together Bob Alice apples now)): (n z 4)

*10 steps of the word theory, 129 of the calculator and the entry rules, 139 in all.*

Read as the pupil's reasoning: the question asks for boxes; boxes are a division, so first the apples to pack and
what a box holds (step 2); *they* means together, so first Bob's and Alice's apples (step 3); the three numbers are
stated (4 to 6); add 5 and 7 (7), which is 12 (8); divide 12 by 3 (9), which is 4 with nothing over (10).

- **Which operations:** the calculator was asked exactly two things, and each request carries the rule that
  asked (`ein saturate boxes12.ein --dump`):

```lisp
(add (n z 5) (n z 7) :rule together-means-sum)
(div (n (n z 1) 2) (n z 3) :rule holds-means-divide)
```

- **In what order:** nothing in the problem states it. `holds-means-divide` matches `(value (together Bob Alice
  apples now) ?a)`, which exists only after `together-is-the-sum` has read the sum, so the division is asked after the sum is read: steps 9, 21 and 22 of the trace.
- **Why:** each step's `:why` above, from the question (step 1) back to the three stated numbers (steps 4 to 6)
  and forward to the answer (step 10).
- **No rule fires for a computation that does not answer the question.** 5 x 3, 7 ÷ 3 and 7 − 5 are never
  requested: the list above is every `add`, `sub`, `mul`, `div` and `cmp` fact of the saturated state. What the
  division does inside itself is the calculator's: 3's multiples by 1 to 9, and the trial subtractions.

**How many steps are whose.** Of the 139 steps, **10 are the word
theory's** and 129 are the calculator's (two of those the entry rules
`arith` and `words`, which close the relations). The long division 12 ÷ 3 alone is steps 23 to 138.

**Is there a way today to show only the word theory's steps? No.** `--trace` has one pruning, `-l`, *prune each
derivation to the goal-relevant slice*, which keeps 137 of the
139 steps, since every step of the calculation is relevant to the answer. Nothing folds an
imported theory's steps into one line, *12 ÷ 3 = 4, remainder 0 (116 steps of `std.arith`)*, which is what a
pupil's page would show. The filter used here reads the trace file and is 42 lines
of Python; it is a finding, [E-17](findings.md), not a proposal to keep the script.

One more thing the trace does not show: a plan rule concludes several facts, `(wanted …)` and `(needs …)`, and
the *Derives* line names the first ([E-5](findings.md)). The `needs` facts, which are the plan, are in the state
(`ein saturate boxes12.ein --dump`):

```lisp
(needs (needed boxes apples (together Bob Alice apples now)) (together Bob Alice apples now) :rule need-plan)
(needs (needed boxes apples (together Bob Alice apples now)) (capacity boxes apples) :rule need-plan)
(needs (together Bob Alice apples now) (amount Bob apples now) :rule together-plan)
(needs (together Bob Alice apples now) (amount Alice apples now) :rule together-plan)
```


## 3. A missing quantity

*Bob has 5 apples. He gave 2 to Alice. How many apples do they have together?* (the user's). How many apples Alice
had before is not stated.

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(has Bob apples (n z 5) t0)` |
| He gave 2 to Alice. | `(gave Bob Alice apples (n z 2) t0 t1)` |
| How many apples do they have together? | `(asked (together Bob Alice apples t1))` |

With `words` alone the plan reaches `(wanted (amount Alice apples t0))`, no rule gives it a value, and nothing
more happens: Bob's 3 is derived, *together* is not, and no fact says why. The three mechanisms the stage lists,
each tried:

### 3.1 (b) The gap as a derived fact, by `absent` — `p/gap_b.ein`, the module `p/gaps.ein`

`gaps.ein` is a module of its own, so that `words` keeps no `absent`. Its rule: a quantity that is wanted, has no
value, and *needs* nothing (no sentence lets it be computed from something else) is `(not-stated Q)`; a question
that rests on it, through the plan's `needs` edges, is `(unanswered QUESTION Q)`.

```text
  ∅                 in R — root is alive and owes nothing
```

```text
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
```

```text
  query 1 of 3      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
  :expect           holds

  query 2 of 3      (value ?q ?n)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 3) · ?q = (amount Bob apples t1)
                        “known: (amount Bob apples t1): (n z 3)”
                      ?n = (n z 5) · ?q = (amount Bob apples t0)
                        “known: (amount Bob apples t0): (n z 5)”
  as :expect        (model (value (amount Bob apples t0) (n z 5))
                           (value (amount Bob apples t1) (n z 3)))
  :expect           holds

  query 3 of 3      (unanswered ?q ?g)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?g = (amount Alice apples t0) · ?q = (together Bob Alice apples t1)
                        “no answer to [(together Bob Alice apples t1)]: the problem does not state [(amount Alice apples t0)]”
  as :expect        (model (unanswered (together Bob Alice apples t1) (amount Alice apples t0)))
  :expect           holds
```

- It says all three things: **Bob has 3** (query 2), ***together* has no answer** (query 1, *the goal has no row*),
  and **the gap by name** (query 3).
- **Is it refused by *forbid*? No.** No refutation rests on the `absent`, and the program loads.
- **Is the module non-monotone? Not on this program.** H is empty, so no commitment can extend `value`, and the
  static check says `monotone` (`ein solve gap_b.ein --monotone`: *monotone yes — no site*). The strata row:
  `;; Strata — stratified, 2 strata` (`ein saturate gap_b.ein --dump`), where every problem without `gaps` has
  1 stratum. The calculator's one stratum still holds of the calculator.
- It **becomes** non-monotone the moment a hypothesis can supply a value (§ 3.5).

### 3.2 (a) The gap as an obligation: the state owes — `p/gap_a.ein`

The same test, concluding `(open value)` instead of a fact:

```lisp
(rule quantity-owed ()
  :match  (and (wanted ?q) (absent (needs ?q ?r)) (absent (value ?q ?v)))
  :assert (open value)
  :why    "the problem owes [{?q}] a value: no sentence states it")
```

```text
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every alive node owes — no commitment pays every debt
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 2 layers

  owing             ∅ — owes 1
                      the problem owes [(amount Alice apples t0)] a value: no sentence states it

  (region (owing (model)))

  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — R is empty: no state to answer from
  as :expect        (false)

  query 2 of 2      (value ?q ?n)
  over              all of R — R is empty: no state to answer from
  as :expect        (false)

  (solve :expect)   holds
```

- **The read-out names the gap** in its `owing` rows, with the rule's `:why`, and `--single` answers
  `Open — R is empty: every alive node owes`.
- **No query is answered.** R is empty, so *Bob has 3* is not said by any query: it is in root's fixpoint
  (`saturate.txt`), in the trace, and in the owing state's file, `out/gap_a/answer/owing/01.ein`. The stage's
  first requirement, *Bob's 3 is answered*, is not met by this mechanism alone.
- **A query cannot tell this from a contradiction.** The read-out offers `as :expect (false)` for each query,
  and a query's `:expect (false)` *holds* here, though root is alive; only the region claim tells the two apart,
  `(solve :expect (region (owing (model))))` holds and `(solve :expect (false))` fails with *(false) says every
  node is dead; R is empty, but alive nodes owe* (`witness/gap_a2x.ein`, `gap_a2y.ein`;
  [E-18](findings.md)).
- With the test written as `(and (wanted ?q) (absent (value ?q ?v)))` alone, root owes 3: *together*, Alice's
  apples after and Alice's apples before (`witness/gap_a1.ein`). The second `absent`, over `needs`, keeps the
  leaf.

### 3.3 (c) The unknown as a hypothesis candidate

**From the blind enumerator: nothing usable.** `words` closes `value`, so there is no candidate
(`witness/gap_c1.ein`, *universe: no candidate*). With `value` left open (`witness/gap_c1o.ein`), the enumerator
proposes pairs of the program's **names**, never a quantity and never a number, since both are nested facts:

```text
  universe          380 candidates
```

```text
H — the universe: 380 candidates over 1 relation, dead alone not known: this run did not visit every candidate alone
  value           380 candidates
    (value 0 1)
    (value 0 10)
    (value 0 2)
```

`(value 0 __closed__)` is among them ([E-1](findings.md)). A bare `ein solve` on it stops with one line and exit
1, *a state's commitments pass 2^128: the state route cannot count them*, as [`design.md`](design.md) records for
an unclosed theory. So: **numbers are not enumerable, and the blind enumerator does not try.**

**From a list of numbers the problem file states: it works, and it is a search** (`p/gap_c.ein`). The file states
`(guess z) (guess (n z 1)) (guess (n z 2))`, an hrule proposes `(value (amount ?p ?k ?t) ?v)` for each wanted
amount and each guess, and a rule kills a quantity with two values:

```text
  states (k_s)      7
  commitments (k_c) 8
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      5 states
  N                 20 minimal dead sets
  universe          12 candidates, 6 dead alone
  work              27 saturations · 772 facts beyond root · 4 layers
```

```text
  query             (and (asked ?q) (value ?q ?n))
  over              all of R — 7 states → 6 projection classes
    class 1 of 6    1 state (1) — the goal has no row: undetermined here
    class 2 of 6    1 state (6)
                      ?n = (n z 3) · ?q = (together Bob Alice apples t1)
                        “(together Bob Alice apples t1): (n z 3)”
    class 3 of 6    1 state (4)
                      ?n = (n z 4) · ?q = (together Bob Alice apples t1)
                        “(together Bob Alice apples t1): (n z 4)”
    class 4 of 6    2 states (5, 7)
                      ?n = (n z 5) · ?q = (together Bob Alice apples t1)
                        “(together Bob Alice apples t1): (n z 5)”
    class 5 of 6    1 state (2)
                      ?n = (n z 6) · ?q = (together Bob Alice apples t1)
                        “(together Bob Alice apples t1): (n z 6)”
    class 6 of 6    1 state (3)
                      ?n = (n z 7) · ?q = (together Bob Alice apples t1)
                        “(together Bob Alice apples t1): (n z 7)”
```

The read-out is *together* as a function of what is supposed: no answer with nothing supposed (class 1), and 3 to
7 under the suppositions. Three of the seven states (4, 5 and 6 in `out/gap_c/solve.txt`) suppose Alice's apples **after** the transfer directly, which
the hrule allows and a pupil would not. The list is the file's: nothing in the problem says Alice had at most 2.

### 3.4 The gap closed by a fact — `p/gap_fact.ein`

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(has Bob apples (n z 5) t0)` |
| Alice has 4 apples. | `(has Alice apples (n z 4) t0)` |
| He gave 2 to Alice. | `(gave Bob Alice apples (n z 2) t0 t1)` |
| How many apples does Bob have? | `(asked (amount Bob apples t1))` |
| How many does Alice have? | `(asked (amount Alice apples t1))` |
| How many apples do they have together? | `(asked (together Bob Alice apples t1))` |

```text
  query             (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 3) · ?q = (amount Bob apples t1)
                        “(amount Bob apples t1): (n z 3)”
                      ?n = (n z 6) · ?q = (amount Alice apples t1)
                        “(amount Alice apples t1): (n z 6)”
                      ?n = (n z 9) · ?q = (together Bob Alice apples t1)
                        “(together Bob Alice apples t1): (n z 9)”
```

3, 6 and 9, search-free, 51 firings.

### 3.5 The gap closed by a stated assumption

Two ways to write it, both run.

**As a fact with a marker of its own, `assumed`** (`p/gap_assume.ein`). `words` turns `(assumed Q N)` into a value
by a rule whose `:why` says ASSUMED, and `gaps` marks every question computed from it, `(rests-on QUESTION Q N)`:

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(has Bob apples (n z 5) t0)` |
| He gave 2 to Alice. | `(gave Bob Alice apples (n z 2) t0 t1)` |
| How many apples do they have together? | `(asked (together Bob Alice apples t1))` |
| ASSUMPTION, not in the problem: Alice had no apples. | `(assumed (amount Alice apples t0) z)` |

```text
  query 1 of 3      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 5) · ?q = (together Bob Alice apples t1)
                        “(together Bob Alice apples t1): (n z 5)”
  as :expect        (model (asked (together Bob Alice apples t1))
                           (value (together Bob Alice apples t1) (n z 5)))
  :expect           holds

  query 2 of 3      (rests-on ?q ?g ?v)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?g = (amount Alice apples t0) · ?q = (together Bob Alice apples t1) · ?v = z
                        “the answer to [(together Bob Alice apples t1)] rests on the assumption that [(amount Alice apples t0)] is z”
  as :expect        (model (rests-on (together Bob Alice apples t1) (amount Alice apples t0) z))
  :expect           holds

  query 3 of 3      (unanswered ?q ?g)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
  :expect           holds
```

The trace's step for the assumption, as written by the run:

```text
### Step 12 — `assumed-gives-value`

> the problem does not state [(amount Alice apples t0)]; it is ASSUMED to be z

Premises: `wanted(amount(Alice, apples, t0))`, `assumed(amount(Alice, apples, t0), z)` — from ASSUMPTION, not in the problem: Alice had no apples.

Derives `value(amount(Alice, apples, t0), z)` — (amount Alice apples t0): z
```

Search-free; the answer 5 is given, and the second query says what it rests on.

**As a hypothesis** (`p/gap_assume_h.ein`): the file states `(may-suppose (amount Alice apples t0) z)`, and an
hrule proposes the value. `gaps` is not imported.

```text
  states (k_s)      2
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      1 state — ⊤, the trivial line: no restriction bites
  lattice           monotone (the static check) — R is convex: no hole
  universe          1 candidate, 0 dead alone
  work              2 saturations · 27 facts beyond root · 3 layers
```

```text
  (region
    ;; state 1 of 2 — ∅, root itself · minimal
    (model (value (amount Bob apples t0) (n z 5))
           (value (amount Bob apples t1) (n z 3)))
    ;; state 2 of 2 — 1 candidate · maximal · from {(value (amount Alice apples t0) z)}
    (model (value (amount Alice apples t0) z)
           (value (amount Alice apples t1) (n z 2))
           (value (amount Bob apples t0) (n z 5))
           (value (amount Bob apples t1) (n z 3))
           (value (together Bob Alice apples t1) (n z 5))))

  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 2 states → 2 projection classes
    class 1 of 2    1 state (1) — the goal has no row: undetermined here
    class 2 of 2    1 state (2)
                      ?n = (n z 5) · ?q = (together Bob Alice apples t1)
                        “(together Bob Alice apples t1): (n z 5)”
  as :expect        (or (model)
                        (model (asked (together Bob Alice apples t1))
                               (value (together Bob Alice apples t1) (n z 5))))
  :expect           holds
```

This is the user's sentence as a region: **state 1, with nothing supposed, knows Bob's 3 and has no answer to
*together*; state 2, *from* the supposition, answers 5**, and the supposition is named in the state's header.
Monotone, two saturations.

**The two together are where it breaks** (`p/gap_assume_hb.ein`: the hypothesis, and `gaps` imported). The static
check flags the program, and the state that supposes the value still holds the fact that says the value is not
stated:

```text
  lattice           not monotone: 1 site (the static check) — no hole in this cube
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            gap-not-stated: absent reads value at absent depth 1, concludes a positive fact — at stake: the state
    status          monotone here (conservative) — the cube: no hole, no dead node with an alive superset, no discharged node with an owing superset
    state           stale facts — 1 of the cube's 2 nodes holds one: a fact whose absent guard the node's own fixpoint falsifies
    stale           {(value (amount Alice apples t0) z)}, in R: (not-stated (amount Alice apples t0)) ← gap-not-stated (absent (value _ _)) (absent (needs _ _))
    paths           no state the read-out prints has an alive parent to check
  universe          1 candidate, 0 dead alone
  route             2, the cube — no licence: the static check flags the program
```

```text
  query 2 of 2      (unanswered ?q ?g)
  over              all of R — 2 states → 1 projection class
    class 1 of 1    2 states (1, 2)
                      ?g = (amount Alice apples t0) · ?q = (together Bob Alice apples t1)
                        “no answer to [(together Bob Alice apples t1)]: the problem does not state [(amount Alice apples t0)]”
  as :expect        (model (unanswered (together Bob Alice apples t1) (amount Alice apples t0)))
```

Query 2 says, of state 2, that *together* has no answer for want of Alice's apples, in the state that supposes
them and answers 5. That is a **stale fact**, the read-out's own word: a node is a fork of root's fixpoint, and
the fork keeps what root derived under an `absent` its own commitment falsifies. It is the open question of
[F21](../../followups/f21_non_monotone_programs_decided.md) (Q-M1e2.2), met here on a program a user would
write. *forbid* does not refuse it, since no refutation rests on the `absent`.

**The obligation and the hypothesis together are monotone** (`p/gap_assume_ha.ein`): the obligation's trigger
reads `wanted`, which no commitment changes.

```text
  states (k_s)      1
  ∅                 not in R — root owes 1 instance
  lattice           monotone (the static check) — R is convex: no hole
  universe          1 candidate, 0 dead alone
```

R is the one state that supposes; root owes and is not in R. **But the read-out does not say what root owes**:
the `owing` rows of § 3.2 are printed only where R is empty. Here the instance and its `:why` are in `--events`
alone ([E-19](findings.md)):

```json
{"e": "owe", "n": 174, "rule": "quantity-owed", "activator": [], "relation": "value", "bindings": [["q", "(amount Alice apples t0)"]], "why": "the problem owes [(amount Alice apples t0)] a value: no sentence states it"}
```


### 3.6 What each mechanism does

| | Bob has 3 | *together* is not answered | the gap is named | an assumption | lattice | search |
|---|---|---|---|---|---|---|
| **(b)** derived by `absent` (`gap_b`, `gap_assume`) | a query row | *the goal has no row* | a query row, `unanswered` | a marked fact; `rests-on` names it beside the answer | monotone, 2 strata | none |
| **(a)** an obligation (`gap_a`) | not by a query: R is empty | R is empty | the `owing` rows | — | monotone | none |
| **(a)** with the assumption a hypothesis (`gap_assume_ha`) | in the one state, under the supposition | — | **not printed**: only *root owes 1 instance* | the state's `from {…}` | monotone | 2 saturations |
| **(c)** hypothesis, no gap rule (`gap_assume_h`) | state 1 | state 1, *undetermined here* | not named | the state's `from {…}` | monotone | 2 saturations |
| (b) and a hypothesis (`gap_assume_hb`) | state 1 | state 1 | named, and **stale in state 2** | the state's `from {…}` | flagged, 1 site | the cube |
| (c) blind (`gap_c1o`) | — | — | — | 380 candidates over names | — | does not finish |

**Recommendation.** For a missing quantity: **(b)**, in a module of its own, with an assumption written as a
marked fact (`gap_b`, `gap_assume`). It is the only form that says all three things in the read-out, it is
search-free, and it stays monotone as long as no hypothesis supplies a quantity. The hypothesis form
(`gap_assume_h`) reads closest to the user's sentence and is the same idea as the readings of § 4; it cannot yet
be combined with a named gap without (b)'s stale fact or (a)'s unprinted debt, and which of F21 or E-19 to wait
for is a question for the stop.

## 4. A missing reading

*Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need?* Every number is stated; 13 ÷ 3 is 4
and 1 over; the word *need* does not say whether the answer is 4 or 5.

| the sentence | the fact |
|---|---|
| Bob has 5 apples, | `(has Bob apples (n z 5) now)` |
| Alice has 8. | `(has Alice apples (n z 8) now)` |
| A box holds 3. | `(holds boxes apples (n z 3))` |
| How many boxes do they need? | `(asked (needed boxes apples (together Bob Alice apples now)))` |

In `words`, the rule `need-exact` answers where nothing is left over. Where something is, `need-left-over` derives
`(left-over QUANTITY 4 1)` and stops: *the answer depends on what need means*.

### 4.1 Readings as hypotheses — `p/readings.ein`, `p/boxes13.ein`

The user's lean, built first. The module holds the readings as facts, each with its sentence:

| the fact | its `:source` |
|---|---|
| `(reading-of need whole-only)` | need = how many containers can be filled; what does not fill one is left out |
| `(reading-of need pack-all)` | need = the least number of containers that hold everything; the last may be part full |

`(reads-as need R)` is the fact the search may commit to: an hrule proposes it for each reading the theory holds,
a rule kills two readings at once, each reading is a rule that answers under it, and a state with something left
over and no reading **owes** one. The problem file adds one line, `(solve :hrules (read-need))`.

```text
  states (k_s)      2
  commitments (k_c) 2
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 1 instance
  minimal line      2 states, from 2 commitments — the least guess is 1 candidate
  maximal line      2 states
  components        2, each a single commitment
  sub-cubes         2, each of dimension 0
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 1 minimal dead set
  E                 2 minimal discharging sets
  lattice           not monotone: 1 site (the static check) — no hole in this cube
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            reading-owed: obligation-trigger reads left-over at absent depth 0, concludes (open …) — at stake: the status: owes
    status          monotone here (conservative) — the cube: no hole, no dead node with an alive superset, no discharged node with an owing superset
    state           no stale fact in the cube's 4 nodes
    paths           no state the read-out prints has an alive parent to check
  universe          2 candidates, 0 dead alone
  route             2, the cube — no licence: the static check flags the program
  work              4 saturations · 37 facts beyond root

  (region
    ;; state 1 of 2 — 1 candidate · minimal and maximal · from {(reads-as need pack-all)}
    (model (reads-as need pack-all))
    ;; state 2 of 2 — 1 candidate · minimal and maximal · from {(reads-as need whole-only)}
    (model (reads-as need whole-only)))

  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 2 states → 2 projection classes
    class 1 of 2    1 state (2)
                      ?n = (n z 4) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “(needed boxes apples (together Bob Alice apples now)): (n z 4)”
    class 2 of 2    1 state (1)
                      ?n = (n z 5) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “(needed boxes apples (together Bob Alice apples now)): (n z 5)”
  as :expect        (or (model (asked (needed boxes apples (together Bob Alice apples now)))
                               (value (needed boxes apples (together Bob Alice apples now)) (n z 4)))
                        (model (asked (needed boxes apples (together Bob Alice apples now)))
                               (value (needed boxes apples (together Bob Alice apples now)) (n z 5))))
  :expect           holds

  query 2 of 2      (and (asked ?q) (value ?q ?n) (value (over ?q) ?o) (reads-as need ?m))
  over              all of R — 2 states → 2 projection classes
    class 1 of 2    1 state (1)
                      ?m = pack-all · ?n = (n z 5) · ?o = z · ?q = (needed boxes apples (together Bob Alice apples now))
                        “with need read as pack-all: (n z 5) boxes, and z apples left unpacked”
    class 2 of 2    1 state (2)
                      ?m = whole-only · ?n = (n z 4) · ?o = (n z 1) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “with need read as whole-only: (n z 4) boxes, and (n z 1) apples left unpacked”
  as :expect        (or (model (asked (needed boxes apples (together Bob Alice apples now)))
                               (reads-as need pack-all)
                               (value (needed boxes apples (together Bob Alice apples now)) (n z 5))
                               (value (over (needed boxes apples (together Bob Alice apples now))) z))
                        (model (asked (needed boxes apples (together Bob Alice apples now)))
                               (reads-as need whole-only)
                               (value (needed boxes apples (together Bob Alice apples now)) (n z 4))
                               (value (over (needed boxes apples (together Bob Alice apples now))) (n z 1))))
  :expect           holds
```

- **The region's states are the readings**, two of them, each `minimal and maximal`, each *from* its one
  commitment. Root is not in R: it owes.
- **The projection classes are the two answers**: 4 boxes and 1 apple left unpacked; 5 boxes and none.
- **The reading is named in each answer** twice: in the state's header, `from {(reads-as need pack-all)}`, and,
  where the query asks for it, as a binding, `?m = pack-all` (query 2). Query 1, which does not ask, still gets
  two classes, since the values differ.
- **What root owes is not printed** (E-19 again): *root owes 1 instance*, and the `:why`, *the problem does not say what need means, and it matters: (n z 4) boxes are filled and (n z 1) apples are left over*, is in `--events` only.

The word theory's steps, root's and each state's (`out/boxes13/pupil.md`):

**Root — 141 steps**

1. **`asked-is-wanted`** (step 2): the question asks [(needed boxes apples (together Bob Alice apples now))]  
   → `wanted(needed(boxes, apples, together(Bob, Alice, apples, now)))` — wanted: (needed boxes apples (together Bob Alice apples now))
2. **`need-plan`** (step 5): how many boxes means a division: first how many apples there are to pack, and how many one of the boxes holds  
   → `wanted(together(Bob, Alice, apples, now))` — wanted: (together Bob Alice apples now)
3. **`together-plan`** (step 6): together means a sum: first what Bob has and what Alice has, of apples alone, at now  
   → `wanted(amount(Bob, apples, now))` — wanted: (amount Bob apples now)
4. **`holds-states`** (step 7): the problem states it: one of the boxes holds (n z 3) apples  
   → `value(capacity(boxes, apples), n(z, 3))` — (capacity boxes apples): (n z 3)
5. **`has-states`** (step 8): the problem states it: Bob has (n z 5) apples at now  
   → `value(amount(Bob, apples, now), n(z, 5))` — (amount Bob apples now): (n z 5)
6. **`has-states`** (step 9): the problem states it: Alice has (n z 8) apples at now  
   → `value(amount(Alice, apples, now), n(z, 8))` — (amount Alice apples now): (n z 8)
7. **`together-means-sum`** (step 10): together means a sum: Bob has (n z 5) and Alice has (n z 8), so add them  
   → `add(n(z, 5), n(z, 8))` — asked: (n z 5) + (n z 8)
8. **`together-is-the-sum`** (step 22): (n z 5) + (n z 8) = (n (n z 1) 3): together Bob and Alice have (n (n z 1) 3) apples  
   → `value(together(Bob, Alice, apples, now), n(n(z, 1), 3))` — (together Bob Alice apples now): (n (n z 1) 3)
9. **`holds-means-divide`** (step 23): one of the boxes holds (n z 3), so the number of boxes is a division: (n (n z 1) 3) divided by (n z 3)  
   → `div(n(n(z, 1), 3), n(z, 3))` — asked: (n (n z 1) 3) divided by (n z 3)
10. **`need-left-over`** (step 141): (n (n z 1) 3) divided by (n z 3) = (n z 4), remainder (n z 1): (n z 4) boxes are filled and (n z 1) apples are left over, so the answer depends on what need means  
   → `left-over(needed(boxes, apples, together(Bob, Alice, apples, now)), n(z, 4), n(z, 1))` — [(needed boxes apples (together Bob Alice apples now))]: (n z 4) are filled, and (n z 1) are left over

*10 steps of the word theory, 131 of the calculator and the entry rules, 141 in all.*

**State 1 of 2 — 1 candidate · minimal and maximal · from {(reads-as need pack-all)} — 13 steps**

1. **`need-pack-all-plan`** (step 1): need read as pack-all: (n z 4) boxes do not hold everything, since (n z 1) apples are left over; one more does, and need asks for the least number, so add 1 to (n z 4)  
   → `add(n(z, 4), n(z, 1))` — asked: (n z 4) + (n z 1)
2. **`need-pack-all`** (step 13): (n z 4) + (n z 1) = (n z 5): (n z 5) boxes hold everything, the last one part full, and no fewer do  
   → `value(needed(boxes, apples, together(Bob, Alice, apples, now)), n(z, 5))` — (needed boxes apples (together Bob Alice apples now)): (n z 5)

*2 steps of the word theory, 11 of the calculator and the entry rules, 13 in all.*

**State 2 of 2 — 1 candidate · minimal and maximal · from {(reads-as need whole-only)} — 1 step**

1. **`need-whole-only`** (step 1): need read as whole-only: (n z 4) boxes are filled, and (n z 1) apples stay unpacked  
   → `value(needed(boxes, apples, together(Bob, Alice, apples, now)), n(z, 4))` — (needed boxes apples (together Bob Alice apples now)): (n z 4)

*1 steps of the word theory, 0 of the calculator and the entry rules, 1 in all.*

**Need as the least number that suffices** is said in `need-pack-all-plan`'s `:why`, above: 4 boxes do not hold
everything, one more does, and need asks for the least. That 13 boxes of one apple each also hold everything is
not derived and not refuted: the theory computes one number per reading and the text says why it is the least.

**The lattice is flagged.** The obligation's trigger reads `left-over`, which is derived from `value`, which a
commitment extends; so the static check flags it, the cube is enumerated (4 nodes), and every read-out carries the
`assumes` row that names F21. It finds no hole and no stale fact here.

### 4.2 When the readings agree: 12 ÷ 3

With `readings` imported and 12 apples (`p/boxes12_readings.ein`), nothing is left over, the hrule proposes
nothing and nothing is owed: the problem is search-free and `need-exact` says *whatever need means*.

```text
  states (k_s)      1
  ∅                 in R — root is alive and owes nothing
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
```

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 4) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “(needed boxes apples (together Bob Alice apples now)): (n z 4)”
  as :expect        (model (asked (needed boxes apples (together Bob Alice apples now)))
                           (value (needed boxes apples (together Bob Alice apples now)) (n z 4)))
  :expect           holds

  query 2 of 2      (reads-as need ?m)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
  :expect           holds
```

The other choice is to propose and owe a reading for **every** *need* question, whatever is left over
(`p/readings_always.ein`, `p/boxes12_always.ein`). Then the read-out itself says that the answer does not depend
on the choice: **2 states, 1 projection class**.

```text
  states (k_s)      2
  ∅                 not in R — root owes 1 instance
  lattice           monotone (the static check) — R is convex: no hole
  universe          2 candidates, 0 dead alone
  work              4 saturations · 5 facts beyond root · 3 layers
```

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 2 states → 1 projection class
    class 1 of 1    2 states (1, 2)
                      ?n = (n z 4) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “(needed boxes apples (together Bob Alice apples now)): (n z 4)”
  as :expect        (model (asked (needed boxes apples (together Bob Alice apples now)))
                           (value (needed boxes apples (together Bob Alice apples now)) (n z 4)))
  :expect           holds

  query 2 of 2      (reads-as need ?m)
  over              all of R — 2 states → 2 projection classes
    class 1 of 2    1 state (1)
                      ?m = pack-all
                        “need is read as pack-all”
    class 2 of 2    1 state (2)
                      ?m = whole-only
                        “need is read as whole-only”
  as :expect        (or (model (reads-as need pack-all))
                        (model (reads-as need whole-only)))
  :expect           holds
```

### 4.3 The three ways to write readings as hypotheses

All three give the two answers on 13 apples. They differ in what root is and in what the static check says.

| | `readings` (owed where it matters) | `readings_always` (always owed) | `readings_plain` (never owed) |
|---|---|---|---|
| 13 apples: file | `boxes13` | `boxes13_always` | `boxes13_plain` |
| states of R | 2 | 2 | 3 |
| ∅ | not in R — root owes 1 instance | not in R — root owes 1 instance | in R — root is alive and owes nothing |
| classes of the question | 2: 4, and 5 | 2: 4, and 5 | 3: *undetermined here*, 4, and 5 |
| lattice | flagged, 1 site; the cube | monotone | monotone |
| 12 apples | search-free, 1 state (`boxes12_readings`) | 2 states, 1 class (`boxes12_always`) | search-free, 1 state (`boxes12_plain`) |
| the gap is named by | the debt's `:why`, in `--events` only | the debt's `:why`, in `--events` only | a derived fact, `(reading-matters need Q 4 1)`, a query row |

`readings_plain` on 13 apples:

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 3 states → 3 projection classes
    class 1 of 3    1 state (1) — the goal has no row: undetermined here
    class 2 of 3    1 state (3)
                      ?n = (n z 4) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “(needed boxes apples (together Bob Alice apples now)): (n z 4)”
    class 3 of 3    1 state (2)
                      ?n = (n z 5) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “(needed boxes apples (together Bob Alice apples now)): (n z 5)”
  as :expect        (or (model)
                        (model (asked (needed boxes apples (together Bob Alice apples now)))
                               (value (needed boxes apples (together Bob Alice apples now)) (n z 4)))
                        (model (asked (needed boxes apples (together Bob Alice apples now)))
                               (value (needed boxes apples (together Bob Alice apples now)) (n z 5))))
  :expect           holds

  query 2 of 2      (reading-matters ?w ?q ?f ?r)
  over              all of R — 3 states → 1 projection class
    class 1 of 1    3 states (1, 2, 3)
                      ?f = (n z 4) · ?q = (needed boxes apples (together Bob Alice apples now)) · ?r = (n z 1) · ?w = need
                        “what need means decides [(needed boxes apples (together Bob Alice apples now))]: (n z 4) filled, (n z 1) left over”
  as :expect        (model (reading-matters need (needed boxes apples (together Bob Alice apples now)) (n z 4) (n z 1)))
  :expect           holds
```

**A reading the problem states** closes the gap as a fact does (`p/boxes13_stated.ein`, with *Every apple must be
packed.* as `(reads-as need pack-all)`): the other reading is proposed and dies with root's.

```text
  states (k_s)      1
  ∅                 in R — root is alive and owes nothing
  N                 1 minimal dead set
  universe          1 candidate, 1 dead alone
```

```text
  query             (and (asked ?q) (value ?q ?n) (value (over ?q) ?o) (reads-as need ?m))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?m = pack-all · ?n = (n z 5) · ?o = z · ?q = (needed boxes apples (together Bob Alice apples now))
                        “with need read as pack-all: (n z 5) boxes, and z apples left unpacked”
```

### 4.4 The other two forms, for comparison

**Two derived answers, each marked with its reading** (`p/readings_both.ein`, `p/boxes13_both.ein`): no
hypothesis; where something is left over both readings are computed, each a fact `(answer-under Q READING N OVER)`.

```text
  states (k_s)      1
  universe          no candidate: R is {∅} or empty, decided by root alone
```

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
  :expect           holds

  query 2 of 2      (and (asked ?q) (answer-under ?q ?m ?n ?o))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?m = pack-all · ?n = (n z 5) · ?o = z · ?q = (needed boxes apples (together Bob Alice apples now))
                        “with need read as pack-all: (n z 5) boxes, and z apples left unpacked”
                      ?m = whole-only · ?n = (n z 4) · ?o = (n z 1) · ?q = (needed boxes apples (together Bob Alice apples now))
                        “with need read as whole-only: (n z 4) boxes, and (n z 1) apples left unpacked”
  as :expect        (model (answer-under (needed boxes apples (together Bob Alice apples now)) pack-all (n z 5) z)
                           (answer-under (needed boxes apples (together Bob Alice apples now)) whole-only (n z 4) (n z 1))
                           (asked (needed boxes apples (together Bob Alice apples now))))
  :expect           holds
```

**A named gap and no answer** (`p/readings_gap.ein`, `p/boxes13_gap.ein`): the gap is a derived fact and the
readings the theory holds are listed beside it.

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
  :expect           holds

  query 2 of 2      (and (reading-matters ?w ?q ?f ?r) (reading-of ?w ?m))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?f = (n z 4) · ?m = pack-all · ?q = (needed boxes apples (together Bob Alice apples now)) · ?r = (n z 1) · ?w = need
                        “no answer to [(needed boxes apples (together Bob Alice apples now))]: (n z 4) are filled and (n z 1) left over, and the problem does not say whether need is read as pack-all”
                      ?f = (n z 4) · ?m = whole-only · ?q = (needed boxes apples (together Bob Alice apples now)) · ?r = (n z 1) · ?w = need
                        “no answer to [(needed boxes apples (together Bob Alice apples now))]: (n z 4) are filled and (n z 1) left over, and the problem does not say whether need is read as whole-only”
  as :expect        (model (reading-matters need (needed boxes apples (together Bob Alice apples now)) (n z 4) (n z 1))
                           (reading-of need pack-all)
                           (reading-of need whole-only))
  :expect           holds
```

| | hypotheses (§ 4.1) | both answers derived | a named gap |
|---|---|---|---|
| the two answers | two states, two classes | two rows of one class, in one state | none |
| the reading named | the state's commitment, and a binding | an argument of the fact | listed, not applied |
| a later quantity computed from the answer | follows in each state, by the same rules | needs every later rule doubled, or a `value` per reading | nothing to compute from |
| search | 4 saturations · 37 facts beyond root | 1 saturation · 0 facts beyond root · 2 layers | 1 saturation · 0 facts beyond root · 2 layers |
| root firings | 141 | 155 (the +1 is computed whether or not it is wanted) | 142 |

**Recommendation: readings as hypotheses, in the form of `readings_plain`**, unless the user wants root out of R.
It is monotone, it is search-free where the readings agree, its state with no reading is the honest partial
answer (4 filled, 1 over, and *what need means decides*), and it names the gap in a query row with no `absent`
and no obligation. It is also the same shape as the hypothesis form of a missing quantity (`gap_assume_h`): **∅
is what is known with nothing supposed, and every other state is one supposition, named in its header.**
`readings_always` is the alternative if R should hold only the readings: monotone too, at the price that no
*need* question is search-free. `readings`, owed only where it matters, is the form the stage file describes, and
it is the one the static check flags.

**What this is a search of** is the user's distinction, kept: the candidates are facts about a word, `(reads-as
need pack-all)`, under rules that stay fixed. Nothing here proposes a rule.

## 5. The model's boundary

What the model leaves out is the theory's to state, once. **How, so that a run prints it:** a string is written
only after `:why`, `:source` or `:goal-text`, so the text has three places, and the prototype uses all three:

```lisp
(relation model-counts-only
  :why "in this model things are counted by number alone: a thing has no size, weight, shape or price, any thing of a kind fits any container that holds that kind, and a person can give away only what they have")
(model-counts-only
  :source "the model of the theory `words`: things are counted by number alone (no size, weight, shape or price), any thing of a kind fits any container that holds that kind, and nobody gives away what they do not have")
```

- **The fact's `:source`** is printed by `--trace` on every step that takes the fact as a premise, and by the
  read-out's `source` rows of a core. `need-exact` and `gave-more-than-had-is-false` take it as a premise, so an
  answer about boxes, and a refutation about giving, each cite the model. The last step of the two-step problem
  (`out/boxes12/trace_G.md`):

```text
### Step 139 — `need-exact`

> (n (n z 1) 2) divided by (n z 3) = (n z 4) and nothing is left over: (n z 4) boxes, each full, whatever need means

Premises: `wanted(needed(boxes, apples, together(Bob, Alice, apples, now)))`, `value(together(Bob, Alice, apples, now), n(n(z, 1), 2))`, `value(capacity(boxes, apples), n(z, 3))`, `quot-of(n(n(z, 1), 2), n(z, 3), n(z, 4), z)`, `model-counts-only` — from the model of the theory `words`: things are counted by number alone (no size, weight, shape or price), any thing of a kind fits any container that holds that kind, and nobody gives away what they do not have

Derives `value(needed(boxes, apples, together(Bob, Alice, apples, now)), n(z, 4))` — (needed boxes apples (together Bob Alice apples now)): (n z 4)
```

- **The relation's `:why`** is printed after the fact in a `--dump-answer` state file
  (`out/boxes12/answer/models/01.ein`):

```text
(model-counts-only) ; source "the model of the theory `words`: things are counted by number alone (no size, weight, shape or price), any thing of a kind fits any container that holds that kind, and nobody gives away what they do not have" — in this model things are counted by number alone: a thing has no size, weight, shape or price, any thing of a kind fits any container that holds that kind, and a person can give away only what they have
```

- **A query's `:goal-text`** prints it in the read-out, beside the answer (`p/boxes12.ein`'s second query):

```text
  query 2 of 2      (model-counts-only)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      the goal holds: it has no variable
                        “this answer holds in a model where things are counted by number alone: a box holds any 3 apples”
```

**Recommendation:** the fact with a `:source`, taken as a premise by the rules whose conclusion depends on the
assumption. It is stated once, in the theory; the trace of an answer cites it exactly where it is used; a
contradiction's core lists it (§ 6.1), which is right, since *nobody gives away what they do not have* is the
model's and not the problem's. A problem that wants it in the read-out adds the one-line query. The prototype has
one such fact; a theory that ships would have one per assumption, so that an answer cites only those it used.

## 6. The other problems

### 6.1 A contradictory problem — `p/contra.ein`

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(has Bob apples (n z 5) t0)` |
| He gave 7 to Alice. | `(gave Bob Alice apples (n z 7) t0 t1)` |
| How many apples does Bob have? | `(asked (amount Bob apples t1))` |

```text
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every node is dead — root's own fixpoint derives (false)
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 1 layer

  unsat core        (asked (amount Bob apples t1))
  unsat core        (fs 7 5 0 2 0)
  unsat core        (gave Bob Alice apples (n z 7) t0 t1)
  unsat core        (has Bob apples (n z 5) t0)
  unsat core        (model-counts-only)
  unsat core        (nz 2)
  source            (asked (amount Bob apples t1)) :source "How many apples does Bob have?"
  source            (gave Bob Alice apples (n z 7) t0 t1) :source "He gave 7 to Alice."
  source            (has Bob apples (n z 5) t0) :source "Bob has 5 apples."
  source            (model-counts-only) :source "the model of the theory `words`: things are counted by number alone (no size, weight, shape or price), any thing of a kind fits any container that holds that kind, and nobody gives away what they do not have"
  refuted by        gave-more-than-had-is-false

  (false)

  query             (and (asked ?q) (value ?q ?n))
  over              all of R — R is empty: no state to answer from
  as :expect        (false)

  (solve :expect)   holds
```

As the user ruled for division by zero: **a flag, then `(false)`**. `giver-gave-too-many` derives the flag
`(gave-more-than-had Bob apples 5 7)` from the calculator's answer, 5 − 7 = nothing, short by 2, and a second rule
concludes `(false)` from the flag and the model's assumption. Root is dead, R is empty, and the core names **the
two sentences** by their `:source`, the question that made Bob's apples wanted, and the model's assumption. It
also names two table rows of the calculator, `(fs 7 5 0 2 0)` and `(nz 2)`: a refutation that passes through
arithmetic lists the rows it used, the risk [P1f.6](README.md) already records, and F22's to make canonical.
Without the question there is no contradiction: nothing is computed that is not asked, so *Bob has 5 and gave 7*
alone derives nothing (`witness/contra_no_question.ein`: root is alive, and `ein saturate` counts 2 firings, the
two entry rules). The word theory's steps:

**Root — 21 steps**

1. **`asked-is-wanted`** (step 2): the question asks [(amount Bob apples t1)]  
   → `wanted(amount(Bob, apples, t1))` — wanted: (amount Bob apples t1)
2. **`gave-plan-giver`** (step 4): Bob gave apples away between t0 and t1: what Bob has at t1 comes from what Bob had at t0  
   → `wanted(amount(Bob, apples, t0))` — wanted: (amount Bob apples t0)
3. **`has-states`** (step 5): the problem states it: Bob has (n z 5) apples at t0  
   → `value(amount(Bob, apples, t0), n(z, 5))` — (amount Bob apples t0): (n z 5)
4. **`gave-means-less`** (step 6): gave away means less: Bob had (n z 5) and gave (n z 7), so take (n z 7) from (n z 5)  
   → `sub(n(z, 5), n(z, 7))` — asked: (n z 5) - (n z 7)
5. **`giver-gave-too-many`** (step 20): Bob had (n z 5) apples and gave (n z 7): (n z 5) is short of (n z 7) by (n z 2)  
   → `gave-more-than-had(Bob, apples, n(z, 5), n(z, 7))` — Bob gave (n z 7) apples and had only (n z 5)
6. **`gave-more-than-had-is-false`** (step 21): nobody gives away what they do not have: Bob cannot give (n z 7) apples out of (n z 5)  
   → `false`.

*6 steps of the word theory, 15 of the calculator and the entry rules, 21 in all.*

A transfer of more than one has needs a refusal or a negative number (S1f.6.7); the prototype refuses.

### 6.2 Two kinds of thing — `p/kinds.ein`, `p/kinds_full.ein`, `p/kinds_assume.ein`

*Bob has 5 apples, Alice has 3 pears. How many apples do they have together?*

| the sentence | the fact |
|---|---|
| Bob has 5 apples, | `(has Bob apples (n z 5) now)` |
| Alice has 3 pears. | `(has Alice pears (n z 3) now)` |
| How many apples do they have together? | `(asked (together Bob Alice apples now))` |

```text
  query 1 of 3      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
  :expect           holds

  query 2 of 3      (value ?q ?n)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 5) · ?q = (amount Bob apples now)
                        “known: (amount Bob apples now): (n z 5)”
  as :expect        (model (value (amount Bob apples now) (n z 5)))
  :expect           holds

  query 3 of 3      (unanswered ?q ?g)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?g = (amount Alice apples now) · ?q = (together Bob Alice apples now)
                        “no answer to [(together Bob Alice apples now)]: the problem does not state [(amount Alice apples now)]”
  as :expect        (model (unanswered (together Bob Alice apples now) (amount Alice apples now)))
  :expect           holds
```

**The program does not answer 5.** The sum is over apples alone, as the stage asks, and how many apples Alice has
is not stated: that she has none is the same silent assumption as *Alice had 0 before*. With the assumption
written (`p/kinds_assume.ein`: *the problem lists everything each has, so Alice has no apples*):

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 5) · ?q = (together Bob Alice apples now)
                        “(together Bob Alice apples now): (n z 5)”
  as :expect        (model (asked (together Bob Alice apples now))
                           (value (together Bob Alice apples now) (n z 5)))
  :expect           holds

  query 2 of 2      (rests-on ?q ?g ?v)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?g = (amount Alice apples now) · ?q = (together Bob Alice apples now) · ?v = z
                        “the answer to [(together Bob Alice apples now)] rests on the assumption that [(amount Alice apples now)] is z”
  as :expect        (model (rests-on (together Bob Alice apples now) (amount Alice apples now) z))
  :expect           holds
```

And with both kinds stated for both (`p/kinds_full.ein`: Bob 5 apples and 2 pears, Alice 3 apples and 4 pears),
the pears are never wanted:

```text
  query 2 of 2      (value ?q ?n)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 3) · ?q = (amount Alice apples now)
                        “(amount Alice apples now): (n z 3)”
                      ?n = (n z 5) · ?q = (amount Bob apples now)
                        “(amount Bob apples now): (n z 5)”
                      ?n = (n z 8) · ?q = (together Bob Alice apples now)
                        “(together Bob Alice apples now): (n z 8)”
```

Whether *has 3 pears* should be read as *has 3 pears and nothing else* is a reading of *has* (§ 8).

### 6.3 The pencils — `p/pencils.ein`

| the sentence | the fact |
|---|---|
| A school has 12 boxes of pencils, | `(has School cartons (n (n z 1) 2) t0)` |
| 144 pencils in each. | `(holds cartons pencils (n (n (n z 1) 4) 4))` |
| A school has 12 boxes of pencils, 144 pencils in each. | `(packed-in School pencils cartons t0)` |
| 250 pencils go to the office. | `(gave School Office pencils (n (n (n z 2) 5) 0) t0 t1)` |
| The rest is shared equally among 31 classes. | `(there-are classes (n (n z 3) 1))` |
| How many pencils does each class get, | `(asked (each-of (amount School pencils t1) classes))` |
| and how many are left over? | `(asked (rest-of (amount School pencils t1) classes))` |

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n (n z 2) 1) · ?q = (rest-of (amount School pencils t1) classes)
                        “(rest-of (amount School pencils t1) classes): (n (n z 2) 1)”
                      ?n = (n (n z 4) 7) · ?q = (each-of (amount School pencils t1) classes)
                        “(each-of (amount School pencils t1) classes): (n (n z 4) 7)”
  as :expect        (model (asked (each-of (amount School pencils t1) classes))
                           (asked (rest-of (amount School pencils t1) classes))
                           (value (each-of (amount School pencils t1) classes) (n (n z 4) 7))
                           (value (rest-of (amount School pencils t1) classes) (n (n z 2) 1)))
  :expect           holds

  query 2 of 2      (value ?q ?n)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n (n (n (n z 1) 4) 7) 8) · ?q = (amount School pencils t1)
                        “(amount School pencils t1): (n (n (n (n z 1) 4) 7) 8)”
                      ?n = (n (n (n (n z 1) 7) 2) 8) · ?q = (amount School pencils t0)
                        “(amount School pencils t0): (n (n (n (n z 1) 7) 2) 8)”
                      ?n = (n (n (n z 1) 4) 4) · ?q = (capacity cartons pencils)
                        “(capacity cartons pencils): (n (n (n z 1) 4) 4)”
                      ?n = (n (n z 1) 2) · ?q = (amount School cartons t0)
                        “(amount School cartons t0): (n (n z 1) 2)”
                      ?n = (n (n z 2) 1) · ?q = (rest-of (amount School pencils t1) classes)
                        “(rest-of (amount School pencils t1) classes): (n (n z 2) 1)”
                      ?n = (n (n z 3) 1) · ?q = (number-of classes)
                        “(number-of classes): (n (n z 3) 1)”
                      ?n = (n (n z 4) 7) · ?q = (each-of (amount School pencils t1) classes)
                        “(each-of (amount School pencils t1) classes): (n (n z 4) 7)”
  as :expect        (model (value (amount School cartons t0) (n (n z 1) 2))
                           (value (amount School pencils t0) (n (n (n (n z 1) 7) 2) 8))
                           (value (amount School pencils t1) (n (n (n (n z 1) 4) 7) 8))
                           (value (capacity cartons pencils) (n (n (n z 1) 4) 4))
                           (value (each-of (amount School pencils t1) classes) (n (n z 4) 7))
                           (value (number-of classes) (n (n z 3) 1))
                           (value (rest-of (amount School pencils t1) classes) (n (n z 2) 1)))
```

12 x 144 = 1728, 1728 − 250 = 1478, 1478 ÷ 31 = 47, remainder 21. The three requests, and nothing else asked of
the calculator:

```lisp
(div (n (n (n (n z 1) 4) 7) 8) (n (n z 3) 1) :rule shared-means-divide)
(mul (n (n z 1) 2) (n (n (n z 1) 4) 4) :rule of-means-times)
(sub (n (n (n (n z 1) 7) 2) 8) (n (n (n z 2) 5) 0) :rule gave-means-less)
```

**Root — 298 steps**

1. **`asked-is-wanted`** (step 2): the question asks [(each-of (amount School pencils t1) classes)]  
   → `wanted(each-of(amount(School, pencils, t1), classes))` — wanted: (each-of (amount School pencils t1) classes)
2. **`asked-is-wanted`** (step 3): the question asks [(rest-of (amount School pencils t1) classes)]  
   → `wanted(rest-of(amount(School, pencils, t1), classes))` — wanted: (rest-of (amount School pencils t1) classes)
3. **`share-plan`** (step 5): shared equally means a division: first how many there are to share, and among how many classes  
   → `wanted(amount(School, pencils, t1))` — wanted: (amount School pencils t1)
4. **`rest-plan`** (step 6): what is left over is known once the sharing is done  
   → `wanted(each-of(amount(School, pencils, t1), classes))` — wanted: (each-of (amount School pencils t1) classes)
5. **`gave-plan-giver`** (step 7): School gave pencils away between t0 and t1: what School has at t1 comes from what School had at t0  
   → `wanted(amount(School, pencils, t0))` — wanted: (amount School pencils t0)
6. **`there-are-states`** (step 8): the problem states it: there are (n (n z 3) 1) classes  
   → `value(number-of(classes), n(n(z, 3), 1))` — (number-of classes): (n (n z 3) 1)
7. **`packed-plan`** (step 9): the pencils of School are in cartons: first how many cartons, and how many pencils one holds  
   → `wanted(amount(School, cartons, t0))` — wanted: (amount School cartons t0)
8. **`has-states`** (step 10): the problem states it: School has (n (n z 1) 2) cartons at t0  
   → `value(amount(School, cartons, t0), n(n(z, 1), 2))` — (amount School cartons t0): (n (n z 1) 2)
9. **`holds-states`** (step 11): the problem states it: one of the cartons holds (n (n (n z 1) 4) 4) pencils  
   → `value(capacity(cartons, pencils), n(n(n(z, 1), 4), 4))` — (capacity cartons pencils): (n (n (n z 1) 4) 4)
10. **`of-means-times`** (step 12): (n (n z 1) 2) cartons of (n (n (n z 1) 4) 4) pencils each means a product: multiply (n (n z 1) 2) by (n (n (n z 1) 4) 4)  
   → `mul(n(n(z, 1), 2), n(n(n(z, 1), 4), 4))` — asked: (n (n z 1) 2) x (n (n (n z 1) 4) 4)
11. **`packed-is-the-product`** (step 75): (n (n z 1) 2) x (n (n (n z 1) 4) 4) = (n (n (n (n z 1) 7) 2) 8): School has (n (n (n (n z 1) 7) 2) 8) pencils at t0  
   → `value(amount(School, pencils, t0), n(n(n(n(z, 1), 7), 2), 8))` — (amount School pencils t0): (n (n (n (n z 1) 7) 2) 8)
12. **`gave-means-less`** (step 76): gave away means less: School had (n (n (n (n z 1) 7) 2) 8) and gave (n (n (n z 2) 5) 0), so take (n (n (n z 2) 5) 0) from (n (n (n (n z 1) 7) 2) 8)  
   → `sub(n(n(n(n(z, 1), 7), 2), 8), n(n(n(z, 2), 5), 0))` — asked: (n (n (n (n z 1) 7) 2) 8) - (n (n (n z 2) 5) 0)
13. **`giver-has-the-difference`** (step 99): (n (n (n (n z 1) 7) 2) 8) - (n (n (n z 2) 5) 0) = (n (n (n (n z 1) 4) 7) 8): School has (n (n (n (n z 1) 4) 7) 8) pencils at t1  
   → `value(amount(School, pencils, t1), n(n(n(n(z, 1), 4), 7), 8))` — (amount School pencils t1): (n (n (n (n z 1) 4) 7) 8)
14. **`shared-means-divide`** (step 100): shared equally means a division: (n (n (n (n z 1) 4) 7) 8) among (n (n z 3) 1) classes  
   → `div(n(n(n(n(z, 1), 4), 7), 8), n(n(z, 3), 1))` — asked: (n (n (n (n z 1) 4) 7) 8) divided by (n (n z 3) 1)
15. **`each-gets-the-quotient`** (step 298): (n (n (n (n z 1) 4) 7) 8) divided by (n (n z 3) 1) = (n (n z 4) 7), remainder (n (n z 2) 1): each of the classes gets (n (n z 4) 7), and (n (n z 2) 1) are left  
   → `value(each-of(amount(School, pencils, t1), classes), n(n(z, 4), 7))` — (each-of (amount School pencils t1) classes): (n (n z 4) 7)

*15 steps of the word theory, 283 of the calculator and the entry rules, 298 in all.*

The first sentence is two facts and a third that joins them, `packed-in`: *12 boxes of 144* is a product only
because the pencils are **in** the boxes, and that word is in the sentence's *of*.

### 6.4 Who has more, and by how much — `p/more.ein`

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(has Bob apples (n z 5) now)` |
| Alice has 12 apples. | `(has Alice apples (n (n z 1) 2) now)` |
| Who has more apples, and by how many? | `(asked (apart Bob Alice apples now))` |

```text
  query             (and (asked (apart ?p ?q ?k ?t)) (has-more ?w ?l ?k ?t) (value (apart ?p ?q ?k ?t) ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?k = apples · ?l = Bob · ?n = (n z 7) · ?p = Bob · ?q = Alice · ?t = now · ?w = Alice
                        “Alice has (n z 7) more apples than Bob”
```

**Root — 34 steps**

1. **`asked-is-wanted`** (step 2): the question asks [(apart Bob Alice apples now)]  
   → `wanted(apart(Bob, Alice, apples, now))` — wanted: (apart Bob Alice apples now)
2. **`apart-plan`** (step 4): who has more means a comparison: first what Bob has and what Alice has  
   → `wanted(amount(Bob, apples, now))` — wanted: (amount Bob apples now)
3. **`has-states`** (step 5): the problem states it: Bob has (n z 5) apples at now  
   → `value(amount(Bob, apples, now), n(z, 5))` — (amount Bob apples now): (n z 5)
4. **`has-states`** (step 6): the problem states it: Alice has (n (n z 1) 2) apples at now  
   → `value(amount(Alice, apples, now), n(n(z, 1), 2))` — (amount Alice apples now): (n (n z 1) 2)
5. **`more-means-compare`** (step 7): who has more: compare (n z 5) with (n (n z 1) 2)  
   → `cmp(n(z, 5), n(n(z, 1), 2))` — asked: (n z 5) compared with (n (n z 1) 2)
6. **`second-has-more`** (step 17): (n z 5) is less than (n (n z 1) 2): Alice has more; by how much means take (n z 5) from (n (n z 1) 2)  
   → `has-more(Alice, Bob, apples, now)` — Alice has more apples than Bob at now
7. **`more-by-second`** (step 34): (n (n z 1) 2) - (n z 5) = (n z 7): Alice has (n z 7) more apples than Bob  
   → `value(apart(Bob, Alice, apples, now), n(z, 7))` — (apart Bob Alice apples now): (n z 7)

*7 steps of the word theory, 27 of the calculator and the entry rules, 34 in all.*

The pupil's way: compare first, then take the smaller from the larger. One subtraction would do, since the
calculator's `diff-of` answers both directions; the prototype compares so that *who* and *by how much* are two
steps with two reasons.

### 6.5 A fact the problem does not need — `p/unused.ein`, `p/bystander.ein`

| the sentence | the fact |
|---|---|
| Bob has 5 apples. | `(has Bob apples (n z 5) t0)` |
| Alice has 7 apples. | `(has Alice apples (n z 7) t0)` |
| Carol has 9 apples | `(has Carol apples (n z 9) t0)` |
| and a box that holds 3. | `(holds boxes apples (n z 3))` |
| Bob gave 2 apples to Alice. | `(gave Bob Alice apples (n z 2) t0 t1)` |
| How many apples does Bob have? | `(asked (amount Bob apples t1))` |

```text
  query 1 of 2      (and (asked ?q) (value ?q ?n))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 3) · ?q = (amount Bob apples t1)
                        “(amount Bob apples t1): (n z 3)”
  as :expect        (model (asked (amount Bob apples t1))
                           (value (amount Bob apples t1) (n z 3)))
  :expect           holds

  query 2 of 2      (value ?q ?n)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 3) · ?q = (amount Bob apples t1)
                        “(amount Bob apples t1): (n z 3)”
                      ?n = (n z 5) · ?q = (amount Bob apples t0)
                        “(amount Bob apples t0): (n z 5)”
  as :expect        (model (value (amount Bob apples t0) (n z 5))
                           (value (amount Bob apples t1) (n z 3)))
```

Alice's 7, Carol's 9 and the box are stated and never wanted: the second query lists every value the run knows,
and they are not among them. The trace has 20 steps, the same 20 as the
problem without the three sentences, and names neither Carol nor the box (`grep -c 'Carol\|boxes'
out/unused/trace_G.md`: 0). An unused fact is not an error.

Asked about someone the event did not touch (`p/bystander.ein`: *Bob has 5 apples. Carol has 9 apples. Bob gave 2
apples to Alice. How many apples does Carol have?*):

**Root — 6 steps**

1. **`asked-is-wanted`** (step 2): the question asks [(amount Carol apples t1)]  
   → `wanted(amount(Carol, apples, t1))` — wanted: (amount Carol apples t1)
2. **`bystander-plan`** (step 4): Carol neither gave nor got apples between t0 and t1: what Carol has is unchanged  
   → `wanted(amount(Carol, apples, t0))` — wanted: (amount Carol apples t0)
3. **`has-states`** (step 5): the problem states it: Carol has (n z 9) apples at t0  
   → `value(amount(Carol, apples, t0), n(z, 9))` — (amount Carol apples t0): (n z 9)
4. **`bystander-keeps`** (step 6): nothing changed for Carol between t0 and t1: still (n z 9) apples  
   → `value(amount(Carol, apples, t1), n(z, 9))` — (amount Carol apples t1): (n z 9)

*4 steps of the word theory, 2 of the calculator and the entry rules, 6 in all.*

## 7. Search-free or a search, and what each costs

`build/s1f.6.20/table.py`, 2026-11-11: the read-out's rows, `ein saturate`'s firing count at root, the trace's
steps (root's and every listed state's) with the word theory's among them, and the wall clock of `ein solve
FILE`, best of 10.

| problem | search-free | universe (H) | k_s | lattice | saturations | root firings | trace steps | of them the word theory's | `ein solve`, wall |
|---|---|---|---:|---|---|---:|---:|---:|---:|
| `bob1` | yes | no candidate | 1 | monotone | 1 saturation | 20 | 20 | 5 | 30 ms |
| `gap_a` | yes | no candidate | 0 | monotone | 1 saturation | 22 | 22 | 7 | 31 ms |
| `gap_b` | yes | no candidate | 1 | monotone | 1 saturation | 31 | 31 | 15 | 30 ms |
| `gap_c` | no | 12 candidates, 6 dead alone | 7 | monotone | 27 saturations | 22 | 139 | 25 | 34 ms |
| `gap_fact` | yes | no candidate | 1 | monotone | 1 saturation | 51 | 51 | 14 | 29 ms |
| `gap_assume` | yes | no candidate | 1 | monotone | 1 saturation | 57 | 57 | 19 | 31 ms |
| `gap_assume_h` | no | 1 candidate, 0 dead alone | 2 | monotone | 2 saturations | 22 | 48 | 11 | 30 ms |
| `gap_assume_hb` | no | 1 candidate, 0 dead alone | 2 | flagged | 2 saturations | 31 | 57 | 19 | 32 ms |
| `contra` | yes | no candidate | 0 | monotone | 1 saturation | 21 | 21 | 6 | 27 ms |
| `kinds` | yes | no candidate | 1 | monotone | 1 saturation | 10 | 10 | 7 | 31 ms |
| `kinds_assume` | yes | no candidate | 1 | monotone | 1 saturation | 23 | 23 | 9 | 31 ms |
| `kinds_full` | yes | no candidate | 1 | monotone | 1 saturation | 19 | 19 | 6 | 29 ms |
| `pencils` | yes | no candidate | 1 | monotone | 1 saturation | 298 | 298 | 15 | 31 ms |
| `more` | yes | no candidate | 1 | monotone | 1 saturation | 34 | 34 | 7 | 29 ms |
| `boxes12` | yes | no candidate | 1 | monotone | 1 saturation | 139 | 139 | 10 | 30 ms |
| `boxes12_readings` | yes | no candidate | 1 | monotone | 1 saturation | 140 | 140 | 10 | 31 ms |
| `boxes12_always` | no | 2 candidates, 0 dead alone | 2 | monotone | 4 saturations | 140 | 140 | 10 | 32 ms |
| `boxes13` | no | 2 candidates, 0 dead alone | 2 | flagged | 4 saturations | 141 | 155 | 13 | 33 ms |
| `boxes13_always` | no | 2 candidates, 0 dead alone | 2 | monotone | 4 saturations | 141 | 155 | 13 | 32 ms |
| `boxes13_plain` | no | 2 candidates, 0 dead alone | 3 | monotone | 4 saturations | 142 | 156 | 14 | 33 ms |
| `boxes13_stated` | no | 1 candidate, 1 dead alone | 1 | flagged | 2 saturations | 154 | 154 | 12 | 32 ms |
| `boxes13_both` | yes | no candidate | 1 | monotone | 1 saturation | 155 | 155 | 13 | 31 ms |
| `boxes13_gap` | yes | no candidate | 1 | monotone | 1 saturation | 142 | 142 | 11 | 32 ms |
| `unused` | yes | no candidate | 1 | monotone | 1 saturation | 20 | 20 | 5 | 30 ms |
| `bystander` | yes | no candidate | 1 | monotone | 1 saturation | 6 | 6 | 4 | 30 ms |
| `form_events` | yes | no candidate | 1 | monotone | 1 saturation | 16 | 16 | 2 | 25 ms |
| `form_chain` | yes | no candidate | 1 | monotone | 1 saturation | 18 | 18 | 4 | 25 ms |
| `gap_assume_ha` | no | 1 candidate, 0 dead alone | 1 | monotone | 2 saturations | 22 | 48 | 11 | 29 ms |
| `boxes12_plain` | yes | no candidate | 1 | monotone | 1 saturation | 140 | 140 | 10 | 32 ms |

- **Every solved problem of the stage's table is search-free**: H empty, k_s 1, R {∅}. So are the contradiction
  (k_s 0, root dead), the gap by `absent` and the gap by obligation (k_s 0, root owes).
- **A search is where a supposition is**: a reading of *need* (2 candidates, 4 saturations), a supposed quantity
  (1 candidate, 2 saturations), or a list of guesses (12 candidates, 27 saturations).
- **The cost is the load.** A run is 25 ms to 34 ms of wall clock and the saturation is under a millisecond
  (`ein saturate`: 0.45 ms for 20 firings, 0.94 ms for the pencils' 298);
  the rest is parsing and loading the tables, [E-15](findings.md).
- **The word theory is 2 to 25 steps of a run**; the calculator is the rest. The pencils are
  298 steps, 15 of them the word theory's.
- The rules of `words` join on a quantity, a nested fact, with a slot left to bind, `(value (amount ?p ?k ?t)
  ?a)`: the scan of [E-3](findings.md). A problem has a handful of `value` facts, so nothing here measures it; a
  batch of problems in one program would.

## 8. Where the theory would live, what it is called, and which readings it holds

**Where.** Not in `stdlib/`. Recommendation: **`examples/word_problems/`**, the modules beside their problems,
imported by relative name as the prototype does (`(import words :symbols (words))`). Three reasons. It is an
application of the calculator and the stage says so; a stdlib rule needs a program under `tests/stdlib/` and
enters the mutation census, which is the wrong price for a prototype vocabulary; and imported flat it occupies
ordinary words, `has`, `gave`, `holds`, `asked`, `wanted`, `needs`, `value`, `amount`, `together`, which a
library should not take from every program. If it proves itself over P1f.8's algebra problems it can move.

**What it is called.** The prototype's names, proposed as they are: `words` (what the words mean), `gaps` (a
missing quantity, by `absent`, kept apart), `readings` (a word with two readings, as hypotheses). One entry rule
each, as `std.arith` has `arith`.

**Which readings of a word it holds**, and which it does not:

| word | the reading held | not held, and what happens |
|---|---|---|
| *has N things* | a stated amount of one kind at one moment | *and nothing else*: someone with pears stated has no stated apples, and the program says so (§ 6.2). It needs `absent` over `has`, or an assumption |
| *gave N to* | a stated number moves from one to the other between two moments; the giver must have them | *gave all*, *gave half*: no rule; the quantity after is a gap |
| *together* | the two people named | *everyone in the problem*: a sum over a set, which needs a chain of the people; no rule |
| *more, by how much* | a comparison of two, then the larger less the smaller | *how many times more*: no rule |
| *holds* | how many of a kind one container holds; any thing fits | sizes, part-filled by rule: outside the model (§ 5) |
| *N boxes of M* | a product, where the things are stated to be in the boxes | — |
| *shared equally among* | quotient and remainder, whole things only | fractions (S1f.6.8) |
| *need* | `whole-only` and `pack-all`, the least number | *any number that suffices*: stated in the `:why` as what need is not |

## 9. Refusals and failures that shaped the design

Each with its witness under `build/s1f.6.20/witness/` (output in `out/witness/`).

1. **A flat import does not bring what the imported module imports.** `(import words :symbols (words))`, with
   `words` importing `std.arith`, brought the calculator's relation declarations, which `words`' rules write, and
   none of its rules: nothing was computed, the calculator's relations were open, 660 candidates such as
   `(add 2 5)`, and a bare `ein solve` stopped with *a state's commitments pass 2^128*, exit 1, and no word about
   an import (`bob1_nested_import.ein`). **Fix in the design:** the entry rule of `words` writes the name of the
   calculator's entry, `(words-uses arith)`, and the closure follows it. [E-20](findings.md).
2. **`:expect` on a query closes the goal's relations over the class, and the class holds the goal's instances
   only.** A claim that listed a second `value` fact the goal did not ask for was `FAILED` with both sides
   printed; the claims on this page list exactly what the goal matches.
3. **An obligation with two `absent` guards over its relation is refused**: *rule 'quantity-owed': 2 `(absent …)`
   guards hold a positive `value` premise — which one states the obligation is not decidable*
   (`gap_a3_two_absents_refused.ein`). So an obligation cannot say both *has no value* and *has no value among the
   guesses*.
4. **An obligation cannot generate its own candidates here.** With no hrule, the obligations rung is the one that
   would propose `(reads-as need R)`; it declines, because the obligation's trigger reads a relation a commitment
   can change (Q-M1e2.10 (a)), and the blind enumerator proposes 420 pairs of names such as `(reads-as 0 1)`: a
   cube of 2^420, a partial answer, no state (`boxes13_rung_declines.ein`). The same with a stated list of numbers
   for a missing quantity: 342 candidates, stopped at the budget (`gap_a3.ein`). **So the readings are proposed by
   an hrule**, and the problem file names it in `(solve :hrules …)`, which a module cannot do for it.
5. **A template reference names a variable and nothing else.** `{(n ?h ?l)}` in a `:why` loads and prints as
   written. The two rules that wanted *the remainder, which is not zero* match `?r` and guard it with `(neq ?r
   z)`, which works on a nested fact.
6. **Names.** A kind is written in the plural, `apples`, `boxes`, since a `:why` cannot inflect: *how many boxes
   are needed*, *one of the boxes holds 3 apples*.
7. No refusal came from the value rule, from `:why` being required, or from a conclusion's variables: the
   quantities are nested facts of ordinary relations and every rule binds what it writes.

## 10. Findings

**Added to [`findings.md`](findings.md)**, each with its witness:

- **E-17** — `--trace` cannot show an importing theory's steps without the imported theory's: `-l` keeps
  137 of 139.
- **E-18** — a query cannot tell an R that is empty because root owes from one that is empty because root is dead.
- **E-19** — the read-out names what root owes only where R is empty.
- **E-20** — a flat import leaves behind the rules of what the imported module imports, in silence.

**Rows the problems meet**, already in `findings.md`: E-1 (`(value 0 __closed__)`, § 3.3), E-3 (the join on a
quantity, § 7), E-5 (the plan rules' `needs`, § 2.2), E-14 (every number on this page), E-15 (§ 7). And two that
are not rows there: the stale fact of § 3.5, which is F21's open question, and the 2^128 stop with exit 1, which
[`design.md`](design.md) records.

No wrong answer of the engine was found: no register row.

## 11. The questions for the user

Each with the recommendation the runs support.

1. **How a problem is written** (§ 1). *Recommended: form 1*, named moments, the event naming the moment before
   and after.
2. **The theory works backwards from the question** (§ 2.1): `wanted` and `needs` facts are the plan, and nothing
   is computed that no question needs. It is what makes the two-step problem's plan readable and an unused fact
   silent; it also means a contradiction in the facts is found only if a question reaches it (§ 6.1). *Recommended:
   keep it.* Not in the stage file's list; it is the prototype's main design decision.
3. **How a missing quantity is detected** (§ 3). *Recommended: (b)*, a derived fact by `absent` in a module of its
   own; (a) names the gap and answers nothing; (c) works only over a list the file states.
4. **How an assumption is written and reported** (§ 3.5). *Recommended: a marked fact, `assumed`*, with
   `rests-on` beside the answer, today. The hypothesis form is the better read-out and waits on question 5.
5. **A named gap together with a supposition.** (b) with a hypothesis leaves a stale fact (F21); (a) with a
   hypothesis is monotone and the read-out does not print the debt (E-19). Which to wait for: F21's ruling, or a
   read-out row for what root owes when R is not empty? *Recommended: the row*, which is smaller and is wanted by
   § 4 too.
6. **A missing reading** (§ 4). Readings as hypotheses is built in three forms. *Recommended: `readings_plain`*:
   ∅ stays in R as the partial answer, and the gap is a derived fact. Or `readings_always`, if R should hold only
   the readings. And whether `need-exact` should answer with no reading where nothing is left over (as built) or a
   reading is always chosen.
7. **Two kinds: is *Alice has 3 pears* also *Alice has no apples*?** (§ 6.2). The prototype says it is not
   stated. *Recommended: keep it a gap*, the same as *Alice had 0 before*, with the assumption available as a
   fact.
8. **The model's boundary** (§ 5). *Recommended: a fact with a `:source`, a premise of the rules that depend on
   it.*
9. **Where the theory lives and what it is called** (§ 8). *Recommended: `examples/word_problems/`, `words`,
   `gaps`, `readings`.*
10. **Which readings of a word the theory holds** (§ 8's table): which of the *not held* column T2 should add.
    *Recommended: none before the rulings above*; *together* over everyone and *gave all* are the two a
    school problem would meet first.
11. **The trace as the pupil's page** (§ 2.2): 10 of 139 steps are the
    reasoning. The acceptance asks that *each derived answer's trace is the reasoning*. Is a way to fold an
    imported theory's steps (E-17) part of this stage, a stage of its own, or is the filter enough? And E-14:
    the reasoning reads *(n z 5) + (n z 7) = (n (n z 1) 2)*.
12. **The evidence for S1f.6.14's question** (an expression layer). Every quantity here took two rules per word,
    one that asks the calculator and one that reads the answer, the second repeating the first's premises; the
    pencils are three operations chained through three pairs. No rule needed two operations at once.

## 12. The prototype in full

`build/s1f.6.20/p/words.ein`:

```lisp
;;; `words` — a prototype theory of elementary-school word problems
;;; (M1f P1f.6, S1f.6.20 T1). It imports the calculator, `std.arith`, and is
;;; imported by a problem. A problem states what its sentences say as facts
;;; (`has`, `gave`, `holds`, `packed-in`, `there-are`, `asked`); the rules
;;; below say what those words mean.
;;;
;;; The theory works backwards from the question and then forwards from the
;;; stated numbers:
;;;   - the PLAN: `(asked Q)` makes Q wanted; a wanted quantity makes the
;;;     quantities it is computed from wanted, each step a `(needs Q Q')`;
;;;   - the COMPUTATION: where the quantities a step needs have a value, the
;;;     rule for the word asks the calculator (`add`, `sub`, `mul`, `div`,
;;;     `cmp`) and reads its answer back as the value of the quantity.
;;; So nothing is computed that no question needs.
;;;
;;; A quantity is a nested fact: `(amount Bob apple t0)`, how many apples Bob
;;; has at the moment t0. A number is the calculator's, `(n z 5)`.

(import std.arith :symbols (arith))

;; ── What the model leaves out, stated once ──────────────────────────────────
(relation model-counts-only
  :why "in this model things are counted by number alone: a thing has no size, weight, shape or price, any thing of a kind fits any container that holds that kind, and a person can give away only what they have")
(model-counts-only
  :source "the model of the theory `words`: things are counted by number alone (no size, weight, shape or price), any thing of a kind fits any container that holds that kind, and nobody gives away what they do not have")

;; ── The problem's sentences ─────────────────────────────────────────────────
(relation has Who Kind Number Moment
  :why "{?1} has {?3} {?2} at {?4}")
(relation gave Who Who Kind Number Moment Moment
  :why "{?1} gave {?4} {?3} to {?2}, between {?5} and {?6}")
(relation holds Container Kind Number
  :why "one of the {?1} holds {?3} {?2}")
(relation packed-in Who Kind Container Moment
  :why "the {?2} of {?1} are all in {?3} at {?4}, each one full")
(relation there-are Kind Number
  :why "there are {?2} {?1}")
(relation asked Quantity
  :why "the question: {?1}")
(relation assumed Quantity Number
  :why "an assumption, not a sentence of the problem: [{?1}] is {?2}")

;; ── Quantities ──────────────────────────────────────────────────────────────
(relation amount Who Kind Moment
  :why "how many {?2} {?1} has at {?3}")
(relation together Who Who Kind Moment
  :why "how many {?3} {?1} and {?2} have together at {?4}")
(relation apart Who Who Kind Moment
  :why "by how many {?3} {?1} and {?2} differ at {?4}")
(relation capacity Container Kind
  :why "how many {?2} one of the {?1} holds")
(relation needed Container Kind Quantity
  :why "how many {?1} are needed for [{?3}]")
(relation over Quantity
  :why "how many are left unpacked, of [{?1}]")
(relation number-of Kind
  :why "how many {?1} there are")
(relation each-of Quantity Kind
  :why "how many each of the {?2} gets when [{?1}] is shared equally")
(relation rest-of Quantity Kind
  :why "how many are left when [{?1}] is shared equally among the {?2}")

;; ── The plan and the values ─────────────────────────────────────────────────
(relation wanted Quantity
  :why "wanted: {?1}")
(relation needs Quantity Quantity
  :why "to know [{?1}] one must know [{?2}]")
(relation value Quantity Number
  :why "{?1}: {?2}")
(relation has-more Who Who Kind Moment
  :why "{?1} has more {?3} than {?2} at {?4}")
(relation same-amount Who Who Kind Moment
  :why "{?1} and {?2} have equally many {?3} at {?4}")
(relation left-over Quantity Number Number
  :why "[{?1}]: {?2} are filled, and {?3} are left over")
(relation gave-more-than-had Who Kind Number Number
  :why "{?1} gave {?4} {?2} and had only {?3}")

(rule asked-is-wanted ()
  :match  (asked ?q)
  :assert (wanted ?q)
  :why    "the question asks [{?q}]")

;; ── has ─────────────────────────────────────────────────────────────────────
(rule has-states ()
  :match  (and (wanted (amount ?p ?k ?t)) (has ?p ?k ?v ?t))
  :assert (value (amount ?p ?k ?t) ?v)
  :why    "the problem states it: {?p} has {?v} {?k} at {?t}")

(rule assumed-gives-value ()
  :match  (and (wanted ?q) (assumed ?q ?v))
  :assert (value ?q ?v)
  :why    "the problem does not state [{?q}]; it is ASSUMED to be {?v}")

;; ── gave ────────────────────────────────────────────────────────────────────
(rule gave-plan-giver ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?p ?r ?k ?x ?t ?u))
  :assert (and (wanted (amount ?p ?k ?t))
               (needs (amount ?p ?k ?u) (amount ?p ?k ?t)))
  :why    "{?p} gave {?k} away between {?t} and {?u}: what {?p} has at {?u} comes from what {?p} had at {?t}")

(rule gave-means-less ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?p ?r ?k ?x ?t ?u)
               (value (amount ?p ?k ?t) ?a))
  :assert (sub ?a ?x)
  :why    "gave away means less: {?p} had {?a} and gave {?x}, so take {?x} from {?a}")

(rule giver-has-the-difference ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?p ?r ?k ?x ?t ?u)
               (value (amount ?p ?k ?t) ?a) (diff-of ?a ?x ?d z))
  :assert (value (amount ?p ?k ?u) ?d)
  :why    "{?a} - {?x} = {?d}: {?p} has {?d} {?k} at {?u}")

(rule giver-gave-too-many ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?p ?r ?k ?x ?t ?u)
               (value (amount ?p ?k ?t) ?a) (diff-of ?a ?x z ?w) (neq ?w z))
  :assert (gave-more-than-had ?p ?k ?a ?x)
  :why    "{?p} had {?a} {?k} and gave {?x}: {?a} is short of {?x} by {?w}")

(rule gave-more-than-had-is-false ()
  :match  (and (gave-more-than-had ?p ?k ?a ?x) (model-counts-only))
  :assert (false)
  :why    "nobody gives away what they do not have: {?p} cannot give {?x} {?k} out of {?a}")

(rule gave-plan-receiver ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?g ?p ?k ?x ?t ?u))
  :assert (and (wanted (amount ?p ?k ?t))
               (needs (amount ?p ?k ?u) (amount ?p ?k ?t)))
  :why    "{?p} was given {?k} between {?t} and {?u}: what {?p} has at {?u} comes from what {?p} had at {?t}")

(rule was-given-means-more ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?g ?p ?k ?x ?t ?u)
               (value (amount ?p ?k ?t) ?a))
  :assert (add ?a ?x)
  :why    "was given means more: {?p} had {?a} and got {?x}, so add {?x} to {?a}")

(rule receiver-has-the-sum ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?g ?p ?k ?x ?t ?u)
               (value (amount ?p ?k ?t) ?a) (sum-of ?a ?x ?s))
  :assert (value (amount ?p ?k ?u) ?s)
  :why    "{?a} + {?x} = {?s}: {?p} has {?s} {?k} at {?u}")

(rule bystander-plan ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?g ?r ?k ?x ?t ?u)
               (neq ?p ?g) (neq ?p ?r))
  :assert (and (wanted (amount ?p ?k ?t))
               (needs (amount ?p ?k ?u) (amount ?p ?k ?t)))
  :why    "{?p} neither gave nor got {?k} between {?t} and {?u}: what {?p} has is unchanged")

(rule bystander-keeps ()
  :match  (and (wanted (amount ?p ?k ?u)) (gave ?g ?r ?k ?x ?t ?u)
               (neq ?p ?g) (neq ?p ?r) (value (amount ?p ?k ?t) ?a))
  :assert (value (amount ?p ?k ?u) ?a)
  :why    "nothing changed for {?p} between {?t} and {?u}: still {?a} {?k}")

;; ── together ────────────────────────────────────────────────────────────────
(rule together-plan ()
  :match  (wanted (together ?p ?q ?k ?t))
  :assert (and (wanted (amount ?p ?k ?t)) (wanted (amount ?q ?k ?t))
               (needs (together ?p ?q ?k ?t) (amount ?p ?k ?t))
               (needs (together ?p ?q ?k ?t) (amount ?q ?k ?t)))
  :why    "together means a sum: first what {?p} has and what {?q} has, of {?k} alone, at {?t}")

(rule together-means-sum ()
  :match  (and (wanted (together ?p ?q ?k ?t))
               (value (amount ?p ?k ?t) ?a) (value (amount ?q ?k ?t) ?b))
  :assert (add ?a ?b)
  :why    "together means a sum: {?p} has {?a} and {?q} has {?b}, so add them")

(rule together-is-the-sum ()
  :match  (and (wanted (together ?p ?q ?k ?t))
               (value (amount ?p ?k ?t) ?a) (value (amount ?q ?k ?t) ?b)
               (sum-of ?a ?b ?s))
  :assert (value (together ?p ?q ?k ?t) ?s)
  :why    "{?a} + {?b} = {?s}: together {?p} and {?q} have {?s} {?k}")

;; ── more than, and by how much ──────────────────────────────────────────────
(rule apart-plan ()
  :match  (wanted (apart ?p ?q ?k ?t))
  :assert (and (wanted (amount ?p ?k ?t)) (wanted (amount ?q ?k ?t))
               (needs (apart ?p ?q ?k ?t) (amount ?p ?k ?t))
               (needs (apart ?p ?q ?k ?t) (amount ?q ?k ?t)))
  :why    "who has more means a comparison: first what {?p} has and what {?q} has")

(rule more-means-compare ()
  :match  (and (wanted (apart ?p ?q ?k ?t))
               (value (amount ?p ?k ?t) ?a) (value (amount ?q ?k ?t) ?b))
  :assert (cmp ?a ?b)
  :why    "who has more: compare {?a} with {?b}")

(rule first-has-more ()
  :match  (and (wanted (apart ?p ?q ?k ?t))
               (value (amount ?p ?k ?t) ?a) (value (amount ?q ?k ?t) ?b)
               (cmp-of ?a ?b gt))
  :assert (and (has-more ?p ?q ?k ?t) (sub ?a ?b))
  :why    "{?a} is more than {?b}: {?p} has more; by how much means take {?b} from {?a}")

(rule second-has-more ()
  :match  (and (wanted (apart ?p ?q ?k ?t))
               (value (amount ?p ?k ?t) ?a) (value (amount ?q ?k ?t) ?b)
               (cmp-of ?a ?b lt))
  :assert (and (has-more ?q ?p ?k ?t) (sub ?b ?a))
  :why    "{?a} is less than {?b}: {?q} has more; by how much means take {?a} from {?b}")

(rule neither-has-more ()
  :match  (and (wanted (apart ?p ?q ?k ?t))
               (value (amount ?p ?k ?t) ?a) (value (amount ?q ?k ?t) ?b)
               (cmp-of ?a ?b eq))
  :assert (and (same-amount ?p ?q ?k ?t) (value (apart ?p ?q ?k ?t) z))
  :why    "{?a} and {?b} are equal: neither has more")

(rule more-by-first ()
  :match  (and (has-more ?p ?q ?k ?t)
               (value (amount ?p ?k ?t) ?a) (value (amount ?q ?k ?t) ?b)
               (diff-of ?a ?b ?d z) (wanted (apart ?p ?q ?k ?t)))
  :assert (value (apart ?p ?q ?k ?t) ?d)
  :why    "{?a} - {?b} = {?d}: {?p} has {?d} more {?k} than {?q}")

(rule more-by-second ()
  :match  (and (has-more ?q ?p ?k ?t)
               (value (amount ?p ?k ?t) ?a) (value (amount ?q ?k ?t) ?b)
               (diff-of ?b ?a ?d z) (wanted (apart ?p ?q ?k ?t)))
  :assert (value (apart ?p ?q ?k ?t) ?d)
  :why    "{?b} - {?a} = {?d}: {?q} has {?d} more {?k} than {?p}")

;; ── holds, and N containers of M ────────────────────────────────────────────
(rule holds-states ()
  :match  (and (wanted (capacity ?c ?k)) (holds ?c ?k ?m))
  :assert (value (capacity ?c ?k) ?m)
  :why    "the problem states it: one of the {?c} holds {?m} {?k}")

(rule packed-plan ()
  :match  (and (wanted (amount ?p ?k ?t)) (packed-in ?p ?k ?c ?t))
  :assert (and (wanted (amount ?p ?c ?t)) (wanted (capacity ?c ?k))
               (needs (amount ?p ?k ?t) (amount ?p ?c ?t))
               (needs (amount ?p ?k ?t) (capacity ?c ?k)))
  :why    "the {?k} of {?p} are in {?c}: first how many {?c}, and how many {?k} one holds")

(rule of-means-times ()
  :match  (and (wanted (amount ?p ?k ?t)) (packed-in ?p ?k ?c ?t)
               (value (amount ?p ?c ?t) ?a) (value (capacity ?c ?k) ?m))
  :assert (mul ?a ?m)
  :why    "{?a} {?c} of {?m} {?k} each means a product: multiply {?a} by {?m}")

(rule packed-is-the-product ()
  :match  (and (wanted (amount ?p ?k ?t)) (packed-in ?p ?k ?c ?t)
               (value (amount ?p ?c ?t) ?a) (value (capacity ?c ?k) ?m)
               (product-of ?a ?m ?w))
  :assert (value (amount ?p ?k ?t) ?w)
  :why    "{?a} x {?m} = {?w}: {?p} has {?w} {?k} at {?t}")

;; ── there are, shared among ─────────────────────────────────────────────────
(rule there-are-states ()
  :match  (and (wanted (number-of ?g)) (there-are ?g ?m))
  :assert (value (number-of ?g) ?m)
  :why    "the problem states it: there are {?m} {?g}")

(rule rest-plan ()
  :match  (wanted (rest-of ?q ?g))
  :assert (and (wanted (each-of ?q ?g)) (needs (rest-of ?q ?g) (each-of ?q ?g)))
  :why    "what is left over is known once the sharing is done")

(rule share-plan ()
  :match  (wanted (each-of ?q ?g))
  :assert (and (wanted ?q) (wanted (number-of ?g))
               (needs (each-of ?q ?g) ?q) (needs (each-of ?q ?g) (number-of ?g)))
  :why    "shared equally means a division: first how many there are to share, and among how many {?g}")

(rule shared-means-divide ()
  :match  (and (wanted (each-of ?q ?g)) (value ?q ?a) (value (number-of ?g) ?m))
  :assert (div ?a ?m)
  :why    "shared equally means a division: {?a} among {?m} {?g}")

(rule each-gets-the-quotient ()
  :match  (and (wanted (each-of ?q ?g)) (value ?q ?a) (value (number-of ?g) ?m)
               (quot-of ?a ?m ?f ?r))
  :assert (and (value (each-of ?q ?g) ?f) (value (rest-of ?q ?g) ?r))
  :why    "{?a} divided by {?m} = {?f}, remainder {?r}: each of the {?g} gets {?f}, and {?r} are left")

;; ── need: how many containers ───────────────────────────────────────────────
(rule need-plan ()
  :match  (wanted (needed ?c ?k ?q))
  :assert (and (wanted ?q) (wanted (capacity ?c ?k))
               (needs (needed ?c ?k ?q) ?q) (needs (needed ?c ?k ?q) (capacity ?c ?k)))
  :why    "how many {?c} means a division: first how many {?k} there are to pack, and how many one of the {?c} holds")

(rule holds-means-divide ()
  :match  (and (wanted (needed ?c ?k ?q)) (value ?q ?a) (value (capacity ?c ?k) ?m))
  :assert (div ?a ?m)
  :why    "one of the {?c} holds {?m}, so the number of {?c} is a division: {?a} divided by {?m}")

(rule need-exact ()
  :match  (and (wanted (needed ?c ?k ?q)) (value ?q ?a) (value (capacity ?c ?k) ?m)
               (quot-of ?a ?m ?f z) (model-counts-only))
  :assert (value (needed ?c ?k ?q) ?f)
  :why    "{?a} divided by {?m} = {?f} and nothing is left over: {?f} {?c}, each full, whatever need means")

(rule need-left-over ()
  :match  (and (wanted (needed ?c ?k ?q)) (value ?q ?a) (value (capacity ?c ?k) ?m)
               (quot-of ?a ?m ?f ?r) (neq ?r z))
  :assert (left-over (needed ?c ?k ?q) ?f ?r)
  :why    "{?a} divided by {?m} = {?f}, remainder {?r}: {?f} {?c} are filled and {?r} {?k} are left over, so the answer depends on what need means")

;; ── The theory's one entry: closes its relations and names its rules ────────
(relation words-rule Rule
  :why "{?1} is a rule of words")
(relation words-uses Theory
  :why "words computes with the theory {?1}")
(words-rule words)

(rule words ()
  :match  ()
  :assert (and (__closed__ model-counts-only)
               (__closed__ has) (__closed__ gave) (__closed__ holds)
               (__closed__ assumed) (__closed__ packed-in) (__closed__ there-are) (__closed__ asked)
               (__closed__ amount) (__closed__ together) (__closed__ apart)
               (__closed__ capacity) (__closed__ needed) (__closed__ over) (__closed__ left-over) (__closed__ number-of)
               (__closed__ each-of) (__closed__ rest-of)
               (__closed__ wanted) (__closed__ needs) (__closed__ value)
               (__closed__ has-more) (__closed__ same-amount)
               (__closed__ gave-more-than-had) (__closed__ words-rule)
               (__closed__ words-uses) (words-uses arith)
               (words-rule asked-is-wanted) (words-rule assumed-gives-value) (words-rule has-states)
               (words-rule gave-plan-giver) (words-rule gave-means-less)
               (words-rule giver-has-the-difference) (words-rule giver-gave-too-many)
               (words-rule gave-more-than-had-is-false)
               (words-rule gave-plan-receiver) (words-rule was-given-means-more)
               (words-rule receiver-has-the-sum)
               (words-rule bystander-plan) (words-rule bystander-keeps)
               (words-rule together-plan) (words-rule together-means-sum)
               (words-rule together-is-the-sum)
               (words-rule apart-plan) (words-rule more-means-compare)
               (words-rule first-has-more) (words-rule second-has-more)
               (words-rule neither-has-more) (words-rule more-by-first)
               (words-rule more-by-second)
               (words-rule holds-states) (words-rule packed-plan)
               (words-rule of-means-times) (words-rule packed-is-the-product)
               (words-rule there-are-states) (words-rule rest-plan)
               (words-rule share-plan) (words-rule shared-means-divide)
               (words-rule each-gets-the-quotient)
               (words-rule need-plan) (words-rule holds-means-divide)
               (words-rule need-exact) (words-rule need-left-over))
  :why    "the word theory guesses nothing: every relation of it is closed")
```

`build/s1f.6.20/p/gaps.ein`:

```lisp
;;; `gaps` — mechanism (b): a quantity the plan wants and the problem does not
;;; state is a derived fact, by `absent`. Kept apart from `words`, which has
;;; no `absent`.
(import words :symbols (asked wanted needs value assumed))

(relation not-stated Quantity
  :why "the problem does not state [{?1}]")
(relation depends Quantity Quantity
  :why "[{?1}] rests on [{?2}]")
(relation unanswered Quantity Quantity
  :why "the question [{?1}] has no answer: the problem does not state [{?2}]")

(relation rests-on Quantity Quantity Number
  :why "the answer to [{?1}] rests on an assumption: [{?2}] is {?3}")

(rule answer-rests-on-assumption ()
  :match  (and (asked ?q) (depends ?q ?g) (assumed ?g ?v))
  :assert (rests-on ?q ?g ?v)
  :why    "[{?q}] is computed from [{?g}], which the problem does not state and which is assumed to be {?v}")

(rule gap-not-stated ()
  :match  (and (wanted ?q) (absent (value ?q ?v)) (absent (needs ?q ?r)))
  :assert (not-stated ?q)
  :why    "[{?q}] is wanted, no sentence states it, and nothing in the problem lets it be computed from something else")

(rule depends-step ()
  :match  (needs ?q ?r)
  :assert (depends ?q ?r)
  :why    "[{?q}] is computed from [{?r}]")

(rule depends-chain ()
  :match  (and (depends ?q ?r) (needs ?r ?s))
  :assert (depends ?q ?s)
  :why    "[{?q}] is computed from [{?r}], which is computed from [{?s}]")

(rule gap-blocks-question ()
  :match  (and (asked ?q) (depends ?q ?g) (not-stated ?g))
  :assert (unanswered ?q ?g)
  :why    "the question needs [{?g}], which the problem does not state")

(rule gap-is-the-question ()
  :match  (and (asked ?q) (not-stated ?q))
  :assert (unanswered ?q ?q)
  :why    "the question asks for [{?q}], which the problem does not state")

(relation gaps-rule Rule
  :why "{?1} is a rule of gaps")
(gaps-rule gaps)
(rule gaps ()
  :match  ()
  :assert (and (__closed__ not-stated) (__closed__ depends) (__closed__ unanswered) (__closed__ rests-on)
               (__closed__ gaps-rule)
               (gaps-rule gap-not-stated) (gaps-rule answer-rests-on-assumption) (gaps-rule depends-step) (gaps-rule depends-chain)
               (gaps-rule gap-blocks-question) (gaps-rule gap-is-the-question))
  :why    "the gap theory guesses nothing")
```

`build/s1f.6.20/p/readings.ein`:

```lisp
;;; `readings` — a word with two readings, as hypotheses. The theory holds the
;;; readings of *need* as facts; which one a problem means is a fact the
;;; search may commit to, `(reads-as need R)`, and each reading is a rule that
;;; answers under it. A state that has a remainder and no reading OWES one.
(import words :symbols (wanted value left-over needed over))

(relation reading-of Word Reading
  :why "{?2} is a reading of the word {?1} that the theory holds")
(relation reads-as Word Reading
  :why "the word {?1} is read as {?2}")
(relation whole-only
  :why "need, read as whole-only: count the containers that are filled, and leave the rest unpacked")
(relation pack-all
  :why "need, read as pack-all: everything is packed, and the last container may be part full")

(reading-of need whole-only
  :source "need = how many containers can be filled; what does not fill one is left out")
(reading-of need pack-all
  :source "need = the least number of containers that hold everything; the last may be part full")

(rule reading-owed ()
  :match  (and (left-over (needed ?c ?k ?q) ?f ?r)
               (absent (and (reading-of need ?m) (reads-as need ?m))))
  :assert (open reads-as)
  :why    "the problem does not say what need means, and it matters: {?f} {?c} are filled and {?r} {?k} are left over")

(hrule read-need ()
  :match  (and (left-over (needed ?c ?k ?q) ?f ?r) (reading-of need ?m))
  :assert (reads-as need ?m)
  :why    "suppose need is read as {?m}")

(rule one-reading ()
  :match  (and (reads-as ?w ?a) (reads-as ?w ?b) (neq ?a ?b))
  :assert (false)
  :why    "a word is read one way in one problem: {?w} cannot be both {?a} and {?b}")

(rule need-whole-only ()
  :match  (and (left-over (needed ?c ?k ?q) ?f ?r) (reads-as need whole-only))
  :assert (and (value (needed ?c ?k ?q) ?f) (value (over (needed ?c ?k ?q)) ?r))
  :why    "need read as whole-only: {?f} {?c} are filled, and {?r} {?k} stay unpacked")

(rule need-pack-all-plan ()
  :match  (and (left-over (needed ?c ?k ?q) ?f ?r) (reads-as need pack-all))
  :assert (add ?f (n z 1))
  :why    "need read as pack-all: {?f} {?c} do not hold everything, since {?r} {?k} are left over; one more does, and need asks for the least number, so add 1 to {?f}")

(rule need-pack-all ()
  :match  (and (left-over (needed ?c ?k ?q) ?f ?r) (reads-as need pack-all)
               (sum-of ?f (n z 1) ?s))
  :assert (and (value (needed ?c ?k ?q) ?s) (value (over (needed ?c ?k ?q)) z))
  :why    "{?f} + (n z 1) = {?s}: {?s} {?c} hold everything, the last one part full, and no fewer do")

(relation readings-rule Rule
  :why "{?1} is a rule of readings")
(readings-rule readings)
(rule readings ()
  :match  ()
  :assert (and (__closed__ reading-of) (__closed__ whole-only) (__closed__ pack-all)
               (__closed__ readings-rule)
               (readings-rule reading-owed) (readings-rule read-need) (readings-rule one-reading)
               (readings-rule need-whole-only) (readings-rule need-pack-all-plan)
               (readings-rule need-pack-all))
  :why    "the readings a word has are the theory's and are not guessed; which one a problem means, reads-as, is left open")
```

`readings_always.ein` is `readings.ein` with `(wanted (needed ?c ?k ?q))` in place of `(left-over (needed ?c ?k
?q) ?f ?r)` in `reading-owed` and in `read-need`. `readings_plain.ein` is `readings.ein` without `reading-owed`,
and with:

```lisp
(relation reading-matters Word Quantity Number Number
  :why "what {?1} means decides the answer to [{?2}]: {?3} are filled and {?4} are left over")
(rule need-matters ()
  :match  (left-over (needed ?c ?k ?q) ?f ?r)
  :assert (reading-matters need (needed ?c ?k ?q) ?f ?r)
  :why    "{?f} {?c} are filled and {?r} {?k} are left over: whether they need another one is what need means, and the problem does not say")
```

`readings_both.ein` and `readings_gap.ein` are § 4.4's, a few rules each, under `build/s1f.6.20/p/`.
