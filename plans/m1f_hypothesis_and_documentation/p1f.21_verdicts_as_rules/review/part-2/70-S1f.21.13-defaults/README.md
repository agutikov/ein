# 70 · The defaults S1f.21.13 took

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.13_the_search_and_rules.md#record)

S1f.21.13 is done: the lattice written as rules is in the corpus as a pair of programs,
`examples/lattice/18_two_candidates.ein`, two candidates searched by the engine, and
`examples/lattice/18_two_candidates_as_rules.ein`, the same lattice as rules in one saturation. Each verdict of the
oracle on the first is a fact of the second: `(owes (w))` for ∅, `(in-R (w (s a)))` and `(in-R (w (s b)))` for the
two states of $R$, `(in (w (s a) (s b)) (false))` for the dead one. Both are monotone and held by `ein test`.

## What you should know first

- **The example is not written with `open`** (default 2). In the language's form it reads `open` facts, so it is
  flagged and no suite may hold it; and that form met two defects of the build, which S1f.21.20 fixes
  ([V-61](../../../findings.md#v-61--a-written-open-fact-breaks-the-answer-dumps-round-trip),
  [V-62](../../../findings.md#v-62--a-note-of-the-mechanisms-precedes-a-flagged-programs-warning)). Both `open`
  forms are kept beside the stage's record.
- **What was declined stays declined**, each reason checked against the engine as it is: search control from
  inside Ein, the search as rules in the engine, and facts inferred from the search. One reason moved: a rule can
  read `open` now, at the price of the flag. The stage's record has each.
- **V-8 still holds**: the survey's whole program, with its last rule, answers wrong on a program the static check
  calls monotone. It is the non-monotone milestone's.

## The three

| # | question | the default taken | the other reading |
|---|---|---|---|
| 1 | [Q-M1f.631](../../../../open_questions.md#q-m1f631--where-the-example-of-the-lattice-as-rules-goes-and-its-name) | The example is a pair under `examples/lattice/`, at the next number, sharing it: `18_two_candidates.ein`, two candidates searched by the engine, and `18_two_candidates_as_rules.ein`, the same lattice written as rules in one saturation. | one file, the program as rules alone. |
| 2 | [Q-M1f.632](../../../../open_questions.md#q-m1f632--how-the-program-as-rules-writes-its-candidates) | As the survey's page wrote them, as facts of its own, `(question F)`, its header saying they are what the object program's root holds as `(open F)`. So the program is monotone and `ein test` holds its claims. Written with `(open F)`, the language's form, it is flagged for reading `open`, no suite may hold it, and it met two defects of the build (V-61, V-62); both `open` forms are kept beside the stage's record as witnesses. | the example in the language's form, flagged, with no claim held, once the two defects are fixed. |
| 3 | [Q-M1f.633](../../../../open_questions.md#q-m1f633--what-the-program-as-rules-states-beside-the-surveys) | A third verdict, `owes`, a world that is alive and pays nothing, so that each of the oracle's three verdicts is a fact of the program; every relation declared with its `:why`; a query for it with its claim. | the survey's program as it stood, two verdicts. |

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 70. Blank keeps the three as built.
