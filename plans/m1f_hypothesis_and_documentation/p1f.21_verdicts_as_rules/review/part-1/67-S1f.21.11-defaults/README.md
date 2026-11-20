# 67 · The defaults S1f.21.11 took

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.11_kill_and_three_conventions.md#record)

S1f.21.11 is built: the contradiction is a theory a program imports, the kernel's own check of a fact beside its
negation is gone, the argument-count check is gone, and a relation may be declared more than once. **No corpus
program's answer moved.** You said to run the phase on defaults and review the questions after. These are the ten
it took. Each is built; a word from you changes it.

## What you should know first

- **36 corpus programs gained `(import std.contradiction)`**, and three modules of the library import it:
  `std.algebra`, `std.bijection` and `std.slots`. So a program that imports one of those three has the
  contradiction without asking. With the kernel's check off and no import, 47 of the 931 corpus programs moved, 36
  of them in their answer; the stage's record lists each.
- **`not` is now a name everywhere but at a form's head**, so the library writes `(clash-1 not)` and a program may
  write `(relation not Claim)`.
- **A form at the wrong count loads in silence**, as you ruled: a request to the calculator such as
  `(add (n z 5))` gets no answer and no warning.
- **The cost of the change is not measured yet.** I owe it, on a quiet machine
  ([V-39](../../../findings.md#v-39--the-cost-of-the-contradiction-as-rules-is-not-measured)).

## The ten

| # | question | the default taken | the other reading |
|---|---|---|---|
| 1 | [Q-M1f.585](../../../../open_questions.md#q-m1f585--which-programs-and-modules-gain-the-contradictions-import) | Every corpus program whose answer rests on a fact beside its stored negation gained `(import std.contradiction)`, 36 files, so that no answer moves; and the library's modules that conclude a stored negation and mean a death by it, `std.algebra`, `std.bijection` and `std.slots`, import it, so a program that imports one of those has the contradiction. | a program left without it where it reads better as a theory with no contradiction; none was left so. |
| 2 | [Q-M1f.586](../../../../open_questions.md#q-m1f586--an-explanation-reads-through-the-contradictions-rules) | A dead state's core and its `refuted by` read through `std.core`'s rule to the `(false)` it matched, and through the module's rule to the pair, so they name the program's facts and rules as before. | the module's rules named in an explanation. |
| 3 | [Q-M1f.587](../../../../open_questions.md#q-m1f587--a-rule-that-refutes-a-pair-fires-at-once) | A rule that concludes `(false)` from a fact beside a fact that holds it as its one argument fires in the band of a rule that states the engine's word, so a state ends at the firing that completes the pair, whatever was queued. Which rules those are is read off the compiled plans, by no name. | such a rule waits its turn in the agenda, and a state can derive more before it ends. |
| 4 | [Q-M1f.588](../../../../open_questions.md#q-m1f588--what-a-denial-is-once-the-pair-is-a-rules) | A candidate that root holds the negation of is out of $H$, and is counted as denied by licence L8 and by the structure, only where a compiled rule refutes the candidate beside that negation. Where no rule refutes the pair, root denies nothing. | `(not F)` at root denies F whatever the program imports. |
| 5 | [Q-M1f.589](../../../../open_questions.md#q-m1f589--the-static-check-reads-not-as-any-relation) | The static check's relations are names with no *negated* side; a conclusion that is the holding side of a pair the rules refute is its own kind of site. No corpus verdict moved; two flagged programs' labels read `not` where they read `(not r1)`. | the check keeps a negated side for each relation. |
| 6 | [Q-M1f.590](../../../../open_questions.md#q-m1f590--a-hypothesis-rule-is-matched-as-it-is-reached) | The look-ahead over a premise that reads `not`, which let a hypothesis rule see a negation another candidate was about to bring, is gone: no caller writes between two candidates. | the look-ahead kept, read off the rules. |
| 7 | [Q-M1f.591](../../../../open_questions.md#q-m1f591--the-words-of-the-surfaces-stay) | *holds a contradiction*, `:expect (false)`, the events' `kind: dead` and the row `refuted by` are as they were. | words that name the rule that ended the state. |
| 8 | [Q-M1f.592](../../../../open_questions.md#q-m1f592--the-registers-rows-the-change-answers) | The two rows of case W36 answer right now and are regressions, with their witnesses; W10's recorded core is today's, another explanation of the same answer. | none |
| 9 | [Q-M1f.593](../../../../open_questions.md#q-m1f593--not-is-a-name-everywhere-but-at-a-forms-head) | `not` is read as a name wherever a name goes, and as the negation only at the head of a form, so `(clash-1 not)` and `(relation not Claim)` are written bare. A relation, a rule, a hypothesis rule and an alias may be named `false` or `not`; a macro may not, since its calls would be every `(false)` and every `(not X)`. | `not` stays reserved and the library writes it quoted, which drew it as two nodes in a knowledge base's drawing. |
| 10 | [Q-M1f.594](../../../../open_questions.md#q-m1f594--the-witnesses-of-the-count-check-and-a-relation-declared-at-several-counts) | The seven `arity-*` programs load, and their regression rows hold the loaded answer; the fixture of a duplicated declaration is gone and `examples/features/22_declared_twice.ein` shows one. A relation declared at several counts is read at each by every reader of a declaration; the read-out's scope note names it once. | the seven rows removed with the refusal. |

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 67. Blank keeps the ten as built.
