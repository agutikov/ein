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
cargo test --release --manifest-path ein.rs/Cargo.toml -p ein-cli --test region_dump the_oracle_and_the_per_subset_route_agree_node_for_node
```

Every command takes `--json` for one row per program. Taken 2026-11-02 at
`7a1353fc` with this stage's
[`node_census`](../../../ein.rs/crates/ein-infer/examples/node_census.rs) and
`utils/region.py` as T1e2.1b.2.1 commits them, release build, stdlib = the
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
