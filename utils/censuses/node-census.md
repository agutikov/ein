# The node census — 2026-11-25

**S1e2.1b.2's measurement** (the stage), re-taken, the
evidence for [Q-M1e2.2](../../docs/decisions/answer/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md)
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

**What holds today.** A flagged program loads, every command answers it by the
cube, today's fork, under a `WARNING` that it is not monotone, and nothing is
claimed of its answer
([Q-M1f.564](../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)):
no program is refused for an `absent`. F21's D2 option (D), *refuse a cycle
through an odd `absent`*, is a refusal and is ruled out by that decision; the
census still measures the cycles (§ 5), which is what a stratified reading
reads. **A state holds `open` facts** (Q-M1f.566): the fork read here holds
the ones root holds, what root supposes and what it owes, and scratch holds
none, so on every node of a program with a candidate the two differ by those
facts. Every comparison below says whether it counts them; *beyond the `open`
facts* is the comparison the stage made.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census                  # § 2, the flagged cubes, 11 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --control      # § 3, the monotone cubes, 507 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- \
    plans/m1n_non_monotone_programs/f21_probes/*.ein                                                                     # § 2.2, S1e2.4.1's probes: none loads today
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --written      # § 2.3, six programs written into it, 0.6 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --strata       # § 5, every program that loads, 11 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance   # § 7, the levers, 20 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cost         # § 8, 11 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths        # § 11, every split and chain (S1e2.1b.7), 12 s
cargo test --release --manifest-path ein.rs/Cargo.toml -p ein-cli --test region_dump the_oracle_and_the_per_subset_route_agree_node_for_node   # 0 tests: gone
```

Every command takes `--json` for one row per program. Taken 2026-11-25 at
`213e8a92`, which this census's re-take adds nothing to but the four programs
§ 2.3 writes, rewritten in today's spelling (each relation and rule with its
`:why`, no `:priority`); release build, stdlib = the checkout, 16 jobs on a
32-thread machine, each command through `utils/bench_env.sh --cores 0-31` on
AC. Every number on this page is from those commands unless it names another.

**What it selects.** With no file, the programs of `site_census`'s selection
(`corpus_files()` and the regressions' witnesses, 933 files, 748 of which load
and saturate) that the static check flags and whose cube is within
`--region-limit 16`: **52**, [`hole_census.md`](hole-census.md)'s selection.
`--control` takes the ones the check calls monotone: **650**.
**S1e2.4.1's 14 probes are not re-taken**: each is written in a form the
parser refuses today (`hrule`, `:priority`, no `:why`), and none loads.

**Why a library prototype and not `region.py`.** The stage chose
`region.py` first, to be overruled only by a measurement that said it was too
slow. It was overruled for two other reasons, which stand:

- **It was wrong on one program.** It called a node dead only when the dump
  held a `false` fact. A pair contradiction, `X` beside `(not X)`, derives
  none, and on [`d3-unsat-core.ein`](../../examples/ein-bugs/d3-unsat-core.ein)
  12 of the 32 nodes die that way. The stage fixed it: a node is dead also
  when `--json-summary`'s `region.root` says so, which is the engine's own
  test. `region_dump.rs::the_oracle_and_the_per_subset_route_agree_node_for_node`,
  which ran it on `d3-unsat-core` as well, is gone: the command at the head
  runs 0 tests today (1.8 s).
- **Staleness needs provenance**, which no CLI output carries, and the
  control's 428 639 nodes need a process-free node.

So both routes are read in process, and `region.py` is the check that the
library's scratch route is `region.py`'s. No flag reaches the prototype, and
no command reaches it.

---

## 1. The headline

| | count |
|---|---:|
| flagged cubes within the limit | 52, **656 nodes** |
| … nodes where fork and scratch give a different status | **17** |
| … nodes where they hold different facts | 621; **34 beyond the `open` facts**, the 17 above among them |
| … programs on which the fork read here is not the oracle's node | 0 |
| S1e2.4.1's probes | not re-taken: none loads |
| monotone cubes within the limit (the control) | 650, **428 639 nodes**: 0 statuses differ; 361 858 differ in their facts, not split by kind (§ 3) |
| facts only scratch holds, on any node of any cube | **0** |
| nodes holding a stale fact, by fork | 29 flagged, 0 monotone |
| … from scratch | **1** flagged (`10_fifo_gap`), **1** monotone (`11_fifo_gap_static`) |
| programs with a cycle through an odd `absent` over D (F21 D2's (D)), of the 748 that load and saturate | **4**: `features/07`, `f07-fpoff`, `f07-rules-swapped`, `d3-unsat-core` |
| `features/12` | n = 0 today: root alone, dead by fork and from scratch |
| levers that move either route: `--jobs`, 8 interners, every flag | **none**, on 52 flagged cubes and the 357 programs of the cut control (§ 7) |
| … equal-priority rule order | moves **both** routes' universe on `features/07`, `f07-fpoff` and `f07-rules-swapped`, and a state on `10_fifo_gap` |
| a scratch node's saturation against a fork node | ×0.8–20; with its parse and load ×31–677 |

## 2. The flagged cubes

### 2.1 The corpus

*Facts differ* counts every node whose two states differ; *beyond `open`*
leaves out the `open` facts the fork holds and scratch does not.

| program | n | nodes | status differs | facts differ | … beyond `open` |
|---|---:|---:|---:|---:|---:|
| `branching/13_lookahead_naf_world` | 2 | 4 | 0 | 2 | 0 |
| `branching/14_lookahead_unjudgeable` | 2 | 4 | 0 | 2 | 0 |
| `ein-bugs/activator-all-int`, `-nested`, `-symbol` | 1 | 2 each | **1** each | 2 each | **1** each |
| `ein-bugs/alive-empty-interlayer` | 3 | 8 | 0 | 8 | 0 |
| [`ein-bugs/d3-unsat-core`](../../examples/ein-bugs/d3-unsat-core.ein) | 5 | 32 | 0 | 31 | 0 |
| `ein-bugs/naf-upward-closure`, `wrong-answers/naf-nowb` | 2 | 4 each | 0 | 3 each | 0 |
| the seven `wrong-answers/activator-chain-*` and two `core-activator-*` witnesses | 2 | 4 each | 0 | 4 each | 0 |
| `wrong-answers/derived-activator`, `-never`; `features/19`, `/20` | 6 | 64 each | 0 | 63 each | 0 |
| [`wrong-answers/f07-fpoff`](../../examples/ein-bugs/wrong-answers/f07-fpoff.ein), `f07-rules-swapped` | 1 | 2 each | 0 | 2 each | **1** each |
| `wrong-answers/kw-neq-in-absent`, `-twin` | 3 | 8 each | 0 | 8 each | **4** each |
| the ten `wrong-answers/oblig-*` witnesses | 1–4 | 2–16 | 0 | 1–7 | 0 |
| `wrong-answers/open-derived-activator` | 2 | 4 | 0 | 4 | **4** |
| `wrong-answers/rung-connex-import` | 4 | 16 | **12** | 16 | **12** |
| [`wrong-answers/stale-naf-fork`](../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein), `-warn` | 1 | 2 each | **1** each | 2 each | **1** each |
| [`features/07_unstratifiable`](../../examples/features/07_unstratifiable.ein) | 1 | 2 | 0 | 2 | **1** |
| `features/23_a_rule_reads_open` | 1 | 2 | 0 | 2 | **2** |
| `lattice/04`, `08`, `09`, `10_fifo_gap`, `12`, `17`; `synth/local_not_global`, `maximal_not_top`, `three_components`, `trigger_hole` | 2–7 | 4–128 | 0 | all | 0 |

**Every node where the two differ beyond the `open` facts**, 34 on 13
programs, with the facts only the fork holds and the rule that derived each:

| program | nodes | fork | scratch | facts only fork holds, beyond `open` |
|---|---|---|---|---|
| `stale-naf-fork`, `-warn` | `{(q one)}` | dead, refuted by `p-and-q-clash` | in R | `(p one)` ← `derive-p`, a site's rule; `(false)` ← `p-and-q-clash` |
| `features/07`, `f07-fpoff` | `{(q one)}` | in R | in R | `(p one)` ← `derive-p`, a site's rule |
| `f07-rules-swapped` | `{(p one)}` | in R | in R | `(q one)` ← `derive-q`, a site's rule |
| `kw-neq-in-absent`, `-twin` | the 4 nodes holding `(neqw a)` (`(nneqw a)`) | in R | in R | `(hit a)` ← `r`, a site's rule |
| `open-derived-activator` | all 4 | in R | in R | `(q (p k))`, `(q (q k))` ← `copy`, a site's rule |
| `features/23_a_rule_reads_open` | ∅ (owing), `{(r a b)}` | as scratch | as fork | `(seen r)`, `(seen (r a b))` ← `noted`, a rule that reads an `open` fact |
| `activator-all-int`, `-nested`, `-symbol` | the one candidate's node | dead | in R | `(watch …)` ← `flag`, `(alarm …)` ← `watch`, `(false)` ← `alarm` |
| `rung-connex-import` | 12 of 16 | dead | in R | `(false)` ← `connex`, a site's rule |

**Why each differs.** It is one mechanism every time. Root derives a fact by
a rule under an `absent`, and the commitment falsifies that guard. The fork
starts from root's fixpoint, so it holds the fact and never withdraws it.
Scratch writes the commitment before the boundary judges any guard, so the
rule never fires. On `stale-naf-fork` the stale `(p one)` meets `(q one)` in
`p-and-q-clash`, and the node dies. On `activator-all-*` and
`rung-connex-import` **root itself is dead** on such a fact (a chain of
activators from `flag`'s `(absent (checked ?x))`; `std.algebra`'s `connex`
over `(absent (lt A C))` and `(absent (lt C A))`), so every fork is dead and
scratch, with the commitment given, is alive. Elsewhere nothing refutes, and
the node is in R on both routes, holding a different state. On
`features/23` the fork's extra facts are read off root's `open` facts, which
scratch does not hold: a rule that reads `open` flags its program
(Q-M1f.569).

**Where they agree.** On the other 39 programs every node agrees in its status
and, beyond the `open` facts, in every fact it holds. That includes the 16
with a hole, so the holes [`hole_census`](hole-census.md) reads are the same
holes under either route. No node on any cube holds a fact that only scratch
holds.

### 2.2 S1e2.4.1's probes

Not re-taken: the 14 files are written with `hrule`, `:priority` and no
`:why`, and none parses today. On 2026-11-02 one node of 206 differed, in its
state only: `not_conclusion`'s `{(q A)}`, by fork holding `(not (p A))` ←
`exclude` under `(absent (q A))`, the same mechanism with a stored negative.
`kw-neq-in-absent` and `open-derived-activator` show the mechanism on the
corpus today.

### 2.3 Programs written into the census

`--written` runs six programs the census carries as text, for readings no
corpus program within the limit shows: four of its own, rewritten in today's
spelling for this re-take, and the two of
[`examples/shared/written.rs`](../../ein.rs/crates/ein-infer/examples/shared/written.rs).
No corpus file is added.

| program | what it is | fork | scratch |
|---|---|---|---|
| `dead_root` | the target `region_synth.py` declines (`RootDead`): ∅ dead, `{(h A)}` alive. One rule, `(and (thing ?x) (absent (h ?x))) → (false)` | R = ∅: root is dead, so every fork is | R = `{(h A)}`: committed, `(h A)` blocks the refutation |
| `dead_root_via` | the same through a derived positive, S1e2.4.1's `via` class: `p ← ¬h`, `p → ⊥` | R = ∅ | R = `{(h A)}` |
| `stratified_stale` | `p ← ¬q` and `q ← ¬r`, `derive-p` written first. **Stratified** | R = {∅, `{(r one)}`}; `{(r one)}` holds `(p one), (q one), (r one)` | R the same; `{(r one)}` holds `(p one), (r one)` |
| `fifo_gap_static` | § 12.2's program as written into the census before it was banked; monotone | R = {∅, `{(b one)}`}; `{(b one)}` holds `(b one) (q one)` | R the same; `{(b one)}` holds `(b one) (p one) (q one)`, `(p one)` stale |
| `activator_refutes`, `activator_under_absent` | `written.rs`'s, each with a dead hole ([hole census § 2](hole-census.md#2-by-cause-and-kind)) | R of 3 and of 7 states | the same, node for node, beyond the `open` facts |

The first two are § 6.3's reading of the synthesizer. The third is § 4's: on a
stratified program, scratch's own root holds a stale fact. Every row is the
stage's, unmoved.

## 3. The control: the monotone cubes

On a program the static check calls monotone, fork and scratch build the same
state by the theorem
([`08-monotonicity.md` § 8.2](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#82-path-independent-if-and-only-if-monotone-given-cumulativity)).
`--control` checks that premise node for node:

| | |
|---|---:|
| monotone cubes within the limit | 650 |
| nodes | 428 639 |
| nodes whose status differs | **0** |
| nodes whose facts differ | 361 858 |
| … beyond the `open` facts | not split: the control keeps no node's facts |
| programs on which the fork read here is not the oracle's node | 0 |
| wall, 16 jobs | 507 s |

On the control the census keeps no node's facts, only a digest, so the
361 858 nodes whose facts differ are not split by kind as § 2.1's are: what
they hold beyond the `open` facts is not read here. What does read facts says
the two routes agree. The stale check, run on every node, finds **one** stale
node, from scratch: `11_fifo_gap_static`'s `{(b one)}` (§ 12.2), the FIFO race
over a static read; none by fork. The path census's control (§ 11.4) finds
that node path-dependent and no other of 19 041. **So on the corpus the two
routes agree in status wherever the static check says they must**, and the
comparison over monotone cubes is evidence for no option, as the stage
expected; the one state that differs is the race, which a *monotone* verdict
does not exclude.

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
to the patterns and counts the fact *inexact*: **0** inexact checks on the
corpus, 1 on the written `activator_refutes`.

| run | nodes | stale nodes by fork | … of them in R | … dead, the death resting on it | stale nodes from scratch |
|---|---:|---:|---:|---:|---:|
| the flagged cubes (§ 2.1) | 656 | **29** | 12 | 14 | **1** |
| the control (§ 3) | 428 639 | 0 | 0 | 0 | **1** |
| the written programs (§ 2.3) | 20 | **4** (`dead_root`, `dead_root_via`, `stratified_stale` 2) | 2 | 2 | **2** (`stratified_stale`'s ∅, `fifo_gap_static`'s `{(b one)}`) |

- **By fork, the stale nodes are the nodes where the routes differ, less six,
  and one more.** 28 of the 34 nodes of § 2.1 that differ beyond `open` hold a
  stale fact; `open-derived-activator`'s 4 and `features/23`'s 2 hold none,
  their extra facts resting on no falsified guard. The 29th is `10_fifo_gap`'s
  `{(a one), (b one)}`, stale on both routes alike (§ 12.1). By program:
  `rung-connex-import` 12 (dead on it), `kw-neq-in-absent` and its twin 4 each
  (in R), `activator-all-*` 1 each (dead, its death not resting on it: root is
  dead), `stale-naf-fork` and its twin 1 each (dead on it), `features/07`,
  `f07-fpoff`, `f07-rules-swapped` 1 each (in R), `10_fifo_gap` 1 (in R).
- **From scratch, one corpus node is stale on each side**: `10_fifo_gap`'s
  `{(a one), (b one)}`, and the control's `11_fifo_gap_static` `{(b one)}`
  (§ 12.2), the FIFO race inside a node; no other node of the 52 flagged cubes
  or the 650 monotone ones.

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
rules in the agenda's order, FIFO in written order within each half
(Q-M1f.313), not in stratum order: both wait at root's first boundary in the
same half, and `derive-p`, written first, goes first. The program's perfect
model at ∅ is `{(q one)}`. Swapping the two rules' text gives that model and a
different universe (§ 7.1). On the corpus it is `10_fifo_gap` and
`11_fifo_gap_static` (§ 12). It is what (B) leaves open on a stratified
program until admission follows the strata.

## 5. The stratification check

**The graph.** Its nodes are relations as the static check names them, `R` or
`not` (a stored negation's relation is `not`), taken from the saturation plans
root compiled. Each plan gives an edge from every relation it reads to every
relation it writes, **negative** when the read is at odd `absent` depth. A
rule with an activator adds a positive edge from its activator relation. A
**cycle through an odd `absent`** is a strongly connected component that holds
a negative edge. *Over D* keeps only the edges between relations in D. The
census also says whether some cycle in the component has an **odd** number of
negative edges, where a stable model need not exist, or every cycle an even
number, where there can be several.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --strata    # 11 s
```

Over the 748 programs of the selection that load and saturate:

| scope | programs with a cycle through an odd `absent` |
|---|---:|
| **over D**, what F21's D2 option (D) would read | **4** |
| over all relations | 7 |

| program | n | flagged | over D | the cycle | parity |
|---|---:|---|---|---|---|
| [`ein-bugs/d3-unsat-core.ein`](../../examples/ein-bugs/d3-unsat-core.ein) | 5 | yes | **yes** | `fire-1` reads `not` under `absent` and writes `r1` (`not` →¬ `r1`) | odd |
| [`features/07_unstratifiable.ein`](../../examples/features/07_unstratifiable.ein) | 1 | yes | **yes** | `derive-p`: `q` →¬ `p`; `derive-q`: `p` →¬ `q` | even |
| [`ein-bugs/wrong-answers/f07-fpoff.ein`](../../examples/ein-bugs/wrong-answers/f07-fpoff.ein) | 1 | yes | **yes** | the same | even |
| `ein-bugs/wrong-answers/f07-rules-swapped.ein` | 1 | yes | **yes** | the same, the two rules written the other way | even |
| [`syntax/rule-forall-and-not.ein`](../../examples/syntax/rule-forall-and-not.ein) | 0 | no | no | `f` reads `r` under a `forall` and writes `r` and `not` | odd |
| [`syntax/rule-guards.ein`](../../examples/syntax/rule-guards.ein) | 0 | no | no | `g`: `s` →¬ `s` | odd |
| [`syntax/rule-shared-guard-var.ein`](../../examples/syntax/rule-shared-guard-var.ein) | 0 | no | no | `sh`: `r` →¬ `r` | odd |

**(D) as a refusal is ruled out** (Q-M1f.564): the four programs load,
flagged, and are answered under the warning. What the count still says is
where a stratified reading has no stratum order: those four, and no other
program of the 748. Three of the four are W8's witnesses, `features/07` and
its two twins, whose right answer is what D2 rules; the fourth is
`d3-unsat-core`, a fuzz finding. `stale-naf-fork` is stratified (`q` is no rule's
conclusion), and (D) does not reach it, as the stage said.

**Over all relations** three syntax fixtures add a cycle. Each states no
candidate, n = 0, and D is empty. That is the case for reading (D) over D, as
the stage does.

**What the graph does not see.** A rule with an activator that no fact
activates at root has no plan, so it has no edges. A program whose node can
compile a plan root lacks (the activator chains of
[`hole_census.md` § 2](hole-census.md#2-by-cause-and-kind),
`activator-chain-node-writes` of [`taint_census.md` § 3](taint-census.md#3-the-readings-of-tainted-side-by-side))
would need the check re-run on the node's plans, which the census does not
do.

## 6. The three known cases

### 6.1 The `-y` abort on `d3-unsat-core`

Before the switch, `-y` (`lattice-sanity-check`) aborted on
`d3-unsat-core.ein` with *"Saturation commutativity violated"*. The stage's
census ran the library function `-y` ran on every node of two facts or more,
one-step parents as `-y` built them, and found 8 nodes of `d3-unsat-core`, 2
of `lattice/08` and 1 of the probe `via_hole` where a parent path reached
another state. **§ 11.2 corrected it**: `d3-unsat-core`'s four dead nodes were
`-y`'s direct path stopping at its first contradiction, and built to
quiescence they agree. That check is no column of this census today: it is the
read-out's `paths` row, the detector's spot check, which on `lattice/08` prints
*path-dependent — 1 of 20 parent paths checked reaches another state*. § 11
reads every path.

### 6.2 L0 on `features/12`

On 2026-11-02 `features/12_expect_false.ein` had 20 candidates, past the
oracle, and a dead root, and the stage ran its whole scratch cube (all
1 048 576 nodes dead, 23 s). **Today it has none**: the static check calls it
monotone, `ein solve` prints *no candidate: R is {∅} or empty, decided by root
alone*, and the cube is root alone, dead by fork and from scratch:

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- \
    --scratch-cube --jobs 16 examples/features/12_expect_false.ein                                          # 0.6 s, 1 node
```

L0's *reason* is the stage's: under scratch a dead root does not kill its
supersets, as `dead_root` (§ 2.3) shows with `{(h A)}` in R above a dead ∅,
and as `activator-all-*` and `rung-connex-import` now show on the corpus
(§ 2.1). Under (B), a flagged program with a dead root needs another licence,
or the cube.

### 6.3 The synthesizer's dead ∅

`region_synth.py` declines a target with ∅ dead under a non-empty R
(`RootDead`), because every fork of a dead root is dead. The census's
`dead_root` and `dead_root_via` are that target, written by hand, one
directly and one through a derived positive:

| option | `dead_root`, `dead_root_via` |
|---|---|
| (A) | R = ∅. The target cannot be written |
| (B) | R = `{(h A)}`. The target can be written |
| (C) over (A) | R = ∅; `{(h A)}` is the third outcome, dead with its death resting on a stale fact |
| (C) over (B) | as (B) |
| (D) | both are stratified, and a cycle check finds nothing |

Under the stage's *forbid*, `dead_root` was the shape refused at load; today
it loads, flagged (Q-M1f.564). So under (B) the synthesizer could write the
target. The corpus has the shape today: `activator-all-int`, `-nested`, `-symbol` and
`rung-connex-import` are dead at root by fork and alive from scratch above it.
No program is refused for it (Q-M1f.564).

## 7. Invariance

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance                         # the flagged cubes, 20 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance --written                # 1.1 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance --control --max-n 9    # cut: see below
```

Each route is re-run under four levers, and each run is compared with the
route's own default run, node by node, keyed by the commitment's text:

- **`--jobs`.** Fork: `region::compute` at 1 and at 16 jobs, status and state
  class of every node. Scratch: the node fan-out at 1 and at 16 threads.
- **Permuted interners.** 8 seeds, `id_order_invariance`'s (1 + 7919 i). Each
  route is rebuilt on a `Terms` whose id space is a permutation of the default
  run's, by that test's `permuted`, copied into the census.
- **Equal-priority rule orders.** Every order of the program's `(rule …)`
  forms within each half of the agenda: all of them where there are at most
  24, else the reverse and 8 seeded shuffles.
- **Every `(config …)` flag** flipped from its default. A flag the program
  already sets is skipped.

A run moves **R** when a node's status moves, a **state** when a node's facts
move, and the **universe** when the candidate list moves. On every run below
the two routes moved on the same runs, so one count stands for both:

| run | cubes | `--jobs` | interners | rule orders | flags |
|---|---:|---|---|---|---|
| the flagged cubes | 52 | 0 of 52 | 0 of 416 | **3 of 448** move the universe (`features/07`, `f07-fpoff`, `f07-rules-swapped`); **12** move a state (`10_fifo_gap`) | 0 of 104 |
| the written programs | 6 | 0 of 6 | 0 of 48 | **1 of 39** moves the universe (`stratified_stale`); **3** move a state from scratch alone (`fifo_gap_static`) | 0 of 12 |
| the control, **cut**: 357 programs of the selection read | 357 | 0 of 357 | 0 of 2 856 | 2 of 1 004 move the universe (`features/07`, `f07-fpoff`); 12 a state on both routes (`10_fifo_gap`); **3 a state from scratch alone** (`11_fifo_gap_static`) | 0 of 711 |

**The control was not taken whole.** Run as the stage ran it, one thread,
`--invariance --control --max-n 9` read 29 programs in its first 5 minutes,
where the stage's read 149 in 10 s. It was cut and run again in eight shards
of the selection's 933 files at once; a shard given its files explicitly
filters none, so the shards read flagged programs and cubes past n = 9 as
well. Two shards ended on `stdlib/math/int.ein` and
`stdlib/math/positional.ein`, read as programs, past 24 GB of memory; four
were cut after 21 minutes on cubes larger than the stage's bound
(`binary_logic/` and others). The 357 programs read, 83 of them under
`stdlib/` and `tests/stdlib/`, move nothing but the four above. What is not
read is the rest of the 650 monotone cubes; machine time spent, about 5 CPU-hours.

### 7.1 What rule order moves

**`features/07`, `f07-fpoff` and `f07-rules-swapped`.** Write `derive-q`
before `derive-p`, and root derives `(q one)` where it derived `(p one)`. The
candidate list becomes `{(p one)}`, and R is `{∅, {(p one)}}`, the mirror image.
That happens **on both routes**; `f07-rules-swapped` is the program written in
the other order. The oracle's own R on `features/07` depends on which of two rules in one half
of the agenda is written first, which is the config principle's *stable*
([`08-monotonicity.md` § 11.1](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#111-three-words):
*"it does not depend … on rule order"*) broken by today's node, not only by
scratch. The program is flagged, so nothing is claimed of either answer.

**`10_fifo_gap`.** Of 23 orders, 12 move the state of `{(a one), (b one)}` on
both routes, and none moves a status or the universe: the FIFO race of § 12.1.

**`stratified_stale`**, written. With `derive-q` first, root is `{(q one)}`,
the program's perfect model, and the universe grows to `{(p one), (r one)}`:
R is ∅, `{(p one)}`, `{(r one)}` and both, on both routes. This one is
stratified, so a stratification check does not reach it (§ 4).
**`fifo_gap_static`**, written, and `11_fifo_gap_static` in the control: the
three orders that put `r1` before `r2` move scratch's state of `{(b one)}`, and
nothing of the fork's.

### 7.2 What settles it

- **A stratification check** reads `features/07` and its two twins: of the
  52 flagged cubes and 357 programs of the control this run reorders, they are
  the only ones whose universe
  depends on rule order. As a refusal, (D), it is ruled out (Q-M1f.564).
- **(C) does not settle it.** Over (A), R is `{∅}` in both orders, but ∅'s state
  is `{(p one)}` in one and `{(q one)}` in the other. Over (B), both orders keep
  both stable models, under different universes. (C) removes stale states, and
  each order's states are stale-free.
- **On a stratified program**, what settles it is admitting rules in stratum
  order. Then scratch's root is the perfect model in every order (§ 4's
  argument); on the corpus that moved `10_fifo_gap`'s one state under the
  stage's emulation (§ 12.4, not re-taken).

## 8. Cost

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cost              # the flagged cubes, 11 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cost --control    # the ten monotone cubes with n ≥ 12, 4 096 nodes each, 128 s
```

One thread, each node read to its two bits only. **Fork**: branch root, write
the commitment, resume, saturate, detect, tally. **Scratch**: parse and load
the text with the commitment, saturate, detect, tally, with the saturation's
share (from the end of the load) shown on its own. **Derived** is the facts
the node's own saturation adds. For scratch it is beyond the load, which
re-derives root's. **For fork the column is not read today**: it is the
node's facts less root's and the commitment's, and root's count includes the
`open` facts its pass writes in a layer no fork inherits, so where a fork
holds fewer facts than that sum the census's count wraps below zero and
prints a number near 2⁶⁴ divided by the nodes timed (`4.6e18` on a cube of 4
nodes). It is printed as `—` below.

| program | n | fork µs | scratch µs | × | saturation alone µs | × | derived: scratch | root |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `activator-all-int` | 1 | 3.8 | 368.6 | 95.8 | 20.2 | 5.3 | 1.5 | 4 |
| `d3-unsat-core` | 5 | 11.7 | 1053.9 | 90.1 | 92.8 | 7.9 | 7.8 | 11 |
| `naf-upward-closure` | 2 | 4.8 | 252.1 | 52.1 | 8.1 | 1.7 | 0.3 | 2 |
| `oblig-stdlib-total-owed` | 1 | 18.7 | 3625.2 | 193.9 | 114.5 | 6.1 | 5.0 | 6 |
| `rung-connex-import` | 4 | 4.7 | 3154.1 | 677.3 | 93.9 | 20.2 | 2.3 | 7 |
| `stale-naf-fork` | 1 | 5.2 | 279.6 | 54.2 | 10.6 | 2.1 | 0.5 | 2 |
| `features/07_unstratifiable` | 1 | 2.5 | 281.6 | 112.2 | 12.7 | 5.0 | 0.5 | 2 |
| `lattice/04_obligation_trigger_hole` | 2 | 16.0 | 3334.3 | 208.5 | 128.6 | 8.0 | 6.5 | 8 |
| `lattice/08_absent_witness_hole` | 4 | 14.6 | 3414.3 | 234.5 | 151.7 | 10.4 | 8.1 | 13 |
| `lattice/10_fifo_gap` | 2 | 6.5 | 388.6 | 59.7 | 12.6 | 1.9 | 0.8 | 2 |
| `lattice/synth/three_components` | 7 | 13.9 | 713.0 | 51.2 | 27.7 | 2.0 | 0.8 | 8 |
| `lattice/synth/trigger_hole` | 5 | 12.9 | 485.6 | 37.6 | 15.8 | 1.2 | 0.1 | 6 |
| the other 40 flagged | 1–6 | 2.4–24.8 | 217.5–3636.7 | 31.8–243.7 | 5.7–156.6 | 0.8–8.3 | | |
| **control**, monotone, n ≥ 12 | | | | | | | | |
| `binary_logic/synthesis` | 16 | 145.8 | 8145.5 | 55.9 | 1966.7 | 13.5 | 254.9 | 255 |
| `branching/01_saturate_only` | 16 | 60.9 | 1289.1 | 21.2 | 155.6 | 2.6 | 22.1 | 28 |
| `underscore-core-two-ways-twin` | 14 | 7.9 | 401.7 | 51.0 | 18.9 | 2.4 | 4.3 | 18 |
| `features/02_star_in_identifiers` | 15 | 57.5 | 420.7 | 7.3 | 74.3 | 1.3 | 16.7 | 25 |
| `lattice/07_two_pairs_two_tables` | 16 | 73.7 | 3613.5 | 49.0 | 211.2 | 2.9 | 9.0 | 25 |
| `lattice/14_seats_nothing_proposes` | 16 | 73.2 | 3934.8 | 53.8 | 236.0 | 3.2 | 11.0 | 27 |
| `saturation/type-exclusivity/pets` | 15 | 14.5 | 990.4 | 68.2 | 76.9 | 5.3 | 13.2 | 25 |
| `syntax/constraint-scopes` | 12 | 4.5 | 214.6 | 48.0 | 1.6 | 0.4 | 0.0 | 12 |
| `syntax/equality` | 12 | 4.3 | 189.2 | 44.1 | 1.6 | 0.4 | 0.0 | 12 |
| `tests/stdlib/bijection/06_blind_enumeration` | 16 | 114.4 | 6300.9 | 55.1 | 410.7 | 3.6 | 30.6 | 31 |

The µs are means over every node, or over 4 096 nodes spread evenly over the
cube where it has more. Two runs of the flagged cubes, 2026-11-25: a mean moved
by up to 23 % and a ratio by up to 33 % between them, a few µs each over 2–128
nodes; the table is the first run. The control was run once, in 128 s.

- **The saturation scratch adds is small**: ×0.8–20 of the fork's whole node
  on the flagged cubes, ×0.4–13.5 on the control. It re-derives root at every node:
  scratch's derived column is root's and the node's own.
- **Parse and load dominate the prototype**, and more than on 2026-11-02:
  ×31–677 with them on the flagged cubes, ×7.3–68 on the control. A program that imports a module of the standard library
  pays about 3.2 ms a node (`lattice/04`, `08`, `09`, `12`, `17`, the `oblig-*`
  witnesses over `std.algebra`), and every program about 0.25 ms, since every
  load carries `std.core`. A build of (B) inside the engine would branch a
  loaded, unsaturated program as the fork branches root, and would not pay
  them per node. That is an argument; this prototype does not measure it.

## 9. W8's two witnesses, computed

Each option's R, from the route that option defines, on the two witnesses and
their twins. W8's rows left the register with Q-M1f.564 and are F21's; the
witnesses are corpus programs. Both twins give their witness's answer on every
option (§ 2.1). The node census prints these as `A_fork`, `B_scratch`,
`C_over_A` and `C_over_B`. "State" is the facts beyond the program's own and
beyond the `open` facts.

**`features/07_unstratifiable.ein`** (twins `f07-fpoff`, and `f07-rules-swapped`
in the mirror):

| option | R | k_s | maximal line | its state |
|---|---|---:|---|---|
| (A) fork | ∅, `{(q one)}` | 2 | `{(q one)}` | `(p one) (q one)`, which the fixture's header says is *"not a model of this program under any reading"* |
| (B) scratch | ∅, `{(q one)}` | 2 | `{(q one)}` | `(q one)`. ∅'s state is `(p one)`: the program's two stable models, one per node |
| (C) over (A) | ∅ | 1 | ∅ | `(p one)`; `{(q one)}` is the third outcome, alive and holding a stale fact |
| (C) over (B) | as (B) | | | |
| (D) | — | | | a refusal, ruled out (Q-M1f.564) |

**`stale-naf-fork.ein`** (twin `stale-naf-fork-warn`):

| option | R | k_s | maximal line | its state |
|---|---|---:|---|---|
| (A) fork | ∅ | 1 | ∅ | `(p one)` |
| (B) scratch | ∅, `{(q one)}` | 2 | `{(q one)}` | `(q one)` |
| (C) over (A) | ∅ | 1 | ∅ | `(p one)`; `{(q one)}` is the third outcome, dead with its death resting on a stale fact |
| (C) over (B) | as (B) | | | |
| (D) | not reached: stratified | | | (A), (B) or (C), as composed |

**The rows' hand derivation was (B)'s answer, not (C)'s over (A).** Both
rows' `right` block was labelled *guards are premises*, and both put
`{(q one)}` in R with the state `(q one)`: *"committing (q one) withdraws (p
one)"*. That is retraction, and computed it is (B), or (C) over (B). (C) over
the fork does not withdraw `(p one)`. It disqualifies the state that holds
it, and gives R = {∅} on both programs. So *guards are premises* names two
different answers, depending on which node it is read over. The rows are
F21's to rule; every number above is the stage's, unmoved.

## 10. The options against the evidence

| option | what moves, measured | stable under rule order | cost |
|---|---|---|---|
| **(A) fork** | nothing | **no** on `features/07` and its twins (§ 7.1) | as today |
| **(B) scratch** | the 17 nodes of § 2.1 whose status differs: `stale-naf-fork` and `-warn` gain `{(q one)}` in R; `activator-all-*` gain their one candidate's node and `rung-connex-import` 12 of its 16, above a root dead by fork; and 17 more nodes change their state beyond `open` (`features/07` and its twins, `kw-neq-in-absent` and its twin, `open-derived-activator`, `features/23`). No node of the 650 monotone cubes but `11_fifo_gap_static`'s `{(b one)}` (§ 12.2). L0 is no longer the node's construction on a flagged program with a dead root | no on `features/07` and its twins; no on `10_fifo_gap` and the written `stratified_stale` until admission follows the strata | saturation ×0.8–20 a node; root re-derived at every node |
| **(C) guards are premises** | over (A): `features/07` and its twins lose `{(q one)}` (`{(p one)}`) from R; `stale-naf-fork`'s `{(q one)}` becomes the third outcome; the in-R stale nodes of `kw-neq-in-absent` and its twin and `10_fifo_gap` leave R. Over (B): `10_fifo_gap`'s `{(a one), (b one)}` and `11_fifo_gap_static`'s `{(b one)}` | does not settle it (§ 7.2) | a matcher call per fact with an `absent` premise |
| **(D) refuse** | ruled out: Q-M1f.564 | — | — |

### The stage's recommendation, re-examined (mine)

The stage recommended **(B) with (D)**, amended to admit candidates in stratum
order. Q-M1f.564 rules out (D) as a refusal: a program with a cycle through an
odd `absent` loads, flagged. What stands of the rest:

- **(B) with admission in stratum order** makes a stratified program's node
  its perfect model, whatever order its rules are written in (§ 4's argument,
  measured under the stage's emulation, § 12.4), and a node a function of its
  commitment, with no path (§ 6.1, § 11).
- **What it moves on the corpus** is § 10's (B) row: 17 statuses, all on
  flagged programs, 15 of them above a root that is dead by fork. That every
  corpus scratch node is stale-free but the FIFO race's two still holds.
- **What it costs**: root re-derived inside every node, ×0.8–20 a node in
  saturation; L0 stops being a construction on a flagged program with a dead
  root.
- **On an unstratifiable program** (`features/07` and its twins,
  `d3-unsat-core`), no option above gives one answer, and nothing is claimed
  of one today.

## 11. The path census — S1e2.1b.7

S1e2.1b.7's T1e2.1b.7.2, for
the rows [`02-saturation-and-negation.md` § 4.4 What holds it](../../docs/ein/ein.rs/08-reasoning-in-code/02-saturation-and-negation.md#44-what-holds-it)
left *unverified*: every split C1 ⊆ C and longer chains, the state-memoized
path DAG and (F)'s per-node counts, and the premise that a step is a function
of the KB's contents. § 6.1 above read one-step parents, as `-y` built them.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths                    # § 11.2, the flagged cubes, 12 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths --written          # § 11.3, 0.6 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths --control --max-n 12   # § 11.4, 62 s
```

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
  level within the bound. Within the limit no flagged cube has n > 7, so on
  them every maximal chain is read.
- **The premise**: every step any construction takes is keyed by *(its
  parent's facts, what it adds)*. A key met twice is a step from the same
  contents with another history. Two that differ in their result would be a
  **conflict**.
- **The checks on the route**: whether the fork built here is `region::Nodes`'
  fork, the oracle's node, fact for fact and bit for bit, and whether scratch
  built here is § 2's text load, fact for fact, on every node read.

Per node it counts the distinct states the constructions reach. A node is
**path-dependent** when there are two or more. *Alive paths* keeps the states
reached through no dead intermediate state, root included, which is what a
route that never extends a dead node meets. *From root* keeps the paths that
start at sat(P), the path tree as F21 states it. (F) is read both ways: the
states of the node whose facts are not stale, from P and from root.

### 11.2 The flagged cubes

52 programs, 656 nodes, 34 411 steps, 22 593 chain-tree nodes:

| | count |
|---|---:|
| the fork here is the oracle's node, on every node | **0 of 52** programs: see below |
| scratch here is the text load, on every node | **52 of 52** |
| steps from contents met before (the premise checked) | 28 466 |
| … with another result (a conflict) | **0** |
| path-dependent nodes | **51**, on 24 programs |
| … through alive paths only | 20 |
| … reaching a state that is neither the fork's nor scratch's | 23 |
| nodes whose status depends on the path | 34 |
| … through alive paths only | **3** |
| (F) from P: stale-free states per node | **exactly 1 on all 656** |
| (F) from root | 1 on 628; **0 on 28** |

**The fork here is not the oracle's node on any program**, where it was on 18
of 18: the comparison is of the two forks' facts, and `region::Nodes`' fork
holds the `open` facts root holds, which a step of this census (`Saturator`
alone, no pass) does not write. Its statuses and every other count are of the
census's own constructions, each built the same way, so they compare with one
another as the stage's did.

| program | n | nodes | path-dependent (alive paths) | beyond fork and scratch | status differs (alive paths) | splits differ | chains differ | what differs |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `d3-unsat-core` | 5 | 32 | 4 (4) | 4 | 0 (0) | 4 | 4 of 32 | `(r1 o2 o2)` ← `fire-1 (absent (not (r1 o3 o3)))`, kept by way of a parent without `(r1 o3 o2)`, and stale there |
| `naf-upward-closure`, `naf-nowb` | 2 | 4 | 1 (0) | 1 | 1 (0) | 1 | 1 of 4 | `{(p A), (q A)}` is dead by way of the dead `{(p A)}`, which keeps `(false)` ← `bad (absent (q A))` |
| the six `activator-chain` witnesses with a dead hole | 2 | 4 | 1 (0) | 1 | 1 (0) | 1 | 1 of 4 | `{(p a), (p2 a)}` is dead by way of the dead `{(p a)}`, which keeps the chain's first fact, `(q a)` ← `mk (absent (p2 a))` |
| `activator-all-int`, `-nested`, `-symbol` | 1 | 2 | 1 (0) | 0 | 1 (0) | 1 | 0 of 2 | the one candidate's node: dead by way of the dead root, in R from scratch and by a split |
| `rung-connex-import` | 4 | 16 | 12 (0) | 0 | 12 (0) | 12 | 0 of 16 | the same, 12 nodes: `(false)` ← `connex` kept from the dead root |
| `features/07`, `f07-fpoff`, `f07-rules-swapped` | 1 | 2 | 1 (1) | 0 | 0 (0) | 1 | 0 of 2 | the one node: the fork's stale fact against scratch (§ 2.1) |
| `stale-naf-fork`, `-warn` | 1 | 2 | 1 (1) | 0 | **1 (1)** | 1 | 0 of 2 | `{(q one)}`: dead by fork, in R by scratch (§ 2.1) |
| `kw-neq-in-absent`, `-twin` | 3 | 8 | 4 (4) | 0 | 0 (0) | 4 | 0 of 8 | the four nodes holding `(neqw a)`: the fork's stale `(hit a)` against scratch |
| `lattice/08_absent_witness_hole` | 4 | 16 | 2 (2) | 2 | **1 (1)** | 2 | 2 of 16 | by way of a parent without `(allergic Ann)` a node keeps `(meal Ann Soup)`, stale. **The hole `{(allergic Ann), (seats Ann C1)}` is in R that way**, owing by fork and from scratch |
| `lattice/10_fifo_gap` | 2 | 4 | 1 (1) | 1 | 0 (0) | 1 | 1 of 4 | `{(a one), (b one)}`: fork and scratch hold the stale `(p one)`; by way of `{(a one)}` it is not derived (§ 12.1) |
| `synth/three_components` | 7 | 128 | 3 (0) | 3 | 3 (0) | 3 | 3 of 128 | three nodes of R are dead by way of a dead hole, `(false)` ← `dead-3` or `dead-7` kept |
| `synth/trigger_hole` | 5 | 32 | 5 (0) | 5 | 5 (0) | 5 | 5 of 32 | the same, through `dead-1`'s holes |
| the other 28 | | | 0 | 0 | 0 | 0 | 0 | one state per node by every construction |

**What it adds to § 6.1.**

- **`d3-unsat-core`'s four dead nodes of § 6.1 do not depend on the path.**
  Built to quiescence by every construction, each has one state. Its four
  path-dependent nodes are nodes of R, reached with a stale `(r1 o2 o2)` by
  way of a parent.
- **A path through a dead parent** is where `naf-upward-closure`, the six
  `activator-chain` witnesses, `three_components` and `trigger_hole` depend
  on the path: a dead node keeps its `(false)` when the fact that would lift
  it is added. A route that never extends a dead node does not meet it. A
  route that learns from a dead node meets exactly it, which is W3.
  **A dead root** is the same at ∅: on `activator-all-*` and
  `rung-connex-import` every path from root is dead, and scratch and a split
  are alive.
- **Along alive paths the status moves both ways.** On `lattice/08` an owing
  hole is in R by way of its parent, so a route that extends parents would
  report a node of R the oracle calls owing; on `stale-naf-fork` a node of R
  by scratch is dead by fork. On the written `activator_under_absent` (§ 11.3) a
  node of R is dead by way of an alive parent, so the same route would lose a
  node of R.

### 11.3 The written programs

| program | n | nodes | path-dependent (alive paths) | beyond fork and scratch | status differs (alive paths) | (F) from P · from root | what differs |
|---|---:|---:|---:|---:|---:|---|---|
| `dead_root`, `dead_root_via` | 1 | 2 | 1 (0) | 0 | 1 (0) | 1 on all · 0 on 1 | `{(h A)}`: dead by fork, through the dead root; in R from scratch (§ 6.3) |
| `stratified_stale` | 1 | 2 | 1 (1) | 0 | 0 (0) | **0 on ∅** · 0 on both | ∅ has one state, scratch's root, and it holds the stale `(p one)` (§ 4); `{(r one)}` by fork holds two stale facts |
| `fifo_gap_static` | 1 | 2 | 1 (1) | 0 | 0 (0) | 1 on all | `{(b one)}`: the fork's `(q one)` against scratch's stale `(p one)` (§ 12.2) |
| `activator_refutes` | 2 | 4 | 1 (0) | 1 | 1 (0) | 1 on all | `{(p A), (q A)}` is dead by way of the dead `{(p A)}`, through the plan `watch` compiled for `(watch q)` |
| `activator_under_absent` | 3 | 8 | 2 (2) | 2 | **1 (1)** | 1 on all | by way of `{(p A)}` a node keeps `(watch r)` ← `act (absent (q A))`; `{(p A), (q A), (r A)}`, in R, is dead that way |

The premise is met on all of them: 61 repeated keys, 0 conflicts. Every row is
the stage's, unmoved.

### 11.4 The control: the monotone cubes

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths --control --max-n 12   # 62 s
```

**The bound is n ≤ 12**, where the DAG and the splits cost 3ⁿ ≤ 531 441 steps
a program: 643 of the 650 monotone cubes within the limit; the seven at n = 13
to 16 are not read.

| | count |
|---|---:|
| monotone cubes, n ≤ 12 | 643 |
| nodes | 19 041 |
| steps | 3 391 231 |
| chain-tree nodes | 161 461; 7 492 nodes' chains checked |
| steps from contents met before | 1 794 060 |
| … a conflict | **0** |
| path-dependent nodes | **1**: `11_fifo_gap_static`'s `{(b one)}` (§ 12.2) |
| the fork here is the oracle's node | 463 of 643; not on 180, as on § 11.2's flagged cubes |
| scratch here is the text load | 641 of 643 |
| (F) from P and from root | 1 stale-free state on every node |

### 11.5 What it settles, and what it does not

- **The premise holds where it was read**: a step is a function of the KB's
  contents on 1 822 587 steps from contents met before, flagged, written and
  monotone, with no conflict. So memoizing the path tree by the state is sound
  on every cube read here, and the DAG's states at a node are every path's.
- **Path dependence is bounded by two states** on every node read. It occurs
  on 24 of the 52 flagged cubes, on all 6 written programs, and on one
  monotone cube, `11_fifo_gap_static`, the FIFO race.
- **(F)'s counts.** From P, exactly 1 stale-free state on every node of every
  corpus cube read, flagged or monotone; 0 on the written `stratified_stale`'s
  ∅, because the FIFO gap reaches root itself. From root, 0 on 28 flagged
  nodes: wherever root derived a fact the commitment falsifies
  (`features/07` and its twins, `stale-naf-fork` and its twin,
  `kw-neq-in-absent` and its twin) or root is dead (`activator-all-*`,
  `rung-connex-import`). So under (F) a node's count is a property of the
  admission order as much as of the program. No node of any cube read here
  reaches two stale-free states.
- **What it does not read**: the seven monotone cubes at n = 13 to 16, the
  flagged programs past the limit, chains past depth 7, and S1e2.4.1's probes.

## 12. The cumulativity census, and the FIFO gap — S1e2.1b.7

T1e2.1b.7.3, for three rows of
[`02-saturation-and-negation.md` § 4.4 What holds it](../../docs/ein/ein.rs/08-reasoning-in-code/02-saturation-and-negation.md#44-what-holds-it):
*a stratified program evaluated stratum by stratum is cumulative*, *the FIFO
example is path-dependent in the engine as written*, and *a Monotone verdict
can leave scratch ≠ fork through a FIFO race over a static read*.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cumulative                      # § 12.3, the flagged cubes, 11 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cumulative --written            # 0.6 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --cumulative --control --max-n 12   # § 12.3: stopped at 20 GB after 1 248 s, no row
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --stratum-order --jobs 4            # § 12.4: its rewrite no longer loads
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --stratum-order --jobs 4 --written  # § 12.4: the same
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --paths --invariance \
    examples/lattice/10_fifo_gap.ein examples/lattice/11_fifo_gap_static.ein                                                    # § 12.1–12.2, 0.6 s
```

Taken 2026-11-25 at `213e8a92`, as above. The two fixtures are banked: the
flagged selection holds `10_fifo_gap` among its 52, and the monotone one
`11_fifo_gap_static` among its 650.

### 12.1 The FIFO gap, banked

[`examples/lattice/10_fifo_gap.ein`](../../examples/lattice/10_fifo_gap.ein)
is the counter-example as written: r2 `a ∧ b ∧ absent(q) → p` before r1
`a ∧ absent(z) → q`, in one half of the agenda, `a` and `b` the two candidates.
It is stratified (z < q < p), flagged (r2's `absent` reads `q`, which `a`
makes dynamic), and every node is in R. **The engine reproduces the gap as
written**:

| node | built at once (fork; scratch is the same, root derives nothing) | by way of `{(a one)}` |
|---|---|---|
| `{(a one), (b one)}` | `(a one) (b one) (p one) (q one)`, with `(p one) ← r2 (absent (q one))` **stale** | `(a one) (b one) (q one)` |

- **`--paths`**: 1 of 4 nodes depends on the path, through an alive parent;
  the premise holds (0 conflicts).
- **`--cumulative`**: at `{(a one), (b one)}`, adding the derived `(q one)`
  before saturating takes `(p one)` away, by fork and from scratch: X ⊆ Y ⊆
  sat(X) and sat(Y) ≠ sat(X). It is the one node of the 52 flagged cubes where
  cumulativity fails (§ 12.3).
- **`--invariance`**: of 23 orders, 12 move the state of `{(a one), (b one)}`
  on both routes, and none moves a status or the universe (§ 7.1).

### 12.2 The static variant, banked

[`examples/lattice/11_fifo_gap_static.ein`](../../examples/lattice/11_fifo_gap_static.ein)
is the same program with `(a one)` given and only `b` a candidate. `q` is no
relation a commitment can change, so r2's `absent` is static and **the static
check calls the program monotone**.

| node | by fork from root | from scratch |
|---|---|---|
| ∅ | `(a one) (q one)` | the same |
| `{(b one)}` | `(a one) (b one) (q one)` | `(a one) (b one) (p one) (q one)`, `(p one)` **stale** |

Root settles the race before `b` exists, and a fork inherits `(q one)`.
Scratch re-derives root inside the node, where r2 and r1 wait at one boundary
and r2 is admitted first. **So a *monotone* verdict does not make fork and
scratch one route**: the control's one stale node (§ 3), the path census's
one path-dependent monotone node (§ 11.4), the invariance's three scratch
states that move by rule order (§ 7) and the stage's cumulativity control,
which failed from scratch on it alone, are this node.

### 12.3 The cumulativity census

For every node, each construction's X (scratch: P ∪ C; fork: sat(P) ∪ C), and
every fact f its fixpoint derived beyond X: is sat(X ∪ {f}) = sat(X)? One fact
at a time, so it reads the single-fact case of X ⊆ Y ⊆ sat(X).

| run | programs (stratified) | nodes | tests: scratch · fork | change the fixpoint: scratch · fork | where |
|---|---:|---:|---:|---:|---|
| the flagged cubes | 52 (48) | 656 | 1 204 · 504 | **1 · 1** | `10_fifo_gap`'s `{(a one), (b one)}` + `(q one)` |
| the written programs | 6 (6) | 20 | 15 · 6 | **2** · 0 | `stratified_stale`'s ∅ + `(q one)`; `fifo_gap_static`'s `{(b one)}` + `(q one)` |
| the monotone cubes, n ≤ 12 | **not taken**: the run was stopped at 20 GB of memory after 1 248 s (5.4 CPU-hours), before it printed a row; the stage's took 6.7 s | | | | |

S1e2.4.1's probes are not read (none loads).

- **Every failure is a FIFO race on a stratified program**: two `absent` rules
  waiting at one boundary, the reader of the other's conclusion admitted
  first. No failure on an unstratifiable program: `features/07` and its twins
  and `d3-unsat-core` are cumulative on every node, one fact at a time. Their
  path dependence (§ 11) comes from a commitment that falsifies a guard, a
  state function that is not monotone, and not from cumulativity.
- **The engine's saturation is not cumulative on a stratified program in
  general**: it fails where the FIFO race reaches, `10_fifo_gap`,
  `stratified_stale` and `fifo_gap_static`, and on no other node read; the
  monotone cubes were not read today (above), and on 2026-11-02 they failed
  on `11_fifo_gap_static`'s `{(b one)}` alone.

### 12.4 Admission in stratum order, emulated

F21's D2 names *admission in stratum order* as what closes the gap.
`--stratum-order` emulates it without changing the engine: it rewrites the
program's text so that every saturation rule reading under an `absent` is
admitted in the order of its stratum, and reads every node again. A relation's
stratum is the most `absent` edges on a path into it over `strata`'s graph; a
cycle through one has no stratum order. Only rules written in the program's
own text move. It is an emulation, with one gap it does not close: a
candidate whose guard fails now and passes later, at an earlier stratum, is
not waited for.

**Not re-taken: the emulation no longer runs.** It gives each rule the
priority of its stratum, `:priority 10 + its stratum`, and the parser refuses
`:priority` since the agenda took its place (Q-M1f.313): on `10_fifo_gap` the
rewrite moves `r1` to 11 and `r2` to 12, and the text it writes does not load,
so every row reads `null`, on every program. On 2026-11-02, under the
emulation, every stratified program read was cumulative on every node and
every node built from scratch was stale-free; the fork was not repaired by it
(a fork keeps root's facts, so a fact root derived under a guard a commitment
falsifies stays stale); and on the corpus it moved one state, the fixture's.
An emulation today would order rules within the agenda's halves by stratum,
which the census does not do.
