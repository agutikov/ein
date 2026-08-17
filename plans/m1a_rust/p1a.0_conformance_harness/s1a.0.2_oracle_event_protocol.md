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
