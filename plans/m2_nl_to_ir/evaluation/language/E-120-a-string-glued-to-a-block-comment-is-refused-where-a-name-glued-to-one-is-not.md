# E-120 — A string glued to a block comment is refused, where a name glued to one is a name and a comment

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.3, 2026-11-26, on `680a94ad`
**Owner:** none
**Since P2.2's close (2026-11-27):** none; the lexer's, for the iteration of Ein

## What

**Where a token ends is not one rule.** `ein.l`'s head says that a token runs to a separator, *a space, a paren, `;`,
`#|` or the end*, and that the catch-alls `BAD`, `BADTEXT` and `BADESCAPE` make it so. For a name it holds: `(p
a#|c|#)` is the fact `(p a)` and a comment, since `BAD` takes no `#` followed by `|`. For a string it does not:
`BADTEXT`'s tail is `[\x00-\x7F--[\s();\x1c-\x1f]]+`, which takes `#`, so `(relation p T :why "{?3} is p"#|c|#)` is
one bad token, refused at the parse with *unexpected `"{?3} is p"#|c|#`: a string glued to what follows* and the repair
*Insert a string, Delete …*. A line comment glued to a string is a comment (`"x";c` loads), since the tail takes no
`;`. Neither the syntax page nor the grammar page states the difference; the grammar page's table of the lexer's rules
shows it to a reader who reads `BADTEXT`'s class.

## Evidence

Run on `680a94ad`'s `ein` (`ein.rs/target/gate/ein solve FILE --no-search`): `(relation p T :why "{?3} is p"#|c|#)`
refused as above; `(relation p T :why "{?3} is p")` and `(p a#|c|#)`, root alive; `(relation p T :why "{?3} is p";c`
and the paren on the next line, root alive. EinAf's whole grammar has to say it by hand: after a string, a separator
does not begin with `#` (`gap-text`, `gap-after-text` in `einaf/grammars/ein.gbnf`, the inventory's row). Of the five
hand-written rules, this is one the corpus and the parser's banked table of refused shapes do not see
([E-121](E-121-the-banked-table-of-refused-shapes-misses-two-of-the-lexer-s-token-ends.md)).

## What it costs

Little to a model: a grammar written in the model's layout puts no comment beside a string. It costs a reader of Ein's
syntax a rule that is stated one way and works another, and a second reader of the language (EinAf's grammar) a rule
written by hand that nothing in Ein tests but the parser itself.
