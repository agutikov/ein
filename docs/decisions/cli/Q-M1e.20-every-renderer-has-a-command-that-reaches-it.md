# Q-M1e.20 — Every renderer has a command that reaches it

No renderer the engine builds is reachable from Rust alone. The three that once were each have a door:

- `ein render program FILE` draws the program as written, one DOT digraph per group;
- `ein render kb FILE [--since FILE]` draws a KB, FILE loaded and saturated, and a `--dump-answer` state file is a
  FILE, so a model draws as root does;
- `ein solve FILE --dump-lattice DIR` writes every node the run visited, and `ein render lattice` draws them.

The dump is a reader of the run: it combines with every other flag and moves no byte of stdout, stderr or the exit
code.

**Why.** A renderer with no door is read by nobody but its own test, and the one who needs these is the reader of a
wrong answer, who reaches the engine through the binary.

**Held by.** `ein-cli/tests/render_doors.rs`: `every_door_prints_dot_that_dot_draws`,
`dump_lattice_lists_the_nodes_the_run_entered`, `the_dump_combines_with_the_other_flags`. The pages:
[lattice-dump.md](../../ein/ein.rs/lattice-dump.md),
[04_dot_rendering.md](../../kernel/ir/03-ein-lang/04_dot_rendering.md).
