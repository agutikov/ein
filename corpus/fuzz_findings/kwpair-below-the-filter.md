# kwpair-below-the-filter

- found: 2026-08-23, `utils/fuzz_ein.py --seed 20260823 --iters 5000
  --no-id-order --jobs 8`, mode `mixed` — the
  [T1a.7.2.6](../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md)
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

(Verbatim, because `utils/fuzz_ein.py` dedups a panic on **its site and
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
`value_label` says it panics "as ein.py's `TypeError` does" — a rationale
whose subject was deleted at
[S1a.10.5](../../plans/m1a_rust/p1a.10_single_implementation/README.md), so
what used to be parity is now just an abort. The two candidates are:

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
