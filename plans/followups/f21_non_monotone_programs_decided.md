# F21 — Non-monotone programs: node functions and path dependence, decided

**Filed 2026-11-02**, from M1e2's
[P1e2.1b](../m1e2_correctness/p1e2.1b_non_monotone_programs/README.md), on
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
[`monotonicity.md` § 7](../../docs/kernel/inference/monotonicity.md#7-two-properties-of-the-node-functions-and-path-dependence).
Each option's measured consequences per program are P1e2.1b's
[S1e2.1b.7](../m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_the_consequences_written_down.md)'s.

**Trigger** (any one):

- a route that builds a node by extending a parent's fixpoint is to run on a
  program the static check flags (D2, D3);
- R is wanted past the oracle's limit on a flagged program, such as
  `lattice/06_friends_tables`, the conditional obligation (D7);
- M1e2's [S1e2.2.5](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md)
  builds *forbid*'s refusal: (E) is taken with it or explicitly declined, and
  D6's window closes;
- the wrong-answer revisit ([S1e2.1.7](../m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md))
  cannot settle W8 under today's documented node
  ([S1e2.2.6 § The deferral](../m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md#the-deferral-and-the-milestones-acceptance));
- M1f's [S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.6_rule_priority.md),
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
([`node_census.md`](../m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md),
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
of C alone?** Longer chains and every split C1 ⊆ C are not measured yet
(S1e2.1b.7's path census).

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
  | **the state**, the KB | always, if `sat(s ∪ {h})` is a function of s's contents and not of its insertion history (premise unverified for the engine) | a DAG of reachable states, acyclic because a step only adds |

- **Path-independent: both keys are sound, and the result is smaller than the
  cube.**
  - The reachable states are the image of C ↦ sat(P ∪ C).
  - Ordered by ⊆, they form a lattice: the join is sat(P ∪ C1 ∪ C2), the
    bottom is root, and the meet exists but is not the intersection.
  - This is the read-out's `merged`, k_c commitments onto k_s states. Zebra's
    270 454 alive commitments land on 27 alive states
    ([`walk_and_region.md` § 6](../m1e2_correctness/p1e2.1a_the_solution_region/walk_and_region.md#6-what-this-means-for-question-3)).
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
  - These per-C counts are **unverified**. S1e2.1b.7's path census can
    compute them on small cubes, over the state-memoized DAG.

### The state: path independence ⇔ monotonicity, given cumulativity

The user's *"path-independence === monotonicity"* holds for the state, given
one more property of saturation, and the engine's admission order is where it
fails.

- **Definitions.** The state is **path-independent** when every path to C
  gives one fact set. It is **monotone** when C ⊆ C′ ⇒ `state(C)` ⊆
  `state(C′)`. Saturation is **cumulative** when X ⊆ Y ⊆ sat(X) ⇒
  sat(Y) = sat(X).
- **⇒ always holds.** Construction is append-only: the KB never retracts
  ([`absent_semantics.md`](../../docs/kernel/inference/absent_semantics.md):
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
  ([`node_census.md` § 12](../m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#12-the-cumulativity-census-and-the-fifo-gap--s1e21b7)):
  as the engine admits today, saturation is cumulative on every node read
  except where the FIFO race below reaches, four programs, one of them
  monotone and failing from scratch only; with admission in stratum order,
  emulated by priorities, every stratified program read is cumulative on
  every node and every scratch node is stale-free.
- **The FIFO counter-example.** The boundary admits equal-priority candidates
  one a round in (priority, FIFO) order, not in stratum order
  ([`node_census.md` § 4](../m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#4-the-staleness-census)).
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
  ([`node_census.md` § 12.1](../m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#121-the-fifo-gap-banked)).

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
  fact, and `d3-unsat-core`'s other 4 are dead nodes that differ by admission
  order on an odd `absent` loop (`node_census.md` § 2, § 4, § 6.1).
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
([`monotonicity.md` § 5.2](../../docs/kernel/inference/monotonicity.md#52-on-the-cube)).

**The check's verdict is relative to root**, which the analysis above did not
say and the docs now do
([`monotonicity.md` § 7.6](../../docs/kernel/inference/monotonicity.md#76-the-program-the-third-stronger-property)).
A read outside D is static because root completes it and a fork never
re-derives it. A scratch build re-derives root in every node, so the FIFO race
can reach a static read: with `a` given and only `b` a candidate, r2
`a ∧ b ∧ absent(q) → p` before r1 `a ∧ absent(z) → q` has no site, and
scratch on {b} would hold a stale `p` that the fork does not. **Measured since
S1e2.1b.7**: banked as `examples/lattice/11_fifo_gap_static.ein`, the check
calls it monotone, route 1 answers it, and from scratch `{(b one)}` holds the
stale `p` ([`node_census.md` § 12.2](../m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#122-the-static-variant-banked)).
So D2's (B) needs admission in stratum order on programs the check calls
monotone as well.

| property | of | broken by | its subject |
|---|---|---|---|
| **state monotone** (⇔ path-independent, given cumulativity) | `state`: L → KB | an odd-depth `absent` over D; the FIFO gap | which KB a node holds; whether a route that extends a parent agrees with the oracle (D1–D3) |
| **status monotone** | `(dead, owes)`: KB → {0,1}² | `dead`: as the state. `owes`: a trigger, or an activator that compiles an obligation | holes, and whether two lines describe R (D4) |
| **monotone program** | the operator, on every KB | any site | the static check's verdict, route 1's licence |

### What *forbid* covers, and what it does not

*Forbid*, ruled 2026-11-02
([S1e2.4.1 § 5](../m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#5-the-ruling)),
refuses `(absent P)` ⟹ `(false)` or `(not …)` over a relation a commitment
can still extend. Its refusal is not built: S1e2.2.5 builds it, with F1–F5
open, in P1e2.2, the last phase of the milestone's order. It does not cover:

- **an `absent` that concludes a positive fact**: `derive-p` on `features/07`
  and `stale-naf-fork`, `soup-unless-allergic` on `lattice/08`. Every state
  difference above is of this kind;
- **the derived refutation**, the `via` class: `stale-naf-fork`'s
  `p-and-q-clash` reads a `p` derived under `absent`, and
  [`via_hole.ein`](../m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/via_hole.ein)
  is a dead hole through one. Unless F2 takes `via`;
- **a refutation compiled at a node** through an activator
  (`ACTIVATOR_REFUTES`), unless F3's refusal reads the plans a node compiles;
- **a refutation through an activator derived under an `absent`**
  (`ACTIVATOR_UNDER_ABSENT`, in `ein-infer/examples/shared/written.rs`): the
  plan the node compiles reads no `absent`, so even a check of the plans a
  node compiles misses it, unless F2's `via` follows a plan's activator. (E)
  refuses its `act`. Measured by P1e2.1b's
  [taint census § 7](../m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md#7-the-refutation-a-node-compiles-for-f3);
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
- **What it costs.** [`site_census.md` § 2](../m1e2_correctness/p1e2.1b_non_monotone_programs/site_census.md#2-the-sites)
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
  [S1e2.4.1 § 1](../m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#1-the-exposed-set-re-counted)
  (`6e7dfc49`). Under F1 (b) and F2 with `via`, (E) refuses 8 programs
  *forbid* does not. Under F1 (c), fact grain, `features/05` and `/12` load
  under both. What each would be rewritten as, and whether the rewrite keeps
  R, is S1e2.1b.7's T1e2.1b.7.5.
- **(E) is an option, not a ruling.**

## The decisions

Each one: the question, its options, its evidence, and what holds until it is
taken. **What holds is documented and warned** (P1e2.1b's S1e2.1b.5), and no
answer moves until a decision is taken.

### D1 — (E): *forbid* extended to every `absent` site

[§ Option (E)](#option-e-and-its-13-programs) above. **Until decided:**
*forbid* as ruled, and the 13 programs load.

### D2 — Q-M1e2.2: what a node's state is

[Q-M1e2.2](../m1e2_correctness/open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in),
asked of every node. Moot under D1's (E).

| | a node's state | measured consequence |
|---|---|---|
| **(A) fork** | `sat(sat(P) ∪ C)`, today's | nothing moves. W8's rows close `not-wrong` with the reason. Not stable under rule order on `features/07`, `f07-fpoff` |
| **(B) scratch** | `sat(P ∪ C)` | `stale-naf-fork` and its twin gain `{(q one)}`; `features/07`, `f07-fpoff` and the probe `not_conclusion` lose a stale fact from one state. Saturation ×0.2–5.6 a node, root re-derived at every node. L0 stops being the node's construction. Exposed to the FIFO race on a stratified program (`examples/lattice/10_fifo_gap.ein`) **and on one the check calls monotone**, measured since S1e2.1b.7: [`11_fifo_gap_static.ein`](../../examples/lattice/11_fifo_gap_static.ein), whose node `{(b one)}` holds a stale fact from scratch and not by fork. Admission in stratum order, emulated, removes both ([`node_census.md` § 12](../m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md#12-the-cumulativity-census-and-the-fifo-gap--s1e21b7)) |
| **(C) guards are premises** | (A) or (B), and a state holding a stale fact is not a solution | over (A): `features/07`, `f07-fpoff` lose `{(q one)}`; over (B): nothing on the corpus. A third node outcome, its own status or dead |
| **(D) refuse a cycle through an odd `absent`** | a load error | over D: `features/07`, `f07-fpoff`, `d3-unsat-core` |
| **(F) the stable endpoints of every path** | the set of states the path tree reaches at C whose facts are not stale ([§ The traversal](#the-traversal-a-tree-of-paths-and-what-it-may-be-memoized-by)); default logic's extensions | not measured. Per C: 0 on an odd cycle, 1 if stratified, several on an even cycle. The answer becomes a set of (C, state) pairs, so k_s counts states and not nodes. The cost is the state DAG, not the cube |

Sub-questions: **(D)'s scope**, over D (3 programs) or over all relations (6,
three syntax fixtures with n = 0 added); and **stratum-order admission**,
which closes the FIFO gap. Evidence:
[`node_census.md`](../m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md)
§ 4, § 5, § 7, § 9 and § 10, which also carries the agent's recommendation,
not the user's: (B) with (D), admission in stratum order. The register's W8
`right` blocks are (B)'s answer (§ 9 there). **Until decided:** (A).

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

Evidence: § 6.1 there, and S1e2.1b.7's path and cumulativity censuses.
**Until decided:** (ii). P1e2.1c's S1e2.1c.2 and .4 are scoped to monotone
programs.

### D4 — what is reported when R has holes: Q-M1e2.7 (d)

[Q-M1e2.7 (d)](../m1e2_correctness/open_questions.md#sub-questions-for-p1e21as-first-stage):
(a) R, its lines named as R's extremes, every minimal hole with its cause and
repair edges; (b) R as convex pieces; (c) a qualifier; (d) a refusal. With
(d1) dead and owing holes, one word or two, (d2) a claim form, (d3) the stub's
words, (d4) `lattice/04`'s register row. Evidence:
[S1e2.1b.3](../m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.3_holes.md)'s
[`hole_census.md`](../m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md)
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
(§ 10 there).
**Until decided:** the listing is R, and the `lattice` row names up to four
minimal holes with why each is out
([`read_out.md` § 6.3](../m1e2_correctness/p1e2.1a_the_solution_region/read_out.md#63-holes-the-stub-never-silent)).

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
[F18](f18_world_aware_negatives.md)'s narrow form; (c) the refusal first, a
reorder of the milestone. Evidence:
[S1e2.1b.4](../m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.4_refutation_resting_on_absent.md)'s
[taint census](../m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md)
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

**Until decided:** (a). F18 closes without being done once the refusal covers
every refutation a route can learn from: D6's (c), or D1 with F2's `via` and
F3. Since S1e2.1b.4, `via` there has to follow a plan's activator, or D1's (E)
has to be taken, for `ACTIVATOR_UNDER_ABSENT` to be covered.

### D7 — the non-monotone search past the oracle's limit

Was M1e2's [S1e2.1c.3](../m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.3_the_non_monotone_search.md),
moved here 2026-11-02; its file keeps the design notes. (a) the guard split;
(b) the cube past the limit when asked by name, `--region-limit`; (c) partial,
C3. Evidence: [`monotonicity.md` § 6.4](../../docs/kernel/inference/monotonicity.md#64-the-guard-split-prototyped):
the split agrees with the oracle on 19 of 21 cubes and disagrees on W8's two,
because a cell builds its node from scratch; g ≪ n on 2 of the 8 flagged
programs past the limit, and g = n = 19 on `lattice/06_friends_tables`. The
split presumes D2. **Until decided:** (c), and (b) when asked.

## Why it is parked

- The user's direction, above.
- Each option moves answers or refuses programs that are named, and the
  measurements are in. What none of them has is a program that needs the
  decision: within the oracle's limit, every flagged cube with a violation was
  written to show one ([`monotonicity.md` § 2](../../docs/kernel/inference/monotonicity.md#2-the-ladder)).
  `lattice/06`'s conditional obligation is the exception the user wrote, and
  it is past the limit.
- Until a decision, what holds is documented and every surface that reports R
  says so.

## Read before deciding

- [`monotonicity.md`](../../docs/kernel/inference/monotonicity.md): the two
  kinds of program compared; § 7, this analysis with every claim labelled by
  its standing; § 8, what is open.
- [`node_census.md`](../m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md)
  (D2, D3), [`site_census.md`](../m1e2_correctness/p1e2.1b_non_monotone_programs/site_census.md)
  (D1, D7), S1e2.1b.3's `hole_census.md` (D4), S1e2.1b.4's taint census (D6),
  S1e2.1b.7's consequences, path and cumulativity censuses (all).
- [S1e2.4.1](../m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md):
  *forbid*, its F1–F5, the `via` set and the rewrites that keep R.
- [`absent_semantics.md`](../../docs/kernel/inference/absent_semantics.md):
  *"Retraction / truth maintenance: not provided"*, which D2's (B) and (C)
  would qualify.
- [F18](f18_world_aware_negatives.md), whose closure rode on *forbid* (D6).
- Prior art: stable models (Gelfond & Lifschitz, ICLP 1988), the guard split's
  move; stratified programs and their perfect model (Apt, Blair & Walker,
  1988), D2's (D) and stratum order; the well-founded semantics (Van Gelder,
  Ross & Schlipf, *J. ACM* 1991), a third reading none of the options names;
  truth maintenance (Doyle, *Artificial Intelligence* 1979), what (B) and (C)
  approximate.
