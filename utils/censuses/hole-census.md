# The hole census — 2026-11-25

**S1e2.1b.3's measurement** (the stage), re-taken, the evidence
for [F21](../../plans/m1n_non_monotone_programs/f21_non_monotone_programs_decided.md)'s D4, which is
[Q-M1e2.7](../../docs/decisions/answer/Q-M1e2.7-the-answer-is-a-region-r-of-the-hypothesis.md)'s
sub-question (d): *what is reported when R has holes?* It lists every hole of
every flagged cube the oracle reaches, by kind and by cause, each with the
site that made it, and R's shape around its holes. It decides nothing.

**What holds today.** A flagged program loads, every command answers it by the
cube under a `WARNING` that it is not monotone, and nothing is claimed of its
answer
([Q-M1f.564](../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)):
no program is refused for an `absent`. The census reads the cube the oracle
computes, which is the R every command prints of such a program. Of the four
ways to report holes the stage drafted, the fourth, a refusal, is ruled out by
that decision ("a refusal would say that such a program is an error, which is
not known"), and is no longer drafted (§ 9, § 10).

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census                    # § 1–6, the flagged cubes, 11 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census -- --written       # § 2, the programs in shared/written.rs, 0.6 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census -- \
    plans/m1n_non_monotone_programs/f21_probes/*.ein                                                                          # S1e2.4.1's probes: none loads today (below)
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census -- \
    --limit 19 examples/lattice/06_friends_tables.ein                                                         # § 7, 266 s at 16 jobs
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example site_census -- FILE…           # § 8, the selection less tests/saved/04 (below)
```

Every command takes `--json` for one row per program, and `--no-scratch` to
skip § 6. Taken 2026-11-25 at `213e8a92`, release build, stdlib = the
checkout, 16 jobs on a 32-thread machine, each command through
`utils/bench_env.sh --cores 0-31` on AC. Every number on this page is from
those commands unless it names another.

**What it selects.** `site_census`'s selection, `corpus_files()` plus the
regressions' witnesses: **933 files**. 185 do not load, every one a fixture or
a regression witness whose point is a parse or a load error
(`examples/broken/` 62, `examples/ein-bugs/` 123). Of the 748 that load and
saturate the static check calls 689 monotone and flags **59**: **52** within
`--region-limit 16`, which is [`node_census.md`](node-census.md)'s selection,
and 7 past it (`features/01`, `features/04`, `lattice/06`,
`wrong-answers/past-limit-lines` and the three `square-unique` programs).
**S1e2.4.1's 14 probes are not re-taken**: every one is written in a form the
parser refuses today (`hrule`, `:priority`, a relation or rule with no
`:why`), so none loads; they are F21's, and no census rewrites them. The two
programs in
[`examples/shared/written.rs`](../../ein.rs/crates/ein-infer/examples/shared/written.rs)
get their own run: `ACTIVATOR_REFUTES`, which `ein-infer/tests/region.rs`
reads from there, and `ACTIVATOR_UNDER_ABSENT`, written for this census (§ 2).

**What it reads.** [`hole_census`](../../ein.rs/crates/ein-infer/examples/hole_census.rs)
runs `region::compute` and reads `Topology` over the status array:
`holes`, `minimal_holes`, `repairs`, `local_max` and `local_min`,
`components`, `subcubes`, the two lines and the duality. A hole's **kind** is
the bit that puts it out of R, `dead` before `owing`. Its **cause** and
**site** come from
[`examples/shared/attribution.rs`](../../ein.rs/crates/ein-infer/examples/shared/attribution.rs),
which reads the oracle's own node: a dead node by `refuted_by` and its
`absent` premises, an owing one by the instances its fixpoint owes and the
largest node of R below it does not (`region::Nodes`, the node function every
route shares). The **convex pieces** and the **scratch** statuses are the
census's own (§ 5, § 6). No route reads any of it.

---

## 1. The headline

| | count |
|---|---:|
| flagged cubes within the limit | 52 |
| … with a hole | **16** |
| holes | **25**: 14 dead, 11 owing |
| … minimal | 19 |
| … dead and owing at once | 0 |
| … no site named | **6**, every one through a chain of activators (§ 2) |
| causes seen in the corpus | 5 of the 6 (§ 2) |
| S1e2.4.1's probes | not re-taken: none of the 14 loads today |
| the programs in `written.rs` | 2, one dead hole each, both *an activator that compiles a refutation* |
| `lattice/06_friends_tables` at `--limit 19` | **166 holes**, all owing, 18 minimal; R is 21 isolated states |
| a hole whose status differs by fork and scratch | **0**, on every program above |

## 2. By cause and kind

The six causes of the stage, each as
`attribution.rs` recognises it, and the holes it names no site for:

| cause | kind | holes: corpus (programs) | written | `06` at 19 | the rule that names the site |
|---|---|---:|---:|---:|---|
| an `absent` that concludes `(false)` | dead | **8** (4: `naf-upward-closure` 1, `naf-nowb` 1, `synth/three_components` 3, `synth/trigger_hole` 3) | 0 | 0 | an `absent` site whose rule refuted the node, over a premise's relation |
| a refutation through a fact derived under an `absent` (`via`) | dead | 0 | 0 | 0 | an `absent` site over a premise's relation, the refuting rule reading none |
| an activator that compiles a refutation | dead | **1** (`wrong-answers/activator-chain-0`) | **2** | 0 | the activator site of a refuting rule with no plan at root |
| an `absent` that takes a witness away | owing | **1** (`lattice/08`) | 0 | 0 | an `absent` site whose rule derived a fact the node of R below holds and the hole does not |
| an obligation's trigger | owing | **8** (4: `lattice/04` 1, `lattice/17_flagged_region_claim` 1, `synth/local_not_global` 2, `synth/trigger_hole` 4) | 0 | **166** | the trigger site of an instance owed in the hole and not below |
| an activator that compiles an obligation | owing | **1** (`lattice/09`) | 0 | 0 | the activator site of an owed instance whose plan root never compiled |
| **no site named** | dead | **5** (`wrong-answers/activator-chain-1`, `-2`, `-node-absent`, `-node-writes`, `-param-absent`) | 0 | 0 | — |
| **no site named** | owing | **1** (`lattice/12_activator_chain_hole`) | 0 | 0 | — |

- **Every hole has at most one cause.** No hole is dead and owing at once,
  and no hole owes two instances with different causes.
- **The six holes with no site are one shape: a chain of activators.** In
  `activator-chain-1`, `mk` derives `(q a)` under `(absent (p2 a))`, which
  activates `q`; `q` derives `(r2 a)`, which activates `r2`; and `r2`
  refutes. The static check names `mk`'s `absent` and `q`'s activator as
  sites; the rule that refutes is `r2`, whose activator is no site, and
  `attribution.rs`'s activator clause looks only at the refuting rule's own
  activator. `activator-chain-0`, where the site's rule refutes itself, is
  attributed. `lattice/12` is the owing twin: `serves` derives the activator
  of `total-owed`, which owes in the hole, and the site named is `serves`'s.
  The attribution gap § 8 closed is open again for chains longer than one.
- **The one dead cause no corpus program shows** is `via`. It was
  S1e2.4.1's probe `via_hole`, which does not load today.
- **`ACTIVATOR_UNDER_ABSENT`**, written for this census. `act` writes the
  activator `(watch r)` while `(q ?x)` is absent, which compiles `watch` for
  `r`, and `(r A)` then refutes. Its dead hole `{(p A), (r A)}` has
  `absent_premises` empty: a derivation does not record the activator its
  plan was compiled from, so the `absent` the death rests on is in no
  premise. The site rule names the activator site from `refuted_by`, not from
  the premises (its core reads `(p A) (r A)` today).
- **No row is refused.** The stage set each row against *forbid*'s refusal;
  that refusal is gone (Q-M1f.564), and every program above loads, flagged.

## 3. Per program

The 16 flagged corpus cubes within the limit with a hole, the written
programs, and `06` at 19. *Lines* is the minimal line's and the maximal
line's members, and *between* is how many nodes lie between them: R and its
holes, since a hole is exactly a node outside R between two members.
*Pieces* is § 5's.

| program | n | \|R\| | holes | dead | owing | minimal | repair edges | one-fact test wrong: max / min | components | sub-cubes | lines | between | pieces (states in two or more) |
|---|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---|---:|---|
| `ein-bugs/naf-upward-closure` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `ein-bugs/wrong-answers/activator-chain-0` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `ein-bugs/wrong-answers/activator-chain-1` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `ein-bugs/wrong-answers/activator-chain-2` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `ein-bugs/wrong-answers/activator-chain-node-absent` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `ein-bugs/wrong-answers/activator-chain-node-writes` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `ein-bugs/wrong-answers/activator-chain-param-absent` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `ein-bugs/wrong-answers/naf-nowb` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `lattice/04_obligation_trigger_hole` | 2 | 3 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `lattice/08_absent_witness_hole` | 4 | 13 | 1 | 0 | 1 | 1 | 2 | 0 / 0 | 1 | 3 | 3 + 1 | 14 | 2 (9) |
| `lattice/09_activator_hole` | 2 | 3 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `lattice/12_activator_chain_hole` | 2 | 3 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `lattice/17_flagged_region_claim` | 2 | 3 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `lattice/synth/local_not_global` | 2 | 2 | 2 | 0 | 2 | 2 | 2 | **1 / 1** | 2 | 2 | 1 + 1 | 4 | 2 (0) |
| `lattice/synth/three_components` | 7 | 13 | 3 | 3 | 0 | 2 | 3 | 0 / 0 | 3 | 5 | 3 + 3 | 16 | 6 (5) |
| `lattice/synth/trigger_hole` | 5 | 9 | 7 | 3 | 4 | 2 | 6 | 0 / 0 | 1 | 4 | 1 + 1 | 16 | 6 (7) |
| written `activator_refutes` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| written `activator_under_absent` | 3 | 7 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 3 | 1 + 1 | 8 | 2 (3) |
| `lattice/06_friends_tables`, `--limit 19` | 19 | 21 | 166 | 0 | 166 | 18 | 68 | **7 / 20** | 21 | 21 | 1 + 14 | 187 | 21 (0) |

`lattice/08` has four candidates today, where it had three: the union of the
hypotheses' writers (Q-M1f.570) adds `(meal Ann Soup)` beside `(meal Ann
Bread)`, so R is 13 states and its hole two repair edges.

The other 36 flagged corpus cubes within the limit have no hole. On 35 of them
the duality holds both ways and R is one convex piece or empty; the 36th is
`synth/maximal_not_top`, whose violation is a discharged node with an owing
superset and no node of R above it, and whose maximal duality fails. On all
36 the one-fact test agrees with the cube and the lines describe R exactly.
Eight of them have an empty R (root dead, or every node dead or owing):
`activator-all-int`, `-nested`, `-symbol`, `rung-connex-import`, and four of
the `oblig-*` witnesses.

## 4. Each minimal hole

With why the oracle puts it out (its core, or what it owes), its cause, the
site named, and its repair edges: a fact whose addition takes it into R.

| program | minimal hole | kind | why | cause | site | repaired by |
|---|---|---|---|---|---|---|
| `ein-bugs/naf-upward-closure` | `{(p A)}` | dead | core `(p A)` | `absent` → `(false)` | `bad` | +`(q A)` |
| `wrong-answers/activator-chain-0` | `{(p a)}` | dead | core `(p a) (seed a)` | activator → refutation | `q` (activator) | +`(p2 a)` |
| `wrong-answers/activator-chain-1`, `-2`, `-node-absent`, `-param-absent` | `{(p a)}` | dead | core `(p a) (seed a)` | — | **none** | +`(p2 a)` |
| `wrong-answers/activator-chain-node-writes` | `{(p a)}` | dead | core `(p a)` | — | **none** | +`(p2 a)` |
| `wrong-answers/naf-nowb` | `{(p A)}` | dead | core `(p A)` | `absent` → `(false)` | `bad` | +`(q A)` |
| `lattice/04_obligation_trigger_hole` | `{(seats Ann C1)}` | owing | owes 1: *meal owes Ann a Food.* | trigger | `total-owed [activator meal is-a]` | +`(meal Ann Soup)` |
| `lattice/08_absent_witness_hole` | `{(allergic Ann), (seats Ann C1)}` | owing | owes 1: *meal owes Ann a Food.* | `absent` takes a witness | `soup-unless-allergic` | +`(meal Ann Bread)`, +`(meal Ann Soup)` |
| `lattice/09_activator_hole` | `{(opens K1)}` | owing | owes 1: *meal owes Ann a Food.* | activator → obligation | `total-owed` (activator) | +`(meal Ann Soup)` |
| `lattice/12_activator_chain_hole` | `{(opens K1)}` | owing | owes 1: *meal owes Ann a Food.* | — | **none** | +`(meal Ann Soup)` |
| `lattice/17_flagged_region_claim` | `{(seats Ann C1)}` | owing | owes 1: *meal owes Ann a Food.* | trigger | `total-owed [activator meal is-a]` | +`(meal Ann Soup)` |
| `lattice/synth/local_not_global` | `{(h A)}` | owing | owes 1: *with A: one of B is required* | trigger | `owe-1` | +`(h B)` |
| ″ | `{(h B)}` | owing | owes 1: *with B: one of A is required* | trigger | `owe-2` | +`(h A)` |
| `lattice/synth/three_components` | `{(h A), (h F)}` | dead | core `(h A) (h F)` | `absent` → `(false)` | `dead-3` | +`(h B)` |
| ″ | `{(h A), (h G)}` | dead | core `(h A) (h G)` | `absent` → `(false)` | `dead-7` | +`(h D)` |
| `lattice/synth/trigger_hole` | `{(h A), (h B)}` | owing | owes 1: *with B: one of C is required* | trigger | `owe-2` | +`(h C)` |
| ″ | `{(h A), (h D)}` | dead | core `(h A) (h D)` | `absent` → `(false)` | `dead-1` | +`(h E)` |
| written `activator_refutes` | `{(p A)}` | dead | core `(p A) (r A)` | activator → refutation | `watch` (activator) | +`(q A)` |
| written `activator_under_absent` | `{(p A), (r A)}` | dead | core `(p A) (r A)` | activator → refutation | `watch` (activator) | +`(q A)` |

`06`'s 18 are in § 7. Every site named is one the program was written to
show; the six with none are § 2's chains. On `activator_under_absent` the
site is the activator, `watch`; the `absent` its activator rests on is
`act`'s, which the static check also reports as a site, and which no premise
of the refutation records (§ 2).

**A dead hole's core does not say why it is dead.** `naf-upward-closure`'s
core is `(p A)`, and the `absent` it rests on, `(absent (q A))`, is in the
node's `absent_premises`, which the read-out does not print. On the two
written programs the core is `(p A) (r A)`; what kills the node is the plan
the hole compiles. The read-out's `minimal hole … — dead: (p A)` row prints the
core only.

## 5. R's shape around its holes

**The lines.** On every program with a hole, the minimal and maximal lines
describe R *and its holes*: the nodes between them are R plus exactly the
holes (the *between* column of § 3). So two lines over-state R by its holes,
1 node of 4 on `naf-upward-closure`, 7 of 16 on `trigger_hole`, **166 of 187**
on `06`. They are R's own minimal and maximal states either way: no hole is
on a line.

**The one-fact test.** `Topology`'s `local_max` and `local_min` list the nodes
of R the one-fact test classifies differently from the cube. They are empty
on every program with a hole within the limit except
`synth/local_not_global` (`∅` locally maximal, `{(h A), (h B)}` locally
minimal, which `region.rs::across_a_hole_the_one_fact_test_misreads_the_lines`
holds). On `06` at 19 it misreads **7 maximal** and **20 minimal**: `∅` and
the six two- and three-seat groupings that are maximal only locally, and every
node of R but `∅`, each of whose one-fact-down neighbours is a hole. Across a
hole the one-fact test sees the hole and not the node of R beyond it.

**The duality.** The N/E duality's maximal side fails on every program with a
dead hole (`naf-upward-closure`, `naf-nowb`, the six `activator-chain`
witnesses, `three_components`, `trigger_hole`, both written programs) and on
`06` and `maximal_not_top`; it holds on `lattice/04`, `08`, `09`, `12`, `17`
and `local_not_global`. That is
[`08-monotonicity.md` § 6](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#6-the-two-kinds-compared)'s
row, measured over more programs.

**Repair edges.** One per hole and missing fact, from the hole to a node of R.
Every minimal hole within the limit has at least one. On `06` 8 of the 18
minimal holes have **none**: a trio member seated alone owes two seats, and no
one fact pays both (§ 7).

**The convex pieces.** A **convex piece** is a subset of R that is convex
(every node between two members is a member) and connected by one-fact edges,
and maximal among such subsets. The census computes them per component of R:
a component with no hole between two of its members is one piece; any other
is split into its maximal convex connected subsets, by enumeration (up to 16
members). **They are not a partition.** Where a hole sits between two members
of a component, a member next to it can belong to two pieces:

| program | components | pieces | states in two or more | line members, the two lines | line members, the pieces |
|---|---:|---:|---:|---:|---:|
| `naf-upward-closure` (and the 12 others of 3 states) | 1 | 2 | 1 | 2 | 4 |
| `lattice/08` | 1 | 2 | 9 | 4 | 8 |
| `local_not_global` | 2 | 2 | 0 | 2 | 4 |
| `three_components` | 3 | 6 | 5 | 6 | 14 |
| `trigger_hole` | 1 | 6 | 7 | 2 | 18 |
| `06` at 19 | 21 | 21 | 0 | 15 | 42 |

On `naf-upward-closure` the pieces are `[∅ … {(q A)}]` and `[{(q A)} …
{(p A), (q A)}]`, which share `{(q A)}`. On `lattice/08` the two pieces, of
10 and 12 states, share 9. A partition would need a choice of which piece
keeps a shared state, which no rule here makes. On `06` every state is its
own component, so every piece is one state and lists it twice.

## 6. Fork and scratch

Every node of every cube above is built a second time from scratch, the
program's text with the commitment as given facts (`utils/region.py`'s
node), on 16 threads, and its two bits are held to the oracle's fork.

| run | cubes | nodes | status differs | holes the same |
|---|---:|---:|---:|---|
| the flagged corpus cubes | 52 | 656 | **17**, none a hole: `stale-naf-fork`'s and its `-warn` twin's `{(q one)}`; the one candidate's node of `activator-all-int`, `-nested` and `-symbol`; 12 of `rung-connex-import`'s 16 | yes, all 52 |
| `written.rs` | 2 | 12 | 0 | yes |
| `06` at 19 | 1 | 524 288 | 0 | yes, 260 s |

Each of the 17 is dead by fork and in R from scratch. On the four programs
besides W8's pair, **root is dead** on an `absent` a commitment fills: on
`activator-all-*` a chain of activators from `flag`'s `(absent (checked ?x))`
ends in `alarm`'s `(false)`, and on `rung-connex-import` `std.algebra`'s
`connex` refutes `(absent (lt A C))` beside `(absent (lt C A))`. Every fork of
a dead root is dead (L0); a commitment written in as a given fact blocks the
refutation. It is [`node_census.md` § 6.3](node-census.md#63-the-synthesizers-dead)'s
`dead_root` shape, met in the corpus.

So [`node_census.md` § 2.1](node-census.md#21-the-corpus)'s finding,
extended to the written programs and `06`, still holds: **on every program
with a hole, the holes are the same by fork and from scratch.** A hole here
needs no path dependence, as `08-monotonicity.md` argues for owing holes. Where
the routes differ in status the program has no hole: its R is empty by fork,
or one state.

## 7. Past the limit: `06_friends_tables` at 19

[`06_friends_tables.ein`](../../examples/lattice/06_friends_tables.ein),
the conditional obligation, *a group that sits owes its members' seats*. It is
the one flagged program in the 17–19 band, and a bare `ein solve` stops at the
budget with *holes not known: the cube was not enumerated*. At
`--limit 19` the oracle takes 5.3 s on 16 jobs and the scratch check 260 s,
where it took 23 s: a scratch node loads the program again, and the program
now carries `std.core` and its imports' own modules.

- **R**: 21 states, each its own component and its own sub-cube, one minimal
  line member (`∅`) and 14 maximal.
- **Holes: 166**, every one owing, every one by `group-seated-owed`'s trigger.
  **18 minimal**: each one-person seating, `{(sits Ann B1)}` and the like
  (`Eve` never sits at `B1`, so 5 + 5 + 3 + 3 + 2).
- **Repair edges: 68** in all, **16** from the minimal holes. A pair member
  seated alone has two (`+(sits Bob B2)`, `+(sits Bob B3)` from
  `{(sits Ann B1)}`). A trio member seated alone has none.
- **The lines describe 187 nodes**, of which 21 are R.
- **The one-fact test** misreads 7 maximal and 20 minimal (§ 5).
- **Pieces**: 21, one per state.

Every number is the stage's: the program's R and holes have not moved. What a
search past the limit can say about holes is F21's D7; what the read-out says
about them is its D4.

## 8. The attribution gap

S1e2.1b.6's re-take of the site census printed *a violation no site is named
for: 2* ([`08-monotonicity.md` § 4](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#4-what-the-cube-shows)).
`site_census` named a site for a dead node by its `absent` premises, and for
an owing node by the trigger sites of what it owes. The owing holes of
`lattice/08` and `lattice/09` fit neither rule: `lattice/08` owes because an
`absent` took its witness away, `lattice/09` through an obligation a node
compiled.

Since T1e2.1b.3.2 `site_census` reads the rule this census reads,
[`examples/shared/attribution.rs`](../../ein.rs/crates/ein-infer/examples/shared/attribution.rs),
and prints each item's cause beside its sites. The rule grew three clauses
over S1e2.1b.1's:

- **An `absent` that takes a witness away.** An owing node whose new debt is
  not a trigger site's and whose plan root compiled is attributed to the
  `absent` site whose rule derived a fact the node of R below it holds and it
  does not. On `lattice/08` that is `(meal Ann Soup)`, by
  `soup-unless-allergic`, held at `{(seats Ann C1)}` and lost at `{(allergic
  Ann), (seats Ann C1)}`.
- **An activator that compiles an obligation.** An owing node whose new
  debt's `(rule, activator)` has no plan at root is attributed to that rule's
  activator site. On `lattice/09` that is `total-owed` for `meal is-a`, which
  only `{(opens K1)}` compiles.
- **An activator that compiles a refutation.** A dead node refuted by a rule
  with no plan at root is attributed to that rule's activator site, read from
  `refuted_by`. It is the one clause that does not read the node's `absent`
  premises, because on `ACTIVATOR_UNDER_ABSENT` there are none (§ 2).

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example site_census -- FILE…   # the selection less tests/saved/04, 70 s on a loaded machine
```

**The re-take, 2026-11-25 at `213e8a92`.** `site_census` with no file
**panics** on `tests/saved/04_a_saved_node.ein`, a saved node of its
program's lattice (Q-M1f.402): its second load, the oracle's, commits the
node's choice and has no candidate where the first has one (*two loads, two
universes*). Run over the other 932 files of the selection, it reads: 747 load
and saturate; **59 flagged**, 52 within the limit (17 exercised, 35
conservative) and 7 past it; **84 sites** (34 `absent`, 21 activator, 11
obligation guard, 10 obligation trigger, 6 hypothesis, 2 `open`-read), 70
written in a program and 14 in `std.algebra`; 25 minimal holes and violation
witnesses attributed by cause, *absent-refutes* 9, *trigger* 10,
*absent-witness* 2, *activator-obligation* 2, *activator-refutes* 2; and

> a violation no site is named for: 6; a monotone cube with a violation: 0

The six are § 2's: `activator-chain-1`, `-2`, `-node-absent`,
`-node-writes`, `-param-absent` and `lattice/12_activator_chain_hole`. Every
other site named is the one the program was written to show; on `lattice/08`
and `lattice/09` the stage's four items are named as the stage named them:

| program | item | cause | site |
|---|---|---|---|
| `lattice/08` | minimal hole `{(allergic Ann), (seats Ann C1)}` | absent-witness | `soup-unless-allergic` |
| `lattice/08` | `{(seats Ann C1)}` ⊂ `{(allergic Ann), (seats Ann C1)}`, discharged below owing | absent-witness | `soup-unless-allergic` |
| `lattice/09` | minimal hole `{(opens K1)}` | activator-obligation | `total-owed` (activator) |
| `lattice/09` | `∅` ⊂ `{(opens K1)}`, discharged below owing | activator-obligation | `total-owed` (activator) |

**The gap is open again for a chain of activators**, which the corpus gained
after the stage: the six holes of § 2 with no site, and their violation
witnesses. Each of the three clauses reads the rule that refutes or owes; a
site two activators up is no clause's. It is example code: no route reads it,
and no answer moves.

## 9. Each option, drafted on real programs

The options of the stage, for F21's D4, less the refusal, which Q-M1f.564
rules out. **None is asked here.** Each is drafted as the rows it would print
in place of the `lattice` row and its `minimal hole` rows, by the census
itself from the cube it computed:

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census -- --drafts --no-scratch \
    examples/ein-bugs/naf-upward-closure.ein examples/lattice/04_obligation_trigger_hole.ein \
    examples/lattice/08_absent_witness_hole.ein examples/lattice/synth/trigger_hole.ein                       # 0.7 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census -- --drafts --no-scratch \
    --limit 19 examples/lattice/06_friends_tables.ein                                                         # 6.2 s
```

**Today's rows are the binary's**, `ein solve FILE` at `213e8a92`, and not the
census's own *today* block, which still prints the stub as it read before the
warning (*… — 1 hole; assumes a monotone lattice for what a node is*). Today
the read-out heads a flagged program with three `WARNING` rows, and its
`lattice` block is the row with the hole count, then `assumes`, a `site` row
for each site, `status`, `state`, `paths` and up to four `minimal hole` rows.
**Every other block is drafted**: no command prints it, and each replaces only
the `lattice` row and its `minimal hole` rows; the `WARNING` rows and the
`assumes`, `site`, `status`, `state` and `paths` rows stay under every option.
The wording is a placeholder for the option, not a proposal for its text,
which is (d3)'s.

- **(a) R, its lines and its holes named.** The `lattice` row says the lines
  are R's own extremes and that not every node between them is in R. Then
  every minimal hole, dead and owing named apart ((d1) drafted as two words):
  a dead one with its refuting rule and the `absent` premises its refutation
  rests on, an owing one with what it owes; its cause and site; and a second
  row with its repair edges. The `(region …)` listing is R, as today.
- **(b) R as convex pieces.** The `lattice` row, then § 5's pieces, each with
  its own two lines, in place of the hole rows.
- **(c) A qualifier.** Today's rows, with the stub reworded to say the lines
  bound R and do not describe it.

**Lines.** `ein solve`'s stdout today, its `lattice` block, and the rows
each option drafts in place of the `lattice` row and its hole rows. The
**listings** (R's states, a line's members, each query's classes,
`as :expect`) share 100 lines, and the rows above them do not count;
`--dump-answer DIR` writes every listing whole to `DIR/answer.txt`.

| program | stdout today | `lattice` block today | … its `lattice` and hole rows | (a) | (b) | (c) |
|---|---:|---:|---:|---:|---:|---:|
| `naf-upward-closure` | 41 | 7 | 2 | 3 | 4 | 2 |
| `lattice/04` | 47 | 10 | 2 | 3 | 4 | 2 |
| `lattice/08` | 71 | 7 | 2 | 3 | 4 | 2 |
| `synth/trigger_hole` | 99 | 10 | 3 | 5 | 8 | 3 |
| `06_friends_tables`, bare | 122, cut | 6 | 1 | 1 | 1 | 1 |
| `06_friends_tables`, `--region-limit 19` | 135, cut | 11 | 5 | **37** | **23** | 5 |

On the four small programs every option fits beside the listings. On `06` at
19 the listings are already cut today (*… 7 more classes: --dump-answer DIR
writes them to DIR/answer.txt*); (a)'s 36 hole rows under the cap would leave
64 lines for R's 21 states and the query's 21 classes, and the rest would go
to `answer.txt`. `ein oracle --dump DIR`'s `region.md` already lists every
minimal hole with why it is out (18 on `06`), so (a)'s `answer.txt` would
repeat it in the read-out's form. Bare, past the limit, the cube is not
enumerated and every option prints what today prints: *holes not known*.

### naf-upward-closure, one dead hole

`examples/ein-bugs/naf-upward-closure.ein`.

Today, printed by `ein solve`, the `lattice` block:

<!-- output -->
```
  lattice           not monotone: 1 site (the static check) — 1 hole
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is not decided: Q-M1e2.2
    site            bad: absent reads q at absent depth 1, concludes (false) — at stake: the state, and dead with it
    status          non-monotone here (exercised, by bad) — the cube: 1 hole · 1 dead node with an alive superset · 0 discharged nodes with an owing superset
    state           no stale fact in the cube's 4 nodes
    paths           every parent path checked reaches the same state — 1 parent of 1 printed state
    minimal hole    {(p A)} — dead: (p A)
```

(a), drafted:

```
  lattice           not monotone: 1 site (the static check) — 1 hole (1 dead): the lines are R's own minimal and maximal states, and R is not every node between them
    dead hole       {(p A)} — refuted by bad, resting on (absent (q A)) (absent-refutes: bad)
                      repaired by +(q A)
```

(b), drafted:

```
  lattice           not monotone: 1 site (the static check) — 1 hole: R is its convex pieces, below
    pieces          2 convex pieces, each described by its own two lines; 1 state lies in more than one
      piece 1 of 2  2 states — minimal ∅ · maximal {(q A)}
      piece 2 of 2  2 states — minimal {(q A)} · maximal {(p A), (q A)}
```

(c), drafted:

```
  lattice           not monotone: 1 site (the static check) — 1 hole: the two lines bound R and do not describe it; R is the listing
    minimal hole    {(p A)} — dead: (p A)
```

### lattice/04, one owing hole by a trigger

`examples/lattice/04_obligation_trigger_hole.ein`. The static check names four
sites today, where it named one: the two hypothesis rules `dine` and `seat`
read `is-a`, and so does `total-owed`'s guard.

Today, printed by `ein solve`: the `lattice` row (*not monotone: 4 sites (the
static check) — 1 hole*), `assumes`, four `site` rows, `status`, `state`,
`paths`, and `minimal hole {(seats Ann C1)} — owes 1`.

(a), drafted:

```
  lattice           not monotone: 4 sites (the static check) — 1 hole (1 owing): the lines are R's own minimal and maximal states, and R is not every node between them
    owing hole      {(seats Ann C1)} — owes 1: meal owes Ann a Food. (trigger: total-owed [activator meal is-a])
                      repaired by +(meal Ann Soup)
```

(b), drafted:

```
  lattice           not monotone: 4 sites (the static check) — 1 hole: R is its convex pieces, below
    pieces          2 convex pieces, each described by its own two lines; 1 state lies in more than one
      piece 1 of 2  2 states — minimal ∅ · maximal {(meal Ann Soup)}
      piece 2 of 2  2 states — minimal {(meal Ann Soup)} · maximal {(meal Ann Soup), (seats Ann C1)}
```

(c), drafted:

```
  lattice           not monotone: 4 sites (the static check) — 1 hole: the two lines bound R and do not describe it; R is the listing
    minimal hole    {(seats Ann C1)} — owes 1
```

### lattice/08, an owing hole from an `absent`

`examples/lattice/08_absent_witness_hole.ein`.

Today, printed by `ein solve`: the `lattice` row (*not monotone: 1 site (the
static check) — 1 hole*), `assumes`, the `site` row of `soup-unless-allergic`,
`status`, `state`, a `paths` row that finds the node `{(allergic Ann), (meal
Ann Bread), (seats Ann C1)}` path-dependent by way of its parent without
`(allergic Ann)`, and `minimal hole {(allergic Ann), (seats Ann C1)} — owes 1`.

(a), drafted:

```
  lattice           not monotone: 1 site (the static check) — 1 hole (1 owing): the lines are R's own minimal and maximal states, and R is not every node between them
    owing hole      {(allergic Ann), (seats Ann C1)} — owes 1: meal owes Ann a Food. (absent-witness: soup-unless-allergic)
                      repaired by +(meal Ann Bread), +(meal Ann Soup)
```

(b), drafted:

```
  lattice           not monotone: 1 site (the static check) — 1 hole: R is its convex pieces, below
    pieces          2 convex pieces, each described by its own two lines; 9 states lie in more than one
      piece 1 of 2  10 states — minimal {(meal Ann Bread)}, {(meal Ann Soup)}, {(seats Ann C1)} · maximal {(allergic Ann), (meal Ann Bread), (meal Ann Soup)}, {(meal Ann Bread), (meal Ann Soup), (seats Ann C1)}
      piece 2 of 2  12 states — minimal {(meal Ann Bread)}, {(meal Ann Soup)} · maximal {(allergic Ann), (meal Ann Bread), (meal Ann Soup), (seats Ann C1)}
```

(c), drafted:

```
  lattice           not monotone: 1 site (the static check) — 1 hole: the two lines bound R and do not describe it; R is the listing
    minimal hole    {(allergic Ann), (seats Ann C1)} — owes 1
```

### synth/trigger_hole, both kinds

`examples/lattice/synth/trigger_hole.ein`.

Today, printed by `ein solve`: the `lattice` row (*not monotone: 3 sites (the
static check) — 7 holes*), `assumes`, three `site` rows (`dead-1`, `dead-2`,
`owe-2`), `status`, `state`, `paths`, and the two minimal holes,
`{(h A), (h B)} — owes 1` and `{(h A), (h D)} — dead: (h A) (h D)`.

(a), drafted:

```
  lattice           not monotone: 3 sites (the static check) — 7 holes (3 dead, 4 owing): the lines are R's own minimal and maximal states, and R is not every node between them
    owing hole      {(h A), (h B)} — owes 1: with B: one of C is required (trigger: owe-2)
                      repaired by +(h C)
    dead hole       {(h A), (h D)} — refuted by dead-1, resting on (absent (h B)), (absent (h E)) (absent-refutes: dead-1)
                      repaired by +(h E)
```

(b), drafted:

```
  lattice           not monotone: 3 sites (the static check) — 7 holes: R is its convex pieces, below
    pieces          6 convex pieces, each described by its own two lines; 7 states lie in more than one
      piece 1 of 6  4 states — minimal {(h A)} · maximal {(h A), (h C), (h E)}
      piece 2 of 6  4 states — minimal {(h A), (h C)} · maximal {(h A), (h B), (h C), (h E)}
      piece 3 of 6  5 states — minimal {(h A), (h C)}, {(h A), (h E)} · maximal {(h A), (h B), (h C)}, {(h A), (h C), (h E)}, {(h A), (h D), (h E)}
      piece 4 of 6  4 states — minimal {(h A), (h E)} · maximal {(h A), (h C), (h D), (h E)}
      piece 5 of 6  5 states — minimal {(h A), (h B), (h C)}, {(h A), (h C), (h E)}, {(h A), (h D), (h E)} · maximal {(h A), (h B), (h C), (h E)}, {(h A), (h C), (h D), (h E)}
      piece 6 of 6  4 states — minimal {(h A), (h C), (h E)} · maximal {(h A), (h B), (h C), (h D), (h E)}
```

(c), drafted:

```
  lattice           not monotone: 3 sites (the static check) — 7 holes: the two lines bound R and do not describe it; R is the listing
    minimal hole    {(h A), (h B)} — owes 1
    minimal hole    {(h A), (h D)} — dead: (h A) (h D)
```

### `06_friends_tables` at `--region-limit 19`, 166 holes

Today, printed by `ein solve examples/lattice/06_friends_tables.ein
--region-limit 19`: the `lattice` row, an `in flight` row (*a state the run
built holds (open (sits Dan S1)), which root does not hold*), `assumes`, the
`site` row, `status`, `state`, `paths`, and the first four of the 18 minimal
holes, each `— owes 1`.

(a), drafted, 37 lines, of which the first five:

```
  lattice           not monotone: 1 site (the static check) — 166 holes (166 owing): the lines are R's own minimal and maximal states, and R is not every node between them
    owing hole      {(sits Ann B1)} — owes 1: Ann's group Pair sits, and Bob has no seat (trigger: group-seated-owed)
                      repaired by +(sits Bob B2), +(sits Bob B3)
    owing hole      {(sits Ann B2)} — owes 1: Ann's group Pair sits, and Bob has no seat (trigger: group-seated-owed)
                      repaired by +(sits Bob B1), +(sits Bob B3)
    …
```

A trio member's hole reads *owes 2* and *no one fact takes it into R*. (b),
drafted, 23 lines: the `lattice` row, `21 convex pieces, each described by its
own two lines`, and one row per state, each its own minimal and maximal
member. (c): today's five rows, the stub reworded.

### The written programs

On `ACTIVATOR_REFUTES` (a)'s row reads *refuted by watch, resting on (absent
(q A))*, and on `ACTIVATOR_UNDER_ABSENT` *refuted by watch, resting on no
recorded `absent`*: § 2's finding, printed as (a) would print it. The other
options print what they print on `naf-upward-closure`.

## 10. The consequences, per option

What each option would move, counted on the corpus at `213e8a92`. The goldens
are `ein-render/tests/golden/corpus_shapes.md5`, which banks no read-out and
no region trace of a flagged program (Q-M1f.564: no line of the shape golden
holds a flagged program's answer); `ein-cli/tests/golden/corpus_exits.txt`;
the read-out's golden page, `09-read-out-examples.md`, whose X8 and X17 are
`lattice/04` and X9 is `06_friends_tables` past the limit; and
`ein-summary/2`'s `region.holes`, which lists every minimal hole with `set`,
`dead`, `core` and `owes`. **(d), a refusal, is not counted**: Q-M1f.564 rules
it out.

| | (a) holes named | (b) convex pieces | (c) a qualifier |
|---|---|---|---|
| **refuses** | nothing | nothing | nothing |
| **read-out lines** | the `describe` read-out of a program with a hole: the `lattice` row's text, and one row per minimal hole plus one of repair edges, where today has up to four | the same read-out: a `pieces` row and one per piece | the `lattice` row, wherever the stub prints it today: every flagged program, every mode |
| **`corpus_shapes.md5`** | none: no read-out of a flagged program is banked | none | none |
| **`corpus_exits.txt`** | 0 | 0 | 0 |
| **`09-read-out-examples.md`** | X8, X17 | X8, X17 | X8, X9, X17 |
| **`ein-summary/2`** | `holes.minimal[*]` gains `cause`, `sites` and `repairs`: additive | a `pieces` block: additive | none, unless the qualifier is a field |
| **the register** | nothing | nothing | nothing |
| **new notions** | none: the oracle has every field | *convex piece*, which is not a partition (§ 5): a choice of which piece keeps a shared state, or overlapping pieces on the page | none |
| **`06` past the limit** | *holes not known: the cube was not enumerated*, as today; nothing to name | *pieces not known* | the qualifier, with the lines it qualifies not known |
| **`06` at `--region-limit 19`** | 37 rows for 5 today, +32; the listings were already cut at 100 | 23 rows, +18 | 5 rows, as today |

**What (a) prints that today does not, everywhere.** The `absent` a dead
hole's refutation rests on (today only the core, § 4), the cause and site,
the repair edges, and *every* minimal hole (today four). On
`activator_under_absent` it says *no recorded `absent`*, which is § 2's
finding printed. On the six chains of § 2 it would print no site, until the
attribution reads a chain.

**What (b) costs on the corpus.** 6 pieces against 2 line members on
`trigger_hole`, with 7 states in more than one piece; 9 states in both pieces
of `lattice/08`; 21 pieces of one state on `06`. Its rows say less than (a)'s
about why a node is out, and the pieces need a rule the census had to choose
(§ 5).

**What (c) leaves out.** It is the stub reworded and needs no new row. What it
does not say is which nodes are holes beyond the first four, why a dead hole
is dead, or which fact repairs one: on `06`, 14 of 18 minimal holes stay
unnamed on stdout.

**`lattice/04`'s row, (d4).** No row is filed. Fork and scratch agree on every
node of `lattice/04` (§ 6), so no reading of D2 moves its R. Under (a), (b) or
(c) the answer is the R `ein solve` lists today and only the rows around the
hole change, which is not a wrong answer; and nothing is claimed of it, the
program being flagged.

**The sub-questions, against the drafts.** (d1): (a) is drafted with two
words, *dead hole* and *owing hole*; with one word the `why` already tells them
apart, as today. (d2): no option needs a claim form, since `(solve :expect
(region …))` states R and R fixes its holes; (b) would want one only if the
pieces were part of the answer and not of its description. (d3): (c) is the
stub reworded. (d4): above.

**The stage's recommendation, re-examined against the evidence (mine, not a
ruling).** (a), with dead and owing holes named apart and no new claim form,
kept. Three things the census adds to it: the dead row should print the
`absent` premises the refutation rests on, which today's row does not, and say
so when there are none (`activator_under_absent`); a repair edge is one fact,
so on `06` 8 of 18 minimal holes have none to print; and a cause and site
need the attribution to follow a chain of activators, or six of the corpus's
25 holes print none. (b)'s pieces are not a partition and need a rule to be
one; (c) prints less than the oracle holds.
