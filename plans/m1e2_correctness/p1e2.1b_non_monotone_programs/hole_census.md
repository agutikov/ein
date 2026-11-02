# The hole census — 2026-11-02

**S1e2.1b.3's measurement** ([the stage](s1e2.1b.3_holes.md)), the evidence
for [F21](../../followups/f21_non_monotone_programs_decided.md)'s D4, which is
[Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)'s
sub-question (d): *what is reported when R has holes?* It lists every hole of
every flagged cube the oracle reaches, by kind and by cause, each with the
site that made it, and R's shape around its holes. It decides nothing.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census                    # § 1–6, the flagged cubes, 0.2 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census -- --written       # § 2, the programs in shared/written.rs
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census -- \
    plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/*.ein                                  # § 2, S1e2.4.1's probes
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example hole_census -- \
    --limit 19 examples/lattice/06_friends_tables.ein                                                         # § 7, 29 s at 16 jobs
```

Every command takes `--json` for one row per program, and `--no-scratch` to
skip § 6. Taken 2026-11-02 at `072e6d0d` with the example T1e2.1b.3.1 adds,
release build, stdlib = the checkout, 16 jobs on a 32-thread machine. Every
number on this page is from those commands unless it names another.

**What it selects.** `site_census`'s selection, `corpus_files()` plus the
register's witnesses: 271 files, 217 of which load and 208 load and saturate.
Of those, the static check flags 26: **18** within `--region-limit 16`, which
is [`node_census.md`](node_census.md)'s selection, and 8 past it. S1e2.4.1's
14 probes get their own run, as in the other two censuses, because no test may
point into `plans/`. So do the two programs in
[`examples/shared/written.rs`](../../../ein.rs/crates/ein-infer/examples/shared/written.rs):
`ACTIVATOR_REFUTES`, which `ein-infer/tests/region.rs` now reads from there,
and `ACTIVATOR_UNDER_ABSENT`, written for this census (§ 2).

**What it reads.** [`hole_census`](../../../ein.rs/crates/ein-infer/examples/hole_census.rs)
runs `region::compute` and reads `Topology` over the status array:
`holes`, `minimal_holes`, `repairs`, `local_max` and `local_min`,
`components`, `subcubes`, the two lines and the duality. A hole's **kind** is
the bit that puts it out of R, `dead` before `owing`. Its **cause** and
**site** come from
[`examples/shared/attribution.rs`](../../../ein.rs/crates/ein-infer/examples/shared/attribution.rs),
which reads the oracle's own node: a dead node by `refuted_by` and its
`absent` premises, an owing one by the instances its fixpoint owes and the
largest node of R below it does not (`region::Nodes`, the node function every
route shares). The **convex pieces** and the **scratch** statuses are the
census's own (§ 5, § 6). No route reads any of it.

---

## 1. The headline

| | count |
|---|---:|
| flagged cubes within the limit | 18 |
| … with a hole | **9** |
| holes | **18**: 8 dead, 10 owing |
| … minimal | 12 |
| … dead and owing at once | 0 |
| … no site named | **0** |
| causes seen in the corpus | 4 of the 6 (§ 2) |
| S1e2.4.1's probes | 14: 5 flagged, 4 with a hole; 12 holes, 1 dead (`via_hole`), 11 owing |
| the programs in `written.rs` | 2, one dead hole each, both *an activator that compiles a refutation* |
| `lattice/06_friends_tables` at `--limit 19` | **166 holes**, all owing, 18 minimal; R is 21 isolated states |
| a hole whose status differs by fork and scratch | **0**, on every program above |

## 2. By cause and kind

The six causes of [the stage](s1e2.1b.3_holes.md#context), each as
`attribution.rs` recognises it:

| cause | kind | holes: corpus (programs) | probes | written | `06` at 19 | the rule that names the site |
|---|---|---:|---:|---:|---:|---|
| an `absent` that concludes `(false)` | dead | **8** (4: `naf-upward-closure` 1, `naf-nowb` 1, `synth/three_components` 3, `synth/trigger_hole` 3) | 0 | 0 | 0 | an `absent` site whose rule refuted the node, over a premise's relation |
| a refutation through a fact derived under an `absent` (`via`) | dead | 0 | **1** (`via_hole`) | 0 | 0 | an `absent` site over a premise's relation, the refuting rule reading none |
| an activator that compiles a refutation | dead | 0 | 0 | **2** | 0 | the activator site of a refuting rule with no plan at root |
| an `absent` that takes a witness away | owing | **1** (`lattice/08`) | 0 | 0 | 0 | an `absent` site whose rule derived a fact the node of R below holds and the hole does not |
| an obligation's trigger | owing | **8** (4: `lattice/04` 1, `synth/local_not_global` 2, `synth/trigger_hole` 4, `tests/region/07_cube` 1) | **11** (3: `naf_open_hrules` 1, `three_components_owe` 3, `trigger_hole_owe` 7) | 0 | **166** | the trigger site of an instance owed in the hole and not below |
| an activator that compiles an obligation | owing | **1** (`lattice/09`) | 0 | 0 | 0 | the activator site of an owed instance whose plan root never compiled |

- **Every hole has exactly one cause.** No hole is dead and owing at once, and
  no hole owes two instances with different causes.
- **The two dead causes no corpus program shows** are both written:
  `via_hole` by S1e2.4.1, `ACTIVATOR_REFUTES` by S1e2.1b.6. The corpus's
  activator sites are `lattice/09`'s, which compiles an obligation, and
  `features/01`'s, past the limit and never exercised
  ([site census § 2](site_census.md#2-the-sites)).
- **`ACTIVATOR_UNDER_ABSENT` is new**, written for this census. `act` writes
  the activator `(watch r)` while `(q ?x)` is absent, which compiles `watch`
  for `r`, and `(r A)` then refutes. Its dead hole `{(p A), (r A)}` has
  `absent_premises` empty: a derivation does not record the activator its
  plan was compiled from, so the `absent` the death rests on is in no
  premise. The site rule names the activator site from `refuted_by`, not from
  the premises. What this means for learning from a refutation is
  [S1e2.1b.4](s1e2.1b.4_refutation_resting_on_absent.md)'s.
- **Under *forbid***, once S1e2.2.5's refusal ships: the first row is refused;
  the second if its F2 takes `via`; the third if its F3 reads the plans a
  node compiles. The three owing rows are not refutations and stay.

## 3. Per program

The 9 flagged corpus cubes within the limit with a hole, the probes and
written programs with one, and `06` at 19. *Lines* is the minimal line's and
the maximal line's members, and *between* is how many nodes lie between them:
R and its holes, since a hole is exactly a node outside R between two members.
*Pieces* is § 5's.

| program | n | \|R\| | holes | dead | owing | minimal | repair edges | one-fact test wrong: max / min | components | sub-cubes | lines | between | pieces (states in two or more) |
|---|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---|---:|---|
| `ein-bugs/naf-upward-closure` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `ein-bugs/wrong-answers/naf-nowb` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `lattice/04_obligation_trigger_hole` | 2 | 3 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `lattice/08_absent_witness_hole` | 3 | 5 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 2 + 1 | 6 | 2 (3) |
| `lattice/09_activator_hole` | 2 | 3 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| `lattice/synth/local_not_global` | 2 | 2 | 2 | 0 | 2 | 2 | 2 | **1 / 1** | 2 | 2 | 1 + 1 | 4 | 2 (0) |
| `lattice/synth/three_components` | 7 | 13 | 3 | 3 | 0 | 2 | 3 | 0 / 0 | 3 | 5 | 3 + 3 | 16 | 6 (5) |
| `lattice/synth/trigger_hole` | 5 | 9 | 7 | 3 | 4 | 2 | 6 | 0 / 0 | 1 | 4 | 1 + 1 | 16 | 6 (7) |
| `tests/region/07_cube` | 2 | 3 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| probe `naf_open_hrules` | 2 | 3 | 1 | 0 | 1 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| probe `three_components_owe` | 7 | 13 | 3 | 0 | 3 | 2 | 3 | 0 / 0 | 3 | 5 | 3 + 3 | 16 | 6 (5) |
| probe `trigger_hole_owe` | 5 | 9 | 7 | 0 | 7 | 2 | 6 | 0 / 0 | 1 | 4 | 1 + 1 | 16 | 6 (7) |
| probe `via_hole` | 3 | 7 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 3 | 1 + 1 | 8 | 2 (3) |
| written `activator_refutes` | 2 | 3 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 2 | 1 + 1 | 4 | 2 (1) |
| written `activator_under_absent` | 3 | 7 | 1 | 1 | 0 | 1 | 1 | 0 / 0 | 1 | 3 | 1 + 1 | 8 | 2 (3) |
| `lattice/06_friends_tables`, `--limit 19` | 19 | 21 | 166 | 0 | 166 | 18 | 68 | **7 / 20** | 21 | 21 | 1 + 14 | 187 | 21 (0) |

The other 9 flagged corpus cubes within the limit have no hole: the 8 the site
census calls conservative, and `synth/maximal_not_top`, whose violation is a
discharged node with an owing superset and no node of R above it. On those 9
R is one convex piece (two on none), the one-fact test agrees with the cube,
and the lines describe R exactly.

S1e2.4.1's rewrites keep R and turn dead holes into owing ones, node for node:
`three_components_owe` and `trigger_hole_owe` have the holes of
`three_components` and `trigger_hole` (3 and 7), each now owing by a trigger.

## 4. Each minimal hole

With why the oracle puts it out (its core, or what it owes), its cause, the
site named, and its repair edges: a fact whose addition takes it into R.

| program | minimal hole | kind | why | cause | site | repaired by |
|---|---|---|---|---|---|---|
| `ein-bugs/naf-upward-closure` | `{(p A)}` | dead | core `(p A)` | `absent` → `(false)` | `bad` | +`(q A)` |
| `ein-bugs/wrong-answers/naf-nowb` | `{(p A)}` | dead | core `(p A)` | `absent` → `(false)` | `bad` | +`(q A)` |
| `lattice/04_obligation_trigger_hole` | `{(seats Ann C1)}` | owing | owes 1: *meal owes Ann a Food.* | trigger | `total-owed [activator meal is-a]` | +`(meal Ann Soup)` |
| `lattice/08_absent_witness_hole` | `{(allergic Ann), (seats Ann C1)}` | owing | owes 1: *meal owes Ann a Food.* | `absent` takes a witness | `soup-unless-allergic` | +`(meal Ann Bread)` |
| `lattice/09_activator_hole` | `{(opens K1)}` | owing | owes 1: *meal owes Ann a Food.* | activator → obligation | `total-owed` (activator) | +`(meal Ann Soup)` |
| `lattice/synth/local_not_global` | `{(h A)}` | owing | owes 1: *with A: one of B is required* | trigger | `owe-1` | +`(h B)` |
| ″ | `{(h B)}` | owing | owes 1: *with B: one of A is required* | trigger | `owe-2` | +`(h A)` |
| `lattice/synth/three_components` | `{(h A), (h F)}` | dead | core `(h A) (h F)` | `absent` → `(false)` | `dead-3` | +`(h B)` |
| ″ | `{(h A), (h G)}` | dead | core `(h A) (h G)` | `absent` → `(false)` | `dead-7` | +`(h D)` |
| `lattice/synth/trigger_hole` | `{(h A), (h B)}` | owing | owes 1: *with B: one of C is required* | trigger | `owe-2` | +`(h C)` |
| ″ | `{(h A), (h D)}` | dead | core `(h A) (h D)` | `absent` → `(false)` | `dead-1` | +`(h E)` |
| `tests/region/07_cube` | `{(seats Ann C1)}` | owing | owes 1: *meal owes Ann a Food.* | trigger | `total-owed [activator meal is-a]` | +`(meal Ann Soup)` |
| probe `naf_open_hrules` | `{(p A)}` | owing | owes 1: *A has p without q* | trigger | `bad-owed` | +`(q A)` |
| probe `three_components_owe` | `{(h A), (h F)}` | owing | owes 1: *A F is dead while B is missing* | trigger | `owe-dead-3` | +`(h B)` |
| ″ | `{(h A), (h G)}` | owing | owes 1: *A G is dead while D is missing* | trigger | `owe-dead-7` | +`(h D)` |
| probe `trigger_hole_owe` | `{(h A), (h B)}` | owing | owes 1: *with B: one of C is required* | trigger | `owe-2` | +`(h C)` |
| ″ | `{(h A), (h D)}` | owing | owes 1: *A D is dead while B and E are missing* | trigger | `owe-dead-1` | +`(h E)` |
| probe `via_hole` | `{(r one), (s one)}` | dead | core `(r one) (s one)` | `via` | `derive-p` | +`(q one)` |
| written `activator_refutes` | `{(p A)}` | dead | core `(r A)` | activator → refutation | `watch` (activator) | +`(q A)` |
| written `activator_under_absent` | `{(p A), (r A)}` | dead | core `(r A)` | activator → refutation | `watch` (activator) | +`(q A)` |

`06`'s 18 are in § 7. Every site named is the one the program was written to
show. On `activator_under_absent` the site is the activator, `watch`; the
`absent` its activator rests on is `act`'s, which the static check also
reports as a site, and which no premise of the refutation records (§ 2).

**A dead hole's core does not say why it is dead.** `naf-upward-closure`'s
core is `(p A)`, and the `absent` it rests on, `(absent (q A))`, is in the
node's `absent_premises`, which the read-out does not print. On the two
written programs the core is `(r A)`, a fact that is in R alone; what kills
the node is the plan `{(p A)}` compiles. Today's `minimal hole … — dead: (p
A)` row prints the core only.

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
dead hole (`naf-upward-closure`, `naf-nowb`, `three_components`, `trigger_hole`,
`via_hole`, both written programs) and on `06` and `maximal_not_top`; it holds
on `lattice/04`, `08`, `09`, `local_not_global`, `07_cube` and the three
owing probes. That is [`monotonicity.md` § 3](../../../docs/kernel/inference/monotonicity.md#3-the-comparison)'s
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
| `naf-upward-closure` | 1 | 2 | 1 | 2 | 4 |
| `lattice/08` | 1 | 2 | 3 | 3 | 6 |
| `local_not_global` | 2 | 2 | 0 | 2 | 4 |
| `three_components` | 3 | 6 | 5 | 6 | 14 |
| `trigger_hole` | 1 | 6 | 7 | 2 | 18 |
| `06` at 19 | 21 | 21 | 0 | 15 | 42 |

On `naf-upward-closure` the pieces are `[∅ … {(q A)}]` and `[{(q A)} …
{(p A), (q A)}]`, which share `{(q A)}`. A partition would need a choice of
which piece keeps a shared state, which no rule here makes. On `06` every
state is its own component, so every piece is one state and lists it twice.

## 6. Fork and scratch

Every node of every cube above is built a second time from scratch, the
program's text with the commitment as given facts (`utils/region.py`'s
node), on 16 threads, and its two bits are held to the oracle's fork.

| run | cubes | nodes | status differs | holes the same |
|---|---:|---:|---:|---|
| the flagged corpus cubes | 18 | 256 | **2**: `stale-naf-fork`'s and its `-warn` twin's `{(q one)}`, neither a hole | yes, all 18 |
| S1e2.4.1's flagged probes | 5 | 174 | 0 | yes |
| `written.rs` | 2 | 12 | 0 | yes |
| `06` at 19 | 1 | 524 288 | 0 | yes, 23 s |

This is [`node_census.md` § 2.1](node_census.md#21-the-corpus)'s finding,
extended to the probes, the written programs and `06`: **on every program
with a hole, the holes are the same by fork and from scratch.** A hole here
needs no path dependence, as `monotonicity.md` § 7.5 argues for owing holes.
The dead holes agree too: on none of these programs does a site's rule fire
at root ([site census § 2](site_census.md#2-the-sites)), and a fact root
derived under an `absent` is what makes the two routes differ.

## 7. Past the limit: `06_friends_tables` at 19

[`06_friends_tables.ein`](../../../examples/lattice/06_friends_tables.ein),
the conditional obligation, *a group that sits owes its members' seats*. It is
the one flagged program in the 17–19 band, and a bare `ein solve` stops at the
budget with *holes not known: the cube was not enumerated*. At
`--limit 19` the oracle takes about 5.5 s on 16 jobs and the scratch check
23 s.

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

What a search past the limit can say about holes is F21's D7; what the
read-out says about them is its D4.
