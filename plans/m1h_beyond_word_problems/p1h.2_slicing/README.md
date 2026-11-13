# P1h.2 — Slicing: what of a program an answer used, shown

**Milestone:** [M1h](../README.md)
**Status:** planned 2026-11-12 as a section of the milestone's README, on the user's reply of that day
([§ Where it comes from](#where-it-comes-from)); **written out as this directory on 2026-11-13,** on the
user's instruction of that day (*"materialize M1h phases"*). Not started. It carries no estimate (the user's
ruling of 2026-11-08). **An engine phase where it adds a surface.** Its fourth stage **stops for the user**
with the cut computed outside the engine on real programs and every proposed surface shown, before the engine
or anything it prints changes; the two stages after the stop are named here and their tasks are written at
the stop.
**Depends on:** [M1f](../../m1f_hypothesis_and_documentation/README.md)'s calculator phase,
[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md), for the word problems banked
under [`examples/word_problems/`](../../../examples/word_problems/README.md), on which both opening questions
were first met. Nothing of M1g.
**Blocks:** [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md) reads the slice for how an answer
reports what it rests on, and [P1h.4](../p1h.4_the_formula_and_the_calculation/README.md) reads both; each
can start on [S1h.2.1](s1h.2.1_what_a_slice_is_today.md)'s script before any surface exists.

The milestone README's section, [§ P1h.2](../README.md#p1h2--slicing), is the phase's source and is kept as
written. This page carries the user's words in full, what exists today as re-taken on the binary, and the
stages.

## Where it comes from

Question 27 of the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q27) asked whether the
word theory works backwards from the question, so that nothing is computed that no question needs. The user,
2026-11-12:

> (a), another new phase in M1h, theory slicing or something like - question and inference of the answer
> cuts the program (which is a model of a problem) into 2 parts used/unused, required/superfluous. Ein must
> allow showing (print, dump) those parts.

So: **a question and the inference of its answer cut the program into two parts, and Ein shows both.** The
program is *a model of a problem*; the parts are *used / unused*, *required / superfluous*; and *showing* is
both *print* and *dump*.

**Its two opening questions** are from
[round 2, item 33](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q33). There the
user's reply 33 was answered:

> Is this a finite set of assumptions? What is it limited by? Used names/words? Probably we don't need to say
> that things are not alive and don't run out of the boxes, becasue the problem doesn't contain words "alive"
> and "run", or do we?
> But the same time we assume things fit boxes, while problem doesn't have this word.
> Btw, if we add rules and ontology that restricts throwing things away and after that the solution will not
> touch those rules and facts (see my answer 27) - that means this is not required information?

and both points of the answer were sent to this phase, to which the user replied *"OK"* (`dc88520b`):

1. **Assumptions as premises make the slice name them.** *If an assumption is a fact that the dependent
   rules take as a premise, then the slice of an answer (your phase from 27) contains exactly the assumptions
   that answer rests on, with no extra mechanism, and one the derivation never touches is not required for
   "that" answer.* So the user's last question above has the answer *yes, for that answer*, **provided**
   assumptions are written that way. [S1h.2.2](s1h.2.2_assumptions_as_premises.md) tests it.
2. **A restriction that was checked and held leaves no trace.** *"Nobody gives away what they do not have"
   derives nothing when it holds; it would only have derived a contradiction. So it leaves no trace in the
   answer's derivation, yet the answer is valid only because it passed. "Untouched" then has two meanings,
   "not applicable to this problem" and "applicable, tested, satisfied", and only the first is superfluous
   information.* [S1h.2.3](s1h.2.3_a_restriction_that_was_checked_and_held.md) finds what records that a
   restriction was *tested*.

## What it is

The cut **as an object, of the program**: its facts, its rules, its imported modules' rules, divided by one
answer into the part that answer used and the part it did not, printed and dumped. What exists today cuts
something else: a trace, or a contradiction.

The further questions the milestone README lists, each in the stage that takes it:

