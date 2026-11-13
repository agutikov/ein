# S1h.2.3 — The second opening question: a restriction that was checked and held leaves no trace

**Phase:** [P1h.2](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. An investigation with a prototype **outside the engine**: it
reads engine source and changes none. Its question,
[Q-M1h.12](../open_questions.md#q-m1h12--is-a-restriction-that-was-tested-and-held-recorded), is put to the
user at [S1h.2.4](s1h.2.4_what_used_means_and_the_stop.md).
**Depends on:** [S1h.2.1](s1h.2.1_what_a_slice_is_today.md)'s script and set of programs.

## The question it rests on

[Round 2, item 33](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q33), to which
the user replied *"OK"*:

> One thing slicing by derivation does not show, and M1h's slicing phase should: a **restriction that was
> checked and held**. *Nobody gives away what they do not have* derives nothing when it holds; it would only
> have derived a contradiction. So it leaves no trace in the answer's derivation, yet the answer is valid
> only because it passed. *Untouched* then has two meanings, *not applicable to this problem* and
> *applicable, tested, satisfied*, and only the first is superfluous information.

The milestone README: *So a slice by derivation alone under-reports what an answer needs, and the phase has
to find what records that a restriction was "tested": its premises matched in part, or the rule's plan was
run against the state.*

## What it starts from

- **The witness.** `examples/word_problems/bob1.ein`: *Bob has 5 apples. He gave 2 to Alice.* The run has 22
  `fire` events and none is of `giver-gave-too-many` or of `gave-more-than-had-is-false`
  (`ein solve FILE --events E.jsonl`, 2026-11-13); `--events-level verbose` adds `hypskip` lines and nothing
  of a rule that did not fire. On `contra.ein`, *gave 7 of 5*, both fire and root is dead.
- **The word theory's restrictions are two rules each.** The user's ruling for a conflict is a flag and then
  `(false)`: `giver-gave-too-many` concludes the flag `(gave-more-than-had …)` from the calculator's answer,
  and `gave-more-than-had-is-false` concludes `(false)` from the flag and the model's assumption. So the
  *test* is in the first rule, which concludes an ordinary fact, and a definition of *restriction* that looks
  only for `(false)` in a conclusion misses it.
- **The stdlib's pairs.** A rule that refutes has two programs under `tests/stdlib/`, one where it fires and
  one where it is *loaded, activated and satisfied* ([`tests/README.md`](../../../tests/README.md)). The
  second of each pair is this question's witness outside the word problems, already written.
- **What a run counts.** `--events` has `enqueue` and `fire`
  ([`events.md`](../../../docs/kernel/inference/events.md)); the matcher's own counts of candidates exist
  only in a build with `--features counters`, which no shipped binary has.

## What it produces

A prototype under `build/s1h.2.3/` that classifies, for a saturated state and each restriction of a program,
*not applicable*, *applicable, tested, satisfied* or *fired*, computed outside the engine; the third section
of `slicing.md`; and Q-M1h.12 with its evidence and a price.

## Tasks

1. **T1h.2.3.1 — what a restriction is.** A definition the classification can run on, covering both shapes
   the corpus has: a rule that concludes `(false)`, and a rule that concludes a flag from which a rule
   concludes `(false)` (the chain followed back to the rule that tests). The page counts both shapes over
   `stdlib/`, `examples/word_problems/` and the set's other programs, as text, and names what the definition
   leaves out: an obligation that was owed and paid is *tested and satisfied* in another sense, and whether
   the phase's surfaces say so is a question the page raises and S1h.2.4 carries.
2. **T1h.2.3.2 — the README's two candidates, made precise and tried.**
   - ***Its premises matched in part.*** For a restriction rule and a final state, the longest part of its
     `:match` that some binding satisfies, and the premise that then fails. On `bob1.ein`,
     `giver-gave-too-many` matches *Bob's amount is wanted*, *Bob gave*, *Bob had 5*, and fails on the
     shortfall: the difference 5 − 2 has none. That failing premise is the reason the restriction held, and
     it is the line a reader wants.
   - ***The rule's plan was run against the state.*** Every loaded rule is run against the state, so by
     itself this says nothing; the task says what narrower event could carry it (a rule that was enqueued
     because a fact of one of its premises arrived) and what `enqueue` already tells.
   The prototype computes the first from the dumped state and the rule's text, with a matcher of its own
   over ground facts, and is held to the engine on one point: a rule it classifies *fired* has a `fire`
   event, and no other has.
3. **T1h.2.3.3 — the table.** Each restriction of the word theory against each banked problem: *not
   applicable* (no binding satisfies even its first premise that names the problem's facts), *applicable,
   tested, satisfied* (with the failing premise), or *fired*. The same for the stdlib pairs of the set, and
   for one zebra program, where the count of *tested and satisfied* instances is the thing to see: a
   `functional` or `bijective` declaration is tested against every pair.
4. **T1h.2.3.4 — what *matched in part* depends on.** The order of a rule's premises is the author's, and
   the engine plans a match in its own order: the page shows on one rule that the longest satisfied part
   differs with the order, and proposes the reading that does not depend on it (a largest satisfiable subset
   of the premises, or the premises that name given facts), with what each costs to compute.
5. **T1h.2.3.5 — the price of an engine record.** Read off `ein-infer/src/match_.rs` and the saturator,
   without changing either: where a partial match is known and dropped; what recording it would cost a run
   that did not ask (it must cost nothing: *minimum work per question*); whether it is an event kind under
   `--events-level verbose`, a pass taken after the run only where a slice is asked for, as a core is, or the
   prototype's own re-match kept as a tool. The count of records it would write on the zebra program of T3
   is on the page.
6. **T1h.2.3.6 — the page,** and Q-M1h.12 with the three options of T5 and what each prints on `bob1.ein`.

## Acceptance

- The definition of a restriction is on the page with its two shapes counted.
- The table of T3 has a row for every restriction of the word theory and a column for every banked problem,
  each cell computed by the prototype, and the failing premise named in every *tested, satisfied* cell.
- The prototype agrees with the engine's `fire` events on which restrictions fired, on every program of the
  set.
- The price of an engine record is on the page with the three options.
- No file under `ein.rs/`, `examples/`, `stdlib/` or `tests/` moved.

## What it must not do

- **Not change the matcher, the saturator or an event's schema.** The record, if any, is the user's to rule
  and [S1h.2.5](s1h.2.5_the_cut_as_an_object.md)'s to build.
- **Not call a restriction *required* because it was tested.** The page keeps the round's words: *applicable,
  tested, satisfied*. What the slice calls it is
  [Q-M1h.11](../open_questions.md#q-m1h11--what-used-means).
- **Not count a rule that never matched anything as tested.** That is the first meaning of *untouched*, and
  the only superfluous one.

## What it may find

- **That *tested* has no reading independent of the premises' order.** Then the page says so, and the
  proposal is the order-free one of T4 or none.
- **That the count of tested instances on a zebra program is too large to print.** Then what a surface shows
  is a count a rule, and the instances are a dump's.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Scratch stays under `build/s1h.2.3/`. The
link check is run before the commit, which changes pages only.
