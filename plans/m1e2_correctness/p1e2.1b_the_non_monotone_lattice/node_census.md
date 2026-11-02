# The node census — 2026-11-02

**S1e2.1b.2's measurement** ([the stage](s1e2.1b.2_fork_or_scratch.md)), the
evidence for [Q-M1e2.2](../open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in)
asked of every node. A node's state is built two ways, and the two are
compared node for node on every cube the oracle reaches:

- **Fork** is the oracle's node (`region.rs` § What a node is): root's
  fixpoint, the commitment written in, the saturation resumed. The census
  reads it through `region::Nodes`, the node function every route shares, and
  holds its two status bits to `region::compute`'s on a second load of the
  same program.
- **Scratch** is `utils/region.py`'s node: the program's text with the
  commitment appended as given facts, loaded and saturated from nothing.

Both are read the same way: the two status bits (dead, owing), every fact of
the fixpoint, and the facts that are **stale** (§ 4).

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census                  # § 2, the flagged cubes, 0.1 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --control      # § 3, the monotone cubes, 59 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- \
    plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/*.ein                             # § 2.2, S1e2.4.1's probes
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --written      # § 2.3, three programs written into it
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --strata       # § 5, every program that loads, 0.1 s
cargo test --release --manifest-path ein.rs/Cargo.toml -p ein-cli --test region_dump the_oracle_and_the_per_subset_route_agree_node_for_node
```

Every command takes `--json` for one row per program. Taken 2026-11-02 at
`bc1dff73` (T1e2.1b.2.1), which adds this stage's
[`node_census`](../../../ein.rs/crates/ein-infer/examples/node_census.rs) and
its fix to `utils/region.py`, release build, stdlib = the
checkout, 16 jobs on a 32-thread machine. Every number on this page is from
those commands unless it names another.

**What it selects.** With no file, the programs of `site_census`'s selection
(`corpus_files()` and the register's witnesses, 271 files, 208 of which load
and saturate) that the static check flags and whose cube is within
`--region-limit 16`: **18**. That is [site census § 3](site_census.md#3-the-flagged-programs-within-the-limit)'s
14, plus the four fixtures S1e2.1b.6 banked after it (`lattice/08`,
`lattice/09`, `synth/maximal_not_top`, `synth/local_not_global`). `--control`
takes the 155 that the check calls monotone. S1e2.4.1's probes get their own
run, as in the site census, because no test may point into `plans/`.

**Why a library prototype and not `region.py`.** The stage chose
`region.py` first, to be overruled only by a measurement that said it was too
slow. It is not too slow: 5–27 ms a node, two processes each, and 2.3 s for
all 256 flagged nodes. It was overruled for two other reasons:

- **It was wrong on one program.** It called a node dead only when the dump
  held a `false` fact. A pair contradiction, `X` beside `(not X)`, derives
  none, and on [`d3-unsat-core.ein`](../../../corpus/fuzz_findings/d3-unsat-core.ein)
  12 of the 32 nodes die that way. `region.py` called those 12 alive, while the
  oracle and both library routes call them dead. The stage fixed it: a node
  is dead also when `--json-summary`'s `region.root` says so, which is the
  engine's own test. `region_dump.rs::the_oracle_and_the_per_subset_route_agree_node_for_node`
  now runs it on `d3-unsat-core` as well, and fails on the old line. After
  the fix `region.py` and the library scratch agree on all 256 flagged nodes
  and all 206 probe nodes (a scratch script over the census's `--json` rows,
  2026-11-02).
- **Staleness needs provenance**, which no CLI output carries, and the
  control's 268 213 nodes need a process-free node.

So both routes are read in process, and `region.py` is the check that the
library's scratch route is `region.py`'s. No flag reaches the prototype, and
no command reaches it.

---

## 1. The headline

| | count |
|---|---:|
| flagged cubes within the limit | 18, **256 nodes** |
| … nodes where fork and scratch give a different status | **2** |
| … nodes where they hold different facts | **4**, the two above among them |
| … programs on which the fork read here is not the oracle's node | 0 |
| S1e2.4.1's flagged probes | 5, 174 nodes: 0 statuses differ, **1** state |
| monotone cubes within the limit (the control) | 155, **268 213 nodes**: 0 statuses differ, 0 states |
| facts only scratch holds, on any node of any cube | **0** |
| nodes holding a stale fact, by fork | 4 flagged, 1 probe, 0 monotone: **the nodes above** |
| … from scratch | **0** on every corpus and probe node |
| programs (D) would refuse, of the 208 that load and saturate | **3**: `features/07`, `f07-fpoff`, `d3-unsat-core` |
| nodes where `-y`'s parent path and the direct fork differ | 10 corpus, 1 probe; the direct fork is scratch's node on all of them |
| `features/12`, n = 20: scratch nodes that are not dead | 0 of 1 048 576 |

## 2. The flagged cubes

### 2.1 The corpus

| program | n | nodes | status differs | state differs |
|---|---:|---:|---:|---:|
| [`corpus/fuzz_findings/d3-unsat-core.ein`](../../../corpus/fuzz_findings/d3-unsat-core.ein) | 5 | 32 | 0 | 0 |
| [`branching/13_lookahead_naf_world.ein`](../../../examples/branching/13_lookahead_naf_world.ein) | 2 | 4 | 0 | 0 |
| [`branching/14_lookahead_unjudgeable.ein`](../../../examples/branching/14_lookahead_unjudgeable.ein) | 2 | 4 | 0 | 0 |
| [`ein-bugs/alive-empty-interlayer.ein`](../../../examples/ein-bugs/alive-empty-interlayer.ein) | 3 | 8 | 0 | 0 |
| [`ein-bugs/naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein) | 2 | 4 | 0 | 0 |
| [`ein-bugs/wrong-answers/f07-fpoff.ein`](../../../examples/ein-bugs/wrong-answers/f07-fpoff.ein) | 1 | 2 | 0 | **1** |
| [`ein-bugs/wrong-answers/naf-nowb.ein`](../../../examples/ein-bugs/wrong-answers/naf-nowb.ein) | 2 | 4 | 0 | 0 |
| [`ein-bugs/wrong-answers/stale-naf-fork-warn.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein) | 1 | 2 | **1** | **1** |
| [`ein-bugs/wrong-answers/stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein) | 1 | 2 | **1** | **1** |
| [`features/07_unstratifiable.ein`](../../../examples/features/07_unstratifiable.ein) | 1 | 2 | 0 | **1** |
| [`lattice/04_obligation_trigger_hole.ein`](../../../examples/lattice/04_obligation_trigger_hole.ein) | 2 | 4 | 0 | 0 |
| [`lattice/08_absent_witness_hole.ein`](../../../examples/lattice/08_absent_witness_hole.ein) | 3 | 8 | 0 | 0 |
| [`lattice/09_activator_hole.ein`](../../../examples/lattice/09_activator_hole.ein) | 2 | 4 | 0 | 0 |
| [`lattice/synth/local_not_global.ein`](../../../examples/lattice/synth/local_not_global.ein) | 2 | 4 | 0 | 0 |
| [`lattice/synth/maximal_not_top.ein`](../../../examples/lattice/synth/maximal_not_top.ein) | 3 | 8 | 0 | 0 |
| [`lattice/synth/three_components.ein`](../../../examples/lattice/synth/three_components.ein) | 7 | 128 | 0 | 0 |
| [`lattice/synth/trigger_hole.ein`](../../../examples/lattice/synth/trigger_hole.ein) | 5 | 32 | 0 | 0 |
| [`tests/region/07_cube.ein`](../../../tests/region/07_cube.ein) | 2 | 4 | 0 | 0 |

**Every node where the two differ**, with the facts that differ and the rule
that derived each:

| program | node | fork | scratch | facts only fork holds |
|---|---|---|---|---|
| `stale-naf-fork` | `{(q one)}` | dead, refuted by `p-and-q-clash` | in R | `(p one)` ← `derive-p`, a site's rule; `(false)` ← `p-and-q-clash` |
| `stale-naf-fork-warn` | `{(q one)}` | dead, refuted by `p-and-q-clash` | in R | the same |
| `features/07` | `{(q one)}` | in R, state `{(p one), (q one)}` | in R, state `{(q one)}` | `(p one)` ← `derive-p`, a site's rule |
| `f07-fpoff` | `{(q one)}` | in R, the same | in R, the same | the same |

**Why each differs.** It is one mechanism four times. Root derives
`(p one)` by `derive-p`, under `(absent (q one))`. The commitment `{(q one)}`
falsifies that guard. The fork starts from root's fixpoint, so it holds
`(p one)` and never withdraws it. Scratch writes `(q one)` before the boundary
judges any guard, so `derive-p` never fires. On `stale-naf-fork` the stale
`(p one)` meets `(q one)` in `p-and-q-clash`, and the node dies. On
`features/07` and its twin nothing refutes the pair, and the node is in R on
both routes, holding a different state.

These are the four programs [site census § 2](site_census.md#2-the-sites)
names as the only ones within the limit where a site's rule fires at root.
The census called that an argument for where the routes *can* differ. Here
the routes differ there, on one node each, and nowhere else.

**Where they agree.** On the other 14 programs every node agrees in its
status and in every fact it holds. That includes the nine with a hole, so the
holes S1e2.1b.3 reads are the same holes under either route. No node on any
cube holds a fact that only scratch holds.

### 2.2 S1e2.4.1's probes

14 files: 13 within the limit and compared, 206 nodes; `features05_scoped`
(n = 26) is past it. Of the 13, five are flagged (`naf_open_hrules`,
`not_conclusion`, `three_components_owe`, `trigger_hole_owe`, `via_hole`,
174 nodes). **One node differs**, in its state only:

| probe | node | fork | scratch | facts only fork holds |
|---|---|---|---|---|
| `not_conclusion` | `{(q A)}` | in R, state `{(not (p A)), (q A)}` | in R, state `{(q A)}` | `(not (p A))` ← `exclude`, under `(absent (q A))` |

It is the same mechanism, with a stored negative as the fact root derived. The
other eight probes are monotone and agree everywhere.

### 2.3 Three programs written into the census

`--written` runs three programs the census carries as text, for readings no
corpus program within the limit shows. No corpus file is added.

| program | what it is | fork | scratch |
|---|---|---|---|
| `dead_root` | `region_synth.py`'s refused target: ∅ dead, `{(h A)}` alive. One rule, `(and (thing ?x) (absent (h ?x))) → (false)` | R = ∅: root is dead, so every fork is | R = `{(h A)}`: committed, `(h A)` blocks the refutation |
| `dead_root_via` | the same through a derived positive, S1e2.4.1's `via` class: `p ← ¬h`, `p → ⊥` | R = ∅ | R = `{(h A)}` |
| `stratified_stale` | `p ← ¬q` and `q ← ¬r`, at one priority, `derive-p` written first. **Stratified** | R = {∅, `{(r one)}`}; `{(r one)}` holds `(p one), (q one), (r one)` | R the same; `{(r one)}` holds `(p one), (r one)` |

The first two are § 6.3's reading of the synthesizer. The third is § 4's: on a
stratified program, scratch's own root holds a stale fact.

## 3. The control: the monotone cubes

On a program the static check calls monotone, fork and scratch build the same
state by the theorem ([phase README § What non-monotone means](README.md#what-non-monotone-means-and-what-is-not)).
`--control` checks that premise node for node, on every fact; `region_dump.rs`
checks the statuses and closures of nine programs:

| | |
|---|---:|
| monotone cubes within the limit | 155 |
| nodes | 268 213 |
| nodes whose status differs | **0** |
| nodes whose facts differ | **0** |
| programs on which the fork read here is not the oracle's node | 0 |
| wall, 16 jobs | 59 s |

So on the corpus the two routes are one route wherever the static check says
they must be, and the comparison over monotone cubes is evidence for no
option, as the stage expected.

## 4. The staleness census

**What is counted.** A fact is **stale** in a node when its primary
derivation was admitted under an `absent` guard that the node's own fixpoint
now falsifies. `region::Node::absent_premises` reads the `absent` premises of a
contradiction. The census asks the question of every fact. For each fact
whose primary provenance records `absent` patterns, it finds the plan and the
disjunct that record exactly those patterns, rebuilding the saturator's
`negative_premises` from the provenance's bindings. Then it asks the engine's
matcher (`Matcher::holds`) whether that guard's query now has a match in the
node's fixpoint. Where no plan reproduces the recorded patterns, it falls back
to the patterns and counts the fact *inexact*: **0** inexact checks on every
run below.

| run | nodes | stale nodes by fork | … of them in R | … dead, the death resting on it | stale nodes from scratch |
|---|---:|---:|---:|---:|---:|
| the flagged cubes (§ 2.1) | 256 | **4** | 2 | 2 | **0** |
| S1e2.4.1's probes (§ 2.2) | 206 | **1** | 1 | 0 | **0** |
| the control (§ 3) | 268 213 | 0 | 0 | 0 | **0** |
| the written programs (§ 2.3) | 6 | 4 | 2 | 2 | **1** |

Each stale node holds one stale fact, except `stratified_stale`'s `{(r one)}`
by fork, which holds two.

- **By fork, the stale nodes are exactly the nodes where the routes differ.**
  On `stale-naf-fork` and its twin, `{(q one)}` holds `(p one)` ←
  `derive-p (absent (q one))`, and the node's `(false)` rests on it. On
  `features/07` and `f07-fpoff`, `{(q one)}` holds the same stale `(p one)` and
  is in R. On the probe `not_conclusion`, `{(q A)}` holds `(not (p A))` ←
  `exclude (absent (q A))` and is in R.
- **From scratch, no corpus node is stale.** That is every node of the 18
  flagged cubes, the 13 probes and the 155 monotone cubes.

**What a stale fact means.** This is an argument, not a test, and the census
checks its premise on every node. Take a fixpoint the engine reaches that
holds no stale fact. Each of its facts has a derivation whose positive
premises are in it and whose `absent` guards hold in it. The engine ran to
quiescence, so every rule whose body holds in it has fired: a guard that
failed once keeps failing, because facts only grow. The fixpoint is then the
least model of the program reduced by its own negation, which is a **stable
model** (Gelfond–Lifschitz) of the program plus the commitment. A stratified
program has exactly one stable model, its perfect model, whatever order its
rules fired in. So **a stale-free node of a stratified program is a function
of its commitment alone**, and a stale fact is exactly where a route's state
is not a model of the facts it commits to. On an unstratified program a
stale-free state is one stable model of several (`features/07` has two,
`{p}` and `{q}`), or there is none and some fact must be stale.

**Scratch alone does not make a stratified program stale-free.** The written
`stratified_stale` is stratified (`r` < `q` < `p`), and its scratch root holds
`(p one)` ← `derive-p (absent (q one))` beside `(q one)`. The boundary admits
one candidate a round in (priority, FIFO) order, not in stratum order. Both
candidates wait at root's first boundary at priority 200, and `derive-p`,
written first, goes first. The program's perfect model at ∅ is `{(q one)}`.
Swapping the two rules' text gives that model and a different universe
(§ 7.1). No corpus program shows this: scratch is stale-free on every corpus
node. It is what (B) leaves open on a stratified program until admission
follows the strata.

## 5. The stratification check

**The graph.** Its nodes are relations as the static check names them, `R` or
`(not R)`, taken from the saturation plans root compiled. Each plan gives an
edge from every relation it reads to every relation it writes, **negative**
when the read is at odd `absent` depth. A rule with an activator adds a
positive edge from its activator relation. A **cycle through an odd `absent`**
is a strongly connected component that holds a negative edge. *Over D* keeps
only the edges between relations in D. The census also says whether some
cycle in the component has an **odd** number of negative edges, where a
stable model need not exist, or every cycle an even number, where there can
be several.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --strata    # 0.1 s
```

Over the 208 programs of the selection that load and saturate:

| scope | programs with a cycle through an odd `absent` |
|---|---:|
| **over D**, what option (D) refuses | **3** |
| over all relations | 6 |

| program | n | flagged | over D | the cycle | parity |
|---|---:|---|---|---|---|
| [`corpus/fuzz_findings/d3-unsat-core.ein`](../../../corpus/fuzz_findings/d3-unsat-core.ein) | 5 | yes | **yes** | `fire-1` reads `(not r1)` under `absent` and writes `r1`; `fire-0` reads `r1` and writes `(not r1)` | odd |
| [`features/07_unstratifiable.ein`](../../../examples/features/07_unstratifiable.ein) | 1 | yes | **yes** | `derive-p`: `q` →¬ `p`; `derive-q`: `p` →¬ `q` | even |
| [`ein-bugs/wrong-answers/f07-fpoff.ein`](../../../examples/ein-bugs/wrong-answers/f07-fpoff.ein) | 1 | yes | **yes** | the same | even |
| [`syntax/rule-forall-and-not.ein`](../../../examples/syntax/rule-forall-and-not.ein) | 0 | no | no | `f` reads `r` under a `forall` and writes `r` and `(not r)` | odd |
| [`syntax/rule-guards.ein`](../../../examples/syntax/rule-guards.ein) | 0 | no | no | `g`: `s` →¬ `s` | odd |
| [`syntax/rule-shared-guard-var.ein`](../../../examples/syntax/rule-shared-guard-var.ein) | 0 | no | no | `sh`: `r` →¬ `r` | odd |

**What (D) would refuse, and what that costs, by name.**

- **`features/07`**: its corpus entry's goldens, 6 lines of `corpus_exits.txt`
  and 35 of `corpus_shapes.md5`, and the register row `W8/features-07`. The
  fixture exists to pin S1.21.8's *at most one admission per round*, whose
  soundness argument is this program. That discipline still matters on a
  stratified program (§ 4's written case), but no corpus program would then
  show it.
- **`f07-fpoff`**: 1 line of `corpus_exits.txt` and 35 of `corpus_shapes.md5`,
  and its place as `W8/features-07`'s twin.
- **`d3-unsat-core`**: no corpus entry and no golden line. It is the witness
  of `W1/d3-unsat-core` and `W11/d3-unsat-core`, both `fixed`, and both
  would be re-stated with a refusal as `right`.

Two of the three are W8's own witnesses, so whether "refused" is a right
answer for them is the ruling itself. The third is a fuzz finding kept for two
rows the switch fixed. `stale-naf-fork` is stratified (`q` is no rule's
conclusion), and (D) does not reach it, as the stage said.

**Over all relations** a classical check would also refuse three syntax
fixtures. Each states no fact, so nothing fires, n = 0 and D is empty. That
is the case for reading (D) over D, as the stage does.

**What the graph does not see.** A rule with an activator that no fact
activates at root has no plan, so it has no edges. 9 programs have one: the
five stdlib modules loaded as programs (`algebra`, `bijection`, `elim`, `slots`, `typing`), `domain_elim/b_branch`,
`syntax/rule-nary-and-or`, `broken/compile/activator_arity` and
`tests/stdlib/closure/01_infer_closure`. All 9 are monotone, so no commitment
can derive an activator there and no node compiles a plan root lacks. On the
corpus the graph is complete. A program whose node can compile a refutation
through an activator (`region.rs`'s `ACTIVATOR_REFUTES`) would need the check
re-run on the node's plans. That is S1f.5.6's to design, or this phase's if
(D) is ruled.

## 6. The three known cases

### 6.1 The `-y` abort on `d3-unsat-core`

Before the switch, `-y` (`lattice-sanity-check`) aborted on
`d3-unsat-core.ein` with *"Saturation commutativity violated"*. The census
runs the library function `-y` ran, `sanity::check_commutativity`, on every
node of two facts or more of every flagged cube and probe: 192 corpus nodes
and 162 probe nodes. It asks whether a parent path reaches the same state as
the direct one.

| program | nodes checked | a parent path differs | the path's extra fact | stale on the path |
|---|---:|---:|---|---|
| `d3-unsat-core` | 26 | **8** | `(r1 o2 o2)` on 4, `(r1 o3 o3)` on 4 | `(r1 o2 o2)` ← `fire-1 (absent (not (r1 o3 o3)))` on the 4; the other 4 are dead nodes whose fixpoints differ by a fact that is not stale |
| `lattice/08_absent_witness_hole` | 4 | **2** | `(meal Ann Soup)` | yes, ← `soup-unless-allergic (absent (allergic Ann))` |
| probe `via_hole` | 4 | **1** | `(p one)` | yes, ← `derive-p (absent (q one))` |
| every other flagged cube and probe | 320 | 0 | | |

The census rebuilds each path state as `-y` built it (the parent's fixpoint,
the last fact written, a full saturation) and reads it for staleness. **The
direct fork is scratch's node on every one of these nodes** (§ 2). What
differs is the path: a parent derives a fact under an `absent` that the last
fact falsifies, and the path keeps it. On an odd loop, `d3-unsat-core`'s four
dead nodes, which facts a dead fixpoint holds also depends on the order of
admission.

- **(A)**: no shipped route meets this. The oracle, route 1 and route 2 build
  every node from root at once. But the node is *root plus the commitment at
  once*, so a search that extends a parent's fixpoint builds a different node
  on a flagged program. `lattice/08` is stratified, so (D) does not remove the
  case. That constrains [P1e2.1c](../p1e2.1c_the_region_search/README.md)'s
  search.
- **(B)**: a node has no path, and the direct node already is scratch's. The
  abort says nothing against (B).
- **(C)**: the 7 path states that differ by a stale fact are caught by it.
  The other 4 are dead under every option.
- **(D)**: refuses `d3-unsat-core`. `lattice/08` and `via_hole` stay.

### 6.2 L0 on `features/12`

Root is dead by `no-room`, and n = 20, past the oracle. By fork, L0 decides
every node: `explore::Licence::ForkOnlyAdds`, *"the node function's own
construction, not a theorem about the program"*. From scratch, the census runs
the whole cube:

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- \
    --scratch-cube --jobs 16 examples/features/12_expect_false.ein                                          # 23 s
```

**All 1 048 576 nodes are dead** (23.3 s at 16 jobs). So `features/12`'s
answer, R = ∅, is the same under (B). L0's *reason* is not: under scratch a
dead root does not kill its supersets, as `dead_root` (§ 2.3) shows with
`{(h A)}` in R above a dead ∅. Under (B), a flagged program with a dead root
needs another licence, or the cube. On `features/12` the arguments that exist
are fact grain (its reads are static at fact grain, [site census § 5.3](site_census.md#53-what-leaves-the-set))
and S1e2.2.5's refusal under *forbid*. (C) would change nothing, by the same
fact-grain argument and not by a run: the guard root's `(false)` rests on reads
`(instance ?h Hole)`, which no candidate and no fact derived from one
matches, so no node falsifies it. (D) does not reach it: `features/12` is
stratified.

### 6.3 The synthesizer's dead ∅

`region_synth.py` refuses a target with ∅ dead under a non-empty R
(`RootDead`), because every fork of a dead root is dead. The census's
`dead_root` and `dead_root_via` are that target, written by hand, one
directly and one through a derived positive:

| option | `dead_root`, `dead_root_via` |
|---|---|
| (A) | R = ∅. The target cannot be written |
| (B) | R = `{(h A)}`. The target can be written |
| (C) over (A) | R = ∅; `{(h A)}` is the third outcome, dead with its death resting on a stale fact |
| (C) over (B) | as (B) |
| (D) | both are stratified, and neither is refused |

Under *forbid*, `dead_root` is the shape S1e2.2.5 refuses: a `(false)` from an
`absent` over a relation a commitment can extend. `dead_root_via` is the `via`
class its F2 decides. So under (B) the synthesizer could write the target, and
under *forbid* the program it writes would be refused.
