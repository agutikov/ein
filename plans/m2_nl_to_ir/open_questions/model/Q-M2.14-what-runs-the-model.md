# Q-M2.14 — What runs the model?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25:** *"I expect to use llama-cpp with rust wrapper, not ollama, because ollama does
not allow full LLM control and GBNF, does it?"*.

**The question in it, answered by the host check** ([`analysis/host.md`](../../analysis/host.md)): it does not. Ollama
0.40.16, as 0.35.1 before it, takes a JSON schema as `format` and no grammar: a GBNF `grammar` at the top level
and inside `options` came back as free prose, and a GBNF string as `format` was refused. On the same file
`llama-server`, of llama.cpp 0.9.0, honoured the grammar against the prompt, with the whole model on the GPU.

**Read as:** the model runs **inside EinAf's process, through a Rust binding to llama.cpp**: no server, no
container and no HTTP between the harness and the model. Ollama is no runtime of the plan; its store is where the
file was pulled to. What *full control* buys, and the plan uses:

- the sampler chain, with the grammar in it, the seed and every parameter a field of the record;
- thinking on or off a call, and a grammar that begins where the thinking ends (in the host check a grammar over
  the whole output of a thinking model took the thinking too);
- each token's log-probabilities;
- the context kept between the turns of one loop, so that a repair pays for the new tokens only;
- the chat template as a file of the record, not a server's default.

**Left to [S2.2.2](../../p2.2_formalizer/s2.2.2_model_runtime.md), each a default:**

- **Which binding.** On crates.io at the check: `llama-cpp-2`, `llama-cpp-4`, `llama_cpp`, `llama-cpp-bindings`;
  none is tried. Whether it builds a llama.cpp of its own or links the host's `libllama.so.0` is part of the
  choice, and the first thing it is held to is that the llama.cpp it gets knows the model's architecture,
  `qwen35`.
- **`llama-server`, of the same package, is the reference** a binding is compared with (one file, one grammar,
  one seed) and what the plan falls back to if no binding loads the model. The Compose service of the old plan,
  and `packaging/` with it, are dropped.
