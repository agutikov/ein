# Q-M1e2.31 — Each question about R does the least work it needs

A run does the work its question needs and no more. *Is there a solution* stops at the first state of R it finds;
*is it the only one*, `--single`, stops at the second, or where a sound rule closes the rest; `-n N` stops at N;
`--minimal`, `--maximal` and `--count` each do what their line or their count needs; a bare `ein solve` does what
the description of R needs. Each stop rests on a licence, a fact about the program's class the static check proves,
and a run without one goes on or reports a part. What a surface does not print is not computed: a core is taken only
where a surface prints one. The whole lattice, 2ⁿ saturations, is walked only when it is asked for by name, by
`ein oracle`. A budget only stops work, and the run says its answer is a part
([Q-M1e2.30](Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)). `ein test` asks a file the least
question that decides the claims it states ([Q-M1f.221](Q-M1f.221-ein-test-asks-a-file-the-least-question-that.md)).

**Why.** A question that pays for more than it asks costs its asker time and memory for an answer they did not
want, and on a lattice of 2ⁿ nodes the difference is the whole cost. No mode adds work of another kind as a
fallback: where its licence does not hold, it says the answer is partial.

**Held by.** `ein-infer/tests/explore.rs`: `a_square_cube_answers_its_first_questions_in_a_handful_of_saturations`,
`a_run_that_prints_no_core_takes_none_and_answers_alike`; `ein-cli/tests/read_out.rs`:
`count_prints_describes_rows_and_reads_no_state`; and, since less work must never mean another answer,
`every_mode_is_the_oracle` in `explore.rs`.
