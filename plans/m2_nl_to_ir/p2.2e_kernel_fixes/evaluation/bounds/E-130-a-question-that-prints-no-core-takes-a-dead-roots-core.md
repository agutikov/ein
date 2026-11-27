# E-130 — A question that prints no core takes a dead root's core all the same

**Kind:** bug
**Status:** closed by [S2.2e.6](../../s2.2e.6_a_core_only_where_printed.md) (`6b359cce`)
**Filed:** by [S2.2e.3](../../s2.2e.3_clock_and_a_dead_root.md), 2026-11-27, on `44e3bef1`
**Owner:** [S2.2e.6](../../s2.2e.6_a_core_only_where_printed.md), on the user's word, 2026-11-27: *"E-130 - yes, add
S2.2e.6"*; the fix leaves Ein's syntax and semantics as they are and moves no answer and no printed byte
([Q-M2.16](../../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md))

## What

**A question that prints no core takes a dead root's core all the same.** The node function takes a dead node's core
where `axes.cores || facts.is_empty()` (`ein-infer/src/region.rs`, `node_in_region`): root's on every run, whatever
the question. Of the questions, a description (`ein solve`) and `--no-search` print root's core (the read-out's
`unsat core` rows, the summary's `region.core`); `--single`, `-n`, `--minimal`, `--maximal` and `--count` print none,
their summaries writing `region.core` `null` though root is dead. So the core is searched and thrown away, against two
decisions: [Q-M1f.29](../../../../../docs/decisions/read-out/Q-M1f.29-a-question-that-prints-no-core-writes-a-dead.md)
(*a count's question is how many, and a mode's is a part of R; neither takes a core it would not print*) and
[Q-M1e2.31](../../../../../docs/decisions/answer/Q-M1e2.31-each-question-about-r-does-the-least-work-it.md) (*a core is
taken only where a surface prints one*). The code says it means it: `region::Cores`' note, *root's is taken on every
run*, and [`03-design/07-search.md` § 6](../../../../../docs/ein/ein.rs/03-design/07-search.md#6-cores-and-explanations),
*root's and, on the cube, the minimal holes' always*. `explore.rs`' `a_run_that_prints_no_core_takes_none_and_answers_alike`
compares the cores of N's members and not root's.

Where root dies on a link several rules derive, every derivation recorded, the core is the whole cost of the run
(E-126): the model's Zebra program answers `--single` in 72 s, of which root's saturation and everything else are some
60 ms. Since S2.2e.3 the search reads the clock, so under a clock every such question ends at it with no answer, where
the answer costs what the lever's run costs.

## Evidence

The model's Zebra program, `p2.2_formalizer/from_text/runs/library/zebra/program.ein`, on `44e3bef1`'s binary
(release, pinned to a P-core, on AC):

| question | wall | what prints root's core |
|---|---|---|
| `ein solve --single` | 72.0 s, *Contradiction* | nothing: the read-out has no `unsat core` row, the summary's `region.core` is `null` |
| `ein solve` (describe) | 72.0 s | the read-out's two `unsat core` rows, `cut at its budget of 64 environments per fact` |
| the same with `(config :record-alternative-justifications false)`, `--single` | 0.05 s | nothing |

Where the 72 s go, sampled with `gdb` every four seconds of a `--single` run: every sample in
`explain::contradiction_frontier` called from `region::node_in_region` under `explore::Space::dead_root`; the
search's 15 rounds fold 19 000 justifications each, some 24 million environments a round at their peak, 10 to 12 s a
round (S2.2e.3's record).

## What it cost the application

Every question of EinAf's bounded call on a model's program whose root dies this way costs the core's search, though
only a description's object carries the core (`einaf::kernel::feedback`, under `Listing::False` with a description);
under the call's clock, since S2.2e.3, each such question is *not decided* at the clock where it could be answered in
milliseconds.

## A fix

Not made here (S2.2e.3 bounds the search by the clock, and leaves which cores are taken as they were). Root's core
taken only where the question prints it: a description and `--no-search`, and the surfaces that print a visited node's
core (`--events`, `--dump-lattice`); not under a mode and not under `--count`, which takes describe's route with
describe's mode (`explore::run_with`), so the count's question has to reach the node function's axes. No answer and no
printed byte moves: the summaries already write `null` there (Q-M1f.29). The design page's sentence and `region::Cores`'
note change with it, and the test that a run that prints no core takes none would compare root's core too.

## Closed

By S2.2e.6, `6b359cce`, 2026-11-27. `region::Cores` gains `root`, a dead root's core, which a description and
`--no-search` print, beside `visited` and `nogoods`; the node function takes root's where the axes say so
(`Axes::taking_cores` reads the three) and no longer on every run. **The question reaches the axes**:
`explore::Options::cores_taken` clears `root`, and `nogoods`, where the question prints neither, a mode and `--count`,
and `explore::run_with` reads it before `--count` takes describe's route with describe's mode, so the count's
question, which the route no longer sees, has already reached every route's axes. A surface that prints every visited
node's core (`--events`, `--dump-lattice`, `ein render lattice`) still takes root's under any question, as a visited
node's. `ein test` asks root's only where its `--json-summary` writes it, under a description or `--no-search`: its
stdout prints no core. **No answer and no printed byte moved**: the summaries already wrote `null` there (Q-M1f.29),
and the sweeps that read every answer and every byte are green (the stage's record). On the model's Zebra program,
`--single`, `-n 2`, `--minimal`, `--maximal` and `--count` answer in some 70 ms where they took 72 s, and under `-T 5`
answer in the same time where they were the clock's abort; a description and `--no-search` still print their two
`unsat core` rows after 72 s, and are the clock's abort under `-T 5`. Held by `ein-infer/tests/explore.rs`'
`a_run_that_prints_no_core_takes_none_and_answers_alike`, which now reads root's core under every question,
`--count` among them, and fails on the code before the fix, and by `ein-cli/tests/bounded_saturation.rs`'
`a_question_that_prints_no_core_answers_a_dead_root_within_the_clock`, on `dead_link.ein`, which fails before it too;
EinAf's `outcomes.rs`' `a_dead_roots_core_search_is_not_decided_at_the_kernels_clock` follows (`1a8608ff`).

Two more cores taken where nothing prints them, found on the way and not root's, are
[E-131](E-131-two-more-cores-a-question-takes-and-prints-nowhere.md).
