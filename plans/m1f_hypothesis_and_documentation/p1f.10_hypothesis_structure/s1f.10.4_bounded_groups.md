# S1f.10.4 — Exactly one: owed groups and grids

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** **ran 2026-11-15**: [§ Record](#record-2026-11-15). *Exactly one* is computed by the owner: an owed
group is a group whose members are exactly the alternatives of an instance L5 licenses, and a grid is found from
the cover alone. The control holds: on I-Z2 the five grids are the five declared relations, and all 36 of their
rows and columns are owed groups. Where the stage file's wording assumed root's exclusion graph it is read under
the definition of a group in force ([§ How the stage file is read](#how-the-stage-file-is-read-under-the-definition-in-force));
what is this stage's to default is [§ Defaults taken](#defaults-taken), the user's to revise at the phase's stop.
**Depends on:** [S1f.10.2](s1f.10.2_groups.md), the cover and what each program declares beside it.
**Read by:** [S1f.10.12](s1f.10.12_the_structure_shown.md); of track R,
[S1f.10.14](s1f.10.14_exactly_one_spent.md), which spends what this stage finds.

## Context

The instruction's second half:

> Next step is somehow functionally find that there are bounded hyp groups, e.g. permutations of pets,
> permutations of nationalities.

A group says *at most one*. A **bounded** group says *exactly one*, and that is a different and stronger claim:
it is what makes a set of alternatives jointly exhaustive, which is what L5 needs to branch on them alone
([`tree.rs`](../../../ein.rs/crates/ein-infer/src/tree.rs), § L5).

*At most one* is a negative and comes free from the dead pairs. *Exactly one* is a positive: some member
**must** hold, and no amount of pairwise refutation implies that. In the engine as it stands the *must* has one
source, an **obligation**: a state that owes an instance is not in R until a witness arrives, and
`oblgen::alternatives` reads at root the facts that can witness each instance and whether L5's premise holds of
it.

So this stage is a reading of three things against each other, per program:

| | what | from |
|---|---|---|
| **owed** | the instances root owes, each with its alternatives, licensed or not, and the reason where not | `oblgen::Alternatives` |
| **excluding** | the groups | S1f.10.2 |
| **grid** | two families of groups over one member set, every member in one group of each: rows and columns | computed here |

and the objects it names:

- an **owed group**: an owed instance whose alternatives are pairwise excluding. *Exactly one.*
- an **owed set that is not a group**: an instance whose alternatives do not exclude each other. *At least
  one.*
- a **group nothing owes**: *at most one*, and the program does not say one must hold.
- a **grid**, and whether each of its rows and columns is owed: a bijection when all are.

**The instances already disagree in instructive ways.** I-Z2 declares five bijections and root owes 36
instances of two kinds, a value owed a house and a house owed a value. I-Z1 gets its debts from `std.slots`, and the tree licenses the
`slot-owed-room` instances and not the `slot-owed-fill` ones, whose witnesses are the symmetric
`(co-located House-n v)` that the hrule never proposes: half of each grid is owed and unspendable. I-B06
declares `functional` and `sibling-exclusive` as plain rules and owes nothing: its groups are *at most one*
with no *must* anywhere.

## Tasks

1. **T1f.10.4.1 — the table of the three**, for every corpus program that searches: owed groups, owed sets
   that are not groups, groups nothing owes, grids and how much of each is owed. For the instances, the lists.
2. **T1f.10.4.2 — every bijection, found or declared.** A program that states `(bijective R)` and a program
   whose groups form a fully owed grid over `R`'s candidates are compared: found and declared; declared and
   not found, with what at root broke the grid; found and declared nowhere. I-Z2 is the control and must be
   all *found and declared*.
3. **T1f.10.4.3 — where the *must* is missing, what would supply it.** Three routes, as analysis and not as
   something this stage builds:

   | route | the claim | where the *must* comes from |
   |---|---|---|
   | **counting** | a grid of *n* rows and *n* columns over *n* objects | the pigeonhole, which needs the object list to be complete: nothing may introduce another value. The alive-set invariant gives that of names ([`invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs)); whether it gives it of a *type's* members is the question |
   | **what is declared, read more widely** | `functional` and `total` together, without the word `bijective`; [`stdlib/closure.ein`](../../../stdlib/closure.ein) concludes `(__closed__ R)` from that pair | a declaration the program already carries under another name |
   | **the obligation** | the group is what an obligation rule would owe, were it stated | zero new machinery; the program has to say it |

   For each instance with a group nothing owes: which routes apply, and for the third, the lines the program
   would have to add, written out and run by `ein oracle` on a copy to show the R it gives.
4. **T1f.10.4.4 — the unlicensed half.** For every owed instance L5 does not license, the reason in the
   program's terms. For I-Z1's `slot-owed-fill`: each witness is in the closure of a candidate (its symmetric
   twin), which S1f.10.1's *implies* shows. Whether a witness that a candidate implies can stand for the
   candidate is an argument for S1f.10.14; this task supplies the census: how many unlicensed instances in the
   corpus have every witness implied by a candidate.

## Acceptance

- T1's table and T2's comparison are recorded here for the corpus, and for the instances in
  `build/structure/<program>/`.
- **On I-Z2 every row and every column of five grids is an owed group.** On I-Z1 the stage says exactly which
  halves are owed, which are licensed, and what is missing for the rest.
- **Where exactly one cannot be derived, the stage says so and says what the program would have to state.** A
  written *not without a declaration* is a full result.
- Nothing is synthesised: no rule, no obligation, no closure. The stage reads.
- No answer, route or surface changes.

## What it must not do

- **Not conclude *exactly one* from *at most one* and a count** without the premise written and checked. *An
  invariant that is the warrant for a skip and is enforced nowhere* is the defect this repo has already had
  once (the alive-set invariant before `invariant.rs` evaluated it).
- **Not read `is-a`, `bijective` or any stdlib name inside the engine** to recognise a declaration. The
  comparison with what is declared is the census's, by the mechanisms the engine already reads.

## Record (2026-11-15)

The stage ran. The control came first and holds, so the four tasks ran: **on I-Z2 the cover alone finds five
grids, one a declared relation, and every one of their 36 rows and columns is an owed group**. The working tree
over `ad5a4fc5`, on AC; every number below is a count unless it is called a wall.

### How the stage file is read under the definition in force

The stage file was written when a group was a clique of root's exclusion graph. A group is now a maximal clique
of the exclusions the theory makes alone, over all of H
([S1f.10.2 § Defaults taken](s1f.10.2_groups.md#defaults-taken), 3), and four of its sentences are read so:

| the stage file | read as |
|---|---|
| *an owed group: an owed instance whose alternatives are pairwise excluding* | an instance L5 licenses whose alternatives are **exactly a group's members**: pairwise excluding by the theory alone, and maximal. A member dead alone at root is an alternative like any other. Alternatives that pairwise exclude and are strictly inside a group are kept apart (`Owing::Inside`); the corpus has none |
| *two families of groups over one member set* | the member set is a **connected component of the theory's exclusion graph**, one of `Cover::components` |
| *on I-Z2 every row and every column of five grids is an owed group* | over all 56 candidates of H, the 21 dead alone at root among them, as S1f.10.2's control was reworded |
| *each witness is in the closure of a candidate, which S1f.10.1's implies shows* (T4) | *implies* is between two candidates, and these witnesses are no candidates, so no list of S1f.10.1's holds one. Read as *the node of a candidate alive alone holds the witness*, by the same single nodes; and found false of 18 of I-Z1's 56 ([§ T4](#t4-the-unlicensed-half)) |

### What was built

- **The owner**, [`ein-infer/src/structure.rs`](../../../ein.rs/crates/ein-infer/src/structure.rs), a fourth
  section of its header and of the module:
  - **`Owed`**, what root owes as the engine reads it: `root_owes`, the node function's count; `instances`, L5's
    table in its order, or `NoTable` (`Flagged`, `NoObligation`); each an **`OwedInstance`** with its `rule`, its
    `text`, `alternatives` (the candidates that can witness it, or why L5 does not license it), `denied`, and
    **`witnesses`**, every fact that can witness it, licensed or not. A **`Witness`** has its `text`, what root
    says of it (`WitnessIs`: `Candidate(b)`, `Denied`, `Unproposed`), and the candidates whose own node holds it,
    at root (`held_at_root`) and in the small KB (`held_in_theory`).
  - **`ExactlyOne`**, `Owed` read against a `Cover`: `owing`, how each instance stands (**`Owing`**: `Group(g)`,
    `Inside(groups)`, `NotExcluding { alive, holds }`, `One(b)`, `None`, `Unlicensed { payers, about }`);
    **`owed_by`**, group to the licensed instances whose alternatives are exactly its members; `about`, group to
    the unlicensed instances that are about it; `grids`. `is_owed(g)`, `owed_groups()`, **`unowed()`** (the
    groups nothing owes), **`not_groups()`** (the owed sets that are not a group), `unlicensed()`,
    `brought(k)` (of an instance's witnesses that are no candidate and not denied, how many, and how many a
    candidate alive alone brings), `share(grid)` (a **`Share`** a family: its groups, those owed, those an
    unlicensed instance is about) and **`is_bijection(grid)`**.
  - **`Cover::grids() -> Grids`**, from the cover alone: `grids`, each a **`Grid`** (its `component`, its two
    `families` of groups); `not`, each component that is no grid with why (**`NotAGrid`**: `Cut`,
    `MoreThanTwo { most }`, `OneGroup`, `InOne { members }`, `NoTwoFamilies`, `SharedTwice`); `of_group`, group
    to its grid and family.
  - Three ways in, as the cover has: **`exactly_one(root, terms, ast, &CoverOptions) -> (Cover, ExactlyOne)`**
    for a freshly loaded program; `exactly_one_in(sp, &loaded, &opts)` over a program's prepared space, beside
    `cover_in`; and `Theory::exactly_one()` off a theory's share, which now holds `Theory::owed`.
- **Where S1f.10.2's reading was, and what became of it.** S1f.10.2's T3 read the declared side in the example,
  `pair_layer.rs`'s `declared_beside`: `oblgen::alternatives_of` of the program loaded again, each instance's
  alternatives through `Cover::standing`. That reading is now the owner's, by the same two calls: the table is
  `oblgen::alternatives` over the program's own space and the standing is `Cover::standing`'s. The example
  reads `ExactlyOne` and computes nothing of it. **The census's `owed` object is the same on all 644 lines**
  but for one key's text ([§ Defaults taken](#defaults-taken), 6), and every other field of every line but the
  `wall`s is S1f.10.2's
  (`jq -c -S 'del(.theory.cover.exactly_one) | del(.theory.cover.owed.unlicensed_why) | walk(if type == "object" then del(.wall) else . end)'`
  over `build/structure/s4/census_before.jsonl` and `build/structure/census.jsonl`, `md5sum`, 2026-11-15).
- **Two readers the owner needed**, neither read by a route. `oblgen::witness_rows`: every fact that can witness
  each instance root owes, in the table's order and by its scan, with L5's static premise beside; the
  enumeration of the instances a plan owes is cut out of `oblgen::alternatives` into `owed_by`, which both
  call, so the two lists are one enumeration. `explore::Space::owed`: the table and those rows, **not handed to
  the node function**; `Space::alternatives`, the route's own, is as it was and now calls the same private
  `table`. `Space::root_owes` is a reader.
- **Where the nodes come from.** No node is added. The table and the rows are one pass of root's matcher each,
  read before any node; `held_at_root` reads the singles the pair layer has just put in the space's cache, and
  `held_in_theory` the small KB's singles in its own space. `Cover::root_work` and `small_work` are what they
  were on every program.
- **The vehicle**, [`pair_layer`](../../../ein.rs/crates/ein-infer/examples/pair_layer.rs): with the theory's
  share each program's directory gets **`exactly_one.json`**, raw (every instance with every witness, every
  group with who owes it, the grids, the components that are none), and **`exactly_one.md`**, the same as
  tables; the line gets an `exactly_one` object inside `theory.cover`.
- **The tests**, all in the gate: [`ein-infer/tests/exactly_one.rs`](../../../ein.rs/crates/ein-infer/tests/exactly_one.rs),
  six; the sweep of [`cover.rs`](../../../ein.rs/crates/ein-infer/tests/cover.rs) and both sweeps of
  [`structure_invariance.rs`](../../../ein.rs/crates/ein-infer/tests/structure_invariance.rs), extended in
  place; one in the module ([§ The tests](#the-tests)).
- Docs: `structure.rs`'s row in [`architecture.md`](../../../docs/ein/ein.rs/architecture.md), the
  example's in [`utils/README.md`](../../../utils/README.md), and two rows of [`findings.md`](findings.md),
  K-10 and K-11.

### The definitions

> **An owed group** is a group whose members are exactly the alternatives of an instance root owes and L5
> licenses. Every fact that can witness the instance is then a member of the group or denied at root. A state of
> R has paid the instance, so it holds a member; it holds no two, the group's pairs being dead at root. A member
> dead alone at root is an alternative and is in no alive state, so **every state of R holds exactly one of the
> group's open members**.

> **An owed set that is not a group** is a licensed instance whose alternatives do not pairwise exclude by the
> theory: *at least one*.

> **A group nothing owes** is a group no licensed instance's alternatives are: *at most one*, and nothing the
> engine reads says a member must hold.

> **A grid** is a connected component of the theory's exclusion graph in which every candidate is in exactly two
> groups, the groups are two families no two groups of one of which share a member, and two groups share at
> most one member. Each family holds every member of the component once: rows and columns, and a member one
> cell.

> **A bijection** is a grid with as many groups in one family as in the other, every one of them an owed group:
> every state of R holds exactly one member of each row and of each column.

**The correspondence, worked out on I-Z2.** Root owes 36 instances and the table licenses all 36. Each has five
witnesses, a value's five houses or a house's five values: its alternatives are those of the five that are
candidates of H, 2 to 4 of them, and root denies the others (`alternatives + denied = 5` on every one,
`a_declared_bijections_rows_and_columns_are_owed_groups`). **The alternatives are the group's members, all of
them, and not its open members**: 26 of the 36 instances have an alternative dead alone at root, as `(color-loc
Red House-4)` is of *Red owes a house*, a candidate of H that can witness the debt in L5's reading and that no
alive state holds. Only 10 of the 36 groups have every member open, and 11 have one open member, which every
state of R therefore holds.

**What an owed group rests on, and what fails if it stops being true.** (1) The instance is licensed and its
alternatives are the members: the table is `oblgen::alternatives`, held to the oracle on every monotone program
it reaches by `region.rs`'s `l5_is_the_oracle_on_every_monotone_program_it_reaches`, checked at every alive node
the obligation tree registers (`tree.rs`, `check_owed`, which refuses the run), and held here to the table of
the program loaded again by `cover.rs`'s sweep. (2) The members pairwise exclude at root:
`Cover::alive_at_root`, empty on every corpus program, by the same sweep. (3) The program is monotone, so a
pair dead at root is dead above it: the static check; a flagged program has no table at all.

**An instance L5 does not license** has no alternatives, only the reason, and is never *owed* in the sense
above. Its witnesses are read all the same. Where the candidates whose own node **in the small KB** holds one of
its witnesses are exactly a group's members, and every witness root does not deny is held by such a node, the
instance is **about** that group (`Owing::Unlicensed { payers, about }`). That is a reading and not a licence:
the group stays *nothing owes*, and nothing is concluded from a witness a candidate brings.

The alternatives, each with the program that falsifies it:

| rejected | the program | what it shows |
|---|---|---|
| **an owed group is a group whose *open* members are an instance's alternatives** | **I-Z2**, `zebra2.ein` | 26 of its 36 instances have an alternative dead alone at root: only 10 of the 36 declared rows and columns would be owed |
| **an owed group is any instance whose alternatives pairwise exclude**, maximal or not, the stage file's wording | none in the corpus: of 258 licensed instances 239 are a group, 19 do not pairwise exclude, and none is strictly inside a group | kept apart as `Owing::Inside`, since such an instance would say *exactly one of these* and leave the group's other members witnessing nothing; the module's test graph has one |
| **a grid is the candidates of one relation** | **I-Z1**, `zebra.ein` | one relation, `co-located`, and five grids; `branching/08_hypothesis_relation_whitelist.ein` has six of one relation. And it reads a name |
| **a grid is over the candidates alive alone** | **I-Z2** | 11 of its 36 groups have one open member, so over the 35 candidates alive alone those lines are no groups, their one member is in one group only, and none of the five declared bijections is a grid |
| **a grid's member is in *at most* one group of each family**, a line of one candidate allowed | **`branching/15_lookahead_two_step_on.ein`** | three candidates of a unary relation, `(p A)`, `(p B)`, `(p C)`, and two rules that set A against B and A against C: two groups of two that share `(p A)`, which that reading calls a grid of two rows and two columns. Any path of exclusions is one. **What the strict reading does not find**: 19 components on 7 programs, 3 of them `branching/05_mini_zebra.ein`'s, each a grid of two by two a cell of which root holds, so that one row and one column have one candidate left ([§ T1](#t1-the-table-of-the-three)) |
| **every member in two groups is enough**, without the two families or without *at most one member shared* | none in the corpus: all 44 components whose every member is in exactly two groups are two families that share at most one member a pair | kept by the module's test, which writes a ring of five candidates (five groups of two, no two families) and a component two of whose groups share two members (two cells in one row and one column) |
| **a member may be in more than two groups** | **`lattice/06_friends_tables.ein`** | a rule seats two friends at one table, so a friend's seats at one table and another's at the other are a clique: 26 groups over 19 candidates, a candidate in 9 or 10, one component and no two families of anything |
| **a bijection is a grid every group of which is owed**, whatever the two families' sizes | none in the corpus: the 21 are square. The stage's probe `letters_2x4_surjective.ein` | two rows and four columns, all six owed, and R is empty by the oracle ([§ T3](#t3-where-the-must-is-missing)); `send_surjective.ein` is 8 by 10 the same way |
| **an unlicensed instance is about the candidates that bring its witnesses *at root*** | **I-Z1** | at root a witness is in the closure of every candidate that implies its mirror, 1 to 17 of them: the bringers of the 18 instances are 6 to 22 candidates each and no group. By the theory alone each witness has one bringer, and the 18 sets are 18 groups |
| **about, without *every witness root does not deny is brought*** | **`lattice/06_friends_tables_everyone_sits.ein`** | Cat owes a seat; the candidates that bring a witness are her three seats at the big table, a group; and `(sits Cat S1)`, `(sits Cat S2)` can witness the debt, are no candidates, are not denied, and no node brings them. The instance is about no group |

**What the definition does where the stage asked.** On I-Z1 the one `co-located` relation holds five grids, the
five components of the theory's graph, found with no relation read and no kind: three of four by four and two
of three by three, the same member sets as I-Z2's five under the renaming
(`one_relation_holds_five_grids_and_half_of_each_is_owed`). On I-SEND the 80 candidates are one component and
one grid, eight groups of ten against ten of eight, the eight owed. On `lattice/06_friends_tables.ein` the one
component is no grid, `NotAGrid::MoreThanTwo { most: 10 }`, and on its `_everyone_sits` twin the same with two
owed groups in it.

### The controls

`build/structure/<program>/exactly_one.json` and `.md` hold the lists: every instance with every witness, every
group with who owes it, every grid with its two families.

| | root owes, in the table | groups | **owed groups** | **groups nothing owes** (with an open member) | **owed sets that are not a group** | not licensed (about a group) | grids: the two families' groups, and how many of each are owed | components that are no grid |
|---|---:|---:|---:|---|---:|---|---|---|
| **I-Z2** `zebra2.ein` | 36 | 36 | **36** | 0 | 0 | 0 | **5, all bijections**: 3 and 3 twice, 4 and 4 three times, every group owed | none |
| **I-Z1** `zebra.ein` | 36 | 36 | 18 | 18 (18) | 0 | 18 (18) | 5: 3 and 3 twice, 4 and 4 three times; of each, **one family owed whole and the other by nothing L5 licenses**, an unlicensed instance about every group of it | none |
| **I-Z2M** `zebra2-minus-15.ein` | 46 | 46 | **46** | 0 | 0 | 0 | **5, all bijections**: 4 and 4 twice, 5 and 5 three times | none |
| **I-SEND** `send_more_money.ein` | 8 | 18 | 8 | 10 (10) | 0 | 0 | 1: 8 and 10, **the 8 owed and the 10 not** | none |
| **I-B06** `branching/06_lookahead_on.ein` | 0; no table, the program states no obligation | 10 | 0 | 10 (0) | 0 | 0 | none | 10, each one group |
| **I-L02** `lattice/02_genuine_3set_death.ein` | 0; no table | 0 | 0 | 0 | 0 | 0 | none | none: no pair, no component |
| `lattice/06_friends_tables.ein` | 0; no table, the static check flags it | 26 | 0 | 26 (26) | 0 | 0 | none | 1: a member is in more than two groups, 10 |
| `lattice/06_friends_tables_everyone_sits.ein` | 5 | 26 | 2 | 24 (24) | 0 | 3 (0) | none | 1: the same |

- **I-Z2, the control: it holds, with no deviation.** Five grids, and the members of each are the candidates of
  one declared relation, which the owner did not read and the test does:

  | grid | its members' relation | members (alive alone) | one family | owed by | the other | owed by |
  |---|---|---|---|---|---|---|
  | 0 | `color-loc` | 7 (6) | 3 groups | `std.algebra.total-owed`, 3 | 3 groups | `std.algebra.surjective-owed`, 3 |
  | 1 | `drink-loc` | 8 (5) | 3 | `total-owed`, 3 | 3 | `surjective-owed`, 3 |
  | 2 | `nation-loc` | 13 (6) | 4 | `total-owed`, 4 | 4 | `surjective-owed`, 4 |
  | 3 | `pet-loc` | 14 (10) | 4 | `total-owed`, 4 | 4 | `surjective-owed`, 4 |
  | 4 | `smoke-loc` | 14 (8) | 4 | `total-owed`, 4 | 4 | `surjective-owed`, 4 |

  Every one of the 36 groups is owed by exactly one instance; in each grid one family is *a value owes a house*
  and the other *a house owes a value*; no instance is unlicensed and no group is left.
- **I-Z1: exactly which halves.** The same five member sets under the renaming. Of each grid the family *an
  attribute's houses* is owed, by `std.slots.slot-owed-room`, 18 instances, all licensed, each one's
  alternatives a group: **18 owed groups**. The family *the attributes of one kind at one house* is owed by no
  licensed instance: **18 groups nothing owes**, every one with an open member. `std.slots.slot-owed-fill` owes
  18 instances that L5 does not license, *a fact can witness it that is no candidate and that root does not
  deny*, and each is about exactly one of those 18 groups, each group the subject of one. **What is missing for
  that half is the licence and not the debt**: the debt is stated and R already pays it. What the licence lacks
  is [§ T4](#t4-the-unlicensed-half)'s, and what would supply it [§ T3](#t3-where-the-must-is-missing)'s.

  | grid | members (alive alone) | one family | owed by | the other | owed by | an unlicensed instance is about |
  |---|---|---|---|---|---|---|
  | 0 | 14 (10) | 4 groups | `slot-owed-room`, 4 | 4 groups | nothing | `slot-owed-fill`, 4 |
  | 1 | 8 (5) | 3 | `slot-owed-room`, 3 | 3 | nothing | `slot-owed-fill`, 3 |
  | 2 | 14 (11) | 4 | `slot-owed-room`, 4 | 4 | nothing | `slot-owed-fill`, 4 |
  | 3 | 13 (6) | 4 | `slot-owed-room`, 4 | 4 | nothing | `slot-owed-fill`, 4 |
  | 4 | 7 (6) | 3 | `slot-owed-room`, 3 | 3 | nothing | `slot-owed-fill`, 3 |

- **I-Z2M**: 46 of 46 owed, five bijections over 96 candidates, none dead alone.
- **I-SEND**: one grid of 80 members. The eight groups of ten, a letter's digits, are owed by `total-owed`, one
  instance each. The ten groups of eight, a digit's letters, nothing owes, and every one has 6 members or more
  alive alone. No instance is unlicensed.
- **I-B06**: no table, since the program states no obligation rule, and no grid: each of its ten groups is a
  component of its own, and none has an open member.
- **I-L02**: nothing to owe and nothing owed.
- **`friends_tables`**: flagged, so no table. Its `_everyone_sits` twin is monotone: `total-owed` owes five
  friends a seat; Ann's and Bob's five seats are owed groups, and Cat's, Dan's and Eve's instances are not
  licensed and are about no group ([§ T4](#t4-the-unlicensed-half)).

### T1: the table of the three

**644** files load, the engine refuses **19**, the other **625** are read and **188 search**. Of those
(`jq -s -r -f build/structure/s4/exactly_one.jq build/structure/census.jsonl`, 2026-11-15):

| | |
|---|---:|
| programs with a group | 53 |
| a table is read, and holds an instance | 43, 41 |
| no table: the program states no obligation; the static check flags it | 116; 29 |
| … root owes all the same | 7 instances on 6 flagged programs |
| instances root owes, in a table | 288 |
| groups | 496 |
| **owed groups**, *exactly one* | **239**, on 27 programs |
| **groups nothing owes**, *at most one* | **257**, on 31 programs |
| … with an open member; with none | 202, on 25 programs; 55, on 9 |
| … an unlicensed instance is about | 18, `zebra.ein`'s |
| **owed sets that are not a group**, *at least one* | **19**, on 7 programs, none of which has a group at all |
| instances L5 does not license | 30, on 9 programs |
| components of the theory's graph | 152 |
| **grids** | **44**, on 16 programs |
| … their two families' groups | 2 and 2 × 11, 3 and 3 × 9, 4 and 4 × 16, 5 and 5 × 6, 3 and 10 × 1, 8 and 10 × 1 |
| … **bijections**, every group owed | **21**, on 5 programs |
| … one family owed whole and the other not | 8: the families as many on 6 (22 groups in the unowed families), the unowed family the larger on 2 |
| … owed nowhere | 15 |
| components that are no grid: one group; a member in one group only; a member in more than two | 87; 19; 2 |
| grids whose members are of more than one relation | 0 |
| programs one relation of which holds more than one grid | 2: `zebra.ein`, 5; `branching/08_hypothesis_relation_whitelist.ein`, 6 |

The 19 owed sets that are not a group are the ones S1f.10.2 counted, on `ein-bugs/wrong-answers/owed-seats.ein`,
its `-renamed` twin, `uncovered.ein` and `uncovered-knows.ein` (4 each), `owed-two-step.ein`, `tree-dead.ein`
and `lattice/synth/trivial_line.ein` (1 each): owed where nothing is exclusive.

**The 19 components a member of which is in one group only** are on `branching/05_mini_zebra.ein` (3),
`10_kill_cache_on.ein` and `11_kill_cache_off.ein` (6 each), `15_lookahead_two_step_on.ein` and its `_off` twin
(1 each), `ein-bugs/complete-records-stale.ein` (1) and `ein-bugs/unsat-core-id-space.ein` (1). The first three
programs' are grids a cell of which root holds: `05_mini_zebra.ein`'s first is `(co-located Alice Coffee)`,
`(co-located Bob Coffee)`, `(co-located Bob Tea)`, two people by two drinks with Alice's tea stated. **The
definition does not find those**, and says of each *a member is in one group only*; the reading that would is
the one `15_lookahead_two_step_on.ein` falsifies.

Every searching program with a group or an instance. *Table* is the instances root owes that the engine reads,
empty where no table is read; *open* the groups nothing owes that have an open member; *about* the unlicensed
instances that are about a group; a grid is its two families' groups and how many of each are owed, **b** a
bijection (`jq -r -f build/structure/s4/rows.jq build/structure/census.jsonl`, 2026-11-15).

| program | rung | table | groups | owed groups | groups nothing owes | … open | owed sets not a group | not licensed | … about | grids | components that are no grid |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---|
| `examples/beyond_arith/cryptarithm.ein` | hrules | 3 | 13 | 3 | 10 | 10 | 0 | 0 | 0 | 3×10: 3+0 |  |
| `examples/beyond_arith/equation_by_search.ein` | hrules | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |  | it is one group 3 |
| `examples/beyond_arith/equation_no_solution.ein` | hrules | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 |  | it is one group 2 |
| `examples/beyond_arith/equation_two_unknowns.ein` | hrules | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |  | it is one group 3 |
| `examples/beyond_arith/send_more_money.ein` | hrules | 8 | 18 | 8 | 10 | 10 | 0 | 0 | 0 | 8×10: 8+0 |  |
| `examples/binary_logic/equivalent.ein` | obligations | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |  | it is one group 3 |
| `examples/binary_logic/knights.ein` | obligations | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |  | it is one group 3 |
| `examples/binary_logic/not_equivalent.ein` | obligations | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |  | it is one group 3 |
| `examples/binary_logic/satisfiable.ein` | obligations | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |  | it is one group 3 |
| `examples/binary_logic/synthesis.ein` | obligations | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/binary_logic/truth_table.ein` | obligations | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |  | it is one group 3 |
| `examples/binary_logic/unsatisfiable.ein` | obligations | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |  | it is one group 3 |
| `examples/branching/01_saturate_only.ein` | blind |  | 4 | 0 | 4 | 0 | 0 | 0 | 0 |  | it is one group 4 |
| `examples/branching/02_one_dead_one_alive.ein` | blind |  | 6 | 0 | 6 | 0 | 0 | 0 | 0 |  | it is one group 6 |
| `examples/branching/03_five_hyps_one_alive.ein` | blind |  | 10 | 0 | 10 | 0 | 0 | 0 | 0 |  | it is one group 10 |
| `examples/branching/04_two_levels.ein` | blind |  | 6 | 0 | 6 | 0 | 0 | 0 | 0 |  | it is one group 6 |
| `examples/branching/05_mini_zebra.ein` | hrules |  | 6 | 0 | 6 | 6 | 0 | 0 | 0 |  | a member is in one group only 3 |
| `examples/branching/06_lookahead_on.ein` | blind |  | 10 | 0 | 10 | 0 | 0 | 0 | 0 |  | it is one group 10 |
| `examples/branching/07_lookahead_off.ein` | blind |  | 10 | 0 | 10 | 0 | 0 | 0 | 0 |  | it is one group 10 |
| `examples/branching/08_hypothesis_relation_whitelist.ein` | blind |  | 24 | 0 | 24 | 24 | 0 | 0 | 0 | 2×2: 0+0, 2×2: 0+0, 2×2: 0+0, 2×2: 0+0, 2×2: 0+0, 2×2: 0+0 |  |
| `examples/branching/09_hrule.ein` | hrules |  | 6 | 0 | 6 | 2 | 0 | 0 | 0 | 3×3: 0+0 |  |
| `examples/branching/10_kill_cache_on.ein` | hrules |  | 12 | 0 | 12 | 12 | 0 | 0 | 0 |  | a member is in one group only 6 |
| `examples/branching/11_kill_cache_off.ein` | hrules |  | 12 | 0 | 12 | 12 | 0 | 0 | 0 |  | a member is in one group only 6 |
| `examples/branching/12_typed_blind_solve.ein` | blind |  | 4 | 0 | 4 | 4 | 0 | 0 | 0 | 2×2: 0+0 |  |
| `examples/branching/15_lookahead_two_step_on.ein` | hrules |  | 2 | 0 | 2 | 2 | 0 | 0 | 0 |  | a member is in one group only 1 |
| `examples/branching/16_lookahead_two_step_off.ein` | hrules |  | 2 | 0 | 2 | 2 | 0 | 0 | 0 |  | a member is in one group only 1 |
| `examples/domain_elim/b_branch.ein` | hrules |  | 6 | 0 | 6 | 2 | 0 | 0 | 0 | 3×3: 0+0 |  |
| `examples/ein-bugs/complete-records-stale.ein` | hrules |  | 2 | 0 | 2 | 2 | 0 | 0 | 0 |  | a member is in one group only 1 |
| `examples/ein-bugs/fork-delta-order.ein` | hrules |  | 1 | 0 | 1 | 1 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/ein-bugs/unsat-core-id-space.ein` | blind |  | 4 | 0 | 4 | 3 | 0 | 0 | 0 |  | a member is in one group only 1 |
| `examples/ein-bugs/wrong-answers/einb-dump-answer.ein` | blind |  | 1 | 0 | 1 | 1 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/ein-bugs/wrong-answers/forall-core-elim.ein` | hrules |  | 1 | 0 | 1 | 1 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/ein-bugs/wrong-answers/forall-core-false.ein` | hrules |  | 1 | 0 | 1 | 1 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein` | declined | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein` | declined | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-declared.ein` | declined | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-two-declared.ein` | declined | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein` | declined | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name.ein` | declined | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/owed-seats-renamed.ein` | obligations | 4 | 0 | 0 | 0 | 0 | 4 | 0 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/owed-seats.ein` | obligations | 4 | 0 | 0 | 0 | 0 | 4 | 0 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/owed-two-step.ein` | obligations | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/tree-dead.ein` | obligations | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/uncovered-knows.ein` | obligations | 4 | 0 | 0 | 0 | 0 | 4 | 0 | 0 |  |  |
| `examples/ein-bugs/wrong-answers/uncovered.ein` | obligations | 4 | 0 | 0 | 0 | 0 | 4 | 0 | 0 |  |  |
| `examples/features/11_expect_ambiguity.ein` | hrules |  | 4 | 0 | 4 | 4 | 0 | 0 | 0 | 2×2: 0+0 |  |
| `examples/features/13_mixed_solution_and_open.ein` | hrules | 1 | 1 | 0 | 1 | 1 | 0 | 1 | 0 |  | it is one group 1 |
| `examples/lattice/05_owed_seats_bounded.ein` | obligations | 4 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 2×2: 2+2 **b** |  |
| `examples/lattice/06_friends_tables.ein` | hrules |  | 26 | 0 | 26 | 26 | 0 | 0 | 0 |  | a member is in more than two groups 1 |
| `examples/lattice/06_friends_tables_everyone_sits.ein` | hrules | 5 | 26 | 2 | 24 | 24 | 0 | 3 | 0 |  | a member is in more than two groups 1 |
| `examples/lattice/07_two_pairs_two_tables.ein` | hrules | 4 | 8 | 4 | 4 | 4 | 0 | 0 | 0 | 4×4: 4+0 |  |
| `examples/lattice/synth/merged_states.ein` | hrules | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/lattice/synth/one_model_per_component.ein` | hrules | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/lattice/synth/trivial_line.ein` | hrules | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |  |  |
| `examples/word_problems/boxes12.ein` | hrules | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/word_problems/boxes13.ein` | hrules | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |  | it is one group 1 |
| `examples/zebra.ein` | hrules | 36 | 36 | 18 | 18 | 18 | 0 | 18 | 18 | 4×4: 4+0, 3×3: 3+0, 4×4: 4+0, 4×4: 4+0, 3×3: 3+0 |  |
| `examples/zebra2-hints.ein` | hrules | 0 | 26 | 0 | 26 | 26 | 0 | 0 | 0 | 2×2: 0+0, 3×3: 0+0, 4×4: 0+0, 4×4: 0+0 |  |
| `examples/zebra2-minus-15-obligations.ein` | obligations | 46 | 46 | 46 | 0 | 0 | 0 | 0 | 0 | 5×5: 5+5 **b**, 4×4: 4+4 **b**, 4×4: 4+4 **b**, 5×5: 5+5 **b**, 5×5: 5+5 **b** |  |
| `examples/zebra2-minus-15.ein` | hrules | 46 | 46 | 46 | 0 | 0 | 0 | 0 | 0 | 5×5: 5+5 **b**, 4×4: 4+4 **b**, 4×4: 4+4 **b**, 5×5: 5+5 **b**, 5×5: 5+5 **b** |  |
| `examples/zebra2-obligations.ein` | obligations | 36 | 36 | 36 | 0 | 0 | 0 | 0 | 0 | 3×3: 3+3 **b**, 3×3: 3+3 **b**, 4×4: 4+4 **b**, 4×4: 4+4 **b**, 4×4: 4+4 **b** |  |
| `examples/zebra2.ein` | hrules | 36 | 36 | 36 | 0 | 0 | 0 | 0 | 0 | 3×3: 3+3 **b**, 3×3: 3+3 **b**, 4×4: 4+4 **b**, 4×4: 4+4 **b**, 4×4: 4+4 **b** |  |
| `tests/region/02_merged_single.ein` | hrules | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |  | it is one group 1 |
| `tests/region/09_query_over_maximal.ein` | hrules |  | 4 | 0 | 4 | 4 | 0 | 0 | 0 | 2×2: 0+0 |  |
| `tests/stdlib/bool/inputs/01_in_drives.ein` | obligations | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 |  | it is one group 2 |
| `tests/stdlib/bool/inputs/02_required.ein` | obligations | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 |  | it is one group 2 |

### T2: every bijection, found or declared

*Found* is the owner's: a grid with as many groups in each family, every one owed. *Declared* is read by a
script, by name: the program states `(bijective R)`, under any module prefix
(`python3 build/structure/s4/bijections.py build/structure build/structure/census.jsonl`, 2026-11-15; the script
is under [§ The commands](#the-commands)). A declared relation is set against the grids its candidates are
members of. Over the 188 programs that search:

| | |
|---|---:|
| relations declared bijective | 29, on 9 programs |
| **found and declared** | **20**, over 20 grids: `zebra2.ein`, `zebra2-obligations.ein`, `zebra2-minus-15.ein` and `zebra2-minus-15-obligations.ein`, five each |
| declared, and nothing of it left in H | 1: `zebra2-hints.ein`'s `color-loc`, which root decides whole |
| **declared and not found** | **8** |
| **found and declared nowhere** | **1** |

**I-Z2, the control: all five found and declared**, each declared relation's candidates the members of one
grid, that grid a bijection.

**Declared and not found**, with what broke it:

| program | relation | its candidates | grids over them | what broke it |
|---|---|---:|---:|---|
| `examples/zebra2-hints.ein` | `drink-loc`, `nation-loc`, `pet-loc`, `smoke-loc` | 4, 7, 13, 12 | 1 each: 2 and 2, 3 and 3, 4 and 4, 4 and 4 | **the grid is found and nothing owes a group of it.** The program states `bijective` through a rule of its own, `bijective-properties`, which concludes `functional`, `injective`, `total` and `surjective` and no obligation; `std.algebra`'s two obligation rules are loaded with the module and nothing activates them, so the table is read and is empty. 26 groups, all *at most one* |
| `examples/domain_elim/b_branch.ein` | `color-loc` | 7 | 1: 3 and 3 | the same: a `bijective-properties` of its own and no obligation rule at all |
| `examples/domain_elim/b_only.ein` | `color-loc` | 1 | 0 | one candidate is left, and one candidate is no pair |
| `examples/syntax/constraint-scopes.ein` | `next-to` | 6 | 0 | the theory excludes no pair of them: the program states `(bijective next-to)` and imports no rule that reads it (S1f.10.2's finding of the same program) |
| `tests/stdlib/bijection/06_blind_enumeration.ein` | `seats` | 16 | 0 | the theory excludes no pair of them: the ladder declines and the blind enumerator built H, the class [S1f.10.10](s1f.10.10_the_theorys_share.md#the-rule-as-the-corpus-left-it) says its rule does not fit |

So of the eight, five are a *must* the program states in a form the engine does not read as one, and three are
no grid at all. **A declaration `bijective` is not what makes a bijection found: an obligation is.**

**Found and declared nowhere**: `examples/lattice/05_owed_seats_bounded.ein`, one grid of 2 and 2 over `seats`,
every group owed. The program does not say `bijective`: it states `(functional seats)`, `(injective seats)`,
`(total-owed seats is-a)` and `(surjective-owed seats is-a)`, the four halves under their own names.

**Half found**: the eight grids one family of which is owed whole are `zebra.ein`'s five, `send_more_money.ein`'s
and `cryptarithm.ein`'s (letters owed, digits not) and `lattice/07_two_pairs_two_tables.ein`'s (4 and 4, the
friends owed a seat and no seat owed a friend). None declares `bijective`.

### T3: where the *must* is missing

Analysis, and nothing built: no rule, obligation or closure is added by the engine, and `ExactlyOne` concludes
nothing from a count. The 257 groups nothing owes first, sorted by what a count could say of each
(`python3 build/structure/s4/unowed.py build/structure build/structure/census.jsonl`, 2026-11-15):

| a group nothing owes | groups | where |
|---|---:|---|
| has **no open member**: no alive state holds a member, and no *must* is in question | 55 | 9 programs, I-B06 among them |
| is in a grid whose other family is owed whole, **the two families as many** | 22 | `zebra.ein` 18, `lattice/07_two_pairs_two_tables.ein` 4 |
| the same, and its own family is **the larger** | 20 | `send_more_money.ein` 10, `cryptarithm.ein` 10 |
| is in a grid neither family of which is owed whole | 66 | 7 programs: `zebra2-hints.ein` 26, `branching/08_hypothesis_relation_whitelist.ein` 24, and five others |
| is in no grid | 94 | 14 programs |

**Route 1, counting.** *A grid of p rows and q columns, every row an owed group.* A state of R then holds
exactly one member of each row, p members, and no two of them in one column, so p columns hold one member each.
Where q = p every column holds exactly one; where q > p, q − p columns hold none; where q < p no state can do it
and R is empty. Its premises, each with what fails when it stops being true:

| premise | enforced by |
|---|---|
| every row is an owed group: its instance is licensed and its alternatives are exactly the row's members, so no fact outside the grid pays a row's debt | L5's table, held to the oracle (`region.rs`, `l5_is_the_oracle_on_every_monotone_program_it_reaches`) and checked at every alive node the tree registers (`tree.rs`, `check_owed`); the alternatives against the members by `cover.rs`'s sweep and `exactly_one.rs` |
| every column is *at most one* at every node | the group's pairs are dead at root (`Cover::alive_at_root` empty, `cover.rs`'s sweep) and the program is monotone (the static check) |
| every member of a row is in exactly one column, and a row and a column share at most one member | the grid's definition, computed; `cover.rs`'s sweep holds each family to *every member once* |
| q = p | `ExactlyOne::share`, a count of groups |

**The alive-set invariant is not among them, and it does not give what the stage asked of it.**
[`invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs) says no rule asserts a name the program
lacks. It says nothing of a *type's members*: a rule may give a type a member under a commitment by a name the
program has. `examples/ein-bugs/wrong-answers/oblig-fresh-name.ein` does: its rule `special` concludes `(food
Z)` once someone likes something, `Z` is a name of the program, and the foods a person may be owed grow at a
node. What closes the object list is **L5's own static premise**, per instance: the obligation's trigger and
its guards beside the witness step read no relation a commitment can change (`oblgen::static_premises` over
`monotone::changes`), and L5 refuses that program's instance for exactly that, *it reads `food` beside its
witness, a relation a commitment can change*. So route 1 needs no premise of its own about names: an owed row
already has a complete list of what can pay it, or it is not an owed group.

**Counting transfers a *must* and creates none.** Two families of *at most one* and equal counts allow the empty
state. So route 1 applies only where one family is owed: to 22 of the 257 groups it gives *exactly one*, to 20
it says some of them hold nothing, and of the other 215 it says nothing.

**Route 2, what is declared read more widely.** `functional` and `total` together, without the word
`bijective`. Two things in the way:

- The engine reads no stdlib name, so *read more widely* would be the census's reading and never a licence.
- **The declaration is not a *must* in the engine, and reading it as one changes R.** `std.algebra`'s `total` is
  a refutation: it fires when every partner carries a stored negative. A state in which a guest has no chair
  and no chair is excluded is alive and owes nothing. The probe `seats_declared.ein` states `functional`,
  `injective`, `total` and `surjective` of two guests and two chairs and no obligation: **R is 7 states**, the
  empty one and the four of one guest seated among them. With `total-owed` and `surjective-owed` beside
  (`seats_owed.ein`), **R is 2 states**. [`stdlib/closure.ein`](../../../stdlib/closure.ein) reads the same
  pair, `(functional R)` and `(total R)`, to conclude `(__closed__ R)`, *its extension is fixed*, which takes
  the relation out of what is hypothesised; its own header calls that an operational witness and says not to
  import it where the relation needs branching. Neither reading of the pair is *some member must hold*.
- On the instances there is nothing more to read. I-Z1 declares the whole property, `slot-partition`, and
  both debts are stated. I-SEND declares `total-owed`, `functional` and `injective`: an injection, which is
  what it is. I-B06 declares `functional`, `sibling-exclusive`, `symmetric` and `transitive`, and no totality
  under any name. The corpus's five declared-and-unowed relations (`zebra2-hints.ein`'s four, `b_branch.ein`'s)
  are where route 2 would act, and there it would change R.

**Route 3, the obligation.** The lines the program would have to add, each run on a copy under
`build/structure/s4/p/` against the program without them. **`ein oracle` refuses every instance**: 56, 80 and
132 candidates against its limit of 16, 2^n saturations. So each instance has two comparisons: the full copy
by `ein solve --count`, which takes describe's route (`ein-infer/tests/explore.rs` holds every mode to the
oracle on the programs the oracle reaches); and a copy reduced to 16 candidates or fewer by `ein oracle`
itself. The probes are not corpus
entries; each is verbatim under [§ The probes](#the-probes).

| | groups nothing owes | route 1 | route 2 | route 3: the lines, and the R they give against the program's own |
|---|---|---|---|---|
| **I-Z1** `zebra.ein` | 18, *the attributes of one kind at one house*, all with an open member | **applies, every premise enforced**: 5 grids, each of as many groups a family, one family owed whole. *Exactly one* of each of the 18 follows. Not acted on | nothing to read: the property and both debts are stated | `fill_mirrored.lines`, the debt `slot-owed-fill` already states, with its witness written value first as the hrule proposes it. **The same R**: 1 state and 261 982 commitments by `--count` on both, root owing 54 instances where it owed 36; on the two-house copy `ein oracle` gives 2 states and 30 commitments on both. With the lines all 36 groups are owed and the five grids are bijections. `--single` takes 48 saturations where the program takes 49 |
| **I-SEND** `send_more_money.ein` | 10, *a digit's letters*, all with an open member | applies and says **no**: 8 rows owed against 10 columns, so in every state of R two digits' groups hold nothing | nothing to read: an injection is declared | `(surjective-owed val is-a)`. **Another R: it is empty.** On the copy of two letters and four digits `ein oracle` gives 2 states without the line and none with it, *every alive node owes*; on two letters and two digits, 1 state with and without. At full size `--single` is not decided within the default budget (110 840 saturations, no state met), where the program's is decided in 17 747. The *must* is false of this program, and stating it is a different puzzle |
| **I-B06** `branching/06_lookahead_on.ein` | 10, *an object's nine other types*, none with an open member | does not apply: no grid, and nothing owed | nothing to read: `functional` and no totality | `typed_owed.lines`, *every colour and every house owes a kind*. Root states each one's kind, so the debt is paid at root by a fact that is no candidate and no member: of each group **none** holds, in every state. **Another R, and not by the *must***: the program's own is 2 states and 4 commitments over 132 candidates; with the lines the ladder is on its second rung, root owes nothing, H is empty and R is `{∅}` ([`findings.md`](findings.md)'s K-11). On `branching/01_saturate_only.ein`, 16 candidates, `ein oracle` gives `{∅}` both ways, over 16 candidates and over none |

**Where *exactly one* cannot be derived, what the program would have to state.** For a group nothing owes that
has an open member and that no count reaches, 160 of the 257, *exactly one* is **not derivable without a
declaration**, and the declaration is an obligation rule whose witnesses are the group's members and whose
other reads no commitment can change: `total-owed` or `surjective-owed` where the group is a row or a column
of a relation with a type on each side (`zebra2-hints.ein`, `b_branch.ein`), a rule of the program's own
otherwise. For the 20 of `send_more_money.ein` and `cryptarithm.ein` it is not true, and no statement makes it
so without changing R. For the 55 with no open member it is not in question. For I-Z1's 18 and
`lattice/07_two_pairs_two_tables.ein`'s 4 it follows by route 1 from what is enforced, and on I-Z1 it is also
what the program's own unlicensed debt already says; whether a route may spend either is
[S1f.10.14](s1f.10.14_exactly_one_spent.md)'s.

#### The probes

`build/structure/s4/p/`, written by `build/structure/s4/probes.sh` ([§ The commands](#the-commands)) from the
corpus programs and the pieces below, 2026-11-15; the outputs are under `build/structure/s4/out/`.

`zebra_fill_mirrored.ein` is `examples/zebra.ein` followed by `fill_mirrored.lines`, and
`slots_2x2_fill_mirrored.ein` is `slots_2x2.ein` followed by the same:

```lisp

;; The probe's lines: the debt `std.slots.slot-owed-fill` states, a slot has no member of a type yet, with its
;; witness written the way the hrule proposes it, value first.
(rule slot-owed-fill-mirrored (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?i ?index)
               (?sub ?Tv ?super)
               (neq ?Tv ?index)
               (absent (and (?isa ?v ?Tv) (?R ?v ?i))))
  :assert (open ?R)
  :why    "slot {?i} has no {?Tv} yet, read from the value's side")
(slot-owed-fill-mirrored co-located instance type Attribute House)
```

`slots_2x2.ein`, I-Z1 reduced to two houses and two kinds, 8 candidates:

```lisp
;;; A probe, not a corpus entry: zebra.ein's one-relation ontology over two houses and two kinds, small enough
;;; for `ein oracle`. Eight candidates, `(co-located v House-n)`.
(import std.algebra :symbols (symmetric symmetric-negative-setup))
(import std.slots :symbols (slot-partition-setup))
(hrule guess (?R ?T1 ?T2)
  :match  (and (instance ?a ?T1) (instance ?b ?T2))
  :assert (?R ?a ?b)
  :why    "guess: ({?R} {?a} {?b})?")
(relation type       T T :why "{?1} is a subtype of {?2}")
(relation instance   T T :why "{?1} is an instance of {?2}")
(relation co-located Attribute Attribute :why "{?1} and {?2} are in the same house")
(symmetric co-located)
(slot-partition co-located instance type Attribute House)
(type Attribute T) (type House Attribute) (type Color Attribute) (type Pet Attribute)
(instance House-1 House) (instance House-2 House)
(instance Red Color) (instance Green Color)
(instance Dog Pet) (instance Cat Pet)
(co-located Red Dog :source "the dog lives in the red house")
(solve :hrules (guess (co-located Color House) (co-located Pet House)))
(query :goal (co-located Dog ?h))
```

`send_surjective.ein` is `examples/beyond_arith/send_more_money.ein` with `surjective-owed` added to its import
of `std.algebra` and one line after `(injective val)`:

```lisp
(surjective-owed val is-a)   ; the probe's line: and every digit owes a letter
```

The four `letters_*.ein`, I-SEND reduced to A + A = B over four digits and over two, each with and without that
line, are written by `p/make_letters.sh`:

```sh
#!/usr/bin/env bash
# The four letters probes: A + A = B over the digits given, with and without the line that owes a digit a letter.
letters() { # $1 = the digits declared, $2 = an import symbol more, $3 = a line more
cat <<EOF
;;; A probe, not a corpus entry: A + A = B over the digits $1, in send_more_money.ein's vocabulary, small enough
;;; for \`ein oracle\`.
(import std.math.digits :symbols (fa nz))
(import std.algebra :symbols (total-owed functional injective$2))
(relation is-a Thing Type :why "{?1} is a {?2}")
(relation val Letter Digit :why "{?1} = {?2}")
(relation column Column Letter Letter Letter :why "column {?1}: {?2} + {?3} gives {?4}")
(relation col Column Digit Digit :why "column {?1} adds up to digit {?2}, carry {?3}")
$(for d in $1; do printf '(is-a %s Digit) ' "$d"; done)
(is-a A Letter) (is-a B Letter)
(column c1 A A B)
(total-owed val is-a)   ; every letter owes a digit
(functional val)        ; and has one
(injective val)         ; and no two letters share one$3
(hrule guess ()
  :match  (and (is-a ?l Letter) (is-a ?d Digit))
  :assert (val ?l ?d)
  :why    "suppose {?l} = {?d}")
(solve :hrules (guess))
(rule add-column ()
  :match  (and (column ?c ?x ?y ?z) (val ?x ?a) (val ?y ?b) (fa ?a ?b 0 ?d ?k))
  :assert (col ?c ?d ?k)
  :why    "{?x} + {?y}: {?a} + {?b} is digit {?d}, carry {?k}")
(rule column-wrong ()
  :match  (and (col ?c ?d ?k) (column ?c ?x ?y ?z) (val ?z ?v) (neq ?v ?d))
  :assert (false)
  :why    "column {?c} adds up to {?d}, and {?z} is {?v}")
(rule carry-left ()
  :match  (and (col ?c ?d ?k) (nz ?k))
  :assert (false)
  :why    "a carry is left over column {?c}")
(query :goal (and (val A ?a) (val B ?b)))
EOF
}
SURJ=$'\n(surjective-owed val is-a)   ; the probe\'s line: and every digit owes a letter'
cd "$(dirname "$0")"
letters "1 2 3 4" "" "" > letters_2x4.ein
letters "1 2 3 4" " surjective-owed" "$SURJ" > letters_2x4_surjective.ein
letters "1 2" "" "" > letters_2x2.ein
letters "1 2" " surjective-owed" "$SURJ" > letters_2x2_surjective.ein
```

`b06_typed_owed.ein` is `examples/branching/06_lookahead_on.ein` followed by `typed_owed.lines`:

```lisp

;; The probe's lines: every colour and every house owes a kind. Root states each one's kind, so nothing is owed.
(relation kind  Kind  :why "{?1} is a kind")
(relation thing Thing :why "{?1} is a thing that has a kind")
(kind Color) (kind House)
(thing Red) (thing Green) (thing Ivory) (thing Yellow) (thing Blue)
(thing H1) (thing H2) (thing H3) (thing H4) (thing H5)
(rule typed-owed ()
  :match  (and (thing ?x) (absent (and (kind ?t) (is-a ?x ?t))))
  :assert (open is-a)
  :why    "{?x} has no kind yet")
```

and `b01_typed_owed.ein` is `examples/branching/01_saturate_only.ein` followed by `typed_owed_small.lines`:

```lisp

;; The probe's lines: every colour and every drink owes a kind. Root states each one's kind, so nothing is owed.
(relation kind  Kind  :why "{?1} is a kind")
(relation thing Thing :why "{?1} is a thing that has a kind")
(kind Color) (kind Drink)
(thing Red) (thing Blue) (thing Tea) (thing Coffee)
(rule typed-owed ()
  :match  (and (thing ?x) (absent (and (kind ?t) (is-a ?x ?t))))
  :assert (open is-a)
  :why    "{?x} has no kind yet")
```

The two `seats_*.ein`, route 2's, are written by `p/make_seats.sh`:

```sh
#!/usr/bin/env bash
# The two seats probes: two guests and two chairs, `seats` functional and injective, and total and surjective
# stated by the declaration's own rules; then with the two obligations beside.
seats() { # $1 = import symbols more, $2 = lines more
cat <<EOF
;;; A probe, not a corpus entry: lattice/05_owed_seats_bounded.ein's two guests and two chairs, with an hrule so
;;; that the ladder is on its first rung in both variants.
(import std.algebra :symbols (functional injective total surjective$1))
(relation is-a   T T         :why "{?1} is a {?2}")
(relation seats  Guest Chair :why "{?1} is seated at {?2}")
(is-a Ann Guest) (is-a Bob Guest)
(is-a C1 Chair)  (is-a C2 Chair)
(functional seats)
(injective  seats)
(total      seats is-a)   ; declared: no guest may have every chair excluded
(surjective seats is-a)   ; declared: no chair may have every guest excluded$2
(hrule seat ()
  :match  (and (is-a ?g Guest) (is-a ?c Chair))
  :assert (seats ?g ?c)
  :why    "suppose {?g} sits at {?c}")
(solve :hrules (seat))
(query :goal (seats ?g ?c))
EOF
}
cd "$(dirname "$0")"
seats "" "" > seats_declared.ein
seats " total-owed surjective-owed" $'\n(total-owed      seats is-a)   ; the probe\'s lines: every guest owes a chair\n(surjective-owed seats is-a)   ; and every chair a guest' > seats_owed.ein
```

**What each gave**, `build/structure/s4/out/summary.txt`. *States* and *commitments* are R's two counts;
`ein oracle` where the program has 16 candidates or fewer and `ein solve --count` where it has more:

| program | by | candidates | root owes | states | commitments | against the program without the lines |
|---|---|---:|---:|---:|---:|---|
| `slots_2x2.ein` | oracle | 8 | 8 | 2 | 30 | |
| `slots_2x2_fill_mirrored.ein` | oracle | 8 | 12 | 2 | 30 | **the same R** |
| `examples/zebra.ein` | `--count` | 56 | 36 | 1 | 261 982 | the oracle refuses: 2^56 |
| `zebra_fill_mirrored.ein` | `--count` | 56 | 54 | 1 | 261 982 | **the same R** |
| `letters_2x2.ein` | oracle | 4 | 2 | 1 | 1 | |
| `letters_2x2_surjective.ein` | oracle | 4 | 4 | 1 | 1 | **the same R** |
| `letters_2x4.ein` | oracle | 8 | 2 | 2 | 2 | |
| `letters_2x4_surjective.ein` | oracle | 8 | 6 | 0 | 0 | **another R: empty** |
| `examples/beyond_arith/send_more_money.ein` | `--single` | 80 | 8 | 1 | | *Solution*, decided, 17 747 saturations; the oracle refuses: 2^80 |
| `send_surjective.ein` | `--single` | 80 | 18 | | | **not decided** at the default budget, 110 840 saturations, no state met |
| `examples/branching/06_lookahead_on.ein` | `--count` | 132 | 0 | 2 | 4 | the oracle refuses: 2^132 |
| `b06_typed_owed.ein` | `--count` | 0 | 0 | 1 | 1 | **another R**: `{∅}`, no candidate |
| `examples/branching/01_saturate_only.ein` | oracle | 16 | 0 | 1 | 1 | 65 536 saturations |
| `b01_typed_owed.ein` | oracle | 0 | 0 | 1 | 1 | `{∅}` both ways; the read-out lists the state's facts of `co-located` and `is-a` in the first and none in the second, those relations having no candidate there |
| `seats_declared.ein` | oracle | 4 | 0 | 7 | 7 | |
| `seats_owed.ein` | oracle | 4 | 4 | 2 | 2 | **another R** |

The probes read by the owner (`pair_layer --theory` over them, `build/structure/s4/out/structure.jsonl`):
`zebra_fill_mirrored.ein` has 54 instances, 36 owed groups and five bijections, its 18 `slot-owed-fill`
instances unlicensed as before; `slots_2x2.ein` two grids of 2 and 2 with one family owed, and with the lines
two bijections; `letters_2x2_surjective.ein` and `seats_owed.ein` one bijection each; and
`letters_2x4_surjective.ein` and `send_surjective.ein` a grid **every group of which is owed and whose families
differ in number**, 2 and 4, and 8 and 10, which is why a bijection is defined with *as many*.

### T4: the unlicensed half

**30 instances on 9 searching programs** are not licensed. The reason in the program's terms, and what is true
of the witnesses, by the engine's nodes (`exactly_one.json`'s `witnesses`; the counts are
`build/structure/s4/exactly_one.jq`'s):

| program | instances | L5's reason | in the program's terms | witnesses that are no candidate and not denied | … in the closure of a candidate alive alone at root | … not, their bringer by the theory dead alone at root | … held by no single's node, at root or in the small KB | instances every such witness of which a candidate brings at root |
|---|---:|---|---|---:|---:|---:|---:|---:|
| `examples/zebra.ein` | 18, `std.slots.slot-owed-fill` | a fact can witness it that is no candidate and that root does not deny | a house owes an attribute of a kind, witnessed by `(co-located House-n v)`; the hrule proposes `(co-located v House-n)`; `symmetric` makes each the other's consequence | 56 | 38 | 18 | 0 | **5** |
| `examples/lattice/06_friends_tables_everyone_sits.ein` | 3, `total-owed` | the same | Cat, Dan and Eve owe a seat among all five; the hrule proposes a friend only the seats of a table her group fits, so two seats at the small table can witness the debt and nothing proposes them. No rule of the program concludes `sits` | 6 | 0 | 0 | 6 | 0 |
| `examples/features/13_mixed_solution_and_open.ein` | 1, `total-owed` | the same | Ann owes a food; the hrule proposes `(choice p)`, and a rule derives `(likes Ann Soup)` from `(choice Ann)`: the witness is of a relation no candidate is of | 1 | 1 | 0 | 0 | **1** |
| `ein-bugs/wrong-answers/oblig-derived-domain.ein`, `oblig-fresh-name.ein`, `oblig-fresh-name-two.ein` and their `-declared` twins | 8, the programs' own `owes-food` | it reads `food` beside its witness, a relation a commitment can change | a rule gives the witness's type a member under a commitment, so the facts that can pay the debt are not known at root | 0 | 0 | 0 | 0 | none has such a witness |
| | **30** | | | **63** | **39** | **18** | **6** | **6** |

**The census the stage asked for: 6 of the 30 unlicensed instances have every witness implied by a
candidate**, in the one sense the nodes give: every witness that is no candidate and that root does not deny is
in the closure of a candidate alive alone at root. 16 have a witness of which that fails, and 8 have no such
witness, their reason being the static premise.

**I-Z1's `slot-owed-fill`, against the claim.** The stage file says each witness is in the closure of a
candidate, its symmetric twin, *which S1f.10.1's implies shows*. By the nodes
(`one_relation_holds_five_grids_and_half_of_each_is_owed`):

- The 18 instances have 90 witnesses. Root **denies 34**. The other **56 are no candidate and not denied, and
  are exactly the mirrors of the 56 candidates of H**, one each: `(co-located House-n v)` for `(co-located v
  House-n)`. S1f.10.1's record stands: H holds one orientation and no twin, so no witness is a candidate and no
  entry of *implies*, which is between candidates, names one.
- **By the theory alone each is brought by its mirror and by no other candidate**: in the small KB the node of
  `(co-located v House-n)` holds `(co-located House-n v)`, and no other single's node does. That is what makes
  each instance *about* a group, the mirrors of its witnesses being the group's members.
- **At root, 38 are in the closure of a candidate and 18 are in no single's closure.** The 38 are those whose
  mirror is alive alone: its node derives the witness, and so does the node of every candidate that implies the
  mirror, 1 to 17 candidates a witness. The 18 are those whose mirror is **dead alone at root**: no alive single
  node holds them. None is a fact nothing derives: the small KB's node of its mirror holds each. At root that
  node is dead, and what a dead node holds is not read here.
- So the claim holds of **5 of the 18 instances** whole (those no witness of which mirrors a candidate dead
  alone) and fails of 13, each of which has 1 to 3 witnesses no alive single brings.

Whether a witness a candidate brings may stand for the candidate, and whether a witness whose only bringer is
dead alone may be left out as a denied one is, are [S1f.10.14](s1f.10.14_exactly_one_spent.md)'s. What this
stage supplies for it beside the census: on I-Z1 the instance's payers by the theory are a group's members on
18 of 18, and stating the same debt with the witness mirrored gives the same R
([§ T3](#t3-where-the-must-is-missing)). And one thing it did not ask: on
`06_friends_tables_everyone_sits.ein` the six witnesses nothing brings are of a relation, `sits`, that no rule
of the program concludes, which the program's text says and no reader of this stage checks.

### The tests

| test | what it holds | wall, the target |
|---|---|---:|
| `exactly_one.rs`: `a_declared_bijections_rows_and_columns_are_owed_groups` | **the control**, I-Z2: 36 instances, each one's alternatives a group's members and its five witnesses those members and what root denies; every group owed once; five grids, 3 and 3 twice and 4 and 4 three times, each of one relation, each a bijection, one family owed by `total-owed` and the other by `surjective-owed` | |
| `one_relation_holds_five_grids_and_half_of_each_is_owed` | I-Z1: the five grids are I-Z2's under the renaming; 18 owed by `slot-owed-room`, 18 unlicensed `slot-owed-fill` each about one of the 18 groups nothing owes; in every grid one family owed whole and the other the subject of an instance a group, as many groups in each; and the witnesses, one the mirror of each candidate, brought by the theory by that mirror alone, 38 in its closure at root and 18 in no single's, 5 instances whole | |
| `an_under_determined_puzzle_has_its_five_bijections` | I-Z2M, root read as far as its singles: 46 owed groups, five bijections | |
| `a_cryptarithms_letters_are_owed_and_its_digits_are_not` | I-SEND: one grid of 8 and 10, the owed groups the eight of ten members, the groups nothing owes the ten of eight | |
| `a_program_with_no_table_owes_no_group` | I-B06, I-L02 and a flagged program: no table and why, no grid, ten components each one group | |
| `a_member_in_more_than_two_groups_is_in_no_grid` | `06_friends_tables_everyone_sits.ein`: one component, `MoreThanTwo { most: 10 }`; two owed groups; three unlicensed instances about no group, two witnesses of each brought by nothing | 5.3 s |
| `cover.rs`: `every_cover_is_whole_and_its_groups_are_the_maximal_cliques`, extended | **what *exactly one* is**, in the cover's own reading of 612 programs: the owner's table is `oblgen::alternatives_of` of the program loaded again, instance for instance; each instance stands as `Cover::standing` reads its alternatives; a licensed instance's witnesses are its alternatives and what root denies; `owed_by` is the inverse of `owing`; the owed and the unowed are all the groups once; each grid is a component whose two families each hold every member once, two groups sharing at most one; every component is a grid or says why. 231 instances, 99 owed groups, 30 grids, 6 bijections | 14.4 s |
| `structure_invariance.rs`: both sweeps, extended | **the two invariances of *exactly one***, in the same readings: what root owes with every witness and its holders, how each instance stands, who owes each group, the grids and the components that are none. Under a permuted id space place for place; under every import form as sets of texts, a grid its two families in either order | 21.1 s, and 56.4 s with `EIN_CORPUS_SLOW=1` |
| `a_structure_that_moves_is_caught`, extended | two instances exchanged, a group owed by another instance, a grid's families exchanged: each caught place for place; the last is the same as sets, and a group that loses the instance that owes it is not | |
| `--lib structure`: `a_grid_is_two_families_and_every_other_component_says_why` | on graphs written by hand: a grid of two rows and three columns, and one component of each kind that is none (one group, a path, a ring of five, two cells in one row and column, a member in three groups); every `Owing`; `owed_by`, `about`, `brought`, `share`; a grid every group of which is owed and whose families differ is no bijection, and a square one is | under 0.1 s |

The walls are each target whole on four P-core threads with four workers, on AC
(`EIN_TEST_JOBS=4 utils/bench_env.sh --cores PT:4 --require-ac cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test <target> -- --nocapture`,
2026-11-15). **What the stage added to the per-commit sweeps**: `structure_invariance` took 21.3 s before and
takes 21.1 s; `cover` took 14.4 s before and takes 14.4 s, a program that loads an obligation rule being loaded
a second time for the table a route reads and no other; `exactly_one` is new, 5.3 s. No sweep is a second pass
over the corpus.

**Both invariances failed before they held**, and each failure was the test's to find (2026-11-15). Under a
permuted id space `zebra.ein` and `features/13_mixed_solution_and_open.ein` moved, in L5's reason
([`findings.md`](findings.md)'s K-10), which the owner now states itself. Under the import forms the programs
that owe through `std.bool.inputs` moved, an instance's text writing a binding as `R=in`, where the helper that
takes a module's prefix off a name does not look after `=`; the comparison now spaces the sign.

### Acceptance, met

| | | evidence |
|---|---|---|
| T1's table and T2's comparison are recorded here for the corpus, and for the instances in `build/structure/<program>/` | met | [§ T1](#t1-the-table-of-the-three), [§ T2](#t2-every-bijection-found-or-declared); `exactly_one.json` and `.md` for each of the 625 programs read |
| **on I-Z2 every row and every column of five grids is an owed group** | **met** | `a_declared_bijections_rows_and_columns_are_owed_groups`: five grids, 36 groups, 36 owed, five bijections; read over all 56 candidates, as the group is defined |
| on I-Z1 the stage says exactly which halves are owed, which are licensed, and what is missing for the rest | met | [§ The controls](#the-controls): 18 owed and licensed, `slot-owed-room`; 18 owed by `slot-owed-fill` and not licensed, each about one of the 18 groups nothing owes; missing, the licence, for a witness no candidate proposes |
| **where exactly one cannot be derived, the stage says so and says what the program would have to state** | met | [§ T3](#t3-where-the-must-is-missing): 160 groups not without an obligation, 20 of which it is not true, 55 of which it is not in question, 22 that follow by a count whose premises are enforced |
| nothing is synthesised: no rule, no obligation, no closure | met | the owner reads L5's table and the cover; `ExactlyOne` has no field that says a count's conclusion |
| no answer, route or surface changes | met | no route calls the module; `Space::alternatives` builds the same table by the same call and `oblgen::alternatives` enumerates by the same loop, cut out; `explore`, `region`, `pair_layer`, `theory_share` green with no expectation edited, and the census is S1f.10.2's on all 644 lines but for the additions |

Of the stage's *must not*: *exactly one* is concluded from a count nowhere in the engine, and in this record
only with each premise written beside what enforces it; `is-a`, `bijective` and every stdlib name are read by
the census's scripts and by the tests, and by no line of `structure.rs`, `oblgen.rs` or `explore.rs`.

### Defaults taken

All are this stage's, taken on 2026-11-15, and the user's to revise at the phase's stop.

1. **An owed group is a group whose members are *exactly* a licensed instance's alternatives**, where the stage
   file has *pairwise excluding*. The alternatives are over all of H, a member dead alone at root among them.
   Alternatives that pairwise exclude inside a larger group are a standing of their own and no owed group.
2. **A grid is strict: every member in exactly two groups.** The looser reading, a line of one candidate
   allowed, would find 19 components more on 7 programs, some of them grids a cell of which root holds, and
   would call any path of exclusions a grid.
3. **A grid is over all of a component's candidates**, those dead alone at root among them, as a group is.
4. **A bijection has as many groups in each family.** A grid every group of which is owed and whose families
   differ in number is reported as that; no corpus program has one.
5. **An unlicensed instance is *about* a group by the small KB's nodes**: the candidates whose node there holds
   one of its witnesses are the group's members, and every witness root does not deny is so held. The stage file
   did not ask for the object by name; it is what lets the record say which 18 groups I-Z1's unlicensed half is
   about without reading a kind. It licenses nothing, and `ExactlyOne::is_owed` does not read it.
6. **The owner states one reason where L5 has two**, `UNPROPOSED`, and reads the table before any node, as a
   route does (K-10). So the census's `owed.unlicensed_why` has that sentence where S1f.10.2's had *a … fact
   can witness it that no candidate proposes*, on the same 22 instances of the searching programs.
7. **The table is read by a reader of the space that hands it to no node** (`Space::owed`), so a route that
   later takes its own table through `Space::alternatives` is as it was.
8. ***Declared* is `(bijective R)`**, under any module prefix, read by a script. A relation with the four
   properties stated one by one is *declared nowhere* by that word, and is listed with what it states.
9. **Where `ein oracle` refuses, R is compared by `ein solve --count`**, and the oracle runs on a reduced copy
   written for it. `--region-limit` was not raised, and nothing was run past the default budget.
10. **No new tier and no new step**: `exactly_one.rs` reads no flag, and the corpus checks are folded into the
    sweeps `cover.rs` and `structure_invariance.rs` already make.
11. **A count's conclusion is in no value**: `ExactlyOne::share` gives the two families' groups and how many are
    owed. Route 1 is analysis here, and whether anything spends it is S1f.10.14's.

### What it did not do

- **`./run_tests.sh` was not run**, nor the nightly, nor `utils/ci_container.sh`: the gate is the batch's. What
  was run is below.
- **No route reads it.** `exactly_one_in`'s one caller is `exactly_one`, over a space it has just opened.
- **That the table's order and a witness's place do not depend on the interner's ids is held by the sweep and
  not argued**: on the 188 searching programs no list moves.
- **No corpus program** has an owed set strictly inside a group, an instance with one alternative or with none,
  a component whose every member is in two groups and that is no grid, or a fully owed grid of unequal
  families. Those paths are held by the module's test on graphs written for it, and the last also by two
  probes.
- **`send_surjective.ein`'s R is not decided** at the default budget. That it is empty rests on the count, 8
  letters against 10 digits, and on the oracle over the reduced copy.
- **What a dead node holds is not read**: of I-Z1's 18 witnesses no alive single brings, the record says the
  small KB's node of the mirror holds each, and does not say what root's dead node of it holds.
- **Route 3's lines for `zebra2-hints.ein` and `b_branch.ein`** are named and not run.
- **The declared classes of S1f.10.2**, `functional` and `injective` by name, were not re-taken.

### The commands

2026-11-15, the working tree over `ad5a4fc5`, on AC. The census, one P-core (`cpu4`), governor `powersave`; it
is the one command S1f.10.10 and S1f.10.2 give, and *exactly one* comes with the cover:

```sh
mkdir -p build/structure
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer -p ein-cli
utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer --routes --check \
    > build/structure/census.jsonl 2> build/structure/census.env
```

The scripts below are under `build/structure/s4/`, not checked in, and whole here. The corpus's headline:

```jq
# jq -s -r -f build/structure/s4/exactly_one.jq build/structure/census.jsonl
def x: .theory.cover.exactly_one;
def count(f): map(select(f)) | length;
def total(f): map(f) | add // 0;
map(select(has("error") | not)) as $read
| ($read | map(select(.searches))) as $s
| ($s | map(select(x.groups > 0))) as $g
| ($s | map(x.grids[]?)) as $grids
| "read \($read | length), searching \($s | length); with a group \($g | length); a table read on \($s | count(x.table != null)), with an instance on \($s | count((x.table // 0) > 0)); no table: \($s | map(select(x.table == null) | x.no_table) | group_by(.) | map("\(.[0]) \(length)") | join(", ")); root owes with no table on \($s | count(x.table == null and x.root_owes > 0)) (\($s | map(select(x.table == null) | x.root_owes) | add) instances)",
  "instances root owes, in a table: \($s | total(x.table // 0))",
  "groups \($g | total(x.groups)): OWED GROUPS \($g | total(x.owed_groups)) on \($g | count(x.owed_groups > 0)) programs; GROUPS NOTHING OWES \($g | total(x.groups_nothing_owes.all)) on \($g | count(x.groups_nothing_owes.all > 0)): with an open member \($g | total(x.groups_nothing_owes.with_an_open_member)) on \($g | count(x.groups_nothing_owes.with_an_open_member > 0)), with none \($g | total(x.groups_nothing_owes.all - x.groups_nothing_owes.with_an_open_member)); an unlicensed instance is about \($g | total(x.groups_nothing_owes.an_unlicensed_instance_is_about)) on \($g | count(x.groups_nothing_owes.an_unlicensed_instance_is_about > 0))",
  "OWED SETS THAT ARE NOT A GROUP \($s | total(x.owed_sets_not_a_group)) on \($s | count(x.owed_sets_not_a_group > 0)) programs: \($s | map(select(x.owed_sets_not_a_group > 0) | "\(.path | split("/") | last) \(x.owed_sets_not_a_group)") | join(", "))",
  "instances L5 does not license \($s | total(x.unlicensed.all)) on \($s | count(x.unlicensed.all > 0)); about a group \($s | total(x.unlicensed.about_a_group))",
  ($s | map(x.unlicensed.why | to_entries[]) | group_by(.key)[] | "  why: \(.[0].key): \(map(.value) | add)"),
  "  their witnesses: not read on \($s | total(x.unlicensed.witnesses_not_read)) instances; no candidate and not denied \($s | total(x.unlicensed.witnesses_no_candidate_and_not_denied)), of them brought by a candidate alive alone at root \($s | total(x.unlicensed.of_them_brought_by_a_candidate_at_root)), not brought and the theory's bringers all dead alone at root \($s | total(x.unlicensed.not_brought_and_the_theorys_bringers_all_dead_alone_at_root)), brought by nothing, at root or in the theory \($s | total(x.unlicensed.not_brought_and_brought_by_nothing_in_the_theory))",
  "  instances: EVERY SUCH WITNESS BROUGHT BY A CANDIDATE AT ROOT \($s | total(x.unlicensed.every_such_witness_brought)); some not brought \($s | total(x.unlicensed.some_such_witness_not_brought)); no such witness \($s | total(x.unlicensed.no_such_witness))",
  "components \($s | total(x.components)): GRIDS \($grids | length) on \($s | count((x.grids | length) > 0)) programs; not grids: \($s | map(x.not_grids | to_entries[]) | group_by(.key) | map("\(.[0].key) \(map(.value) | add)") | join(", "))",
  "  grids by kind: \($s | map(x.grid_kinds | to_entries[]) | group_by(.key) | map("\(.[0].key) \(map(.value) | add)") | join("; ")); BIJECTIONS \($s | total(x.bijections)) on \($s | count(x.bijections > 0))",
  "  grids by shape (the two families' groups, the smaller first): \($grids | map(.groups | sort | "\(.[0])x\(.[1])") | group_by(.) | map("\(.[0]) × \(length)") | join(", "))",
  "  grids one family owed whole and the other not: \($grids | count((.bijection | not) and (.owed[0] == .groups[0] or .owed[1] == .groups[1]))): as many groups in each family \($grids | count((.bijection | not) and (.owed[0] == .groups[0] or .owed[1] == .groups[1]) and .groups[0] == .groups[1])), the unowed family's groups there \($grids | map(select((.bijection | not) and (.owed[0] == .groups[0] or .owed[1] == .groups[1]) and .groups[0] == .groups[1]) | .groups[0]) | add // 0); more groups in the unowed family \($grids | count((.bijection | not) and ((.owed[0] == .groups[0] and .groups[1] > .groups[0]) or (.owed[1] == .groups[1] and .groups[0] > .groups[1])))); fewer \($grids | count((.bijection | not) and ((.owed[0] == .groups[0] and .owed[1] < .groups[1] and .groups[1] < .groups[0]) or (.owed[1] == .groups[1] and .owed[0] < .groups[0] and .groups[0] < .groups[1]))))",
  "  grids of more than one relation: \($grids | count((.relations | length) > 1)); relations that hold more than one grid: \($s | map(x.grids | map(.relations[]) | group_by(.) | map(select(length > 1)) | length) | add)"
```

The per-program table:

```jq
# jq -r -f build/structure/s4/rows.jq build/structure/census.jsonl
select(has("error") | not) | select(.searches) | .theory.cover.exactly_one as $x
| select($x.groups > 0 or ($x.table // 0) > 0)
| "| `\(.path)` | \(.theory.rung) | \($x.table // "") | \($x.groups) | \($x.owed_groups) | \($x.groups_nothing_owes.all) | \($x.groups_nothing_owes.with_an_open_member) | \($x.owed_sets_not_a_group) | \($x.unlicensed.all) | \($x.unlicensed.about_a_group) | \($x.grids | map("\(.groups[0])×\(.groups[1]): \(.owed[0])+\(.owed[1])" + (if .bijection then " **b**" else "" end)) | join(", ")) | \($x.not_grids | to_entries | map("\(.key) \(.value)") | join("; ")) |"
```

T2, `python3 build/structure/s4/bijections.py build/structure build/structure/census.jsonl`:

```python
"""Every bijection, found or declared: the grids the owner finds against what a program declares by name.

A script, and not the engine: it reads `bijective` (under any module prefix) in the facts a program states, and a
relation's name off a candidate's text, which the owner may not. *Declared*: the program states `(bijective R)` of a
relation R. *Found*: a grid of the owner's every group of which is an owed group (`exactly_one.json`'s `bijection`).
A declared R is set against the grids whose members are candidates of R:

  found and declared      R has candidates in H, every one is a member of a grid, and each of those grids is a bijection
  declared, nothing left  R has no candidate in H: root decides the whole relation
  declared and not found  anything else, with what broke it
  found, declared nowhere a bijection none of whose members' relations is declared bijective

    python3 build/structure/s4/bijections.py build/structure build/structure/census.jsonl
"""
import json, sys
from collections import Counter

out, rows = sys.argv[1], [json.loads(l) for l in open(sys.argv[2])]
tot, lines, why_not, undeclared, piecewise = Counter(), [], Counter(), [], []
for d in rows:
    if 'error' in d or not d.get('searches'): continue
    x = d.get('theory', {}).get('cover', {}).get('exactly_one')
    if not x: continue
    rel = d['path'][:-4]
    t = json.load(open(f'{out}/{rel}/theory.json')); e = json.load(open(f'{out}/{rel}/exactly_one.json'))
    cov = json.load(open(f'{out}/{rel}/cover.json'))
    declared, props = set(), {}
    for s in t['stated']:
        f = s['fact'].strip('()').split()
        if len(f) < 2: continue
        name = f[0].split('.')[-1]
        if name == 'bijective' and len(f) == 2: declared.add(f[1])
        if name in ('functional', 'injective', 'total', 'surjective', 'total-owed', 'surjective-owed'):
            props.setdefault(f[1], set()).add(name)
    cands = {}                                   # relation -> its candidates
    for c in cov['candidates']:
        cands.setdefault(c['candidate'].strip('()').split()[0], []).append(c)
    grids = e['grids']
    in_grid = {}                                 # a member's text -> its grid
    for g in grids:
        for f in g['families'][0]['groups']:
            for m in e['groups'][f]['members']: in_grid[m] = g['grid']
    tot['programs'] += 1
    tot['grids'] += len(grids); tot['bijections'] += sum(g['bijection'] for g in grids)
    for g in grids: tot['grids: ' + g['kind']] += 1
    for R in sorted(declared):
        tot['declared'] += 1
        mine = cands.get(R, [])
        if not mine:
            tot['declared, nothing left'] += 1; continue
        gs = sorted({in_grid[c['candidate']] for c in mine if c['candidate'] in in_grid})
        loose = [c for c in mine if c['candidate'] not in in_grid]
        if not loose and all(grids[k]['bijection'] for k in gs):
            tot['found and declared'] += 1; tot['found and declared, grids'] += len(gs); continue
        tot['declared and not found'] += 1
        if loose:
            n = Counter(len(c['groups']) for c in loose)
            why = ('a candidate is in more than two groups' if max(n) > 2 else
                   'no pair of its candidates is excluded by the theory' if set(n) == {0} else
                   'a candidate is in fewer than two groups')
        else:
            owed = [tuple(f['owed'] for f in grids[k]['families']) for k in gs]
            about = sum(f['an_unlicensed_instance_is_about'] for k in gs for f in grids[k]['families'])
            why = ('its grids are found and nothing owes a group of them' if not any(any(o) for o in owed) else
                   'its grids are found and one family of each is owed' + (', the other only by instances L5 does not license' if about else ''))
        why_not[why] += 1
        lines.append((d['path'], R, len(mine), len(gs), why))
    for g in grids:
        if g['bijection'] and not (set(g['relations']) & declared):
            tot['found, declared nowhere'] += 1
            undeclared.append((d['path'], g['relations'], g['members'], [len(f['groups']) for f in g['families']],
                               sorted(set().union(*(props.get(r, set()) for r in g['relations'])))))
print(f"searching programs read: {tot['programs']}; grids {tot['grids']}, of them bijections {tot['bijections']}")
for k in sorted(tot):
    if k.startswith('grids: '): print(f"  {k}: {tot[k]}")
print(f"relations declared bijective: {tot['declared']}: found and declared {tot['found and declared']} (over {tot['found and declared, grids']} grids), "
      f"declared with nothing left in H {tot['declared, nothing left']}, declared and not found {tot['declared and not found']}")
for w, n in sorted(why_not.items()): print(f"  not found, {w}: {n}")
print(f"bijections found and declared nowhere: {tot['found, declared nowhere']}")
print('| program | relation | its candidates | grids over them | what broke it |'); print('|---|---|---:|---:|---|')
for r in lines: print('| `%s` | `%s` | %d | %d | %s |' % r)
print('| program | the grid\'s relations | members | groups a family | what the program states of them, by name |'); print('|---|---|---:|---|---|')
for p, rels, m, fam, props in undeclared: print(f"| `{p}` | {' '.join('`%s`' % r for r in rels)} | {m} | {fam[0]} and {fam[1]} | {' '.join(props)} |")
```

T3's sort of the groups nothing owes, `python3 build/structure/s4/unowed.py build/structure
build/structure/census.jsonl`:

```python
"""The groups nothing owes, by what a count could say of each: T3's first route, over the corpus.

For every searching program, each group no licensed instance owes is one of:
  no open member          every member is dead alone at root: no alive state holds one, and no *must* is in question
  counted, as many        it is in a grid whose other family is owed whole, and the two families have as many groups
  counted, more           the same, and its own family has more groups than the owed one: some group of it stays empty
  counted, fewer          the same, and its own family has fewer
  no owed family          it is in a grid, and neither family is owed whole
  in no grid              its component is no grid
    python3 build/structure/s4/unowed.py build/structure build/structure/census.jsonl
"""
import json, sys
from collections import Counter, defaultdict
out, rows = sys.argv[1], [json.loads(l) for l in open(sys.argv[2])]
tot, by = Counter(), defaultdict(Counter)
for d in rows:
    if 'error' in d or not d.get('searches'): continue
    x = d.get('theory', {}).get('cover', {}).get('exactly_one')
    if not x or not x['groups']: continue
    e = json.load(open(f"{out}/{d['path'][:-4]}/exactly_one.json"))
    for g in e['groups_nothing_owes']:
        G = e['groups'][g]
        if not G['open']: k = 'no open member'
        elif G['grid'] is None: k = 'in no grid'
        else:
            grid = e['grids'][G['grid']['grid']]; mine = grid['families'][G['grid']['family']]; other = grid['families'][1 - G['grid']['family']]
            if other['owed'] != len(other['groups']): k = 'no owed family'
            elif len(mine['groups']) == len(other['groups']): k = 'counted, as many'
            elif len(mine['groups']) > len(other['groups']): k = 'counted, more'
            else: k = 'counted, fewer'
        tot[k] += 1; by[k][d['path']] += 1
print(f"groups nothing owes: {sum(tot.values())}")
for k, n in sorted(tot.items()):
    print(f"  {k}: {n} on {len(by[k])} programs" + (': ' + ', '.join(f'{p} {m}' for p, m in sorted(by[k].items())) if len(by[k]) <= 8 else ''))
```

The probes, `bash build/structure/s4/probes.sh`, with the pieces under [§ The probes](#the-probes) in
`build/structure/s4/p/`; and the owner's reading of them:

```sh
#!/usr/bin/env bash
# T3's probes: each program with the lines that would state the *must*, against the program without them.
#     bash build/structure/s4/probes.sh          # from the repo root; writes build/structure/s4/out/
set -u
cd "$(dirname "$0")/../../.."
EIN=${EIN_BIN:-ein.rs/target/release/ein}
P=build/structure/s4/p
OUT=build/structure/s4/out
mkdir -p "$OUT"
bash $P/make_letters.sh
bash $P/make_seats.sh
cat examples/zebra.ein $P/fill_mirrored.lines > $P/zebra_fill_mirrored.ein
cat $P/slots_2x2.ein $P/fill_mirrored.lines > $P/slots_2x2_fill_mirrored.ein
sed -e 's/(import std.algebra :symbols (total-owed functional injective))/(import std.algebra :symbols (total-owed functional injective surjective-owed))/' \
    -e 's/^(injective val)  .*$/&\n(surjective-owed val is-a)   ; the probe'"'"'s line: and every digit owes a letter/' \
    examples/beyond_arith/send_more_money.ein > $P/send_surjective.ein
cat examples/branching/06_lookahead_on.ein $P/typed_owed.lines > $P/b06_typed_owed.ein
cat examples/branching/01_saturate_only.ein $P/typed_owed_small.lines > $P/b01_typed_owed.ein

name() { echo "$1" | sed -e 's|/|_|g' -e 's|\.ein$||'; }
r() { # the region block of a read-out, with the two counts above it
  grep -E '^  (states|commitments|exhausted|empty|∅|universe|work|verdict|decided)' "$1"
  sed -n '/^  (region/,/^$/p' "$1"
}
oracle() { for f in "$@"; do "$EIN" oracle "$f" > "$OUT/$(name "$f").oracle.txt" 2>&1; echo "exit $?" >> "$OUT/$(name "$f").oracle.txt"; done; }
count()  { for f in "$@"; do "$EIN" solve "$f" --count > "$OUT/$(name "$f").count.txt" 2>&1; echo "exit $?" >> "$OUT/$(name "$f").count.txt"; done; }
single() { for f in "$@"; do "$EIN" solve "$f" --single > "$OUT/$(name "$f").single.txt" 2>&1; echo "exit $?" >> "$OUT/$(name "$f").single.txt"; done; }
same() { # two read-outs' R: the counts and the region block
  if diff <(r "$1" | grep -vE '^  (∅|universe|work)') <(r "$2" | grep -vE '^  (∅|universe|work)') > /dev/null; then echo "SAME R      $1  $2"; else echo "ANOTHER R   $1  $2"; fi
}

# The oracle, where 2^n is within its limit.
oracle $P/slots_2x2.ein $P/slots_2x2_fill_mirrored.ein \
       $P/letters_2x4.ein $P/letters_2x4_surjective.ein $P/letters_2x2.ein $P/letters_2x2_surjective.ein \
       examples/branching/01_saturate_only.ein $P/b01_typed_owed.ein \
       $P/seats_declared.ein $P/seats_owed.ein
# And where it is not (56, 80 and 132 candidates): the oracle's own refusal, then describe's route and --single.
oracle examples/zebra.ein examples/beyond_arith/send_more_money.ein examples/branching/06_lookahead_on.ein
count  examples/zebra.ein $P/zebra_fill_mirrored.ein examples/branching/06_lookahead_on.ein $P/b06_typed_owed.ein
single examples/zebra.ein $P/zebra_fill_mirrored.ein examples/beyond_arith/send_more_money.ein $P/send_surjective.ein

{
  same $OUT/$(name $P/slots_2x2.ein).oracle.txt   $OUT/$(name $P/slots_2x2_fill_mirrored.ein).oracle.txt
  same $OUT/$(name $P/letters_2x2.ein).oracle.txt $OUT/$(name $P/letters_2x2_surjective.ein).oracle.txt
  same $OUT/$(name $P/letters_2x4.ein).oracle.txt $OUT/$(name $P/letters_2x4_surjective.ein).oracle.txt
  same $OUT/$(name examples/branching/01_saturate_only.ein).oracle.txt $OUT/$(name $P/b01_typed_owed.ein).oracle.txt
  same $OUT/$(name $P/seats_declared.ein).oracle.txt $OUT/$(name $P/seats_owed.ein).oracle.txt
  same $OUT/$(name examples/zebra.ein).count.txt  $OUT/$(name $P/zebra_fill_mirrored.ein).count.txt
  same $OUT/$(name examples/branching/06_lookahead_on.ein).count.txt $OUT/$(name $P/b06_typed_owed.ein).count.txt
  for f in $OUT/*.oracle.txt $OUT/*.count.txt $OUT/*.single.txt; do echo "--- $f"; grep -E '^  (states|commitments|exhausted|empty|∅|universe|work|verdict|decided)|^exit|refus|limit' "$f" | cut -c1-200; done
} > $OUT/summary.txt
cat $OUT/summary.txt
```

```sh
ein.rs/target/release/examples/pair_layer --theory --out build/structure/s4/out/structure \
    build/structure/s4/p/zebra_fill_mirrored.ein build/structure/s4/p/slots_2x2.ein \
    build/structure/s4/p/slots_2x2_fill_mirrored.ein build/structure/s4/p/letters_2x2.ein \
    build/structure/s4/p/letters_2x2_surjective.ein build/structure/s4/p/letters_2x4.ein \
    build/structure/s4/p/letters_2x4_surjective.ein build/structure/s4/p/send_surjective.ein \
    build/structure/s4/p/b06_typed_owed.ein build/structure/s4/p/seats_declared.ein \
    build/structure/s4/p/seats_owed.ein > build/structure/s4/out/structure.jsonl
```

What was run to hold the stage, each green:

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --lib --test cover --test structure_invariance --test pair_layer --test theory_share --test explore --test region --test exactly_one
EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test structure_invariance
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test nightly_steps --test gate_steps --test config_reference
cargo fmt --manifest-path ein.rs/Cargo.toml --all --check
cargo clippy --manifest-path ein.rs/Cargo.toml --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -q --manifest-path ein.rs/Cargo.toml --workspace --no-deps
python3 utils/check_hashmap_iteration.py
python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md \
    examples/README.md stdlib/README.md utils/README.md c/README.md
```
