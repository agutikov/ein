# S1e.4.1 — Correctness (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 0.5 days
**Depends on:** nothing.
**Findings:** [`CO-L1`](../review/correctness/low.md).

## Context

One finding, and it is a limit that is bounded in the wrong unit.

The `Interner` stores span starts as `u32` — `self.arena.len() as u32`
([`intern.rs:116-127`](../../../ein.rs/crates/ein-core/src/intern.rs)) — and
the `FactStore` stores `args_at` as `u32`
([`facts.rs:126-141`](../../../ein.rs/crates/ein-core/src/facts.rs)). The
`CAPACITY` guard limits only the **number of ids** (2³⁰). So a table with
fewer than 2³⁰ entries whose total text or args exceed 4 GiB would silently
wrap the offset and corrupt spans, with no error. `CAPACITY`'s own doc comment
(*"Reaching it needs ≥ 4 GB of symbol text"*) conflates the two limits — it
describes the byte limit while guarding the count.

Unreachable for any corpus-scale input, and the review says so. What makes it
worth a task is the module's **stated design principle**, twenty lines above
the defect
([`intern.rs:26-32`](../../../ein.rs/crates/ein-core/src/intern.rs)):

> hitting a limit is an error somebody can read rather than a silent wrap into
> another value's identity

The wrap here is exactly the silent kind, and *"another value's identity"* is
literally what a wrapped span start produces.

## Acceptance

- Both limits are guarded, or the unguarded one is documented as a stated
  bound with the reason it is not checked.
- `CAPACITY`'s doc comment states **both** limits and which one it guards.
- The fix costs nothing measurable on the intern path — verified with the
  bench set, not assumed.

## Tasks

### Task T1e.4.1.1 — Guard the arena, fix the comment

Two small changes:

1. **A checked cast or an arena-size guard** beside the existing id-count
   guard, in both `intern.rs` and `facts.rs`. The natural form matches what is
   there: the same error the count guard raises, with a message naming the
   arena rather than the id space. A `debug_assert` is not enough here — the
   principle the module states is about a *readable error*, and a debug
   assertion in a release binary is a silent wrap.
2. **The doc comment**: state the id limit (2³⁰ entries) and the arena limit
   (4 GiB of text / args), and say which the guard enforces — which, after
   (1), is both.

The cost question is real and cheap to settle: interning is a hot path, and a
comparison per insert is the kind of thing that is invisible until it is not.
Run the bench set before and after; if it moves, hoist the check to the one
place the arena can grow rather than per call.

### Task T1e.4.1.2 — Check for siblings

The pattern — a `u32` offset into an arena whose size is bounded by a
different count — is worth one grep. Any other `as u32` over a `len()` in
`ein-core`, and the `.einb` container's offsets in
[`ein-einb`](../../../ein.rs/crates/ein-einb/src/header.rs), which is the one
crate permitted `unsafe` and therefore the one where a wrapped offset would be
worst. The container's own invariants may already cover it; establish which,
and record the answer — it is a small deposit against
[Q9](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md)'s unaudited
`cast.rs`.

## Notes

The honest disposition might be `accepted`: the limit is unreachable, the
guard costs something on a hot path, and the repo does not usually pay for
impossible cases. If that is the outcome, the deliverable is the **comment**,
stating both limits and saying the arena bound is unchecked and why — which
is the part of the finding that is actually wrong today, since the comment
currently claims a guard the code does not have.
