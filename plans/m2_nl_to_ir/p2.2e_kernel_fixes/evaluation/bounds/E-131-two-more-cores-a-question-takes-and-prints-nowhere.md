# E-131 — Two more cores a question takes and prints nowhere: the cube's minimal holes' under a line mode, and the whole node's under a progress watch

**Kind:** bug
**Status:** open
**Filed:** by [S2.2e.6](../../s2.2e.6_a_core_only_where_printed.md), 2026-11-27, on `6b359cce`, read off the code and
not measured
**Owner:** none; a fix leaves Ein's syntax and semantics as they are and moves no answer and no printed byte
([Q-M2.16](../../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md)), so a stage of M2 that
needs it may make it

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
