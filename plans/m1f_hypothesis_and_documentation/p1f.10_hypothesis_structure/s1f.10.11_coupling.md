# S1f.10.11 — Coupling: what ties the groups together

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** ran 2026-11-15, through a stop at T4. T1 to T5 are done and the owner computes the coupling; over
the conflicts' and the derivations' edges the components were finer than `R` on six corpus programs, by a debt,
and under the main session's default a debt is an edge of its own kind ([Record](#record-2026-11-15)).
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md) (*interacts*), [S1f.10.2](s1f.10.2_groups.md) (the
groups).
**Read by:** [S1f.10.12](s1f.10.12_the_structure_shown.md); of track R, [S1f.10.5](s1f.10.5_ordering.md),
whose witness needs it.

## Context

Groups say which candidates answer one question. They do not say which questions bear on each other, and two
of the phase's instances turn on exactly that:

- **I-L02**: three candidates, no pair excludes, the three together are dead. Pairs see nothing.
- **I-SEND**: *different letters, different digits* is a matter of pairs. *D + E gives Y* is not: no pair of
  `(val D d)`, `(val E e)`, `(val Y y)` is dead, and most triples are. The by-column declaration order ends in
  17 747 saturations because it guesses the three letters of a column next to each other; the written order
  does not end.

A **coupling** is a set of groups that some conflict or some derivation needs a member of each of. With the
groups as nodes it is a hyperedge, and the result is the groups' **hypergraph**: a CSP's constraint graph, with
a reason on every edge. What a reader gets from it: which questions are independent of which (its components,
which is where R factors), which statement of the program ties which questions (a condition of `zebra2`, a
column of SEND), and what an order should keep together.

The exact object exists already for a program whose describe exhausts: N, the minimal dead sets, and the
members of N with three candidates or more are the couplings no pair shows. But a describe is the expensive
thing, and on I-SEND and I-Z2M it does not finish. So the stage's work is to find a **cheap reading**, and to
say honestly what each one misses.

## The readings to try, cheapest first

| | reading | finds | misses |
|---|---|---|---|
| **1** | **pairs that interact**: S1f.10.1 already has, for every alive pair, whether its fixpoint holds a fact neither single's does. An edge between the groups of the two | every coupling of two groups: `zebra2`'s `co-located` conditions; D with E in SEND, if the pair derives the column's sum, as its `col` relation suggests | a third group that only joins once two are known: Y |
| **2** | **the support of what a pair derives**: for each fact a pair's node adds, the rules that can read it next and the candidate relations those rules still wait for | the third member, as *what this pair's conclusion is waiting for* | anything two steps away |
| **3** | **one node with every alive candidate in**: saturated to quiescence, dead at once and read anyway, as a dead node's debts are; every firing's support in H is a hyperedge of any arity | every coupling one firing shows, in one saturation | couplings that need a chain of firings through derived facts, unless supports are followed back; and its size is unknown until measured |
| **4** | **N**, where a describe exhausts | exactly the minimal conflicts | every program where it does not exhaust |

Reading 4 is the reference the others are held to on the programs it reaches.

## Tasks

