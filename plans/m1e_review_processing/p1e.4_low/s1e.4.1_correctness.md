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

---

## ✅ Done 2026-09-01 — the limit was in the wrong unit, and the next line was a panic

**Disposition: fixed.** Not `accepted`, which § Notes floated on two premises,
and neither survived contact.

### What each was, as reported and as found

| | as reported | as found |
|---|---|---|
| the arena bound | a `u32` offset guarded only by an id count — unreachable, and it contradicts the module's stated principle | as reported, **and the comment is wrong in the binding direction**: it says the id ceiling *"needs ≥ 4 GB of symbol text"*, which holds only at a mean symbol length of exactly 4.0 bytes. Measured through `ein kb save`'s symbol-table header (2026-09-01) the corpus runs **7.55–21.00 B/symbol**, so the *byte* bound arrives at 19–53 % of the id ceiling. The unguarded limit was the one that binds first |
| the cost | *"interning is a hot path… a comparison per insert is invisible until it is not"* | **not measurable, and the reason was already in the repo**: `counters.rs` records `FactStore::intern` assigning 505 ids in 2 318 949 calls on `branching/06 -e`, and `intern.rs` says the symbol table is *"effectively frozen after load"*. Each arena has exactly **one** growth site, so the guard hoists there and runs on the miss path only |
| the sibling sweep (T1e.4.1.2) | *"worth one grep"* | it found **a reachable process panic on the line below the one the review named** |

### The panic

`facts.rs`'s `u16::try_from(args.len()).expect("a fact's arity fits u16")` is
not unreachable. A **three-line** program of 447 KB —

```
$ ein solve wide.ein          # (p A0 … A65535)
thread 'main' panicked at crates/ein-core/src/facts.rs:130:46:
a fact's arity fits u16: TryFromIntError(PosOverflow)
exit=101
```

— which is the failure shape
[`defined_behaviour.md` § 4.3](../../../docs/kernel/defined_behaviour.md)
ruled out for `(eq ?x)` **one phase earlier** in this same milestone, and the
one `corpus_cli::every_refusal_carries_a_diagnostic` forbids. `Overflow`'s own
doc comment already promised it did not exist: *"a research finding, not a
crash — so it is a `Result` at the three sites that assign ids, and not a
panic."* There were four such sites, and the fourth was an `expect`. It now
reads `kb load error: a fact takes at most 65535 arguments`, exit 1. The
threshold is exact: 65 535 solve.

### What was changed

`ARENA_CAPACITY` and one `arena_room` helper beside `CAPACITY`, so the bound is
stated once; three new `Overflow` variants (`SymbolText`, `FactArgs`,
`FactArity`) keeping the enum's per-space granularity; the guard at the one
growth site of each arena; and `CAPACITY`'s doc comment rewritten to state
**both** limits, which binds first, and the measurement that says so.

### What holds it

- `ein_core::facts::tests::a_fact_wider_than_the_arity_field_is_refused_and_one_narrower_is_not`
  — both sides of the exact threshold, and that the store is unchanged by the
  refusal.
- `ein_core::intern::tests::the_arena_bound_is_the_byte_bound_and_not_the_id_bound`
  — the boundary where it is *computed* (4 GiB is not allocatable in a unit
  test), the `checked_add`, and a last assertion that pins the **claim**: at
  the corpus's shortest mean symbol length the byte bound still binds first, so
  an edit restoring the old sentence fails a test rather than merely reading
  oddly.
- [`defined_behaviour.md` § 4.5](../../../docs/kernel/defined_behaviour.md) —
  the five limits, their messages and their exit code, in § 4.3's form.

### The cost, measured rather than assumed

`cargo bench --bench engine`, guarded against unguarded, 2026-09-01, the three
groups that touch a growth site:

| | before | after |
|---|---:|---:|
| `load/zebra2` | 739.62 µs | **732.66 µs** |
| `saturate_root/zebra2` | 1.2239 ms | **1.2091 ms** |
| `fork/zebra2` | 370.65 ns | **293.79 ns** |

Every difference is noise and two of the three are nominally *faster* guarded,
which is the same statement. The acceptance's *"if it moves, hoist the check"*
did not fire; it is hoisted anyway, because one growth site per arena is where
the bound belongs.

### T1e.4.1.2 — the sibling sweep, and its deposit against Q9

Recorded where a reader of the container will find it
([`ein-einb/src/lib.rs`](../../../ein.rs/crates/ein-einb/src/lib.rs) §
*The `u32`-offset sweep*), because `ein-einb` is the crate the question is
about — the one `cast.rs` permits `unsafe` in and the one that reads bytes it
did not write. The answer splits three ways: the **reader** is covered by three
named refusals in `sections.rs` (offsets must close, must be sorted, and every
slice is a `get`), on the path a forged file takes *after* the digest has been
made to match; the **writer** is not, and does not need to be now that the
bound lives one crate down; and the one real hole was not an offset at all but
the arity panic, which `tests/corruption.rs`'s two fuzzers were structurally
unable to reach — they mutate bytes of a 20 KB seed whose widest row has 147
arguments, and a byte flip cannot manufacture 65 536.

**Gate:** `cargo test --workspace` — **806 tests, 0 failures** (804 before, and
the two new ones are the pins above). No golden moved: no corpus program comes
near any of the five limits, the widest fact in the tree has arity 5, and the
largest symbol arena is 3 692 bytes.
