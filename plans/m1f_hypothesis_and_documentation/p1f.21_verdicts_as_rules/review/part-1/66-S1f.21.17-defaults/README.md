# 66 · The defaults S1f.21.17 took

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.17_the_loader_warns_and_std_core_by_import.md#record)

S1f.21.17 is built (`24491e80`): the loader refuses nothing for what a rule could derive, `(import std.core)` is
an import like any other, and `$EIN_STDLIB` may replace `std.core`. You said to run the phase on defaults and
review the questions after. These are the five it took. Each is built; a word from you changes it.

## What you should know first

- **Four of the five programs the refusal stopped are flagged as not monotone once they load**
  (`examples/features/19_derived_activator.ein`, `20_derived_activator_never.ein`, and under
  `examples/ein-bugs/wrong-answers/`, `derived-activator.ein` and `derived-activator-never.ein`): a commitment
  derives an activator root has no plan of. Each answers 64 states under the `WARNING`, and nothing is claimed of
  it.
- **The fifth, `nested-fact-activator.ein`, is monotone**, and was edited (default 3 below).

## The five

| # | question | the default taken | the other reading |
|---|---|---|---|
| 1 | [Q-M1f.580](../../../../open_questions.md#q-m1f580--the-warning-for-an-activator-a-rule-could-derive-its-words-and-when-it-is-printed): the warning's words, and when it is printed | each line the refusal printed, ending *…, so such an activator activates nothing*; a second line where a state holds such an activator. All `warning:` lines are printed when the command's run ends, after a flagged program's `WARNING` | each line where it arises, the load's before the run |
| 2 | [Q-M1f.581](../../../../open_questions.md#q-m1f581--the-regression-rows-of-the-two-derived-activator-witnesses-now-flagged): the two witnesses' regression rows | removed: the programs are flagged, and no row holds a flagged program's answer. The witnesses stay in the corpus | none that a suite may hold |
| 3 | [Q-M1f.582](../../../../open_questions.md#q-m1f582--nested-fact-activatorein-states-solve-no-search): `nested-fact-activator.ein` | it gained `(solve :no-search)`. Loaded as written it has a blind cube of 16 candidates, 65 536 states, in every sweep; its question is about root. Its $R$ is now {∅} | left as written, and marked slow in the corpus |
| 4 | [Q-M1f.583](../../../../open_questions.md#q-m1f583--where-the-fixtures-that-load-now-are-kept): where the fixtures that load now are kept | moved to `examples/features/19_…`, `20_…` and `21_import_of_core.ein`, unchanged but for their headers | deleted |
| 5 | [Q-M1f.584](../../../../open_questions.md#q-m1f584--how-stdcores-rule-prints-under-an-import-that-renames-it): `std.core`'s rule under `:as` or `:symbols` | it prints as the import spells it, as any module's rule does; `refuted by` reads through it as before | always the full name |

## Also found, each yours to call

- **Seventeen warning lines for one program** that imports `std.algebra`, one a site
  ([V-32](../../../findings.md#v-32--the-loads-warning-is-one-line-a-site)): gathered into one a module, or left.
- **The milestone's L-109 program loads now**, under one warning
  ([V-34](../../../findings.md#v-34--the-milestones-l-109-program-loads)).

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 66. Blank keeps the five as built.
