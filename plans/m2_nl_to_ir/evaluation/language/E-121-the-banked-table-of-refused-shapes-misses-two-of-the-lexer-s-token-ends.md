# E-121 — The parser's banked table of refused shapes, with the corpus, misses two of the lexer's token ends

**Kind:** gap
**Status:** closed by [S2.2e.4](../../p2.2e_kernel_fixes/s2.2e.4_order_and_table_rows.md) (`e2b7c0c9`)
**Filed:** by S2.2.3, 2026-11-26, on `680a94ad`
**Owner:** none
**Since P2.2's close (2026-11-27):** [S2.2e.4](../../p2.2e_kernel_fixes/s2.2e.4_order_and_table_rows.md)

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

The table ([`ein-ir/tests/golden/grammar_table.txt`](../../../../ein.rs/crates/ein-ir/tests/golden/grammar_table.txt),
one row a shape the grammar refuses, each with its parse error) has a row for a string glued to a name (`"a"b`) and to
a character that is not ASCII, and none for a string glued to a block comment ([E-120](E-120-a-string-glued-to-a-block-comment-is-refused-where-a-name-glued-to-one-is-not.md));
it has no row for a form whose closing paren a line comment takes (`(p a ;c)` with no line break after it). No corpus
file is either.

## Evidence

[`plans/m2_nl_to_ir/p2.2_formalizer/grammars/mutants/mutants.rs`](../../p2.2_formalizer/grammars/mutants/mutants.rs), run as an EinAf
test on `680a94ad` and removed; its output is [the stage's report](../../p2.2_formalizer/grammars/README.md#2-the-grammar-held-to-the-parser).

## What it costs

A second reader held to the corpus and the table alone would accept texts the parser refuses, and a model under it
could write one: an *invalid* outcome no grammar on would let it avoid. The two rows are the table's to add, as a shape
the parse refuses; EinAf holds its own grammar by the sampler meanwhile.

## Closed

By S2.2e.4, `e2b7c0c9`, 2026-11-27: the table's last two rows are the two shapes, under a heading of their own
(*where a token ends, at a comment*), each with the parse error it gets today, banked with `EIN_BLESS=1`:

| row | program | the parse error |
|---|---|---|
| 352, *a block comment glued after a string* | `(relation t T :why "t"#\|c\|#)` | at 1:20, *unexpected* the one token `"t"#\|c\|#`, *a string glued to what follows*; the repair *Insert a string, Delete* that token |
| 353, *a form's closing paren after a line comment* | `(relation p T) (p a ;c)` | at 1:24, *unexpected end of input*; the repair *Insert* `)` |

Nothing the parser accepts or refuses moved; no row before them was renumbered. The grammar page's regions, written
from `ein.l` and `ein.y`, did not move: the page shows the lexer's rules and not this table. EinAf's
`every_shape_of_the_kernels_table_is_refused_by_the_grammar` reads the table and takes all 354 rows refused by its
whole grammar, the two new ones among them, so a second reader held to the table now meets both shapes. Whether a
string glued to a block comment should be refused at all is [E-120](E-120-a-string-glued-to-a-block-comment-is-refused-where-a-name-glued-to-one-is-not.md)'s,
the lexer's question for the iteration of Ein, and untouched: the row banks what the parser does today.
