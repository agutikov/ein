# Q-M1f.88 — A misplaced `open` is a parse error with a note saying what `open` is

`open` where the grammar does not take it, the bare `(open)`, `open` directly under `open`
([Q-M1f.568](../language/Q-M1f.568-directly-under-absent-or-open-the-other.md)), and `open` as the name a
`relation`, a `rule`, a `macro` or an import's `:as` binds, is refused with the parse error's line,
``<file>:<line>:<col>: unexpected `open` ``, or ``unexpected `)` `` at the bare `(open)`'s end, and under it the
note `open takes one argument: a relation's name, which a state owes, or a fact a state could hold, which is a
hypothesis; directly under it neither open nor absent is written, and no declarator binds it`, then the error's
repairs. Where the kernel reads `open` and where it is a name:
[Q-M1f.66](../language/Q-M1f.66-open-is-a-name-and-the-kernel-reads.md).

**Why.** A parse error has one spelling: the first line names the token and its place, and a note says what a
decision says about it, as for `_` and for a string where an object goes.

**Held by.** `ein_ir::grammar::OPEN_NOTE`; `ein-cli/tests/open_reserved.rs`:
`open_is_refused_where_a_program_would_make_it_more_than_the_kernels`, every place on every command with one first
line; `ein-ir/tests/grammar_pages.rs`' table, a row per place, banked in `ein-ir/tests/golden/grammar_table.txt`;
`examples/broken/load/reserved_open_direct.expected`.
