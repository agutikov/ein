# Q-M1f.61 — A query has one `:goal-text`, and each conjunct reads by its relation's `:why`

A query's `:goal-text` is one sentence for the whole goal, filled with the goal's variables for each row of a
projection class. There is no template per conjunct: where a surface prints a conjunct's fact, it reads by its
relation's `:why`, as every fact does.

**Why.** Each conjunct's fact already has a statement, its relation's; a template per conjunct would be a language
change that says again what the declarations say.

**Held by.** `ein-cli/tests/natural_language_text.rs`: `the_read_out_prints_the_goal_text_as_written`;
[natural-language.md](../../ein/user/syntax-and-semantics/natural-language.md#1-the-four-texts) § 1 and § 5.
