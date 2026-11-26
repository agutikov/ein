# S2.2.3's grammars: the whole grammar, a grammar a step, and the model under them

[S2.2.3](../s2.2.3_gbnf.md)'s report. **EinAf writes Ein's grammar as GBNF** from Ein's two grammar files, holds it
to Ein's parser in both directions with no model and to llama.cpp's own grammar sampler with the model's vocabulary,
writes **a grammar a step** of the formalization from what the step is given, and runs the model under them: where a
grammar begins, how an output ends, what a grammar costs a generation. The code is `einaf::grammar`
([the grammars' page](../../../../docs/einaf/06-grammars.md)); the records are the files of this folder
([§ 8](#8-the-files)). Everything was run on 2026-11-26 on the machine the work runs on, on AC power, the GPU free of
any other model (ollama not running). The code every call ran is `680a94ad`'s for the first recorded generations and
test 3 on the corpus and the table, and `e657682d`'s for the rest; each plan was committed before its first
generation (`3ac3ecf9`, `b4f63367`, `61a566a7`).

**Amended on the user's word the same day**: the grammars a model writes under leave out the chat template's markers
and the fence ([the amendment's records](narrowed/README.md)); what follows is the stage as it ran.

**What it comes to.**

- **Ein's grammar is a GBNF grammar, exactly.** Generated from `ein.l` and `ein.y` (531 rules; five hand-written
  pieces stand for what the lexer does beside the two files), it accepts every one of the corpus's 937 files the
  parser accepts and refuses every one it refuses, refuses all 352 shapes of the parser's banked table, and 2 000 texts
  sampled from it all parse. **llama.cpp reads it as EinAf does**: 781 251 tokens of the corpus, the table's rows and
  the steps by hand through its own grammar sampler with the model's vocabulary, no difference.
- **The corpus and the table alone would not have held it**: of five hand-written rules broken in turn, two are seen
  by neither, and only the sampled texts see all five (E-121); one of the two is a lexer rule stated one way in
  `ein.l` and working another (E-120).
- **A grammar a step** exists for steps 1, 2, 2b and 3, written at request time; each accepts the contract's steps by
  hand, rendered on their vocabulary, and every text sampled from it parses and is of its step. Steps 2b and 3,
  rendered on the program before them, close every pattern, conclusion and activating fact to a declared relation at
  a declared count.
- **A grammar must begin where the program begins.** Put at the first token against a prompt that asks for a fenced
  block, it refuses the model's fence, and the model writes the program its prompt shows nearest, another text's
  (L-M2.14). Inside the fence, **14 of 15 library programs are the model's own token for token**: a grammar under
  greedy decoding acts only at the tokens it refuses, and the model's Ein was Ein's but where it was not (one
  keyword out of place, now `:why`; one `config` the contract's root refuses, now a fact `configure`). With thinking
  on, a grammar begun at the token `</think>` costs the thinking nothing and holds the answer after it
  (`(youngest Cy)`); one begun at a pattern stalls the generation (§ 4.1). Every thinking-off output under the fenced
  grammars ended its turn at its closing fence; none ran into its budget.
- **A grammar costs a generation nothing measurable** when the chain's choice is checked first (50 to 64 tokens a
  second against 64, the same tokens), and most of its speed when the grammar is applied to every candidate first (11
  to 26); its sampler is built in under 0.6 ms.
- **Q-M2.6's default: many**, a grammar a step, the whole grammar kept for a program in one call and as the parser's
  second reader (§ 6).

## 1. The whole grammar

`einaf grammar write` writes [`einaf/grammars/ein.gbnf`](../../../../einaf/grammars/ein.gbnf) from
`ein.rs/crates/ein-ir/src/ein.l` and `ein.y`, which the build embeds, read through grmtools' readers as Ein's grammar
page is written from them: `lrlex` and `regex-syntax` for the lexer's rules, each regular expression translated node
for node; `cfgrammar` for the phrase rules, each production an alternative and a list the repetition it derives. 2 007
lines, **531 rules**, 50 131 bytes; its digest is the manifest's,
[`MANIFEST.sha256`](../../../../einaf/grammars/MANIFEST.sha256). The inventory of every grammar and every
hand-written rule is [`einaf/grammars/README.md`](../../../../einaf/grammars/README.md).

**Where the rules come from.**

| part | rules | from |
|---|---:|---|
| the two roots, `program` and `root`, and their forms' states | 6 | hand-written over `ein.y`'s `Program`, `TopForm` and `Form` |
| `ein.y`'s rules, compiled into a rule a state | 337 | `ein.y`'s 101 rules: a list and a rule that begins or ends with more than one kind of token written into the rules that use it, `Template` written once a kind |
| a name less the 29 words an earlier lexer rule takes whole | 175 | `ein.l`'s `NAME`, and the 25 words, `_` and the three writeback names before it: a tree over their prefixes |
| a variable less `?_` | 3 | `ein.l`'s `VAR` and `VARUNDERSCORE` |
| a string | 1 | `ein.l`'s `TEXT`, node for node |
| the separators | 9 | `ein.l`'s skip rule's three pieces node for node, and six hand-written: where a token ends, a line comment's end, a string's |

**Why the phrase grammar is compiled into states.** GBNF is one grammar over characters; Ein's lexer is a pass of its
own, and a token that is not a paren runs to a separator or a paren. So the separator before a token depends on the
kind of the token before it: none or any between a paren and anything, one or more between two words, and after a
string one that does not begin with `#`
([E-120](../../evaluation/language/E-120-a-string-glued-to-a-block-comment-is-refused-where-a-name-glued-to-one-is-not.md)).
Each rule's productions are compiled into the Glushkov automaton of their regular expression over atoms (a token, or a
rule of one start and one end kind), each state a rule, the separator before each atom chosen by the two atoms'
kinds, states that behave alike one rule.

## 2. The grammar held to the parser

**Test 1, the corpus through EinAf's acceptor** (`einaf/tests/grammar.rs`,
`the_grammar_accepts_a_corpus_file_exactly_when_the_parser_does`): **937 files** under `examples/`, `stdlib/` and
`tests/`, 2 530 523 bytes: **856 accepted by both, 81 refused by both, 0 apart**; each of the 81 refused on the line of
the parser's first error, 48 at its column. Neither named refusal occurs in the corpus: no file is nested past 1 000
parentheses or holds a byte that is not UTF-8. **The banked table**: all **352 rows** of
`ein-ir/tests/golden/grammar_table.txt` refused by the parser and by the grammar. 0.3 s for the corpus on the gate's
worker pool.

**Test 2, the grammar's texts through the parser** (`every_text_sampled_from_the_whole_grammar_parses`): **2 000
texts from the seed 20 261 126**, half from each root, 632 561 characters: **0 refused**. They hold 36 028 tokens that
are one of the 25 words, 3 006 names that begin with one or are one cut short (`relationneq`, `note`, `abse79+`), 38 932
tokens glued to a paren, 19 032 line comments and 17 684 block comments. 1.7 s.

**Test 3, llama.cpp's reading** (by hand, `einaf grammar llama`, the `cuda` build on `680a94ad`, the model's
vocabulary alone): each file tokenised, its tokens fed one by one to llama.cpp's grammar sampler made from the same
text, from the root `program`, the end of turn offered after the last.

| files | tokens | accepted by both | refused by both, at the token that holds the character EinAf refuses | read otherwise | the pieces are the text |
|---|---:|---:|---:|---:|---|
| the corpus, 937 | 781 251 | 856 | 81 | **0** | every file |
| the banked table's rows, 352 | — | 0 | 352 | **0** | every row |
| the steps by hand, 23 parts, each under its step's grammar (on `e657682d`) | 5 636 | 23 | 0 | **0** | every part |

llama.cpp makes the whole grammar's sampler in 0.53 ms (the median over the corpus), and reads a token in 0.076 ms,
offered alone. **llama.cpp reads the grammar as EinAf's acceptor does**: no difference in 1 312 texts.

**What the corpus and the table would have missed.** Each of five hand-written rules broken in turn, and the broken
grammar run against the three checks ([`mutants/mutants.rs`](mutants/mutants.rs), run as an EinAf test and removed):

| the rule broken | corpus files apart (of 937) | table rows taken (of 352) | sampled texts refused (of 2 000) |
|---|---:|---:|---:|
| a name may be one of the 29 words | 40 | 111 | 12 |
| `?_` is a variable | 0 | 13 | 2 |
| a string may be glued to `#` | **0** | **0** | 156 |
| a line comment need not end its line | **0** | **0** | 914 |
| two words need no separator | 26 | 74 | 726 |

Two of the five are seen by the sampler alone
([E-121](../../evaluation/language/E-121-the-banked-table-of-refused-shapes-misses-two-of-the-lexer-s-token-ends.md)):
test 2 is what holds them.

## 3. A grammar a step

`einaf::grammar::steps::render`, the whole grammar restricted to the forms a step writes, in its order, written at
request time on what the step is given ([the page § 3](../../../../docs/einaf/06-grammars.md#3-a-grammar-a-step)).
**The catalogue** they are rendered on is a stand-in read off the library's texts (`einaf grammar catalogue`, every
module of `std` but `std.core`;
[L-M2.12](../findings/einaf/L-M2.12-the-grammars-catalogue-is-every-module-and-every-rule.md)).

| step | rendered on | Zebra: bytes, rules | `apples`: bytes, rules | what its vocabulary closes |
|---|---|---|---|---|
| 1 | the text's sentences | 14 928, 150 | 14 265, 150 | a `:source` is a sentence of the text; a declaration's `:why` references only its arrows; one answer query; nothing of a fact's relation or count |
| 2 | the catalogue; step 1's program | 55 557, 172 | 55 557, 172 | an import is a module of the catalogue and lists names it declares, not one step 1 declared; an activating fact is the catalogue's, at its count; the ontology is open ([Q-M2.29](../../open_questions/formalizer/Q-M2.29-are-step-2-s-facts-held-to-the-relations-step-2-declares.md)) |
| 2b | the catalogue; the program of steps 1 and 2 | 19 349, 131 | 35 854, 131 | every pattern and conclusion of a relation the program or a module it imports declares, at a declared count, over the program's names |
| 3 | the same | 23 781, 136 | 36 404, 136 | as 2b; and every activating fact of a module the program imports, at its count, over its names |

**Held to the parser with no model** (`einaf/tests/grammar.rs`): each step's grammar, rendered for each of the six
texts of the contract's steps by hand, **accepts the part that step wrote by hand** (23 parts; a marker written above
each rule of step 3 and of step 2b, which the steps by hand do not carry,
[L-M2.13](../findings/einaf/L-M2.13-the-steps-by-hand-carry-no-marker-of-the-grammars-shape.md)); and **2 400 texts
sampled from them**, 100 a grammar from the seed 20 261 126, every one parses and is of its step's forms in its order.
llama.cpp reads the 23 parts as EinAf does (§ 2).

## 4. Where a grammar begins, and how an output ends

Every run is the probe's request of the same item ([`../model_runtime/plan.jsonl`](../model_runtime/plan.jsonl)), its
prompt, sampler (greedy) and seed unchanged, with a grammar added, so that the probe's record is each run with no
grammar. Under greedy decoding with the chain's choice checked first **a grammar acts only at the tokens it refuses**:
up to the first, the output is the probe's token for token. `einaf probe run` judged each answer as the probe did: the
program it holds (its fenced block where it has one) parsed, loaded, and asked `--no-search` and `--single` under a
30-second clock.

### 4.1 The first plan: the grammar from the first token, no fence

[`first/plan.jsonl.gz`](first/plan.jsonl.gz), on `680a94ad`; `python3 report.py first/record.jsonl.gz`. The probe's
prompts ask for the program in one fenced block, and the grammar held the whole output with no fence in it. **The
model's first token, the fence, was refused in every run, and what it wrote next was the program its prompt shows
nearest**: the excerpt's `youngest` program for six library items of other families (`before`, `bob1`, `boxes12`,
`contra`, `gap`, `seating`, each *one*, *“Cy is the youngest”*); for step 1 of the Zebra puzzle and of the seating, the
excerpt's `older` facts with the text's sentences as `:source` (`(older Ann Bob :source "The Englishman lives in the
red house.")`); and four runs went on to the budget repeating a form (step 1 of `apples` and `more`, the library's
`more` and `unused`, cycles of 25 to 214 tokens from the 184th to the 768th). With no grammar every one of them wrote
its own text's program ([L-M2.14](../findings/model/L-M2.14-a-grammar-that-leaves-out-the-fence-a-prompt-asks-for-derails-the-model.md)).
**With thinking on**, the probe's step 1 of `youngest` under a grammar begun at the pattern `[\s\S]*?(</think>)[\s\S]*`
had not ended after 19 minutes, the GPU idle between tokens, and was stopped (the same request with no grammar
generates its 16 384 tokens in 274 s): llama.cpp matches the whole thinking against the pattern at every token. Its
short twin, `demo.on.youngest`, was stopped after nine minutes for the same reason. Neither is in the record.

### 4.2 The second plan: inside the fence

[`fenced/plan.jsonl.gz`](fenced/plan.jsonl.gz), on `e657682d`; `python3 report.py fenced/record.jsonl.gz`. Every
grammar begins inside the fence the prompt asks for, and after the thinking at the model's token `</think>`.

| run | tokens, stop | refused after the first token | the program under the grammar | with no grammar | the probe's tokens kept | where the two part: no grammar's text, the grammar's |
|---|---|---:|---|---|---|---|
| step 1, `zebra` | 1237, end-of-turn | 2 | load (goal) | loads; --single several | the first 210 | `)⏎(lives-in Englishman red)⏎(owns `; ` :source "There are five houses.")` |
| step 1, `youngest` | 217, end-of-turn | 0 | loads; --single several | loads; --single several | **all** | — |
| step 1, `seating` | 379, end-of-turn | 1 | loads; --single several | loads; --single several | the first 228 | `⏎;; ──── The ontology ────────────`; `;; ──── The ontology ─────────────` |
| step 1, `apples` | 305, end-of-turn | 3 | loads; --single several | loads; --single several | the first 141 | `⏎;; ──── The ontology ────────────`; `;; ──── The ontology ─────────────` |
| step 1, `together` | 287, end-of-turn | 0 | loads; --single several | loads; --single several | **all** | — |
| step 1, `more` | 311, end-of-turn | 3 | loads; --single several | loads; --single several | the first 171 | `⏎;; ──── The ontology ────────────`; `;; ──── The ontology ─────────────` |
| whole, `before` | 405, end-of-turn | 0 | loads; --single one | loads; --single one | **all** | — |
| whole, `bob1` | 688, end-of-turn | 0 | loads; --single not-decided | loads; --single not-decided | **all** | — |
| whole, `boxes12` | 655, end-of-turn | 0 | load (unbound-conclusion) | load (unbound-conclusion) | **all** | — |
| whole, `boxes13` | 583, end-of-turn | 0 | loads; --single one | loads; --single one | **all** | — |
| whole, `bystander` | 534, end-of-turn | 0 | load (unbound-conclusion) | load (unbound-conclusion) | **all** | — |
| whole, `contra` | 592, end-of-turn | 0 | loads; --single one | loads; --single one | **all** | — |
| whole, `gap` | 513, end-of-turn | 0 | load (unbound-conclusion) | load (unbound-conclusion) | **all** | — |
| whole, `gap_fact` | 507, end-of-turn | 0 | load (full-name) | load (full-name) | **all** | — |
| whole, `kinds` | 488, end-of-turn | 0 | loads; --single one | loads; --single one | **all** | — |
| whole, `kinds_full` | 550, end-of-turn | 0 | load (full-name) | load (full-name) | **all** | — |
| whole, `more` | 526, end-of-turn | 0 | loads; --single inconsistent | loads; --single inconsistent | **all** | — |
| whole, `pencils` | 776, end-of-turn | 1 | load (unbound-conclusion) | parse (parse) | the first 350 | `source "250 pencils go to the offi`; `why "the number of pencils for the` |
| whole, `seating` | 299, end-of-turn | 0 | loads; --single one | loads; --single one | **all** | — |
| whole, `unused` | 632, end-of-turn | 0 | loads; --single one | loads; --single one | **all** | — |
| whole, `youngest` | 364, end-of-turn | 0 | loads; --single one | loads; --single one | **all** | — |
| whole, `first-experiment` | 2910, end-of-turn | 11 | loads; --single one | loads; --single one | the first 2245 | ` :record-alternative-justification`; `ure :source "the Zebra puzzle" :so` |

**Under the whole grammar, 14 of the 15 library programs are the probe's token for token**, and the grammar refused
nothing after the fence: the model's own Ein was already Ein's. The one it changed is `pencils`, whose `(relation
pencils-office Number :source "250 pencils go to the office.")` the parser refused with no grammar (S2.2.2's one
`parse` error of the library condition): the grammar refused `:source` there and the model wrote `:why`, so the
program parses, and is refused at the load for a conclusion's variable no premise binds (`unbound-conclusion`, which
no grammar can prevent). And the first experiment's, whose `(config :record-alternative-justifications false)` the
contract's root does not take: the grammar refused `config`, and the model wrote the name `configure` in its place, a
fact `(configure :source "the Zebra puzzle" …)`, a form no sentence states, and the program still answers *one*,
*“The Norwegian drinks water. The Japanese owns the zebra.”* (a grammar too tight writes what it must: here a reserved
word made a name). **The library condition under the whole grammar**: 15 of 15 programs parse (14 with no grammar),
9 load (9), and the same outcomes.

