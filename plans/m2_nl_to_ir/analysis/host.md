# The host, checked again (2026-11-25)

**Taken** 2026-11-25 on the user's word (*"check installed software again"*), on the machine the work runs on, on
AC power, after the analysis beside this page was written. Three packages were installed after the audits ran
(`llama-cpp`, `swi-prolog`, `clingo`), and the audits had not looked in ollama's stores, so the analysis's two
sentences about the host are corrected from here. **Everything on this page is a fact about one machine on one
day**: re-take it (§ How it was taken) before leaning on it.

## What changed against the analysis

| | the analysis said | now |
|---|---|---|
| `llama-server` | not installed | installed (`llama-cpp 0.9.0-1`, 2026-11-25) and **does not start** |
| a model file | none | `qwen3.5:4b` is in ollama's system store, a GGUF file, pulled 2026-11-20: it was there, and the audits did not look in the store |
| `swipl`, `clingo` | not installed | installed (`swi-prolog 10.0.2`, `clingo 5.8.3`, both 2026-11-25) |
| how a model runs | not checked | **on the CPU**: neither runtime has a GPU backend installed |

## The machine

| | |
|---|---|
| GPU | NVIDIA GeForce RTX 4060 Laptop, 8 188 MiB, driver 610.57.04; `cuda 13.3.1` installed |
| memory | 31 GiB, 31 GiB swap |
| disk | 778 GiB free under `/home`, 86 GiB under `/` |
| containers | Docker 29.7.2, Compose 5.5.0, `nvidia-container-toolkit 1.20.0`; `docker info` lists the GPU's CDI devices; no `llama` or `ollama` image is pulled |
| toolchains | `rustc` and `cargo` 1.97.1, Python 3.14.7, `uv` 0.12.5; of the Python packages a model client would use, `requests` and `nltk` are installed and `openai`, `httpx`, `pydantic`, `torch`, `transformers` are not |

## The model runtimes

| runtime | state |
|---|---|
| **`ollama 0.35.1-1`** | installed 2026-11-20; the repository has 0.40.16. The `ollama` package holds CPU backends only (`/usr/lib/ollama/libggml-cpu-*.so`); `ollama-cuda` is in the repository and not installed. **The service is inactive and disabled.** Two stores: the system's, `/var/lib/ollama` (3.4 GB: `qwen3.5:4b`, `nomic-embed-text`), and the user's, `~/.ollama/models` (20 GB: `nemotron-9b-v2-q4`, `nemotron-nano-9b-v2-q4k-ctx131k-b64`, `dagbs/deepseek-coder-v2-lite-instruct:iq4_xs`, `thirdeyeai/Qwen2.5-Coder-7B-Instruct-Uncensored:Q4_0`) |
| **`llama-cpp 0.9.0-1`** (`llama-server`, `llama-cli`) | installed 2026-11-25. **Neither binary starts**: `error while loading shared libraries: libssl.so.4`. The package is built against OpenSSL 4; the system has `openssl 3.6.3` (`/usr/lib/libssl.so.3`), the sync database has 4.1.0, and 1 239 packages wait for an upgrade. Its backends are the `ggml 0.29.0` package's, CPU only; `ggml-cuda` and `ggml-vulkan` are in the repository and not installed |
| a `llama-server` container | not tried. The plan's route ([S2.2.2](../p2.2_formalizer/s2.2.2_llama_server_and_client.md)) brings its own libraries and would meet neither host problem; pulling an image is that stage's act |

## The model

