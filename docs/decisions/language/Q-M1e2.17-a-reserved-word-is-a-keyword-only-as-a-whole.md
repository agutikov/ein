# Q-M1e2.17 — A reserved word is a keyword only as a whole token

A reserved word is read as itself only where the token is that word and nothing more. Followed by any name
character it is the longer name it spells: `neq-1`, `notx`, `rule-x` and `rule.x` are names, so `(neq-1 ?x)` is a
pattern of the relation `neq-1`. In a `(trace …)` the one event is `step`, read whole: `(trace (stepx :a b))` is a
parse error, as `(trace (foo :a b))` is.

**Why.** A keyword and a name never overlap: the lexer takes the longest match, so no reserved word is split off
the front of a name, and the reading of a token does not depend on the character after it.

**Held by.** The lexer, `ein-ir/src/ein.l`. `ein-ir/tests/grammar_decisions.rs`:
`a_keyword_is_a_whole_word_in_every_place`, every reserved word followed by each name character in every place a
keyword is read, and `the_documented_ambiguities_keep_their_decisions`. The name rule:
[Q-M1e2.24](Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md).