1. **T1f.10.11.1 — reading 1, free.** From S1f.10.1's census: the graph of groups joined by an interacting or
   excluding pair of members in different groups, its components, and per edge the stated facts the
   interaction rests on (S1f.10.10's support). For I-Z2 the expected picture is the twelve conditions that
   name two attributes, each an edge or a small set of edges between the groups it names.
2. **T1f.10.11.2 — the controls.** I-L02 must show its triple. I-SEND must show its columns: {D, E, Y},
   {N, R, E} with the carry, and so on to the leading M. Reading 1 is expected to fail both; readings 2 and 3
   are tried in that order until the controls appear, each one's cost in saturations and facts recorded.
3. **T1f.10.11.3 — held to N.** On every corpus program whose describe exhausts within the default budget:
   every member of N with three candidates or more lies inside one hyperedge of the reading taken (no minimal
   conflict spans groups the hypergraph calls unrelated). The converse is not claimed: a coupling may never
   produce a death.
4. **T1f.10.11.4 — the components against R.** Where a describe exhausts, the read-out's `components` row and
   the hypergraph's components are compared, and the stage says what the relation is. If the hypergraph's
   components are finer than R's the reading has missed a coupling, and that is a stop.
5. **T1f.10.11.5 — the record**: the reading taken, what it costs per program, what it provably misses, and
   the hypergraph of every instance under `build/structure/<program>/`, raw and drawn.

## Acceptance

- The definition of a coupling and of the hypergraph is written here, with the reading that computes it and
  that reading's stated blind spot.
- **I-L02 shows one coupling of its three candidates. I-SEND shows its columns.** Or the stage says which of
  the two no reading short of N finds, with the attempts recorded; that is a result, and S1f.10.5 then knows
  what it cannot read.
- T3's test is in the gate, reads nothing past the default budget, and is green.
- The cost of the reading is recorded per program, and beside the pair layer's.
- No answer, route or surface changes.

## What it must not do

- **Not run a describe to get a coupling**, except as T3's and T4's reference on programs where one already
  finishes.
- **Not read a relation by name.** A coupling is what nodes and supports show; `column` and `co-located` are
  the programs' words, not the engine's.
- **Not order anything.** What an order does with the hypergraph is S1f.10.5's.

## Notes

In the literature this is the constraint hypergraph, its components are the independent subproblems, and the
heuristics that read it are the degree heuristics (Brélaz) and the weighted-degree family (*dom/wdeg*), which
learn the weights from failures. [`docs/lib/`](../../../docs/lib/README.md) is where they are catalogued; the
second is the *learned* counterpart of reading 3 and belongs to track R if anywhere.

## Record (2026-11-15)

**Ran to its end, through a stop at T4.** The coupling is computed by the owner, the controls are met, T3
holds on every corpus program a describe exhausts, and **T4's stop fired**: over the edges of conflicts and
derivations the hypergraph's components are finer than `R` on six corpus programs, each by a **debt**, an
instance root owes whose payers no conflict and no derivation ties. The stage stopped there and reported; the
main session took a default, **every debt root owes is an edge of its own kind, licensed by L5 or not**, and
the stage was finished under it ([T4](#t4-the-components-against-r),
[the stop](#the-stop-and-the-default-taken-on-it)). `R` is then the product of its parts over the components
on every compared program. No answer, route or surface changes.

### The definition, as built

- **A node is a question**: a group of the cover, *which of these, if any*, or a candidate in no group, *this
  one, or not* (`structure::Question`). I-L02 has no group and three questions.
- **A coupling is a set of two or more questions that bear on each other**: a hyperedge (`structure::Edge`),
  of one of two kinds (`EdgeKind`). **Some conflict or some derivation needs a member of each of them**, the
  stage's sentence with *groups* read as *questions*; or **some debt needs a member of one of them**, an
  instance root owes whose payers lie in two questions or more (`structure::Debt`), whether or not L5
  licenses it, which is the default taken at T4's stop.
- **Which questions a conflict is about is not in the candidates it reads**, since a group is a clique of a
  cover and a candidate of a grid is in two: `(val D 5)` is a digit of `D` and a letter that is `5`. A **way**
  to a conclusion, the candidates one derivation reads, has one **description** per choice of a question for
  each of them, and the edges are a **cover of the ways by descriptions**: a class of one question before any
  other (such a way is that question's own and is no edge), then the class that describes the most ways not
  yet covered, the one that describes more ways of the whole node among equals, then the one of fewer
  questions, and classes that stand equal in all of that chosen **together**, so that the choice follows no
  numbering. This is what makes SEND's column the letters it adds and not the digits they may be: every way to
  `c1`'s contradiction reads a digit of `D`, of `E` and of `Y`, and no two of them the same three digits.
- **The components** are of the candidates alive alone at root: two are together where they share a question
  or an edge holds a question of each. `Coupling::components` are over every edge and are what `R` is the
  product over; `Coupling::conflict_components`, over the conflicts' and the derivations' edges alone, stand
  beside them.
- **Beside the edges**, under the reading taken, **the edges read one application back**
  (`Coupling::one_step`), off the same node and the same pair layer, which are the finer edges a union of
  labels absorbs.

All of it is `ein_infer::structure::Coupling`, in `ein-infer/src/structure/coupling.rs`, a child module of the
owner: `coupling` for a freshly loaded program, `theory_coupled` for a reader that wants the theory's share
of the same reading, and `pub(crate) coupling_in` over an open `explore::Space`, as `cover_in` and
`exactly_one_in` are, which is what S1f.10.5 reads. Canonical order throughout: a question is its place, the
groups in the cover's order and then the candidates in no group, and every list ascending.

### The readings

| | reading | as built | what it reads |
|---|---|---|---|
| 1 | the pairs | `CouplingReading::Pairs` | the **ties** of root's pair layer: two candidates alive alone, in no question together, that root excludes (and the theory does not), one of which implies the other, or that interact. An edge is the cover's class of the ties |
| 3, no follow-back | the whole node, each application's own premises | `CouplingReading::Direct` | one node with every candidate alive alone committed, saturated to quiescence though dead, every application read; an application that reads a derived fact is not read |
| 2's reach | one application back | `CouplingReading::OneStep` | the same node, each derived premise followed back one application |
| **3, taken** | the whole node, followed back to the candidates | `CouplingReading::Node` | the same node; a derived fact's **label** is one class of questions, the union of the classes its ways' cover chose, settled in rounds |

**Reading 2 as the stage tables it was not built.** *The rules that can read a pair's conclusion next and the
candidate relations they wait for* is, at relation grain, every candidate of the relation, and at fact grain a
join the engine has only as a saturation. The saturation that answers it for every pair at once is reading
3's node, so reading 2's reach is measured there, as `OneStep`. As planned it starts from a pair that
interacts, and I-L02 has none (`jq '.interacting | length' build/structure/examples/lattice/02_genuine_3set_death/pairs.json`,
0): it cannot show the triple whatever it costs.

**What reading the node took in the engine**, none of it on a route: `Firing` carries the facts its
universals read (`universal`; a redundant application's only where the saturator is asked,
`Saturator::read_redundant_universals`, which nothing but this reader calls; `Firing`'s `Debug` form is as it
was), `region::Nodes::with_fixpoint_supported` is the node function's fork with that asked and a bound on
applications, and `explore::Space::supported` reaches it from an open space. The node's derivation graph is
every application's premises, its universals' reads and the activator its plan was compiled from, and beside
them every justification the store kept, which is where a mirror's derivation is.

**Two rules the corpus forced**, each falsified as first written:

- *A way that reads two candidates no alive state holds together is not read*, two of one question or a pair
  root excludes, but for the pair's own contradiction. Without it SEND's `(col c1 0 0)`, which only `D = 0`
  with `E = 0` derives, is described by the one question *who is 0*, and that label reaches every column:
  `OneStep` gave 12 edges where the column is 2, ten of them *a digit's letters with `Y`*.
- *Among classes that describe as many of a fact's ways, the one that describes more ways of the whole node
  goes first.* `(col c2 8 1)` has two ways, `N = 8` with `R = 9` and the reverse, which `{D, E, N, R}` and
  `{D, E, who is 8, who is 9}` describe alike; chosen by the questions' numbering it was the digits, and
  SEND had 86 edges where its columns are 5.

**And one the corpus refused.** Labels kept apart, a fact's label the classes of its cover and not their
union, is the exact reading, and it does not end: on I-Z2 the labels were still moving after 64 rounds with
22 040 edges and 11 075 labels cut at 64 classes, 124 s; at four classes a label they were still moving after
64 rounds on I-Z2 and I-Z2M. A zebra's node derives every exclusion from every other through the clues, round
and round, and a set of sets of questions over that is the minimal-support problem. One class a fact settles
in 8 rounds on both.

### The controls

`cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test coupling`, 2026-11-15; each
row is a test of that file.

| | pairs (1) | direct | one step (2's reach) | **node (3)** |
|---|---|---|---|---|
| **I-L02** | 0 ties, 0 edges, 3 components | **1 edge of the 3 candidates**, 1 conflict, 1 component | the same | the same |
| **I-SEND** | 90 ties, all *interacts*; 1 edge, `{D, E}` | 1 edge, `{D, E}` | 2 edges: `{D, E}` and **`{D, E, Y}`**, each naming `(column c1 D E Y)` | **5 edges**: `{D, E}`, `{D, E, Y}` by `(column c1 D E Y)`, `{D, E, N, R}` by `(column c2 N R E)`, `{D, E, N, O, R}` by `(column c3 E O N)`, `{D, E, M, N, O, R, S}` by `(column c4 S M O)` and `(carried c4 M)` |

**Both controls are met, by reading 3.** Reading 1 fails both, as expected. I-L02 needs no following back: its
one application reads the three candidates. I-SEND's units column needs one step and every other column the
whole node, since a column's carry is derived from the columns below it; each column's edge is its own
letters *with the letters its carry comes from*, which is the stage's *with the carry*, and the edges nest
`{D, E}` ⊂ `{D, E, N, R}` ⊂ `{D, E, N, O, R}` ⊂ `{D, E, M, N, O, R, S}` beside `{D, E, Y}`. No digit's
question, *who is 5*, is in any edge. Each edge's most named statement is the column's own fact, read by
the last application of each way; the digit table's rows come after it, once each.

### What it costs

Taken again with the final binary at 00:23 on 2026-11-16, the working tree over `8b7efbde`, on AC, one
P-core (`cpu4`), governor `powersave` (`build/structure/s11/instances.env`). Work in the read-out's units; the pair layers are root's and the small
KB's together; the node's wall is the whole reading under `node` less the whole reading under `pairs`, two
runs, so a few ms either way is noise.

```sh
utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer --routes --readings \
    --out build/structure/s11/out examples/zebra.ein examples/zebra2.ein examples/zebra2-minus-15.ein \
    examples/branching/06_lookahead_on.ein examples/lattice/02_genuine_3set_death.ein \
    examples/beyond_arith/send_more_money.ein \
    > build/structure/s11/instances.jsonl 2> build/structure/s11/instances.env
```

| instance | pair layers: saturations / facts / wall | **the node**: saturations / facts / applications / wall | a describe: saturations / facts / wall | `--single`: saturations / wall |
|---|---|---|---|---|
| I-Z1 | 2 167 / 248 332 / 2 374 ms | 1 / 393 / 5 307 / within the noise | 445 / 106 829 / 1 125 ms | 49 / 189 ms |
| I-Z2 | 2 062 / 100 471 / 1 313 ms | 1 / 182 / 2 509 / within the noise | 374 / 42 705 / 629 ms | 45 / 108 ms |
| I-Z2M | 9 203 / 567 437 / 6 877 ms | 1 / 222 / 2 564 / within the noise | 8 391 / 796 678 / 10 548 ms, **not exhausted** | 39 / 47 ms |
| I-B06 | 8 891 / 24 948 / 45 ms | 1 / 2 / 2 / 0 ms | 133 / 440 / 2 ms | 8 / 0 ms |
| I-L02 | 14 / 18 / 0 ms | 1 / 4 / 1 / 0 ms | 8 / 13 / 0 ms | 2 / 0 ms |
| I-SEND | 6 325 / 13 937 / 117 ms | 1 / 162 / 2 784 / 6 ms | 336 101 / 1 422 943 / 14 341 ms, **not exhausted** | 17 747 / 814 ms |

**The node is one saturation and the pair layers are thousands.** What the coupling costs beyond the cover is
the node, which over three takings of this table was between the noise of two runs and 8 ms on the zebras
and 6 or 7 ms on I-SEND; the readings `direct` and `one step` cost the same saturation and read
less. **The node is charged to no budget**: it is no node of the routes' cache
(`explore::Space::supported`), it is not read where its facts beyond root pass the layer's budget or its
applications 2^20 (`NodeStop`), and `Coupling::node` is its count, which is what
[S1f.10.13](s1f.10.13_the_price_list.md) prices. The edges one application back are read off the same node
and cost no second saturation and no second pair layer. A program's line of `build/structure/census.jsonl` has all
of it under `.theory.coupling`.

### The corpus

The census, the one command of S1f.10.10, S1f.10.2 and S1f.10.4 with the coupling asked under every reading,
taken with the final binary, the release example rebuilt from the tree as it stands, from 00:16 on 2026-11-16,
on AC, one P-core (`build/structure/census.env`); every program's `coupling.json`, `coupling.md` and
`coupling.dot` are that run's, and the census as S1f.10.4 left it is
`build/structure/s11/census_before.jsonl`:

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer -p ein-cli
utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer --routes --check --readings \
    > build/structure/census.jsonl 2> build/structure/census.env
jq -s -f build/structure/s11/coupling.jq build/structure/census.jsonl
```

Over the 188 corpus programs that search, each with a coupling (`coupling.jq`, whole below):

- **173 read whole**, and on 15 no candidate is alive alone, so there is no node to commit to. No bound cut
  anything, and no program's labels were left unsettled; the most rounds are 38
  (`beyond_arith/equation_two_unknowns.ein`, whose node is also the largest, 17 620 facts). 29 are flagged.
- **2 010 questions**, 496 groups and **1 514 candidates in no group**.
- **19 090 ties** on 62 programs: 3 484 *excludes*, which are the corpus's 3 484 root-excluded pairs that are
  not the theory's ([S1f.10.10](s1f.10.10_the_theorys_share.md#record-2026-11-15)), each between two questions
  and none inside one; 11 430 *implies*; 4 176 *interacts*.
- **13 719 edges** on 78 programs: 13 623 the pairs give, 106 the node gives and **29 a debt's**, each of
  those a debt's alone; **74 of three questions or more, on 45 programs**, 63 of them a conflict's or a
  derivation's and 11 a debt's, the largest of 130 (`ein-bugs/wrong-answers/count-past-2-128.ein`).
- **Reading by reading**, edges / of three or more / programs with one of three or more / components summed:
  pairs 13 652 / 11 / 1 / 665; direct 13 655 / 14 / 4 / 659; one step 14 846 / 870 / 37 / 629; **node 13 719 /
  74 / 45 / 623**; the 11 of three or more under the pairs are the debts' on `zebra.ein`, which every reading
  has. So the node finds a conflict's or a derivation's coupling of three or more on 45 programs where the
  pairs find none on any, and **it lists fewer of them than one step does on the programs both reach**:
  followed to the candidates a fact's label is the union of its ways' classes, and on a program whose
  derivations go round the node's edges come to one edge of every question the clues reach (below, I-Z2).
  **The value keeps both**: `Coupling::one_step` under the reading taken is the one-step reading's edges on
  all 188, by arity and by what gave them (`one_step_kept.same_as_the_one_step_reading`).
- **Components**: 623 over every edge and 637 over the conflicts' and the derivations' alone; 54 programs
  have two or more, 119 one, and **a debt joins components on 6**.
- **Debts**: **36 are edges, on 7 programs**, over 29 sets of questions; 18 of them, on 6 programs, have
  payers in two of the conflicts' components. **19 are of an instance L5 does not license**, 18 on `zebra.ein`
  and 1 on `ein-bugs/wrong-answers/oblig-fresh-name-two-declared.ein`; L5 does not license 30 instances in
  all, on 9 programs, and the other 11 have their payers read in one question, or none read.
- **Work**: the pair layers 131 604 saturations and 1 903 721 facts beyond root; the nodes **173 saturations
  and 21 091 facts**, 68 126 applications.

**What moved when the debt became an edge**, against the census of the first version of this record: the
edges, 13 690 to 13 719, and each reading's by the same 29; those of three or more, 63 to 74, the 11 a
debt's; the programs with an edge, 72 to 78; the components summed, 637 to 623 under the node (679, 673 and
643 to 665, 659 and 629 under the three others), with the 637 now the conflicts' components; the programs
with one component, 115 to 119, and with several, 58 to 54. The questions, the ties, the conflicts' and the
derivations' edges, the nodes' work and the pair layers' did not move. (Under the main session's first
instruction, licensed instances alone, they were 13 707 edges, 17 debts on 5 programs and 624 components.)

### T3: held to N

`ein-infer/tests/coupling.rs`, `the_coupling_is_held_to_n_and_to_r`, in the gate, on the worker pool, at the
default budget and nothing past it (`cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer
--test coupling -- --nocapture`, taken again on 2026-11-16 with the debts' edges in, 6.4 s at 16 workers for
the file's 8 tests):

- **585 programs compared**: every corpus file and register witness that loads, is monotone and whose
  describe exhausts, but the `slow` entries; 22 more are flagged and not compared. Under `EIN_CORPUS_SLOW=1`,
  587.
- **N: 1 642 members of two candidates, and 1 141 of three or more on 18 programs. Every one lies inside one
  edge of a conflict or a derivation**: an edge holds a question of each of its candidates, and a debt's edge
  alone is not read for it (`Coupling::edge_over`). The claim was planned for three or more; it holds of two
  as well, a pair being either inside one edge or two members of one question. None of these counts moved
  when the debt became an edge.
- **The pairs' blind spot, measured**: 1 091 of the 1 141, on 16 programs, hold two candidates that no edge
  of the pair layer holds a question of each of.
- **The converse is not claimed**, and is false: an edge may be of a derivation that ends in no death.
- **It fails when the node is not read.** With the node's edges taken out the test reports 1 157
  disagreements and four of the file's tests fail (tried on the final tree and reverted, 2026-11-16; 1 149
  on the first version).

Why it holds, where it does: on a monotone program every application of a node of alive candidates is an
application of the whole node, so the derivation of a minimal dead set's contradiction is one of the ways
read; no two of the set's candidates clash, the set being minimal, so the way is not dropped; and the class
chosen for it, or the coarser one a label's union gives, holds a question of each candidate it reads, which
are all of the set, again by minimality. The slow half is a nightly step and the dev host's, 44.1 s on four
P-core threads with four workers, on AC (`EIN_CORPUS_SLOW=1 EIN_TEST_JOBS=4 utils/bench_env.sh --cores PT:4
--require-ac cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test coupling --
the_coupling_is_held_to_n_and_to_r`, 2026-11-16; 587 programs, N 2 033 members of two candidates and 1 159
of three or more on 19 programs).

### The instances

Each is under `build/structure/<program>/`: `coupling.json`, raw, `coupling.md`, the same as tables, and
`coupling.dot`, the hypergraph, an edge of two questions a line and an edge of more a point joined to each;
so is every other corpus program, written by the census.

Edges by arity under the reading taken, and one application back, which the same value holds
(`jq -c '.theory.coupling | [.edges.by_arity, .one_step.by_arity]' build/structure/s11/instances.jsonl`, each
entry `[arity, edges, of the pairs, of the node, of a debt]`):

| instance | **the node**: edges by arity | **one step back**: edges by arity |
|---|---|---|
| I-Z1 | 374 of two (all the pairs'), 1 of 32 (the node's), and 11 a debt's, of 9 to 22 | 387 of two (374 the pairs', 41 the node's), 20 of three, 4 of four, 2 of five, and the same 11 a debt's |
| I-Z2 | 392 of two (all the pairs'), 1 of 27 | 406 of two (392 the pairs', 62 the node's), 56 of three, 17 of four, 10 of five, 5 of six, 1 of seven |
| I-Z2M | 354 of two (all the pairs'), 1 of 45 | 490 of two (354 the pairs', 222 the node's), 131 of three, 30 of four, 17 of five, 10 of six, 10 of seven, 7 of eight |
| I-SEND | 1 of two, 1 of three, 1 of four, 1 of five, 1 of seven | 1 of two, 1 of three |

I-Z2, I-Z2M and I-SEND have no debt's edge: every instance's payers are one question. **I-Z1 has 18 debts
that are edges, all of instances L5 does not license**, `slot-owed-fill`'s, over 11 sets of 9 to 22
questions: the payers read of one are the candidates whose own node holds one of its witnesses, and they
lie across that many questions. They join nothing there, I-Z1 being one component by its conflicts.

- **I-L02**: 3 questions, each a candidate; one edge of the three; one component.
- **I-SEND**: 18 questions, the 8 letters and the 10 digits; 5 edges, all of letters, the four columns and
  `{D, E}`; one component. The ten digits' questions are in no edge: *different letters, different digits* is
  inside each of them.
- **I-Z2M**, the under-determined one: 46 questions, 2 835 ties (1 110 *excludes*, 111 *implies*, 1 614
  *interacts*), 354 edges of two questions and one of 45. **Of the 11 statements that name two values, 7 have
  an edge of exactly their own two questions**, *where is the one* with *where is the other*
  (`jq -r -f build/structure/s11/conditions.jq build/structure/examples/zebra2-minus-15/coupling.json`):
  `Kools`/`Yellow` and `Old_Gold`/`Snail` each cover 20 exclusions and 5 implications, the whole of a
  *co-located*; `Englishman`/`Red`, `Japanese`/`Parliament`, `Spaniard`/`Dog` and `Lucky_Strike`/`Juice` 12
  and 3 or 4, and `Chesterfields`/`Fox` 17 and 4. The other four have none, and each is the statement
  most named on other edges (`Coffee`/`Green` on 36).
- **I-Z2**, the control: 36 questions, 535 ties, 392 edges of two questions and one of 27. **The twelve
  conditions do not appear as an edge each.** Four have an edge of their own two questions
  (`Coffee`/`Green`, `Englishman`/`Red`, `Ukrainian`/`Tea`, `Old_Gold`/`Snail`), each covering one or two
  exclusions and implications; eight have none. Root has applied every clue: 21 of the 56 candidates are dead
  alone, 535 of the 595 pairs of the 35 left are ties, and a pair root excludes is the clues' joint
  consequence, so the pair layer's edges are those of a puzzle nearly decided and `Englishman`/`Red` is the
  statement most named on 78 edges. The stage's expected picture is I-Z2M's and not I-Z2's. The node's one
  edge is of 27 of the 36 questions, for 979 ways to a contradiction: it says little more than the one
  component does, and the 89 edges of three to seven questions one step back are the finer reading
  (`Coupling::one_step`; `build/structure/examples/zebra2/coupling.md` lists them under their own heading).
- **I-Z1**: the same 36 questions under the renaming, 629 ties, 374 edges of two questions and one of 32.
- **I-B06**: 52 questions, the 10 groups and 42 candidates in no group; 130 of the 132 candidates are dead
  alone, the two left imply each other, and that is the one tie and the one edge.

### T4: the components against R

T4 as written compares the read-out's `components` row with the hypergraph's components. They are not
comparable as they stand: the read-out's are the components of `R`'s **commitments** under one-fact edges
(`described::Shape::components`), and the hypergraph's are sets of **candidates**. What *finer than `R`*
has to mean is the sentence the stage's context gives the components, *where `R` factors*: **`R` is the
product of its parts over the components**, so that for two commitments of `R` and a component, the one's
candidates inside it with the other's outside it are a commitment of `R`. That is what the test reads, on
every state's closure and every generator, which decides it for all commitments where dead is upward-closed
and discharged is too.

**Over the conflicts' and the derivations' edges alone it fails on six programs**, and that was the stage's
stop: `ein-bugs/wrong-answers/owed-seats.ein`, `owed-seats-renamed.ein`, `uncovered.ein`,
`uncovered-knows.ein` and `oblig-fresh-name-two-declared.ein`, and `lattice/synth/trivial_line.ein`. The
reading had missed nothing it could read: on each the mix is alive and **owes**. `owed-seats.ein`: `(seats
Ann C1)` and `(seats Bob C2)` are tied by no conflict and no derivation, each is a candidate in no group and
a component of its own, and `R` holds no commitment that seats Bob and not Ann, because root owes Ann a seat.
A debt needs *one of* its payers, which is neither *a conflict needs a member of each* nor *a derivation
needs a member of each*.

**As built, on the main session's default** ([Defaults taken](#defaults-taken), 5): **every debt root owes is
an edge of its own kind.** Every instance of L5's table whose payers alive alone lie in two questions or more
is an edge of their questions, its reason the instance (`structure::Debt`, `EdgeKind::Debt`); the edge set
does not depend on what the other edges join. One whose payers are all in one question is no edge: a letter's
ten digits are one question already, and that is an owed group's *exactly one*. **Whether L5 licenses the
instance does not decide it**, and the edge says which (`Debt::licensed`): the licence says whether a search
may confine itself to the alternatives, not whether root owes, and `R` is discharged either way. A licensed
instance's payers are L5's alternatives; an unlicensed one's are **the payers read**, the candidates alive
alone whose own node holds one of its witnesses. `Coupling::components` are over every edge, and
`Coupling::conflict_components` over the conflicts' and the derivations' alone stand beside them, with
`Coupling::debts_across`, the debts whose payers lie in two of those.

**The blind spot of an unlicensed edge**: its payers are only those a single candidate's node shows. A
witness that only two candidates together bring is not seen, nor one that is no candidate and that nothing
brings alone, so the edge may be too small. Components too coarse claim no independence that is false, and
components too fine would, which is why the unlicensed instance is an edge all the same; **nothing follows
for a search from an unlicensed edge beyond *do not take these questions for independent***.

The test, `the_coupling_is_held_to_n_and_to_r`, on the per-commit set (the command of T3, 2026-11-16):

- **`R` is the product of its parts over `components` on every compared program**, with no exception: all 41
  that the conflicts and the derivations leave two components or more of; on 37 of them there are still two
  or more over every edge.
- **Over the conflicts' components alone `R` is no product on exactly the six**, and on each a debt's payers
  lie in two of them; the test fails if it is none anywhere that no debt spans two.
- **The debts**: 36 are edges, on 7 programs, 19 of them of an instance L5 does not license, on 2; 18 have
  payers in two of the conflicts' components, on the six.
- **It fails when the debts' edges are not read.** With them taken out the test reports the six programs,
  each twice, the component finer than `R` and no debt across, and `a_debt_is_an_edge` fails; with the
  node's edges taken out it reports 1 157 disagreements and four tests fail (each tried on the final tree
  and reverted, 2026-11-16).

`oblig-fresh-name-two-declared.ein` is the one of the six whose debt L5 does not license, in L5's words *it
reads `food` beside its witness, a relation a commitment can change* (the rule `special` derives a `food` at a
node). Its payers read are `(likes Bob Soup)` and `(likes Bob Z)`, two candidates in no group, and `R` holds
no commitment with neither.

### The stop, and the default taken on it

The stage stopped at T4 and reported. Three ways were put:

1. *an edge of its own kind*, the questions of an owed instance's payers, where they are not one question
   already;
2. *listed beside the edges and not one*: the hypergraph is of what conflicts and derivations tie, as
   *exactly one* stands beside the cover;
3. *the question is the debt*: a licensed instance's alternatives are a question whether or not they
   exclude, which changes what a node of the hypergraph is and reaches back into S1f.10.4's group.

**The main session took (1), as a default for the user to see at the phase's stop**, 2026-11-15, not the
user: the stage gives the hypergraph's purpose as *which questions are independent of which (its components,
which is where `R` factors)*; `R` is the commitments alive and discharged, so an instance root owes ties its
payers in `R` as a conflict does; (3) would move S1f.10.4's definition of a group; (2) leaves `components`
meaning something `R` does not factor over. With it: every instance root owes whose payers lie in two
questions or more, not only those across two components, and the conflicts' components kept beside.

**Licensed or not.** The main session's first instruction left an instance L5 does not license no edge, *L5
licenses nothing there*, and withdrew it on 2026-11-16 when the tree built to it left `R` no product over the
components on `oblig-fresh-name-two-declared.ein`: the licence is about what a search may do and not about
what root owes, and components too fine claim an independence that is false, which S1f.10.5 would read.

### Alternatives rejected

| alternative | what falsified it |
|---|---|
| a hyperedge is the union of every group of every candidate a conflict reads | I-SEND: `c1`'s conflicts are then `{D, E, Y}` with three digits' questions, a different edge for every three digits, and no edge is the column |
| labels kept apart, a fact's label the classes of its cover | I-Z2 and I-Z2M: unsettled after 64 rounds at 64 classes and at 4; 22 040 edges and 124 s on I-Z2 |
| a way is read whatever candidates it holds | I-SEND under one step: 12 edges, ten of them *a digit's letters with `Y`*, by the one fact only `D = 0` with `E = 0` derives |
| among classes that describe as many ways, the lower in the questions' numbering | I-SEND: `(col c2 8 1)` has two ways, `N = 8` with `R = 9` and the reverse, which `{D, E, N, R}` and `{D, E, who is 8, who is 9}` describe alike; 86 edges where there are 5 |
| the lower in the numbering, where classes stand equal in everything | the numbering is the universe's order, which a name's spelling moves: classes that stand equal are chosen together, and the coupling is a function of the program (`structure_invariance.rs`) |
| reading 1 alone | I-L02, and 1 091 members of N on 16 corpus programs |
| reading 2, one step back | I-SEND's columns `c2` to `c4`; it lists 859 edges of three or more where the node lists 63, and its components are finer than the node's on the corpus, 643 against 637, so it misses couplings the node has |
| an edge only of a conflict or a derivation, the components where `R` factors | six programs, each by a debt (T4) |

### Defaults taken

Each the user's to revise.

1. **A node is a question: a group, or a candidate in no group**, alive alone or not. I-L02 has no group and
   its triple has to be an edge of something.
2. **An edge is a class of the ways' cover**, not the union of every group of every candidate read.
3. **The reading taken is the pairs with the whole node, followed back to the candidates, a fact's label one
   class.** Coarse where a fact has ways of two kinds, and it settles; the exact reading does not.
4. **The whole node commits the candidates alive alone at root.** One root holds is there already, and one
   dead alone is in no alive state.
5. **Every debt root owes is an edge of its own kind, licensed by L5 or not.** Taken by the **main session,
   not by the user**, at T4's stop, for the user to see at the phase's stop, over the two other ways put,
   *listed beside the edges* and *the question is the debt*
   ([the stop](#the-stop-and-the-default-taken-on-it)). The main session first left an unlicensed instance no
   edge and withdrew that, since `R` is discharged whether or not L5 licenses and components too fine claim a
   false independence. It joins the components of the six programs the stop named: `owed-seats.ein`,
   `owed-seats-renamed.ein`, `uncovered.ein`, `uncovered-knows.ein`, `lattice/synth/trivial_line.ein` and, by
   an unlicensed instance's payers as read, `oblig-fresh-name-two-declared.ein`.
6. **T4 is the product of `R` over the components**, since the read-out's `components` are of commitments and
   are not comparable with sets of candidates. The plan's wording is kept as a contradiction, in T4.
7. **T3 is held of members of two candidates as well.**
8. **A statement is the puzzle's where the small KB drops it**, S1f.10.10's rule, and a premise root holds
   names the statements the derivation root recorded *first* for it reaches. So SEND's edges name the digit
   table's rows beside the column, once each: the rule drops them (K-5).
9. **The bounds are constants of the module**: 2^20 applications of the node, 64 rounds, 256 ways an
   application, 1 024 descriptions a way. Past the last two the reading is coarser and says so; past the
   first the node is not read. No lever reaches any, and no corpus program that searches meets one.
10. **The node is charged to no budget and not read past the layer's.**
11. **The slow half of `coupling.rs` is a nightly step and the dev host's**, as `pair_layer`'s is (K-2's
    default).
12. **The engine was touched in three places, none on a route**: the saturator keeps what an application's
    universals read where asked (`read_universals`, `take_universal`), `region::Nodes::with_fixpoint_supported`
    and `explore::Space::supported`. `explore`, `region` and `claim_routes` are green with nothing edited.
13. **The per-edge statements of a tie come from its support only where supports are asked**
    (`CouplingOptions::supports`; the census asks them), since a support costs its node a second time (K-7).

### The commands

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --lib --test cover \
    --test structure_invariance --test pair_layer --test theory_share --test exactly_one --test explore \
    --test region --test coupling --test claim_routes
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test nightly_steps --test gate_steps \
    --test config_reference
```

All green on 2026-11-16, on the tree as it stands, with `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `utils/check_hashmap_iteration.py`
and `utils/doc_audit.py --links --check` with the gate's argument list. `structure_invariance.rs` holds the
coupling beside the cover: place for place under a permuted id space on the 175 programs that search, as sets
of texts under every import form on the 94 forms of the 47 that import, on all 188 and 106 under
`EIN_CORPUS_SLOW=1` (57.6 s on four P-core threads with four workers, on AC, by the command above with that
target), and on the program written there, which a form reorders and which now ties two of its questions by
one stated fact and two by two debts, one edge of each kind. What is compared of the coupling: the questions,
the ties, the edges with their counts, their debts and their statements, the edges one application back, the
components over every edge and over the conflicts' alone, the debts and the instances L5 does not license.

`coupling.rs` has two controls more since the default: `a_debt_is_an_edge`, on `owed-seats.ein`, each debt an
edge of its payers' questions, of the debt's kind alone, read by no conflict, with
`oblig-fresh-name-two-declared.ein`'s one debt, unlicensed, an edge of its two payers read, and SEND's
letters, whose payers are one question, having none; and `the_node_keeps_the_edges_one_step_back`, the value's `one_step`
against the one-step reading's edges on four programs, and more edges of three or more than the node's own on
I-Z2.

`build/structure/s11/coupling.jq`:

```jq
# jq -s -f build/structure/s11/coupling.jq build/structure/census.jsonl
def c: .theory.coupling;
def count(f): map(select(f)) | length;
def total(f): map(f) | add // 0;
map(select(has("error") | not)) as $read
| ($read | map(select(.searches))) as $s
| ($s | map(select(c != null and (c | has("error") | not)))) as $c
| {
  read: ($read | length),
  searching: ($s | length),
  with_a_coupling: ($c | length),
  no_coupling: ($s | map(select(c == null or (c | has("error")))) | map(.path)),
  complete: ($c | count(c.complete)),
  flagged: ($c | count(c.domain != null)),
  node: ($c | map(c.node_stopped // "read") | group_by(.) | map({(.[0]): length}) | add),
  cut: {ways: ($c | count(c.cut.ways > 0)), descriptions: ($c | count(c.cut.descriptions > 0)),
        unsettled: ($c | count(c.cut.unsettled))},
  questions: {all: ($c | total(c.questions)), alone: ($c | total(c.alone))},
  ties: {all: ($c | total(c.ties.all)), excludes: ($c | total(c.ties.excludes)),
         implies: ($c | total(c.ties.implies)), interacts: ($c | total(c.ties.interacts)),
         programs: ($c | count(c.ties.all > 0))},
  edges: {all: ($c | total(c.edges.all)), of_the_pairs: ($c | total(c.edges.of_the_pairs)),
          of_the_node: ($c | total(c.edges.of_the_node)),
          of_a_debt: ($c | total(c.edges.of_a_debt)), of_a_debt_alone: ($c | total(c.edges.of_a_debt_alone)),
          of_three_or_more: ($c | total(c.edges.of_three_or_more)),
          programs: ($c | count(c.edges.all > 0)),
          programs_with_one_of_three_or_more: ($c | count(c.edges.of_three_or_more > 0)),
          largest: ($c | map(c.edges.largest) | max)},
  one_step_kept: {edges: ($c | total(c.one_step.all)), of_three_or_more: ($c | total(c.one_step.of_three_or_more)),
                  programs_with_one_of_three_or_more: ($c | count(c.one_step.of_three_or_more > 0)),
                  same_as_the_one_step_reading: ($c | count(c.one_step.by_arity == c.readings["one-step"].edges.by_arity))},
  by_reading: (["pairs", "direct", "one-step"] | map({(.): (. as $r | {
          edges: ($c | total(c.readings[$r].edges.all)),
          of_three_or_more: ($c | total(c.readings[$r].edges.of_three_or_more)),
          programs_with_one_of_three_or_more: ($c | count(c.readings[$r].edges.of_three_or_more > 0)),
          components: ($c | total(c.readings[$r].components))})}) | add),
  components: {all: ($c | total(c.components)), none: ($c | count(c.components == 0)),
               one: ($c | count(c.components == 1)), several: ($c | count(c.components > 1)),
               of_conflicts: ($c | total(c.conflict_components)),
               programs_a_debt_joins: ($c | count(c.conflict_components > c.components))},
  debts: {edges: ($c | total(c.debts)), programs: ($c | count(c.debts > 0)),
          across_conflict_components: ($c | total(c.debts_across_conflict_components)),
          programs_with_one_across: ($c | count(c.debts_across_conflict_components > 0)),
          unlicensed_edges: ($c | total(c.debts_unlicensed)),
          programs_with_an_unlicensed_edge: ($c | map(select(c.debts_unlicensed > 0)) | map([.path, c.debts_unlicensed])),
          unlicensed_instances: ($c | total(c.unlicensed)), programs_with_one_unlicensed: ($c | count(c.unlicensed > 0))},
  work: {
    pair_layers: {saturations: ($c | total(.saturations + (.theory.small.saturations // 0))),
                  facts: ($c | total(.facts + (.theory.small.facts // 0)))},
    node: {saturations: ($c | total(c.node.saturations // 0)), facts: ($c | total(c.node.facts // 0)),
           applications: ($c | total(c.node.applications // 0)), steps: ($c | total(c.node.steps // 0)),
           most_facts: ($c | map(c.node.facts // 0) | max), most_rounds: ($c | map(c.node.rounds // 0) | max),
           merged: ($c | total(c.node.merged // 0))}
  }
}
```

`build/structure/s11/conditions.jq`:

```jq
# jq -r -f build/structure/s11/conditions.jq build/structure/examples/zebra2/coupling.json
# Each statement of two values: is there an edge of exactly the two questions
# "where is the one" and "where is the other", and what does it cover?
. as $c
| [.edges[].stated[][0]] | unique
| map(select(test("^\\((co-located|adjacent-via) ")))
| map(. as $s
    | ($s | ltrimstr("(") | rtrimstr(")") | split(" ")) as $w
    | ($w[-4:]) as $v
    | ["(\($v[0]) \($v[1]) *)", "(\($v[2]) \($v[3]) *)"] as $q
    | ($c.edges | map(select((.questions | sort) == ($q | sort))) | first) as $e
    | [$s, (if $e then "an edge" else "no edge" end),
       ($e.pairs // {} | "\(.excludes // 0)/\(.implies // 0)/\(.interacts // 0)"),
       ($c.edges | map(select(.stated[0][0] == $s)) | length)]
    | @tsv)
| .[]
```

### What was not done

- **The per-edge statements of an interacting tie.** A support is of a death; a tie that interacts or
  implies has none, and no reader was added for the support of a derived fact. An edge the node gives names
  its statements whatever its ties are, by the last application of each way.
- **A reading that shows one clue as one edge on a puzzle root has decided.** The small KB with one of the
  puzzle's statements put back, read as a pair layer, would give that statement's own ties; it costs a pair
  layer a statement and was not built or measured.
- **Reading 2 as tabled**, statically, for the reason above.
- **Flagged programs are not held to anything**: 22 of the programs whose describe exhausts are flagged, and
  the test counts them and compares nothing on them.
- **A sensitivity check of the invariances by mutation** was not run for the coupling, as it was for the
  cover (a cut cover, K-8): the comparisons are the cover's own, and `a_structure_that_moves_is_caught` holds
  that an edge lost is caught both place for place and as sets, and a component in another order place for
  place only. The corpus's forms reorder a universe in 2 of 106, as K-8 says, and the program written there
  has one edge.
- **The edges at every depth between one step and the whole node** are not kept: `Coupling::one_step` is
  the one reading short of the node that the value holds beside the node's own.
- **A witness of an unlicensed instance that only two candidates together bring** is not read for its
  edge's payers: no corpus program a describe exhausts needs it for the product, and none was written to show
  it.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh` were not run by this stage: one gate for the
  batch is the coordinator's.
