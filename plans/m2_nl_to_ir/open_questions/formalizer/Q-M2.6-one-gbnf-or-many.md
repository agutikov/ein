# Q-M2.6 — One GBNF or many?

**Status:** open; [S2.2.3](../../p2.2_formalizer/s2.2.3_gbnf.md)'s default, for the user's review: *many*, a grammar a
step, the whole grammar kept

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

**Since (S2.2.3, 2026-11-26): the default the stage took, for the user's review — *many*.** A grammar a step, steps
1, 2, 2b (step 2's second call) and 3, each the whole grammar restricted to the step's forms in its order and rendered
at request time on what the step is given (step 1 on the input's sentences, step 2 on the catalogue and step 1's
program, steps 2b and 3 on the catalogue and the program of steps 1 and 2); **and the whole grammar kept**, generated
from `ein.l` and `ein.y` (`einaf/grammars/ein.gbnf`), as the root of a program written in one call and as the
parser's second reader. Each is carried inside the fence the prompt asks for, begun after the thinking at the token
`</think>` where thinking is on, and held to by the chain's choice first. The evidence, from
[S2.2.3's report § 6](../../p2.2_formalizer/grammars/README.md#6-q-m26-one-grammar-or-many):

- **The probe's errors, by what would have prevented each.** The whole grammar removes the fence lines, a keyword a
  form does not take and the described condition's shapes. **A step's grammar removes three kinds more**: a module
  listed as a name of another (`import-list`), a name declared and listed from an import (`full-name`, where step 1
  declared it), a `:why` naming an arrow its declaration lacks (E-119); and in steps 2b and 3 a pattern of an
  undeclared relation or at another count (E-12). **Neither removes** a conclusion's variable no premise binds
  (`unbound-conclusion`, half the library condition's refusals), or anything that made a loaded program wrong.
- **The speeds.** Held by the chain's choice first, a step's grammar costs a generation 1 to 3 % of its speed (more
  where it refuses often: step 3, 8 and 22 % with 12 and 16 refusals in 256 tokens) and the whole grammar 0 to 1 %; a
  grammar's sampler is built in 0.19 to 0.56 ms against a prompt evaluated in 1.4 s. Applied to every candidate first,
  the step grammars run at 12.7 to 25.6 tokens a second and the whole grammar at 11.2 to 12.8, against 64: *smaller
  grammars decode faster* only there ([Q-M2.28](../../p2.2_formalizer/open_questions/model/Q-M2.28-is-a-token-held-to-its-grammar-by-the-chain-s-choice-first.md)).
- **What the vocabulary closure is worth.** Steps 2b and 3 close every pattern, conclusion, activating fact and
  hypothesis to a relation the program or a module it imports declares, at a declared count, over the program's
  names. Step 1 closes no fact (under its grammar the model still wrote a fact of a relation it never declared), and
  step 2 none of its ontology's facts ([Q-M2.29](../../p2.2_formalizer/open_questions/formalizer/Q-M2.29-are-step-2-s-facts-held-to-the-relations-step-2-declares.md)).

**What would have reversed it**, by S2.2.3's terms: the probe finding no error a step's grammar removes and the whole
one does not (it found three kinds), or a step's grammar costing more to build a call than it saves (under a
millisecond). Neither held. Left open for the user.

**What S2.2.4's runs measured** (2026-11-26, [its report § 9](../../p2.2_formalizer/steps/README.md#9-with-no-grammar)),
the steps under their grammars against the same requests with no grammar, qwen3.5:4b, thinking off, greedy: a step's
grammar changed a generation only where it refused a token (6 of 16 texts' three steps byte for byte the free runs');
where it refused, it kept out of the programs 11 activating facts under a name the import does not list, 7 `:source`
that are no sentence and 3 second answer queries, and four programs from a refusal (14 load against 11); and it sent
7 of 49 calls into a loop to the budget that the free runs did not enter
([L-M2.19](../../p2.2_formalizer/findings/model/L-M2.19-a-grammar-s-refusal-sends-greedy-decoding-into-a-loop-it-does-not-enter-free.md)).
The answers were the same with and without. The whole grammar's root, in one call, refused a token in 2 calls of 16.
The default, *many*, stands.
