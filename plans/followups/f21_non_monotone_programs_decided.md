# F21 — Non-monotone programs: node functions and path dependence, decided

**Filed 2026-11-02**, from M1e2's
P1e2.1b, on
the user's direction of that day, verbatim:

> Currently my goal is not to build complete theory os solving the
> non-monotonic problems; But I want to have a robust ways to detect
> non-monotonocity, understand consequences of solving it either way, be able
> to at least show warnings about the program that is not monotonic, etc.
> Let's investigate ein non-monotonicity as much as possible now, collect all
> information, write it down in docs, provide robust way to explicitely check
> if program is monotone or not, build-in cheap and fast detector into full
> `solve` command that is expected to find complete R, and so on - so prepare
> everything we are able to do now, without making final decisions; And then
> leave further investigation of non-monotonic lattices as followup.

and, once the analysis below was given back the same day: *"Ok, just add all
of this into followup and docs - will return to it later"*.

So P1e2.1b detects, documents and warns, and decides nothing. This page is
the record the user will return to: [§ The analysis](#the-analysis-2026-11-02)
is everything established on 2026-11-02, and [§ The decisions](#the-decisions)
is every decision P1e2.1b was to take, with its options, its evidence, and
what the engine does until it is taken. The docs half of the analysis, each
claim labelled by its standing, is
[`08-monotonicity.md` § 2](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#2-the-state-the-status-and-the-program).
Each option's measured consequences per program are P1e2.1b's
S1e2.1b.7's.

**Trigger** (any one):

- a route that builds a node by extending a parent's fixpoint is to run on a
  program the static check flags (D2, D3);
- R is wanted past the oracle's limit on a flagged program, such as
  `lattice/06_friends_tables`, the conditional obligation (D7);
- M1e2's S1e2.2.5
  builds *forbid*'s refusal: (E) is taken with it or explicitly declined, and
  D6's window closes. **Fired 2026-11-04**: the refusal is built, with the
  user's F2, *via, per activator*; (E) was declined for that build (D1 stays
  open, below), and D6's window closed (below);
- the wrong-answer revisit (S1e2.1.7)
  cannot settle W8 under today's documented node
  (S1e2.2.6 § The deferral);
- M1f's S1f.5.15,
  which plans the stratification diagnostic, starts (D2's (D) and stratum
  order);
- the detector (P1e2.1b's S1e2.1b.5) flags a program added to the corpus that
  is not a fixture written to show a cause.

## The analysis, 2026-11-02

### The words: a domain and two arrows

The user, on the words: *"Also maybe better to say not "non-monotonic
lattice" because lattice can't be non-monotonic; but non-monotonic function
of lattice node, or path-dependent function"*. Then: *"lattice is a domain of
non-binary truth (liveness, dischargeness) function, or 2 binary functions;
And while lattice is set-indexed dag - then path-independence ===
monotonicity; Correct me if I'm wrong"*.

```
L ──state──▶ KB ──(dead, owes)──▶ {0,1}²
```

- **The lattice** L = (2^H, ⊆), the commitments over the candidate universe
  H, is the domain. It is a Boolean lattice. It is never monotone or
  non-monotone.
- **The state**, `state(C)`, is the KB a node holds.
- **The status** is two binary functions of the state, `dead` and `owes`:
  the oracle's two bits per node. R = alive ∧ discharged.

So *monotone* is asked of an arrow, and there are two arrows, plus a third,
stronger question about the program.

### Paths: the general form

The user: *"I guess this is a special case of more common sat(P+C1+C2) !=
sat(sat(P+C1)+C2) ? so in general non-monotonic is not only about root
scratch and fixpoint"*. **This is right.** A node's state can be built along
a path, each step forking the last fixpoint, writing the next facts and
saturating:

| construction | form | used by |
|---|---|---|
| scratch | sat(P ∪ C) | `utils/region.py`; the guard split's cells |
| fork from root | sat(sat(P) ∪ C): C1 = ∅ | every shipped route: the oracle, route 1, route 2 |
| a parent path | sat(sat(P ∪ C \ {h}) ∪ {h}), and longer chains | the walk's `-y` check before the switch; any route that extends a parent's fixpoint |

Every shipped route builds a node from root with all of C at once, which is
why C1 = ∅ is the only case a route has met. Measured
([`node-census.md`](../../utils/censuses/node-census.md),
`bc1dff73`, 2026-11-02):

- fork and scratch hold different facts on 4 of the 256 nodes of the 18
  flagged cubes within the limit, on 1 of the 174 nodes of S1e2.4.1's 5
  flagged probes, and on none of the 268 213 nodes of the 155 monotone cubes
  (§ 1–3 there);
- a one-step parent path reaches a different state from the direct fork on 8
  of `d3-unsat-core`'s 26 checked nodes, 2 of `lattice/08`'s 4 and 1 of the
  probe `via_hole`'s 4, and the direct fork is scratch's node on all of them
  (§ 6.1). `lattice/08` shows it with nothing derived at root:
  `soup-unless-allergic` fires at the parent `{seats}` and keeps
  `(meal Ann Soup)` after `(allergic Ann)` is added.

So under today's fork a node's state on a flagged program **can depend on the
path**: on `lattice/08`, `via_hole` and `d3-unsat-core` it is not a function
of C. A search that extends a parent's fixpoint
(P1e2.1c) would build a node other than the oracle's, against the user's
ruling of 2026-10-16, *"results must be consistent: region is a region of
oracle"*. The real question under Q-M1e2.2 is **is a node's state a function
of C alone?** Every split C1 ⊆ C and every maximal chain are measured since
S1e2.1b.7 ([`node-census.md` § 11](../../utils/censuses/node-census.md#11-the-path-census--s1e21b7),
`node_census -- --paths`, 2026-11-02): 20 of the 256 flagged nodes within the
limit reach two states, none three; 10 only through a dead parent; along alive
paths the status moves on 3, `lattice/08`'s hole into R among them; on the
probe `via_hole` and on `ACTIVATOR_UNDER_ABSENT` a node of R is dead by way of
an alive parent; and four of `d3-unsat-core`'s eight one-step differences were
`-y`'s own fail-fast stop.

### The traversal: a tree of paths, and what it may be memoized by

The user, 2026-11-02: *"So, in general case that includes non-monotonic
programs full traversal is again a tree that traverse all paths in a lattice,
where steps are `add H_i` and `sat`; and in path-independent case it callapses
back to lattice or something smaller (sat merges nodes) but in path-dependent
doesn't?"* **Yes, with one refinement: a second memoization survives.**
Argued, not tested:

- **The general object is the path tree.** A node is an ordered sequence of
  distinct candidates, and a step is `s → sat(s ∪ {h})`. Unmemoized it has
  Σₖ n!/(n−k)! = ⌊e·n!⌋ nodes, ≈ 5.7·10¹³ at n = 16 against the cube's
  2¹⁶ = 65 536.
- **Two keys can memoize it:**

  | memoize by | sound when | the tree collapses onto |
  |---|---|---|
  | **C**, the subset | the state is path-independent | the cube 2ⁿ, today's oracle |
  | **the state**, the KB | always, if `sat(s ∪ {h})` is a function of s's contents and not of its insertion history (premise measured: 688 809 steps from contents another history reached, 0 conflicts, [`node-census.md` § 11.5](../../utils/censuses/node-census.md#115-what-it-settles-and-what-it-does-not)) | a DAG of reachable states, acyclic because a step only adds |

- **Path-independent: both keys are sound, and the result is smaller than the
  cube.**
  - The reachable states are the image of C ↦ sat(P ∪ C).
  - Ordered by ⊆, they form a lattice: the join is sat(P ∪ C1 ∪ C2), the
    bottom is root, and the meet exists but is not the intersection.
  - This is the read-out's `merged`, k_c commitments onto k_s states. Zebra's
    270 454 alive commitments land on 27 alive states
    (`walk_and_region.md` § 6).
- **Path-dependent: memoizing by C is unsound**, which is exactly the
  parent-path difference above.
  - Memoizing by state stays sound, but the DAG is not a lattice: one C
    reaches several states, so there is no unique join.
  - It can be larger than the cube.
- **The reading this suggests, (F) in D2.** A node is *the set of states its
  paths reach whose endpoint holds no stale fact*, that is, no `absent` guard
  a path used is contradicted at its end.
  - This is the stability condition of Reiter's default logic and of stable
    models. An `absent` rule is a default; an extension is an application
    order whose justifications all survive.
  - Per C it would give 0 states on an odd cycle (`d3-unsat-core`), 1 on a
    stratified program, and several on an even cycle (`features/07`, where
    the guard split already finds *two stable readings*).
  - **Measured, these counts do not hold** ([`node-census.md` § 11.5](../../utils/censuses/node-census.md#115-what-it-settles-and-what-it-does-not)):
    from P, every node of every flagged cube within the limit has exactly
    one stale-free state, the odd cycle included; from root, W8's four
    `{(q one)}` nodes have none, and the even cycle never reaches its second
    stable model, because the boundary admits `derive-p` first; the written
    stratified `stratified_stale` has none at ∅. So the count depends on the
    admission order and on where a path starts.

### The state: path independence ⇔ monotonicity, given cumulativity

The user's *"path-independence === monotonicity"* holds for the state, given
one more property of saturation, and the engine's admission order is where it
fails.

- **Definitions.** The state is **path-independent** when every path to C
  gives one fact set. It is **monotone** when C ⊆ C′ ⇒ `state(C)` ⊆
  `state(C′)`. Saturation is **cumulative** when X ⊆ Y ⊆ sat(X) ⇒
  sat(Y) = sat(X).
- **⇒ always holds.** Construction is append-only: the KB never retracts
  ([`absent_semantics.md`](../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md):
  *"Retraction / truth maintenance: not provided"*). Take C ⊆ C′ and build
  C′ by way of C. The result holds `state(C)`, and by path independence it is
  `state(C′)`.
- **⇐ needs cumulativity.** Take a path ∅ = K_0 ⊆ K_1 ⊆ … ⊆ K_k = C, with
  S_0 = sat(P) and each step S_i = sat(S_{i−1} ∪ K_i). Suppose
  S_{i−1} = sat(P ∪ K_{i−1}). By monotonicity S_{i−1} ⊆ sat(P ∪ K_i), so
  P ∪ K_i ⊆ S_{i−1} ∪ K_i ⊆ sat(P ∪ K_i), and by cumulativity
  S_i = sat(P ∪ K_i). By induction every path ends at sat(P ∪ C).
- **Who is cumulative.** A monotone program is, by fork: its saturation is a
  closure operator. A stratified program evaluated stratum by stratum should
  be, since its fixpoint is its perfect model. **Measured since S1e2.1b.7**
  ([`node-census.md` § 12](../../utils/censuses/node-census.md#12-the-cumulativity-census-and-the-fifo-gap--s1e21b7)):
  as the engine admits today, saturation is cumulative on every node read
  except where the FIFO race below reaches, four programs, one of them
  monotone and failing from scratch only; with admission in stratum order,
  emulated by priorities, every stratified program read is cumulative on
  every node and every scratch node is stale-free.
- **The FIFO counter-example.** The boundary admits equal-priority candidates
  one a round in (priority, FIFO) order, not in stratum order
  ([`node-census.md` § 4](../../utils/censuses/node-census.md#4-the-staleness-census)).
  Take r2 `a ∧ b ∧ absent(q) → p`, written before r1 `a ∧ absent(z) → q`.
  Built at once, {a, b} gives {a, b, p, q}, with `p` stale. By way of {a} it
  gives {a, b, q}. The state function is monotone, and the state is
  path-dependent: cumulativity fails, since {a, b, q} ⊆ sat({a, b}) and
  sat({a, b, q}) ≠ sat({a, b}). Its relative, the written `stratified_stale`
  (`p ← ¬q`, `q ← ¬r`, one priority, `derive-p` first), is measured: its
  scratch root holds a stale `(p one)`, and swapping the two rules' text
  changes its universe (§ 2.3 and § 7.1 there). **The example as written runs
  as argued**: banked by S1e2.1b.7 as
  [`examples/lattice/10_fifo_gap.ein`](../../examples/lattice/10_fifo_gap.ein),
  `{a, b}` holds a stale `p` built at once and not by way of `{a}`
  ([`node-census.md` § 12.1](../../utils/censuses/node-census.md#121-the-fifo-gap-banked)).

So the state has one property, monotone ⇔ path-independent, broken by an
odd-depth `absent` over D and by the FIFO gap. **It is Q-M1e2.2's subject.**

### A state difference needs an `absent`

The user: *"Is this possible without absent? We already decide the case with
absent in s1e2.4.1"*.

- **For the state, no.** A path or scratch difference needs a fact admitted
  under a read that a later fact falsifies. A positive read, or a read at even
  `absent` depth, only becomes true as facts grow, and a stored negative read
  positively only grows. An activator only adds plans as facts grow. An
  obligation rule asserts `(open …)` and derives nothing. What is left is an
  odd-depth `absent` in a saturation rule (`SiteKind::Absent`). That is an
  argument; the census agrees with it on every node it read: each of the 5
  nodes where fork and scratch differ holds a fact derived under an `absent`
  the commitment falsifies; 7 of the 11 parent-path differences hold such a
  fact (`node_census.md` § 2, § 4, § 6.1). `d3-unsat-core`'s other 4 were
  `-y`'s direct path stopping at its first contradiction, and built to
  quiescence they agree; S1e2.1b.7's path census finds each of the 21 states
  a path reaches beyond fork and scratch holding such a fact
  ([`node-census.md` § 11.5](../../utils/censuses/node-census.md#115-what-it-settles-and-what-it-does-not)).
- **An activator can hide the `absent`.** `region.rs`'s `ACTIVATOR_REFUTES`
  compiles `rule watch` with `(absent (?R ?x))` at a node. Its only site is
  the activator, and a check that reads root's plans does not see the
  `absent`.
- **For the status, yes**: next section.

### The status: `dead` follows the state, `owes` does not

- **`dead`** inherits monotonicity from the state: a contradiction is facts,
  and on a monotone state the facts stay.
- **`owes`** does not. An instance is owed when its trigger holds and no
  witness does. An obligation's trigger, or an activator that compiles an
  obligation, adds a debt over a monotone, path-independent state: holes on
  `lattice/04`, `lattice/09` and `synth/trigger_hole`. Fork and scratch agree
  on every node of `lattice/04` and `lattice/09` (`node_census.md` § 2.1).
  **A hole needs no path dependence.**
- **It is the hole question**, Q-M1e2.7 (d) (D4), and not a question about
  the state.

### The program: a third, stronger property

A **monotone program** (the logic-programming term, kept) has a consequence
operator that is monotone on every KB. Its saturation is then a closure
operator, and both arrows are monotone. That is what the static check
approximates: a site says the operator is not monotone over D. The node
functions can still be monotone on the program's own lattice, and those are
the check's **conservative** flags: 8 of the 18 flagged programs within the
limit at `4562bf70`
([`08-monotonicity.md` § 4](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#4-what-the-cube-shows)).

**The check's verdict is relative to root**, which the analysis above did not
say and the docs now do
([`08-monotonicity.md` § 2](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#2-the-state-the-status-and-the-program)).
A read outside D is static because root completes it and a fork never
re-derives it. A scratch build re-derives root in every node, so the FIFO race
can reach a static read: with `a` given and only `b` a candidate, r2
`a ∧ b ∧ absent(q) → p` before r1 `a ∧ absent(z) → q` has no site, and
scratch on {b} would hold a stale `p` that the fork does not. **Measured since
S1e2.1b.7**: banked as `examples/lattice/11_fifo_gap_static.ein`, the check
calls it monotone, route 1 answers it, and from scratch `{(b one)}` holds the
stale `p` ([`node-census.md` § 12.2](../../utils/censuses/node-census.md#122-the-static-variant-banked)).
So D2's (B) needs admission in stratum order on programs the check calls
monotone as well.

| property | of | broken by | its subject |
|---|---|---|---|
| **state monotone** (⇔ path-independent, given cumulativity) | `state`: L → KB | an odd-depth `absent` over D; the FIFO gap | which KB a node holds; whether a route that extends a parent agrees with the oracle (D1–D3) |
| **status monotone** | `(dead, owes)`: KB → {0,1}² | `dead`: as the state. `owes`: a trigger, or an activator that compiles an obligation | holes, and whether two lines describe R (D4) |
| **monotone program** | the operator, on every KB | any site | the static check's verdict, route 1's licence |

### What *forbid* covers, and what it does not

*Forbid*, ruled 2026-11-02
(S1e2.4.1 § 5),
refuses `(absent P)` ⟹ `(false)` or `(not …)` over a relation a commitment
can still extend. Its refusal is not built: S1e2.2.5 builds it, with F1–F5
open, in P1e2.2, the last phase of the milestone's order. It does not cover:

- **an `absent` that concludes a positive fact**: `derive-p` on `features/07`
  and `stale-naf-fork`, `soup-unless-allergic` on `lattice/08`. Every state
  difference above is of this kind;
- **the derived refutation**, the `via` class: `stale-naf-fork`'s
  `p-and-q-clash` reads a `p` derived under `absent`, and
  [`via_hole.ein`](f21_probes/via_hole.ein)
  is a dead hole through one. Unless F2 takes `via`;
- **a refutation compiled at a node** through an activator
  (`ACTIVATOR_REFUTES`), unless F3's refusal reads the plans a node compiles;
- **a refutation through an activator derived under an `absent`**
  (`ACTIVATOR_UNDER_ABSENT`, in `ein-infer/examples/shared/written.rs`): the
  plan the node compiles reads no `absent`, so even a check of the plans a
  node compiles misses it, unless F2's `via` follows a plan's activator. (E)
  refuses its `act`. Measured by P1e2.1b's
  [taint census § 7](../../utils/censuses/taint-census.md#7-the-refutation-a-node-compiles-for-f3);
- **obligation triggers and activators**, which are not refutations;
- **the FIFO gap**.

### Option (E), and its 13 programs

**(E)** extends *forbid* to every `absent` site (`SiteKind::Absent`: odd
depth, in a saturation rule, over a relation a commitment can extend),
whatever it concludes. *Forbid*'s own statement in S1e2.4.1 § 5 applies word
for word: such a read *"judges a state that has not yet decided P as if P
were false"*.

- **What it settles.** No program that loads keeps a state-breaking site, so
  the state is monotone and path-independent on it: fork = every path =
  scratch. Q-M1e2.2 and its (D) are moot, D3 has nothing to constrain, and
  only trigger and activator holes remain (D4). That holds if the refusal
  reads D, not fact grain, and sees the plans a node compiles (F1, F3); it is
  an argument from the two sections above.
- **What it costs.** `site_census.md` § 2
  (`4b822a8d`) lists 12 programs with an `absent` site that concludes a
  positive fact, and `lattice/08` was banked after it (`6743dd17`): 13.

  | program | site → concludes | under *forbid* already |
  |---|---|---|
  | `corpus/fuzz_findings/d3-unsat-core.ein` | `fire-1` → `r1` | if F2 takes `via` |
  | `ein-bugs/wrong-answers/stale-naf-fork.ein` | `derive-p` → `p` | if F2 takes `via` |
  | `ein-bugs/wrong-answers/stale-naf-fork-warn.ein` | `derive-p` → `p` | if F2 takes `via` |
  | `features/05_stdlib_domain_elim.ein` | `std.elim`'s `domain-elimination` → `color-of` | its three refuting `std.elim` sites, under F1 (a) or (b) |
  | `features/12_expect_false.ein` | `forced` → `in` | its `no-room`, under F1 (a) or (b) |
  | `ein-bugs/wrong-answers/f07-fpoff.ein` | `derive-p` → `p`, `derive-q` → `q` | no |
  | `features/07_unstratifiable.ein` | `derive-p` → `p`, `derive-q` → `q` | no |
  | `features/01_not_and_absent.ein` | `unknown-feeling` → `unknown-feeling` | no |
  | `features/04_open.ein` | `open-candidate` → `open-slot` | no |
  | `saturation/square-unique/corner-house.ein` | `square-unique` → `co-located` | no |
  | `saturation/square-unique/cul-de-sac.ein` | `square-unique` → `co-located` | no |
  | `saturation/square-unique/terminus.ein` | `square-unique` → `co-located` | no |
  | `lattice/08_absent_witness_hole.ein` | `soup-unless-allergic` → `meal` | no |

  The last column is read off the `direct` and `via` sets of
  S1e2.4.1 § 1
  (`6e7dfc49`). Under F1 (b) and F2 with `via`, (E) refuses 8 programs
  *forbid* does not, 9 since S1e2.1b.7 banked `lattice/10_fifo_gap`. Under F1
  (c), fact grain, `features/05` and `/12` load under both. **Each reading,
  measured**: `docs/kernel/inference/monotonicity.md` at `50739fdb` § 8.1, D1.
  **What each would be rewritten as**: § 8.2 (`docs/kernel/inference/monotonicity.md` at `50739fdb`) there, a
  probe each. Within the limit 5 of 6 keep R; `d3-unsat-core` has no rewrite
  that keeps R found.
- **(E) is an option, not a ruling.**

## The decisions

Each one: the question, its options, its evidence, and what holds until it is
taken. **What holds is documented and warned** (P1e2.1b's S1e2.1b.5), and no
answer moves until a decision is taken.

### D1 — (E): *forbid* extended to every `absent` site

**Declined for S1e2.2.5's build, 2026-11-04**, as that stage's recorded
default: *forbid* is built as ruled, `direct` and *via, per activator*, and
the programs whose `absent` concludes a positive fact load. D1 stays open
here. Of (E)'s 13 programs below, three are refused since then by *forbid*
itself, through `via` (`d3-unsat-core`, `stale-naf-fork` and its twin), and
`features/05` and `/12` close the relation their refutations read and load.

[§ Option (E)](#option-e-and-its-13-programs) above. **Its consequences,
per program and per reading of F1 and F2**:
`docs/kernel/inference/monotonicity.md` at `50739fdb` § 8.1, D1,
and the rewrites (E) would ask for, § 8.2 (`docs/kernel/inference/monotonicity.md` at `50739fdb`). **Until
decided:** *forbid* as ruled, and the 13 programs load, 14 with
`lattice/10_fifo_gap`.

### D2 — Q-M1e2.2: what a node's state is

[Q-M1e2.2](../../docs/decisions/answer/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md),
asked of every node. Moot under D1's (E).

| | a node's state | measured consequence |
|---|---|---|
| **(A) fork** | `sat(sat(P) ∪ C)`, today's | nothing moves. W8's rows close `not-wrong` with the reason. Not stable under rule order on `features/07`, `f07-fpoff` |
| **(B) scratch** | `sat(P ∪ C)` | `stale-naf-fork` and its twin gain `{(q one)}`; `features/07`, `f07-fpoff` and the probe `not_conclusion` lose a stale fact from one state. Saturation ×0.2–5.6 a node, root re-derived at every node. L0 stops being the node's construction. Exposed to the FIFO race on a stratified program (`examples/lattice/10_fifo_gap.ein`) **and on one the check calls monotone**, measured since S1e2.1b.7: [`11_fifo_gap_static.ein`](../../examples/lattice/11_fifo_gap_static.ein), whose node `{(b one)}` holds a stale fact from scratch and not by fork. Admission in stratum order, emulated, removes both ([`node-census.md` § 12](../../utils/censuses/node-census.md#12-the-cumulativity-census-and-the-fifo-gap--s1e21b7)) |
| **(C) guards are premises** | (A) or (B), and a state holding a stale fact is not a solution | over (A): `features/07`, `f07-fpoff` lose `{(q one)}`; over (B): nothing on the corpus. A third node outcome, its own status or dead |
| **(D) refuse a cycle through an odd `absent`** | a load error | over D: `features/07`, `f07-fpoff`, `d3-unsat-core` |
| **(F) the stable endpoints of every path** | the set of states the path tree reaches at C whose facts are not stale ([§ The traversal](#the-traversal-a-tree-of-paths-and-what-it-may-be-memoized-by)); default logic's extensions | measured by S1e2.1b.7's path census, and not as argued: from P, one stale-free state on every flagged node, which is (B)'s wherever scratch is stale-free; from root, none on W8's four `{(q one)}`, so R is (C) over (A)'s there. The answer becomes a set of (C, state) pairs, so k_s counts states and not nodes. The cost is the state DAG, not the cube |

Sub-questions: **(D)'s scope**, over D (3 programs) or over all relations (6,
three syntax fixtures with n = 0 added); and **stratum-order admission**,
which closes the FIFO gap. Evidence:
[`node-census.md`](../../utils/censuses/node-census.md)
§ 4, § 5, § 7, § 9 and § 10, which also carries the agent's recommendation,
not the user's: (B) with (D), admission in stratum order. The register's W8
`right` blocks are (B)'s answer (§ 9 there). Stratum-order admission,
emulated, is measured in § 12.4 there. **Each option's consequences, per
program**: `docs/kernel/inference/monotonicity.md` at `50739fdb` § 8.1, D2.
**Until decided:** (A).

**Since 2026-11-04 one of W8's two witnesses is refused**: S1e2.2.5's F2 took
`via`, so *forbid* refuses `stale-naf-fork` (its `p-and-q-clash` reads the
`p` that `derive-p` derives under `(absent (q ?x))`), `W8/stale-naf-fork` is
fixed by the refusal and owned by S1e2.2.5, and `d3-unsat-core`'s two rows
hold the refusal as `right`. D2's W8 question was `W8/features-07`'s alone,
the one row this followup owned, until M1e2 S1e2.2.25 filed three more
(2026-11-05, [§ Handed over](#handed-over-by-m1e2s-inventory-2026-11-05)):
`W8/f07-rules-swapped`, `W8/fifo-gap` and `W8/corner-house`, all four
reviewed `deferred` with this page as `owner`.

**At M1e2's wrong-answer revisit** (S1e2.1.7,
2026-11-03) the user kept this deferral and amended the milestone's acceptance
instead (*"(2) Amend acceptance"*): W8's two rows, `W8/features-07` and
`W8/stale-naf-fork`, stay `open` in `corpus/wrong_answers.toml`, reviewed
`deferred`, with **F21 as their `owner`**, and M1e2 closes naming them. Deciding
D2 settles them. `W1/d3-unsat-core` and `W11/d3-unsat-core` were confirmed
there under (A), today's node, on which fork and scratch agree on all 32 of
their witness's nodes. (D) refuses that witness, so taking it re-opens both
rows with a refusal as `right`, as M1e2 S1e2.2.5's F2 would if it takes `via`.

### D3 — path dependence as a constraint on incremental search

Must a node's state be a function of C alone, so that a route that builds a
node from a parent's fixpoint agrees with the oracle?

- **(i) required of every program:** only a reading under which it holds is
  admissible: (B), or D1's (E). (C) over (A) catches the 7 of 11 path states
  that differ by a stale fact. The other 4, on `d3-unsat-core`, are dead
  whatever the path and differ only in which facts a dead fixpoint holds
  (`node_census.md` § 6.1), so (C) is admissible if a dead node's facts may
  depend on the path. On a stratified program, (B) needs stratum-order
  admission as well (the FIFO gap).
- **(ii) required of monotone programs only:** a flagged program gets no
  route that extends a parent, and keeps the routes that build each node from
  root at once.

Evidence: § 6.1 there, and S1e2.1b.7's path and cumulativity censuses
([§ 11–12 there](../../utils/censuses/node-census.md#11-the-path-census--s1e21b7)). **The consequences for
P1e2.1c**: `docs/kernel/inference/monotonicity.md` at `50739fdb` § 8.1, D3.
**Until decided:** (ii). P1e2.1c's S1e2.1c.2 and .4 are scoped to monotone
programs.

### D4 — what is reported when R has holes: Q-M1e2.7 (d)

Q-M1e2.7 (d):
(a) R, its lines named as R's extremes, every minimal hole with its cause and
repair edges; (b) R as convex pieces; (c) a qualifier; (d) a refusal. With
(d1) dead and owing holes, one word or two, (d2) a claim form, (d3) the stub's
words, (d4) `lattice/04`'s register row. Evidence:
S1e2.1b.3's
[`hole-census.md`](../../utils/censuses/hole-census.md)
(2026-11-02, `cargo run --release -p ein-infer --example hole_census`):

- **the holes**: 18 on 9 of the 18 flagged corpus cubes within the limit, 8
  dead and 10 owing, each with one of six causes and a site (§ 1–4); 166, all
  owing, on `lattice/06_friends_tables` at `--region-limit 19` (§ 7); the same
  by fork and from scratch on every program (§ 6);
- **the shape**: the lines describe R plus its holes; the one-fact test
  misreads 7 maximal and 20 minimal nodes on `06`; R's convex pieces are not a
  partition (§ 5);
- **each option drafted** on six real programs and two written ones, with its
  line count and what it refuses and moves: (d) refuses the 26 flagged
  programs, 8 of them with no hole; (c) moves 150 lines of
  `corpus_shapes.md5`, (a) and (b) 18 (§ 9–10);
- **(d4)**: no row filed; on `lattice/04` it follows from D4 alone.

The agent's recommendation, not the user's: (a), with dead and owing holes
named apart and the dead row naming the `absent` its refutation rests on
(§ 10 there). The counts re-taken with the fixtures S1e2.1b.7 banked:
`docs/kernel/inference/monotonicity.md` at `50739fdb` § 8.1, D4.
**Until decided:** the listing is R, and the `lattice` row names up to four
minimal holes with why each is out
(`read_out.md` § 6.3).

### D5 — the handling: refuse, qualify, or search differently

The question S1e2.1b.5 carried until 2026-11-02, for what the read-out does on
a flagged program once the node is decided: (a) qualify, (b) state what the
static check and the cube saw and qualify nothing, (c) refuse only what D1 or
D2 refuses, (d) search differently (D7). **Until decided:** route 2,
unchanged, and S1e2.1b.5's warnings, which state what is assumed and what was
observed, and decide nothing.

### D6 — learning from a refutation that rests on an `absent`

Until *forbid*'s refusal ships, and afterwards for whatever S1e2.2.5's F2
leaves loading (the `via` class) and F3 does not see (a refutation a node
compiles through an activator): may a route carry such a refutation past the
node that derived it? (a) per program: no route learns on a flagged program;
(b) per refutation: learn from a dead node whose derivation has no `absent`
premise over D, read off provenance, which is
F18's narrow form; (c) the refusal first, a
reorder of the milestone. Evidence:
S1e2.1b.4's
[taint census](../../utils/censuses/taint-census.md)
(2026-11-02, `cargo run --release -p ein-infer --example taint_census`) and the
test it banks, `ein-infer/tests/region.rs::an_untainted_death_is_dead_in_every_superset`:

- on the 18 flagged cubes within the limit, 122 dead nodes, 100 of them
  resting on no `absent` over D and **none of those with an alive superset**;
  every dead hole tainted. The readings *in D* and *eligible*, and the
  oracle's own `absent_premises`, agree on every one;
- **(b) read off provenance alone is unsound** on the written
  `ACTIVATOR_UNDER_ABSENT`: a derivation does not record the activator its
  plan was compiled from, so a death resting on an activator derived under an
  `absent` looks untainted. With each plan's activator followed, (b) holds on
  every cube the test reads;
- what (b) buys: 91 of the 256 nodes of those cubes, 88 on
  `synth/three_components`. Past the limit nothing is measured.

The agent's recommendation, not the user's, conditional on the test as the
stage put it: (b), with the taint following each plan's activator, which the
test holds; (a) if that walk is not what is built.

Its consequences beside the path census:
`docs/kernel/inference/monotonicity.md` at `50739fdb` § 8.1, D6.

**Closed 2026-11-04**, by D6's (c) in the form this paragraph names: the
refusal shipped (M1e2 S1e2.2.5) and covers every refutation a route can learn
from that the static check sees: F2 took `via` following each plan's
activator, and F3 reads root's plans and every rule with parameters whose
activator relation is in D, at load. No program that loads has a death a
route could carry past its node on the corpus, the probes or the written
programs ([`refutation_under_absent.rs`](../../ein.rs/crates/ein-infer/tests/refutation_under_absent.rs),
`region.rs::an_untainted_death_is_dead_in_every_superset`, which studies the
refused shapes past the refusal). F18 closes with it. Until then: (a). F18
closes without being done once the refusal covers
every refutation a route can learn from: D6's (c), or D1 with F2's `via` and
F3. Since S1e2.1b.4, `via` there has to follow a plan's activator, or D1's (E)
has to be taken, for `ACTIVATOR_UNDER_ABSENT` to be covered.

### D7 — the non-monotone search past the oracle's limit

Was M1e2's S1e2.1c.3,
moved here 2026-11-02; its file keeps the design notes. (a) the guard split;
(b) the cube past the limit when asked by name, `--region-limit`; (c) partial,
C3. Evidence: `docs/kernel/inference/monotonicity.md` at `50739fdb` § 6.4:
the split agrees with the oracle on 19 of 21 cubes and disagrees on W8's two,
because a cell builds its node from scratch; g ≪ n on 2 of the 8 flagged
programs past the limit, and g = n = 19 on `lattice/06_friends_tables`. The
split presumes D2. Re-taken by S1e2.1b.7, 20 of 22 agree:
`docs/kernel/inference/monotonicity.md` at `50739fdb` § 8.1, D7.
**Until decided:** (c), and (b) when asked.

## Handed over by M1e2's inventory, 2026-11-05

The inventory of 2026-11-05, taken before M1e2's close at the user's
request, found items on flagged programs that had this followup's decisions as
their owner and no record here. The user ruled where each goes the same day
(Q-M1e2.16 (3) and (5)).

**Three wrong answers, filed as rows.** Each answers by today's documented
node, (A), and is wrong under at least one reading D2 lists.
S1e2.2.25 filed each as an `open` register row, reviewed `deferred`,
with this page as its `owner` (2026-11-05), each re-taken at `94c75204` in
release and debug, and each row's `right` giving the answer under each
reading D2 lists:

| inventory id | what is false | decision | row |
|---|---|---|---|
| W-2 | `features/07`'s R depends on which of its two priority-200 rules is written first: as written R = {∅, {(q one)}}, swapped R = {∅, {(p one)}} | D2 (rule order) | `W8/f07-rules-swapped`, witness [`f07-rules-swapped.ein`](../../examples/ein-bugs/wrong-answers/f07-rules-swapped.ein); primary `right` (D)'s refusal, the one reading that gives either order one answer |
| W-5 | the FIFO gap: `lattice/10_fifo_gap`'s node `{a, b}` holds a stale `p`, where the stratified reading gives `{a, b, q}` | D2 (stratum order) | `W8/fifo-gap`, witness `examples/lattice/10_fifo_gap.ein`; primary `right` admission in stratum order |
| W-6 | members of R past the oracle's limit hold a fact whose `absent` guard their own fixpoint falsifies: `features/01`, 24 754 of 64 655 nodes (`ein solve FILE --json-summary`, `monotonicity.state`, 2026-11-05), `features/04_open`, `square-unique/*` | D2, D7 | `W8/corner-house`, the cheapest of the five (`examples/saturation/square-unique/corner-house.ein`, 5 595 of 65 389, 1.4 s); primary `right` (C), no member of R holding a stale fact |

With `W8/features-07`, this page owns four rows. (Five since 2026-11-17: `W8/fifo-gap-root`,
[below](#handed-over-by-m1f-2026-11-17-the-fifo-gap-at-root).)

**Five labels.** None moves an answer. Each is a label the detector or the
static check prints or reads, and each is decided with the reading it labels.
M1e2's close hands them over here:

| inventory id | what | source |
|---|---|---|
| S-6 | the detector's `RootPlans` keys root's plans by the activator's symbols, so an all-int obligation activator derived at a node would read as root's and its hole go unattributed. No witness | S1e2.2.16 § Notes |
| S-7 | the detector's chain attribution reads every application of a rule as its activator, and seeds an owing node from symbol arguments only. No witness; S1e2.2.19's `Prov::activator` could feed it | S1e2.2.17 § Notes |
| S-8 | the static check's D does not follow activators, so its site list calls a read static that a plan compiled at a node makes dynamic; the verdict holds. `forbid-chain-node-writes.ein`, which *forbid* refuses | S1e2.2.17 § Notes (its default 2) |
| S-9 | `absent_premises` (the dumps, `region.md`'s *rests on*, the detector's input) is read off the first contradiction's positive side, and follows neither the activator nor the negative side the core follows since S1e2.2.19 and S1e2.2.21. Library only | S1e2.2.19 § Notes; S1e2.2.21 § Notes |
| S-11 | `features/01`'s activator site is a relation-grain name collision: the rule writes `unknown-feeling` at arity 2, its activator has arity 1. It cannot bite, and it keeps the program flagged | `site_census.md` § 2 |

## Handed over by M1f, 2026-11-14

One label more, E-66 of M1f's P1f.6, moved here
on the user's word of 2026-11-14 (*"Q-M1f.271 (a) is fine, E-66 and E-68 - to followups"*). Like S-6 to S-11 above it moves no answer and no command's output: it
is a reading the static check makes of a program's rules, and it is decided with what that check labels.

| id | what | source |
|---|---|---|
| E-66 | the static map of what rules derive, `ein_infer::compile::asserted_relation` and `negated_relation`, reads a plan's first conclusion alone (`plan.assert_template()`), so a relation a rule concludes only as its second or a later conjunct is not *producible* to it: `naf_deps::compute_naf_map` files an `(absent …)` over it as watching a declared-only relation, so the stratification advisory (*watches rule-derived relation(s)*) is not given for it, and `closed::emit_closed`, the auto-closure pass, closes it; `naf_deps::refuting_conclusion` reads every conclusion. No command prints either: the shape digests (`shape::naf_map`, `hyp_shape_with`) and tests read them. Reading every conclusion moves those digests on each program whose rule concludes two relations and reads the second under `absent` elsewhere | S1f.6.37's T1f.6.37.2, the inventory of what reads a conclusion (2026-11-14) |

## Handed over by M1f, 2026-11-16: rewriting a non-monotone program

**The question**, the user's, verbatim:

> into F21: is it possible to rewrite some or any non-monotone program preserving semantics? what means this
> rewriting? example - replacement of refutation under absent with debt under absent.

**Its setting.** The same day the user put non-monotone programs out of scope of any claim (Q-M1f.313's ruling):
*"we know about them, can detect them, can solve them somehow, but don't claim about solution soundness,
correctness or completeness or anything"*, and gave the reason: *"for non-monotone programs `sat(root + C1 + C2) =
sat(sat(root + C1) + C2)` is not always true"*. So a non-monotone program has no one region to preserve: the engine
computes the region of its own construction of a node (a fork of root's fixpoint with all of C written at once, D2
above), and that is a convention, not a meaning.

**What a rewriting would be** (the main session's reading, for the user to correct). A transformation T of a
program's text, total on a class of programs the static check flags, such that:

1. **T(P) passes the static check**: it is monotone, so every licence applies to it and the engine can claim of it
   what it claims of any monotone program;
2. **it preserves a named semantics of P**: R(T(P)) = R(P) over the same H, or, where T adds candidates, the
   projection of R(T(P)) onto H is R(P). *Which* semantics of P is preserved has to be chosen first: the region
   of today's construction (D2's fork route), the region of another construction (from scratch), or an intended
   reading of `absent` (stratified, stable, well-founded). Choosing it is D2 in another form;
3. **it is checked, not argued**: P and T(P) are held to one region by the oracle on every cube within its limit,
   the *two forms, one answer* invariance the gate already uses for the import forms and the id order.

It could be offered three ways: a load-time rewrite (the program as run is T(P)), a suggestion beside the
refusal or the warning (the author writes T(P)), or a check (`ein` says which flagged programs have a rewrite).

**The example: a refutation under `absent` → a debt under `absent`.**

```text
P:     p(x) ∧ absent q(x)  ⇒  (false)          a refutation: the state is dead
T(P):  p(x) ∧ absent q(x)  ⇒  owe q(x)         an obligation: the state owes q(x), and a witness q(x) pays it
```

- **Same R, where it applies.** R is the alive and discharged commitments, so a state that holds `p(x)` and not
  `q(x)` at its fixpoint is outside R either way: dead in P, owing in T(P). A state that holds `q(x)` is not refuted
  in P and owes nothing in T(P). A `(false)` derives nothing further, so the rest of the fixpoint is the same.
- **Why T(P) can be monotone where P is not.** In P, *dead* is not upward-closed: `C` without `q(x)` is dead and
  `C ∪ {q(x)}` alive, which L1 would get wrong, and which is why *forbid* refuses such a refutation over a relation a
  commitment can extend (§ What *forbid* covers). In T(P) the same set *owes*, and *discharged* stays upward-closed:
  a witness, once in a state, stays. The obligation is read on the quiescent KB, where `absent q(x)` is read today.
- **Where it does not apply.** T(P) is monotone only if the debt's **trigger**, `p(x)`, is static: if a commitment
  can bring `p(x)`, the obligation's trigger site flags T(P) as it flags any obligation over a relation a commitment
  can change (`lattice/04`, `lattice/06`'s conditional obligation). And where `q` is a relation no commitment can
  extend, P is not flagged and needs no rewrite.
- **What differs beyond R.** A dead node and an owing node report different things: a core and a place in N against
  a debt and a place in the owing frontier; L1 no longer prunes above the set (correctly, now), and L5 can branch on
  `q(x)`'s candidates where its premise holds. So the surfaces move and R does not.
- **It is what *forbid* refuses today.** Such a P does not load. T would turn a refusal into a monotone program with
  the region P's fork construction gives, which the refusal's message could name.

**Some, not any.** Over the same H, no: a monotone program's region is convex (an alive down-set met with a
discharged up-set, so no holes, § Paths above), and a non-monotone program's region can be any subset of the cube
(`utils/region_synth.py` writes a program for any target). A region with a hole has no monotone program over the
same H. Over a larger H, perhaps every one: add a candidate for each `absent`'s truth value, guess it, and check
the guess by obligations and refutations, the guess-and-check of answer-set programming. That preserves a chosen
semantics, the stable one, which need not be the fork construction's region: so *any* is possible only by deciding
what the program means, which is D2.

**What picking it up would take**: the flagged corpus programs by the class of their non-monotone site (the taint
census's classes: a refutation under `absent`, an `absent` that concludes a positive fact, the `via` class, the
activator classes, obligation triggers); for each class whether a rewrite exists and under which semantics; each
rewrite held to the oracle on both forms; and the semantics chosen. Nothing is built; it is parked with the rest of
this page.

## Handed over by M1f, 2026-11-16: four findings of P1f.10

P1f.10's close proposed these four for this followup, and the user accepted them as proposed (*"Accept as
proposed"*, 2026-11-16). They are moved here verbatim from the phase's `findings.md` when its folder is deleted
(S1f.20.9); links into the phase's own pages are their label text. H6-1 is the static check's (an hrule that reads
what a commitment can change, which nothing flags); H6-3 and H6-4 go with it; K-41 is the coupling's view of a
native mirror.

#### H6-1 — An hrule that reads what a commitment can change proposes at a node what it did not at root, and nothing flags it

Q-M1e.11's loss without an obligation derived at a node. H is the generator's list at root's fixpoint, and an
hrule is matched there and nowhere else. Where an hrule's `:match` reads a relation another hrule proposes, the
facts it would propose once a commitment holds one are never candidates.

`hr/reads_h.ein`:

```
;; S1f.10.6, T1, the hrule rung with no derived activator: both debts are
;; stated, there is no rule `arm`, and the hrule that proposes a drink reads
;; `likes`, which the other hrule proposes.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why "{?p} likes {?f}; guess: (drinks {?p} {?d})?")
```

`hr/reads_h_stated.ein`:

```
;; S1f.10.6, T1: hr/reads_h.ein with the fact its second hrule reads stated.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(likes Ann Soup)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why "{?p} likes {?f}; guess: (drinks {?p} {?d})?")
```

`ein solve` and `ein oracle` on the first: H is `(likes Ann Soup)`, one candidate; the node that holds it owes
*drinks owes Ann a Drink*; **R is empty, every alive node owing**. On the second, which states the fact the
second hrule reads: H is `(drinks Ann Tea)` and R one state. `owed_census --iterate` on the first: the
generator at `{(likes Ann Soup)}` proposes `(drinks Ann Tea)`, and R over the two candidates is one state,
`{(drinks Ann Tea) (likes Ann Soup)}`, which `ein oracle` answers on the copy that states both
(`price/reads_h_r2.ein`).

**Nothing detects it.** No rule concludes an activator and the static check does not read an hrule's plan, so
the program is *monotone*: `ein solve` takes the state route and `--single` the obligation tree with L5. Root
owes both instances, so `check_owed` has nothing to refuse. The read-out's `outside` row is silent: no
saturation rule writes `drinks`, no query reads it, and an hrule asserts it.

**The corpus** (`owed_census`, 2026-11-15, the command in the stage's record). Four programs have an active
hrule whose `:match` reads a relation in `D`: `examples/ein-bugs/wrong-answers/hrule-reads-unknown.ein`
(`paid`), and `examples/ein-bugs/wrong-answers/past-limit-lines.ein`,
`examples/lattice/04_obligation_trigger_hole.ein` and `tests/region/07_cube.ein` (`is-a`). On none does a node
propose a fact outside H: three by the cube at the oracle's default limit, the second at `--limit 17`. Of the
88 programs on the hrule rung the cube read 61, and **no corpus answer is known to be concerned**.

**How it reads against the rulings.**
[`06-solution-semantics.md` § 1.4](../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe) defines a candidate as
*a fact the generator proposes at root*, so R is right by the definition. The same section holds that *H must
contain every fact a model needs* (ruling (h)), and that where it does not *either the program is wrong … or
the engine is*: which of the two an hrule conditioned on a guess is, is the user's. **Not filed as a register
row.**

**What it bears on.**

- **Q-M1f.307.** Answer (A) read over nodes (the record's reading 2) gives the twin's model here. Read
  statically from activators (reading 1) it does not reach this program, and (B), (C) and (D) as worded do
  not either: no activator is derived, and the program is monotone.
- **README § The domain's second paragraph**, *H does not grow*, is enforced for
  debts on a monotone program and for nothing an hrule proposes. Track S reads H as root builds it either way;
  what is not enforced is that no node would propose more.

**Today:** open, to be shown with Q-M1f.307. No owner.

#### H6-3 — The mechanisms check reads root's activators, so a list that takes out what a later debt owes is accepted

`mechanisms::check` refuses a program in which an obligation owes a relation that `:no-hypothesis`,
`:hypothesis-relations` or a closure takes out (O + NH, O + HR, O + C), and reads an obligation as *a plan
root's activators compile*. An obligation whose activator only a commitment derives is not read, so the same
list beside it loads.

The witness with `drinks` in its list, `(solve :no-hypothesis (is-a drinks))`: it loads, H is
`{(likes Ann Soup)}`, and R is empty, every alive node owing; the node that likes Soup owes a drink no commitment
may be about. With `(total-owed drinks is-a)` also stated the program is refused at load: *solve: its
:no-hypothesis names drinks, and obligation 'total-owed', activated by (total-owed drinks is-a) … owes drinks — an
obligation is paid by facts of the relation it owes, so a state that owes drinks and may not be given one is
stuck: drop drinks from the list*. Both runs are the same before task 4 and after it (`ein solve` on the two
programs, the binary of `a3c719bf` and the fixing commit's).

Since task 4 the obligations rung knows that debt: it reads the activator off `arm`, compiles its plan, and
scopes it out by the list, as it scopes out a plan of root's, without the `hypskip` line or the *stuck* report a
debt root owes gets. So the engine now holds, at load, the fact the check would need. **Today:** an empty R of
the owing kind where the twin that states the activator gets a load error that says why. Not a wrong answer by
the definitions: the program says no commitment is about `drinks`. A fix would have the check read the rung's
later debts, which is one more reader of `forbid::derivable_activators`; whether a debt a commitment *can*
bring refuses a program at load, at relation grain and so on the *over* case too, is a ruling.

#### H6-4 — `hypgen::eligible_relations` does not name a later debt's relation

`eligible_relations` answers *which relations can this program's generator still propose*, for
`naf_deps`' refutation advisory and for `structure.rs`, and on the obligations rung it is the `(open ?R)`
arguments of the plans root's activators compile. Since task 4 the rung proposes facts of a later debt's
relation too, and that relation is not in the set (the function's doc says so). Reading it there would mean
running the rung's scans, which the advisory's one caller does before root has a hypothesis.

**Today:** no corpus program differs: the one program with a later debt, the witness, has no `absent` for the
advisory to read, and *forbid*, which is what refuses a refutation resting on an `absent` over a relation a
commitment extends, reads `D` from H's own relations and so does see it. `scope::outside`, the read-out's
`outside` row, was given the later relations in task 4 and is right.

#### K-41

| id | what | witness, and the command | found by | today | state |
|---|---|---|---|---|---|
| K-41 | **The whole node does not read a native mirror's derivation of a fact committed in it, so the coupling can leave a member of N in no edge.** The saturator's `__symmetric__` mirror returns a firing only where it writes a new fact; where the mirror is there already it records an alternative justification, which the store refuses for a fact whose primary is a commitment (`Kb::accepts_justification`) and which `:record-alternative-justifications false` does not ask for. The whole node commits every candidate, so of two candidates that mirror each other neither is read as derived from the other, and a death that reads one through the other's mirror is read as reading it committed. The probe: `(__symmetric__ knows)`, hrules proposing `(knows Ann Bob)` and `(knows Bob Ann)` (from `(pair Ann Bob)`, `(pair Bob Ann)`), `(likes Cid)` and `(sings Cid)`, and one rule concluding `(false)` from `(knows Bob Ann)`, `(likes Cid)` and `(sings Cid)`. The oracle's N is `{(knows Ann Bob), (likes Cid), (sings Cid)}` and the same with `(knows Bob Ann)`; the structure's coupling has two edges, `{(knows Ann Bob), (knows Bob Ann)}` (a tie, *implies*) and `{(knows Bob Ann), (likes Cid), (sings Cid)}`, so no edge holds a question of each member of the first. That is the claim `ein-infer/tests/coupling.rs` holds wherever a describe exhausts on the corpus, *every member of N lies inside one edge*; of the corpus programs that search, one writes `__symmetric__`, `ein-bugs/wrong-answers/dunder-object-symmetric-by-rule.ein`, and nothing in it dies | the program above, kept as `build/s18/probes/p5_native_mirror.ein`; `ein render h-space FILE --dump DIR`, then `jq '.coupling.edges[].questions' DIR/h-space.json`; `ein oracle FILE --dump DIR2`, N in `DIR2/region.md` | S1f.10.18, 2026-11-16, T1's item *the kernel's mirror* | not a wrong answer: the coupling orders the tree and moves no answer. A claim of the structure that holds on the corpus and not in general, and a premise of S1f.10.18's narrowed claim (no `__symmetric__` relation) | open, no owner. The next way is the whole node reading a mirror's redundant application as a firing (`Nodes::with_fixpoint_supported_within`), and the probe banked with a corpus entry |

## Handed over by M1f, 2026-11-17: the FIFO gap at root

M1f's documentation phase, P1f.20, found it writing the saturation pages (its finding L-61, the kb-evolution
lane; reproduced on the gate binary), proposed a register row and left the row the user's call. It is filed at
that phase's close, under the standing rule that a wrong answer is a row: **`W8/fifo-gap-root`**, open, this
page's. With it this page owns five rows of W8. The phase's folder is deleted
(`git log --diff-filter=D -- 'plans/m1f_hypothesis_and_documentation/p1f.20_*'`); what it found and
left is [the milestone's findings page](../m1f_hypothesis_and_documentation/findings.md).

**What is false.** Rule order decides root's fixpoint on a program the static check calls monotone, and nothing
says so. Two rules that each wait on an `absent`, the reader written before the producer, with both their positive
premises given: the boundary admits the one written first, so `r2` derives `(p one)` under `(absent (q one))`
before `r1` derives `(q one)`, and root holds a fact its own fixpoint's guard refutes. Written in the other order,
or with `r1` also concluding a stored negation (which the agenda fires first,
[Q-M1f.313](../../docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)), root holds no
`p`. All three print *lattice monotone (the static check)*, with no warning: the check is of what a commitment can
change, and takes root as given
([`08-monotonicity.md` § 3.4](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md)).

It is the FIFO gap of `W8/fifo-gap` (`examples/lattice/10_fifo_gap.ein`, where `a` and `b` are candidates and the
program is flagged) and of [`11_fifo_gap_static.ein`](../../examples/lattice/11_fifo_gap_static.ein) (`a` given, `b`
a candidate, a monotone verdict, and no answer wrong because every node is a fork of root), **moved one step
down**: with `a` and `b` both given the race is root's own, and every answer is built on root.

| program, under [`plans/wrong_answers/witnesses/`](../wrong_answers/witnesses/) | what differs | `ein solve`: the query `(p ?x)` | root holds (`ein saturate --dump`) |
|---|---|---|---|
| [`fifo-gap-root.ein`](../wrong_answers/witnesses/fifo-gap-root.ein), the witness | `r2` (reads `q` under an absent) written before `r1` (derives `q`) | `?x = one` | `(p one :rule r2)` and `(q one :rule r1)` |
| [`fifo-gap-root-r1-first.ein`](../wrong_answers/witnesses/fifo-gap-root-r1-first.ein) | `r1` written first, nothing else | *the goal has no row* | `(q one :rule r1)` |
| [`fifo-gap-root-negation.ein`](../wrong_answers/witnesses/fifo-gap-root-negation.ein) | the witness's order; `r1` concludes `(and (q ?x) (not (w ?x)))`, `w` read by nothing | *the goal has no row* | `(q one :rule r1)` and `(not (w one) :rule r1)` |

Each states `(solve :no-search)`, so nothing is a candidate and R is {∅}; each prints
*lattice monotone (the static check) — R is convex: no hole*. `python3 utils/wrong_answers.py` holds the row's run.

**The row's `right`** is the answer under **D2's stratum-order admission**, the reading `W8/fifo-gap`'s row takes:
the program is stratified (`r1`'s stratum before `r2`'s; nothing derives `z`), so its perfect model is
`(a one) (b one) (q one)` and the query has no row. It is the one reading of D2's under which the three programs,
which differ in nothing a stratified program's meaning depends on, have one answer. The alternatives the row
records: (A) or (B) in FIFO order, today's answer (at root the two are one node); (C) over (A), under which root
holds a stale fact and is no solution, so R is empty on the witness and {∅} on its twins; (D), which does not
refuse it, there being no cycle. The row is not reviewed, and its reading rules nothing
([Q-M1f.134](../m1f_hypothesis_and_documentation/open_questions.md#q-m1f134--what-a-register-row-records-as-right-while-f24s-question-is-not-ruled)).

**What it adds to D2**, beyond `W8/fifo-gap`:

- **A monotone verdict does not make the answer a function of the program.** The engine claims nothing of a
  flagged program; this one is not flagged. The detector's order findings (`monotonicity.order`) are a flagged
  program's alone, and the summary's `monotonicity.state` is empty here.
- **Stratum-order admission is the only one of D2's readings that settles it**: (A) and (B) coincide at root and
  both keep the race, and (C) moves the dependence on rule order from a fact to R's emptiness.
- **The agenda's first half already reorders the admission** where a rule concludes a stored negation, so today's
  order is neither the written one nor the strata's.

**The user's calls**, none taken:

1. Whether the row's reading stands, with D2's sub-question on stratum order.
2. Whether the static check should report an order finding at root: a program in which a rule's `absent` reads
   what another rule waiting on an `absent` derives is not *order-free at the boundary*
   ([`03-transformations.md`](../../docs/ein/reasoning/03-rule-evolution/03-transformations.md)), and that is
   readable off the rules.
3. Three statements still say more than holds, and were not edited: the *Why* of Q-M1f.313's decision file (*on a
   monotone program the fixpoint does not depend on the order*); the header of
   `ein-infer/tests/naf_semantics.rs` (*priority stops deciding what is derivable*); the first line of
   `ein-infer/tests/agenda.rs` (*on every monotone program*: it is a sweep of the corpus's). The pages were scoped
   at the review: the order decides no answer only where no rule's `absent` reads what another rule waiting on an
   `absent` derives (`AGENTS.md` § The language says it so).

### Met again by P1f.22: *everybody's thoughts are false* at a dead root

**Handed over** at the close of M1f's P1f.22, nested facts (2026-11-18): its finding N-15, from the user's notes of
2026-11-17 (*"to infer false if everybody's thought are false (just an example)"*), worked out on that phase's
page on contexts and left as the user's call. No register row is filed for it: whether it is a row, and whose, is
the call below.

**What.** *Everybody's thoughts are false*, written as two rules over `absent`, one that derives `(some-sane)` for a
person with no `(think ?p (false))` and one that concludes `(false)` where `(some-sane)` is absent, answers dead with
one person not confused, under `lattice monotone (the static check)`. Both rules wait on an `absent` at one boundary
and the agenda admits the one that concludes `(false)` first. `fifo-gap-root`'s witness shows a stale fact at an
alive root; here R is empty where the stratified reading gives {∅}. The same sentence written with `forall`, one
nested absence, is right.

`x2b.ein`:

```lisp
(person Ann)
(person Bob)
(think Ann (false))
(rule confused () :match (think ?p (false)) :assert (confused ?p) :why "c")
(rule someone-sane () :match (and (person ?p) (absent (think ?p (false)))) :assert (some-sane) :why "s")
(rule all-confused () :match (absent (some-sane)) :assert (false) :why "a")
```

`ein solve x2b.ein`: `states (k_s) 0`, `lattice monotone (the static check) — R is convex: no hole`, `unsat core ∅,
the empty set: the refutation rests on no fact`, `refuted by all-confused`. `x3b.ein`, the `forall` form:

```lisp
(import std.macro :symbols (forall))
(person Ann)
(person Bob)
(think Ann (false))
(rule all-confused ()
  :match  (forall ?p (person ?p) (think ?p (false)))
  :assert (false)
  :why    "everybody's thoughts are false")
```

`ein solve x3b.ein`: `states (k_s) 1`, `∅ in R — root is alive and owes nothing`. Both re-run on the gate build at
P1f.22's close. With `(think Bob (false))` added, the `forall` form is dead, as it should be
(`examples/beliefs/10_everybody_contradicts.ein` holds it; `09_false_in_a_context.ein` the alive case).

**The user's call**: whether it is a twin of `W8/fifo-gap-root`, read with that row's reading (D2's stratum-order
admission, under which R is {∅}), or a row of its own; and with that row's first call, whether the reading stands.

## Why it is parked

- The user's direction, above.
- Each option moves answers or refuses programs that are named, and the
  measurements are in. What none of them has is a program that needs the
  decision: within the oracle's limit, every flagged cube with a violation was
  written to show one ([`08-monotonicity.md` § 5](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#5-the-ladder)).
  `lattice/06`'s conditional obligation is the exception the user wrote, and
  it is past the limit.
- Until a decision, what holds is documented and every surface that reports R
  says so.

## Read before deciding

- [`08-monotonicity.md`](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md): the two
  kinds of program compared; § 7, this analysis with every claim labelled by
  its standing; § 8, what is open.
- [`node-census.md`](../../utils/censuses/node-census.md)
  (D2, D3), `site_census.md`
  (D1, D7), S1e2.1b.3's `hole_census.md` (D4), S1e2.1b.4's taint census (D6),
  S1e2.1b.7's consequences (`docs/kernel/inference/monotonicity.md` at `50739fdb` § 8.1–8.2,
  all) and its path and cumulativity censuses
  ([`node-census.md` § 11–12](../../utils/censuses/node-census.md#11-the-path-census--s1e21b7), D2 and D3).
  The causes as one table: [`08-monotonicity.md` § 9](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#9-the-causes).
- S1e2.4.1:
  *forbid*, its F1–F5, the `via` set and the rewrites that keep R. Its fourteen
  probes are kept here, in [`f21_probes/`](f21_probes/), since M1e2's folder is
  deleted: the hole, node and taint censuses run them, and `via_hole.ein` is
  this page's dead hole through `via`.
- [`absent_semantics.md`](../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md):
  *"Retraction / truth maintenance: not provided"*, which D2's (B) and (C)
  would qualify.
- F18, whose closure rode on *forbid* (D6).
- Prior art: stable models (Gelfond & Lifschitz, ICLP 1988), the guard split's
  move; stratified programs and their perfect model (Apt, Blair & Walker,
  1988), D2's (D) and stratum order; the well-founded semantics (Van Gelder,
  Ross & Schlipf, *J. ACM* 1991), a third reading none of the options names;
  truth maintenance (Doyle, *Artificial Intelligence* 1979), what (B) and (C)
  approximate.
