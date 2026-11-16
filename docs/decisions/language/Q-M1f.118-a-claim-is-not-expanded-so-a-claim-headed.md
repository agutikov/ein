# Q-M1f.118 — A claim is not expanded, so a claim headed by a macro's call is refused

A claim, a query's `:expect` or a `(solve … :expect (region …))`, lists facts, and no macro call in it is expanded.
A fact a claim lists headed by a macro's name, `(model (settled Ann))` beside `(macro settled (?g) (paid ?g))`,
names no relation of the program and is refused at load as any such claim is: `:expect names settled, and the
program names no relation settled`. A call inside a listed fact is refused as a written fact's is.

**Why.** A macro is a premise's shorthand, and its expansion is a premise, `(and (absent …) …)` for `unknown`,
which is no fact a claim could list. A claim says what states hold, written in facts.

**Held by.** `ein-cli/tests/goal_premises.rs`: `a_goal_a_rule_would_be_refused_for_is_refused_on_every_command`,
whose `claim-lists-a-macro-call` shape is refused on every command; `ein-cli/tests/macro_facts.rs`:
`a_macros_call_written_as_a_fact_is_refused_on_every_command`, a call inside a fact a claim lists among its shapes.
