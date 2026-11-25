# Q-M1f.520 — A shared rule's words are one text under every base

`std.math.positional`'s rules are written once for every base, and so are their `:why` texts: the walk that checks a
numeral reads *the digit 1 is one: … is still to check* and *every place of … holds a digit* under binary as under
decimal, a bit being a digit of base two. Only a numeral a text writes is the base's own
([Q-M1f.298](Q-M1f.298-a-shared-rules-text-writes-the-bases-own-numeral.md)); no base passes a word of its own.

**Why.** A rule's `:why` is filled by its variables
([Q-M1f.58](../../read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)), and a word that a base passed would be
one more parameter of the macro that connects the base, and of every base's call of it, for a word that is right
under both.

**Held by.** `stdlib/math/positional.ein`: `numeral-step`, `numeral-step-zero` and `numeral-valid`;
`tests/stdlib/math/binary/01_well_formed_everywhere.ein`, binary's numerals walked by them. The page:
[`std.math.positional`'s rules](../../../ein/user/04-reference/04-stdlib/10-positional-rules.md#the-checks-walk).
