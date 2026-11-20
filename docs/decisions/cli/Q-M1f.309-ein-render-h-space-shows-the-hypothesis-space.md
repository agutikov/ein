# Q-M1f.309 — `ein render h-space` shows the hypothesis space, and `ein show hyps` lists it

The structure of a program's hypothesis set H, its candidates and which of them exclude each other, the groups, the
owed groups and grids, and what couples the questions, is shown by a view of `ein render`: `ein render h-space FILE`
prints a read-out, or with `--view` a graph as DOT or the document `h-space.json`, and `--dump DIR` writes them all.
`ein show hyps FILE` lists H itself as Ein facts, one candidate a line, uncut, and nothing else on stdout, with a
`note:` on stderr that names the writer that built H and counts it, at the cost of root's saturation and the pass
that writes root's `open` facts. `ein solve` computes none of it and prints no row of it.

**Why.** The structure costs more than most answers: reading it saturates the node of every candidate and of every
pair of them, thousands of saturations on a program whose search is tens. As a view of its own it is paid for only by
the reader who asks for it, and every question of `ein solve` keeps the least work it needs.

**Held by.** `ein-cli/tests/h_space_view.rs`: `the_document_is_the_owners_reading_on_every_instance`,
`the_dump_is_every_view_and_moves_no_stdout`, `jobs_and_the_container_move_no_byte`;
`ein-cli/tests/show_hyps.rs`: `stdout_is_the_facts_and_stderr_one_note`, `the_door`. The pages:
[The hypothesis space and its structure](../../ein/reasoning/02-kb-evolution/05-hypothesis-space.md), what the
structure is, and [hypotheses and the answer, in code](../../ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md),
§ 1.3 for `ein show hyps` and § 2.3 and § 2.4 for `ein render h-space` and `h-space.json`.
