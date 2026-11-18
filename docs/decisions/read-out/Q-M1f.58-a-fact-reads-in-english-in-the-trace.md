# Q-M1f.58 — A fact reads in English in the trace, from its first recorded justification

A fact has two texts: its **statement**, its relation's `:why` filled with its arguments, and its **reason**, read
off its first recorded justification: a given's `:source`, or the deriving rule's `:why` filled with that firing's
bindings. `--trace` writes a firing's reason in its step's quote, a given premise with its `:source`, and each fact
the step derives with its statement; `ein render kb` labels a fact's arrow with its source or its rule. Where a fact
has no text it reads as its s-expression, never as invented prose. The read-out's `(model …)` blocks, `ein solve`'s
stdout and `answer.txt` stay s-expressions, since a claim is pasted from them, and a saved state holds no fact of the
state to render ([Q-M1f.392](../answer/Q-M1f.392-a-saved-state-is-saved-and-loaded-as-itself.md)).

**Why.** A line of the trace is where a reader wants the program's own words, and it stops no claim form being Ein.
A fact's justifications arrive in the agenda's order, so its reason is an explanation, as a core is
([Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md)); within a run it is deterministic.

**Held by.** `ein_render::sentence`; `ein-cli/tests/natural_language_text.rs`:
`the_trace_prints_the_rules_why_and_the_premises_source`; `ein-cli/tests/natural_language_page.rs`:
`the_pages_trace_lines_are_a_runs`. The ladder:
[14-natural-language.md](../../ein/user/03-syntax-and-semantics/14-natural-language.md#3-a-fact-in-english-the-fallback-ladder) § 3.
