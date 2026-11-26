# L-M2.7 — A lazy grammar is fed from its trigger's first non-empty group, so the trigger is the grammar's own first text

**Status:** open
**Found:** by S2.2.2, 2026-11-26, running the host probe again through the binding (its request 9, a grammar that
begins after the thinking)
**Owner:** [S2.2.3](../../p2.2_formalizer/s2.2.3_gbnf.md), where a grammar begins
**Since P2.2's close (2026-11-27):** [S2.2b.1](../../p2.2b_guided_thinking/s2.2b.1_questions_by_hand.md), the thinking's grammar joined to a step's

## What

llama.cpp's lazy grammar (`llama_sampler_init_grammar_lazy_patterns`, the binding's `grammar_lazy_patterns`) waits
until the generated text matches a trigger pattern whole, and then feeds the grammar the text from **the first
non-empty capture group** on; where every group is empty it feeds **the whole buffer**. Three requests of the host
probe through the binding, with thinking on and the grammar `(youngest Ann|Bob|Cy)`:

| trigger pattern | grammar | what came back |
|---|---|---|
| `[\s\S]*?</think>([\s\S]*)` | `root ::= [ \n]* "(youngest " … ")"` | the failure *the grammar refused its own token 248069* (`</think>`): the group is empty when `</think>` arrives, so the whole thinking was fed to the grammar |
| `[\s\S]*?</think>([\s\S]+)` | the same | 268 thinking tokens, then **1 780 tokens of white space** to the budget: `[ \n]*` let greedy decoding write newlines for ever, since its first choice, `Cy`, was forbidden |
| `[\s\S]*?(</think>)[\s\S]*` | `root ::= "</think>" "\n"? "\n"? "(youngest " … ")"` | 268 thinking tokens, then `(youngest Cy)`, the grammar beginning at the token after the thinking (`grammar.began_at` 268); llama-server, given the same pattern as a full-match trigger, generates the same 275 tokens |

So **a trigger is the grammar's own first text**, `</think>`, as llama.cpp's own tool-call grammars begin with
`<tool_call>`, and the white space a grammar allows after it is bounded. The trigger buffer is the whole thinking,
matched against the pattern at every token: a pattern that starts `[\s\S]*?` is matched against a growing text,
which for a thinking of thousands of tokens is std::regex's work at every token. A trigger token, the id of
`</think>`, is the other form the API takes and needs no regex; the runtime's `GrammarStart` names patterns only.

## Evidence

`plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/host_probe/09_youngest_grammar_after_thinking.json` (the third
row) and its record in `host_probe/record.jsonl`; the first two rows' requests and records in
`host_probe/lazy_attempts.jsonl`.

## What it costs

S2.2.3's grammar that begins after the thinking is written with `"</think>"` as its first text, and its white space
bounded, or it fails on its first token or never ends. Whether the trigger is a token or a pattern is its choice.
