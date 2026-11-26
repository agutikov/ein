# L-M2.19 — A grammar's refusal can send greedy decoding into a loop it does not enter with no grammar

**Status:** open
**Found:** by S2.2.4, 2026-11-26, setting its runs under the steps' grammars beside the same runs with none (`7aaefe6f`)
**Owner:** [Q-M2.31](../../open_questions/model/Q-M2.31-does-a-step-s-grammar-bound-the-comment-lines-the-model-writes.md)
and [Q-M2.27](../../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md),
greedy decoding; the steps' grammars, S2.2.3's
**Since P2.2's close (2026-11-27):** [S2.2c.1](../../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md)

## What

**Under the steps' grammars, 7 of 49 calls ran to their budget of 12 288 tokens; with no grammar, none of 48 did.**
Held by the chain's choice first, a grammar changes a generation only at the tokens it refuses (the envelope's
`chosen_again`): where it refuses none, the run is the free run token for token (the Zebra puzzle, the seating,
`before`, `bystander`, `contra`, `gap_fact`: all their steps alike under both). **Each of the seven is a call whose
grammar refused a token** (`youngest`'s step 2 one token, `unused`'s step 2 five, `boxes12`'s step 3 nine,
`boxes13`'s step 2's second call two, `pencils`' step 3 one) **or a call whose prompt carried such a loop** (`unused`'s
and `boxes13`'s step 3, given the program with the looped step in it). The same steps with no grammar ended their turn
in 27 to 607 tokens (`boxes13` with no grammar wrote no marker and had no second call). Where the grammar refuses, greedy decoding takes the next most likely token and goes on from a
text it would not have written, and on these the text it went on to was a loop: comment lines (L-M2.16), a premise
repeated, a fact repeated.

## Evidence

[S2.2.4's record](../../p2.2_formalizer/steps/README.md): the conditions `steps` and `steps-free`, the calls' envelopes
(`runs/steps/*/calls.jsonl.gz`, `grammar.chosen_again`) and each step's output under both; the report's § 9.

## What it costs

The seven calls cost their budget (about four minutes each) and gave no form or a cut one; and the grammar's gain
(three programs more load) comes with them. Sampling instead of greedy decoding, a bound on comment lines, or a
refusal that ends the call rather than steering it are each a way out; none is taken.
