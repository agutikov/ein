# M1e2 — Correctness: every known wrong answer

**Estimate:** ~23 weeks: 7 estimated phases and 66 stages, five of them
dropped, **116 days** of stage estimates (P1e2.1 12, P1e2.1a 23, P1e2.1b
21, P1e2.1c 16.5, P1e2.2 30, P1e2.3 11.5, P1e2.4 2). It was 115 until the
user's ruling of 2026-11-05 on S1e2.2.25's finding
([Q-M1e2.19](open_questions.md#q-m1e219--what-do---minimal-and---maximal-answer-past-the-limit))
gave P1e2.2 S1e2.2.26 (W27, the line modes past the limit, 1 day). It was
109 until the
user's ruling on [the inventory of 2026-11-05](inventory_2026-11-05.md)
([Q-M1e2.16](open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items)) gave P1e2.2 four stages, S1e2.2.22 (W22, 1 day),
S1e2.2.23 (W23, 1 day), S1e2.2.24 (W24, 1.5 days) and S1e2.2.25 (the
deferred rows, 1 day), and P1e2.3 one, S1e2.3.10 (N7 and N8, 1 day), and
widened S1e2.3.8 by a dumped state's provenance (1 day → 1.5). It was 104 until
P1e2.2 gained S1e2.2.20 (W20, 1.5 days) and S1e2.2.21 (W21, 2 days) and
P1e2.3 S1e2.3.8 (N5, 1 day) and S1e2.3.9 (N6, 0.5 day) on 2026-11-05, on
the user's rulings of 2026-11-04. It was 102.5 until P1e2.2
gained S1e2.2.19 (W19, a dead node's core missing the activator its
refutation rests on, 1.5 days) on 2026-11-04, from S1e2.2.17's notes. It was 100 until P1e2.2
gained S1e2.2.17 (W17, 1.5 days) and S1e2.2.18 (W18, 1 day) on 2026-11-04,
from S1e2.2.16's notes. It was 99 until P1e2.2 gained
S1e2.2.16 (W16, an activator's int and string sharing a compile key, 1 day)
on 2026-11-04. It was 95.5 until P1e2.2
gained S1e2.2.14 (W15, the user's ruling on names, 2 days) and S1e2.2.15
(W14, the obligations rung's universe, 1.5 days) on 2026-11-04. It was 96.5 until the revisit,
S1e2.1.7, on 2026-11-03: it dropped P1e2.2's S1e2.2.13 (0.5 day, W12 closed by
`solve_interface.md` § 0), and the user's ruling on C1 restated S1e2.2.10 (2
days → 1.5). It was 95.5 until the user put
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
also"*: four stages, 5 days, and seven since 2026-11-03; it **closed
2026-11-03**, every stage done.
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
| [P1e2.1](p1e2.1_seeing_the_answer/README.md) | Seeing the answer: dumps, and the answer rendered for review | 7 | 12 d | `utils/review_bundle.py` → `build/review/`: every case's problem, every run's every artefact rendered with every graph as SVG, the right answer and the diff; the register as a cargo test; Q-M1e.20 answered; **the review held**: done 2026-11-03, every row reviewed |
| [P1e2.1a](p1e2.1a_the_solution_region/README.md) | The solution region: R defined, computed, drawn and reported | 10 | 23 d | the answer redefined as a **region R** of the hypothesis lattice, with its minimal and maximal lines ([Q-M1e2.7](open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)), for a **monotone** program; a non-monotone one detected by a static check and reported through one stub; an oracle that computes R by saturation alone for n ≤ 16; the whole lattice drawn with the walk overlaid (a region run since S1e2.1a.7); programs written for a chosen region (S1e2.1a.8); the language and the read-out speaking R; the engine checked against the oracle under every lever; **the review resumed** |
| [P1e2.1b](p1e2.1b_non_monotone_programs/README.md) | Non-monotone programs: detected, documented, warned | 7 | 21 d | **decides nothing** (re-planned 2026-11-02; every decision is [F21](../followups/f21_non_monotone_programs_decided.md)'s): the site census; a node's state by fork and from scratch, measured (Q-M1e2.2 deferred); holes by cause (Q-M1e2.7 (d) deferred); the evidence on a refutation that rests on an `absent`, for S1e2.2.5; monotone and non-monotone programs compared in [`monotonicity.md`](../../docs/kernel/inference/monotonicity.md), with the two properties of the node functions and every option's consequences; an explicit monotonicity check, in a CLI form the user approves; a cheap detector in `ein solve`, warning on every surface that reports R, with no answer moved |
| [P1e2.1c](p1e2.1c_the_region_search/README.md) | The region search: R past the oracle's reach | 4 | 16.5 d | `ein solve`'s default describes R for every **monotone** program the corpus holds, or says why not: the alive set at scale, the lines by duality, the flags at scale; the old walk's fate, and P1e2.2 re-scoped. **Done 2026-11-03**: no row closed by construction, four of P1e2.2's stages dropped, the walk deleted around S4 and `-L`, `-K` and four `enable-*` fields retired on the user's rulings, `-y` the spot check unbounded. A flagged program keeps route 2 and P1e2.1b's warnings, and the non-monotone search moved to F21 (2026-11-02) |
| [P1e2.2](p1e2.2_wrong_answers/README.md) | The wrong answers, one stage each | 26, 5 dropped | 30 d | every register row `fixed` or ruled `not-wrong`, or `deferred` to a followup by the user; re-scoped by S1e2.1c.5 on 2026-11-03 (27.5 d before) and by the review the same day (S1e2.2.13 dropped, S1e2.2.10 restated), eight stages live; no wrong answer at default settings; no lever that changes a certified answer. **Started 2026-11-04; S1e2.2.4 done** that day: W4's two rows fixed by seeding the candidates with a rule's constants, 23 fixed, 11 open, 1 not-wrong. **S1e2.2.5 done** the same day: *forbid*'s refusal built, with the user's F2, *via, per activator*; W3's four rows, `W1/alive-empty-interlayer` and `W8/stale-naf-fork` fixed by it, `:warn-derived-naf` retired, 29 fixed, 5 open, 1 not-wrong. **S1e2.2.14 (W15) and S1e2.2.15 (W14) filed** the same day, ten stages live. **S1e2.2.14 done** the same day: the blind enumerator's objects are every name of the program (Q-M1e2.9 (a)), W15's eight rows fixed, W7's `right` restated to R, 37 fixed, 5 open, 1 not-wrong. **S1e2.2.15 done** the same day: Q-M1e2.10 ruled (a), the rung declines on a guard over D, W14's four rows fixed, 41 fixed, 5 open, 1 not-wrong. **S1e2.2.6 done** the same day, as far as the deferral allows: `W8/features-07` re-checked and still `deferred` to F21's D2, `stale-naf-fork` refused at every command that takes the universe, the witnesses' headers corrected, nothing moved. **S1e2.2.7 done** the same day: Q-M1e.16's fix (1), a rule application's identity interns its activator's every argument, W7's row fixed in both profiles, its pair promoted to `examples/ein-bugs/`, Q-M1e.16 closed, 42 fixed, 4 open, 1 not-wrong. **S1e2.2.8 done** the same day: Q-M1e2.3 (a) built, every question's read-out names what H leaves out (the `outside` row, `root.hypgen.outside`), W6's two rows fixed, no answer moved, 44 fixed, 2 open, 1 not-wrong. **S1e2.2.10 done** the same day: Q-M1e.12 (a) built, `branching/06` and `/07` typed (R two states), a `types` row on every read-out where the blind enumerator binds a type as an object, C1's and `W1/branching-07`'s rows moved to the retyped R, still 44, 2 and 1. **S1e2.2.11 done** the same day: paused on Q-M1e2.11 and ruled (i)(b), (ii)(b); Q-M1e.15's (3) built, `W10/alt-cap-core` fixed, `W10/alt-cap-shared` open and deferred to F22, the core exempt under `:record-alternative-justifications`, Q-M1e.15 closed, 45 fixed, 2 open (both deferred), 1 not-wrong. **S1e2.2.12 done** the same day: the user's second visit to W11, both rows confirmed; the id-order sweep compares every core and asserts nothing moves; a resumed fork's `FactId`-ordered delta could move a dead node's core, so `W11/fork-delta-order` was filed and fixed by the delta in the order its commitment was written (`10858b96`), no tie-break needed; W11's witnesses promoted; 46 fixed, 2 open (both deferred), 1 not-wrong. **S1e2.2.16 done** the same day, the phase's last live stage: a compile plan keyed by its activator's values, so an int and a string with one text no longer share one and lose a derivation, and *forbid* asks a plan's rule for its parameters, so an activator with no symbol argument is one; W16's four rows fixed (`3257488f`), 50 fixed, 2 open (both deferred), 1 not-wrong. **S1e2.2.17 (W17) and S1e2.2.18 (W18) filed** the same day from S1e2.2.16's notes. **S1e2.2.17 done** the same day: *forbid* reads *extendable* as D followed through the rules root has no plan of, every such rule read and its own `absent` tainting what it derives; W17's five rows fixed (`f5a0dca9`), the status line names the site up an activator chain or says why none, 55 fixed, 2 open (both deferred), 1 not-wrong. **S1e2.2.18 done** the same day: a string in a rule's `:match`, `:assert` or guard is the symbol it names, as in a fact, and a name of the program; W18's five rows fixed (`259c16e4`), and the line a conclusion no fact can hold still stops the run with names no Python class, 60 fixed, 2 open (both deferred), 1 not-wrong. **S1e2.2.19 (W19) filed** the same day from S1e2.2.17's notes, and **started**: three rows filed under schema `/8`, every core measured under three readings; following every activator moves 40 corpus programs' cores and a derived one three's, so it is **paused on Q-M1e2.12**, 60 fixed, 5 open, 1 not-wrong. Q-M1e2.12 ruled (c) the same day, and **S1e2.2.19 done**: a firing records its activator, a dead node's core follows one a node derives or commits, W19's three rows fixed (`94c7bcfb`), no corpus core moved, 63 fixed, 2 open (both deferred), 1 not-wrong. The user's other questions ruled the same day. **S1e2.2.20 (W20) started** 2026-11-05: five rows filed (`_` in a premise, in the head, under an `absent`, in an hrule; a range), the argument `_` and the range prototyped with no corpus run moved; a head `_` needs a second matching path, so it is **paused on Q-M1e2.14**, 63 fixed, 7 open, 1 not-wrong. Q-M1e2.14 ruled (c) the same day, and **S1e2.2.20 done**: `_` in a rule's pattern matches any value and binds nothing, a range and a `_` written as a relation refused at load, W20's five rows fixed (`eddea251`), no answer moved outside W20's programs, 68 fixed, 2 open (both deferred), 1 not-wrong. **S1e2.2.21 (W21) started** the same day: four rows filed, every core measured; the pair fix stops 2 172 cores naming their commitment, through a `(false)` concluded by a `forall`, so it is **paused on Q-M1e2.15**, 68 fixed, 6 open, 1 not-wrong. Q-M1e2.15 ruled (b) the same day, with a core taken only where a surface prints one; **W21 fixed**: a contradiction's facts together, a `forall`'s support recorded, six rows fixed, 74 fixed, 2 open (both deferred), 1 not-wrong. **S1e2.2.21 done** the same day: a core taken only where a surface prints one, no run moved; the class-4 claim a corpus-wide test. **Four stages added** the same day on the user's ruling on the inventory of 2026-11-05 (Q-M1e2.16), specified the same day: S1e2.2.22 (W22, a keyword prefix split in a rule), S1e2.2.23 (W23, a core search cut at its budget), S1e2.2.24 (W24, three rule shapes refused at load) and S1e2.2.25 (the owned wrong answers filed as deferred rows). **S1e2.2.22 (W22) started** the same day: fourteen rows filed, one per place the parser's keyword-prefix split changes an answer, the whole-word fix prototyped with no program outside W22's moved; `(neq-1 ?x)` and the trace events **paused on Q-M1e2.17**, 74 fixed, 16 open, 1 not-wrong. Q-M1e2.17 ruled (i)(a) and (ii)(a) the same day; **W22 fixed**: a keyword is a whole word, fourteen rows fixed, 88 fixed, 2 open (both deferred), 1 not-wrong; **S1e2.2.22 done** (`ed45a470`). **S1e2.2.23 (W23) started** the same day: one row filed under schema `/9`, the corpus's cut cores counted (19 of N's 5 442, at the search's rounds on `zebra.ein` and its environments elsewhere), the qualifier prototyped with every move listed, 88 fixed, 3 open, 1 not-wrong; **W23 fixed**: a core's status beside every core on every surface, held by a corpus test, 89 fixed, 2 open (both deferred), 1 not-wrong; **S1e2.2.23 done** (`be7893d9`). **S1e2.2.24 (W24) started** the same day: three rows filed, one per shape, every shape probed in a rule, an hrule and an obligation, the refusal prototyped with no answer outside W24's programs moved; four more shapes that read nothing and a corpus fixture that writes the second, so it is **paused on Q-M1e2.18**, 89 fixed, 5 open, 1 not-wrong. Q-M1e2.18 ruled as recommended the same day; **W24 fixed**: each shape refused at load, the ruling's four filed and fixed, seven rows, 96 fixed, 2 open (both deferred), 1 not-wrong; **S1e2.2.24 done** (`3ab9a231`). **S1e2.2.25 started** the same day: paused on a finding of its re-take (past `--region-limit` on a flagged program, `--minimal` and `--maximal` label every member met as on their line), ruled the same day (Q-M1e2.19, a new stage S1e2.2.26, W27); eight rows filed `open`, `deferred` to F21, F22 and M1f's S1f.10.6, schema `/10`, 96 fixed, 10 open (all deferred), 1 not-wrong; **S1e2.2.25 done** (`eb882d53`), and **S1e2.2.26 (W27) specified** the same day, one stage and one day more. **S1e2.2.26 started** the same day: W27 filed as four `open` rows under schema `/11`, every move of the fix listed, 96 fixed, 14 open, 1 not-wrong; **W27 fixed** the same day: a line mode past the limit answers from describe's partial description, 100 fixed, 10 open (all deferred), 1 not-wrong; **S1e2.2.26 done** (`c21f93fc`). **P1e2.2 is complete** (2026-11-05). Next, P1e2.3's S1e2.3.8 (N5 and a dumped state's provenance), S1e2.3.9 (N6) and S1e2.3.10 (N7, N8), then the inventory re-taken and shown to the user; the milestone's close waits for the user's go-ahead |
| [P1e2.3](p1e2.3_surfaces_that_misreport_the_answer/README.md) | Surfaces that misreport the answer | 10 | 11.5 d | every witness re-taken 2026-11-02 (S1–S3 and S6–S9 gone, S4 and P1 live, S5 moved to `-g`, S10 the forms past 100 lines); the summary's `root` block as ruled and unmoved by every lever (S1e2.3.1); no render view panics on a keyword pair (S1e2.3.2); the forms under the 100-line ruling (S1e2.3.3); Q-M1e.5 and Q-M1e.17 ruled and closed, the re-take re-takable (S1e2.3.4). **S1e2.3.1–.4 done 2026-11-03**: no surface of the re-take is live, each under its ruling; **S1e2.3.5 done 2026-11-03**: the gate checks its own scratch; **S1e2.3.7 done 2026-11-03**: N1–N4 say what they print, each held by a test, no exit code moved; **S1e2.3.6 done 2026-11-03**: every nightly step green under `./run_tests.sh --nightly`, which `nightly_steps.rs` holds to the workflow, the mutation bank's eight kills explained (165 of 217). **Closed 2026-11-03**, its acceptance re-taken line by line. **Re-opened 2026-11-05** for S1e2.3.8 (N5, the fact printer) and S1e2.3.9 (N6, a core the run did not take), on the user's ruling of 2026-11-04, and on the inventory's ruling the same day (Q-M1e2.16 (3)) widened: a dumped state's provenance joins S1e2.3.8, and S1e2.3.10 takes N7 (`refuted by`) and N8 (`--json-report`'s rows); the three run after P1e2.2's S1e2.2.25. **S1e2.3.8 done 2026-11-06** (`712f23fd`): one printer on every surface, the provenance a comment, every state file reloading by value; Q-M1e2.20 ruled (a), W28 filed and fixed, 103 fixed, 10 open, 1 not-wrong. **S1e2.3.9 done 2026-11-06** (`b6139e62`): past `--region-limit` a member of N is written `not-kept`, never with an empty core, and no answer moved. Next S1e2.3.10 |
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
stated criterion decides. Only what it leaves ambiguous goes to the user. P1e2.3
closed on 2026-11-03, so by this order the revisit, S1e2.1.7, was next. **It
ran and closed on 2026-11-03**: 23 rows settled by stated criteria, and the
five no criterion decided ruled by the user the same day
([S1e2.1.7 § The user's rulings](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#the-users-rulings-2026-11-03)).
**The milestone's next step is P1e2.2**, in its stage order
([P1e2.2 § Stages](p1e2.2_wrong_answers/README.md#stages)). It started on
2026-11-04 with S1e2.2.4 and S1e2.2.5, both done that day. Two stages were
filed the same day from S1e2.2.4's residue probe: S1e2.2.14 (W15, a name the
program writes outside a top-level fact or an `:assert`, on the user's ruling
[Q-M1e2.9](open_questions.md#q-m1e29--which-constants-are-names-of-the-program))
and S1e2.2.15 (W14, the obligations rung's universe), to run in that order.
S1e2.2.14 is done (2026-11-04): W15's eight rows are fixed. S1e2.2.15 is
done the same day: the user ruled
[Q-M1e2.10](open_questions.md#q-m1e210--when-the-obligations-rung-cannot-trust-its-domain-scan-whose-candidates-is-r-over)
(a), the obligations rung declines on a guard over a relation in D and the
blind enumerator builds H, and W14's four rows are fixed. S1e2.2.6 is done
the same day, as far as the deferral allows: W8's one row,
`W8/features-07`, answers what it records and stays `open`, `deferred` to
F21's D2, the row the milestone closes naming, and the witnesses' headers say
so. S1e2.2.7 is done the same day: Q-M1e.16's fix (1), a rule
application's identity interns its activator's every argument, as the
compile cache's key does, so W7's int and nested fact in one activator
position derive the same `noted` facts in either order and either build;
its row is fixed, its pair promoted to `examples/ein-bugs/`, and Q-M1e.16
closed. S1e2.2.16 (W16) was filed from its notes the same day, to run last,
and a surface found beside it is parked as P1e2.3's
[N5](p1e2.3_surfaces_that_misreport_the_answer/README.md#candidates-found-on-the-way),
the fact printer, to be put to the user when P1e2.2 ends.
S1e2.2.8 is done the same day: on the user's ruling of 2026-11-03,
[Q-M1e2.3](open_questions.md#q-m1e23--what-does-exhausted-certify-when-the-rung-leaves-relations-uncovered)
(a), every question's read-out names the relations H leaves out that the
program can conclude or ask about, W6's `knows` and `r3`, and no answer moved;
W6's two rows are fixed.
S1e2.2.10 is done the same day: on the user's ruling of 2026-11-03 on
[Q-M1e.12](../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object)
(a), `branching/06` and `/07` type `co-located`, so their R is two states,
Blue in H5, and every read-out names a type the blind enumerator binds as an
object, a `types` row under `universe`; C1 stays not wrong, and its row and
`W1/branching-07` moved to the retyped program's R.
**S1e2.2.11 started the same day and is paused on
[Q-M1e2.11](open_questions.md#q-m1e211--which-derivations-is-the-unsat-core-the-smallest-over-and-may-a-config-flag-change-it)**
(T1e2.2.11.1): W10's pair re-taken, the closure size measured on a prototype
of Q-M1e.15's (3), and every move listed before any edit; on the corpus (3)
moves `alt-cap-core`'s core alone, as an uncapped store does. But no bounded
retention is exact, and `:record-alternative-justifications false` already
decides the core, which the fix makes `alt-cap-core` show; both are put to
the user, the recommendation (3) with the residual deferred, and the core
exempt under that one flag. **The user ruled (i)(b) and (ii)(b) the same day,
and S1e2.2.11 is done** (2026-11-04): a full alternatives list keeps beside it
the arrival with the smallest closure, so `alt-cap-core` and its twin report
one core and `W10/alt-cap-core` is fixed; W10's second witness, a saving
between two facts behind a full list, is open and deferred to
[F22](../followups/f22_exact_core_past_the_cap.md), named at the close beside
`W8/features-07`; the core alone is exempt from the config principle under
`:record-alternative-justifications false`; Q-M1e.15 is closed.
S1e2.2.12 is done the same day, the second of the user's two visits to W11,
read by the agent: both rows confirmed; with every core a run reports compared
under eight permutations of the interner no corpus core moved, but a resumed
fork's delta, sorted by `FactId`, could move a dead node's core, which
`W11/fork-delta-order` showed and `10858b96` fixed by writing the delta in the
order its commitment was written; nothing moves since, narration included.
S1e2.2.16 is done the same day, W16, the phase's last stage: the compile key
is the activator's values, not their text, so the int `1` and the string
`"1"` key two plans and both activator orders derive the four `noted` facts
in either build; and *forbid* asks whether a plan has an activator by its
rule's parameters, so a refutation resting on an activator with no symbol
argument is refused as the symbol control is (`3257488f`). **Every live stage
of P1e2.2 is done.** The register reads 50 fixed, 2 open and 1 not-wrong,
and the two open rows are the ones the user deferred to a followup,
`W8/features-07` (F21's D2) and `W10/alt-cap-shared` (F22), which the
milestone's acceptance names at the close. Two findings in
[S1e2.2.16 § Notes](p1e2.2_wrong_answers/s1e2.2.16_activator_text_key.md#notes)
are wrong answers settled by a stated criterion, and were filed the same day
as **S1e2.2.17** (W17, a refutation *forbid* misses with symbols only, when a
chain's last activator first appears at a node) and **S1e2.2.18** (W18, a
string in a rule's `:match` matches nothing, and in an `:assert` stops the
run). **S1e2.2.17 is done the same day**: *forbid* reads *extendable* as D
followed through every rule with parameters whose activator relation it
holds, reads every such rule, and lets its own `absent` taint what it
derives, so W17's five witnesses are refused at every command that takes the
universe, D's other readers unmoved, and the status line names the site up an
activator chain or says why none (`f5a0dca9`). **S1e2.2.18 is done the
same day**: a string written in a rule's `:match`, `:assert` or guard, at any
depth, is the symbol it names, as the loader holds it in a fact, and a name
of the program, so W18's five witnesses answer their atom twins' R in either
build; the line a conclusion no fact can hold still stops the run with is the
rule's own words, not a `TypeError` (`259c16e4`). The register reads 60
fixed, 2 open (both deferred) and 1 not-wrong. **One more wrong answer was
filed the same day**, from
[S1e2.2.17 § Notes](p1e2.2_wrong_answers/s1e2.2.17_forbid_activator_chain.md#notes),
settled by the core's own definition: **S1e2.2.19** (W19, a dead node's core
leaves out the activator its refutation rests on, on a program that loads).
**S1e2.2.19 started the same day and paused** on
[Q-M1e2.12](open_questions.md#q-m1e212--which-activators-does-a-dead-nodes-core-follow),
which activators a dead node's core follows: the stage file's two readings
moved the cores of 40 and of 3 corpus programs beyond the witness, a third,
*one only a node derives*, none (the stage's sweep, 2026-11-04). The user
ruled that third, (c), the same day, and **S1e2.2.19 is done**: a firing
records the activator its plan was compiled from, and a dead node's core
follows one the node derived or committed, never root's; W19's three rows
are fixed (`94c7bcfb`) and no corpus core moved. The register reads 63
fixed, 2 open (both deferred) and 1 not-wrong. **The user ruled the
remaining questions the same day**: S1e2.2.18's wildcard and range in a
rule become W20 (S1e2.2.20: `_` in a rule's pattern matches any value, as
the grammar says, head and argument; a range in a rule is refused at load);
the pair contradiction's core, parked in
[S1e2.2.19 § Notes](p1e2.2_wrong_answers/s1e2.2.19_core_misses_activator.md#notes),
is W21 (S1e2.2.21); P1e2.3's
[N5](p1e2.3_surfaces_that_misreport_the_answer/README.md#candidates-found-on-the-way),
the fact printer, becomes P1e2.3's S1e2.3.8; and after those land, P1e2.2
and M1e2 close. **Next: S1e2.2.20 (W20), S1e2.2.21 (W21) and S1e2.3.8 (N5),
filed after S1e2.2.19, then the milestone's close.** **Filed 2026-11-05**:
[S1e2.2.20](p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md),
[S1e2.2.21](p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md),
[S1e2.3.8](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md),
and by default [S1e2.3.9](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md),
P1e2.3's N6 (a run stopped past `--region-limit` dumps the members of N it
met with an empty core), from S1e2.2.19's notes. They run in that order,
then the close: P1e2.2 and M1e2 closed as the earlier milestones were, a
`docs/history/` record and `plans/m1e2_correctness/` deleted, F21 and F22
owning the two deferred rows, the record shown to the user before anything
is deleted (the user's ruling of 2026-11-04). **S1e2.2.20 is next.**
**S1e2.2.20 started 2026-11-05** and is **paused on
[Q-M1e2.14](open_questions.md#q-m1e214--how-does-a-head-wildcard-match-every-relation)**
(T1e2.2.20.1): W20 filed as five `open` rows (63 fixed, 7 open, 1
not-wrong), the argument `_` (a slot that matches any value and binds
nothing) and the range's refusal at load prototyped, every argument-wildcard
witness answering its twin's R and no corpus run moving; a head `_` is no
slot, and matching every relation needs a second path, a plan per relation
or a step over every relation, so the head is the user's, with the agent's
recommendation to refuse it at load. **The user ruled (c) the same day, and
S1e2.2.20 is done** (`eddea251`): `_` in a rule's pattern is a slot that
matches any value and binds nothing, a range in a rule and a `_` written as a
relation are refused at load, W20's five rows are fixed, and the register
reads 68 fixed, 2 open (both deferred) and 1 not-wrong. **Next: S1e2.2.21
(W21), then P1e2.3's S1e2.3.8 and S1e2.3.9, then the milestone's close.**
**S1e2.2.21 started 2026-11-05** and is **paused on
[Q-M1e2.15](open_questions.md#q-m1e215--what-does-a-derivation-through-a-forall-rest-on)**
(T1e2.2.21.1): W21 filed as four `open` rows (68 fixed, 6 open, 1
not-wrong), and the pair fix prototyped with every core the corpus reports
compared. It starts 69 201 cores naming their commitment and stops 2 172, on
`zebra.ein`, `zebra2`, `zebra2-minus-15` and their twins, where a `(false)`
concluded through a `forall` (`total`, `slot-no-room`, …) then wins with two
facts root holds: a firing records none of the facts a universal read. What
such a derivation rests on is put to the user with the agent's
recommendation, its support followed at each fact's smallest frontier: every
core the corpus reports then names its commitment and derives a
contradiction with root, but `lattice/06`'s empty ones (N6), at ×6 on `zebra2`'s bare `ein solve` (0.59 → 3.68
s, 2026-11-05). **The user ruled (b) the same day**, and that a core is
taken only where a surface prints one, inside the stage; **S1e2.2.21 is
done** (2026-11-05): a contradiction's facts explained together and a
`forall`'s support recorded (`67d07fab`), W21's six rows fixed, the `forall`
gap filed as rows of W21, the register at 74 fixed, 2 open (both deferred)
and 1 not-wrong; a core taken only where a surface prints one, no run moved,
a bare `ein solve examples/zebra2.ein` 0.53 s (`3fb9d712`); and every core
the corpus reports holding the class-4 claim, as a test (`af521c6e`).
**The inventory the user asked for before the close was taken the same
day**, [`inventory_2026-11-05.md`](inventory_2026-11-05.md), at `cdcc3994`:
every register row, every stage's notes, every open question and followup,
and every corpus program solved and checked against the oracle, each item
placed by whether a program it reaches is monotone or non-monotone and by
what it does to R. The user ruled on its items the same day
([Q-M1e2.16](open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items)), and four stages were added here: S1e2.2.22 (W22),
S1e2.2.23 (W23), S1e2.2.24 (W24) and S1e2.2.25 (the deferred rows).
**Next: S1e2.2.22, .23, .24 and .25, then P1e2.3's S1e2.3.8, S1e2.3.9 and
S1e2.3.10, then the inventory re-taken and shown to the user. The
milestone's close waits for the user's go-ahead.**
**S1e2.2.22 started 2026-11-05** and is **paused on
[Q-M1e2.17](open_questions.md#q-m1e217--where-does-a-keyword-end)**
(T1e2.2.22.1): W22 filed as fourteen `open` rows (74 fixed, 16 open, 1
not-wrong), one per place the parser's keyword-prefix split changes an
answer, each witness banked with its twin, and the whole-word fix prototyped
with every move listed: 710 of 12 360 runs, on W22's witnesses and the
stage's probes, no other program. Where a keyword ends before `-` and a
digit, and what a trace event reads as, are the user's, with the agent's
recommendation, the word boundary `00_ebnf.md` § 1 states and a parse error. **The user ruled both as recommended the same day, and W22 is fixed**: a
keyword is a whole word, each witness answers its twin's R, and the register
reads 88 fixed, 2 open (both deferred) and 1 not-wrong. **S1e2.2.22 is
done** (`ed45a470`), the pages that stated the split corrected.
**S1e2.2.23 started the same day** (T1e2.2.23.1): W23 filed as one `open`
row under the register's schema `/9` (88 fixed, 3 open, 1 not-wrong), the
witness banked with its control and its five-pair twin, the cores the corpus
cuts counted on every surface that takes them (19 of N's 5 442, on five
programs; the search's 64 rounds bite on `zebra.ein` and its 64 environments
per fact elsewhere, each core the one a budget sixteen times larger gives),
and the qualifier prototyped, 2 798 of 13 546 runs moving by the qualifier
alone. **W23 is fixed the same day**: a core's status is carried beside every
core a surface prints, and a corpus test holds that none is printed
unqualified from a cut search; the register reads 89 fixed, 2 open (both
deferred) and 1 not-wrong. **S1e2.2.23 is done** (`be7893d9`), the pages
that said a cut is reported corrected; a finding off its list, not W23's, is
parked in its notes.
**S1e2.2.24 started the same day** and is **paused on
[Q-M1e2.18](open_questions.md#q-m1e218--what-else-in-a-rule-reads-nothing-in-silence)**
(T1e2.2.24.1): W24 filed as three `open` rows, one per shape (89 fixed, 5
open, 1 not-wrong), every shape probed in a rule, an hrule and an
obligation, and the refusal prototyped in S1e2.2.20's load pass, 2 597 of
18 018 runs moving, on W24's programs, the probes and three corpus fixtures
already refused, whose line moves. Four more shapes that read nothing in
silence, and a corpus fixture that writes the second shape, are the user's,
with the agent's recommendation. **The user ruled every part as recommended
the same day, and W24 is fixed**: each shape refused at load, one line per
slot in Ein's spelling, and the compiler a relation parameter a rule binds to
a nested fact; the ruling's four filed and fixed, the two compile-negative
fixtures moved to `broken/load/`, and the register reads 96 fixed, 2 open
(both deferred) and 1 not-wrong. **S1e2.2.24 is done** (`3ab9a231`), the
pages that said what a premise and a guard's argument may be corrected.
**S1e2.2.25 started the same day** and paused on a finding of its re-take:
past `--region-limit` on a program the static check flags, `--minimal` and
`--maximal` label every member met as on their line, where describe
certifies one minimal state and says the maximal line is not known. The user
ruled it the same day (Q-M1e2.19): a new stage, S1e2.2.26, files it as W27
and fixes it before the close. **Its rows are filed** (T1e2.2.25.1): every
item re-taken in either build, W-10 and W-15 shown by the stage's probes,
eight rows `open` and `deferred` under the register's schema `/10`, and the
register reads 96 fixed, 10 open (every one deferred) and 1 not-wrong.
**S1e2.2.25 is done** (`eb882d53`): F21, F22 and M1f's S1f.10.6 each list the
rows they own, and the acceptance below names every deferred row. The
finding is
[Q-M1e2.19](open_questions.md#q-m1e219--what-do---minimal-and---maximal-answer-past-the-limit),
and [S1e2.2.26](p1e2.2_wrong_answers/s1e2.2.26_line_modes_past_the_limit.md)
(W27) is specified, one stage and one day more. **S1e2.2.26 started the
same day** (T1e2.2.26.1): every mode re-taken past the limit on every flagged
program and register witness and the monotone routes under six budgets, in
either build; no monotone route mislabels and describe certifies nothing the
oracle does not; the smallest witness banked, W27 filed as four `open` rows
under the register's schema `/11`, and the register reads 96 fixed, 14 open
and 1 not-wrong; the fix prototyped from describe's own rule, every move a
line mode on a flagged program past the limit. **W27 is fixed the same
day** (T1e2.2.26.2): past the limit `--minimal` lists the members describe
certifies on the minimal line and every `--maximal` question lists none, the
maximal line not known, each counting the other members met, held by a
corpus-wide test; what moved is step 1's list, and the register reads 100
fixed, 10 open (every one deferred) and 1 not-wrong. **S1e2.2.26 is done**
(`c21f93fc`), the pages that said what a line mode answers past the limit
corrected. **P1e2.2 is complete** (2026-11-05): every register row is
`fixed`, `not-wrong`, or `open` and deferred to its owner. **Next: P1e2.3's
S1e2.3.8, S1e2.3.9 and S1e2.3.10, then the inventory re-taken and shown to
the user. The milestone's close waits for the user's go-ahead.**

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
rulings, S7 and S8 pinned, Q-M1e.5 and Q-M1e.17 closed. The user added three
stages the same day. **S1e2.3.5** (test scratch left in `/tmp`) and
**S1e2.3.7** (four lines that misname what they print, N1–N4) **are done**,
and so is **S1e2.3.6** (the nightly workflow, last): every nightly step is
green locally under `./run_tests.sh --nightly`, a new runner that
`ein-cli/tests/nightly_steps.rs` holds to `nightly.yml`. **The phase closed
2026-11-03**, its acceptance re-taken line by line
([§ Acceptance for the phase, as met](p1e2.3_surfaces_that_misreport_the_answer/README.md#acceptance-for-the-phase-as-met)).
The nightly is green here, not yet on GitHub: the user pushes, and the first
scheduled run after the next push is the confirmation.

**Re-opened 2026-11-05** for two stages on the user's ruling of 2026-11-04:
[S1e2.3.8](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md)
(N5, the fact printer) and
[S1e2.3.9](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md)
(N6, a core the run did not take). On the inventory's ruling the same day
([Q-M1e2.16](open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items) (3)), a dumped state's provenance joins S1e2.3.8, and
[S1e2.3.10](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_refuted_by_and_report_rows.md)
takes N7 (`refuted by` names a rule the core does not use) and N8
(`--json-report` drops `outside` and `types`). The three run after P1e2.2's
S1e2.2.25, and since 2026-11-05 after S1e2.2.26 (Q-M1e2.19). **S1e2.3.8
started 2026-11-05 and paused 2026-11-06** after its step 1: a claim and a
query's projection classes compare printed text, so the one printer moves
an answer on three probes and on no corpus program, which the stage's brief
does not allow; put to the user as
[Q-M1e2.20](open_questions.md#q-m1e220--does-a-claim-compare-values-or-their-printed-text),
ruled (a) the same day. **W28 filed** (2026-11-06): three rows, schema
`/12`, 100 fixed, 13 open and 1 not-wrong. **S1e2.3.8 done** the same day
(`712f23fd`): one printer, a value as Ein text the loader reads back to the same
value on every surface; a `--dump-answer` state file's provenance a comment,
and every state file reloading to its state by value, held over every corpus
program; W28's three rows fixed, 103 fixed, 10 open (every one deferred) and
1 not-wrong. **S1e2.3.9 done** the same day (`b6139e62`): a member of N a run
past `--region-limit` met, which the run keeps as its two bits, is written
with a `null` core and rules and `core_status: "not-kept"`, and `--trace`
says why, where both wrote an empty core that read as *refuted by nothing*
(`lattice/06_friends_tables.ein`, 68 of 68); no run that did not stop
prints an empty core, and no answer moved. **Next: S1e2.3.10, then the
inventory re-taken and shown to the user. The milestone's close waits for
the user's go-ahead.**

## What it inherits from M1e

| question | here |
|---|---|
| [Q-M1e.8](../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set): `exhausted` certifies the lattice, not the model set | W1, S1e2.2.1 |
| [Q-M1e.9](../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent): `dead` and `absent` | W3, S1e2.2.5; the language ruling moved here from M1f's S1f.10.8 on 2026-11-01, as P1e2.4's S1e2.4.1, and was ruled *forbid* on 2026-11-02 |
| [Q-M1e.12](../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object): the untyped blind rung | C1, S1e2.2.10; ruled (a) 2026-11-03, C1 not wrong; built 2026-11-04 |
| [Q-M1e.15](../../docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported): the alternatives cap | W10, S1e2.2.11; closed 2026-11-04 by fix (3) on the user's rulings on Q-M1e2.11, the residual `W10/alt-cap-shared` deferred to F22 |
| [Q-M1e.16](../../docs/history/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one): the binding key | W7, S1e2.2.7; closed 2026-11-04 by fix (1), the binding key's activator half the compile key's every argument |
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
  **The one exception, the user's ruling of 2026-11-03** (*"(2) Amend
  acceptance"*): a row whose reading the user deferred to a followup stays
  `open`, reviewed `deferred`, with that followup as its `owner`, and the
  milestone closes naming it. W8's rows were those, owned by
  [F21](../followups/f21_non_monotone_programs_decided.md)'s D2; since
  2026-11-04 one is, `W8/features-07`, because S1e2.2.5's F2 (*via*, the
  user's ruling of that day) refuses `stale-naf-fork` and fixes its row by the
  refusal. **A second since 2026-11-04**: `W10/alt-cap-shared`, owned by
  [F22](../followups/f22_exact_core_past_the_cap.md), on the user's ruling
  of that day on [Q-M1e2.11](open_questions.md#q-m1e211--which-derivations-is-the-unsat-core-the-smallest-over-and-may-a-config-flag-change-it)
  (i)(b): a saving between two facts behind a full alternatives list, which
  no bounded list keeps; S1e2.2.11 fixed W10's first witness.
  **More since 2026-11-05**, on the user's ruling on the inventory
  ([Q-M1e2.16](open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items) (5)): the rows
  [S1e2.2.25](p1e2.2_wrong_answers/s1e2.2.25_the_deferred_rows.md) filed for
  the live wrong answers that already had an owner and no row, each `open`,
  `deferred` to its owner, and named at the close. **The deferred rows, as
  of S1e2.2.25** (2026-11-05), ten:
  - [F21](../followups/f21_non_monotone_programs_decided.md): `W8/features-07`,
    `W8/f07-rules-swapped`, `W8/fifo-gap`, `W8/corner-house`;
  - [F22](../followups/f22_exact_core_past_the_cap.md): `W10/alt-cap-shared`,
    `W10/fork-delta-order`, `W10/closure-shared-given`,
    `W10/second-preparation`, `W25/guard-only-second`;
  - M1f's [S1f.10.6](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md):
    `W26/oblig-derived-activator`.
- **No wrong answer at default settings.** No lever changes an answer the
  engine *certifies*. A lever may change cost, and may decline to certify
  (`exhausted = false`, said why), and that is all.
- **Every contested case has a ruling**, the user's, written where the
  definition lives (`solution_semantics.md`, `absent_semantics.md`), not only in
  a stage file, or the user's deferral of it to a followup, written in the
  followup (W8: F21's D2, 2026-11-03).
- **The inventory is re-taken at the close**, and shown to the user before
  the close starts (the user, 2026-11-05: *"stop before close m1e2, I want to
  see inventory of known yet incorrect results, known solution gaps and their
  intersections with monotone/non-monotone lattices and region"*). **Since
  the user's ruling of 2026-11-05** ([Q-M1e2.16](open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items) (6)) the re-take is
  two things. **The register's tests**: `wrong_answers.rs` runs every row's
  witness in today's spelling, holds an `open` row to its wrong answer and a
  `fixed` one to its right one. **The inventory's sweep**: every corpus program
  solved and checked against the oracle, by the scripts kept in
  [`inventory_2026-11-05/`](inventory_2026-11-05/) (`sweep_describe.py`,
  `utils/region_census.py --all`, `sweep_cores.py`, `classify.py`), with
  [`inventory_2026-11-05.md`](inventory_2026-11-05.md) re-written from them.
  A new wrong answer found then becomes a register row and is not waved
  through. Until that ruling the re-take was
  [`probes/capture.sh`](probes/capture.sh)'s W lines and the corpus sweeps of
  [inventory § 4](inventory.md#4-reach--corpus-sweeps-run-for-this-inventory),
  which cannot run as written: the W lines spell the walk's options, most
  refused since S1e2.1a.5's switch (`-e`, `-m`, `-L`, `-K`), as P1e2.3's
  S1e2.3.4 reported on 2026-11-03, and the sweeps read the walk.
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
  contested case have a ruling. W8's two rows meet both head on. **Ruled
  2026-11-03**, the user at the revisit: the acceptance is amended, and W8's
  rows stay open with F21 as owner (§ Acceptance). **One row since
  2026-11-04**: S1e2.2.5's F2 (*via*) refuses `stale-naf-fork`, so
  `W8/stale-naf-fork` is fixed by the refusal and `W8/features-07` is the
  row the milestone closes naming. The two ways were: read W8
  under today's documented node, or amend the acceptance
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
- [`inventory_2026-11-05.md`](inventory_2026-11-05.md): the inventory taken
  before the close, at `cdcc3994`, with its probes, findings, scripts and
  `corpus.csv` in [`inventory_2026-11-05/`](inventory_2026-11-05/); what the
  close re-takes.
- [F23](../followups/f23_diagnostics_in_eins_words.md): the diagnostics and
  spellings this milestone found and did not fix.
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
