# Q-M1f.82 — A malformed primitive a macro writes is refused at the position of the expanded form

A kernel primitive a macro's expansion writes in a shape the grammar refuses is refused at load with one line
behind `kb load error: `: `<file>:<line>:<col>: rule 'r': its :match writes (eq ?x), and eq takes exactly 2
arguments, not 1 — it compares two values, so write one guard per pair`. The position is the first one at or under
the expanded form: the macro body's for a form the body writes, and for a form the grammar builds for a keyword,
`(not …)`, `(neq …)` or `(or …)`, which keeps none, its first part's, the call's argument, so that a form a library
macro writes is placed in the program's file. In a `.einb` it is the stored program's. The rule is named as the
loader names it, `hrule 'h'` for an hrule.

**Why.** The form is the macro's, not the rule's, so the rule's name alone does not find it; its position does.

**Held by.** `ein-ir`'s `from_ir::tests::a_macros_expansion_is_held_to_the_grammars_shapes`;
`ein-cli/tests/macro_bodies.rs`: `every_malformed_primitive_is_refused_before_any_path_runs_it`;
`examples/broken/load/eq_in_a_macro_body.expected`; the stored program's position,
`ein-cli/tests/program_read.rs`: `a_containers_positions_name_its_stored_program`.
