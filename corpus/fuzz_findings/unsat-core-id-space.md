# unsat-core-id-space

A generated program, minimised to four forms, whose reported core could depend on the order ids were assigned in.
Under R it is three states, and its four minimal dead sets have one core each, none of which moves under a permuted
interner: `explain.rs` reports the smallest frontier, then the least by `repr` rank, never by id, and what the store
is offered, a resumed fork's delta, is in the order the commitment was written
([`fork-delta-order`](fork-delta-order.md)). The fixture is
[`examples/ein-bugs/unsat-core-id-space.ein`](../../examples/ein-bugs/unsat-core-id-space.ein), which the id-order
sweep permutes on every commit; this note and its `.ein` stay as the signature a later find is compared against and
a frontend-fuzzer seed.

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

## What it guards

A contradiction can have several minimal cores, and the engine is free to report any of them, but *which* one it
reports must be a function of the program and not of interning order: an observable that depends on the order ids
were assigned in is what
[the determinism contract](../../docs/ein/ein.rs/design/determinism-and-order.md#21-ids-are-not-an-output-order)
forbids. A core is an explanation, and canonical only up to that contract.

A symmetric program reaches the same question with no generated input: a blind pigeonhole, three pigeons and two
holes, whose contradiction has an automorphism group, offers equal-size cores that differ only by which pigeon is
which. `examples/features/12_expect_false.ein` **forces its chain instead of searching it** — two givens, survivor
forcing, one core — so that a fixture about `:expect (false)` is not also a symmetry stress test.

```sh
ein solve corpus/fuzz_findings/unsat-core-id-space.ein   # R three states; N four minimal dead sets, one core each
```
