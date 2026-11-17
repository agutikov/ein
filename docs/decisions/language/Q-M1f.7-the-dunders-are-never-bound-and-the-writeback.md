# Q-M1f.7 — The dunders are never bound, and the writeback names never written

The kernel's two dunders, `__closed__` and `__symmetric__`, are refused wherever a program **binds** a name: as a
declared relation, as a rule's, an hrule's or a macro's name, and as an import's `:as`, each with the loader's line
*… shadows a reserved kernel name*. They are legal wherever a name is **used**: as the relation of a fact, a nested
fact, a premise, a conclusion, a goal or a claim's fact, and as an argument, so `std.algebra`'s `(imply1 functional
__closed__)` hands the kernel's trigger to a rule that asserts it. The engine's three writeback rule names,
`<forced-positive>`, `<monotonic-unconditional>` and `<lookahead-dies-immediately>`, are refused wherever a program
writes them.

**Why.** Reserved means used only with its fixed meaning, never redefined, and only a binding position can give a
dunder another meaning; an argument that names the trigger is how a rule reaches it. The writeback names are the
engine's provenance for a conclusion no rule drew, with no meaning a program can write.

**Held by.** `ein-ir/tests/engine_names.rs`: `a_dunder_is_refused_wherever_a_program_binds_it`,
`a_dunder_is_legal_wherever_a_program_uses_it`, `a_writeback_name_is_refused_wherever_a_program_writes_it`,
`a_writeback_name_in_a_string_or_a_comment_or_a_longer_name_is_not_one`. The lexer, `ein-ir/src/ein.l`
(`WRITEBACK`).
