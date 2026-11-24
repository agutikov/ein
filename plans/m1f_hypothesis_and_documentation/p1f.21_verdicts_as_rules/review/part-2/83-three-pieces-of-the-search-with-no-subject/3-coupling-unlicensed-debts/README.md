# 83 · 3 — the coupling's unlicensed debts: payers read off the payment's holders

[← item 83](../README.md) · [← piece 1, L5's last reason](../1-l5-last-reason/README.md) ·
[← piece 2, L9](../2-l9-nothing-writes-it/README.md)

## What it is

`ein render h-space` describes the structure of $H$ ([the hypothesis space](../../../../../../../docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md#coupling-questions-edges-ties-and-debts)).
It reads only; no answer depends on it. Its coupling says which questions bear on each other:

- **A question** is a group, *which of these, if any*, or a candidate in no group, *this one, or not*.
- **An edge** joins two questions or more. A **tie** is one kind: a conflict or a derivation needs a member of each.
- **A debt** is the other kind: an instance root owes whose **payers** lie in two questions or more. Every state of
  $R$ holds one of its payers, so $R$ is no product across those questions.

**Who the payers are.** A licensed instance's payers are its alternatives, the payments that are candidates (piece
1). An unlicensed one has a payment that is no candidate, so the coupling *reads* its payers: **the candidates whose
own node, root with that one candidate committed and saturated, holds a payment**, the payment's *holders*. That is
this piece: `Debt.licensed == false`, `Owing::Unlicensed { payers, about }`, and the document's sentence *an instance
L5 does not license has no payers read* where a stopped layer left the holders unread.

**It is a reading, not a licence.** The holders are read one candidate at a time, so a payment that only two
candidates derive *together* has no holder among them. The payers read can then miss a way to pay, as on
`13_witness_of_two` below.

<img src="runs/concept.svg" alt="two-ways-to-eat before the union: the unlicensed debt's payers read are (orders Ann) and (invites Bob), two questions, so the debt is an edge; 13_witness_of_two before the union: the payers read are (orders Ann) alone, and the pair (cooks Bob) with (shops Cat), which pays only together, is not read; under the union the payment is a candidate and the debt is its own question's">

*Drawn by hand (`../tools/concept-3.dot` → `runs/concept.svg`).*

## The simple example: two ways to eat

[`programs/two-ways-to-eat.ein`](programs/two-ways-to-eat.ein) (twin for the engine before:
[`two-ways-to-eat.old.ein`](programs/two-ways-to-eat.old.ein)): Ann owes a dish, and two candidates each derive the
one payment alone.

```lisp
(total-owed fed is-a)                                          ; Ann owes a dish: (fed Ann Soup) pays
(rule takeaway () :match (orders Ann)  :assert (fed Ann Soup) …)
(rule dinner   () :match (invites Bob) :assert (fed Ann Soup) …)
(rule order  () :match (is-a ?g Guest) :assert (open (orders ?g)) …)
(rule invite () :match (is-a ?h Host)  :assert (open (invites ?h)) …)
```

| | the engine before the union | the final binary |
|---|---|---|
| H | 2: `(orders Ann)`, `(invites Bob)` | 3: and `(fed Ann Soup)`, *from `union`* |
| the instance | **unlicensed**: `(fed Ann Soup)` no candidate, not denied, held at root by `(orders Ann)`'s node and `(invites Bob)`'s | licensed, one alternative: `(fed Ann Soup)` |
| its payers | **read**: `(orders Ann)`, `(invites Bob)`, two questions | `(fed Ann Soup)`, one question |
| the coupling | 2 questions, 1 edge, **1 debt that is an edge, unlicensed** | 3 questions, 2 ties, 3 edges, **0 debts that are edges**: the debt is its question's own, and the two derivations are edges of the whole node |
| $R$ | 3 states | 4 states, `{(fed Ann Soup)}` among them |

From `runs/two-ways-to-eat.old.rows.txt` and `.new.rows.txt`, and the `h-space.json` beside them (`coupling.debts`).

| the coupling before: the debt is the edge, *owed ×1* | the coupling since: no debt edge; a derivation joins all three |
|---|---|
| <img src="runs/two-ways-to-eat.old.coupling.svg" alt="the engine before the union: two questions, (invites Bob) and (orders Ann), joined by one edge labelled 1, owed ×1"> | <img src="runs/two-ways-to-eat.new.coupling.svg" alt="the final engine: three questions, (fed Ann Soup), (invites Bob) and (orders Ann), a hyperedge of the three and two edges from the witness to each"> |

## Where the payers read miss a way to pay

[`13_witness_of_two.ein`](../../../../../../../examples/lattice/13_witness_of_two.ein) on the engine before the
union (piece 1's runs, `../1-l5-last-reason/runs/13_witness_of_two.old.rows.txt`): the instance is unlicensed, and
the payment's one holder is `(orders Ann)`. `(cooks Bob)` and `(shops Cat)` each hold nothing alone. So the payers
read are `{(orders Ann)}`, one question, and the debt is no edge. Yet `{(cooks Bob), (shops Cat)}` is a member of $R$
that holds no payer read. A route never branched on the payers read; this is why they stay a description.

## The corpus program that was its subject: the classic zebra

`examples/zebra.ein`, its whole `ein render h-space` summary on each engine (`runs/zebra.old.h-space.txt`,
`runs/zebra.new.h-space.txt`):

| | the engine before the union | the final binary |
|---|---|---|
| H | 56 candidates, 18 dead alone | 112 candidates, 36 dead alone |
| instances L5 does not license | **18 of the 36** root owes, each about a group | 0 of 36 |
| debts that are edges | **18, all unlicensed** | 0 |
| questions · edges | 36 · 386 | 352 · 33 305 |

Each of the 18 is a slot's fill, `std.slots.slot-owed-fill` (*House-1 has a pet*). Its payments are facts such as
`(co-located House-1 Fox)`, which no hypothesis rule proposed, each held by the nodes of many candidates through the
puzzle's rules, so its payers read lay across many questions. The old coupling is drawn as
`runs/zebra.old.coupling.svg`. The new one is kept as `runs/zebra.new.coupling.dot` only: 34 000 lines, which
Graphviz did not lay out in twelve minutes.

## The code, and the tests that hold it today

| where | what |
|---|---|
| `ein-infer/src/structure/coupling.rs`: `Debt.licensed`; the debts in `Coupling::read` (an unlicensed instance's payers are the candidates whose own node at root holds one of its witnesses, `held_at_root`, read only where root's singles were); `Coupling::payers_read`, `Coupling::unlicensed` | the unlicensed path |
| `ein-infer/src/structure.rs`: `Owing::Unlicensed { payers, about }`, built in `ExactlyOne::of`; `ExactlyOne::unlicensed`, `ExactlyOne::brought` | the owed groups' reading of such an instance |
| `ein-render/src/structure.rs`: the document's rows of such an instance (`why`, `pay_by_the_theory_alone`, `about_group`) and the stopped layer's sentence | what `ein render h-space` prints |

| test | what it asserts today |
|---|---|
| `ein-infer/tests/coupling.rs` `the_coupling_is_held_to_n_and_to_r` | a census it prints: *17 debts are edges, on 5 programs, 0 of them of an instance L5 does not license, on 0* |
| `coupling.rs` `a_debt_is_an_edge` | `send.debts.is_empty() && send.unlicensed == 0` (`send_more_money.ein`) |
| `coupling.rs` `holders_unread_leave_the_coupling_incomplete` | both zebras at a budget of one fact: `unlicensed == 0 && payers_read`, *"both zebras' tables hold none"* |
| `ein-infer/tests/exactly_one.rs`, three tests | `zebra.ein`, a cryptarithm and `06_friends_tables_everyone_sits.ein`: `unlicensed()` empty |
| `ein-infer/tests/structure_invariance.rs` `a_form_that_reorders_the_universe_moves_no_set` | `(c.debts.len(), c.unlicensed) == (2, 0)` |
| `ein-cli/tests/h_space_view.rs`, two tests | `zebra.ein`'s document: `exactly_one.unlicensed` empty; at `--region-budget 1`, `coupling.unlicensed_instances == 0` and `payers_read` |

Each was run on the final tree by S1f.21.19 (`../runs/test.*.txt`), and passes.
