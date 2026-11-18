# Q-M1f.7 — The dunder is never bound, and the writeback names never written

The kernel's dunder, `__closed__`, is refused wherever a program **binds** a name: as a declared relation, as a
rule's, an hrule's or a macro's name, and as an import's `:as`, each with the loader's line *… shadows a reserved
kernel name*. It is legal wherever a name is **used**: as the relation of a fact, a nested fact, a premise, a
conclusion, a goal or a claim's fact, and as an argument, so `std.algebra`'s `(imply1 functional __closed__)` hands
the kernel's trigger to a rule that asserts it. The engine's three writeback rule names, `<forced-positive>`,
`<monotonic-unconditional>` and `<lookahead-dies-immediately>`, are refused wherever a program writes them. No other
name is the kernel's: `__symmetric__`, `is-a` and `symmetric` are names like any other, a theory's or a program's,
which a program may bind.

**Why.** Reserved means used only with its fixed meaning, never redefined, and only a binding position can give the
dunder another meaning; an argument that names the trigger is how a rule reaches it. The writeback names are the
engine's provenance for a conclusion no rule drew, with no meaning a program can write. The kernel expects the
minimal shape of a program, its rules and relations, and no theory's name: a relation's symmetry is the rule
`std.algebra` states, which a program imports, and the kernel mirrors no relation of its own.

**Held by.** `ein-ir/tests/engine_names.rs`: `a_dunder_is_refused_wherever_a_program_binds_it`,
`a_dunder_is_legal_wherever_a_program_uses_it`, `a_name_the_kernel_does_not_read_binds_as_any_name_does`,
`a_writeback_name_is_refused_wherever_a_program_writes_it`,
`a_writeback_name_in_a_string_or_a_comment_or_a_longer_name_is_not_one`. The lexer, `ein-ir/src/ein.l`
(`WRITEBACK`). `ein-infer/tests/stdlib_semantics.rs`: `the_kernel_mirrors_no_relation_and_symmetric_is_a_rule`;
`ein-render/tests/renaming_invariance.rs`, every corpus program answering alike with every name that is not the
kernel's renamed.
