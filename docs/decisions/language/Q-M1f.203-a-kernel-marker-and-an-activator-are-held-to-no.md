# Q-M1f.203 — A kernel marker and an activator are held to no declared count

The declared count holds a declared relation's forms
([Q-M1f.315](Q-M1f.315-a-form-of-a-declared-relation-has-as-many.md)) and nothing else. The kernel's own relations,
which no program declares, have no count: `(__closed__ likes extra)` loads and closes nothing, and the stored
declaration `(relation R ?A ?B)` is a pattern at any count, which is how a rule reads a signature. A fact headed by
a rule's name, its activator, is no declared relation's form: `(sym likes extra)` against a rule `sym (?R)` loads
and activates nothing. An activator in a `(solve :hrules …)` list is held to its hrule's parameters
([Q-M1f.115](Q-M1f.115-an-hrules-list-names-declared-hrules.md)); `eq` is held to two arguments by its own refusal.

**Why.** The check reads declarations, and these relations have none a program writes: a marker's count and a
rule's parameters are the kernel's and the rule's, not a `(relation …)` form's.

**Held by.** `ein-cli/tests/declared_counts.rs`: `what_is_not_a_declared_relations_form_loads`, the cases
`a-kernel-marker` and `the-declaration-as-a-pattern`.
