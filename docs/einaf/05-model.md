# The model's runtime

**The model runs inside EinAf's process**, through a Rust binding to llama.cpp: no server, no HTTP between the
harness and the model. One trait, `einaf::model::Runtime`, takes a request and returns a generation with its
envelope, or a typed failure; two implementations exist, **the binding** (`einaf::model::llama::Llama`, behind a
Cargo feature) and **replay** (`einaf::model::replay::Replay`, always built), which answers from a record and needs
no model. The code is [`einaf/src/model/`](../../einaf/src/model/mod.rs).

| section | what |
|---|---|
| [1. Building with the model](#1-building-with-the-model) | the feature, the toolchain, the llama.cpp the binding builds |
| [2. Giving a run the model](#2-giving-a-run-the-model) | the file, by an option or a variable; the context |
| [3. A request and a generation](#3-a-request-and-a-generation) | what a call takes and returns; the thinking apart from the answer |
| [4. The envelope](#4-the-envelope) | what every call returns and every record keeps |
| [5. The chat template](#5-the-chat-template) | rendered by EinAf, the thinking switch, a declared function |
| [6. A grammar, and where it begins](#6-a-grammar-and-where-it-begins) | from the first token, or after a trigger |
| [7. A kept context](#7-a-kept-context) | a second call pays for its new tokens only |
| [8. Failures](#8-failures) | each a value |
| [9. Replay](#9-replay) | a record answers a request |
| [10. The commands](#10-the-commands) | `render`, `generate`, `grammar`, `model`, `probe` |
| [11. Held by](#11-held-by) | the tests |

## 1. Building with the model

**The runtime is a feature of the `einaf` package, off by default.** Without it EinAf builds and its tests pass with
no llama.cpp and no model, which is what its step of the gate runs; replay, the envelope and the template are built
either way.

```sh
cargo build --release --manifest-path einaf/Cargo.toml                      # no model: replay only
CMAKE_CUDA_ARCHITECTURES=89 \
  cargo build --release --features cuda --manifest-path einaf/Cargo.toml    # the binding, on the GPU
cargo build --release --features llama --manifest-path einaf/Cargo.toml     # the binding, on the CPU
```

| feature | what it adds |
|---|---|
| `llama` | the binding, `llama-cpp-2` 0.1.161, and the llama.cpp it builds: CMake and a C++ compiler |
| `cuda` | `llama`, with llama.cpp's CUDA backend: the CUDA toolkit (`nvcc`, `CUDA_PATH`); `CMAKE_CUDA_ARCHITECTURES` set to the device's compute capability (`89` for an RTX 40 series) builds one architecture, not all |

**The binding builds a llama.cpp of its own**, the one its crate vendors, commit
`d81235049384534c167caea52b85a694f6103d14`, statically into the `einaf` binary; it does not link a llama.cpp the
host has installed. The envelope names it (`runtime.llama_cpp`).

## 2. Giving a run the model

A run is given **the model's file**, by `--model PATH` or, where none is given, `$EINAF_MODEL`. **The path is never
the model's identity**: the file is hashed when it is loaded, and its SHA-256 is the envelope's `model.sha256`.
`--ctx N` is the context's length in tokens, 65 536 by default; the whole model is put on the device. The vision
projector a multimodal model's store holds beside it is a file of its own and is never loaded.

```sh
export EINAF_MODEL=/path/to/model.gguf
einaf/target/release/einaf model                 # the model loaded: its pin, the runtime, the context, as JSON
```

## 3. A request and a generation

A request (`einaf::model::Request`), as JSON:

| field | what |
|---|---|
| `prompt` | `{"kind": "chat", "messages": [{role, content, reasoning_content?}…], "tools": […]}`, rendered through the model's own template; or `{"kind": "text", "text": …}`, tokenised as it stands |
| `thinking` | whether the model thinks before it answers: the template opens the thinking block, or closes it empty |
| `grammar` | `null`, or `{text, root, start, check}`: a GBNF grammar, its root, where it begins (§ 6), and how a token is held to it, `check` (absent, `every-token`) |
| `sampler` | the chain, in order, each sampler with its parameters: `greedy`, `penalties`, `top-k`, `top-p`, `min-p`, `temp`, `dist` |
| `seed` | the seed of a `dist` sampler |
| `max_tokens` | the budget of generated tokens, the thinking included |
| `logprobs` | `null`, or how many alternatives each generated token's log-probability is returned with |
| `keep` | `null`, or the name of a kept context the call continues (§ 7) |

`Sampler::greedy()` is greedy decoding; `Sampler::model_default()` is the model's card's chain: a presence penalty of
1.5, top-k 20, top-p 0.95, temperature 1, then a draw.

**A generation keeps the thinking apart from the answer**: with thinking on, what the model wrote up to its first
`</think>` is `thinking`, and what follows, white space at its start removed, is `answer`; a generation that never
closes its thinking is all thinking. Each token's log-probability is the model's, from its logits before the
grammar and the chain. **Reaching the budget is a stop, not a failure**: `end-of-turn`, `max-tokens` or
`context-full`.

## 4. The envelope

What every call returns, beside the generation, and every record keeps:

| part | the fields |
|---|---|
| `model` | `sha256`, the file's; from its header `architecture`, `size_label`, `quantization`, `trained_context`, `layers`, `vocabulary`; `parameters` as llama.cpp counts them |
| `runtime` | `binding` and `binding_version`; `llama_cpp`, the commit the binding built; `backend` (`CUDA0`, `CPU`) and `device`; `gpu_layers`, the layers on the device |
| `context` | `n_ctx`, `n_batch`, `n_ubatch`, `flash_attention`; the device's free memory before the model was loaded and once the context was made, and its total, in MiB |
| `template` | `sha256` of the template's text, the `renderer`, `thinking`, how many `tools` were declared; `null` for a text prompt |
| `prompt` | `text_sha256` of the rendered text, `tokens_sha256` of its token ids (one a line, in decimal), their count, and how many a kept context `reused` |
| `sampler` | the `chain` in order, and the `seed` |
| `grammar` | `sha256` of its text, its `root`, where it was asked to `start`, the generated token it `began_at`; `build_ms`, the wall time of its sampler's making; its `check`, and under `chosen-first` `chosen_again`, the tokens whose first choice it refused |
| `generation` | the generated `tokens`, the thinking's included and the end of turn not; the counts of `thinking` and `answer` tokens; why it `stop`ped; the wall time of the prompt's evaluation and of the generation, in milliseconds |

A record is JSON lines, a call a line, `einaf::model::Recorded`: the schema `einaf-generation/1`, the request's
`digest` (SHA-256 of the request as JSON), the request, and the generation or the failure.

## 5. The chat template

**The model's file carries its chat template as a Jinja text**, and llama.cpp's own C renderer knows a fixed list of
templates and no Jinja. So **EinAf renders the template's text itself**, `einaf::model::template::render`, with
`minijinja`, as Hugging Face's renderer and llama.cpp's server render a template: blocks trimmed, Python's string
methods, `raise_exception`, and `tojson` writing as Python's `json.dumps` does, keys in the order given. The
template reads `messages`, `tools` (absent where none is declared), `add_generation_prompt` and `enable_thinking`,
which is the request's `thinking`. The rendered text is tokenised with the special tokens parsed. A declared
function (`einaf::model::ToolDecl`, its parameters strings, every one required) is written as the chat API writes
it, `{"type": "function", "function": {…}}`; the model's call comes back in the answer as the model wrote it.

## 6. A grammar, and where it begins

A grammar is GBNF. **Under `every-token`** (a request that names no `check`) it is applied to every candidate before
the request's chain, so that a grammar that leaves no token is seen as such (§ 8); **under `chosen-first`** the chain
chooses first and its choice alone is offered to the grammar, which is applied to every candidate only where it
refuses that choice. Under greedy decoding the two choose the same tokens, and the second reads the vocabulary once
where the first reads it at every token ([the grammars § 5](06-grammars.md#5-how-a-token-is-held-to-a-grammar)).
**From the first token**, `{"at": "first"}`, it holds the whole output, **the thinking too**: with thinking on, a
grammar from the first token writes its text where the thinking goes, and the answer is empty. **Lazily**,
`{"at": "patterns", "patterns": [P…]}`, it begins where the generated text first matches a pattern whole, and is
fed the text from the pattern's first group on; llama.cpp feeds it the whole text where that group is empty, so
**the trigger is the grammar's own first text**: to begin after the thinking, the pattern is
`[\s\S]*?(</think>)[\s\S]*` and the grammar begins with `"</think>"`, the white space after it bounded, or greedy
decoding can write white space for ever. llama.cpp matches the whole text generated so far against each pattern at
every token, which a long thinking pays for. **Lazily at a token**, `{"at": "token", "text": T}`, it begins where the
model generates the one token `T` is (its special tokens parsed; a text of more tokens is the failure `engine`), and
is fed that token's text, with no pattern matched while it waits: `</think>` is one token of the model's. The
envelope's `grammar.began_at` is the first generated token the grammar
constrained. The grammars EinAf writes, and the grammar a request carries, are [the grammars](06-grammars.md)'.

## 7. A kept context

A request with `keep: K` continues the context kept under the name `K`: where the prompt begins with every token
that context holds (its last prompt and what was generated after it), only the rest is evaluated, and the
envelope's `prompt.reused` counts the tokens it did not evaluate again. A conversation that adds a turn to the last
one is such a prompt, a function's call and its response among it. Any other prompt is evaluated whole: a model
with recurrent layers keeps no state it can cut back to an earlier token. **The tokens a kept context holds were
evaluated one at a time as they were generated**, and the same prompt evaluated whole is evaluated in one batch;
the two can differ in the last bits of a logit, and greedy decoding can then part.

## 8. Failures

Each is a value, `einaf::model::Failure`, recorded as a call's result; there is no transport, so nothing is retried.

| failure | when |
|---|---|
| `model-load` | the model's file does not load: with the file's path and llama.cpp's word |
| `template` | the template does not render the messages: its `raise_exception`'s message, or a Jinja error |
| `prompt-too-long` | the prompt's tokens do not fit the context: both lengths |
| `grammar-parse` | the grammar's text does not parse from its root |
| `grammar-dead-end` | the grammar leaves no token at a generated token, its index: a bug of the grammar, never retried |
| `not-recorded` | replay holds no call for the request: its digest |
| `engine` | llama.cpp refused a step of the call |

## 9. Replay

**A record answers a request**: `einaf::model::replay::Replay` reads JSON lines, each a recorded call at the top of
the line or under its `recorded` key, and answers a request with the generation or the failure of the call whose
digest is the request's, field for field. So a run replayed asks the model nothing, and a record is the input of a
second run: `--replay FILE` in place of `--model PATH`, on every command that makes a call.

## 10. The commands

```sh
einaf render --template TEMPLATE REQUEST.json        # the prompt a request renders to, as text; no model
einaf generate --model PATH REQUEST.json...          # a call a request, in order, in one process: a record line each
einaf generate --replay RECORD.jsonl REQUEST.json    # the same, answered by a record
einaf grammar --json-schema SCHEMA.json              # the GBNF llama.cpp writes for a JSON schema (`llama`); the other `grammar` commands are [the grammars § 6](06-grammars.md#6-the-commands)'s
einaf model --model PATH                             # the model's pin, the runtime and the context, as JSON (`llama`)
einaf probe plan --out PLAN.jsonl                    # every run of the probe of Ein written with no grammar, its request whole
einaf probe run --plan PLAN.jsonl --out RECORD.jsonl --model PATH [--only PREFIX]...
einaf probe run --plan PLAN.jsonl --out AGAIN.jsonl --replay RECORD.jsonl
```

**The probe** (`einaf::probe`) asks the model for Ein with no grammar, under a few conditions of what the prompt
holds, and judges each answer through [the kernel](README.md#what-it-depends-on), read whole and by its first fenced
block: `einaf::kernel::parse` (every error with its kind and place), and for a text that parses the bounded call,
`--no-search` and `--single`, under a clock of 30 seconds, and its forms set against the program the item is
measured from. Its prompts are files of EinAf's, [`einaf/prompts/`](../../einaf/prompts/), each named with its
version: the excerpt of Ein every prompt holds, Ein in a paragraph, the word theory's vocabulary and the statement
of step 1 ([the contract § 3.1](04-contract.md#31-each-step)). `probe run` appends a line a run, and does not make a
run its output already holds. With `--replay` every call is answered from the record and every answer judged again.

## 11. Held by

[`einaf/tests/model.rs`](../../einaf/tests/model.rs), over calls the binding made and recorded under
[`einaf/tests/fixtures/model/`](../../einaf/tests/fixtures/model/): each recorded call replayed field for field, and
a request no call holds `not-recorded`; each envelope written and read back as itself, its template the record's
file and its prompt the one EinAf renders offline over the request; each failure of § 8 but `not-recorded` and
`engine`, replayed from its recorded cause; and a line of the probe's record, replayed and judged again through the
kernel, judged as it was. [`einaf/tests/prompts.rs`](../../einaf/tests/prompts.rs): every program the excerpt shows
loads, and every program the probe's plan quotes; and `einaf::kernel::parse` on a fenced answer, read whole two
errors at the fence's lines, each with its place and its repair, and by its block a program that parses. The template's rendering of the thinking switch, of a system
message beside a declared function and of a chat with no question, and the readings of an answer, are unit tests of
[`template.rs`](../../einaf/src/model/template.rs) and [`mod.rs`](../../einaf/src/model/mod.rs). That the binding
renders and tokenises a prompt as llama.cpp's server does, and generates the same tokens, is a measurement by hand,
not a test: the gate has no model.
