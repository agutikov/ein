# P1e2.1b — Non-monotone programs: detected, documented, warned

**Estimate:** ~4 weeks: 7 stages, **21 days**, 12.5 of them still to do.
Re-planned 2026-11-02 on the user's direction below, from 6 stages and 14
days: [S1e2.1b.5](s1e2.1b.5_the_detector.md) became the detector (3 → 5.5
days), [S1e2.1b.7](s1e2.1b.7_the_consequences_written_down.md) is new (4.5
days), and S1e2.1b.2–.4 end at their evidence, the days they had for a ruling
spent on evidence instead. Before that the phase was 15.5 days when its stages
were specified (2026-11-02), and 14 after
[S1e2.1b.1's census](site_census.md#9-the-re-estimate) re-estimated them the
same day.
**Status:** created 2026-10-15 on the user's instruction, given while
answering S1e2.1a.1's open questions:

> I propose to move non-monotonic solution question into phase p1e2.1b; in
> this phase we have to 1) complete handling of monotonic lattice 2)
> understand if any `absent` cause non-monotonicity or only some specific
> cases, clossify them OR move this investigation to p1e2.1b; 3) implement
> detection of non-monotonicity and add a stub for handling this case in
> `solve` and other tools

"This phase" in that quote is [P1e2.1a](../p1e2.1a_the_solution_region/README.md).
**Re-planned 2026-11-02**: the phase no longer decides what R is on a
non-monotone program. It detects, documents, warns and prepares
([§ The goal](#the-goal-since-2026-11-02)), and every decision it was to take
is [F21](../../followups/f21_non_monotone_programs_decided.md)'s.
**Done:** [S1e2.1b.1](s1e2.1b.1_the_site_census.md) (the site census; the
grain ruled *relation, as shipped*), [S1e2.1b.6](s1e2.1b.6_monotone_and_non_monotone_programs_compared.md)
([`monotonicity.md`](../../../docs/kernel/inference/monotonicity.md), one
fixture per cause, the guard split held to the oracle) and
[S1e2.1b.2](s1e2.1b.2_fork_or_scratch.md), at its evidence
([`node_census.md`](node_census.md)), with Q-M1e2.2 deferred to F21.
[S1e2.1b.3](s1e2.1b.3_holes.md) is done at its evidence
([`hole_census.md`](hole_census.md)), with Q-M1e2.7 (d) deferred to F21's D4.
**Next:** [S1e2.1b.4](s1e2.1b.4_refutation_resting_on_absent.md).
**Depends on:** S1e2.1a.1, whose detector gives this phase its census and
whose stub [S1e2.1b.5](s1e2.1b.5_the_detector.md) replaces with warnings.
Since 2026-11-02 also [P1e2.4](../p1e2.4_what_the_switch_left/README.md),
done: its S1e2.4.1 ruled *forbid*.
**Blocks:** [P1e2.1c](../p1e2.1c_the_region_search/README.md), whose
S1e2.1c.2 and .4 are scoped to monotone programs by this phase's deferral;
[S1e2.2.5](../p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md) (W3), which
takes [S1e2.1b.4](s1e2.1b.4_refutation_resting_on_absent.md)'s evidence for its
F1–F3; [S1e2.2.6](../p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md) (W8),
which takes S1e2.1b.2's evidence and the deferral; and the revisit
([S1e2.1.7](../p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)) of every
register row whose program the static check flags, which reads the detector's
warnings.

---

## The goal, since 2026-11-02

The user, verbatim:

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

**What the phase does:**

- **Collects the evidence** for every decision, measured on the real
  programs: the sites (S1e2.1b.1), a node's state by fork and from scratch
  (S1e2.1b.2), holes by cause (S1e2.1b.3), refutations that rest on an
  `absent` (S1e2.1b.4).
- **Writes it down**: [`monotonicity.md`](../../../docs/kernel/inference/monotonicity.md)
  compares the two kinds of program (S1e2.1b.6). Its § 7, written with this
  re-plan at the user's *"Ok, just add all of this into followup and docs -
  will return to it later"*, holds the analysis of 2026-11-02 with every
  claim labelled by its standing, and § 8 what is open. S1e2.1b.7 measures
  what § 7 left unverified, and adds a taxonomy of the causes and, per option,
  what each would move on which program. Every open cell names F21.
- **Gives an explicit check**, *is this program monotone?*, in a CLI form the
  user approves on real corpus programs before it is built, and a **cheap
  detector inside a bare `ein solve`**, with a warning on every surface that
  reports R (S1e2.1b.5).

**What it does not do:**

- **It decides no non-monotone semantics.** What a node's state is
  (Q-M1e2.2, options (A)–(E)), whether a node's state must be independent of
  the path that built it, what is reported when R has holes (Q-M1e2.7 (d)),
  refuse, qualify or search differently, what a search may learn before
  *forbid*'s refusal ships, and the search past the oracle's limit are
  [F21](../../followups/f21_non_monotone_programs_decided.md)'s D1–D7.
- **No answer moves.** R on every program stays what the engine computes
  today: each node forked from root, route 2 within `--region-limit`, partial
  past it. The warnings say so.
- **Nothing is refused** that loads today. *Forbid*'s refusal is
  [S1e2.2.5](../p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md)'s.
- **The saturator is not changed.** Admission in stratum order is one of
  F21's options.

## The words

The user, 2026-11-02: *"Also maybe better to say not "non-monotonic lattice"
because lattice can't be non-monotonic; but non-monotonic function of lattice
node, or path-dependent function"*, and then: *"lattice is a domain of
non-binary truth (liveness, dischargeness) function, or 2 binary functions;
And while lattice is set-indexed dag - then path-independence ===
monotonicity; Correct me if I'm wrong"*. The reading this phase uses:

```
L ──state──▶ KB ──(dead, owes)──▶ {0,1}²
```

- **The lattice** L = (2^H, ⊆), the commitments over the candidate universe
  H, is the domain. It is a Boolean lattice, and it is neither monotone nor
  not.
- **The status** is two binary functions of the state, `dead` and `owes`:
  the oracle's two bits per node. R = alive ∧ discharged.
- **Two properties, one per arrow:**
  1. **The state is monotone**: C ⊆ C′ ⇒ `state(C)` ⊆ `state(C′)`. Given
     **cumulative** saturation, X ⊆ Y ⊆ sat(X) ⇒ sat(Y) = sat(X), this is
     the same as **path independence**: `state(C)` is a function of C alone,
     whatever built it (a fork from root, a parent's fixpoint, scratch).
     Path independence ⇒ monotone always holds, because construction is
     append-only: a path through C reaches a state ⊇ `state(C)`. The converse
     needs cumulativity. A monotone program has it. A stratified program
     evaluated stratum by stratum should have it, which is unverified and
     measured by S1e2.1b.7's cumulativity census. The property is broken by
     an odd-depth `absent` over D, and by the boundary's **FIFO admission** of
     equal-priority `absent` rules, which breaks cumulativity: r2 `a ∧ b ∧
     absent(q) → p` written before r1 `a ∧ absent(z) → q` gives {a, b, p, q},
     with `p` stale, when {a, b} is built at once, and {a, b, q} by way of
     {a}. That state is path-dependent while the state function is monotone
     (`node_census.md`'s written `stratified_stale`). This property is
     Q-M1e2.2's subject.
  2. **The status is monotone**: `dead` and *discharged* upward-closed.
     `dead` follows the state. `owes` does not: an obligation's trigger, or
     an activator that compiles an obligation, adds a debt over a monotone,
     path-independent state (`lattice/04`). **A hole needs no path
     dependence.** This property is S1e2.1b.3's subject.
- **A monotone program** is a third, stronger property, and keeps its
  logic-programming name: its consequence operator is monotone on every KB,
  so saturation is a closure operator and both node properties hold. It is
  what the static check approximates, and it is why the check has
  *conservative* flags: on those programs the node functions are monotone on
  the program's own lattice while the program is not.

The directory keeps its old name, `p1e2.1b_non_monotone_programs`, until a
mechanical rename. Verbatim quotes keep their words.

## Why a phase of its own

[Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)
describes the answer as a region R with two lines, and two lines describe R
only when R has no holes. A hole is a node outside R between two nodes inside
it, and it needs a commitment that takes a conclusion away or adds a debt: a
node function that is not monotone. Every argument P1e2.1a makes about R (the
lines are local tests on edges, the N/E duality, a learned nogood is sound)
rests on that not happening. So P1e2.1a finishes the monotone case and
detects the other one, and this phase was made to decide what the other one
means. Since 2026-11-02 it documents and warns, and the deciding is F21's.

The two questions S1e2.1a.1 could not settle without a ruling:

- **A node's state, fork or scratch.** The engine builds a node from root's
  fixpoint with the commitment written in. `utils/region.py` builds it from
  the program with the commitment written in as given facts. On a monotone
  program the two agree, which is the theorem below. On
  [`stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein)
  (W8) they do not: forked from root, `{(q one)}` is dead, because root
  already holds `(p one)`, which was derived under `(absent (q one))`. From
  scratch, `{(q one)}` is in R. This is
  [Q-M1e2.2](../open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in)
  asked of every node. The evidence is S1e2.1b.2's, and the decision is F21's
  D2.
- **What is reported when R has holes**, Q-M1e2.7's sub-question (d). The
  evidence is S1e2.1b.3's, and the decision is F21's D4.

**Fork and scratch are one case of a general one.** The user, 2026-11-02:
*"I guess this is a special case of more common sat(P+C1+C2) !=
sat(sat(P+C1)+C2) ? so in general non-monotonic is not only about root
scratch and fixpoint"*. Fork from root is C1 = ∅. Every shipped route builds
a node from root with all of C at once, which is why it was the only case
seen. A parent path reaches a different state on `lattice/08`, the probe
`via_hole` and `d3-unsat-core`, with nothing derived at root on `lattice/08`
([`node_census.md` § 6.1](node_census.md#61-the--y-abort-on-d3-unsat-core)).
So under today's fork a node's state on a flagged program can depend on the
path, and the question under Q-M1e2.2 is *is a node's state a function of C
alone?*, which given cumulativity is *is the state monotone?*
([S1e2.1b.2 § The general form](s1e2.1b.2_fork_or_scratch.md#the-general-form-a-path-not-only-root)).

## What non-monotone means, and what is not

Measured 2026-10-15, and the reason the phase is not *"`absent`"*. A hole needs
no `absent` in any saturation rule. On this eleven-line program
([`examples/lattice/04_obligation_trigger_hole.ein`](../../../examples/lattice/04_obligation_trigger_hole.ein),
banked as a corpus fixture by S1e2.1a.1):

```lisp
(import std.algebra :symbols (total-owed))
(relation is-a  T T)
(relation seats Guest Chair)
(relation meal  Diner Food)
(is-a Ann Guest) (is-a C1 Chair) (is-a Soup Food)
(rule seated-dines () :match (seats ?g ?c) :assert (is-a ?g Diner) :priority 200)
(total-owed meal is-a)
(hrule seat () :match (and (is-a ?g Guest) (is-a ?c Chair)) :assert (seats ?g ?c))
(hrule dine () :match (and (is-a ?g Guest) (is-a ?f Food))  :assert (meal ?g ?f))
(query :goal (meal ?g ?f))
```

`region.py … '(seats Ann C1)' '(meal Ann Soup)'` gives ∅ in R, `{(seats Ann
C1)}` **owing 1**, and both facts together in R. The owing node is a hole.
The two hrules make the generator's root list exactly those two facts. The
walk answered `Solution`, k = 1, `{(meal Ann Soup)}`, which is on neither
line. Since the switch (2026-10-18), `ein solve` lists the cube's three states
and names the hole: `minimal hole {(seats Ann C1)} — owes 1` (`ein solve
examples/lattice/04_obligation_trigger_hole.ein`, 2026-11-02, `7b33c3ab`).
Seating Ann derives `(is-a Ann Diner)`, and that is a debt `total-owed`
did not have at ∅. A rule that states an obligation is a negation of its
witness, so its **trigger** is non-monotone in the same sense an `absent` is.

So the classification S1e2.1a.1 implements is about where a read sits, and
`absent` is one of two places it can sit. Let *H* be the relations of the
candidate list, and *D* the relations a commitment can change: H, plus
everything a compiled plan asserts once it reads a relation already in D.
Every relation a plan reads is then one of:

| class | what it is | example | can it make a hole |
|---|---|---|---|
| **static** | not in D. Complete at root, the same at every node | zebra2's `adjacent-via-*` `absent`s over `right-of` / `next-to`; the stdlib's `is-a` scans when `is-a` is never hypothesised | no |
| **dynamic, monotone polarity** | in D, read where growth can only add conclusions or remove debts | `total`'s `forall … (not (R …))` (even depth); `total-owed`'s witness `absent` | no |
| **dynamic, non-monotone polarity** | in D, read where growth can remove a conclusion or add a debt | an `absent` at odd depth in a saturation rule (W3's `bad`, W8's `derive-p`); an obligation's trigger (the probe above) | **yes** |

**The theorem the detector rests on.** Suppose a program has no read of the
third kind. Then every relation in D grows with the commitment, static
relations do not move, `(false)` is upward-closed, and every debt shrinks as
the commitment grows. So R has no holes and is every node between its two
lines. That is a sufficient condition, not a necessary one. A third-kind read
can be harmless in practice, and the cube says whether it was: S1e2.1a.1
reports both the sites and the violations the cube observed.

**Which property each cause breaks** (2026-11-02, from the census pages):

- **The state** stops being monotone, or path-independent, only through an
  `absent`. One way is an odd-depth `absent` in a saturation rule over D
  (`SiteKind::Absent`): every corpus node where fork, scratch and a parent
  path differ traces to one ([`node_census.md`](node_census.md) § 2 and
  § 6.1), and an activator can hide one from a check of root's plans
  (`region.rs`'s `ACTIVATOR_REFUTES` compiles `(absent (?R ?x))` at a node).
  The other is the FIFO gap above, between equal-priority `absent` rules of a
  stratified program, which no corpus program shows (`node_census.md` § 4).
- **The status alone** stops being monotone through an obligation's trigger,
  or an activator that compiles an obligation, with no `absent`: holes on
  `lattice/04`, `lattice/09` and `synth/trigger_hole`. An obligation rule
  derives nothing, so every route builds the same state there.

## Stages

| ID | title | est. | status | ends with |
|---|---|---:|---|---|
| [S1e2.1b.1](s1e2.1b.1_the_site_census.md) | The site census | 2 d | **done** 2026-11-02 | [`site_census.md`](site_census.md): every site, every flagged cube's violations attributed, the fact-grain prototype held to the cube; the grain ruled *relation, as shipped* |
| [S1e2.1b.2](s1e2.1b.2_fork_or_scratch.md) | A node's state, fork or scratch | 2.5 d | **done at its evidence** 2026-11-02 | [`node_census.md`](node_census.md): both routes node for node, staleness, stratification, invariance, cost, W8 computed under each option; the general path form recorded; **Q-M1e2.2 deferred to F21** (D1–D3) |
| [S1e2.1b.3](s1e2.1b.3_holes.md) | Holes | 1.5 d | **done at its evidence** 2026-11-02 | `hole_census.md`: every hole by cause and kind, each attributed to a site (the gap S1e2.1b.6 found closed), the read-out today, and each reporting option drafted on real programs with its consequences. The decision is F21's D4 |
| [S1e2.1b.4](s1e2.1b.4_refutation_resting_on_absent.md) | A refutation that rests on an `absent` | 1 d | evidence only; not started | the taint census, its test banked, both readings of *extendable* per dead node and the compiled-refutation case handed to S1e2.2.5 (F1–F3). What may learn is F21's D6 |
| [S1e2.1b.5](s1e2.1b.5_the_detector.md) | The detector | 5.5 d | specified 2026-11-02 | an explicit check in the CLI form the user approved; a cheap detector in a bare `ein solve`, zero extra work on an unflagged program; a warning on every surface that reports R; held to the cube and to random programs |
| [S1e2.1b.6](s1e2.1b.6_monotone_and_non_monotone_programs_compared.md) | Monotone and non-monotone programs compared | 4 d | **done** 2026-11-02 | [`monotonicity.md`](../../../docs/kernel/inference/monotonicity.md), *current*; one fixture per cause; seven drawings; the guard split held to the oracle (19 of 21 cubes) |
| [S1e2.1b.7](s1e2.1b.7_the_consequences_written_down.md) | The consequences written down | 4.5 d | specified and re-scoped 2026-11-02 (the analysis and three pages' terms were written with the re-plan) | the path and cumulativity censuses and the FIFO-gap fixture, so that `monotonicity.md` § 7.8's unverified rows are measured; the taxonomy of causes; each option's consequences per program; (E)'s rewrites; `monotone.rs`'s module docs |

**Work order:** .1, .6, .2 and .3 are done. Next .4, the evidence. Then S1e2.1b.5's first task, the check's CLI form and the
warnings drafted on real programs, which goes to the user; .7 is written while
the user reads it. Then .5's build, last: its warnings cite .7's taxonomy, and
its tests make .7's claims executable.

**Questions the phase puts to the user:** one, the explicit check's CLI form
and the warnings' text, shown on real corpus programs with outputs before
anything is built (S1e2.1b.5's T1e2.1b.5.1, the user's rule of 2026-10-16 and
the [`solve_interface.md`](../p1e2.1a_the_solution_region/solve_interface.md)
precedent). No stage ends in a ruling.

## Acceptance

- **Nothing decided.** Every decision the phase carried is in
  [F21](../../followups/f21_non_monotone_programs_decided.md) with its options,
  its evidence and what holds until it is taken. Q-M1e2.2 and Q-M1e2.7 (d)
  are marked *deferred to F21* in [`open_questions.md`](../open_questions.md).
- **The evidence**, each page dated with the commands that re-take it:
  `site_census.md`, `node_census.md`, `hole_census.md`, S1e2.1b.4's taint
  census, and S1e2.1b.7's path and cumulativity censuses.
- **The docs.** `monotonicity.md` carries the two properties of the node
  functions (§ 7, written 2026-11-02), the taxonomy of causes and each
  option's consequences per program; its open cells read as open and name
  F21, and no claim in § 7 is left *unverified* without a stage that checks
  it. `monotonicity.md`, `solution_semantics.md` § 1.6 and `region_dump.md`
  § Monotonicity say which property they mean (done with the re-plan), and
  `monotone.rs`'s module docs too (S1e2.1b.7).
- **The explicit check** exists in the form the user approved. Its verdict and
  sites are the ones every other surface prints, held by a test.
- **The detector in `ein solve`**: static (sites by kind, cycles through an
  `absent` with their parity, equal-priority `absent` rules whose result
  depends on written order) and dynamic (the cube's violations; stale facts in
  the states the run lists). **Zero extra work on an unflagged program**,
  measured over the corpus. A warning on every surface that reports R, saying
  what is assumed and whether the program is exercised or conservative.
- **Robust**: a *Monotone* verdict implies no cube violation and the same
  state by fork, from scratch and by every split C1 ⊆ C, on every node of
  every corpus cube within the limit and of random programs.
- **No answer moves. Every answer exits 0.** `lever_invariance.rs` green;
  every golden the warnings move is named before it moves; `./run_tests.sh`
  green.

## What it inherits

| from | evidence here | decided |
|---|---|---|
| [Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice) (d), holes | [S1e2.1b.3](s1e2.1b.3_holes.md) | F21's D4 |
| [Q-M1e2.2](../open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in), W8 | [S1e2.1b.2](s1e2.1b.2_fork_or_scratch.md), done | F21's D1–D3 |
| [Q-M1e.9](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent), [Q-M1e2.5](../open_questions.md#q-m1e25--does-s1f108s-ruling-move-here), W3 | [S1e2.1b.4](s1e2.1b.4_refutation_resting_on_absent.md) | the language: ruled *forbid*, built by S1e2.2.5. What a route may learn: F21's D6 |
| `naf_deps.rs`'s `RefutationUnderAbsentWarning` (M1e S1e.2.3), the containment | [S1e2.1b.4](s1e2.1b.4_refutation_resting_on_absent.md): both readings of *extendable* per dead node | under *forbid* it becomes the refusal's text, S1e2.2.5's T1e2.2.5.3 |
| S1e2.1a.1's `non_monotone` stub | [S1e2.1b.5](s1e2.1b.5_the_detector.md): the stub's words become warnings that say what is assumed | refuse, qualify or search differently: F21's D5 |
| [S1e2.1a.3's census](../p1e2.1a_the_solution_region/region_census.md#5-holes-upward-closure-the-duality): five programs with a hole, of which `lattice/04_obligation_trigger_hole.ein` still has no register row | [S1e2.1b.3](s1e2.1b.3_holes.md), done: no row filed ([`hole_census.md` § 10](hole_census.md#10-the-consequences-per-option)) | the row follows from F21's D4: its (d4). Fork and scratch agree on `lattice/04`, so D2 does not move it |
| the same census's [§ 9](../p1e2.1a_the_solution_region/region_census.md#9-the-config-principle): `-y` aborted `corpus/fuzz_findings/d3-unsat-core.ein` before the switch | [S1e2.1b.2](s1e2.1b.2_fork_or_scratch.md) ([`node_census.md` § 6.1](node_census.md#61-the--y-abort-on-d3-unsat-core)) | path dependence: F21's D3. The lever `-y`: [S1e2.1c.5](../p1e2.1c_the_region_search/s1e2.1c.5_wrong_answers_re_scoped.md) |
| the census's [§ 8](../p1e2.1a_the_solution_region/region_census.md#8-monotonicity-the-static-check-against-the-cube): 13 programs flagged, a violation observed on 5 | [S1e2.1b.1](s1e2.1b.1_the_site_census.md), done | the grain: ruled, relation |

## Where it stops

P1e2.1a defines, computes, draws and reports R for **monotone** programs, and
detects the rest. This phase documents the rest, warns wherever R is
reported, and gives an explicit check. It decides what R is on none of them:
that is F21's. Neither phase makes a search find R past the oracle's limit on
a flagged program; that is F21's D7, moved there from P1e2.1c on 2026-11-02.
The wrong answers are [P1e2.2](../p1e2.2_wrong_answers/README.md)'s. W8's two
rows depend on F21's D2, and the tension that puts on the milestone's
acceptance is parked for the revisit in
[S1e2.2.6](../p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md#the-deferral-and-the-milestones-acceptance).