**Under step 1's grammar, two of six are the probe's token for token** (`youngest`, `together`). The others part where
the grammar refuses: three at a second blank line before a comment header (the layout allows one), and the Zebra
puzzle's at its 210th token, where with no grammar its first statement closes with no `:source` (none of its
statements has one, S2.2.2's step 1): the grammar requires the sentence, the model wrote `:source "There are five
houses."` and went on to another program, whose answer query reads a relation it no longer writes (`goal`, refused at
the load). Five of six load and are *several* at root's
`--single`, as with no grammar.

**How an output ends.** Every thinking-off run ended its turn at the closing fence (22 of 22): the grammar allows the
end of turn after the fence and nothing else, and the model took it. **None ran into the budget by repeating a form**,
where the first plan's four did; with no grammar none of these did either. Both thinking-on runs ran to their budgets
in the thinking, a cycle of 105 tokens from the 3 483rd and of 41 from the 2 824th, and the grammar never began
(`began_at` none): what Q-M2.27 has every greedy thinking do on these prompts. **A grammar that waits for its token
costs a thinking nothing**: the probe's step 1 of `youngest`, 16 384 tokens all thinking, in 273.6 s under the grammar
and 273.9 s with none, the same tokens.

### 4.3 A grammar begun where a thinking ends

[`after/plan.jsonl.gz`](after/plan.jsonl.gz), on `e657682d`, committed before its run (`61a566a7`): three short
prompts with thinking on, each grammar begun at the model's token `</think>` with no fence (none asks for one), 2 048
tokens at most; `python3 report.py after/record.jsonl.gz`.

