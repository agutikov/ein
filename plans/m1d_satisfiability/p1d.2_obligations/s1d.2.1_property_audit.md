# S1d.2.1 — What each property enforces today, rule by rule

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 3 days
**Depends on:** [S1c.1.1](../../../docs/history/m1c_external_validation/README.md#s1c11--what-the-stdlib-promises-and-what-is-exercised)'s
promise inventory and the [layer census](../p1d.10_exhaustive_search/layer_census.md) — both taken.
**Status: done 2026-08-25** — banked as
[`property_audit.md`](property_audit.md), 73 rules classified from the parsed
rule text, joined to two census runs (`tests/` and all 180 entries) and four
fixtures. What it found:

- **The `≥` half has fifteen rules and no middle.** Five refute at zero
  candidates, five force at one, four generate a witness that is unique by
  construction, one refutes open-world-naively — and **none** records
  anything at two or more. The phase's premise, as a number over all 73.
- **F1** — `std.elim`'s positional markers `(functional R 0 1)` / `(total R
  0)` are premises nothing checks: a KB violating functionality solves, and
  `domain-elimination` forces arrows on that unverified premise. Named, not
  filled; the duals avoid the positional spelling.
- **F2** — `connex` is the one `≥` refutation written in the open-world-naive
  form: it fires `(false)` on an empty state where `total` reports
  `Solution`. Documented as opt-in in the module header; S1d.2.2's refutation
  contract inherits the caveat rather than papering over it.
- **F3** — `std.elim` has no range side, and the bijective pair's two
  endpoints live in *different modules* joined by one fan-out. The duals
  therefore belong beside the 0-endpoints in `std.algebra`, plus `std.slots`.
- **F4, and it moves a downstream acceptance** — an obligation rule cannot be
  parameter-less, because a variable relation head must come from the
  activator (`CompileError: unbound relation head ?R in a premise`). So the
  duals need activator facts, so the fan-outs grow, so **13 entries gain 50
  stored facts** and S1d.2.4's bit-identity claim is restated: saturation
  firings unchanged, fact store +2 per declaration and diffed as a predicted
  number, goldens re-blessed once.

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

**Reframed 2026-08-25**: the duals do not land in a band at all — they run
after the fixpoint, in a pass of their own
([S1d.2.3](s1d.2.3_the_form.md) item 1, *When they run*), so the question is
no longer "what does a 500-band read disturb" but "what could reach the
saturation loop from outside it". The list is shorter and the argument is
stronger, and both are still owed:

- every rule whose behaviour depends on what has fired by the time it runs —
  negative-completion (240), the violations (250), the eliminations (400) —
  named with what it depends on, because that is the list S1d.2.4's
  bit-identity check has to hold across;
- the ways an obligation rule could still touch the loop despite the
  separation: shared activators (`bijective-setup` gains two), a
  `:no-hypothesis` or `__closed__` interaction, and the rule *count* itself,
  which several counters are keyed on.

That argument is what makes S1d.2.4's "every existing verdict unchanged"
bullet checkable rather than hoped.

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
- The disturbance argument written — the order-sensitive set named (27 rules
  with a NAF premise, 16 matching a stored `(not …)`, 42 pure joins that
  cannot care), and why a pass after the fixpoint sits after all of them by
  construction.
- **Every channel that does reach the loop named with its number**, not
  argued away — §4.2 of the audit is the one that contradicts a downstream
  acceptance bullet, which is what an audit is for.
- Any missing endpoint named as a finding, not silently filled.
