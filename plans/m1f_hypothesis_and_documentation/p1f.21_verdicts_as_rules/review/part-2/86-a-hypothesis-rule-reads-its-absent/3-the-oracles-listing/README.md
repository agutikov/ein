# 86 · 3 — the oracle's listing of a node (default 3, Q-M1f.687)

[← item 86](../README.md) · [← part 1](../1-the-absent-is-read/README.md) · [← part 2](../2-a-read-under-absent-flags-nothing/README.md) ·
[part 4 →](../4-the-witness-and-its-decision/README.md)

## The question

`ein oracle FILE --dump DIR --states` keeps every node's state, as text, in `DIR/region.json`. To see part 2, that
listing must show the `open` facts a node holds. Two things make that need a choice:

- **A fork does not inherit root's `open` layer.** Each node's `open` facts are written by the pass at its own
  fixpoint (Q-M1f.655). So "the facts beyond root's" would leave out every `open` fact a node holds that root also
  held, and part 2's point, *this node no longer holds `(open (culprit Ann))`*, would not show at all.
- **`-P`** is the flag that leaves the `open` and `(not …)` facts out of the states of $R$ a solve lists.

**The default:** a node's listing is **its facts beyond root's and every `open` fact it holds**, root's node
included. `-P` keeps leaving them out of the states of $R$ it lists, and no positive form of `--states` is added.
**The other readings:** the listing keeps to what is beyond root, an `open` fact listed only where root does not hold
it; or `-P` beside `--states` drops the `open` and `(not …)` facts from every node's listing.

<img src="runs/concept.svg" alt="the node {(alibi Ann)} is a fork of root that does not inherit root's open layer; it holds its own facts beyond root's, (alibi Ann), and its own open layer written by the pass at its fixpoint; its listing, the default, is both, root's node listing its open facts too; before the fix a listing held no open fact">

*Drawn by hand (`../tools/concept-3.dot` → `runs/concept.svg`).*

## Before and since, on part 2's program

`ein oracle alibi-supposed.ein --dump DIR --states`, each node's commitment and the `open` facts its listing holds
(`runs/alibi-supposed.pre.nodes.txt`, `runs/alibi-supposed.new.nodes.txt`; the dumps' own pages,
`runs/alibi-supposed.*.region.md`):

| the node | before the fix | since |
|---|---|---|
| ∅ (root) | *no `open` fact listed* | `(open (alibi Ann))` `(open (alibi Bob))` `(open (culprit Ann))` `(open (culprit Bob))` |
| `{(alibi Ann)}` | *no `open` fact listed* | `(open (alibi Ann))` `(open (alibi Bob))` `(open (culprit Bob))` |
| `{(alibi Bob)}` | *no `open` fact listed* | `(open (alibi Ann))` `(open (alibi Bob))` `(open (culprit Ann))` |
| `{(alibi Ann), (alibi Bob)}` | *no `open` fact listed* | `(open (alibi Ann))` `(open (alibi Bob))` |

Before the fix no listing held an `open` fact, root's node included (V-91, fixed with this default by `1f19de58`).
Since, each node shows what its own pass wrote. Under the first other reading, the node `{(alibi Ann)}` would list
nothing of `open`, since root holds all three of its `open` facts, and the one that is gone would not show either.

## `-P` beside it

`ein solve FILE -P` prints, after the read-out, each state of $R$ it showed, positive: each is headed *final-state
facts (beyond the program as loaded, (not …) and (open …) omitted: root's derivations, the commitment and what it
derives; N facts)* (`../2-a-read-under-absent-flags-nothing/runs/alibi-supposed.new.solve-P.txt`). The default keeps
that, and adds no positive form of `--states`, so a node's listing in the oracle's dump always holds its `open`
facts.
