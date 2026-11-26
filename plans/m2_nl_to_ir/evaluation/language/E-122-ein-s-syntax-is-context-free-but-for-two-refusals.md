# E-122 — Ein's syntax, its lexer included, is a context-free language but for two refusals, and a second reader takes it exactly

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by S2.2.3, 2026-11-26, on `680a94ad`
**Owner:** —

## What

**What Ein's parser accepts is a context-free language, but for a text nested past 1 000 parentheses and a byte that
is not UTF-8.** EinAf generated a GBNF grammar from `ein.l` and `ein.y` through grmtools' readers: each lexer rule's
regular expression node for node, each rule of `ein.y` compiled into a rule a state, and five rules written by hand for
what the lexer does as a pass of its own (the longest match and the earlier rule on a tie, where a token ends, a line
comment's end, the tokens no production uses). It accepts every one of the corpus's 937 files the parser accepts and
refuses every one it refuses (856 and 81, each refusal on the line of the parser's first error), refuses every one of
the 352 shapes of the parser's banked table, and every one of 2 000 texts sampled from it parses. llama.cpp's own
grammar sampler, fed the same files token by token with the model's vocabulary, reads them as EinAf's acceptor does.

**What it cost.** The lexer's longest match is regular where it matters: a name is the run of its characters less the
29 words an earlier rule takes whole, which is a tree of 175 rules over their prefixes; and a separator is required
between two tokens that are neither a paren, which needs each production compiled into states that know the kind of
the token before (337 rules from `ein.y`'s 101, of the grammar's 531). Neither is anything a context-free grammar cannot say.

## Evidence

[The stage's report](../../p2.2_formalizer/grammars/README.md), and EinAf's tests, `einaf/tests/grammar.rs`
(`the_grammar_accepts_a_corpus_file_exactly_when_the_parser_does`, `every_shape_of_the_kernels_table_is_refused_by_the_grammar`,
`every_text_sampled_from_the_whole_grammar_parses`); `einaf grammar llama`'s record, `grammars/llama/corpus.jsonl.gz`.

## What it serves

A grammar of Ein is a mechanism, not an approximation: what a filter allows is what the parser takes, so an output
held to it parses, and the parse's *invalid* is left to the loader's refusals and the contract's check.
