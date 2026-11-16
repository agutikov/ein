# Q-M1f.74 — A nested `(or …)` a macro's expansion writes is refused as a written one is

An `(or …)` is a whole `:match`, each of its premises one way it holds. One that stands as a premise, inside an
`(and …)`, an `(absent …)` or another `(or …)`, is refused at load, one line, whether the rule writes it or a
macro's expansion does: `(macro either (?x) (or (p ?x) (q ?x)))` called inside an `(and …)`, or `std.macro`'s
`forall` given an `(or …)`. The loader reads the rule after expansion, so one walk refuses both. A macro whose body
is an `(or …)` and that is called as a whole `:match` loads, and so does a macro no rule calls.

**Why.** A call is replaced by its body, so the rule that runs is the expanded one, and it is refused for what it
is, in the same words and at the same step as the rule written out. The refusal comes at the call, which is where
[Q-M1f.67](Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md) places a macro's checks.

**Held by.** `ein-ir`'s `from_ir::tests::the_compilers_refusals_a_program_reached_are_refused_at_load`, with the
`either` and `forall` cases beside the written ones; `ein-cli/tests/cli_semantics.rs`:
`a_compile_refusal_reads_the_same_on_every_command`.
