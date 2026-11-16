# Q-M1f.73 — A rule binds at most 256 variables, counted by the loader

The loader counts every distinct variable a rule's `:match` writes, at any depth, together with the rule's
parameters, and every variable a query's `:goal` writes, and refuses a clause whose count passes 256, on every
command, with a line that says what to do. The count bounds each register file the compiler builds for the clause,
one for the plan and one for each lifted `(absent …)`, so the compiler meets no clause past its file and its own
check is an invariant.

**Why.** One count a reader can make from the program's text, *a rule binds at most 256 variables*, said in the
program's words; refused at load, the bound reaches a rule no state activates as surely as one that fires. A clause
whose variables split between its premises and an `(absent …)` could fit the compiler's files and is still refused:
the bound is far past any rule written by hand.

**Held by.** `ein_ir::MAX_MATCH_VARIABLES`, which `ein_infer::MAX_REGS` is; `ein-infer/tests/compile_limits.rs`:
`a_rule_at_the_ceiling_compiles`, `one_variable_past_the_ceiling_is_refused_at_load`. How wide the corpus's rules
are against it: [Q-M1f.173](Q-M1f.173-the-register-file-stays-256-wide.md).
