# 83 · 2 — L9: a payment nothing proposes and nothing writes is in no state

[← item 83](../README.md) · [← piece 1, L5's last reason](../1-l5-last-reason/README.md) ·
[piece 3, the coupling →](../3-coupling-unlicensed-debts/README.md)

## What it is

Piece 1 refuses an instance when one of its payments is neither a candidate nor denied at root, because a rule could
derive that payment in some state. **L9 rescues the instances where no rule can**
([the licences](../../../../../../../docs/ein/reasoning/02-kb-evolution/07-finding-r.md#4-a-monotone-program-three-properties-and-the-licences)).
A fact enters a state in only three ways: it is committed (a candidate), it is root's, or a rule concludes it.
A payment that is no candidate, not in root, and of a relation **no rule concludes a fact of** enters no state. So it
can be dropped, and where every other payment is a candidate or denied, L5's conclusion holds after all, with the
alternatives the candidates among the payments:

$$
w \notin H,\; w \notin \mathrm{root},\; \text{no rule concludes a fact of } w\text{'s relation} \;\implies\; \forall C.\; w \notin \operatorname{st}(C)
$$

*No rule concludes* is read off the program's rules, the plans root compiled, and not off a run: a rule that could
write the relation and never fires still counts. L5's verdict and its reason stay as they were; the route takes
L9's licence beside them, and its row names L9.

<img src="runs/concept.svg" alt="Ann owes a seat; (sits Ann S1) is a candidate and (sits Ann S2) is neither, so L5 refuses; if a rule could conclude a sits fact, L9 declines and the search takes the state route; if none can, S2 enters no state and L9 licenses with S1 the one alternative; under the union S2 is a candidate and L9 is not asked">

*Drawn by hand (`../tools/concept-2.dot` → `runs/concept.svg`).*

## The simple example: a seat nothing proposes

[`programs/a-seat-nothing-proposes.ein`](programs/a-seat-nothing-proposes.ein) (twin for the engine before:
[`a-seat-nothing-proposes.old.ein`](programs/a-seat-nothing-proposes.old.ein)):

```lisp
(import std.algebra :symbols (functional total-owed))
(relation sits   Guest Seat :why "{?3} sits in seat {?4}")
(relation window Seat       :why "seat {?3} is by the window")
(is-a Ann Guest) (is-a S1 Seat) (is-a S2 Seat)
(window S1)
(functional sits)                                         ; one seat each
(total-owed sits is-a)                                    ; Ann owes a seat: S1 or S2 pays
(rule seat () :match (and (is-a ?g Guest) (window ?s)) :assert (open (sits ?g ?s)) …)   ; proposes S1 only
```

| | the engine before the union | the final binary |
|---|---|---|
| H | 1: `(sits Ann S1)` | 2: `(sits Ann S1)`, `(sits Ann S2)`, *from `union`* |
| `(sits Ann S2)` | no candidate, not denied, not in root; **no rule concludes `sits`** | a candidate: the obligation wrote it |
| L5 | refuses (its last reason) | licenses: a group, both seats |
| **L9** | **licenses**: alternatives `{(sits Ann S1)}` | not asked |
| the route | the tree and the classes, L1 to L5 **and L9** | the tree and the classes, L1 to L5 |
| $R$ | 1 state: `{(sits Ann S1)}` | 2 states: `{(sits Ann S1)}`, `{(sits Ann S2)}` |

From `runs/a-seat-nothing-proposes.old.rows.txt` and `.new.rows.txt`. $R$ moves with the union: S2 is supposed too.

## L9's limit: a rule that could write the payment

[`programs/a-seat-a-rule-could-write.ein`](programs/a-seat-a-rule-could-write.ein) adds one rule that would seat a
guest of honour in S2, and nothing says Ann is one:

```lisp
(relation vip Guest :why "{?3} is a guest of honour")
(rule honour () :match (vip Ann) :assert (sits Ann S2) …)   ; never fires
```

On the engine before the union **L9 declines**: `sits` is a relation a rule concludes, though no run derives a fact
of it, so the route falls back to **the state route**, L1, L3, L4 (`runs/a-seat-a-rule-could-write.old.rows.txt`).
On the final binary the union makes S2 a candidate, and the program answers as the one above.

| before, nothing writes `sits`: the tree, by L9 | before, `honour` could: the state route | the final binary |
|---|---|---|
| <img src="runs/a-seat-nothing-proposes.old.lattice.svg" alt="the engine before the union, the tree route by L9: the empty commitment owes, {(sits Ann S1)} is in R"> | <img src="runs/a-seat-a-rule-could-write.old.lattice.svg" alt="the engine before the union, the state route: every alive state once"> | <img src="runs/a-seat-nothing-proposes.new.lattice.svg" alt="the final engine: both seats are candidates, each alone in R, both together dead"> |

## The corpus program that was its subject

[`14_seats_nothing_proposes.ein`](../../../../../../../examples/lattice/14_seats_nothing_proposes.ein): four friends,
two tables of two seats, the pair (Ann, Bob) fits both, the duo (Cat, Dan) only the big one, and everyone owes a
seat. The hypothesis rule proposes a friend only the seats of a table their group fits.

| | the engine before the union | the final binary |
|---|---|---|
| H | 12, *built by the rung `hrules`* | 16, *from `union`*: Cat's and Dan's small-table seats among them |
| instances L5 does not license | **2 of the 4**: Cat's and Dan's, each with `(sits Cat S1)`, `(sits Cat S2)` (and Dan's) no candidate, not denied, held by no node | 0 of 4 |
| the route | the tree and the classes, L1 to L5 **and L9** | the tree and the classes, L1 to L5 |
| $R$ | 4 states: the pair at the small table, the duo at the big one | 8 states: also the duo at the small table, since the union supposes those seats too |
| saturations · facts beyond root | 69 · 226 | 149 · 556 |

From `runs/14_seats_nothing_proposes.old.rows.txt` and `.new.rows.txt`; the lattices and regions beside them. On the
engine before, `06_friends_tables_everyone_sits.ein` named L9 too; on the final binary no corpus route does (the
census on the item's page).

## The code, and the tests that hold it today

| where | what |
|---|---|
| `ein-infer/src/unheld.rs`: `Unwritten::of`, `is_unwritten` | the relations no plan root compiled writes |
| `unheld.rs`: `breach`, `refusal` | the premise read at a node, and the run refused where it fails |
| `unheld.rs`: `without`, `trusting` | two switches only tests reach |
| `ein-infer/src/oblgen.rs`: `Instance::unheld`, `Unheld`, `Alternatives::with_l9`, `by_l9`, `unheld_relations`; the scan in `witnesses`, reading past L5's refusal while each such witness is of an unwritten relation | L9's licence of an instance L5 refuses for its last reason alone |
| `ein-infer/src/region.rs`: `NodeOut::unheld`; `ein-infer/src/explore.rs`: `Licence::Unheld` | the node's check, and the route row's words |

| test | what it asserts today |
|---|---|
| `ein-infer/tests/unheld.rs` `l9_licenses_where_nothing_writes_the_witness_relation` | over every corpus program and regression witness, the instances L9 licenses: **the list is empty** |
| `unheld.rs` `l9_is_the_oracle_on_every_monotone_program_it_reaches` | `programs == 0 && nodes_read == 0 && owing == 0`; it prints `L9: 0 programs, 0 nodes, 0 owed instances L9 licenses` |
| `unheld.rs` `every_question_answers_alike_with_l9_and_without` | returns at once where no corpus program has an instance L9 licenses: today it reads nothing |
| `unheld.rs` `a_witness_a_rule_writes_is_a_candidate_too` | under the union a witness a rule writes is a candidate; under `unheld::trusting` every question answers as without it |

Each was run on the final tree by S1f.21.19 (`../runs/test.unheld.txt`), and passes.