| run | the grammar | thinking | began at | what came after the thinking | stop | the program |
|---|---|---:|---:|---|---|---|
| `after.question`: *Who is the youngest? Answer with one name.* (S2.2.2's host request 02) | the whole grammar's `root` | 268 | 268 | `(Cy)` | end of turn | loads; `--single` *one* |
| `after.fact`: the same, *Answer with one Ein fact of the relation `youngest`* | the whole grammar's `root` | 310 | 310 | `(youngest Cy)` | end of turn | loads; `--single` *one* |
| `after.step1`: the same text, its step 1 asked for | step 1's | 2 048 | never | — | the budget, in the thinking | — |

**The grammar begins where the thinking ends**, at the first token after `</think>` (`began_at` the count of thinking
tokens), and holds the answer: where the model's answer is a form, as in `after.fact`, the grammar refuses nothing and
the answer is the model's own, `(youngest Cy)`, and the turn ends at it; where it is not, as in `after.question`, the
grammar refuses the bare name and the model writes the nearest form, `(Cy)`, a fact of a relation called `Cy` with no
argument, which loads (a grammar holds an output to Ein, not to what was meant). The thinking is the host probe's
request 02's, token for token: a grammar that waits for its token changes nothing before it. A step 1 asked for with
thinking on loops in the thinking, as every step's prompt did.

## 5. The speeds

[`speed/README.md`](speed/README.md) is the reading, written before the run; [`speed.sh`](speed.sh) made the 66 calls
in one process, through `utils/bench_env.sh --cores P --require-ac`, on 2026-11-26 at 12:21 (*i9-14900HX, pinned to
the 8 physical P-cores, governor powersave, turbo on, AC, loadavg 0.76*, [`speed/env.txt`](speed/env.txt)), with
`e657682d`'s binary and `b4f63367`'s requests. Each request is the probe's step-1 request of its text (3 761 prompt
tokens for the Zebra puzzle, 3 613 for `apples`, evaluated in 1.43 to 1.67 s), its grammar in the fenced frame, 256
tokens; every call ran to the budget. **Tokens a second, the median of three calls** (the three within 1.6 of each
other; `python3 report.py --speeds` has every call):

