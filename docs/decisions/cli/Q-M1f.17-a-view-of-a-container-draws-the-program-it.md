# Q-M1f.17 — A view of a container draws the program it stores

`ein render program`, `rules`, `rule` and `constraints`, handed a `.einb`, draw the program the container stores:
the resolved, import-flattened form list `ein kb save` wrote, read under the container's name. On a program that
imports nothing, each view prints for the container what it prints for the `.ein`; on one that imports, the forms the
imports brought are drawn too, since the container holds them in place of each `(import …)`.

**Why.** A container holds no source text, and the program it stores is the program its answer was computed from.
Refusing a container would be one command refusing a file every other command answers; re-reading the source would
draw another file than the one the command was handed.

**Held by.** `ein-cli/tests/program_read.rs`:
`every_command_answers_a_program_its_container_and_a_file_relative_import`, each view's stdout on a container held to
its `.ein`'s; `a_containers_positions_name_its_stored_program`. Every command reads a program by one path, `ein-cli`'s
`common::read_program`. The answer directory: [Q-M1f.27](Q-M1f.27-a-containers-answer-directory-saves-the-program.md).
