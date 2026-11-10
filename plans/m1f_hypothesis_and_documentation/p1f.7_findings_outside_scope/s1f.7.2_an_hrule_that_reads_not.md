# S1f.7.2 — An hrule that reads `not`: one answer from both builds

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 13. Not started.
**Depends on:** nothing.

## The finding

Recorded in [S1f.5.24's T2 record](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md#t2-record-2026-11-10)
on the release binary at `dbfd7862`:

> **An hrule whose `:match` reads `not` stops a debug build.** `(hrule h () :match (and (a ?x) (absent (not (p
> ?x)))) …)`, written, no macro: `hrule.rs`'s `debug_assert!` that no hrule reads `not` (*an hrule reads `not`,
> which hypgen's kill cache writes mid-enumeration*) fires, exit 101, on every solve question; the release build
> answers, two states. Nothing refuses such an hrule at load, so the two builds disagree.

The assertion is in `Hrules::candidates` ([`ein-infer/src/hrule.rs`](../../../ein.rs/crates/ein-infer/src/hrule.rs)).
Its comment states the premise: the hypothesis generator's lookahead kill cache writes `(not h)` between one
candidate and the next, so an hrule that read `not` would condition on that cache.

## What T1 has to settle first

**Whether the premise still holds.** The comment speaks of the lookahead filter. The walk went at M1e2
S1e2.1c.5, and S1e2.3.1 made the universe config-free. If nothing writes `not` during enumeration any more, the
assertion guards a case that cannot happen. Then an hrule that reads `not` is an ordinary hrule, the release answer
stands, and the fix is to delete the assertion and its comment, with a test that holds the answer to `ein oracle`.

**If something still writes `not` mid-enumeration**, the release answer may depend on the order of the candidates.
Then there are two directions. A refusal at load (one line, every command) agrees with how S1f.5.21 and S1f.5.24
treated a shape the engine cannot answer. Giving the hrule a meaning that does not see the cache is the other
direction, and it is a question for the user.

Either way the two builds give one answer, and it agrees with the oracle.

## Tasks

### T1f.7.2.1 — confirmation, and the premise

Re-take the finding on both builds. Find every writer of a `not` fact between two candidates of one enumeration, or
show there is none. Compare the release answer with `ein oracle` under a permuted candidate order
(`id_order_invariance`'s lever). Census the hrules in the corpus and the stdlib that read `not` (written, or through
`absent`, `unknown` or a macro).

### T1f.7.2.2 — the rows

A row for the debug build's exit 101, where a build's stop on a valid program is a wrong answer. A row for the
release answer if it is wrong or depends on order. Each is filed `open` and held to today's engine.

### T1f.7.2.3 — the fix

Either the stale assertion goes (and `reads_negation` with it, if nothing else reads it), or the load-time refusal
is added and the assertion stays as the invariant that the refusal holds. A test runs the witness on both builds.

### T1f.7.2.4 — the pages

The hrule section of the kernel pages that describe hypothesis generation, `defined_behaviour.md` if a refusal is
added, and the stage's records.

## Acceptance

- The debug and release builds give one answer on every hrule that reads `not`, and it is the oracle's or a refusal.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green, and the debug-build test suite runs the witness.
