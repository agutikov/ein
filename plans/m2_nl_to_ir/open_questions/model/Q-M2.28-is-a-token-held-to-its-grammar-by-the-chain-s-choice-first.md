# Q-M2.28 — Is a generated token held to its grammar by the chain's choice first?

**Status:** open

**Raised 2026-11-26 by [S2.2.3](../../p2.2_formalizer/s2.2.3_gbnf.md)**, from its speeds ([the stage's report](../../p2.2_formalizer/grammars/README.md#5-the-speeds)).

**The question.** S2.2.2's runtime applied a request's grammar to every one of the vocabulary's 248 320 candidates
before the sampler chain, at every generated token: a grammar that leaves no token is then seen at once. It cost the
generation most of its speed (the speeds' table, `every-token`). llama.cpp's own sampler, the one its server runs,
holds a token to a grammar the other way round: the chain chooses among every candidate, its choice alone is offered to
the grammar, and only where the grammar refuses it is the grammar applied to every candidate and the chain asked
again. S2.2.3 built that order as a request's `check`, `chosen-first`, beside `every-token`. **Under greedy decoding the
two choose the same tokens**, the most likely token the grammar allows, and the speeds' runs hold it token for token.
Under a chain that draws, they are two distributions where the chain cuts the candidates before its draw (top-k,
top-p, min-p): the second is llama.cpp's server's, and not the grammar's mask applied before the cut.

**The default taken**: `chosen-first` for every grammar EinAf writes (`einaf::grammar::request_grammar`), and the
envelope says which (`grammar.check`), with the count of tokens whose first choice was refused (`chosen_again`).
`every-token` stays a request's choice and is what a request that names no `check` gets, so that S2.2.2's records
replay as they were made.

**What the default costs.** Nothing under the phase's greedy runs. Under the arms' chain
([S2.5.2](../../p2.5_harness/s2.5.2_baselines.md), the model's own by default: top-k 20, top-p 0.95), a filter
that draws as llama.cpp's server draws, and not as a mask over the model's distribution would.

**The alternatives.** `every-token` for every grammar, at its cost; or `chosen-first` under greedy decoding and
`every-token` under a chain that draws.
