# Q-M1e2.28 — Each form takes a closed list of keywords

Each form takes a fixed list of keywords, and any other keyword is a parse error at its position, refused before
anything runs:

| form | its keywords |
|---|---|
| `relation`, `macro` | `:why` |
| `rule` | `:match`, `:assert`, `:ein`, `:why` |
| a fact, a stored negation | `:source`, `:rule`, `:using` |
| `query` | `:goal`, `:goal-text`, `:over`, `:expect` |
| `solve` | `:hypothesis-relations`, `:no-hypothesis`, `:no-search`, `:expect` |
| `config` | `:lattice-sanity-check`, `:record-alternative-justifications` |
| `import` | `:symbols`, `:as` |
| a trace's `step` | `:rule`, `:using`, `:derives`, `:source`, `:assumes` |

A term and a nested fact take no keyword, so a keyword pair inside a pattern, a premise, a conclusion, a goal or a
macro's call is refused; there is no `:where`, and a guard is a premise of its own, `(neq ?a ?c)`.

**Why.** A keyword a form does not read, accepted and dropped, is a typo that still loads: `:match (and (p ?x)
:where (neq ?x a))` would fire on `(p a)`. A closed list makes every such slip an error at the place it is written.

**Held by.** The grammar, `ein-ir/src/ein.y` and `ein-ir/src/ein.l`: each keyword is a token of the forms that take
it, and any other `:word` is a token no production takes. Its table, `ein-ir/tests/grammar_pages.rs`, one row for
each form and for each place in a term; `ein-ir/tests/ir_semantics.rs`: `a_keyword_is_never_a_value_nor_a_head`;
`examples/broken/expect_unknown_keyword.ein`, `solve_unknown_keyword.ein`, `query_lattice_keyword.ein`.
