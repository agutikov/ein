# P1h.10 — A school text, fifty questions, five theories

**Milestone:** [M1h](../README.md)
**Status:** planned 2026-11-12 as a section of the milestone's README, on the user's instruction of that day
([§ The instruction](#the-instruction)); **written out as this directory on 2026-11-13,** on the user's
instruction of that day (*"materialize M1h phases"*). Not started. It carries no estimate (the user's ruling
of 2026-11-08). **It stops for the user twice,** by the milestone README: after its first stage proposes the
short list of texts, and after its second proposes the five groups and the fifty questions. The three stages
after the second stop are named here with what the instruction says of each, and their tasks are written at
that stop, since every one of them depends on the text and the questions the user accepts.
**The text, chosen by the user on 2026-11-13** at S1h.10.1's stop: *Winnie-the-Pooh* by A. A. Milne, 1926, in
English, Project Gutenberg's eBook #67098, and of it **Chapter II**, *In which Pooh goes visiting and gets into
a tight place* ([S1h.10.1 § The choice](s1h.10.1_the_text.md#the-choice-2026-11-13)). S1h.10.2 is next.
**Depends on:** the language and the engine as they ship; nothing else for its first two stages. Its later
stages use, where they exist, [P1h.1](../p1h.1_temporal_and_causal_theories/README.md)'s timeline and causes,
[P1h.2](../p1h.2_slicing/README.md)'s slice and
[P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md)'s gaps and assumptions
([§ The order](#the-order)).
**Blocks:** nothing.

The milestone README's section,
[§ P1h.10](../README.md#p1h10--a-school-text-fifty-questions-five-theories), is the phase's source and is kept
as written.

## The instruction

The user, 2026-11-12, in the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md), above the questions
on word problems. As typed:

> p1h.10 - pick a prose text for like end of elementry school (3-5th grade) 50-100 sentences, very famous
> everybody would remember from school. Write 50 questions grouped into 5 groups by complexity from
> elementary (who did what) to very complex (idk maybe: feelings, motivation explanation, call of duty, moral
> dilemmas, what auther wanted to say, etc.)
> And then formalize the text and questions, and then write 5 theories (or one theory and 5 ontologies) that
> allow answering those groups of questions. I expect theories complexity would grow (measuring complexity is
> itself a question, but let's now just count rules, rules size, facts, objects and names).

## What it is

The steps are the instruction's, in its order, a stage each:

| | the step, in the instruction's words | stage | what is not said, and where it is asked |
|---|---|---|---|
| 1 | *"pick a prose text for like end of elementry school (3-5th grade) 50-100 sentences, very famous everybody would remember from school"* | [S1h.10.1](s1h.10.1_the_text.md) | which text, in which language, from whose school ([Q-M1h.4](../open_questions.md#q-m1h4--the-text-and-its-language)); where it and its programs live, and whether its text is reproduced ([Q-M1h.18](../open_questions.md#q-m1h18--where-the-text-its-facts-and-the-theories-live)) |
| 2 | *"Write 50 questions grouped into 5 groups by complexity from elementary (who did what) to very complex (idk maybe: feelings, motivation explanation, call of duty, moral dilemmas, what auther wanted to say, etc.)"* | [S1h.10.2](s1h.10.2_fifty_questions_in_five_groups.md) | what the five groups are, the upper ones being the user's guesses, marked *"idk maybe"*; whether each has ten questions ([Q-M1h.19](../open_questions.md#q-m1h19--the-five-groups-and-the-fifty-questions)) |
| 3 | *"And then formalize the text and questions"* | [S1h.10.3](s1h.10.3_the_text_and_the_questions_formalized.md) | — |
| 4 | *"and then write 5 theories (or one theory and 5 ontologies) that allow answering those groups of questions"* | [S1h.10.4](s1h.10.4_five_theories_or_one_theory_and_five_ontologies.md) | which of the two shapes ([Q-M1h.3](../open_questions.md#q-m1h3--five-theories-or-one-theory-and-five-ontologies)) |
| 5 | *"I expect theories complexity would grow (measuring complexity is itself a question, but let's now just count rules, rules size, facts, objects and names)"* | [S1h.10.5](s1h.10.5_how_the_theories_grow.md) | what the size of a rule counts ([Q-M1h.20](../open_questions.md#q-m1h20--what-rules-size-counts)) |

What it will show first, in the milestone README's words: *a question about a feeling or a motive needs facts
no sentence states, and the theory that supplies them is the measured thing.*

## What it is not

- **Not natural-language parsing.** The text's sentences are written as facts by hand, each with its sentence
  as `:source`. Producing those facts from the text is [M2](../../m2_nl_to_ir/README.md)'s, and every pair of
  a sentence and its facts is an instance of what M2 would have to produce.
- **Not a theory of literature, of feelings or of morals.** A theory here is the rules and facts that let one
  group's questions be answered about one text, and its size is the result. Whether it carries to a second
  text is noted where it is seen and not pursued.
- **Not a better measure of complexity.** *"let's now just count"*: the five counts are taken, and a better
  measure is written down as a question.
- **Not a benchmark run.** The fifty questions with expected answers are a small benchmark with graded
  difficulty, and the phase makes it; running a formalizer or a model against it is M2's.
- **Not new syntax, and not engine arithmetic.** Where a question wants something the language lacks, that is
  a finding, shown.

## What it starts from

- **The word problems' way of writing a text:** one fact a sentence where it can be, each with its sentence
  as `:source`, and a `:why` on every relation so that an answer reads as a sentence
  ([`examples/word_problems/README.md`](../../../examples/word_problems/README.md);
  [`09_natural_language.md`](../../../docs/kernel/ir/03-ein-lang/09_natural_language.md)).
- **A string holds any Unicode; a name is ASCII.** `(went Vanya forest :source "Ваня пошёл в лес.")` loads,
  answers, and is written back with its sentence by `ein saturate --dump` and by a `--dump-answer` state
  file (a scratch program of four forms on the release binary, 2026-11-13). So a text in another language
  than English can stand beside its facts, with its people and things under transliterated names.
- **A query states its expected answer** with `:expect`, and `ein test` holds it
  ([`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query)). A claim on a query
  closes the goal's relations over the class, so a claim lists exactly what the goal matches.
- **Two scripts that already read programs as text or count what reads as English:**
  [`utils/rule_census.py`](../../../utils/rule_census.py), M12's census of rule shapes, with an S-expression
  reader of its own, and `utils/nl_census.py`.
- **The glossary's *ontology layer*:** *types, instances, value domains, a-priori inter-type relations*
  ([`plans/README.md` § Glossary](../../README.md#glossary)). It is what *"one theory and 5 ontologies"*
  would vary while the rules stay.

## The stages

| ID | title | depends on | stops for the user |
|---|---|---|---|
| [S1h.10.1](s1h.10.1_the_text.md) | The text: a short list, with each candidate's length, language and whether it may be reproduced | nothing | **yes: the first stop.** **Done 2026-11-13 by the user's choice:** *Winnie-the-Pooh*, A. A. Milne, 1926, Chapter II (Q-M1h.4, Q-M1h.21). Q-M1h.18 carried to S1h.10.3 |
| [S1h.10.2](s1h.10.2_fifty_questions_in_five_groups.md) | Fifty questions in five groups by complexity, each with its expected answer in words | the text chosen | **yes: the second stop,** before any question is formalized. Q-M1h.19 |
| [S1h.10.3](s1h.10.3_the_text_and_the_questions_formalized.md) | The text and the questions formalized: sentences as facts by hand, questions as queries, the expected answers written before any theory | the questions accepted; Q-M1h.18's ruling | no; **its tasks are written at S1h.10.2** |
| [S1h.10.4](s1h.10.4_five_theories_or_one_theory_and_five_ontologies.md) | Five theories, or one theory and five ontologies: each group's questions answered | S1h.10.3 | **once,** inside it: the two shapes on the first two groups, shown, before the other three are written (Q-M1h.3). **Its tasks are written at S1h.10.2** |
| [S1h.10.5](s1h.10.5_how_the_theories_grow.md) | How the theories grow: the five counts, per group and cumulatively, and the close | S1h.10.4 | no; its table is **shown**. **Its tasks are written at S1h.10.2** |

No stage carries an estimate. Findings go to `findings.md` in this directory, opened at the first one
([M1h § Stage, phase, milestone](../README.md#stage-phase-milestone)).

## The order

The instruction's. S1h.10.1 and it stops; S1h.10.2 and it stops; then S1h.10.3, S1h.10.4 and S1h.10.5.

Against the milestone's other phases: the milestone's order puts this phase after P1h.1 to P1h.4, *by their
numbers*. Its first two stages write a list and questions in words and need nothing of them. From S1h.10.3
on:

- **P1h.1.** *Who did what, when, and why* wants a timeline and causes. If the two modules have shipped, the
  text's facts use them, and the story is the first large program that is not a word problem to do so. If
  not, moments are names, as in the banked word problems, and a stated *because* is a fact of the text; the
  stage says which, and the counts of S1h.10.5 say what a module would have saved.
- **P1h.2.** What of the text a question's answer used is its slice. If no surface exists,
  [S1h.2.1](../p1h.2_slicing/s1h.2.1_what_a_slice_is_today.md)'s script gives it; if that has not run
  either, the column is left out and the page says so.
- **P1h.3.** A story leaves almost everything unsaid. What a theory supplies that no sentence states is
  written as that phase's page has an assumption written, if it has ruled; as a fact with its own `:source`,
  marked as the theory's and not the text's, if not.

Where M1h sits against M1g is [Q-M1h.1](../open_questions.md#q-m1h1--where-m1h-sits-against-m1g).

## Acceptance

From the milestone's: *the text, the 50 questions in 5 groups, their formalization, the theories, and the
table of the five counts.* In checkable terms:

- **The user chose the text** from a list that states each candidate's length, language and whether it may
  be reproduced; the choice is recorded in the user's words.
- **The user accepted the five groups and the fifty questions** before any was formalized; each question has
  an expected answer written before any theory existed, and the commit that holds the expected answers is
  earlier than the first commit of a theory.
- **Every sentence of the text is one or more facts with that sentence as `:source`,** or is listed as
  carrying none, with the reason.
- **Each question is a query,** and for each group the page says how many of its questions the group's theory
  answers as expected, held by `ein test`; a question that is not answered is listed with what it would
  need.
- **The table of the five counts exists,** per group and cumulatively, written by a script, with each
  count's definition on the page.
- **The repo's discipline:** every `.ein` file that is tracked has a corpus entry, a `:why` on every relation
  and rule, and an `:expect` where it states a claim; a wrong answer of the engine is a row of
  [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), filed `open` before its fix; every other
  finding is a row of this phase's `findings.md`, shown.

## Risks, each with what checks it

- **No text meets every word of the instruction.** *Prose*, *50-100 sentences*, *very famous*, *3-5th
  grade*, and able to carry questions up to *moral dilemmas* and *what the author wanted to say*. S1h.10.1's
  list says, for each candidate, which of these it meets and which it strains, and does not hide a miss.
- **The text may not be reproducible in the repository.** Then the facts are written against a numbered
  edition and the repository holds sentence numbers and short quotations. S1h.10.1 states it for each
  candidate, with the source of the statement.
- **The upper groups' expected answers are opinions.** *What the author wanted to say* has no answer a run
  can check against the text. S1h.10.2 writes each expected answer with what it rests on, and a question
  whose answer rests on nothing in the text is marked so before the user reads the list.
- **A theory is fitted to its questions.** A theory that answers exactly the questions asked and nothing
  else measures the questions. One check is to hold a few questions of each group back from whoever writes
  the theory and report how many of those it then answers. The instruction does not ask for it: S1h.10.2
  puts it to the user at its stop, and nothing is held back unless the user says so.
- **The counts measure style.** Two writers produce different rule counts for one theory. The script counts
  what it is given; S1h.10.5 says what it does not control for and counts both shapes of Q-M1h.3 where both
  were written.
- **A story's program is large.** Fifty to a hundred sentences, each a fact or several, under theories that
  join on them. Every theory's programs state their lattice, every run carries `-T`, and a program in which
  a hypothesis leads far is run alone before a gate.

## Connections

- **[M2](../../m2_nl_to_ir/README.md):** each sentence with its hand-written facts is a pair M2 would have to
  produce, and the fifty questions with expected answers are a small benchmark with graded difficulty.
- **[P1h.1](../p1h.1_temporal_and_causal_theories/README.md), [P1h.2](../p1h.2_slicing/README.md),
  [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md):** the timeline and the causes, the slice, the
  gaps and assumptions.
- **[M12](../../m12_rules_as_data/README.md):** its census of rule shapes is a count of rules by another
  measure; S1h.10.5 reads its script.
- **[F26](../../followups/f26_consistency_and_completeness.md):** its senses of *complete*, and its table of
  common sense as benchmark classes, which the upper groups of questions meet.
- **Ideas:** [03, three task classes](../../ideas/03-three-task-classes.md), since a question the text does
  not settle is a gap; [09, puzzles beyond Zebra](../../ideas/09-puzzles-beyond-zebra.md).
- **[`open_questions.md`](../open_questions.md):** Q-M1h.3, .4, .18, .19, .20.
