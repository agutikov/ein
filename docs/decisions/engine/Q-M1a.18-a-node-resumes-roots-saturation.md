# Q-M1a.18 — A node resumes root's saturation

A node's saturation is root's, continued: it starts from root's snapshot at quiescence, with what root has fired
and what it has parked, seeded with the commitment's facts alone, and does not derive root's fixpoint again. Its
derivations are what the commitment adds, and that is what its narration tells; a trace renders root's derivation
as its own section, before the states. The fixpoint is the one a fresh saturation of the grown KB reaches, and so is
every answer and every count.

What may differ from a fresh saturation is the firings a node counts, and which of a fact's equally valid
derivations its record keeps first. Narration is defined once, in the dev-only crate `ein-narration`, as a
fork's derivation and anything keyed on a dying fork's stopping point; `EIN_NARRATION_STRICT=1` compares it byte for
byte as well.

**Why.** A node that re-derived root's whole closure would narrate, before its commitment's first consequence, what
was true without it; a trace that starts at the hypothesis is the one a human walkthrough gives. Nothing promises
which derivation of a fact derived several ways is recorded first.

**Held by.** `ein_infer::saturator::Saturator::resume`, and the module's unit test
`a_resumed_saturation_reaches_the_same_fixpoint`; `ein-render/tests/idea08_acceptance.rs`:
`the_zebra2_trace_exhibits_the_walkthrough_rules`; `ein-infer/tests/event_cut_control.rs`:
`the_cut_reports_a_dropped_derivation_and_ignores_dropped_narration`. A derivation recorded first is part of an
explanation: [Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md).
