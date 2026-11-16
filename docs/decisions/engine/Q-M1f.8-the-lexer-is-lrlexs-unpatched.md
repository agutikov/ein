# Q-M1f.8 — The lexer is lrlex's, unpatched

The lexer is the one `lrlex` generates from `ein.l`, as published: one rule for whitespace and comments, a start
condition, and one lexer built per thread. The only patched crate of the parser is `lrpar`, for deterministic
recovery ([Q-M1e2.25](Q-M1e2.25-the-parser-is-generated-from-the-grammar.md)).

**Why.** The parser stays the grammar's, in one notation a published tool reads. What lexing costs is how fast a
file is read, never what it reads.

**Held by.** `ein-ir/src/ein.l`, `ein-ir/src/grammar.rs` and the exact versions in `ein-ir/Cargo.toml`;
`ein-ir/tests/generated_parser.rs`: `the_lexer_gives_the_tokens_t4s_did`, which holds the lexer's tokens, on every
corpus file and on seeded mutants, to those of the same rules written without the skip rule and the start condition.
