# hrule-reads-not

A hypothesis rule, a rule that concludes `(open F)`, whose `:match` reads the `not` relation, a generated program
minimised to one form. `(not …)` and `(absent (not …))` premises are ordinary ein-lang
(`examples/features/01_not_and_absent.ein` is a fixture for them), so this is a legal program, and every command
answers it as `ein oracle` does, in a release build and a debug one: each hypothesis rule is matched as it is reached
and nothing is written between two of them (Q-M1f.590), so no `not` fact the hypothesis pipeline writes is read
mid-enumeration. The regressions `hrule-reads-absent-not`, `hrule-reads-not` and
`hrule-reads-unknown` hold it, their witnesses `examples/ein-bugs/wrong-answers/hrule-reads-{absent-not,not,unknown}.ein`, and `ein-cli/tests/hrule_reads_not.rs`
runs them, and this file's program, under `cargo test`'s debug assertions.

- property: **no-crash** — a generated program exits 0, 1 or 2, never a panic
- minimised: 19 → 1 form (a second and a third minimum with different relation names reached the same site),
  found as the form the language then had for a hypothesis rule and written today as the rule it is (Q-M1f.567)

```
(rule hyp-0 ()
  :match  (and (r0 ?v0 ?v1) (absent (not (r0 ?v1 ?v2))))
  :assert (open (not (r1 ?v0 ?v0))))
```

## What it guards

The hypothesis layer finishes each rule's walk of its matches before it emits that rule's first candidate, and the
pass that takes H writes nothing between two rules. A `not` fact a filter wrote while candidates were still being
emitted would not be visible to the rest of that enumeration, which cannot change an answer only while no hypothesis
rule's `:match` reads `not`. This program is one that does, and nothing written between two rules is what keeps its
answer the oracle's.

The panic below, its site and its message verbatim as the build that found it printed them, is this find's
signature, which a later find is deduplicated
against; a paraphrase would get the same find filed again:

```
thread 'no_observable_depends_on_the_order_ids_were_assigned_in' panicked at crates/ein-infer/src/hrule.rs:130:13:
an hrule reads `not`, which hypgen's kill cache writes mid-enumeration — see `Hrules::candidates`
```

```sh
ein solve corpus/fuzz_findings/hrule-reads-not.ein   # answers, exit 0, in a release and a debug build
```
