# Q-M1f.66 — `open` is a name, and the kernel reads it at the head of a written fact, a premise or a conclusion

`open` is a word a position gives a meaning to. At the head of a written fact, a premise or a conclusion the kernel
reads it: what a state owes, `(open R)`, and a hypothesis, `(open F)`
([Q-M1f.566](../hypotheses/Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md)). It takes one argument,
and directly under it neither `open` nor `absent` is written
([Q-M1f.568](Q-M1f.568-directly-under-absent-or-open-the-other.md)). Everywhere else it is a name: an
argument, a name `eq` compares, a fact's `:rule`, a trace step's rule, a macro's body, and under an ordinary relation
the head of a fact ([Q-M1f.571](Q-M1f.571-absent-takes-a-premise-and-open-a-name-or-a-fact.md)). It stays refused
where a program would declare it or make it more than the kernel's: a relation, a rule, a macro or an alias named
`open`, a name an import lists, a written `(open x)` with `x` neither a fact nor a relation of the program, an
obligation made to owe `open`, a `(solve …)` list that names it, and a claim's fact of it.

**Why.** A word the kernel reads where it stands, and that is a name wherever it is not read, is what `not` and
`absent` are; what a program could make of it beyond that would be a second meaning of the kernel's own relation.

**Held by.** `ein-ir/src/ein.y`; `ein_ir::grammar::POSITION_WORDS`; `ein_ir::from_ir`'s `write_open_facts` and
`refuse_open_owed`; `ein-cli/tests/open_reserved.rs`; the grammar's table (`ein-ir/tests/grammar_pages.rs`).
