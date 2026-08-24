# S1d.2.1 — What each property enforces today, rule by rule

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 3 days
**Depends on:** [S1c.1.1](../../../docs/history/m1c_external_validation/README.md#s1c11--what-the-stdlib-promises-and-what-is-exercised)'s
promise inventory and the [layer census](../p1d.10_exhaustive_search/layer_census.md) — both taken.

## Context

An audit, not a design. The note's premise — "only half of each property is
stated" — is *nearly* right, and every later stage's shape depends on exactly
how nearly. [`obligation_forms.md` §8](obligation_forms.md#8-what-each-form-does-to-the-stdlib)
already claims the map (six scans, two endpoints each); under the decided form
(G — additive, the scans stay) the audit's job is not to justify a collapse
but to name **what the two new duals must not disturb**.

Four rule families own the territory:
[`std.algebra`](../../../stdlib/algebra.ein) (the checks: `functional` /
`injective` at 250, `total` / `surjective` at 110, the property checks at
110), [`std.bijection`](../../../stdlib/bijection.ein) (negative-completion at
240, eliminations at 400, the `bijective-setup` fan-out at 100),
[`std.elim`](../../../stdlib/elim.ein) (the positional variant), and
[`std.slots`](../../../stdlib/slots.ein) — the one §8 never mentioned, whose
`slot-fill` / `slot-elimination` / `slot-no-room` / `slot-no-fill` are the
same endpoints for the co-location formulation and will want the same duals.

## Tasks

### Task T1d.2.1.1 — the table

Per stdlib rule: which property, which half (`≤` / `≥`), which form (verdict
`(false)` / stored `(not …)` / forced positive), which priority band, and
which `tests/stdlib/` program activates it. The census data exists
(`utils/stdlib_census.py`, 73 of 73 firing); the audit adds the *property*
column the census does not have. Banked as `property_audit.md` beside this
file, the way P1d.10 banked its census.

### Task T1d.2.1.2 — the endpoints, verified

§8's claim per family: candidates = 0 ⇒ `(false)`, = 1 ⇒ forced. Confirm each
against the rule text, including the slots family, and list any property whose
endpoint is *missing* (a `≥` with no unreachability scan is a hole the duals
would paper over silently).

### Task T1d.2.1.3 — the disturbance list

The duals land at the probe band (500). List every rule whose behaviour
depends on what has fired *by* 500 — negative-completion (240), violations
(250), eliminations (400) — and state why a read-only tally at 500 cannot
change any of them. That argument is what makes S1d.2.4's "every existing
verdict unchanged" bullet checkable rather than hoped.

### Task T1d.2.1.4 — the incomplete-candidates set

From the census runs: which corpus entries activate a totality scan at all,
and which end quiescence with the scan *unfired* (no `(false)`, obligations
presumably outstanding). That list is S1d.2.6's census sketched three stages
early, and it is the first look at which of
[Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)'s
ten entries owe anything.

## Acceptance

- `property_audit.md` checked in: every one of the 73 stdlib rules classified
  by property, half, form, band, and activating program.
- §8's endpoint table confirmed or corrected, slots included.
- The disturbance argument written — per band, why a 500-band read cannot
  move it.
- Any missing endpoint named as a finding, not silently filled.
