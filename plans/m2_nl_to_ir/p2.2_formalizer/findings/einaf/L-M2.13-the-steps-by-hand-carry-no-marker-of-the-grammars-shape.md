# L-M2.13 — The steps by hand carry no marker of the grammar's shape before their rules

**Status:** open
**Found:** by S2.2.3, 2026-11-26, holding step 3's grammar to the steps by hand
**Owner:** [S2.2.4](../../s2.2.4_passes.md), where the steps' prompts fix the marker's shape

## What

The marker of a missing property is a comment line of a fixed shape the prompt asks for and the grammar allows
(S2.2.4's T2.2.4.3). S2.2.3's grammars take it as `;; missing: ` and the property's words, the line directly above each
rule of step 3 and of step 2's second call. The contract's steps by hand (`einaf/contract/*/step3.ein`) write
`;; Synthesised: …` above a rule, a sentence that runs over one, two or three comment lines, and no line of the
grammar's shape. So the test that holds each step's grammar to the steps by hand writes a marker of its own above
each rule, `;; missing: ` and the rule's name (`einaf/tests/grammar.rs`, `part`).

## Evidence

`einaf/tests/grammar.rs`' `each_steps_grammar_accepts_the_steps_written_by_hand`, and
`plans/m2_nl_to_ir/p2.2_formalizer/grammars/llama_steps.py`, which writes the same parts for llama.cpp.

## What it costs

A test that writes into what it is held to. Once S2.2.4 fixes the marker's shape and its prompts ask for it, the steps
by hand write it above their rules and the grammar's `MARKER` follows, and the test reads the files as they are.
