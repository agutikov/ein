# Q-M2.19 — Do `ein-summary/2` and `ein-test-report/6` keep their numbers?

**Status:** open

**Raised 2026-11-25 by [S2.1.2](../../s2.1.2_diagnostics_surface.md#t2124-to-t2128--the-surfaces-the-tests-einaf-the-evaluation),
its default 7 of the surfaces**, filed as
[E-92](../../evaluation/diagnostics/E-92-a-schemas-number-is-written-into-an-options-help.md), and the third of the
user's choices in [the phase report's § 7](../../report/README.md#7-the-defaults-taken-for-the-user-to-correct).

**The question.** S2.1.2 added keys to both of Ein's JSON surfaces: the summary's `diagnostics`, written last on every
summary, and `region.core_sources`; the report's `diagnostics` on every row. Does either schema's number move?

**The default taken**, in S2.1.2's record: *"`ein-summary/2` stays. `08-read-out.md` § 11 defines the number by the
summary's one shape, *every key on every run and `null` where it does not apply*, and a key on every run keeps that
shape; the summary took keys under `/2` before (`root.dead`, `root.owes`, `root.owed`, `root.commit`,
`root.hypgen.outside`, `region.stopped.steps`). The report stays `ein-test-report/6` by the same reading, though its
earlier keys each moved its number; both numbers are in an option's help, and `help_shape.txt` may not move (E-92)."*
Ein's decision [Q-M2.9](../../../../../docs/decisions/diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md)
states it: *"The keys are added under `ein-summary/2` and `ein-test-report/6"*.

**What the default costs** (E-92): *"a reader that keys on the number cannot tell the report before the key from
after it"*; the report's number has moved for each of its earlier keys (`/4`, `/5`, `/6`) and did not for this one.

**The alternatives.**

1. **Both kept** (the default), by § 11's reading that the number names the shape's rule, not its keys.
2. **Both moved**, `ein-summary/3` and `ein-test-report/7`: the four option helps of `help_shape.txt` that name them,
   `08-read-out.md` § 11 and § 12, the decision's sentence, and every reader that checks the number move with them.
3. **The report's number moved and the summary's kept**, the report keeping its own history of a number a key.
4. **Either, with the number taken out of the options' help**, so that a schema that moves no longer moves
   `help_shape.txt` (E-92's other half).

**Where the answer lands**: it is about Ein, so a decision under `docs/decisions/`, written with the change it asks
for, or Q-M2.9's sentence where the default stands.
