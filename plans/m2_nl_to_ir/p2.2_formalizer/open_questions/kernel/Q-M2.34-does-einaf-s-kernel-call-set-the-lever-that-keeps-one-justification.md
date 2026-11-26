# Q-M2.34 — Does EinAf's kernel call set the lever that keeps one justification a fact?

**Status:** open

**Raised 2026-11-26 by [S2.2.5](../../s2.2.5_from_text.md)**, from
[E-126](../../evaluation/bounds/E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md).

**The question.** The model's Zebra program, dead at root, costs every kernel call 72 seconds that its clock does not
stop, and 63 ms with `(config :record-alternative-justifications false)`. The lever changes which core a dead node
reports and no state ([Q-M1e2.30](../../../../../docs/decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md));
a program may not hold a `(config …)` ([the contract § 1.4](../../../../../docs/einaf/04-contract.md#14-what-a-program-may-not-hold)).
Does EinAf's bounded call set it on every program, as a bound of its own, so that a model's program whose root dies
on a link several rules derive is judged in milliseconds and its core is one of the smallest rather than the
smallest?

**The default taken: no.** The bounded call loads a program as `ein solve` loads it, and the core the feedback object
carries is the one `ein solve` reports; a run that costs past the clock is recorded as such, and the cost is the
record's (E-126). A change here is a change of what the kernel call is, which the user rules.

**The alternatives.** The call sets the lever always (the core may then differ from `ein solve`'s, and the corpus
test that holds the object to the binary's summary would compare under the same lever); or only where the clock has
been passed once on the same program, a second call with the lever, recorded as such; or a cap of EinAf's on the
justifications recorded, which the kernel does not have.

**What bears on it.** The loop's budget of kernel time ([P2.4](../../../p2.4_loop/README.md)); the core is what the
feedback ladder shows ([S2.4.3](../../../p2.4_loop/s2.4.3_feedback_ladder.md)), and E-125 is a core the model's
misreading is named by, which the lever must keep naming.
