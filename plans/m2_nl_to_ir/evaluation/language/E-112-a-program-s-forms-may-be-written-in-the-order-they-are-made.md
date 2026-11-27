# E-112 — A program's forms may be written in the order they are made

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** —; **Ein states it**, by [Q-M2.24's decision](../../../../docs/decisions/language/Q-M2.24-an-answer-does-not-depend-on-the-order.md) ([S2.2e.4](../../p2.2e_kernel_fixes/s2.2e.4_order_and_table_rows.md), `cc393635`), held by no test, as ruled
**Since P2.2's close (2026-11-27):** — ; whether Ein holds it is [Q-M2.24](../../open_questions/kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md), [S2.2e.4](../../p2.2e_kernel_fixes/s2.2e.4_order_and_table_rows.md); since S2.2e.4 a strength Ein states, [the decision](../../../../docs/decisions/language/Q-M2.24-an-answer-does-not-depend-on-the-order.md)

## What

**A program's forms may be written in the order they are made.** Ein reads a file whole before it loads any form:
a fact before its relation's declaration, a short name before the import that lists it, an activating fact before
its rule, a macro's call before the macro, a query before what it names and a `(solve …)` list before the relations
it names all load, and each prints the read-out its usual order prints. The six Zebra encodings, each written in
synthesis order (the statements and the questions, the ontology, the theory) and in theory-first order, answer
alike under every question (`describe`, `--single`, `--maximal`, `--minimal`, both lines' `--single`, `--count`,
`--no-search`): the same outcome under each reading, readings, classes and rows of the answer query, answer
sentence, states, H (`ein show hyps`, line for line) and `work` counts; the read-out of `ein solve` and of `ein
solve --single` is the same byte for byte, the file's path taken off, and so is the read-out, the core among it,
of each order's contradictory twin. What moves is what [`01-determinism-and-order.md` § 1.3](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#13-what-varies-and-why-it-may)
allows: the trace and the event stream of the four literal encodings (`zebra3.1`: 5 954 of 87 768 trace lines and
3 510 of 18 719 event lines differ between the two orders), the order the rules fire in; `zebra` and `zebra2`'s
traces do not move.

## Evidence

[`../zebra_formulations/probes/order-cases/`](../../p2.2_formalizer/zebra_formulations/probes/order-cases/): `run.sh`, six cases,
each `read-out same`. [`../zebra_formulations/probes/order/`](../../p2.2_formalizer/zebra_formulations/probes/order/), written by
`measure.py order` (the forms unchanged, reordered by `forms.py`), compared in `results/order-compare.json` (every
question, through `einaf feedback` and `ein solve --json-summary`) and `results/order-texts.json` (`measure.py
order-texts`: the read-outs, the traces and the event streams by `diff`, the path taken off; the event streams'
first line names the run's own output paths and differs in every pair).

## What it costs the application

Nothing; it is what lets the steps' outputs be put one after another as they are made
([the phase's design questions](../../p2.2_formalizer/README.md#five-design-questions-each-with-its-default)). It rests on the
loader's structure and this measurement, and since S2.2e.4 on a statement of Ein's: the decision below. No test of
Ein holds it, as ruled.

## Stated by Ein

By S2.2e.4, `cc393635`, 2026-11-27, on the user's ruling of
[Q-M2.24](../../open_questions/kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md):
[the decision](../../../../docs/decisions/language/Q-M2.24-an-answer-does-not-depend-on-the-order.md) says that two programs that differ only in the order of their forms, in the main file or in a
module either imports, have one answer (R, its states and counts, root's standing and what root owes, every
question's outcome, every query's classes and rows over all of R, every claim's verdict, and H), that a run that
answers them differently is a wrong answer, and why, in the user's two reasons; [`01-determinism-and-order.md`
§ 1.2](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#12-what-it-may-not-depend-on) has the row,
*stated, and held by no test*. **What it says may move** is what this entry's measurement saw move, the trace, the
event stream and the order rules fire in, with a core; and what it did not exercise, each seen moving on the binary
at `cc393635` with every other question alike: root's owed instances listed in the written order, the member of R
`-n 1` meets first where two owed instances' alternatives interact (two people, two foods, one food each: `(likes
Ann Soup) (likes Carl Stew)` in one order, `(likes Ann Stew) (likes Carl Soup)` in the other), the `:source` a fact
stated twice keeps, and the `(config …)` a run takes of several, which changes no answer.

**Where it falls short today**, which the decision says: a saturation's boundary admits the rules waiting on an
`absent` in their written order within each half of the agenda, so where a rule's `absent` reads what another such
rule concludes, the one written first can decide the fixpoint. It is the register's
[`W8/fifo-gap-root`](../../../wrong_answers/register.toml) (owner S1n.2.1): two rules and two given facts, `ein
solve` answering the query `(p ?x)` with `?x = k` when the reader is written first and with no row when the producer
is, on a program the static check calls monotone. By the decision that is a wrong answer, known and not fixed; of a
program the check flags, nothing is claimed ([E-31](../answer/E-31-of-a-flagged-program-nothing-is-claimed.md)).
**For the application**: the steps may assemble a program in the order they make it, as the six encodings did; a
program in which one rule's `absent` reads what another rule waiting on an `absent` concludes is where another order
could answer otherwise, and such a run is filed as a wrong answer, not answered by reordering
([L-M2.32](../../findings/wrong-answers/L-M2.32-the-guarantee-of-q-m2-24-has-a-known-wrong-answer-against-it.md)).
