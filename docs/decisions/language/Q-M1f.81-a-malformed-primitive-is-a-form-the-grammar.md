# Q-M1f.81 — A malformed primitive is a form the grammar would refuse where it stands

A form a macro's expansion puts in a rule is malformed where the grammar would refuse it at that position if the
expanded rule were written out:

- `eq` or `neq` with any number of arguments but two;
- `absent` with any number but one;
- `not` of what is no pattern, a conjunction, a disjunction, a `not`, a `neq` or a name, and in a `:match` of a
  variable (a conclusion may negate a variable);
- `false` with an argument;
- an `(or …)` as a conclusion;
- a connective, an `(and …)`, an `(or …)` or a `(neq …)`, where a value goes;
- a conclusion that is no form, a body that is its own parameter called with a variable or a name.

An `(and …)` a call leaves inside a conclusion's `(and …)` is no malformed primitive: a conclusion is read flat.

**Why.** One walk over the grammar's productions by position decides every shape at once, and a rule written out
never fails it, so the expanded rule and the written one are held to one standard and no shape is left to be
matched, concluded or proposed.

**Held by.** `ein_ir::from_ir`'s `refuse_malformed_primitives`; `ein-ir`'s
`from_ir::tests::a_macros_expansion_is_held_to_the_grammars_shapes`, each shape at each site and every shape the
grammar takes loading; `ein-cli/tests/macro_bodies.rs`:
`every_malformed_primitive_is_refused_before_any_path_runs_it`. The lines each shape is refused with:
`defined_behaviour.md` § 4.16.
