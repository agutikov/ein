# P1h.1 — Temporal and causal theories: a timeline and a causal tree, each a stdlib module of its own

**Milestone:** [M1h](../README.md)
**Status:** planned 2026-11-12 as a section of the milestone's README, on the user's reply of that day
([§ Where it comes from](#where-it-comes-from)); **written out as this directory on 2026-11-13,** on the
user's instruction of that day (*"materialize M1h phases"*). Not started. It carries no estimate (the user's
ruling of 2026-11-08). Its fifth stage **stops for the user** with real programs computed on prototypes,
before either module is written; the three stages after the stop are named here and their tasks are written
at the stop.
**Depends on:** [M1f](../../m1f_hypothesis_and_documentation/README.md)'s calculator phase,
[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md): the word theory banked under
[`examples/word_problems/`](../../../examples/word_problems/README.md), whose named moments are where the
phase starts, and
[S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md),
which banks `before.ein`, the witness of the phase's opening question (planned, not run on 2026-11-13; a
stage that needs the file before it is banked uses a scratch copy and says so).
**Read with:** [M1g's P1g.9](../../m1g_logic/p1g.9_one_equation_three_methods/README.md), whose three methods
the opening question meets from a word problem, and [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md),
which shares the answer to what *soft* and *unknown* are. Neither is waited for
([§ The order](#the-order)).
**Blocks:** [P1h.10](../p1h.10_a_school_text/README.md) uses the timeline and the causes (*who did what,
when, and why*); it can start its first two stages without them.

The milestone README's section,
[§ P1h.1](../README.md#p1h1--temporal-and-causal-theories), is the phase's source and is kept as written. This
page carries the user's words in full, what the phase starts from as it stands in the tree, and the stages.

## Where it comes from

Question 26 of the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q26) asked how a
problem's order of events is written: (a) named moments, (b) nested events, (c) a chain of sentences. The
user, 2026-11-12:

> Obviously there are at least 2 parts: order of events on a theoretical/virtual timeline - they can be
> points, they can be ranges, they can be hard, soft, unknown etc. And there is a causal tree. Both are
> separate theories, can be reused and deserve separate files in stdlib, right? it is a new phase, put it in
> the beginning of M1h. In the Bob/Alice prototype - pick the simplest that works - leave what is implemented
> already. This problem already did it's work - showed need of 2 theories - temporal and causal.

So four things are said: **two theories**, temporal and causal; each **reusable** and each **a file of its own
in the stdlib** (the reply ends that sentence *"right?"*, and where the files live is
[Q-M1h.5](../open_questions.md#q-m1h5--where-the-temporal-and-causal-modules-live-and-their-names)); the phase
goes **at the beginning of M1h**; and the banked problems **stay as they are**.

**The opening question** was added the same day, on the user's question about the banked word problems:

> what is just simply change the order: t1 Bob gave Alice 2 apples; t2 Bob has 5 apples; How many apples
> Bob had at t0 (before t1); would the same theory infer plus from gave reverted in time? Or even reverted
> plus directly with reverse arithmetic?

On the banked theory the answer is no ([§ What it starts from](#what-it-starts-from)).
[S1h.1.2](s1h.1.2_an_event_as_an_instruction_or_a_relation.md) is the stage that takes it.

## What it is

- **A temporal theory.** *"order of events on a theoretical/virtual timeline - they can be points, they can
  be ranges, they can be hard, soft, unknown etc."* Events placed on a timeline that need not be a clock: a
  point or a range; an order that is stated, one that is preferred, one the text does not give.
- **A causal theory.** *"And there is a causal tree."* What happened because of what, in the problem's
  world, beside the derivation tree of the answer, which Ein already has.
- **One rule that joins them:** a cause is before its effect.

What the phase has to find out on programs before either is designed, each in the stage named:

| the question (the milestone README's) | stage |
|---|---|
| what of the reply's list the banked named moments already cover | [S1h.1.1](s1h.1.1_what_the_moments_cover_and_the_literature.md) |
| is an event an instruction or a relation; and, once an event is read both ways, whose rule checks that two derivations of one quantity agree | [S1h.1.2](s1h.1.2_an_event_as_an_instruction_or_a_relation.md) |
| which temporal relations between points and ranges the problems need, and what each costs as rules | [S1h.1.3](s1h.1.3_the_temporal_theory_on_a_prototype.md) |
| what *soft* and *unknown* are in a language whose facts hold or do not; whether *hard, soft, unknown* is a property of a temporal fact or Ein's own fact, hypothesis and obligation | S1h.1.3, with [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md) |
| what a causal tree in the problem's world is beside the derivation tree: a relation a program states, one the theory derives, or a reading of the trace | [S1h.1.4](s1h.1.4_the_causal_theory_on_a_prototype.md) |
| how the two theories connect | S1h.1.4 |

## What it is not

- **Not Allen's interval algebra as a decision procedure, not a planner, not the event calculus rebuilt
  whole.** The literature each belongs to is surveyed in the first stage and named; what is built is what the
  phase's problems use.
- **Not a rewrite of the banked problems.** *"pick the simplest that works - leave what is implemented
  already."* No file under `examples/word_problems/` is edited before the stop; whether a module may be added
  beside `words.ein` after it is
  [Q-M1h.7](../open_questions.md#q-m1h7--an-event-as-an-instruction-or-a-relation).
- **Not a clock.** A moment is a name. If a problem of the phase needs a duration or a time of day, the number
  is the calculator's and the need is a finding, shown; nothing adds engine arithmetic.
- **Not new syntax.** Every program is written in the grammar as it ships.
- **Not the comparison of the three methods.** That is M1g's P1g.9; S1h.1.2 meets it and decides nothing of
  it.

## What it starts from

As the tree stands on 2026-11-13. Each figure is re-taken by the stage named beside it.

- **Named moments.** [`words.ein`](../../../examples/word_problems/words.ein) writes
  `(has WHO KIND NUMBER MOMENT)` and `(gave WHO WHOM KIND NUMBER BEFORE AFTER)`: a moment is a name (`t0`,
  `t1`, or `now` where a problem has no event), an event names the moment before it and the moment after, and
  **no fact relates two moments**: the order is the two names on the event.
  [`word_problems.md` § 1](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#1-how-a-problem-is-written)
  ran the same problem in two more forms, nested events (`(after EVENT MOMENT)`) and a chain of numbered
  sentences (`(next s1 s2)`), with its table of what each costs.
- **One direction of time.** The rules for *gave* plan from a later moment to an earlier one
  (`gave-plan-giver`, `gave-plan-receiver`) and compute from the earlier to the later (`gave-means-less`,
  `was-given-means-more`). What did not change is carried by a rule a kind of event, `bystander-plan` and
  `bystander-keeps`.
- **The witness.** *Bob gave Alice 2 apples. Now Bob has 5. How many did Bob have before?* On a scratch copy
  of `words.ein` and `gaps.ein` with the three facts, the question has no row and the query on `not-stated`
  answers `(amount Bob apples t0)` (`ein solve build/m1h/before/before.ein`, on the release binary found in
  `ein.rs/target/release/` on 2026-11-13; the milestone README records the same of 2026-11-12). S1f.6.28 banks
  it as `examples/word_problems/before.ein` with that answer and a header that says the right one is 7.
- **What the stdlib already has for an order.** [`std.algebra`](../../../stdlib/algebra.ein) declares a
  relation's properties by activator: `transitive`, `irreflexive`, `asymmetric`, `antisymmetric`, `connex`,
  `converse`, `compose`, each with its program under `tests/stdlib/algebra/`.
  [`std.slots`](../../../stdlib/slots.ein) has adjacency in an indexed row, which the Zebra puzzle's
  *right of* uses. [`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md) has an entry for Allen's
  interval algebra, and ideas [05](../../ideas/05-zebra-puzzle-graph-reasoner.md) and
  [06](../../ideas/06-inference-rules-completeness.md) name it for spatial relations.
- **The table read backwards.** [`examples/beyond_arith/`](../../../examples/beyond_arith/)'s
  `inverse_addition.ein` and `tables_backwards.ein` are the user's *"reverse arithmetic"* as banked programs.
- **The derivation tree.** Every derived fact records the rule and the premises that gave it:
  `ein saturate FILE --dump` writes a fact's `:rule`, `--trace` writes a step a firing, and `--events`'
  `fire` line carries `rule`, `premises` and `derived`
  ([`events.md`](../../../docs/kernel/inference/events.md)).

**Names.** [S1f.6.31](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md)
moved the calculator's modules under `std.math` (2026-11-14): `std.arith` is `std.math.decimal-arith`, which this
phase's files name; the relations kept their names.

## The stages

| ID | title | depends on | stops for the user |
|---|---|---|---|
| [S1h.1.1](s1h.1.1_what_the_moments_cover_and_the_literature.md) | What the banked moments already cover, the literature named, and the problems the phase is held to | the banked word problems | no; nothing outside `plans/` and `build/` is written |
| [S1h.1.2](s1h.1.2_an_event_as_an_instruction_or_a_relation.md) | An event as an instruction or a relation: the two ways to answer 7, compared on programs | S1h.1.1; `before.ein` or its scratch copy | no; its question is put at S1h.1.5 |
| [S1h.1.3](s1h.1.3_the_temporal_theory_on_a_prototype.md) | The temporal theory on a prototype: points and ranges, what each relation costs, and what *soft* and *unknown* are | S1h.1.1, S1h.1.2 | no; its questions are put at S1h.1.5 |
| [S1h.1.4](s1h.1.4_the_causal_theory_on_a_prototype.md) | The causal theory on a prototype: a causal tree beside the derivation tree, and the rule that joins the two theories | S1h.1.3 | no; its question is put at S1h.1.5 |
| [S1h.1.5](s1h.1.5_the_design_page_and_the_stop.md) | The design page and the stop | S1h.1.1 to S1h.1.4 | **yes: the phase's stop.** [Q-M1h.5](../open_questions.md#q-m1h5--where-the-temporal-and-causal-modules-live-and-their-names), [.7](../open_questions.md#q-m1h7--an-event-as-an-instruction-or-a-relation), [.8](../open_questions.md#q-m1h8--the-temporal-theorys-first-cut-and-what-soft-and-unknown-are), [.9](../open_questions.md#q-m1h9--what-a-causal-tree-is), [.10](../open_questions.md#q-m1h10--the-programs-that-are-not-word-problems). Nothing outside `plans/` and `build/` is written before the user has read its page |
| [S1h.1.6](s1h.1.6_the_temporal_module.md) | The temporal module, in the stdlib | the rulings of S1h.1.5 | no; **its tasks are written at S1h.1.5** |
| [S1h.1.7](s1h.1.7_the_causal_module_and_the_rule_that_joins_them.md) | The causal module, and the rule that joins the two | S1h.1.6; the rulings | no; **its tasks are written at S1h.1.5** |
| [S1h.1.8](s1h.1.8_programs_that_are_not_word_problems.md) | Each module used by a program that is not a word problem; and what the word problems take | S1h.1.6, S1h.1.7; Q-M1h.7, Q-M1h.10 | no; **its tasks are written at S1h.1.5** |

No stage carries an estimate. Findings go to `findings.md` in this directory, opened at the first one
([M1h § Stage, phase, milestone](../README.md#stage-phase-milestone)).

## The order

1. **S1h.1.1 first.** It says what the named moments already are in the reply's terms and fixes the set of
   problems every later stage is run on.
2. **S1h.1.2,** the opening question, before the temporal prototype: whether an event is an instruction or a
   relation decides what the temporal theory relates.
3. **S1h.1.3, then S1h.1.4.** The causal prototype needs an order to join to.
4. **S1h.1.5, and it stops.**
5. **S1h.1.6 to S1h.1.8** after the user's rulings, in that order.

Against other phases and milestones, stated and not resolved:

- **M1g's P1g.9.** If its S1g.9.1 has run, S1h.1.2 reads its programs for *arithmetic read backwards* and
  *algebra*; if not, S1h.1.2 writes its own from `examples/beyond_arith/`, and S1g.9.1 takes the word
  problem as the witness its README already lists. Neither waits for the other.
- **M1g's P1g.10.** The algebra step, *before = after + x, so before is after plus x*, is
  [S1g.10.4](../../m1g_logic/p1g.10_school_algebra/s1g.10.4_linear_equations.md)'s linear equation. If it has
  not run, S1h.1.2 writes the one rule it needs in its prototype and says that the rule is that stage's.
- **P1h.3.** What *soft* and *unknown* are is asked once. Whichever of S1h.1.3 and
  [S1h.3.1](../p1h.3_gaps_assumptions_and_readings/s1h.3.1_the_three_forms_of_a_gap.md) runs first writes the
  three forms on its own problems; the other re-reads them and adds its own.
- **Where M1h sits against M1g** is [Q-M1h.1](../open_questions.md#q-m1h1--where-m1h-sits-against-m1g), the
  user's.

## Acceptance

- **S1h.1.5's page was read and ruled on by the user** before any file outside `plans/` and `build/` was
  written.
- **Each row of [§ What it is](#what-it-is)'s table has a program whose answer the binary computed,** or the
  page says what could not be done and why, with the program that shows it.
- **Two stdlib modules, a temporal and a causal theory** (or what the user ruled in their place at S1h.1.5),
  each with what a stdlib module has: a program per rule or tight family under `tests/stdlib/`, an `:expect`
  in each, the mutation census, a `:why` on every relation and rule, the manifest, a corpus entry a file
  ([`AGENTS.md` § Edits that go together](../../../AGENTS.md)).
- **Each is used by a program that is not a word problem,** banked with its claim.
- **The opening question is answered on programs:** the two ways side by side on one page, with what each
  costs and what each does to a problem that states an amount on both sides of an event.
- **The banked problems answer as they did** on every commit of the phase, but for `before.ein` if the user
  rules that its claim moves.
- **The repo's discipline:** a wrong answer of the engine is a row of
  [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), filed `open` before its fix; every other
  finding is a row of this phase's `findings.md`, shown; every number on a page carries its command and date;
  `./run_tests.sh` is green at every stage boundary that changed a tracked file outside `plans/`.

## Risks, each with what checks it

- **A transitive closure over a text's events does not stay small.** An order closed forward over n events is
  on the order of n² facts. S1h.1.3 measures it at several n (facts, firings, time through
  `utils/bench_env.sh`) and tries the calculator's remedy, demand-driven rules that answer only what is
  asked.
- **A saturation that does not stop.** A moment built as a term from a moment, `(after E M)`, is one rule
  away from the engine's missing word
  ([M1g's C-1](../../m1g_logic/p1g.6_engine_requirements/README.md#the-candidates-as-seeded-2026-11-11)).
  Every prototype run carries `-T`, and a theory that builds terms is written demand-driven.
- ***Soft* reads as *retractable*,** and a fact that is retracted is a non-monotone program
  ([F21](../../followups/f21_non_monotone_programs_decided.md)). Every prototype is put through
  `ein solve --monotone`, and a flagged one is shown with its site and not shipped without the user's word.
- **H is not empty by default.** A program beside a theory that does not close its relations hands them to
  the blind enumerator. Every program of the phase states its lattice or imports a closing rule, as the word
  theory does.
- **Ordinary words.** Imported flat, a temporal theory occupies `before` and `after` in every program that
  imports it. Q-M1h.5, with S1f.6.30's investigation.
- **The causal tree and the derivation tree blur.** A page that says *the cause* where it means *the premise
  of the rule that fired* misleads. S1h.1.4 fixes the two words on programs before any page uses them.
- **Scope.** Each of Allen's algebra, planning and the event calculus is a subject. S1h.1.1 names them and
  the problem set is the boundary: a relation no problem of the set uses is not built.

## Connections

- **[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md):** the word theory, its three
  forms of an order, and `before.ein`.
- **[M1g](../../m1g_logic/README.md):** P1g.9, one equation by three methods; P1g.10's linear equations;
  C-1, a saturation that does not stop.
- **[P1h.2](../p1h.2_slicing/README.md) and [P1h.4](../p1h.4_the_formula_and_the_calculation/README.md):** if
  the causal tree is a reading of the trace, it is their slice and their pupil's page seen from this side;
  S1h.1.4 says so where it finds it.
- **[P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md):** *soft* and *unknown* are its gaps and
  readings met in time.
- **[P1h.10](../p1h.10_a_school_text/README.md):** a story is the first program that is not a word problem
  with both a timeline and causes.
- **Ideas:** [09, puzzles beyond Zebra](../../ideas/09-puzzles-beyond-zebra.md), whose state-transition
  puzzles are where a timeline becomes a plan, which this phase stops short of.
- **[`open_questions.md`](../open_questions.md):** Q-M1h.5 and Q-M1h.7 to .10.