| grammar | Zebra: `every-token` | Zebra: `chosen-first` (choices refused) | `apples`: `every-token` | `apples`: `chosen-first` (choices refused) | its sampler built in, ms: Zebra, `apples` |
|---|---:|---:|---:|---:|---|
| none | 64.1 | | 63.8 | | — |
| the whole grammar, `root` | 11.2 | 63.4 (0) | 12.8 | 64.2 (0) | 0.56, 0.55 |
| step 1 | 24.7 | 63.0 (1) | 24.6 | 63.0 (3) | 0.20, 0.19 |
| step 2 | 12.7 | 62.7 (0) | 18.3 | 62.1 (9) | 0.44, 0.44 |
| step 2b | 25.6 | 62.7 (4) | 25.6 | 63.3 (3) | 0.20, 0.31 |
| step 3 | 20.1 | 58.7 (12) | 14.6 | 50.1 (16) | 0.25, 0.33 |

- **Under `every-token` a grammar costs a generation most of its speed**: 11.2 to 25.6 tokens a second against 64,
  the grammar's work on the vocabulary's 248 320 tokens at every token, 25 ms (step 1) to 73 ms (the whole grammar) a
  token beside the model's 15.6 ([L-M2.11](../findings/model/L-M2.11-a-grammar-applied-to-every-candidate-costs-a-generation-most-of-its-speed.md)).
