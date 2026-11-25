# Q-M2.5 — Local model choice

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25:** *"Qweb3.5:4b"*.

**Read as `qwen3.5:4b`**, the ollama tag, which is in the host's system ollama store (pulled 2026-11-20):
architecture `qwen35`, 4.2B parameters, quantization Q4_K_M, a 2.47 GiB GGUF file (`sha256:a9d0a3e1d8c9…`) with a
0.63 GiB vision projector, context length 262 144, capabilities `completion`, `vision`, `tools` and `thinking` (on
by default), default parameters `temperature 1`, `top_k 20`, `top_p 0.95`, `presence_penalty 1.5`. This was Q23
here; its working answer (Qwen3-30B-Instruct, Phi-4 14B as the fallback) and its list of candidates are gone.

**What stays of the entry:** the model is an **experimental constant**, not a product choice. It is pinned by its
file's SHA for the whole of Level C; a second model would be a replication, not a fallback, and none is named.

**The user since, 2026-11-25:** *"qwen model (or other suitable models)"*. The model of the plan stays
`qwen3.5:4b`; another suitable model is a second value of the record's model field, taken when an experiment asks
for one, and the comparison of two models is then a replication.

**On the host** ([`analysis/host.md`](../../analysis/host.md), checked twice 2026-11-25): the tag names a GGUF file in
ollama's system store, which llama.cpp reads as it is. With the GPU backend the user installed, `llama-server`
loaded the whole model on the device (2 771 MiB at a context of 4 096 tokens, without the projector) and
generated about 66 tokens a second. What runs it is [Q-M2.14](Q-M2.14-what-runs-the-model.md).
