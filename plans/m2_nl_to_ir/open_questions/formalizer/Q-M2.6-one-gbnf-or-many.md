# Q-M2.6 — One GBNF or many?

**Status:** open

**Since (the analysis, 2026-11-25).** This was Q24 here. The grammar's source is not the page the plan names:
`01-grammar.md` holds pictures and one table. The text is `ein.l` and `ein.y` (LR(1), 101 rules), or the twenty
generated EBNF diagram sources. Reserved words out of `NAME`, and the ends of tokens, are still hand work.

**Working answer**: many — one GBNF per pass (ontology, theory, facts,
query; the alternatives wrapper if S2.4.5 keeps it). Smaller grammars
decode faster and let the prompt focus the LLM. Decided in
[S2.2.3](../../p2.2_formalizer/s2.2.3_gbnf.md), which also measures the one
thing that could reverse it — decoding speed under the full ein-lang
grammar.

**Since (the host checks and the user's answers, 2026-11-25).** A grammar is llama.cpp's: `llama-server` honoured
one on the model's file, and ollama takes none ([`analysis/host.md`](../../analysis/host.md)), so the model runs
through a binding to llama.cpp ([Q-M2.14](../model/Q-M2.14-what-runs-the-model.md)). **The grammar is the filter's**: the
tool writes Ein with none ([Q-M2.13](../einaf/Q-M2.13-who-calls-the-kernel.md)), and the user expects that a model may
write Ein without one, which [S2.2.2](../../p2.2_formalizer/s2.2.2_model_runtime.md) measures before a grammar is
built. A grammar over the whole output of a thinking model took the thinking too: where the grammar begins is
part of [S2.2.3](../../p2.2_formalizer/s2.2.3_gbnf.md)'s design.

**Since (the re-plan of P2.2 to the user's steps, 2026-11-26).** The formalization is made in three steps, the facts
and a partial ontology, then the complete ontology and a partial theory, then the complete theory
([the user's notes on P2.2](../../p2.2_formalizer/README.md#formalization-in-steps-the-users-notes-on-the-whole-phase)),
so *many* reads **one grammar a step** (and one for step 2's second call, which writes rules on a marker), as
[S2.2.3](../../p2.2_formalizer/s2.2.3_gbnf.md) now plans it. Step 1 writes its facts before their declarations, so
no grammar closes its vocabulary and EinAf's check holds a step-1 fact to a declaration; the closure a restricted
grammar buys is steps 2 and 3's. The verdict is still S2.2.3's, on the probe's errors, the speeds and what that
closure is worth.
