# Q-M1f.202 — A relation written as a variable is held to no declared count

A pattern or a conclusion whose relation is a variable, `(?R ?a ?b)`, is not held to a declared count, whatever the
program shows its parameter holds. `(rule mirror (?R) :match (?R ?a) …)` activated by `(mirror likes)`, where
`likes` declares two arguments, loads, and its premise matches nothing, since matching is by count. A rule can so
derive, through a parameter, a fact of a declared relation at another count; no written form is one.

**Why.** A rule over a parameter is generic over counts: the stdlib's rules that fan out pass relations of several
counts to one rule, and for an activator whose relation has another count the rule matching nothing is what a
generic rule means. Held to the count, a generic rule would be refused for one activator of many.

**Held by.** `ein-cli/tests/declared_counts.rs`: `what_is_not_a_declared_relations_form_loads`, the case
`through-a-parameter`, which also concludes `(?R ?a ?a ?a)`.
