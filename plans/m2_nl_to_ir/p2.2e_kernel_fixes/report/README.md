# P2.2e — the phase report, for the user's review

**What this is.** P2.2e, *Ein's two bugs, and a clock*, ran all its stages, S2.2e.1 to S2.2e.7, each in a subagent
that committed its work, and **stops here for the user's review. It is not closed**: closing it is the user's word.
This page is what to review: what the phase built, commands to run with their real outputs, a worked example a
stage, renderings, the numbers, the defaults taken, the questions left, and what the phase found.

**The user's words that ran it** (2026-11-27):

- *"Ok I agree, do it"*: the phase planned, to the routing of P2.2's results at its close;
- *"start p2.2e"*, and *"run automatically only to the end of p2.2e, then stop"*: the stages one by one;
- on [Q-M2.24](../../open_questions/kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md):
  *"So for now let's state it and record as a known fact, a guarantee, so if will ever see different behaviour it
  would be considered a bug"*: S2.2e.4's decision;
- on [L-M2.31](../findings/einaf/L-M2.31-einafs-recorded-model-runs-carry-the-librarys-digest.md): *"2"*: S2.2e.5
  added;
- on [E-130](../evaluation/bounds/E-130-a-question-that-prints-no-core-takes-a-dead-roots-core.md): *"E-130 - yes,
  add S2.2e.6"*: S2.2e.6 added;
- on [L-M2.32](../../findings/wrong-answers/L-M2.32-the-guarantee-of-q-m2-24-has-a-known-wrong-answer-against-it.md):
  *"keep, as recommended"*: the guarantee kept whole;
- on [E-131](../evaluation/bounds/E-131-two-more-cores-a-question-takes-and-prints-nowhere.md): *"E-131 - yes, add S2.2e.7"*: S2.2e.7 added;
- on Q-M2.43: *"add short example with consequenses explanation"*, then the correction that moved it out of the
  phase, *"… create a new phase for this question"*: it is
  [P2.2g](../../p2.2g_names_across_an_import/README.md)'s now ([§ 7](#7-questions-for-the-user)).

**How to see it again.** Everything under [`out/`](out/) is written by [`make_report.sh`](make_report.sh) from the
binaries, with no model and no GPU; re-run it on any checkout:

```sh
export CARGO_HOME=…                                                    # where the host's rules say
./build.sh --engine --einaf                                            # ein.rs/target/release/ein, einaf/target/release/einaf
plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/make_report.sh             # everything; 2 × 72 s of it the model's Zebra program
plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/make_report.sh --quick     # leaves out the eight slow runs (the cores' searches, h-space)
```

