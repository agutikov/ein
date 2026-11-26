# L-M2.14 — A grammar that leaves out the fence a prompt asks for refuses the model's first token, and the model writes the program its prompt shows nearest

**Status:** closed by S2.2.3 (`eff7d634`): a grammar a request carries begins inside the fence the prompt asks for
**Found:** by S2.2.3, 2026-11-26, in its first recorded generations under a grammar
**Owner:** S2.2.3; the steps' prompts, [S2.2.4](../../s2.2.4_passes.md)

## What

**Under greedy decoding with the chain's choice checked first, a grammar acts only at the tokens it refuses**: where
the model's own output is in the grammar, the grammar changes nothing. The probe's prompts ask for the program in one
fenced block, and with no grammar the model writes ```` ```ein ```` first, every time. The first recorded
generations put the grammar at the first token with no fence in it, the same requests otherwise. The fence refused,
the model's next choice under the whole grammar's root was the program its prompt shows nearest, **the excerpt's
`youngest` program, written for six other texts** (`before`, `bob1`, `boxes12`, `contra`, `gap`, `seating`: each 212
to 228 tokens, *one*, *“Cy is the youngest”*), and for the Zebra puzzle under step 1's grammar, the excerpt's `older`
facts each quoting a Zebra sentence (`(older Ann Bob :source "The Englishman lives in the red house.")`). Three runs
went on to the budget repeating a form (`step1-grammar.off.apples`, `.more`, `whole-grammar.off.more`, `.unused`:
cycles of 25 to 214 tokens). With no grammar the same requests wrote their own texts' programs
(the probe's record).

## Evidence

[`../../grammars/first/record.jsonl.gz`](../../grammars/first/record.jsonl.gz), the first plan's 24 runs, beside the
probe's same requests with no grammar: `python3 grammars/report.py grammars/first/record.jsonl.gz`;
[the stage's report § 4](../../grammars/README.md#4-where-a-grammar-begins-and-how-an-output-ends).

## What it costs

A grammar is the filter's mechanism, and a filter that moves the model off its own path writes a worse program than
no filter: here, another text's program. So **a grammar begins where the program begins**: the fenced frame
(`einaf::grammar::Frame`, `einaf grammar render --fence`) takes the model's own fence, and the second plan's runs are
under it. A prompt that asks for no fence is the other way; the two must agree, which is the steps' prompts' to keep
(S2.2.4).
