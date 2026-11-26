# L-M2.6 — The bounded call returns no writer of H

**Status:** open
**Found:** by S2.2.1, 2026-11-26, writing the contract's tests of the steps by hand
**Owner:** the stage that builds the record or the contract's check, S2.2.4 or S2.2.5 (the record's field), with
[E-39](../../../evaluation/hypotheses/E-39-the-answer-does-not-say-who-wrote-the-candidates.md) the kernel's side

## What

The contract names a field of the record, *the writer of H the kernel reported*, and the steps' judgement reads it
(`blind` with no `(solve …)` list is a mechanism not stated). `einaf::kernel::call` returns the feedback object,
built from `ein_infer::read_out::run`'s answer, whose `Answer` holds H's candidates and not the rung that wrote them:
the rung is `explore::Space::rung`, crate-private. `ein_infer::structure::hypotheses` returns it, at the price of
root's saturation a second time, and `ein show hyps` prints it. So `einaf/tests/contract.rs` reads the writer and
the candidates off `ein show hyps` from the binary the gate built, as `corpus.rs` reads `--json-summary`, and the
feedback object's schema stays as it was.

## Evidence

`grep -n 'fn rung' ein.rs/crates/ein-infer/src/explore.rs`; `ein.rs/crates/ein-cli/src/show.rs`'s `cmd_hyps`;
`einaf/tests/contract.rs`'s `hypotheses`.

## What it costs

A record of a model's run cannot hold the writer until `einaf::kernel` returns it: either a key of the feedback
object (one more saturation of root a call, under the call's bounds) or a value of the library's read-out (a change
of Ein's API, which Q-M2.16 allows as a feature).
