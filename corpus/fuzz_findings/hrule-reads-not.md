# hrule-reads-not

- found: 2026-08-21, `utils/fuzz_ein.py --seed 11 --iters 100`, mode `mixed`,
  the session that re-opened the fuzzer after
  [S1a.10.4](../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md)
  T1a.10.4.2 rewrote it around single-engine properties
- property: **no-crash** — a generated program exits 0, 1 or 2, never a panic
- minimised: 19 → 1 form (a second and a third minimum with different
  relation names reached the same site and were deduped)

```
(hrule hyp-0 ()
  :match  (and (r0 ?v0 ?v1) (absent (not (r0 ?v1 ?v2))))
  :assert (not (r1 ?v0 ?v0)))
```

## What happens

```
thread 'no_observable_depends_on_the_order_ids_were_assigned_in' panicked at crates/ein-infer/src/hrule.rs:113:13:
an hrule reads `not`, which hypgen's kill cache writes mid-enumeration — see `Hrules::candidates`
```

(Verbatim, because `utils/fuzz_ein.py` dedups a panic on **its site and
message** and seeds that set from these notes — a note that paraphrases the
abort gets the same finding re-filed every session.)

Reproduce, both ways:

```sh
# release: answers, exit 0 — the assertion is compiled out
ein solve corpus/fuzz_findings/hrule-reads-not.ein --max-set-size 2

# debug: aborts on the assertion
EIN_ID_FILES=$PWD/corpus/fuzz_findings \
  cargo test --manifest-path ein.rs/Cargo.toml -p ein-render \
             --test id_order_invariance
```

## Why it is here rather than fixed

`Hrules::candidates` documents its own precondition: ein.py's hypothesis
enumerator writes into the live `_facts_by_relation` list, so a `not` fact the
kill cache writes mid-enumeration is visible to the rest of that enumeration;
ein.rs borrows the KB immutably for the whole walk and finishes it first. **The
two agree exactly whenever the only relation the pipeline writes — `not` — is
not one an hrule's `:match` reads**, and the `debug_assert` is that condition,
justified by "no corpus hrule reads it".

The generator wrote one. `(not …)` and `(absent (not …))` premises are
ordinary ein-lang — `examples/features/01_not_and_absent.ein` is a fixture for
them — so this is a **legal program the engine asserts against in a debug
build and answers in a release build**, and nobody has said which answer is
right. That is a semantics question, not a `utils/` clean-up, and it is
exactly the shape [accepted loss L1](../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#6-accepted-loss)
names: a wrong answer on a program shape nobody wrote a fixture for. The
difference is that this one has a fixture now.

Not a corpus entry yet: an entry is a program with a *settled* expectation,
and this one's expectation is the open question. When it settles it becomes a
`regression` entry in the same commit, and this file goes.
