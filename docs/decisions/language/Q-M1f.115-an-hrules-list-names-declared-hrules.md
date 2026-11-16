# Q-M1f.115 — An `:hrules` list names declared hrules, and an activator has its hrule's count of arguments

Every bare name an `:hrules` list holds, first or not, is an hrule the program declares, and every activator gives
its hrule as many arguments as the hrule has parameters, an hrule without parameters none. Anything else is refused
at load, one line, placed at the name: `:hrules names gues, and the program declares no hrule gues — it declares
guess`, or `:hrules gives hrule 'guess' (at Color), 2 arguments, and guess takes 3, (?R ?T1 ?T2) — an activator
binds each parameter once`.

**Why.** Every name a `(solve …)` list holds names something the program has, and an activator that binds its
hrule's parameters short or long binds nothing: read as nothing, either would leave the hypotheses empty in
silence.

**Held by.** `ein_ir::from_ir`'s `refuse_solve_names`; `ein-cli/tests/solve_lists.rs`:
`a_list_naming_nothing_is_refused_on_every_command` and `a_list_naming_what_the_program_has_loads`.
