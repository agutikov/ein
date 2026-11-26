# L-M2.13 — The steps by hand carry no marker of the grammar's shape before their rules

**Status:** closed by S2.2.4 (`7aaefe6f`): the steps by hand write the marker above each rule, and the test reads it
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

## How it was closed

The marker's shape is the prompts' (`einaf/prompts/steps/step3.v1.md`, `step2.v1.md`, `step2b.v1.md`): a comment line
`;; missing: ` and the property's words, directly above the rule. The five steps 3 by hand that write rules
(`youngest`, `seating`, `apples`, `together`, `more`, twelve rules) write it above each, below their `;; Synthesised: …`
comment; `einaf/tests/grammar.rs`' `part` reads the marker from the file and fails where a rule has none, and writes
nothing into what it is held to. The contract's page shows the worked example's marker.
