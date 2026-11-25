# The host, checked again (2026-11-25, twice)

**Taken** 2026-11-25 on the user's word, on the machine the work runs on, on AC power: once in the morning
(*"check installed software again"*) and again in the afternoon, after the user upgraded the system and installed
the GPU backends (*"lean - `source ~/.elan/env`; cuda backends installed"*). **This page states the afternoon's
state**; what the morning found and has since changed is the first table. **Everything here is a fact about one
machine on one day**: re-take it (§ How it was taken) before leaning on it.

## What changed

| | the analysis said | the morning | the afternoon |
|---|---|---|---|
| `llama-server` | not installed | installed (`llama-cpp 0.9.0-1`), **did not start**: it needs `libssl.so.4` and the system had OpenSSL 3.6.3 | **starts**: the system is upgraded (OpenSSL 4.1.0, no package waiting) |
| GPU inference | not checked | **none**: neither runtime had a GPU backend, the model ran on the CPU at about 20 tokens a second | **works**: `ggml-cuda 0.29.0` and `ollama-cuda 0.40.16` are installed, `llama-server` offloads the whole model, about 66 tokens a second |
| a model file | none | `qwen3.5:4b` in ollama's system store, a GGUF file, pulled 2026-11-20 (the audits had not looked in the store) | the same file |
| GBNF | — | ollama 0.35.1 does not honour a grammar | ollama 0.40.16 does not either; **`llama-server` does**, on the same file |
| `swipl`, `clingo` | not installed | installed | installed |
| `lean` | not installed | not found | **installed by `elan`**, outside `PATH`: `source ~/.elan/env` first (the user) |

## The machine

