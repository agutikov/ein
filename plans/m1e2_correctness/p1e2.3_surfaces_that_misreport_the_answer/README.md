# P1e2.3 — Surfaces that misreport the answer

**Milestone:** [M1e2](../README.md) (Correctness)
**Estimate:** 5 days, four stages: S1e2.3.1 2, S1e2.3.2 0.5, S1e2.3.3 1.5,
S1e2.3.4 1. Each stage file gives its basis.
**Status:** specified 2026-11-02; **started 2026-11-03**. S1e2.3.1's
T1e2.3.1.1 took its evidence at `32496b97`, after P1e2.1c closed, and put Q1
to the user the same day; the user ruled (a) that day, and **S1e2.3.1 is
done** (T1e2.3.1.2–.4, 2026-11-03): S4 is fixed. The other stages have not
started. The survey of 2026-10-14 named
the surfaces ([inventory § 1](../inventory.md#1-table)), and the milestone
README carried them as a *proposed* phase. On 2026-11-02 the user ruled that
they *become a phase*, after P1e2.1c, *specified when it starts*
([Q-M1e2.6](../open_questions.md#q-m1e26--does-p1e23-become-a-phase)). Later
the same day the user asked, verbatim: *"write P1e2.3 plans also"*. That
request moved the phase from proposed to specified. Every witness was re-taken
first, and the stages are written against what is still live.
**Depends on:** [P1e2.1c](../p1e2.1c_the_region_search/README.md), by the
user's order of 2026-11-02: P1e2.4, P1e2.1b, P1e2.1c, then this phase
([README § Phases](../README.md#phases)). Two stages read what P1e2.1c leaves.
S1e2.3.1's block calls the walk's generator, whose code S1e2.1c.5 keeps or
deletes. S1e2.3.3's forms print the states S1e2.1c.4's flags list. Each stage
re-takes its witness on the day it starts.
**Blocks:** the wrong-answer revisit,
[S1e2.1.7](../p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md) and then
[P1e2.2](../p1e2.2_wrong_answers/README.md), which the user put after this
phase. The revisit reads every case on a bundle rebuilt at that commit, and the
bundle's root pages print S4's block (S1e2.3.1).

---

## Why a phase

A surface here is a line the engine prints, or a field it writes, that states
something false about a run whose **answer** is right. No register row covers
it, because no answer moves. The milestone's rule still applies: a reader who
cannot trust what a run says about itself cannot check the answer either. The
survey found nine such surfaces and one reachable panic, S1–S9 and P1
([inventory § 3](../inventory.md#3-cases)).

S1e2.1a.5's switch (`fbcae3b2`, 2026-10-18) removed the walk's CLI path, and
with it most of the surfaces. Q-M1e2.6's note of that day read eight as gone.
The note of 2026-11-01 brought S4 back, found again by S1e2.1a.6's lever test.
The note of 2026-11-02 sent this phase one item that is not a misreport: the
forms printed after the read-out are outside the user's 100-line ruling. So
the phase starts from a re-take, not from the survey's list.

## The re-take, 2026-11-02

At `7b33c3ab`, on `ein.rs/target/release/ein` (features `einb, parallel,
snmalloc`). `cargo build --release -p ein-cli` was a no-op: the binary was built
from `88607b8a`'s tree, the last commit that touches `ein.rs/`. Commands run
from the repo root. `probes/` is [`../probes/`](../probes/).

*Live* means the same surface still states the false thing. *Moved* means the
false statement now sits on another surface. *Gone* means no surface states it,
and the row names what removed it and what fails if it comes back, or says
that nothing checks it ([`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md):
an absence is settled by naming the thing that checks it).

| id | the witness, re-taken | today | class | removed by, and what holds it |
|---|---|---|---|---|
| **S1** | `ein solve -e -m 1 -s examples/zebra2.ein`; `ein solve -n 1 examples/zebra2.ein`; `ein test -v probes/tree-owed.ein`; `ein test -v --region-budget 2 examples/branching/16_lookahead_two_step_off.ein` | `-e` is refused, exit 2, naming `--maximal`. `-n 1` says *"no — stopped as asked; each state shown is certified, nothing is said of the rest"*. The probe no longer loads: `:no-hypothesis` belongs on `(solve …)` since S1e2.1a.4. `NOT CHECKED` says *"the claim is about all of R, and this run is partial"*. Neither old sentence appears in `ein test -v examples tests stdlib` (81 held) or in `ein solve -n 1` over the 281 files below | **gone** | the switch, `fbcae3b2`. Held by `read_out_examples.rs` (four blocks say *stopped as asked*) and `region_modes::the_retired_options_are_refused_with_what_replaced_them`. Both sentences survive uncalled in the walk's library: `verdict.rs:200`, `expect.rs:153`, and `answer.rs:169`'s twin. Nothing checks them there. S1e2.1c.5 decides whether that code stays |
| **S2** | `ein solve --trace T.md examples/saturation/type-exclusivity/pets.ein`, and the same with `--region-budget 100` | `# Region trace`, headed by the read-out's rows: `states (k_s) 11 339`, `exhausted yes`. Cut: `16 (a lower bound: complete up to commitments of 1 candidate)` and *"no — route 1 stopped before layer 2 … past --region-budget 100"*. No *No solution* | **gone** | `fbcae3b2`, which also deleted `trace_unsat.md`. Held by `golden_trace.rs`'s real-run goldens (`trace_dead.md`, `trace_owing.md`, …) and `cli_semantics::a_stopped_run_does_not_report_r_empty` |
| **S3** | `--json-summary J` on the same two runs | `ein-summary/2`: no `type`, and `verdict` is `null` without `--single`. Bare: `region.exhausted` true, `k_s` 11 339. Cut: `exhausted: false`, `stopped.why: "budget"`, `empty: null` | **gone** | `fbcae3b2`. Held by `region_answer::the_summary_has_one_shape_on_every_arm_and_says_what_stdout_says` and `cli_semantics::the_json_summary_declares_its_schema_and_its_block_shape` |
| **S4** | `ein solve -n 1 -H F --json-summary J` on `examples/branching/02_one_dead_one_alive.ein`, `examples/zebra2.ein`, `examples/zebra2-minus-15-obligations.ein`, `examples/ein-bugs/wrong-answers/{arity3,owed-seats}.ein`; each config flag flipped in the program; `-L`, `-K`, `-y`; `EIN_OBLIGATION_CHOICE=off\|fail-first`. The census is [S1e2.3.1 § How it is measured](s1e2.3.1_the_summary_root_block.md#how-it-is-measured) | `-H` lists H, which equals the summary's own `region.universe.candidates`. `root.hypgen.emitted` differs from H on **71 of 212** files, in both directions: 30 against 48 on `branching/02`, 0 against 4 on `owed-seats`, 0 against 2 on `arity3`, 6 against 5 on `d3-unsat-core`. `root.facts` differs from root's fixpoint on **175 of 212**: 27 against 22 on `branching/02`, one `(__closed__ …)` written by `emit_closed` and four `(not h)` written by the kill cache. The block moves when the program flips either flag (21 files) and under `EIN_OBLIGATION_CHOICE` (32); `-L`, `-K` and `-y` move it on none, so the `config` echo says *false* beside a block computed with *true*. The same block is written by `ein oracle` and `ein test --json-summary` and by `--dump-answer`'s `answer.json`, and the review bundle's root pages print it | **live** at the re-take; **done 2026-11-03** | `-H`'s half went at T1e2.1a.5.2 (2026-10-16), when `-H` began to list H. The block went at **S1e2.3.1**, on the user's ruling (a) of 2026-11-03: it is root's fixpoint and H as every route starts from them, config-free, on all five writer paths (22 facts and 48 emitted on `branching/02`). Held by `summary_properties::the_description_is_coherent_on_every_corpus_cell` (`root.facts` against root's fixpoint, `root.hypgen.emitted` against H, `--monotone`'s block against the region run's) and `lever_invariance.rs`, which compares the block whole. `EIN_OBLIGATION_CHOICE` retired with it |
| **S5** | `ein solve -E=-3 examples/zebra2.ein`, `-m=-3`, then every count option at `-3` and `0` on `solve`, `test` and `oracle` | `-E` and `-m` are retired, exit 2, each naming its replacement. `-n`, `--region-budget`, `--region-limit` and `-j` refuse with a reason, exit 2. `-T=-3` aborts and prints `--max-time -3 s`, as typed. **`-v -g=0` and `-v -g=-3` log every node, as `-g 1` does, with no word** (14 node lines on `branching/05`). The clamp is `.max(1)` at `modes.rs:641`. `saturate --progress-every 0` disables instead | **moved**, to `-g` | the `-E`/`-m` half: `fbcae3b2`. Held by `region_modes::the_retired_options_are_refused_with_what_replaced_them` and, for `-n`, `cli_semantics.rs`. No test feeds a negative to `--region-budget`, `--region-limit`, `-j`, `-T` or `-g`. **S1e2.3.4** |
| **S6** | `ein solve examples/lattice/02_genuine_3set_death.ein` | class 1 of 2: *"4 states (1, 3, 4, 7) — the goal has no row: undetermined here"* | **gone** | `fbcae3b2`. Held by `read_out_examples.rs`, whose page holds this command's block. The old sentence survives uncalled at `answer.rs:317` |
| **S7** | `EIN_ID_FILES=D cargo test --manifest-path ein.rs/Cargo.toml -p ein-render --test id_order_invariance`, with D holding `corpus/fuzz_findings/d3-goal-row-order.ein` | 44 permutations over 35 ops: **0 answers differ**, and 1 moved only in the derivation it narrates | **gone** | the read-out sorts a class's rows and prints them all (T1e2.1a.5.2), and the walk's `trace[answer]` op went with the walk. The corpus sweep holds the class on every corpus file. **Nothing checks this witness**, which is not a corpus file |
| **S8** | `ein solve probes/nested-core.ein` | `unsat core (not (p A))`, `unsat core (q A)`, `refuted by boom` | **gone** | `fbcae3b2`: the region read-out prints s-expressions. **Nothing checks it**: the read-out page's one core is flat, `(loops A A)` |
| **S9** | `EIN_TRAVERSAL=tree ein solve -s F --json-summary J` against the same without, F = `owed-seats.ein` | stdout identical, timing rows aside; summaries identical. `ein-summary/2` has no `layers_explored` | **gone** | `fbcae3b2`: only the walk reads `EIN_TRAVERSAL` (`solve.rs:932`), and no command reaches the walk. Held by `lever_invariance.rs`'s environment cells (`EIN_TRAVERSAL=tree` over ten files) and `cli_semantics`'s `WALK_ONLY` (`nogood`, `traversal`, `writeback` never emitted) |
| **P1** | `ein render constraints corpus/fuzz_findings/kwpair-below-the-filter.ein`; then `render rules\|constraints\|program\|kb`, `solve` (bare, `--maximal`, `--minimal`, `--single`) and `oracle` over the 281 files | panics, exit 101, at `dot_util.rs:150:32`, *not a value node: KwPair*. **`ein render program` panics on the same file**: a door added at S1e2.1.3, after the survey. `saturate` and `solve` exit 0 on it. No other panic in the sweep | **live**, on two doors | **S1e2.3.2** |
| **S10** | `ein solve examples/saturation/type-exclusivity/pets.ein` with `--models grid`, `--models key`, `-p`, `-P`, `-f`; `ein solve -n 1 -H examples/branching/06_lookahead_on.ein` | the bare read-out is 124 lines. `--models grid` adds to 34 143 lines (2.2 MB), `-p` 178 858, `-P` 102 622, `-f` 136 639; `--models key` declines, 130. `-H` takes `branching/06`'s 22 lines to 230. No form reaches `answer.txt`: `--dump-answer` on `colors.ein --models grid -H` writes neither | **live**, and not a misreport: every line is true | named here, from Q-M1e2.6's note of 2026-11-02. **S1e2.3.3** |

The files of the sweeps are the 281 `.ein` files under `examples/`, `tests/`,
`stdlib/`, `corpus/fuzz_findings/` and `plans/m1e2_correctness/probes/`
(`find examples tests corpus/fuzz_findings stdlib plans/m1e2_correctness/probes
-name '*.ein'`). 212 of them write a summary under `ein solve -n 1`. The panic
sweep ran each command with a 20 s timeout (15 s for `solve` and `oracle`) and
counted exits 101, 134 and 139.

### What the re-take says that the plans do not

- **S4 is wider than the README's paragraph.** The paragraph names one
  program and 19 that move with a flag. The census finds `root.hypgen` wrong
  against H on 71 files and `root.facts` wrong on 175. On 155 of those 175 the
  excess is exactly `emit_closed`'s `(__closed__ …)` writes, a pre-pass no
  route runs. On 19 the kill cache's `(not h)` writes come on top. On one,
  `tests/stdlib/closure/01_infer_closure.ein`, the program derives one of the
  two markers itself. `EIN_OBLIGATION_CHOICE` moves the block on 32 files, not
  only on `owed-seats`.
- **Root's fixpoint on `branching/02` holds 22 facts, not 23.** The milestone
  README and `configuration.md` § 2.3 both say 23. The 23 is the block with the
  program's lookahead off, and it still counts `emit_closed`'s one write. The 22
  is `ein saturate`'s *facts (total)* after saturation. The `--dump-answer`
  state file for ∅ lists no `(__closed__ …)` fact.
- **P1 has a second door**, `render program`.
- **S5 moved, not gone.** Q-M1e2.6's note of 2026-10-18 read it as gone.
- **[`probes/capture.sh`](../probes/capture.sh) can no longer re-take the
  inventory.** Of its 49 `run` lines, 39 are refused as retired options and 2
  are load errors (`tree-owed.ein`). Its S4 lines grep for `root hyps`, which no
  command prints, and read the two files where the numbers agree, `zebra2` and
  `zebra2-minus-15-obligations`. The
  milestone's acceptance re-takes the inventory with this script at the close.
  This phase re-points its S and P lines (S1e2.3.4). The W lines are the
  milestone's, and the gap is reported to it.
- **AGENTS.md still says S2 and S3 are live**: *"Two surfaces still say the old
  word, the `--trace` headline and `--json-summary`'s `verdict.type`. They are S2
  and S3 of M1e2's proposed P1e2.3."* Both went on 2026-10-18.
- **The S4 row of the milestone README is stale on `arity3`.** It says `-H`
  shows 0 candidates where the run enters 3. Today `-H` lists 2, and
  `root.hypgen.emitted` says 0.
- **Two doc comments in `ein-cli/src/summary.rs` are stale.** They say
  `root_block` runs on the live event log, so its second root saturation is
  recorded. `build_region` passes it a silent log: `--events` on `branching/02`
  is 67 846 lines with `--json-summary` and without.

## Stages

**Grouped by mechanism, not by id.** Each live item has one cause, and a
stage takes one cause:

- **S4** is one function, `summary::root_block`, behind four writers and the
  bundle's root pages. Its three fields and the `config` echo disagree for the
  same reason, so they are one stage.
- **P1** is one function, `dot_util::value_label`, behind two doors, so one
  stage.
- **S10** is one rule. The read-out cuts its listings at 100 shared lines, and
  the six forms printed after it (`--models key|grid`, `-p`, `-P`, `-f`,
  `--diff-models`, `-H`) are not under it. One ruling and one stage.
- **S5** has no stage of its own. What `-g` should do with 0 and a negative is
  [Q-M1e.17](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e17--three-int_arg-options-silently-reinterpret-a-negative)'s
  ruling, so it goes with the inherited questions. Q-M1e.5 goes there too,
  because its fourth instance was S1, which is gone. The same stage closes
  the phase.

Gone items get no stage. The table above records each and closes it here.

| ID | title | est. | ends with |
|---|---|---:|---|
| [S1e2.3.1](s1e2.3.1_the_summary_root_block.md) | The summary's `root` block describes a generator no route runs (S4). **Done 2026-11-03**: ruled (a), applied | 2 d | the user's ruling (H, root's fixpoint, or neither) applied on all five writer paths. `lever_invariance.rs` stops scrubbing the three fields, and an identity in `summary_properties.rs` holds the block to what it describes. `configuration.md` loses its S4 paragraph |
| [S1e2.3.2](s1e2.3.2_a_keyword_pair_panics_two_views.md) | A keyword pair panics two render views (P1) | 0.5 d | `value_label` total, or the views filtering, as ruled. The witness is banked as a `regression` corpus entry whose `render constraints` and `render program` runs exit 0 |
| [S1e2.3.3](s1e2.3.3_the_forms_past_100_lines.md) | The forms after the read-out, past 100 lines (S10) | 1.5 d | the forms under the user's ruling, `--dump-answer` writing whatever stdout leaves out, and the review bundle still carrying every form uncut |
| [S1e2.3.4](s1e2.3.4_the_inherited_questions_and_the_record.md) | The inherited questions, `-g`'s clamp, and the record (S5, Q-M1e.5, Q-M1e.17) | 1 d | Q-M1e.5 and Q-M1e.17 ruled and closed in M1e's register, and `-g` following the ruling with a test. `capture.sh`'s S and P lines re-take this table. The links to the proposed § are re-pointed, and Q-M1e2.6 is closed |

**Order.** S1e2.3.1–.3 are independent. S1e2.3.4's rulings can be taken on
the first day, and its record task runs last. Every question below can be put
to the user at once, at the phase's start. Each one's evidence is already in
its stage file.

## Questions for the user

Each is framed in its stage file, with what every option costs. The
recommendations are labelled as mine.

| # | stage | the question | options | recommended |
|---|---|---|---|---|
| Q1 | S1e2.3.1, **ruled (a) 2026-11-03**, with `leftover` config-free and `EIN_OBLIGATION_CHOICE` retired | What does `--json-summary`'s `root` block describe? Under (b) or (c), does the schema go to `ein-summary/3`? And does `EIN_LEFTOVER`'s `leftover` block follow (sub-question 2)? | (a) H and root's fixpoint, both config-free, keeping the key set; (b) root's fixpoint only, with H left to `region.universe` and `-H`; (c) neither, and the block goes | (a), re-taken 2026-11-03 (it was (b) on 2026-11-02): `emitted` is H's size by construction, so (a) needs no new key, no extra work and no schema bump; `--monotone`'s summary has no other H. `/3` under (b) or (c). `leftover` follows, config-free, and `EIN_OBLIGATION_CHOICE`, which then changes nothing, retires |
| Q2 | S1e2.3.2 | A keyword pair below a form: render it or filter it? | (a) `value_label` renders `:key value`, as `ir_dot.rs` already does, for every caller; (b) each view filters keyword pairs at every depth; (c) (b) for `render constraints` and (a) for `render program`, the program as written | (a) |
| Q3 | S1e2.3.3 | Do the forms share stdout's 100 lines? | (a) they share them, and `--dump-answer` writes each form uncut; (b) each form gets its own 100 lines, with the rest in the dump; (c) they are exempt, because a form is asked for by name, and the rule says so; (d) a form goes to a file only | (a) |
| Q4 | S1e2.3.4 | Q-M1e.5: is *experimental* a licence to ship a false surface? | (a) answered by the ruling of 2026-10-16 (*no two commands may disagree; every answer agrees with the oracle*), so no; close it, and hand the learning half to S1e2.1c.4 and M1f's S1f.10.7; (b) keep it open for P1e2.1c's tree route, the next surface that could be experimental; (c) close it as moot, with no rule | (a) |
| Q5 | S1e2.3.4 | Q-M1e.17, now on `-g`: what does a count of 0 or below mean? | (a) refuse below 1, as every other count option has since the switch; (b) 0 means no node lines, as `saturate --progress-every 0` does, and a negative is refused; (c) keep the clamp, and say so in the help | (b) |
| Q6 | S1e2.3.4 | S7's and S8's witnesses are gone, and nothing checks them. Bank them? | (a) pin both (a read-out cell for `nested-core.ein`; `d3-goal-row-order.ein` in the id-order sweep), about a quarter of a day; (b) close them on this re-take alone | (a) |

## Candidates found on the way, not added

These came up during the re-take. None is in a stage. The user decides
whether any joins one, and S1e2.3.3 is where the first two would go.

- **N1. `-f`'s header names `:hrules`.** It prints *"positive
  hypothesis-relation facts; :hrules ['color-loc', 'drink-loc', …]"* on
  `zebra2-obligations.ein`, which declares no `:hrules`. The list is the
  candidates' relations (`modes.rs:450`, `factdump.rs:35`), written in a
  Python list's spelling.
- **N2. `-p` and `-P` call a commitment *derived*.** Their header says
  *"derived facts"* over a state's facts beyond root. On a six-line scratch
  program, state 3's two facts are `(not (p A))` and `(q A)`, and `(q A)` is
  the commitment, not a derivation. `read_out.md` § 8.6 defines `-p` as the
  facts beyond root.
- **N3. `ein saturate --max-steps`'s limit line.** It names
  `Saturator._last_firing`, which no crate defines. It spells the firing as
  `Firing(rule='symmetric', … Fact(relation_name='co-located', …))`, S8's
  spelling on another surface (`saturate.rs:592`). It exits 0.
  `defined_behaviour.md` § 4's row for the step limit names a
  `--max-firings` budget that no command spells, with exit 1 and
  `SaturatorStepLimitError`.

## Acceptance for the phase

- **No live row.** S4, P1 and S10 are fixed as ruled, each with the test that
  holds it. S5 follows Q5. Re-taking this page's table on the day the phase
  closes finds every row *gone*, with what holds it named.
- **S4:** the summary's `root` block says what Q1's ruling says, on `ein solve`,
  `ein solve --monotone`, `ein oracle`, `ein test` and `answer.json`. Every config flag, CLI lever and
  environment lever leaves it unmoved on every program `lever_invariance.rs`
  sweeps, with the three fields no longer scrubbed. The bundle's root page
  agrees with its own sentence that root is never written.
- **P1:** no render view exits 101 on a program the parser accepts. The
  witness is a corpus entry, so `corpus_shapes.rs` runs every view over it.
- **S10:** the forms obey Q3's ruling. Whatever stdout leaves out is in the
  dump, and the review bundle still carries every form uncut.
- **Q-M1e.5 and Q-M1e.17** are ruled and closed where they live, and Q-M1e2.6
  is closed.
- **The re-take can be re-taken.** `capture.sh`'s S and P lines run today's
  spellings and write this table's evidence. Under Q6 (a), S7 and S8 are
  pinned.
- **Every golden a stage moves is named in its file before it moves.** An
  unexplained corpus diff is a stop, not a re-bless. `./run_tests.sh` is green
  at every stage boundary.
- **No answer moves.** The register, `read_out_examples.md` and every corpus
  read-out are byte-identical before and after each stage, except where a
  stage names the line it moves.

## What it inherits

| from | what | here |
|---|---|---|
| [Q-M1e2.6](../open_questions.md#q-m1e26--does-p1e23-become-a-phase) | the question whether this is a phase, ruled 2026-11-02; its notes of 2026-10-18 (eight gone), 2026-11-01 (S4 back) and 2026-11-02 (the forms) | this page's re-take; S1e2.3.1; S1e2.3.3; closed in S1e2.3.4 |
| [Q-M1e.5](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) | *is experimental a licence to ship a lying surface?* Its fourth instance was S1, which is gone. `EIN_TRAVERSAL=tree` reaches no command, and `configuration.md` calls it the one experimental surface in the past tense | Q4, S1e2.3.4 |
| [Q-M1e.17](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e17--three-int_arg-options-silently-reinterpret-a-negative) | three `int_arg` options clamp a negative. `-m` and `-E` are retired. The clamp lives on in `-g` | Q5, S1e2.3.4 |
| [the milestone README's S4 paragraph](../README.md#p1e23--surfaces-that-misreport-the-answer-proposed) | S1e2.1a.6's re-finding: `root_block` runs the walk's `hypgen::generate` on a fork, `root.facts` and `root.hypgen` move with the program's flags and `EIN_OBLIGATION_CHOICE`, and `lever_invariance.rs` leaves the three fields out. *"Whether the block should describe H, root's fixpoint, or neither is this phase's to rule."* | Q1, S1e2.3.1 |
| [S1e2.4.2's notes](../p1e2.4_what_the_switch_left/s1e2.4.2_models_key_declines.md#notes) | the forms are outside `read_out.md` § 4.1's 100 lines | S10, S1e2.3.3 |

## Connections

- [`inventory.md` § 3](../inventory.md#3-cases): each surface's witness as
  the survey took it, at `71dd237`. It is carried as written, and this page's
  table is what holds today.
- [`read_out.md`](../p1e2.1a_the_solution_region/read_out.md): the six rules
  every surface follows (§ 3), the 100 lines (§ 4.1), the summary (§ 8.1),
  and the other surfaces (§ 8.6).
- [`configuration.md` § 2.3](../../../docs/kernel/configuration.md): where S4 is
  written down today as the one exception to *a lever changes nothing
  printed*.
- [`corpus/fuzz_findings/kwpair-below-the-filter.md`](../../../corpus/fuzz_findings/kwpair-below-the-filter.md):
  P1's finding, and the two fixes it named.