`out/INDEX.tsv` lists every command with its exit code, its seconds and the command as run
([§ 11](#11-every-command-make_reportsh-runs) has it as a table); `out/NAME.out` is a command's stdout and
`out/NAME.err` its stderr. The script puts `ein` and `einaf` on `PATH` as the release builds, so each command reads
as you would type it. **Nothing under `out/` is committed** (`plans/m2_nl_to_ir/.gitignore`, the user's rule: what a
script writes again); the links below into it work once the script has run.

**Before and after.** Where a stage changed the library (S2.2e.2), *before* is today's binary on the library as it
was (`git archive 0ab06c69 stdlib`, into `out/s2/old/`), run here. Where a stage changed the binary (S2.2e.1,
S2.2e.3, S2.2e.6, S2.2e.7), *before* is **quoted from the stage's record**, said so where it stands, and no old
binary is built; *after* is today's binary, run here.

**Written on** `3deda3d6`, the phase's last commit (Ein's code as of `87d5db47`), with release builds of it made for
this run, on AC power, the GPU free ([`out/RUN.txt`](out/RUN.txt)). The timed runs are pinned to one P-core thread by
`utils/bench_env.sh --cores PT:1`, whose fingerprint is each run's stderr.

**The gate:** green on `3deda3d6`, the phase's last commit: `./run_tests.sh` exited 0, every static check, then Ein's and EinAf's tests, 1 536 passed and 0 failed, run 15:17 to 15:47 on AC. Since `3deda3d6` only plans have changed, this page and its script among them.

**Contents**: [1 What the phase built](#1-what-the-phase-built) ·
[2 Commands to try](#2-commands-to-try-with-their-real-outputs) · [3 Examples, worked](#3-examples-worked) ·
[4 Renderings](#4-renderings) · [5 The numbers](#5-the-numbers) ·
[6 Defaults](#6-the-defaults-taken-for-the-user-to-correct) · [7 Questions](#7-questions-for-the-user) ·
[8 What the phase found](#8-what-the-phase-found) · [9 What is left](#9-what-is-left) ·
[10 Housekeeping](#10-housekeeping-disclosures) · [11 Every command](#11-every-command-make_reportsh-runs)

---

## 1. What the phase built

| stage | what it ended with | commits | record |
|---|---|---|---|
| **S2.2e.1** a relation declared again, under an import | A program that imports loads a relation's second declaration, at another count or with other labels, as one that imports nothing does ([Q-M1f.447](../../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)): the imports pass lets a `relation` form through, and still folds a rule or a macro written twice the same and refuses one with two bodies. A feature file, `zebra3.2.ein` declaring `the` at both its counts, the model's `unused` loading in both conditions. **Closes [E-104](../../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md)**; files E-129 | Ein `8d1f89ba` | [record](../s2.2e.1_declared_again_under_an_import.md#record) (`0ab06c69`) |
| **S2.2e.2** `std.slots`: two neighbours do not share a place | `slot-prune-fwd` and `-bwd` without the `neq` that let the two ends of a *next to* clue share a place, the rules as their page states them. With sentence (15) dropped, the four encodings over `std.slots` answer 32 states, as the two over `std.bijection` and `nl2smt` do, where they answered 55; the full puzzle `Solution` on all six. Three programs under `tests/stdlib/slots/`; no corpus answer moved; three counts of `zebra.ein`'s H structure moved, to `zebra2.ein`'s. **Closes [E-105](../../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md)**; files L-M2.31 | Ein `b2d8276f` | [record](../s2.2e.2_slots_neighbours.md#record) (`d20e3b29`) |
| **S2.2e.3** the clock, and a root that dies | Where the model's Zebra program's 72 s went: **root's core**, searched with no read of the clock, under every question, though `--single` prints none. The clock now read inside the search, every 4 096 units of its work: under `-T` the run is the clock's abort within milliseconds of it. The fixture `dead_link.ein`, two tests; EinAf's feedback page § 4. **Closes [E-126](../../evaluation/bounds/E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md)**; files E-130 | Ein `3a5c162b`, EinAf `de6d61d0` | [record](../s2.2e.3_clock_and_a_dead_root.md#record) (`8524e41f`) |
| **S2.2e.4** the order of a program's forms, two rows of a table | [Q-M2.24's decision](../../../../docs/decisions/language/Q-M2.24-an-answer-does-not-depend-on-the-order.md): an answer does not depend on the order a program's forms are written in, a guarantee held by no test, as ruled, saying where a run falls short of it today; its row of the determinism page. The parser's table of refused shapes gains rows 352 and 353. **Closes [E-121](../../evaluation/language/E-121-the-banked-table-of-refused-shapes-misses-two-of-the-lexer-s-token-ends.md)**; [E-112](../../evaluation/language/E-112-a-program-s-forms-may-be-written-in-the-order-they-are-made.md) a strength Ein states; files L-M2.32 | Ein `e2b7c0c9`, `cc393635` | [record](../s2.2e.4_order_and_table_rows.md#record) (`e8d51997`) |
| **S2.2e.5** EinAf's recordings follow the library | The catalogue a prompt shows carries no digest of the library; the record does (`stdlib`, the run's and each step's, and the one shot's `config.json`). The two recorded runs recorded again with the model, once; the probe line judged again with no model; a replay survives a library change that leaves the catalogue's text alone. EinAf only. **Closes [L-M2.31](../findings/einaf/L-M2.31-einafs-recorded-model-runs-carry-the-librarys-digest.md)**; files L-M2.33 | EinAf `8e3c6f4a`, `969ae30d` | [record](../s2.2e.5_recordings_follow_the_library.md#record) (`737a0bd3`, `5c99eca9`) |
| **S2.2e.6** a dead root's core only where a question prints it | Root's core taken by a description and `--no-search` (and by `--events`, `--dump-lattice` and `ein render lattice`, which print a visited node's), by no mode and not by `--count`. On the model's Zebra program `--single`, `-n`, the two lines and `--count` answer in a sixth of a second where they took 72 s, and under `-T 5` answer where they were the clock's abort; no answer and no printed byte moved. **Closes [E-130](../evaluation/bounds/E-130-a-question-that-prints-no-core-takes-a-dead-roots-core.md)**; files E-131 | Ein `6b359cce`, EinAf `1a8608ff` | [record](../s2.2e.6_a_core_only_where_printed.md#record) (`9f2529d1`) |
| **S2.2e.7** two more cores only where a question prints them | The cube's minimal holes' cores taken by a description and `--count` alone, and the whole node's only where the run is narrated and asks every visited node's core (`--events`, `--dump-lattice`), not under `-v` or `--layer-progress`. Two fixtures beside `dead_link.ein`: on them the line modes answer in a fifth of a second where they took 18 s, and `--single -v` answers in 0.6 s where it ran 18 s, past a 5 s clock; no answer and no printed byte moved. **Closes [E-131](../evaluation/bounds/E-131-two-more-cores-a-question-takes-and-prints-nowhere.md)**; files E-132 | Ein `87d5db47` | [record](../s2.2e.7_two_more_cores_only_where_printed.md#record) (`6a7e9ce6`) |

**Ein changed** in four places of code, each a bug fix or a bound the ruling allows
([Q-M2.16](../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md): no change of Ein's
syntax or semantics): the imports pass (`ein-ir/src/imports.rs`, S2.2e.1); two premises of `std.slots`
(S2.2e.2); the core's search under the clock (`ein-infer/src/explain.rs`, S2.2e.3); which cores a question takes
(`region.rs`, `explore.rs`, S2.2e.6; with `structure/coupling/node.rs`, S2.2e.7). One decision is new, Q-M2.24's; the others the fixes rest on were
already written (Q-M1f.447, Q-M1f.29, Q-M1e2.31, Q-M2.16). **No answer of a corpus program moved** in any stage.
**EinAf** followed in the same batches (its feedback page and a test, S2.2e.3 and S2.2e.6) and changed on its own in
S2.2e.5.

**The phase's acceptance**, from [its README](../README.md#acceptance), item by item:

| item | met by | here |
|---|---|---|
| `ein solve` loads a relation's second declaration with and without an import; `declared_counts.rs` holds both | S2.2e.1 | [§ 2.1](#21-s22e1-a-relation-declared-again-under-an-import), [§ 3.1](#31-s22e1-the-models-unused-refused-then-loading-now) |
| With clue 15 dropped, the Zebra puzzle answers the same states over `std.slots` as over `std.bijection` on every encoding; the full puzzle still `Solution` on each | S2.2e.2 (`acceptance.rs`, `without_sentence_15_every_encoding_answers_the_same_states`) | [§ 3.2](#32-s22e2-the-zebra-puzzle-without-clue-15-counted-three-ways) |
| `ein solve --single -T 5` on the model's Zebra program returns within its bound; with no clock it answers as before; since S2.2e.6, `--single` takes no core it does not print and answers within the bound | S2.2e.3, S2.2e.6 (and S2.2e.7 for the two other cores) | [§ 3.3](#33-s22e3-and-s22e6-the-models-zebra-program-question-by-question) |
| Every corpus answer that moved listed with the reason; no claim re-stated to a wrong answer | none moved: each record's table; S2.2e.2's three counts of H's structure, not answers, re-stated with why | [§ 5](#5-the-numbers) |
| Each fix has its decision where it decides something; the entry it closes says which stage | Q-M2.24 new; E-104, E-105, E-126, E-121, E-130, E-131 each closed where it is | [§ 8](#8-what-the-phase-found) |
| The gate is green on the phase's last commit | the coordinator's | the header |

## 2. Commands to try, with their real outputs

From the repository's root, after `./build.sh --engine --einaf`. Below, `ein` is `ein.rs/target/release/ein`,
`einaf` is `einaf/target/release/einaf`, and three names stand for paths:

```sh
P=plans/m2_nl_to_ir/p2.2e_kernel_fixes                                   # this phase
Z=plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/library/zebra/program.ein   # the model's Zebra program (P2.2's run)
DL=ein.rs/crates/ein-cli/tests/bounded/dead_link.ein                     # S2.2e.3's fixture
```

Every output is the file `make_report.sh` wrote, linked beside it; where it is long, an excerpt, named in the comment
after the command, and the whole is the linked file. `out/` paths are the script's, under `$P/report/`.

### 2.1 S2.2e.1, a relation declared again, under an import

`likes` declared three times, twice at two arguments and once at one, beside an import
([`25_declared_twice_with_an_import.ein`](../../../../examples/features/25_declared_twice_with_an_import.ein)): every
claim holds, each declaration a fact of `relation` ([out](out/s1/feature-25-solve.out)).

<!-- output -->
```text
$ ein test examples/features/25_declared_twice_with_an_import.ein
ok           examples/features/25_declared_twice_with_an_import.ein
1 file, 7 expectations: 7 held, 0 FAILED, 0 not checked, 0 errors  (0.01 s)

$ ein solve examples/features/25_declared_twice_with_an_import.ein      # query 4 of 7
  query 4 of 7      (relation likes ?a ?b)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?a = Person · ?b = Drink
                      ?a = Person · ?b = Thing
  as :expect        (model (relation likes Person Drink)
                           (relation likes Person Thing))
  :expect           holds
```

E-104's own probe, `(import std.algebra)` and `likes` at two counts: **refused before the fix**, `kb load error:
conflicting definitions of 'likes': same name, different body` (quoted from
[S2.2e.1's context](../s2.2e.1_declared_again_under_an_import.md#context)); today it answers as the same program with
no import does ([no import](out/s1/e104-two-counts.out), [with](out/s1/e104-two-counts-import.out)):

<!-- output -->
```text
$ ein solve $P22/zebra_formulations/probes/language/declared-at-two-counts-with-an-import.ein      # the query
  query             (likes ?who ?what)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?what = Tea · ?who = Ann
  as :expect        (model (likes Ann Tea))
```

`zebra3.2.ein`, which could not declare `the` at both the counts it writes, now does, and still answers the puzzle
([out](out/s1/zebra3.2-single.out)):

<!-- output -->
```text
$ grep -n '^(relation the ' examples/zebra3.2.ein
93:(relation the         T T     :why "the {?3} {?4}")
94:(relation the         T T T   :why "the {?5} {?3} {?4}")

$ ein solve examples/zebra3.2.ein --single      # the verdict and the answer
  asked             is R one state? (--single)
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  query             (and (same ?who_water (the Water (is drunk by) Man)) (same ?who_zebra (the Zebra (is owned by) Man)))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water. The Japanese owns the zebra.”
```

### 2.2 S2.2e.2, `std.slots`: two neighbours do not share a place

The fix, two premises taken out ([out](out/s2/slots-fix-diff.out), the diff's code lines):

<!-- output -->
```text
$ git diff 0ab06c69 b2d8276f -- stdlib/slots.ein      # the rules' hunks
@@ -417,7 +424,6 @@
   :match  (and (?S ?a ?b)
                (?R ?b ?p1) (?isa ?p1 ?PT)
                (?isa ?p_o ?PT)
-               (neq ?p_o ?p1)
                (absent (?S ?p_o ?p1)))
   :assert (not (?R ?a ?p_o))
   :why    "{?b} is at {?p1}, so {?a} is at an {?S}-source of {?p1}; {?p_o} is not one.")
@@ -426,7 +432,6 @@
   :match  (and (?S ?a ?b)
                (?R ?a ?p2) (?isa ?p2 ?PT)
                (?isa ?p_o ?PT)
-               (neq ?p_o ?p2)
                (absent (?S ?p2 ?p_o)))
   :assert (not (?R ?b ?p_o))
   :why    "{?a} is at {?p2}, so {?b} is at an {?S}-target of {?p2}; {?p_o} is not one.")
```

The six Zebra encodings with sentence (15) dropped, each variant written by the phase's probe, on today's library
and on the library before the fix ([after](out/s2/six-after.out), [before](out/s2/six-before.out)):

<!-- output -->
```text
$ env EIN_BIN=ein.rs/target/release/ein python3 $P/probes/slots-neighbours/six.py out/s2/six-after
zebra     drop-15 --maximal  32 states, 10 classes (32 states) · 114 saturations · 18 175 facts beyond root | whole --single Solution · 9 saturations · 784 facts beyond root
zebra3.1  drop-15 --maximal  32 states, 10 classes (32 states) · 104 saturations · 29 949 facts beyond root | whole --single Solution · 9 saturations · 1 679 facts beyond root
zebra3.2  drop-15 --maximal  32 states, 10 classes (32 states) · 104 saturations · 25 536 facts beyond root | whole --single Solution · 9 saturations · 1 286 facts beyond root
zebra3.3  drop-15 --maximal  32 states, 10 classes (32 states) · 114 saturations · 40 201 facts beyond root | whole --single Solution · 9 saturations · 1 956 facts beyond root
zebra2    drop-15 --maximal  32 states, 10 classes (32 states) · 108 saturations · 8 601 facts beyond root | whole --single Solution · 7 saturations · 278 facts beyond root
zebra3.4  drop-15 --maximal  32 states, 10 classes (32 states) · 126 saturations · 9 751 facts beyond root | whole --single Solution · 11 saturations · 449 facts beyond root
zebra classes equal zebra2's: True
zebra3.1 classes equal zebra2's: True
zebra3.2 classes equal zebra2's: True
zebra3.3 classes equal zebra2's: True
zebra2 classes equal zebra2's: True
zebra3.4 classes equal zebra2's: True

$ env EIN_BIN=ein.rs/target/release/ein python3 $P/probes/slots-neighbours/six.py out/s2/six-before --stdlib out/s2/old/stdlib      # the library before the fix
zebra     drop-15 --maximal  55 states, 13 classes (55 states) · 148 saturations · 24 403 facts beyond root | whole --single Solution · 9 saturations · 784 facts beyond root
zebra3.1  drop-15 --maximal  55 states, 13 classes (55 states) · 141 saturations · 43 561 facts beyond root | whole --single Solution · 9 saturations · 1 679 facts beyond root
zebra3.2  drop-15 --maximal  55 states, 13 classes (55 states) · 140 saturations · 35 770 facts beyond root | whole --single Solution · 9 saturations · 1 278 facts beyond root
zebra3.3  drop-15 --maximal  55 states, 13 classes (55 states) · 148 saturations · 54 477 facts beyond root | whole --single Solution · 9 saturations · 1 956 facts beyond root
zebra2    drop-15 --maximal  32 states, 10 classes (32 states) · 108 saturations · 8 601 facts beyond root | whole --single Solution · 7 saturations · 278 facts beyond root
zebra3.4  drop-15 --maximal  32 states, 10 classes (32 states) · 126 saturations · 9 751 facts beyond root | whole --single Solution · 11 saturations · 449 facts beyond root
```

`tests/stdlib/slots/14_prune_placed_position.ein`, a seat next to a placed cat: the placed end's own seat is
excluded now, and the test fails on the library before ([after](out/s2/solve-14.out),
[before](out/s2/test-14_prune_placed_position-old.out)):

<!-- output -->
```text
$ ein test tests/stdlib/slots/14_prune_placed_position.ein
ok           tests/stdlib/slots/14_prune_placed_position.ein
1 file, 1 expectation: 1 held, 0 FAILED, 0 not checked, 0 errors  (0.02 s)

$ env EIN_STDLIB=out/s2/old/stdlib ein test tests/stdlib/slots/14_prune_placed_position.ein      # exit 1
FAILED       tests/stdlib/slots/14_prune_placed_position.ein
  solve · :expect (region …) — FAILED
    R's states and the listed models do not match one to one
      a state of R no model lists: (model (excluded Bob S3))
      a listed model no state of R is: (model (excluded Ann S2) (excluded Bob S1) (excluded Bob S3))
      the model lists (excluded Ann S2), which the state does not hold
      the model lists (excluded Bob S1), which the state does not hold
1 file, 1 expectation: 0 held, 1 FAILED, 0 not checked, 0 errors  (0.01 s)

$ ein test tests/stdlib/slots      # the module's suite
16 files, 16 expectations: 16 held, 0 FAILED, 0 not checked, 0 errors  (0.16 s)
```

### 2.3 S2.2e.3 and S2.2e.6, the clock and a dead root's core

The model's Zebra program, root dead: `--single` answers in what root costs, with no clock or under one
([out](out/s3/model-zebra-single-timing.out), [out](out/s3/q/single-5.out)):

<!-- output -->
```text
$ ein solve $Z --single -t      # the verdict and the timing
  verdict           Contradiction — R is empty: every node is dead
  decided           yes — R is empty
timing
  load          36.36 ms   parse, imports, the program
  route         22.71 ms   root, H and every node visited: 1 saturation
  read-out       0.01 ms   each listed state's model, the queries
  print          0.07 ms   the text
  total         59.15 ms

$ utils/bench_env.sh --cores PT:1 ein solve $Z --single -T 5 --json-summary out/s3/q/single-5.json      # exit 0, 0.17 s with bench_env's fingerprint
  verdict           Contradiction — R is empty: every node is dead
  decided           yes — R is empty

$ utils/bench_env.sh --cores PT:1 ein solve $Z -T 5 --json-summary out/s3/q/describe-5.json      # exit 2, 5.10 s: a description prints root's core, whose search is 72 s
** aborted: --max-time 5 s reached before the question was decided **
```

The fixture, a line of thirty towns ([`dead_link.ein`](../../../../ein.rs/crates/ein-cli/tests/bounded/dead_link.ein)):
root saturates in milliseconds and its core's search takes seconds. Under a clock of 0.3 s, a question that prints
no core answers, one that prints it is the clock's abort, and `ein test` answers unless a summary asks the core:

<!-- output -->
```text
$ ein solve $DL --single -T 0.3      # exit 0, 0.03 s
  verdict           Contradiction — R is empty: every node is dead
  decided           yes — R is empty

$ ein solve $DL -T 0.3      # exit 2, 0.31 s
** aborted: --max-time 0.3 s reached before the question was decided **

$ ein test $DL -T 0.3      # exit 0
ok           ein.rs/crates/ein-cli/tests/bounded/dead_link.ein
1 file, 1 expectation: 1 held, 0 FAILED, 0 not checked, 0 errors  (0.03 s)

$ ein test $DL -T 0.3 --json-summary out/s3/dead-link-test-summary.json      # exit 2
ERROR        ein.rs/crates/ein-cli/tests/bounded/dead_link.ein
    aborted before an answer
1 file, 0 expectations: 0 held, 0 FAILED, 0 not checked, 1 error  (0.30 s)
** aborted: --max-time 0.3 s reached before the question was decided **
```

EinAf's bounded call on the same program, a clock of 0.2 s:

<!-- output -->
```text
$ einaf outcome $DL --single -T 0.2
einaf outcome · ein.rs/crates/ein-cli/tests/bounded/dead_link.ein --single
  inconsistent  over all of R           Contradiction: R is empty and every node is dead

$ einaf outcome $DL -T 0.2
einaf outcome · ein.rs/crates/ein-cli/tests/bounded/dead_link.ein
  not decided   over all of R           the clock ran out before the question was decided
  not decided   over the minimal line   the clock ran out before the question was decided
  not decided   over the maximal line   the clock ran out before the question was decided
```

### 2.4 S2.2e.4, the order of a program's forms, and two rows of a table

The phase's probe asks seven questions of two pairs, each pair one program in two orders, and diffs the read-outs
([out](out/s4/order-run.out)); the first pair is L-M2.32's, a known wrong answer, the second differs only where the
decision says it may:

<!-- output -->
```text
$ env EIN_BIN=ein.rs/target/release/ein sh $P/probes/order-of-forms/run.sh
gap-reader-first / gap-producer-first []: differ
    24,26c24,25
    <     class 1 of 1    1 state (1)
    <                       ?x = k
    <   as :expect        (model (p k))
    ---
    >     class 1 of 1    1 state (1) — the goal has no row: undetermined here
    >   as :expect        (model)
gap-reader-first / gap-producer-first [--single]: differ
    15,17c15,16
    <     class 1 of 1    1 state (1)
    <                       ?x = k
    <   as :expect        (model (p k))
    ---
    >     class 1 of 1    1 state (1) — the goal has no row: undetermined here
    >   as :expect        (model)
gap-reader-first / gap-producer-first [-n 1]: differ
    15,16c15
    <     class 1 of 1    1 state
    <                       ?x = k
    ---
    >     class 1 of 1    1 state — the goal has no row: undetermined here
gap-reader-first / gap-producer-first [--minimal]: differ
    15,16c15
    <     class 1 of 1    1 state (1)
    <                       ?x = k
    ---
    >     class 1 of 1    1 state (1) — the goal has no row: undetermined here
gap-reader-first / gap-producer-first [--maximal]: differ
    15,16c15
    <     class 1 of 1    1 state (1)
    <                       ?x = k
    ---
    >     class 1 of 1    1 state (1) — the goal has no row: undetermined here
gap-reader-first / gap-producer-first [--count]: same
gap-reader-first / gap-producer-first [--no-search]: differ
    12,13c12
    <     class 1 of 1    1 state
    <                       ?x = k
    ---
    >     class 1 of 1    1 state — the goal has no row: undetermined here
one-food-each-ann-first / one-food-each-carl-first []: differ
    7d6
    <     owed            likes owes Ann a Food.
    8a8
    >     owed            likes owes Ann a Food.
one-food-each-ann-first / one-food-each-carl-first [--single]: same
one-food-each-ann-first / one-food-each-carl-first [-n 1]: differ
    10,11c10,11
    <   ;; state 1 of 1 — 2 candidates · from {(likes Ann Soup), (likes Carl Stew)}
    <   (model (likes Ann Soup) (likes Carl Stew))
    ---
    >   ;; state 1 of 1 — 2 candidates · from {(likes Ann Stew), (likes Carl Soup)}
    >   (model (likes Ann Stew) (likes Carl Soup))
    16,17c16,17
    <                       ?what = Soup · ?who = Ann
    <                       ?what = Stew · ?who = Carl
    ---
    >                       ?what = Soup · ?who = Carl
    >                       ?what = Stew · ?who = Ann
one-food-each-ann-first / one-food-each-carl-first [--minimal]: same
one-food-each-ann-first / one-food-each-carl-first [--maximal]: same
one-food-each-ann-first / one-food-each-carl-first [--count]: same
one-food-each-ann-first / one-food-each-carl-first [--no-search]: differ
    4d3
    <     owed            likes owes Ann a Food.
    5a5
    >     owed            likes owes Ann a Food.
```

The table's two new rows, as banked, and the binary and EinAf's grammar on the two programs, each written as the row
writes it:

<!-- output -->
```text
$ sed -n '/^## 352\./,$p' ein.rs/crates/ein-ir/tests/golden/grammar_table.txt
## 352. a block comment glued after a string (was: —)
    (relation t T :why "t"#|c|#)
<row>:1:20: unexpected `"t"#|c|#`: a string glued to what follows
  repair: Insert a string, Delete `"t"#|c|#`

## 353. a form's closing paren after a line comment (was: —)
    (relation p T) (p a ;c)
<row>:1:24: unexpected end of input
  repair: Insert `)`

$ ein solve out/s4/row-352.ein      # exit 1
plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/out/s4/row-352.ein:1:20: unexpected `"t"#|c|#`: a string glued to what follows
  repair: Insert a string, Delete `"t"#|c|#`

$ ein solve out/s4/row-353.ein      # exit 1
plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/out/s4/row-353.ein:1:24: unexpected end of input
  repair: Insert `)`

$ einaf grammar accept out/s4/row-352.ein out/s4/row-353.ein      # exit 1
plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/out/s4/row-352.ein: refused at 1:23, '#'
plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/out/s4/row-353.ein: refused at 1:24, the end
```

### 2.5 S2.2e.5, EinAf's recordings follow the library

The catalogue's sentence as every prompt shows it now, and as the old fixture's prompts showed it:

<!-- output -->
```text
$ bash -c 'einaf catalogue | sed -n 3p'
A program selects a theory by importing its module, listing after :symbols every short name of the module the program writes, and writing the facts that select the theory over its own relations, exactly as the entry writes them. A name the program writes and the import does not list is the program's own, and a fact of it selects nothing, in silence. Each entry is a program the kernel ran, with its answer.

$ bash -c "git show 969ae30d^:einaf/tests/fixtures/steps/bob1/calls.jsonl | grep -o 'The library is sha256:[0-9a-f]*\.' | sort | uniq -c"
      2 The library is sha256:292894befd300e536cdd1181db772f44fe3f3f7ad6ff52b1467f1735ff83d681.
```

The two runs recorded again, replayed with no model ([zebra-contradiction](out/s5/zc-replay.err),
[bob1](out/s5/bob1-steps.out)); the replayed program is the fixture's byte for byte (`cmp`, exit
0), and the record names the library:

<!-- output -->
```text
$ einaf from-text einaf/tests/fixtures/from_text/zebra-contradiction/text.txt --id zebra-contradiction --answer-line zebra --replay einaf/tests/fixtures/from_text/zebra-contradiction/generations.jsonl --gold-answer @contradiction --record out/s5/zc
The statements contradict each other.
[stderr]
einaf: replaying 3 recorded calls
step 1     4793 prompt tokens, 1065 generated (end-of-turn, 16.9 s), 56 forms; at root not decided; H from blind (8428); 0 findings
step 2     12601 prompt tokens, 1278 generated (end-of-turn, 21.8 s), 82 forms; at root inconsistent; H from obligations (70); 1 findings
step 3     13929 prompt tokens, 352 generated (end-of-turn, 6.1 s), 17 forms
the program: H from obligations (70); 10 findings of the check
  --no-search: inconsistent (root is dead, so every node is), 1.8 s
  --single: inconsistent (Contradiction: R is empty and every node is dead), 0.0 s
the answer: inconsistent (from einaf)
einaf from-text: 4.4 s
against the gold answer "@contradiction": right (the fixed text "The statements contradict each other.")

$ python3 -c 'import json, sys; r = json.load(open(sys.argv[1]))["run"]; print(r["stdlib"], sorted({s["stdlib"] for s in r["steps"]}))' out/s5/zc/run.json
sha256:59b1794aa5935646cd535492964f3bd119b2575a8948c9967ab9b5833ec396b8 ['sha256:59b1794aa5935646cd535492964f3bd119b2575a8948c9967ab9b5833ec396b8']

$ einaf from-text einaf/tests/fixtures/steps/bob1/text.txt --answer-line word --replay einaf/tests/fixtures/steps/bob1/calls.jsonl --gold-answer 3
Bob has 5 apples.
[stderr]
the answer: lines (from program)
against the gold answer "3": wrong (no line states 3)
```

A library whose digest moved and whose catalogue did not (a comment added to `std.math.gcd`, its manifest line
written again, as EinAf's test does): the catalogue is the same text, the recorded run replays to the same program
(`cmp`, exit 0) and answer, and its record names the moved library:

<!-- output -->
```text
$ env EIN_STDLIB=out/s5/moved/stdlib ein --version      # the last line
stdlib     sha256:9bbb696a3cc48c0ce2349ecd11baf520fb1dcefc2bbbea5c1c296b2ffe1e1845  $EIN_STDLIB plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/out/s5/moved/stdlib

$ bash -c "cmp <(EIN_STDLIB=out/s5/moved/stdlib einaf catalogue) <(einaf catalogue) && echo 'einaf catalogue: the same text under the moved library as under the checkout'"
einaf catalogue: the same text under the moved library as under the checkout

$ env EIN_STDLIB=out/s5/moved/stdlib einaf from-text einaf/tests/fixtures/from_text/zebra-contradiction/text.txt --id zebra-contradiction --answer-line zebra --replay einaf/tests/fixtures/from_text/zebra-contradiction/generations.jsonl --gold-answer @contradiction --record out/s5/zc-moved
The statements contradict each other.

$ python3 -c 'import json, sys; r = json.load(open(sys.argv[1]))["run"]; print(r["stdlib"], sorted({s["stdlib"] for s in r["steps"]}))' out/s5/zc-moved/run.json
sha256:9bbb696a3cc48c0ce2349ecd11baf520fb1dcefc2bbbea5c1c296b2ffe1e1845 ['sha256:9bbb696a3cc48c0ce2349ecd11baf520fb1dcefc2bbbea5c1c296b2ffe1e1845']
```

The probe line, judged again from its own record with no model ([compare](out/s5/probe-compare.txt)):

<!-- output -->
```text
$ einaf probe run --plan out/s5/probe-plan.jsonl --out out/s5/probe-again.jsonl --replay einaf/tests/fixtures/model/probe_lines.jsonl
einaf: replaying 2 recorded calls
[1/2] library.off.youngest 364 tokens (0 thinking), EndOfTurn, 5.8 s: whole=parse error (parse) fenced=loads, --single one
[2/2] library.off.gap 513 tokens (0 thinking), EndOfTurn, 8.2 s: whole=parse error (parse) fenced=load error (unbound-conclusion)
2 lines judged again, 2 in the fixture
line 1: the same as the fixture, field for field, but the judge's own times (work.run_us, load_us, ask_us)
line 2: the same as the fixture, field for field, but the judge's own times (work.run_us, load_us, ask_us)
```

### 2.6 S2.2e.7, two more cores only where a question prints them

Two fixtures beside `dead_link.ein`, under `ein.rs/crates/ein-cli/tests/bounded/`:
[`dead_link_hole.ein`](../../../../ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein), a program the static
check flags, whose one minimal hole dies on the link; and
[`dead_link_whole.ein`](../../../../ein.rs/crates/ein-cli/tests/bounded/dead_link_whole.ein), a monotone one whose
whole node, every candidate together, dies on it. A line mode prints no hole and takes no hole's core; a watch prints
no core and takes none; a description still prints the hole's, at its cost. Each run pinned:

<!-- output -->
```text
$ utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein --maximal -T 5      # exit 0, 0.20 s
  WARNING           THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER.
  maximal line      1 state — all of them

$ utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein --minimal      # exit 0, 0.17 s
  minimal line      1 state — all of them

$ utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein      # exit 0, 17.49 s: the hole's core, its search the cost
  WARNING           THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER.
  states (k_s)      3
    minimal hole    {(gate a), (opened c hand)} — dead: (gate a) (road1 t1 t2) (road1 t10 t11) (road1 t11 t12) (road1 t12 t13) (road1 t13 t14) (road1 t14 t15) (road1 t15 t16) (road1 t16 t17) (road1 t17 t18) (road1 t18 t19) (road1 t19 t20) (road1 t2 t3) (road1 t20 t21) (road1 t21 t22) (road1 t22 t23) (road1 t23 t24) (road1 t24 t25) (road1 t25 t26) (road1 t26 t27) (road1 t27 t28) (road1 t28 t29) (road1 t29 t30) (road1 t3 t4) (road1 t4 t5) (road1 t5 t6) (road1 t6 t7) (road1 t7 t8) (road1 t8 t9) (road1 t9 t10) (core search cut at its budget of 64 environments per fact)

$ utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_whole.ein --single -v -T 5      # exit 0, 0.61 s
  verdict           Ambiguity — R holds two states or more; two of them below

$ utils/bench_env.sh --cores PT:1 ein solve $P/probes/two-more-cores-only-where-printed/whole-node-clock.ein --single --events out/s7/wclock-events.jsonl -T 5      # exit 2, 17.61 s: past the clock, E-132
** aborted: --max-time 5 s reached before the question was decided **
```


## 3. Examples, worked

### 3.1 S2.2e.1: the model's `unused`, refused then, loading now

P2.2's one shot met E-104 on the text `unused`: the model declared `has` twice beside an import, and both conditions'
programs were *invalid*. Their committed records say so; today the same programs load, through EinAf's one door:

<!-- output -->
```text
$ grep -h -- '--no-search: invalid' $P22/from_text/runs/library/unused/stderr.txt
  --no-search: invalid (refused at the load (declaration): conflicting definitions of 'has': same name, different body (at unused.ein:23:11)), 0.0 s

$ einaf outcome $P22/from_text/runs/library/unused/program.ein --single
einaf outcome · plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/library/unused/program.ein --single
  one           over all of R           Solution: one state

$ grep -h -- '--no-search: invalid' $P22/from_text/runs/module-dir/unused/stderr.txt
  --no-search: invalid (refused at the load (declaration): conflicting definitions of 'has': same name, different body (at unused.ein:19:11)), 0.0 s

$ einaf outcome $P22/from_text/runs/module-dir/unused/program.ein --single
einaf outcome · plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/module-dir/unused/program.ein --single
  inconsistent  over all of R           Contradiction: R is empty and every node is dead
```

The module-dir program loads and its root is dead: its own `std.slots` selection, not the declarations. What the
pass still does that a program with no import does not, a rule or a macro written twice
([E-129](../../evaluation/language/E-129-a-rule-or-a-macro-written-twice-the-same-is-read-once-where-the-program-imports.md),
open, no owner), the phase's eight probes:

| probe | no import | with `(import std.algebra)` |
|---|---|---|
| a rule written twice the same | `kb load error: duplicate rule name 'r' at None` (exit 1) | loads, read once (exit 0) |
| a rule written twice, two bodies | `kb load error: duplicate rule name 'r' at None` | `kb load error: conflicting definitions of 'r': same name, different body (at plans/m2_nl_to_ir/p2.2e_kernel_fixes/probes/rule-twice-two-bodies-with-an-import.ein:6:7)` |
| a macro written twice the same | `kb load error: duplicate macro 'pp' at None` | loads, read once (exit 0) |
| a macro written twice, two bodies | `kb load error: duplicate macro 'pp' at None` | `kb load error: conflicting definitions of 'pp': same name, different body (at plans/m2_nl_to_ir/p2.2e_kernel_fixes/probes/macro-twice-two-bodies-with-an-import.ein:6:8)` |

**A module's relation, declared in the program that imports it: today's behaviour, for
[P2.2g](../../p2.2g_names_across_an_import/README.md).** The stage stopped short of the user's statement on it and
filed [Q-M2.43](../../p2.2g_names_across_an_import/open_questions/kernel/Q-M2.43-is-a-programs-declaration-of-a-modules-relation-a-second-declaration-of-it.md),
which is P2.2g's now, with the user's correction: in the user's reading the module's declaration is
`(m.relation m.likes m.Person m.Drink)` and a program's `(relation m.likes Person Drink)`, two different facts. What
Ein writes today, on the probes under [`probes/a-modules-relation/`](../probes/a-modules-relation/) (the module `m`
declares `likes`, a rule `mfan` over it and `(likes Cy Milk)`): under `(import m :as a)` with the program's own
`(relation a.likes …)`, the module's declaration is `(relation m.likes Person Drink)`, the program's
`(relation a.likes Person Drink)`, two relations, and the module's rule reads only `Cy`:

<!-- output -->
```text
$ ein saturate $P/probes/a-modules-relation/aliased.ein --dump      # the dump
;; Relations — declared (2)
(relation m.likes Person Drink)
(relation a.likes Person Drink)

;; Relations — auto-vivified (1)
;;   a.mfan

;; Rules — the saturation agenda's (2)
;;   a.mfan  :params ()  stratum 0
;;   std.core.inconsistent  :params ()  :ein inconsistent

;; Strata — stratified, 1 stratum

;; ── BACKGROUND (6 facts) ──
  (a.likes Ann Tea)
  (m.likes Cy Milk)
;;   relation (4)
  (relation m.likes Person Drink)
  (relation m.likes)
  (relation a.likes Person Drink)
  (relation a.likes)

;; ── DERIVED (1 fact) ──
  (a.mfan Cy :rule a.mfan)

;; Query

$ ein saturate $P/probes/a-modules-relation/aliased-undeclared.ein --dump      # with no declaration line: one relation, the module's
;; Relations — declared (1)
(relation a.likes Person Drink)

;; Relations — auto-vivified (1)
;;   a.mfan

;; Rules — the saturation agenda's (2)
;;   a.mfan  :params ()  stratum 0
;;   std.core.inconsistent  :params ()  :ein inconsistent

;; Strata — stratified, 1 stratum

;; ── BACKGROUND (4 facts) ──
;;   a.likes (2)
  (a.likes Cy Milk)
  (a.likes Ann Tea)
;;   relation (2)
  (relation a.likes Person Drink)
  (relation a.likes)

;; ── DERIVED (2 facts) ──
;;   a.mfan (2)
  (a.mfan Cy :rule a.mfan)
  (a.mfan Ann :rule a.mfan)

;; Query

$ ein solve $P/probes/a-modules-relation/listed.ein      # exit 1 (Q-M1f.234)
kb load error: plans/m2_nl_to_ir/p2.2e_kernel_fixes/probes/a-modules-relation/listed.ein:3:9: 'likes' is bound to two full names in the program, likes (its own declaration) and m.likes — one short name names one declaration in a file: list it from one module, or write the other by its full name

$ ein solve $P/probes/a-modules-relation/full-name.ein      # exit 1 (Q-M1f.239)
kb load error: plans/m2_nl_to_ir/p2.2e_kernel_fixes/probes/a-modules-relation/full-name.ein:4:11: the program declares 'm.likes', a full name of m, which the program imports — a declaration's full name is its own file's, and a full name of m is one m declares (Q-M1f.239)
```

### 3.2 S2.2e.2: the Zebra puzzle without clue 15, counted three ways

**Is it the benchmark's problem?** The phase began by checking its own premise, and the counts disagreed as the
library stood. Here they are again, one table, today's binary: `nl2smt` (NLTK to Racket to cvc5, an enumeration that
is not Ein's) on the text with sentence (15) dropped; each of the six encodings on today's library; and, after the
bar, each on the library before the fix. A row is a class of the answer query, *who drinks water / who owns the
zebra*, and its states ([out](out/s2/classes.txt), [nl2smt](out/s2/nl2smt-minus-15.out)):

<!-- output -->
```text
nl2smt: 32 solutions, 10 classes (who drinks water / who owns the zebra)
class                  nl2smt     zebra  zebra3.1  zebra3.2  zebra3.3    zebra2  zebra3.4   | the library before the fix: zebra zebra3.1 zebra3.2 zebra3.3 zebra2 zebra3.4
Englishman/Japanese         2         2         2         2         2         2         2   | 3 3 3 3 2 2
Englishman/Norwegian        1         1         1         1         1         1         1   | 7 7 7 7 1 1
Englishman/Ukrainian        3         3         3         3         3         3         3   | 3 3 3 3 3 3
Japanese/Japanese           0         0         0         0         0         0         0   | 1 1 1 1 0 0
Japanese/Norwegian          0         0         0         0         0         0         0   | 1 1 1 1 0 0
Japanese/Ukrainian          3         3         3         3         3         3         3   | 3 3 3 3 3 3
Norwegian/Englishman        1         1         1         1         1         1         1   | 7 7 7 7 1 1
Norwegian/Japanese          6         6         6         6         6         6         6   | 10 10 10 10 6 6
Norwegian/Norwegian         3         3         3         3         3         3         3   | 4 4 4 4 3 3
Norwegian/Ukrainian        10        10        10        10        10        10        10   | 10 10 10 10 10 10
Spaniard/Japanese           0         0         0         0         0         0         0   | 2 2 2 2 0 0
Spaniard/Norwegian          1         1         1         1         1         1         1   | 2 2 2 2 1 1
Spaniard/Ukrainian          2         2         2         2         2         2         2   | 2 2 2 2 2 2
states                     32        32        32        32        32        32        32   | 55 55 55 55 32 32
zebra: classes as nl2smt's, each over the same count: after True, before False
zebra3.1: classes as nl2smt's, each over the same count: after True, before False
zebra3.2: classes as nl2smt's, each over the same count: after True, before False
zebra3.3: classes as nl2smt's, each over the same count: after True, before False
zebra2: classes as nl2smt's, each over the same count: after True, before True
zebra3.4: classes as nl2smt's, each over the same count: after True, before True
```

The 23 states too many were the four `std.slots` encodings' alone: a state in which the Chesterfields smoker shares
the fox's house, or the Kools smoker the horse's, which no reading of *next to* admits (S2.2e.2's record; three
classes of them, *the Japanese drinks water and owns the zebra* among them, no solution has). `std.bijection`'s two
encodings answered 32 before and after, and `zebra2.ein`'s own rules had dropped the same guard long before
(S2.2e.2's record, T2.2e.2.2).

**A small instance, written for the check** ([`three-houses.ein`](../probes/slots-neighbours/three-houses.ein): three
houses, *the Englishman lives next to the man with the dog*, *milk is drunk in the middle house*, *the Spaniard drinks
tea*; who owns the cat?), counted by brute force over the 216 assignments and by Ein on each library:

<!-- output -->
```text
$ python3 $P/probes/slots-neighbours/three_houses.py
12 states; {'Englishman': 6, 'Norwegian': 2, 'Spaniard': 4}

$ ein solve $P/probes/slots-neighbours/three-houses.ein --count      # states
  states (k_s)      12

$ env EIN_STDLIB=out/s2/old/stdlib ein solve $P/probes/slots-neighbours/three-houses.ein --count      # states, the library before
  states (k_s)      16

$ ein solve $P/probes/slots-neighbours/three-houses.ein --dump-answer out/s2/three-houses-answer      # out/s2/three-houses-answer/answer.txt, the query's classes
  query             (owns ?who Cat)
  over              all of R — 12 states → 3 projection classes
    class 1 of 3    6 states (1, 5, 6, 7, 8, 12)
                      ?who = Englishman
    class 2 of 3    2 states (3, 10)
                      ?who = Norwegian
    class 3 of 3    4 states (2, 4, 9, 11)
                      ?who = Spaniard

# the same, on the library before the fix
  query             (owns ?who Cat)
  over              all of R — 16 states → 3 projection classes
    class 1 of 3    6 states (1, 7, 8, 9, 10, 16)
                      ?who = Englishman
    class 2 of 3    4 states (2, 5, 12, 15)
                      ?who = Norwegian
    class 3 of 3    6 states (3, 4, 6, 11, 13, 14)
                      ?who = Spaniard
```

The four states too many, two in the Norwegian's class and two in the Spaniard's, put the Englishman in the dog's
house (S2.2e.2's record).

**What it cost, and what it moved beside the answers.** On the full puzzle `--single` costs the same 9 saturations on
every encoding (8 facts more on `zebra3.2`, [§ 2.2](#22-s22e2-stdslots-two-neighbours-do-not-share-a-place)'s
`six.py` lines); with the clue dropped it saves work (`zebra3.1`: 141 saturations to 104). Three counts of the
structure of H on `zebra.ein` moved, each to `zebra2.ein`'s, which the stage re-stated in four tests
([§ 5](#5-the-numbers)).

### 3.3 S2.2e.3 and S2.2e.6: the model's Zebra program, question by question

The model's Zebra program from P2.2's one shot ([`$Z`](../../p2.2_formalizer/from_text/runs/library/zebra/program.ein))
has a dead root: its step 2 put four kinds with no member under `std.slots`' partition (P2.2's L-M2.23). Root's
saturation records every derivation of a link several rules derive, and the core's search over them was the whole
cost. **Before**, quoted from [S2.2e.6's record](../s2.2e.6_a_core_only_where_printed.md#t22e65--what-the-stage-found)
(`dd3910aa`'s release binary, after S2.2e.3's clock; S2.2e.3's record has the program at 72 s under `-T 5` before the
clock was read in the search), and **after**, today's binary, both pinned to one P-core thread through
`utils/bench_env.sh` (the wall includes its fingerprint, 0.08 to 0.10 s alone):

| question | clock | before: exit, wall (s) | after, today: exit, wall (s) | after, the answer's row | `unsat core` rows after |
|---|---|---|---|---|---|
| describe | none | 0, 71.95 | 0, 71.71 | ``   empty             every node is dead — root's own fixpoint holds a contradiction `` | 2 |
| `--single` | none | 0, 71.95 | 0, 0.17 | ``   verdict           Contradiction — R is empty: every node is dead `` | 0 |
| `-n 2` | none | 0, 72.02 | 0, 0.16 | ``   states of R       0 states — none: R is empty, every node is dead `` | 0 |
| `--minimal` | none | 0, 71.97 | 0, 0.17 | ``   minimal line      0 states — none: R is empty, every node is dead `` | 0 |
| `--maximal` | none | 0, 71.95 | 0, 0.17 | ``   maximal line      0 states — none: R is empty, every node is dead `` | 0 |
| `--count` | none | 0, 71.94 | 0, 0.18 | ``   empty             every node is dead — root's own fixpoint holds a contradiction `` | 0 |
| `--no-search` | none | 0, 71.94 | 0, 71.73 | ``   R                 not searched — --no-search builds no node past root: a bare `ein solve FILE` describes R `` | 2 |
| describe | `-T 5` | 2, 5.10 | 2, 5.10 | `** aborted: --max-time 5 s reached before the question was decided **` | 0 |
| `--single` | `-T 5` | 2, 5.11 | 0, 0.17 | ``   verdict           Contradiction — R is empty: every node is dead `` | 0 |
| `-n 2` | `-T 5` | 2, 5.12 | 0, 0.18 | ``   states of R       0 states — none: R is empty, every node is dead `` | 0 |
| `--minimal` | `-T 5` | 2, 5.11 | 0, 0.16 | ``   minimal line      0 states — none: R is empty, every node is dead `` | 0 |
| `--maximal` | `-T 5` | 2, 5.11 | 0, 0.18 | ``   maximal line      0 states — none: R is empty, every node is dead `` | 0 |
| `--count` | `-T 5` | 2, 5.10 | 0, 0.15 | ``   empty             every node is dead — root's own fixpoint holds a contradiction `` | 0 |
| `--no-search` | `-T 5` | 2, 5.11 | 2, 5.11 | `** aborted: --max-time 5 s reached before the question was decided **` | 0 |

The table is [`out/s3/questions.tsv`](out/s3/questions.tsv), written from the runs' outputs and summaries; the
before column is the record's. The two questions that print root's core still cost its search, 72 s, and under
`-T 5` are the clock's abort: that is E-113 and F22's, and the lever
[Q-M2.34](../../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md)
asks about answers them at once, with another core (a copy with `:record-alternative-justifications false`,
0.06 s; [out](out/s3/model-zebra-lever-describe.out)):

<!-- output -->
```text
$ ein solve out/s3/model-zebra-lever.ein      # the core's first rows and its rule
  empty             every node is dead — root's own fixpoint holds a contradiction
  unsat core        (instance BlueHouse House)
  unsat core        (instance GreenHouse House)
  unsat core        (instance House1 House)
…
  refuted by        std.slots.slot-no-room
```

**EinAf's bounded call** on the fixture, each question its own outcome since S2.2e.6
([`03-feedback.md` § 4](../../../../docs/einaf/03-feedback.md#4-what-the-bounds-do-not-stop)): a description and
`--no-search` *not decided* at the clock with the deadline's diagnostic, every other question *inconsistent*, and no
object carrying a core ([out](out/s3/einaf-dead-link-feedback-fields.txt)):

<!-- output -->
```text
einaf feedback --single -T 0.2:
  asked        {"question": "--single", "which": "all", "ask": "single", "budget": 1048576, "limit": 16, "max_steps": 1048576, "clock_us": 200000}
  outcome      [{"part": "all", "outcome": "inconsistent", "why": "Contradiction: R is empty and every node is dead"}]
  core         null
  diagnostics  [{"kind": "claim-not-checked", "severity": "note", "message": "ein.rs/crates/ein-cli/tests/bounded/dead_link.ein: (solve :expect) NOT CHECKED — the claim is about all of R, and this run asked whether it is one state (--single)", "place": null}]
  work         {"saturations": 1, "facts": 0, "layers": null, "run_us": 30115, "load_us": 2979, "ask_us": 30828}
einaf feedback -T 0.2:
  asked        {"question": "describe", "which": "all", "ask": "describe", "budget": 1048576, "limit": 16, "max_steps": 1048576, "clock_us": 200000}
  outcome      [{"part": "all", "outcome": "not-decided", "why": "the clock ran out before the question was decided"}, {"part": "minimal", "outcome": "not-decided", "why": "the clock ran out before the question was decided"}, {"part": "maximal", "outcome": "not-decided", "why": "the clock ran out before the question was decided"}]
  core         null
  diagnostics  [{"kind": "deadline", "severity": "error", "message": "--max-time 0.2 s reached before the question was decided", "place": null}]
  work         {"saturations": null, "facts": null, "layers": null, "run_us": null, "load_us": 2959, "ask_us": 205216}
```

**The fixture under every question**, a clock of 0.3 s:

| question | exit | s | what it printed |
|---|---:|---:|---|
| describe | 2 | 0.31 | `** aborted: --max-time 0.3 s reached before the question was decided **` |
| `--no-search` | 2 | 0.31 | `** aborted: --max-time 0.3 s reached before the question was decided **` |
| `--single` | 0 | 0.03 | ``   verdict           Contradiction — R is empty: every node is dead `` |
| `-n 2` | 0 | 0.03 | ``   states of R       0 states — none: R is empty, every node is dead `` |
| `--minimal` | 0 | 0.03 | ``   minimal line      0 states — none: R is empty, every node is dead `` |
| `--maximal` | 0 | 0.03 | ``   maximal line      0 states — none: R is empty, every node is dead `` |
| `--count` | 0 | 0.03 | ``   empty             every node is dead — root's own fixpoint holds a contradiction `` |
| `--single --events FILE` (prints every visited node's core) | 2 | 0.31 | `** aborted: --max-time 0.3 s reached before the question was decided **` |
| `ein oracle` | 2 | 0.31 | `** aborted: --max-time 0.3 s reached before the cube was done **` |
| `ein test` | 0 | 0.03 | `` ok           ein.rs/crates/ein-cli/tests/bounded/dead_link.ein `` |
| `ein test --json-summary FILE` | 2 | 0.31 | `` ERROR        ein.rs/crates/ein-cli/tests/bounded/dead_link.ein `` |

With no clock, its description prints its core after 17.29 s
([out](out/s3/dead-link-describe.out)):

<!-- output -->
```text
$ ein solve $DL      # the core's first rows, of 30, and its end
  unsat core        (apart t1 t30)
  unsat core        (road1 t1 t2)
  unsat core        (road1 t10 t11)
…
  core search       cut at its budget of 64 environments per fact: a smaller core may exist
  refuted by        clash
```

### 3.4 S2.2e.4: one program, two orders

**The decision** ([Q-M2.24](../../../../docs/decisions/language/Q-M2.24-an-answer-does-not-depend-on-the-order.md)):
two programs that differ only in the order of their forms have one answer: R, its states and counts, root's standing
and what root owes, every question's outcome, every query's classes and rows, every claim's verdict, and H. What may
move is how a run reaches it and the order it says it in: the trace and the events, the order rules fire in and so a
core, a listing in the written order, which member of R a question asking for some meets first, the `:source` a fact
stated twice keeps, the `(config …)` a run takes of several. Held by no test, as ruled. The determinism page's row is
[§ 1.2](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#12-what-it-may-not-depend-on).

**Where a run falls short of it today** ([L-M2.32](../../findings/wrong-answers/L-M2.32-the-guarantee-of-q-m2-24-has-a-known-wrong-answer-against-it.md),
the register's `W8/fifo-gap-root`, M1n's): a saturation's boundary admits the rules that wait on an `absent` in their
written order. Two rules, the only difference which is written first
([diff](out/s4/gap-diff.out)):

<!-- output -->
```text
$ diff $P/probes/order-of-forms/gap-reader-first.ein $P/probes/order-of-forms/gap-producer-first.ein
6d5
< (rule reader () :match (and (a ?x) (b ?x) (absent (q ?x))) :assert (p ?x) :why "{?x} is p where no q")
7a7
> (rule reader () :match (and (a ?x) (b ?x) (absent (q ?x))) :assert (p ?x) :why "{?x} is p where no q")

$ ein solve $P/probes/order-of-forms/gap-reader-first.ein      # the query
  query             (p ?x)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?x = k
  as :expect        (model (p k))

$ ein solve $P/probes/order-of-forms/gap-producer-first.ein      # the query
  query             (p ?x)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)
```

With the reader first, `reader` is admitted while `(q k)` is absent and concludes `(p k)`, then `producer` concludes
`(q k)`; with the producer first, `(q k)` comes first and `reader` never fires. Both read-outs say `lattice monotone
(the static check)`. The user kept the guarantee whole: such a run is a wrong answer by the decision, known and not
fixed, until M1n decides the boundary's order. Root's fixpoint of each is drawn in [§ 4](#4-renderings).

**What may move, and does**: two people, two foods, one food each. `-n 1` meets a different member of R first (the
obligation tree branches on the owed instance written first among equals), and root's owed instances list in the
written order; every other question alike ([§ 2.4](#24-s22e4-the-order-of-a-programs-forms-and-two-rows-of-a-table)):

<!-- output -->
```text
$ ein solve $P/probes/order-of-forms/one-food-each-ann-first.ein -n 1      # the state met
  ;; state 1 of 1 — 2 candidates · from {(likes Ann Soup), (likes Carl Stew)}
  (model (likes Ann Soup) (likes Carl Stew))

$ ein solve $P/probes/order-of-forms/one-food-each-carl-first.ein -n 1
  ;; state 1 of 1 — 2 candidates · from {(likes Ann Stew), (likes Carl Soup)}
  (model (likes Ann Stew) (likes Carl Soup))
```

### 3.5 S2.2e.5: the two runs recorded again, and what moved beyond the line

The re-recording, 2026-11-27 13:52 to 13:54, the CUDA build, `qwen3.5:4b` pinned, greedy, seed 1, each fixture's
own settings (S2.2e.5's record). **What moved, call by call**, quoted from
[its record](../s2.2e.5_recordings_follow_the_library.md#t22e52--the-recordings-again):

| call | request | prompt tokens | generated | what it wrote |
|---|---|---|---|---|
| `zebra-contradiction` step 1 | the same | 4 793, the same | 1 065, the same token digest | the same `step1.ein`, byte for byte |
| `zebra-contradiction` step 2 | moved (the catalogue's line) | 12 601, was 12 666 | 1 278, was 1 218 | 82 forms, was 78: the houses `House2` to `House5` for `Second` to `Fifth`, `house-and-zebra` and `house-and-water` (with `(type Zebra Thing)` and `(type Water Drink)`) for `house-and-house`, `includes` and `symmetric` over other relations, two comments |
| `zebra-contradiction` step 3 | moved (also step 2's program) | 13 929, was 13 934 | 352, was 164; the grammar refused the chain's first choice twice, was never | 17 forms, was 9: a `top` and a `meet` for each relation of a house, `(converse is-immediately-to-the-right-of is-immediately-to-the-right-of)`, and **no `(solve :no-search)`** beside the obligations |
| `bob1` step 1 | the same | 4 636, the same | 142, the same tokens | the same |
| `bob1` step 2 | moved (the catalogue's line) | 11 524, was 11 589 | 122, the same tokens | the same |
| `bob1` step 3 | moved (the catalogue's line) | 12 286, was 12 351 | 144, was 230; parts from the old at its 23rd token | one rule where it wrote two: `giving-is-the-difference`, its `:why` reworded, and **no `giving-asks-the-difference`** |

The catalogue's line was 65 prompt tokens; greedy decoding parted where the prompt moved and not where it did not.
**What that does to `bob1`**, the programs before (the fixture at `969ae30d^`) and after, both run today:

<!-- output -->
```text
$ diff out/s5/bob1-before.ein out/s5/bob1-after.ein
24,28d23
< (rule giving-asks-the-difference ()
<   :match  (and (gave ?giver ?k ?receiver) (has ?giver ?a ?thing))
<   :assert (sub ?a ?k)
<   :why    "{?giver} had {?a} and gave {?k} to {?receiver}: ask {?a} less {?k}")
< ;; missing: the difference, read back as what the giver has left
32c27
<   :why    "{?a} less {?k} is {?d}, so {?giver} has {?d} {?thing}s left")
---
>   :why    "{?giver} had {?a} {?thing}s, gave {?k} to {?receiver}: {?giver} has {?d} {?thing}s left")

$ ein solve out/s5/bob1-before.ein --single      # the query
  query             (has Bob ?n Apple)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 1)
                        “Bob has (n z 1) apples.”
                      ?n = (n z 3)
                        “Bob has (n z 3) apples.”
                      ?n = (n z 5)
                        “Bob has (n z 5) apples.”

$ ein solve out/s5/bob1-after.ein --single      # the query
  query             (has Bob ?n Apple)
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?n = (n z 5)
                        “Bob has (n z 5) apples.”
```

The old program's rule read its own conclusion, a row a pass (E-124); the new one asks the calculator nothing, so
`diff-of` never holds and its rule never fires, and only the stated fact answers. The gold is 3: wrong both times,
for different reasons, and the contract's check finds neither (a rule that reads a result no rule asks for is none of
its seven findings; the record files nothing, S2.2d.4 is where it would go). **`zebra-contradiction`** is still
*right for a wrong reason* (P2.2's [L-M2.22](../../findings/model/L-M2.22-a-fixed-text-scored-right-can-rest-on-a-contradiction-of-the-model-s-own.md)):
its root dies at step 2 on a contradiction of the model's own, not on the sentence the text adds, and over other facts
than before ([before](out/s5/zc-before-core.out), [after](out/s5/zc-core.out)):

<!-- output -->
```text
$ ein solve out/s5/zc-before.ein --no-search      # the program before the re-recording
  unsat core        (instance Englishman Person)
  unsat core        (instance First House)
  unsat core        (is-immediately-to-the-right-of First Second)
  refuted by        std.contradiction.clash-2

$ ein solve einaf/tests/fixtures/from_text/zebra-contradiction/program.ein --no-search      # after
  unsat core        (instance House2 House)
  unsat core        (instance Norwegian Person)
  unsat core        (lives-in-next-to Norwegian Blue)
  refuted by        std.contradiction.clash-2
```

### 3.6 S2.2e.7: a hole and a whole node, dead on the link

**The hole.** `dead_link_hole.ein` puts `dead_link.ein`'s thirty towns apart only where gate a is open and gate b is
not, an `absent`, so the static check flags it, and owes a gate c. Its cube is eight nodes; R is three, and
{(gate a), (opened c hand)} is a hole, dead between two members, on the link. Drawn from an oracle dump
([§ 4](#4-renderings)): the purple ellipse. A description prints the hole's core (the read-out's `minimal hole` row)
and `--count` writes it in its summary; a line mode prints none, and since S2.2e.7 takes none. The program is flagged,
so every read-out of it opens with the warning that nothing is claimed of its answer; what is compared here is a
run's cost, and that its answer and its bytes did not move.

**The whole node.** `dead_link_whole.ein`: five people each owe a house, and Ann and Bob both in H1 put the two ends
apart, so the whole node, every candidate together, dies on the link. The obligation tree reads that node for the
coupling of its choices; `--events` and `--dump-lattice` print its core with every visited node's, and `-v` and
`--layer-progress` print a node's commitment and kind and no core, and since S2.2e.7 take none.

**Before**, quoted from [S2.2e.7's record](../s2.2e.7_two_more_cores_only_where_printed.md#t22e75--what-the-stage-found) (`1cbd071d`'s release binary, the code as
at `6b359cce`); **after**, today's binary; both pinned through `utils/bench_env.sh` (the wall includes its
fingerprint):

| fixture | question | clock | before: exit, wall (s) | after, today: exit, wall (s) | after, the answer's row or the abort |
|---|---|---|---|---|---|
| `dead_link_hole.ein` | `--maximal` | none | 0, 17.84 | 0, 0.19 | ``   maximal line      1 state — all of them `` |
|  | `--maximal` | `-T 5` | 2, 5.09 | 0, 0.20 | ``   maximal line      1 state — all of them `` |
|  | `--minimal` | none | 0, 17.90 | 0, 0.17 | ``   minimal line      1 state — all of them `` |
|  | `--minimal` | `-T 5` | 2, 5.11 | 0, 0.15 | ``   minimal line      1 state — all of them `` |
|  | `--maximal --single` | `-T 5` | 2, 5.07 | 0, 0.19 | ``   verdict           Solution — the maximal line is one state `` |
|  | `--count` | `-T 5` | 2, 5.10 | 2, 5.10 | `** aborted: --max-time 5 s reached before the question was decided **` |
|  | describe | none | 0, 17.92 | 0, 17.49 | ``   states (k_s)      3 `` |
| `dead_link_whole.ein` | `--single` | none | 0, 0.54 | 0, 0.55 | ``   verdict           Ambiguity — R holds two states or more; two of them below `` |
|  | `--single -v` | `-T 5` | 2, 18.24 | 0, 0.61 | ``   verdict           Ambiguity — R holds two states or more; two of them below `` |
|  | `--single --layer-progress` | `-T 5` | 2, 18.15 | 0, 0.59 | ``   verdict           Ambiguity — R holds two states or more; two of them below `` |
|  | `-n 2 -v` | `-T 5` | 2, 18.02 | 0, 0.59 | ``   states of R       2 states, as asked — R may hold more `` |
| `whole-node-clock.ein` (E-132's probe) | `--single --events E` | `-T 5` | — (E-132; the record's after, 2, 18.47) | 2, 17.61 | `** aborted: --max-time 5 s reached before the question was decided **` |

A description and `--count` of the hole fixture cost the core's search as before and are the clock's abort under
`-T 5`; `--single --events` on E-132's probe still runs past its clock, the whole node's core search reading none
([E-132](../../evaluation/bounds/E-132-the-whole-nodes-core-search-reads-no-clock.md), open). The record's `compare.sh` held every pair both binaries answered to the same stdout past its
header and the same summary.


## 4. Renderings

`ein render`, each digraph drawn to SVG by Graphviz (`dot`) as the script runs; every picture is under
[`out/render/`](out/render/), with its size in [`SIZES.tsv`](out/render/SIZES.tsv). `--fold` draws a fact of two
arguments as one labelled arrow; `ein render program` is the program as written (its knowledge base, a picture a
rule, its query), `ein render kb` the knowledge base saturated, `--query N` or `--goal` its slice that answers a goal.

**S2.2e.2, the rule before and after** (`ein render rule --name slot-prune-fwd` on the module as a program, each
library's): the `neq` between the placed end's position `?p1` and the excluded `?p_o` is gone, and the `absent`
decides that position as every other:

![slot-prune-fwd before the fix: its match reads S, R, isa, and a neq between ?p_o and ?p1 beside the absent of S from ?p_o to ?p1; it asserts not R of ?a and ?p_o](out/render/slot-prune-fwd-before.svg)

![slot-prune-fwd after the fix: the same match with no neq; the absent of S from ?p_o to ?p1 decides every position, the placed end's own among them](out/render/slot-prune-fwd-after.svg)

**S2.2e.2, test 14's exclusions** (`ein render kb --fold --query 1`), after: Ann out of the cat's seat S2, Bob out of
the dog's seat S1 and out of S3; before, Bob out of S3 alone:

![tests/stdlib/slots/14 after the fix: three excluded arrows, Ann to S2, Bob to S1, Bob to S3, each by probe-excluded](out/render/slots-14-excluded-after.svg)

![tests/stdlib/slots/14 on the library before the fix: one excluded arrow, Bob to S3](out/render/slots-14-excluded-before.svg)

**S2.2e.3, the fixture's shape** on three towns (`--goal '(link ?a ?b)'`, coloured by origin): each link drawn with
one rule that derives it, though the road rules, `mirror` and `chain` each derive many of them; on thirty towns
those derivations are what the core's search folds:

![dead_link on three towns: t1, t2, t3 in a row, links between neighbours by road1-links and mirror, and t1 to t3 both ways by chain](out/render/dead-link-3-towns-links.svg)

**S2.2e.4, root's fixpoint of the pair that differs by order** (`ein render kb --fold`): the reader first, `k` holds
`p` by `reader` and `q` by `producer`; the producer first, `k` holds `q` alone:

![gap-reader-first: the atom k with a loop p by reader and a loop q by producer, beside the given a and b](out/render/gap-reader-first-kb.svg)

![gap-producer-first: the atom k with a loop q by producer and no p](out/render/gap-producer-first-kb.svg)

**S2.2e.5, `bob1`'s answer rows before and after the re-recording** (`ein render kb --fold --query 1`): three `has`
facts, two by `giving-is-the-difference`, against the one stated:

![bob1 before: has Bob 5 Apple as stated, and has Bob 3 and has Bob 1 by giving-is-the-difference, each numeral an n of z and a digit](out/render/bob1-before-rows.svg)

![bob1 after: has Bob 5 Apple, the stated fact alone](out/render/bob1-after-rows.svg)

**S2.2e.7, the hole fixture's cube** (`ein render region` of an `ein oracle --dump`, from a copy with
`:record-alternative-justifications false`, which moves a dead node's core and nothing else, so that the oracle's
cores cost nothing): R green, its minimal line double-bordered and its maximal heavy; owing amber; dead red; the hole,
{(gate a), (opened c hand)}, purple, dead between `(opened c hand)` and `(gate a) (gate b) (opened c hand)`, with
the repair edge `+(gate b)`:

![dead_link_hole's cube: eight nodes from the empty set; R is (opened c hand), (gate b) (opened c hand) and all three; (gate a) with (opened c hand) is a hole, a dead node with a purple border between two members of R](out/render/dead-link-hole-region.svg)


**Every picture**:

| picture | what it shows |
|---|---|
| [feature-25/kb](out/render/feature-25/kb.svg), [query](out/render/feature-25/query.svg), [rule fond](out/render/feature-25/rule_fond_lhs_rhs.svg) | S2.2e.1's feature file as written: `likes`' three declarations, three facts of `relation`, beside `fan`'s |
| [feature-25-kb](out/render/feature-25-kb.svg) | the same, saturated: `fond`'s `(fan Ann)` |
| [slot-prune-fwd-before](out/render/slot-prune-fwd-before.svg), [after](out/render/slot-prune-fwd-after.svg) | the rule, each library's |
| [slots-14-excluded-before](out/render/slots-14-excluded-before.svg), [after](out/render/slots-14-excluded-after.svg), [slots-14-kb](out/render/slots-14-kb.svg) | test 14's exclusions, each library's; its whole saturated KB |
| [dead-link/](out/render/dead-link/) kb, a rule each (`road1-links`, `road2-links`, `mirror`, `chain`, `clash`) | the fixture as written, thirty towns |
| [dead-link-3-towns-links](out/render/dead-link-3-towns-links.svg) | its links on three towns |
| [gap-reader-first/](out/render/gap-reader-first/) kb, `reader`, `producer`; [gap-reader-first-kb](out/render/gap-reader-first-kb.svg), [gap-producer-first-kb](out/render/gap-producer-first-kb.svg) | the order pair: the rules (the `absent` each waits on), root's fixpoint of each order |
| [bob1-before/](out/render/bob1-before/), [bob1-after/](out/render/bob1-after/), [bob1-before-rows](out/render/bob1-before-rows.svg), [bob1-after-rows](out/render/bob1-after-rows.svg) | `bob1`'s programs, their rules, their answer rows |
| [dead-link-hole-region](out/render/dead-link-hole-region.svg) | S2.2e.7's hole fixture's cube, R, owing, dead, the hole |

The edge counts are in [`SIZES.tsv`](out/render/SIZES.tsv); the largest is the fixture's thirty towns as written. **Left
out, and why**: the saturated KB of the fixture and of the model's Zebra program (hundreds of links, too many edges to
read), `ein render lattice` of either (root is dead: one red node), and of `three-houses.ein` (1 571 saturations);
`ein render h-space` drawn (its read-out is in [§ 5](#5-the-numbers)).

## 5. The numbers

**S2.2e.6, the model's Zebra program, every question**: the table of
[§ 3.3](#33-s22e3-and-s22e6-the-models-zebra-program-question-by-question); the script's own,
[`out/s3/questions.tsv`](out/s3/questions.tsv), has each summary's `region.core` beside it: a description's root
core, and `null` under every mode and `--count`:

<!-- output -->
```text
question	clock	exit	wall s (bench_env's fingerprint included)	the answer's row	unsat core rows	region.core
describe	none	0	71.71	empty every node is dead — root's own fixpoint holds a contradiction	2	["(instance GreenHouse House)", "(type Color Attribute)"]
count	none	0	0.18	empty every node is dead — root's own fixpoint holds a contradiction	0	null
```

**S2.2e.3, what the clock costs where it is not reached**, quoted from
[its record](../s2.2e.3_clock_and_a_dead_root.md#t22e32--the-clock-there) (on AC, one P-core thread, before the
binary of `44e3bef1`, after the fix, interleaved; not re-taken here, since it needs the record's probe built against
two commits):

| what | before | after, no clock | after, a clock not reached |
|---|---|---|---|
| the model's program, `ein solve --single` (before S2.2e.6), two rounds | 72.03 s, 72.01 s | 72.04 s, 72.85 s | 72.28 s, 72.26 s (`-T 1000`) |
| every corpus program that is not `slow`, 740, described with every visited dead node's core taken, three rounds | 14.45 s, 14.40 s, 14.44 s | 14.51 s, 14.26 s, 14.26 s | 14.35 s, 14.28 s, 14.25 s |

The fixture's family (S2.2e.3's record, the core's search under `--no-search`): 10 towns 0.14 s, 20 3.1 s, 30
17.1 s, 40 40.2 s; root's saturation 30 ms at thirty. Today's description of the thirty-town fixture took
17.29 s ([§ 3.3](#33-s22e3-and-s22e6-the-models-zebra-program-question-by-question)).

**S2.2e.2, the six encodings with sentence (15) dropped**: [§ 2.2](#22-s22e2-stdslots-two-neighbours-do-not-share-a-place)'s
`six.py` lines, each library; the record's table has the same states, classes and work.

**S2.2e.2, the structure of H on `zebra.ein`**, `ein render h-space`, today's binary on each library, and
`zebra2.ein`'s ([after](out/s2/h-space-zebra.out), [before](out/s2/h-space-zebra-old.out),
[zebra2](out/s2/h-space-zebra2.out)): candidates dead alone 36 → 42, and of the pairs excluded at root, the theory's
and the puzzle's, 224 and 1 432 → 180 and 1 128, `zebra2.ein`'s counts:

<!-- output -->
```text
$ ein render h-space examples/zebra.ein      # H and theory
H            112 candidates · 42 dead alone · 0 at root · from `union`
theory       512 pairs excluded by the theory alone · of root's 1 308, 180 the theory's and 1 128 the puzzle's · the small KB drops 18 of 66 stated facts and kills 0 candidates alone

$ env EIN_STDLIB=out/s2/old/stdlib ein render h-space examples/zebra.ein
H            112 candidates · 36 dead alone · 0 at root · from `union`
theory       512 pairs excluded by the theory alone · of root's 1 656, 224 the theory's and 1 432 the puzzle's · the small KB drops 18 of 66 stated facts and kills 0 candidates alone

$ ein render h-space examples/zebra2.ein
H            56 candidates · 21 dead alone · 0 at root · from `union`
theory       128 pairs excluded by the theory alone · of root's 327, 45 the theory's and 282 the puzzle's · the small KB drops 18 of 71 stated facts and kills 0 candidates alone
```

**S2.2e.2's work on the full puzzle**, quoted from [its record](../s2.2e.2_slots_neighbours.md#t22e23--what-moves):
a description of each `std.slots` encoding, the same 353 saturations and 1.5 % to 2.3 % more facts beyond root
(`zebra.ein` 34 060 → 34 580); `--single` unchanged but 8 facts on `zebra3.2.ein`. The mutation survivors of the slots
rules: 8 lines fewer, none new (`utils/stdlib_mutants.py -k slot`, 58 of 92 killed).

**S2.2e.7, the two fixtures**: the table of
[§ 3.6](#36-s22e7-a-hole-and-a-whole-node-dead-on-the-link). On the corpus, quoted from
[its record](../s2.2e.7_two_more_cores_only_where_printed.md#t22e71--the-two-routes-and-a-program-for-each): no program
shows a cost. Every corpus program outside the slow set under `--maximal` and `--minimal`, by both binaries: 1 850
runs, no stdout past its header and no exit different, the 106 that take the cube 15.88 s before and 15.94 s after in
all; the corpus's one program with a whole node the tree reads dead, `send_more_money.ein`, has a cheap core
(`--single` 0.52 s, with `-v` 0.53 s).

## 6. The defaults taken, for the user to correct

Each stage numbered its defaults in its record, with the reason for each: **48 in all** — the plan's
[5](../README.md#defaults-taken-at-the-plan), S2.2e.1 [4](../s2.2e.1_declared_again_under_an_import.md#defaults-taken),
S2.2e.2 [6](../s2.2e.2_slots_neighbours.md#defaults-taken), S2.2e.3 [6](../s2.2e.3_clock_and_a_dead_root.md#defaults-taken),
S2.2e.4 [6](../s2.2e.4_order_and_table_rows.md#defaults-taken), S2.2e.5 [4](../s2.2e.5_recordings_follow_the_library.md#defaults-taken),
S2.2e.6 [9](../s2.2e.6_a_core_only_where_printed.md#defaults-taken), S2.2e.7 [8](../s2.2e.7_two_more_cores_only_where_printed.md#defaults-taken). **The ones that matter most:**

1. **A phase of its own, before P2.3** (the plan's 1;
   [Q-M2.42](../../open_questions/kernel/Q-M2.42-are-e-104-and-e-105-fixed-before-the-benchmark-is-built.md)'s
   default, the question still open): E-104 and E-105 fixed before the benchmark is built, where the frame's default
   would have made each a task of the stage that needs it.
2. **`std.slots`' guard goes for every `S`, and the page gains no condition** (S2.2e.2, 1): the page states
   $S(a, b) \iff S(\operatorname{slot}(a), \operatorname{slot}(b))$ with no condition, and the rule reads `S` at the
   placed end's position, so a reflexive `S` keeps it (test 15) and an irreflexive one excludes it (test 14).
3. **The clock's unit is the search's work**, 4 096 units a read (a fact read, a justification folded, an environment
   made), a run stopping within some milliseconds of `-T` (S2.2e.3, 2). No decision: Q-M2.16 already says a run the
   clock stops is no answer, wherever (S2.2e.3, 3).
4. **What may move under another order is listed whole in the decision**, five things (S2.2e.4, 3), so that none is
   later filed as a breach. The guarantee stated for every program (S2.2e.4, 1) is no longer a default: the user kept
   it, *"keep, as recommended"*.
5. **The catalogue's digest moves by digest, no template renamed** (S2.2e.5, 1), and the library's digest is a field of
   the run's record and of each step's, not of the model's call records (S2.2e.5, 2), which know nothing of a library.
6. **`ein test` asks root's core only where its `--json-summary` writes it** (S2.2e.6, 4), a step past the stage's
   acceptance: a `(solve :expect (false))` on such a program is decided by `ein test` in what root costs
   ([§ 3.3](#33-s22e3-and-s22e6-the-models-zebra-program-question-by-question): `ok` without a summary, `ERROR` with
   one under the clock).
7. **The core a question takes is a field of `region::Cores`, `root`, read against the question in one place**
   (S2.2e.6, 1 and 2), before `--count` takes describe's route.
8. **Filed, not fixed**: E-129, a rule or a macro written twice read once under an import (S2.2e.1, 1: its fix has two
   shapes, folding everywhere or refusing everywhere); E-131 (S2.2e.6, 3), which S2.2e.7 then took on the user's word.
9. **Whether EinAf's call sets the lever** that keeps one justification a fact stays
   [Q-M2.34](../../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md)'s
   (the plan's 2), and since S2.2e.6 it bears on a description and `--no-search` alone.
10. **The whole node's core is taken where the run is narrated *and* asks every visited node's core** (S2.2e.7, 1),
    which is where a surface prints it (`--events`, `--dump-lattice`); `ein test` asks the holes' cores only where its
    summary writes them (S2.2e.7, 3), as it asks root's; **E-132 filed, not fixed** (S2.2e.7, 6): the clock's reach,
    not which cores a question takes.

## 7. Questions for the user

**The phase holds no open question of its own**: Q-M2.43, which S2.2e.1 raised, moved to
[P2.2g](../../p2.2g_names_across_an_import/README.md), the phase the user planned for it
([its file](../../p2.2g_names_across_an_import/open_questions/kernel/Q-M2.43-is-a-programs-declaration-of-a-modules-relation-a-second-declaration-of-it.md)),
with the user's correction; [§ 3.1](#31-s22e1-the-models-unused-refused-then-loading-now) shows what Ein does today.
What waits for the user's word, from the phase:

| item | what | the default taken |
|---|---|---|
| **closing the phase** | the user's word; at the close the open entries move to the milestone's ledgers | not closed |
| [Q-M2.42](../../open_questions/kernel/Q-M2.42-are-e-104-and-e-105-fixed-before-the-benchmark-is-built.md) (milestone) | Are E-104 and E-105 fixed before the benchmark is built? | yes, by this phase; fixed. The question is open in the ledger, for the user to rule it as done |
| [E-129](../../evaluation/language/E-129-a-rule-or-a-macro-written-twice-the-same-is-read-once-where-the-program-imports.md) | a rule or a macro written twice the same is read once under an import, refused with none; two bodies refused by two different lines | open, no owner; a fix leaves Ein's semantics as they are, so a stage may make it on the user's word |
| [Q-M2.34](../../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md) (milestone) | Does EinAf's kernel call set the lever that keeps one justification a fact? | no; since S2.2e.6 it bears on a description and `--no-search` alone ([§ 3.3](#33-s22e3-and-s22e6-the-models-zebra-program-question-by-question)) |
| [L-M2.33](../../findings/einaf/L-M2.33-a-replayed-slim-record-written-again-loses-each-calls-token-digest.md) | a replayed slim record, written again, loses each call's token digest | open; S2.5.1's by default |
| a test of Q-M2.24's guarantee | a sweep of the corpus in another order | none, as ruled; waits for the user's word |
| [E-132](../../evaluation/bounds/E-132-the-whole-nodes-core-search-reads-no-clock.md) | the whole node's core search reads no clock, so under `--events` or `--dump-lattice` a run whose whole node dies on such a link runs past `-T` ([§ 3.6](#36-s22e7-a-hole-and-a-whole-node-dead-on-the-link)) | open, no owner; whether it gets a stage is the user's word |

## 8. What the phase found

**Of Ein**, in [the phase's evaluation](../evaluation/README.md) (the milestone's next free id is E-133):

| id | what | kind | status |
|---|---|---|---|
| [E-129](../../evaluation/language/E-129-a-rule-or-a-macro-written-twice-the-same-is-read-once-where-the-program-imports.md) | a rule or a macro written twice the same is read once where the program imports, refused where it imports none | bug | open, no owner |
| [E-130](../evaluation/bounds/E-130-a-question-that-prints-no-core-takes-a-dead-roots-core.md) | a question that prints no core takes a dead root's core all the same | bug | closed by S2.2e.6 (`6b359cce`) |
| [E-131](../evaluation/bounds/E-131-two-more-cores-a-question-takes-and-prints-nowhere.md) | two more cores taken and printed nowhere: the cube's minimal holes' under a line mode, the whole node's under `-v` or `--layer-progress` | bug | closed by S2.2e.7 (`87d5db47`) |
| [E-132](../../evaluation/bounds/E-132-the-whole-nodes-core-search-reads-no-clock.md) | the whole node's core search reads no clock, so `--events` or `--dump-lattice` on a run whose whole node dies on the link runs past `-T` | bug | open, no owner |

**The milestone's entries the phase closed or changed**: [E-104](../../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md)
closed by S2.2e.1, [E-105](../../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md)
by S2.2e.2, [E-126](../../evaluation/bounds/E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md)
by S2.2e.3, [E-121](../../evaluation/language/E-121-the-banked-table-of-refused-shapes-misses-two-of-the-lexer-s-token-ends.md)
by S2.2e.4; [E-112](../../evaluation/language/E-112-a-program-s-forms-may-be-written-in-the-order-they-are-made.md) a
strength Ein states, by Q-M2.24's decision; noted where they stand,
[E-113](../../evaluation/bounds/E-113-a-dead-state-s-core-costs-minutes-on-a-link-re-derived-many.md) (what the
smallest core costs, open, F22's), [E-98](../../evaluation/bounds/E-98-the-clock-is-not-read-where-nothing-fires-and-the-cap-counts.md)
(two works still outside the clock, open) and
[E-124](../../evaluation/answer/E-124-one-state-may-answer-a-question-of-one-value-with-a-row-a-pass.md) (its evidence,
`bob1`'s old fixture, re-read). No row of the wrong-answer register was filed or fixed.

**Of EinAf and the process**, in [the phase's findings](../findings/README.md):

| id | what | status |
|---|---|---|
| [L-M2.31](../findings/einaf/L-M2.31-einafs-recorded-model-runs-carry-the-librarys-digest.md) | EinAf's recorded model runs carried the library's digest, so a change to `stdlib/` failed three of its tests | closed by S2.2e.5 |
| [L-M2.32](../../findings/wrong-answers/L-M2.32-the-guarantee-of-q-m2-24-has-a-known-wrong-answer-against-it.md) | the guarantee of Q-M2.24 has a known wrong answer against it, `W8/fifo-gap-root` | open, a known wrong answer; the guarantee kept whole on the user's word; owner the register row's, S1n.2.1 (M1n) |
| [L-M2.33](../../findings/einaf/L-M2.33-a-replayed-slim-record-written-again-loses-each-calls-token-digest.md) | a slim record replayed and written again loses each call's token digest | open; S2.5.1's by default |

**Found by this report's re-runs**: nothing that contradicts a record. Every number the records state that this
report re-took came out the same (S2.2e.2's 55 and 32, the classes, the work on each encoding, `three-houses.ein`'s
16 and 12, the three counts of H's structure; S2.2e.5's replays, cores and probe line; S2.2e.6's answers and exits
under each question; S2.2e.7's answers and exits on its two fixtures, and its after walls within a second of
today's). Two things the records do not say, both consistent with them: the model's `unused` of the
module-dir condition, which now loads, has a dead root (*inconsistent*, its own `std.slots` selection); and
`zebra-contradiction`'s program before the re-recording, run today, dies on the same core its record names.

## 9. What is left

- **The nightly**, after this report, the coordinator's; the gate is green on the phase's last commit (the header).
- **Closing the phase**, when the user says so: by the ledgers' rules, E-129 and E-132 and the findings
  L-M2.32 and L-M2.33 move to the milestone's `evaluation/` and `findings/`; E-130, E-131, L-M2.31 and the stages'
  records stay in the folder, which M2 keeps.
- **E-129**, open, no owner (§ 7).
- **A description and `--no-search` of a root that dies on a link several rules derive still cost the core's
  search** (72 s on the model's Zebra program, the clock's abort under one): E-113 and F22, what the smallest core
  costs; Q-M2.34, the lever.
- **E-98**'s two works outside the clock (the matcher's join that fires nothing, the blind enumerator), untouched.
- **L-M2.32**: what Q-M2.24 changes of `W8/fifo-gap-root`'s readings and of Q-M1f.313's *Why*, M1n's (S1n.2.1,
  P1n.2); no test holds the guarantee, as ruled.
- **L-M2.33**, S2.5.1's; **E-120** (a string glued to a block comment, refused), the lexer's, for the iteration of Ein.
- **P2.2's other records** (`plans/m2_nl_to_ir/p2.2_formalizer/`) were made against prompts that showed the library's
  digest; since S2.2e.2 none that showed the catalogue replays, and they are of their time (S2.2e.5's record).
- **P2.3**, which the phase blocks, can start on the user's word: the benchmark's attribute programs select `std.slots`
  as it now is.
- **[E-132](../../evaluation/bounds/E-132-the-whole-nodes-core-search-reads-no-clock.md)**, the whole node's core search, which reads no clock under `--events` or `--dump-lattice`:
  open, the user's word whether it gets a stage. A description and `--count` of a program whose minimal hole dies on
  such a link, and `--events` or `--dump-lattice` of one whose whole node does, still cost the core's search, E-113's
  and F22's.

## 10. Housekeeping disclosures

- **No model ran and no GPU was used** by this report: every model call is a replay of a committed record, every
  command ran with `CUDA_VISIBLE_DEVICES` empty, and `nvidia-smi` showed the desktop's compositor alone before the run
  ([`out/RUN.txt`](out/RUN.txt)). S2.2e.5's recording is the phase's one use of the model, 13:52 to 13:54, recorded in
  its record.
- **Writes**: this report wrote under `plans/m2_nl_to_ir/p2.2e_kernel_fixes/report/` alone (`out/`, which git
  ignores, the library copies of `out/s2/old/` and `out/s5/moved/` among it); the probe's `mktemp` went to `out/tmp/`,
  removed at the end. Two `--quick` trial runs, while the script was written, wrote the same folder; the full run
  starts it empty. Each output block of this page was pasted from `out/` by a small filler (a marker a block, the
  file and the lines it takes), not typed.
- **`~/.cargo/.global-cache`** was written during S2.2e.5 (its record, *Housekeeping*): Part 1's cargo commands ran
  `--offline` with the default `CARGO_HOME`, fetching nothing, and cargo wrote its record of last use there; it was
  written again at 13:51:29, before Part 2's first cargo command, by something else on the host. Nothing to remove: it is cargo's own
  bookkeeping.
- **The probes' outputs** under each `probes/*/out/` are the stages' runs, ignored by git as this report's are.
- **The binaries were rebuilt for this run**, from HEAD's code (`87d5db47`'s), since the gate builds only
  `target/gate/`: `cargo build --manifest-path ein.rs/Cargo.toml --workspace --release --offline` and
  `CMAKE_CUDA_ARCHITECTURES=89 cargo build --offline --release --features cuda --manifest-path einaf/Cargo.toml`, each
  with the scratch `CARGO_HOME` of S2.2.2, nothing fetched; compiling the CUDA build uses no GPU.

## 11. Every command make_report.sh runs

Every command `make_report.sh` ran on this checkout, in its order, from [`out/INDEX.tsv`](out/INDEX.tsv): its exit
code and wall seconds, with what it printed. `$P` is `plans/m2_nl_to_ir/p2.2e_kernel_fixes`, `$P22`
`plans/m2_nl_to_ir/p2.2_formalizer`, `$Z` the model's Zebra program and `$DL` the fixture; `out/` is `$P/report/out/`.
A non-zero exit is each command's own: a load or parse error (1), a refuted claim (1), the clock's abort (2), `diff`
and `cmp` finding a difference (1), as each row's output says.

| # | output | command | exit | s |
|---:|---|---|---:|---:|
| 1 | `s1/feature-25-solve` [out](out/s1/feature-25-solve.out) | `ein solve examples/features/25_declared_twice_with_an_import.ein` | 0 | 0.01 |
| 2 | `s1/feature-25-test` [out](out/s1/feature-25-test.out) | `ein test examples/features/25_declared_twice_with_an_import.ein` | 0 | 0.01 |
| 3 | `s1/e104-two-counts` [out](out/s1/e104-two-counts.out) | `ein solve $P22/zebra_formulations/probes/language/declared-at-two-counts.ein` | 0 | 0.01 |
| 4 | `s1/e104-two-counts-import` [out](out/s1/e104-two-counts-import.out) | `ein solve $P22/zebra_formulations/probes/language/declared-at-two-counts-with-an-import.ein` | 0 | 0.02 |
| 5 | `s1/zebra3.2-the` [out](out/s1/zebra3.2-the.out) | `grep -n '^(relation the ' examples/zebra3.2.ein` | 0 | 0.00 |
| 6 | `s1/zebra3.2-single` [out](out/s1/zebra3.2-single.out) | `ein solve examples/zebra3.2.ein --single` | 0 | 0.04 |
| 7 | `s1/unused-library-recorded` [out](out/s1/unused-library-recorded.out) | `grep -h -- '--no-search: invalid' $P22/from_text/runs/library/unused/stderr.txt` | 0 | 0.00 |
| 8 | `s1/unused-library-outcome` [out](out/s1/unused-library-outcome.out) | `einaf outcome $P22/from_text/runs/library/unused/program.ein --no-search` | 0 | 0.09 |
| 9 | `s1/unused-library-single` [out](out/s1/unused-library-single.out) | `einaf outcome $P22/from_text/runs/library/unused/program.ein --single` | 0 | 0.08 |
| 10 | `s1/unused-module-dir-recorded` [out](out/s1/unused-module-dir-recorded.out) | `grep -h -- '--no-search: invalid' $P22/from_text/runs/module-dir/unused/stderr.txt` | 0 | 0.00 |
| 11 | `s1/unused-module-dir-outcome` [out](out/s1/unused-module-dir-outcome.out) | `einaf outcome $P22/from_text/runs/module-dir/unused/program.ein --no-search` | 0 | 0.05 |
| 12 | `s1/unused-module-dir-single` [out](out/s1/unused-module-dir-single.out) | `einaf outcome $P22/from_text/runs/module-dir/unused/program.ein --single` | 0 | 0.05 |
| 13 | `s1/e129-rule-twice-the-same` [err](out/s1/e129-rule-twice-the-same.err) | `ein solve $P/probes/rule-twice-the-same.ein` | 1 | 0.01 |
| 14 | `s1/e129-rule-twice-the-same-with-an-import` [out](out/s1/e129-rule-twice-the-same-with-an-import.out) | `ein solve $P/probes/rule-twice-the-same-with-an-import.ein` | 0 | 0.02 |
| 15 | `s1/e129-rule-twice-two-bodies` [err](out/s1/e129-rule-twice-two-bodies.err) | `ein solve $P/probes/rule-twice-two-bodies.ein` | 1 | 0.01 |
| 16 | `s1/e129-rule-twice-two-bodies-with-an-import` [err](out/s1/e129-rule-twice-two-bodies-with-an-import.err) | `ein solve $P/probes/rule-twice-two-bodies-with-an-import.ein` | 1 | 0.01 |
| 17 | `s1/e129-macro-twice-the-same` [err](out/s1/e129-macro-twice-the-same.err) | `ein solve $P/probes/macro-twice-the-same.ein` | 1 | 0.01 |
| 18 | `s1/e129-macro-twice-the-same-with-an-import` [out](out/s1/e129-macro-twice-the-same-with-an-import.out) | `ein solve $P/probes/macro-twice-the-same-with-an-import.ein` | 0 | 0.02 |
| 19 | `s1/e129-macro-twice-two-bodies` [err](out/s1/e129-macro-twice-two-bodies.err) | `ein solve $P/probes/macro-twice-two-bodies.ein` | 1 | 0.01 |
| 20 | `s1/e129-macro-twice-two-bodies-with-an-import` [err](out/s1/e129-macro-twice-two-bodies-with-an-import.err) | `ein solve $P/probes/macro-twice-two-bodies-with-an-import.ein` | 1 | 0.01 |
| 21 | `s1/q-m2.43-listed` [err](out/s1/q-m2.43-listed.err) | `ein solve $P/probes/a-modules-relation/listed.ein` | 1 | 0.01 |
| 22 | `s1/q-m2.43-aliased` [out](out/s1/q-m2.43-aliased.out) | `ein solve $P/probes/a-modules-relation/aliased.ein` | 0 | 0.01 |
| 23 | `s1/q-m2.43-aliased-undeclared` [out](out/s1/q-m2.43-aliased-undeclared.out) | `ein solve $P/probes/a-modules-relation/aliased-undeclared.ein` | 0 | 0.01 |
| 24 | `s1/q-m2.43-full-name` [err](out/s1/q-m2.43-full-name.err) | `ein solve $P/probes/a-modules-relation/full-name.ein` | 1 | 0.01 |
| 25 | `s1/q-m2.43-aliased-saturate` [out](out/s1/q-m2.43-aliased-saturate.out) | `ein saturate $P/probes/a-modules-relation/aliased.ein --dump` | 0 | 0.01 |
| 26 | `s1/q-m2.43-aliased-undeclared-saturate` [out](out/s1/q-m2.43-aliased-undeclared-saturate.out) | `ein saturate $P/probes/a-modules-relation/aliased-undeclared.ein --dump` | 0 | 0.01 |
| 27 | `s2/slots-fix-diff` [out](out/s2/slots-fix-diff.out) | `git diff 0ab06c69 b2d8276f -- stdlib/slots.ein` | 0 | 0.01 |
| 28 | `s2/version-old-library` [out](out/s2/version-old-library.out) | `env EIN_STDLIB=out/s2/old/stdlib ein --version` | 0 | 0.00 |
| 29 | `s2/six-after` [out](out/s2/six-after.out) | `env EIN_BIN=ein.rs/target/release/ein python3 $P/probes/slots-neighbours/six.py out/s2/six-after` | 0 | 1.41 |
| 30 | `s2/six-before` [out](out/s2/six-before.out) | `env EIN_BIN=ein.rs/target/release/ein python3 $P/probes/slots-neighbours/six.py out/s2/six-before --stdlib out/s2/old/stdlib` | 0 | 1.85 |
| 31 | `s2/zebra3.1-drop15-maximal` [out](out/s2/zebra3.1-drop15-maximal.out) | `ein solve out/s2/six-after/zebra3.1.drop-15.ein --maximal` | 0 | 0.21 |
| 32 | `s2/zebra3.1-drop15-maximal-old` [out](out/s2/zebra3.1-drop15-maximal-old.out) | `env EIN_STDLIB=out/s2/old/stdlib ein solve out/s2/six-after/zebra3.1.drop-15.ein --maximal` | 0 | 0.33 |
| 33 | `s2/zebra2-minus-15-maximal` [out](out/s2/zebra2-minus-15-maximal.out) | `ein solve examples/zebra2-minus-15.ein --maximal` | 0 | 0.12 |
| 34 | `s2/zebra3.1-single` [out](out/s2/zebra3.1-single.out) | `ein solve examples/zebra3.1.ein --single` | 0 | 0.04 |
| 35 | `s2/nl2smt-minus-15` [out](out/s2/nl2smt-minus-15.out) | `python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra-minus-15.txt --solutions 100` | 0 | 14.87 |
| 36 | `s2/three-houses-brute` [out](out/s2/three-houses-brute.out) | `python3 $P/probes/slots-neighbours/three_houses.py` | 0 | 0.02 |
| 37 | `s2/three-houses-count` [out](out/s2/three-houses-count.out) | `ein solve $P/probes/slots-neighbours/three-houses.ein --count` | 0 | 0.47 |
| 38 | `s2/three-houses-count-old` [out](out/s2/three-houses-count-old.out) | `env EIN_STDLIB=out/s2/old/stdlib ein solve $P/probes/slots-neighbours/three-houses.ein --count` | 0 | 0.54 |
| 39 | `s2/three-houses-describe` [out](out/s2/three-houses-describe.out) | `ein solve $P/probes/slots-neighbours/three-houses.ein --dump-answer out/s2/three-houses-answer` | 0 | 0.48 |
| 40 | `s2/three-houses-describe-old` [out](out/s2/three-houses-describe-old.out) | `env EIN_STDLIB=out/s2/old/stdlib ein solve $P/probes/slots-neighbours/three-houses.ein --dump-answer out/s2/three-houses-answer-old` | 0 | 0.54 |
| 41 | `s2/tests-slots` [out](out/s2/tests-slots.out) | `ein test tests/stdlib/slots` | 0 | 0.17 |
| 42 | `s2/test-14_prune_placed_position` [out](out/s2/test-14_prune_placed_position.out) | `ein test tests/stdlib/slots/14_prune_placed_position.ein` | 0 | 0.02 |
| 43 | `s2/test-14_prune_placed_position-old` [out](out/s2/test-14_prune_placed_position-old.out) | `env EIN_STDLIB=out/s2/old/stdlib ein test tests/stdlib/slots/14_prune_placed_position.ein` | 1 | 0.02 |
| 44 | `s2/test-15_prune_reflexive` [out](out/s2/test-15_prune_reflexive.out) | `ein test tests/stdlib/slots/15_prune_reflexive.ein` | 0 | 0.02 |
| 45 | `s2/test-15_prune_reflexive-old` [out](out/s2/test-15_prune_reflexive-old.out) | `env EIN_STDLIB=out/s2/old/stdlib ein test tests/stdlib/slots/15_prune_reflexive.ein` | 0 | 0.02 |
| 46 | `s2/test-16_spatial_endpoints` [out](out/s2/test-16_spatial_endpoints.out) | `ein test tests/stdlib/slots/16_spatial_endpoints.ein` | 0 | 0.02 |
| 47 | `s2/test-16_spatial_endpoints-old` [out](out/s2/test-16_spatial_endpoints-old.out) | `env EIN_STDLIB=out/s2/old/stdlib ein test tests/stdlib/slots/16_spatial_endpoints.ein` | 0 | 0.02 |
| 48 | `s2/solve-14` [out](out/s2/solve-14.out) | `ein solve tests/stdlib/slots/14_prune_placed_position.ein` | 0 | 0.02 |
| 49 | `s2/solve-14-old` [out](out/s2/solve-14-old.out) · [err](out/s2/solve-14-old.err) | `env EIN_STDLIB=out/s2/old/stdlib ein solve tests/stdlib/slots/14_prune_placed_position.ein` | 1 | 0.02 |
| 50 | `s2/h-space-zebra` [out](out/s2/h-space-zebra.out) | `ein render h-space examples/zebra.ein` | 0 | 6.93 |
| 51 | `s2/h-space-zebra-old` [out](out/s2/h-space-zebra-old.out) | `env EIN_STDLIB=out/s2/old/stdlib ein render h-space examples/zebra.ein` | 0 | 8.13 |
| 52 | `s2/h-space-zebra2` [out](out/s2/h-space-zebra2.out) | `ein render h-space examples/zebra2.ein` | 0 | 1.11 |
| 53 | `s3/q/describe-none` [out](out/s3/q/describe-none.out) · [err](out/s3/q/describe-none.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --json-summary out/s3/q/describe-none.json` | 0 | 71.71 |
| 54 | `s3/q/single-none` [out](out/s3/q/single-none.out) · [err](out/s3/q/single-none.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --single --json-summary out/s3/q/single-none.json` | 0 | 0.17 |
| 55 | `s3/q/n2-none` [out](out/s3/q/n2-none.out) · [err](out/s3/q/n2-none.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z -n 2 --json-summary out/s3/q/n2-none.json` | 0 | 0.16 |
| 56 | `s3/q/minimal-none` [out](out/s3/q/minimal-none.out) · [err](out/s3/q/minimal-none.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --minimal --json-summary out/s3/q/minimal-none.json` | 0 | 0.17 |
| 57 | `s3/q/maximal-none` [out](out/s3/q/maximal-none.out) · [err](out/s3/q/maximal-none.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --maximal --json-summary out/s3/q/maximal-none.json` | 0 | 0.17 |
| 58 | `s3/q/count-none` [out](out/s3/q/count-none.out) · [err](out/s3/q/count-none.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --count --json-summary out/s3/q/count-none.json` | 0 | 0.18 |
| 59 | `s3/q/no-search-none` [out](out/s3/q/no-search-none.out) · [err](out/s3/q/no-search-none.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --no-search --json-summary out/s3/q/no-search-none.json` | 0 | 71.73 |
| 60 | `s3/q/describe-5` [err](out/s3/q/describe-5.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z -T 5 --json-summary out/s3/q/describe-5.json` | 2 | 5.10 |
| 61 | `s3/q/single-5` [out](out/s3/q/single-5.out) · [err](out/s3/q/single-5.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --single -T 5 --json-summary out/s3/q/single-5.json` | 0 | 0.17 |
| 62 | `s3/q/n2-5` [out](out/s3/q/n2-5.out) · [err](out/s3/q/n2-5.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z -n 2 -T 5 --json-summary out/s3/q/n2-5.json` | 0 | 0.18 |
| 63 | `s3/q/minimal-5` [out](out/s3/q/minimal-5.out) · [err](out/s3/q/minimal-5.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --minimal -T 5 --json-summary out/s3/q/minimal-5.json` | 0 | 0.16 |
| 64 | `s3/q/maximal-5` [out](out/s3/q/maximal-5.out) · [err](out/s3/q/maximal-5.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --maximal -T 5 --json-summary out/s3/q/maximal-5.json` | 0 | 0.18 |
| 65 | `s3/q/count-5` [out](out/s3/q/count-5.out) · [err](out/s3/q/count-5.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --count -T 5 --json-summary out/s3/q/count-5.json` | 0 | 0.15 |
| 66 | `s3/q/no-search-5` [err](out/s3/q/no-search-5.err) | `utils/bench_env.sh --cores PT:1 ein solve $Z --no-search -T 5 --json-summary out/s3/q/no-search-5.json` | 2 | 5.11 |
| 67 | `s3/model-zebra-single-timing` [out](out/s3/model-zebra-single-timing.out) | `ein solve $Z --single -t` | 0 | 0.07 |
| 68 | `s3/model-zebra-lever-describe` [out](out/s3/model-zebra-lever-describe.out) | `ein solve out/s3/model-zebra-lever.ein` | 0 | 0.06 |
| 69 | `s3/dead-link-describe` [out](out/s3/dead-link-describe.out) | `ein solve $DL` | 0 | 17.29 |
| 70 | `s3/dead-link-describe-T0.3` [err](out/s3/dead-link-describe-T0.3.err) | `ein solve $DL -T 0.3` | 2 | 0.31 |
| 71 | `s3/dead-link-single-T0.3` [out](out/s3/dead-link-single-T0.3.out) · [err](out/s3/dead-link-single-T0.3.err) | `ein solve $DL --single -T 0.3` | 0 | 0.03 |
| 72 | `s3/dead-link-count-T0.3` [out](out/s3/dead-link-count-T0.3.out) · [err](out/s3/dead-link-count-T0.3.err) | `ein solve $DL --count -T 0.3` | 0 | 0.03 |
| 73 | `s3/dead-link-n2-T0.3` [out](out/s3/dead-link-n2-T0.3.out) · [err](out/s3/dead-link-n2-T0.3.err) | `ein solve $DL -n 2 -T 0.3` | 0 | 0.03 |
| 74 | `s3/dead-link-minimal-T0.3` [out](out/s3/dead-link-minimal-T0.3.out) · [err](out/s3/dead-link-minimal-T0.3.err) | `ein solve $DL --minimal -T 0.3` | 0 | 0.03 |
| 75 | `s3/dead-link-maximal-T0.3` [out](out/s3/dead-link-maximal-T0.3.out) · [err](out/s3/dead-link-maximal-T0.3.err) | `ein solve $DL --maximal -T 0.3` | 0 | 0.03 |
| 76 | `s3/dead-link-no-search-T0.3` [err](out/s3/dead-link-no-search-T0.3.err) | `ein solve $DL --no-search -T 0.3` | 2 | 0.31 |
| 77 | `s3/dead-link-test-T0.3` [out](out/s3/dead-link-test-T0.3.out) | `ein test $DL -T 0.3` | 0 | 0.03 |
| 78 | `s3/dead-link-test-summary-T0.3` [out](out/s3/dead-link-test-summary-T0.3.out) · [err](out/s3/dead-link-test-summary-T0.3.err) | `ein test $DL -T 0.3 --json-summary out/s3/dead-link-test-summary.json` | 2 | 0.31 |
| 79 | `s3/dead-link-oracle-T0.3` [err](out/s3/dead-link-oracle-T0.3.err) | `ein oracle $DL -T 0.3` | 2 | 0.31 |
| 80 | `s3/dead-link-single-events-T0.3` [err](out/s3/dead-link-single-events-T0.3.err) | `ein solve $DL --single --events out/s3/dead-link-single-events.jsonl -T 0.3` | 2 | 0.31 |
| 81 | `s3/einaf-dead-link-describe` [out](out/s3/einaf-dead-link-describe.out) | `einaf outcome $DL -T 0.2` | 0 | 0.25 |
| 82 | `s3/einaf-dead-link-single` [out](out/s3/einaf-dead-link-single.out) | `einaf outcome $DL --single -T 0.2` | 0 | 0.08 |
| 83 | `s3/einaf-dead-link-count` [out](out/s3/einaf-dead-link-count.out) | `einaf outcome $DL --count -T 0.2` | 0 | 0.08 |
| 84 | `s3/einaf-dead-link-no-search` [out](out/s3/einaf-dead-link-no-search.out) | `einaf outcome $DL --no-search -T 0.2` | 0 | 0.25 |
| 85 | `s3/einaf-dead-link-feedback-single` [out](out/s3/einaf-dead-link-feedback-single.out) | `einaf feedback $DL --single -T 0.2` | 0 | 0.08 |
| 86 | `s3/einaf-dead-link-feedback-describe` [out](out/s3/einaf-dead-link-feedback-describe.out) | `einaf feedback $DL -T 0.2` | 0 | 0.26 |
| 87 | `s3/einaf-model-zebra-single` [out](out/s3/einaf-model-zebra-single.out) | `einaf outcome $Z --single` | 0 | 0.11 |
| 88 | `s3/einaf-model-zebra-describe-T5` [out](out/s3/einaf-model-zebra-describe-T5.out) | `einaf outcome $Z -T 5` | 0 | 5.09 |
| 89 | `s3/einaf-model-zebra-no-search-T5` [out](out/s3/einaf-model-zebra-no-search-T5.out) | `einaf outcome $Z --no-search -T 5` | 0 | 5.09 |
| 90 | `s4/order-run` [out](out/s4/order-run.out) | `env EIN_BIN=ein.rs/target/release/ein sh $P/probes/order-of-forms/run.sh` | 0 | 0.33 |
| 91 | `s4/gap-reader-first` [out](out/s4/gap-reader-first.out) | `ein solve $P/probes/order-of-forms/gap-reader-first.ein` | 0 | 0.01 |
| 92 | `s4/gap-producer-first` [out](out/s4/gap-producer-first.out) | `ein solve $P/probes/order-of-forms/gap-producer-first.ein` | 0 | 0.01 |
| 93 | `s4/gap-diff` [out](out/s4/gap-diff.out) | `diff $P/probes/order-of-forms/gap-reader-first.ein $P/probes/order-of-forms/gap-producer-first.ein` | 1 | 0.00 |
| 94 | `s4/one-food-ann-first-n1` [out](out/s4/one-food-ann-first-n1.out) | `ein solve $P/probes/order-of-forms/one-food-each-ann-first.ein -n 1` | 0 | 0.01 |
| 95 | `s4/one-food-carl-first-n1` [out](out/s4/one-food-carl-first-n1.out) | `ein solve $P/probes/order-of-forms/one-food-each-carl-first.ein -n 1` | 0 | 0.02 |
| 96 | `s4/one-food-ann-first` [out](out/s4/one-food-ann-first.out) | `ein solve $P/probes/order-of-forms/one-food-each-ann-first.ein` | 0 | 0.02 |
| 97 | `s4/one-food-carl-first` [out](out/s4/one-food-carl-first.out) | `ein solve $P/probes/order-of-forms/one-food-each-carl-first.ein` | 0 | 0.02 |
| 98 | `s4/decision-held-by` [out](out/s4/decision-held-by.out) | `grep -A1 '^\*\*Held by' docs/decisions/language/Q-M2.24-an-answer-does-not-depend-on-the-order.md` | 0 | 0.00 |
| 99 | `s4/table-rows` [out](out/s4/table-rows.out) | `sed -n '/^## 352\./,$p' ein.rs/crates/ein-ir/tests/golden/grammar_table.txt` | 0 | 0.00 |
| 100 | `s4/row-352-solve` [err](out/s4/row-352-solve.err) | `ein solve out/s4/row-352.ein` | 1 | 0.01 |
| 101 | `s4/row-353-solve` [err](out/s4/row-353-solve.err) | `ein solve out/s4/row-353.ein` | 1 | 0.01 |
| 102 | `s4/rows-einaf-accept` [out](out/s4/rows-einaf-accept.out) | `einaf grammar accept out/s4/row-352.ein out/s4/row-353.ein` | 1 | 0.06 |
| 103 | `s5/catalogue-line3` [out](out/s5/catalogue-line3.out) | `bash -c 'einaf catalogue \| sed -n 3p'` | 0 | 0.10 |
| 104 | `s5/catalogue-line3-recorded` [out](out/s5/catalogue-line3-recorded.out) | `bash -c "git show 969ae30d^:einaf/tests/fixtures/steps/bob1/calls.jsonl \| grep -o 'The library is sha256:[0-9a-f]*\.' \| sort \| uniq -c"` | 0 | 0.01 |
| 105 | `s5/catalogue-generated-head` [out](out/s5/catalogue-generated-head.out) | `bash -c 'einaf catalogue --generated \| sed -n 1,3p'` | 0 | 0.11 |
| 106 | `s5/zc-replay` [out](out/s5/zc-replay.out) · [err](out/s5/zc-replay.err) | `einaf from-text einaf/tests/fixtures/from_text/zebra-contradiction/text.txt --id zebra-contradiction --answer-line zebra --replay einaf/tests/fixtures/from_text/zebra-contradiction/generations.jsonl --gold-answer @contradiction --record out/s5/zc` | 0 | 4.44 |
| 107 | `s5/zc-same-program`  | `cmp out/s5/zc/program.ein einaf/tests/fixtures/from_text/zebra-contradiction/program.ein` | 0 | 0.00 |
| 108 | `s5/zc-run-stdlib` [out](out/s5/zc-run-stdlib.out) | `python3 -c 'import json, sys; r = json.load(open(sys.argv[1]))["run"]; print(r["stdlib"], sorted({s["stdlib"] for s in r["steps"]}))' out/s5/zc/run.json` | 0 | 0.03 |
| 109 | `s5/zc-core` [out](out/s5/zc-core.out) | `ein solve einaf/tests/fixtures/from_text/zebra-contradiction/program.ein --no-search` | 0 | 1.59 |
| 110 | `s5/bob1-steps` [out](out/s5/bob1-steps.out) · [err](out/s5/bob1-steps.err) | `einaf steps run einaf/tests/fixtures/steps/bob1/text.txt --id bob1 --answer-line word --out out/s5/bob1 --replay einaf/tests/fixtures/steps/bob1/calls.jsonl` | 0 | 2.08 |
| 111 | `s5/bob1-from-text` [out](out/s5/bob1-from-text.out) · [err](out/s5/bob1-from-text.err) | `einaf from-text einaf/tests/fixtures/steps/bob1/text.txt --answer-line word --replay einaf/tests/fixtures/steps/bob1/calls.jsonl --gold-answer 3` | 0 | 2.08 |
| 112 | `s5/bob1-program-diff` [out](out/s5/bob1-program-diff.out) | `diff out/s5/bob1-before.ein out/s5/bob1-after.ein` | 1 | 0.00 |
| 113 | `s5/bob1-before-single` [out](out/s5/bob1-before-single.out) | `ein solve out/s5/bob1-before.ein --single` | 0 | 0.07 |
| 114 | `s5/bob1-after-single` [out](out/s5/bob1-after-single.out) | `ein solve out/s5/bob1-after.ein --single` | 0 | 0.08 |
| 115 | `s5/zc-program-diff` [out](out/s5/zc-program-diff.out) | `git diff --stat '969ae30d^' 969ae30d -- einaf/tests/fixtures/from_text/zebra-contradiction einaf/tests/fixtures/steps/bob1` | 0 | 0.01 |
| 116 | `s5/zc-program-diff-text` [out](out/s5/zc-program-diff-text.out) | `git diff '969ae30d^' 969ae30d -- einaf/tests/fixtures/from_text/zebra-contradiction/program.ein` | 0 | 0.01 |
| 117 | `s5/zc-before-core` [out](out/s5/zc-before-core.out) | `ein solve out/s5/zc-before.ein --no-search` | 0 | 0.41 |
| 118 | `s5/probe-replay` [err](out/s5/probe-replay.err) | `einaf probe run --plan out/s5/probe-plan.jsonl --out out/s5/probe-again.jsonl --replay einaf/tests/fixtures/model/probe_lines.jsonl` | 0 | 0.08 |
| 119 | `s5/moved-version` [out](out/s5/moved-version.out) | `env EIN_STDLIB=out/s5/moved/stdlib ein --version` | 0 | 0.01 |
| 120 | `s5/moved-catalogue-same` [out](out/s5/moved-catalogue-same.out) | `bash -c "cmp <(EIN_STDLIB=out/s5/moved/stdlib einaf catalogue) <(einaf catalogue) && echo 'einaf catalogue: the same text under the moved library as under the checkout'"` | 0 | 0.11 |
| 121 | `s5/moved-zc-replay` [out](out/s5/moved-zc-replay.out) · [err](out/s5/moved-zc-replay.err) | `env EIN_STDLIB=out/s5/moved/stdlib einaf from-text einaf/tests/fixtures/from_text/zebra-contradiction/text.txt --id zebra-contradiction --answer-line zebra --replay einaf/tests/fixtures/from_text/zebra-contradiction/generations.jsonl --gold-answer @contradiction --record out/s5/zc-moved` | 0 | 4.48 |
| 122 | `s5/moved-zc-same-program`  | `cmp out/s5/zc-moved/program.ein einaf/tests/fixtures/from_text/zebra-contradiction/program.ein` | 0 | 0.00 |
| 123 | `s5/moved-zc-run-stdlib` [out](out/s5/moved-zc-run-stdlib.out) | `python3 -c 'import json, sys; r = json.load(open(sys.argv[1]))["run"]; print(r["stdlib"], sorted({s["stdlib"] for s in r["steps"]}))' out/s5/zc-moved/run.json` | 0 | 0.03 |
| 124 | `s7/q/hole-maximal` [out](out/s7/q/hole-maximal.out) · [err](out/s7/q/hole-maximal.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein --maximal` | 0 | 0.19 |
| 125 | `s7/q/hole-maximal-T5` [out](out/s7/q/hole-maximal-T5.out) · [err](out/s7/q/hole-maximal-T5.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein --maximal -T 5` | 0 | 0.20 |
| 126 | `s7/q/hole-minimal` [out](out/s7/q/hole-minimal.out) · [err](out/s7/q/hole-minimal.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein --minimal` | 0 | 0.17 |
| 127 | `s7/q/hole-minimal-T5` [out](out/s7/q/hole-minimal-T5.out) · [err](out/s7/q/hole-minimal-T5.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein --minimal -T 5` | 0 | 0.15 |
| 128 | `s7/q/hole-maximal-single-T5` [out](out/s7/q/hole-maximal-single-T5.out) · [err](out/s7/q/hole-maximal-single-T5.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein --maximal --single -T 5` | 0 | 0.19 |
| 129 | `s7/q/hole-count-T5` [err](out/s7/q/hole-count-T5.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein --count -T 5` | 2 | 5.10 |
| 130 | `s7/q/hole-describe` [out](out/s7/q/hole-describe.out) · [err](out/s7/q/hole-describe.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_hole.ein` | 0 | 17.49 |
| 131 | `s7/q/whole-single` [out](out/s7/q/whole-single.out) · [err](out/s7/q/whole-single.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_whole.ein --single` | 0 | 0.55 |
| 132 | `s7/q/whole-single-v-T5` [out](out/s7/q/whole-single-v-T5.out) · [err](out/s7/q/whole-single-v-T5.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_whole.ein --single -v -T 5` | 0 | 0.61 |
| 133 | `s7/q/whole-single-layer-progress-T5` [out](out/s7/q/whole-single-layer-progress-T5.out) · [err](out/s7/q/whole-single-layer-progress-T5.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_whole.ein --single --layer-progress -T 5` | 0 | 0.59 |
| 134 | `s7/q/whole-n2-v-T5` [out](out/s7/q/whole-n2-v-T5.out) · [err](out/s7/q/whole-n2-v-T5.err) | `utils/bench_env.sh --cores PT:1 ein solve ein.rs/crates/ein-cli/tests/bounded/dead_link_whole.ein -n 2 -v -T 5` | 0 | 0.59 |
| 135 | `s7/q/wclock-events-T5` [err](out/s7/q/wclock-events-T5.err) | `utils/bench_env.sh --cores PT:1 ein solve $P/probes/two-more-cores-only-where-printed/whole-node-clock.ein --single --events out/s7/wclock-events.jsonl -T 5` | 2 | 17.61 |
| 136 | `render/feature-25/render-program` [out](out/render/feature-25/render-program.out) | `ein render program examples/features/25_declared_twice_with_an_import.ein` | 0 | 0.01 |
| 137 | `render/feature-25-kb` [out](out/render/feature-25-kb.out) | `ein render kb --fold examples/features/25_declared_twice_with_an_import.ein` | 0 | 0.01 |
| 138 | `render/slot-prune-fwd-after` [out](out/render/slot-prune-fwd-after.out) | `ein render rule --name slot-prune-fwd stdlib/slots.ein` | 0 | 0.02 |
| 139 | `render/slot-prune-fwd-before` [out](out/render/slot-prune-fwd-before.out) | `env EIN_STDLIB=out/s2/old/stdlib ein render rule --name slot-prune-fwd out/s2/old/stdlib/slots.ein` | 0 | 0.02 |
| 140 | `render/slots-14-excluded-after` [out](out/render/slots-14-excluded-after.out) | `ein render kb --fold --query 1 tests/stdlib/slots/14_prune_placed_position.ein` | 0 | 0.02 |
| 141 | `render/slots-14-excluded-before` [out](out/render/slots-14-excluded-before.out) | `env EIN_STDLIB=out/s2/old/stdlib ein render kb --fold --query 1 tests/stdlib/slots/14_prune_placed_position.ein` | 0 | 0.02 |
| 142 | `render/slots-14-kb` [out](out/render/slots-14-kb.out) | `ein render kb --fold tests/stdlib/slots/14_prune_placed_position.ein` | 0 | 0.02 |
| 143 | `render/dead-link/render-program` [out](out/render/dead-link/render-program.out) | `ein render program $DL` | 0 | 0.01 |
| 144 | `render/dead-link-3-towns-solve` [out](out/render/dead-link-3-towns-solve.out) | `ein solve out/render/dead-link-3-towns.ein --no-search` | 0 | 0.01 |
| 145 | `render/dead-link-3-towns-links` [out](out/render/dead-link-3-towns-links.out) | `ein render kb --fold --colour-by origin --goal '(link ?a ?b)' out/render/dead-link-3-towns.ein` | 0 | 0.01 |
| 146 | `render/gap-reader-first/render-program` [out](out/render/gap-reader-first/render-program.out) | `ein render program $P/probes/order-of-forms/gap-reader-first.ein` | 0 | 0.01 |
| 147 | `render/gap-reader-first-kb` [out](out/render/gap-reader-first-kb.out) | `ein render kb --fold $P/probes/order-of-forms/gap-reader-first.ein` | 0 | 0.01 |
| 148 | `render/gap-producer-first-kb` [out](out/render/gap-producer-first-kb.out) | `ein render kb --fold $P/probes/order-of-forms/gap-producer-first.ein` | 0 | 0.01 |
| 149 | `render/bob1-before/render-program` [out](out/render/bob1-before/render-program.out) | `ein render program out/s5/bob1-before.ein` | 0 | 0.07 |
| 150 | `render/bob1-after/render-program` [out](out/render/bob1-after/render-program.out) | `ein render program out/s5/bob1-after.ein` | 0 | 0.07 |
| 151 | `render/bob1-before-rows` [out](out/render/bob1-before-rows.out) | `ein render kb --fold --query 1 out/s5/bob1-before.ein` | 0 | 0.07 |
| 152 | `render/bob1-after-rows` [out](out/render/bob1-after-rows.out) | `ein render kb --fold --query 1 out/s5/bob1-after.ein` | 0 | 0.07 |
| 153 | `render/dead-link-hole-oracle` [out](out/render/dead-link-hole-oracle.out) · [err](out/render/dead-link-hole-oracle.err) | `ein oracle out/render/dead-link-hole-lever.ein --dump out/render/dead-link-hole-cube` | 0 | 0.08 |
| 154 | `render/dead-link-hole-region` [out](out/render/dead-link-hole-region.out) | `ein render region out/render/dead-link-hole-cube` | 0 | 0.01 |
