# Q-M1f.14 — A keyword pair is only ever a form's clause

The parser builds a keyword pair, `:key value`, only as a form's own clause, so no term and no nested fact holds
one, and nothing in the engine drops, skips or labels a pair inside a term: the compiler, the macro expander, the
loader's validator and the renderers have no such arm, and where a match needs one it is an `unreachable!` naming
`ein.y`'s production. The walks that descend into a pair's value as into a form's arguments, the macro expander's,
the loader's term walkers, the dumper and the import resolver among them, stay total over every node, so a library
caller may hand one a whole form.

**Why.** A pair inside a term is a shape the grammar refuses, so an arm that dropped or labelled one would read a
program no file can write; a walker that stays total costs nothing and keeps a whole-form walk correct.

**Held by.** `ein_ir::Node::KwPair`; `ein-infer/src/compile.rs`' arm *a pattern's argument is no keyword pair*;
`ein-ir/tests/fuzz_properties.rs`: `no_accepted_program_carries_a_keyword_its_form_does_not_list`.
