# S1f.10.14 — Exactly one, spent: branching nothing licenses yet

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-15, at the re-plan. Not started. The phase's most speculative stage, and the one
most likely to end in a written *no*.
**Depends on:** [S1f.10.4](s1f.10.4_bounded_groups.md) (which groups are *exactly one*, and which owed
instances L5 does not license); [S1f.10.13](s1f.10.13_the_price_list.md) (the *alive and owing* share per
question).

## Context

*At most one* removes dead nodes. *Exactly one* removes **alive** ones: at a state that owes an instance, L5
branches on that instance's alternatives alone, and every other extension of the state is never built. The
re-plan's probe says where that matters: `zebra2-minus-15`'s describe spends 5 530 of its 8 391 saturations on
alive owing states before the budget stops it, while `--maximal`, which takes the tree, finds the 32 states in
2 346.

Three places where the engine has an *exactly one* it does not spend, in rising order of doubt:

| | where | what is missing |
|---|---|---|
| **1** | **an owed instance L5 does not license.** `zebra.ein`'s `slot-owed-fill`: its witnesses are `(co-located House-n v)`, which the hrule never proposes, so a state that owes only these branches on every extension | an argument that a witness which a candidate **implies** may stand for that candidate |
| **2** | **a group nothing owes**, on a program with no obligation at all (I-B06) | the *must*. S1f.10.4 says what the program would have to state; nothing here invents it |
| **3** | **describe**, which takes the state route and never the tree | describe reads `k_c`, N and the merged states off every alive state it met (`states.rs`, § Where N and E come from). Whether R's states could come from the tree and the rest be computed another way is a change to what describe's route is |

## Tasks

1. **T1f.10.14.1 — the implied witness (1).** The claim: let instance `I`'s witnesses be `w₁ … wₖ`, none a
   candidate, each in the closure of some candidate, and let `A(I)` be the candidates whose closure holds a
   witness. If every other read of `I`'s guard is static, as L5's premise already requires, then every member
   of R above a state owing `I` holds a member of `A(I)`. Written out as `tree.rs` writes L5's argument, under
   [`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md): the premise checked at load by
   `oblgen::alternatives` and on every node the route computes, a node that breaks it refusing the run. The
   gap to close in writing: a witness could also be **derived** at a node by no single candidate, which the
   premise must exclude or the claim is false.
2. **T1f.10.14.2 — built, if T1's argument holds**, and held: `ein-infer/tests/explore.rs` against the oracle
   on every monotone cube, with no expectation edited; the with-and-without test of S1f.10.3's T3 extended to
   it; the work delta on I-Z1 and every program S1f.10.4's census named, losers named.
3. **T1f.10.14.3 — a group nothing owes (2).** No engine work. The record: for each such program, what
   S1f.10.4 said it would have to state, and the work of its questions before and after on the copy that
   states it, by `ein solve` on both. That is advice to a program's author, and it goes to the bundle.
4. **T1f.10.14.4 — describe (3), as a question with numbers.** How much of each program's describe is alive
   owing states (S1f.10.13's share); what of the read-out needs them (`k_c`, N, E, the merged states, the
   shape) and what needs only R's states (`k_s`, the states listed, the maximal line). Put to the user under
   the next free id, at the phase's close and not before: *should a describe that the budget would stop answer
   what the tree can reach instead?* **Nothing is built here**: describe's route is a decision of M1e2's that
   this phase reports on and does not reopen by itself.

## Acceptance

- T1's argument is written in this file whether or not it holds; if it does not, the program that breaks it is
  banked under `examples/` with a corpus entry.
- If T2 is built: R identical on every corpus program in every mode, by the sweeps the README names; a node
  that breaks the premise refuses the run, shown by a test that makes one; the work delta recorded; the
  `route` row names the licence in words, with `read_out.md` in the same commit.
- T3's advice is in the bundle for every program it concerns.
- T4's numbers are recorded and its question filed.
- **A written *no* for any of the three is a full result.**

## What it must not do

- **Not synthesise an obligation, a closure or a rule.** A program's *must* is the program's to state.
- **Not branch on a group because it is a group.** *At most one* licenses a skip of dead nodes only.
- **Not change describe's route.**
- **Not ship a licence whose premise is argued and not checked at every node.** *Exactly one* is a claim that
  removes models when it is wrong.
