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

---

## Outcome — 2026-08-17

Everything in Acceptance is met. What is worth recording is what the
instrument found while being built, because the phase's premise ("it finds
ein.py bugs") turned out to be literally true on the first Python-vs-Python
run.

### The determinism audit (T1a.0.1.5)

| hazard | predicted? | outcome |
|---|---|---|
| **H1** — `frozenset` iteration in the symmetric-mirror seed | yes | reproduced with three `(__symmetric__ R)` markers; `ein saturate --dump` came out in three different orders across `PYTHONHASHSEED ∈ {0, 7, 42, 99}`. Fixed with `sorted(…)`; `examples/features/06_symmetric_native.ein` is the reproducer, since **nothing in `examples/` or the stdlib marked a relation** and the corpus could not have caught it. |
| **H2** — `sorted()` over mixed-type fact args | yes | reproduced, and **narrower than the audit assumed**: blind hypgen cannot reach it (candidates come from `kb.names`, which `rebuild_indexes` only feeds `if isinstance(a, str)`), so it takes an `hrule` whose `:assert` carries a binding through. Recorded not repaired (Q-M1a.4). |
| **H3** — `--shuffle` needs CPython's MT19937 | yes | confirmed benign in the direction that matters: same seed is byte-identical, and across seeds the verdict, every counter and the root shape agree. Only *which of k models is found first* moves — which is why `--json-summary` sorts its `solutions` array by model. |
| **H4** — `unsat_core` iterated raw at two display sites | **no** | found by the harness. `render/slice.py`'s `⊥` edges land verbatim in `solve --trace`, so **the same puzzle produced two different trace files across runs**; `_lattice_dump.py` carried the same instability into `--dump-states`. Both `sorted(…, key=repr)` now. [design/02](../design/02_determinism_and_order.md) §4's "safe" row is corrected, and its rule replaced: a `set` is safe only when *every* reader is checked. |

### Q-M1a.14's proposed rule was wrong

The crash-parity fixture raises `TypeError: '<' not supported between instances
of 'int' and 'str'` — and *which operand is named first* depends on the
`frozenset` iteration order inside `sorted`, so ein.py alternates between two
messages across hash seeds. "Exit code + the first line of stderr" would have
made the determinism sweep fail on a difference that is not one. The group
compares exit code + **exception class**.

### The loader's error surface (T1a.0.1.2)

29 fixtures, up from the ~20 cases the tests inlined — `:symbols`
(not-a-list / empty), `:as` (not a bare name), file-relative not-found, a
`(config …)` type error and the second end of the import cycle had no test at
all. The census in [`examples/broken/load/README.md`](../../../examples/broken/load/README.md)
records three things:

- **ten loader messages are unreachable** from a `.ein` file — six because the
  grammar rejects the shape first, four because they are internal-invariant
  assertions the top-level router cannot reach;
- **one is not expressible as a file** ("file-relative import needs a base
  directory" fires only when `base_dir is None`), and says so;
- **almost every message ends `at None`** — Q-M1a.6 as data, and the reason
  load-error parity needs almost no path normalisation.

Building the corpus also found the three entry points disagreeing about how to
report one broken file: `solve` printed `kb load error: …`, **`saturate` raised
through to a traceback**, and `render` said "no rule forms". The first two are
now the same message (`render`'s views render the IR, never the KB — correct as
it stands, and now pinned as such). Fixed in ein.py rather than ported, per the
milestone's non-goals: ein.rs reproducing a Python traceback is not a behaviour
anyone wants preserved.

### The corpus (T1a.0.1.1)

95 entries / 556 cells; 78 entries and 438 cells in the per-commit tier.
`slow` is measured, not guessed — the whole 7-run matrix was probed under
CPython 3.14 and an entry is slow at ≥ 3 s. Seven entries lose a run that
outlives a 150 s budget, each with the reason recorded: a run nobody can finish
is not coverage, and leaving it in would make the nightly tier report "timed
out on both sides" forever.

Q-M1a.16 is opened by this stage: only **four of the ten** `SolverConfig`
levers `utils/feature_matrix.py` drives are reachable from the CLI, and a
harness that shells out cannot flip the rest.

### One bug in the harness itself

The first whole-corpus run hung. Two cells the probe had measured at 0.3 s and
1.6 s sat for two minutes with no output: `execute` polled `try_wait()` on a
child whose stdout was a **pipe**, and `render lattice` writes more DOT than a
pipe's ~64 KB will hold, so the child blocked on `write` while the harness
waited for it to exit. `wait_with_output` does not fix it — the deadlock
happens before the wait. The streams are redirected to files now, which has no
such limit and leaves both sides' output on disk where a hand investigation
wants it.

Worth recording because it is the failure mode a harness is *least* able to
report on itself: it does not crash, it does not diff, it simply never
finishes — and on a corpus with `slow` entries in it, "still running" is not
obviously wrong.
