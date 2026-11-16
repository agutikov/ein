# Q-M1f.95 — A file is refused at its first character that is not ASCII

A file holding characters that are not ASCII outside a string and a comment is refused at the first of them, with
one error: `<file>:<line>:<col>: 'é' (U+00E9) is not ASCII: Unicode is allowed only inside a string literal (after
:why, :source or :goal-text) or in a comment`. The rest of the file is not reported.

**Why.** The lexer reads the whole file before the parse and stops at its first lexing error, which is where the
file stops being Ein. Finding every such character would mean a second scan of every string's and comment's extent.

**Held by.** `ein-cli/tests/unicode_only_in_a_string.rs`:
`every_character_in_every_place_is_refused_by_the_parser`, whose refusal is the one line, and
`a_character_that_is_not_ascii_is_refused_on_every_command`. The line is `ein_ir::grammar::not_ascii`'s.
