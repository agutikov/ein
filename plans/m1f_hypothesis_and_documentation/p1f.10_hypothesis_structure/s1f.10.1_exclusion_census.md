# S1f.10.1 — The pair layer: what one candidate and two decide at root

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** built and measured 2026-11-15: [§ Record](#record-2026-11-15). One acceptance bullet is met
under a narrower reading than it is written in, and it is S1f.10.2's first question.
**Depends on:** nothing.
**Read by:** every other stage of tracks S and R.

## What it is

The first object of [§ The structure](README.md#the-structure), read from the program's own lattice. For H, the
universe of `n` candidates root builds:

| object | read from | definition |
|---|---|---|
| **dead alone** | the node `{a}` | it is dead |
| **implies** | the node `{a}`, alive | `b` is in its closure: committing `a` brings `b` |
| **equivalent** | two alive single nodes | one closure, as a candidate and its symmetric twin in `zebra.ein` |
| **excludes** | the node `{a, b}`, both alive alone, neither implying the other | it is dead |
| **interacts** | the same node, alive | its fixpoint holds a fact that neither `{a}`'s nor `{b}`'s does |

Each is a statement about a node [`region.rs`](../../../ein.rs/crates/ein-infer/src/region.rs) defines, so each
is true of the lattice R is a region of, and the excluded pairs are by definition the members of N that hold
two candidates. *Interacts* is recorded here because the node is already computed; it is
[S1f.10.11](s1f.10.11_coupling.md)'s input.

**What needs no saturation.** A pair with a member dead alone is dead (L1). A pair one of whose members implies
the other has the larger closure (L3). Both hold on a program the static check calls monotone, and the census
says for each program which licence it used; on a flagged program it saturates every pair and prints the
[domain](README.md#the-domain)'s sentence.

## The vehicle

A measurement example in the crate, `ein-infer/examples/pair_layer.rs`, beside `state_reach` and
`route1_reach` and in their shape: every corpus file and register witness that loads, or the `FILE`s given; one
JSON line per program; `--jobs`; a budget in facts beyond root. It computes a node through the function the
routes use (`explore::Space`'s), never a second one, which is what makes its answer comparable with a
describe's.

The old plan built this as a script over `ein saturate` on a rewritten copy of the program. That reads a pair
as two **stated** facts of another program, which is the from-scratch route; the lattice's node is a fork of
root, and where the two differ is F21's. The small-KB check the user's instruction describes is
[S1f.10.10](s1f.10.10_the_theorys_share.md)'s, where it is the subject.

## Tasks

1. **T1f.10.1.1 — the example.** Singles first, then the pairs the two licences leave. Per program, one line:
   `n`, the static verdict, dead alone, the implications, the equivalence classes, pairs saturated, pairs
   excluded, pairs interacting, saturations and facts beyond root spent, wall; and the lists themselves, as
   rendered text, in a file per program under `build/structure/<program>/pairs.json`.
2. **T1f.10.1.2 — held to N.** A test, in the gate: on every corpus program whose describe exhausts within the
   default budget, the excluded pairs are exactly N's members of two candidates and the candidates dead alone
   are the read-out's `dead alone`. Two computations, one answer. N is read from
   [`described.rs`](../../../ein.rs/crates/ein-infer/src/described.rs), which takes it from the maximal alive
   states by duality, so the two share no code past the node. The sweep uses `ein_corpus::pool::map`.
3. **T1f.10.1.3 — the census.** Over every corpus program that searches (`corpus_ops::searches`): the line
   above, and three derived columns: the share of all pairs that exclude; the share of the candidates in at
   least one excluded pair; and the layer's cost against the program's describe, `--single` and `--maximal`
   where they finish, in saturations. Recorded in this file, with the command and the date.
4. **T1f.10.1.4 — what it does not reach.** Every program the budget stops is named with its `n` and the
   pairs left. Every flagged program is named.

## Acceptance

- The example exists, is listed in [`utils/README.md`](../../../utils/README.md)'s measurement set, and is
  re-takable by one command.
- T2's test is in the gate and green; it reads no program past the default budget.
- The census is recorded here: per program the columns of T3, the programs T4 names, and for the
  [instances](README.md#the-instances) the lists themselves, linked.
- **I-L02 has no excluded pair**, three alive singles and no implication.
- **I-Z2's excluded pairs include every same-value and every same-house pair of each `*-loc` relation** among
  the candidates root leaves, and the census says how many more there are; those are the puzzle's, and
  S1f.10.10 says which.
- No answer, route or surface changes. The example reads.

## What it must not do

- **Not filter pairs by type or by relation.** The kernel imposes no type system; a pair is skipped only by L1
  or L3. What a cheaper way of getting the pairs would be is [S1f.10.13](s1f.10.13_the_price_list.md)'s to
  price.
- **Not write a second node function.**
- **Not add a CLI option or a subcommand.** The surface is [S1f.10.12](s1f.10.12_the_structure_shown.md)'s,
  after the user has seen the structure.

## The standing rules

Config never changes an answer. No test runs past the default `--region-budget`. A corpus sweep uses the worker
pool. The gate is `./run_tests.sh`, once on the last commit of the batch this stage's code is in.

## Record (2026-11-15)

### What was built

- **The owner**: [`ein-infer/src/structure.rs`](../../../ein.rs/crates/ein-infer/src/structure.rs), the module
  `ein_infer::structure`. `pair_layer(root, terms, ast, &Options) -> PairLayer` takes a freshly loaded program;
  inside the crate `pair_layer_in(&mut Space)` takes **the program's prepared space**, which is the entry
  [S1f.10.2](s1f.10.2_groups.md)'s T4 and track R want. A node is `explore::Space::eval`'s, the cache the
  obligation tree reads, over `region::fan_out`: no second node function. For it `Space::open` became
  crate-visible and `Space` got six readers (`candidates`, `is_monotone`, `root_is_dead`, `work`, `affordable`,
  `states`); nothing a route calls changed.
- **The vehicle**: [`ein-infer/examples/pair_layer.rs`](../../../ein.rs/crates/ein-infer/examples/pair_layer.rs),
  listed in [`utils/README.md`](../../../utils/README.md)'s measurement set. One JSON line per program; the
  lists as rendered text in `build/structure/<program>/pairs.json`, `<program>` the path from the repo root
  without `.ein`; `--facts`, `--jobs`, `--out`; `--routes` for T3's third column.
- **The tests**: [`ein-infer/tests/pair_layer.rs`](../../../ein.rs/crates/ein-infer/tests/pair_layer.rs), seven,
  T2's among them; and one in the module, the licences against the nodes they skip. A row of
  [`architecture.md`](../../../docs/ein/ein.rs/architecture.md)'s module map.

### What the definition had to settle

Each is stated in the module's header, and none is a lever.

1. **Root's node is read**, one saturation, for the candidates root's own fixpoint holds (`at_root`): committing
   one is committing nothing, so they are in no implication and no class. The corpus has none.
2. **A dead root** leaves every candidate dead alone by L0 without a node, as the read-out counts them; 148 of
   the 625 programs read.
3. **A flagged program**: every pair saturated, and two lists more, `against_l1` (a pair alive over a member
   dead alone) and `against_l3` (a pair one of whose members implies the other, whose node is not that
   member's). **Both are empty on all 29 flagged programs of the corpus.** So no corpus program shows L1 or L3
   failing *at the pair layer*, though the static check cannot promise it.
4. **L3 is used as this file says and no further.** Two equivalent candidates have one state, so their pairs
   with a third are one node; the layer saturates each, and the example prints `open_by_class`, what reading L3
   over the classes would leave: 71 of I-Z2's 408 saturated pairs, 113 of I-Z1's 513, 1 798 of I-Z2M's 4 449.
   That is a cheaper way of getting the pairs and so S1f.10.13's to price.
5. **The budget** is the cache's own projection, taken in steps no larger than it affords, so a stopped layer
   has read every single or says how many it left, and lists exactly the pairs it saturated.

### The commands

```sh
mkdir -p build/structure
utils/bench_env.sh cargo run --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer -- \
    --routes > build/structure/census.jsonl
```

That is the one command. The census below was taken as its two halves, the example built first and
`utils/bench_env.sh ein.rs/target/release/examples/pair_layer --routes` run after, so that no build is in a
`wall`: 2026-11-15, the working tree over `d7050ccf`, one P-core (`cpu4`), on AC, governor `powersave`, load
average 4.6 at the start, the default budget of 2^20 facts, `--jobs 1`, 2 min 22 s for the sweep. Taken twice: every field but the four `wall`s is the same in both, so the counts are the program's and only
the `wall` columns are this machine's. The tables below are
`build/structure/census.jsonl` through the `jq` program under [§ Every program that searches](#every-program-that-searches).

### The census, T3

**644** corpus files and register witnesses load. *Forbid* refuses **19**, and the layer of the other **625** is
read, **none stopped by the budget**: the largest, I-Z2M's, is 475 193 facts. 426 of them have no candidate and
148 a dead root. **188 search** (`corpus_ops::searches`' rule, by the same call), and of those:

| | programs |
|---|---:|
| with a candidate dead alone | 64 |
| with an implication | 41 |
| with an equivalence class | 15 |
| with an excluded pair | 44 |
| with an interacting pair | 40 |
| a licence decided a pair: L1 alone, L3 alone, both, none | 37, 27, 11, 113 |
| flagged, every pair saturated | 29 |

Over the 188 together: 93 208 pairs, 35 664 dead with a member (L1), 11 404 implied (L3), 46 140 saturated, of
which 5 565 exclude and 4 176 interact; 49 015 saturations and 1 389 866 facts beyond root.

**Against the routes, in saturations**, each route asked at the same budget, root's node counted on both sides:

| the layer costs | more | the same | fewer | where the route finishes |
|---|---:|---:|---:|---:|
| than a describe | 4 | 117 | 51 | 172 |
| than `--single` | 132 | 53 | 3 | 188 |
| than `--maximal` | 70 | 73 | 38 | 181 |

The four it costs more than a describe on are `zebra.ein` (570 against 445), `zebra2.ein` and
`zebra2-obligations.ein` (465 against 374) and `zebra2-hints.ein` (94 against 78): the puzzles this phase is
about. The three it costs fewer than `--single` on are I-SEND (3 084 against 17 747),
`beyond_arith/equation_two_unknowns.ein` (466 against 524) and
`lattice/06_friends_tables_everyone_sits.ein` (173 against 207). The re-plan's second premise stands as measured:
a question of 39 or 45 saturations does not pay for this layer.

### The instances

| | n | dead alone | implies | classes (candidates) | pairs | L1 | L3 | saturated | excluded | interacting | excluded, of all pairs | candidates in an excluded pair | saturations | facts beyond root | wall |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **I-Z1** `zebra.ein` | 56 | 18 | 237 | 9 (30) | 1 540 | 837 | 190 | 513 | 414 | 81 | 26.9 % | 38 | 570 | 181 088 | 1.95 s |
| **I-Z2** `zebra2.ein` | 56 | 21 | 234 | 9 (30) | 1 540 | 945 | 187 | 408 | 327 | 66 | 21.2 % | 35 | 465 | 68 385 | 1.01 s |
| **I-Z2M** `zebra2-minus-15.ein` | 96 | 0 | 149 | 32 (67) | 4 560 | 0 | 111 | 4 449 | 1 428 | 1 614 | 31.3 % | 96 | 4 546 | 475 193 | 5.99 s |
| **I-B06** `branching/06_lookahead_on.ein` | 132 | 130 | 2 | 1 (2) | 8 646 | 8 645 | 1 | 0 | 0 | 0 | 0 % | 0 | 133 | 440 | 2 ms |
| **I-L02** `lattice/02_genuine_3set_death.ein` | 3 | 0 | 0 | 0 | 3 | 0 | 0 | 3 | 0 | 0 | 0 % | 0 | 7 | 9 | under 1 ms |
| **I-SEND** `beyond_arith/send_more_money.ein` | 80 | 2 | 0 | 0 | 3 160 | 157 | 0 | 3 003 | 609 | 90 | 19.3 % | 78 | 3 084 | 6 897 | 72 ms |

| the routes, saturations (wall) | describe | `--single` | `--maximal` |
|---|---:|---:|---:|
| I-Z1 | 445 (1.13 s) | 49 (0.19 s) | 49 (0.19 s) |
| I-Z2 | 374 (0.63 s) | 45 (0.11 s) | 45 (0.11 s) |
| I-Z2M | stopped by the budget at 8 391 (10.5 s) | 39 (0.05 s) | 2 346 (5.87 s) |
| I-B06 | 133 | 8 | 258 |
| I-L02 | 8 | 2 | 7 |
| I-SEND | stopped by the budget at 336 101 (13.8 s) | 17 747 (0.80 s) | 17 754 (0.80 s) |

The lists themselves are written by the command above, and are not checked in: `build/structure/examples/zebra/pairs.json`,
`…/examples/zebra2/pairs.json`, `…/examples/zebra2-minus-15/pairs.json`,
`…/examples/branching/06_lookahead_on/pairs.json`, `…/examples/lattice/02_genuine_3set_death/pairs.json` and
`…/examples/beyond_arith/send_more_money/pairs.json`, each with `candidates`, `dead_alone`, `implies`,
`equivalent`, `excluded` and `interacting` as the program writes its facts. What each says:

- **I-L02.** Three alive singles, no implication, three alive pairs, none interacting. Pairs see nothing, as
  planned; its N is the triple, which `a_conflict_of_three_shows_no_excluded_pair` reads off a describe.
- **I-Z2.** Of H's 56 candidates **21 are dead alone**, the read-out's count. 128 pairs of H share a row or a
  column of one `*-loc` relation (the same value, or the same house). **45 of them have both members alive
  alone, and all 45 exclude. The other 83 hold a member dead alone**: they are dead (L1) and by this file's
  definition are not exclusions. **282 excluded pairs more** are neither: 258 across two relations, 24 within one
  relation with another value and another house, as `(color-loc Green House-4)` with `(color-loc Red House-3)`.
  Those are the clues', and S1f.10.10 says which. Its nine classes are the candidates one commitment brings
  together, the largest of seven: `(color-loc Green House-4)`, `(color-loc Ivory House-3)`,
  `(color-loc Red House-5)`, `(drink-loc Coffee House-4)`, `(drink-loc Juice House-5)`,
  `(nation-loc Englishman House-5)`, `(smoke-loc Lucky_Strike House-5)`.
- **I-Z2M.** No candidate dead alone, so all 318 row and column pairs exclude, and 1 110 pairs more (1 028
  across relations, 82 within one). Every candidate is in an excluded pair. 1 614 alive pairs interact, more
  than exclude. **Nothing holds this layer to N**: its describe does not exhaust and its cube is 2^96.
- **I-Z1.** One relation, `co-located`, and H holds one orientation of it, `(co-located <name> House-n)`, 56
  candidates. 414 excluded pairs, with no relation to tell a row from a column: 58 pairs that share a house
  are alive, two attributes of one house. Its nine classes have I-Z2's sizes (7, 5, 4, 3, 3, 2, 2, 2, 2).
- **I-B06.** 130 of 132 dead alone; the two left are `(co-located Blue H5)` and `(co-located H5 Blue)`, one
  class, each implying the other, so its one pair not dead by L1 is decided by L3 and **no pair is saturated**.
  The layer costs what its describe costs, 133.
- **I-SEND.** `(val M 0)` and `(val S 0)` are dead alone. **Its 609 excluded pairs are exactly the pairs of
  alive candidates with the same letter or the same digit**: different letters different digits, and one digit
  a letter. **Its 90 interacting pairs are exactly `(val D d)` with `(val E e)`, `d ≠ e`**: the one column both
  of whose letters a pair can hold. Its singles derive almost nothing, 2.2 facts a node, which is why 3 084
  saturations are 6 897 facts and 72 ms.

### Three things the plan had otherwise

1. **The same-row and same-column acceptance holds over the candidates alive alone, not over all root leaves.**
   This file's bullet says I-Z2's excluded pairs include every such pair *among the candidates root leaves*. Root
   leaves 56 and 21 of them are dead alone, so 83 of the 128 such pairs are dead by a dead member and are not
   exclusions under the definition at the top of this file. The definition was kept and the bullet is read over
   the 35 alive: 45 of 45. **This is S1f.10.2's first question**: a clique of the exclusion graph cannot hold a
   candidate dead alone, so the discovered rows and columns will be those of the 35, and the phase's acceptance
   *over the candidates root leaves* needs the same reading or a ruling that a dead-alone candidate belongs to
   the groups of its row and column.
2. **`zebra.ein` has no symmetric twin in H.** The table's example for *equivalent* is wrong of I-Z1: its hrule
   proposes one orientation. The twin is I-B06's, where the blind enumerator proposes both.
3. **The layer is not cheaper than a describe where it matters most**: more saturations than a describe on the
   four zebra programs whose describe finishes. Track S's acceptance does not ask it to be; S1f.10.13 prices it.

### What it does not reach, T4

- **The budget stops no program** at the default 2^20 facts. A smaller one does, and says so:
  `a_stopped_layer_says_what_it_read`.
- **The 29 flagged programs**, each with every pair saturated and the domain's sentence on its line, `n` in
  parentheses: under `examples/ein-bugs/wrong-answers/`, `core-activator-committed` (2),
  `core-activator-guard-only` (2), `f07-fpoff` (1), `f07-rules-swapped` (1), `forbid-chain-no-absent` (2),
  `guard-only-first` (2), `guard-only-second` (2), `kw-neq-in-absent-twin` (3), `kw-neq-in-absent` (3),
  `oblig-derived-activator-declared` (2), `oblig-derived-activator` (1), `oblig-stdlib-total-owed-declared` (6),
  `oblig-stdlib-total-owed` (6), `past-limit-lines` (17); under `examples/features/`, `01_not_and_absent` (35),
  `04_open` (81), `07_unstratifiable` (1); under `examples/lattice/`, `04_obligation_trigger_hole` (2),
  `06_friends_tables` (19), `08_absent_witness_hole` (3), `09_activator_hole` (2), `10_fifo_gap` (2),
  `12_activator_chain_hole` (2), `synth/local_not_global` (2), `synth/maximal_not_top` (3); under
  `examples/saturation/square-unique/`, `corner-house` (118), `cul-de-sac` (114), `terminus` (153); and
  `tests/region/07_cube` (2).
- **The 19 programs *forbid* refuses** have no layer, as they have no answer.
- **Sixteen searching programs' layers are held to no N**, because no describe of theirs exhausts: I-SEND,
  I-Z2M and `zebra2-minus-15-obligations.ein`, the six `saturation/square-fwd` and `square-bwd` demos (the
  budget), and seven flagged programs past `--region-limit` (`past-limit-lines`, `features/01` and `/04`,
  `lattice/06_friends_tables`, the three `square-unique`). Their layers are read by the node function the other
  609 are, and by nothing else.

### Acceptance

| | | evidence |
|---|---|---|
| the example exists, is listed, is re-takable by one command | met | [§ The commands](#the-commands) |
| T2's test is in the gate and green, and reads nothing past the default budget | met | `the_excluded_pairs_are_the_pairs_of_n`, in `cargo test --workspace`: 607 programs compared per commit, 38 with an excluded pair, 22 flagged, 148 dead roots; 609 under `EIN_CORPUS_SLOW=1`. Both sides at 2^20. It fails on a layer that drops one excluded pair or one dead single (128 disagreements, tried and reverted) |
| the census is recorded here | met | above and below |
| I-L02 has no excluded pair, three alive singles and no implication | met | `a_conflict_of_three_shows_no_excluded_pair` |
| I-Z2's excluded pairs include every same-value and every same-house pair | **met over the candidates alive alone; not as written** | `a_declared_bijection_excludes_along_its_rows_and_columns`; [§ Three things](#three-things-the-plan-had-otherwise), 1 |
| no answer, route or surface changes | met | no route calls the module; `explore.rs`'s change is visibility and readers |

The slow half of T2's test is run by hand and is in no tier: it adds two programs to the comparison. Whether it
becomes a nightly step is [`findings.md`](findings.md)'s K-2.

### Every program that searches

`n`; `v` the static verdict, `m` monotone or `f` flagged; the classes with their candidates in parentheses; the
pairs, those dead with a member (L1), implied (L3) and saturated; the two shares in per cent; the layer's
saturations and facts beyond root; and each route's saturations where it finishes, `> k` where the budget
stopped it at `k`, `limit` where a flagged program's cube is past `--region-limit`.

```sh
jq -r '
def route(k): .routes[k] | if .finished then "\(.saturations)" elif .stopped.why == "limit" then "limit" else "> \(.saturations)" end;
def pct: if . == null then "" else "\((. * 1000 | round) / 10)" end;
select(.searches)
| "| `\(.path)` | \(.n) | \(if .monotone then "m" else "f" end) | \(.dead_alone) | \(.implications) | \(.equivalence_classes) (\(.equivalent_candidates)) | \(.pairs.all) | \(.pairs.dead_member) | \(.pairs.implied) | \(.pairs.saturated) | \(.pairs.excluded) | \(.pairs.interacting) | \(.share_excluded | pct) | \(.share_in_excluded | pct) | \(.saturations) | \(.facts) | \(route("describe")) | \(route("single")) | \(route("maximal")) |"
' build/structure/census.jsonl
```

| program | n | v | dead alone | implies | classes | pairs | L1 | L3 | saturated | excluded | interacting | excluded % | in an excluded pair % | saturations | facts | describe | `--single` | `--maximal` |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `examples/beyond_arith/cryptarithm.ein` | 30 | m | 2 | 0 | 0 (0) | 435 | 57 | 0 | 378 | 215 | 1 | 49.4 | 93.3 | 409 | 1246 | 417 | 119 | 128 |
| `examples/beyond_arith/equation_by_search.ein` | 30 | m | 9 | 0 | 0 (0) | 435 | 225 | 0 | 210 | 99 | 1 | 22.8 | 70 | 241 | 598 | 251 | 31 | 38 |
| `examples/beyond_arith/equation_no_solution.ein` | 20 | m | 9 | 0 | 0 (0) | 190 | 135 | 0 | 55 | 55 | 0 | 28.9 | 55 | 76 | 224 | 76 | 21 | 21 |
| `examples/beyond_arith/equation_two_unknowns.ein` | 30 | m | 0 | 0 | 0 (0) | 435 | 0 | 0 | 435 | 135 | 0 | 31 | 100 | 466 | 1035 | 1466 | 524 | 1165 |
| `examples/beyond_arith/send_more_money.ein` | 80 | m | 2 | 0 | 0 (0) | 3160 | 157 | 0 | 3003 | 609 | 90 | 19.3 | 97.5 | 3084 | 6897 | > 336101 | 17747 | 17754 |
| `examples/binary_logic/equivalent.ein` | 6 | m | 0 | 0 | 0 (0) | 15 | 0 | 0 | 15 | 3 | 4 | 20 | 100 | 22 | 87 | 30 | 15 | 15 |
| `examples/binary_logic/knights.ein` | 6 | m | 0 | 0 | 0 (0) | 15 | 0 | 0 | 15 | 5 | 6 | 33.3 | 100 | 22 | 97 | 26 | 10 | 14 |
| `examples/binary_logic/not_equivalent.ein` | 6 | m | 0 | 0 | 0 (0) | 15 | 0 | 0 | 15 | 3 | 4 | 20 | 100 | 22 | 87 | 30 | 12 | 20 |
| `examples/binary_logic/satisfiable.ein` | 6 | m | 0 | 0 | 0 (0) | 15 | 0 | 0 | 15 | 3 | 4 | 20 | 100 | 22 | 83 | 30 | 17 | 17 |
| `examples/binary_logic/synthesis.ein` | 16 | m | 14 | 0 | 0 (0) | 120 | 119 | 0 | 1 | 1 | 0 | 0.8 | 12.5 | 18 | 169 | 18 | 4 | 31 |
| `examples/binary_logic/truth_table.ein` | 6 | m | 0 | 0 | 0 (0) | 15 | 0 | 0 | 15 | 3 | 4 | 20 | 100 | 22 | 83 | 30 | 9 | 27 |
| `examples/binary_logic/unsatisfiable.ein` | 6 | m | 0 | 0 | 0 (0) | 15 | 0 | 0 | 15 | 4 | 3 | 26.7 | 100 | 22 | 84 | 28 | 13 | 13 |
| `examples/branching/01_saturate_only.ein` | 16 | m | 16 | 0 | 0 (0) | 120 | 120 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 56 | 17 | 17 | 17 |
| `examples/branching/02_one_dead_one_alive.ein` | 48 | m | 30 | 34 | 7 (18) | 1128 | 975 | 17 | 136 | 56 | 24 | 5 | 33.3 | 185 | 1136 | 193 | 2 | 208 |
| `examples/branching/03_five_hyps_one_alive.ein` | 132 | m | 130 | 2 | 1 (2) | 8646 | 8645 | 1 | 0 | 0 | 0 | 0 | 0 | 133 | 440 | 133 | 32 | 235 |
| `examples/branching/04_two_levels.ein` | 46 | m | 38 | 8 | 4 (8) | 1035 | 1007 | 4 | 24 | 16 | 0 | 1.5 | 17.4 | 71 | 236 | 71 | 4 | 141 |
| `examples/branching/05_mini_zebra.ein` | 9 | m | 6 | 0 | 0 (0) | 36 | 33 | 0 | 3 | 0 | 3 | 0 | 0 | 13 | 60 | 13 | 8 | 12 |
| `examples/branching/06_lookahead_on.ein` | 132 | m | 130 | 2 | 1 (2) | 8646 | 8645 | 1 | 0 | 0 | 0 | 0 | 0 | 133 | 440 | 133 | 8 | 258 |
| `examples/branching/07_lookahead_off.ein` | 132 | m | 130 | 2 | 1 (2) | 8646 | 8645 | 1 | 0 | 0 | 0 | 0 | 0 | 133 | 440 | 133 | 8 | 258 |
| `examples/branching/08_hypothesis_relation_whitelist.ein` | 38 | m | 12 | 58 | 9 (26) | 703 | 378 | 29 | 296 | 136 | 24 | 19.3 | 63.2 | 335 | 2676 | 335 | 2 | 335 |
| `examples/branching/09_hrule.ein` | 7 | m | 6 | 0 | 0 (0) | 21 | 21 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 34 | 8 | 4 | 12 |
| `examples/branching/10_kill_cache_on.ein` | 18 | m | 12 | 0 | 0 (0) | 153 | 138 | 0 | 15 | 0 | 12 | 0 | 0 | 34 | 192 | 58 | 4 | 27 |
| `examples/branching/11_kill_cache_off.ein` | 18 | m | 12 | 0 | 0 (0) | 153 | 138 | 0 | 15 | 0 | 12 | 0 | 0 | 34 | 192 | 58 | 4 | 27 |
| `examples/branching/12_typed_blind_solve.ein` | 51 | m | 47 | 4 | 2 (4) | 1275 | 1269 | 2 | 4 | 4 | 0 | 0.3 | 7.8 | 56 | 664 | 56 | 52 | 84 |
| `examples/branching/15_lookahead_two_step_on.ein` | 3 | m | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 2 | 0 | 66.7 | 100 | 7 | 12 | 7 | 2 | 6 |
| `examples/branching/16_lookahead_two_step_off.ein` | 3 | m | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 2 | 0 | 66.7 | 100 | 7 | 12 | 7 | 2 | 6 |
| `examples/domain_elim/b_branch.ein` | 7 | m | 6 | 0 | 0 (0) | 21 | 21 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 13 | 8 | 2 | 14 |
| `examples/domain_elim/b_only.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/alive-empty-phase1.ein` | 2 | m | 2 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 4 | 3 | 3 | 3 |
| `examples/ein-bugs/alive-set-fresh-name-declared.ein` | 6 | m | 4 | 0 | 0 (0) | 15 | 14 | 0 | 1 | 0 | 0 | 0 | 0 | 8 | 19 | 8 | 4 | 10 |
| `examples/ein-bugs/alive-set-fresh-name.ein` | 6 | m | 4 | 0 | 0 (0) | 15 | 14 | 0 | 1 | 0 | 0 | 0 | 0 | 8 | 19 | 8 | 4 | 10 |
| `examples/ein-bugs/binding-key-both.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/binding-key-int-only.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/binding-key-nested-only.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/binding-key-swapped.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/complete-records-stale.ein` | 3 | m | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 2 | 0 | 66.7 | 100 | 7 | 11 | 7 | 2 | 7 |
| `examples/ein-bugs/d3-goal-row-order.ein` | 4 | m | 0 | 2 | 0 (0) | 6 | 0 | 2 | 4 | 0 | 0 | 0 | 0 | 9 | 18 | 10 | 2 | 4 |
| `examples/ein-bugs/fork-delta-order.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 1 | 0 | 100 | 100 | 4 | 165 | 4 | 2 | 4 |
| `examples/ein-bugs/mixed-type-hypothesis.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/nested-fact-hypothesis.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/unsat-core-id-space.ein` | 5 | m | 3 | 0 | 0 (0) | 10 | 9 | 0 | 1 | 1 | 0 | 10 | 40 | 7 | 11 | 7 | 4 | 11 |
| `examples/ein-bugs/wrong-answers/activator-nested-value.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/arity3.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/assert-neq-w-x.ein` | 1 | m | 1 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 2 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/core-activator-committed.ein` | 2 | f | 1 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 6 | 4 | 2 | 4 |
| `examples/ein-bugs/wrong-answers/core-activator-given.ein` | 2 | m | 1 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 3 | 3 | 3 | 3 |
| `examples/ein-bugs/wrong-answers/core-activator-guard-only.ein` | 2 | f | 1 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 8 | 4 | 3 | 4 |
| `examples/ein-bugs/wrong-answers/core-activator-root-derived.ein` | 2 | m | 1 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 3 | 3 | 3 | 3 |
| `examples/ein-bugs/wrong-answers/count-past-2-128.ein` | 130 | m | 0 | 16770 | 1 (130) | 8385 | 0 | 8385 | 0 | 0 | 0 | 0 | 0 | 131 | 16900 | 131 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/count-past-2-64.ein` | 64 | m | 0 | 4032 | 1 (64) | 2016 | 0 | 2016 | 0 | 0 | 0 | 0 | 0 | 65 | 4096 | 65 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/einb-dump-answer.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 1 | 0 | 100 | 100 | 4 | 5 | 4 | 2 | 4 |
| `examples/ein-bugs/wrong-answers/empty-match-activator.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/empty-match-fact.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/empty-match-hrule.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/f07-fpoff.ein` | 1 | f | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/f07-rules-swapped.ein` | 1 | f | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/forall-core-elim.ein` | 2 | m | 1 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 4 | 3 | 3 | 3 |
| `examples/ein-bugs/wrong-answers/forall-core-false.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 1 | 0 | 100 | 100 | 4 | 9 | 4 | 2 | 4 |
| `examples/ein-bugs/wrong-answers/forbid-chain-no-absent.ein` | 2 | f | 1 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 10 | 4 | 3 | 4 |
| `examples/ein-bugs/wrong-answers/forced-1-two.ein` | 2 | m | 2 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 4 | 3 | 3 | 3 |
| `examples/ein-bugs/wrong-answers/forced-1.ein` | 1 | m | 1 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 2 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/forced-2-off.ein` | 1 | m | 1 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 3 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/forced-2-two.ein` | 2 | m | 2 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 6 | 3 | 3 | 3 |
| `examples/ein-bugs/wrong-answers/forced-2.ein` | 1 | m | 1 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 3 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/form-in-guard.ein` | 8 | m | 0 | 0 | 0 (0) | 28 | 0 | 0 | 28 | 0 | 0 | 0 | 0 | 37 | 64 | 256 | 2 | 9 |
| `examples/ein-bugs/wrong-answers/fresh-name-default.ein` | 6 | m | 4 | 0 | 0 (0) | 15 | 14 | 0 | 1 | 0 | 0 | 0 | 0 | 8 | 19 | 8 | 4 | 10 |
| `examples/ein-bugs/wrong-answers/fresh-name-nospawn.ein` | 2 | m | 2 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 4 | 3 | 3 | 3 |
| `examples/ein-bugs/wrong-answers/guard-before-binder.ein` | 8 | m | 0 | 0 | 0 (0) | 28 | 0 | 0 | 28 | 0 | 0 | 0 | 0 | 37 | 64 | 256 | 2 | 9 |
| `examples/ein-bugs/wrong-answers/guard-only-first.ein` | 2 | f | 1 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 10 | 4 | 3 | 4 |
| `examples/ein-bugs/wrong-answers/guard-only-second.ein` | 2 | f | 1 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 8 | 4 | 3 | 4 |
| `examples/ein-bugs/wrong-answers/h-long-relation.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/hrule-reads-absent-not.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/hrule-reads-not.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/hrule-reads-unknown.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/import-directives-as.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/import-directives-twice.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/kw-and-in-match-twin.ein` | 3 | m | 0 | 1 | 0 (0) | 3 | 0 | 1 | 2 | 0 | 0 | 0 | 0 | 6 | 9 | 6 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/kw-and-in-match.ein` | 3 | m | 0 | 1 | 0 (0) | 3 | 0 | 1 | 2 | 0 | 0 | 0 | 0 | 6 | 9 | 6 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/kw-neq-in-absent-twin.ein` | 3 | f | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 9 | 8 | 2 | 8 |
| `examples/ein-bugs/wrong-answers/kw-neq-in-absent.ein` | 3 | f | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 9 | 8 | 2 | 8 |
| `examples/ein-bugs/wrong-answers/kw-neq-in-fact-twin.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/kw-neq-in-fact.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/kw-neq-in-goal-twin.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/kw-neq-in-goal.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/kw-neq-in-match-twin.ein` | 3 | m | 0 | 1 | 0 (0) | 3 | 0 | 1 | 2 | 0 | 0 | 0 | 0 | 6 | 9 | 6 | 2 | 4 |
| `examples/ein-bugs/wrong-answers/kw-neq-in-match.ein` | 3 | m | 0 | 1 | 0 (0) | 3 | 0 | 1 | 2 | 0 | 0 | 0 | 0 | 6 | 9 | 6 | 2 | 4 |
| `examples/ein-bugs/wrong-answers/kw-or-in-hrule-twin.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/kw-or-in-hrule.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/kw-relation-fact-twin.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/kw-relation-fact.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/match-only-declared.ein` | 6 | m | 4 | 0 | 0 (0) | 15 | 14 | 0 | 1 | 0 | 0 | 0 | 0 | 8 | 16 | 8 | 4 | 10 |
| `examples/ein-bugs/wrong-answers/match-only.ein` | 6 | m | 4 | 0 | 0 (0) | 15 | 14 | 0 | 1 | 0 | 0 | 0 | 0 | 8 | 16 | 8 | 4 | 10 |
| `examples/ein-bugs/wrong-answers/negative-only-declared.ein` | 5 | m | 3 | 0 | 0 (0) | 10 | 9 | 0 | 1 | 0 | 0 | 0 | 0 | 7 | 10 | 7 | 4 | 8 |
| `examples/ein-bugs/wrong-answers/negative-only.ein` | 5 | m | 3 | 0 | 0 (0) | 10 | 9 | 0 | 1 | 0 | 0 | 0 | 0 | 7 | 10 | 7 | 4 | 8 |
| `examples/ein-bugs/wrong-answers/oblig-derived-activator-declared.ein` | 2 | f | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 4 | 4 |
| `examples/ein-bugs/wrong-answers/oblig-derived-activator.ein` | 1 | f | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 2 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein` | 6 | m | 5 | 0 | 0 (0) | 15 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 11 | 7 | 7 | 12 |
| `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein` | 6 | m | 5 | 0 | 0 (0) | 15 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 17 | 7 | 7 | 12 |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-declared.ein` | 6 | m | 5 | 0 | 0 (0) | 15 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 11 | 7 | 7 | 10 |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-two-declared.ein` | 12 | m | 7 | 0 | 0 (0) | 66 | 56 | 0 | 10 | 0 | 0 | 0 | 0 | 23 | 39 | 39 | 20 | 39 |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein` | 12 | m | 7 | 0 | 0 (0) | 66 | 56 | 0 | 10 | 0 | 0 | 0 | 0 | 23 | 61 | 39 | 20 | 39 |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name.ein` | 6 | m | 5 | 0 | 0 (0) | 15 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 7 | 17 | 7 | 7 | 10 |
| `examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein` | 6 | f | 1 | 0 | 0 (0) | 15 | 0 | 0 | 15 | 0 | 0 | 0 | 0 | 22 | 42 | 64 | 14 | 64 |
| `examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein` | 6 | f | 1 | 0 | 0 (0) | 15 | 0 | 0 | 15 | 0 | 0 | 0 | 0 | 22 | 63 | 64 | 14 | 64 |
| `examples/ein-bugs/wrong-answers/owed-seats-renamed.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 0 | 0 | 0 | 0 | 11 | 16 | 16 | 6 | 7 |
| `examples/ein-bugs/wrong-answers/owed-seats.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 0 | 0 | 0 | 0 | 11 | 16 | 16 | 6 | 7 |
| `examples/ein-bugs/wrong-answers/owed-two-step.ein` | 2 | m | 2 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 6 | 3 | 3 | 3 |
| `examples/ein-bugs/wrong-answers/pair-core-root-derived.ein` | 1 | m | 1 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 2 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/pair-core-root-given.ein` | 1 | m | 1 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 2 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/pair-core-root-negative.ein` | 1 | m | 1 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 2 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/parameter-nested-assert.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/past-limit-lines.ein` | 17 | f | 15 | 0 | 0 (0) | 136 | 0 | 0 | 136 | 0 | 0 | 0 | 0 | 154 | 591 | limit | 2 | limit |
| `examples/ein-bugs/wrong-answers/range-in-match.ein` | 3 | m | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 12 | 8 | 2 | 4 |
| `examples/ein-bugs/wrong-answers/relvar-eq-premise.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 2 | 3 |
| `examples/ein-bugs/wrong-answers/tree-dead.ein` | 2 | m | 2 | 0 | 0 (0) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 4 | 3 | 3 | 3 |
| `examples/ein-bugs/wrong-answers/uncovered-knows.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 0 | 0 | 0 | 0 | 11 | 16 | 16 | 6 | 7 |
| `examples/ein-bugs/wrong-answers/uncovered.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 0 | 0 | 0 | 0 | 11 | 16 | 16 | 6 | 7 |
| `examples/ein-bugs/wrong-answers/wild-in-absent-twin.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/wild-in-head-twin.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/wild-in-hrule-twin.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/ein-bugs/wrong-answers/wild-in-match-twin.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/features/01_not_and_absent.ein` | 35 | f | 0 | 0 | 0 (0) | 595 | 0 | 0 | 595 | 0 | 0 | 0 | 0 | 631 | 1429 | limit | 2 | limit |
| `examples/features/02_star_in_identifiers.ein` | 15 | m | 0 | 24 | 0 (0) | 105 | 0 | 24 | 81 | 0 | 20 | 0 | 0 | 97 | 708 | 1736 | 2 | 10 |
| `examples/features/03_forall.ein` | 5 | m | 0 | 1 | 0 (0) | 10 | 0 | 1 | 9 | 0 | 1 | 0 | 0 | 15 | 28 | 22 | 2 | 4 |
| `examples/features/04_open.ein` | 81 | f | 0 | 0 | 0 (0) | 3240 | 0 | 0 | 3240 | 0 | 0 | 0 | 0 | 3322 | 6561 | limit | 2 | limit |
| `examples/features/05_stdlib_domain_elim.ein` | 52 | m | 47 | 0 | 0 (0) | 1326 | 1316 | 0 | 10 | 0 | 0 | 0 | 0 | 63 | 119 | 79 | 32 | 77 |
| `examples/features/07_unstratifiable.ein` | 1 | f | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/features/11_expect_ambiguity.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 4 | 0 | 66.7 | 100 | 11 | 20 | 11 | 2 | 10 |
| `examples/features/13_mixed_solution_and_open.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 1 | 0 | 100 | 100 | 4 | 7 | 4 | 4 | 4 |
| `examples/lattice/01_subset_pruned.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 1 | 0 | 16.7 | 50 | 11 | 17 | 13 | 2 | 8 |
| `examples/lattice/02_genuine_3set_death.ein` | 3 | m | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 9 | 8 | 2 | 7 |
| `examples/lattice/03_state_hash_collision.ein` | 3 | m | 0 | 2 | 0 (0) | 3 | 0 | 2 | 1 | 0 | 0 | 0 | 0 | 5 | 7 | 7 | 2 | 3 |
| `examples/lattice/04_obligation_trigger_hole.ein` | 2 | f | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 6 | 4 | 2 | 4 |
| `examples/lattice/05_owed_seats_bounded.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 4 | 0 | 66.7 | 100 | 11 | 20 | 11 | 6 | 9 |
| `examples/lattice/06_friends_tables.ein` | 19 | f | 1 | 0 | 0 (0) | 171 | 0 | 0 | 171 | 67 | 0 | 39.2 | 94.7 | 191 | 447 | limit | 27 | limit |
| `examples/lattice/06_friends_tables_everyone_sits.ein` | 19 | m | 1 | 0 | 0 (0) | 171 | 18 | 0 | 153 | 67 | 0 | 39.2 | 94.7 | 173 | 393 | 345 | 207 | 213 |
| `examples/lattice/07_two_pairs_two_tables.ein` | 16 | m | 0 | 0 | 0 (0) | 120 | 0 | 0 | 120 | 64 | 0 | 53.3 | 100 | 137 | 320 | 177 | 44 | 101 |
| `examples/lattice/08_absent_witness_hole.ein` | 3 | f | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 11 | 8 | 4 | 8 |
| `examples/lattice/09_activator_hole.ein` | 2 | f | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 6 | 4 | 2 | 4 |
| `examples/lattice/10_fifo_gap.ein` | 2 | f | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 4 | 7 | 4 | 2 | 4 |
| `examples/lattice/11_fifo_gap_static.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/lattice/12_activator_chain_hole.ein` | 2 | f | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 8 | 4 | 2 | 4 |
| `examples/lattice/synth/local_not_global.ein` | 2 | f | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 4 | 4 | 4 | 4 |
| `examples/lattice/synth/maximal_not_top.ein` | 3 | f | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 9 | 8 | 5 | 8 |
| `examples/lattice/synth/merged_states.ein` | 5 | m | 0 | 2 | 0 (0) | 10 | 0 | 2 | 8 | 1 | 0 | 10 | 40 | 14 | 30 | 18 | 3 | 9 |
| `examples/lattice/synth/one_model_per_component.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 6 | 0 | 100 | 100 | 11 | 22 | 11 | 4 | 11 |
| `examples/lattice/synth/trivial_line.ein` | 5 | m | 0 | 0 | 0 (0) | 10 | 0 | 0 | 10 | 0 | 0 | 0 | 0 | 16 | 25 | 32 | 3 | 6 |
| `examples/saturation/hypothesis-contradiction/coloc-disproved.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/saturation/hypothesis-contradiction/next-to-disproved.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/saturation/hypothesis-contradiction/right-of-disproved.ein` | 1 | m | 0 | 0 | 0 (0) | 0 | 0 | 0 | 0 | 0 | 0 |  | 0 | 2 | 1 | 2 | 2 | 2 |
| `examples/saturation/implies/org-chart.ein` | 2 | m | 0 | 1 | 0 (0) | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 3 | 3 | 2 | 2 |
| `examples/saturation/implies/parent-to-ancestor.ein` | 2 | m | 0 | 1 | 0 (0) | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 3 | 3 | 2 | 3 |
| `examples/saturation/implies/right-then-next.ein` | 2 | m | 0 | 1 | 0 (0) | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 3 | 3 | 2 | 3 |
| `examples/saturation/square-bwd/floors.ein` | 20 | m | 0 | 3 | 0 (0) | 190 | 0 | 3 | 187 | 0 | 48 | 0 | 0 | 208 | 559 | > 85574 | 2 | 11 |
| `examples/saturation/square-bwd/houses.ein` | 20 | m | 0 | 3 | 0 (0) | 190 | 0 | 3 | 187 | 0 | 48 | 0 | 0 | 208 | 559 | > 85574 | 2 | 20 |
| `examples/saturation/square-bwd/meetings.ein` | 20 | m | 0 | 3 | 0 (0) | 190 | 0 | 3 | 187 | 0 | 48 | 0 | 0 | 208 | 559 | > 85574 | 2 | 11 |
| `examples/saturation/square-fwd/floors.ein` | 20 | m | 0 | 3 | 0 (0) | 190 | 0 | 3 | 187 | 0 | 48 | 0 | 0 | 208 | 559 | > 85574 | 2 | 11 |
| `examples/saturation/square-fwd/houses.ein` | 20 | m | 0 | 3 | 0 (0) | 190 | 0 | 3 | 187 | 0 | 48 | 0 | 0 | 208 | 559 | > 85574 | 2 | 21 |
| `examples/saturation/square-fwd/meetings.ein` | 20 | m | 0 | 3 | 0 (0) | 190 | 0 | 3 | 187 | 0 | 48 | 0 | 0 | 208 | 559 | > 85574 | 2 | 11 |
| `examples/saturation/square-unique/corner-house.ein` | 118 | f | 0 | 4 | 0 (0) | 6903 | 0 | 0 | 6903 | 0 | 86 | 0 | 0 | 7022 | 14574 | limit | 2 | limit |
| `examples/saturation/square-unique/cul-de-sac.ein` | 114 | f | 0 | 8 | 0 (0) | 6441 | 0 | 0 | 6441 | 0 | 49 | 0 | 0 | 6556 | 14228 | limit | 2 | limit |
| `examples/saturation/square-unique/terminus.ein` | 153 | f | 0 | 14 | 0 (0) | 11628 | 0 | 0 | 11628 | 0 | 79 | 0 | 0 | 11782 | 25968 | limit | 2 | limit |
| `examples/saturation/transitive/colocation-chain.ein` | 3 | m | 0 | 2 | 0 (0) | 3 | 0 | 2 | 1 | 0 | 1 | 0 | 0 | 5 | 8 | 7 | 2 | 3 |
| `examples/saturation/transitive/mealtimes.ein` | 3 | m | 0 | 2 | 0 (0) | 3 | 0 | 2 | 1 | 0 | 1 | 0 | 0 | 5 | 8 | 7 | 2 | 2 |
| `examples/saturation/transitive/taxonomy.ein` | 3 | m | 0 | 2 | 0 (0) | 3 | 0 | 2 | 1 | 0 | 1 | 0 | 0 | 5 | 8 | 7 | 2 | 2 |
| `examples/saturation/type-exclusivity/colors.ein` | 8 | m | 0 | 0 | 0 (0) | 28 | 0 | 0 | 28 | 0 | 2 | 0 | 0 | 37 | 68 | 173 | 2 | 54 |
| `examples/saturation/type-exclusivity/nationalities.ein` | 8 | m | 0 | 0 | 0 (0) | 28 | 0 | 0 | 28 | 0 | 2 | 0 | 0 | 37 | 68 | 173 | 2 | 56 |
| `examples/saturation/type-exclusivity/pets.ein` | 15 | m | 0 | 0 | 0 (0) | 105 | 0 | 0 | 105 | 0 | 6 | 0 | 0 | 121 | 237 | 11351 | 2 | 430 |
| `examples/syntax/constraint-scopes.ein` | 12 | m | 0 | 0 | 0 (0) | 66 | 0 | 0 | 66 | 0 | 0 | 0 | 0 | 79 | 144 | 4096 | 2 | 13 |
| `examples/syntax/equality.ein` | 12 | m | 0 | 0 | 0 (0) | 66 | 0 | 0 | 66 | 0 | 0 | 0 | 0 | 79 | 144 | 4096 | 2 | 13 |
| `examples/syntax/unary-and-nary.ein` | 3 | m | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 9 | 8 | 2 | 4 |
| `examples/word_problems/boxes12.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 1 | 0 | 100 | 100 | 4 | 5 | 4 | 3 | 4 |
| `examples/word_problems/boxes13.ein` | 2 | m | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 1 | 0 | 100 | 100 | 4 | 37 | 4 | 3 | 4 |
| `examples/zebra.ein` | 56 | m | 18 | 237 | 9 (30) | 1540 | 837 | 190 | 513 | 414 | 81 | 26.9 | 67.9 | 570 | 181088 | 445 | 49 | 49 |
| `examples/zebra2-hints.ein` | 36 | m | 18 | 158 | 3 (17) | 630 | 477 | 96 | 57 | 55 | 0 | 8.7 | 44.4 | 94 | 12367 | 78 | 4 | 123 |
| `examples/zebra2-minus-15-obligations.ein` | 96 | m | 0 | 149 | 32 (67) | 4560 | 0 | 111 | 4449 | 1428 | 1614 | 31.3 | 100 | 4546 | 475193 | > 8391 | 39 | 2346 |
| `examples/zebra2-minus-15.ein` | 96 | m | 0 | 149 | 32 (67) | 4560 | 0 | 111 | 4449 | 1428 | 1614 | 31.3 | 100 | 4546 | 475193 | > 8391 | 39 | 2346 |
| `examples/zebra2-obligations.ein` | 56 | m | 21 | 234 | 9 (30) | 1540 | 945 | 187 | 408 | 327 | 66 | 21.2 | 62.5 | 465 | 68385 | 374 | 45 | 45 |
| `examples/zebra2.ein` | 56 | m | 21 | 234 | 9 (30) | 1540 | 945 | 187 | 408 | 327 | 66 | 21.2 | 62.5 | 465 | 68385 | 374 | 45 | 45 |
| `tests/region/01_states.ein` | 3 | m | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 9 | 8 | 2 | 7 |
| `tests/region/02_merged_single.ein` | 5 | m | 0 | 2 | 0 (0) | 10 | 0 | 2 | 8 | 1 | 0 | 10 | 40 | 14 | 30 | 18 | 3 | 9 |
| `tests/region/03_merged_or.ein` | 3 | m | 0 | 2 | 0 (0) | 3 | 0 | 2 | 1 | 0 | 1 | 0 | 0 | 5 | 8 | 7 | 2 | 3 |
| `tests/region/07_cube.ein` | 2 | f | 0 | 0 | 0 (0) | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 4 | 6 | 4 | 2 | 4 |
| `tests/region/08_query_classes.ein` | 3 | m | 0 | 0 | 0 (0) | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 7 | 9 | 8 | 2 | 7 |
| `tests/region/09_query_over_maximal.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 4 | 0 | 66.7 | 100 | 11 | 20 | 11 | 2 | 10 |
| `tests/stdlib/bijection/06_blind_enumeration.ein` | 16 | m | 16 | 0 | 0 (0) | 120 | 120 | 0 | 0 | 0 | 0 | 0 | 0 | 17 | 104 | 17 | 17 | 17 |
| `tests/stdlib/bool/inputs/01_in_drives.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 2 | 4 | 33.3 | 100 | 11 | 38 | 11 | 5 | 11 |
| `tests/stdlib/bool/inputs/02_required.ein` | 4 | m | 0 | 0 | 0 (0) | 6 | 0 | 0 | 6 | 5 | 1 | 83.3 | 100 | 11 | 45 | 11 | 8 | 9 |
