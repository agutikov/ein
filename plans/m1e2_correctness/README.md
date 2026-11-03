# M1e2 — Correctness: every known wrong answer

**Estimate:** ~19 weeks: 7 estimated phases and 50 stages, four of them
dropped, **96.5 days** of stage estimates (P1e2.1 12, P1e2.1a 23, P1e2.1b 21,
P1e2.1c 16.5, P1e2.2 13.5, P1e2.3 8.5, P1e2.4 2). It was 95.5 until the user put
P1e2.3's N1–N4 in a stage, S1e2.3.7 (four lines that misname what they print,
1 day), on 2026-11-03. It was 93 until the user
added P1e2.3's S1e2.3.5 (test scratch left in `/tmp`, 1 day) and S1e2.3.6 (the
nightly workflow, 1.5 days) on 2026-11-03. It was 89.5 until S1e2.1c.5's
own estimate was re-taken on the user's two rulings of 2026-11-03, 2.5 days →
6: the walk's deletion around S4 and the levers' retirement were applied in
that stage. It was 103.5 until P1e2.1c's
[S1e2.1c.5](p1e2.1c_the_region_search/s1e2.1c.5_wrong_answers_re_scoped.md)
re-scoped P1e2.2 on 2026-11-03, from 27.5 days to 13.5: four of its stages
are dropped, because their rows were fixed by S1e2.1a.5's switch and their
fixes were to the walk, three shrank, one was restated and five were kept.
It was 101 until
the user re-planned P1e2.1b on
2026-11-02 to detect, document and warn, and to decide nothing: P1e2.1b went
from 14 days to 21 (the detector 5.5, a new stage of consequences 4.5), and
P1e2.1c from 17.5 to 13, its non-monotone search moved to
[F21](../followups/f21_non_monotone_programs_decided.md). P1e2.1b was 15.5
before S1e2.1b.1's census re-estimated S1e2.1b.2–.5 the same day, three of
the four downwards. P1e2.1b and P1e2.1c, added or grown 2026-10-16, had one stage
estimated between them until 2026-11-02, when the other nine were specified.
Where one of their stages ends in the user's ruling, its days are for the
evidence. P1e2.4, added
2026-11-01, is done, at its two stage files' 1 day each. P1e2.3, ruled a phase
2026-11-02, was specified the same day at the user's *"write P1e2.3 plans
also"*: four stages, 5 days, and six since 2026-11-03; its S1e2.3.1–.4 are
done (2026-11-03).
**Status:** created 2026-10-14 on the user's instruction: *"Create new
milestone for correctness issues. … first phase is for solution dumps and
rendering to markdown with graphs, so I can review all existing wrong
solutions and initial problems."* It comes out of
[M1e](../../docs/history/m1e_review_processing/README.md)'s close, whose probes found wrong
answers that the review never listed and that M1e's own rule filed rather than
fixed, and out of a survey taken the same day,
[`inventory.md`](inventory.md), which re-ran every known case and found
**five more**.
**Depends on:** nothing. Every case is stated against `71dd237`, and the
first phase reads before anything changes.
**Blocks:** [M1f](../m1f_hypothesis_and_documentation/README.md)'s S1f.10.3,
whose acceptance is *not one model moved* and which can only measure that
against a lattice that records the right models. And, by argument rather than
by link, [M2](../m2_nl_to_ir/README.md): the research plan's Stage A forbids
the kernel to let *`unknown` silently become `false`, `ambiguous`, or
`contradiction`*, and W1 and W2 are the engine saying *contradictory*, with
`exhausted = true`, about satisfiable programs.

---

## What this is

A **well-formed** program, a run the CLI accepts, and an answer that is false
under the definitions the user has ruled on. That means the verdict word, the
count, the model set, the certification or the core. The survey of
2026-10-14 found twelve such cases and one contested reading. **All but one
are reachable at default settings** (W9 needs `-K`), and three of them get the
verdict word itself wrong (W1, W2, W5). Two are seven-line programs:

