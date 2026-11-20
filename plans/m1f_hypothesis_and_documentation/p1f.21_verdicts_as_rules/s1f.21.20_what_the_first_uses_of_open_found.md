# S1f.21.20 — what the first uses of `open` found

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator, under the user's word to run the phase to its end on defaults
([the README](README.md#the-word-to-run-to-the-end)); **not started**.
**Waits on:** [S1f.21.18](s1f.21.18_the_hrule_form_removed.md), which changes the same files of the engine.
**Writes:** `ein-infer/src/` (`region.rs`, `open.rs`, `oblgen.rs`, `explain.rs`), `ein-cli` (the order of stderr's
lines, `ein saturate`'s counts), their tests; the pages and goldens that move with a report's words.

## What it is

[S1f.21.13](s1f.21.13_the_search_and_rules.md) was the first stage to write a program with `open` facts after
[S1f.21.12](s1f.21.12_hypotheses_as_facts.md) built them, and met three defects of that build, none of which moves
a corpus answer. This stage fixes them.

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

## What must hold

Every corpus answer as before; every mode against the oracle; `answer_round_trip.rs`, `non_monotone_warning.rs`;
`08-read-out.md` in the same commit as a row that moves; the gate on the last commit.

## Record

Nothing yet.