- **Under `chosen-first` the same tokens come at 50.1 to 64.2 tokens a second**, every request's tokens the
  `every-token` call's (10 of 10, as greedy decoding says): within 3 % of no grammar but where the grammar refuses the
  model's choice and reads the vocabulary once for it, 12 and 16 times in 256 tokens under step 3 (58.7 and 50.1).
  Every call spent most of its 256 tokens on the comment header the model opens its block with, which every grammar
  allows; on whole programs, **the second plan's 22 thinking-off runs generated 13 758 tokens at 62.5 tokens a
  second, and the probe's same requests with no grammar 13 186 at 62.5** (those runs' times are no measurement: no
  `bench_env.sh`, the judgement beside them).
- **Building a grammar's sampler costs 0.19 to 0.56 ms** (the envelope's `build_ms`), the whole grammar's the most,
  against a prompt evaluated in 1.4 s. **Writing a grammar** in EinAf, the catalogue read off the library's 28 module
  texts, takes 36 to 38 ms for the whole grammar and step 1's and 98 to 100 ms for steps 2, 2b and 3, the process
  started and ended (`speed/render.sh`, ten runs each through `bench_env.sh`, [`speed/render.txt`](speed/render.txt)).
- **A grammar a step costs a call nothing it can measure**, and neither does the whole grammar, held by the chosen
  token first. With thinking on, a grammar begun at its token costs the thinking nothing (§ 4.2); one begun at a
  pattern costs it everything (§ 4.1).