```
$ ein solve -e examples/ein-bugs/wrong-answers/forced-2.ein   # one object, one hypothesis that leads to (false)
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (0 facts)
  exhausted        true
```

The program without any hypothesis is consistent, so its right answer is
`Solution`, model ∅. The engine never entered a commitment, refuted the
program, certified the refutation, and named no fact it rests on.

| id | case | class | settings | stage |
|---|---|---|---|---|
| **W1** | a maximal alive commitment is never recorded, at layer 0 too | verdict + certification | **default**, and 9 corpus entries under `-L` | [S1e2.2.1](p1e2.2_wrong_answers/s1e2.2.1_maximal_not_recorded.md) |
| **W2** | the forced-positive cascade refutes a consistent program | verdict + certification + core | **default** | [S1e2.2.2](p1e2.2_wrong_answers/s1e2.2.2_forced_positive_refutes.md) |
| **W5** | the obligations rung records a superset of a model, chosen by name order | verdict + model set | **default** | [S1e2.2.3](p1e2.2_wrong_answers/s1e2.2.3_obligations_join_superset.md) |
| **W4** | a rule's fresh constant never becomes a hypothesis | model set | **default** | [S1e2.2.4](p1e2.2_wrong_answers/s1e2.2.4_fresh_name_unreachable.md) |
| **W3** | `dead` is not upward-closed under `absent` | model set | **default** | [S1e2.2.5](p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md) |
| **W8** | a positive derived under `absent` survives the commitment that falsifies it | model set, *contested* | **default** | [S1e2.2.6](p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md) |
| **W7** | two activator layouts share a binding key, and a derivation is lost | model set | **default** (release) | [S1e2.2.7](p1e2.2_wrong_answers/s1e2.2.7_binding_key_layout.md) |
| **W6** | `exhausted = true` over relations the rung never hypothesises | certification, *contested* | **default** | [S1e2.2.8](p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md) |
| **W9** | with the kill cache off, an unnoticed entailment makes a state a model | verdict | `-K` | [S1e2.2.9](p1e2.2_wrong_answers/s1e2.2.9_kill_cache_off_entailment.md) |
| **C1** | the untyped blind rung binds a type as an object | model set, *contested* | **default** | [S1e2.2.10](p1e2.2_wrong_answers/s1e2.2.10_types_as_objects.md) |
| **W10** | the alternatives cap decides which core is reported | core | default | [S1e2.2.11](p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md) |
| **W11** | which core is reported depends on interner id order | core | default | [S1e2.2.12](p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md) |
| **W12** | an exhausted `Contradiction` reports a 0-fact core | core, *contested* | default | [S1e2.2.13](p1e2.2_wrong_answers/s1e2.2.13_empty_core.md) |

**Five of these had no id before the survey**: W2, W5, W8, W11 as a merge of
two fuzz findings nobody had owned, and W1's layer-0 form. The other eight
were M1e questions with *owner unassigned*, M1d measurements with no owner,
or fuzz findings described as *a decision nobody has taken*.

## Why the first phase is seeing, not fixing

