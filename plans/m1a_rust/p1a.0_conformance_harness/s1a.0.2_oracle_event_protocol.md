# S1a.0.2 — The oracle event protocol

**Phase:** P1a.0 (Conformance harness + shared assets)
**Estimate:** 3 days
**Depends on:** [S1a.0.1](s1a.0.1_parity_contract_and_corpus.md)
**Implements design:** [design/01](../design/01_parity_contract.md) §3

## Context

T2 parity — "the two engines took the same steps" — needs both
implementations to narrate what they did in a comparable format. ein.py
already has half of one: `MonotonicDumper` / `LatticeDumper` emit a
`00_timeline.jsonl` through `_TimelineMixin._emit_timeline` under
`--dump-states`, covering the search layer's five lifecycle hooks. What
is missing is the deductive layer — firings, enqueues, parks,
admissions, retirements, quiescences, alternative justifications, hypgen
verdicts — which is where a port most easily drifts.

This stage adds `--events FILE` to ein.py: one JSON object per line,
opt-in, off by default, with no branch on the hot path when disabled.
The same schema is what ein.rs emits and what
[the server streams](../design/09_server_mode.md) §5.

> This edits ein.py source. That is planned M1a work, not a retro-fit
> into a closed M1 phase: the flag exists to serve the port and lands
> under this stage's number.

## Acceptance

- `ein solve --events out.jsonl <file>` and `ein saturate --events …`
  emit the schema in [design/01](../design/01_parity_contract.md) §3.
- With `--events` absent, a profile of exhaustive zebra2 shows no
  measurable change (< 1 %) versus before the stage.
- `ein-conformance diff a.jsonl b.jsonl` reports the first differing
  event with a structural diff and a divergence-class summary.
- Python-vs-Python T2 is green across the corpus, including under
  `--shuffle --seed N`.
- The schema is documented in `conformance/EVENTS.md` with a version
  number, and the version is emitted in the `run` event.

## Tasks

### Task T1a.0.2.1 — Schema and writer

Define the event schema (v1) as `conformance/EVENTS.md`. Implement a
tiny writer in ein.py — a module-level `Optional[TextIO]` plus
`emit(kind, **fields)` that returns immediately when unset. Facts are
rendered with the existing `cli._factdump.fact_sexpr`, so the protocol
carries no implementation-internal ids.

### Task T1a.0.2.2 — Instrument the deductive layer

Call sites: `Engine.compile_for` (miss only), `Saturator._enqueue_binding`
(with `parked` flag), `Saturator._closure_step` (each yielded `Firing`),
`_next_mirror_firing`, `_admit_from_boundary` (park / admit / retire, and
the round boundary), `store.record_justification` (on a True return),
`fire`.

Care: emitting must not change behaviour. In particular do not
materialise generators, do not stringify inside a hot branch when the
writer is unset, and do not perturb `_tiebreaker`.

### Task T1a.0.2.3 — Instrument the search layer

`try_commitment_set` result, `emit_nogood`,
`_emit_negated_fact_writeback`, `_promote_forced_positives`, each hypgen
candidate verdict (`emitted` or the filter/pre-skip name that dropped
it), the final verdict with all counters. Reuse the existing dumper hook
sites where they already exist rather than adding parallel ones.

### Task T1a.0.2.4 — The differ

`ein-conformance diff`: stream both files, compare event by event with
normalisation applied, and on the first mismatch print the preceding N
events from both sides plus a field-level diff. Add a `--classes`
summary (how many events of each kind on each side) so a wholesale
divergence is obvious before the first-diff detail.

### Task T1a.0.2.5 — `--json-summary`

The T0/T1 structured summary
([S1a.0.1](s1a.0.1_parity_contract_and_corpus.md) T1a.0.1.4): verdict
type, `k`, `exhausted`, every counter, the model as a sorted fact list,
the unsat core, goal bindings. Additive flag, stable field order.

## Notes

- Event volume on exhaustive zebra2 will be large (≈ 40 k firings plus
  ~194 k redundant ones). Make `fire` events for redundant firings
  opt-in via `--events-level {normal,verbose}` so the default file stays
  navigable — but ensure T2 comparisons run at `verbose`, since a
  redundant firing is exactly the kind of thing a port drops.
- The writer should flush per line; a crashed run's prefix is the most
  useful artefact it can leave.

---

## Outcome — 2026-08-17

`--events FILE` and `--events-level {normal,verbose}` on both `solve` and
`saturate`; 17 event kinds across both layers; the schema versioned as
`ein-events/1` in [`conformance/EVENTS.md`](../../../conformance/EVENTS.md) and
emitted in the `run` event. `ein-conformance diff` reads it by hand and
`tier::compare` reads it as the T2 gate — the same comparison, so the tool and
the gate cannot drift apart.

Python-vs-Python at **T2** over the per-commit tier: **215 cells compared, 0
differences**; the other 223 are `render …` and negative cells that never emit
a log, and the tier reports those as *skipped* rather than as a green it did
not earn.

### What the schema gained over design/01 §3

- **`hypskip`** — the pre-candidate skips (`closed_relation`,
  `relation_not_whitelisted`, `no_hypothesis_relation`, `self_edge`) get their
  own kind rather than a `hyp` with an invented `fact` field: at that point no
  candidate exists, and three of the four are decisions about a *relation*.
  Verbose-only; `self_edge` alone fires once per (object, filler, relation,
  slot).
- **`hyp`'s verdict is a filter name, not a boolean.** `_apply_filters` now
  returns the name of the filter that dropped the candidate — the same name it
  bumps in `stats.filtered` — so a counter difference between two
  implementations locates itself instead of having to be bisected.
- **A mirror emits `mirror` and not `fire`**, so a firing is reported exactly
  once whichever path made it.

### Three things the differ has to ignore, and why

`n` is compared as a **position, not a field**: one extra event on either side
renumbers every line after it, and reporting all of them would bury the
difference that caused them. The `run` event's `impl` (which engine ran — the
premise of the comparison) and `argv` (the artefact paths the *caller* chose)
are excluded for the same class of reason. All three stay in the file, where
they document the run.

### Placement matters more than it looks

The log opens immediately before `solve()`, not before the diagnostics.
`--timing` runs an isolated `Engine.compile_all()` and `--hyp-stats` saturates
a fork of root; with the writer already open, either would stream a second set
of `compile` / `fire` events and make `--events` mean something different
depending on which *other* flags were passed.

### Cost when off

`events.ON` is a module-level `bool` and every call site reads it before
building anything, so with the flag absent the cost is one global read — no
kwargs dict, no formatting. Writing `events.emit(...)` unguarded would pack a
`dict` at every call whatever the flag says, which on the firing path
(≈ 234 k calls on exhaustive zebra2) is not free.

Measured, `solve_exhaustive` on `zebra2.ein` under CPython 3.14, three
best-of-3 runs per side, the baseline taken from a `git worktree` at the
pre-instrumentation commit (`utils/bench_baseline.py`, `EIN_SRC=`):

| | run 1 | run 2 | run 3 | mean |
|---|---:|---:|---:|---:|
| pre-instrumentation | 5637.2 ms | 5607.5 ms | 5640.9 ms | 5628.5 ms |
| instrumented, flag off | 5662.8 ms | 5645.2 ms | 5632.5 ms | 5646.8 ms |

**+0.32 %** on the mean, against a ~0.6 % spread *within* each side — so the
overhead is at or below this machine's noise floor, and well under the 1 %
the stage asks for. The honest reading is not "0.32 % slower" but "not
distinguishable from unchanged at three runs".
