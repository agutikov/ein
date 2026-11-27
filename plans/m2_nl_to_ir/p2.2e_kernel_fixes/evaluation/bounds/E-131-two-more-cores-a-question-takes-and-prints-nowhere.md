# E-131 — Two more cores a question takes and prints nowhere: the cube's minimal holes' under a line mode, and the whole node's under a progress watch

**Kind:** bug
**Status:** closed by [S2.2e.7](../../s2.2e.7_two_more_cores_only_where_printed.md) (`87d5db47`)
**Filed:** by [S2.2e.6](../../s2.2e.6_a_core_only_where_printed.md), 2026-11-27, on `6b359cce`, read off the code and
not measured
**Owner:** [S2.2e.7](../../s2.2e.7_two_more_cores_only_where_printed.md), on the user's word, 2026-11-27: *"E-131 - yes,
add S2.2e.7"*; a fix leaves Ein's syntax and semantics as they are and moves no answer and no printed byte
([Q-M2.16](../../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md))

## What

**Two more dead nodes' cores are searched and thrown away, against
[Q-M1e2.31](../../../../../docs/decisions/answer/Q-M1e2.31-each-question-about-r-does-the-least-work-it.md)'s *a core is
taken only where a surface prints one*.** S2.2e.6 brought a dead root's core into line with it
([E-130](E-130-a-question-that-prints-no-core-takes-a-dead-roots-core.md)); its list of every surface that prints a
core and every question's route to the node function found two other cores taken where nothing prints them:

- **The cube's minimal holes', under a line mode.** On a program the static check flags, within `--region-limit`, a
  line mode the cube does not decide early (`--maximal` under any ask, `--minimal` where root owes) computes the whole
  cube (`explore::flagged` → `region::compute_prepared`), and `region::compute_prepared` takes each dead minimal
  hole's core after the walk, whenever `Cores::visited` is off, whatever the question. `explore::of_cube` builds a
  description for describe alone, so a mode's read-out and summary print no hole, and the cores are dropped. A
  description prints them (the read-out's `minimal hole` rows, the summary's `region.holes.minimal[].core`), and
  `--count` writes them in its summary.
- **The whole node's, under a progress watch.** The modes' obligation tree reads the whole node, every candidate
  together, for the coupling of its choices (`tree.rs`, `read_structure`, under the order `Order::of` gives `--single`,
  `-n` and the maximal line), and where the node is dead `structure::coupling::node` takes its core where the run is
  narrated, `Space::narrated()`: the event stream or a watch. `--events` and `--dump-lattice` print it (the node's
  `enter` line, `nodes.json`), and they ask `Cores::visited`; `-v` and `--layer-progress` are a watch too, which print
  a node's commitment and kind and no core, so under them alone the core is searched and dropped.

## What it costs

Nothing measured. The first is on flagged programs, of which nothing is claimed, within the limit, where the cube's
`2ⁿ` saturations are most of a run; the second on a monotone program asked with `-v` or `--layer-progress`, whose
whole node dies. Either costs what the core's search costs, which on a link several rules derive is the whole run
(E-113, E-126).

## A fix

Not made: neither is a dead root's core, which S2.2e.6 was planned for. The first: the minimal holes' cores taken
where the question prints them, a description and `--count`, as `N`'s are under `explore::Options::cores_taken`. The
second: the whole node's core taken where `Cores::visited` asks it, rather than wherever the run is narrated. Neither
moves a printed byte; the explore test that a run printing no core takes none would read both.

## Closed

By S2.2e.7, `87d5db47`, 2026-11-27. **The minimal holes' cores are read off the question**: `region::Cores` gains
`holes`, each dead minimal hole's on the cube, beside `root`, `visited` and `nogoods`; `region::compute_prepared` takes
them after the walk where `holes` asks, no longer whenever `visited` is off; `explore::Options::cores_taken` keeps
`holes` under a description and `--count`, whose summary writes them, and clears it under a mode, which prints no
hole. `ein solve` asks it always and lets the question decide; `ein test` asks it where its `--json-summary` writes
it, as it asks root's; `ein oracle`, a description, asks it on every run. **The whole node's core is taken where a
surface that prints every visited node's reads the run**: `explore::Space::takes_whole_core`, the run narrated and
`Cores::visited` asked, which `structure::coupling::node` reads where it read `Space::narrated`; `--events` and
`--dump-lattice` print it as before, and `-v` and `--layer-progress`, which print a node's commitment and kind, are
handed the node with no core, as every other node. **No answer and no printed byte moved** (the stage's record).
Two fixtures beside `dead_link.ein` show the cost, each on `dead_link.ein`'s link: `dead_link_hole.ein`, a flagged
program within the limit whose one minimal hole dies on it, and `dead_link_whole.ein`, a monotone program whose whole
node, which the tree reads under `--single` and `-n 2`, dies on it. On the hole's fixture `--maximal` took 17.84 s and takes 0.19 s, `--minimal`
17.90 s and 0.17 s, and under `-T 5` each was the clock's abort and answers; a description and `--count` take the
core's 18 s as before. On the whole node's, `--single -v` took 18.32 s and takes 0.60 s, `--single --layer-progress`
18.69 s and 0.61 s, `-n 2 -v` 18.37 s and 0.60 s, as with no watch, 0.56 s; under `-T 5` each ran past the clock to
the abort and answers (release binaries, pinned to a P-core, on AC; the stage's record). Held by
`ein-infer/tests/explore.rs`' `a_run_that_prints_no_core_takes_none_and_answers_alike`, whose lazy run is now read by
a watch and which reads both fixtures, and by `ein-cli/tests/bounded_saturation.rs`'
`a_line_mode_takes_no_holes_core_and_a_watch_no_whole_nodes`; each fails on the code before the fix. Found on the
way: the whole node's core search reads no clock, [E-132](E-132-the-whole-nodes-core-search-reads-no-clock.md).
