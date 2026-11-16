# S1f.10.14 — Exactly one, spent: branching nothing licenses yet

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-15, at the re-plan. **Running** (2026-11-16): T1 written, a *no*, its witness banked
([§ Record](#record-2026-11-16)). The phase's most speculative stage, and the one most likely to end in a written
*no*.
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

## Record (2026-11-16)

Taken over from an earlier agent stopped when its session ended, from what it left on disk (§ What was taken
over, below, once the stage closes).

### T1: the implied witness — *no*

**The claim.** Let `I`, an instance root owes, have witnesses `w₁ … wₖ`, none a candidate, each in the closure of
some candidate, and every other read of `I`'s guard static (L5's premise). Let `A(I)` be the candidates whose
closure holds a witness. Then every member of R above a state owing `I` holds a member of `A(I)`, and an owing
state may branch on `A(I)` as L5 branches on its alternatives.

**The argument it would need**, written as `tree.rs` writes L5's. Let `D ⊇ C` be a member of R and `C` owe `I`.
The program is monotone and `I`'s trigger static, so `I` is an instance at `D`, and `D` is discharged, so a guard
of `I` holds at `D`; its other reads are static, so a witness `w` is in `D`'s fixpoint, and `w` is one of the
facts the guard admits at root. **Here L5's argument has its step**: `w` is a candidate (or denied, and then in
no alive fixpoint), so `w ∈ cl(D)` and `cl(D) ⊇ C ∪ {w}`. The claim needs instead that **some `a ∈ cl(D)` has
`w` in the fixpoint of `{a}`**, and nothing gives it: the fixpoint of `D` is not the union of the fixpoints of
its members wherever a rule reads two facts a commitment can change. The gap the stage file names, *a witness
derived at a node by no single candidate*, is real on a monotone program whose every other read is static, so
the premise does not exclude it and the claim is false.

**The witness**, [`examples/lattice/13_witness_of_two.ein`](../../../examples/lattice/13_witness_of_two.ein),
banked with a corpus entry and a catalogue line, and held to the oracle by `ein-infer/tests/region.rs`'s
`a_witness_two_candidates_derive_together_stands_for_neither`. Ann owes a dish (`total-owed`); her one witness,
`(fed Ann Soup)`, is derived by `takeaway` from `(orders Ann)` alone and by `dinner` from `(cooks Bob)` and
`(shops Cat)` together. Of the three single candidates only `(orders Ann)`'s node holds it, so `A(I) =
{(orders Ann)}`. `ein oracle` gives R as five states, each its own commitment, and **`{(cooks Bob), (shops Cat)}`
is one of them and holds none of `A(I)`**: a route that branched root on `A(I)` would lose it. The program is
monotone, the guard's other read is `is-a`, static. L5 does not license the debt (*a `fed` fact can witness it
that no candidate proposes*), so no branch takes it, every mode takes the state route, and every mode answers the
oracle's R (`explore.rs`); `ein test` holds its `(solve :expect (region …))`.

**What a premise would have to say**, recorded and not built: that every derivation of a witness at a node rests
on one candidate, for instance that every plan writing a relation a commitment can change reads at most one
fact of such a relation. Then the fixpoint of `D`, read on the witnesses, is the union of its members'. It
would not reach the instance the stage file opened with: on `zebra.ein` the witnesses are `co-located` facts,
and `std.slots`' `slot-locate` reads two of them; and of `zebra.ein`'s 56 witnesses S1f.10.4 found 18 in no
alive single's closure, which is where `A(I)` would be emptiest. **So T2 as planned is not built**: the plan
builds it only if T1's argument holds. `zebra.ein`'s `slot-owed-fill` debts stay unlicensed, which S1f.10.13
found costs I-Z1 nothing today: every owing state it visits also owes a `slot-owed-room` instance, which the tree
branches on.

