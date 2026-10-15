# M1e2 — Correctness: every known wrong answer

**Estimate:** ~12 weeks: 3 estimated phases and 28 stages, **58.5 days** of
stage estimates (P1e2.1 12, P1e2.1a 19, P1e2.2 27.5). One more phase,
P1e2.1b, has five stages named and none estimated, and one phase is proposed
and not specified.
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
unread cases are read against it.

**Then the oracle's first question split the definition in two.** A node can
be built on root's fixpoint or from scratch, and the two differ exactly where
adding a fact can take a conclusion away. The user moved that case into a
phase of its own, [P1e2.1b](p1e2.1b_the_non_monotone_lattice/README.md),
on 2026-10-15. P1e2.1a finishes the monotone lattice and *detects* the
non-monotone one. P1e2.1b decides what R is there, and its first finding is
already in: a hole needs no `absent`, because an obligation's trigger is the
same kind of read.

## Phases

| ID | title | stages | est. | ends with |
|---|---|---:|---:|---|
| [P1e2.1](p1e2.1_seeing_the_answer/README.md) | Seeing the answer: dumps, and the answer rendered for review | 7 | 12 d | `utils/review_bundle.py` → `build/review/`: every case's problem, every run's every artefact rendered with every graph as SVG, the right answer and the diff; the register as a cargo test; Q-M1e.20 answered; **the review held** |
| [P1e2.1a](p1e2.1a_the_solution_region/README.md) | The solution region: R defined, computed, drawn and reported | 8 | 19 d | the answer redefined as a **region R** of the hypothesis lattice, with its minimal and maximal lines ([Q-M1e2.7](open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice)), for a **monotone** program; a non-monotone one detected by a static check and reported through one stub; an oracle that computes R by saturation alone for n ≤ 16; the whole lattice drawn with the walk overlaid; programs written for a chosen region (S1e2.1a.8); the language and the read-out speaking R; the engine checked against the oracle under every lever; **the review resumed** |
| [P1e2.1b](p1e2.1b_the_non_monotone_lattice/README.md) | The non-monotone lattice: what R is when adding a fact can take one away | 5 named | — | the site census; a node's state, fork or scratch (Q-M1e2.2, W8); holes (Q-M1e2.7 (d)), from `absent` and from obligation triggers; whether a refutation may rest on an `absent` (Q-M1e.9, W3); the stub replaced |
| [P1e2.2](p1e2.2_wrong_answers/README.md) | The wrong answers, one stage each | 13 | 27.5 d | every register row `fixed` or ruled `not-wrong`; no wrong answer at default settings; no lever that changes a certified answer |
| P1e2.3 | Surfaces that misreport the answer | — | *proposed* | below |

### P1e2.3 — Surfaces that misreport the answer (proposed)

*Proposed* means named and not specified, the
[M1f S1f.5.3](../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/README.md)
precedent. The survey found nine surfaces that state something false about a
run whose **answer** is right, and one panic a well-formed program reaches.
They are correctness issues and not wrong answers, so they are recorded here
with their witnesses ([inventory § 3](inventory.md#3-cases)) and wait for the
user to say whether they become a phase:

| id | what is false | settings |
|---|---|---|
| **S1** | `ein test`'s NOT CHECKED names two causes, and neither applies under the tree; `(not certified — pass --exhaustive)` is printed on runs that **did** pass `-e` (`ein solve -e -m 1 zebra2.ein`). Q-M1e.5's fourth instance plus a new site, `verdict.rs:200` | default (the hint) |
| **S2** | the `--trace` headline says *No solution — 12 commitments refuted* on `pets.ein`, which has 39 models. Pinned by a golden named `trace_unsat.md` | default |
| **S3** | `--json-summary` says `type = "Contradiction"` with a 6-fact core on the same truncated run | default |
| **S4** | `-H` and `--json-summary`'s `root.hypgen` disagree, and each is wrong on one file (56 vs 38 on `zebra2`; 81 vs 96 on `zebra2-minus-15-obligations`; `-H` says 0 candidates where the run enters 3) | default |
| **S5** | `-E=-3` prints `max-enterings (0)`; `-m=-3` is silently `-m 0` (Q-M1e.17) | lever |
| **S6** | *"(no query goal to project)"* printed for a model in which the goal simply has no row | default |
| **S7** | the displayed goal row is the first of an unsorted match, and moves under a permuted interner | default |
| **S8** | a core prints `(not Fact(relation_name='p', args=('A',)))` | default |
| **S9** | under the tree, `layers_explored` carries the deepest node (documented) | lever |
| **P1** | `ein render constraints corpus/fuzz_findings/kwpair-below-the-filter.ein` panics, exit 101, at `dot_util.rs:152`, on a program the loader accepts | default |

## What it inherits from M1e

| question | here |
|---|---|
| [Q-M1e.8](../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set): `exhausted` certifies the lattice, not the model set | W1, S1e2.2.1 |
| [Q-M1e.9](../../docs/history/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent): `dead` and `absent` | W3, S1e2.2.5, with the language ruling proposed to move here from M1f's S1f.10.8 |
| [Q-M1e.12](../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object): the untyped blind rung | C1, S1e2.2.10 |
| [Q-M1e.15](../../docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported): the alternatives cap | W10, S1e2.2.11 |
| [Q-M1e.16](../../docs/history/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one): the binding key | W7, S1e2.2.7 |
| [Q-M1e.21](../../docs/history/m1e_review_processing/open_questions.md#q-m1e21--a-rule-may-name-an-object-the-search-can-never-hypothesise-about): fresh constants | W4, S1e2.2.4 |
| [Q-M1e.20](../../docs/history/m1e_review_processing/open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable): two renderers unreachable | S1e2.1.3, answered (a) |
| [Q-M1e.5](../../docs/history/m1e_review_processing/open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) and [Q-M1e.17](../../docs/history/m1e_review_processing/open_questions.md#q-m1e17--three-int_arg-options-silently-reinterpret-a-negative) | S1 and S5 of the proposed P1e2.3 |

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
  waved through.
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
