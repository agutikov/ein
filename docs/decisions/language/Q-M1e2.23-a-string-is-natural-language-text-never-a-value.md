# Q-M1e2.23 — A string is natural-language text, never a value

A string is written only as the value of `:why`, `:source` and `:goal-text`, the three keywords that hold natural
language. Anywhere else, a fact's argument, a pattern, a guard, a conclusion, a claim, it is refused at the parse,
with the note that a name is written bare: `(p "1")` is refused, and `(p 1)` is the fact. A variable in a fact is
refused the same way. A string may hold any Unicode text, with the escapes `\n`, `\t`, `\r`, `\"`, `\\` and
`\u{…}`, which must name a character; any other escape is refused.

**Why.** A name needs no quotes: every character a name may hold is written bare
([Q-M1e2.24](Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)). A string that spelled a name would be a second
spelling of one value, and the text a program shows a reader is the one place where any character belongs.

**Held by.** The lexer, `ein-ir/src/ein.l` (`TEXT`). `ein-cli/tests/cli_semantics.rs`:
`a_string_is_natural_language_and_no_value`; `ein-ir/tests/generated_parser.rs`:
`a_strings_escapes_are_the_rulings`; `ein-cli/tests/unicode_only_in_a_string.rs`:
`a_string_literal_and_a_comment_hold_any_character`; `ein-infer/tests/rule_semantics.rs`:
`a_string_in_a_rule_means_what_it_means_in_a_fact`.
