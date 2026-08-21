# unsat-core-id-space

- found: 2026-08-21, `utils/fuzz_ein.py --seed 20260821 --minutes 4`, mode
  `mixed`, by the rewritten fuzzer
  ([S1a.10.4](../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md))
- property: **id-order** — the same answer under a permuted interner
- minimised: 19 → 4 forms

```
(relation r1 T T)
(is-a o1 T)
(r1 o4 o5)
(rule fire-2 ()
  :match  (or (and (r1 ?v0 ?v1) (r1 ?v4 ?v0)) (and (r1 ?v0 ?v1) (r1 ?v0 ?v1) (not (r1 ?v2 ?v3))))
  :assert (false))
```

## What happens

The verdict is `Contradiction` either way. **Which facts the unsat core names,
and how many, is not determined by the data** — six of the 45 rendering ops
move under one permutation:

```
[solve[default]] / [solve[exhaustive]] / [solve[shuffled]]
  plain:    CORE 1 [(r1 o4 o5)]
  permuted: CORE 2 [(r1 o1 o5) (r1 o4 o1)]

[trace[answer]]
  plain:    No solution — the constraints are contradictory (unsat core: 1 facts).
  permuted: No solution — the constraints are contradictory (unsat core: 2 facts).

[dot[lattice]] / [dot[lattice-full]]
  tooltip="unsat-core: r1(o4, o5) …"  vs  tooltip="unsat-core: r1(o1, o5) …"
```

Reproduce:

```sh
mkdir -p /tmp/unsat-core && cp corpus/fuzz_findings/unsat-core-id-space.ein /tmp/unsat-core/
EIN_ID_FILES=/tmp/unsat-core cargo test --manifest-path ein.rs/Cargo.toml \
    -p ein-render --test id_order_invariance
```

## Why it is here rather than fixed

It is the class
[design/02](../../plans/m1a_rust/design/02_determinism_and_order.md) forbids —
an observable that depends on the order ids were assigned in — and it is the
second one the id-space sweep has found that the corpus does not reach (the
first is [`d3-goal-row-order`](d3-goal-row-order.md)). Both arrived within
minutes of `utils/fuzz_ein.py` being pointed at that sweep, which is the
argument for pointing it there.

Same *surface* as [`d3-unsat-core`](d3-unsat-core.md) and a different cause:
that one's 6-vs-4 core is the resumed fork saturator (`EIN_FORK_DELTA=0`
reproduces the six), and it is **green** under a permuted id space. This one
has one engine, one build, and two answers.

The engine is not wrong to have more than one minimal core — a contradiction
can have several — but it is wrong for *which* one it reports to be a function
of interning order rather than of the program. Whether the fix is a canonical
choice (smallest, then `Interner::rank`-least) or reporting every core is a
decision nobody has taken, which is why this is a fixture and a question. When
it settles it becomes a `regression` entry in the same commit and this file
goes.
