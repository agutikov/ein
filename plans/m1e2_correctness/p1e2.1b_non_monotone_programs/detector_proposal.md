# The detector: the explicit check's form and the warnings, on real programs

**Stage:** [S1e2.1b.5](s1e2.1b.5_the_detector.md), task T1e2.1b.5.1.
**Status:** drafted 2026-11-02. **Approved by the user on 2026-11-03**,
through the question dialog, with all four of § 8's recommendations selected:
(1) the form is **(a1)**, `ein solve FILE --monotone`, exactly as § 2.2 drafts
it, with `ein oracle` as it is (a4), and no `ein check` and no `:sites` claim;
(2) the cube's half is **(i)**, static only, the `bit` row saying *not
computed* and naming `ein oracle` and a bare `ein solve`; (3) the parent-path
spot check **ships**, at ≤ 3 parents of each state the read-out prints; (4) the
wording is **approved as drafted** in § 4.2 and § 4.3: the `assumes`, `site`,
`cycle`, `order`, `status`, `state` and `paths` rows before today's hole rows,
which stay unchanged (their form is F21's D4), the status and the state
reported apart, and the stale check reading every node the run computes. The
user did not object to § 8's *decided here unless the user objects*, so those
details stand as written. The build is S1e2.1b.5's T1e2.1b.5.2–.6. Below, every
*drafted* row is printed by a prototype no route reads, as it was when the page
was put to the user; every row marked *today* is the release binary's at
`e7bec8f0`.
**Taken at:** `e7bec8f0` (the parent of the commit that adds this page),
2026-11-02. Release build from `./build.sh --engine`: `ein 0.1.0`,
`ein-events/1`, features `einb, parallel, snmalloc`, stdlib
`sha256:7899e676…` (the checkout), `-j 1` (the default), on a 32-thread
machine. Walls are one run each.
**How to read it:** § 1 is the programs and what every surface says today.
§ 2 drafts the explicit check in the four forms of the stage's (a), and § 3
asks whether it reports the cube's half. § 4 drafts the warning on every
surface of the stage's (b). § 5 prices the parent-path spot check. § 6 names
the goldens each choice moves. § 7 is what the drafting found on the way.
§ 8 is the recommendation and the **four questions**, the only ones the phase
asks.

```sh
./build.sh --engine
D=plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example detector_draft -- FILE…    # every drafted row; 10 s for § 1's programs
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example detector_draft -- --bound 1 FILE…   # § 5's bound
ein.rs/target/release/ein solve FILE;  ein.rs/target/release/ein oracle FILE                               # today's, § 1
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- $D/*.ein
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example node_census -- --invariance $D/*.ein
```

The long outputs are files beside this page, in
[`detector_proposal/outputs/`](detector_proposal/outputs/): every program's
`ein solve` and `ein oracle` stdout today (`today/`, with `today.tsv` for exit,
wall and line counts), the prototype's whole output
([`detector_draft.txt`](detector_proposal/outputs/detector_draft.txt)), the
spot check at three bounds
([`spot_check_bounds.txt`](detector_proposal/outputs/spot_check_bounds.txt)),
and `node_census` on the four programs written for this page.

**The prototype** is
[`ein-infer/examples/detector_draft.rs`](../../../ein.rs/crates/ein-infer/examples/detector_draft.rs).
It takes the static check through `census::root_of` (what `region::prepare`
does), the cycles through an `absent` from `node_census`'s graph, a new order
check, a bare describe through `explore::run`, the cube's two bits and
`Topology::of` within the limit, the violations attributed by
`shared/attribution.rs` (S1e2.1b.3's rule), the stale-fact check of
[`node_census.md` § 4](node_census.md#4-the-staleness-census), and the spot
check. Then it prints the rows this page proposes. Example code: no route reads
it, and it writes nothing.

---

## 0. What is asked, and under which rulings

The user, 2026-11-02 ([README § The goal](README.md#the-goal-since-2026-11-02)):
*"provide robust way to explicitely check if program is monotone or not,
build-in cheap and fast detector into full `solve` command that is expected to
find complete R"*, and *"be able to at least show warnings about the program
that is not monotonic"*.

The page is built under the standing rulings, none re-opened here:

- **Nothing decided.** No warning chooses a node, refuses a program or moves
  R. What a node is, and what is reported when R has holes, are
  [F21](../../followups/f21_non_monotone_programs_decided.md)'s D1–D7.
- **One tool, consistent answers.** No command can disagree with another, and
  every answer agrees with the oracle.
- **Minimum work per question.** *Is this program monotone?* costs the static
  check, not a solve.
- **Config never changes the answer.** No option turns the detector on or
  off. It runs on a program `monotone::classify` flags, and only there.
- **Every answer exits 0.** Only a refuted claim exits 1.
- **Long output goes to a file.** On stdout the warning names at most 4 sites
  and 4 stale facts, then a `…` row that names `--dump-answer`, which writes
  them whole.
- **The words.** A lattice is neither monotone nor not. A *function of a
  node* is: the **state**, and the **status** (`dead`, `owes`). The third,
  stronger property is the **monotone program**
  ([`monotonicity.md` § 7.6](../../../docs/kernel/inference/monotonicity.md#76-the-program-the-third-stronger-property)).
  *Monotone lattice* appears below only inside today's output, quoted.

## 1. The programs, and what every surface says today

The stage's list, plus three programs written for the census pages and one for
this page. `fifo_gap.ein` stands in for S1e2.1b.7's FIFO-gap fixture, which
does not exist yet. It is
[`monotonicity.md` § 7.3](../../../docs/kernel/inference/monotonicity.md#73-the-state-path-independent--monotone-given-cumulativity)'s
counter-example written out: r2 `a ∧ b ∧ absent(q) → p` before r1
`a ∧ absent(z) → q`, both at priority 200, `a` and `b` the candidates. It is a
scratch file beside this page, not banked, and **drafted**. So are the copies
of `node_census`'s `stratified_stale` and `written.rs`'s
`ACTIVATOR_UNDER_ABSENT`. No test reads any of the three.

| program | n | what it shows | `lattice` row today | `ein oracle`'s closing line, exit 0: dead nodes with an alive superset · discharged nodes with an owing superset | `ein solve`: exit · wall · stdout lines |
|---|---:|---|---|---|---|
| [`zebra.ein`](../../../examples/zebra.ein) | 56 | monotone | `monotone (the static check) — R is convex: no hole` | refused, exit 2: 2^56 | 0 · 1 853 ms (partial at the budget) · 59 |
| [`branching/13_lookahead_naf_world`](../../../examples/branching/13_lookahead_naf_world.ein) | 2 | conservative | `… 1 site … — no hole in this cube; assumes a monotone lattice …` | 0 · 0 — *the flagged sites break nothing here* | 0 · 6.7 ms · 39 |
| [`ein-bugs/naf-upward-closure`](../../../examples/ein-bugs/naf-upward-closure.ein) | 2 | an `absent` that refutes | `… — 1 hole; …` + 1 hole row | 1 dead node with an alive superset · 0 | 0 · 7.3 ms · 35 |
| [`lattice/08_absent_witness_hole`](../../../examples/lattice/08_absent_witness_hole.ein) | 3 | an `absent` that takes a witness away | `… — 1 hole; …` + 1 | 0 · 1 discharged node with an owing superset | 0 · 4.2 ms · 44 |
| [`lattice/04_obligation_trigger_hole`](../../../examples/lattice/04_obligation_trigger_hole.ein) | 2 | a trigger | `… — 1 hole; …` + 1 | 0 · 1 | 0 · 4.5 ms · 35 |
| [`lattice/09_activator_hole`](../../../examples/lattice/09_activator_hole.ein) | 2 | an activator | `… — 1 hole; …` + 1 | 0 · 1 | 0 · 7.7 ms · 35 |
| [`ein-bugs/wrong-answers/stale-naf-fork`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein) | 1 | fork ≠ scratch (W8) | `… — no hole in this cube; …` | 0 · 0 — *break nothing here* | 0 · 7.1 ms · 27 |
| [`features/07_unstratifiable`](../../../examples/features/07_unstratifiable.ein) | 1 | an even cycle; rule order moves the universe | `… 2 sites … — no hole …` | 0 · 0 — *break nothing here* | 0 · 6.7 ms · 24 |
| [`corpus/fuzz_findings/d3-unsat-core`](../../../corpus/fuzz_findings/d3-unsat-core.ein) | 5 | an odd cycle | `… — no hole …` | 0 · 0 — *break nothing here* | 0 · 8.3 ms · 68 |
| [`fifo_gap.ein`](detector_proposal/fifo_gap.ein), *drafted* | 2 | the FIFO gap | `… — no hole …` | 0 · 0 — *break nothing here* | 0 · 6.9 ms · 36 |
| [`stratified_stale.ein`](detector_proposal/stratified_stale.ein), *drafted* | 1 | the FIFO gap, stratified, at root | `… 2 sites … — no hole …` | 0 · 0 — *break nothing here* | 0 · 7.1 ms · 32 |
| [`activator_under_absent.ein`](detector_proposal/activator_under_absent.ein), *drafted* | 3 | an activator derived under an `absent` | `… 2 sites … — 1 hole; …` + 1 | 1 · 0 | 0 · 6.9 ms · 35 |
| [`via_hole.ein`](../p1e2.4_what_the_switch_left/s1e2.4.1_probes/via_hole.ein) (probe) | 3 | `via` | `… — 1 hole; …` + 1 | 1 · 0 | 0 · 6.9 ms · 43 |
| [`lattice/06_friends_tables`](../../../examples/lattice/06_friends_tables.ein) | 19 | past the limit, a trigger | `… — holes not known: the cube was not enumerated; …` | refused, exit 2: 2^19 | 0 · 2 447 ms · 114 |
| [`features/04_open`](../../../examples/features/04_open.ein) | 81 | past the limit, an `absent` | as `06` | refused, exit 2: 2^81 | 0 · 1 241 ms · 121 |

`lattice/synth/trigger_hole.ein` (both kinds, three sites) and
[`fifo_gap_priority.ein`](detector_proposal/fifo_gap_priority.ein) (§ 7) are
in the prototype's output too. Every flagged row today ends `; assumes a monotone
lattice for what a node is (P1e2.1b)`. Exit codes, walls and line counts are
[`today.tsv`](detector_proposal/outputs/today.tsv), and each stdout is in
[`today/`](detector_proposal/outputs/today/).

## 2. The explicit check: four forms

*Is this program monotone, and if not, where and why?* The forms are not
exclusive.

### 2.1 What it says, in any form

The body is the same whatever the form. It is the static check
(`monotone::classify`, what every bare run already calls) and the static
detector of the stage's (b), computed on root's fixpoint and on no other
node. *Drafted*, from the prototype.

**Its work**, measured in process from the load (load, root's fixpoint, the
universe, root's tally, `classify`), 2026-11-02:

- 0.08–0.42 ms on the flagged programs within the limit;
- 0.44 ms on `06` and 0.22 ms on `features/04`;
- 5–8 ms on `zebra.ein`, over four runs.

The static detector (cycles and order) adds 5–16 µs on a flagged program and
nothing on zebra. The closest command that ships, `ein saturate`, takes 5–11
ms of wall clock on the same files, process start included. A bare
`ein solve` takes 1 853 ms on zebra, 2 447 ms on `06` and 1 241 ms on
`features/04` (§ 1). So, process start included, the check would answer in a
170th to a 500th of what describing R costs on those three.

### 2.2 (a1) a mode of `ein solve`: `--monotone`

`ein solve FILE --monotone` answers a question about the program instead of
describing R. It is exclusive with `--minimal`, `--maximal`, `--single` and
`-n`. `--json-summary` writes `asked: "monotone"`, `region: null` and the
`monotonicity` block, and `--events` writes the static event. It exits 0
whatever the verdict. *Drafted:*

```
solve · examples/zebra.ein --monotone
──────────────────────────────────────────────────────────────
  asked             is the program monotone? — the static check, on root's fixpoint
  monotone          yes — no site: dead and discharged are upward-closed, so R has no hole and its two lines describe it
  universe          56 candidates
  route             none — a question about the program: root's fixpoint, then the static check
  work              1 saturation, root's · 0 facts beyond root
```

```
solve · examples/ein-bugs/naf-upward-closure.ein --monotone
──────────────────────────────────────────────────────────────
  asked             is the program monotone? — the static check, on root's fixpoint
  monotone          no — 1 site
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            bad: absent reads q at absent depth 1, concludes (false) — at stake: the state, and dead with it
    bit             not computed — the cube is 2^2 = 4 saturations: `ein oracle FILE` computes it, and so does a bare `ein solve FILE`
  universe          2 candidates
  route             none — a question about the program: root's fixpoint, then the static check
  work              1 saturation, root's · 0 facts beyond root
```

```
solve · examples/features/07_unstratifiable.ein --monotone
──────────────────────────────────────────────────────────────
  asked             is the program monotone? — the static check, on root's fixpoint
  monotone          no — 2 sites
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            derive-p: absent reads q at absent depth 1, concludes a positive fact — at stake: the state
    site            derive-q: absent reads p at absent depth 1, concludes a positive fact — at stake: the state
    cycle           even through an absent — derive-p: q →¬ p · derive-q: p →¬ q — several states with no stale fact can exist
    order           derive-p and derive-q each read under an absent what the other concludes, both at priority 200: which the boundary admits first is the order they are written in
    bit             not computed — the cube is 2^1 = 2 saturations: `ein oracle FILE` computes it, and so does a bare `ein solve FILE`
  universe          1 candidate
  route             none — a question about the program: root's fixpoint, then the static check
  work              1 saturation, root's · 0 facts beyond root
```

On `06` the `bit` row reads `not computed — the cube is 2^19 nodes, past
--region-limit 16`, and the rest is the trigger's site row. That is 7 lines on
a monotone program and 10–13 on a flagged one. Every program's block is in
[`detector_draft.txt`](detector_proposal/outputs/detector_draft.txt).

**Why it gives no second answer.** It computes no node beyond root, so it has
no R to disagree about. Its verdict and its sites are `classify`'s, the same
function the `lattice` row, the summary and `ein test` read. The stage's test
(c) 5 holds the rows byte-equal.

### 2.3 (a2) a subcommand: `ein check FILE`

The same body under a sixth subcommand, `check · FILE`. *Drafted:* the
header line is `check · examples/ein-bugs/naf-upward-closure.ein`, and every
row below it is § 2.2's. It needs its own `--json-summary` and `--events` to be
read by a script, so it adds a parser and two options. Like `ein saturate` it
answers without a lattice. Against it: the user's ruling of 2026-10-16 asked
for *"solve with subcommands + oracle command for full oracle"*, and a second
command whose rows are a copy of `solve`'s `lattice` block is one more surface
to hold equal.

### 2.4 (a3) a claim: `(solve … :sites (…))`

A program states the sites it expects, and `ein test` holds the claim with
the static check alone: load, root, `classify`, and no node. A refuted claim
exits 1. *Drafted*, on two real programs with the keyword added:

```lisp
;; examples/lattice/04_obligation_trigger_hole.ein: a fixture that exists to show one cause
(solve :sites (total-owed))
;; examples/zebra.ein: its existing (solve :hrules …) form gains one keyword
(solve :hrules (guess (co-located Color House) … (co-located Pet House)) :sites ())
```

```
$ ein test -v examples/lattice/04_obligation_trigger_hole.ein
ok           examples/lattice/04_obligation_trigger_hole.ein
  solve · :sites (total-owed) — holds (the static check: 1 site, 1 saturation)

$ ein test -v zebra-copy.ein            # zebra.ein with :sites (bad)
FAILED       zebra-copy.ein
  solve · :sites (bad) — expected 1 site, bad; the static check finds none
```

What it buys is the verdict stated in the program's own text, where a reader of
a cause fixture (`lattice/04`, `08`, `09`, `synth/*`) sees it. What it does
not buy is robustness the gate lacks. Every corpus program's verdict is
already pinned by the `lattice` row in five read-outs of `corpus_shapes.md5`,
and the stage's test (c) 5 holds the verdict equal across surfaces.

### 2.5 (a4) `ein oracle FILE`, as it is

*Today*, from the binary. Within the limit it prints the read-out over the cube
and closes with the cube's half:

```
the cube   1 dead node with an alive superset · 0 discharged nodes with an owing superset
```

That is `naf-upward-closure`. On a conservative program the line ends
`— the flagged sites break nothing here`, and on a monotone one
`— the static verdict holds`. It exits 1 when a *Monotone* verdict meets a
violation, which would be a defect in `classify`; none occurs on the corpus's
155 monotone cubes ([site census § 1](site_census.md#1-the-headline)). Past
the limit it refuses with exit 2: zebra (2^56), `06` (2^19) and `features/04`
(2^81) all do. So it cannot say whether zebra is monotone, which the static
check says in 5–8 ms. It costs 2ⁿ saturations on every program, monotone or
not.

### 2.6 The four side by side

| | answers | work, on zebra · `naf-upward-closure` · `06` | exits | the cube's half | moves (§ 6) |
|---|---|---|---|---|---|
| **(a1)** `solve --monotone` | the program's verdict, its sites, cycles, order, what is assumed | 5–8 ms · 0.09 ms · 0.44 ms, one saturation each | 0 | no (§ 3) | 1 option |
| **(a2)** `ein check` | the same | the same | 0 | no | 1 parser, 2 options |
| **(a3)** `:sites` claim | whether the program's stated sites are `classify`'s | the same, per claim | 0, or 1 when refuted | no | the grammar, `ein test`'s report |
| **(a4)** `ein oracle` | the cube's half; exit 1 on a *Monotone* verdict a cube contradicts | refused (2^56) · 4 saturations · refused (2^19) | 0, 1, or 2 past the limit | yes, within the limit | nothing |

## 3. Does the check report the cube's half?

*Exercised* or *conservative* is a property of one program's cube, not of the
program. Two options:

- **(i) the static check alone.** The check prints `bit  not computed — the
  cube is 2^n = … saturations: ein oracle FILE computes it, and so does a bare
  ein solve FILE`, as drafted above. Its cost is § 2.1's on every program.
- **(ii) the cube within the limit.** The check also enumerates 2ⁿ nodes on a
  flagged program within `--region-limit`. That is 2–32 saturations on the
  programs of § 1 and up to 65 536 at n = 16, while a monotone program pays
  nothing because the theorem decides. Past the limit it says *not known*.

Under (i) the half has two homes, each already computing the cube for its own
question. `ein oracle` computes it by name. A bare `ein solve` computes it
anyway on a flagged program within the limit (route 2), and § 4's `status`
row prints it there at no extra work. Under (ii) the explicit check would
become a third route to the same 2ⁿ, for a question that is not the program's.

## 4. The warnings

### 4.1 The three things, as rows

Each warning says the three things the stage names, each in its own row. The
`lattice` row keeps its first words (`not monotone: n sites (the static check)
— h holes`), so `region_dump.rs`'s prefix test still matches. What follows it
is new:

| says | row | from | on the programs of § 1 |
|---|---|---|---|
| **what is assumed** | `assumes` | constant: every node is a fork of root, and what a node is here is open (Q-M1e2.2, F21) | every flagged program |
| **which property is at stake** | `site`, one per site, ≤ 4 on stdout | `classify`'s site, what its rule concludes, and the property: an `absent` site puts **the state** at stake (and `dead` with it when it concludes `(false)`); a trigger puts **the status** (`owes`); an activator, the status or the state through the plan a node compiles. Drafted on [`monotonicity.md` § 7.6](../../../docs/kernel/inference/monotonicity.md#76-the-program-the-third-stronger-property)'s table; S1e2.1b.7's taxonomy supplies the final words | every flagged program |
| | `cycle` | the dependency graph over D, `node_census --strata`'s, with its parity | `features/07` (even), `d3-unsat-core` (odd) |
| | `order` | new: an `absent` rule that reads under an `absent` a relation in D another `absent` rule concludes. At one priority the written order decides; at an earlier priority the reader goes first in every order (§ 7) | `features/07`, `fifo_gap`, `stratified_stale` |
| **whether it bit** | `status` | the cube's violations, attributed by S1e2.1b.3's rule: **non-monotone here (exercised, by *site*)**, **monotone here (conservative)**, or **not known** past the limit | 6 exercised (7 with `synth/trigger_hole`), 6 conservative, 2 not known |
| | `state` | **stale facts** in every node the run computed: a fact whose `absent` guard the node's own fixpoint falsifies, up to 4 named on stdout | `stale-naf-fork`, `features/07`, `fifo_gap`, `stratified_stale`, `features/04` |
| | `paths` | the spot check, if it ships (§ 5): a parent path that reaches another state | `lattice/08`, `d3-unsat-core`, `fifo_gap`, `activator_under_absent`, `via_hole` |

Then **today's minimal-hole rows, unchanged** (§ 4.4). A monotone program's row
does not change, so no monotone program's output moves.

**Why the state's evidence is apart from the status's.** *Exercised* and
*conservative* are the cube's words, and the cube reads the two status bits.
On five programs the cube says *conservative* while a node's state holds a
stale fact or a parent path reaches another state: `stale-naf-fork`,
`features/07`, `d3-unsat-core`, `fifo_gap` and `stratified_stale`. A single
word would call those programs harmless.

**Why every node the run computed, and not only the listed states.** The stage
says *stale facts in the listed states*. On W8's own witness,
`stale-naf-fork`, the stale fact is in `{(q one)}`, which is dead and so never
listed: the listed state, ∅, holds none. Reading every node the route already
holds costs one matcher call per fact with an `absent` premise (§ 5's table).

### 4.2 The `lattice` block, on the programs

*Drafted*, the prototype's rows, with the spot check's row last. Today's row on
each is `not monotone: … ; assumes a monotone lattice for what a node is
(P1e2.1b)`, then up to four hole rows.

**`ein-bugs/naf-upward-closure.ein`**: exercised, the status.

```
  lattice           not monotone: 1 site (the static check) — 1 hole
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            bad: absent reads q at absent depth 1, concludes (false) — at stake: the state, and dead with it
    status          non-monotone here (exercised, by bad) — the cube: 1 hole · 1 dead node with an alive superset · 0 discharged nodes with an owing superset
    state           no stale fact in the cube's 4 nodes
    paths           every parent path checked reaches the same state — 1 parent of 1 printed state
    minimal hole    {(p A)} — dead: (p A)
```

**`branching/13_lookahead_naf_world.ein`**: conservative.

```
  lattice           not monotone: 1 site (the static check) — no hole in this cube
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            self-block: absent reads cand at absent depth 1, concludes (false) — at stake: the state, and dead with it
    status          monotone here (conservative) — the cube: no hole, no dead node with an alive superset, no discharged node with an owing superset
    state           no stale fact in the cube's 4 nodes
    paths           no state the read-out prints has an alive parent to check
```

**`ein-bugs/wrong-answers/stale-naf-fork.ein`**: W8. The status is
conservative, and the state bit in a dead node.

```
  lattice           not monotone: 1 site (the static check) — no hole in this cube
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            derive-p: absent reads q at absent depth 1, concludes a positive fact — at stake: the state
    status          monotone here (conservative) — the cube: no hole, no dead node with an alive superset, no discharged node with an owing superset
    state           stale facts — 1 of the cube's 2 nodes holds one: a fact whose absent guard the node's own fixpoint falsifies
    stale           {(q one)}, dead: (p one) ← derive-p (absent (q one))
    paths           no state the read-out prints has an alive parent to check
```

**`features/07_unstratifiable.ein`**: an even cycle, and the written order.

```
  lattice           not monotone: 2 sites (the static check) — no hole in this cube
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            derive-p: absent reads q at absent depth 1, concludes a positive fact — at stake: the state
    site            derive-q: absent reads p at absent depth 1, concludes a positive fact — at stake: the state
    cycle           even through an absent — derive-p: q →¬ p · derive-q: p →¬ q — several states with no stale fact can exist
    order           derive-p and derive-q each read under an absent what the other concludes, both at priority 200: which the boundary admits first is the order they are written in
    status          monotone here (conservative) — the cube: no hole, no dead node with an alive superset, no discharged node with an owing superset
    state           stale facts — 1 of the cube's 2 nodes holds one: a fact whose absent guard the node's own fixpoint falsifies
    stale           {(q one)}, in R: (p one) ← derive-p (absent (q one))
    paths           no state the read-out prints has an alive parent to check
```

**`corpus/fuzz_findings/d3-unsat-core.ein`**: an odd cycle. Only the spot
check sees it bite.

```
  lattice           not monotone: 1 site (the static check) — no hole in this cube
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            fire-1: absent reads (not r1) at absent depth 1, concludes a positive fact — at stake: the state
    cycle           odd through an absent — fire-1: (not r1) →¬ r1 — a state with no stale fact need not exist
    status          monotone here (conservative) — the cube: no hole, no dead node with an alive superset, no discharged node with an owing superset
    state           no stale fact in the cube's 32 nodes
    paths           path-dependent — 3 of 34 parent paths checked reach another state: {(r1 o2 o3), (r1 o3 o2)} by way of its parent without (r1 o3 o2)
```

**`fifo_gap.ein`**, *drafted* (S1e2.1b.7's fixture does not exist yet).
Conservative, stale, and order-sensitive.

```
  lattice           not monotone: 1 site (the static check) — no hole in this cube
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            r2: absent reads q at absent depth 1, concludes a positive fact — at stake: the state
    order           r2 reads q under an absent and r1 concludes it, both at priority 200: which the boundary admits first is the order they are written in
    status          monotone here (conservative) — the cube: no hole, no dead node with an alive superset, no discharged node with an owing superset
    state           stale facts — 1 of the cube's 4 nodes holds one: a fact whose absent guard the node's own fixpoint falsifies
    stale           {(a one), (b one)}, in R: (p one) ← r2 (absent (q one))
    paths           path-dependent — 1 of 2 parent paths checked reaches another state: {(a one), (b one)} by way of its parent without (b one)
```

**`lattice/08_absent_witness_hole.ein`**: exercised in the status. Only the
spot check sees the state's path dependence, which
[`node_census.md` § 6.1](node_census.md#61-the--y-abort-on-d3-unsat-core)
found with nothing derived at root.

```
  lattice           not monotone: 1 site (the static check) — 1 hole
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            soup-unless-allergic: absent reads allergic at absent depth 1, concludes a positive fact — at stake: the state
    status          non-monotone here (exercised, by soup-unless-allergic) — the cube: 1 hole · 0 dead nodes with an alive superset · 1 discharged node with an owing superset
    state           no stale fact in the cube's 8 nodes
    paths           path-dependent — 1 of 7 parent paths checked reaches another state: {(allergic Ann), (meal Ann Bread), (seats Ann C1)} by way of its parent without (allergic Ann)
    minimal hole    {(allergic Ann), (seats Ann C1)} — owes 1
```

**`lattice/04`** reads `site total-owed [activator meal is-a]:
obligation-trigger reads is-a at absent depth 0, concludes (open …) — at stake:
the status: owes` and `status non-monotone here (exercised, by total-owed
[activator meal is-a])`. **`lattice/09`** reads `site total-owed: its activator
relation is one a commitment can change; root compiled no plan of it — at
stake: the status: owes, through a plan a node compiles`. Neither has a stale
fact or a path that differs: a hole that needs no path dependence
([`monotonicity.md` § 7.5](../../../docs/kernel/inference/monotonicity.md#75-the-status-dead-follows-the-state-owes-does-not)).

**`ACTIVATOR_UNDER_ABSENT`**, as `activator_under_absent.ein`. The static check
reports **2 sites**: `act`, an `absent` site over `q` that concludes a
positive fact (the activator `(watch r)`), and `watch`'s activator. Today's
hole row is `minimal hole {(p A), (r A)} — dead: (r A)`. The core names a
commitment, the oracle's `absent_premises` is empty, and provenance alone
calls the death untainted
([taint census § 3](taint_census.md#3-the-readings-of-tainted-side-by-side)).
The drafted detector:

```
  lattice           not monotone: 2 sites (the static check) — 1 hole
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            act: absent reads q at absent depth 1, concludes a positive fact — at stake: the state
    site            watch: its activator relation is one a commitment can change; root compiled no plan of it — at stake: the state, through a plan a node compiles
    status          non-monotone here (exercised, by watch) — the cube: 1 hole · 1 dead node with an alive superset · 0 discharged nodes with an owing superset
    state           no stale fact in the cube's 8 nodes
    paths           path-dependent — 1 of 6 parent paths checked reaches another state: {(p A), (q A)} by way of its parent without (q A)
    minimal hole    {(p A), (r A)} — dead: (r A)
```

So the warning names `act`, the `absent` the death rests on, which the hole
row cannot. It names it as a **site**, not as the hole's cause: the
attribution names `watch`, whose plan no premise records. The stale check finds
nothing, because no node's own fixpoint holds `(watch r)` beside `(q A)`. Only
the spot check sees `(watch r)` kept along the path `{(p A)} + (q A)`, where
it is stale (`node_census -- $D/activator_under_absent.ein`,
[`node_census_drafted.txt`](detector_proposal/outputs/node_census_drafted.txt)).
Whether a route may learn from that death is F21's D6, and whether *forbid*
covers it is S1e2.2.5's F2. The warning decides neither.

**Past the limit.** On `lattice/06_friends_tables.ein`:

```
  lattice           not monotone: 1 site (the static check) — holes not known: the cube was not enumerated
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            group-seated-owed: obligation-trigger reads sits at absent depth 0, concludes (open …) — at stake: the status: owes
    status          not known — the cube is 2^19 nodes, past --region-limit 16
    state           no stale fact in the 21 members met
    paths           not checked — the read-out prints no state of two commitments or more; its 21 members met would cost 104 saturations
```

On `features/04_open.ein` the state bit, where today's read-out says only
*holes not known*. The `status` row reads as on `06`, and the `state` rows are
these, three more `stale` rows elided:

```
    state           stale facts — 21 042 of the 64 761 members met hold one: a fact whose absent guard the node's own fixpoint falsifies
    stale           {(house-color House-1 Blue)}, in R: (open-slot House-1 Blue) ← open-candidate (absent (house-color House-1 Blue)) (absent (not (house-color House-1 Blue)))
    …               21 038 more: --dump-answer DIR writes them
```

**Line counts.** The warning sits above the listings, whose 100 lines it does
not take ([`hole_census.md` § 9](hole_census.md#9-each-option-drafted-on-real-programs)),
and it is bounded on its own: 4 `site` rows and 4 `stale` rows at most, each
followed by one `…` row. A bare `ein solve`'s stdout grows by 4–9 rows on a
flagged program, and by one more with the spot check:

| | lines today | + the warning | + `paths` |
|---|---:|---:|---:|
| `naf-upward-closure`, `lattice/04`, `09`, `branching/13` | 35, 35, 35, 39 | +4 | +1 |
| `lattice/08`, `via_hole` | 44, 43 | +4 | +1 |
| `stale-naf-fork`, `d3-unsat-core`, `activator_under_absent` | 27, 68, 35 | +5 | +1 |
| `fifo_gap`; `features/07`, `stratified_stale` | 36; 24, 32 | +6; +8 | +1 |
| `06_friends_tables`; `features/04` | 114; 121 | +4; +9 | +1 |
| `zebra.ein` and every monotone program | 59 | 0 | 0 |

### 4.3 Every other surface

Each is *drafted* on two programs, `naf-upward-closure` (exercised) and
`stale-naf-fork` (conservative, stale), unless it says otherwise.

**`--json-summary`'s `monotonicity` block.** Today it holds `monotone`,
`hypothesised`, `dynamic`, `plans`, `reads`, `absent_reads` and `sites`. The
draft adds fields, so `ein-summary/2` keeps its name:

```json
"monotonicity": {
  "monotone": false, "hypothesised": ["p", "q"], "dynamic": ["false", "p", "q"], "…": "today's fields",
  "sites": [{"rule": "bad", "activator": [], "kind": "absent", "relation": "q", "depth": 1,
             "concludes": "(false)", "at_stake": "state, dead"}],
  "assumes": {"node": "fork from root", "open": "Q-M1e2.2, F21"},
  "cycles": [],
  "order": [],
  "status": {"read": "cube", "bit": "exercised", "by": ["bad"], "holes": 1,
             "dead_not_upward_closed": 1, "discharged_not_upward_closed": 0},
  "state": {"read": "every node computed", "nodes": 4, "stale_nodes": 0, "stale": []},
  "paths": {"bound": 3, "states": 1, "parents": 1, "differ": []}
}
```

On `stale-naf-fork` the block has `"status": {"bit": "conservative", …}` and
`"state": {"nodes": 2, "stale_nodes": 1, "stale": [{"node": "{(q one)}",
"class": "dead", "facts": ["(p one) ← derive-p (absent (q one))"]}]}`. Past the
limit it has `"status": {"read": null, "bit": "not known"}`. Every stale fact is
written here whatever the cap.

**`ein test`, and `--json-report`.** A held claim prints only its `ok` line by
default, as today. Under `-v`, and under every claim that FAILED or was NOT
CHECKED, one line follows a claim on a flagged program:

```
ok           examples/ein-bugs/naf-upward-closure.ein
  solve · :expect (region …) — holds (route 2, k_s = 3, k_c = 3)
    not monotone: 1 site · every node a fork of root; what a node is here is open (F21) · status non-monotone (exercised, by bad) · no stale fact
ok           examples/features/12_expect_false.ein
  solve · :expect (false) — holds (route 1, k_s = 0, k_c = 0)
    not monotone: 2 sites · every node a fork of root, so a dead root kills every node (L0); what a node is here is open (F21) · status not known (2^20)
```

`features/12` is the case where *what is assumed* decides the answer. L0 is
the fork's own construction, and its R = ∅ holds from scratch only by
measurement ([`node_census.md` § 6.2](node_census.md#62-l0-on-features12)).
The report's row gains the same fields beside its `monotone`:
`"region": {"route": "route 2", "monotone": false, "k_s": 3, "k_c": 3,
"status": "exercised", "stale_nodes": 0, "paths_differ": 0}`. The claim
check's past-the-limit line, *R was not described: the program is not monotone
and n = … is past the oracle's limit*, already uses the word of the third
property, and stays.

**`--dump-answer`.** `answer.txt` is the read-out uncut, so it holds every
`site` and every `stale` row: 21 042 `stale` rows on `features/04`, where
stdout prints 4. `answer.json` is the summary above.

**The event stream.** The `monotonicity` event is emitted at `prepare`, before
any node, so it carries the static half. The new fields are optional, so
`ein-events/1` stays ([`events.md` § Versioning](../../../docs/kernel/inference/events.md#versioning)):

```json
{"e": "monotonicity", "n": 7, "verdict": "non-monotone", "sites": 1, "site": ["bad: absent reads q at absent depth 1"], "at_stake": ["state, dead"], "cycles": [], "order": [], "assumes": "fork"}
{"e": "monotonicity", "n": 12, "verdict": "non-monotone", "sites": 1, "site": ["derive-p: absent reads q at absent depth 1"], "at_stake": ["state"], "cycles": [], "order": [], "assumes": "fork"}
```

The dynamic half is in the summary and the read-out. No new event kind is
proposed. `events.md`'s entry loses *a stub* and says this.

**`ein oracle` and its `--dump`.** Its stdout is the read-out, so it prints
§ 4.2's block, with `status` always computed. The closing line keeps its
counts and rewords what it concludes, because *the flagged sites break nothing
here* is said today of five programs whose state bit (§ 7):

```
the cube   1 dead node with an alive superset · 0 discharged nodes with an owing superset — the status is not monotone here, by bad
the cube   0 dead nodes with an alive superset · 0 discharged nodes with an owing superset — the status is monotone here; 1 node holds a stale fact (above)
```

A monotone program keeps `— the static verdict holds`, and exit 1 on a
contradicted *Monotone* verdict stays. `region.md`'s header, today *"Everything
below that assumes a monotone lattice says so; what the region means here is
P1e2.1b's"*, becomes:

```
**Not monotone: 1 site.** A commitment can take a conclusion away or add a debt, so the region may have
holes and its two lines may not describe it. Every node below is a fork of root: root's fixpoint, its
commitment added, saturated. What a node is on such a program is open (Q-M1e2.2, F21), and a line marked
*(nodes by fork)* is computed over the nodes so built.

- bad: absent reads q at absent depth 1, concludes (false) — at stake: the state, and dead with it
- the status: non-monotone here (exercised, by bad) — 1 hole · 1 dead node with an alive superset · 0 discharged nodes with an owing superset
- the state: no stale fact in the cube's 4 nodes
```

Each *(assumes a monotone lattice)* becomes *(nodes by fork)*. `region.json`'s
`assumes_monotone` keeps its name, which only `ein_render::dump::region`
writes and no test or script reads, and
[`region_dump.md`](../../../docs/kernel/inference/region_dump.md) gives it its
meaning: *the static check flags the program, and the lines and components are
computed over nodes built by fork*.

**`ein render region`.** The graph label `NOT monotone (1 sites)` becomes
`NOT monotone (1 site): status exercised · no stale fact`, and its plural is
fixed. The legend row *"not monotone — sites: bad · absent · q; the lines
assume a monotone lattice (P1e2.1b)"* becomes:

```
not monotone — sites: bad · absent · q (the state, and dead with it); nodes by fork from root, what a node is here is open (F21)
```

**The review bundle.** A case page's `the cube, 2^2 nodes · **not monotone**`
becomes `· **not monotone**: status exercised (by bad) · no stale fact`.
`regions.py`'s gallery column `monotone` changes from `yes`/`**no**` to `yes`,
`**no**: exercised`, `**no**: conservative` or `**no**: not known`, with a
`stale` column beside it.

**The register.** Five strings move from P1e2.1b to F21, and no field changes:

| row | field | today | drafted |
|---|---|---|---|
| `W1/alive-empty-interlayer`, `W1/d3-unsat-core`, `W11/d3-unsat-core` | `right.qualifier` | *assumes a monotone lattice: the static check flags this program, and what R is here is P1e2.1b's* | *every node a fork of root: the static check flags this program, and what a node is here is open (F21)* |
| `W8/features-07` | `open_because` | *… what a node is when a guard's witness arrives later is P1e2.1b's, and R is over the fork route under \*assumes a monotone lattice\*. R's maximal line agrees …* | *… is F21's D2, and R is over nodes built by fork from root. R's maximal line agrees …* |
| `W8/stale-naf-fork` | `open_because` | *… is P1e2.1b's, and R is over the fork route under \*assumes a monotone lattice\*, which keeps the stale (p one)* | *… is F21's D2, and R is over nodes built by fork from root, which keeps the stale (p one)* |

### 4.4 Where the draft touches F21's D4

D4 is *what is reported when R has holes*
([F21 § D4](../../followups/f21_non_monotone_programs_decided.md#d4--what-is-reported-when-r-has-holes-q-m1e27-d),
[`hole_census.md` § 9–10](hole_census.md#10-the-consequences-per-option)).
The draft keeps **today's hole listing exactly**: the `minimal hole` rows, up
to four, `— dead: core` or `— owes n`, and `region.holes` in the summary. It
adds no repair edge, no piece, no qualifier on the lines and no refusal, and it
names no hole's cause in a hole row.

It touches D4 in two places:

- **(d3), the stub's words.** The `lattice` row loses *"; assumes a monotone
  lattice for what a node is (P1e2.1b)"* and gains the `assumes` row.
  [`hole_census.md` § 10](hole_census.md#10-the-consequences-per-option) says
  S1e2.1b.5 rewords the stub in the meantime, whatever D4 decides. It does not
  say *the lines bound R*, which is D4's option (c).
- **The `status` row names the sites the cube's violations are attributed
  to.** That is a property of the program and of its cube, `ein oracle`'s
  closing line with attribution. It is not a hole row's content. If D4 takes
  (a), each hole row will name its own cause and site, and this row stays a
  summary.

## 5. The parent-path spot check, priced

**What it does.** For each state of k ≥ 2 commitments that the read-out
prints, it takes up to 3 of its parents C \ {h}. It saturates each parent,
writes h, saturates again, and compares the result with the state the route
built. A dead parent is skipped, as `sanity::check_commutativity` skips one.
It changes no answer and refutes nothing. The `-y` lever stays unread, and its
fate is S1e2.1c.5's.

**What it finds that nothing else does.** On `lattice/08`, `d3-unsat-core`,
`via_hole` and `ACTIVATOR_UNDER_ABSENT`, the stale check finds nothing (no
node's own fixpoint falsifies a guard) while a parent path reaches another
state. That is the path dependence
[`node_census.md` § 6.1](node_census.md#61-the--y-abort-on-d3-unsat-core)
measured, now on the states a user is shown. Without it, the warning on those
four says *no stale fact* and nothing more about the state.

**What it costs**, from the prototype, 2026-11-02, bound 3:

| program | states printed, k ≥ 2 | parents checked | dead skipped | saturations | wall | differ | the run's saturations |
|---|---:|---:|---:|---:|---:|---:|---:|
| `naf-upward-closure` | 1 | 1 | 1 | 3 | < 0.1 ms | 0 | 4 |
| `lattice/08` | 3 | 7 | 0 | 14 | 0.1 ms | **1** | 8 |
| `lattice/04`, `lattice/09` | 1 | 2 | 0 | 4 | < 0.1 ms | 0 | 4 |
| `d3-unsat-core` | 14 | 34 | 0 | 68 | 0.5 ms | **3** | 32 |
| `fifo_gap` (*drafted*) | 1 | 2 | 0 | 4 | < 0.1 ms | **1** | 4 |
| `activator_under_absent`, `via_hole` | 3 | 6 | 1 | 13 | 0.1 ms | **1** | 8 |
| `synth/trigger_hole` | 8 | 21 | 1 | 43 | 0.2 ms | 0 | 32 |
| `branching/13`, `stale-naf-fork`, `features/07`, `stratified_stale` | 0 | 0 | 0 | 0 | — | 0 | 2–4 |
| `06_friends_tables`, if it read the 21 members met | 20 | 52 | 0 | 104 | 1.5 ms | 0 | 65 299 |
| `features/04`, if it read the 64 761 members met | 64 679 | 190 797 | 0 | 381 594 | 3.8 s | 0 | 64 761 (1 241 ms) |

Past the limit the read-out prints only the certified line members, ∅ on both
programs, so under *the states the read-out prints* the spot check does
nothing there. The last two rows price the other scope, every member met. On
`features/04` that costs three times the run and finds nothing.

**The bound** ([`spot_check_bounds.txt`](detector_proposal/outputs/spot_check_bounds.txt)).
One parent per state finds 2 of the 5 programs (`lattice/08`, `via_hole`), and
three find all 5. Every parent finds the same 5, and 4 differences on
`d3-unsat-core` against 3, at 70 saturations against 68. So the cost is at
most 2 × 3 saturations per printed state, and the 100-line cap bounds the
printed states at any n.

**The stale check's price**, the `state` row's, for comparison. It is one
matcher call per fact with an `absent` premise, per node:

- 1–19 µs per cube within the limit;
- 10 µs on `06`'s 21 members met; its 65 299 visited nodes are not measured,
  and 31 ms is derived at the same rate per node;
- **357 ms on `features/04`'s 64 761 members met**, against that run's
  1 241 ms. Its corpus runs cost 329 ms together
  ([`corpus.toml`](../../../corpus/corpus.toml), `utils/corpus_cost.py`,
  2026-10-18).

The prototype re-saturates each node to read it, and the build reads the
fixpoint the route already holds. Only the matcher's share is counted above.
T1e2.1b.5.6 measures the build, and `features/04` is the entry to watch
against the 1 s line.

## 6. What each choice moves

**The warnings, whatever the form:**

| golden or file | what moves | how much |
|---|---|---|
| [`corpus_shapes.md5`](../../../ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5) | `read-out[describe]`, `[maximal]`, `[minimal]`, `[single]`, `[n2]` and `region-trace` of every flagged corpus entry | **150 lines**: 6 × 25 entries, counted from the golden 2026-11-02. `d3-unsat-core` has no corpus entry |
| `corpus_exits.txt` | nothing | 0: no exit code moves |
| [`read_out_examples.md`](../p1e2.1a_the_solution_region/read_out_examples.md) | X8 (`naf-upward-closure`), X9 (`06_friends_tables`), re-written by `utils/read_out_examples.py`, which `read_out_examples.rs` diffs | 2 sections |
| `region_dump.rs::the_non_monotone_stub_says_so_where_there_is_a_site` | still passes, because the prefix is kept. The stage replaces it with a test of the warning | 1 test |
| `ein-summary/2` | the `monotonicity` block gains fields | additive; no golden holds a flagged summary whole |
| the event | the `monotonicity` event gains optional fields | no version bump; not in `corpus_shapes.md5` |
| `region.md`, the `ein render region` label, the review bundle | reworded | no golden |
| [`wrong_answers.toml`](../../../corpus/wrong_answers.toml) | 3 qualifiers, 2 `open_because` (§ 4.3) | 5 strings, data only |
| the docs | [`events.md`](../../../docs/kernel/inference/events.md)'s `monotonicity` entry; [`region_dump.md`](../../../docs/kernel/inference/region_dump.md)'s stub paragraph and its `assumes_monotone` row; [`read_out.md` § 6.3](../p1e2.1a_the_solution_region/read_out.md#63-holes-and-the-warning-never-silent); [`01_grammar.md` § Solve](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#solve)'s *"under assumes a monotone lattice"*; [`monotonicity.md`](../../../docs/kernel/inference/monotonicity.md) § 5 and § 8; [`configuration.md`](../../../docs/kernel/configuration.md)'s *"a node of a non-monotone lattice"*; the fixture headers and code comments naming P1e2.1b as the node's owner | the stage's list |

**The spot check, if it ships**, adds one row inside the same 150 lines and no
golden of its own.

**The form:**

| | moves |
|---|---|
| **(a1)** `--monotone` | `help_shape.txt` +1 `OPTION` line under `COMMAND ein solve`; `help_surface.rs`'s `62` (twelve parsers) → 63; [`configuration.md` § 3](../../../docs/kernel/configuration.md) 71 → 72 options, `ein solve` 33 → 34, which `config_reference.rs` reads off the golden; `read_out.md`'s vocabulary gains the `asked` phrase and the `monotone` row; a new test that the block is the `lattice` row's. No corpus golden |
| **(a2)** `ein check` | `help_shape.txt` +5 lines (`COMMAND`, `ABOUT`, 2 `OPTION`s, `POSITIONAL`); `help_surface.rs`'s parser and option totals; `configuration.md` fifteen parsers → sixteen, 71 → 73; AGENTS.md's *five subcommands*; `ein --help` |
| **(a3)** `:sites` | [`01_grammar.md` § Solve](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#solve) (a fifth keyword) and the loader's allow-list; `ein-test-report/3`'s `expect.shape` gains `sites`, a version question; `test_cli.rs`. A fixture that adopts it moves its own `load`, `ir[*]` and `dot[ir*]` lines of `corpus_shapes.md5` |
| **(a4)** as is | nothing |

## 7. What the drafting found

1. **The state bites where the status does not, and today's surfaces call it
   harmless.** `stale-naf-fork`, `features/07`, `d3-unsat-core`, `fifo_gap` and
   `stratified_stale` are *conservative* by the cube, and each holds a stale
   fact or has a parent path that reaches another state. On each, `ein
   oracle` closes with *"— the flagged sites break nothing here"*, which is
   true of the status only. § 4.3 rewords the line.
2. **Past the limit, the state bit on `features/04`**: 21 042 of its 64 761
   members met hold a stale `(open-slot …)` that root derived under
   `(absent (house-color …))`, which their commitment falsifies. Today's
   read-out says only *holes not known*.
3. **The stage's scope for stale facts, the listed states, misses W8's own
   witness.** `stale-naf-fork`'s stale fact is in a dead node. § 4.1 proposes
   every node the run computed.
4. **The order check as the stage words it** (two `absent` rules at one
   priority) **misses a reader at an earlier priority.**
   [`fifo_gap_priority.ein`](detector_proposal/fifo_gap_priority.ein) is
   `fifo_gap` with r2 at priority 100. There r2 is admitted first in every
   order, and `{(a one), (b one)}` holds a stale `(p one)`, fork and scratch
   alike. `node_census --invariance` finds no equal-priority pair to permute,
   so rule order moves nothing (both outputs beside this page). The drafted
   order check reports both cases, the second as `r2 (priority 100) reads q
   under an absent and r1 (200) concludes it: r2 is admitted first, whatever
   the order`.
5. **The FIFO gap reproduces in the engine as written.** On `fifo_gap.ein`,
   `{(a one), (b one)}` holds a stale `(p one)` by fork and from scratch. By
   way of `{(a one)}` it holds `(q one)` without `(p one)`, and writing r1
   before r2 moves that state
   ([`node_census_drafted.txt`](detector_proposal/outputs/node_census_drafted.txt),
   [`node_census_invariance_drafted.txt`](detector_proposal/outputs/node_census_invariance_drafted.txt)).
   That is the claim
   [`monotonicity.md` § 7.8](../../../docs/kernel/inference/monotonicity.md#78-the-standing-of-each-claim)
   lists as *unverified*. Banking it, and re-labelling the row, is S1e2.1b.7's.
   This page only drafts on it.
6. **`ACTIVATOR_UNDER_ABSENT`** (§ 4.2): the static check reports `act`, the
   `absent` the death rests on, as a site, though no premise of the
   refutation records it. Of the drafted detector's rows, only `paths` shows
   the state's path dependence.
7. **A *Monotone* verdict is relative to root, and so is a detector gated
   on it.**
   Three syntax fixtures hold an odd cycle over relations outside D, with
   n = 0 ([`node_census.md` § 5](node_census.md#5-the-stratification-check)),
   and the detector stays silent on them. This is consistent with
   [`monotonicity.md` § 7.6](../../../docs/kernel/inference/monotonicity.md#76-the-program-the-third-stronger-property):
   a race over a static relation is settled at root, and every fork inherits
   its result.

## 8. Recommendation, and the questions

**Mine, not a ruling:**

- **The form: (a1), `ein solve FILE --monotone`, with (a4) unchanged beside
  it.** It keeps `solve` the one tool, costs one option, and answers in one
  saturation. Every surface's `lattice` block repeats its rows. (a2) adds a
  parser to hold equal for no new answer. (a3) is worth adding when a fixture
  needs to state its own verdict, and nothing needs it now.
- **The cube's half: (i).** The check stays static and names `ein oracle` and
  the price. A bare `ein solve` already prints the half within the limit at
  no extra work.
- **The spot check: yes, bounded.** It covers up to 3 parents of each state
  the read-out prints, which comes to ≤ 6 saturations a state and ≤ 68 on
  every program here. It is the only check that sees the path dependence of
  `lattice/08`, `d3-unsat-core`, `via_hole` and `ACTIVATOR_UNDER_ABSENT`.
  Past the limit it reads nothing, because the read-out prints only ∅ there.
- **The wording: as drafted in § 4.** The `assumes`, `site`, `cycle`, `order`,
  `status`, `state` and `paths` rows come first, then today's hole rows. The
  status keeps *exercised*, *conservative* and *not known*, the state's
  evidence goes apart, and the stale check reads every node the run computed.

**Decided here unless the user objects**, details that follow from the four
answers: `ein test` prints the warning under `-v` and beside a failing claim,
and its default output is unchanged. The event carries the static half only.
`region.json` keeps `assumes_monotone`, now documented. The summary's new
fields are additive under `ein-summary/2`. The render label's *1 sites*
becomes *1 site*.

**The questions** (the only ones the phase puts to the user):

1. **The check's CLI form**: (a1) `ein solve FILE --monotone`, (a2)
   `ein check FILE`, (a3) a `(solve :sites (…))` claim, (a4) `ein oracle`
   alone, or a combination? *Recommended: (a1), with (a4) as it is.*
2. **The cube's half**: does the explicit check enumerate the cube within the
   limit to say *exercised* or *conservative* (ii), or say *not computed* and
   name `ein oracle` (i)? *Recommended: (i).*
3. **The parent-path spot check**: ship it, at ≤ 3 parents of each state the
   read-out prints (≤ 6 saturations a state, ≤ 68 per program here), or not?
   *Recommended: yes.*
4. **The warning's wording**: § 4.2's `lattice` block and § 4.3's surfaces,
   with the status and the state reported apart, and the stale check over
   every node the run computed. Approve as drafted, or amend? *Recommended:
   approve as drafted.*
