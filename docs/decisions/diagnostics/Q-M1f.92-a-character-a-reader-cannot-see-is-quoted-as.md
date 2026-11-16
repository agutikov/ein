# Q-M1f.92 — A character a reader cannot see is quoted as its escape

The line that refuses a character that is not ASCII names it in quotes with its code point. A space, a control, a
zero-width or bidirectional format character, a combining mark or a variation selector is quoted as the escape a
string writes it with, `'\u{a0}' (U+00A0)`; any other character is quoted as itself, `'é' (U+00E9)`.

**Why.** Written between the quotes, a no-break space or a byte-order mark shows nothing, and a right-to-left
override reorders the line. The escape shows what the line names, and one error stays one line.

**Held by.** `ein_ir::grammar::shown_char`; `ein-cli/tests/unicode_only_in_a_string.rs`:
`every_character_in_every_place_is_refused_by_the_parser`, whose characters each carry the way the line shows them.
