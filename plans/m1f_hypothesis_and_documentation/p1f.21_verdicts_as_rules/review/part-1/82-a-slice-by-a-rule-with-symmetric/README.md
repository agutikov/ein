# 82 · A slice by a rule, activated with `symmetric`, loads now

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-34--the-milestones-l-109-program-loads)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it.

## What it is

The milestone's finding L-109, left as your call: your slice by a rule, `about-1`, whose parameter names the
relation of the facts it reads, appended to `examples/zebra2.ein` with `(about-1 symmetric)`, was refused at load,
because a rule of `std.algebra` *could* derive an activator binding a relation's parameter to a fact. That
refusal is the warning now (S1f.21.17): the program loads under one warning that names `std.algebra.imply1`, and
root derives `(in-slice-1 symmetric next-to)`. Whether a slice is had by such a rule was the question, and still
is.

## The question

- **(a)** It is: the program loads and answers under its warning, and L-109 is closed by that. **Default.**
- **(b)** It is not what a slice should be: say what you want of it.

## On the built engine

The programs and the runs on the final engine are added by
[S1f.21.19](../../../s1f.21.19_the_review_items_on_the_built_engine.md).