| the question | stage |
|---|---|
| whether a slice is of one answer or of a query's every class | [S1h.2.4](s1h.2.4_what_used_means_and_the_stop.md) |
| whether *required* means *used by this derivation* or *used by every derivation* (a fact with two justifications has two slices) | S1h.2.4, [Q-M1h.11](../open_questions.md#q-m1h11--what-used-means) |
| what the unused part of an imported theory is (most of the calculator, for any one sum) | S1h.2.1 (counted), S1h.2.4 (what is shown of it) |
| the surface: a read-out section, a `--dump-answer` file a slice, a view of `ein render program` with the two parts told apart | S1h.2.4, [Q-M1h.13](../open_questions.md#q-m1h13--the-slices-surfaces) |

## What it is not

- **Not a change to any answer.** R, N, H, every claim's outcome and every exit code are the same before and
  after, on every corpus program.
- **Not work on every run.** A slice is taken only where a surface prints it (*minimum work per question*,
  the user's ruling of 2026-10-16), as a core is.
- **Not a lever.** No option changes an answer; a slice is an explanation of one.
- **Not a canonical object unless it is defined as one.** A core is an explanation and may vary with rule
  order (the user's ruling of 2026-11-08), and a slice by one derivation inherits that; an intersection or a
  union over derivations would not. Q-M1h.11.
- **Not program slicing of the engine's Rust,** and not dead-code elimination at load: the loader's
  tree-shake (`ein_ir::resolve_and_minimize`) already drops what no live name reaches, before any question
  is asked. A slice is finer and is of one answer.

## What it starts from

As the tree stands on 2026-11-13, on the release binary found in `ein.rs/target/release/` that day. Each
figure is re-taken by [S1h.2.1](s1h.2.1_what_a_slice_is_today.md).

- **A justification a fact.** Every derived fact records the rule and the premises that gave it:
  `ein saturate FILE --dump` writes a fact's `:rule`; `--events`' `fire` line carries `rule`, `bindings`,
  `premises` and `derived`, and an `alt` line a further justification of a fact already held
  ([`events.md`](../../../docs/kernel/inference/events.md)).
- **`--trace -l`,** *prune each derivation to the goal-relevant slice*
  (`ein-render/src/trace/relevance.rs`): a walk back along provenance from the facts of the query's
  relations and the `:hrules` targets, which keeps the firings on a path to one. It cuts the **trace**, not
  the program. Steps written by `ein solve FILE -G --trace T.md`, and with `-l`, counted by
  `grep -c '^### Step'`:

  | program | steps | with `-l` |
  |---|---:|---:|
  | `examples/word_problems/boxes12.ein` | 145 | 137 |
  | `examples/word_problems/unused.ein` | 22 | 16 |
  | `examples/word_problems/bob1.ein` | 22 | 16 |
  | `examples/word_problems/bystander.ein` | 6 | 4 |
  | `examples/word_problems/contra.ein` | 23 | 3 |

  (The calculator's investigation counted 139 and 137 for the two-step problem on its prototype of
  2026-11-11, E-17 of its
  [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md).) On `contra.ein`,
  whose root is dead, the three steps kept are the plan and the stated amount: the walk starts from the
  query's relations, and the refutation is on no path to them.
- **A dead node's core,** a slice of another kind: the given facts a contradiction rests on, with the rule
  of its last step. `contra.ein`'s read-out prints six `unsat core` rows, four of them with a `source` row,
  and `refuted by gave-more-than-had-is-false`.
- **The model's one assumption as a premise.** [`words.ein`](../../../examples/word_problems/words.ein)
  states `(model-counts-only)` once and two rules take it as a premise, `need-exact` and
  `gave-more-than-had-is-false`. The `-l` trace of `boxes12.ein` names it; those of `bob1.ein`,
  `unused.ein` and `bystander.ein` do not.
- **A restriction that held leaves no event.** `ein solve examples/word_problems/bob1.ein --events E.jsonl`
  writes 22 `fire` lines, and none is of `giver-gave-too-many` or `gave-more-than-had-is-false`.
  `--events-level verbose` adds 2 120 `hypskip` lines and no other kind.
- **Two banked problems with a fact no question needs,** `unused.ein` and `bystander.ein`
  ([`word_problems.md` § 6.5](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#65-a-fact-the-problem-does-not-need--punusedein-pbystanderein)).
- **The pairs under `tests/stdlib/`:** a rule that refutes has two programs, one where it fires and one
  where it is *loaded, activated and satisfied* ([`tests/README.md`](../../../tests/README.md)). The second
  of each pair is the phase's second opening question, already written as a test.

## The stages

| ID | title | depends on | stops for the user |
|---|---|---|---|
| [S1h.2.1](s1h.2.1_what_a_slice_is_today.md) | What a slice is today: the trace's, the core's, and the cut of a program computed by a script over `--events` | the banked word problems | no; read-only on the engine |
| [S1h.2.2](s1h.2.2_assumptions_as_premises.md) | The first opening question: assumptions as premises make the slice name them | S1h.2.1 | no; its answer is shown at S1h.2.4 |
| [S1h.2.3](s1h.2.3_a_restriction_that_was_checked_and_held.md) | The second opening question: a restriction that was checked and held leaves no trace | S1h.2.1 | no; [Q-M1h.12](../open_questions.md#q-m1h12--is-a-restriction-that-was-tested-and-held-recorded) is put at S1h.2.4 |
| [S1h.2.4](s1h.2.4_what_used_means_and_the_stop.md) | What *used* means, the surfaces shown on real programs, and the stop | S1h.2.1 to S1h.2.3 | **yes: the phase's stop.** [Q-M1h.11](../open_questions.md#q-m1h11--what-used-means), [.12](../open_questions.md#q-m1h12--is-a-restriction-that-was-tested-and-held-recorded), [.13](../open_questions.md#q-m1h13--the-slices-surfaces). No engine source and nothing a surface prints changes before the user has read its page |
| [S1h.2.5](s1h.2.5_the_cut_as_an_object.md) | The cut as an object: an engine stage | the rulings of S1h.2.4 | no; **its tasks are written at S1h.2.4** |
| [S1h.2.6](s1h.2.6_the_cut_printed_and_dumped.md) | The cut printed and dumped, the page that says what *used* means, and the close: an engine stage | S1h.2.5 | no; **its tasks are written at S1h.2.4** |

No stage carries an estimate. Findings go to `findings.md` in this directory, opened at the first one
([M1h § Stage, phase, milestone](../README.md#stage-phase-milestone)).

## The order

S1h.2.1 first; S1h.2.2 and S1h.2.3 after it, in either order or side by side, since they share only its
script; S1h.2.4, and it stops; then S1h.2.5 and S1h.2.6 on the rulings.

Against other phases: P1h.3's [S1h.3.2](../p1h.3_gaps_assumptions_and_readings/s1h.3.2_assumptions_marked_supposed_and_universal.md)
and [S1h.3.3](../p1h.3_gaps_assumptions_and_readings/s1h.3.3_which_assumptions_a_model_has_to_state.md) read
S1h.2.2 and may run on S1h.2.1's script before this phase's stop. P1h.4's
[S1h.4.3](../p1h.4_the_formula_and_the_calculation/s1h.4.3_the_trace_as_the_pupils_page.md) asks whether the
pupil's page is this slice drawn another way and reads S1h.2.4's page if it exists.
[P1h.1](../p1h.1_temporal_and_causal_theories/README.md)'s
[S1h.1.4](../p1h.1_temporal_and_causal_theories/s1h.1.4_the_causal_theory_on_a_prototype.md) reads the script
for its third form of a causal tree. None waits for the engine stages.

## The discipline

The phase changes what surfaces print, so the repo's rules for that are restated here and each engine stage
cites this section:

- **Real computed examples first.** Every row, file and view proposed is shown on real programs, its content
  computed by S1h.2.1's script, before the printer moves (the user's rule of 2026-10-16).
- **A change to what `ein solve` prints changes
  [`read_out.md`](../../../docs/kernel/inference/read_out.md) in the same commit,** with
  `read_out_examples.md` re-written by `utils/read_out_examples.py`, and
  `ein-cli/tests/read_out_reference.rs` holds the page to the printer. A file added to `--dump-answer`'s
  directory changes [`answer_dump.md`](../../../docs/kernel/inference/answer_dump.md) in the same commit, and
  `ein-cli/tests/answer_round_trip.rs` holds what a dump reloads to.
- **No answer moves.** R, N, H, every claim's outcome and every exit code are the same before and after, on
  every corpus program, by a sweep of the old binary against the new; `ein oracle` is the reference.
- **A slice is taken only where a surface prints it.** No run that did not ask for one pays for one.
- **Config never changes an answer,** and a new option is a row of
  [`configuration.md`](../../../docs/kernel/configuration.md) with its *does it change the answer* column,
  pinned by `ein-cli/tests/config_reference.rs`.
- **Stdout listings share 100 lines;** `--dump-answer` writes a listing whole (the user's rule of
  2026-10-16).
- **Every golden a stage moves is counted with a command and named in its record before it is blessed.**
- **A wrong answer met on the way** is a row of
  [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), filed `open` in a commit of its own
  before its fix.
- **The gate, then the nightly,** on each code commit: `./run_tests.sh`, then `./run_tests.sh --nightly`;
  `utils/ci_container.sh` too on a commit that changes a DOT writer.

## Acceptance

- **S1h.2.4's page was read and ruled on by the user** before any engine source or printed line changed.
- **For a query's answer, Ein prints and dumps the part of the program it used and the part it did not,** on
  the surfaces the user ruled.
- **The page says what *used* means,** with the definition the user ruled and what the others would have
  given on the same programs.
- **The two opening questions are answered on programs:** the table of which assumptions each banked
  problem's answer rests on, and the table that tells *not applicable* from *applicable, tested, satisfied*
  for each restriction of the word theory on each banked problem.
- **No answer moved** (the sweep), and every moved golden was named first.
- The kernel pages that describe a changed surface changed in the same commit, and their tests hold.

## Risks, each with what checks it

- **A slice by derivation under-reports.** A restriction that held is in no derivation (opening question 2),
  and neither is a fact read by `absent`: an absence has no fact to point at. S1h.2.3 takes the first;
  S1h.2.1's fourth task puts the second on the page with [`gaps.ein`](../../../examples/word_problems/gaps.ein)
  as its witness, and S1h.2.4 says what the slice names in its place.
- **A slice is one explanation among several.** Two justifications, two slices. S1h.2.4 measures how far
  they differ before anything is called *required*.
- **The unused part is most of the program.** For one sum, most of the calculator. A listing of it would
  drown the read-out: S1h.2.4 shows what a count per module says against a listing, within the 100 lines.
- **Taking a slice costs.** It is a walk over every justification on a path to the answer. S1h.2.5 measures
  it and takes it only where a surface asks.
- **The script and the engine disagree.** S1h.2.1's script is kept as an independent check: S1h.2.5's test
  holds the engine's cut to the script's on the programs both reach.
- **A used part that does not answer alone.** If the used facts and rules, loaded by themselves, do not
  derive the answer, the cut is not what it claims. S1h.2.4 tries exactly that on every banked problem and
  shows where it fails and why.

## Connections

- **[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md):** replies 27 and 33, round 2's
  item 33, and E-17, whose 137 of 139 steps is what the trace's slice does on a calculation.
- **[P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md):** *how an answer reports what it rests on* is
  this slice; *which assumptions a model has to state* is counted through it.
- **[P1h.4](../p1h.4_the_formula_and_the_calculation/README.md):** one of the three forms of its pupil's
  page is this phase's slice *drawn another way*.
- **[P1h.20](../p1h.20_requirements_verification/README.md):** the slice as *which requirements a design
  decision rests on*, to be confirmed by that phase's survey and not assumed.
- **[F22](../../followups/f22_exact_core_past_the_cap.md):** cores made canonical; a slice defined by one
  derivation has the same question.
- **[F24](../../followups/f24_written_provenance.md):** a fact written with a `:rule` has no premises and
  leaves every core. It leaves every slice the same way, and S1h.2.1 shows it on F24's witness.
- **[Idea 03](../../ideas/03-three-task-classes.md):** a core is class C's explanation and a slice is class
  A's.
- **[`open_questions.md`](../open_questions.md):** Q-M1h.11 to .13.
