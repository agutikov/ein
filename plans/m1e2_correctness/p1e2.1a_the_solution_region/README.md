# P1e2.1a — The solution region: R defined, computed, drawn and reported

**Estimate:** ~5 weeks: 10 stages, 23 days. It was 15.5 days when written.
S1e2.1a.1 grew by 1.5 on 2026-10-15, and S1e2.1a.8 (2 days) was added the
same day at the user's request. S1e2.1a.9 (4 days) was added on 2026-10-16,
when Q-M1e2.8 was ruled. S1e2.1a.10 (1.5 days) was split out of S1e2.1a.6
and S1e2.1a.7 the same day, and they shrank by what it took. S1e2.1a.4 is
re-estimated after its grammar diff.
**Status:** in progress. S1e2.1a.1, S1e2.1a.2 and S1e2.1a.8 were done
2026-10-15. On 2026-10-16, ahead of S1e2.1a.3's census, the user ruled every
sub-question but (d) for the monotone case, and the shape of `solve`'s default
([§ The rulings of 2026-10-16](#the-rulings-of-2026-10-16)). S1e2.1a.3 was
done the same day: the census ([`region_census.md`](region_census.md))
contradicts no ruling, and
[`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)
now defines R. The terms are in [`definitions.md`](definitions.md), and on
that page since S1e2.1a.3.
S1e2.1a.9 was done the same day, apart from the switch, which is S1e2.1a.5's.
Route 1 is in the library, held to the oracle node for node, and measured
([`route1_reach.md`](route1_reach.md)): it describes R on 162 of 195 programs
and counts zebra's (one state, 261 982 commitments). The interface
([`solve_interface.md`](solve_interface.md)) was approved, and its additive
half is built: the region modes and `ein oracle`, every mode held to the
oracle. **The rest of the phase was re-ordered the same day**
([§ Stages](#stages)), because the switch needs two pieces of work the plan
had after it.
S1e2.1a.4's grammar is ruled, its real examples
([`region_examples.md`](region_examples.md)) accepted, and the build done:
`(solve …)` with the lattice keywords moved off `(query …)`, and the region
claim checked by `ein test` against R. The query's new `:expect` lands with the
switch. S1e2.1a.10 is done: the census
([`walk_census.md`](walk_census.md)), and the register under R.
**Depends on:** [P1e2.1](../p1e2.1_seeing_the_answer/README.md) S1e2.1.1–6,
which built the register, the doors and the bundle this phase extends.
**Blocks:** the rest of the review,
[S1e2.1.7](../p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md), which paused
after W1, and therefore every stage of
[P1e2.2](../p1e2.2_wrong_answers/README.md).
**Source:** the user's instruction of 2026-10-15, after reading W1: *"we need
to modify ein language, ein.rs and update docs to fully cover R in all cases
with holes and without, meaning of "solution", etc. And only after that we
can revisit the review; Also add search lattice rendering tooling is the new
phase, while it's visulaization become so important for solution correctness
analysis."* The reading this phase implements is
[Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice).

---

## Why this comes before the review

The review's first case changed the definition every other case is judged
against. W1 asked whether `{(p A)}` is a solution of `branching/15`. The
user's answer was that the question had the wrong shape. The answer to a
program is a **region** of its hypothesis lattice, *R = the nodes that are
alive and discharged*. R lies between a **minimal line**, which the
obligations set, and a **maximal line**, which the restrictions set, and it
may have **holes** where `absent` breaks monotonicity. Under
[Q-M1e.6](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
a solution was one kind of node. Under Q-M1e2.7 the answer is R's shape,
and `{none, one, several}` becomes *R is empty*, *R has one node on the line
that matters*, *R has several*.

Twelve cases are still unread, and five of them are contested readings
(W3, W5, W6, W8, W12). Each one would be argued against a definition that is
about to change, so the review waits.

The second reason is the user's: **the lattice has to be seen.** Every W1
row was decided by looking at a handful of nodes, and nothing in the bundle
draws them. [`ein render lattice`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md)
shows survivors and their pruned siblings only, and its `--view full` falls
back to that. The brute-force script behind Q-M1e2.7's table,
`probes/region.py` (now [`utils/region.py`](../../../utils/region.py)), listed the whole lattice for seven
programs in a few seconds. It settled W1 by doing so, and it found the first
measured hole, on W3's witness.

**The phase is about the monotone lattice** (the user's answer of
2026-10-15, recorded in [S1e2.1a.1](s1e2.1a.1_the_oracle.md#scope-after-the-users-answer-of-2026-10-15)).
A program where adding a fact can take a conclusion away has a lattice the
two lines do not describe. Such a program is **detected** here, by a static
check over its compiled plans, and its answer is reported as *assuming a
monotone lattice*. What its R is belongs to
[P1e2.1b](../p1e2.1b_the_non_monotone_lattice/README.md). That includes
holes, Q-M1e2.7's (d), and whether a node is built from root's fixpoint or
from scratch.

## The rulings of 2026-10-16

[Q-M1e2.7 § The rulings, and the questions back](../open_questions.md#the-rulings-and-the-questions-back-2026-10-16)
has them verbatim. What they change here:

- **The general case comes first.** R is, in general, its node set. The two
  lines are a simplification that convexity licenses, and monotonicity
  guarantees convexity. The phase still finishes the monotone case and leaves
  the non-monotone one to P1e2.1b, but the definition page starts from the
  general case
  ([S1e2.1b.6](../p1e2.1b_the_non_monotone_lattice/s1e2.1b.6_the_two_lattices_compared.md)'s
  ladder).
- **`k` is replaced by k_s and k_c** (a), (e): \|R\| in states and in
  commitments, both reported. The walk cannot count R, so
  [S1e2.1a.9](s1e2.1a.9_solve_describes_the_region.md) makes `solve`'s
  **default an exhaustive, certifying, full description of R**, with flags
  that make the task simpler: any solution, a single one expected, maximal
  only, minimal only (Q-M1e2.8, ruled). Beyond what it reaches is
  [P1e2.1c](../p1e2.1c_the_region_search/README.md).
- **Solving and asking come apart** (b), (c): `(solve :hrules …)`,
  `(solution …)` for the expected region, and `(query … :expect …)` over the
  goal's projection. S1e2.1a.4 grows, and is re-estimated after its grammar
  diff.
- **The verdict words judge a stated expectation** (i). Without one, `ein
  solve` explores and describes R.
- **(f), (g), (h) hold for the monotone case**, which is this phase's. ∅ is
  a solution exactly when root is alive and discharged, whatever the
  generator proposes.
- **W11 is not this phase's question.** It is revisited twice after the
  refactoring: in S1e2.1.7's review, then in S1e2.2.12.
- **Ruled 2026-10-16:** all four of [`solve_flags.md` § 9](solve_flags.md#9-what-needs-the-users-ruling)'s
  questions.
  - The flag names are as proposed.
  - The default switches once, at S1e2.1a.5.
  - Route 1's budget is 2¹⁹ facts beyond root (A), after
    [`walk_and_region.md`](walk_and_region.md) showed why the walk and R
    differ. The state route (B) is P1e2.1c's first step.
  - **`ein walk` was withdrawn**, in the message that answered question 3.
    There is one tool for R, `ein solve`, and it does the least work each
    question needs. `ein oracle` computes the cube, only when asked. Every
    answer is consistent with the oracle's, whether or not the program is
    monotone. So R's node excludes the search's writes, which was
    `walk_and_region.md` § 7.
- **Ruled 2026-10-16: [`solve_interface.md`](solve_interface.md), all of
  it.** It has the reference, the question → command guide, and each mode's
  route and cost. The user's reason, verbatim: *"walk main principle was
  mining and reusing all possible information in favor of speed. Now we put
  the correctness and consistency as an absolute requirement and switch from
  searching the solution to exploration of the region in general"*.
- **Still open:** whether `(region …)` with `(merged …)`, the third answer's
  sketch of how a state's class is shown, is one form for both directions,
  printed by `solve` and stated by a program. That is S1e2.1a.4's grammar
  diff. And whether root is never written by the search, which the plan
  proposed and S1e2.1a.9 assumes.

## What exists, and what does not

Taken 2026-10-15 against `caef67a`.

| want | what exists | the gap |
|---|---|---|
| R, defined | [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md) § 2: a solution is a saturated, consistent state that owes nothing **or** is maximal (Q-M1e.6) | no region, no minimal line, no holes; `alive₀` is defined after the forced-positive cascade, so the universe depends on config |
| R, computed | nothing. The engine's walk is the only computation, and it is what is being checked | an **oracle**: every node of the lattice classified by saturation alone |
| R, drawn | `ein render lattice` (survivors + pruned siblings), `--dump-lattice` (one page per entering), the bundle's `pages/lattice/` | the **whole lattice**, every node's status and the reason for it, R's lines and holes, with the engine's walk overlaid |
| R, stated by a program | `:expect (model …)`, `(or (model …) …)`, `(false)`: one model set | no way to say *which line*, or to state a region |
| R, reported | `verdict.k`, `solutions`, `open_states`, `exhausted` | no line named, no hole reported, and `k` is a count of recorded nodes, not of a line |
| R, checked | [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml): hand-derived right answers for 26 witnesses | no computed right answer anywhere, and nothing checks a lever against the answer without it |

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S1e2.1a.1](s1e2.1a.1_the_oracle.md) | The oracle, and the monotone lattice | 4 d | `ein_infer::monotone`: the static check that classifies every read as static, dynamic-monotone or dynamic-non-monotone, and the `non_monotone` stub in `solve`, `ein test` and the dump; `ein_infer::region`: every commitment over the root candidate list, held as an n-bit mask with two independent bits (dead; owing, read on dead states too) by saturation alone, with no search-time write; R's lines, components, maximal sub-cubes, holes with their repair edges, the minimal dead sets N and discharging sets E, and the N/E duality cross-check; the n-limit as ruled; reached by `ein solve --dump-region DIR` |
| [S1e2.1a.2](s1e2.1a.2_the_lattice_drawn.md) | The lattice drawn | 2.5 d | `ein render region DIR`: the cube's Hasse diagram, L0 at the top, every node coloured by status with its reason one hover away, every edge classed by the statuses it joins, components coloured, sub-cubes listed, holes with their repair edges; this run's walk overlaid (entered, pre-killed, pruned, never generated, recorded) and every learned nogood checked against the cube; views that stay readable as n grows. **Done 2026-10-15**, with saturation drawn as a move inside the pure cube (`C ⇒ cl(C)`, the user's input of the same day) |
| [S1e2.1a.8](s1e2.1a.8_region_to_program.md) | From a region to a program | 2 d | the opposite task, the user's of 2026-10-15: `utils/region_synth.py` writes a program for a chosen region (prime implicants → `(false)` rules with `absent`, obligations with witnesses), held to it node for node by the oracle and the static check by its monotonicity; the user's three-component shape and four more banked in `examples/lattice/synth/`; the solution as files; the region gallery (`review_bundle.py --regions`). **Done 2026-10-15** |
| [S1e2.1a.3](s1e2.1a.3_the_definition.md) | The region census, and the definition | 2.5 d | R measured over every corpus entry the oracle reaches; Q-M1e2.7's sub-questions (a)–(i) ruled by the user with the census on the table; `solution_semantics.md` rewritten around R; the config principle in `configuration.md`. **Done 2026-10-16**: `utils/region_census.py`, 159 regions, every ruling met by the census, and one flag that changes R (`enable-symmetric-mirror`) |
| [S1e2.1a.4](s1e2.1a.4_the_language.md) | The language states a region | 2.5 d, to be re-estimated | the forms (b) and (c) ruled 2026-10-16: `(solve :hrules …)` split from `(query …)`, `(solution …)` for the expected region, `(query … :expect …)` over the goal's projection; `ein test` and `--json-report` check them; the corpus migration named first; the grammar page says so. **Done 2026-10-16** apart from what moves at the switch: `(solve …)` and its region claim, checked against R; the query's projection `:expect` and the claims' migration are S1e2.1a.5's |
| [S1e2.1a.9](s1e2.1a.9_solve_describes_the_region.md) | `ein solve` describes the region | 4 d | the user's default of 2026-10-16: no flag means R in full, certified. Route 1 is a monotone program's alive down-set, held to the oracle node for node; route 2 is the oracle at n ≤ the limit; past both, a partial description with bounds and no walk. The flags *any*, *single*, *minimal* and *maximal* on two axes. **Done 2026-10-16** apart from the switch: route 1 (`ein_infer::downset`) and its reach; the flags ruled and the interface ([`solve_interface.md`](solve_interface.md)) approved, with no `ein walk`; T1e2.1a.9.4's additive half, which is `ein_infer::explore`, the region flags, `ein oracle`, and every mode held to the oracle. The retirements and the golden move are S1e2.1a.5's |
| [S1e2.1a.10](s1e2.1a.10_the_register_under_r.md) | The register under R, before the switch | 1.5 d | the walk's last census, every declared run and lever cell against R, while the walk can still be run; every walk wrong answer with no row filed: the census of 2026-10-16 ([`walk_census.md`](walk_census.md)) found 8 programs with no row, two of them new, and 17 twins; every `right` block restated under R, computed by the oracle or route 1, with no hand derivation left, and held by a test; W9 and W11 reversed; W1 re-read. Taken from S1e2.1a.6 and S1e2.1a.7 on 2026-10-16. **Done 2026-10-16**: 34 rows, each with its witness's `[case.region]` held to `explore` by `ein-infer/tests/register_region.rs`; 25 right answers are R's maximal line, two restated (W11/d3, W12), W1's confirmation stands |
| [S1e2.1a.5](s1e2.1a.5_the_read_out.md) | The read-out speaks R, and the default switches | 2 d | k_s and k_c replace `k` (a), (e); the default read-out describes R, and the verdict words judge a stated expectation (i), from a flag or from the program; every surface says what it counts; empty R split into its two kinds; holes through the stub. **The switch**, in one commit: bare `ein solve` describes R, the nine options and nine config fields retire, the last path to the walk goes, `ein oracle` gets its other five options, the corpus, the summaries and the 46 register runs move, and the 15 walk rows and S1e2.1a.10's flip to `fixed`. Every golden named before it moves |
| [S1e2.1a.6](s1e2.1a.6_engine_against_oracle.md) | The engine against the oracle, under every lever | 1 d | the modes' answers held to R under every lever cell the corpus declares (C4), now that the walk's levers are the modes'; the `enable-symmetric-mirror` row. *The engine equals the oracle* at default levers is already `explore.rs`'s test, and the walk's census is S1e2.1a.10's |
| [S1e2.1a.7](s1e2.1a.7_the_bundle_shows_r.md) | The bundle shows R, and the review resumes | 1 d | a § *The region* on every case page and in `--corpus` mode; S1e2.1.7 resumes on the twelve unread cases |

**Work order: .1, .2, .8, .3, .9 (done), then .4 and .10 in parallel, .5, .6,
.7.** Re-ordered 2026-10-16 after the interface was approved. The switch is
S1e2.1a.5's, and it removes the last CLI path to the walk. That commit needs
two things the plan had filed later:

- **the register's right answers restated under R**, which were S1e2.1a.7's.
  After the switch no run reproduces the walk's wrong answers, so its 15 rows
  must flip to `fixed` in that commit, and a row flips only against a `right`
  block restated under R;
- **the walk's wrong answers with no row filed**, which was S1e2.1a.6's. Six
  are known: `zebra2-hints`, `branching/07` and four flagged programs. After
  the switch nothing reproduces them.

S1e2.1a.10 is both, taken out of the two stages before the switch. S1e2.1a.9
comes before .5 because .5 prints what .9 computes. S1e2.1a.4 comes before .5
because the region form .5 prints is the one .4 reads back, and the round-trip
test needs both.
**Before that the order was .9 and .4 in parallel, then .5, .6, .7. Before
2026-10-16 it was id order, with one overlap, and S1e2.1a.8 after .2.** It was
added on 2026-10-15 and done the same day, after S1e2.1a.2's drawings: its
programs are the cases S1e2.1a.3 rules on and S1e2.1a.6 checks.
**Otherwise:** The oracle and the drawing come
first, because they show R before anyone rules on it, which is this
milestone's *see first* order. The oracle's classification (dead, owing,
discharged) depends on none of Q-M1e2.7's sub-questions. S1e2.1a.3 needs
S1e2.1a.1 for its census. S1e2.1a.4 and S1e2.1a.5 need the rulings.
S1e2.1a.6 needs S1e2.1a.5, because it compares what the engine *reports*.
S1e2.1a.7 needs all of them.

## Where this phase stops and P1e2.2 starts

This phase **defines, computes, draws and reports** R. Since 2026-10-16 it also
makes `solve`'s default compute R wherever that is cheap and exact
(S1e2.1a.9). It does not fix the walk. Taking the default further is
[P1e2.1c](../p1e2.1c_the_region_search/README.md)'s work, and the wrong
answers that survive are [P1e2.2](../p1e2.2_wrong_answers/README.md)'s, one
case at a time. After S1e2.1a.5 no CLI path reaches the walk. The register's
rows whose wrong answer was the walk's flip to `fixed` in the switch commit,
against the `right` blocks S1e2.1a.10 restated. What that leaves of P1e2.2's
stages, W1's [S1e2.2.1](../p1e2.2_wrong_answers/s1e2.2.1_maximal_not_recorded.md)
among them, is the re-scoping P1e2.1c's last stage does.

## Acceptance

- **R is defined for a monotone program**, in `solution_semantics.md`, and
  every verdict word, `k` and `exhausted` is defined in R's terms. Each of
  Q-M1e2.7's sub-questions except (d) carries the user's ruling, in the
  question and on the page. (d) is P1e2.1b's.
- **A non-monotone program is detected, and says so.** The static check names
  its sites, the cube counts the violations it observes, and `solve`, `ein
  test` and `--dump-region` report it through one stub P1e2.1b replaces.
- **The oracle classifies by saturation alone.** A test holds that no
  search-time root write reaches it. The witness is Q-M1e2.7's: the commitment
  `{(seats Ann C1)}` on `owed-seats` with `functional` and `injective` owes 2,
  where `solve -m 0` says 0. The limit is as ruled: n ≤ 16 silent; 17–19
  refused unless asked for, then one warning line; ≥ 20 refused unless asked
  for, then a prominent warning stating 2ⁿ and the projected cost.
- **The oracle's answers on the seven programs in Q-M1e2.7's table equal the
  table**, which was taken by a different route
  ([`utils/region.py`](../../../utils/region.py): `ein saturate` per subset, one
  process each). Two implementations agreeing is the evidence that the
  library oracle is right. One of the seven has a hole.
- **Every case page draws its lattice**, every node with its status and why
  (the core, or the owed instances), R's lines and holes, and the run's walk
  overlaid. **Missing** (on R's reported line, not recorded) and **spurious**
  (recorded, not on that line) are named.
- **No config option changes R.** `configuration.md` says so, and S1e2.1a.6's
  test checks every lever over every entry the oracle reaches. A lever that
  changes the answer is a register row, not a *weaker mode*.
- **A program can state a region**, and `ein test` checks the statement.
- `./run_tests.sh` green at every stage boundary. Every golden a stage moves
  is named in its file before it moves.

## Risks

- **Language design without end.** The milestone's own risk list said a
  ruling that needs a new language feature becomes a followup. The user
  overrode that for R on 2026-10-15, and moved the line again on 2026-10-16:
  (b) and (c) split solving from asking. The line is now **those forms**:
  - `(solve …)`;
  - `(solution …)`;
  - the query's projection `:expect`;
  - a query's choice of subset.

  Anything else is a followup.
- **2ⁿ.** The oracle reaches small programs only. The zebra family has
  candidate lists in the thousands, so its region stays a definition there, not a
  measurement. S1e2.1a.3's census says how much of the corpus the oracle
  reaches, and the phase claims nothing about the rest.
- **The oracle shares a saturator with the engine it checks.** If saturation
  is wrong (W7 loses a derivation), the oracle inherits the error. The
  oracle checks the **walk**, not the saturator, and the pages say so. The
  scratch prototype's per-subset `ein saturate` route is a second path for
  small n, and S1e2.1a.1 compares the two.
- **Holes.** Without holes, two lines describe R. With holes they do not, and a
  report that silently assumes convexity would be a new wrong answer. The
  oracle never assumes it: it enumerates, and does not prune by closure. A
  hole needs no `absent`: an obligation whose trigger a commitment can change
  makes one too
  ([P1e2.1b](../p1e2.1b_the_non_monotone_lattice/README.md#what-non-monotone-means-and-what-is-not)).
  What a hole means is P1e2.1b's. This phase finds them and labels what it
  reports around them.
- **The trivial line.** In an obligations-only program the maximal line is
  everything, which is cheap for the oracle at n ≤ 16 and exponential for the
  search. Ruling (f) decides whether it is reported, and the search must not
  chase it either way.
- **The read-out change moves many goldens.** Every `Solution` and
  `Ambiguity` summary in the corpus could move at S1e2.1a.5. They are
  named first, and an unexplained move is a stop.

## Connections

- [Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)
  is the reading. Its sub-questions are S1e2.1a.3's.
- [Q-M1e.6](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
  becomes a special case, and
  [Q-M1e2.1](../open_questions.md#q-m1e21--is-a-discharged-superset-of-a-discharged-state-a-new-model)
  becomes sub-question (a).
- [Q-M1e.8](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set):
  `exhausted` certifies the walk. Sub-question (g) makes it certify the lines.
- [Q-M1e.9](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)
  and [Q-M1e2.2](../open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in)
  are two of the ways R gets holes, and an obligation trigger is the third.
  All three are [P1e2.1b](../p1e2.1b_the_non_monotone_lattice/README.md)'s.
- [`lattice_dump.md`](../../../docs/kernel/inference/lattice_dump.md) and
  [`04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md)
  are the pages the drawing extends.
- [M1f S1f.10.3](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.3_the_restricted_join.md)
  restricts the join and claims *not one model moved*. The oracle is what
  can check that claim on small programs.
- [M20](../../m20_gui/README.md): a GUI that draws the lattice binds
  `ein_infer::region`, not the DOT.
