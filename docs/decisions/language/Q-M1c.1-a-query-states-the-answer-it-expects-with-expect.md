# Q-M1c.1 — A query states the answer it expects with `:expect`

A program states what it expects of its own answer in the program, as one keyword on the question it asks: `(query
:goal … :expect <claim>)`. The claim is shaped like the answer, a `(model …)` of facts, an `(or …)` of them, or
`(false)` ([Q-M1c.2](Q-M1c.2-an-expectation-is-ground-and-naming-a-relation.md)), so it is written by running the
program and reading the answer. A file may ask several queries, each with its own `:expect`; every one is answered
off the one run of the file, every one is checked, and any claim that fails sets the exit code. A claim about the
region as a whole is the `(solve …)` form's `:expect`.

**Why.** An expectation that travels with the program needs no sidecar file and no new form, and one shaped like
the answer reads as the answer. A file whose second check were silently dropped would be worse than no check, so a
file's queries are all asked and all held.

**Held by.** The grammar, `ein-ir/src/ein.y` (`Query`, `Claim`). `ein-cli/tests/expect_cli.rs`:
`every_query_runs_and_any_failure_sets_the_code`, `an_artefact_flag_over_several_queries_writes_the_one_run`,
`a_false_expectation_exits_one_and_names_the_fact`; `examples/features/10_expect.ein`.
