# LLM & Constrained Generation

Decoding-time constraints, grammar-guided sampling, and the new wave of
"constrained reasoning" frameworks that put a symbolic filter between a
language model and its output.

---

## 1. Inference internals

### Logit
Raw pre-softmax score the network assigns to each vocabulary token.
Operations like temperature, top-k, top-p and grammar masking are all
defined on logits because the space is unbounded reals rather than a
probability simplex.
- Wikipedia: <https://en.wikipedia.org/wiki/Logit>

### Softmax
Converts a vector of logits into a probability distribution.
- Wikipedia: <https://en.wikipedia.org/wiki/Softmax_function>

### Temperature, top-k, top-p (nucleus) sampling
Common decoding strategies that reshape or truncate the logit/probability
distribution before sampling.
- Holtzman et al., *The Curious Case of Neural Text Degeneration*
  (nucleus sampling): <https://arxiv.org/abs/1904.09751>

### Tokenization / vocabulary
Modern LLM vocabularies are 30k–200k learned text fragments (subwords),
not words or characters. Examples by family:
| family | approx vocab |
|---|---:|
| Llama | ~128k |
| Qwen | ~150k+ |
| Mistral | ~32k |
| GPT-style BPE | ~50k–200k |

Tokenization affects reasoning quality, multilingual ability, code
performance, grammar-constraint interaction and inference speed.
- Wikipedia (BPE): <https://en.wikipedia.org/wiki/Byte_pair_encoding>
- OpenAI Tokenizer UI: <https://platform.openai.com/tokenizer>

### llama.cpp
C/C++ runtime for LLM inference; the host where GBNF lives.
- <https://github.com/ggml-org/llama.cpp>

---

## 2. Grammar-constrained decoding

### GBNF — GGML BNF
Grammar format used by llama.cpp to restrict next-token sampling at
inference time. Grammar is compiled to a parser automaton; tokens whose
acceptance would leave the parser in an invalid state are masked out
(`logit = −∞`). Cost: the model still has its full knowledge but loses
expressive freedom; symptom is repetitive or degenerate output when the
grammar is too strict or tokenizer-misaligned.
- llama.cpp grammar docs: <https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md>

### BNF / EBNF
The classical context-free grammar metalanguages GBNF derives from.
- Wikipedia (BNF): <https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form>
- Wikipedia (EBNF): <https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form>

### Parser-guided decoding
Generalisation of GBNF: the decoder consults an external parser/automaton
to mask invalid continuations. Used for JSON, XML, SQL, tool-call
syntaxes, programming-language subsets, enums, and structured agents.

### "Hybrid scratchpad" pattern
Let the model reason in free text inside `<scratchpad>…</scratchpad>`
and only constrain the final emitted block with GBNF. Preserves reasoning
quality while guaranteeing machine-readable output. Conceptually the
same insight CRANE formalises (below).

---

## 3. Constrained reasoning frameworks

### Constraints-of-Thought (Const-o-T)
Uses Monte Carlo Tree Search to guide an LLM through planning. Each
reasoning step is an `(intent, constraint)` pair; the constraint prunes
the search space and ties generation to user intent.
- <https://arxiv.org/abs/2510.08992>

### Graph-Constrained Reasoning (GCR)
Restricts decoding to valid paths in a Knowledge Graph via a KG-Trie
index, so answers can only follow grounded knowledge paths.
- <https://arxiv.org/abs/2410.13080>

### CRANE — Constrained Reasoning Augmented Generation
Alternates unconstrained generation (for thinking) with constrained
generation (for output format). Direct motivation: pure constrained
decoding damages reasoning trajectories.
- <https://arxiv.org/html/2502.15652v1>

### Schema-Guided Reasoning (SGR)
Uses explicit templates / Pydantic-style schemas to define ordered
reasoning steps. Common in compliance, finance, structured extraction.
- <https://abdullin.com/schema-guided-reasoning/>
- <https://arxiv.org/html/2502.09061v4>

### KG-Trie
Compact trie index of paths in a knowledge graph, used as the constraint
set for GCR-style decoding. See GCR above.

### Pydantic
Python data-validation library; canonical schema substrate for SGR-style
"force JSON to match this type" patterns.
- <https://docs.pydantic.dev/>

---

## 4. Neuro-symbolic / verifier-guided generation

### Neuro-symbolic AI
Umbrella term for systems that combine neural generation with symbolic
verification/search (SAT, SMT, planners, theorem provers, graph
reasoners).
- Wikipedia: <https://en.wikipedia.org/wiki/Neuro-symbolic_AI>

### Verifier-guided / proof-constrained inference
LLM proposes candidates → external symbolic engine validates → trajectory
restricted to verified states. Generalisation of grammar-constrained
decoding from token-level syntax to state-space / semantic admissibility.

### Self-modifying constraint languages (open direction)
Radical extension: the LLM emits not only content but proposed updates
to its own grammar / constraint system. Requires an invariant kernel +
versioned compatibility layer to avoid "recursive semantic drift"
(private-language collapse).

### Tool-calling / structured output / agent protocols
Practical sweet spot for grammar-constrained decoding today — APIs, JSON
function calls, schema-bound extractors.

---

## 5. Levels of constraint

Three structurally distinct layers, easy to conflate but worth keeping
separate:

| layer | example | scope |
|---|---|---|
| syntax | GBNF, parser-guided decoding | token-local |
| semantic admissibility | KG-Trie, ontology checks, type checks | state-space |
| global validity | theorem checker, SMT verifier | whole-proof |

Implication: grammar alone never controls semantics — a separate
typechecker / interpreter / verifier is required to keep the output
*meaningful* and not just *parseable*.

---

## 6. Reading list

- Const-o-T: <https://arxiv.org/abs/2510.08992>
- GCR: <https://arxiv.org/abs/2410.13080>
- CRANE: <https://arxiv.org/html/2502.15652v1>
- SGR overview: <https://abdullin.com/schema-guided-reasoning/>
- llama.cpp grammars README:
  <https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md>

## Cross-references

- Solvers used as the "verifier" half →
  [02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md)
- Knowledge graphs used by GCR →
  [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- S-expression IR design (favoured over Haskell-/Prolog-like surface
  syntax for LLM-emitted languages) →
  [04-programming-languages.md](04-programming-languages.md)
- The "LLM as policy over proof states" idea →
  [09-cognitive-architectures-neurosymbolic.md](09-cognitive-architectures-neurosymbolic.md)
