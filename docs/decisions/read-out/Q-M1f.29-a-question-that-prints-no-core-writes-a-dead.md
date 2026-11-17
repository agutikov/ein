# Q-M1f.29 — A question that prints no core writes a dead root's core `null`

On a program whose root is dead, a bare `ein solve` prints root's core and the rules under it, and its summary
writes them as `region.core` and `region.refuted_by`. `--count` and a mode print no core, and their summaries write
both `null`, though root is dead and its core exists: the summary holds stdout's facts, and `null` is what the run
does not state ([Q-M1e2.27](Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-no-object.md)).

**Why.** A count's question is *how many*, and a mode's is a part of R; neither takes a core it would not print, and
a run does only the work its question needs.

**Held by.** [08-read-out.md](../../ein/user/04-reference/08-read-out.md#11---json-summary-the-answer-blocks) § 11, held to the printer by
`ein-cli/tests/read_out_reference.rs`: `every_answer_field_of_the_summary_has_a_row`.
`ein-infer/tests/explore.rs`: `a_run_that_prints_no_core_takes_none_and_answers_alike`.
