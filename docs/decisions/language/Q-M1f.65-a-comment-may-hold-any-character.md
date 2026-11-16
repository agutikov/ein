# Q-M1f.65 — A comment may hold any character

A character that is not ASCII may be written inside a string literal and inside a comment, a `;` comment or a `#| …
|#` one; anywhere else it is refused ([Q-M1f.96](Q-M1f.96-a-byte-order-mark-is-refused-as-any-character.md)). The
line that refuses one says so: *Unicode is allowed only inside a string literal (after :why, :source or :goal-text)
or in a comment*.

**Why.** A comment is free text, as a string is: no reader of the program reads it, so a character in it can
neither make two names look like one nor change an answer. The corpus's comments use box-drawing characters and
arrows, and holding them to ASCII would buy nothing.

**Held by.** `ein-cli/tests/unicode_only_in_a_string.rs`: `a_string_literal_and_a_comment_hold_any_character`, each
of twelve characters in a `:why`, a `:source`, a `:goal-text`, a `;` comment and a `#| … |#` comment, loaded and
answered by every command. The lexer: `ein-ir/src/ein.l`.
