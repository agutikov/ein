# E-112 — A program's forms may be written in the order they are made

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** —; whether Ein states and holds it is [Q-M2.24](../../open_questions/kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md)

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
of each order's contradictory twin. What moves is what [`01-determinism-and-order.md` § 1.3](../../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#13-what-varies-and-why-it-may)
allows: the trace and the event stream of the four literal encodings (`zebra3.1`: 5 954 of 87 768 trace lines and
3 510 of 18 719 event lines differ between the two orders), the order the rules fire in; `zebra` and `zebra2`'s
traces do not move.

## Evidence

[`../zebra_formulations/probes/order-cases/`](../../zebra_formulations/probes/order-cases/): `run.sh`, six cases,
each `read-out same`. [`../zebra_formulations/probes/order/`](../../zebra_formulations/probes/order/), written by
`measure.py order` (the forms unchanged, reordered by `forms.py`), compared in `results/order-compare.json` (every
question, through `einaf feedback` and `ein solve --json-summary`) and `results/order-texts.json` (`measure.py
order-texts`: the read-outs, the traces and the event streams by `diff`, the path taken off; the event streams'
first line names the run's own output paths and differs in every pair).

## What it costs the application

Nothing; it is what lets the steps' outputs be put one after another as they are made
([the phase's design questions](../../README.md#five-design-questions-each-with-its-default)). It rests on the
loader's structure and this measurement: no page states it and no test of Ein holds it (Q-M2.24).
