# Q-M1f.114 — A relation a `(solve …)` list names is one the program writes elsewhere

A relation named in `:hypothesis-relations` or `:no-hypothesis`, or given to an `:hrules` activator at a parameter
that names a relation, is one the program writes as a relation outside the three lists: one it declares or states a
fact of; `is-a`, `eq` and `neq`; the relation of any form in a rule's, an hrule's or an obligation's `:match` or
`:assert`, in a query's `:goal` or in a fact's argument; or a name a fact or a conclusion passes where a relation's
name is held, as `(bijective color-loc)` passes `color-loc`. Any other name is refused at load, one line per name,
placed at the name, on every command: `:no-hypothesis names like, and the program names no relation like — …`.

**Why.** One misspelt character in a list moves the hypotheses and so the answer, in silence. What tells a misspelt
name is that the program writes it nowhere else; whether something makes the relation cannot, since an activator's
hrule makes whatever relation it is given, and a relation only a rule derives is a relation of the program.

**Held by.** `ein_ir::from_ir`'s `refuse_solve_names`, over `program_relations` and the goals' relations;
`ein-cli/tests/solve_lists.rs`: `a_list_naming_nothing_is_refused_on_every_command` and
`a_list_naming_what_the_program_has_loads`.
