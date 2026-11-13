# P1h.4 — The formula and the calculation: the two parts of a solution, and the line between them

**Milestone:** [M1h](../README.md)
**Status:** planned 2026-11-12 as a section of the milestone's README, on the user's reply of that day
([§ Where it comes from](#where-it-comes-from)); **written out as this directory on 2026-11-13,** on the
user's instruction of that day (*"materialize M1h phases"*). Not started. It carries no estimate (the user's
ruling of 2026-11-08). **An engine phase where a view of the trace is one.** Its fifth stage **stops for the
user** with the formula derived on prototypes and every proposed view shown on real traces, before anything
ships; the two stages after the stop are named here and their tasks are written at the stop.
**Depends on:** [M1f](../../m1f_hypothesis_and_documentation/README.md)'s calculator phase: the word problems
banked under [`examples/word_problems/`](../../../examples/word_problems/README.md), and the three checks of
a formula's form that phase built
([S1f.6.23](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.23_the_loaders_arity_check.md),
[S1f.6.12](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.12_malformed_numbers.md)'s build,
[S1f.6.24](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.24_division_by_zero_at_the_call.md),
each done by 2026-11-13).
**Reads:** [P1h.2](../p1h.2_slicing/README.md)'s slice and
[P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md)'s page on readings, *both*, by the milestone's
order; and [M1g's P1g.10](../../m1g_logic/p1g.10_school_algebra/README.md), whose expressions are where a
formula can be a value ([Q-M1h.2](../open_questions.md#q-m1h2--whether-it-follows-p1g10)). None is waited
for ([§ The order](#the-order)).
**Blocks:** nothing.

The milestone README's section, [§ P1h.4](../README.md#p1h4--the-formula-and-the-calculation), is the phase's
source and is kept as written. This page carries the user's words in full, what the phase starts from as
re-taken on the binary, and the stages.

## Where it comes from

Question 36 of the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q36) asked whether
folding an imported theory's steps in a trace (*the trace as the pupil's page*) is part of the word-problem
stage. The user, 2026-11-12:

> How we can divide 2 parts? inference of a formula and calculations, the second is already solved (in
> arithmetic at least), but only if formula is well-formed, correct as a formula itself (all operations have
> required arguments, numbers and names correctly used, no division by zero, etc...). Yes, this is a phase in
> M1h

So a solution has **two parts**: *inference of a formula*, and *calculations*. The second *"is already solved
(in arithmetic at least)"*, on one condition, that the formula is *"well-formed, correct as a formula
itself"*: every operation has its arguments, numbers and names are used correctly, no division by zero. The
phase is the first part, and the line between the two.

And reply 31 of the same day, on a word with two readings, asks three things of a solution, the last of which
the user rules out of scope:

> reading always chosen, it can't be not chosen: result depends on formula, formula depends on selected
> arith operations, selection depend on reading. The synthesized program must be able answer question about
> the solution, like "why you divided those numbers? why not 3/12, not 3+12, not 12-3, and so on" btw there
> could be the case when answer is correct, but solution is not, right? And finally the solution could be
> correct, but reasoning not - this case is M2 and Ein theory of itself or theory of solutions - so out of
> the scope here.

## What it is

The phase's four questions, as the milestone README lists them, each in the stage that takes it:

| the question | stage |
|---|---|
| **The trace as the pupil's page:** a view of a run at one theory's level, the imported theory's steps folded to a line each. Whether that is a `--trace` option, a property of an import, or P1h.2's slice drawn another way | [S1h.4.3](s1h.4.3_the_trace_as_the_pupils_page.md) |
| **The formula inferred and shown:** the word theory deriving the formula as a fact before, or instead of, asking the calculator step by step; then *why divided and not subtracted* is a question about which rule built which node of the formula, answerable from its justification | [S1h.4.2](s1h.4.2_the_formula_as_an_object.md) |
| **The alternatives not taken.** *"why not 3/12, not 3+12, not 12-3"* asks about formulas the theory did **not** build. A derivation explains what was derived; an explanation of what was not is a different object. *What Ein can say there is the phase's hardest question and may end as a stated limit* | [S1h.4.4](s1h.4.4_the_alternatives_not_taken.md) |
| **Answer correct, solution not:** how a program or a reader tells them apart, given two states with one answer; what a *wrong* solution is when both readings are admissible | S1h.4.4 |

And before them, the line itself: which steps of a run are the formula's and which the calculation's, and the
condition *the formula is well-formed* stated as the conjunction of the checks that exist, with what is still
missing from it ([S1h.4.1](s1h.4.1_the_two_parts_and_a_well_formed_formula.md)).

## What it is not

- **Not a theory of solutions, and not of Ein itself.** *"the solution could be correct, but reasoning not -
  this case is M2 and Ein theory of itself or theory of solutions - so out of the scope here."* The phase
  goes as far as *formula against calculation* and stops there.
- **Not an expression layer of its own.** A formula as a value is P1g.10's expression data
  ([S1g.10.2](../../m1g_logic/p1g.10_school_algebra/s1g.10.2_expressions_and_evaluation.md)); the phase
  reads what that stage built before proposing anything.
- **Not a change to the calculator.** The second part *"is already solved"*; no rule of `std.arith` moves.
- **Not a number printed as a number.** A step reads *(n z 5) + (n z 7) = (n (n z 1) 2)* today (E-14 of the
  calculator's findings); whether a number may print as one is asked by
  [M1g's S1g.6.3](../../m1g_logic/p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md), and this phase
  shows its pages with the numbers as they print.
- **Not new syntax by default.** One of the three forms of the pupil's page, a property of an import, would
  be a grammar change; it is shown on real programs and is the user's
  ([Q-M1h.16](../open_questions.md#q-m1h16--the-pupils-page)).

## What it starts from

As the tree stands on 2026-11-13, on the release binary found in `ein.rs/target/release/` that day. Each
figure is re-taken by the stage that uses it.

- **E-17** of the calculator's
  [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md): `--trace` cannot show
  an importing theory's steps without the imported theory's. On the prototype of 2026-11-11 the two-step
  word problem was 139 steps, 10 of them the word theory's, and `-l` kept 137. On the banked
  `examples/word_problems/boxes12.ein` today it is **145 steps, 12 of them by rules declared under
  `examples/word_problems/`** (ten of the word theory's and the two closing rules), **and `-l` keeps 137,
  10 of them the word theory's** (`ein solve FILE -G [-l] --trace T.md`; steps counted by their rule's
  name against the rules the three modules declare). Nothing folds a call into one line, *12 ÷ 3 = 4,
  remainder 0 (116 steps of `std.arith`)*. The investigation's filter, `build/s1f.6.20/pupil.py`, 42 lines
  that read the trace file, is scratch and not tracked. P1g.10 met the same thing on an equation: 3 steps of
  the pupil's in 123 ([its F-2](../../m1g_logic/p1g.10_school_algebra/findings.md)).
- **The formula exists nowhere today.** It is the order in which rules ask the calculator. What a run does
  hold is the requests, each with the rule that asked
  ([`word_problems.md` § 2.2](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#22-the-two-step-problem-in-full),
  `ein saturate boxes12.ein --dump`): `(add (n z 5) (n z 7) :rule together-means-sum)` and
  `(div (n (n z 1) 2) (n z 3) :rule holds-means-divide)`, and the order between them is nowhere written:
  the second is derived from the first one's answer.
- **The checks of a formula** the reply lists are each something an earlier stage built or ruled:
  *all operations have required arguments*, the loader's arity check (S1f.6.23); *numbers and names correctly
  used*, the calculator's flags `(malformed REQUEST)` and `(undeclared NAME REQUEST)`
  ([`stdlib/README.md` § Malformed requests](../../../stdlib/README.md#malformed-requests-what-is-flagged-and-what-stays-silent-m1f-s1f612),
  S1f.6.12's build), and used and recursive names, which the user sent to P1g.10's S1g.10.2 with the
  expression layer (reply 10) and which is not built; *no division by zero*, the flag at the call
  (S1f.6.24).
- **Two states with one answer.**
  [S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md)
  banks the first corpus witness of *answer correct, solution not necessarily* (12 apples and boxes of 3:
  two readings, two states, one answer) and writes down what the trace says of *why divided* as far as the
  rules go (planned, not run on 2026-11-13).
  [S1h.3.4](../p1h.3_gaps_assumptions_and_readings/s1h.3.4_readings.md) reads those two states.
- **P1g.10's expressions,** as planned: `(num N)`, `(var x)`, `(Plus A B)`, `(Times A B)`, `(Over A B)`,
  with `(evaluate E)` answered by `(value-of E N)`, and the calculator's `(calc E)` and `(let NAME E)` moved
  there by the user's reply 10. Not built on 2026-11-13.

**Names.** [S1f.6.31](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md)
moves the calculator's modules under `std.math`; where this phase's files say `std.arith`, read
`std.math.decimal-arith` once that stage has run.

## The stages

| ID | title | depends on | stops for the user |
|---|---|---|---|
| [S1h.4.1](s1h.4.1_the_two_parts_and_a_well_formed_formula.md) | The two parts on the banked problems, and *well-formed* as a conjunction of the checks that exist | the banked word problems | no; read-only on the engine |
| [S1h.4.2](s1h.4.2_the_formula_as_an_object.md) | The formula as an object: the word theory deriving it as a fact, *before* and *instead of* | S1h.4.1; P1g.10's S1g.10.2 if it has run | no; [Q-M1h.17](../open_questions.md#q-m1h17--the-formula-as-an-object) is put at S1h.4.5 |
| [S1h.4.3](s1h.4.3_the_trace_as_the_pupils_page.md) | The trace as the pupil's page: three forms of a view, each on real traces | S1h.4.1; [S1h.2.1](../p1h.2_slicing/s1h.2.1_what_a_slice_is_today.md)'s script | no; [Q-M1h.16](../open_questions.md#q-m1h16--the-pupils-page) is put at S1h.4.5 |
| [S1h.4.4](s1h.4.4_the_alternatives_not_taken.md) | The alternatives not taken, and *answer correct, solution not*: what Ein can say, and the stated limit | S1h.4.2; [S1h.3.4](../p1h.3_gaps_assumptions_and_readings/s1h.3.4_readings.md) | no |
| [S1h.4.5](s1h.4.5_the_page_and_the_stop.md) | The page and the stop | S1h.4.1 to S1h.4.4 | **yes: the phase's stop.** Q-M1h.16, Q-M1h.17. No engine source, no printed line and nothing outside `plans/` and `build/` changes before the user has read its page |
| [S1h.4.6](s1h.4.6_the_pupils_page_built.md) | The pupil's page, built as ruled: an engine stage where it is one | the ruling on Q-M1h.16 | no; **its tasks are written at S1h.4.5** |
| [S1h.4.7](s1h.4.7_the_formula_banked_and_the_close.md) | The formula in the banked word problems, the page on what Ein can and cannot say about a solution, and the close | the ruling on Q-M1h.17; S1h.4.6 | no; **its tasks are written at S1h.4.5** |

No stage carries an estimate. Findings go to `findings.md` in this directory, opened at the first one
([M1h § Stage, phase, milestone](../README.md#stage-phase-milestone)).

## The order

S1h.4.1 first. S1h.4.2 and S1h.4.3 after it, in either order. S1h.4.4 after S1h.4.2, since an alternative is
a formula that was not built. S1h.4.5, and it stops. S1h.4.6 and S1h.4.7 on the rulings.

Against other phases and milestones, stated and not resolved:

- **P1h.2 and P1h.3,** which the milestone's order has this phase read. S1h.4.3 needs S1h.2.1's script, to
  draw the slice by module, and reads S1h.2.4's page if it exists; S1h.4.4 needs S1h.3.4's two states. If
  either has not run, the stage that needs it does the small part itself (a filter over the trace by the
  module of a rule; the two states re-run from S1f.6.28's files or the investigation's) and says so.
- **M1g's P1g.10.** If [S1g.10.2](../../m1g_logic/p1g.10_school_algebra/s1g.10.2_expressions_and_evaluation.md)
  has run, S1h.4.2 writes the formula in its expression data. If not, the prototype declares the heads it
  needs in a scratch file, in that stage's planned spelling, and the page says the data is provisional.
  Whether M1h follows P1g.10 at all is Q-M1h.2, the user's.
- **P1g.10's question f** (*Is the school solution a view of the trace?*) is Q-M1h.16 met on an equation. It
  is asked once: whichever of S1h.4.5 and
  [S1g.10.4](../../m1g_logic/p1g.10_school_algebra/s1g.10.4_linear_equations.md) comes first puts it, with
  both phases' witnesses, and the other follows the ruling.
- **S1f.6.30,** the import mechanism's investigation, which names the calculator's tagged calls as an
  interface. A *call* is what a fold would fold, so S1h.4.3 reads that stage's page if it exists.

## The discipline

Where the phase changes the engine or a surface (S1h.4.6), the repo's rules, which that stage cites:

- **Real computed examples first:** every view is shown on real traces, written out by a script, before a
  printer moves.
- **A change to what a surface prints moves its reference page in the same commit,** and the test that holds
  the page to the printer; a new option is a row of
  [`configuration.md`](../../../docs/kernel/configuration.md), pinned by `ein-cli/tests/config_reference.rs`.
- **A change to the grammar needs the user's ruling,** on real programs shown first; the grammar's pages are
  generated from `ein.l` and `ein.y`, and a shape newly refused gets a row in the grammar page's table.
- **No answer moves:** R, N, H, every claim's outcome and every exit code are the same before and after on
  every corpus program, by a sweep.
- **A view is taken only where it is asked for;** a run without the option is byte-identical.
- **Every golden a stage moves is counted with a command and named before it is blessed.**
- **A wrong answer met on the way** is a row of
  [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), filed `open` in a commit of its own
  before its fix.
- **The gate, then the nightly,** on each code commit.

## Acceptance

- **S1h.4.5's page was read and ruled on by the user** before anything shipped.
- **A solved problem's formula is an object a reader can see apart from its calculation,** on the banked
  two-step problem at the least, by the form the user ruled.
- **The condition *the formula is well-formed* is stated** as the conjunction of the checks that exist, each
  with a program that fails it and today's output, and the page lists what is still missing from it.
- ***Why divided and not subtracted* is answered from the formula's justification** for one problem, on the
  page.
- **The page says what Ein can and cannot say about a solution:** of the alternatives not taken, and of an
  answer that is correct by a solution that is not; each *cannot* with the program that shows it.
- **The pupil's page exists** in the form the user ruled, or the page says the user chose to leave the
  filter.
- **No answer moved,** and every moved golden was named first.

## Risks, each with what checks it

- **An explanation of what was not derived may not exist in Ein.** A derivation explains what was derived.
  S1h.4.4 tries two constructions on one problem, and the milestone README already allows the outcome: *may
  end as a stated limit*. The limit is then stated with its program.
- **The formula duplicates the plan.** The banked theory already derives `(needs Q Q')`, which is the
  formula's shape without its operations. S1h.4.2 shows the two side by side and says what a formula adds.
- **A formula as a nested fact joins on nested values** (E-3 of the calculator's findings, owned by
  [M1g's S1g.6.2](../../m1g_logic/p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md)). One problem
  is small and does not notice. S1h.4.2 notes the join and does not build an index.
- **The fold hides what the reader needs.** A folded division that left a remainder is where a reading
  starts to matter. S1h.4.3's written-out pages include the 13-apple problem, where the fold must still
  show the remainder.
- **The phase drifts into a theory of solutions.** The user drew the line; each stage's *What it must not
  do* repeats it.
- **Two phases ask the user one question.** Q-M1h.16 and P1g.10's f. [§ The order](#the-order) says who asks.

## Connections

- **[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md):** replies 31 and 36; E-17;
  the three checks; S1f.6.28's witness.
- **[P1g.10](../../m1g_logic/p1g.10_school_algebra/README.md):** expressions as data; *the trace is the
  school solution*, its acceptance line, is this phase's subject on equations; F-2 and question f.
- **[P1h.2](../p1h.2_slicing/README.md):** the slice by module is one of the three forms of the pupil's
  page.
- **[P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md):** readings, from the other side.
- **[P1h.1](../p1h.1_temporal_and_causal_theories/README.md):** a causal tree read off the trace
  ([S1h.1.4](../p1h.1_temporal_and_causal_theories/s1h.1.4_the_causal_theory_on_a_prototype.md)'s third form)
  is the pupil's page with the plan left out.
- **[M2](../../m2_nl_to_ir/README.md):** *"the solution could be correct, but reasoning not"* is M2's, by
  the user's line; a problem with its formula and its expected answer is a pair M2 would have to produce.
- **[M12](../../m12_rules_as_data/README.md):** *why not this formula* asks what the rule set does not
  hold, which is a question about rules as data.
- **[Idea 08](../../ideas/08-human-style-deductive-trace.md):** the human-style deductive trace, which the
  pupil's page is an instance of.
- **[`open_questions.md`](../open_questions.md):** Q-M1h.2, Q-M1h.16, Q-M1h.17.
