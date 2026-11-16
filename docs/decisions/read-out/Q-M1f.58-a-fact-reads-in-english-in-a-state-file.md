# Q-M1f.58 — A fact reads in English in a state file and the trace, from its first recorded justification

A fact has two texts: its **statement**, its relation's `:why` filled with its arguments, and its **reason**, read
off its first recorded justification: a given's `:source`, or the deriving rule's `:why` filled with that firing's
bindings. A `--dump-answer` state file writes both in the comment after each fact, and `--trace`'s *Derives* line
carries the statement. Where a fact has no text it reads as its s-expression, never as invented prose. The read-out's
`(model …)` blocks, `ein solve`'s stdout and `answer.txt` stay s-expressions, since a claim is pasted from them.

**Why.** A comment in a state file and a line of the trace are where a reader wants the program's own words, and
neither stops the file reloading or the claim form being Ein. A fact's justifications arrive in the agenda's order,
so its reason is an explanation, as a core is ([Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md)); within a
run it is deterministic.

**Held by.** `ein_render::sentence`; `ein-cli/tests/natural_language_text.rs`:
`a_state_file_writes_each_fact_in_english`; `ein-cli/tests/natural_language_page.rs`:
`the_pages_state_file_lines_are_a_runs`. The ladder:
[09_natural_language.md](../../kernel/ir/03-ein-lang/09_natural_language.md) § 3.
