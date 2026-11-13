# M1h — Beyond word problems: time and cause, what an answer used, what a problem leaves out, and three applications

**Status:** planned 2026-11-12, on the user's instruction of that day ([§ The instruction](#the-instruction)).
**Written out as phase directories and stage files on 2026-11-13,** on the user's instruction of that day:
*"materialize M1h phases"*. Not started. It carries no estimate (the user's ruling of 2026-11-08). Each phase
has a directory with a README and a file a stage ([§ The phases](#the-phases)); each phase's section below is
its source and is kept as it was written on 2026-11-12, with a pointer at its head. The questions are in
[`open_questions.md`](open_questions.md), Q-M1h.1 to .20, none ruled.
**Depends on:** [M1f](../m1f_hypothesis_and_documentation/README.md)'s calculator phase,
[P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md): the word problems banked under
[`examples/word_problems/`](../../examples/word_problems/README.md)
([S1f.6.20](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md),
[S1f.6.28](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md)) are
what every question below was first met on, and
[`word_problems.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md) is the
investigation's page.
**Where it sits against [M1g](../m1g_logic/README.md) is not ruled** ([§ Open questions](#open-questions), 1).
**Blocks:** nothing.

## The instruction

The user wrote the milestone into
[`questions.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md) of the calculator's
phase on 2026-11-12, above the questions on word problems. In full, as typed:

> TODO: New milestone M1h - further work beyond word problems. 3 phases:
> p1h.10 - pick a prose text for like end of elementry school (3-5th grade) 50-100 sentences, very famous everybody would remember from school. Write 50 questions grouped into 5 groups by complexity from elementary (who did what) to very complex (idk maybe: feelings, motivation explanation, call of duty, moral dilemmas, what auther wanted to say, etc.)
> And then formalize the text and questions, and then write 5 theories (or one theory and 5 ontologies) that allow answering those groups of questions. I expect theories complexity would grow (measuring complexity is itself a question, but let's now just count rules, rules size, facts, objects and names).
> p1h.20 - requirements verification. How is it done in real world? e.g. in systems design, or market research, construction, renovation, hair styling, cooking, anywhere. First survey methods of requirement verification. I think there are many, different tables, analysis, answering pre-defined or template questions and other approaches. How it is done? Probably my modelling. Like threats modelling, failures modelling, and many different modellings.
> p1h.30 - TRIZ/TIPS by Genrikh Saulovich Altshuller; can we formalize it in Ein?

In the replies below it, the same day, the user sent four more subjects to M1h, each from a question about
the word problems: two theories *"in the beginning of M1h"* (reply 26), *"another new phase in M1h"* (reply
27), *"return back to investigations of this in M1h"* (reply 29) and *"those questions go into M1h"* (reply
30), and *"Yes, this is a phase in M1h"* (reply 36). They are the phases P1h.1 to P1h.4 here; each section
quotes its reply in full.

**The numbers.** P1h.10, P1h.20 and P1h.30 are the user's own. The four phases that came out of the replies
take 1 to 4: reply 26 says its phase goes *"in the beginning of M1h"*, and the user's three were numbered
with room before them. The numbers 5 to 9 and those between the user's three are free.

## What this is

The calculator's word problems were the first Ein programs in which a theory of *what words mean* stood
between a text and an answer. They are small on purpose (*Bob has 5 apples*), and by the user's reading of
them they *"already did it's work"*: each question the prototype could not settle by itself points at a
subject larger than school arithmetic. This milestone takes those subjects one at a time, then tries what
they give on three applications that are not arithmetic.

| phase | the subject | what it comes from |
|---|---|---|
| [P1h.1](#p1h1--temporal-and-causal-theories) | Temporal and causal theories: two reusable stdlib theories, a timeline and a causal tree | reply 26 |
| [P1h.2](#p1h2--slicing) | Slicing: what of a program an answer used, shown | reply 27; round 2's item 33 |
| [P1h.3](#p1h3--gaps-assumptions-and-readings) | Gaps, assumptions and readings | replies 28 to 33; round 2's items 30 and 33 |
| [P1h.4](#p1h4--the-formula-and-the-calculation) | The formula and the calculation | reply 36; reply 31; E-17 |
| [P1h.10](#p1h10--a-school-text-fifty-questions-five-theories) | A school text, fifty questions, five theories | the instruction |
| [P1h.20](#p1h20--requirements-verification) | Requirements verification: a survey first | the instruction |
| [P1h.30](#p1h30--triz) | TRIZ: *can we formalize it in Ein?* | the instruction |

## What this is not

- **Not natural-language parsing.** A text's sentences are written as facts by hand, with the sentence
  beside them as a `:source`. Producing those facts from the text is [M2](../m2_nl_to_ir/README.md)'s, and
  every pair of a sentence and its facts this milestone writes is an instance of what M2 would have to
  produce.
- **Not a theory of solutions or of Ein itself.** The user drew that line in reply 31: *"the solution could
  be correct, but reasoning not - this case is M2 and Ein theory of itself or theory of solutions - so out
  of the scope here."* P1h.4 goes as far as *formula against calculation* and stops there.
- **Not engine arithmetic, and not new syntax by default.** Every number is the calculator's; every program
  is written in the grammar as it ships. Where a phase wants something the engine or the grammar lacks, it
  is a finding of that phase, shown, and a question for the user.
- **Not decided ahead of its programs.** Each phase starts with real programs whose answers the binary
  computed, shown to the user before a module ships (the user's rule: real examples first).

## The phases

Written out 2026-11-13. A stage past a phase's stop is a file in outline: it says what it rests on, what it
carries and what it must not do, and its tasks are written at the stop, to the user's rulings
([§ Stage, phase, milestone](#stage-phase-milestone)).

| ID | title | stages | stops for the user | ends with |
|---|---|---:|---|---|
| [P1h.1](p1h.1_temporal_and_causal_theories/README.md) | Temporal and causal theories: a timeline and a causal tree, each a stdlib module of its own | 8, three of them in outline | [S1h.1.5](p1h.1_temporal_and_causal_theories/s1h.1.5_the_design_page_and_the_stop.md), the design page: an event as an instruction or a relation, the first cut of each theory, where the modules live | two stdlib modules, or what the user rules in their place; each used by a program that is not a word problem; the opening question answered on programs |
| [P1h.2](p1h.2_slicing/README.md) | Slicing: what of a program an answer used, shown. An engine phase where it adds a surface | 6, two in outline | [S1h.2.4](p1h.2_slicing/s1h.2.4_what_used_means_and_the_stop.md): what *used* means, whether a restriction that was tested and held is recorded, which surfaces | the cut printed and dumped, on the surfaces ruled; the page that says what *used* means; the two opening questions answered on programs; no answer moved |
| [P1h.3](p1h.3_gaps_assumptions_and_readings/README.md) | Gaps, assumptions and readings: what a problem leaves out, and how a program says so | 7, one in outline | [S1h.3.6](p1h.3_gaps_assumptions_and_readings/s1h.3.6_the_page_and_the_stop.md): which of the banked forms change, and universal assumptions as a module | the forms of a gap, of an assumption and of a reading each compared on the same problems; the questions of replies 30 and 33 answered with programs; what the user rules, built |
| [P1h.4](p1h.4_the_formula_and_the_calculation/README.md) | The formula and the calculation: the two parts of a solution, and the line between them. An engine phase where a view of the trace is one | 7, two in outline | [S1h.4.5](p1h.4_the_formula_and_the_calculation/s1h.4.5_the_page_and_the_stop.md): the pupil's page, and the formula as an object | a solved problem's formula an object a reader can see apart from its calculation; the page on what Ein can and cannot say about a solution |
| [P1h.10](p1h.10_a_school_text/README.md) | A school text, fifty questions, five theories | 5, three in outline | **twice:** [S1h.10.1](p1h.10_a_school_text/s1h.10.1_the_text.md), the short list of texts, and [S1h.10.2](p1h.10_a_school_text/s1h.10.2_fifty_questions_in_five_groups.md), the five groups and the fifty questions; and once inside S1h.10.4, on the two shapes of a theory | the text, the fifty questions, their formalization, the theories, and the table of the five counts |
| [P1h.20](p1h.20_requirements_verification/README.md) | Requirements verification: a survey first | 2: the survey, and the place of the plan that follows it | [S1h.20.1](p1h.20_requirements_verification/s1h.20.1_the_survey.md): the user reads the survey; whether the phase ends in programs | the survey, read; the plan that followed it |
| [P1h.30](p1h.30_triz/README.md) | TRIZ: *can we formalize it in Ein?* | 2: the survey with one problem by hand, and the place of the plan that follows it | [S1h.30.1](p1h.30_triz/s1h.30.1_the_survey_and_one_problem_by_hand.md): the user reads the survey before anything is built | the survey, read; the plan that followed it |

Thirty-seven stage files (`ls plans/m1h_beyond_word_problems/p1h.*/s1h.*.md | wc -l`, 2026-11-13), thirteen
of them in outline, two of those holding a place with no content.

## P1h.1 — Temporal and causal theories

**Written out 2026-11-13:** [`p1h.1_temporal_and_causal_theories/`](p1h.1_temporal_and_causal_theories/README.md),
eight stages. This section is the phase's source and is as it was written on 2026-11-12.

Question 26 of the calculator's `questions.md` asked how a problem's order of events is written: named
moments, nested events, or a chain of sentences. The user, 2026-11-12:

> Obviously there are at least 2 parts: order of events on a theoretical/virtual timeline - they can be
> points, they can be ranges, they can be hard, soft, unknown etc. And there is a causal tree. Both are
> separate theories, can be reused and deserve separate files in stdlib, right? it is a new phase, put it in
> the beginning of M1h. In the Bob/Alice prototype - pick the simplest that works - leave what is implemented
> already. This problem already did it's work - showed need of 2 theories - temporal and causal.

So the phase is **two theories, each a stdlib module of its own, each reusable** outside word problems:

- **A temporal theory.** *"order of events on a theoretical/virtual timeline - they can be points, they can
  be ranges, they can be hard, soft, unknown etc."* Events placed on a timeline that need not be a clock:
  a point or a range; *hard* (stated), *soft* (preferred or likely, and so retractable), *unknown* (an
  order the text does not give). What the banked problems have today is the simplest case, named moments,
  `t0` and `t1`, with every event carrying two names; the phase starts by saying what of the reply's list
  that already covers.
- **A causal theory.** *"And there is a causal tree."* What happened because of what: *Bob has 3 because he
  had 5 and gave 2*. In Ein a derivation is already a tree of justifications; the phase's first question is
  what a causal tree *in the problem's world* is beside the derivation tree of the answer, and whether it
  is a relation a program states, one the theory derives, or a reading of the trace.

**What it has to find out, on programs,** before either module is designed:

- which of the temporal relations between points and ranges (before, during, overlapping) the phase's
  problems need, and what each costs as rules: an order is a transitive relation, and a transitive closure
  over a text's events is the first thing that will not stay small;
- what *soft* and *unknown* are in a language whose facts hold or do not: a soft order as a hypothesis
  with nothing owed, an unknown order as a debt, or as states of R (the two orders of two events are two
  states). This is P1h.3's question about gaps and readings met again in time, and the two phases share
  the answer;
- whether *hard, soft, unknown* is a property of a temporal fact or the general distinction between a
  fact, a hypothesis and an obligation that Ein already has;
- how the two theories connect: a cause is before its effect, which is one rule joining them.

**Its opening question: is an event an instruction or a relation?** Added 2026-11-12, on the user's
question of that day about the banked word problems:

> what is just simply change the order: t1 Bob gave Alice 2 apples; t2 Bob has 5 apples; How many apples
> Bob had at t0 (before t1); would the same theory infer plus from gave reverted in time? Or even reverted
> plus directly with reverse arithmetic?

On the banked theory the answer is no: the question has no row and the amount at `t0` is named as not
stated (`ein solve` on a scratch copy of `examples/word_problems/words.ein` and `gaps.ein`, 2026-11-12).
The rules read *gave* forwards only, *after = before − x*, and plan only from a later moment to an earlier
one. The problem is banked as a witness by
[S1f.6.28](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md),
`examples/word_problems/before.ein`, with today's answer. Two ways to make it answer 7, for this phase to
compare on programs:

- **Each word written twice,** once in each direction of time: *if the amount before a giving is wanted and
  the amount after is known, it is after + x*. It works over the calculator as shipped; the planning then
  runs both ways along the timeline.
- **The event stated once, as a relation between the moments on both sides of it,** *before = after + x*,
  and whichever of the three is unknown asked for. The direction is then in no word rule. Finding the
  unknown is either a rule that knows the inverse of a sum, which is the algebra step, or the addition table
  read backwards, the user's *"reverted plus directly with reverse arithmetic"*: the three methods of
  [M1g's P1g.9](../m1g_logic/p1g.9_one_equation_three_methods/README.md), met from a word problem.

And what follows from either: once an event is read both ways, a problem that states an amount on both
sides of it has two derivations of one quantity, and where they disagree the problem contradicts itself.
Nothing checks that today. The temporal theory says what an event relates; whether that check is its rule
or the word theory's is part of the question.

**Where it stops:** not Allen's interval algebra as a decision procedure, not a planner, not the event
calculus rebuilt whole. The literature each of those belongs to is surveyed in the phase's first stage and
named; what is built is what the phase's problems use.

The banked problems are **not** rewritten to it: *"pick the simplest that works - leave what is implemented
already."*

## P1h.2 — Slicing

**Written out 2026-11-13:** [`p1h.2_slicing/`](p1h.2_slicing/README.md), six stages. This section is the
phase's source and is as it was written on 2026-11-12.

Question 27 asked whether the word theory works backwards from the question (nothing computed that no
question needs). The user, 2026-11-12:

> (a), another new phase in M1h, theory slicing or something like - question and inference of the answer
> cuts the program (which is a model of a problem) into 2 parts used/unused, required/superfluous. Ein must
> allow showing (print, dump) those parts.

So: **a question and the inference of its answer cut the program into two parts, and Ein shows both.** The
program is *a model of a problem*; the parts are *used / unused*, *required / superfluous*.

What exists to start from: every derived fact records its justification, so the facts and rules an answer
rests on can be collected by walking it; `--trace -l` prints *the goal-relevant slice* of a trace; a dead
node's core is a slice of another kind, the facts a contradiction rests on; the banked `unused.ein` and
`bystander.ein` are problems with a fact no question needs. What does not exist: the cut as an object, of
the **program** (its facts, its rules, its imported modules' rules) and not of the trace, printed and
dumped; and its complement, the part the answer did not touch.

**Its two opening questions** are from
[round 2, item 33](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q33), where the
user's reply 33 (quoted under P1h.3) was answered and both points were sent here, to which the user replied
*"OK"* (`dc88520b`):

1. **Assumptions as premises make the slice name them.** *If an assumption is a fact that the dependent
   rules take as a premise, then the slice of an answer contains exactly the assumptions that answer rests
   on, with no extra mechanism, and one the derivation never touches is not required for that answer.* So
   the user's question, *"if we add rules and ontology that restricts throwing things away and after that
   the solution will not touch those rules and facts (see my answer 27) - that means this is not required
   information?"*, has the answer *yes, for that answer*, **provided** assumptions are written that way.
   The phase tests it on the banked problems.
2. **A restriction that was checked and held leaves no trace.** *"Nobody gives away what they do not have"
   derives nothing when it holds; it would only have derived a contradiction. So it leaves no trace in the
   answer's derivation, yet the answer is valid only because it passed. "Untouched" then has two meanings,
   "not applicable to this problem" and "applicable, tested, satisfied", and only the first is superfluous
   information.* So a slice by derivation alone under-reports what an answer needs, and the phase has to
   find what records that a restriction was *tested*: its premises matched in part, or the rule's plan was
   run against the state.

**Further questions the phase meets:** whether a slice is of one answer or of a query's every class;
whether *required* means *used by this derivation* or *used by every derivation* (a fact with two
justifications has two slices; a core is an explanation and may vary with rule order, the user's ruling of
2026-11-08, and a slice inherits that unless it is defined as an intersection or a union); what the unused
part of an imported theory is (most of the calculator, for any one sum); and the surface: a read-out
section, a `--dump-answer` file a slice, a view of `ein render program` with the two parts told apart.

It is an engine phase where it adds a surface, and follows the repo's discipline there: the read-out's
reference page and test move with the printer; no answer moves; a slice is taken only where a surface
prints it (*minimum work per question*).

## P1h.3 — Gaps, assumptions and readings

**Written out 2026-11-13:**
[`p1h.3_gaps_assumptions_and_readings/`](p1h.3_gaps_assumptions_and_readings/README.md), seven stages. This
section is the phase's source and is as it was written on 2026-11-12.

Six replies of 2026-11-12, to the questions on what a problem leaves out, each in full.

**Question 28, how a missing quantity is detected** (*Bob has 5, gave 2 to Alice; how many together?*
Nothing says what Alice had):

> see my answer in question 24; in general - need to discuss, analyze, needs a separate stage. But I want
> already written word problems banked as is, so pick recommended, and note that is is not only and not
> final.

**Question 29, how an assumption is written and reported** (*Assume that Alice had none.*):

> again - for this stage (a) is OK, return back to investigations of this in M1h

**Question 30, a named gap together with a supposition:**

> that simple problem couldn't have non-monotone solution, I mean the problem is too simple and too obvious,
> or not? or that means that simple problem, containing ambiguity, could already be non-monotone? Or that
> means ambiguity == non-monotone solution == required additional information? Again - those questions go
> into M1h. Here I don't care - so pick (a).

**Question 31, a word with two readings** (*13 apples, a box holds 3: how many boxes do they need?*):

> reading always chosen, it can't be not chosen: result depends on formula, formula depends on selected
> arith operations, selection depend on reading. The synthesized program must be able answer question about
> the solution, like "why you divided those numbers? why not 3/12, not 3+12, not 12-3, and so on" btw there
> could be the case when answer is correct, but solution is not, right? And finally the solution could be
> correct, but reasoning not - this case is M2 and Ein theory of itself or theory of solutions - so out of
> the scope here.

**Question 32, is *Alice has 3 pears* also *Alice has no apples*:**

> what is not listed is zero - good idea, it could be a reusable universal assumption.

and, in round 2, to the proposal that it be a module a problem imports when it adopts it (`dc88520b`):

> Yes, relation arguments are marked with some objects that can be treated as types. So can add a rule that
> assigns zero to every unknown and deduce (has Alice apples 0). It doesn't work in zebra for example - if
> nobody lives in the house means we yet don't know who lives there.

**Question 33, how the theory states what its model leaves out:**

> Is this a finite set of assumptions? What is it limited by? Used names/words? Probably we don't need to say
> that things are not alive and don't run out of the boxes, becasue the problem doesn't contain words "alive"
> and "run", or do we?
> But the same time we assume things fit boxes, while problem doesn't have this word.
> Btw, if we add rules and ontology that restricts throwing things away and after that the solution will not
> touch those rules and facts (see my answer 27) - that means this is not required information?

**What is banked meanwhile,** and is *"not only and not final"*: the gap as a derived fact by `absent`, in
a module of its own; an assumption as a marked fact with a `rests-on` fact beside the answer; readings as
hypotheses, always owed after the calculator's S1f.6.28; *what is not listed is zero* as a module a problem
imports; and a read-out row for what root owes (S1f.6.26).

**What round 2 answered in outline,** which the user accepted there (`dc88520b`: *"OK (a)"* on item 30, *"OK"* on
item 33) and which the phase starts from and re-takes on programs:

- [Item 30](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q30), on the user's
  chain *ambiguity == non-monotone solution == required additional information*: *Not quite, and the
  difference is worth having for M1h.* **Ambiguity is monotone:** R has several states and nothing is
  retracted. What is non-monotone is one sentence, *Alice's amount is **not stated***, true of the problem
  as given and false as soon as anything supposes or supplies the amount. The gap written **as a fact** is
  non-monotone, for every problem, however simple; the gap written **as a debt** is monotone, because it is
  the engine and not a rule that says *missing*. So: *required additional information = a debt; ambiguity =
  several states; neither needs non-monotone rules unless the program wants the missing thing as a fact to
  reason from.*
- [Item 33](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q33), on whether the
  set of assumptions is finite: *In the world the set is not finite (the frame problem). In a theory it is
  bounded by the rules, not by the text's words. An assumption has to be stated exactly where a rule's
  conclusion depends on it.* *Things fit boxes* is needed although the text never says *fit*, because the
  rule that reads *a box holds 3* as a division is valid only under it; *things are not alive* is needed by
  no rule. *The set grows with the vocabulary the theory covers, a few per rule.*

**The phase's questions,** each to be answered with programs:

- **The three forms of a gap, compared properly:** a debt, a fact by `absent`, a hypothesis over a list.
  What each says in the read-out, what each lets a rule reason from, and what each costs; with the
  open-against-false question (the user's *"see my answer in question 24"*), which is ruled in
  [M1g's P1g.7](../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md): a missing amount that is simply
  not stated and an equation with no root both leave a debt unpaid, for different reasons.
- **Assumptions:** marked facts against hypotheses the search may commit to; which of a theory's
  assumptions are *universal* and reusable across theories (*what is not listed is zero* is the closed
  world for one relation; the user calls it *"a reusable universal assumption"* and names where it fails:
  *"It doesn't work in zebra"*, where an empty slot is *not known yet*; so *universal* means reusable,
  adopted relation by relation, and the phase says what decides whether a relation may adopt it), and whether such
  assumptions are a stdlib module; how an answer reports what it rests on (P1h.2's slice).
- **Which assumptions a model has to state at all:** the user's question 33, tested: for each rule of the
  word theory, the assumption that makes its reading valid, written as a premise; then the count, and
  whether *a few per rule* holds.
- **Readings:** *a reading is always chosen*, and its consequence, two states with one answer. What tells a
  right answer by the wrong reading from a right solution, which is P1h.4's subject from the other side.
- **Non-monotone or not:** round 2's distinction held to programs, with
  [F21](../followups/f21_non_monotone_programs_decided.md), which owns what a non-monotone program means.

## P1h.4 — The formula and the calculation

**Written out 2026-11-13:**
[`p1h.4_the_formula_and_the_calculation/`](p1h.4_the_formula_and_the_calculation/README.md), seven stages.
This section is the phase's source and is as it was written on 2026-11-12.

Question 36 asked whether folding an imported theory's steps in a trace (*the trace as the pupil's page*)
is part of the word-problem stage. The user, 2026-11-12:

> How we can divide 2 parts? inference of a formula and calculations, the second is already solved (in
> arithmetic at least), but only if formula is well-formed, correct as a formula itself (all operations have
> required arguments, numbers and names correctly used, no division by zero, etc...). Yes, this is a phase in
> M1h

So a solution has **two parts**: *inference of a formula*, and *calculations*. The second *"is already
solved (in arithmetic at least)"*, on one condition, that the formula is *"well-formed, correct as a formula
itself"*: every operation has its arguments, numbers and names are used correctly, no division by zero. The
phase is the first part, and the line between the two.

**What it starts from:**

- **E-17** of the calculator's
  [`findings.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md): `--trace` cannot show
  an importing theory's steps without the imported theory's. The two-step word problem is 139 steps, 10 of
  them the word theory's and the rest the calculator's; `-l`, the goal-relevant slice, keeps 137. Nothing
  folds a call into one line, *12 ÷ 3 = 4, remainder 0 (116 steps of `std.arith`)*. A script on the
  prototype filters the trace. P1g.10 met the same thing on an equation (3 steps of the pupil's in 123).
- **The formula as an object.** Today the formula of a word problem exists nowhere: it is the order in
  which rules ask the calculator. P1g.10's expressions (`(calc E)`, expression data) are where a formula
  can be a value, so the phase reads what P1g.10 built before proposing its own.
- **The checks of a formula** the reply lists are, each, something an earlier stage built or ruled: *all
  operations have required arguments* (the loader's arity check, S1f.6.23); *numbers and names correctly
  used* (malformed numbers and undeclared names, S1f.6.12's build; used and recursive names, P1g.10's
  S1g.10.2); *no division by zero* (the flag at the call, S1f.6.24). The phase states the condition *the
  formula is well-formed* as their conjunction and shows what is still missing from it.
- **Reply 31's questions about a solution:** *"why you divided those numbers? why not 3/12, not 3+12, not
  12-3, and so on"*, and *"there could be the case when answer is correct, but solution is not, right?"*.
  The calculator's S1f.6.28 banks the first corpus witness of the second (12 apples and boxes of 3: two
  readings, two states, one answer) and writes down what the trace says of the first as far as the rules
  go.

**The phase's questions:**

- **The trace as the pupil's page:** a view of a run at one theory's level, the imported theory's steps
  folded to a line each. Whether that is a `--trace` option, a property of an import, or P1h.2's slice
  drawn another way; an engine change where it is one, with the reference pages moving with it.
- **The formula inferred and shown:** the word theory deriving the formula as a fact before, or instead
  of, asking the calculator step by step; then *why divided and not subtracted* is a question about which
  rule built which node of the formula, answerable from its justification.
- **The alternatives not taken.** *"why not 3/12, not 3+12, not 12-3"* asks about formulas the theory did
  **not** build. A derivation explains what was derived; an explanation of what was not is a different
  object (each alternative, and the premise it lacks). What Ein can say there is the phase's hardest
  question and may end as a stated limit.
- **Answer correct, solution not:** how a program or a reader tells them apart, given two states with one
  answer; what a *wrong* solution is when both readings are admissible.

**Where it stops:** *"the solution could be correct, but reasoning not"* is M2's and a theory of
solutions', by the user's own line.

## P1h.10 — A school text, fifty questions, five theories

**Written out 2026-11-13:** [`p1h.10_a_school_text/`](p1h.10_a_school_text/README.md), five stages. This
section is the phase's source and is as it was written on 2026-11-12.

The instruction:

> p1h.10 - pick a prose text for like end of elementry school (3-5th grade) 50-100 sentences, very famous
> everybody would remember from school. Write 50 questions grouped into 5 groups by complexity from
> elementary (who did what) to very complex (idk maybe: feelings, motivation explanation, call of duty, moral
> dilemmas, what auther wanted to say, etc.)
> And then formalize the text and questions, and then write 5 theories (or one theory and 5 ontologies) that
> allow answering those groups of questions. I expect theories complexity would grow (measuring complexity is
> itself a question, but let's now just count rules, rules size, facts, objects and names).

The steps are the instruction's, in its order:

1. **Pick the text:** prose, of the end of elementary school (3rd to 5th grade), 50 to 100 sentences,
   *"very famous everybody would remember from school"*. The choice is the user's; the phase proposes a
   short list with, for each, its length, its language and whether its text may be reproduced in the
   repository, and stops.
2. **Write 50 questions in 5 groups by complexity,** from the elementary (*"who did what"*) to the very
   complex. The upper groups are the user's guesses, marked as such in the instruction (*"idk maybe"*):
   *feelings, motivation explanation, call of duty, moral dilemmas, what auther wanted to say*. The phase
   proposes the five groups and the questions and stops for the user before formalizing any.
3. **Formalize the text and the questions:** the sentences as facts, by hand, each with its sentence as
   `:source`; each question as a query, with the expected answer written before the theory exists.
4. **Write five theories, *"or one theory and 5 ontologies"*,** each allowing the questions of its group
   to be answered. Which of the two shapes is a question the phase answers by trying
   ([§ Open questions](#open-questions), 3).
5. **Measure how the theories grow.** *"I expect theories complexity would grow (measuring complexity is
   itself a question, but let's now just count rules, rules size, facts, objects and names)."* The five
   counts are taken for each theory by a script, per group and cumulatively, and reported as a table; a
   better measure of complexity is noted as a question and not pursued.

What it uses of the earlier phases: P1h.1's timeline and causes (*who did what, when, and why*), P1h.2's
slice (what of the text a question's answer used), P1h.3's gaps and assumptions (a story leaves almost
everything unsaid). What it will show first is where those end: a question about a feeling or a motive
needs facts no sentence states, and the theory that supplies them is the measured thing.

## P1h.20 — Requirements verification

**Written out 2026-11-13:**
[`p1h.20_requirements_verification/`](p1h.20_requirements_verification/README.md), the survey's stage and the
place of the plan that follows it. This section is the phase's source and is as it was written on
2026-11-12.

The instruction:

> p1h.20 - requirements verification. How is it done in real world? e.g. in systems design, or market
> research, construction, renovation, hair styling, cooking, anywhere. First survey methods of requirement
> verification. I think there are many, different tables, analysis, answering pre-defined or template
> questions and other approaches. How it is done? Probably my modelling. Like threats modelling, failures
> modelling, and many different modellings.

**A survey first,** and the phase's plan past the survey is written after the user has read it. The
survey's scope is the instruction's: how requirements are verified *"in real world"*, across fields as far
apart as the user lists (*systems design, market research, construction, renovation, hair styling, cooking,
anywhere*); the methods (*"different tables, analysis, answering pre-defined or template questions and
other approaches"*); and the user's own conjecture, that it is done by **modelling**, *"threats modelling,
failures modelling, and many different modellings"*.

What the survey has to return, so that the next step can be planned from it: for each method, what its
input is (a list of requirements, a design, a model), what it produces (a table, a verdict, a list of gaps
or conflicts), and which part of it is *reasoning over stated facts*, which is the part Ein could do. Its
catalogue entries go to [`docs/lib/`](../../docs/lib/) beside the external-tech notes, by that directory's
conventions.

Its natural meeting points with Ein, to be confirmed by the survey and not assumed: the three task classes
([idea 03](../ideas/03-three-task-classes.md): solve, gaps, contradictions) read as *the requirements are
satisfiable, incomplete, or in conflict*; P1h.3's gaps and assumptions; P1h.2's slice as *which
requirements a design decision rests on*.

## P1h.30 — TRIZ

**Written out 2026-11-13:** [`p1h.30_triz/`](p1h.30_triz/README.md), the survey's stage and the place of the
plan that follows it. This section is the phase's source and is as it was written on 2026-11-12.

The instruction:

> p1h.30 - TRIZ/TIPS by Genrikh Saulovich Altshuller; can we formalize it in Ein?

The phase is that question. TRIZ, the theory of inventive problem solving (TIPS in English) of Genrikh
Saulovich Altshuller, is asked about as a body of method to be written as an Ein theory. Nothing of it is
planned past the first step, which is a survey that says what TRIZ consists of as *statable* things (its
tables, its lists of principles, its notion of a contradiction between requirements, its procedures) and,
for each, whether it is facts, rules, a search, or something Ein has no form for; with one small worked
problem carried through by hand as the target. The user reads that before anything is built. Its
neighbour is P1h.20: a contradiction between two requirements is where requirements verification ends and
where, as the survey will have to confirm, TRIZ begins.

## The order

P1h.1 first (*"put it in the beginning of M1h"*), then P1h.2, P1h.3 and P1h.4, which are numbered in the
order their replies came and lean on each other as the sections say: P1h.3 reads P1h.2's slice, and P1h.4
reads both. Then the user's three, by their numbers. P1h.20's and P1h.30's surveys read nothing of the
earlier phases and may be run at any time the user wants them.

Each phase's README has its own order, stage by stage, and says for every stage that reads another phase or
another milestone which stage builds what it reads and what it does if that has not run. The order above
stands. What the stage files add is how little of an earlier phase a later one waits for: P1h.3 and P1h.4
read [S1h.2.1](p1h.2_slicing/s1h.2.1_what_a_slice_is_today.md)'s script and do not wait for P1h.2's
surfaces, and P1h.10's first two stages, a list of texts and questions in words, read nothing of P1h.1 to
P1h.4.

## Stage, phase, milestone

How the schema of [`plans/README.md`](../README.md) was applied when the phases were written out on
2026-11-13. These are the plan's conventions and not rulings; each is revisable.

- **A phase** is one of the seven subjects, with its own source in this README, its own stop for the user
  and its own acceptance.
- **A stage** is one question with one deliverable that an agent can finish from the stage file and the
  standing rules: a page with computed programs, a prototype under `build/`, a survey, a module, an engine
  change. Each stage file gives what it rests on, in the user's words; what it starts from; what it
  produces; its tasks, with ids `T1h.<phase>.<stage>.<n>`; its acceptance; and what it must not do.
- **Real examples first, and a stop.** Every phase that could ship a stdlib module, change the grammar or
  change what a surface prints begins with stages that write real programs, run them on the binary and put
  the computed answers on a page; then a stage **stops for the user**, and nothing outside `plans/` and
  `build/` is written before the user has read its page (the user's rule of 2026-10-16). P1h.10 stops where
  its section says, twice; P1h.20 and P1h.30 are a survey each, read by the user before anything else.
- **Past a stop, an outline.** A stage that follows a stop is a file that names what it rests on, what it
  carries whatever is ruled, and what it must not do. Its tasks are written at the stop, to the rulings. The
  two that follow a survey ([S1h.20.2](p1h.20_requirements_verification/s1h.20.2_the_plan_after_the_survey.md),
  [S1h.30.2](p1h.30_triz/s1h.30.2_the_plan_after_the_survey.md)) hold a place and nothing else.
- **An engine stage says it is one,** and carries the repo's discipline: a wrong answer is an `open` row of
  [`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml) before its fix; a change to what `ein solve`
  prints moves [`read_out.md`](../../docs/kernel/inference/read_out.md) in the same commit; no answer, exit
  code or R moves unless the stage is about that; a slice or a core is taken only where a surface prints it;
  config never changes an answer; the gate, then the nightly. The engine stages are S1h.2.5, S1h.2.6 and
  S1h.4.6.
- **A stage that reads something not yet built says who builds it and what it does meanwhile.** Across
  phases (P1h.3 reads P1h.2's slice, and P1h.4 reads both) and across milestones (P1g.7's ruling, P1g.9's
  three methods, P1g.10's expressions, S1f.6.28's banked files, F21). None of these dependencies is resolved
  here: where this milestone sits against M1g is
  [Q-M1h.1](open_questions.md#q-m1h1--where-m1h-sits-against-m1g), the user's.
- **Findings stay in the phase.** Each phase keeps what it finds about the engine or the language, outside
  its own scope, in a `findings.md` in its directory, opened at the first finding and shown at the phase's
  stop (the user's ruling of 2026-11-11: a phase keeps what it finds). A row's id is the letter H, the
  phase's number and a count: `H2-1` is P1h.2's first. No file is opened empty.
- **Questions** that are the user's go to [`open_questions.md`](open_questions.md) under the next free
  `Q-M1h.<n>`. No recommendation there is a default: a stage that needs an answer stops and asks.
- **Scratch** stays under `build/s1h.<phase>.<stage>/`, which is not tracked; a page that shows a scratch
  program carries it in full.
- **The numbers.** No stage is numbered into the gaps this README reserves: the phase numbers 5 to 9 and
  those between the user's three stay free. A phase's later stages are numbered on from its last (S1h.20.3,
  S1h.30.3) when its plan past a survey is written.
- **Names.** [S1f.6.31](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md)
  moves the calculator's modules under `std.math`. Where a stage file says `std.arith`, read
  `std.math.decimal-arith` once that stage has run; the relations keep their names.

## Acceptance

Stated for the milestone as far as it can be today; each phase's own acceptance is in its README, and a
stage past a stop has its acceptance written with its tasks.

- Every phase began with programs whose answers the binary computed, or with a survey, **shown to the user
  before a module shipped**.
- **P1h.1:** two stdlib modules, a temporal and a causal theory, each with what a stdlib module has (a
  program per rule under `tests/stdlib/`, an `:expect` in each, the mutation census, a `:why` on every
  relation and rule), and each used by a program that is not a word problem.
- **P1h.2:** for a query's answer, Ein prints and dumps the part of the program it used and the part it
  did not, and the page says what *used* means, with the two opening questions answered on programs.
- **P1h.3:** the forms of a gap, of an assumption and of a reading each compared on the same problems, and
  the user's questions of replies 30 and 33 answered with programs.
- **P1h.4:** a solved problem's formula is an object a reader can see apart from its calculation, and the
  page says what Ein can and cannot say about a solution.
- **P1h.10:** the text, the 50 questions in 5 groups, their formalization, the theories, and the table of
  the five counts.
- **P1h.20 and P1h.30:** the survey, read by the user, and the plan that followed it.
- **The repo's discipline throughout:** a wrong answer of the engine is a row in
  `corpus/wrong_answers.toml`, filed `open` before its fix; each phase keeps what it finds in a
  `findings.md` of its own and shows it; a change to the grammar or to what a surface prints shows real
  computed examples first; config never changes an answer; every answer exits 0.

## Open questions

They are in [`open_questions.md`](open_questions.md) since 2026-11-13, as `Q-M1h.<n>`. None is ruled, and none
has a default that is acted on. The six that were kept here keep their numbers, so *question 1* of this page
is Q-M1h.1:

1. [Q-M1h.1](open_questions.md#q-m1h1--where-m1h-sits-against-m1g): where M1h sits against M1g. The same
   question is [Q-M1g.15](../m1g_logic/open_questions.md#q-m1g15--where-m1h-sits-against-this-milestone),
   asked once.
2. [Q-M1h.2](open_questions.md#q-m1h2--whether-it-follows-p1g10): whether it follows
   [P1g.10](../m1g_logic/p1g.10_school_algebra/README.md), school algebra; part of question 1.
3. [Q-M1h.3](open_questions.md#q-m1h3--five-theories-or-one-theory-and-five-ontologies): five theories, or
   one theory and five ontologies (P1h.10).
4. [Q-M1h.4](open_questions.md#q-m1h4--the-text-and-its-language): the text (P1h.10), and its language.
5. [Q-M1h.5](open_questions.md#q-m1h5--where-the-temporal-and-causal-modules-live-and-their-names): where
   the temporal and causal modules live in the stdlib's layout, and their names.
6. [Q-M1h.6](open_questions.md#q-m1h6--whether-p1h20-and-p1h30-end-in-programs-at-all): whether P1h.20 and
   P1h.30 end in programs at all.

Q-M1h.7 to .20 were raised by the stage files, each a thing this README leaves unsaid: each names the stage
that puts it to the user and what waits for it.

## Connections

- **[P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md):** the word problems, the
  questions and their replies ([`questions.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md),
  [`questions_round2.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md), and
  the index [`replies_2026-11-12.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/replies_2026-11-12.md)).
- **[P1g.10](../m1g_logic/p1g.10_school_algebra/README.md)** (school algebra; M1f's P1f.8 until 2026-11-13): expressions
  as data; *the trace is the school solution*, its acceptance line, is P1h.4's subject on equations.
- **[M1g](../m1g_logic/README.md):** P1g.7, unsatisfiable against owing; P1g.8, a theory of types; and
  [F26](../followups/f26_consistency_and_completeness.md)'s senses of *complete*, which a text that leaves
  things unsaid meets at once.
- **[M2](../m2_nl_to_ir/README.md):** each text with its hand-written facts is a pair M2 would have to
  produce, and P1h.10's fifty questions with expected answers are a small benchmark with graded difficulty.
- **[F21](../followups/f21_non_monotone_programs_decided.md):** non-monotone programs, for P1h.3.
- **Ideas:** [03, three task classes](../ideas/03-three-task-classes.md);
  [08, the human-style deductive trace](../ideas/08-human-style-deductive-trace.md), which P1h.4's *pupil's
  page* is an instance of; [09, puzzles beyond Zebra](../ideas/09-puzzles-beyond-zebra.md).
- **[`open_questions.md`](open_questions.md):** Q-M1h.1 to .20.
