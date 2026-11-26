# The grammars narrowed: what a model writes under leaves out the chat template's markers and the fence

[S2.2.3](../../s2.2.3_gbnf.md#amended-on-the-users-word)'s grammars, amended on the user's word, and the records of
the amendment. **The grammars a model writes under**, the whole grammar's root `root` and every step's grammar, **no
longer let it write `<`, `>`, `|` or a backtick but inside a sentence of the input**, nor `/` in a name: a name or a
variable of letters, digits and `_ . * + = -`, no block comment, and none of the four in a string or a comment. The
root `program` is Ein's language exactly, as before. The code is `23d582aa`
([the grammars' page](../../../../../docs/einaf/06-grammars.md#1-the-whole-grammar)). Everything here was run on
2026-11-26 on the machine the work runs on, on AC power, the GPU free of any other model, with `23d582aa`'s binary.

## 1. The sizes

[`sizes.md`](sizes.md), `python3 sizes.py`: before is `98e6e0a3`'s file and S2.2.3's speed requests.

| grammar | before: bytes, rules | after: bytes, rules |
|---|---|---|
| the file `einaf/grammars/ein.gbnf`, both roots | 50 131, 531 | 87 649, 910 |
| **what a request for a whole program carries** (`einaf grammar render whole`) | 50 131, 531 (the file) | **38 289, 383** (`root` and the rules it reaches) |
| the rules `root` reaches (their bytes, comments aside) | 38 719, 454 | 34 249, 383 |
| `root`'s name tree | 175, 68 of them the writeback names' (after `<`) | 107 |
| the rules `program` reaches | 42 793, 528 | 42 793, 528 |
| the Zebra puzzle's step 1, 2, 2b, 3, fenced | 14 980, 151; 55 609, 173; 19 401, 132; 23 833, 137 | 15 024, 151; 55 653, 173; 19 413, 132; 23 845, 137 |

**The file grew and what the model is given shrank**: `root`'s rules are `ein.y`'s compiled a second time over its own
lexical rules (`m-…`), beside `program`'s, and a request carries `root`'s alone. A step's grammar is the same rules,
each class of a string's, a comment's and the marker's characters four characters longer.

## 2. With no model

`cargo test --manifest-path einaf/Cargo.toml --profile gate --test grammar -- --nocapture` on `23d582aa`, 9 tests, all
pass. Test 1 and the table as before (937 files, 856 and 81, 0 apart; 352 rows refused). Test 2: 2 000 texts, 0
refused, none of `root`'s holding a character it leaves out; the steps' 2 400, 0. **The root against the corpus and
the contract** (`the_root_accepts_the_contracts_programs_and_the_corpus_in_its_characters`): of the 856 files the parser
takes, **with their comments emptied 805 accepted**, 48 refused for a form the contract does not admit, 2 for a backtick
in a string (`examples/word_problems/words.ein`, `tests/stdlib/algebra/18_totality_open_world.ein`), 1 for `$` in a
name (`examples/syntax/arg-kinds.ein`'s `$x`); as written 21 accepted, 787 refused at a character of a comment. The
contract's 21 programs: all accepted with their comments emptied; as written 12, and 9 refused at a backtick in a
comment. Every reading as the scan of its tokens says, both ways.

## 3. The model's programs

[`models.md`](models.md), `python3 models.py`: **every program the model wrote in S2.2.2's and S2.2.3's records that
loaded, 60; the narrowed root accepts 57**, the root before it 59. The three it refuses are the first experiment's,
each at a backtick in a comment of its header (*"… is `(drinks Ukrainian Tea)` …"*); S2.2.2's also holds a `config`
form, which the root refused before. No program held a marker, a fence or a name of the characters left out.

## 4. llama.cpp's reading (test 3)

[`llama.sh`](llama.sh) → [`llama/`](llama/), `einaf grammar llama` with the model's vocabulary alone, from `root` in
the text a request carries:

| texts | tokens | accepted by both | refused by both, at the token that holds the character EinAf refuses | read otherwise |
|---|---:|---:|---:|---:|
| the corpus's 937 files, as written | 781 251 | 21 | 916 | **0** |
| the same, their comments emptied ([`empty.py`](empty.py)) | 428 524 | 805 | 132 | **0** |
| the steps by hand, 23 parts, each under its narrowed step grammar | 5 636 | 23 | 0 | **0** |

The root's sampler is built in 0.38 ms (the median over the corpus; the whole grammar's, before, 0.53).

## 5. The special tokens, on their ids

[`llama/special.jsonl`](llama/special.jsonl): each text of [`special/`](special/) tokenised with the model's special
tokens parsed (`einaf grammar llama --special`), so that `</think>` is the token 248069 and `<tool_call>` 248058, as the
model generates them, each offered alone to llama.cpp's grammar sampler from `root`:

| text | its token ids | the root before (`98e6e0a3`) | the narrowed root |
|---|---|---|---|
| `(p </think>)` | 1250 220 **248069** 8 | every token taken, then the end | **248069 refused** (token 2) |
| `(p <tool_call>)` | 1250 220 **248058** 8 | every token taken, then the end | **248058 refused** (token 2) |
| `(p </think> <tool_call>)` | 1250 220 **248069** 220 **248058** 8 | every token taken | 248069 refused (token 2) |
| `(p a) ; </think>` | 1250 264 8 2502 220 **248069** | every token taken | 248069 refused (token 5) |
| `(relation p T :why "{?3} is </think>")` | … 369 220 **248069** 871 | every token taken | 248069 refused (token 12) |
| ```` (p ``` ```` and a line break, `x)` | 1250 52451 198 830 8 | every token taken | 52451, ```` ``` ````, refused (token 1) |

**So the answer to the open question is yes**: under the grammar before, llama.cpp's sampler offered the special token
`</think>` (and `<tool_call>`) itself as the text of a name, of a comment and of a `:why`, since it matches a special
token by its piece written out; under the narrowed root each is refused at its token id. EinAf's acceptor reads every
text alike.

## 6. The speeds

[`plan.py`](plan.py) wrote the requests before the run ([`speed/requests.jsonl.gz`](speed/requests.jsonl.gz),
[`speed/order.txt`](speed/order.txt)): S2.2.3's speed request for the Zebra puzzle, 256 tokens, in the fenced frame,
with no grammar, with the root before (S2.2.3's own requests, byte for byte) and with the narrowed root, under both
checks, three calls each, 15 calls in one process through `utils/bench_env.sh --cores P --require-ac`
([`speed.sh`](speed.sh); [`speed/env.txt`](speed/env.txt): 8 P-cores, AC, loadavg 0.92; `python3 report.py`):

| request | tokens a second (median of 3) | the three | sampler built in, ms | chosen again | the same tokens |
|---|---:|---|---:|---:|---|
| no grammar | 64.2 | 65.5, 64.0, 64.2 | — | — | |
| the root before, `every-token` | 10.7 | 10.7, 10.6, 11.1 | 0.54 | — | |
| the root before, `chosen-first` | 63.0 | 63.0, 62.9, 63.4 | 0.56 | 0 | as `every-token` |
| **the narrowed root, `every-token`** | **12.6** | 12.7, 12.6, 12.6 | 0.41 | — | as the root before |
| **the narrowed root, `chosen-first`** | **62.8** | 62.9, 62.8, 62.5 | 0.42 | 0 | as `every-token` and the root before |

S2.2.3 measured 11.2 and 63.4 for the root before, 64.1 with no grammar. Every call ran to its budget on the model's
comment header, which no grammar refused. **Under `every-token` the narrowed root is 18 % faster** (12.6 against 10.7
in one session); under `chosen-first` the two are within 3 % of no grammar's speed, as before.

## 7. A fixed layout, measured and not adopted

[Q-M2.30](../../../open_questions/model/Q-M2.30-does-a-grammar-the-model-writes-under-fix-its-layout.md):
[`fixed_layout.rs`](fixed_layout.rs), an EinAf test run once and removed, wrote [`fixed-layout.gbnf`](fixed-layout.gbnf),
the narrowed root with one space between two words, none beside a paren, a line break between two forms and no
comment: **31 199 bytes and 376 rules**, against the narrowed root's 34 631 bytes (its rules, comments aside) and 383
rules; 400 rules and 4 702 elements as llama.cpp reads it, against 411 and 5 091.

## 8. The files, and how each is taken again

| file | what |
|---|---|
| [`sizes.py`](sizes.py), [`sizes.md`](sizes.md) | § 1 |
| [`models.py`](models.py), [`models.md`](models.md) | § 3 |
| [`llama.sh`](llama.sh), [`empty.py`](empty.py), [`special/`](special/), [`llama/`](llama/) | §§ 4 and 5: the corpus as written and emptied, the steps by hand (`../llama_steps.py`), the special tokens |
| [`plan.py`](plan.py), [`speed.sh`](speed.sh), [`report.py`](report.py), [`speed/`](speed/) | § 6: the requests, the run, the table, the record and `bench_env.sh`'s fingerprint |
| [`fixed_layout.rs`](fixed_layout.rs), [`fixed-layout.gbnf`](fixed-layout.gbnf) | § 7 |

From the repository's root, with the model's file in `M`:

```sh
CMAKE_CUDA_ARCHITECTURES=89 cargo build --release --features cuda --manifest-path einaf/Cargo.toml
D=plans/m2_nl_to_ir/p2.2_formalizer/grammars/narrowed
python3 $D/sizes.py > sizes.md; python3 $D/models.py > models.md
$D/llama.sh $M                                   # test 3 and the special tokens, into $D/llama/
python3 $D/plan.py; $D/speed.sh $M speed.jsonl    # the speeds' requests, byte for byte, and their 15 calls
einaf/target/release/einaf grammar accept --root root $D/special/*.txt   # each refused
```
