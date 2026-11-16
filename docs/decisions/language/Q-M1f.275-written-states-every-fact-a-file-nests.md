# Q-M1f.275 — `written` states every fact a file nests, and a conclusion's ground ones

For each fact stored from a file, the program's, a local module's or a stdlib module's, the loader states every
fact nested among its arguments, at any depth, once: `(add (n (n z 1) 2) (n z 5))` gives `(written (n (n z 1) 2))`,
`(written (n z 1))` and `(written (n z 5))`. For a stored negation `(not P)` it states the facts nested in P's
arguments, and nothing of P, the fact denied. And for every rule's and hrule's conclusion, the nested facts that
hold no variable, whether the rule fires or not. A nested fact a rule assembles from a variable is stated by
nothing, and a fact of `eq`, checked and never stored, states nothing.

**Why.** A term a rule's text writes whole is written as surely as one in a fact, and its check needs no firing.
One assembled from a variable has no text to check until a firing builds it, and is no term a file writes.

**Held by.** `ein-cli/tests/written_facts.rs`: `what_the_loader_states`, from a fact, a negation, a module's fact
and a rule's conclusion, and from nothing else; `tests/stdlib/math/decimal/04_in_a_rules_conclusion.ein`, a rule
that never fires and whose numeral is flagged.
