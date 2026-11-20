# Q3 — Ein is an S-expression language

A program is a sequence of S-expressions: every form is a parenthesised list whose head is a declarator (`relation`,
`rule`, `macro`, `import`, `query`, `solve`, `config`, `trace`), a fact's relation, or `not` before a
stored negation, and whose parts are names, variables, nested forms, keyword pairs and, after `:why`, `:source` or
`:goal-text`, a string. Names carry the meaning; the syntax adds only the parentheses, `?` before a variable, and
`:` before a keyword.

**Why.** A small, uniform syntax is one context-free grammar short enough to be read in full, to be held to its
parser by a test, and to constrain a generator that writes Ein; the meaning stays in names a reader and a rule can
both see.

**Held by.** The grammar, `ein-ir/src/ein.l` and `ein-ir/src/ein.y`, from which the parser is generated; the
language page's two grammar blocks, held to them by `ein-ir/tests/grammar_pages.rs`:
`the_pages_grammar_blocks_are_the_parsers`.
