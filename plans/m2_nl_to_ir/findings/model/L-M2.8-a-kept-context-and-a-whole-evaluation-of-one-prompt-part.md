# L-M2.8 — A kept context and a whole evaluation of one prompt part under greedy decoding

**Status:** open
**Found:** by S2.2.2, 2026-11-26, showing that a call on a kept context pays for its new tokens only
**Owner:** [P2.4](../../p2.4_loop/README.md), whose loop and tool arm keep the context between turns; the record's
layout, [S2.5.1](../../p2.5_harness/s2.5.1_experiment_record.md)
**Since P2.2's close (2026-11-27):** [S2.4.1](../../p2.4_loop/s2.4.1_state_machine.md), [S2.4.6](../../p2.4_loop/s2.4.6_ein_as_a_tool.md), [S2.5.1](../../p2.5_harness/s2.5.1_experiment_record.md)

## What

A tool call's round trip, through the binding, greedy: the first turn (299 prompt tokens) writes the call
`ein_solve` on `(older Ann Bob)`; the second turn's prompt is the first's, the call, and the function's response
(365 tokens). **Continued on the kept context**, 329 of the 365 tokens are not evaluated again (`prompt.reused`
329, 7 ms against 30 ms); **evaluated whole**, the same 365 tokens give another answer:

| the second turn | its answer |
|---|---|
| on the kept context (the binding) | *The program `(older Ann Bob)` loads successfully. The root holds this program, and the outcome is "one".* |
| evaluated whole (the binding) | *The program `(older Ann Bob)` evaluates to **one**.* |
| evaluated whole (`llama-server`, build 12110) | *… loads successfully. The root holds the expression `(older Ann Bob)`.* |

The prompts' token ids are equal in all three (the record's digest). The kept context's tokens were evaluated one
at a time as they were generated, and the whole prompt's in one batch: the logits differ in their last bits, and
greedy decoding parts where two candidates are near (at the binding's 8th token against its own whole evaluation).
And the binding's llama.cpp is 681 commits behind the server's, which parts the whole evaluations at the 8th token
too, while the first turn and nine other requests of the host probe are generated token for token alike.

## Evidence

`plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/host_probe/10a_tool_call_kept.json`, `10b_tool_response_kept.json`,
`10c_tool_response_whole.json`, their records in `host_probe/record.jsonl`, and `reference/host_probe.jsonl`'s
rows for them.

## What it costs

A run that keeps its context and a run that does not are two inputs to greedy decoding, not one: a loop's record
says which it did (the envelope's `prompt.reused`), and a replay of a loop replays its calls, not a fresh evaluation.
An arm compared with another is compared under one of the two.
