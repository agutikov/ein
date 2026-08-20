# S1a.10.3 — The corpus without a second engine

**Phase:** P1a.10 (One implementation)
**Estimate:** 2 days
**Depends on:** [S1a.10.1](s1a.10.1_bank_the_oracle.md)

## Context

`conformance/` is two things wearing one name:

1. **the corpus manifest** — `corpus.toml`, one entry per `.ein` file with the
   runs it is exercised under, plus the completeness check that fails when a
   file under `examples/` or `stdlib/` has no entry. Several ein.rs tests read
   it today. This is *inventory*, and it survives the oracle intact.
2. **the differential runner** — `ein-conformance`, which shells out to two
   implementations and diffs them at four tiers, plus `EVENTS.md`'s protocol
   and `ein-parity`'s normalisation list. This is what has no second operand
   any more.

The instinct "remove conformance" would take the first with the second. The
stage exists to separate them.

## Acceptance

- The manifest lives somewhere a single-implementation repo can defend, and
  the completeness check still fails on an unlisted `.ein` file.
- `ein-conformance` and `ein-oracle` are gone; `ein-parity`'s
  normalisation list is either gone or reduced to whatever ein.rs still
  normalises against its own goldens, with the difference recorded.
- The `runs` column keeps its meaning: it is now "the invocations this entry is
  *exercised* under" rather than "…*compared* under", and whatever runs them
  says which.
- `--tier` disappears from the vocabulary, or is re-defined against goldens.
  Leaving T0–T3 in the documentation with no runner is the failure mode this
  acceptance is written against.

## Tasks

### Task T1a.10.3.1 — Decide where the manifest lives

Options: keep `conformance/corpus.toml` and let the directory mean "the
corpus"; or move it under `examples/` (which it describes) or `ein.rs/`
(which reads it). **Recommendation: keep the path**, rename the concept in
`conformance/README.md`, and take the churn in prose rather than in every test
that reads it — the path is referenced from `CLAUDE.md`, both suites and a
dozen plan documents.

### Task T1a.10.3.2 — A runner over one engine

What replaces the harness is not a diff, it is a sweep: run every entry under
every declared run, assert it does not crash, and compare against a golden
where one was banked. That is closer to `utils/render_examples.sh` than to
`ein-conformance`, and it belongs in `cargo test`.

### Task T1a.10.3.3 — Retire the crates

`ein-oracle` (ein.py behind a JSON-Lines protocol) is dev-only and dies with
its subject. `ein-conformance` carries the corpus *bench* set
(`crates/ein-conformance/benches/engine.rs`) — **the eight-bench set moves,
it does not die**; P1a.6's whole record is denominated in it.

### Task T1a.10.3.4 — The events protocol

`conformance/EVENTS.md` specifies `--events`, which is a *product* surface
(T2's operand, but also a debugging tool and M1b's likely feed). The protocol
document survives; its framing as "the oracle event protocol" does not.

## Notes

- The bench set is the quiet dependency here, and the one most likely to be
  noticed only after it is gone. `plans/m1a_rust/p1a.6_performance/baseline.md`
  is unreadable without it.
