# Q-M1e2.25 — The parser is generated from the grammar, and its recovery is deterministic

The grammar is two files of `ein-ir`, `src/ein.l` and `src/ein.y`, in grmtools' notation, and the parser is generated
from them when the crate is built, so a conflict in the grammar fails the build. The grammar's page is read out of
the same two files. A file the parser refuses reports every error recovery reaches, each at its true position with
what was found there and at most three ranked repairs, and stays refused: no repaired program is loaded.

Recovery is deterministic: `lrpar` is a patched copy, `ein.rs/vendor/lrpar/`, whose `ein.patch` is the whole
difference from the published crate. It keeps equal repairs in the order its search found them and stops at a count
of search steps rather than at a wall-clock time, and a report cut there says so. The patch stays in the tree.

**Why.** One statement of the language, which the parser is, rather than a grammar, a parser and a loader held
together by tests; and a refused file reports the same errors on every run and every machine.

**Held by.** `ein-ir/build.rs`, `ein_ir::grammar`; `ein-ir/tests/generated_parser.rs`: `a_report_is_the_same_twice`,
`a_report_cut_at_the_budget_says_so`; `ein-ir/tests/fuzz_properties.rs`:
`a_generated_refusal_is_the_same_twice_and_within_its_budget`; `ein-ir/tests/grammar_pages.rs`:
`the_pages_grammar_blocks_are_the_parsers`, which holds the grammar's page,
[00_ebnf.md](../../kernel/ir/03-ein-lang/00_ebnf.md), to the two files.
