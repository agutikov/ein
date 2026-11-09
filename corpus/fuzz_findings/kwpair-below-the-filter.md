# kwpair-below-the-filter

**Fixed 2026-11-03, M1e2
[S1e2.3.2](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.2_a_keyword_pair_panics_two_views.md),
commit `38b2bff3`**, on the user's ruling (a), written down in
[`defined_behaviour.md` § 2.5](../../docs/kernel/defined_behaviour.md#25-a-render-view-labels-an-ir-form-as-written-a-keyword-pair-as-key-value): `dot_util::value_label` labels a keyword pair
`:key value` for every caller, the spelling `ir_dot`'s `(query …)` note already
used, so both views below draw each `(step …)` as written and exit 0. The
witness was banked as the `regression` corpus entry
`examples/ein-bugs/kwpair-below-the-filter.ein`, whose `render constraints`
and `render program` runs `corpus_cli.rs` held at exit 0; `corpus_shapes.rs`
and `dot_wellformed.rs` ran every view over it.

**Refused at the parse since M1f S1f.5.5's swap (T1f.5.5.4, 2026-11-08)**, at
its first `:rule`: no keyword pair parses inside a term or a nested fact,
S1f.5.11's ruling (its default; Q-M1e2.28). The program that reached
`value_label`'s keyword-pair arm is no program now, a parse error rather than a
load one, so the witness moved to
[`examples/broken/kwpair-below-the-filter.ein`](../../examples/broken/kwpair-below-the-filter.ein),
a `parse-negative` entry whose `.expected` is the report, and every run of it
exits 1. The arm went with S1f.5.11's T1f.5.11.2 (2026-11-09), shown unreachable
first: no value holds a keyword pair, and `value_label` holds the arm
`unreachable!`, naming the grammar's productions. The `.ein` beside this file
stays a fuzz seed, its recorded outcome the parse refusal (`fuzz_seeds.txt`). The
second door, `render program`, arrived at M1e2 S1e2.1.3, after the finding.
This file and the `.ein` beside it stay: the panic signature below is what
`utils/fuzz_engine.py` dedups a recurrence on, and the `.ein` is one of the
frontend fuzzer's seeds.

- found: 2026-08-23, `utils/fuzz_engine.py --seed 20260823 --iters 5000
  --no-id-order --jobs 8`, mode `mixed` — the
  [T1a.7.2.6](../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)
  stress session, whose subject was the `jobs` property. This is the only
  thing 25 000 runs turned up, and it is not about `--jobs`
- property: **no-crash** — a generated program exits 0, 1 or 2, never a panic
- minimised: 2 → 1 form, mutated from `examples/syntax/trace-steps.ein`
  (`trace` → `tracex`)

```
(tracex (step s1 :rule from-condition :using (c10) :derives (lives-in N H1))
  (step s2 :rule adjacent :using (and s1 c15) :derives (color-loc Blue H2))
  (step s3 :using (s2))
  (step s4 :derives (final X)))
```

## What happens

```
thread 'main' (3482912) panicked at crates/ein-render/src/dot_util.rs:152:32:
not a value node: KwPair
```

(Verbatim, because `utils/fuzz_engine.py` dedups a panic on **its site and
message** and seeds that set from these notes.)

```sh
ein render constraints corpus/fuzz_findings/kwpair-below-the-filter.ein   # exit 101
ein saturate            corpus/fuzz_findings/kwpair-below-the-filter.ein   # exit 0
```

The second line is the finding's other half: the loader **accepts** this
program. `tracex` is not a kernel head and not a declared relation, so
`constraints.rs` treats the form as an ontology declaration and labels its
arguments — and `value_label` panics rather than labelling.

## Why it panics, and why the filter did not catch it

`constraints.rs` already filters keyword pairs out of a declaration's
arguments:

```rust
.filter(|a| !matches!(ast.node(*a), Node::KwPair { .. }))
.map(|a| value_label(ast, a))
```

but the filter is one level deep and `value_label` recurses. Each `(step s1
:rule … :using … :derives …)` is an `SForm` whose *own* arguments are keyword
pairs, and the recursion reaches them with nothing between.

## Why it is here rather than fixed

Because there are two defensible answers and nothing has chosen one.
`value_label` panics on a keyword pair by design, and nothing says why a
renderer should abort on a program the loader accepts. The two candidates
are:

- **render it** — `ir_dot.rs:339` already spells a keyword pair as
  `:{key} {value}`, so the crate has the rendering; or
- **filter it recursively** — which is what the one-level filter above says
  the constraints view thinks a keyword pair is worth.

They differ in what `render constraints` *shows*, which makes this a
defined-behaviour question rather than a crash to paper over, and
[`docs/kernel/defined_behaviour.md`](../../docs/kernel/defined_behaviour.md)
is where the answer would be written down. Whichever wins, the fix is small
and this file becomes a `regression` corpus entry in the same commit.

Not a `--jobs` finding, and worth saying so plainly: it is reached through
`render constraints`, which takes no `--jobs`, and the `jobs` property was
green on all 10 000 of the session's paired runs.
