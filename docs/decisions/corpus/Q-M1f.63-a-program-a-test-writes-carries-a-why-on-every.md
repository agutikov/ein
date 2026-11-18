# Q-M1f.63 — A program a test writes carries a `:why` on every form

A test that writes a program for the engine to load and answer, a generator of programs among them, writes a `:why`
on every relation, rule, hrule and macro: a mechanical one that says what the form says, its clauses in words, and
never a placeholder. The program loads under the default options. A generator keeps its random draws, so a seed
writes the program it wrote before. A test that mutates the corpus's own programs starts from programs that carry
their `:why`s, and keeps its draws too.

**Why.** A generated program that loads only with the requirement off asks the engine a question no user's program
asks. A `:why` that states the form keeps every surface that prints one, a trace or a read-out, readable on a
generated program as on a written one.

**Held by.** The generators of the engine's tests: `ein-infer/tests/detector.rs`, `compile_limits.rs` and
`match_alloc.rs`, `ein-ir/tests/imports_semantics.rs`, `ein-cli/tests/region_dump.rs`; and
`ein-ir/tests/fuzz_properties.rs`, which mutates the corpus's programs.
