# E-118 — A numeral written with two digits in one place makes root dead, and the outcome reads as a contradiction

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.2, 2026-11-26, on `35fcc5af`
**Owner:** none
**Since P2.2's close (2026-11-27):** none; [S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md) reads the rule that refuted, and [S2.5.3](../../p2.5_harness/s2.5.3_metrics.md) which text such a root prints

## What

**A model writes twelve as `(n z 12)`**: one place, two digits. The excerpt it was given says *`(n z 5)` is 5,
`(n (n z 1) 2)` is 12*, and its examples write `(n (n z 1) 2)`; the probe's `library.off.more` wrote
`(has Alice (n z 12) Apple Start :source "Alice has 12 apples.")`. `12` is a name and no digit, so
`std.math.decimal` labels the numeral `not-decimal`, and its rule `not-decimal-is-false` concludes `(false)`: root is
dead and R empty. **The kernel's own account is exact**: a description's core is the one fact, its sentence *"Alice
has 12 apples."*, and `refuted by std.math.decimal.not-decimal-is-false`. **The outcome is `inconsistent`**, the
word every reader of the outcome takes for *the statements contradict each other*, and `--single`, the question a
loop asks first, carries no core: what was asked shows a contradiction, not a malformed number.

## Evidence

The probe's record, `plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/probe.jsonl.gz`, the line `library.off.more`,
its fenced reading: `--single` *inconsistent*; `ein solve` of that reading, the core and `refuted by` above. The
probe's report counts every program whose numerals are written so.

## What it costs the application

A repair that reads the outcome rewrites the statements, when one numeral is to be written again; it has to ask a
description for the core to see why, and to read a rule's name of the calculator's (`not-decimal-is-false`) as
*malformed*. A malformed numeral is the representation's fault (`(malformed …)`, `(not-decimal …)`), and the kernel
does not set it apart from a contradiction of the statements' own: both are a dead root.
