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
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance   # § 7, the levers, 0.9 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cost         # § 8
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths        # § 11, every split and chain (S1e2.1b.7)
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
| levers that move either route: `--jobs`, 8 interners, every flag | **none**, on 183 cubes |
| … equal-priority rule order | moves **both** routes' universe on `features/07` and `f07-fpoff` |
| a scratch node's saturation against a fork node | ×0.2–5.6; root re-derived at every node |

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

## 7. Invariance

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance                         # the flagged cubes, 0.9 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance --written
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance \
    plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/*.ein
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance --control --max-n 9    # 149 monotone cubes, 10 s
```

Each route is re-run under four levers, and each run is compared with the
route's own default run, node by node, keyed by the commitment's text:

- **`--jobs`.** Fork: `region::compute` at 1 and at 16 jobs, status and state
  class of every node. Scratch: the node fan-out at 1 and at 16 threads.
- **Permuted interners.** 8 seeds, `id_order_invariance`'s (1 + 7919 i). Each
  route is rebuilt on a `Terms` whose id space is a permutation of the default
  run's, by that test's `permuted`, copied into the census.
- **Equal-priority rule orders.** Every order of the program's `(rule …)`
  forms within each priority: all of them where there are at most 24, else
  the reverse and 8 seeded shuffles.
- **Every `(config …)` flag** flipped from its default. A flag the program
  already sets is skipped.

A run moves **R** when a node's status moves, a **state** when a node's facts
move, and the **universe** when the candidate list moves. On every run below
the two routes moved on the same runs, so one count stands for both:

| run | cubes | `--jobs` | interners | rule orders | flags |
|---|---:|---|---|---|---|
| the flagged cubes | 18 | 0 of 18 | 0 of 144 | **2 of 17** move the universe | 0 of 121 |
| S1e2.4.1's probes | 13 | 0 of 13 | 0 of 104 | 0 of 13 | 0 of 89 |
| the written programs | 3 | 0 of 3 | 0 of 24 | **1 of 1** moves the universe | 0 of 21 |
| monotone, n ≤ 9 | 149 | 0 of 149 | 0 of 1 192 | 0 of 117 | 0 of 1 036 |

### 7.1 What rule order moves

**`features/07` and `f07-fpoff`.** Write `derive-q` before `derive-p`, and
root derives `(q one)` where it derived `(p one)`. The candidate list becomes
`{(p one)}`, and R is `{∅, {(p one)}}`, the mirror image of today's. That
happens **on both routes**. The oracle's own R on `features/07` depends on
which of two rules at priority 200 is written first, which is the config
principle's *stable* ([`monotonicity.md` § 6.1](../../../docs/kernel/inference/monotonicity.md#61-three-words):
*"it does not depend … on rule order"*) broken by today's node, not only by
scratch.

**`stratified_stale`**, written. With `derive-q` first, root is `{(q one)}`,
the program's perfect model, and the universe grows to `{(p one), (r one)}`.
This one is stratified, so (D) does not reach it (§ 4).

### 7.2 What settles it

- **(D)** refuses `features/07` and `f07-fpoff`. Of the 167 cubes
  this run reorders (18 flagged, 149 monotone), they are the only ones whose
  answer depends on rule order.
- **(C) does not settle it.** Over (A), R is `{∅}` in both orders, but ∅'s state
  is `{(p one)}` in one and `{(q one)}` in the other. Over (B), both orders keep
  both stable models, under different universes. (C) removes stale states, and
  each order's states are stale-free.
- **On a stratified program**, what settles it is admitting candidates in
  stratum order. Then scratch's root is the perfect model in every order
  (§ 4's argument), and on the corpus no node moves, because every scratch
  node there is already stale-free.

## 8. Cost

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cost              # the flagged cubes, 0.1 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cost --control    # the six monotone cubes with n ≥ 12, 4 096 nodes each, 7.6 s
```

One thread, each node read to its two bits only. **Fork**: branch root, write
the commitment, resume, saturate, detect, tally. **Scratch**: parse and load
the text with the commitment, saturate, detect, tally, with the saturation's
share (from the end of the load) shown on its own. **Derived** is the facts
the node's own saturation adds. For fork that is beyond root and beyond the
commitment. For scratch it is beyond the load, which re-derives root's.

| program | n | fork µs | scratch µs | ×  | saturation alone µs | × | derived: fork | scratch | root |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `d3-unsat-core` | 5 | 6.6 | 57.3 | 8.7 | 10.5 | 1.6 | 1.4 | 1.4 | 0 |
| `branching/13` | 2 | 4.1 | 58.0 | 14.1 | 8.3 | 2.0 | 0.3 | 1.3 | 1 |
| `branching/14` | 2 | 3.4 | 59.2 | 17.3 | 7.1 | 2.1 | 0.3 | 0.3 | 0 |
| `alive-empty-interlayer` | 3 | 3.0 | 70.6 | 23.7 | 6.8 | 2.3 | 1.0 | 1.0 | 0 |
| `naf-upward-closure` | 2 | 2.9 | 51.9 | 17.8 | 4.4 | 1.5 | 0.3 | 0.3 | 0 |
| `f07-fpoff` | 1 | 1.5 | 50.1 | 33.0 | 7.8 | 5.1 | 0.0 | 0.5 | 1 |
| `naf-nowb` | 2 | 2.3 | 40.2 | 17.3 | 3.7 | 1.6 | 0.3 | 0.3 | 0 |
| `stale-naf-fork-warn` | 1 | 2.8 | 50.3 | 17.7 | 6.2 | 2.2 | 0.5 | 0.5 | 1 |
| `stale-naf-fork` | 1 | 2.7 | 46.3 | 17.4 | 5.7 | 2.2 | 0.5 | 0.5 | 1 |
| `features/07` | 1 | 1.4 | 50.6 | 36.9 | 7.5 | 5.4 | 0.0 | 0.5 | 1 |
| `lattice/04` | 2 | 8.2 | 303.9 | 37.0 | 10.0 | 1.2 | 0.5 | 0.5 | 0 |
| `lattice/08` | 3 | 7.0 | 306.7 | 44.1 | 12.5 | 1.8 | 0.3 | 0.3 | 0 |
| `lattice/09` | 2 | 6.8 | 290.1 | 42.6 | 7.5 | 1.1 | 0.5 | 0.5 | 0 |
| `synth/local_not_global` | 2 | 5.2 | 59.2 | 11.4 | 5.0 | 1.0 | 0.0 | 0.0 | 0 |
| `synth/maximal_not_top` | 3 | 5.6 | 57.0 | 10.2 | 5.2 | 0.9 | 0.0 | 0.0 | 0 |
| `synth/three_components` | 7 | 7.8 | 104.3 | 13.3 | 14.4 | 1.8 | 0.8 | 0.8 | 0 |
| `synth/trigger_hole` | 5 | 6.8 | 74.8 | 11.0 | 9.3 | 1.4 | 0.1 | 0.1 | 0 |
| `tests/region/07_cube` | 2 | 8.5 | 308.7 | 36.3 | 9.8 | 1.2 | 0.5 | 0.5 | 0 |
| **control**, monotone | | | | | | | | | |
| `branching/01_saturate_only` | 16 | 45.9 | 137.0 | 3.0 | 58.9 | 1.3 | 10.1 | 18.1 | 8 |
| `features/02_star_in_identifiers` | 15 | 47.6 | 111.9 | 2.4 | 58.8 | 1.2 | 6.7 | 16.7 | 10 |
| `lattice/07_two_pairs_two_tables` | 16 | 54.2 | 391.1 | 7.2 | 53.9 | 1.0 | 1.0 | 1.0 | 0 |
| `type-exclusivity/pets` | 15 | 8.5 | 53.4 | 6.3 | 12.4 | 1.5 | 2.6 | 8.6 | 6 |
| `syntax/constraint-scopes` | 12 | 2.0 | 33.2 | 16.9 | 0.4 | 0.2 | 0.0 | 0.0 | 0 |
| `tests/stdlib/bijection/06_blind_enumeration` | 16 | 101.2 | 792.0 | 7.8 | 204.8 | 2.0 | 15.6 | 36.6 | 21 |

The µs are means over every node, or over 4 096 nodes spread evenly over the
cube where it has more. Three runs on 2026-11-02: on the six control cubes no mean
moved by more than 1 %; on the flagged cubes, a few µs each over 2–128 nodes,
a mean moved by up to 17 % and a ratio by up to 22 %. The table is the first
run, and the ranges in the text cover all three.

- **The saturation scratch adds is small**: ×0.2–5.6 of the fork's whole
  node. It re-derives root at every node, which is the "derived" columns:
  scratch = fork + root, to the rounding (`bijection/06`: 15.6 + 21 ≈ 36.6).
  The biggest factors are on the smallest programs, where a fork derives
  nothing and costs a branch.
- **Parse and load dominate the prototype**: ×2.4–44 with them, about 290 µs
  for any program that imports the stdlib (`lattice/04`, `08`, `09`,
  `07_cube`). A build of (B) inside the engine would branch a loaded,
  unsaturated program as the fork branches root, and would not pay them per
  node. That is an argument; this prototype does not measure it.
- `region.py`, two processes a node: 5–27 ms (§ Why a library prototype).

## 9. W8's two rows, computed

Each option's R, from the route that option defines, on the two witnesses and
their twins. Both twins give their witness's answer on every option (§ 2.1).
The node census prints these as `A_fork`, `B_scratch`, `C_over_A` and
`C_over_B`. "State" is the facts beyond the program's own.

**`W8/features-07`** (`features/07_unstratifiable.ein`, twin `f07-fpoff`):

| option | R | k_s | maximal line | its state |
|---|---|---:|---|---|
| (A) fork | ∅, `{(q one)}` | 2 | `{(q one)}` | `(p one) (q one)`, which the fixture's header says is *"not a model of this program under any reading"* |
| (B) scratch | ∅, `{(q one)}` | 2 | `{(q one)}` | `(q one)`. ∅'s state is `(p one)`: the program's two stable models, one per node |
| (C) over (A) | ∅ | 1 | ∅ | `(p one)`; `{(q one)}` is the third outcome, alive and holding a stale fact |
| (C) over (B) | as (B) | | | |
| (D) | — | | | refused at load, exit 1 |

**`W8/stale-naf-fork`** (`stale-naf-fork.ein`, twin `stale-naf-fork-warn`):

| option | R | k_s | maximal line | its state |
|---|---|---:|---|---|
| (A) fork | ∅ | 1 | ∅ | `(p one)` |
| (B) scratch | ∅, `{(q one)}` | 2 | `{(q one)}` | `(q one)` |
| (C) over (A) | ∅ | 1 | ∅ | `(p one)`; `{(q one)}` is the third outcome, dead with its death resting on a stale fact |
| (C) over (B) | as (B) | | | |
| (D) | not refused: stratified | | | (A), (B) or (C), as composed |

**The register's hand derivation is (B)'s answer, not (C)'s over (A).** Both
rows' `right` block is labelled *guards are premises*, and both put
`{(q one)}` in R with the state `(q one)`: *"committing (q one) withdraws (p
one)"*. That is retraction, and computed it is (B), or (C) over (B). (C) over
the fork does not withdraw `(p one)`. It disqualifies the state that holds
it, and gives R = {∅} on both programs. So *guards are premises* names two
different answers, depending on which node it is read over. The `[case.now]`
of `W8/features-07` (`solve --maximal`, projected on `q`) reads
`[["(q one)"]]` under (A) and (B) alike, and `[[]]` under (C) over (A).

## 10. The options against the evidence

| option | what moves, measured | stable under rule order | cost |
|---|---|---|---|
| **(A) fork** | nothing. W8's rows close `not-wrong` with the reason | **no** on `features/07`, `f07-fpoff` (§ 7.1) | as today |
| **(B) scratch** | `stale-naf-fork` and `-warn`: R gains `{(q one)}`, k_s 1 → 2. `features/07` and `f07-fpoff`: `{(q one)}`'s state loses `(p one)`. The probe `not_conclusion`: `{(q A)}` loses `(not (p A))`. No other node of the 18 flagged and 155 monotone cubes. L0 is no longer the node's construction on a flagged program with a dead root, and `features/12` keeps R = ∅ by measurement (§ 6.2). The guard split agrees with it on all 21 cubes S1e2.1b.6 split, by that stage's run and § 2: its two disagreements with the oracle are (B)'s two extra nodes | no on `features/07`, `f07-fpoff`; no on the written `stratified_stale` until admission follows the strata | saturation ×0.2–5.6 a node; root re-derived at every node |
| **(C) guards are premises** | over (A): `features/07` and `f07-fpoff` lose `{(q one)}` from R, k_s 2 → 1; `stale-naf-fork`'s `{(q one)}` becomes the third outcome; the probe's `{(q A)}` leaves R. Over (B): nothing on the corpus | does not settle it (§ 7.2) | a matcher call per fact with an `absent` premise |
| **(D) refuse** | `features/07`, `f07-fpoff`, `d3-unsat-core` become load errors: 7 lines of `corpus_exits.txt`, 70 of `corpus_shapes.md5`, rows `W8/features-07`, `W1/d3-unsat-core`, `W11/d3-unsat-core` | settles it on the corpus | a graph pass at load |

The goldens (B) touches are those of its four programs:
`stale-naf-fork.ein` and `stale-naf-fork-warn.ein` (1 line of
`corpus_exits.txt` and 35 of `corpus_shapes.md5` each), `features/07` (6 and
35) and `f07-fpoff` (1 and 35). Which of those lines move is the build's to
say (S1e2.1b.5).

### The stage's recommendation, re-examined (mine)

The stage recommended **(B) with (D)**, on two conditions: the stratification
check refuses no program the corpus needs, and scratch is invariant on every
stratified flagged cube.

- **The second condition holds on the corpus**: every stratified flagged cube
  and probe is unmoved by every lever on both routes (§ 7). It **fails on a
  program written to break it**, `stratified_stale`. The reason is not
  scratch. The boundary admits candidates in (priority, FIFO) order, not in
  stratum order.
- **The first is the user's to judge.** (D) refuses three programs: W8's two
  witnesses, whose right answer is what is being ruled, and a fuzz finding
  that witnesses two fixed rows. It refuses no puzzle, no stdlib test, and
  nothing a monotone program uses.

**So I keep it, amended.** (B) with (D), and the boundary admits candidates
in stratum order. Under that order a stratified program's node is its perfect
model, the one stable model, whatever order its rules are written in. By § 4's
argument, no corpus node moves beyond the five listed above, because every
corpus scratch node is already stale-free. The staleness check then stays as
an assertion, not as an option: a stale fact in a node of a stratified program
would be an engine defect. Two things favour it beyond the conditions:

- it is **the reading the guard split already computes**, since a cell builds
  its nodes from scratch. On all 21 cubes S1e2.1b.6 split, the split's R is
  (B)'s node for node: its two disagreements with today's oracle are (B)'s two
  extra nodes, and everywhere else (B) is the oracle (§ 2);
- it is the only reading under which **a node is a function of its
  commitment**, with no path (§ 6.1).

**What it costs.** Root is re-derived inside every node: ×0.2–5.6 a node in
saturation. L0 stops being a construction on a flagged program with a dead
root. Building stratum-ordered admission is a change to the saturator, and
whether it goes into S1e2.1b.5's build is part of the ruling.

**If the user would rather not change the saturator**, (B) with (D) still
holds on every corpus program. The written case then stays a known gap, and
(C) over (B) would mark its ∅ *not a solution*, which is wrong under the
stratified reading. **If the user would rather keep the fork**, (A) with (D)
gives the same corpus as today minus the three refused, and W8's
`stale-naf-fork` rows close `not-wrong`. The guard split, though, then needs
a fork-aware cell, which S1e2.1c.3 would have to design.

## 11. The path census — S1e2.1b.7

[S1e2.1b.7](s1e2.1b.7_the_consequences_written_down.md)'s T1e2.1b.7.2, for
the rows [`monotonicity.md` § 7.8](../../../docs/kernel/inference/monotonicity.md#78-the-standing-of-each-claim)
left *unverified*: every split C1 ⊆ C and longer chains, the state-memoized
path DAG and (F)'s per-node counts, and the premise that a step is a function
of the KB's contents. § 6.1 above read one-step parents, as `-y` built them.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths                    # § 11.2, the flagged cubes, 1.3 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths \
    plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/*.ein                                        # § 11.3, 1.1 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths --written          # § 11.3
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths --control --max-n 12   # § 11.4, 36 s
```

Taken 2026-11-02 at `8a2dffe3` (the parent of the commit that adds `--paths`),
release build, stdlib = the checkout, 16 jobs on a 32-thread machine, one
program a job. `--written` carries six programs since this stage: § 2.3's
three and the two of
[`examples/shared/written.rs`](../../../ein.rs/crates/ein-infer/examples/shared/written.rs)
(`activator_refutes`, `activator_under_absent`), and § 12's `fifo_gap_static`
from T1e2.1b.7.3 on.

### 11.1 What it builds

One load per program. Every construction is one **step**, `s → sat(s ∪ Δ)`:
branch a sealed KB, write Δ with a hypothesis's provenance, `Saturator::new`,
saturate to quiescence. `base` is the program as loaded and not saturated, so
`step(base, C)` is scratch, sat(P ∪ C), and `step(root, C)` is the fork,
sat(sat(P) ∪ C). Every KB of a program shares one `Terms` and one memo.

- **The DAG**: every path from P. A first step sat(P ∪ K₁), where K₁ = ∅ is
  root and K₁ = C is scratch, then steps with any non-empty Δ, memoized by
  *(C, the state's facts)*. Its states at C are what every ordered set
  partition of C reaches, given the premise below. Σ_C Σ_{Δ ⊆ C} |states| ≈
  3ⁿ steps.
- **The splits**: sat(sat(P ∪ C1) ∪ (C \ C1)) for every C1 ⊆ C, from
  scratch's own KB, apart from the DAG: 3ⁿ steps. C1 = ∅ is the fork, C1 = C
  is scratch.
- **The chains**: the unmemoized path tree from root, one candidate a step,
  Σ_{j ≤ k} n!/(n − j)! nodes to depth k. **The bound** is 13 700 tree nodes
  (`--chain-bound`, which is ⌊e·7!⌋): every maximal chain of every C on a cube
  with n ≤ 7, and of every C with |C| ≤ k on a larger one, k the deepest
  level within the bound (5 at n = 8, 4 at n = 9 and at n = 12). Within the
  limit no flagged cube has n > 7, so on them every maximal chain is read.
- **The premise**: every step any construction takes is keyed by *(its
  parent's facts, what it adds)*. A key met twice is a step from the same
  contents with another history: a chain against another chain, a split from
  scratch's KB against the DAG's representative, a scratch load against a
  fork when root derived nothing. Two that differ in their result would be a
  **conflict**.
- **The checks on the route**: the fork built here is `region::Nodes`' fork,
  the oracle's node, fact for fact and bit for bit, and scratch built here is
  § 2's text load, fact for fact, on every node read.

Per node it counts the distinct states the constructions reach. A node is
**path-dependent** when there are two or more. *Alive paths* keeps the states
reached through no dead intermediate state, root included, which is what a
route that never extends a dead node meets. *From root* keeps the paths that
start at sat(P), the path tree as F21 states it, which never builds from P
with a commitment written in. (F) is read both ways: the states of the node
whose facts are not stale, from P and from root.

### 11.2 The flagged cubes

18 programs, 256 nodes, 19 910 steps, 14 448 chain-tree nodes:

| | count |
|---|---:|
| the fork here is the oracle's node, on every node | **18 of 18** programs |
| scratch here is the text load, on every node | **18 of 18** |
| steps from contents met before (the premise checked) | 17 243 |
| … with another result (a conflict) | **0** |
| path-dependent nodes | **20**, on 10 programs |
| … through alive paths only | 10 |
| … reaching a state that is neither the fork's nor scratch's | 16 |
| nodes whose status depends on the path | 13 |
| … through alive paths only | **3**: `lattice/08`'s hole, and `stale-naf-fork`'s and its twin's `{(q one)}` |
| distinct states at one node, at most | 2 |
| (F) from P: stale-free states per node | **exactly 1 on all 256** |
| (F) from root | 1 on 252; **0 on 4**: `{(q one)}` of `features/07`, `f07-fpoff`, `stale-naf-fork`, `stale-naf-fork-warn` |

| program | n | nodes | path-dependent (alive paths) | beyond fork and scratch | status differs (alive paths) | splits differ | chains differ | what differs |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `d3-unsat-core` | 5 | 32 | 4 (4) | 4 | 0 (0) | 4 | 4 of 32 | `(r1 o2 o2)` ← `fire-1 (absent (not (r1 o3 o3)))`, kept by way of a parent without `(r1 o3 o2)`, and stale there |
| `naf-upward-closure`, `naf-nowb` | 2 | 4 | 1 (0) | 1 | 1 (0) | 1 | 1 of 4 | `{(p A), (q A)}` is dead by way of the dead `{(p A)}`, which keeps `(false)` ← `bad (absent (q A))` |
| `features/07`, `f07-fpoff` | 1 | 2 | 1 (1) | 0 | 0 (0) | 1 | 0 of 2 | `{(q one)}`: the fork's stale `(p one)` against scratch (§ 2.1) |
| `stale-naf-fork`, `-warn` | 1 | 2 | 1 (1) | 0 | 1 (1) | 1 | 0 of 2 | `{(q one)}`: dead by fork, in R by scratch (§ 2.1) |
| `lattice/08_absent_witness_hole` | 3 | 8 | 2 (2) | 2 | **1 (1)** | 2 | 2 of 8 | by way of `{(seats Ann C1)}` a node keeps `(meal Ann Soup)`, stale. **The hole `{(allergic Ann), (seats Ann C1)}` is in R that way**, owing by fork and from scratch |
| `synth/three_components` | 7 | 128 | 3 (0) | 3 | 3 (0) | 3 | 3 of 128 | three nodes of R are dead by way of a dead hole, `(false)` ← `dead-3` or `dead-7` kept |
| `synth/trigger_hole` | 5 | 32 | 5 (0) | 5 | 5 (0) | 5 | 5 of 32 | the same, through `dead-1`'s holes |
| the other 8 | | 64 | 0 | 0 | 0 | 0 | 0 | `lattice/04`, `lattice/09`, `synth/local_not_global`, `synth/maximal_not_top`, `tests/region/07_cube`, `branching/13`, `/14`, `alive-empty-interlayer`: one state per node by every construction |

**What it adds to § 6.1.**

- **Longer paths add what a one-step parent misses in two ways.** A path
  through a dead parent, which `-y` skips (next item). And a chain through an
  alive parent that is itself path-dependent: on the probe `via_hole` and the
  written `activator_under_absent` (§ 11.3) the second node is reached that
  way, and § 6.1 and the proposal's spot check, reading one-step parents,
  each saw one node of the two. No node reaches a third state.
- **`d3-unsat-core`'s four dead nodes of § 6.1 do not depend on the path.**
  Built to quiescence by every construction, each has one state. `-y`'s
  direct path ran `commitment::try_commitment_set`, which under
  `enable-fail-fast-fork` (the default) stops at the first contradiction
  (`saturate_until_dead`), and the facts it had not yet derived, `(r1 o3 o3)`,
  were § 6.1's difference. The oracle's node saturates to quiescence, and the
  four agree with it.
- **A path through a dead parent** is where `naf-upward-closure`,
  `three_components` and `trigger_hole` depend on the path: a dead node keeps
  its `(false)` when the fact that would lift it is added. A route that never
  extends a dead node does not meet it. A route that learns from a dead node
  meets exactly it, which is W3.
- **Along alive paths the status moves both ways.** On `lattice/08` an owing
  hole is in R by way of its parent, so a route that extends parents would
  report a node of R the oracle calls owing. On the probe `via_hole` and the
  written `activator_under_absent` (§ 11.3) a node of R is dead by way of an
  alive parent, so the same route would lose a node of R.

### 11.3 S1e2.4.1's probes and the written programs

| program | n | nodes | path-dependent (alive paths) | beyond fork and scratch | status differs (alive paths) | (F) from P · from root | what differs |
|---|---:|---:|---:|---:|---:|---|---|
| probe `via_hole` | 3 | 8 | 2 (2) | 2 | **1 (1)** | 1 on all · 1 on all | by way of `{(s one)}`, which derives `(p one)` under `(absent (q one))`, `{(q one), (r one), (s one)}` keeps it and `clash` refutes: **a node of R dead by way of an alive parent** |
| probe `not_conclusion` | 1 | 2 | 1 (1) | 0 | 0 (0) | 1 on all · 0 on 1 | `{(q A)}`: the fork's stale `(not (p A))` (§ 2.2) |
| the other 11 within the limit | | 196 | 0 | 0 | 0 | 1 on all | `three_components_owe` and `trigger_hole_owe`, the rewrites of the two synthetic programs with dead holes, depend on no path: their holes owe, and an owing parent keeps no `(false)` |
| written `dead_root`, `dead_root_via` | 1 | 2 | 1 (0) | 0 | 1 (0) | 1 on all · 0 on 1 | `{(h A)}`: dead by fork, through the dead root; in R from scratch (§ 6.3) |
| written `stratified_stale` | 1 | 2 | 1 (1) | 0 | 0 (0) | **0 on ∅** · 0 on both | ∅ has one state, scratch's root, and it holds the stale `(p one)` (§ 4); `{(r one)}` by fork holds two stale facts |
| written `activator_refutes` | 2 | 4 | 1 (0) | 1 | 1 (0) | 1 on all | `{(p A), (q A)}` is dead by way of the dead `{(p A)}`, through the plan `watch` compiled for `(watch q)` |
| written `activator_under_absent` | 3 | 8 | 2 (2) | 2 | **1 (1)** | 1 on all | by way of `{(p A)}` a node keeps `(watch r)` ← `act (absent (q A))`; `{(p A), (q A), (r A)}`, in R, is dead that way |

The premise is met on all of them: 16 625 repeated keys over the probes and 59
over the written programs, 0 conflicts. `features05_scoped` (n = 26) is past
the limit.

### 11.4 The control: the monotone cubes

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths --control --max-n 12 --json
```

**The bound is n ≤ 12**, where the DAG and the splits cost 3ⁿ ≤ 531 441
steps a program: 150 of the 155 monotone cubes within the limit.
`syntax/constraint-scopes` (n = 12) takes 36 s of the 36 s wall, 1 072 130
steps. The five at n = 15 and 16 would take 14 to 43 million steps each and
are not read.

| | count |
|---|---:|
| monotone cubes, n ≤ 12 | 150 |
| nodes | 6 069 |
| steps | 1 207 419 |
| chain-tree nodes | 68 458; every maximal chain on 2 437 nodes |
| steps from contents met before | 654 882 |
| … a conflict | **0** |
| path-dependent nodes | **0** |
| the fork here is the oracle's node · scratch here is the text load | 150 of 150 · 150 of 150 |
| (F) from P and from root | 1 stale-free state on every node |

### 11.5 What it settles, and what it does not

- **The premise holds where it was read**: a step is a function of the KB's
  contents on 688 809 steps from contents met before, flagged and monotone,
  with no conflict. So memoizing the path tree by the state is sound on every
  cube read here, and the DAG's states at a node are every path's.
- **Path dependence is bounded by two states** on every node read. It occurs
  on 10 of the 18 flagged cubes, on 2 of the 13 probes within the limit and
  on all 5 written programs, and on no monotone cube read. Each of the 21
  states a path reaches beyond fork and scratch holds a stale fact: a fact
  derived under an `absent` the node's commitment falsifies, `(false)` kept
  from a dead parent among them.
- **(F)'s counts are not § 7.2's.** § 7.2 argued 0 stale-free states on an
  odd cycle, 1 on a stratified program and several on an even one. Measured:
  - on the odd cycle, `d3-unsat-core`, 1 on every node, from P and from root:
    its contradictions are not stale, and its dead states are stale-free;
  - on the even cycle, `features/07` and `f07-fpoff`, 1 from P on both nodes,
    and **0 on `{(q one)}` from root**: no path reaches the program's second
    stable model at ∅, because the boundary admits `derive-p` first in the
    written order, and from root every path to `{(q one)}` keeps root's
    `(p one)`;
  - on the stratified `stale-naf-fork`, 0 on `{(q one)}` from root, for the
    same reason; on the written stratified `stratified_stale`, **0 on ∅ from
    P as well**, because the FIFO gap reaches root itself.
  So under (F) a node's count is a property of the admission order as much as
  of the program, and from root it is 0 on every node read here where root
  derived a fact the commitment falsifies. No node of any cube read here
  reaches two stale-free states.
- **What it does not read**: the five monotone cubes at n = 15–16, the
  flagged programs past the limit, and chains past depth 7.
