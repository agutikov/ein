# S1f.10.13 — The price list

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** ran 2026-11-16: [§ Record](#record-2026-11-16). Planned 2026-11-15, at the re-plan. **Re-scoped 2026-11-15 on the user's correction**
([README § The two ideas](README.md#the-two-ideas)): track R is required and depends on track S's results. So
this stage runs after track S's stages and not beside them; it prices each object of the structure (the theory's
exclusions, root's exclusions, the groups, the owed groups and grids, the coupling) and not the excluded pairs
alone; and a *no* is a verdict on one way of spending for one question, which sends the track to the next way and
does not close it. Where the text below says otherwise, this line stands.
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md), [S1f.10.10](s1f.10.10_the_theorys_share.md),
[S1f.10.2](s1f.10.2_groups.md), [S1f.10.4](s1f.10.4_bounded_groups.md) and [S1f.10.11](s1f.10.11_coupling.md).
**Read by:** [S1f.10.3](s1f.10.3_the_restricted_join.md), [S1f.10.5](s1f.10.5_ordering.md) and
[S1f.10.14](s1f.10.14_exactly_one_spent.md), none of which builds anything this stage calls a *no*; and the
phase's stop, [S1f.10.12](s1f.10.12_the_structure_shown.md), where the user sees the table.

## Context

The user's second idea is a question with a *maybe* in it: *maybe hyp structure investigation is cheap and can
improve search of R*. This stage answers both halves with numbers before any route changes. A *no* is a
result about one way of spending the structure for one question; the track is required and goes on to the next
way.

**What the re-plan's probe found**, 2026-11-15, the release binary at `dbedc535`, on AC
([README § What was measured at the re-plan](README.md#what-was-measured-at-the-re-plan)): between 0 % and 95 %
of a run's saturations are deaths that hold a pair of one relation agreeing in exactly one argument. The probe
was syntactic. Its script, kept so the table can be re-taken until this stage replaces it:

```python
# probe.py NODES_JSON…  — over `ein solve FILE [MODE] --dump-lattice DIR`'s DIR/nodes.json
import json, sys, itertools, collections
def parse(f): return tuple(f.strip("()").split())
def structural(a, b):
    A, B = parse(a), parse(b)
    if A[0] != B[0] or len(A) != 3 or len(B) != 3: return False
    return (A[1] == B[1]) != (A[2] == B[2])
for path in sys.argv[1:]:
    d = json.load(open(path)); nodes = d["nodes"]
    alive = [(frozenset(n["commitment"]), frozenset(n.get("closure") or n["commitment"]))
             for n in nodes if n["kind"] != "dead"]
    dead = [n for n in nodes if n["kind"] == "dead"]; hit = 0
    for n in dead:
        C = frozenset(n["commitment"]); cl = set(C)
        for c, k in alive:
            if c < C: cl |= k
        hit += any(structural(a, b) for a, b in itertools.combinations(sorted(cl), 2))
    print(d["source"], d["route"], len(nodes), collections.Counter(n["kind"] for n in nodes), hit)
```

## What is priced

**The work, per program and per question.** Every corpus program that searches; the questions `ein solve`
asks: describe, `--count`, `--single`, `-n 1`, `--minimal`, `--maximal`. Per cell: saturations and facts beyond
root, and the saturations split four ways:

| part | what would remove it |
|---|---|
| dead, and the commitment with the closures under it holds an **excluded pair** of S1f.10.1, split in two: a pair **of the theory**, two members of one group ([S1f.10.10](s1f.10.10_the_theorys_share.md), [S1f.10.2](s1f.10.2_groups.md)), and a pair **of the puzzle** | *at most one*, [S1f.10.3](s1f.10.3_the_restricted_join.md); the theory's pairs are known for every puzzle over one ontology, the puzzle's are paid per program |
| dead otherwise, split in two: inside one hyperedge of [S1f.10.11](s1f.10.11_coupling.md)'s coupling, and not | an order that keeps a coupling's groups together, [S1f.10.5](s1f.10.5_ordering.md); the rest nothing in this phase |
| alive and owing, split in two: owing an instance whose alternatives are an **owed group** ([S1f.10.4](s1f.10.4_bounded_groups.md)), and owing otherwise | *exactly one*, [S1f.10.14](s1f.10.14_exactly_one_spent.md), or an order that reaches a leaf sooner, [S1f.10.5](s1f.10.5_ordering.md) |
| in R | nothing: it is the answer |

**The cost of having the pairs**, by each way of getting them:

| source | what it costs | when it is paid |
|---|---|---|
| **the pair layer whole** | S1f.10.1's saturations for the program | before the first node of any question |
| **the pair layer by need** | one saturation per pair first asked about: the pair `{a, b}` is saturated when a route is about to extend a state holding `a` by `b` | during the run; never more than the layer whole |
| **the pair of a death** | when a set dies, the pair in it that is dead, found from the death's support with no saturation, or by saturating the pairs of the last candidate added with the members of the state | at each death |
| **the small KB** | [S1f.10.10](s1f.10.10_the_theorys_share.md)'s check, a saturation of a smaller KB per pair | before the run, and reusable across puzzles over one ontology |

**The cost of every other object of the structure**, each as its stage's record counts it and re-taken here on
one binary, per program: the cover (the small KB's pair layer, then the cliques, which saturate nothing), the
owed groups and grids (L5's table and its witnesses beside the cover), and the coupling (the reading
S1f.10.11 took). A way of spending is priced with the cost of **every object it reads**, not of the pairs
alone.

**Why not a saturation rule instead.**
[`domain_elim_vs_hypothesis.md`](../../../docs/kernel/inference/domain_elim_vs_hypothesis.md) measured the
same structure written as rules (`domain-elimination`, `no-room-left`). The stage restates that comparison in
today's terms: what the rules derive at a node is already in every number above, so what is priced here is only
what remains after them.

## The rule for *go*, written before the table is taken

For a question and a source:

- **go** when, over the corpus programs that search, the total work of that question with the source's cost
  added is lower, **and no program's is higher by more than one tenth**;
- **no** otherwise, with the programs that lose named.

The second clause is the user's ruling that each mode does the least work its question needs: a 45-saturation
`--single` may not be handed a 1 500-saturation preamble because describe gains from it. A source may be *go*
for one question and *no* for another, and then a route takes it only for the first. The tenth is this stage's
number and the user's to revise at the stop.

## Tasks

1. **T1f.10.13.1 — the work table**, by an example in the crate beside S1f.10.1's, reading each route's
   visited nodes in process (what `--dump-lattice` writes) against S1f.10.1's pairs. Within the default budget;
   a cell the budget stops is recorded as stopped, with what it had spent.
2. **T1f.10.13.2 — the cost table**, each source simulated on the same runs: how many saturations it would
   have added and how many removed, counted from the visited nodes, with no route changed.
3. **T1f.10.13.3 — the verdicts**, by the rule above, per question and source; and the two shares the other
   stages need: the *alive and owing* share per question (S1f.10.14), and for `--single` and `-n 1` the
   saturations a best order would have needed, read off the leaves found (S1f.10.5's ceiling).
4. **T1f.10.13.4 — the record**, in this file, with commands and the date, and the page of it
   S1f.10.12's bundle shows.

## Acceptance

- The rule for *go* above is committed before T1's table is.
- Every corpus program that searches has its cells, or is named as stopped.
- Each source has a verdict per question, and each *no* names its losers.
- **A *no* names the way of spending and the question it is about, and closes nothing else.** Track R is
  required: where a way does not pay, the record says which object's cost sank it and on which programs, and
  what the next way to try is, for S1f.10.3, S1f.10.5 and S1f.10.14 to start from. Track S is untouched.
- No answer, route or surface changes. The stage measures.

## What it must not do

- **Not change a route to measure it.** The simulation reads visited nodes.
- **Not count a saved saturation without the cost that bought it.**
- **Not run a cell past the default budget.**

## Record (2026-11-16)

**Ran to its end.** No route, no answer and no surface changed. Every corpus program that searches has its six
cells: **188 programs, 1 128 cells, none an error**; 16 describes, 16 `--count`s, 3 `--minimal`s and 7 `--maximal`s
were stopped, each priced on what it had visited. The working tree over `9f80ea3e`, on AC, one P-core; every
number below is a count unless it is called a wall.

**The verdicts in one paragraph.** By the rule above, read in saturations and in facts beyond root: **the pair
layer whole is *no* for all six questions**, and so is every way that reads the small KB's layer, which is the
theory's pairs paid here, the cover, the owed groups and the coupling. Each is sunk by a layer that costs more
than the question: 161 of the 188 `--single` cells are under ten saturations. **The pair layer asked by need is
*go* for `--minimal` alone, and asked after a death for describe, `--count` and `--minimal`.** Two ways add no
saturation and are *go* for all six: **reading what a state's own fixpoint denies** (`denials`, found here and
priced; `zebra2.ein --single` 45 to 7, `zebra2-minus-15.ein --maximal` 2 346 to 100, and nothing on SEND), and
**the core of a death**, whose cost is a search and is in no unit of the work. The theory's pairs with their cost
left out are a bound that is *go* everywhere, so the question they leave is where they can be had for nothing.
Nothing here closes a door: [§ What each stage starts from](#what-each-stage-starts-from).

### What was built

- **The example**, [`ein-infer/examples/price_list.rs`](../../../ein.rs/crates/ein-infer/examples/price_list.rs),
  beside `pair_layer`, with its reading in
  [`examples/shared/prices.rs`](../../../ein.rs/crates/ein-infer/examples/shared/prices.rs), which the test
  includes. A row of [`utils/README.md`](../../../utils/README.md)'s measurement set.
- **One entry in the owner**: `structure::pair_layer_with(root, terms, ast, opts, events)`, `pair_layer` with the
  space's nodes narrated into a caller's `Events`, so that root's layer can be priced node by node. `with_space`
  is cut in two for it and every other caller passes `Events::off()` as before. A sentence of `structure.rs`'s
  row in [`architecture.md`](../../../docs/ein/ein.rs/architecture.md).
- **The test**, [`ein-infer/tests/price_list.rs`](../../../ein.rs/crates/ein-infer/tests/price_list.rs), two, per
  commit, 5.5 s at 16 workers, on `ein_corpus::pool::map`, with no slow half.
- **Two findings**, K-12 and K-13 of [`findings.md`](findings.md).
- **The artefacts**, under the untracked `build/structure/`, by the one command below:
  `prices/prices.jsonl` (a line a program), `prices/verdicts.json`, `prices/prices.md` (every table of this
  record over every program, 748 KB), `prices/verdicts.md` and `prices/prices.env`; and per program
  `<program>/prices.json`, `<program>/prices.md` and `<program>/prices/<question>.nodes.jsonl`, every node the
  run visited with its commitment, what it is, its facts, its part and its core. They stand beside the other
  stages' files and overwrite none. The `jq` programs and the script this record quotes are in
  `build/structure/prices/` too and are verbatim under [§ The commands](#the-commands).

### How a route's nodes are read, and what holds that no route changed

**Through the watch `--dump-lattice` reads**, `ein_infer::events::Watch`: every node a route computes is handed
to it on the committing thread, in visit order, with its commitment, what it is, its closure, its facts beyond
root and, where the run takes cores, its core. It was there already; nothing was added to a route, to the node
function or to `explore::Space`. The state route, the obligation tree, route 1 and the cube all narrate through
it (`states.rs`, `explore.rs`'s `Space::eval`, `downset.rs`, `region.rs`).

**Each cell is asked twice**, the program loaded again each time: plain, with no watch and the options `ein
solve` gives the question (cores for the read-out alone; `N` not taken under `--count`), for its work and its
wall; and watched, with a core taken at every death as `--dump-lattice` takes them. What holds the two to each
other:

| | |
|---|---|
| the watched run answers as the plain one: saturations, facts beyond root, route, exhausted, verdict, stopped or not, the states listed | **1 128 of 1 128 cells** (`same_work_watched` in every cell's row) |
| the nodes handed to the watch are the run's saturations, and their facts sum to its facts | the same 1 128 |
| a text in a commitment or a closure that is no candidate's | 0 |
| the cells the census of [S1f.10.1](s1f.10.1_exclusion_census.md) took, a describe's, a `--single`'s and a `--maximal`'s, 564 of them: saturations and facts | **564 of 564 the same** (`against_census.jq`) |
| the corpus run taken twice: every count of every cell | one digest ([§ The commands](#the-commands)) |
| the split's parts sum to the cell's saturations and to its facts | by construction, and `a_routes_nodes_are_read_and_no_route_is_changed` |
| `explore`, `region`, `claim_routes` and `ein-cli`'s `lever_invariance`, no expectation edited | green ([§ Verification](#verification)) |

`structure.rs`'s change is one new public function and a private helper cut in two; no route calls either.

### What the split had to settle

Each is stated in `shared/prices.rs`'s header.

- **The closures under a commitment.** Both routes extend a state by one candidate and saturate `cl(S) ∪ {b}`
  (`tree.rs`' `extend`, `states.rs`), so a commitment mostly holds its parent's closure already. `K(C)` is `C`
  with the closures of the alive nodes visited **before** it whose commitment lies inside it: what the run
  could have known when it reached `C`. Nothing visited after `C` is read for it, where the re-plan's probe read
  every alive node.
- **A pair of the theory** is two members of one group, which is a pair of `Cover::excluded`, over all of H, a
  member dead alone at root among them. **A pair of the puzzle** is a pair root excludes that the theory does
  not.
- **A part the stage file did not have, `dead_single`**: a dead node whose `K` holds a candidate dead alone and
  no excluded pair. A candidate's own dead node is one, and so is a larger set a route built before it had met
  the single. It is 66 507 of describe's saturations and 67 522 of `--maximal`'s, nearly all of them the cubes
  of flagged programs past the limit.
- **Dead otherwise, inside one edge**: the death's *core* lies inside one edge of the coupling that a conflict or
  a derivation gave, under the reading named (the commitment, where no core names a candidate). A core is the
  facts the death rests on, stated ones and committed ones; its candidates are what is read. Under the whole
  node's reading **every one of them is inside an edge, on every question**, which is S1f.10.11's T3 seen from
  the routes; the edges one application back hold a part, counted beside.
- **Which instances an alive node owes** is read off its closure: a licensed instance is owed exactly when none
  of its alternatives is in the closure, the equivalence `tree.rs`' `check_owed` refuses a run over. No node
  owes fewer instances than that reading gives it (`owed_read_off_closure_exceeds`, 0 over the corpus).
- **A flagged program** has its cells and its split, and no way is applied to it: no dead set says anything of
  its supersets there ([README § The domain](README.md#the-domain)), a route would read no structure and pay
  for none.

### The commands

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example price_list
mkdir -p build/structure/prices
utils/bench_env.sh --require-ac ein.rs/target/release/examples/price_list \
    > build/structure/prices/verdicts.md 2> build/structure/prices/prices.env
```

2026-11-16, from 01:17, 12 min: the working tree over `9f80ea3e`, the release example, one P-core (`cpu4`), on
AC, governor `powersave`, load average 1.25 at the start, the default budget of 2^20 facts
(`build/structure/prices/prices.env` has the fingerprint and a line a program). **Taken twice**, the first time
before the `denials` way was written: with that way and the walls taken out, the two runs' `prices.jsonl` are
one digest,

```sh
F='del(.denials_read) | .structure |= (del(.wall) | del(.root_layer.wall))
   | .cells |= map_values(del(.wall) | del(.wall_watched) | del(.ways.denials)
       | .detail |= with_entries(select(.key | test("denied") | not)))'
jq -c -S "$F" build/structure/prices/prices.jsonl | md5sum     # 9c307cdb07076cd9146052ac0dbf36a9, both runs
```

so the counts are the programs' and the walls alone are this machine's. The whole run's walls: the structure
read once a program, 24.1 s over the 188, 19.2 s of it root's layers read a second time node by node; the six
questions asked plain, 176.8 s; asked again watched, 442.0 s.

### T1: the work, and its split

**Per question, over the 188** (`prices.md` § The work, per question). Saturations a part, with its share of
the question's.

| question | stopped | saturations | facts beyond root | in R | owing, an owed group's instance | owing otherwise | dead, a pair of the theory | dead, a pair of the puzzle | dead, a candidate dead alone | dead otherwise, inside one edge | dead otherwise, in none |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| describe | 16 | 1 350 675 | 11 563 253 | 863 642 (63.9 %) | 342 854 (25.4 %) | 469 | 68 766 (5.1 %) | 1 014 | 66 507 (4.9 %) | 7 423 | 0 |
| `--count` | 16 | 1 350 675 | 11 563 253 | the same in every cell: describe's route, `N` not taken | | | | | | | |
| `--single` | 0 | 19 677 | 292 755 | 351 (1.8 %) | 2 038 (10.4 %) | 155 | 9 639 (49.0 %) | 128 | 728 (3.7 %) | 6 638 (33.7 %) | 0 |
| `-n 1` | 0 | 17 198 | 227 637 | 186 (1.1 %) | 2 036 (11.8 %) | 126 | 8 170 (47.5 %) | 121 | 397 (2.3 %) | 6 162 (35.8 %) | 0 |
| `--minimal` | 3 | 231 776 | 3 405 717 | 3 523 (1.5 %) | 199 300 (86.0 %) | 206 | 13 873 (6.0 %) | 1 048 | 658 | 13 168 (5.7 %) | 0 |
| `--maximal` | 7 | 483 283 | 3 520 830 | 325 857 (67.4 %) | 2 082 | 442 | 79 435 (16.4 %) | 482 | 67 522 (14.0 %) | 7 463 | 0 |

**The totals are a few programs'.** Describe's 16 stopped cells are 1 321 919 of its 1 350 675 saturations: the
seven flagged programs past `--region-limit` (64 655 to 65 536 each), the six `saturation/square-*` demos
(85 574 each), I-SEND (336 101) and the two `zebra2-minus-15` programs (8 391 each). So the same split over
**the monotone programs whose cell no budget stopped** (`shares.jq`), which is where a way of spending can act:

| question | cells | saturations | in R | owing, an owed group's | owing otherwise | dead, the theory's pair | dead, the puzzle's pair | dead, a single | dead otherwise |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| describe, `--count` | 150 | 28 540 | 23 597 | 1 465 | 160 | 985 | 422 | 836 | 1 075 |
| `--single` | 159 | 19 557 | 293 | 2 038 | 115 | 9 634 | 128 | 711 | 6 638 |
| `-n 1` | 159 | 17 155 | 157 | 2 036 | 114 | 8 170 | 121 | 395 | 6 162 |
| `--minimal` | 156 | 4 745 | 497 | 1 465 | 160 | 1 109 | 310 | 152 | 1 052 |
| `--maximal` | 159 | 27 475 | 986 | 2 082 | 133 | 14 476 | 482 | 1 853 | 7 463 |

**And the cells are small.** Of the 188 cells of a question, those under 10 saturations, of 10 to 99, and of 100
or more: describe 112, 38, 38; `--single` 161, 23, 4; `-n 1` 173, 11, 4; `--minimal` 155, 22, 11; `--maximal`
116, 50, 22. The rule's second clause is decided there: one saturation added to a cell of nine is more than a
tenth.

**Dead otherwise, by the reading.** Every such death's core lies inside one edge of the whole node's reading.
Inside an edge **one application back**: 1 831 of describe's 7 423, 1 066 of `--single`'s 6 638, 1 057 of
`-n 1`'s 6 162, 1 812 of `--minimal`'s 13 168, 1 856 of `--maximal`'s 7 463. The same test on the commitment in
place of the core: 7 423, 1 102, 1 084, 7 626 and 1 773 under the node's reading, since a tree's commitment
holds its parent's closure and a core does not.

**The instances** (`jq -r -f build/structure/prices/instances.jq build/structure/prices/prices.jsonl`). The last
column is the dead-otherwise deaths whose core is inside an edge one application back; walls in ms, plain and
watched.

| | question | route | saturations | facts | | in R | owing, an owed group's | owing otherwise | dead: theory | dead: puzzle | dead: single | dead otherwise | … one step | wall | watched |
|---|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **I-Z1** | describe | states | 445 | 106 829 | | 81 | 139 | 0 | 130 | 77 | 18 | 0 | 0 | 1 292 | 1 520 |
| | `--single` | tree | 49 | 17 620 | Solution | 1 | 4 | 0 | 40 | 2 | 2 | 0 | 0 | 220 | 263 |
| | `-n 1` | tree | 6 | 721 | | 1 | 4 | 0 | 1 | 0 | 0 | 0 | 0 | 12 | 13 |
| | `--minimal` | tree | 451 | 109 187 | | 81 | 139 | 0 | 132 | 79 | 20 | 0 | 0 | 1 321 | 1 563 |
| | `--maximal` | tree | 49 | 17 620 | | 1 | 4 | 0 | 40 | 2 | 2 | 0 | 0 | 219 | 263 |
| **I-Z2** | describe | states | 374 | 42 705 | | 81 | 103 | 0 | 110 | 54 | 21 | 5 | 1 | 699 | 4 309 |
| | `--single` | tree | 45 | 7 582 | Solution | 1 | 3 | 0 | 38 | 1 | 2 | 0 | 0 | 123 | 777 |
| | `-n 1` | tree | 5 | 302 | | 1 | 3 | 0 | 0 | 0 | 1 | 0 | 0 | 9 | 23 |
| | `--minimal` | tree | 377 | 43 251 | | 81 | 103 | 0 | 110 | 55 | 23 | 5 | 1 | 709 | 4 351 |
| | `--maximal` | tree | 45 | 7 582 | | 1 | 3 | 0 | 38 | 1 | 2 | 0 | 0 | 122 | 777 |
| **I-Z2M** | describe | states | 8 391 | 796 678 | **stopped** | 865 | 5 530 | 0 | 1 106 | 296 | 0 | 594 | 77 | 11 227 | 36 826 |
| | `--single` | tree | 39 | 3 843 | Ambiguity | 2 | 26 | 0 | 0 | 10 | 0 | 1 | 0 | 51 | 199 |
| | `-n 1` | tree | 36 | 3 466 | | 1 | 25 | 0 | 0 | 9 | 0 | 1 | 0 | 47 | 180 |
| | `--minimal` | tree | 10 846 | 1 047 055 | **stopped** | 1 481 | 6 877 | 0 | 1 412 | 369 | 0 | 707 | 77 | 14 536 | 47 628 |
| | `--maximal` | tree | 2 346 | 509 140 | | 32 | 48 | 0 | 2 234 | 20 | 0 | 12 | 0 | 6 562 | 35 524 |
| **I-B06** | describe | states | 133 | 440 | | 3 | 0 | 0 | 0 | 0 | 130 | 0 | 0 | 2 | 4 |
| | `--single` | states | 8 | 34 | Ambiguity | 2 | 0 | 0 | 0 | 0 | 6 | 0 | 0 | 0 | 1 |
| | `-n 1`, `--minimal` | states | 1 | 0 | | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| | `--maximal` | states | 258 | 1 158 | | 2 | 0 | 0 | 0 | 0 | 256 | 0 | 0 | 4 | 7 |
| **I-L02** | describe | states | 8 | 13 | | 7 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0 |
| | `--single` | states | 2 | 1 | Ambiguity | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| | `-n 1`, `--minimal` | states | 1 | 0 | | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| | `--maximal` | states | 7 | 12 | | 6 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0 |
| **I-SEND** | describe | states | 336 101 | 1 422 943 | **stopped** | 0 | 330 329 | 0 | 610 | 0 | 2 | 5 160 | 648 | 14 217 | 14 786 |
| | `--single` | tree | 17 747 | 222 924 | Solution | 1 | 1 772 | 0 | 9 350 | 0 | 440 | 6 184 | 648 | 865 | 1 178 |
| | `-n 1` | tree | 15 854 | 192 556 | | 1 | 1 772 | 0 | 8 017 | 0 | 338 | 5 726 | 648 | 758 | 1 023 |
| | `--minimal` | tree | 205 166 | 1 048 551 | **stopped** | 1 | 184 081 | 0 | 9 940 | 0 | 442 | 10 702 | 652 | 8 048 | 8 537 |
| | `--maximal` | tree | 17 754 | 223 050 | | 1 | 1 772 | 0 | 9 357 | 0 | 440 | 6 184 | 648 | 866 | 1 185 |

A `--count`'s row is its describe's. What they say:

- **The re-plan's probe was right in every cell it took**, and its syntactic pair is the theory's:
  `zebra2.ein` describe 110 of 190 deaths and `--single` 38 of 41, `zebra2-minus-15.ein` describe 1 106 of 1 996,
  `--single` none of 11, `--maximal` 2 234 of 2 266, SEND `--single` 9 350 of 15 974.
- **I-Z2M `--maximal` is the pair's case**: 2 234 of its 2 346 saturations are sets of 24 candidates that hold a
  pair of the theory.
- **I-SEND is three programs in one.** Under `--single`, 9 350 deaths hold a letter's two digits or a digit's
  two letters, 6 184 are conflicts of three candidates or more, a column's, and 1 772 nodes are alive and
  owing. Under describe 330 329 of 336 101 are alive and owing, and no exclusion touches them.
- **I-B06 is the single's case**: its 130 candidates dead alone are 256 of `--maximal`'s 258 saturations, each
  met twice.
- **I-L02**: one death, the triple, inside the coupling's one edge; nothing for a pair.

### T2: what each object costs

Re-taken here on one binary, per program in `prices.md` § Every program: what each object costs; every figure
is the earlier record's.

| object | read by | saturations, the 188 | facts beyond root | when |
|---|---|---:|---:|---|
| **root's pair layer, whole** | [S1f.10.1](s1f.10.1_exclusion_census.md) | 49 015 | 1 389 866 | before the first node |
| … root's node and the singles | | 2 875 | 52 903 | |
| … the pairs L1 and L3 leave | | 46 140 | 1 336 963 | |
| **the small KB's pair layer** | [S1f.10.10](s1f.10.10_the_theorys_share.md) | 82 589 | 513 855, beyond its own root | before the run; the same for every puzzle of one ontology |
| **the cover** | [S1f.10.2](s1f.10.2_groups.md) | 0 beyond the small KB's layer and root's singles | 0 | the cliques saturate nothing |
| **what root owes, the owed groups and the grids** | [S1f.10.4](s1f.10.4_bounded_groups.md) | 0 beyond the cover | 0 | two passes of root's matcher |
| **the coupling**, the pairs' reading | [S1f.10.11](s1f.10.11_coupling.md) | 0 beyond both layers | 0 | |
| **the coupling**, the node's reading, direct, one step back or whole | S1f.10.11 | **173**, one a program with a candidate alive alone | 21 091 | one saturation whichever of the three; the edges one step back are read off the same node |

| | n | root's layer | facts | … singles | … pairs | the small KB's layer | facts | groups | owed groups | grids | edges | … one step back | components | the node: facts | applications | the structure's wall, ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| I-Z1 | 56 | 570 | 181 088 | 57 | 513 | 1 597 | 67 244 | 36 | 18 | 5 | 386 | 424 | 1 | 393 | 5 307 | 2 749 |
| I-Z2 | 56 | 465 | 68 385 | 57 | 408 | 1 597 | 32 086 | 36 | 36 | 5 | 393 | 495 | 1 | 182 | 2 509 | 1 462 |
| I-Z2M | 96 | 4 546 | 475 193 | 97 | 4 449 | 4 657 | 92 244 | 46 | 46 | 5 | 355 | 695 | 1 | 222 | 2 564 | 7 457 |
| I-B06 | 132 | 133 | 440 | 133 | 0 | 8 758 | 24 508 | 10 | 0 | 0 | 1 | 1 | 1 | 2 | 2 | 45 |
| I-L02 | 3 | 7 | 9 | 4 | 3 | 7 | 9 | 0 | 0 | 0 | 1 | 1 | 1 | 4 | 1 | under 1 |
| I-SEND | 80 | 3 084 | 6 897 | 81 | 3 003 | 3 241 | 7 040 | 18 | 8 | 1 | 5 | 2 | 1 | 162 | 2 784 | 122 |

### T2: each way of spending, simulated

**What every simulation assumes.** The route's order is held as it was. A node a way removes is one the route
would have treated as dead without building it, and a dead node gives a route nothing but its death, so with
the order held no other node appears or disappears and the count is exact. **What is not held in a built
route**: the tree chooses the instance with the fewest alternatives *not known dead* (`tree.rs`,
`branch_rest`), so a tree told the pairs would also choose otherwise. That is an order's effect, S1f.10.5's, and
no simulation here has it. The rest, way by way:

| way | reads | adds | removes | what it assumes |
|---|---|---|---|---|
| `layer-whole` | root's pair layer, before the first node | the layer's nodes the route does not visit itself: the cache is keyed by commitment, so the pair `{a, b}` is the route's node only where the route asks that set, and not `cl(a) ∪ {b}` | a dead node whose `K` holds a candidate dead alone or a pair root excludes | L1 |
| `layer-by-class` | the same with L3 over the classes: one node for the pairs between two classes of candidates with one closure | fewer pairs, a visited one where the route visits any | the same | L1, L3 |
| `singles` | root's node and the singles | those the route does not visit | a dead node whose `K` holds a candidate dead alone | L1 |
| `layer-by-need` | the layer's nodes as they are asked: before a node of two candidates or more, every pair of `K` not inside an alive closure the run holds, **singles first**, ascending, up to the first dead one | each node asked that the route never visits | the node, where a pair asked is dead | a protocol among several; never more than the layer |
| `death-pairs` | the same, asked only after a node of three or more has died | the same | later dead nodes whose `K` holds a pair or a single learned | the same |
| `death-core-pair`, `death-core` | the core of each death the run keeps, where it is of at most two candidates, or of any size | **no saturation**: a core is a search over the dead node's own derivation | later dead nodes whose `K` holds a core learned | a core is sufficient for the death, which is what it is; on a monotone program a set that holds it is dead. Its cost is in [§ What a core costs](#what-a-core-costs) |
| `denials` | the candidates each alive node's own fixpoint **denies**, a stored `(not b)` | no saturation: read where the node is computed | a dead node whose `K` holds a candidate that an alive node visited before it and inside it denies | on a monotone program a node holds what the nodes under it hold. **No alive node of any cell holds a candidate a node under it denies** (0 of the corpus's alive nodes on its 159 monotone programs) |
| `theory` | the small KB's pair layer | all of it: its nodes are another program's | a dead node whose `K` holds a pair the theory excludes or a candidate it kills alone | that the small KB derives no more than the program, which `theory_share.rs` holds per commit |
| `theory-shared` | the same pairs, paid once for every puzzle of one ontology | **nothing here: a bound** | the same | that something keeps a theory's pairs across programs, which nothing does today |
| `cover` | the small KB's layer and root's singles | both | `theory`'s, and a dead node whose `K` holds a candidate dead alone | |
| `wipe-out` | the cover and what root owes | the same | `cover`'s, and under `--single`, `-n 1` and `--maximal` **any** node whose `K` leaves an owed group no member, each dead alone or excluded by the theory with a member of `K` | L5 with L1: no member of R is above such a node. A licence no route has; describe and `--minimal` read the alive states that owe, so nothing is skipped there |
| `wipe-out-layer` | the same with root's pair layer | all three | the same, a member also out where root excludes it with a member of `K` | |
| `factor` | the coupling: both layers and the whole node | all three | every node but the distinct projections of the visited nodes onto `Coupling::components` | that R is the product of its parts, which `coupling.rs` holds wherever a describe exhausts. **Exact under describe** where every projection is itself a visited node, which it is in 148 of the 150 cells no budget stopped (in `ein-bugs/d3-goal-row-order.ein` and `features/03_forall.ein` two projections are not, and the count is an estimate); **an estimate under a tree**, whose order over one component need not be its order under another's state. A describe that reads R off its parts is another route |

**What an alive node denies is not in what a watch is handed**, so it is read apart: each alive node's fixpoint
computed again through `region::Nodes`, the node function's public reader, and asked `Kb::is_negated` of each
candidate; 886 887 saturations over the corpus, which are the reader's and no cell's.

**The instances, each cell with each way**, in saturations (`ways.jq`, which prints the facts beside);
`·` the cell itself.

| `--single` | the cell | `layer-whole` | `layer-by-class` | `singles` | `denials` | `layer-by-need` | `death-pairs` | `death-core-pair` | `death-core` | `theory` | `theory-shared` | `cover` | `wipe-out` | `wipe-out-layer` | `factor` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| I-Z1 | 49 | 572 | 172 | 82 | **9** | 73 | 102 | 47 | 46 | 1 606 | 9 | 1 658 | 1 655 | 2 168 | 2 214 |
| I-Z2 | 45 | 466 | 129 | 77 | **7** | 79 | 111 | 45 | 44 | 1 604 | 7 | 1 656 | 1 655 | 2 063 | 2 105 |
| I-Z2M | 39 | 4 569 | 1 918 | 133 | **29** | 126 | 46 | 33 | 33 | 4 696 | 39 | 4 790 | 4 790 | 9 225 | 9 237 |
| I-B06 | 8 | 133 | 133 | 133 | 8 | 8 | 8 | 8 | 8 | 8 766 | 8 | 8 891 | 8 891 | 8 891 | 8 892 |
| I-L02 | 2 | 7 | 7 | 4 | 2 | 2 | 2 | 2 | 2 | 9 | 2 | 11 | 11 | 14 | 15 |
| I-SEND | 17 747 | 10 940 | 10 940 | 17 012 | 17 747 | 10 506 | 10 078 | 8 230 | **8 220** | 11 638 | 8 397 | 11 268 | 11 268 | 14 181 | 23 962 |

| describe | the cell | `layer-whole` | `layer-by-class` | `singles` | `denials` | `layer-by-need` | `death-pairs` | `death-core-pair` | `death-core` | `theory` | `theory-shared` | `cover` | `wipe-out-layer` | `factor` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| I-Z1 | 445 | 740 | 347 | 445 | 314 | 278 | 296 | 317 | **268** | 1 912 | 315 | 1 912 | 2 337 | 2 526 |
| I-Z2 | 374 | 607 | 277 | 374 | 263 | 234 | 248 | 268 | **227** | 1 861 | 264 | 1 861 | 2 204 | 2 350 |
| I-Z2M, stopped | 8 391 | 10 202 | 8 073 | 8 391 | 7 260 | 7 330 | 7 444 | 7 336 | **6 921** | 11 942 | 7 285 | 11 942 | 14 859 | 15 968 |
| I-B06 | 133 | 133 | 133 | 133 | 133 | 133 | 133 | 133 | 133 | 8 891 | 133 | 8 891 | 8 891 | 8 892 |
| I-L02 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 15 | 8 | 15 | 15 | 16 |
| I-SEND, stopped | 336 101 | 336 100 | 336 100 | 336 100 | 336 101 | 336 100 | 336 100 | 336 100 | 336 100 | 338 732 | 335 491 | 338 732 | 339 341 | 339 343 |

| `--maximal` | the cell | `layer-whole` | `layer-by-class` | `singles` | `denials` | `layer-by-need` | `death-pairs` | `death-core-pair` | `death-core` | `theory` | `theory-shared` | `cover` | `wipe-out` | `wipe-out-layer` | `factor` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| I-Z1 | 49 | 572 | 172 | 82 | **9** | 75 | 106 | 47 | 47 | 1 606 | 9 | 1 658 | 1 655 | 2 168 | 2 214 |
| I-Z2 | 45 | 466 | 129 | 77 | **7** | 80 | 113 | 45 | 45 | 1 604 | 7 | 1 656 | 1 655 | 2 063 | 2 105 |
| I-Z2M | 2 346 | 4 632 | 1 981 | 2 440 | **100** | 1 021 | 1 136 | 343 | 341 | 4 769 | 112 | 4 863 | 4 863 | 9 280 | 11 544 |
| I-B06 | 258 | 133 | 133 | **133** | 258 | 132 | 258 | 144 | 144 | 9 016 | 258 | 8 891 | 8 891 | 8 891 | 9 018 |
| I-L02 | 7 | 8 | 8 | 8 | 7 | 8 | 8 | 7 | 7 | 14 | 7 | 15 | 15 | 15 | 16 |
| I-SEND | 17 754 | 10 940 | 10 940 | 17 019 | 17 754 | 10 508 | 10 082 | 8 232 | **8 222** | 11 638 | 8 397 | 11 268 | 11 268 | 14 181 | 23 969 |

Under `-n 1`: I-Z1 6, to 5 by `denials`; I-Z2 5, which no way lowers; I-Z2M 36, to 27 by `denials` and 31 by a
core; I-SEND 15 854, to 10 482 by the layer whole, 10 017 by need, 9 577 after a death, 7 760 by a core, 7 837
by the theory's pairs for nothing.

### T3: the verdicts

By the rule written before the table: *go* where the corpus total with the way's cost added is lower and no
program's cell is higher by more than one tenth; taken in saturations and in facts beyond root, and *go* only
where both say so ([§ Defaults taken](#defaults-taken)). A cell: the verdict; the total with the way, where the
question's total is the first row's; then the programs whose cell is lower and those higher by more than a
tenth in saturations, `+k` for programs higher in facts alone
(`jq -r -f build/structure/prices/matrix.jq build/structure/prices/verdicts.json`).

| way | describe, `--count` | `--single` | `-n 1` | `--minimal` | `--maximal` |
|---|---|---|---|---|---|
| *the question's total* | *1 350 675* | *19 677* | *17 198* | *231 776* | *483 283* |
| `layer-whole` | no · 1 355 446 · 3 / 9 +1 | no · 27 790 · 1 / 110 | no · 27 227 · 1 / 156 | no · 228 908 · 5 / 126 +2 | no · 484 951 · 24 / 78 |
| `layer-by-class` | no · 1 349 686 · 13 / 0 +1 | no · 20 964 · 1 / 110 | no · 20 401 · 1 / 156 | no · 223 147 · 10 / 121 +2 | no · 478 125 · 27 / 76 |
| `singles` | **go**, by one node of a stopped cell · 1 350 674 · 1 / 0 | no · 20 414 · 1 / 103 | no · 18 516 · 1 / 152 | no · 232 302 · 6 / 118 | no · 482 787 · 28 / 72 |
| `denials` | **go** · 1 348 020 · 10 / 0 | **go** · 19 541 · 5 / 0 | **go** · 17 179 · 3 / 0 | **go** · 228 523 · 5 / 0 | **go** · 478 477 · 10 / 0 |
| `layer-by-need` | no · 1 348 203 · 11 / 2 +1 | no · 13 032 · 2 / 15 | no · 11 840 · 1 / 14 | **go** · 218 909 · 10 / 0 | no · 474 595 · 25 / 46 |
| `death-pairs` | **go** · 1 348 260 · 11 / 0 | no · 12 471 · 3 / 14 | no · 11 160 · 2 / 12 | **go** · 219 267 · 10 / 0 | no · 473 683 · 9 / 17 |
| `death-core-pair` | go in the work's units · 1 348 077 · 13 / 0 | go so · 10 052 · 8 / 0 | go so · 9 011 · 5 / 0 | go so · 218 693 · 10 / 0 | go so · 468 813 · 43 / 0 |
| `death-core` | go in the work's units · 1 347 116 · 13 / 0 | go so · 10 037 · 11 / 0 | go so · 9 011 · 5 / 0 | go so · 214 801 · 10 / 0 | go so · 468 616 · 47 / 0 |
| `theory` | no · 1 399 483 · 0 / 147 +1 | no · 62 688 · 1 / 157 +1 | no · 61 809 · 1 / 158 | no · 270 668 · 2 / 157 | no · 521 337 · 7 / 146 +1 |
| `theory-shared`, a bound | go as a bound · 1 346 680 · 67 / 0 | go so · 9 885 · 39 / 0 | go so · 9 006 · 12 / 0 | go so · 217 865 · 30 / 0 | go so · 468 534 · 69 / 0 |
| `cover` | no · 1 399 671 · 0 / 148 | no · 64 011 · 1 / 158 | no · 63 435 · 1 / 158 | no · 271 603 · 2 / 157 | no · 521 616 · 1 / 158 |
| `wipe-out` | no · 1 399 671 · 0 / 148 | no · 64 006 · 1 / 158 | no · 63 434 · 1 / 158 | no · 271 603 · 2 / 157 | no · 521 611 · 1 / 158 |
| `wipe-out-layer` | no · 1 408 249 · 0 / 148 | no · 80 590 · 1 / 158 | no · 80 027 · 1 / 158 | no · 281 711 · 1 / 157 | no · 537 735 · 1 / 158 |
| `factor` | no, an estimate on two cells · 1 400 985 · 7 / 143 | no, an estimate · 90 812 · 0 / 159 | no, an estimate · 88 674 · 0 / 159 | no, an estimate · 295 282 · 0 / 158 | no, an estimate · 553 323 · 0 / 159 |

Every way is applied on the 159 monotone programs and on none of the 29 flagged. **Over the cells no budget
stopped the verdicts are the same but one** (`prices.md` § Over the cells no budget stopped): `singles` under
describe and `--count` is *go* by a single node, the ⊤ of 80 candidates that I-SEND's stopped describe saturates
once at its end, and without the stopped cells it adds nothing, removes nothing and is no *go*. Two of
`layer-whole`'s nine losers under describe are the stopped `zebra2-minus-15` cells.

**Which object's cost sank each *no*, and where the losers are.** The losers by the size of their cell, under
10 saturations, 10 to 99, 100 and over (`losers_by_size.jq`); all of them are named in
[§ The losers, named](#the-losers-named).

| way | the dearest object | describe | `--single` | `-n 1` | `--minimal` | `--maximal` |
|---|---|---|---|---|---|---|
| `layer-whole` | **root's pairs**: 46 140 saturations, 7 631 of them beyond a describe's own nodes and 18 188 beyond a `--single`'s | 0 / 2 / 7 | 90 / 17 / 3 | 142 / 11 / 3 | 121 / 0 / 5 | 47 / 24 / 7 |
| `layer-by-class` | root's pairs, 2 115 beyond a describe's and 11 362 beyond a `--single`'s | none in saturations; one in facts | 90 / 17 / 3 | 142 / 11 / 3 | 121 / 0 / 0 | 47 / 24 / 5 |
| `singles` | **root's singles**: `n + 1` saturations before a question of fewer | none | 88 / 15 / 0 | 142 / 9 / 1 | 118 / 0 / 0 | 48 / 23 / 1 |
| `layer-by-need` | root's pairs; on the zebras **the singles asked before a pair** | 0 / 0 / 2 | 1 / 12 / 2 | 5 / 8 / 1 | none | 21 / 21 / 4 |
| `death-pairs` | the same | none | 0 / 12 / 2 | 3 / 8 / 1 | none | 3 / 12 / 2 |
| `theory`, `cover`, `wipe-out`, `wipe-out-layer`, `factor` | **the small KB's layer**: 52 803 saturations over the 159 monotone programs, against 19 677 for every `--single` of the corpus | 92 / 35 / 20 | 135 / 19 / 3 | 144 / 11 / 3 | 128 / 20 / 9 | 96 / 36 / 14 |

So three things sink a way, and they are not one. **A preamble of any size on a cell of a few saturations**:
most losers of every preamble are cells under ten, where the rule's tenth is less than one saturation.
**Root's pairs on the puzzles this phase is about**: the layer whole is more than the describe on the four
zebra programs whose describe finishes (`zebra.ein` 445 to 740, `zebra2.ein` 374 to 607), which is S1f.10.1's
finding again. **The small KB's layer everywhere**: it is dearer than root's own, 1 597 against 465 on I-Z2,
because fewer of its candidates are dead alone, and no way that reads the cover or the coupling as built can be
cheaper than it.

#### What a core costs

`death-core-pair` and `death-core` add no saturation, so in the work's units they have no loser. They read a
core at every death the run keeps, a search, and that is a cost the rule's units do not see: **on I-Z2 a core
is 19 ms where a saturation is 1.9 ms** (describe: 3.61 s more over 190 deaths, 0.70 s for 374 saturations),
and on I-SEND 20 µs where a saturation is 49 µs. In seconds, on this machine: the cells' plain walls summed,
and an estimate with the way, a kept saturation at the plain run's wall a saturation and a core at the watched
run's extra wall a death (`prices.md` § What a core costs).

| question | wall | with `death-core-pair`, estimated | with `death-core`, estimated | slower by more than a tenth, of the cells of 10 ms or more |
|---|---:|---:|---:|---|
| describe | 56.38 | 81.33 | 67.66 | 6: `beyond_arith/equation_two_unknowns`, `zebra2-hints`, the two `zebra2-minus-15`, `zebra2-obligations`, `zebra2` (0.70 to 1.24 s by `death-core`) |
| `--single` | 1.57 | 2.79 | 2.75 | 6: `equation_two_unknowns`, `zebra` (0.220 to 0.246), the two `zebra2-minus-15`, `zebra2-obligations`, `zebra2` (0.123 to 0.76) |
| `-n 1` | 0.99 | 0.94 | 0.94 | 3: `equation_two_unknowns`, the two `zebra2-minus-15` (0.047 to 0.107) |
| `--minimal` | 40.13 | 68.72 | 51.70 | 6: `cryptarithm`, `equation_two_unknowns`, the two `zebra2-minus-15`, `zebra2-obligations`, `zebra2` |
| `--maximal` | 20.80 | 19.13 | 19.01 | 6: `equation_two_unknowns`, `branching/08_hypothesis_relation_whitelist`, `zebra`, `zebra2-hints`, `zebra2-obligations`, `zebra2` |

**So the core of a death is *go* in saturations and facts and a loss in time on the zebras as cores cost
today.** It pays where a core is cheap beside its node, SEND, and on `zebra2-minus-15.ein --maximal`, where 2 005
of 2 266 deaths repeat a core already read (6.6 s to an estimated 4.3 s). [`findings.md`](findings.md)'s K-7 is
the same cost from the support's side.

#### Each *no*, and the next way

A *no* is about one way for one question. For each, what was tried next on the same runs, and what it cost.

| the *no* | the next way | its price, from the same runs |
|---|---|---|
| **the pair layer whole**, all six questions | the same layer **by need**, then **after a death**; and the layer with **L3 over the classes** | by need: *go* for `--minimal` (231 776 to 218 909, no loser), *no* elsewhere with 2 losers under describe (`branching/02_one_dead_one_alive` 193 to 220, `branching/08_hypothesis_relation_whitelist` 335 to 401), 15 under `--single`, 14 under `-n 1`, 46 under `--maximal`. After a death: *go* for describe, `--count` and `--minimal`, with 14, 12 and 17 losers under the three others. Over the classes: 7 631 of root's pairs beyond a describe's nodes become 2 115, no loser in saturations under describe and one in facts, and the losers of the other questions stay |
| **by need and after a death**, under `--single`, `-n 1` and `--maximal` | a source with **no saturation**: what a state denies; the core of a death. And **a pair asked without its singles first**, which is what loses on the zebras (`zebra2.ein --single` 45 to 79 by need, 44 of the 75 nodes asked being singles) | `denials`: *go* for all six, no loser, lower on 10 programs under describe and `--maximal`, 5 under `--single`. The core: above. A pair asked alone is **not priced**: such a node is not always one of the layer's, whose facts are what was read |
| **the theory's pairs, the cover, the owed groups, the coupling**, all six questions, by the small KB's layer | **the same pairs at no cost here** | `theory-shared`, a bound: *go* for all six, lower on 67 programs under describe and 39 under `--single`, which it takes from 19 677 to 9 885. Three places they could come from, none built: **(a) what a state denies**, which on the zebras is every pair of the theory already (I-Z2 `--single` 7 by `denials` and 7 by the bound) and on SEND none; **(b) kept across the puzzles of one ontology**, as the stage's own table has it, which needs something to keep them and a key for *one ontology*; **(c) the small KB asked by need**, a node of which is 20 facts on I-Z2 where root's is 147, and as many saturations as `layer-by-need` asks: its nodes were not read one by one here, so it is **not priced** |
| **`wipe-out`**, an owed group left no member | nothing cheaper is needed: it removes almost nothing | 5 nodes under `--single` on 3 programs, 1 under `-n 1`, 5 under `--maximal`, one of them alive each time; with root's pairs too, 3, 3 and 19. The routes' own L5 already takes the alternatives |
| **`factor`**, the components solved apart | **the components at the node's price**: the whole node is one saturation, and it needs the singles, which a describe visits anyway | where R factors it is large: `syntax/constraint-scopes.ein` and `syntax/equality.ein` 4 096 to 13 nodes over 12 components, `features/02_star_in_identifiers.ein` 1 736 to 220 over 4, 10 607 saturations of describe over the corpus, on 7 programs net of the coupling's cost. What sinks it is the two layers, 60 917 added. A coupling read from the node alone, its questions the candidates and the owed instances, was **not built**, so its components are not known to be these; if they are, the price is one saturation a program, which still loses on every describe under ten saturations that does not factor |
| **the order** ([S1f.10.5](s1f.10.5_ordering.md)) and ***exactly one* under describe** ([S1f.10.14](s1f.10.14_exactly_one_spent.md)) | **cannot be simulated from visited nodes**: each changes which nodes a route builds | bounds only, below |

### The two shares

**Alive and owing, per question** ([S1f.10.14](s1f.10.14_exactly_one_spent.md)).

| question | saturations | alive and owing | … an owed group's instance | … no licensed instance at all | built above a state that owes no licensed instance |
|---|---:|---:|---:|---:|---:|
| describe, `--count` | 1 350 675 | 343 323 (25.4 %) | 342 854 | 423 | not read: describe takes every extension |
| `--single` | 19 677 | 2 193 (11.1 %) | 2 038 | 136 | 241 |
| `-n 1` | 17 198 | 2 162 (12.6 %) | 2 036 | 107 | 222 |
| `--minimal` | 231 776 | 199 506 (86.1 %) | 199 300 | 160 | 273 |
| `--maximal` | 483 283 | 2 524 (0.5 %) | 2 082 | 423 | 272 |

- **Describe's share is three stopped cells**: I-SEND's 330 329 and the two `zebra2-minus-15` programs' 5 530
  each are 341 389 of the 343 323. Over the monotone cells no budget stopped it is 1 625 of 28 540. `--minimal`'s
  is the same three, 184 081 and twice 6 877, 197 835 of 199 506.
- **It is an upper bound on what *exactly one* could remove under describe, and not a simulation**: describe
  reads `k_c`, N and the merged states off the alive states, so removing them is another route (S1f.10.14's T4).
- **Its place 1 does not occur on I-Z1.** Under no question does `zebra.ein` visit a state that owes and owes
  no licensed instance: every owing state there owes a `slot-owed-room` instance too, and the tree branches on
  it. The instances L5 does not license cost I-Z1 nothing in today's routes. Where a state does owe unlicensed
  instances alone, monotone programs only: `lattice/06_friends_tables_everyone_sits.ein`, **176 of whose 207
  `--single` saturations are built above such a state**; `ein-bugs/wrong-answers/oblig-fresh-name-two.ein` and
  its `-declared` twin, 19 of 20; `oblig-derived-domain`, `oblig-fresh-name` and their `-declared` twins, 6 of 7;
  `features/13_mixed_solution_and_open.ein`, 3 of 4. The count is the nodes a tree built by taking every
  extension, an upper bound on what a licence for those instances removes at those states.

**What a best order could not go under** ([S1f.10.5](s1f.10.5_ordering.md)), read off the leaves found: root's
node and one node a candidate of the leaves' commitments, the fewest nodes of any tree that reaches them. It is
a floor for finding the leaves; what it takes to show there is no other, under `--single`, is not read off a
leaf.

| question | cells with a leaf | their saturations | the floor | above the floor | without I-SEND | cells with no leaf |
|---|---:|---:|---:|---:|---|---:|
| `--single` | 182 | 19 620 | 437 | 19 183 (97.8 %) | 1 873 against 428 | 6, 57 saturations |
| `-n 1` | 182 | 17 141 | 270 | 16 871 (98.4 %) | 1 287 against 261 | 6, 57 saturations |

I-SEND: `--single` 17 747 and `-n 1` 15 854 against a floor of 9, root and the eight letters. I-Z1 49 and 6
against 3; I-Z2 45 and 5 against 3; I-Z2M 39 against 6, two leaves, and 36 against 5.

### Why not a saturation rule: the comparison restated

[`domain_elim_vs_hypothesis.md`](../../../docs/kernel/inference/domain_elim_vs_hypothesis.md) set a rule that
derives an answer at root against a search that forks and refutes, on three fixtures. In today's terms
(`ein.rs/target/release/examples/price_list --all --out DIR examples/domain_elim/*.ein`, 2026-11-16): with the
elimination rules, `ab.ein`, root's saturation decides every placement, **H is empty** and each of the six
questions is root's node, 1 saturation; with negative completion alone, `b_only.ein`, root denies all but one
placement, H is 1 candidate and a describe is 2 saturations; with neither, `b_branch.ein`, H is 7 candidates of
which 6 are dead alone, a describe is 8 saturations and a `--maximal` 14, twelve of them dead. So the page's one claim that survives is true and is in every number above: **what a rule
decides at root never reaches the search**, since a candidate root holds or denies is not in H, and
`dead_single` is what the rules left. **The stage file's sentence is false one step further up**: what the
rules derive *at a node above root* is not in the numbers, because no route reads it. A state whose fixpoint
stores `(not b)` is still extended by `b`, and `S ∪ {b}` is saturated to watch it die
([`findings.md`](findings.md)'s K-13). That is the `denials` way: 2 655 of describe's saturations, 136 of
`--single`'s, 3 253 of `--minimal`'s and 4 806 of `--maximal`'s, 2 582, 116, 3 176 and 4 643 of them deaths
that hold a pair of the theory. Whether a program's exclusions arrive so depends on how its rules are written:
the zebras' store the negation; `std.algebra`'s `functional` and `injective` conclude `(false)` from the two
facts and store nothing, so none of SEND's 9 350 pair deaths is denied. The page itself states three things
that are not true today, K-12, and was not edited.

### What each stage starts from

- **[S1f.10.3](s1f.10.3_the_restricted_join.md), *at most one* spent.** Not from the pair layer whole, which is
  *no* for every question. **First, what a state denies**: no saturation, no structure, *go* for all six, and on
  the zebras it is every pair of the theory. It needs the node function to report a node's denials beside its
  closure, and L1's memory to take them; its licence is the one L1 has. **Second, a death's core**, which is the
  pair of the death where the pair is the reason and a larger nogood where it is not: 9 640 of `--single`'s
  19 677 saturations over the corpus and 14 667 of `--maximal`'s, at the price of a search that today costs ten
  saturations' time on the zebras, so it starts from K-7. **Third, the layer by need or after a death**, which
  pays under describe, `--count` and `--minimal` and not under the three small questions as the protocol stands.
  **The theory's pairs** are the ceiling of all of it, 9 885 for `--single`: what is left to find is a source
  for them that costs SEND nothing, and the cheapest candidate is in the stdlib, a `functional` that stores the
  negation it now only uses.
- **[S1f.10.5](s1f.10.5_ordering.md), the order.** Its ceiling is above: nearly all of `--single` and `-n 1` is
  above the floor, and nearly all of that is I-SEND. On I-SEND the pairs, by whatever source, take 17 747 to
  about 8 300 and stop: **what is left is 6 184 deaths of three candidates or more and 1 772 alive states that
  owe**, against a floor of 9. Every one of those deaths has its core inside an edge of the whole node's reading
  and 648 inside the edges one application back, `{D, E}` and `{D, E, Y}`, so the coupling does name them. Its price as built
  is both layers and the node, 6 326 saturations on I-SEND, a third of the `--single` it would order; the node
  alone is one saturation and 162 facts. So the stage starts from **the node read without the layers**, its
  questions the owed instances L5's table already has at root, and from measuring an order with *at most one*
  already spent, since the pairs are half of what an order would otherwise be credited with.
- **[S1f.10.14](s1f.10.14_exactly_one_spent.md), *exactly one* spent.** Its place 1, an instance L5 does not
  license, is not where I-Z1's work is: no state of `zebra.ein` owes such instances alone under any question.
  Its witness is `lattice/06_friends_tables_everyone_sits.ein`, 176 of 207. Its place 3, describe, is three
  stopped programs' alive states, 341 389 saturations, which no exclusion and no order removes: that is the
  number its question to the user carries. An owed group left no member is 5 nodes of the corpus's `--single`s,
  so *exactly one* has nothing to add to L5 on today's trees but the order.
- **[S1f.10.12](s1f.10.12_the_structure_shown.md), the bundle**: `build/structure/prices/prices.md` and each
  program's `prices.md` are the pages; `prices.jsonl`, `verdicts.json` and the `nodes.jsonl` files the raw.

### Against the plan and the earlier records

- **No count of an earlier record is contradicted.** The 564 cells the census took are the same here; the
  objects' costs are S1f.10.1's, S1f.10.10's and S1f.10.11's to the saturation and the fact; the re-plan's probe
  is reproduced in all six of its cells.
- **The stage file's *what the rules derive at a node is already in every number above* is false above root**
  ([§ Why not a saturation rule](#why-not-a-saturation-rule-the-comparison-restated)), and
  [S1f.10.3](s1f.10.3_the_restricted_join.md)'s Notes repeat it.
- **[S1f.10.14](s1f.10.14_exactly_one_spent.md)'s Context** has `zebra.ein`'s `slot-owed-fill` leave *a state
  that owes only these* to branch on every extension. No visited state of `zebra.ein` is one.
- **The stage file's split is of four parts and this one of eight**: the two halves of each of its rows, and
  `dead_single`, which it did not have.
- **[S1f.10.3](s1f.10.3_the_restricted_join.md)'s *not spend a pair of the theory that is not a node of this
  program*** stands against the `theory` ways as a way to build. They are priced because the stage file lists
  the small KB as a source; what S1f.10.3 may spend of them is its own.
- **K-7's *a core costs a few [ms]*** is 19 ms on I-Z2 here, as `--dump-lattice` takes it.

### Verification

2026-11-16, on the tree as it stands, each green:

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --lib --test price_list --test pair_layer \
    --test theory_share --test cover --test exactly_one --test coupling --test structure_invariance \
    --test explore --test region --test claim_routes
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test lever_invariance --test nightly_steps \
    --test gate_steps --test config_reference
cargo fmt --manifest-path ein.rs/Cargo.toml --all --check
cargo clippy --manifest-path ein.rs/Cargo.toml --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -q --manifest-path ein.rs/Cargo.toml --workspace --no-deps
python3 utils/check_hashmap_iteration.py
python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md \
    examples/README.md stdlib/README.md utils/README.md c/README.md
```

`ein-infer/tests/price_list.rs`, on seven programs, the two larger under the questions a tree decides:
`a_routes_nodes_are_read_and_no_route_is_changed` (the watched run is the plain run in answer and in work; the
nodes read are its saturations and its facts; the split is a partition; a way is applied on a monotone program
alone, removes no more than the cell holds and asks for no more than the layer has; no alive node holds a
candidate a node under it denies; two members of one group under a dead node are the theory's to remove) and
`a_conflict_of_three_is_no_pairs_to_remove` (I-L02: no way removes a node under any question, and its one death
beyond root is inside the coupling's one edge). Each fails when it should: with a part of the split counted twice the
first reports every cell that has that part, and with the watched run asked at another budget it reports the
cells the budget then stops (each tried on the final tree and reverted, 2026-11-16).

### Defaults taken

Each the user's to revise. The first six are how the rule was read where it does not say, written here before
the cells they decide were looked at.

1. **Work is taken in both units, and a *go* needs both**: saturations and facts beyond root. The table gives
   each unit's verdict, and they differ in one row, `layer-by-class` under describe.
2. **A stopped cell counts with what it had spent**: the way is priced on the nodes it visited, which is what the
   same route would have spent to the same frontier. The verdicts over the cells no budget stopped are given
   beside and are the same.
3. **A flagged program is in every total and no way is applied to it.**
4. **A unit in which a cell is 0** loses by any positive net. None of the 1 128 cells is an error, so none is
   left out.
5. **The small KB's facts are counted as facts**, though they are beyond its own root.
6. **The coupling's node is charged to the way that reads it**, where the engine charges it to no budget.
7. **The rule is applied to a way of spending**, which is a source with what is done with it, since a source
   removes nothing by itself. The stage file's four sources are `layer-whole`, `layer-by-need`, `death-pairs`
   with the two `death-core`s, and `theory`.
8. **`dead_single` is a part of its own.**
9. **A core's cost is given in time**, since it is in neither unit of the work, and the two `death-core` ways are
   called *go in the work's units* and not *go*.
10. **`theory-shared` is called a bound**, and not a verdict, because its cost is not zero and is not known.
11. **Ways the stage file did not name were priced**: `layer-by-class` (S1f.10.1's `open_by_class`, which its
    record left to this stage), `singles`, `denials`, `wipe-out` and `factor`, each a use of an object track S
    built or, for `denials`, of what a node already holds.
12. **The by-need protocol**: singles first, pairs ascending, up to the first dead one. Another protocol has
    another price.
13. **No slow half and no nightly step.** The test reads seven programs per commit.

### What was not done

- **An order is not simulated**, nor *exactly one* under describe: each needs a route that builds other nodes.
  The record gives a floor and a share.
- **The small KB asked by need, a pair asked without its singles, and a coupling read from the node alone** are
  named as next ways and not priced.
- **The coupling under its other readings** was not read again: the pairs' reading costs no saturation beyond
  the layers and the three node readings the one node, as S1f.10.11 measured; the components simulated are the
  whole node's.
- **I-SENDW** is not banked; it is S1f.10.5's first task.
- **The walls are one run on one P-core**, with no spread. The counts were taken twice.
- **`denials` is simulated, not built**: the node function does not report what a node denies.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh` were not run: the gate is the batch's.

### The `jq` programs

`build/structure/prices/`, each with its command in its first line: `instances.jq`, `ways.jq`, `matrix.jq`,
`shares.jq`, `losers.jq`, `losers_by_size.jq`, `against_census.jq`, and `losers_named.py`, which writes the
section below.

`build/structure/prices/instances.jq`:

```jq
# jq -r -f build/structure/prices/instances.jq build/structure/prices/prices.jsonl
# The six instances' cells: the work and its split, saturations a part.
def inst: {"examples/zebra.ein": "I-Z1", "examples/zebra2.ein": "I-Z2", "examples/zebra2-minus-15.ein": "I-Z2M",
  "examples/branching/06_lookahead_on.ein": "I-B06", "examples/lattice/02_genuine_3set_death.ein": "I-L02",
  "examples/beyond_arith/send_more_money.ein": "I-SEND"};
select(inst[.path]) | inst[.path] as $i | .cells | to_entries[] | .value as $c
| [$i, $c.flags, $c.route, $c.work.saturations, $c.work.facts,
   (if $c.finished then ($c.verdict // "") else "stopped" end),
   ($c.split | .in_r[0], .owing_owed_group[0], .owing_other[0], .dead_theory[0], .dead_puzzle[0], .dead_single[0], .dead_edge[0], .dead_other[0]),
   ($c.detail.core_in_one_edge_one_step // 0), ($c.wall * 1000 | round), ($c.wall_watched * 1000 | round)]
| @tsv
```

`build/structure/prices/ways.jq`:

```jq
# jq -r --arg q single -f build/structure/prices/ways.jq build/structure/prices/prices.jsonl
# The six instances under question $q: the cell, then the cell with each way, in saturations and (facts).
def inst: {"examples/zebra.ein": "I-Z1", "examples/zebra2.ein": "I-Z2", "examples/zebra2-minus-15.ein": "I-Z2M",
  "examples/branching/06_lookahead_on.ein": "I-B06", "examples/lattice/02_genuine_3set_death.ein": "I-L02",
  "examples/beyond_arith/send_more_money.ein": "I-SEND"};
def ways: ["layer-whole", "layer-by-class", "singles", "denials", "layer-by-need", "death-pairs", "death-core-pair", "death-core",
  "theory", "theory-shared", "cover", "wipe-out", "wipe-out-layer", "factor"];
select(inst[.path]) | inst[.path] as $i | .cells[$q] as $c
| [$i, "\($c.work.saturations) (\($c.work.facts))"]
  + [ways[] as $w | $c.ways[$w] | if .applies then "\(.with.saturations) (\(.with.facts // "–"))" else "·" end]
| @tsv
```

`build/structure/prices/matrix.jq`:

```jq
# jq -r -f build/structure/prices/matrix.jq build/structure/prices/verdicts.json
# The verdicts as a matrix: a way a row, a question a column. A cell is the verdict, then
# "total → with the way" in saturations, then the programs lower / higher by more than a tenth
# in saturations, and "+k" for those higher in facts alone.
def qs: ["describe", "count", "single", "n1", "minimal", "maximal"];
def ways: ["layer-whole", "layer-by-class", "singles", "denials", "layer-by-need", "death-pairs", "death-core-pair",
  "death-core", "theory", "theory-shared", "cover", "wipe-out", "wipe-out-layer", "factor"];
.verdicts as $v
| (["way"] + qs | @tsv),
  (ways[] as $w
   | [$w] + [qs[] as $q | $v[$q][$w] as $x
       | ($x.saturations.losers | map(.program)) as $ls
       | ([$x.facts.losers[] | select(.program as $p | $ls | index($p) | not)] | length) as $more
       | "\(if $x.go == true then "GO" elif $x.go == false then "no" else "?" end) \($x.saturations.total)→\($x.saturations.with) \($x.saturations.lower_on)/\($ls | length)\(if $more > 0 then "+\($more)" else "" end)"]
   | @tsv)
```

`build/structure/prices/shares.jq`:

```jq
# jq -s -r -f build/structure/prices/shares.jq build/structure/prices/prices.jsonl
# Per question: the cells, the saturations, the split's shares, over every program and over the
# monotone programs whose cell no budget stopped.
def qs: ["describe", "count", "single", "n1", "minimal", "maximal"];
def parts: ["in_r", "owing_owed_group", "owing_other", "dead_theory", "dead_puzzle", "dead_single", "dead_edge", "dead_other"];
def row($cells): ($cells | map(.work.saturations) | add // 0) as $s
  | [($cells | length), $s] + [parts[] as $p | ($cells | map(.split[$p][0]) | add // 0)];
. as $all
| qs[] as $q
| ([$all[] | .cells[$q] | select(.error | not)]) as $every
| ([$all[] | select(.monotone) | .cells[$q] | select(.error | not) | select(.finished)]) as $kept
| ([$q, "every program"] + row($every) | @tsv), ([$q, "monotone, not stopped"] + row($kept) | @tsv)
```

`build/structure/prices/losers.jq`:

```jq
# jq -r --arg q single --arg w layer-by-need -f build/structure/prices/losers.jq build/structure/prices/verdicts.json
# One verdict: its totals, and every program whose cell is higher by more than one tenth.
.verdicts[$q][$w] as $v
| "\($q), \($w): applied on \($v.applied_on); go \($v.go)",
  "saturations \($v.saturations.total) → \($v.saturations.with) (added \($v.saturations.added), removed \($v.saturations.removed)); lower on \($v.saturations.lower_on), higher by more than a tenth on \($v.saturations.losers | length); the dearest object: \($v.saturations.dearest)",
  "facts \($v.facts.total) → \($v.facts.with); lower on \($v.facts.lower_on), higher on \($v.facts.losers | length)",
  "losers in saturations: " + ($v.saturations.losers | map("\(.program) \(.cell) → \(.with) (\(.dearest))\(if .stopped then ", stopped" else "" end)") | join("; ")),
  "losers in facts alone: " + ([$v.facts.losers[] | select(.program as $p | $v.saturations.losers | map(.program) | index($p) | not) | "\(.program) \(.cell) → \(.with)"] | join("; "))
```

`build/structure/prices/losers_by_size.jq`:

```jq
# jq -r -f build/structure/prices/losers_by_size.jq build/structure/prices/verdicts.json
# Per way and question: the programs higher by more than one tenth in saturations, by the size of
# their cell: under 10 saturations / 10 to 99 / 100 and over.
def qs: ["describe", "count", "single", "n1", "minimal", "maximal"];
def ways: ["layer-whole", "layer-by-class", "singles", "denials", "layer-by-need", "death-pairs", "death-core-pair",
  "death-core", "theory", "theory-shared", "cover", "wipe-out", "wipe-out-layer", "factor"];
.verdicts as $v
| (["way"] + qs | @tsv),
  (ways[] as $w
   | [$w] + [qs[] as $q | $v[$q][$w].saturations.losers
       | "\(map(select(.cell < 10)) | length)/\(map(select(.cell >= 10 and .cell < 100)) | length)/\(map(select(.cell >= 100)) | length)"]
   | @tsv)
```

`build/structure/prices/against_census.jq`:

```jq
# jq -r -n --slurpfile c build/structure/census.jsonl --slurpfile p build/structure/prices/prices.jsonl -f build/structure/prices/against_census.jq
# Each cell the census took, a describe's, a --single's and a --maximal's, against the price list's.
($c | map(select(.searches)) | map({key: .path, value: .routes}) | from_entries) as $census
| [ $p[] | .path as $path | $census[$path] as $r
    | (["describe", "describe"], ["single", "single"], ["maximal", "maximal"]) as [$mine, $theirs]
    | {path: $path, q: $mine, mine: [.cells[$mine].work.saturations, .cells[$mine].work.facts],
       theirs: [$r[$theirs].saturations, $r[$theirs].facts]} ]
| {programs: ($p | length), in_the_census: ($census | length), cells: length,
   differ: map(select(.mine != .theirs))}
```

`build/structure/prices/losers_named.py`:

```python
#!/usr/bin/env python3
"""The losers of every *no*, named, as the stage record's appendix.

    python3 build/structure/prices/losers_named.py > build/structure/prices/losers_named.md

Reads prices.jsonl and verdicts.json beside it. A program is `examples/<name>.ein` unless it is
written with its root. Where more than half of the programs a way is applied on lose, the list is
of those that do not."""
import json, os, collections
here = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(here, "prices.jsonl"))]
verdicts = json.load(open(os.path.join(here, "verdicts.json")))["verdicts"]
mono = sorted(r["path"] for r in rows if r["monotone"])
def short(p):
    p = p[:-4] if p.endswith(".ein") else p
    return p[len("examples/"):] if p.startswith("examples/") else p
flags = {"describe": "describe", "count": "`--count`", "single": "`--single`", "n1": "`-n 1`",
         "minimal": "`--minimal`", "maximal": "`--maximal`"}
ways = ["layer-whole", "layer-by-class", "singles", "denials", "layer-by-need", "death-pairs",
        "death-core-pair", "death-core", "theory", "theory-shared", "cover", "wipe-out",
        "wipe-out-layer", "factor"]
out = collections.OrderedDict()
for w in ways:
    for q in flags:
        v = verdicts[q][w]
        ls = sorted(set(l["program"] for l in v["saturations"]["losers"]))
        lf = sorted(set(l["program"] for l in v["facts"]["losers"]) - set(ls))
        if not ls and not lf:
            continue
        if len(ls) == len(mono):
            text = f"**{len(ls)}**, every monotone program"
        elif len(ls) > len(mono) / 2:
            rest = [p for p in mono if p not in ls]
            text = f"**{len(ls)}**, every monotone program but {len(rest)}: " + ", ".join(f"`{short(p)}`" for p in rest)
        elif ls:
            text = f"**{len(ls)}**: " + ", ".join(f"`{short(p)}`" for p in ls)
        else:
            text = "none in saturations"
        if lf:
            text += f"; and in facts alone {len(lf)}: " + ", ".join(f"`{short(p)}`" for p in lf)
        out.setdefault((w, text), []).append(flags[q])
for (w, text), qs in out.items():
    print(f"- `{w}`, {' and '.join(qs)}: {text}")
```

### The losers, named

Every program whose cell is higher by more than one tenth in saturations, per way and question, and those
higher in facts alone; written by `python3 build/structure/prices/losers_named.py`. A program is
`examples/<name>.ein` unless it is written with its root. Where more than half of the 159 monotone programs
lose, the list is of those that do **not**. The ways with no loser in either unit, `denials`, the two
`death-core`s and `theory-shared`, have no line.

- `layer-whole`, describe and `--count`: **9**: `branching/02_one_dead_one_alive`, `branching/04_two_levels`, `branching/08_hypothesis_relation_whitelist`, `zebra`, `zebra2-hints`, `zebra2-minus-15-obligations`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2`; and in facts alone 1: `ein-bugs/d3-goal-row-order`
- `layer-whole`, `--single`: **110**, every monotone program but 49: `beyond_arith/send_more_money`, `branching/01_saturate_only`, `branching/12_typed_blind_solve`, `domain_elim/b_only`, `ein-bugs/alive-empty-phase1`, `ein-bugs/binding-key-int-only`, `ein-bugs/wrong-answers/assert-neq-w-x`, `ein-bugs/wrong-answers/core-activator-given`, `ein-bugs/wrong-answers/core-activator-root-derived`, `ein-bugs/wrong-answers/empty-match-activator`, `ein-bugs/wrong-answers/empty-match-hrule`, `ein-bugs/wrong-answers/forall-core-elim`, `ein-bugs/wrong-answers/forced-1-two`, `ein-bugs/wrong-answers/forced-1`, `ein-bugs/wrong-answers/forced-2-off`, `ein-bugs/wrong-answers/forced-2-two`, `ein-bugs/wrong-answers/forced-2`, `ein-bugs/wrong-answers/fresh-name-nospawn`, `ein-bugs/wrong-answers/hrule-reads-absent-not`, `ein-bugs/wrong-answers/hrule-reads-not`, `ein-bugs/wrong-answers/hrule-reads-unknown`, `ein-bugs/wrong-answers/kw-neq-in-fact-twin`, `ein-bugs/wrong-answers/kw-neq-in-fact`, `ein-bugs/wrong-answers/kw-neq-in-goal-twin`, `ein-bugs/wrong-answers/kw-neq-in-goal`, `ein-bugs/wrong-answers/kw-or-in-hrule-twin`, `ein-bugs/wrong-answers/kw-or-in-hrule`, `ein-bugs/wrong-answers/kw-relation-fact-twin`, `ein-bugs/wrong-answers/kw-relation-fact`, `ein-bugs/wrong-answers/oblig-derived-domain-declared`, `ein-bugs/wrong-answers/oblig-derived-domain`, `ein-bugs/wrong-answers/oblig-fresh-name-declared`, `ein-bugs/wrong-answers/oblig-fresh-name`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/pair-core-root-derived`, `ein-bugs/wrong-answers/pair-core-root-given`, `ein-bugs/wrong-answers/pair-core-root-negative`, `ein-bugs/wrong-answers/parameter-nested-assert`, `ein-bugs/wrong-answers/tree-dead`, `ein-bugs/wrong-answers/wild-in-absent-twin`, `ein-bugs/wrong-answers/wild-in-head-twin`, `ein-bugs/wrong-answers/wild-in-hrule-twin`, `ein-bugs/wrong-answers/wild-in-match-twin`, `features/13_mixed_solution_and_open`, `lattice/11_fifo_gap_static`, `saturation/hypothesis-contradiction/coloc-disproved`, `saturation/hypothesis-contradiction/next-to-disproved`, `saturation/hypothesis-contradiction/right-of-disproved`, `tests/stdlib/bijection/06_blind_enumeration`
- `layer-whole`, `-n 1`: **156**, every monotone program but 3: `beyond_arith/send_more_money`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/tree-dead`
- `layer-whole`, `--minimal`: **126**, every monotone program but 33: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_no_solution`, `beyond_arith/equation_two_unknowns`, `beyond_arith/send_more_money`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/synthesis`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `ein-bugs/wrong-answers/oblig-derived-domain-declared`, `ein-bugs/wrong-answers/oblig-derived-domain`, `ein-bugs/wrong-answers/oblig-fresh-name-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two`, `ein-bugs/wrong-answers/oblig-fresh-name`, `ein-bugs/wrong-answers/owed-seats-renamed`, `ein-bugs/wrong-answers/owed-seats`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/tree-dead`, `ein-bugs/wrong-answers/uncovered-knows`, `ein-bugs/wrong-answers/uncovered`, `features/13_mixed_solution_and_open`, `lattice/05_owed_seats_bounded`, `lattice/06_friends_tables_everyone_sits`, `lattice/07_two_pairs_two_tables`, `lattice/synth/merged_states`, `lattice/synth/trivial_line`, `tests/region/02_merged_single`, `tests/stdlib/bool/inputs/01_in_drives`, `tests/stdlib/bool/inputs/02_required`; and in facts alone 2: `lattice/synth/merged_states`, `tests/region/02_merged_single`
- `layer-whole`, `--maximal`: **78**: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_no_solution`, `beyond_arith/equation_two_unknowns`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `branching/10_kill_cache_on`, `branching/11_kill_cache_off`, `branching/15_lookahead_two_step_on`, `branching/16_lookahead_two_step_off`, `ein-bugs/binding-key-both`, `ein-bugs/binding-key-nested-only`, `ein-bugs/binding-key-swapped`, `ein-bugs/d3-goal-row-order`, `ein-bugs/mixed-type-hypothesis`, `ein-bugs/nested-fact-hypothesis`, `ein-bugs/wrong-answers/activator-nested-value`, `ein-bugs/wrong-answers/arity3`, `ein-bugs/wrong-answers/count-past-2-128`, `ein-bugs/wrong-answers/count-past-2-64`, `ein-bugs/wrong-answers/empty-match-fact`, `ein-bugs/wrong-answers/form-in-guard`, `ein-bugs/wrong-answers/guard-before-binder`, `ein-bugs/wrong-answers/h-long-relation`, `ein-bugs/wrong-answers/import-directives-as`, `ein-bugs/wrong-answers/import-directives-twice`, `ein-bugs/wrong-answers/kw-and-in-match-twin`, `ein-bugs/wrong-answers/kw-and-in-match`, `ein-bugs/wrong-answers/kw-neq-in-match-twin`, `ein-bugs/wrong-answers/kw-neq-in-match`, `ein-bugs/wrong-answers/owed-seats-renamed`, `ein-bugs/wrong-answers/owed-seats`, `ein-bugs/wrong-answers/range-in-match`, `ein-bugs/wrong-answers/relvar-eq-premise`, `ein-bugs/wrong-answers/uncovered-knows`, `ein-bugs/wrong-answers/uncovered`, `features/02_star_in_identifiers`, `features/03_forall`, `lattice/01_subset_pruned`, `lattice/02_genuine_3set_death`, `lattice/03_state_hash_collision`, `lattice/05_owed_seats_bounded`, `lattice/06_friends_tables_everyone_sits`, `lattice/07_two_pairs_two_tables`, `lattice/synth/merged_states`, `lattice/synth/trivial_line`, `saturation/implies/org-chart`, `saturation/implies/parent-to-ancestor`, `saturation/implies/right-then-next`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/transitive/colocation-chain`, `saturation/transitive/mealtimes`, `saturation/transitive/taxonomy`, `saturation/type-exclusivity/colors`, `saturation/type-exclusivity/nationalities`, `saturation/type-exclusivity/pets`, `syntax/constraint-scopes`, `syntax/equality`, `syntax/unary-and-nary`, `zebra`, `zebra2-minus-15-obligations`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2`, `tests/region/01_states`, `tests/region/02_merged_single`, `tests/region/03_merged_or`, `tests/region/08_query_classes`, `tests/stdlib/bool/inputs/02_required`
- `layer-by-class`, describe and `--count`: none in saturations; and in facts alone 1: `ein-bugs/d3-goal-row-order`
- `layer-by-class`, `--single`: **110**, every monotone program but 49: `beyond_arith/send_more_money`, `branching/01_saturate_only`, `branching/12_typed_blind_solve`, `domain_elim/b_only`, `ein-bugs/alive-empty-phase1`, `ein-bugs/binding-key-int-only`, `ein-bugs/wrong-answers/assert-neq-w-x`, `ein-bugs/wrong-answers/core-activator-given`, `ein-bugs/wrong-answers/core-activator-root-derived`, `ein-bugs/wrong-answers/empty-match-activator`, `ein-bugs/wrong-answers/empty-match-hrule`, `ein-bugs/wrong-answers/forall-core-elim`, `ein-bugs/wrong-answers/forced-1-two`, `ein-bugs/wrong-answers/forced-1`, `ein-bugs/wrong-answers/forced-2-off`, `ein-bugs/wrong-answers/forced-2-two`, `ein-bugs/wrong-answers/forced-2`, `ein-bugs/wrong-answers/fresh-name-nospawn`, `ein-bugs/wrong-answers/hrule-reads-absent-not`, `ein-bugs/wrong-answers/hrule-reads-not`, `ein-bugs/wrong-answers/hrule-reads-unknown`, `ein-bugs/wrong-answers/kw-neq-in-fact-twin`, `ein-bugs/wrong-answers/kw-neq-in-fact`, `ein-bugs/wrong-answers/kw-neq-in-goal-twin`, `ein-bugs/wrong-answers/kw-neq-in-goal`, `ein-bugs/wrong-answers/kw-or-in-hrule-twin`, `ein-bugs/wrong-answers/kw-or-in-hrule`, `ein-bugs/wrong-answers/kw-relation-fact-twin`, `ein-bugs/wrong-answers/kw-relation-fact`, `ein-bugs/wrong-answers/oblig-derived-domain-declared`, `ein-bugs/wrong-answers/oblig-derived-domain`, `ein-bugs/wrong-answers/oblig-fresh-name-declared`, `ein-bugs/wrong-answers/oblig-fresh-name`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/pair-core-root-derived`, `ein-bugs/wrong-answers/pair-core-root-given`, `ein-bugs/wrong-answers/pair-core-root-negative`, `ein-bugs/wrong-answers/parameter-nested-assert`, `ein-bugs/wrong-answers/tree-dead`, `ein-bugs/wrong-answers/wild-in-absent-twin`, `ein-bugs/wrong-answers/wild-in-head-twin`, `ein-bugs/wrong-answers/wild-in-hrule-twin`, `ein-bugs/wrong-answers/wild-in-match-twin`, `features/13_mixed_solution_and_open`, `lattice/11_fifo_gap_static`, `saturation/hypothesis-contradiction/coloc-disproved`, `saturation/hypothesis-contradiction/next-to-disproved`, `saturation/hypothesis-contradiction/right-of-disproved`, `tests/stdlib/bijection/06_blind_enumeration`
- `layer-by-class`, `-n 1`: **156**, every monotone program but 3: `beyond_arith/send_more_money`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/tree-dead`
- `layer-by-class`, `--minimal`: **121**, every monotone program but 38: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_no_solution`, `beyond_arith/equation_two_unknowns`, `beyond_arith/send_more_money`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/synthesis`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `ein-bugs/wrong-answers/oblig-derived-domain-declared`, `ein-bugs/wrong-answers/oblig-derived-domain`, `ein-bugs/wrong-answers/oblig-fresh-name-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two`, `ein-bugs/wrong-answers/oblig-fresh-name`, `ein-bugs/wrong-answers/owed-seats-renamed`, `ein-bugs/wrong-answers/owed-seats`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/tree-dead`, `ein-bugs/wrong-answers/uncovered-knows`, `ein-bugs/wrong-answers/uncovered`, `features/13_mixed_solution_and_open`, `lattice/05_owed_seats_bounded`, `lattice/06_friends_tables_everyone_sits`, `lattice/07_two_pairs_two_tables`, `lattice/synth/merged_states`, `lattice/synth/trivial_line`, `zebra`, `zebra2-minus-15-obligations`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2`, `tests/region/02_merged_single`, `tests/stdlib/bool/inputs/01_in_drives`, `tests/stdlib/bool/inputs/02_required`; and in facts alone 2: `lattice/synth/merged_states`, `tests/region/02_merged_single`
- `layer-by-class`, `--maximal`: **76**: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_no_solution`, `beyond_arith/equation_two_unknowns`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `branching/10_kill_cache_on`, `branching/11_kill_cache_off`, `branching/15_lookahead_two_step_on`, `branching/16_lookahead_two_step_off`, `ein-bugs/binding-key-both`, `ein-bugs/binding-key-nested-only`, `ein-bugs/binding-key-swapped`, `ein-bugs/d3-goal-row-order`, `ein-bugs/mixed-type-hypothesis`, `ein-bugs/nested-fact-hypothesis`, `ein-bugs/wrong-answers/activator-nested-value`, `ein-bugs/wrong-answers/arity3`, `ein-bugs/wrong-answers/count-past-2-128`, `ein-bugs/wrong-answers/count-past-2-64`, `ein-bugs/wrong-answers/empty-match-fact`, `ein-bugs/wrong-answers/form-in-guard`, `ein-bugs/wrong-answers/guard-before-binder`, `ein-bugs/wrong-answers/h-long-relation`, `ein-bugs/wrong-answers/import-directives-as`, `ein-bugs/wrong-answers/import-directives-twice`, `ein-bugs/wrong-answers/kw-and-in-match-twin`, `ein-bugs/wrong-answers/kw-and-in-match`, `ein-bugs/wrong-answers/kw-neq-in-match-twin`, `ein-bugs/wrong-answers/kw-neq-in-match`, `ein-bugs/wrong-answers/owed-seats-renamed`, `ein-bugs/wrong-answers/owed-seats`, `ein-bugs/wrong-answers/range-in-match`, `ein-bugs/wrong-answers/relvar-eq-premise`, `ein-bugs/wrong-answers/uncovered-knows`, `ein-bugs/wrong-answers/uncovered`, `features/02_star_in_identifiers`, `features/03_forall`, `lattice/01_subset_pruned`, `lattice/02_genuine_3set_death`, `lattice/03_state_hash_collision`, `lattice/05_owed_seats_bounded`, `lattice/06_friends_tables_everyone_sits`, `lattice/07_two_pairs_two_tables`, `lattice/synth/merged_states`, `lattice/synth/trivial_line`, `saturation/implies/org-chart`, `saturation/implies/parent-to-ancestor`, `saturation/implies/right-then-next`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/transitive/colocation-chain`, `saturation/transitive/mealtimes`, `saturation/transitive/taxonomy`, `saturation/type-exclusivity/colors`, `saturation/type-exclusivity/nationalities`, `saturation/type-exclusivity/pets`, `syntax/constraint-scopes`, `syntax/equality`, `syntax/unary-and-nary`, `zebra`, `zebra2-obligations`, `zebra2`, `tests/region/01_states`, `tests/region/02_merged_single`, `tests/region/03_merged_or`, `tests/region/08_query_classes`, `tests/stdlib/bool/inputs/02_required`
- `singles`, `--single`: **103**, every monotone program but 56: `beyond_arith/cryptarithm`, `beyond_arith/equation_two_unknowns`, `beyond_arith/send_more_money`, `branching/01_saturate_only`, `branching/12_typed_blind_solve`, `domain_elim/b_only`, `ein-bugs/alive-empty-phase1`, `ein-bugs/binding-key-int-only`, `ein-bugs/wrong-answers/assert-neq-w-x`, `ein-bugs/wrong-answers/core-activator-given`, `ein-bugs/wrong-answers/core-activator-root-derived`, `ein-bugs/wrong-answers/empty-match-activator`, `ein-bugs/wrong-answers/empty-match-hrule`, `ein-bugs/wrong-answers/forall-core-elim`, `ein-bugs/wrong-answers/forced-1-two`, `ein-bugs/wrong-answers/forced-1`, `ein-bugs/wrong-answers/forced-2-off`, `ein-bugs/wrong-answers/forced-2-two`, `ein-bugs/wrong-answers/forced-2`, `ein-bugs/wrong-answers/fresh-name-nospawn`, `ein-bugs/wrong-answers/hrule-reads-absent-not`, `ein-bugs/wrong-answers/hrule-reads-not`, `ein-bugs/wrong-answers/hrule-reads-unknown`, `ein-bugs/wrong-answers/kw-neq-in-fact-twin`, `ein-bugs/wrong-answers/kw-neq-in-fact`, `ein-bugs/wrong-answers/kw-neq-in-goal-twin`, `ein-bugs/wrong-answers/kw-neq-in-goal`, `ein-bugs/wrong-answers/kw-or-in-hrule-twin`, `ein-bugs/wrong-answers/kw-or-in-hrule`, `ein-bugs/wrong-answers/kw-relation-fact-twin`, `ein-bugs/wrong-answers/kw-relation-fact`, `ein-bugs/wrong-answers/oblig-derived-domain-declared`, `ein-bugs/wrong-answers/oblig-derived-domain`, `ein-bugs/wrong-answers/oblig-fresh-name-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two`, `ein-bugs/wrong-answers/oblig-fresh-name`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/pair-core-root-derived`, `ein-bugs/wrong-answers/pair-core-root-given`, `ein-bugs/wrong-answers/pair-core-root-negative`, `ein-bugs/wrong-answers/parameter-nested-assert`, `ein-bugs/wrong-answers/tree-dead`, `ein-bugs/wrong-answers/wild-in-absent-twin`, `ein-bugs/wrong-answers/wild-in-head-twin`, `ein-bugs/wrong-answers/wild-in-hrule-twin`, `ein-bugs/wrong-answers/wild-in-match-twin`, `features/13_mixed_solution_and_open`, `lattice/06_friends_tables_everyone_sits`, `lattice/11_fifo_gap_static`, `saturation/hypothesis-contradiction/coloc-disproved`, `saturation/hypothesis-contradiction/next-to-disproved`, `saturation/hypothesis-contradiction/right-of-disproved`, `word_problems/boxes12`, `word_problems/boxes13`, `tests/stdlib/bijection/06_blind_enumeration`
- `singles`, `-n 1`: **152**, every monotone program but 7: `beyond_arith/equation_two_unknowns`, `beyond_arith/send_more_money`, `ein-bugs/wrong-answers/oblig-fresh-name-two-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/tree-dead`, `lattice/06_friends_tables_everyone_sits`
- `singles`, `--minimal`: **118**, every monotone program but 41: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_no_solution`, `beyond_arith/equation_two_unknowns`, `beyond_arith/send_more_money`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/synthesis`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `ein-bugs/wrong-answers/oblig-derived-domain-declared`, `ein-bugs/wrong-answers/oblig-derived-domain`, `ein-bugs/wrong-answers/oblig-fresh-name-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two-declared`, `ein-bugs/wrong-answers/oblig-fresh-name-two`, `ein-bugs/wrong-answers/oblig-fresh-name`, `ein-bugs/wrong-answers/owed-seats-renamed`, `ein-bugs/wrong-answers/owed-seats`, `ein-bugs/wrong-answers/owed-two-step`, `ein-bugs/wrong-answers/tree-dead`, `ein-bugs/wrong-answers/uncovered-knows`, `ein-bugs/wrong-answers/uncovered`, `features/13_mixed_solution_and_open`, `lattice/05_owed_seats_bounded`, `lattice/06_friends_tables_everyone_sits`, `lattice/07_two_pairs_two_tables`, `lattice/synth/merged_states`, `lattice/synth/one_model_per_component`, `lattice/synth/trivial_line`, `word_problems/boxes12`, `word_problems/boxes13`, `zebra`, `zebra2-minus-15-obligations`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2`, `tests/region/02_merged_single`, `tests/stdlib/bool/inputs/01_in_drives`, `tests/stdlib/bool/inputs/02_required`
- `singles`, `--maximal`: **72**: `beyond_arith/equation_by_search`, `beyond_arith/equation_no_solution`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `branching/15_lookahead_two_step_on`, `branching/16_lookahead_two_step_off`, `ein-bugs/binding-key-both`, `ein-bugs/binding-key-nested-only`, `ein-bugs/binding-key-swapped`, `ein-bugs/complete-records-stale`, `ein-bugs/d3-goal-row-order`, `ein-bugs/mixed-type-hypothesis`, `ein-bugs/nested-fact-hypothesis`, `ein-bugs/wrong-answers/activator-nested-value`, `ein-bugs/wrong-answers/arity3`, `ein-bugs/wrong-answers/count-past-2-128`, `ein-bugs/wrong-answers/count-past-2-64`, `ein-bugs/wrong-answers/empty-match-fact`, `ein-bugs/wrong-answers/form-in-guard`, `ein-bugs/wrong-answers/guard-before-binder`, `ein-bugs/wrong-answers/h-long-relation`, `ein-bugs/wrong-answers/import-directives-as`, `ein-bugs/wrong-answers/import-directives-twice`, `ein-bugs/wrong-answers/kw-and-in-match-twin`, `ein-bugs/wrong-answers/kw-and-in-match`, `ein-bugs/wrong-answers/kw-neq-in-match-twin`, `ein-bugs/wrong-answers/kw-neq-in-match`, `ein-bugs/wrong-answers/owed-seats-renamed`, `ein-bugs/wrong-answers/owed-seats`, `ein-bugs/wrong-answers/range-in-match`, `ein-bugs/wrong-answers/relvar-eq-premise`, `ein-bugs/wrong-answers/uncovered-knows`, `ein-bugs/wrong-answers/uncovered`, `features/02_star_in_identifiers`, `features/03_forall`, `features/11_expect_ambiguity`, `lattice/01_subset_pruned`, `lattice/02_genuine_3set_death`, `lattice/03_state_hash_collision`, `lattice/05_owed_seats_bounded`, `lattice/07_two_pairs_two_tables`, `lattice/synth/merged_states`, `lattice/synth/trivial_line`, `saturation/implies/org-chart`, `saturation/implies/parent-to-ancestor`, `saturation/implies/right-then-next`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/transitive/colocation-chain`, `saturation/transitive/mealtimes`, `saturation/transitive/taxonomy`, `syntax/constraint-scopes`, `syntax/equality`, `syntax/unary-and-nary`, `zebra`, `zebra2-obligations`, `zebra2`, `tests/region/01_states`, `tests/region/02_merged_single`, `tests/region/03_merged_or`, `tests/region/08_query_classes`, `tests/region/09_query_over_maximal`, `tests/stdlib/bool/inputs/01_in_drives`, `tests/stdlib/bool/inputs/02_required`
- `layer-by-need`, describe and `--count`: **2**: `branching/02_one_dead_one_alive`, `branching/08_hypothesis_relation_whitelist`; and in facts alone 1: `ein-bugs/d3-goal-row-order`
- `layer-by-need`, `--single`: **15**: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_two_unknowns`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `lattice/07_two_pairs_two_tables`, `zebra`, `zebra2-minus-15-obligations`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2`
- `layer-by-need`, `-n 1`: **14**: `beyond_arith/equation_by_search`, `beyond_arith/equation_two_unknowns`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `lattice/07_two_pairs_two_tables`, `zebra`, `zebra2-minus-15-obligations`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2`
- `layer-by-need`, `--maximal`: **46**: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_two_unknowns`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/truth_table`, `binary_logic/unsatisfiable`, `ein-bugs/d3-goal-row-order`, `ein-bugs/wrong-answers/form-in-guard`, `ein-bugs/wrong-answers/guard-before-binder`, `ein-bugs/wrong-answers/kw-and-in-match-twin`, `ein-bugs/wrong-answers/kw-and-in-match`, `ein-bugs/wrong-answers/kw-neq-in-match-twin`, `ein-bugs/wrong-answers/kw-neq-in-match`, `ein-bugs/wrong-answers/owed-seats-renamed`, `ein-bugs/wrong-answers/owed-seats`, `ein-bugs/wrong-answers/range-in-match`, `ein-bugs/wrong-answers/uncovered-knows`, `ein-bugs/wrong-answers/uncovered`, `features/02_star_in_identifiers`, `features/03_forall`, `lattice/01_subset_pruned`, `lattice/02_genuine_3set_death`, `lattice/07_two_pairs_two_tables`, `lattice/synth/merged_states`, `lattice/synth/trivial_line`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/type-exclusivity/colors`, `saturation/type-exclusivity/nationalities`, `saturation/type-exclusivity/pets`, `syntax/constraint-scopes`, `syntax/equality`, `syntax/unary-and-nary`, `zebra`, `zebra2-obligations`, `zebra2`, `tests/region/01_states`, `tests/region/02_merged_single`, `tests/region/08_query_classes`
- `death-pairs`, `--single`: **14**: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_two_unknowns`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/unsatisfiable`, `lattice/07_two_pairs_two_tables`, `zebra`, `zebra2-minus-15-obligations`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2`
- `death-pairs`, `-n 1`: **12**: `beyond_arith/equation_by_search`, `beyond_arith/equation_two_unknowns`, `binary_logic/equivalent`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/unsatisfiable`, `lattice/07_two_pairs_two_tables`, `zebra`, `zebra2-minus-15-obligations`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2`
- `death-pairs`, `--maximal`: **17**: `beyond_arith/cryptarithm`, `beyond_arith/equation_by_search`, `beyond_arith/equation_two_unknowns`, `binary_logic/equivalent`, `binary_logic/knights`, `binary_logic/not_equivalent`, `binary_logic/satisfiable`, `binary_logic/unsatisfiable`, `lattice/02_genuine_3set_death`, `saturation/type-exclusivity/colors`, `saturation/type-exclusivity/nationalities`, `zebra`, `zebra2-obligations`, `zebra2`, `tests/region/01_states`, `tests/region/08_query_classes`, `tests/stdlib/bool/inputs/01_in_drives`
- `theory`, describe and `--count`: **147**, every monotone program but 12: `beyond_arith/send_more_money`, `features/02_star_in_identifiers`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/type-exclusivity/pets`, `syntax/constraint-scopes`, `syntax/equality`, `tests/stdlib/bijection/06_blind_enumeration`; and in facts alone 1: `tests/stdlib/bijection/06_blind_enumeration`
- `theory`, `--single`: **157**, every monotone program but 2: `beyond_arith/send_more_money`, `tests/stdlib/bijection/06_blind_enumeration`; and in facts alone 1: `tests/stdlib/bijection/06_blind_enumeration`
- `theory`, `-n 1`: **158**, every monotone program but 1: `beyond_arith/send_more_money`
- `theory`, `--minimal`: **157**, every monotone program but 2: `beyond_arith/send_more_money`, `lattice/06_friends_tables_everyone_sits`
- `theory`, `--maximal`: **146**, every monotone program but 13: `beyond_arith/send_more_money`, `branching/12_typed_blind_solve`, `ein-bugs/alive-set-fresh-name-declared`, `ein-bugs/alive-set-fresh-name`, `ein-bugs/wrong-answers/fresh-name-default`, `ein-bugs/wrong-answers/match-only-declared`, `ein-bugs/wrong-answers/match-only`, `ein-bugs/wrong-answers/oblig-derived-domain-declared`, `ein-bugs/wrong-answers/oblig-derived-domain`, `ein-bugs/wrong-answers/oblig-fresh-name-declared`, `ein-bugs/wrong-answers/oblig-fresh-name`, `features/05_stdlib_domain_elim`, `tests/stdlib/bijection/06_blind_enumeration`; and in facts alone 1: `tests/stdlib/bijection/06_blind_enumeration`
- `cover`, describe and `--count`: **148**, every monotone program but 11: `beyond_arith/send_more_money`, `features/02_star_in_identifiers`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/type-exclusivity/pets`, `syntax/constraint-scopes`, `syntax/equality`
- `cover`, `--single` and `-n 1` and `--maximal`: **158**, every monotone program but 1: `beyond_arith/send_more_money`
- `cover`, `--minimal`: **157**, every monotone program but 2: `beyond_arith/send_more_money`, `lattice/06_friends_tables_everyone_sits`
- `wipe-out`, describe and `--count`: **148**, every monotone program but 11: `beyond_arith/send_more_money`, `features/02_star_in_identifiers`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/type-exclusivity/pets`, `syntax/constraint-scopes`, `syntax/equality`
- `wipe-out`, `--single` and `-n 1` and `--maximal`: **158**, every monotone program but 1: `beyond_arith/send_more_money`
- `wipe-out`, `--minimal`: **157**, every monotone program but 2: `beyond_arith/send_more_money`, `lattice/06_friends_tables_everyone_sits`
- `wipe-out-layer`, describe and `--count`: **148**, every monotone program but 11: `beyond_arith/send_more_money`, `features/02_star_in_identifiers`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/type-exclusivity/pets`, `syntax/constraint-scopes`, `syntax/equality`
- `wipe-out-layer`, `--single` and `-n 1` and `--maximal`: **158**, every monotone program but 1: `beyond_arith/send_more_money`
- `wipe-out-layer`, `--minimal`: **157**, every monotone program but 2: `beyond_arith/send_more_money`, `lattice/06_friends_tables_everyone_sits`
- `factor`, describe and `--count`: **143**, every monotone program but 16: `beyond_arith/send_more_money`, `ein-bugs/wrong-answers/form-in-guard`, `ein-bugs/wrong-answers/guard-before-binder`, `features/02_star_in_identifiers`, `lattice/synth/trivial_line`, `saturation/square-bwd/floors`, `saturation/square-bwd/houses`, `saturation/square-bwd/meetings`, `saturation/square-fwd/floors`, `saturation/square-fwd/houses`, `saturation/square-fwd/meetings`, `saturation/type-exclusivity/colors`, `saturation/type-exclusivity/nationalities`, `saturation/type-exclusivity/pets`, `syntax/constraint-scopes`, `syntax/equality`
- `factor`, `--single` and `-n 1` and `--maximal`: **159**, every monotone program
- `factor`, `--minimal`: **158**, every monotone program but 1: `beyond_arith/send_more_money`