## 6. Q-M2.6: one grammar or many

**The default the stage took, for the user's review: *many*.** A grammar a step, steps 1, 2, 2b and 3, each the whole
grammar restricted to the step's forms in its order and rendered at request time on what the step is given; and the
whole grammar kept, generated, as the root of a program written in one call and as the parser's second reader. Each
is carried inside the fence the prompt asks for, begun after the thinking at `</think>` where thinking is on, and held
to by the chain's choice first. It is written into
[Q-M2.6](../../open_questions/formalizer/Q-M2.6-one-gbnf-or-many.md), left open.

- **The probe's errors, by what would have prevented each** (S2.2.2, thinking off). *Any grammar*: the fence lines
  (every answer read whole: the fenced frame takes them), a keyword a form does not take (`pencils`' `:source` on a
  declaration: under the whole grammar it became `:why`, § 4.2), and the described condition's shapes (14 of 15).
  *Only a step's grammar*: a module listed as a name of another (`import-list`, 2: step 2's imports are the
  catalogue's modules, each listing names it declares), a name declared and listed from an import (`full-name`, 2:
  step 2's lists leave out what step 1 declared), a `:why` naming an arrow its declaration lacks (E-119, 6
  declarations of S2.2.2's step 1: a step's declaration holds its references to its arrows), and in steps 2b and 3 a
  pattern of a relation nothing declares or at another count (E-12). *No grammar*: a conclusion's variable no premise
  binds (`unbound-conclusion`, 3, still refused at the load under the whole grammar), and every way a loaded program
  was wrong. **So three kinds the probe met are removed by a step's grammar and not by the whole one; the one kind
  that is half of the library condition's refusals is removed by neither.**
