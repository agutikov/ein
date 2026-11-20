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

Nothing yet.
