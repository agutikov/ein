# 79 · A program with no candidate is never flagged

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-22--a-program-with-no-candidate-is-never-flagged)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it: the row is in the register either way until you say.

## What it is

`W8/fifo-gap-root`, a witness of the wrong-answer register, states `:no-search` and has no candidate. The static
check looks for an `absent` a commitment can change, and with no commitment finds no site: the read-out says the
lattice is monotone and prints no warning. But the defect is at root: two rules that wait on an `absent` are
admitted in written order, and root holds a fact its own fixpoint's premises deny. So your ruling, *a
non-monotone program loads under a warning and nothing is claimed of it*, does not reach this program: the engine
claims its answer, and the answer is wrong. S1f.21.15 removed the row with the register's other rows of
non-monotone programs; the coordinator put it back.

## The question

- **(a)** It stays a row of the register: a wrong answer of a program the engine claims, the non-monotone milestone's to fix. **Default.**
- **(b)** It goes: the program is counted non-monotone, and what is owed is a detection that warns of it, that milestone's too.

## On the built engine

The programs and the runs on the final engine are added by
[S1f.21.19](../../../s1f.21.19_the_review_items_on_the_built_engine.md).