- **The speeds** (§ 5): under `chosen-first` a step's grammar costs a generation 1 to 3 % of its speed but where it
  refuses (step 3: 8 and 22 % with 12 and 16 refusals in 256 tokens, which a whole program does far less often), and
  the whole grammar 0 to 1 %; under `every-token` the step grammars run at 12.7 to 25.6 tokens a second and the whole
  grammar at 11.2 to 12.8, so the working answer's *smaller grammars decode faster* holds there and nowhere else. A
  sampler is built in 0.19 to 0.56 ms and a step's grammar written in 0.1 s, against a prompt evaluated in 1.4 s: **no
  step's grammar costs more to build a call than it saves**.
- **What the vocabulary closure is worth**: steps 2b and 3 close everything they write, rendered on the program
  before them; step 1 closes no fact (its facts come before their declarations: under its grammar the model still
  wrote a fact of a relation it never declared, `(person Ann)` in `youngest`, the first plan), and step 2 none of its
  ontology's facts ([Q-M2.29](../../open_questions/formalizer/Q-M2.29-are-step-2-s-facts-held-to-the-relations-step-2-declares.md)).
  What it closes is what the contract's check would otherwise report after the call (S2.2.4).

**What would reverse it**, by the stage's terms: the probe finding no error a step's grammar removes and the whole one
does not (it found three kinds), or a step's grammar costing more to build a call than it saves (a sampler in under a
millisecond). Neither held.

## 7. What it found

