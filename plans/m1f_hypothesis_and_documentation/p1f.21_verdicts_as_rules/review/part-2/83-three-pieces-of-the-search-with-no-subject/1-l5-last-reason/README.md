# 83 · 1 — L5's last reason: a payment that is no candidate

[← item 83](../README.md) · [piece 2, L9 →](../2-l9-nothing-writes-it/README.md) ·
[piece 3, the coupling →](../3-coupling-unlicensed-debts/README.md)

## The words

- **An owed instance.** An obligation the program activates, `(total-owed fed is-a)`, *every guest is fed some
  dish*, gives one instance for each guest root knows: *Ann owes a dish*. A state that holds no fact paying it owes
  it, and an owing state is no member of $R$.
- **Its witnesses, or payments**: the facts that would pay it, read at root off the obligation's guard. Ann's one
  payment is `(fed Ann Soup)`.
- **L5** ([the licences](../../../../../../../docs/ein/reasoning/02-kb-evolution/07-finding-r.md#4-a-monotone-program-three-properties-and-the-licences)):
  *an owed instance's alternatives are exhaustive*. Where it holds, every member of $R$ above a state that owes the
  instance holds one of its **alternatives**, the payments that are candidates, $\operatorname{alt}(I) = \operatorname{pay}(I) \cap H$.
  So the search can branch an owing state on those alternatives alone: the obligation tree, the route *the tree and
  the classes*.
- **What L5 needs.** Every payment is a candidate, or is denied at root (no alive state holds it). Then a state of
  $R$ that pays the instance does so by a candidate it holds, and the tree misses nothing.

**L5 refuses an instance for four reasons** (`ein-infer/src/oblgen.rs`, `alternatives`): its rule is the bare
`(open)`; its witness step does not resolve; its trigger or a guard read beside the witness is of a relation a
commitment can change; and **the last**: *a fact can witness it that is no candidate and that root does not deny*.
That last reason is this piece. Where any instance is refused, the route cannot branch on its debt, and a describing
run walks **every alive state once** (*the state route*), without L2 and L5.

<img src="runs/concept.svg" alt="one-way-to-eat before the union: the witness (fed Ann Soup) is no candidate and not denied, so L5 refuses and the search takes the state route; under the union the witness is a candidate, L5 licenses with the witness its one alternative, and the tree route branches on it">

*Drawn by hand (`../tools/concept-1.dot` → `runs/concept.svg`).*

## Why the reason is needed: a payment derived from candidates

A payment that is no candidate can still enter a state: a rule can derive it. If the search branched on the
candidates alone, it would never find a state that pays by derivation. The corpus's
[`13_witness_of_two.ein`](../../../../../../../examples/lattice/13_witness_of_two.ein) is the witness of this.
`(fed Ann Soup)` is derived from `(orders Ann)` alone, by `takeaway`, and from `(cooks Bob)` and `(shops Cat)`
together, by `dinner`. On the engine before the union, `{(cooks Bob), (shops Cat)}` is a member of $R$ that holds
none of the debt's candidates. L5 refused the instance, rightly, and the route walked every alive state: 5 states,
8 saturations (`runs/13_witness_of_two.old.rows.txt`).

## The simple example: one way to eat

[`programs/one-way-to-eat.ein`](programs/one-way-to-eat.ein) (and its twin for the engine before,
[`one-way-to-eat.old.ein`](programs/one-way-to-eat.old.ein)):

```lisp
(import std.algebra :symbols (total-owed))
(relation fed    Guest Dish :why "{?3} is fed {?4}")
(relation orders Guest      :why "{?3} orders a takeaway")
(is-a Ann Guest) (is-a Soup Dish)
(total-owed fed is-a)                                    ; Ann owes a dish
(rule takeaway () :match (orders Ann) :assert (fed Ann Soup) …)
(rule order    () :match (is-a ?g Guest) :assert (open (orders ?g)) …)   ; the one hypothesis rule
```

| | the engine before the union (`ddf09492`) | the final binary |
|---|---|---|
| H | 1: `(orders Ann)`, *built by the rung `hrules`* | 2: `(fed Ann Soup)`, `(orders Ann)`, *from `union`* |
| the witness `(fed Ann Soup)` | **no candidate, not denied**, held at root by `(orders Ann)`'s node | **a candidate**: the obligation wrote it |
| L5 | **refuses**: *a fact can witness it that is no candidate and that root does not deny* | licenses: one alternative, `(fed Ann Soup)` |
| the route | **the state route**, L1, L3, L4 | the tree and the classes, L1 to L5 |
| $R$ | 1 state: `{(orders Ann)}` | 2 states: `{(fed Ann Soup)}` and `{(fed Ann Soup), (orders Ann)}`, the second reached by 2 commitments |

From `runs/one-way-to-eat.old.rows.txt` and `runs/one-way-to-eat.new.rows.txt`. $R$ moves too: the union supposes
the payment outright, a meal with no one who orders, which [item 58](../../58-the-corpus-under-the-rulings/README.md)
lists with the corpus.

| the lattice before: every alive state once | the lattice since: the tree branches on the debt's alternative |
|---|---|
| <img src="runs/one-way-to-eat.old.lattice.svg" alt="the engine before the union visits the empty commitment, which owes, and {(orders Ann)}, which is in R"> | <img src="runs/one-way-to-eat.new.lattice.svg" alt="the final engine visits the empty commitment, {(fed Ann Soup)}, {(orders Ann)} and both; R is the states holding the witness"> |

The same pair for `13_witness_of_two`: `runs/13_witness_of_two.old.lattice.svg` and `.new.lattice.svg`. The regions
by `ein oracle` are `runs/*.region.svg`, and root's knowledge base is `runs/*.kb.svg`.

## Where the reason is still given: `(solve :no-search)`

[`programs/one-way-to-eat-no-search.ein`](programs/one-way-to-eat-no-search.ein) is the example stating
`(solve :no-search)`. H is empty by the program's word, so `(fed Ann Soup)` is no candidate, and `ein render h-space`
gives the last reason on the final binary: *1 instance L5 does not license, of the 1 root owes*
(`runs/one-way-to-eat-no-search.new.rows.txt`). Nothing is searched, so the reason steers nothing. The census of
the item's page finds 15 corpus programs of this kind, all under `tests/`.

## The code, and the tests that hold it today

| where | what |
|---|---|
| `ein-infer/src/oblgen.rs`: `alternatives` | L5's premise at load; the last of its four refusals |
| `oblgen.rs`: `witnesses` | the scan of one instance's witnesses: *a `R` fact can witness it that no candidate proposes*, or *`F` can witness it and is not a candidate* |
| `ein-infer/src/structure.rs`: `UNPROPOSED`, `WitnessIs::Unproposed` | the hypothesis space's reading of the same reason (`ein render h-space`) |

| test | what it asserts today |
|---|---|
| `ein-infer/tests/region.rs` `a_witness_two_candidates_derive_together_is_a_candidate_itself` | `13_witness_of_two.ein`: 4 candidates, its one instance licensed by L5, the witness `(fed Ann Soup)` its one alternative |
| `ein-infer/tests/unheld.rs` `a_witness_a_rule_writes_is_a_candidate_too` | under the union a witness a rule writes is a candidate: L5 licenses the debt, $R$ is five states |

Each was run on the final tree by S1f.21.19 (`../runs/test.*.txt`), and passes.
