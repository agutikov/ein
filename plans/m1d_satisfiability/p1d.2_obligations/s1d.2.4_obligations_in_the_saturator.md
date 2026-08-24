# S1d.2.4 — Obligations in the saturator

**Phase:** [P1d.2](README.md) (Obligations)
**Estimate:** 4 days
**Depends on:** [S1d.2.3](s1d.2.3_the_form.md) (the atom loads),
[S1d.2.1](s1d.2.1_property_audit.md) (the disturbance list),
[S1d.2.2](s1d.2.2_domains.md) (the domain contract)

## Context

The report stratum, whole — and nothing else: after this stage a saturated
state can say **what it still owes**, through every surface a fact store is
inspected through, while every verdict word, counter and golden stands.

The mechanism is decided (`obligation_forms.md` § G): obligation rules are
evaluated at the NAF boundary against *that* KB, the way `absents_still_pass`
already re-checks every negative premise; their conclusions are **tallied,
never admitted** — a stored `open` would survive its own discharge in a fork.
The atom is terminal like `(false)`: no rule matches it, the engine reads it.
An instance is (rule, bindings); it is **discharged** iff `∃b: G ∧ B` is
present in this KB; the per-KB tally counts undischarged instances, deduped
by instance identity so two rules owing the same slot count once each and one
rule firing twice counts once.

## Tasks

### Task T1d.2.4.1 — the boundary tally

Evaluate `open`-asserting rules per quiescence at the probe band (500),
compute discharge, keep the tally and the instance list in engine state
(fork-local, copied like the layer state — not in the fact store). The
disturbance argument from S1d.2.1 is the check that nothing before 500 moves.

### Task T1d.2.4.2 — the three surfaces

- **`--events`**: a new `owe` event kind (per undischarged instance at
  quiescence: rule, bindings, rendered `:why`) — the events schema moves, and
  `every_event_kind_the_schema_defines_is_reachable_from_the_corpus` demands
  a corpus entry reaches it, so the fixtures of T1d.2.4.4 are not optional.
- **`--json-summary`**: an `owes` block — count plus instances.
- **the trace**: an "outstanding obligations" section rendering the `:why`s.

[`events.md`](../../../docs/kernel/inference/events.md) and the summary
schema doc move with it.

### Task T1d.2.4.3 — the stdlib duals

`total-owed` / `surjective-owed` in `std.algebra` (or `std.bijection` — the
audit says which file owns totality today), fanned out by `bijective-setup`
gaining two activators; the slots-family dual per S1d.2.1's finding. No
puzzle changes a line — the fan-out reaches every `(bijective R)` declarer.

### Task T1d.2.4.4 — the conformance programs

Per new rule, the P1c.1 pair: one program where it **fires** (an unmet
obligation, the `owe` event observable, the summary count in the `:expect`
story) and one where it is **loaded, activated and satisfied** — fires
nothing, owes nothing. The stdlib-coverage gate enforces the first half
mechanically; the second is the negative case a guard bug lives in.

### Task T1d.2.4.5 — the two numbers

- **The §5 number, from the engine.** `zebra2-minus-15` at root quiescence
  reports **owes = 46** — the number
  [`obligation_forms.md` §5](obligation_forms.md#5-what-this-looks-like-on-zebra2-minus-15)
  measured by hand (23 forward + 23 backward). The engine reproducing the
  hand census is the stage's acceptance in one line.
- **The conservation audit.** A declared `bijective` n×n owes 2n at the
  unwitnessed base; the predicted-vs-emitted diff is a test
  (`ideas.md`'s "obligated facts × arity", the layer-census style of claim).

### Task T1d.2.4.6 — the cost guard

The tally is per-quiescence work in the hot loop — the §1 cost shape P1a.6
spent twelve stages buying back. Instrument with `ein_core::counters`
(compiled out unless asked), and hold the line: `zebra -e` and `zebra2 -e`
within their P1a.6 baselines; the stage that cannot hold it redesigns the
band, not the budget.

## Acceptance

- A saturated state reports outstanding obligations through `--events`,
  `--json-summary` and the trace — the phase README's second acceptance
  bullet, delivered here.
- `ein solve examples/zebra2-minus-15.ein` (root) reports owes = 46; the
  conservation audit passes on every `bijective` declarer in the corpus.
- **Every existing verdict, counter and golden unchanged** — this stage adds
  a report line and nothing else. New goldens only for the new fixtures.
- Timings hold (`zebra -e` 47.5 ms-class, per the F9/P1a.6 discipline);
  the counter instrumentation shows where the tally's time goes.
