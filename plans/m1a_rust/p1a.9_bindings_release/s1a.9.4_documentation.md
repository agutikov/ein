# S1a.9.4 — Documentation

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 1 day
**Depends on:** [S1a.9.3](s1a.9.3_packaging.md)
**Implements:** the milestone's closing obligation

## Context

The repo's documentation currently describes ein.py as *the*
implementation: `docs/kernel/inference/python_impl.md` is the engine
internals page, `docs/api/` is "the Python embedding API", and
`AGENTS.md` orients a reader entirely around `ein.py/`. After this
milestone that is wrong in a specific way — ein.py is still real and
still green, but it is the **oracle and reference**, and ein.rs is what
ships.

This stage makes the documentation say that, without rewriting the
kernel docs (which are implementation-independent by design and stay
exactly as they are — they are the specification both engines
implement).

## Acceptance

- A reader arriving at the repo learns, within one screen, that there
  are two implementations, which one ships, and why the other exists.
- No documentation page claims ein.py is the only implementation.
- `plans/README.md`'s status table records M1a as shipped, with its date
  and its measured outcome.
- Every number this milestone changed is updated where it is quoted, with
  the old value labelled rather than deleted.
- `./run_tests.sh` still green — the oracle is part of the deliverable.

## Tasks

### Task T1a.9.4.1 — Orientation

`AGENTS.md` (= `CLAUDE.md`): add `ein.rs/` beside `ein.py/` under *Where
things live*, note that `stdlib/` and `examples/` are shared, that
`conformance/` holds the parity harness, and that **ein.py is the parity
oracle and must stay green**. Terse — the file is a map.

`README.md`: the same, plus install pointers.

### Task T1a.9.4.2 — Engine internals

Add `docs/kernel/inference/rust_impl.md` as a sibling of
`python_impl.md`, describing ein.rs's internals at the same altitude:
the integer data model, the register matcher, the layered KB, the
parallel levels. Cross-link the two, and add a line to each saying that
`architecture_and_algorithms.md` is the shared vocabulary.

### Task T1a.9.4.3 — Embedding docs

`docs/api/README.md` gains a Rust-embedding page (using the crates
directly) and a note that `ein_rs` mirrors the Python contract. Each
existing page gets a line saying the same contract holds for `ein_rs`,
linking the parity suite
([S1a.9.2](s1a.9.2_api_parity_tests.md), written from `docs/api/`'s
perspective as `../../plans/m1a_rust/…`).

### Task T1a.9.4.4 — Update the quoted numbers

`docs/kernel/inference/features.md` (regenerated in
[S1a.6.7](../p1a.6_performance/s1a.6.7_relever_matrix.md)),
`architecture_and_algorithms.md` §7's cost split, and any stage or
followup that quotes a wall-clock. Label the old numbers with their date
and engine rather than deleting them — the arc from naive → semi-naive →
fail-fast → boundary → native is the interesting part, and it is the
project's memory.

### Task T1a.9.4.5 — Close the milestone

`plans/README.md`: status table row for M1a → **shipped**, with the date
and the headline measurement. `plans/m1a_rust/README.md`: a closing
`**Status:** done — <date>` line under the heading, per the repo's
"don't delete; the trail is the project's memory" rule.

Update [`divergences.md`](../divergences.md) with its final state (ideally
empty) and [`open_questions.md`](../open_questions.md) with each
question's resolution.

### Task T1a.9.4.6 — Downstream pointers

[M1b](../../m1b_gui/README.md): note that the engine it binds to is
ein.rs, linked as ordinary crates (its Tauri backend *is* the Rust
process — there is no server between them), and that `.einb`
([P1a.8](../p1a.8_binary_container/README.md)) is available for saved
sessions. [M2](../../m2_nl_to_ir/README.md): note that PyO3 is the
boundary, the socket alternative having gone with the server.
[F11](../../followups/f11_deductive_layer_perf.md): closed or updated by
[S1a.6.7](../p1a.6_performance/s1a.6.7_relever_matrix.md).

## Notes

- Do not touch `docs/kernel/ir/**`. The language specification does not
  change in this milestone — that is the whole point of invariant I1 —
  and editing it would suggest otherwise.
- The one genuinely new user-facing concept is `--jobs` and its
  guarantees; make sure that lands in `docs/api/inference.md` and not
  only in the plan.
