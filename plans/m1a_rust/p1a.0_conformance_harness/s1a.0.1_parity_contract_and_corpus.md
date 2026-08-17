# S1a.0.1 — Parity contract, corpus manifest, divergence ledger

**Phase:** P1a.0 (Conformance harness + shared assets)
**Estimate:** 3 days
**Depends on:** nothing
**Implements design:** [design/01](../design/01_parity_contract.md),
[design/02](../design/02_determinism_and_order.md)

## Context

The port's whole risk profile is "a difference nobody noticed". This
stage builds the instrument: a corpus with an explicit run matrix, a
runner that executes any two implementations over it, a differ that
compares at four tiers, and a ledger for the differences we choose to
accept.

It also runs the **determinism audit** against ein.py itself. That audit
is expected to find real bugs — [design/02](../design/02_determinism_and_order.md)
§5 names three hazards, one of which (H1, `frozenset` iteration in the
symmetric mirror) would make ein.py's own output depend on
`PYTHONHASHSEED`. Fixing the oracle first is mandatory; porting a
nondeterminism is not a thing that can be done.

## Acceptance

- `conformance/corpus.toml` enumerates every `.ein` under `examples/`
  and `stdlib/`, with a group and a run list; a CI check fails on any
  unlisted file.
- `ein-conformance run --impl-a <cmd> --impl-b <cmd> [--tier T0..T3]`
  produces a per-entry table and a non-zero exit on any diff.
- Python-vs-Python is green at T3.
- The `PYTHONHASHSEED` sweep is green, with any failure fixed in ein.py
  and pinned by a regression test.
- Load-negative fixtures are extracted from the Python tests into files,
  and ein.py's tests read them from there.
- `plans/m1a_rust/divergences.md` exists, with the entry template and a
  statement that an empty ledger is the P1a.5 goal.

## Tasks

### Task T1a.0.1.1 — Corpus manifest

Enumerate `examples/**` (57 `.ein` files — 53 positive + 4 parse-negative)
and `stdlib/**` (7) into
`conformance/corpus.toml` with `group`, `runs`, `levers`, `slow`. Groups:
`positive`, `parse-negative`, `load-negative`, `stdlib`, `golden`,
`generated`, `crash-parity` (Q-M1a.14). Derive the initial run list from
[`examples/README.md`](../../../examples/README.md)'s catalog and from
what the Python tests already exercise per file. Add the completeness
check as a test in both suites.

### Task T1a.0.1.2 — Extract the load-negative fixtures

`KBLoadError` cases currently live as inline source strings inside
`ein.py/tests/**` (~40 of them). Move each to
`examples/broken/load/<name>.ein` with an adjacent
`<name>.expected` holding the exact message; re-point the Python tests at
the files. This is what lets both implementations assert the same text
([design/11](../design/11_shared_assets.md) §4) and it makes the
loader's error surface visible as data rather than as assertions.

### Task T1a.0.1.3 — The runner

`ein-conformance run`: for each corpus entry × run-matrix cell, invoke
both implementations, capture stdout/stderr/exit code/produced files,
apply the normalisation list ([design/01](../design/01_parity_contract.md)
§5), and diff at the requested tier. Parallel across entries (level 4 —
[design/08](../design/08_parallelism.md) §5), output a summary table plus
per-failure detail. Must run with `--no-cache` semantics for any
implementation that has caches.

### Task T1a.0.1.4 — The tier differs

T0 (verdict), T1 (counters), T2 (event log — lands with
[S1a.0.2](s1a.0.2_oracle_event_protocol.md)), T3 (bytes). T0/T1 read a
structured summary the runner asks each implementation for; ein.py grows
a `--json-summary` flag (additive, off by default) that dumps verdict +
every counter listed in [design/01](../design/01_parity_contract.md) §2.

### Task T1a.0.1.5 — Determinism audit of ein.py

Run the corpus under `PYTHONHASHSEED` ∈ {0, 1, 42, and an unset random
seed}, T3-diffing each against seed 0. Investigate every difference.
Expected: H1 (symmetric-mirror `frozenset`). Fix in ein.py with the
minimal change (a `sorted(...)`), add a regression test, and record the
fix in the stage log — it is an M1 semantics-preserving bug fix landing
under M1a's number, which is correct: the work is the port's, not a
retro-fit into a closed phase.

Also verify H3's scope by diffing `--shuffle --seed N` runs, and confirm
H2 by constructing a mixed-`str`/`int`-arg fixture and observing the
`TypeError` (feeding Q-M1a.4).

### Task T1a.0.1.6 — Divergence ledger

Create `plans/m1a_rust/divergences.md` with the template from
[design/01](../design/01_parity_contract.md) §6 and the rule that an
entry requires a written "what would make this unacceptable".

## Notes

- The runner must be able to invoke ein.py under **both** CPython and
  PyPy, since the two are separate potential sources of divergence and
  the acceptance gate runs under PyPy.
- Keep the runner independent of the engine crates: it shells out. A
  harness that links the implementation it is testing can only find the
  bugs it does not share.
