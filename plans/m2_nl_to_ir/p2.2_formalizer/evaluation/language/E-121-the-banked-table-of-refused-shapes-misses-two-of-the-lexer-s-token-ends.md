# E-121 — The parser's banked table of refused shapes, with the corpus, misses two of the lexer's token ends

**Kind:** gap
**Status:** open
**Filed:** by S2.2.3, 2026-11-26, on `680a94ad`
**Owner:** none

## What

**The table and the corpus were not enough to test a second reader of Ein's syntax; texts sampled from the reader's
own grammar were.** EinAf's whole grammar (S2.2.3) writes five of Ein's lexer's rules by hand. Each was broken in turn,
one change to the generated text, and each broken grammar run against the three checks that need no model:

| the hand-written rule broken | corpus files on which grammar and parser part (of 937) | rows of the banked table the grammar takes (of 352) | texts sampled from the broken grammar the parser refuses (of 2 000) |
|---|---:|---:|---:|
| a name may be one of the 29 words | 40 | 111 | 12 |
| `?_` is a variable | 0 | 13 | 2 |
| a string may be glued to `#` | **0** | **0** | 156 |
| a line comment need not end its line | **0** | **0** | 914 |
| two words need no separator | 26 | 74 | 726 |

The table ([`ein-ir/tests/golden/grammar_table.txt`](../../../../../ein.rs/crates/ein-ir/tests/golden/grammar_table.txt),
one row a shape the grammar refuses, each with its parse error) has a row for a string glued to a name (`"a"b`) and to
a character that is not ASCII, and none for a string glued to a block comment ([E-120](E-120-a-string-glued-to-a-block-comment-is-refused-where-a-name-glued-to-one-is-not.md));
it has no row for a form whose closing paren a line comment takes (`(p a ;c)` with no line break after it). No corpus
file is either.

## Evidence

[`plans/m2_nl_to_ir/p2.2_formalizer/grammars/mutants/mutants.rs`](../../grammars/mutants/mutants.rs), run as an EinAf
test on `680a94ad` and removed; its output is [the stage's report](../../grammars/README.md#2-the-grammar-held-to-the-parser).

## What it costs

A second reader held to the corpus and the table alone would accept texts the parser refuses, and a model under it
could write one: an *invalid* outcome no grammar on would let it avoid. The two rows are the table's to add, as a shape
the parse refuses; EinAf holds its own grammar by the sampler meanwhile.
