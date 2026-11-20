# fork-delta-order

Written by hand, not a fuzzer's find: the mechanism by which the core a dead node reports could move with the
interner. A resumed fork's delta is in the order its commitment was written, candidate order, and not in `FactId`
order, so N's one member, `{(on A), (on B)}`, reports `(g G08) (h H40) (on A) (on B)` under every permutation of the
id space, and nothing else a run reports moves either, narration included. The fixture is
[`examples/ein-bugs/fork-delta-order.ein`](../../examples/ein-bugs/fork-delta-order.ein), the witness of
regression `fork-delta-order` ([`corpus/regressions.toml`](../regressions.toml)), which the id-order sweep permutes
on every commit.

- property: **id-order** — the same answer under a permuted interner

```
(sw A) (sw B)          (g G40) … (g G01)          (h H40) … (h H01)
(rule mk-p () :match (and (on A) (g ?x)) :assert (p ?x))
(rule mk-q () :match (and (on B) (h ?y)) :assert (q ?y))
(rule clash () :match (and (p ?x) (q ?y)) :assert (false))
(rule guess () :match (sw ?s) :assert (open (on ?s)) …)
```

## What it guards

R is three states, ∅, `{(on A)}` and `{(on B)}`. N's one member reports a core in `--dump-answer`'s
`nogoods.json`, in `--dump-lattice`'s `nodes.json`, in `ein oracle --dump`'s `region.json` and in `--events`'
`enter` line. Every core of the node is four facts, so the size cannot move; which four could. `clash` derives
`(false)` 1 600 ways, each from two premises of the same closure size, and the store keeps the primary, the first 32
alternatives and the first smallest-closure arrival. Which arrive first is which side the fork derives first, and
that is the order `(on A)` and `(on B)` enter the resumed fork's delta: in `FactId` order, interning order, the
reported core would move when the ids do.

Over every derivation (an uncapped store) the core is `(g G01) (h H01) (on A) (on B)`. Which of the equal-size cores
a full list leaves is the store's bounded retention, `examples/ein-bugs/wrong-answers/alt-cap-shared.ein`'s class, a known gap; this row asks only that it not
move with the ids.
