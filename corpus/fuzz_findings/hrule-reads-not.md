# hrule-reads-not

An `(hrule …)` whose `:match` reads the `not` relation, a generated program minimised to one form. `(not …)` and
`(absent (not …))` premises are ordinary ein-lang (`examples/features/01_not_and_absent.ein` is a fixture for
them), so this is a legal program, and every command answers it as `ein oracle` does, in a release build and a debug
one: where an hrule reads `not`, every hrule is matched before any candidate is emitted, so no `not` fact the
hypothesis pipeline writes is read mid-enumeration (regression W85). The regression entries are
`examples/ein-bugs/wrong-answers/hrule-reads-{absent-not,not,unknown}.ein`, and `ein-cli/tests/hrule_reads_not.rs`
runs them, and this file's program, under `cargo test`'s debug assertions.

- property: **no-crash** — a generated program exits 0, 1 or 2, never a panic
- minimised: 19 → 1 form (a second and a third minimum with different relation names reached the same site)

```
(hrule hyp-0 ()
  :match  (and (r0 ?v0 ?v1) (absent (not (r0 ?v1 ?v2))))
  :assert (not (r1 ?v0 ?v0)))
```

## What it guards

`Hrules::candidates` borrows the KB for the whole walk of the hrules' matches and finishes it before the hypothesis
pipeline's filters run. A `not` fact a filter writes while candidates are still being emitted would not be visible
to the rest of that enumeration, which cannot change an answer only while no hrule's `:match` reads `not`. This
program is an hrule that does, and matching every such hrule first is what keeps its answer the oracle's.

The panic below, its site and its message verbatim, is this find's signature, which a later find is deduplicated
against; a paraphrase would get the same find filed again:

```
thread 'no_observable_depends_on_the_order_ids_were_assigned_in' panicked at crates/ein-infer/src/hrule.rs:130:13:
an hrule reads `not`, which hypgen's kill cache writes mid-enumeration — see `Hrules::candidates`
```

```sh
ein solve corpus/fuzz_findings/hrule-reads-not.ein   # answers, exit 0, in a release and a debug build
```
