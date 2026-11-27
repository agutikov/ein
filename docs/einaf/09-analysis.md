# The text's analysis

**Step 1 may be given an analysis of its text by trained parsers**, beside the text: each sentence's meaning as an
Abstract Meaning Representation (AMR), a rooted graph of concepts and the roles between them, and the named entities a
second model found. **EinAf runs no parser**: the analysis is a recorded file, one a text, written by whoever ran the
parsers, and EinAf reads it, writes it in one of three forms and puts the form after the text in step 1's prompt. A
run given none renders step 1's prompt as it always has. The code is [`einaf::analysis`](../../einaf/src/analysis/mod.rs),
its reader of PENMAN [`penman.rs`](../../einaf/src/analysis/penman.rs), its facts [`facts.rs`](../../einaf/src/analysis/facts.rs),
its command [`analysis_cmd.rs`](../../einaf/src/analysis_cmd.rs); its prompts are under
[`einaf/prompts/analysis/`](../../einaf/prompts/analysis/).

| section | what |
|---|---|
| [1. The recorded analysis](#1-the-recorded-analysis) | `einaf-analysis/1`, key by key |
| [2. PENMAN, read](#2-penman-read) | the notation the parsers print, and a role read the other way |
| [3. The forms](#3-the-forms) | `penman`, `lines` and `facts`; the spelling of AMR in Ein's names; a node's words; the named entities |
| [4. Step 1 given an analysis](#4-step-1-given-an-analysis) | the template, the slot, what moves and what does not; what is refused |
| [5. The record](#5-the-record) | what a run keeps of the analysis it was given |
| [6. The commands](#6-the-commands) | `einaf analysis`, and the options of `einaf steps run` and `einaf from-text` |
| [7. Held by](#7-held-by) | the tests |

## 1. The recorded analysis

A JSON file, `einaf-analysis/1` (`einaf::analysis::Analysis`); the six texts with steps by hand have theirs under
[`einaf/tests/fixtures/analysis/`](../../einaf/tests/fixtures/analysis/), which EinAf's tests read with no Python and
no GPU.

| key | what |
|---|---|
| `schema` | `einaf-analysis/1` |
| `text`, `text_sha256` | the text's name and the digest of the file the parsers read |
| `sentences` | the text's sentences, in order: the parsers' input, a graph each. **A run is given an analysis only of its own text's sentences**, as EinAf splits them (`einaf::contract::sentences`) |
| `amr` | a parser each, the one a run is given where none is named first: `parser`, its short name; `tool`, the tool, its version, its model and the model's digest, its decoding, as the parser's run recorded them; `aligner`, whose markers the graphs carry, `name` and `kind`, `learned` (the parser aligns each node it makes, by what it learned) or `rule-based`; `graphs`, a sentence each, `n` from 1, the `tokens` the markers index from 0, and `penman`, the graph as the run wrote it with the markers in it, or `null` where the parser wrote none |
| `entities` | the named entities a second model found: `tool`, and `sentences`, a list a sentence, each entity its `text` and its `label` |

The six texts' files hold three parsers, SPRING and XFM (`amrlib`, the rule-based aligner RBW's markers `~e.N`) and
IBM's transition parser (its own markers `~N`), and spaCy's named entities.

## 2. PENMAN, read

The parsers print a graph as PENMAN: `(variable / concept :role target …)`, a target a node, a variable met again
(the same node), a constant (`5`, `-`) or a string, each with the aligner's marker after it where it has one.
EinAf reads it by the `penman` package's lexicon (`einaf::analysis::penman::read`) and reads no English word.
**A role read the other way** (`:ARG0-of`, a role ending in `-of` that is none of AMR's three roles spelt so,
`:consist-of`, `:prep-out-of` and `:prep-on-behalf-of`) is the role itself between the two nodes swapped:
`(z2 / person :ARG0-of (z3 / smoke-02))` is the edge from `z3` by `:ARG0` to `z2`. `without_alignments` writes a graph as
it was printed, every character kept but the markers.

## 3. The forms

**`penman`, the default**: each sentence under `# ::snt` and the sentence, its graph as the parser printed it, the
markers removed (they index tokens the prompt does not show). **`lines`**: each sentence under `Sentence n:`, a node a
line, its variable, its concept, and each role it has with its target, a role read the other way written on the node
it starts from. **`facts`**: an Ein program in a vocabulary of the analysis's own, the vocabulary's declarations at its
top, each fact quoting its sentence as its `:source`, and `(solve :no-search)` last. The Zebra puzzle's seventh
sentence under SPRING, with the named entities beside, in each:

<!-- output -->
```text
# ::snt The Old Gold smoker owns snails.
# named entities: Old Gold (PRODUCT)
(z1 / own-01
      :ARG0 (z2 / person
            :ARG0-of (z3 / smoke-02
                  :ARG1 (z4 / gold
                        :mod (z5 / old))))
      :ARG1 (z6 / snail))
```

<!-- output -->
```text
Sentence 7: The Old Gold smoker owns snails.
  named entities: Old Gold (PRODUCT)
  z1 own-01 :ARG0 z2 :ARG1 z6
  z2 person
  z3 smoke-02 :ARG0 z2 :ARG1 z4
  z4 gold :mod z5
  z5 old
  z6 snail
```

```ein
;; 7. The Old Gold smoker owns snails.
(amr-top S7 s7-z1 :source "The Old Gold smoker owns snails.")
(amr-node S7 s7-z1 own-01 :source "The Old Gold smoker owns snails.")
(amr-role s7-z1 ARG0 s7-z2 :source "The Old Gold smoker owns snails.")
(amr-role s7-z1 ARG1 s7-z6 :source "The Old Gold smoker owns snails.")
(amr-node S7 s7-z2 person :source "The Old Gold smoker owns snails.")
(amr-node S7 s7-z3 smoke-02 :source "The Old Gold smoker owns snails.")
(amr-role s7-z3 ARG0 s7-z2 :source "The Old Gold smoker owns snails.")
(amr-role s7-z3 ARG1 s7-z4 :source "The Old Gold smoker owns snails.")
(amr-node S7 s7-z4 gold :source "The Old Gold smoker owns snails.")
(amr-role s7-z4 mod s7-z5 :source "The Old Gold smoker owns snails.")
(amr-node S7 s7-z5 old :source "The Old Gold smoker owns snails.")
(amr-node S7 s7-z6 snail :source "The Old Gold smoker owns snails.")
(named-entity S7 Old_Gold PRODUCT :source "The Old Gold smoker owns snails.")
```

**The vocabulary** is [`vocabulary.v1.ein`](../../einaf/prompts/analysis/vocabulary.v1.ein), EinAf's and no
module of Ein's library: eight relations, each declared with its `:why`.

| AMR | the fact |
|---|---|
| sentence `n` | the name `Sn` |
| the graph's top | `(amr-top Sn sn-z1)` |
| a node `z1 / drink-01` | `(amr-node Sn sn-z1 drink-01)`: a node is named by its sentence's number and the parser's variable |
| `:ARG0 (z2 …)`, and `:ARG0-of` read the other way | `(amr-role sn-z1 ARG0 sn-z2)` |
| `:quant 5`, and every other constant or string | `(amr-const sn-z1 quant 5)` |
| `(z4 / name :op1 "Old" :op2 "Gold")` | `(amr-name sn-z4 Old_Gold)`, beside its `amr-node` |
| `:polarity -` | `(amr-negative sn-z1)` |
| a node's word, where its aligner is learned | `(amr-word sn-z1 drunk)` |
| a named entity | `(named-entity Sn Old_Gold PRODUCT)` |

**What of AMR's spelling Ein's names do not take, and how it is written** (`einaf::analysis::name`): a role's colon,
which begins a keyword in Ein, is dropped (`:ARG0` is `ARG0`); a string, which is no object in Ein, is written as a
name, its words joined by `_` (`"Old" "Gold"` is `Old_Gold`); a character a step's grammar does not let the model
write in a name, anything but `A-Z a-z 0-9 _ -`, is `_` (`"St. Louis"` is `St__Louis`); a word of Ein's grammar, a
declarator, a connective or a position word, is written with `_` after it, so that AMR's conjunction `and` is `and_`;
a name of `_` alone, or of nothing, is `__`. A sense (`drink-01`), a number (`5`), AMR's `-` and its special concepts
(`amr-unknown`, `have-rel-role-91`) are names as they stand.

**A node's words** are the tokens its aligner gave its concept. **A form shows them only where the aligner is
learned**: IBM's, whose every node is aligned, in brackets after the concept in `lines` and as `amr-word` facts; a
rule-based aligner's, which the file keeps and `einaf analysis counts` counts, in no form. `penman` shows none: its
markers are removed.

**The named entities** (`--analysis-entities`) are written under each sentence's line, in `penman` as a `#` line, and
as `named-entity` facts; the note [`entities.v1.md`](../../einaf/prompts/analysis/entities.v1.md) says what the types
are.

**The facts end with `(solve :no-search)`**: the analysis states every fact there is, and the kernel judges root
alone. A program that states no hypothesis mechanism has H written by the blind enumerator over every relation it
declares and every name it writes; the Zebra puzzle's facts with no such line get tens of thousands of candidates at
root, a load of seconds where the line makes it milliseconds, and a drawing of their `open` facts hundreds of times
the analysis's own. With it, `ein solve --no-search` loads the facts with no refusal, root alive, and `ein render kb`
draws them.

## 4. Step 1 given an analysis

**Step 1's request renders [`steps/step1-analysis.v1.md`](../../einaf/prompts/steps/step1-analysis.v1.md)**, which is
[`steps/step1.v1.md`](../../einaf/prompts/steps/step1.v1.md) word for word with one slot more, `analysis`, after
the text: the form, rendered through its template, [`penman.v1.md`](../../einaf/prompts/analysis/penman.v1.md),
[`lines.v1.md`](../../einaf/prompts/analysis/lines.v1.md) or [`facts.v1.md`](../../einaf/prompts/analysis/facts.v1.md),
each a paragraph that says what the form is and that the parser can read a sentence wrong, the note on the named
entities after it where they are beside, and the form in a fenced block. **A template of its own and not a slot
added to step 1's**, so that a run given no analysis renders `steps/step1.v1.md`, under its digest, request for
request as before, and every recorded run replays. Steps 2 and 3 are not given the analysis; their requests are the
same as a run given none, where step 1 wrote the same.

**Refused before any call**: an analysis of other sentences than the text's, a parser the file does not hold, named
entities a file does not hold, an option of the analysis's with no `--analysis` file, and an analysis given to the
whole program in one call (`--mode whole`), which has no step 1. Each is a usage error, exit 2.

## 5. The record

`run.json` (`einaf-steps/1`), and `config.json` of the one shot's record, gain the key `analysis` where one was given
(`einaf::analysis::AnalysisOf`), and a run given none writes no such key:

| key | what |
|---|---|
| `schema`, `file`, `sha256`, `text` | the file's schema, as given, its digest, and the text it is of |
| `parser`, `tool`, `aligner` | the parser whose graphs the prompt held, its tool as the file names it, its aligner |
| `words` | whether the form showed a node's words |
| `form`, `entities` | the form; the named entities' tool, where they were beside |
| `template`, `template_sha256`, `entities_template_sha256`, `vocabulary_sha256` | the form's template and its digest; the note's and the vocabulary's digests, where the prompt held them |
| `block_sha256` | the digest of the block the prompt held |
| `counts` | the parse's `sentences`, `graphs`, `nodes` and the nodes its aligner gave a token, `aligned` |

Step 1's own entry names `steps/step1-analysis.v1.md` and its digest, as every step names its template.

## 6. The commands

```sh
einaf analysis show FILE [--parser spring|xfm|ibm] [--form penman|lines|facts] [--entities] [--block]
einaf analysis counts FILE...
einaf steps run TEXT … [--analysis FILE] [--analysis-parser P] [--analysis-form F] [--analysis-entities]
einaf from-text TEXT … [--analysis FILE] [--analysis-parser P] [--analysis-form F] [--analysis-entities]
```

`show` prints the form alone, for `facts` a program the kernel loads; `--block`, the block as step 1's prompt holds
it. `counts` prints, for each file and each parser, its graphs of its sentences, its nodes, the nodes its aligner
gave a token and those it did not, and the facts of its facts form. `einaf steps run` and `einaf from-text` print a
line that names the analysis, its digest, the parser and the form, above the steps.

<!-- output -->
```text
$ einaf analysis counts einaf/tests/fixtures/analysis/zebra.json
text	parser	aligner	graphs	nodes	aligned	no word	facts
zebra	spring	rule-based	17/17	91	57	34	194
zebra	xfm	rule-based	17/17	90	59	31	190
zebra	ibm	learned	17/17	94	94	0	296
```

## 7. Held by

In EinAf's gate step, with no parser and no model, [`analysis.rs`](../../einaf/tests/analysis.rs):

| test | what it holds |
|---|---|
| `every_recorded_analysis_is_of_its_text_and_every_graph_reads` | each of the six files of its text's sentences; every graph of every parser read; a learned aligner's every node aligned; the three parsers' nodes and aligned nodes over the six texts |
| `the_text_forms_hold_every_sentence_and_no_marker` | `penman`: every sentence, its graph as printed less its markers, no marker left; `lines`: every sentence, a learned aligner's words on every node and a rule-based one's on none; the named entities where asked |
| `the_facts_form_loads_with_no_refusal_and_is_judged_at_root_alone` | every text under every parser, the entities beside, through the bounded call: *not decided* under `--no-search`, *one* under `--single`, no diagnostic, the writer of H `no-search` with no candidate, each `:source` a sentence of the text |
| `ein_render_kb_draws_the_facts` | `ein render kb` on each text's facts, by the binary the gate built |
| `step_1_given_an_analysis_holds_it_after_the_text_and_the_record_names_it` | the Zebra puzzle's steps by hand in each form: step 1's request the plain one with the block after the text and nothing else; every later request the plain run's; the record's `analysis`; none written where none was given |
| `a_run_given_no_analysis_sends_the_step_1_request_it_sent` | step 1 of each recorded run of the model under `einaf/tests/fixtures/` finds its call by its digest, under `steps/step1.v1.md` |
| `an_analysis_that_does_not_fit_the_run_is_refused_before_any_call` | another text's analysis, the whole mode, a parser the file has not, entities it has not |
| `every_template_of_the_analysis_renders_with_its_slots_and_no_other` | the templates' slots; step 1's with the analysis step 1's with the block after the text; the vocabulary parses |
| `einaf_steps_run_and_from_text_take_an_analysis_and_replay_it` | the flags end to end, on the calls of a scripted run given the analysis; the refusals' exit; `einaf analysis show` and `counts` |
