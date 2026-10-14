# binding-key-layout

- found: 2026-08-28, by hand — M1e
  [S1e.1.4](../../docs/history/m1e_review_processing/README.md#s1e14--q-m1a8s-real-trigger-q3),
  answering the review's Q3, and written down as
  [`defined_behaviour.md` § 3.2](../../docs/kernel/defined_behaviour.md)'s
  thirteen-line program
- property: **no-crash** in a debug build; a **lost derivation** in release
- banked here: 2026-10-14, M1e2
  [S1e2.1.1](../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.1_the_case_register.md),
  as the witness of register row `W7/binding-key-layout`
  ([`corpus/wrong_answers.toml`](../wrong_answers.toml)), with
  [`binding-key-layout-swapped.ein`](binding-key-layout-swapped.ein) as its twin

```
(rule note (?R ?f)
  :match  (and (?R ?a ?b) (holds ?f))
  :assert (noted ?a ?f))
(edge 1 2) (edge 2 3)
(holds 1)  (holds 3)
(note edge 1)
(note edge (src Y))
```

## What happens

`BindingKey = (rule, activator, values)` compares two register layouts
position by position. An `int` activator argument binds its parameter and a
nested `Fact` does not, so `(note edge 1)` and `(note edge (src Y))` compile
two plans with **different** layouts under one key, and the second plan's
bindings are deduplicated against the first's.

```
$ ein saturate --dump corpus/fuzz_findings/binding-key-layout.ein        # release
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
  (noted 2 3 :rule note)
$ ein saturate --dump corpus/fuzz_findings/binding-key-layout-swapped.ein
  (noted 1 1 :rule note)
  (noted 1 3 :rule note)
  (noted 2 1 :rule note)
  (noted 2 3 :rule note)
```

The rule has four conclusions; the activator order decides whether one is
lost, and nothing says so. In a debug build `check_layout`
(`ein-infer/src/engine.rs`) is the only enforcement of the key's invariant and
aborts on **both** files:

```
panicked at crates/ein-infer/src/engine.rs:254:21:
assertion `left == right` failed: two plans share (rule, activator) and disagree on their register layout — `BindingKey` compares their value vectors and would be comparing different variables
```

## Why it is here rather than in `examples/`

Every in-process corpus sweep (`corpus_shapes`, `id_order_invariance`,
`jobs_invariance`, `summary_properties`) saturates every corpus file inside a
debug test binary, and this one aborts it. A register row may not point into a
plan directory, and this is the directory the repo keeps for a minimised input
that violates a property the corpus sweeps assert — so the pair lives here
until [S1e2.2.7](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.7_binding_key_layout.md)
fixes it, and moves to `examples/` in the same commit, as the README's rule for
a fixed find says. The in-crate statement of the same program is
`ein-infer/tests/rule_semantics.rs::an_int_beside_a_nested_fact_in_one_position_loses_a_derivation`.