`qwen3.5:4b`, the model the user chose ([Q-M2.5](../open_questions.md#q-m25--local-model-choice)), as
`ollama show` reads it from the system store:

| | |
|---|---|
| architecture, size | `qwen35`, 4.2B parameters, quantization Q4_K_M |
| files | the model, a GGUF of 2.47 GiB, `/var/lib/ollama/blobs/sha256-a9d0a3e1d8c91732a3812becd31f8d4a8ff57414903f226306069f0437e2ac50`; a vision projector, a GGUF of 0.63 GiB (`clip`, 333.51M parameters) |
| context length | 262 144 (ollama's default window was 4 096 with no GPU found) |
| capabilities | `completion`, `vision`, `tools`, `thinking` (on by default) |
| default parameters | `temperature 1`, `top_k 20`, `top_p 0.95`, `presence_penalty 1.5` |
| licence | Apache 2.0 |

Both files begin with the bytes `GGUF`, and ollama's own runner is started as
`/usr/lib/ollama/llama-server --model <that blob> --mmproj <the projector>`: the file is one `llama-server` reads.

## What the model does through ollama: a probe

A temporary `ollama serve`, run as the user over the system store and stopped afterwards, and six requests to
`/api/chat`, each run once at `temperature 0`, `seed 1`. **A probe, not a measurement**: one run each.

| request | what came back |
|---|---|
| *Ann is older than Bob. Bob is older than Cy. Who is the youngest? Answer with one name.*, thinking off | `Cy`, 2 tokens; the model loaded in about 4 s |
| the same, **thinking on** (the default) | `Cy`, after 243 generated tokens, 810 characters of them thinking: the default spends two orders of magnitude more tokens on this question, which matters to a budget counted in tokens |
| the same with **a JSON schema as `format`** (an object whose `youngest` is one of three names) | `{"youngest": "Cy"}`: the schema is honoured |
| *Write three sentences about the sea.* with **a GBNF `grammar`** that allows only `(youngest Ann)`, `(youngest Bob)` or `(youngest Cy)`, as a top-level field | three sentences about the sea: **the grammar is not honoured** |
| the same with the grammar inside `options` | the same prose |
| *Use the ein_solve tool on the program "(older Ann Bob)".*, with one declared function `ein_solve(program)` | a tool call, `ein_solve {"program": "(older Ann Bob)"}`, and no text |

**Where it ran: the CPU.** `ollama ps` said `100% CPU`, the server's log found no GPU (`inference compute id=cpu`,
`total_vram="0 B"`), and the GPU's memory stayed at 90 MiB. Generation was about 20 tokens a second.

## The solvers and parsers the plan names

For [P2.8](../p2.8_representations/README.md) and
[S2.6.4](../p2.6_ablations/s2.6.4_representation_ablations.md).

| installed | not installed |
|---|---|
| `z3 4.16.0`, `cvc5 1.3.0` (`cvc5-bin`), `cvc4 1.8`, `souffle 2.5`, `clingo 5.8.3`, `swi-prolog 10.0.2`, `racket 9.3`, `link-parser 5.13.0` (`link-grammar`), `python-nltk 3.10.3` | `lean`, `minizinc`, `vampire`, `eprover`, `prover9`, `mace4` |

## What needs the user's hand

Nothing here was changed: the session has no `sudo`, and the host is the user's.

1. **`llama-server` does not start** until the system has OpenSSL 4 (the pending upgrade), or the server runs in
   a container.
2. **No GPU inference** until a GPU backend is installed for the runtime that is used (`ggml-cuda` for
   `llama-cpp`, `ollama-cuda` for `ollama`). Whether the 4B model with its projector and a useful context fits
   8 GB is then to be measured, not assumed.
3. **The ollama service is disabled.** The probe ran `OLLAMA_MODELS=/var/lib/ollama OLLAMA_NOPRUNE=1 ollama serve`
   as the user, which reads the system store and cannot write to it.

## How it was taken

```sh
for c in ollama llama-server llama-cli cargo rustc python3 uv racket z3 cvc5 cvc4 clingo swipl souffle \
         lean minizinc vampire eprover prover9 mace4 link-parser; do command -v $c || echo "no $c"; done
pacman -Qo /usr/bin/llama-server; pacman -Qi llama-cpp ggml swi-prolog clingo ollama | grep -E '^(Name|Version|Install Date|Depends On)'
llama-server --version; ldd /usr/bin/llama-server | grep 'not found'; ls /usr/lib/libssl.so*
pacman -Ss '^(ollama|llama-cpp|ggml)'; pacman -Si openssl | grep Version; pacman -Qu | wc -l
systemctl is-active ollama; systemctl is-enabled ollama
find ~/.ollama/models/manifests /var/lib/ollama/manifests -type f
nvidia-smi --query-gpu=name,memory.total,memory.used,driver_version --format=csv
docker info | grep -i nvidia; pacman -Q nvidia-container-toolkit
OLLAMA_MODELS=/var/lib/ollama OLLAMA_NOPRUNE=1 OLLAMA_HOST=127.0.0.1:11434 ollama serve &   # then:
ollama list; ollama show qwen3.5:4b; ollama ps      # and the six POSTs to /api/chat of the table above
```
