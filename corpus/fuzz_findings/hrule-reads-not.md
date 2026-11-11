# hrule-reads-not

- found: 2026-08-21, `utils/fuzz_engine.py --seed 11 --iters 100`, mode `mixed`,
  the session that re-opened the fuzzer after
  [S1a.10.4](../../docs/history/m1a_rust/README.md#s1a104--utils-and-the-fuzzer)
  T1a.10.4.2 rewrote it around the properties it checks
- property: **no-crash** — a generated program exits 0, 1 or 2, never a panic
- minimised: 19 → 1 form (a second and a third minimum with different
  relation names reached the same site and were deduped)

```
(hrule hyp-0 ()
  :match  (and (r0 ?v0 ?v1) (absent (not (r0 ?v1 ?v2))))
  :assert (not (r1 ?v0 ?v0)))
```

**Settled 2026-11-11 by M1f
[S1f.7.2](../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.2_an_hrule_that_reads_not.md)**
(W85): the release answer is right, it is `ein oracle`'s, and the assertion is
gone, so a debug build answers too. No command enumerates with the kill cache
on since M1e2 S1e2.3.1, and where an hrule reads `not` every hrule is matched
before any candidate is emitted, so the cache's writes are not visible with it
on either. The regression entries are
`examples/ein-bugs/wrong-answers/hrule-reads-{absent-not,not,unknown}.ein`,
and `ein-cli/tests/hrule_reads_not.rs` runs them, and this file's program,
under `cargo test`'s debug assertions. What follows is the record of the
finding as it stood.

## What happened

```
thread 'no_observable_depends_on_the_order_ids_were_assigned_in' panicked at crates/ein-infer/src/hrule.rs:130:13:
an hrule reads `not`, which hypgen's kill cache writes mid-enumeration — see `Hrules::candidates`
```

(Verbatim, because `utils/fuzz_engine.py` dedups a panic on **its site and
message** and seeds that set from these notes — a note that paraphrases the
abort gets the same finding re-filed every session.)

The site is the line the assertion is on today (`ein.rs/target/debug/ein solve
corpus/fuzz_findings/hrule-reads-not.ein`, 2026-11-10, M1f S1f.5.28); it was
`hrule.rs:113:13` when the note was written, and a site that moves is a
finding re-filed, so the line is kept current here.

It reproduced both ways until S1f.7.2:

```sh
# release: answers, exit 0 — the assertion is compiled out
# (`--max-set-size 2` until M1e2 S1e2.1a.5's switch retired the walk's options)
ein solve corpus/fuzz_findings/hrule-reads-not.ein

# debug: aborts on the assertion
ein.rs/target/debug/ein solve corpus/fuzz_findings/hrule-reads-not.ein
EIN_ID_FILES=$PWD/corpus/fuzz_findings \
  cargo test --manifest-path ein.rs/Cargo.toml -p ein-render \
             --test id_order_invariance
```

## Why it is here rather than fixed

`Hrules::candidates` documents its own precondition: the matcher borrows the
KB immutably for the whole walk and finishes it before hypgen's filter
pipeline runs, so a `not` fact the kill cache writes is not visible to the
rest of that enumeration. **That cannot change an answer whenever the only
relation the pipeline writes — `not` — is not one an hrule's `:match`
reads**, and the `debug_assert` is that condition, justified by "no corpus
hrule reads it".

The generator wrote one. `(not …)` and `(absent (not …))` premises are
ordinary ein-lang — `examples/features/01_not_and_absent.ein` is a fixture for
them — so this is a **legal program the engine asserts against in a debug
build and answers in a release build**, and nobody has said which answer is
right. That is a semantics question, not a `utils/` clean-up: a possible
wrong answer on a program shape nobody wrote a fixture for, and this one has a
fixture now.

Not a corpus entry yet: an entry is a program with a *settled* expectation,
and this one's expectation is the open question. When it settles it becomes a
`regression` entry in the same commit, and this file goes.
