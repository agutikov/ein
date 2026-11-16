# Q-M1f.96 — A byte-order mark is refused as any character that is not ASCII

A file that starts with a byte-order mark, U+FEFF, is refused at 1:1 with the line every character that is not
ASCII gets outside a string or a comment: `'\u{feff}' (U+FEFF) is not ASCII: Unicode is allowed only inside a
string literal (after :why, :source or :goal-text) or in a comment`. It is not skipped.

**Why.** A character that is not ASCII is written only inside a string literal or a comment
([Q-M1f.65](Q-M1f.65-a-comment-may-hold-any-character.md)), and the file's first position is no exception; one rule
with no special case is the one a reader can predict.

**Held by.** `ein-ir/tests/grammar_pages.rs`, the grammar's table row *the file's first character*;
`ein-cli/tests/unicode_only_in_a_string.rs`: `every_character_in_every_place_is_refused_by_the_parser`, the
byte-order mark among its characters and the file's first character among its places.
