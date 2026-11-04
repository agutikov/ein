# fork-delta-order

**Fixed 2026-11-04** by M1e2 S1e2.2.12 (T1e2.2.12.2, `10858b96`): a resumed fork's
delta is in the order its commitment was written, so N's member reports
`(g G08) (h H40) (on A) (on B)` under every permutation, and nothing else a
run reports moves either, narration included. Promoted the same day to
[`examples/ein-bugs/fork-delta-order.ein`](../../examples/ein-bugs/fork-delta-order.ein),
which the register's row points at and the id-order sweep permutes per
commit; this file stays as the finding's record. The note below is the
finding as it stood.

- found: 2026-11-04, by hand — M1e2
  [S1e2.2.12](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md)'s
  T1e2.2.12.1, the second of the user's two visits to W11, from the mechanism
  its eight-seed id-order sweep found; not a fuzzer's find
- property: **id-order** — the same answer under a permuted interner
- banked here: as the witness of register row `W11/fork-delta-order`
  ([`corpus/wrong_answers.toml`](../wrong_answers.toml)), `open`, because as a
  corpus entry it would fail `id_order_invariance` at its first seed

```
(sw A) (sw B)          (g G40) … (g G01)          (h H40) … (h H01)
(rule mk-p () :match (and (on A) (g ?x)) :assert (p ?x))
(rule mk-q () :match (and (on B) (h ?y)) :assert (q ?y))
(rule clash () :match (and (p ?x) (q ?y)) :assert (false))
(hrule guess () :match (sw ?s) :assert (on ?s) …)
```

## What happens

R is three states, ∅, `{(on A)}` and `{(on B)}`, and does not move. N's one
member, `{(on A), (on B)}`, reports a core in `--dump-answer`'s
`nogoods.json`, in `--dump-lattice`'s `nodes.json`, in `ein oracle --dump`'s
`region.json` and in `--events`' `enter` line, and that core moves with the
interner (`EIN_ID_SEEDS=8`, at `b364c11f`):

```text
as interned                       (g G08) (h H40) (on A) (on B)
permuted, seeds 1, 15839, 23758   (g G40) (h H08) (on A) (on B)
```

Every core of the node is four facts, so the size never moves; which four
does. `clash` derives `(false)` 1 600 ways, each two premises of the same
closure size, and the store keeps the primary, the first 32 alternatives and
the first smallest-closure arrival. Which arrive first is which side the fork
derives first, and that was the order `(on A)` and `(on B)` entered the
resumed fork's delta: `Snapshot::new_facts_of` sorted it by `FactId`, which is
interning order.

## Why it is a register row

It is W11, *which core is reported depends on interner id order*, after the
walk: no corpus program's core moved on the second visit, and the one place
the id order reached a fork, the delta's sort, could move one. The row stays
`open` until the delta is in an order the program decides, and the file is
promoted to `examples/ein-bugs/` in the commit that fixes it.

Over every derivation (an uncapped store) the core is `(g G01) (h H01) (on A)
(on B)`. Which of the equal-size cores a full list leaves is the store's
bounded retention, `W10/alt-cap-shared`'s class, deferred to F22; this row
asks only that it not move with the ids.
