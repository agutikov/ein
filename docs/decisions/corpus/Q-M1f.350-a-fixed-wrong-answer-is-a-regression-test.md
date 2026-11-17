# Q-M1f.350 — A fixed wrong answer is a regression test

A wrong answer the engine gave and no longer gives stays in the project as a regression test: its witness, a
program under `examples/ein-bugs/wrong-answers/`, is a corpus program, and the right answer is recorded beside the
command that asks for it, as test data a test of the engine runs and holds the engine to. The record carries the
witness, the question and the right answer, and nothing of when or under which work it was found or fixed. A wrong
answer the engine still gives is work, and is tracked with the work, not in the project
([Q-M1f.345](../docs/Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)); when it is fixed it joins the
tests, and any claim that stated the wrong answer states the right one.

**Why.** A wrong answer is temporary: while the engine gives it, it is something to do; once fixed, it is a fact
about the engine that must stay true, which is what a regression test is.

**Held by.** `corpus/regressions.toml`, its fixed rows; `ein-cli/tests/regressions.rs`:
`every_row_answers_what_the_register_records`, which runs each row's question and holds a fixed row to its right
answer; the witnesses' entries in `corpus/corpus.toml`.
