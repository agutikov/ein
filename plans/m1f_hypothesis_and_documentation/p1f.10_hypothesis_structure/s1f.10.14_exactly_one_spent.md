# S1f.10.14 — Exactly one, spent: branching nothing licenses yet

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-15, at the re-plan. **Ran to its end, 2026-11-16** ([§ Record](#record-2026-11-16)):
place 1 is a written *no* as planned, its witness `examples/lattice/13_witness_of_two.ein` banked, and a narrower
licence, **L9**, *a fact nothing proposes and nothing writes is in no state*, built for the eight questions the
tree answers by the price list's rule; place 2's advice is in the bundle; place 3 is put to the user as
Q-M1f.311, not acted on.
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

Taken over from an earlier agent stopped when its session ended, from what it left on disk
([§ What was taken over](#what-was-taken-over)). The worktree over `a79eea49` (`master` merged in, the draft
carried across it); every number is a count unless it is called a wall.

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


### L9: a narrower licence, checked and priced — *go* for the tree's eight questions

T1 fails because a witness can be **derived** at a node from candidates that do not derive it alone. A narrower
claim survives that objection: a witness that no node can hold at all. The earlier agent had started it, as
*L9*, beside the stage's tasks: the plan builds T2 only if T1 holds, so L9 is not T2 and was decided on its own,
under the stage's *What it must not do* and the price list's rule.

**The claim.** Let the program be monotone, root owe `I`, every read of `I`'s guard beside its witness step be
static (L5's premise), and let `w` be a fact that can witness `I`, no candidate, not denied at root, of a relation
`R` that **no plan root compiled writes (an hrule's apart) and the kernel does not mirror**. Then `w` is in no
node's fixpoint, alive or dead, so it can be counted with the denied witnesses, and `I`'s alternatives are the
candidates among its witnesses; L5's argument then goes through unchanged.

**The argument, checked against what a node's fixpoint can hold**, every place the engine writes a fact read
(`grep` over `ein-core` and `ein-infer` for the KB's three writers, `add_fact`, `add_and_index_fact` and
`record_justification`):

| writer | at a node? | what it writes | in the argument |
|---|---|---|---|
| the node function (`region::fixpoint_firing`) | yes | the commitment's candidates | `w` is no candidate |
| a plan's firing (`firing::build_fact`) | yes | a conclusion, a fact whose relation is the template's own: a conclusion that is no `Slot::Nested` is refused | `Unwritten::of` reads every plan's `writes_of`; a parameter the activator binds is the plan's constant |
| the symmetric mirror (`Saturator`, from the node's own `(__symmetric__ R)` facts) | yes | `(R b a)` beside `(R a b)` | root's marks are written relations; a plan that writes `__symmetric__` makes every relation written |
| `record_justification` | yes | a second justification of a fact already held | no new fact |
| the generator's lookahead (`hypgen::write_negated`) | no: the node function runs no generator | `(not h)` | `not` is a kernel relation, written |
| `closed::emit_closed`, the loader's `written` facts | no: before root's saturation, at load | `(__closed__ R)`, `(written T)` | kernel relations, written |
| the obligation pass (`obligations::tally`) | reads only | nothing | — |
| `structure.rs`'s small KB | not a node | — | — |

Macros and imports are expanded into rules before anything compiles, so their conclusions are plans' writes. On
a monotone program no rule's activator relation is one a commitment can change, so the plans a node runs are
root's (the static check reads the same memo). So the premise is **computed, not argued**, from root's plans and
marks, beside one argued sentence (*nothing else in a saturation writes a fact*), and that sentence is checked at
every node a route computes: the node function reports a fact of such a relation beyond root's that is no
candidate, and the run refuses.

**What the earlier agent's draft had, and what was changed** (the draft is kept where it was right; § What was
taken over):

1. `Unwritten::of` read only the plans of saturation rules; it now reads every plan but an hrule's, which proposes
   and runs at no node.
2. A plan's stored negation was skipped, so `not` was never written, and a debt whose witness is a stored
   negation would have been called unheld though rules write negations. Every kernel relation (`not`, `false`,
   `open`, `__closed__`, `__symmetric__`, `written`) is now written, whoever writes it.
3. The node check read the node's state key; it now reads the fixpoint's facts of the relations L9 relied on
   (`unheld::breach`, one definition), and the whole node the tree's order reads beside the cache is checked
   too, which the draft did not.
4. The draft gave every reader of L5's table L9's alternatives, the structure's own reading
   (`structure::node_coupling`) among them. Only the tree's route takes them now
   (`Space::route_alternatives`); the structure reads L5's table as it did.
5. **An instance with no candidate among its witnesses is not L9's.** The draft licensed it with no alternative,
   which ends the branch where it owes. On the corpus that is six programs under `tests/` whose lattice is root
   alone (`:no-search`), where it changes no work and moves the `route` row of four modes from the state route
   to *the obligation tree … L5 … L9*, and with it `read_out_examples.md`'s X6; the default that changes the
   fewest surfaces ([§ Defaults taken](#defaults-taken)).
6. The draft's header named tests that did not exist; they are written (below).

**The census** (`ein-infer/tests/unheld.rs`, `l9_licenses_where_nothing_writes_the_witness_relation`, over every
corpus program and register witness): L9 licenses **five instances on two programs**, and nothing else.
`lattice/06_friends_tables_everyone_sits.ein`: Cat, Dan and Eve each owe a seat, three alternatives at the big
table and two witnesses, the small table's seats, of `sits`, which no rule concludes. And
`lattice/14_seats_nothing_proposes.ein`, the fixture written for the oracle (12 candidates, 4 096 nodes): the
same shape with two guests. `zebra.ein`'s 18 `slot-owed-fill` debts are not L9's (`co-located` is written by
`std.slots` and mirrored), nor is `13_witness_of_two.ein`'s (`fed` is written by two rules), nor any of the
other unlicensed instances S1f.10.4's T4 listed (`features/13_mixed_solution_and_open.ein`'s `likes` is written by
a rule; the register's `oblig-*` debts fail L5's static premise first).

**The work**, every question with and without (the switch `unheld::without`, internal, thread-local, no lever),
saturations · facts beyond root, at the default budget, nothing stopped (`unheld.rs`,
`every_question_answers_alike_with_l9_and_without`, 2026-11-16, the gate profile):

| question | `06_…everyone_sits` with L9 | without | `14_seats_nothing_proposes` with L9 | without | the two, with | without |
|---|---:|---:|---:|---:|---:|---:|
| describe | 345 · 969 | 345 · 969 | 99 · 244 | 99 · 244 | 444 · 1 213 | 444 · 1 213 |
| `--count` | 345 · 969 | 345 · 969 | 99 · 244 | 99 · 244 | 444 · 1 213 | 444 · 1 213 |
| `--single` | **61 · 242** | 207 · 788 | **23 · 78** | 54 · 175 | 84 · 320 | 261 · 963 |
| `-n 1` | **56 · 213** | 206 · 783 | **19 · 59** | 52 · 166 | 75 · 272 | 258 · 949 |
| `--minimal` | **427 · 1 421** | 491 · 1 659 | **119 · 340** | 139 · 424 | 546 · 1 761 | 630 · 2 083 |
| `--maximal` | **123 · 640** | 213 · 818 | **39 · 164** | 61 · 208 | 162 · 804 | 274 · 1 026 |
| `--minimal -n 1` | **60 · 229** | 208 · 791 | **22 · 68** | 54 · 172 | 82 · 297 | 262 · 963 |
| `--minimal --single` | **72 · 292** | 211 · 804 | **30 · 101** | 58 · 187 | 102 · 393 | 269 · 991 |
| `--maximal -n 1` | **60 · 241** | 206 · 783 | **21 · 71** | 52 · 166 | 81 · 312 | 258 · 949 |
| `--maximal --single` | **99 · 472** | 213 · 818 | **35 · 140** | 61 · 208 | 134 · 612 | 274 · 1 026 |

Every other program's cells are the same with and without, to the node: where L9 licenses nothing, the route's
table is L5's as read (`Alternatives::with_l9` copies it and names no relation), which the census holds. **The
check costs no saturation and no fact**: it is a scan of the node's facts of `sits`, in the node function's own
pass, and the whole node's when the order reads one.

**The rule** (S1f.10.13's, as its defaults read it): *go* for a question when the corpus total, the check's cost
in it, is lower in saturations and in facts, and no program's cell is higher by more than a tenth. For each of
the eight questions the tree answers, both totals fall by the last two columns' difference and no cell rises
anywhere: **go for all eight**. `06_…everyone_sits --single` is 61 saturations where it was 207, the 176
S1f.10.13 counted above states owing only unlicensed instances being, but for 30, what L9 removes. Describe and
`--count` take the state route, which reads no table, so L9 is not wired there and their cells do not move (the
stage must not change describe's route). There is no loser.

### What will move, named before the route changes

From the bless over every corpus program, `tests/` too (`EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test -p ein-render
--test id_order_invariance`, then `corpus_cli`'s), against `master` with this stage's T1; one golden that moves
and is not here is a stop.

- **`ein-render/tests/golden/corpus_shapes.md5`**: `examples/lattice/06_friends_tables_everyone_sits.ein`'s
  eight lines of the modes the tree answers, `read-out[single]`, `read-out[n2]`, `read-out[minimal]`,
  `read-out[maximal]` (the `route` row names L9, the `work` row moves, a listed state may be another of R's 8)
  and `cores[single]`, `cores[n2]`, `cores[minimal]`, `cores[maximal]` (other dead nodes are visited); and
  `examples/lattice/14_seats_nothing_proposes.ein`'s lines, new, the fixture entering the corpus with the
  licence. No other line.
- **`ein-cli/tests/golden/corpus_exits.txt`**: the fixture's six cells, new, every one exit 0.
- **`docs/kernel/inference/read_out.md`**: the `route` row names L9 in words; **`read_out_examples.md`**: no
  example moves (`python3 utils/read_out_examples.py --check` is clean on the built route: no example's program
  has an instance L9 licenses).
- **`docs/kernel/inference/implementation.md`**: a row for the new module, `unheld.rs`.
- **`corpus/corpus.toml`** and **`examples/README.md`**: the fixture's entry and catalogue line.
- Not moving: `README.md`'s and `docs/guide/04_solving_the_whole_puzzle.md`'s transcripts and
  `docs/api/rust.md`'s block (`zebra2.ein`, no instance L9 licenses), `ein-render/tests/golden/trace_*.md`,
  `ein-cli/tests/region_modes.rs`, every `cost_ms` and the slow set.

### L9, as built

As decided above, committed apart from the list (`fc86f98e`): [`ein_infer::unheld`](../../../ein.rs/crates/ein-infer/src/unheld.rs)
(the argument in its header; `Unwritten`, `breach`, the refusal, the two switches), `oblgen::Instance::unheld`
and `Alternatives::with_l9`, `Space::route_alternatives` for the tree alone, the check in the node function and
at the whole node, `Licence::Unheld` and its label, `tree.rs`'s header (§ L9, a witness no node can hold),
[`read_out.md`](../../../docs/kernel/inference/read_out.md)'s `route` row, the module's row in
[`architecture.md`](../../../docs/ein/ein.rs/architecture.md), the fixture
[`examples/lattice/14_seats_nothing_proposes.ein`](../../../examples/lattice/14_seats_nothing_proposes.ein) with its
corpus entry and catalogue line, and [`ein-infer/tests/unheld.rs`](../../../ein.rs/crates/ein-infer/tests/unheld.rs):

| test | what it holds |
|---|---|
| `l9_licenses_where_nothing_writes_the_witness_relation` | the census above, over every corpus program and register witness, each instance's L5 reason, the list whole (so none of `zebra.ein`'s 18, which the test does not name: it would join the puzzle's world anchors); `13_witness_of_two.ein`'s one debt not L9's |
| `l9_is_the_oracle_on_every_monotone_program_it_reaches` | on `14_seats_nothing_proposes.ein`'s cube, all 4 096 nodes: none holds a fact of `sits` beyond root's that is no candidate; at every alive state each instance L9 licenses is owed exactly when none of its alternatives is in it; every member of R above a state owing one holds an alternative (62 owed instances read) |
| `every_question_answers_alike_with_l9_and_without` | the ten questions on the two programs, with and without (`unheld::without`): one verdict, one `exhausted`, the same part of R listed whole, every listed state in describe's R; describe and `--count` field for field; L9 named only with it; each of the eight tree questions lower in both units over the two |
| `a_node_that_breaks_the_premise_refuses_the_run` | a program whose rule writes a witness of its debt: L9 does not license it and every question answers, R three states; under `unheld::trusting` it is licensed with its candidate alone, and `--maximal` and `--single` refuse at the first node that holds the written witness, `L9: a node holds (fed Ann Stew)` |

`explore.rs` holds every mode to the oracle on `14_seats_nothing_proposes.ein`'s cube with the rest of the corpus,
no expectation edited. **The goldens that moved are the list's, and no other**: `corpus_shapes.md5`'s eight
tree-mode lines of `06_…everyone_sits` and the fixture's new lines, `corpus_exits.txt`'s new cells.

### T3: a group nothing owes — what a program would have to state, and what it would cost

No engine work: nothing is synthesised, and each copy is a program a person wrote the lines into. For each
program [S1f.10.4](s1f.10.4_bounded_groups.md#t3-where-the-must-is-missing) named where *exactly one* is not
derived, the lines that state the *must* and every question of `ein solve` on the program and on the copy, the
built engine, the default budget (`build/structure/s14/probes.sh`, then `must.py`, 2026-11-16; the copies under
`build/structure/s14/p/`, the read-outs under `out/`, the table `must.json`). Saturations, before → after:

| program | groups nothing owes | the lines | R | describe | `--count` | `--single` | `-n 1` | `--minimal` | `--maximal` |
|---|---:|---|---|---:|---:|---:|---:|---:|---:|
| I-Z1 `zebra.ein` | 18 | `slot-owed-fill`'s debt with its witness value first (S1f.10.4's `fill_mirrored.lines`) | **the same**: 1 state, 261 982 commitments | 314 → 314 | 314 → 314 | 9 → 13 | 5 → 5 | 318 → 318 | 9 → 13 |
| `lattice/07_two_pairs_two_tables.ein` | 4 | `(surjective-owed sits is-a)` | **the same**: 8 states | 177 → 177 | 177 → 177 | 44 → 63 | 41 → 36 | 233 → 221 | 101 → 132 |
| I-SEND `beyond_arith/send_more_money.ein` | 10 | `(surjective-owed val is-a)` | **another, empty**: *Solution* → *Open* by `--single` | 336 101 → 336 101, stopped both | the same | 18 030 → 48 807 | 15 855 → 45 137 | 205 166 → 110 840, stopped both | 18 030 → 48 807 |
| `beyond_arith/cryptarithm.ein` | 10 | `(surjective-owed val is-a)` | **another, empty**: 1 state → none | 417 → 417 | 417 → 417 | 119 → 13 | 102 → 13 | 436 → 423 | 128 → 13 |
| I-B06 `branching/06_lookahead_on.ein` | 10 | every colour and house owes a kind (S1f.10.4's `typed_owed.lines`) | **another, and not by the *must***: 2 states, 4 commitments → `{∅}` | 133 → 1 | 133 → 1 | 8 → 1 | 1 → 1 | 1 → 1 | 258 → 1 |
| `zebra2-hints.ein` | 26 | `total-owed` and `surjective-owed` of each of its five `*-loc` | **another**: 6 states, 8 316 commitments → 1 state, 8 188 | 62 → 62 | 62 → 62 | 4 → 3 | 1 → 2 | 1 → 64 | 84 → 3 |
| `domain_elim/b_branch.ein` | 2 | `total-owed` and `surjective-owed` of `color-loc` | **another**: 2 states → 1 | 8 → 8 | 8 → 8 | 2 → 8 | 1 → 2 | 1 → 8 | 14 → 10 |

**What it says to a program's author**, three kinds:

- **Where the *must* is true of the puzzle and follows from what is enforced** (I-Z1 and `07`, S1f.10.4's
  route 1, a grid whose other family is owed whole), stating it leaves R as it is and **does not pay**: describe
  and `--count` take the state route, which reads no debt to choose by, and the tree, with more licensed debts
  to branch on, is dearer under `--single` (9 → 13, 44 → 63) and `07`'s `--maximal` (101 → 132), a little cheaper
  under `07`'s `-n 1` and `--minimal`. *Exactly one* that a count already gives is not worth a line.
- **Where the program declares the property and owes nothing** (`zebra2-hints.ein`, `b_branch.ein`: `bijective`,
  whose `total` half is a refutation over stored negatives, not a debt), stating the debts **changes R**, to the
  one full assignment from six and two states that each leave a value or a house without a partner. That is
  S1f.10.4's route 2 shown: the declaration was never a *must*, and the debts are what the author meant if the
  full assignment is the answer meant. The work moves both ways, `--maximal` 84 → 3 and `--minimal` 1 → 64 on
  `zebra2-hints`.
- **Where the *must* is false** (I-SEND, `cryptarithm`, more digits than letters) R becomes empty, and where it is
  paid at root (I-B06) the ladder moves to the second rung and H is empty (K-11). Neither is advice to state
  anything; they are why nothing here is stated by the engine.

**In the bundle**: `utils/structure_bundle.py` reads `build/structure/s14/must.json` (a required input, as
every other), writes a page *Where exactly one is missing* (`bundle/must.html`, a table of every program and
every question and one table a program, each with its JSON beside it) linked from the index, and a section
*What it would have to state* on the page of each program the file names. Checked on a subset written under the
worktree (`python3 utils/structure_bundle.py --bin ein.rs/target/gate/ein examples/zebra.ein
examples/domain_elim/b_branch.ein examples/lattice/07_two_pairs_two_tables.ein`, then `--check`: 28 pages,
126 tables each with its JSON, 0 errors), the inputs copied from the main tree's `build/structure/` and
`build/s1f.10.{9,6,15}/census/`. **The main tree's `build/structure/s14/` does not exist**: before the bundle is
built there again, `probes.sh` and `must.py` are run there (or the worktree's `build/structure/s14/` copied in).

### T4: describe, as a question with numbers — Q-M1f.311, put

**Nothing is built**, and describe's route is as it was. Describe takes the state route: every alive state once,
its one-up extensions saturated, the owing ones among them. The read-out divides by what it is read from
(`states.rs` § Where N and E come from, `read_out.md` § 2):

- **read off every alive state, the owing ones among them**: `k_c`, counted over each state's alive states below
  it (`f(T) = 2^|T| − Σ f(S)`); the merged states and their minimal commitments; the minimal line, `E`'s alive
  members, as the minimal generators of R's states; `N`, the minimal transversals of the maximal alive states'
  complements, a maximal alive state being owing as often as not; and the shape, components and sub-cubes, over
  R's commitments;
- **read off R's states alone**: `k_s`, the states listed, the maximal line (a state of R whose every one-up
  extension is dead), and a query's classes over R. The obligation tree reaches these without the owing states
  its branches skip (L5, L9): `-n N` lists every state of R once its leaves run out (L2), `--maximal` the line,
  `--single` whether R is one state.

**The numbers**, re-taken on this stage's engine (`build/s14/t4/t4.sh`, then `tally.py`, 2026-11-16, the gate
profile, the default budget; describe's nodes by kind from `--dump-lattice`'s `nodes.json`), saturations · facts
beyond root:

| program | describe | of it, alive owing | in R | dead | the tree: `--maximal` | `--single` | `-n 1` | `--minimal` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| I-SEND `beyond_arith/send_more_money.ein` | 336 101 · 1 422 943, **stopped**, 0 states met | **330 329 · 1 376 430** (98.3 %) | 0 | 5 772 | 18 030, the line whole: 1 state | 18 030, *Solution*: R is one state | 15 855 | 205 166, stopped |
| I-SENDW `…_written.ein` | the same, **stopped** | the same | 0 | 5 772 | 18 030, 1 state | 18 030, *Solution* | 17 542 | 131 134, stopped |
| I-Z2M `zebra2-minus-15.ein` | 7 261 · 545 818, **stopped**, ≥ 32 states | **5 530 · 270 526** (76.2 %) | 865 | 866 | 108, the line whole: 32 states | 39, *Ambiguity* | 35 | 13 102, stopped |
| its `-obligations` twin | the same | the same | 865 | 866 | 108 | 39 | 35 | 13 102, stopped |
| `lattice/06_friends_tables_everyone_sits.ein` | 345 · 969 | 269 · 726 (78.0 %) | 8 | 68 | 123 | 61 | 56 | 427 |
| `lattice/07_two_pairs_two_tables.ein` | 177 · 448 | 105 · 224 (59.3 %) | 8 | 64 | 101 | 44 | 41 | 233 |
| `beyond_arith/cryptarithm.ein` | 417 · 1 301 | 192 · 357 (46.0 %) | 1 | 224 | 128 | 119 | 102 | 436 |
| I-Z1 `zebra.ein` | 314 · 55 346 | 139 · 9 332 (44.3 %) | 81 | 94 | 9 | 9 | 5 | 318 |
| I-Z2 `zebra2.ein` | 263 · 22 503 | 103 · 3 589 (39.2 %) | 81 | 79 | 7 | 7 | 5 | 267 |
| `beyond_arith/equation_two_unknowns.ein` | 1 466 · 60 223 | 331 · 630 (22.6 %) | 3 | 1 132 | 1 165 | 524 | 521 | 1 466 |
| `saturation/square-bwd/floors.ein` | 85 574 · 994 221, **stopped** | 0 | 85 574 | 0 | 11 | 2 | 1 | 1 |

And **`-n 1000000` on I-Z2M lists every state of R, 32, exhausted, in 104 saturations and 10 944 facts**, where
describe is stopped after 7 261 saturations with `k_s` a lower bound and the maximal line *not reached*. On I-SEND
`--single`'s *Solution* is all of R, one state, in 18 030, where describe is stopped after 336 101 having met none.

Over the corpus, S1f.10.13's price list (188 programs, before L8 and the order, so describe's cells are larger
there than above): describe's alive owing share is **343 323 of 1 350 675 saturations, 25.4 %**; the three
monotone cells the budget stopped, I-SEND and the I-Z2M pair, are 341 389 of it, and over the monotone cells no
budget stopped it is 1 625 of 28 540. **So the question bears on four programs today**, the three and I-SENDW
(banked since): monotone, describe stopped by the budget, almost all of it on alive owing states, and the tree
reaching every state of R for a twentieth of describe's work or less (5.4 % on I-SEND, 1.4 % on I-Z2M). Six more describes the budget stops,
`saturation/square-*/`, meet no owing state and already decide R as the whole cube (L1, L2); seven more are
flagged programs past the cube's limit, where no tree runs. Where describe finishes, its owing share runs from
a fifth to four fifths and is what `k_c`, `N`, the minimal line and the merged states are read from.

**Put to the user as [Q-M1f.311](../open_questions.md#q-m1f311--a-describe-the-budget-would-stop-answer-what-the-tree-can-reach-instead)**,
the next free id: *should a describe that the budget would stop answer what the tree can reach instead?*, with
four options and the default the plan's framing gives, (a), describe as it is. **Not acted on**: to be shown at
the phase's close.

### What was taken over

The earlier agent left, uncommitted over `037b7f55`, five files changed and two new (`unheld.rs` and
`13_witness_of_two.ein`; copies in the main tree's `build/p1f10/stopped/`), and no test, no record and no
commit. Carried across `master`'s `a8fa30d3` with no conflict. **Kept**: the witness, its header's last paragraph
corrected (it named a test file that did not exist, and said the tree branches root, where every mode takes the
state route); L9's idea and its two switches; `oblgen`'s scan past L5's first refusal (`Instance::unheld`,
`Unheld`, `with_l9`, `by_l9`), which leaves L5's verdict and reason as they were; `Licence::Unheld`;
`NodeOut::unheld` and the refusal in `Space::eval`; the tree's flag that names L9. **Changed**: the six items of
[§ L9](#l9-a-narrower-licence-checked-and-priced--go-for-the-trees-eight-questions). **Added**: the check at the
whole node, `tests/unheld.rs`, the fixture, the docs, T1's test, T3 and T4.

### Defaults taken

Each the user's to revise.

1. **L9 was decided at all.** The plan builds T2 only if T1 holds; L9 is not T2 but a narrower claim the T1
   witness leaves standing, and it was held to the stage's *What it must not do* and the price list's rule before
   it was built.
2. **An instance with no candidate among its witnesses is not L9's**: the fewest surfaces. Licensed, it would end
   a branch where it is owed; on the corpus it occurs only on six `tests/` programs whose lattice is root alone,
   where it moves four `route` rows each and `read_out_examples.md`'s X6 and no work.
3. **Only the tree's route takes L9.** The structure's readers (`structure::node_coupling`, `Space::owed`, the
   census, `ein render h-space`) read L5's table as before; the tree's coupled order reads the route's table, so an
   instance L9 licenses is a question there.
4. **The premise is read wider than the argument needs**: every plan but an hrule's counts as a writer, every
   kernel relation as written, and a plan that writes `__symmetric__` makes every relation written.
5. **The rule as S1f.10.13's defaults read it**, both units; the check's cost is no saturation and no fact; the
   corpus totals were not re-taken, since every other program's route table is L5's as read (the census holds
   it), so each total falls by the two programs' difference.
6. **The fixture entered the corpus with the licence**, so its golden lines are new in the commit that built L9,
   not moved by it; and T1's test sits in `tests/region.rs` beside L5's, committed before L9.
7. **T3's programs**: S1f.10.4's three of route 3, its two declared-and-unowed programs, and the two its census
   paired with them (`07` beside I-Z1 for route 1, `cryptarithm.ein` beside I-SEND); the lines are S1f.10.4's
   where it wrote them, and `total-owed` with `surjective-owed` where it named them. `must.json` is a required
   input of the bundle, as every other.
8. **T4's numbers re-taken on eleven programs**, the corpus share quoted from S1f.10.13's census; Q-M1f.311's
   default (a) from the plan's framing.
9. **No slow half and no nightly step**: `unheld.rs` reads two programs per commit.

### Verification

In the worktree, the gate profile, on the tree as committed but for this record (the last Rust edit is
`tests/unheld.rs` dropping a reference to the Zebra puzzle, after the first `ein-cli` run below):

- `cargo test --profile gate -p ein-infer`, whole: 54 binaries, 554 tests, green (on `fc86f98e`'s code), and
  `--test unheld` again after its last edit, green.
- `-p ein-render`, whole: green. `-p ein-cli`, whole: green on the second run, 406 tests. The first stopped at
  `world_anchors`: `tests/unheld.rs` named `examples/zebra.ein`, which made it one of the puzzle's world anchors;
  the reference was dropped (the census's list being whole already says it) rather than the anchor list
  re-banked.
- `EIN_CORPUS_SLOW=1`, once each: `explore` (104 s), `known_dead` (206 s), `order` (208 s), `corpus_cores`
  (117 s), `-p ein-render --test id_order_invariance` (65 s): green. No slow half was added.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: clean.
- `python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md
  examples/README.md stdlib/README.md utils/README.md c/README.md`: 0 findings;
  `python3 -m py_compile utils/structure_bundle.py`; `python3 utils/check_hashmap_iteration.py`: ok;
  `python3 utils/read_out_examples.py --check`: clean; `utils/structure_bundle.py --check` on the subset: 0
  errors.
- Not run, as the stage was told: `./run_tests.sh`, the nightly, `utils/ci_container.sh`. The walls above were
  taken with other runs on the machine and are not measurements.

### What is not done

- **T2 as planned**, the implied witness: T1 is *no*. `zebra.ein`'s `slot-owed-fill` debts stay unlicensed, by
  any licence here: their witness relation is written.
- **Q-M1f.311** is put and not ruled; nothing is built for it.
- **The main tree's `build/structure/s14/`** does not exist; the worktree's holds T3's copies, read-outs and
  `must.json`, and T4's runs are under the worktree's `build/s14/t4/`.
- No wall was taken: the check's time is a scan of one relation's facts per node, not measured.
- The gate, the nightly and the runner's container were not run (the main session's, on the batch).

**Findings outside the stage's scope**: none; K-28 stays the last.
