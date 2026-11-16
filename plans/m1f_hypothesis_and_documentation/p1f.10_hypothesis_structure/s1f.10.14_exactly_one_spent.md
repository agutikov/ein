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
   fewest surfaces (§ Defaults taken, once the stage closes).
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
