# Q-M1f.88 — A misplaced `open` is a parse error with a note saying what `open` is

`open` written anywhere but as a rule's whole `:assert` is refused with the parse error's line,
``<file>:<line>:<col>: unexpected `open` ``, and under it the note `open is reserved: it is the verdict a rule's
:assert states, written as the whole :assert, (open) or (open ?R), and nowhere else, so it names no relation and no
value`, then the error's repairs. `open` names no relation and no value
([Q-M1f.66](../language/Q-M1f.66-open-is-written-only-as-a-rules-verdict.md)).

**Why.** A parse error has one spelling: the first line names the token and its place, and a note says what a
decision says about it, as for `_` and for a string where a value goes.

**Held by.** `ein_ir::grammar::OPEN_NOTE`; `ein-cli/tests/open_reserved.rs`:
`open_is_refused_wherever_it_is_not_the_verdict`, every place on every command with the note under the line;
`ein-ir/tests/grammar_pages.rs`' table, a row per place, banked in `ein-ir/tests/golden/grammar_table.txt`.