| | |
|---|---|
| GPU | NVIDIA GeForce RTX 4060 Laptop, 8 GB (CUDA sees 7 834 MiB); `cuda 13.4.3` |
| the driver | **the loaded kernel module is 610.57.04 and the user-space libraries are 620.70.06** (`nvidia-utils`, `nvidia-open-dkms`, upgraded 13:23 and 13:25 with no reboot since). `nvidia-smi` refuses (*Failed to initialize NVML: Driver/library version mismatch*); **CUDA works all the same**, by the probe below. A reboot loads the new module |
| memory | 31 GiB, 31 GiB swap |
| containers | Docker, Compose, `nvidia-container-toolkit 1.20.1`; no `llama` or `ollama` image is pulled, and none is needed by the route the user chose ([Q-M2.14](../open_questions.md#q-m214--what-runs-the-model)) |
| toolchains | `rustc` and `cargo` 1.97.1, Python 3.14.7, `uv` 0.12.5 |

## The model runtimes

| runtime | state |
|---|---|
| **`llama-cpp 0.9.0-1`** | `llama-server --version`: `0.9.0-dev (build 12110, commit cd959bae99)`. The package holds `llama-server`, `llama-cli` and thirty other tools, the libraries `libllama.so.0`, `libllama-common.so.0`, `libggml.so.0`, and the headers `/usr/include/llama.h` and `llama-cpp.h`: **a program can link it**. Its backends are the `ggml` packages': `ggml 0.29.0` (CPU) and `ggml-cuda 0.29.0` (`/usr/lib/ggml/libggml-cuda.so`), with the `ggml-*.h` headers. `llama-server --list-devices` lists `CUDA0` |
| **`ollama 0.40.16-1`** with `ollama-cuda` | the service is **active** (started 13:51 by hand; it is not enabled) and had `qwen3.5:4b` loaded at `100% GPU` by its own `llamacpp` runner when this was taken. Two stores: the system's, `/var/lib/ollama` (`qwen3.5:4b`, `nomic-embed-text`), and the user's, `~/.ollama/models` (four other models, 20 GB) |
| **Rust bindings to llama.cpp** (crates.io, by `cargo search`; none is installed or tried) | `llama-cpp-2 0.1.161` (`utilityai/llama-cpp-rs`; features `cuda`, `dynamic-link`, `system-ggml`, `sampler`, `llguidance`, `mtmd`), `llama-cpp-4 0.12.0` with `llama-cpp-sys-4`, `llama_cpp 0.3.2`, `llama-cpp-bindings 0.31.0`. **Which one, and whether it builds its own llama.cpp or links the host's, is [S2.2.2](../p2.2_formalizer/s2.2.2_model_runtime.md)'s**; what it has to check first is that the llama.cpp it gets knows the architecture `qwen35`, which the host's 0.9.0 does |

## The model

`qwen3.5:4b`, the model the user chose ([Q-M2.5](../open_questions.md#q-m25--local-model-choice)), as
`ollama show` reads it from the system store:

| | |
|---|---|
| architecture, size | `qwen35`, 4.2B parameters, quantization Q4_K_M |
| files | the model, a GGUF of 2.47 GiB, `/var/lib/ollama/blobs/sha256-a9d0a3e1d8c91732a3812becd31f8d4a8ff57414903f226306069f0437e2ac50`, readable by the user; a vision projector, a GGUF of 0.63 GiB (`clip`, 333.51M parameters) |
| context length | 262 144 |
| capabilities | `completion`, `vision`, `tools`, `thinking` (on by default) |
| default parameters | `temperature 1`, `top_k 20`, `top_p 0.95`, `presence_penalty 1.5` |
| licence | Apache 2.0 |

On the GPU, without the projector and with a context of 4 096 tokens, `llama-server` projected 2 771 MiB of device
memory (2 515 the model, 178 the context, 78 compute) and the device's free memory fell from 3 877 MiB to 992 MiB
while it was loaded; ollama's own copy held the rest.

## What the model does through `llama-server`: a probe

`llama-server --model <the blob> -c 4096 -np 1 -ngl 99 --no-mmproj` on a port of its own, stopped afterwards, and
eight requests, each run once at `temperature 0`, `seed 1`. **A probe, not a measurement**: one run each.
Thinking is turned off per request with `chat_template_kwargs: {"enable_thinking": false}`.

| request | what came back |
|---|---|
| *Ann is older than Bob. Bob is older than Cy. Who is the youngest? Answer with one name.*, thinking off | `Cy`, 2 tokens, 0.10 s |
| the same, **thinking on** (the default) | `Cy` after 271 generated tokens (897 characters of reasoning), 4.1 s, about 66 tokens a second |
| *Write three sentences about the sea.* with **a GBNF `grammar`** that allows only `(youngest Ann)`, `(youngest Bob)` or `(youngest Cy)`, thinking off | `(youngest Bob)`: **the grammar is honoured**, against the prompt |
| the question with that grammar, thinking off | `(youngest Cy)` |
| the question with that grammar, **thinking on** | an empty answer and 13 characters of reasoning: **the grammar took the thinking channel too**. A grammar over the whole output and a thinking model do not compose as they stand: thinking is off, or the grammar starts where the thinking ends |
| the question with a JSON schema as `response_format` | `{"youngest": "Cy"}` |
| *Use the ein_solve tool on the program "(older Ann Bob)".* with one declared function `ein_solve(program)` | a tool call, `ein_solve {"program":"(older Ann Bob)"}`, and no text |
| `/completion` with the prompt `(youngest `, a grammar and `n_probs 3` | `Cy)`, with each token's `logprob` and `top_logprobs` |

**Through ollama 0.40.16** (three requests to the running service, thinking off): a GBNF `grammar` at the top
level and inside `options` both came back as free prose about the sea, and a GBNF string as `format` was refused
(HTTP 500). **The answer to the user's question is no**: ollama takes a JSON schema and no grammar, on this
version as on 0.35.1.

## The solvers and parsers the plan names

For [P2.8](../p2.8_representations/README.md) and
[S2.6.4](../p2.6_ablations/s2.6.4_representation_ablations.md).

| installed | not installed |
|---|---|
| `z3 4.16.0`, `cvc5 1.3.0` (`cvc5-bin`), `cvc4 1.8`, `souffle 2.5`, `clingo 5.8.3`, `swi-prolog 10.0.2`, `racket 9.3`, `link-parser 5.13.2` (`link-grammar`; 5.13.0 before the day's upgrade), `python-nltk 3.10.3`; **Lean 4.36.0** with `lake`, by `elan` under `~/.elan`, after `source ~/.elan/env` | `minizinc`, `vampire`, `eprover`, `prover9`, `mace4` |

## What still needs the user's hand

Nothing here was changed by the check: the session has no `sudo`, and the host is the user's.

1. **A reboot** brings the kernel module to the libraries' version, after which `nvidia-smi` reads the GPU again.
   CUDA inference works without it.
2. **Two programs want the same 8 GB.** While ollama keeps a model loaded, `llama-server` has what is left; an
   experiment that counts on the device's memory stops the other first.

## How it was taken

```sh
source ~/.elan/env; lean --version; elan show
pacman -Q ollama ollama-cuda llama-cpp ggml ggml-cuda openssl cuda nvidia-utils; pacman -Qu | wc -l
grep -E 'upgraded (nvidia|openssl|ollama)|installed (ollama-cuda|ggml-cuda)' /var/log/pacman.log | tail
cat /proc/driver/nvidia/version; nvidia-smi
llama-server --version; llama-server --list-devices; pacman -Ql llama-cpp | grep -E '\.h$|\.so'
systemctl is-active ollama; systemctl is-enabled ollama; ollama --version; ollama list; ollama ps
cargo search llama-cpp --limit 12; cargo info llama-cpp-2
M=/var/lib/ollama/blobs/sha256-a9d0a3e1d8c91732a3812becd31f8d4a8ff57414903f226306069f0437e2ac50
llama-server --model $M --host 127.0.0.1 --port 18080 --no-webui --offline -c 4096 -np 1 -ngl 99 --no-mmproj -lv 4 &
# then the eight POSTs of the table to /v1/chat/completions and /completion, and three to ollama's /api/chat
```
