# S1f.21.20 — what the first uses of `open` found

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator, under the user's word to run the phase to its end on defaults
([the README](README.md#the-word-to-run-to-the-end)); **not started**.
**Waits on:** [S1f.21.18](s1f.21.18_the_hrule_form_removed.md), which changes the same files of the engine.
**Writes:** `ein-infer/src/` (`region.rs`, `open.rs`, `oblgen.rs`, `explain.rs`), `ein-core/src/kb.rs` for the
pass's cost, `ein-cli` (the order of stderr's lines, `ein saturate`'s counts), their tests; the pages and goldens
that move with a report's words.

## What it is

[S1f.21.13](s1f.21.13_the_search_and_rules.md) was the first stage to write a program with `open` facts after
[S1f.21.12](s1f.21.12_hypotheses_as_facts.md) built them, and met three defects of that build, none of which moves
a corpus answer. This stage fixes them. The coordinator's cost measurement after S1f.21.18 added two tasks, on
what the pass costs at a state and on the gate's build of one program.

## Tasks

1. **[V-61](findings.md#v-61--a-written-open-fact-breaks-the-answer-dumps-round-trip): the empty commitment's state
   is root's.** A written `(open F)` is held at root alone, and the node a run builds for ∅ is a fork that drops it,
   so a saved ∅ does not load to the state the run saved. **The reading taken, a default
   (Q-M1f.636)**: ∅ is root, so its node holds root's `open` facts, the written ones among them; every node above
   it is a fork and holds none of the written ones, as the user ruled. The other reading: root too drops a written
   one once it is read into $H$, so no state holds it. The witness beside S1f.21.13's record becomes a test, and
   `answer_round_trip.rs` holds a program that writes one.
2. **[V-62](findings.md#v-62--a-note-of-the-mechanisms-precedes-a-flagged-programs-warning): the `WARNING` is
   stderr's first line**, before a load's `note:`; `non_monotone_warning.rs` holds it on the witness.
3. **[V-63](findings.md#v-63--three-reports-do-not-see-what-the-open-writers-did): three reports.** `ein saturate`
   counts the firings of the saturation that resumes after the `open` pass. The rung's name says what built $H$
   where a rule's candidates pay what root owes, and not `stuck`. A core lists the written `(open …)` facts its
   derivation read.
4. **Then the example in the language's form**, if the three are fixed: S1f.21.13 kept its program as rules in the
   survey's spelling to keep the gate green
   ([Q-M1f.632](../open_questions.md#q-m1f632--how-the-program-as-rules-writes-its-candidates)). Whether it is
   written with `open`, flagged and held by no suite, is the user's to say in review item 70: **it is left as it
   is here.**

5. **[V-64](findings.md#v-64--the-pass-that-writes-a-states-open-facts-costs-at-every-state): the pass runs at
   root, and above it only where a state is used.** Ruled by the user 2026-11-20
   ([Q-M1f.655](../open_questions.md#q-m1f655--where-the-pass-that-writes-a-states-open-facts-runs)): *"per-state open pass - we can make it optional, by default run only on root to make an H-set, then only on states that will be printed or saved or used eather way."*
   - **At root** the pass runs as today and makes $H$.
   - **Above root** it runs at a state that is printed or saved (the read-out's states, `-p`, `--dump-answer`,
     `--dump-lattice`, the oracle's dump, a render of a state), and at a state that uses its `open` facts: where
     a rule of the program reads an `open` fact, every state the search builds.
   - **At every state, as before**: what the state owes, its verdict. $R$ needs it, and it is not the writing.
   - **The check in flight** is made where the pass runs, and **a command-line flag runs the pass at every
     state**, the user's ruling of
     [Q-M1f.656](../open_questions.md#q-m1f656--the-check-in-flight-once-the-pass-runs-only-where-a-state-is-used): *"also can have cmdline flag that enables firing H-set for every state to detect non-monotonicity in-flight"* The flag's name is the stage's default. It changes no
     $R$: the lever invariance holds every corpus program to one answer and one printed state with it and
     without, and the options' census names it.
   - Measured again with [the page's script](costs/four_engines.py), a fifth point beside the four.
6. **[V-65](findings.md#v-65--the-gates-checked-build-costs-four-times-the-release-on-one-program): the gate's
   checked build on one program, bounded.** Find the debug check that makes `equation_two_unknowns.ein` cost 3.8
   times as much on the gate profile as on release. If it can be asked once a state, or under a smaller bound,
   and still check what it checks, do so; else record it.

7. **[V-39](findings.md#v-39--the-cost-of-the-contradiction-as-rules-is-not-measured): the agenda's tables kept,
   not built again at every state.** [The investigation](costs/README.md#why-the-contradiction-as-rules-costs)
   finds 37 to 43 % of what the contradiction as rules added in `Saturator::resume`, which starts every state's
   saturation with `first`, `at_once`, `end_plans` and `end_index` empty and builds them over every plan. Keep
   them with the snapshot a state resumes from, and at a state add only what its own facts add (a negation the
   commitment writes can make another plan's conclusion one side of a pair). **What it may not change**: the
   order of any firing, so no golden moves, no core, no trace. `ein-infer/tests/agenda.rs` holds the agenda.
   Measured with the page's script and by `perf stat -e instructions:u`, which the machine's load does not move.
   Not this task: one instance a relation in place of two (V-37), and an index of the instances by the relation
   under `not`, which are the user's call.

Task 6 is bounded: it ends at what it found, and is owed no number.

## What must hold

Every corpus answer as before; every mode against the oracle; `answer_round_trip.rs`, `non_monotone_warning.rs`;
`08-read-out.md` in the same commit as a row that moves; the gate on the last commit.

## Record

Run 2026-11-20 in a worktree fast-forwarded to `master` at `8c5d1bb0`, then rebased onto `master` at its tip; one
release build of the worktree for every number below, the gate profile's for the tests. Tasks 5 and 7 came while
the stage ran: task 5 as the user ruled it (Q-M1f.655, Q-M1f.656), task 7 on the user's question about the cost of
the contradiction as rules. Scratch named `build/s20/…` is the worktree's, not the tree's.

### Commits

| commit | task | what |
|---|---|---|
| `1e2a7478` | 2 | the `WARNING` line before a load's `note:` |
| `39168e86` | 1, 3 | the empty commitment's state is root's; `features/24_written_hypotheses.ein`; the three reports |
| `e2676c28` | 5 | the pass at root and where a state is used; `--in-flight`; the decisions Q-M1f.655 and Q-M1f.656 |
| `0b51c6c8` | 7 | the agenda's tables kept with the snapshot |
| `0a30e8fe` | 6 | the gate build's check of `compile_all`'s shortcut |
| `3a094688` | 5 | the check in flight at a printed state too, folded into every surface (Q-M1f.656 as ruled) |
| this record | | plans only |

The hashes are the branch's after its rebase onto `master` at `dfc5f3a0`.

### 1. The empty commitment's state is root's (V-61)

**Built.** `ein_infer::region`'s `fork_of`, the one fork every reader of a node starts from: where nothing is
committed (no fact and no saved file's floor) the fork holds the `open` facts the program wrote
(`open::written_by_program`), each with its source provenance, as root does; a fork that commits a fact holds none
(Q-M1f.569). Root's snapshot holds them, so the resumed saturation reads none of them as new. `saturated` (every
route's node, the oracle's, every `Nodes` reader) and `with_fixpoint_supported_within` (the structure's) read it;
`split_in_fork` returns the first fixpoint's key where the rest is empty, so a split of ∅ is ∅'s state. Every route
reads ∅ the same way because they share that one function.

**What it fixes, seen.** On the witness [`as_rules_over_open.ein`](s1f.21.13_the_search_and_rules/as_rules_over_open.ein)
the node saved for ∅ holds 18 facts, root's 18, and the core of its `:ein inconsistent` probe
(`build/s20/word-in-a-world.ein`) reads `(before (s a) (s b))`, `(open (s a))`, `(open (s b))` where it read the
first alone (V-63's third report, which the same change fixes at ∅).

**In the corpus.** [`examples/features/24_written_hypotheses.ein`](../../../examples/features/24_written_hypotheses.ein):
the program writes `(open (s a)) (open (s b))`, no rule reads `open`, a clash kills the pair; monotone, H the two,
R = {∅, {(s a)}, {(s b)}}. A region claim and a query's claim, both what `ein oracle` reads. Its corpus entry runs
`solve`, `test`, `saturate`, `saturate --dump`, `solve --single`, `-n 2`, `--maximal`, `--minimal`, `--count`,
`render lattice` and `oracle --dump`, every cell exit 0; the catalogue's `features/` row has a sentence for it; the
shape golden gained its 40 lines and no other line moved, under `EIN_CORPUS_SLOW=1` too. `answer_round_trip.rs`
(9 shards) holds its dump; `explore.rs` holds every mode to the oracle; `alive_invariant.rs` passes (`(thing a)
(thing b)` are written so `a` and `b` are named at top level, as `18_two_candidates_as_rules.ein` does).
`open_facts.rs`: `the_empty_commitments_state_is_roots` (a flagged program that writes and reads its hypotheses: ∅'s
node is root's state fact for fact, the written facts keep their source, a node above ∅ holds none of them), and
`a_fork_drops_every_open_fact_a_written_one_among_them` says of ∅ what is now true.

### 2. The `WARNING` is stderr's first line (V-62)

**Built.** The hypothesis mechanisms' `note:` lines are kept at load and said when the run ends, before the load's
`warning:` lines (`ein-cli`'s `common::keep_notes`, `say_warnings`), so a flagged program's `WARNING`, said once
the static check has read root, is first on every command. `non_monotone_warning.rs`:
`the_warning_comes_before_a_loads_note`, on a test-local flagged program whose load notes `:no-search` beside a rule
that concludes a hypothesis, under `solve`, `solve --no-search`, `oracle`, `show hyps` and `render lattice`; only the
order of the lines is held. `08-read-out.md` § 10's row says when the note is said.

### 3. Three reports (V-63)

- **`ein saturate` counts the firings after root's `open` facts**: `open::at_root` takes a firing sink, and
  `saturate`'s root hands it the run's list and tally. `features/23_a_rule_reads_open.ein` prints `8 firings` and
  `noted 2 / 2` where it printed 6 and no `noted`. `cli_semantics.rs`:
  `saturate_counts_the_firings_after_roots_open_facts`.
- **The rung is `union`, not `stuck`, where a rule's candidates survive beside an obligation that offers none**
  (`oblgen::Beside`: whether the other writers wrote, and whether a candidate of theirs survived the filters).
  `stuck` is a state that owes and to which no writer gave a candidate. Over the corpus (`ein show hyps` of every
  file, point D's binary against this one), one note moved: `lattice/18_two_candidates.ein`, `stuck` → `union`.
  `obligation_rung.rs` holds it; `05-events.md`'s and the reasoning-in-code page's rows of the two words say so.
- **A core lists the written `(open …)` facts its derivation read**, at ∅, by task 1. Above ∅ see Q-M1f.638.

### 4. The example

Left as it is, as the plan says: `18_two_candidates_as_rules.ein` stays in the survey's spelling; whether it reads
`open` is review item 70's.

### 5. The pass where a state is used (V-64; the user's rulings Q-M1f.655, Q-M1f.656)

**Built.** `open::Pass`, chosen by the node function (`region::node_in_region`): `Tally`, what the state owes alone,
at a node of a search nothing prints; `Write`, the writing and the check in flight with it, at a state printed or
saved — every `region::Nodes` reader (the read-out's states and their forms, `-p`, `-P`, `-f`, `--dump-answer`, the
trace, `EIN_LEFTOVER`'s probe, a member of N's core), `ein oracle --states`' nodes, a saved file's node — at every
node of a program a rule of which reads `open`, and at every node under `--in-flight`. Root is as before. What a
state owes, the tally, is read at every state. What the read-out's states find in flight is folded into the
answer it reports (`Nodes::in_flight`, `Monotonicity::with_in_flight`), and `ein oracle --dump` writes
`region.json` once the read-out has read them, so the read-out, the summary and the dump report one finding. `--in-flight` is on `solve`, `test`, `oracle` and `render lattice`
(`cmdline::in_flight`); `ein`'s `run` wraps the command in `open::at_every_state`, a thread-local each route reads
as it builds its axes (`Axes::in_flight`), as `region::to_quiescence` is read. The decisions:
[Q-M1f.655](../../../docs/decisions/engine/Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md),
[Q-M1f.656](../../../docs/decisions/cli/Q-M1f.656-a-flag-runs-the-pass-at-every-state-and-checks.md), each the
user's words and what holds today; Q-M1f.574's file says where the run checks.

**Held by invariance.** `lever_invariance.rs`' `no_config_flag_moves_a_line_of_any_mode` sweeps `--in-flight`
beside the two config flags, in process, on every corpus program and regression witness whose R is known, under
the sixteen questions: the read-out uncut, its stderr and exit, and the summary, the same but for the `in flight`
row and `monotonicity.in_flight`. On the gate's run: 725 programs, 29 967 cells, no cell moved; the option found what
the default does not on `lattice/synth/maximal_not_top.ein` and `ein-bugs/wrong-answers/oblig-fresh-name-two.ein`,
both flagged by a site already, which the test asserts as the evidence that it reached the run. `open_facts.rs`:
`a_hypothesis_first_held_above_root_is_found` under the option and its absence by default;
`the_pass_runs_where_a_state_is_used`, the seats cube's status with the pass at every state and without. The shape
golden did not move, per commit and under `EIN_CORPUS_SLOW=1`. `06-config.md`'s census: 99 options, the four
parsers' counts, and a class of its own in § 3.3, *Does it change the answer: no*; `help_surface.rs`' counts and
the help golden.

**What moved on a page.** `09-read-out-examples.md` (generated) lost X9's three `in flight` rows:
`lattice/06_friends_tables.ein` is flagged by a site, and by default its run checks only the states it prints, of
which none holds a hypothesis root does not; `--in-flight` finds `(sits Dan S1)` again. The rows' table in `08-read-out.md` already said no
example holds one.

**The cost**, measured with a copy of [the page's script](costs/four_engines.py) (`build/s20/costs/five.py`): point
D and this stage's release build back to back, and the same build with `--in-flight` on each run that takes it,
each on the corpus of its own tree, under `utils/bench_env.sh --cores PT` (16 P-core threads, 8 physical cores; AC;
governor `powersave`, turbo on). **The machine was loaded**: another stage's work ran beside it; load average 6.14,
8.89, 9.54 when it started and 8.54, 6.68, 7.82 when it ended, with one release build of this tree (half a minute)
during it. Point D's sums are within 4 % of the page's D, taken idle. Milliseconds, each entry's declared runs
summed; *S* is the build at `0a30e8fe`, tasks 1 to 3, 5 (before its last commit, which adds a set lookup per
hypothesis at a printed state), 6 and 7; *B* and the page's *D* are the page's, for reference; the last column is
`--in-flight` over the runs that take it, against *S* over the same runs:

| entry | B (page) | D (page) | D | S | S / D | S / B (page) | `--in-flight` / S |
|---|---:|---:|---:|---:|---:|---:|---:|
| `zebra.ein` | 382 | 880 | 876 | 819 | 0.94× | 2.14× | 1.06× |
| `features/04_open.ein` | 758 | 1 614 | 1 586 | 624 | 0.39× | 0.82× | 2.39× |
| `beyond_arith/equation_two_unknowns.ein` | 732 | 771 | 766 | 630 | 0.82× | 0.86× | 1.04× |
| `beyond_arith/send_more_money.ein` | 2 858 | 4 288 | 4 300 | 2 464 | 0.57× | 0.86× | 1.51× |
| `beyond_arith/send_more_money_written.ein` | 1 919 | 2 854 | 2 854 | 1 643 | 0.58× | 0.86× | 1.52× |
| `features/01_not_and_absent.ein` | 3 204 | 4 023 | 3 864 | 3 221 | 0.83× | 1.01× | 1.12× |
| `lattice/06_friends_tables.ein` | 5 595 | 7 209 | 7 288 | 5 266 | 0.72× | 0.94× | 1.28× |
| `lattice/07_two_pairs_two_tables.ein` | 4 478 | 5 023 | 5 044 | 4 433 | 0.88× | 0.99× | 1.09× |
| `lattice/14_seats_nothing_proposes.ein` | 293 | 5 058 | 5 023 | 4 462 | 0.89× | 15.25× | 1.12× |
| `zebra2.ein` | 1 910 | 1 962 | 1 965 | 1 916 | 0.98× | 1.00× | 1.03× |
| `zebra2-obligations.ein` | 1 781 | 1 823 | 1 801 | 1 780 | 0.99× | 1.00× | 1.02× |
| `zebra2-hints.ein` | 513 | 521 | 504 | 505 | 1.00× | 0.98× | 1.01× |
| `saturation/square-fwd/meetings.ein` | 1 679 | 1 927 | 1 864 | 1 613 | 0.87× | 0.96× | 1.13× |
| `saturation/square-fwd/houses.ein` | 1 604 | 1 871 | 1 844 | 1 578 | 0.86× | 0.98× | 1.14× |
| `branching/06_lookahead_on.ein` | 51 | 47 | 42 | 38 | 0.91× | 0.74× | 1.15× |
| `branching/07_lookahead_off.ein` | 47 | 50 | 42 | 34 | 0.81× | 0.72× | 1.44× |
| `arith/rat.ein` | 673 | 688 | 656 | 659 | 1.00× | 0.98× | 1.00× |
| `saturation/type-exclusivity/pets.ein` | 526 | 644 | 636 | 527 | 0.83× | 1.00× | 1.10× |
| `zebra2-minus-15.ein` | 12 610 | 13 511 | 13 539 | 12 982 | 0.96× | 1.03× | 1.06× |

Against the engine before the `open` facts (B), every entry is within 0.72 to 1.03 times but the two the union made
larger (V-48 and V-52): `zebra.ein` describes R in 353 saturations where B took 120, and
`14_seats_nothing_proposes.ein`'s oracle enumerates 16 candidates where B had 12. Against D the pass and the
tables give back up to 61 %, most on `04_open.ein`'s lattice render and the two `send_more_money` programs.
`--in-flight` puts back most of what the pass cost: `send_more_money.ein` 1.51 times the default, and the lattice
render of `04_open.ein` 2.39. The rows are `build/s20/costs/five.json`, the log `run.log`. The read-out's § 5.1 and § 10, `11-region-dump.md`'s
`in_flight`, `05-cli.md`'s two synopses, the design pages (`07-search.md`, `02-algorithms.md`, `06-saturation.md`)
and the reasoning-in-code page say where the pass runs; AGENTS.md's standing rule on `open` facts too.

### 6. The gate's checked build (V-65)

**Found**: `Engine::compile_all`'s `debug_assert!(self.would_not_grow(…))`. The walk of `rules × activators` is
skipped when the rule-application counter has not moved, and the debug build checked the skip against a full walk
(`activators_for` over every rule, every activator fact) at every call: every enqueue pass, and every
`match_ends` where a rule that ends a state has parameters, so twice a step where a node stops at its first
contradiction. A profile of the gate binary on `equation_two_unknowns.ein` had `compile_all` at 74 % of the cycles,
nearly all of it the check.

**Built**: the check first counts the facts of the rules' relations by relation (`Kb::n_facts_of`, one probe a
rule, kept apart from the counter it checks); where they are as many as the last walk read, they are the facts it
read, so the walk would compile nothing new and is not made. Elsewhere the full walk runs as before. Release builds
compute none of it.

| one `ein solve`, one P-core thread, task-clock | gate before | gate after | release | after / release |
|---|---:|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 1 365 ms | 447 ms | 313 ms | 1.43× |
| `zebra.ein` | | 333 ms | 295 ms | 1.13× |
| `features/04_open.ein` | | 4 327 ms | 4 017 ms | 1.08× |

### 7. The agenda's tables kept (V-39)

**Built**: `saturator::Tables`, read by `Saturator::snapshot` over the engine's plans: `at_once`, the plans of rules
that end a state and the relations they read (`ends_of`), and `FirstTables` — per plan whether it concludes the
holding side of a pair, whether it leads to an end, the relations it writes, the pairs, the relations the plans
make a held side of, and the relations whose holding side the snapshot's KB holds, read outside its `open` layer.
`Saturator::resume` takes them and adds to the held set only what the state's own facts add (its `open` layer, the
facts beyond the snapshot). A plan compiled at a state reads every table again there, as before. `refresh_first`,
`match_ends` and the snapshot share one reading of each table.

**No firing moved**: `ein solve -p` of every corpus file printed the same bytes before and after
(`build/s20/diffbins.sh`); `agenda.rs`, the id-order and job-count sweeps, the shape golden, `first_contradiction.rs`
and `corpus_cores.rs` passed unchanged, and the gate below.

Instructions, user space, `perf stat -e instructions:u`, one `ein solve` pinned to one P-core thread (pinned, the
count repeats to five digits; unpinned it moved by 10 % between two runs of one binary, the two core types' counters
summed):

| program | before | after | saved |
|---|---:|---:|---:|
| `saturation/type-exclusivity/pets.ein` | 4 185 M | 3 809 M | 376 M |
| `beyond_arith/equation_two_unknowns.ein` | 4 893 M | 4 213 M | 680 M |
| `beyond_arith/send_more_money.ein` | 14 150 M | 12 204 M | 1 946 M |

More than the page's part 1 (132, 214 and 451 M): the page's numbers are what the change from A to B added, at
B's count of plans and `not` facts, and here the tables are read at every state over every plan of this tree's
engine, where task 5 had left the pass at most states, so they were a larger share of what a state cost.

### Defaults taken (Q-M1f.636 to Q-M1f.644)

| id | question | default | the other reading |
|---|---|---|---|
| Q-M1f.636 | which state is ∅'s where the program writes `(open F)` | the plan's: ∅'s state is root's, the written facts among it; a node that commits a fact holds none | root too drops a written one once it is read into H, so no state holds it |
| Q-M1f.637 | the rung's word where a rule's candidates survive beside an obligation that offers none | `union`: the rules wrote beside an activated obligation, and H is the union, the obligations' share empty; `stuck` is a state that owes and to which no writer gave a candidate | `hypothesis-rules`, the writer that built H, though an obligation is activated |
| Q-M1f.638 | a core above ∅ whose derivation, made at root, read a written `(open F)` | it lists what the state holds: a fork holds no written `open` fact (Q-M1f.569), and the explanation grounds out a premise the state does not hold, as it does any; at ∅ the state holds them and the core lists them (`build/s20/core-above.ein`: the core of `{(s b)}` is `(s b)`) | list the written fact, a premise the derivation read, though the state does not hold it |
| Q-M1f.639 | `ein saturate --events` and the firings after root's `open` facts | counted and listed by the read-out, not narrated: the stream of `ein saturate` stays the first three states of `05-events.md`'s machine, and its root pass narrates nothing, as before | narrate them as `fire` lines after root's saturation |
| Q-M1f.640 | when a load's `note:` is said | when the run ends, before the load's `warning:` lines, on every command | before the run but after the `WARNING`, which needs the static check's verdict before any note |
| Q-M1f.641 | the flag's name and its commands | `--in-flight`, on `ein solve`, `ein test`, `ein oracle` and `ein render lattice`, every command that builds the states of a search; no `(config …)` field | `--open-every-state`, the mechanism's name; or a config flag |
| Q-M1f.642 | how what a printed state finds in flight reaches the run's surfaces | the read-out folds the finding of the states it prints into its answer's verdict, where the search found nothing; `ein oracle --dump` writes `region.json` after the read-out, with it; its stderr line is said at once, as every finding's | each surface reports what the search alone found, and a printed state's finding is the stderr line alone |
| Q-M1f.643 | which states are printed or saved | every `region::Nodes` reader (the read-out's states and forms, the dumps' state files, the trace, a member of N's core, `EIN_LEFTOVER`), `ein oracle --states`' nodes and a saved file's node; a state printed only as a count or a verdict, `--dump-lattice`'s nodes, `--events`' `enter` lines, `--count`, writes none; `ein test` as `ein solve`, a claim listing no `open` fact | every state a dump or a render names, `--dump-lattice`'s among them |
| Q-M1f.644 | the cheaper form of task 6's check | count the rules' facts by relation, and walk only where the count moved: no call the full walk would refuse passes, where `Kb::n_facts_of` agrees with the rule-application index, which `Kb::index_fact` keeps together | ask the full walk once a state, at the state's fixpoint, which misses a skip a later walk repairs |

### Findings outside the stage (V-66 to V-68)

- **V-66 — the `outside` row does not see a program's written hypotheses.** On a program whose H is its written
  `(open F)` facts alone (rung `hypothesis-rules`), `scope::outside` reads the relations the rules that conclude a
  hypothesis assert and not the program's written ones, so `features/24_written_hypotheses.ein`'s read-out has the
  row *outside s — no candidate: no hypothesis rule proposes it*, while H is two candidates of `s`. Its
  `union` arm reads both. No answer moves; the new example's shape lines bank the sentence.
- **V-67 — `04-hypothesis-generation.md` § 4.5 defines *stuck* by $H_{\mathrm{ob}}$.** The page's implication
  $\operatorname{stuck}(P) \implies R = \emptyset$ is false on `lattice/18_two_candidates.ein` under its own
  definition ($H_{\mathrm{ob}} = \emptyset$, root owes, R two states). The engine reads stuck by the whole H since
  this stage (Q-M1f.637), under which the implication holds. For the part's pages.
- **V-68 — the stream's state machine has no `fire` after root's obligation pass.** `05-events.md`'s diagram
  goes from `owe` to `monotonicity` or a node; on a program a rule of which reads `open`, `ein solve --events`
  narrates the resumed saturation's `fire` lines there, and at each node after its `owe` lines. A flagged program
  only.

V-69 to V-71 are not used.

### What the user must see

- Task 5 as ruled: by default the pass writes `open` facts at root and at printed or saved states only, and
  `--in-flight` checks every state; the default no longer finds in flight what only an unprinted state shows, which
  moved three rows of a flagged program on the generated examples page (X9).
- Q-M1f.642: what a printed state finds in flight is folded into the run's read-out, summary and `ein oracle`'s
  dump, which `ein oracle --dump` now writes after its read-out.
- The costs above, and task 7's saving, which is larger than the page's estimate.
- V-66, a wrong-sounding row on the new example.