**Of Ein** ([the phase's evaluation](../evaluation/README.md)):

- [E-120](../../evaluation/language/E-120-a-string-glued-to-a-block-comment-is-refused-where-a-name-glued-to-one-is-not.md)
  (weakness): a string glued to a block comment is refused, where a name glued to one is a name and a comment;
  `ein.l` says `#|` ends every token, and `BADTEXT`'s tail takes `#`.
- [E-121](../../evaluation/language/E-121-the-banked-table-of-refused-shapes-misses-two-of-the-lexer-s-token-ends.md)
  (gap): the parser's banked table of refused shapes, with the corpus, misses two of the lexer's token ends, which only
  sampled texts saw (§ 2).
- [E-122](../../evaluation/language/E-122-ein-s-syntax-is-context-free-but-for-two-refusals.md) (strength): Ein's syntax,
  its lexer included, is a context-free language but for two refusals, and a second reader takes it exactly.
- **What of the lexer could not be said in a context-free grammar**: nothing but the two named refusals, which no
  corpus file meets; what saying it cost: a tree of 175 rules for a name, and `ein.y`'s 101 rules compiled into 337.
  **Corpus files on which the acceptor and the parser part for a reason that is the grammar files'**: none of 937.
- **Confirmed**, a line each: E-12 (under step 1's grammar the model wrote a fact of a relation it never declared, which
  no grammar of step 1 can hold to a declaration), E-117 (the model writes its fence first, every time), E-119 (a
  step's grammar holds a declaration's `:why` to its arrows).

**Of EinAf, the plan and the process** ([the phase's findings](../findings/README.md)):
[L-M2.11](../findings/model/L-M2.11-a-grammar-applied-to-every-candidate-costs-a-generation-most-of-its-speed.md)
(a grammar applied to every candidate costs a generation most of its speed; the chosen token first costs it nothing;
closed by the runtime's `check`),
[L-M2.12](../findings/einaf/L-M2.12-the-grammars-catalogue-is-every-module-and-every-rule.md) (the catalogue a
step's grammar is rendered on is a stand-in, every module and every rule of a parameter or more; S2.2.4's narrows it),
[L-M2.13](../findings/einaf/L-M2.13-the-steps-by-hand-carry-no-marker-of-the-grammars-shape.md) (the steps by hand
carry no marker of the grammar's shape),
[L-M2.14](../findings/model/L-M2.14-a-grammar-that-leaves-out-the-fence-a-prompt-asks-for-derails-the-model.md) (a
grammar that leaves out the fence a prompt asks for derails the model; closed by the fenced frame).

**For the user** ([the phase's open questions](../open_questions/README.md)):
[Q-M2.28](../../open_questions/model/Q-M2.28-is-a-token-held-to-its-grammar-by-the-chain-s-choice-first.md), whether a
token is held to its grammar by the chain's choice first (default: yes, for every grammar EinAf writes);
[Q-M2.29](../../open_questions/formalizer/Q-M2.29-are-step-2-s-facts-held-to-the-relations-step-2-declares.md), whether
step 2's facts are held to the relations step 2 declares (default: no grammar holds them, the contract's check does);
and [Q-M2.6](../../open_questions/formalizer/Q-M2.6-one-gbnf-or-many.md)'s default (§ 6).

## 8. The files

| file | what |
|---|---|
| [`plan.py`](plan.py) | writes the second plan, `fenced/plan.jsonl.gz`, and the speed's requests, each request whole, its grammar by `einaf grammar render … --fence --request` |
| [`first/plan.jsonl.gz`](first/plan.jsonl.gz), [`first/record.jsonl.gz`](first/record.jsonl.gz) | the first plan (24 runs, the grammar from the first token with no fence), committed before its first generation (`3ac3ecf9`), and its record: a line a run (`einaf-probe/1`), the call whole and the kernel's judgement of each reading |
| [`fenced/plan.jsonl.gz`](fenced/plan.jsonl.gz), [`fenced/record.jsonl.gz`](fenced/record.jsonl.gz) | the second plan (the grammar inside the fence the prompt asks for), committed before its first generation, and its record |
| [`after/plan.jsonl.gz`](after/plan.jsonl.gz), [`after/record.jsonl.gz`](after/record.jsonl.gz) | the third plan (three short prompts with thinking on, the grammar begun at `</think>`), committed before its first generation (`61a566a7`), and its record |
| [`speed/README.md`](speed/README.md) | the speed's reading, written before the run |
| [`speed/requests.jsonl.gz`](speed/requests.jsonl.gz), [`speed/order.txt`](speed/order.txt), [`speed/record.jsonl.gz`](speed/record.jsonl.gz), [`speed/env.txt`](speed/env.txt) | the speed's 22 requests, the order of its 66 calls, their record (the 66 calls' records in order), and `bench_env.sh`'s fingerprint |
| [`speed/render.sh`](speed/render.sh), [`speed/render.txt`](speed/render.txt), [`speed/render-env.txt`](speed/render-env.txt) | how long EinAf takes to write each grammar a request carries, ten runs each, and `bench_env.sh`'s fingerprint |
| [`llama/corpus.jsonl.gz`](llama/corpus.jsonl.gz), [`llama/table.jsonl.gz`](llama/table.jsonl.gz), [`llama/steps.jsonl.gz`](llama/steps.jsonl.gz), [`llama/corpus.list`](llama/corpus.list) | test 3: a JSON line a text, EinAf's verdict and llama.cpp's reading (`einaf grammar llama`) |
| [`mutants/mutants.rs`](mutants/mutants.rs) | the five broken grammars and the three checks, an EinAf test run once and removed |
| [`run.sh`](run.sh), [`speed.sh`](speed.sh), [`llama_steps.py`](llama_steps.py), [`llama_table.py`](llama_table.py) | the runs, each one command (`run.sh` takes a plan) |
| [`report.py`](report.py) | the tables of § 4 and § 5 from the records |
| [`narrowed/`](narrowed/README.md) | the amendment on the user's word: the grammars a model writes under narrowed, their sizes, tests, the model's programs, test 3, the special tokens on their ids, the speeds |

## 9. How each part is taken again

From the repository's root, with the model's file in `M` (its SHA-256 is checked by the envelope, not by the path):

```sh
CMAKE_CUDA_ARCHITECTURES=89 cargo build --release --features cuda --manifest-path einaf/Cargo.toml
D=plans/m2_nl_to_ir/p2.2_formalizer/grammars
einaf/target/release/einaf grammar write --check                      # the whole grammar is what ein.l and ein.y make
cargo test --manifest-path einaf/Cargo.toml --profile gate --test grammar -- --nocapture   # tests 1 and 2, the steps' grammars
einaf/target/release/einaf grammar llama --model $M --list $D/llama/corpus.list > corpus.jsonl   # test 3, the corpus
python3 $D/llama_table.py $M table.jsonl; python3 $D/llama_steps.py $M steps.jsonl              # test 3, the table and the steps
python3 $D/plan.py                                                    # the second plan and the speed's requests, byte for byte
$D/run.sh $D/fenced/plan.jsonl.gz $M record.jsonl                     # the second plan's generations; a run done is skipped
$D/speed.sh $M speed.jsonl                                            # the speeds, through bench_env.sh
$D/run.sh $D/after/plan.jsonl.gz $M after.jsonl                         # the third plan's thinking-on prompts
$D/speed/render.sh > render.txt                                       # EinAf's time to write each grammar
python3 $D/report.py $D/fenced/record.jsonl.gz; python3 $D/report.py --speeds   # the tables
```
