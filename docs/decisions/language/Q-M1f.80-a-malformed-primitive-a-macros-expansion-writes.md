# Q-M1f.80 — A malformed primitive a macro's expansion writes is refused at load

The loader holds a rule's, an obligation's and a goal's clauses, after macro expansion, to the shapes
the grammar gives a written rule, and refuses a form in another shape
([Q-M1f.81](Q-M1f.81-a-malformed-primitive-is-a-form-the-grammar.md)), one line each, `; `-joined with the loader's
other lines. So every command that reads the program refuses it, the views and `kb save` among them, before any
path compiles, fires, proposes or owes the rule: under a commitment, in a hypothesis, in an obligation, and in a rule
no activator reaches, as at root. A macro no rule calls is not read, and loads.

**Why.** No path may execute a malformed primitive, `(eq k)` among them
([Q-M1f.67](Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md)). A plan is compiled per activator and per
path, so a check made where a plan is compiled is met by some commands and not by others, and two commands would
answer one program differently. Load is the one step every path passes first.

**Held by.** `ein_ir::from_ir`'s `refuse_malformed_primitives`; `ein-ir`'s
`from_ir::tests::a_macros_expansion_is_held_to_the_grammars_shapes`; `ein-cli/tests/macro_bodies.rs`:
`every_malformed_primitive_is_refused_before_any_path_runs_it`, each shape at every site on every command, one
first line, no container written and no step of the rule in `--events`.
