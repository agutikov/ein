# S1a.9.4 — Documentation

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 1 day
**Depends on:** [S1a.9.3](s1a.9.3_packaging.md)
**Implements:** the milestone's closing obligation

> **Amended 2026-08-21.** This stage was written for a tree with two
> implementations in it: it would have said "ein.py is the oracle, ein.rs is
> what ships". [P1a.10](../p1a.10_single_implementation/README.md) removed
> the first, and it runs *before* this phase now, so the "two
> implementations, here is why" framing is gone. What is left is sharper and
> smaller: **there is one engine, and the documentation has to describe its
> two surfaces** — the CLI and `ein_rs`.
>
> The division of labour with
> [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md) is the thing
> to keep straight. S1a.10.6 removes what became false when the Python engine
> left, and states the one gap it cannot close: `docs/api/` documents an
> embedding API with no implementation until this phase ships. **S1a.9.4
> closes that gap**, and it is the last documentation stage in the milestone.

## Context

The repo's documentation described ein.py as *the* implementation:
`docs/kernel/inference/python_impl.md` was the engine internals page,
`docs/api/` is "the Python embedding API", and `AGENTS.md` oriented a reader
entirely around `ein.py/`. S1a.10.6 took the false half out. What it could
not do is give `docs/api/` a subject: the PyO3 module does not exist until
[S1a.9.1](s1a.9.1_pyo3_surface.md).

This stage makes the documentation say what is true, without rewriting the
kernel docs (which are implementation-independent by design and stay exactly
as they are — they are the specification the engine implements).

## Acceptance

- A reader arriving at the repo learns, within one screen, what ein is,
  that ein.rs is the implementation, and how to reach it — as a binary, as
  crates, or as `ein_rs`.
- **`docs/api/` documents `ein_rs`**, page for page, and every page's claims
  are executed by [S1a.9.2](s1a.9.2_api_parity_tests.md)'s suite. This is
  the gap S1a.10.6 recorded and could not close.
- `plans/README.md`'s status table records M1a as shipped, with its date and
  its measured outcome.
- Every number this milestone changed is updated where it is quoted, with
  the old value **labelled** rather than deleted — and every CPython/PyPy
  figure labelled *frozen*, since nothing can re-measure one
  ([S1a.10.4](../p1a.10_single_implementation/s1a.10.4_utils.md)).
- **`git grep -i 'ein\.py'` returns only history**: the divergence ledger,
  the phase records, and dated measurements. No page describes it as
  something a reader can run. (The user's standing requirement, recorded
  here at the phase that closes the milestone: *after P1a completion the
  repository must not contain any reference to the Python implementation
  except as record; ein.rs is the reference implementation*.)



## Tasks

### Task T1a.9.4.1 — Orientation

`AGENTS.md` (= `CLAUDE.md`): S1a.10.5 and S1a.10.6 have already cut the
Python tree out of *Where things live*. What this stage adds is the shipped
surface — the binary, the crates, and `ein_rs` — and where each is
documented. Terse; the file is a map.

`README.md`: the same, plus install pointers (binary download,
`cargo install`, `pip install`).

### Task T1a.9.4.2 — Engine internals

`docs/kernel/inference/` needs **one** engine-internals page describing
ein.rs at the altitude `python_impl.md` used: the integer data model, the
register matcher, the layered KB, the parallel levels. Whether that is a new
`rust_impl.md` or the old page rewritten in place is S1a.10.6's call, since
that stage decides what happens to `python_impl.md`; this one owns the
*content* either way. A line at the top saying
`architecture_and_algorithms.md` is the shared vocabulary.

### Task T1a.9.4.3 — Embedding docs

**The five `docs/api/` pages change subject rather than gaining a note.**
They described `ein.ir` / `ein.kb` / `ein.inference` / `ein.trace`; they
describe `ein_rs` after this. The contract is the same one — that is the
point of having kept it — but the module it names is different, and a page
that says "the same contract holds for `ein_rs`" beside a Python import is
the shape to avoid.

`docs/api/README.md` also gains a **Rust**-embedding page, using the crates
directly, which is the surface M1b binds against and which nothing documents
today.

Every page's claims are executed by [S1a.9.2](s1a.9.2_api_parity_tests.md);
link it from each.

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

Update [`divergences.md`](../divergences.md) with its final state and
[`open_questions.md`](../open_questions.md) with each question's resolution.
"Ideally empty" is no longer the bar and should not be pretended to be:
D1–D3 record where two implementations differed, one implementation is left,
and the file is **history that stays** — the phase's own non-goal says so.
What it should gain is a header saying that, and the note that a *new* entry
now means two surfaces of one engine disagreeing rather than two engines.

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
