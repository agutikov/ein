# S2.2.2's probe: the model's runtime, and Ein written with no grammar

[S2.2.2](../s2.2.2_model_runtime.md)'s report. **The model runs in EinAf's process** through the binding
`llama-cpp-2` 0.1.161 (`einaf --features cuda`, [the runtime's page](../../../../docs/einaf/05-model.md)), every
call returning its envelope; the binding is held to `llama-server` of the host's package, it repeats itself, and its
speed and memory are taken. **Then the probe**: whether `qwen3.5:4b` writes Ein with no grammar, the user's first
experiment first. Every request and generation is kept whole, a JSON line each, with the kernel's reply; the
records are the files of this folder ([§ 9](#9-the-files)). Everything was run on 2026-11-26 on the machine the work
runs on, on AC power, the GPU free of any other model (ollama not running). The plan was committed (`1be58a36`)
before its first generation; the code every call ran is `35fcc5af`'s, built from the working tree before that
commit, which changed doc comments and tests after it.

**What it comes to.**

- **The runtime works, in process, on the GPU, and is held to llama.cpp's own server.** The first candidate binding,
  `llama-cpp-2` 0.1.161, loads `qwen35`, puts all 32 layers on the RTX 4060 and gives every control the ruling
  lists; no fallback. EinAf renders the model's Jinja template itself, and the prompt and its tokens are the
  server's on all fourteen requests compared; the generated tokens are the server's on eleven, the first
  experiment's 2 887 among them, and part on three at a near tie or a kept context. Ten calls of a request give one
  answer. 41 to 67 tokens a second; 3.2 GiB at 4 096 tokens of context, 5.1 at 65 536.
- **The user's first experiment**: thinking off, the model **copies `zebra3.1.ein` whole**, 113 forms of 113, and its
  program answers *one*, *"The Norwegian drinks water. The Japanese owns the zebra."* Thinking on, it decides to copy,
  then **loops** (a cycle of 163 tokens from its 6 154th) to the budget, and writes nothing.
- **The model alone** solves the puzzle in its thinking (by about its 5 900th token: water H1, the Norwegian; the
  zebra H5, the Japanese), goes on to *double check*, and **loops** from its 6 459th token to the budget of 65 000:
  **the decoding stopped it, not a limit**.
- **Thinking off, the model writes loadable Ein in its examples' shape**: over the library 13 of 14 programs parse
  and 8 load (the `youngest` text apart, L-M2.9); with the word theory given 13 of 13 and 11; step 1, 5 of 5 and
  5. With one paragraph on Ein and no example, 1 of 15 parses and none loads: **the excerpt and the examples are what
  makes it write Ein**, not Datalog or Lisp.
- **Thinking on, greedy decoding writes no program**: every one of the 36 runs with thinking on ends in a repetition
  loop that the budget cuts (Q-M2.27).
- **The errors are few, and half of them no grammar prevents.** Every answer is a fenced block, whose fence lines
  are its whole reading's only error in 34 of the 36 off-runs read whole outside the described condition
  (E-117). Inside the block: a keyword a form does not take (any grammar), `full-name` and `import-list` (the
  catalogue), and `unbound-conclusion`, a rule whose conclusion writes a variable nothing binds (nothing a grammar
  can do), three of the library condition's six refusals.
- **A program that loads is not a program that is right.** None of the nine library programs that load
  (`youngest`'s among them) prints the right answer, and the kernel's judgement of a step-1 program at root passes
  all six, faithful or not: what the kernel cannot say (an invented `:source`, a fact at another count, a number as a
  name, a statement for an answer) is the contract's check's.

## 1. The host probe again, through the binding

The eight requests of [`host.md`](../../analysis/host.md#what-the-model-does-through-llama-server-a-probe), which
`llama-server` answered, made again through the binding (`host_probe/01…08`), with a ninth, a grammar that begins
after the thinking, and a tool call's round trip on a kept context (`10a`, `10b`, `10c`). Greedy, seed 1, a context
of 65 536 tokens; `einaf generate --model M --ctx 65536 host_probe/NN_*.json`, the record
[`host_probe/record.jsonl`](host_probe/record.jsonl).

| request | what came back through the binding | `host.md`, through `llama-server` |
|---|---|---|
| 01 *Who is the youngest? Answer with one name.*, thinking off, 3 log-probabilities | `Cy`, 1 token; its log-probability −0.014 (the next, −4.73) | `Cy`, 2 tokens |
| 02 the same, thinking on | `Cy` after 268 thinking tokens, 270 in all, 4.0 s | `Cy` after 271 generated tokens, 4.1 s |
| 03 *Write three sentences about the sea.* with the grammar `(youngest Ann\|Bob\|Cy)`, off | `(youngest Bob)`: **the grammar is honoured against the prompt** | `(youngest Bob)` |
| 04 the question with that grammar, off | `(youngest Cy)` | `(youngest Cy)` |
| 05 the question with that grammar, **on** | the thinking `(youngest Cy)`, 13 characters, and **no answer: the grammar took the thinking** | an empty answer and 13 characters of reasoning |
| 06 the question with a JSON schema (its GBNF by llama.cpp's conversion, `einaf grammar --json-schema`) | `{"youngest": "Cy"}` | `{"youngest": "Cy"}` |
| 07 *Use the ein_solve tool on the program "(older Ann Bob)".*, `ein_solve(program)` declared | the call as the template asks for it, `<tool_call><function=ein_solve><parameter=program>(older Ann Bob)…`, and no text | a tool call, `ein_solve {"program":"(older Ann Bob)"}` |
| 08 the text prompt `(youngest `, a grammar, 3 log-probabilities | `Cy)`, each token's log-probability and its alternatives | `Cy)` |
| 09 the question, thinking on, the grammar **begun after the thinking** (trigger `</think>`) | 268 thinking tokens, then `(youngest Cy)`; `grammar.began_at` 268 | (not asked) |
| 10a, 10b the tool call, then its response on the **kept context** | the second turn's 365 prompt tokens, **329 not evaluated again** (7 ms against 30 ms whole) | (not asked) |

**Thinking is off where asked** (requests 01, 03, 04, 06, 07: no thinking token), **the log-probabilities come
back** (01, 04, 08), **the grammar is honoured against the prompt** (03), and **a grammar from the first token with
thinking on takes the thinking** (05), as through `llama-server`. **What the tool arm needs** is given: the template
rendered with a function declared, and the model's call in the answer as it wrote it (07). How a grammar begins
after the thinking took three tries ([L-M2.7](../findings/model/L-M2.7-a-lazy-grammar-is-fed-from-its-trigger-s-first-group.md)),
and the kept context's second turn is its own input to greedy decoding
([L-M2.8](../findings/model/L-M2.8-a-kept-context-and-a-whole-evaluation-of-one-prompt-part.md)).

## 2. The reference: the binding held to `llama-server`

`llama-server` of the host's package (`0.9.0-dev`, build 12110, commit `cd959bae99`) started on the same file, `-c
65536 -np 1 -ngl 99 --no-mmproj`, bound to `127.0.0.1:18731` and stopped after each use; each request sent once by
[`reference.py`](reference.py): the prompt EinAf renders (`einaf render`) against the server's `/apply-template`;
the server's tokens of it (`/tokenize`, special tokens parsed) against the binding's, by the envelope's digest; and
the server's completion of those tokens (`/completion`, temperature 0, the same seed and grammar, `return_tokens`)
against the binding's generated tokens. The record: [`reference/host_probe.jsonl`](reference/host_probe.jsonl) and
[`reference/probe.jsonl`](reference/probe.jsonl).

| request | prompt rendered alike | prompt tokens alike | generated: binding / server | alike |
|---|---|---|---|---|
| host 01 the youngest, off | yes | yes, 34 | 1 / 1 | **yes** |
| host 02 the youngest, on | yes | yes, 32 | 270 / 270 | **yes** |
| host 03 the sea, a grammar | yes | yes, 19 | 8 / 8 | **yes** |
| host 04 the youngest, a grammar, off | yes | yes, 34 | 6 / 6 | **yes** |
| host 05 the youngest, a grammar, on | yes | yes, 32 | 7 / 7 | **yes** |
| host 06 the youngest, a JSON schema's grammar | yes | yes, 34 | 11 / 11 | **yes** |
| host 07 a tool call | yes | yes, 299 | 30 / 30 | **yes** |
| host 08 a text prompt and a grammar | (a text) | yes, 5 | 2 / 2 | **yes** |
| host 09 a grammar begun after the thinking | yes | yes, 32 | 275 / 275 | **yes** (the server's lazy trigger, a full-match pattern) |
| host 10a the tool call, kept | yes | yes, 299 | 30 / 30 | **yes** |
| host 10b the tool's response, on the kept context | yes | yes, 365 | 23 / 21 | parts at token 13 |
| host 10c the same, evaluated whole | yes | yes, 365 | 12 / 21 | parts at token 7 |
| probe `first-experiment.off.zebra` | yes | yes, 6 156 | 2 887 / 2 887 | **yes**, every token |
| probe `library.off.youngest` | yes | yes, 5 251 | 364 / 364 | parts at token 339 |

**The rendered prompt and its tokens are the server's on all fourteen requests**: EinAf's rendering of the
template, thinking on and off, a system message, a declared function, an assistant's call and a tool's response,
is byte for byte the server's, and its tokens id for id. **The generated tokens are the server's on eleven of
fourteen**, the 2 887 of the first experiment among them. **Where they part, it is a near tie under another
llama.cpp**: at `library.off.youngest`'s token 339 the binding's two best candidates are `1358` (three newlines)
at −0.69768 and `271` (two) at −0.69793, 0.00026 apart (the call again with 5 log-probabilities a token,
[`reference/library_off_youngest_logprobs.jsonl`](reference/library_off_youngest_logprobs.jsonl)), and the server takes the second, a blank line fewer before
the same `;; The hypothesis mechanism` comment; the program's forms are the same. The two second turns of the tool
call part as [L-M2.8](../findings/model/L-M2.8-a-kept-context-and-a-whole-evaluation-of-one-prompt-part.md) says. The
binding's llama.cpp (`d8123504`) is 681 commits behind the server's (`cd959bae`): another CUDA kernel can move a
logit's last bits, which is no bug of the runtime and which no comparison of two builds can rule out. Repeatability
(§ 2.1) says each is deterministic on its own.

### 2.1 Repeatability

[`repeatability.sh`](repeatability.sh): three requests, each sent **N = 5 times in one process and five again in a
second** (the number fixed in the script before the run), greedy:

| request | calls | generated tokens | all ten alike |
|---|---:|---:|---|
| host 01, the youngest, thinking off, 3 log-probabilities | 10 | 1 | **yes** |
| host 02, the youngest, thinking on | 10 | 270 | **yes** |
| probe `library.off.youngest`, a whole program | 10 | 364 | **yes** |

**The same request gives the same tokens**, within a process and across two. The record:
[`repeatability/record/`](repeatability/record/), a file a request and a process.

### 2.2 Throughput and memory

[`throughput.sh`](throughput.sh), through `utils/bench_env.sh --cores P --require-ac`, which printed (2026-11-26
06:59, `1be58a36`): *i9-14900HX, pinned to the 8 physical P-cores, governor `powersave` (epp
`balance_performance`), turbo on, AC (ACAD online=1), battery full, loadavg 2.06*. One process, the whole model on
the GPU, a context of 65 536 tokens; a text prompt (the excerpt of Ein repeated) evaluated whole, then 256 greedy
tokens:

| prompt tokens | its evaluation | prompt tokens a second | 256 tokens generated in | generated tokens a second |
|---:|---:|---:|---:|---:|
| 1 211 | 0.49 s | 2 498 | 3.8 s | **66.9** |
| 5 498 | 2.1 s | 2 639 | 4.2 s | **61.2** |
| 13 745 | 5.7 s | 2 416 | 4.6 s | **56.1** |
| 30 239 | 14.2 s | 2 126 | 5.0 s | **51.4** |
| 54 980 | 29.5 s | 1 865 | 6.2 s | **41.0** |

**The device's memory** for the model and its context, by `einaf model --ctx N` (the free memory before the model
is loaded, 7 685 MiB of 7 857, less the free memory once its context is made; ollama was not running):

| context | the model and its context |
|---:|---:|
| 4 096 | 3 206 MiB |
| 16 384 | 3 608 MiB |
| 32 768 | 4 130 MiB |
| 65 536 | 5 186 MiB |
| 131 072 | 7 298 MiB |

The probe's prompts are 0.2 to 6.2 thousand tokens, and a thinking run's context grows to 61 thousand: the
probe's one context of 65 536 tokens holds every run, at 41 to 67 tokens a second. 131 072 tokens fit, with 387 MiB
left. `llama-server`, by `host.md`, projected 2 771 MiB at 4 096 tokens; the binding's 3 206 count the CUDA runtime's
own memory too, which the server's projection does not. The records: [`throughput/`](throughput/).

## 3. The first experiment: the Zebra puzzle with a working Zebra program as its example

The user's: *"first experiment would be with original zebra problem and with working zerba program as example - test
for dummies"*. The prompt: the excerpt of Ein; the Zebra text beside [`examples/zebra3.1.ein`](../../../../examples/zebra3.1.ein)
whole, the one example; then the same text, and the task (a whole program, in one fenced block). 6 156 prompt tokens
off, 6 154 on. **The item is shown its own program: it is the floor of the probe, and no number of it enters a
table.** The record: the first two lines of [`probe.jsonl.gz`](probe.jsonl.gz).

| | thinking off | thinking on |
|---|---|---|
| generated | 2 887 tokens, all answer, end of turn, 46 s | **55 000 tokens, all thinking, the budget**, 1 112 s |
| what the model wrote | one fenced block, **`zebra3.1.ein` copied whole**: its 113 forms, each the example's (canonically), its comments too | no answer: the thinking never closed |
| read whole | a parse error at the fence's two lines, and nothing else: *unexpected* `` ```ein ``*: a name*, with the repair *Delete* `` ```ein `` (1:1), and the same of `` ``` `` (213:1) | none |
| read by its fenced block | parses, loads; `--no-search` *not decided* (root alive); **`--single` *one*, `Solution`** | none |
| how far from the example | **0 forms changed, 0 missing, 0 added** (113 of 113 the same) | — |
| the sentence it printed | ***"The Norwegian drinks water. The Japanese owns the zebra."*** | — |

**Thinking off, the model copies.** It reproduces 10 kB of a program it was given, byte for byte in its forms, and
the kernel answers it as it answers the example: the floor holds, and says the model can carry Ein's syntax over
three thousand tokens without a slip. It shows nothing of formalization: there was nothing to change.

**Thinking on, greedy decoding loops.** The thinking settles the task early (*"They are identical. So the program I
write should be the same as the one provided in Example 1. I will copy the program from the example"*, about its
2 500th token), notices that the excerpt's table of forms does not name `config`, which the example writes, and
keeps it; then checks the example item by item (*"Wait, I should check if I need to add `Ukrainian`. The example has
it. I will include it."*, 1 874 such lines), and from its 6 154th token repeats one cycle of 163 tokens, six lines
over `Second`, `First`, `Middle`, `Second`, `Fourth`, `Fifth`, to the budget. **The budget stopped a loop**, not a
thought: by 6 154 tokens there was nothing more the model would write, so no budget would have given an answer.

## 4. The model alone on the Zebra text

The other side of the minimal goal ([Q-M2.17](../../open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md)):
the Zebra text, its seventeen lines and nothing else, as the one user message, thinking on, greedy, through the
binding. **The context is 65 536 tokens and the budget 65 000 generated tokens**, against a prompt of 178: the
largest the probe's one context holds, eleven times the 5 900 or so tokens the model took to solve the puzzle in
its thinking. The record: [`probe.jsonl.gz`](probe.jsonl.gz), `model-alone.on.zebra` (and `model-alone.off.zebra`).

| | thinking on | thinking off (a companion run) |
|---|---|---|
| generated | **65 000 tokens, all thinking, the budget**, 1 305 s | 8 192 tokens, all answer, the budget, 128 s |
| what it wrote | a solution in its thinking: by about its 5 900th token (the 19 624th character of 157 613), *"15. Who drinks water? (H1). 16. Who owns the zebra? (H5)"*, every clue checked against a grid whose first house is the Norwegian's and fifth the Japanese's: **the right answer**; then *"Wait, I need to double check the 'Green right of Ivory' logic"*, a second branch (ivory third, green fourth), and from its 6 459th token **one cycle of 85 tokens**, five lines of that branch's grid, repeated 688 times to the budget | a solution as prose: the colours, the drinks (*"**Norwegian drinks Water**"*), then paths re-checked one after another (*"Is it possible …?"*, twenty of them) to the budget, no pet placed, no final answer |
| what stopped it | **a greedy repetition loop, which the budget ended**: nothing was left for the model to write after its 6 459th token, and a larger budget would have lengthened the loop | the budget, in a re-checking that does not repeat exactly |
| the answer it gave | none: the thinking never closed | none |

**What stopped it is the decoding, not a limit.** The user's run through ollama stopped short of a solution at a
limit nobody chose; this run had room for eleven solutions and found one, in its thinking, and then lost it in
a loop greedy decoding cannot leave. The same happened to the first experiment with thinking on (§ 3), and is the
question of [Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md).

## 5. The probe: Ein written with no grammar

87 runs, [the plan](#8-the-plan) fixed before the first; every run greedy, through the binding, in a context of
65 536 tokens; each answer read **whole** and by **its first fenced block**, parsed by `einaf::kernel::parse` and,
where it parses, loaded and asked `--no-search` and `--single` through the bounded call under a 30-second clock. The
record is [`probe.jsonl.gz`](probe.jsonl.gz); `python3 report.py probe.jsonl.gz` prints every table here and a line
a run. **None of these numbers is an accuracy**: the items are few, near one another, and the material prompts are
developed on.

### 5.1 The numbers, condition by condition

The `youngest` runs are set apart: the excerpt every prompt holds shows that text's program
([L-M2.9](../findings/model/L-M2.9-the-excerpt-shows-the-ordering-family-s-program.md)).

| condition | thinking | runs | stopped by the budget | whole: parses | fenced: a block | parses | loads | `--single`: one / several / inconsistent / not decided |
|---|---|---:|---:|---:|---:|---:|---:|---|
| the first experiment | off | 1 | 0 | 0 | 1 | 1 | 1 | 1 / 0 / 0 / 0 |
| the first experiment | on | 1 | 1 | 0 | 0 | 0 | 0 | 0 / 0 / 0 / 0 |
| the model alone | off | 1 | 1 | 0 | 0 | 0 | 0 | 0 / 0 / 0 / 0 |
| the model alone | on | 1 | 1 | 0 | 0 | 0 | 0 | 0 / 0 / 0 / 0 |
| **a whole program over the library** | off | 14 | 0 | 0 | 14 | 13 | 8 | 6 / 0 / 1 / 1 |
| **a whole program over the library** | on | 14 | 14 | 0 | 0 | 0 | 0 | 0 / 0 / 0 / 0 |
| a theory given | off | 13 | 0 | 0 | 13 | 13 | 11 | 10 / 0 / 1 / 0 |
| a theory given | on | 13 | 13 | 0 | 0 | 0 | 0 | 0 / 0 / 0 / 0 |
| described | off | 15 | 6 | 0 | 15 | 1 | 0 | 0 / 0 / 0 / 0 |
| step 1 alone | off | 5 | 0 | 0 | 5 | 5 | 5 | 0 / 5 / 0 / 0 |
| step 1 alone | on | 5 | 5 | 0 | 0 | 0 | 0 | 0 / 0 / 0 / 0 |
| *`youngest`, over the library (L-M2.9)* | off | 1 | 0 | 0 | 1 | 1 | 1 | 1 / 0 / 0 / 0 |
| *`youngest`, over the library (L-M2.9)* | on | 1 | 1 | 0 | 0 | 0 | 0 | 0 / 0 / 0 / 0 |
| *`youngest`, step 1 (L-M2.9)* | off | 1 | 0 | 0 | 1 | 1 | 1 | 0 / 1 / 0 / 0 |
| *`youngest`, step 1 (L-M2.9)* | on | 1 | 1 | 0 | 0 | 0 | 0 | 0 / 0 / 0 / 0 |

*Whole* parses in no run: every answer of every condition with thinking off is a fenced block, and in 34 of the 36
off-runs outside the described condition read whole the fence's two lines are the only error
([E-117](../evaluation/diagnostics/E-117-a-fence-around-a-program-is-the-parse-s-only-error-with-its-repair.md));
the two others are the model alone's prose and the library's `pencils`. *Stopped by the budget* is every run with
thinking on, the model alone's with thinking off, and six of the described condition's runs, whose answers repeat a
form to the budget. **The bound
stopped one run** (`library.off.bob1`, *not decided*, § 5.4).

### 5.2 What the model wrote, condition by condition

**Thinking off, the model writes Ein in the shape of its examples, and the kernel takes most of it.** Every answer
is one fenced block (so the whole reading fails, at the fence's two lines, [E-117](../evaluation/diagnostics/E-117-a-fence-around-a-program-is-the-parse-s-only-error-with-its-repair.md)),
and its block copies the examples' layout to their comments' rules (*"──── The statements ───"*), their section
order, their `(solve :no-search)` and their rules' names, `two-counts-ask-the-difference` written into a program
about a giving. **Over the library** (the main arms' condition), 13 of 14 blocks parse and 8 load, the `youngest`
text apart ([L-M2.9](../findings/model/L-M2.9-the-excerpt-shows-the-ordering-family-s-program.md); with it 14 of 15
and 9); **with the word theory given**, 13 of 13 parse and 11 load, the theory doing the work the model would otherwise write.

**The errors are few and of four kinds**, each refused with a line that says what to do:

| kind | where | what the model wrote | what would have prevented it |
|---|---|---|---|
| `parse`: a keyword the form does not take | library `pencils` | `(relation pencils-office Number :source "250 pencils go to the office.")`, a `:source` on a declaration; the repair *Insert `:why`, Delete `:source`* | any grammar |
| `unbound-conclusion` | library `gap`, `bystander`, `boxes12` | a rule whose `:assert` writes a variable its `:match` never binds (`?giver`, `?x` and `?total`) | nothing a grammar can do |
| `full-name` | library `gap_fact`, `kinds_full` | `(relation sum-of …)` declared beside `(import std.math.decimal-arith :symbols (… sum-of))`: one short name bound to two full names | the catalogue (the library's names, and that a program does not declare them), or a grammar that knows the program's names |
| `import-list` | theory given `boxes12`, `boxes13` | `(import words :symbols (… readings))`: `readings` is a module of its own, not a name of `words` | the catalogue |

**Thinking on, the model writes no program.** All 28 thinking-on runs of the library and theory-given conditions,
and the six of step 1, end at the budget of 16 384 tokens, all thinking, no answer. Each but one ends in an exact
cycle of 11 to 911 tokens begun between its 249th and its 8 901st generated token and repeated to the budget
(`report.py`'s table, the column *loop*): *"I need to declare `has` and `gave` relations with `:why`."* line
after line, *"One detail: `Person` is a label. I will use `Person` as a label."*; the one, `library.on.boxes12`,
repeats a block of five lines whose heading it numbers, *"165. **One more detail**"*. So the thinking-on conditions
measure where greedy decoding loops, and no rate of parsing or loading; the question is
[Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md).

**What loaded in silence and should not count as right.** Of the nine library programs that load, `youngest`'s
among them, seven are *one*, and **none prints the right answer**: three print a stated count as the answer (`contra`, `unused`, `before`: *"(n z
5) apples"*, the query reading the stated `has` at any moment), one an invented one (`seating`: three sentences the
text does not have, quoted as `:source`, and *"Ann sits in the first chair."*), three have no row (`youngest`, its
rule copied from the excerpt with no membership it reads; `kinds`, `boxes13`), one is `inconsistent` for a numeral
(`more`, [E-118](../evaluation/stdlib/E-118-a-numeral-written-with-two-digits-in-one-place-is-refuted-as-a-contradiction.md)),
and one never ends (`bob1`). The kernel's word is right each time; what a program says of its text is the
contract's check's to read: a `:source` that is no sentence of the text (four of the nine: `seating` three
sentences the text does not have; `unused`, `kinds` and `boxes13` a clause where one sentence states two facts, or
a sentence rephrased, as the corpus's own word problems quote, [Q-M2.25](../open_questions/answer/Q-M2.25-is-a-clause-of-a-sentence-a-source-or-only-the-whole.md)),
a fact beside its own negation with no theory of contradiction
([E-18](../../evaluation/stdlib/E-18-a-fact-beside-its-negation-is-a-contradiction-only-where-the.md), `seating`), a
relation concluded and never declared ([E-12](../../evaluation/language/E-12-an-undeclared-relation-and-a-form-of-a-declared-relation-at.md),
`bob1`, `more`), the answer query reading a statement.

### 5.3 Datalog and Lisp alone: the described condition

With one paragraph on Ein in place of the excerpt and no example, **one of fifteen answers parses and none loads**.
The model writes a Datalog of its own in Lisp's brackets: a rule's head as a pattern, `(rule (older ?x ?y) () :match
… )`; a fact wrapped in a `fact` form, `(fact (older Ann Bob) :source "…")`; clauses without their keywords. The
parse refuses it at the first such form, a list or a variable where the form takes a name or a keyword (fourteen of
the fifteen answers), with repairs that are no help (*Delete `(`, Shift `older`, Delete `?x` …*); the one that
parses repeats a rule's name (`declaration`, *duplicate rule name 'total-apples'*, no place). Six run to the budget writing the same
rule again and again. **What the model knows of Datalog and Lisp does not make Ein**: the excerpt and the examples do.

### 5.4 The kinds of error, and what would have prevented each

In the readings by fenced block (the whole reading adds the fence lines everywhere):

| condition, thinking off | kind | runs | what was refused | what would have prevented it |
|---|---|---:|---|---|
| library | `parse` | 1 | a keyword the form does not take (`:source` on a `relation`) | any grammar |
| library | `unbound-conclusion` | 3 | a conclusion's variable no premise binds | nothing a grammar can do |
| library | `full-name` | 2 | a name the program declares and also lists from the calculator (`sum-of`) | the catalogue; or a grammar that knows the program's names |
| theory given | `import-list` | 2 | a module's name listed as a name of another (`readings` from `words`) | the catalogue |
| described | `parse` | 14 | a list or a variable where the form takes a name or a keyword (14), a keyword the form does not take (4), a parenthesis, a string, `_`, a bare `and` (one each) | any grammar |
| described | `declaration` | 1 | a rule's name twice | a grammar that knows the program's names (the probe's table of kinds says *any grammar*: the kind holds a rule's shape and its name alike) |

**One grammar of Ein would have removed** the fence lines, the keyword out of place and every refusal of the
described condition: what a grammar is for is the syntax the model does not know, and with the excerpt and examples
it knows most of it. **A grammar that knows the program's names** would add `full-name` and the duplicate. **The
catalogue** answers `full-name` and `import-list`, both errors of what the library holds. **Nothing a grammar can
do** prevents `unbound-conclusion`, three of the library condition's six refusals: a rule whose conclusion is not
bound by its match is well formed. And none of these is what made the loaded programs wrong (§ 5.2).

**The bound.** `library.off.bob1` concludes `(has ?giver (sub ?have ?given) ?kind ?time)` from `(has ?giver ?have
?kind ?time)`, a fact one deeper than the one it matched, which has no fixpoint: the bounded call returned it *not
decided* at its clock, under both questions, as it should
([E-45](../../evaluation/bounds/E-45-nothing-in-a-solve-stops-a-saturation-that-has-no-fixpoint.md)); it took the
call 94 seconds, the clock once for the load's judgement and once a question
([L-M2.10](../findings/einaf/L-M2.10-a-program-with-no-fixpoint-costs-the-bounded-call-its-clock-once-a-saturation.md)).

## 6. Step 1 with the model

*"First step is simple"* (the user's notes): step 1 alone, the facts and a partial ontology, asked of the model on
the Zebra text and on the five other texts of the contract's steps by hand, with the excerpt, the statement of step 1
([`einaf/prompts/step1.v1.md`](../../../../einaf/prompts/step1.v1.md), the contract's § 3.1 in a page) and one worked
step 1 of another text (`youngest`'s, and `seating`'s for `youngest`), thinking off and on. Each reply is judged at root
(`--no-search`) and `--single`, as the steps' second design question has it, and set against the hand-written step 1
of [`einaf/contract/`](../../../../einaf/contract/) by [`step1.py`](step1.py): per sentence, the facts that quote it;
the names; the memberships. **With thinking on, none of the six writes a program** (each a loop to the budget,
§ 5); with thinking off:

| text | the reply | root, `--single` | the hand's step 1 | its facts against the hand's |
|---|---|---|---|---|
| `zebra` | 15 statements, **no `:source`**, a description written as the colour (`red`, as the hand's `Red`), but *Coffee is drunk in the green house* made active, `(drinks Coffee green)`; *The Old Gold smoker* and *Lucky Strike* two names each, `(owns Old Gold snails)`, a fact of three places beside a declaration of two ([E-12](../../evaluation/language/E-12-an-undeclared-relation-and-a-form-of-a-declared-relation-at.md)); *There are five houses* as `(five (count 5))`; the two questions two queries; kinds as memberships of an undeclared `instance` (`(instance Old Gold smoker)`) | loads; root alive (*not decided*); *several* | 55 forms, H blind of 9 896 candidates, not asked | **0** of the hand's 44 facts; 9 names of the hand's 36 (the nationalities, `Coffee`, `Milk`, `Kools`, `Chesterfields`), the rest lower case or split |
| `youngest` | the two statements word for word with their sentences, the answer query as the hand's, `older` and `youngest` declared; no membership | loads; alive; *several* (blind) | 9 forms, *several* | **2 of 2 statements**, the 3 memberships missing; the same three names |
| `seating` | the two negations; *Bob does not sit in the middle chair* as `(not (sits-in Bob Second))`, the middle chair named by its place; the people as `(person …)`, the chairs as no kind; and **three placements no sentence states**, `(sits-in Ann First)`, `(sits-in Bob Second)`, `(sits-in Cy Third)`, quoting *"Ann sits next to Bob."*, the sentence itself not stated | loads; alive; *several* | 13 forms, *several* | 1 of 3 statements (*Cy does not sit in the first chair*); *sits next to* lost; a guess written as statements |
| `apples` | `(has Bob apples)` and `(gave Bob Alice apples)`, **the numbers out of the facts**, as memberships `(instance 5 Number)`; no moment; the query over an `amount` no fact has | loads; alive; *several* | 5 forms, *one* | 0 of 2; *He* is Bob, right |
| `together` | the same two facts, no number at all; the query `(total-apples ?who ?amount)` | loads; alive; *several* | 6 forms, *one* | 0 of 2 |
| `more` | `(has Bob apples 5)`, `(has Alice apples 12)`, **12 a name**, beside `(relation has Person Number …)` of two places; the query asks the calculator's `diff-of` by name, unimported | loads; alive; *several* | 5 forms, *one* | 0 of 2 |

**Step 1 is not simple for this model, and not for the reason the contract foresaw.** Every reply loads and its root
is alive, so the kernel's judgement of a step-1 program (`--no-search`: it loads, root is alive) passes six of six:
**the judgement at root does not tell a faithful step 1 from an unfaithful one.** The coreferences are mostly right
where the text gives them (*He* is Bob; the middle chair is the second); what goes wrong is the facts' shape: a
number dropped from its fact or written as a name, a name of two words split, a passive made active, a sentence's
content replaced by a guess. Six relation declarations, three of `apples` and three of `more`, read arrows they do
not have ([E-119](../evaluation/natural-language/E-119-a-relation-s-why-that-names-an-arrow-its-declaration-lacks-loads.md)).
Only `youngest`, whose program the excerpt shows ([L-M2.9](../findings/model/L-M2.9-the-excerpt-shows-the-ordering-family-s-program.md)),
states its sentences as the hand does. The contract's check (the step-1 findings of [§ 1.7 of the
contract](../../../../docs/einaf/04-contract.md#17-what-a-formalization-comes-to): a `:source` that is no sentence,
a fact of a relation at another count, a number written as a name) is what would have said each of these; the kernel
says none.

## 7. What it found

**Of Ein** ([the phase's evaluation](../evaluation/README.md)), each entry with the generations it rests on:

- [E-117](../evaluation/diagnostics/E-117-a-fence-around-a-program-is-the-parse-s-only-error-with-its-repair.md)
  (strength): a fence around a program is the parse's only error, each fence line named with the repair that
  deletes it; `first-experiment.off.zebra`, and 34 of 36 off-runs read whole.
- [E-118](../evaluation/stdlib/E-118-a-numeral-written-with-two-digits-in-one-place-is-refuted-as-a-contradiction.md)
  (weakness): `(n z 12)` makes root dead, its core exact, and the outcome reads as a contradiction;
  `library.off.more`.
- [E-119](../evaluation/natural-language/E-119-a-relation-s-why-that-names-an-arrow-its-declaration-lacks-loads.md)
  (weakness): a relation's `:why` that names an arrow its declaration lacks loads and prints as text;
  `step1.off.apples`, `step1.off.more`.
- **Confirmed**, a line each: [E-2](../../evaluation/answer/E-2-a-program-states-its-answer-and-the-kernel-checks-it.md)
  (the model's own `:expect`, refuted by the kernel in six of seven); [E-12](../../evaluation/language/E-12-an-undeclared-relation-and-a-form-of-a-declared-relation-at.md)
  (`has-more` never declared; `(owns Old Gold snails)` at another count); [E-18](../../evaluation/stdlib/E-18-a-fact-beside-its-negation-is-a-contradiction-only-where-the.md)
  (`seating`'s fact beside its negation, *one*); [E-45](../../evaluation/bounds/E-45-nothing-in-a-solve-stops-a-saturation-that-has-no-fixpoint.md)'s
  closing (`bob1`'s rule with no fixpoint, *not decided* at the clock); [E-73](../../evaluation/diagnostics/E-73-a-refusal-about-a-top-level-form-has-no-place.md)
  (`unbound-conclusion` has no place).
- **Which refusals a model's programs meet, and whether the kind and place tell them apart**: with thinking off,
  `parse` (a fence line, a keyword a form does not take, and in the described condition the shapes of Datalog and
  Lisp), `unbound-conclusion`, `full-name`, `import-list` and one `declaration`; each kind names a different fault,
  and `parse` and `full-name` have their line and column, `unbound-conclusion`, `import-list`
  and `declaration` none (`import-list`'s line ends *at None None*). **How the model wrote a number**:
  a numeral where an example showed one (`(n z 5)`), `(n z 12)` once, `12` as a name in step 1, and in step 1 the
  number left out of its fact; **a `:why`**: on every relation and rule, in the examples' words, `{?5}` past the
  labels twice; **an import**: the calculator's three lines copied from the examples, in 26 programs,
  and `std.slots` and `std.algebra` once each where nothing selects them (`library.off.unused`,
  `library.off.boxes12`); the seating, the one text the library's slots fit, imports nothing.

**Of EinAf, the plan and the process** ([the phase's findings](../findings/README.md)):
[L-M2.7](../findings/model/L-M2.7-a-lazy-grammar-is-fed-from-its-trigger-s-first-group.md) (a lazy grammar's trigger
is its own first text), [L-M2.8](../findings/model/L-M2.8-a-kept-context-and-a-whole-evaluation-of-one-prompt-part.md)
(a kept context and a whole evaluation part under greedy decoding),
[L-M2.9](../findings/model/L-M2.9-the-excerpt-shows-the-ordering-family-s-program.md) (the excerpt shows the ordering
family's program), [L-M2.10](../findings/einaf/L-M2.10-a-program-with-no-fixpoint-costs-the-bounded-call-its-clock-once-a-saturation.md)
(a runaway program costs the bounded call its clock once a saturation).

**For the user** ([the phase's open questions](../open_questions/README.md)):
[Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md),
whether the phase's thinking-on runs stay greedy; default: yes, every loop recorded.

**And outside both**: the user's editor's `rust-analyzer`, watching `einaf/Cargo.toml`, fetched the template's crates
(`minijinja`, `minijinja-contrib` and its dependency `memo-map`) into `~/.cargo/registry` when the manifest gained
them, though every cargo command of the stage ran with its own `CARGO_HOME` in the scratchpad.

## 8. The plan

Fixed before the first generation and committed so (`1be58a36`); the prompts' files are under
[`einaf/prompts/`](../../../../einaf/prompts/), and every request is whole in [`plan.jsonl`](plan.jsonl).

### The conditions, in the order they run

| condition | thinking | runs | the prompt | the budget |
|---|---|---:|---|---|
| **the first experiment** (the user's: *"original zebra problem and with working zerba program as example - test for dummies"*) | off, on | 2 | the excerpt; the Zebra text beside `examples/zebra3.1.ein`, the one example; the Zebra text | 16 384; 55 000 |
| **the model alone** | on, off | 2 | the Zebra text, and nothing else | 65 000; 8 192 |
| **a whole program over the library** (the main arms' condition) | off, on | 15 each | the excerpt; the examples of the library condition but the item's family; the text | 8 192; 16 384 |
| **a theory given** (the library ablation's widest; no arm's) | off, on | 13 each | the excerpt and the word theory's vocabulary; the corpus's word problems but the item's family | 8 192; 16 384 |
| **described** (Datalog and Lisp alone) | off | 15 | one paragraph on Ein, no example; the text | 8 192 |
| **step 1 alone** | off, on | 6 each | the excerpt and the statement of step 1; one worked step 1 of another text | 8 192; 16 384 |

Every run is greedy (one sampler, `greedy`, seed 1), in a context of 65 536 tokens, through the binding.

### The items and their families

The families are fixed here, before the run; **no item is shown an example of its own family**, the first
experiment aside, which is shown its own program by the user's word and is the floor of the probe, in no table.

| family | what | items |
|---|---|---|
| zebra | an attribute puzzle of five houses | `zebra` (the first experiment, the model alone, step 1) |
| ordering | an order among people | `youngest` (the contract's worked example) |
| seating | people in a row of chairs | `seating` |
| transfer | a giving between people over time | `bob1`, `gap` (and `gap_assume`), `gap_fact`, `contra`, `unused`, `bystander`, `before`; step 1's `apples`, `together` |
| kinds | two kinds of thing | `kinds` (and `kinds_assume`, `kinds_listed`), `kinds_full` |
| comparison | who has more, and by how many | `more` |
| boxes | how many containers are needed | `boxes12`, `boxes13` |
| sharing | a total shared equally | `pencils` |

**Sixteen word problems, thirteen texts**: `gap_assume` has `gap`'s text, and `kinds_assume` and `kinds_listed`
have `kinds`'. A greedy run of one prompt is one output, so a text is asked once and stands for each problem that
has it.

**The examples**, each a text beside its program: under the library condition the four programs of the
formalization contract's worked steps written over the library alone (`einaf/contract/{youngest,apples,more,seating}/step3.ein`,
families ordering, transfer, comparison, seating); under a theory given, five of the corpus's word problems, one a
family (`bob1`, `kinds_full`, `more`, `boxes12`, `pencils`); for step 1, `einaf/contract/youngest/step1.ein`, and
`seating/step1.ein` for the `youngest` text. **Every program a prompt quotes**: `examples/zebra3.1.ein`, the five
word problems above, and the six contract programs; each is out of every split of the benchmark.

### The readings, and what is measured

An answer is read **whole** (the answer is the program) and **fenced** (its first fenced block). Each reading is
parsed by `einaf::kernel::parse` (every error with its kind and place); a reading that parses is loaded and asked
through the bounded call, `--no-search` and `--single`, each under a clock of 30 seconds; a reading that loads has
its forms read and set against the program it is measured from (`examples/zebra3.1.ein` for the first
experiment, the hand-written step 1 for step 1, the reference program for the small texts and the word problems
under a theory given). Each kind of error is read with what would have prevented it: any grammar, a grammar that
knows the program's names, the catalogue, or nothing a grammar can do.

## 9. The files

| file | what |
|---|---|
| [`plan.jsonl`](plan.jsonl) | the probe's 87 runs, each with its request whole, written by `einaf probe plan` and committed before the first generation |
| [`probe.jsonl.gz`](probe.jsonl.gz) | **the probe's record**: a line a run (`einaf-probe/1`, `einaf::probe::Line`): the plan's entry, the call (request, digest, generation with its thinking, answer, every token and the envelope), and the kernel's judgement of each reading (the parse's findings, the bounded call's objects, the forms and their distance); gzip, 742 kB of a plain file of 10.1 MB |
| [`host_probe/`](host_probe/) | the host probe's requests (`01` to `10c`, `06_schema.json` and its GBNF), their record (`record.jsonl`) and the two attempts at a grammar after the thinking that failed (`lazy_attempts.jsonl`) |
| [`reference/`](reference/) | the comparison with `llama-server`, a JSON line a request: `host_probe.jsonl`, `probe.jsonl`; and `library_off_youngest_logprobs.jsonl`, the call that parts at the near tie, again with its log-probabilities |
| [`repeatability/`](repeatability/) | the third request's file, the ten calls of each request (`record/`, gzip) and the table (`result.md`) |
| [`throughput/`](throughput/) | `einaf model` at five contexts, the speed record (its prompts' text left out), `bench_env.sh`'s fingerprint, the tables |
| [`failures/`](failures/) | the request of each typed failure and their record (`record.jsonl`), as `einaf/tests/fixtures/model/calls.jsonl` holds them |
| [`reference.py`](reference.py), [`repeatability.sh`](repeatability.sh), [`throughput.sh`](throughput.sh), [`failures.sh`](failures.sh) | the measurements, each one command |
| [`report.py`](report.py), [`step1.py`](step1.py), [`extract.py`](extract.py) | the tables of § 5 and § 6 from the record, and runs taken out of it for the reference |

## 10. How each part is taken again

From the repository's root, with the model's file in `M` (its SHA-256 is checked by the envelope, not by the path):

```sh
CMAKE_CUDA_ARCHITECTURES=89 cargo build --release --features cuda --manifest-path einaf/Cargo.toml
D=plans/m2_nl_to_ir/p2.2_formalizer/model_runtime
einaf/target/release/einaf model --model $M                                   # the pin, the runtime, the context
einaf/target/release/einaf generate --model $M $D/host_probe/01_youngest_off.json   # one generation, its record line
einaf/target/release/einaf probe plan --out plan.jsonl                        # the plan, byte for byte $D/plan.jsonl
einaf/target/release/einaf probe run --plan $D/plan.jsonl --out probe.jsonl --model $M --only first-experiment
einaf/target/release/einaf probe run --plan $D/plan.jsonl --out probe.jsonl --model $M   # the rest; a run done is skipped
zcat $D/probe.jsonl.gz > record.jsonl
einaf/target/release/einaf probe run --plan $D/plan.jsonl --out again.jsonl --replay record.jsonl   # no model: replayed, judged again
python3 $D/report.py $D/probe.jsonl.gz                                         # the tables of § 5
python3 $D/step1.py $D/probe.jsonl.gz                                          # § 6, step 1 against the hand's
$D/repeatability.sh $M OUT; $D/throughput.sh $M OUT; $D/failures.sh $M failures.jsonl
python3 $D/reference.py --server http://127.0.0.1:18731 --template einaf/tests/fixtures/model/chat_template.jinja \
  --einaf einaf/target/release/einaf --record $D/host_probe/record.jsonl $D/host_probe/{01,02,…}.json   # llama-server running
```
