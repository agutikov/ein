# Q-M1f.38 — A refusal's line names the fault, then what to write

A line the loader refuses a program with is one finding, then ` — `, then what to write instead. A conclusion's
variable that no premise or parameter binds reads `rule 'r': its :assert writes ?y, which no premise or parameter
binds — to conclude it of the objects a relation holds of, bind it with a premise such as (R ?y), or of every name
of the program with (eq ?y ?y)`; *the objects a relation holds of* are a relation premise's bindings, and the line
names no relation of a theory's ([Q-M1f.436](../engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)). Where the
`:match` is a top-level `(or …)` and only some of its disjuncts bind the variable, the line names those that do not:
`… which no premise or parameter binds where its :match holds through (r1 ?v1 ?v3) — …`. Several lines are joined
by `; ` behind `kb load error: `.

**Why.** `; ` separates findings, so it cannot also set a finding's advice apart; the dash keeps one finding one
segment for anyone splitting the report. Naming the disjuncts at fault tells the reader which arm to fix.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_conclusion_variable_nothing_binds_is_refused_at_load`, whose
witnesses include `examples/ein-bugs/unbound-assert-var.ein` and its two disjunct lines. The line is
`ein_ir::from_ir`'s.
