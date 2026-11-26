# L-M2.11 — A grammar applied to every candidate costs a generation most of its speed; the chain's choice checked first costs it nothing

**Status:** closed by S2.2.3 (`680a94ad`): a request's grammar carries `check`, and every grammar EinAf writes is held
to by the chain's choice first
**Found:** by S2.2.3, 2026-11-26, in its first generations under a grammar
**Owner:** S2.2.3; whether the arms keep the order is [Q-M2.28](../../../open_questions/model/Q-M2.28-is-a-token-held-to-its-grammar-by-the-chain-s-choice-first.md)

## What

S2.2.2's runtime applies a request's grammar to every candidate before the sampler chain (its default 5), so that a
grammar that leaves no token is seen at once. llama.cpp's grammar sampler checks a candidate by walking its piece's
characters against every stack the grammar holds, and the vocabulary is 248 320 tokens: at every generated token,
every token of the vocabulary is read. **The speeds' table** ([the stage's report § 5](../../grammars/README.md#5-the-speeds))
has it grammar by grammar; the first two generations under a grammar showed it first: the probe's step 1 of
`youngest` under step 1's grammar at 22.7 tokens a second, and its library run of `youngest` under the whole grammar at
13.4, against 62 with no grammar. **Checked the other way round**, the chain's choice offered alone and the vocabulary
read only where the grammar refuses that choice (llama.cpp's own sampler's order, which its server runs), the same two
generations ran at 62.3 and 62.4 tokens a second, **the same tokens**, one and none of their choices refused after the
first token.

## Evidence

The speeds' record, [`../../grammars/speed/record.jsonl.gz`](../../grammars/speed/record.jsonl.gz), every grammar
under both checks, three calls each; `einaf::model::GrammarCheck`.

## What it costs

Under `every-token` a grammar a step runs a generation at 12.7 to 25.6 tokens a second and the whole grammar at 11.2
to 12.8, against 64 with none (the speeds, through `bench_env.sh`, 256 tokens, three calls each): the grammar's work on
the vocabulary is 25 to 73 ms a token beside the model's 15.6. Under `chosen-first` the same tokens come at 50.1 to
64.2, within 3 % of none but where the grammar refuses the model's choice. `einaf::grammar::request_grammar` writes `chosen-first`, and
`every-token` stays what a request that names no check gets, so that S2.2.2's records replay as made.
