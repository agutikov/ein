# E-143 — AMR's `and` and `or` are words Ein never takes as a name

**Kind:** weakness, as filed; **expected behaviour**, the user's ruling
**Status:** closed by the user's ruling, 2026-11-27: Ein works as designed; a notation read as facts renames
what collides with Ein's words, as EinAf's spelling does (`and_`)
**Filed:** by [S2.2f.2](../../s2.2f.2_the_analysis_written.md), 2026-11-27, on `dd3910aa` with the stage's
`einaf analysis`
**Owner:** none

**Ruled by the user, 2026-11-27:** *"E-140, a program made only of facts gets 84,309 guessing candidates at root unless it ends with (solve :no-search) - this is expected behaviour; E-141, ein render kb draws every one of those candidates - also expected behaviour, maybe have to print warning and add limit and flag to change it; E-143, and and or can never be names in Ein - also expected behaviour"*.

## What

**AMR's `and` and `or` are words Ein never takes as a name.** Ein's lexer makes ten words tokens of their own that are
never a name: the declarators and the connectives, `and`, `or`, `neq`, `rule`, `query`, `solve`, `config`, `trace`,
`macro`, `import` (`ein.l`; the position words, `not`, `false`, `open` and the rest, are names where a name goes but a
head). AMR's conjunction and disjunction are the concepts `and` and `or`, the first in every graph of a list (*Ann,
Bob and Cy sit in a row*: `(a / and :op1 … :op2 … :op3 …)`); written as an argument, `(amr-node S1 s1-z2 and)` is
refused at the parse. EinAf writes every word of Ein's grammar with `_` after it, `and_`, so that none is read as
Ein's, and says so in the facts form's paragraph. A notation read as facts must carry a renaming of whatever of its
vocabulary collides with these words.

## Evidence

The seating's analysis under each parser holds `and` (sentence 1); the spelling is `einaf::analysis::name`, its
`EIN_WORDS`; `printf '(relation p A :why "a {?3}")\n(p and)\n' > a.ein && ein solve --no-search a.ein` is a parse
error at `and`, and `(p and_)` loads.

## What it costs the application

One rule of the spelling and a sentence of the prompt; a model shown `and_` may write it back. S-expression
notations read as facts ([M1k's P1k.20](../../../../m1k_notations_as_facts/p1k.20_s_expressions_as_facts/README.md))
meet the same words.