Because every case above was found by **reading**: a hand derivation, a
probe written for a question, a twin fixture one fact apart. None was found
by looking at what the engine answered, and the engine makes that hard.
A model is a block of facts on stdout, a refutation is a ` ```dot ` fence no
viewer draws, and the program has no picture at all. Its two graph renderers
are built, tested and reachable from no command
([Q-M1e.20](../../docs/history/m1e_review_processing/open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable)).
And the right answers were also derived by reading. A fix stage that starts
from a misread case fixes the wrong thing in a way no test can catch, because
the test was written from the same misreading.

So [P1e2.1](p1e2.1_seeing_the_answer/README.md) builds the means to look. A
case register holds every wrong answer as data, and each answer becomes a
file. The renderers get CLI doors, and a model renders as the grid a person
would draw. Every artefact the engine can write for a run is rendered,
including `--trace --full-kb-snapshots`, `--dump-states` and `--events`. A
case page sets the wrong run, its control and the right answer side by side.
The phase then ends with the **user's review** of every case, and
[P1e2.2](p1e2.2_wrong_answers/README.md)'s stage list is edited to match
before anything is fixed.

**The review changed the definition on its first case.** Reading W1 on
2026-10-15, the user confirmed all seven of its rows. The user also gave a
reading that replaces *which node is a solution* with *what shape is the set
of nodes that are*: the answer is a **region R** of the hypothesis lattice,
the nodes that are alive and discharged. R lies between a minimal line that
obligations set and a maximal line that restrictions set, and it can have
holes where `absent` breaks monotonicity
([Q-M1e2.7](open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)).
The user also ruled that no config option may change the answer. So the
review paused, and
[P1e2.1a](p1e2.1a_the_solution_region/README.md) defines R, computes it,
draws it and makes the language and the read-out speak it, before the twelve
unread cases are read against it. **It resumed on 2026-11-01**, on the bundle
built at `17422b32`, where every case page draws its run on R
([S1e2.1a.7](p1e2.1a_the_solution_region/s1e2.1a.7_the_bundle_shows_r.md)).

**Then the oracle's first question split the definition in two.** A node can
be built on root's fixpoint or from scratch, and the two differ exactly where
adding a fact can take a conclusion away. The user moved that case into a
phase of its own, [P1e2.1b](p1e2.1b_non_monotone_programs/README.md),
on 2026-10-15. P1e2.1a finishes the monotone case and *detects* the
non-monotone one. P1e2.1b was to decide what R is there, and its first finding
is already in: a hole needs no `absent`, because an obligation's trigger is
the same kind of read. **On 2026-11-02 the user re-planned it** to detect,
document and warn, and to decide nothing: *"prepare everything we are able to
do now, without making final decisions; And then leave further investigation
of non-monotonic lattices as followup"*. The decisions are
[F21](../followups/f21_non_monotone_programs_decided.md)'s.

**Then the user ruled from the general case (2026-10-16).** In general R is
its node set, and the two lines are a simplification. `k` = |R|. `ein solve`
studies the region first. Solving moves into `(solve …)`, and queries ask
about the result. The verdict words judge a stated expectation, and without
one the run explores. The searches that look for the maximal line are right
only for a program that states no obligation, so `solve` *"has to be
reworked"*. The same day the user ruled the default: *no flag means an exhaustive,
certifying, full description of R*, with flags for any solution, a single one
expected, or one line only. That is
[S1e2.1a.9](p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
where it is cheap and exact, and
[P1e2.1c](p1e2.1c_the_region_search/README.md) beyond that, building on P1e2.1b's
[S1e2.1b.6](p1e2.1b_non_monotone_programs/s1e2.1b.6_monotone_and_non_monotone_programs_compared.md).
`k` became **k_s** and **k_c**. The terms are in
[`definitions.md`](p1e2.1a_the_solution_region/definitions.md).

## Phases

| ID | title | stages | est. | ends with |
|---|---|---:|---:|---|
| [P1e2.1](p1e2.1_seeing_the_answer/README.md) | Seeing the answer: dumps, and the answer rendered for review | 7 | 12 d | `utils/review_bundle.py` → `build/review/`: every case's problem, every run's every artefact rendered with every graph as SVG, the right answer and the diff; the register as a cargo test; Q-M1e.20 answered; **the review held** |
| [P1e2.1a](p1e2.1a_the_solution_region/README.md) | The solution region: R defined, computed, drawn and reported | 10 | 23 d | the answer redefined as a **region R** of the hypothesis lattice, with its minimal and maximal lines ([Q-M1e2.7](open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)), for a **monotone** program; a non-monotone one detected by a static check and reported through one stub; an oracle that computes R by saturation alone for n ≤ 16; the whole lattice drawn with the walk overlaid (a region run since S1e2.1a.7); programs written for a chosen region (S1e2.1a.8); the language and the read-out speaking R; the engine checked against the oracle under every lever; **the review resumed** |
| [P1e2.1b](p1e2.1b_non_monotone_programs/README.md) | Non-monotone programs: detected, documented, warned | 7 | 21 d | **decides nothing** (re-planned 2026-11-02; every decision is [F21](../followups/f21_non_monotone_programs_decided.md)'s): the site census; a node's state by fork and from scratch, measured (Q-M1e2.2 deferred); holes by cause (Q-M1e2.7 (d) deferred); the evidence on a refutation that rests on an `absent`, for S1e2.2.5; monotone and non-monotone programs compared in [`monotonicity.md`](../../docs/kernel/inference/monotonicity.md), with the two properties of the node functions and every option's consequences; an explicit monotonicity check, in a CLI form the user approves; a cheap detector in `ein solve`, warning on every surface that reports R, with no answer moved |
| [P1e2.1c](p1e2.1c_the_region_search/README.md) | The region search: R past the oracle's reach | 4 | 16.5 d | `ein solve`'s default describes R for every **monotone** program the corpus holds, or says why not: the alive set at scale, the lines by duality, the flags at scale; the old walk's fate, and P1e2.2 re-scoped. **Done 2026-11-03**: no row closed by construction, four of P1e2.2's stages dropped, the walk deleted around S4 and `-L`, `-K` and four `enable-*` fields retired on the user's rulings, `-y` the spot check unbounded. A flagged program keeps route 2 and P1e2.1b's warnings, and the non-monotone search moved to F21 (2026-11-02) |
| [P1e2.2](p1e2.2_wrong_answers/README.md) | The wrong answers, one stage each | 13, 4 dropped | 13.5 d | every register row `fixed` or ruled `not-wrong`; re-scoped by S1e2.1c.5 on 2026-11-03 (27.5 d before), nine stages live; no wrong answer at default settings; no lever that changes a certified answer |
| [P1e2.3](p1e2.3_surfaces_that_misreport_the_answer/README.md) | Surfaces that misreport the answer | 7 | 8.5 d | every witness re-taken 2026-11-02 (S1–S3 and S6–S9 gone, S4 and P1 live, S5 moved to `-g`, S10 the forms past 100 lines); the summary's `root` block as ruled and unmoved by every lever (S1e2.3.1); no render view panics on a keyword pair (S1e2.3.2); the forms under the 100-line ruling (S1e2.3.3); Q-M1e.5 and Q-M1e.17 ruled and closed, the re-take re-takable (S1e2.3.4). **S1e2.3.1–.4 done 2026-11-03**: no surface of the re-take is live, each under its ruling; **S1e2.3.5 done 2026-11-03**: the gate checks its own scratch; S1e2.3.7 (N1–N4, ruled 2026-11-03) and S1e2.3.6, last, to come |
| [P1e2.4](p1e2.4_what_the_switch_left/README.md) | What the switch left: the `absent` ruling, and the read-out at scale | 2 | 2 d | Q-M1e2.5 ruled, *forbid* or *respect*, and applied to W3's rows (S1e2.4.1); `--models key` declines before it allocates, and the read-out forms measured in memory on the largest regions (S1e2.4.2) |

**The order, ruled 2026-11-02.** P1e2.1a is done, and so is **P1e2.4**
(2026-11-02). Next is P1e2.1b, re-planned the same day to detect, document and
warn, with one question for the user (the explicit check's CLI form, on real
programs), then P1e2.1c, scoped to monotone programs, then P1e2.3. P1e2.4 was first put after P1e2.1c and moved ahead of P1e2.1b the same
day, on the user's *"reorder P1e2.4 before P1e2.1b"*: P1e2.1b's
[S1e2.1b.4](p1e2.1b_non_monotone_programs/s1e2.1b.4_refutation_resting_on_absent.md)
applies the *forbid* / *respect* ruling S1e2.4.1 takes. Only after P1e2.3 are
the wrong answers revisited, S1e2.1.7 and then P1e2.2, and the
revisit is the agent's first. The user, verbatim:
*"only after that revisit wrong answers - first your revisit and then if still remain any ambiguity or no criteria to decide correct solution - then I can review logs and renderings manually"*.
So the revisit reads every unreviewed row against the definitions as they
stand then, on a bundle rebuilt at that commit, and settles each row a
stated criterion decides. Only what it leaves ambiguous goes to the user.

### P1e2.3 — Surfaces that misreport the answer

**A pointer since 2026-11-03.** This § carried the phase while it was
proposed: the survey's ten surfaces and S4's paragraph. **Specified
2026-11-02** as [P1e2.3](p1e2.3_surfaces_that_misreport_the_answer/README.md), at the user's *"write P1e2.3 plans
also"*, and the user ruled that it becomes a phase, after P1e2.1c
([Q-M1e2.6](open_questions.md#q-m1e26--does-p1e23-become-a-phase), closed
2026-11-03). What stood here is in three places now. The survey's table, as
taken at `71dd237`, is [inventory § 3](inventory.md#3-cases). The phase's
[re-take](p1e2.3_surfaces_that_misreport_the_answer/README.md#the-re-take-2026-11-02) has one row per surface,
re-taken on 2026-11-03 with every row gone and what holds each. S4's paragraph,
the summary's `root` block, is
[S1e2.3.1](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md)'s, which fixed it.
`git log -p -- plans/m1e2_correctness/README.md` has the § as it stood.

**S1e2.3.1–.4 are done** (2026-11-03): S4, P1, S10 and S5 under the user's
rulings, S7 and S8 pinned, Q-M1e.5 and Q-M1e.17 closed. **The phase stays
open** for the two stages the user added the same day, S1e2.3.5 (test scratch
left in `/tmp`) and S1e2.3.6 (the nightly workflow, last).

## What it inherits from M1e

| question | here |
|---|---|
| [Q-M1e.8](../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set): `exhausted` certifies the lattice, not the model set | W1, S1e2.2.1 |
| [Q-M1e.9](../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent): `dead` and `absent` | W3, S1e2.2.5; the language ruling moved here from M1f's S1f.10.8 on 2026-11-01, as P1e2.4's S1e2.4.1, and was ruled *forbid* on 2026-11-02 |
| [Q-M1e.12](../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object): the untyped blind rung | C1, S1e2.2.10 |
| [Q-M1e.15](../../docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported): the alternatives cap | W10, S1e2.2.11 |
| [Q-M1e.16](../../docs/history/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one): the binding key | W7, S1e2.2.7 |
| [Q-M1e.21](../../docs/history/m1e_review_processing/open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about): fresh constants | W4, S1e2.2.4 |
| [Q-M1e.20](../../docs/history/m1e_review_processing/open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable): two renderers unreachable | S1e2.1.3, answered (a) |
| [Q-M1e.5](../../docs/history/m1e_review_processing/open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) and [Q-M1e.17](../../docs/history/m1e_review_processing/open_questions.md#q-m1e17--three-int_arg-options-silently-reinterpret-a-negative) | [P1e2.3](p1e2.3_surfaces_that_misreport_the_answer/README.md)'s S1e2.3.4: **both closed 2026-11-03**, Q-M1e.5 by the ruling of 2026-10-16 (the learning half to M1f's S1f.10.7), Q-M1e.17 by `-g`'s ruling (b) |

The ids stay `Q-M1e.<n>`. An id is sticky and is never reused, so this
milestone cites them across the boundary, as M1f does.
[`open_questions.md`](open_questions.md) here holds what **M1e2** raises.

## Acceptance for the milestone

- **No register row is `open`.** Every case in
  `corpus/wrong_answers.toml` is `fixed`, with the test that holds it, or
  `not-wrong`, with the review's reason in the row and in the witness's header.
- **No wrong answer at default settings.** No lever changes an answer the
  engine *certifies*. A lever may change cost, and may decline to certify
  (`exhausted = false`, said why), and that is all.
- **Every contested case has a ruling**, the user's, written where the
  definition lives (`solution_semantics.md`, `absent_semantics.md`), not only in
  a stage file.
- **The inventory is re-taken at the close.** [`probes/capture.sh`](probes/capture.sh)
  re-runs every witness, and every W-case answers its right answer. The
  corpus sweeps of [inventory § 4](inventory.md#4-reach--corpus-sweeps-run-for-this-inventory)
  are re-run. A new wrong answer found then becomes a register row and is not
  waved through. **The script's W lines cannot run as written** (reported by
  P1e2.3's S1e2.3.4, 2026-11-03): they spell the walk's options, and most are
  refused since S1e2.1a.5's switch as retired (`-e`, `-m`, `-L`, `-K`), so
  each W line has to be re-spelt as a region run before this re-take. Its S
  and P lines were re-spelt by S1e2.3.4 and write P1e2.3's table; the W lines
  are this acceptance's, and the register's runs (`corpus/wrong_answers.toml`,
  held by `wrong_answers.rs`) are where today's spellings are.
- **The review bundle builds in the nightly** and its smoke step is in the
  per-commit gate.
- `./run_tests.sh` green at every stage boundary, and every golden a stage
  moves is named in its file before it moves.

## Risks

- **The fixes move answers, by definition.** The repo's discipline is the
  defence: an unexplained change in the per-stage corpus diff is a stop, not a
  re-bless. W1's fix alone reaches 9 entries' `-L` cells and two of the
  corpus's own `:expect`s.
- **W1's fix costs at `-n 1`.** Recording maximality at layer n+1 defers a
  model by one layer when the search stops at the first. D3 called it a
  trade-off to measure, and S1e2.2.1 measures it before choosing.
- **The inventory is a reading too.** It re-ran every case, and still
  collected them by sweeping documents and probing mechanisms. The review in
  S1e2.1.7 is where a case is withdrawn. Its corpus-wide bundle mode is where
  one is *added*. A review that adds none probably did not look at the
  corpus pages.
- **Contested readings can turn into language design.** W3, W8 and C1 each
  border on *what the language means*: `absent`, stratification, types. The
  stages are written to take a ruling and apply it, and the line is that a
  ruling which needs a new language feature becomes a followup, not a stage.
- **W8's reading is now a followup's.** On 2026-11-02 the user deferred
  Q-M1e2.2 to [F21](../followups/f21_non_monotone_programs_decided.md), and
  the acceptance below asks that no register row be `open` and that every
  contested case have a ruling. W8's two rows meet both head on. Parked for
  the revisit, not asked now: it reads W8 under today's documented node, or
  the acceptance is amended
  ([S1e2.2.6 § The deferral](p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md#the-deferral-and-the-milestones-acceptance)).
- **Overlap with M1f.** S1f.10.3 (the restricted join), S1f.10.4 (bounded
  groups) and S1f.10.8 (refutation under `absent`) all touch what this
  milestone fixes. [P1e2.2 § Where this phase meets M1f](p1e2.2_wrong_answers/README.md#where-this-phase-meets-m1f)
  draws the line per stage, and the rule is that **this milestone goes
  first**, because M1f's acceptance is stated relative to the answers.

## Connections

- [`inventory.md`](inventory.md): the evidence, with [`probes/`](probes/)
  (`capture.sh`, four sweep scripts, and the eleven programs no register row
  names). The twenty that are witnesses were banked at S1e2.1.1, eighteen
  under [`examples/ein-bugs/wrong-answers/`](../../examples/ein-bugs/wrong-answers/)
  and two under `corpus/fuzz_findings/`.
- [`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml): the
  register — every case's witnesses, wrong runs and right answers as data,
  held by `ein-cli/tests/wrong_answers.rs`.
- [M1e](../../docs/history/m1e_review_processing/README.md): where most of these were found
  and filed.
- [`docs/kernel/inference/solution_semantics.md`](../../docs/kernel/inference/solution_semantics.md):
  the definition every right answer is derived against.
- [`docs/kernel/standard_of_proof.md`](../../docs/kernel/standard_of_proof.md):
  what *fixed*, *refuted* and *not-wrong* have to rest on.
- [M2 P2.1](../m2_nl_to_ir/p2.1_kernel_as_instrumentation/README.md): the
  next consumer of the verdict vocabulary, and the reason a false
  *contradictory* is not a cosmetic defect.
