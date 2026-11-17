# d3-goal-row-order

One model that satisfies a query's goal twice. The region read-out sorts a class's rows and prints every one, so
no row is chosen: a class whose states hold both lists `?x = A` and then `?x = B` under any id order. The witness's fixture is the
`regression` corpus entry
[`examples/ein-bugs/d3-goal-row-order.ein`](../../examples/ein-bugs/d3-goal-row-order.ein), which
`ein-render/tests/id_order_invariance.rs` permutes on every commit
([Q-M1e2.29](../../docs/decisions/corpus/Q-M1e2.29-a-witness-of-a-fixed-behaviour-is-a-corpus.md)). This note and
the `.ein` beside it stay: the `.ein` is a frontend-fuzzer seed, and the program below is the signature a later find
is compared against.

- property: **id-order** — the same answer under a permuted interner
- minimised: 11 → 7 forms, reached from two seeds and two mutation sources, one of them
  `examples/branching/13_lookahead_naf_world.ein`

```
(relation ok      T)
(relation blessed T)
(relation cand    T)
(rule promote ()
  :match  (and (ok ?x) (blessed ?x))
  :assert (cand ?x))
(ok      B)
(blessed A)
(query
  :goal (cand ?x))
```

## What it guards

The goal `(cand ?x)` is satisfied **twice** in one model: the blind enumerator proposes `(cand A)` and `(cand B)`,
and both survive. A table that printed one row, `rows[0]` of an unsorted match, would print a row the data does not
determine, and it would move when the interner assigns ids in another order, whether or not a fork resumes root's
saturation: an observable that depends on the order ids were assigned in, which
[the determinism contract](../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#21-ids-are-not-an-output-order)
forbids. The read-out prints every row, sorted, and `summary.json`'s `goal_bindings` carries both, sorted.

```sh
ein solve corpus/fuzz_findings/d3-goal-row-order.ein   # class 4 of 4 lists ?x = A, then ?x = B
```
