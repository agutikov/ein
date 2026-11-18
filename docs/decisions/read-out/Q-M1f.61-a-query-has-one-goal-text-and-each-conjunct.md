# Q-M1f.61 — A query has one `:goal-text`, and each conjunct reads as written

A query's `:goal-text` is one sentence for the whole goal, filled with the goal's variables for each row of a
projection class. There is no template per conjunct: where a surface prints a conjunct's fact, it reads as every fact
does, by its reason or as written, since a fact takes no text from its relation's declaration
([Q-M1f.58](Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)).

**Why.** A template per conjunct would be a language change, a text written on no form of its own; the goal's one
sentence says what the query asks.

**Held by.** `ein-cli/tests/natural_language_text.rs`: `the_read_out_prints_the_goal_text_as_written`;
[14-natural-language.md](../../ein/user/03-syntax-and-semantics/14-natural-language.md#1-the-four-texts) § 1 and § 5.
