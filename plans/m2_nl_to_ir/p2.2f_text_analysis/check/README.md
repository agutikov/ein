# S2.2f.1's check: what trained parsers read right where step 1 went wrong

[S2.2f.1](../s2.2f.1_the_check.md)'s report. Every sentence of the six texts with steps by hand (`apples`, `more`,
`seating`, `together`, `youngest`, the Zebra puzzle: 34 sentences) through three AMR parsers, two Universal
Dependencies parses with their named entities, a coreference resolver across sentences and a word-sense
disambiguator, each alone on the GPU, on this host; then **a table read by hand**: 68 cells, a sentence and a
failure class with a case in it, each judged *right*, *wrong* or *not carried* against [the hand's step
1](../../../../einaf/contract/), with the model's step 1 from P2.2 in two columns beside it. Everything was run on
2026-11-27 on the machine the work runs on, on AC power, the card free of any other model (72 MiB used, by the
desktop). No model of the formalizer runs. The environment, the scripts and how to run them again are
[`../tools/`](../tools/); the cells are [`cells.tsv`](cells.tsv) and their counts [`tally.py`](tally.py)'s; the
parsers' outputs and the drawn graphs are under [`runs/`](runs/) (gitignored, on the host that ran them).

**What it comes to.**

- **Everything installs and runs here, and every tool fits 8 GB with room to spare**: the largest, Stanza with its
  coreference model, peaks at 4 000 MiB; the AMR parsers at 2 074 to 3 918 MiB; spaCy runs on the CPU. Two
  environments: Python 3.12 with torch 2.14.1 on CUDA 13.0 for all but one, and Python 3.8 with torch 1.13.1 on
  CUDA 11.7 for IBM's parser, whose pins no newer Python meets. Nothing failed for good; what failed on the way and
  how it was passed is § 1.
- **On the 68 cells the trained parsers are right far more often than step 1 was**: 50 to 56 cells each for the
  three AMR parsers and the two UD parses, against 41 (43 on the reading alone) for the model in S2.2.2's probe and
  27 (28) of 61 for the model in S2.2.4's steps; and **wrong less often**: 3 to 12 cells against 20 to 23. They read
  what step 1 misread: the numbers in their facts (8 of 8 for every parser, 0 or 2 of 8 for the probe), the
  negations, the passives' roles (SPRING 4 of 4, XFM 3, against 1 and 0), *next to* and the other places with
  their arguments in order, and they state nothing a sentence does not.
- **What they read worse than the model**: a pronoun (*He* is Bob), which no sentence parse resolves and the model
  did; an elided noun (*He gave 2*: 2 apples), which the parsers leave a bare quantity; and *Chesterfields*, the one
  brand the model read right, which every AMR parser makes a place and Stanza a person.
- **Where both are wrong, the parsers are wrong otherwise**: the passive *X is drunk in the house*, which the model
  made active with the drink the drinker, IBM's AMR and both UD parses read as intoxication (XFM on milk once;
  SPRING both right); and **the puzzle's brand names**, which the model split or called smokers and every tool
  misreads somewhere: *Parliaments* a legislature, *Old Gold* old gold, *Kools* a word with no kind or Kool-Aid.
- **Coreference across sentences is the analyses' gap**: Stanza's resolver gets *He* (2 of 2) and links *Kools*
  across sentences, but not *they*, *the Norwegian* or *the green house*; it ties the model on its cells (4 of 5).
  IBM's document parser links every repeated name of the seating and is malformed on the four word problems.
- **Word senses do not pass**: WordNet's senses chosen by a trained T5 give the common nouns the kinds the hand
  writes (a beverage, a dwelling, a canine, a seat), the only tool that groups them so, but read every name as a word:
  *Bob* a former British monetary unit, a racing sled, a haircut; *the fox* Charles James Fox.
- **The verdict by the stage's rule** (a tool goes on if it fits 8 GB and is right on more cells than the model's
  step 1): **XFM, SPRING and IBM's AMR, Stanza's and spaCy's UD parses go on**; the coreference resolver (a tie),
  the word senses and IBM's document parser do not (§ 6).

## 1. The tools

| tool | what it gives | version, model | venv | peak GPU, own processes / card (MiB) | load, parse (s) |
|---|---|---|---|---:|---|
| **XFM** | AMR, a sentence a graph, aligned to words by amrlib's rule-based aligner (RBW) | amrlib 0.8.1, `parse_xfm_bart_large-v0_1_0` (BART-large, AMR 3.0), weights `sha256:d9bdcde6…`; beam 4 | main | 2 098 / 2 176 | 2.2, 23.8 |
| **SPRING** | AMR, as XFM's | amrlib 0.8.1, `parse_spring-v0_1_0` (SPRING's linearization on BART-large), weights `sha256:f0dc1145…`; beam 4 | main | 2 074 / 2 152 | 5.4, 16.8 |
| **IBM** | AMR, every node aligned to its token by the parser itself | transition-neural-parser 0.5.4, `AMR3-structbart-L` (Structured-BART, AMR 3.0), checkpoint `sha256:c4b47b57…`; fairseq's `bart.large`, `model.pt sha256:d2438f46…` | ibm | 3 770 / 3 848 | 27.5, 6.4 |
| IBM, document | AMR of a whole text, coreference as `:same-as` | the same, `doc-sen-conll-amr-seed42`, checkpoint `sha256:b359dce1…` | ibm | 3 918 / 3 996 | 27.1, 7.3 |
| **Stanza** | UD parse (UPOS, features, lemma, head, relation), named entities, **coreference across sentences** | Stanza 1.15.1: tokenize `combined_nocharlm`, pos and depparse `combined_charlm`, ner `ontonotes-ww-multi_charlm`, coref `udcoref_xlm-roberta-lora` (XLM-RoBERTa-large) | main | 4 000 / 4 078 | 10.4, 1.6 |
| **spaCy** | UPOS, its own dependency scheme (ClearNLP's, not UD's), named entities | spaCy 3.8.18, `en_core_web_trf` 3.8.0 (RoBERTa-base); on the CPU, no CuPy | main | 0 / 75 | 2.4, 0.2 |
| **WSD** | a WordNet sense for each noun, verb and adjective, and the sense's hypernyms | `wn` 1.1.1, Open English WordNet 2024; `jpwahle/t5-large-word-sense-disambiguation` at `fa17be59…` (T5-large trained on SemCor) choosing among the glosses | main | 3 098 / 3 176 | 2.0, 19.4 |

The card holds 8 188 MiB; 72 were in use before each tool (the desktop). Times are information only, on this host:
load is the model's, parse the six texts' (XFM's includes the word alignment and the six texts given whole). The
AMR runs were made twice into separate folders: **the outputs are byte for byte the same and so are the peaks**.

**Alignment.** IBM's parser aligns **all 182** of its concept nodes to a token, by construction; amrlib's RBW
aligner, rule-based, aligns **97 of XFM's 183** and **95 of SPRING's 181**. Of the unaligned, most are AMR's own
concepts that no word spells (`name` 26, `person` 25, `amr-unknown` 8, `country` 6 of XFM's 86, a name's words
being aligned on its string constants); the content words among them are few, the two `drink-01` of *is drunk*
(lemmatized *drunk*) among them. A node's sentence is known for every node of every parser, since each graph is
one sentence's.

**What failed on the way, and how it was passed** (every one in [`../tools/`](../tools/), said where it is made):

- **amrlib imports three packages it does not declare** (`unidecode`, `cached-property`, `word2number`): added to
  the requirements.
- **SPRING with transformers 5** (5.24.1): `PENMANBartTokenizer has no attribute encoder`; its tokenizer subclasses
  the slow `BartTokenizer`, which 5 dropped. **Pinned below 5** (4.57.6), for the whole main venv: XFM, Stanza and
  the WSD model run on it alike.
- **SPRING with transformers 4.57**: `assert len(model_out) == len(batch)` in amrlib's `parse_sents`; the model's
  config asks for attentions, this transformers carries that into the generation config with
  `return_dict_in_generate`, and `generate()` returns an object. `run_amrlib.py` switches both off after loading.
- **IBM's parser pins torch 1.13.1, numpy ≤ 1.23.5 and fairseq 0.10.2**, whose last wheels are for Python 3.8: a
  second venv, Python 3.8.20 fetched by uv, torch 1.13.1+cu117 (its sm_86 kernels run on the RTX 4060's sm_89),
  torch-scatter 2.1.1 from PyG's wheels.
- **IBM's parser loads BART through `torch.hub.load('pytorch/fairseq', 'bart.large')`**, which fetches fairseq's
  current GitHub main and builds it from source: `run_ibm.py` points that one call at fairseq's `bart.large`
  archive (3.4 GB), read by the installed fairseq 0.10.2. Its models are Git LFS objects, fetched and checked
  against their oids.
- **Stanza's coreference reads XLM-RoBERTa from the hub**: refused offline on the first run; `fetch_models.py`
  builds the pipeline once, allowed to fetch.
- **The first download's connection reset** at `pypi.nvidia.com`: retried with two downloads at a time.
- **The WSD script looked adjectives up as `a` only**: WordNet files most colour adjectives as satellites, `s`;
  fixed and run again before the table was read.

**Surveyed and not tried.** AMRBART (`xfbai/AMRBART-large-finetuned-AMR3.0-AMRParsing-v2`): a third BART
sequence-to-sequence parser beside XFM and SPRING, where IBM's transition parser is a different kind; not needed
for two independent parsers. AMR parsers built on a language model: the hub's are LoRA adapters on LLaMA-7B and
Flan-T5-XXL (14 and 22 GB in half precision, and the first's base is gated) and RL checkpoints of a Vietnamese AMR
project; none fits or is English. UDPipe: Stanza and spaCy cover the UD layer. Other WSD systems (ConSeC, ESC,
EWISER): their checkpoints are behind Google Drive links and their stacks older than IBM's.

## 2. The counts

A cell is a sentence and a failure class with a case in it ([the stage's table of
classes](../s2.2f.1_the_check.md#the-failure-classes)); the cells and the model's two columns were fixed **before
any parser's output was read**. *Right* means the analysis carries what the hand's step 1 writes, as the class
defines it; *not carried*, that it says nothing of it (for a kind, also a kind that is no kind: `Thing`, a word as
its own concept). The rules of judgement:

- **M1** is the model's step 1 in S2.2.2's probe (thinking off, no grammar); **M2** in S2.2.4's steps (under step
  1's grammar; `together` was not among its texts, `bob1` is `apples`' text). A model's fault that is Ein's writing
  rather than the reading (a number written as a name, `(n z 12)`) is counted twice: wrong as the class defines it,
  right *on the reading alone*.
- **A kind**: for the model, a membership or a declaration's label; for an AMR parser, the concept of a common noun
  (`dog`, `chair`) or the concept above a `:name` (`person`, `product`); for a UD parse, the noun's lemma or the
  entity type of a name. A brand read as a company or an organisation is judged wrong (nothing is smoked); as a
  product, right.
- **A tool speaks to the cells of its layer**: AMR and the UD parses to all; the coreference resolver to the
  coreference cells, the word senses to the kind cells, each `-` (counted as not carried) elsewhere.
- **The stage's rule** compares a tool's right cells with the model's on the same cells; it is run against both
  model columns, and against M1 on the reading alone.

68 cells. Each entry: right / wrong / not carried.

| class | cells | M1 | M1 reading | M2 | M2 reading | xfm | spring | ibm | stanza | spacy | coref | wsd |
|---|---:|---|---|---|---|---|---|---|---|---|---|---|
| name | 2 | 0 / 2 / 0 | 0 / 2 / 0 | 2 / 0 / 0 | 2 / 0 / 0 | 1 / 1 / 0 | 1 / 1 / 0 | 1 / 1 / 0 | 2 / 0 / 0 | 2 / 0 / 0 | 0 / 0 / 2 | 0 / 0 / 2 |
| voice | 4 | 1 / 3 / 0 | 1 / 3 / 0 | 0 / 4 / 0 | 0 / 4 / 0 | 3 / 1 / 0 | 4 / 0 / 0 | 2 / 2 / 0 | 2 / 2 / 0 | 2 / 2 / 0 | 0 / 0 / 4 | 0 / 0 / 4 |
| negation | 2 | 2 / 0 / 0 | 2 / 0 / 0 | 0 / 2 / 0 | 0 / 2 / 0 | 2 / 0 / 0 | 2 / 0 / 0 | 2 / 0 / 0 | 2 / 0 / 0 | 2 / 0 / 0 | 0 / 0 / 2 | 0 / 0 / 2 |
| kind | 26 | 18 / 7 / 1 | 18 / 7 / 1 | 7 / 6 / 11 (2 n/a) | 7 / 6 / 11 (2 n/a) | 18 / 4 / 4 | 20 / 4 / 2 | 16 / 8 / 2 | 18 / 4 / 4 | 20 / 1 / 5 | 0 / 0 / 26 | 8 / 9 / 9 |
| number | 8 | 0 / 7 / 1 | 2 / 5 / 1 | 4 / 2 / 0 (2 n/a) | 5 / 1 / 0 (2 n/a) | 8 / 0 / 0 | 8 / 0 / 0 | 8 / 0 / 0 | 8 / 0 / 0 | 8 / 0 / 0 | 0 / 0 / 8 | 0 / 0 / 8 |
| unstated | 1 | 0 / 1 / 0 | 0 / 1 / 0 | 1 / 0 / 0 | 1 / 0 / 0 | 1 / 0 / 0 | 1 / 0 / 0 | 1 / 0 / 0 | 1 / 0 / 0 | 1 / 0 / 0 | 0 / 0 / 1 | 0 / 0 / 1 |
| coreference | 5 | 4 / 0 / 1 | 4 / 0 / 1 | 2 / 1 / 0 (2 n/a) | 2 / 1 / 0 (2 n/a) | 2 / 0 / 3 | 2 / 0 / 3 | 1 / 1 / 3 | 2 / 0 / 3 | 2 / 0 / 3 | 4 / 0 / 1 | 0 / 0 / 5 |
| order | 13 | 10 / 1 / 2 | 10 / 1 / 2 | 7 / 6 / 0 | 7 / 6 / 0 | 12 / 0 / 1 | 11 / 1 / 1 | 12 / 0 / 1 | 12 / 0 / 1 | 12 / 0 / 1 | 0 / 0 / 13 | 0 / 0 / 13 |
| question | 7 | 6 / 1 / 0 | 6 / 1 / 0 | 4 / 2 / 0 (1 n/a) | 4 / 2 / 0 (1 n/a) | 6 / 1 / 0 | 6 / 1 / 0 | 7 / 0 / 0 | 7 / 0 / 0 | 7 / 0 / 0 | 0 / 0 / 7 | 0 / 0 / 7 |
| **all** | 68 | 41 / 22 / 5 | 43 / 20 / 5 | 27 / 23 / 11 (7 n/a) | 28 / 22 / 11 (7 n/a) | 53 / 7 / 8 | 55 / 7 / 6 | 50 / 12 / 6 | 54 / 6 / 8 | 56 / 3 / 9 | 4 / 0 / 64 | 8 / 9 / 51 |

Each tool against the model on the cells its layer speaks to (right cells; the model's on the same cells):

| tool | cells | tool right | tool wrong | M1 right (reading) | M2 right (reading) |
|---|---:|---:|---:|---:|---:|
| xfm | 68 | 53 | 7 | 41 (43) | 27 (28) |
| spring | 68 | 55 | 7 | 41 (43) | 27 (28) |
| ibm | 68 | 50 | 12 | 41 (43) | 27 (28) |
| stanza | 68 | 54 | 6 | 41 (43) | 27 (28) |
| spacy | 68 | 56 | 3 | 41 (43) | 27 (28) |
| coref | 5 | 4 | 0 | 4 (4) | 2 (2) |
| wsd | 26 | 8 | 9 | 18 (18) | 7 (7) |

Cell by cell against M1 on the reading alone (the cells where one is right and the other is not):

| tool | tool right, M1 not | M1 right, tool not | of which the tool wrong |
|---|---|---|---|
| xfm | 16: a1 a3 s2 s4 s5 t1 t3 t6 z1 z2 z5 z8 z12 z16 z18 z26 | 6: a4 a5 m5 t4 t5 z21 | 2: m5 z21 |
| spring | 19: a1 a3 s2 s5 t1 t3 t6 z1 z2 z5 z8 z12 z13 z14 z16 z18 z25 z26 z27 | 7: a4 a5 m5 t4 t5 z9 z21 | 3: m5 z9 z21 |
| ibm | 14: a1 a3 s2 s4 s5 t1 t3 t6 z1 z2 z8 z12 z18 z26 | 7: a4 a5 t4 t5 z6 z21 z24 | 3: z6 z21 z24 |
| stanza | 17: a1 a3 s2 s4 s5 t1 t3 t6 z1 z2 z8 z10 z12 z16 z18 z26 z27 | 6: a4 a5 t4 t5 z7 z21 | 2: z7 z21 |
| spacy | 18: a1 a3 s2 s4 s5 t1 t3 t6 z1 z2 z8 z10 z11 z12 z16 z18 z26 z27 | 5: a4 a5 t4 t5 z4 | 0:  |
| coref | 0:  | 0:  | 0:  |
| wsd | 2: z2 z16 | 12: a2 a5 m2 m4 s1 t2 t5 y2 y4 z7 z9 z21 | 6: a2 m2 s1 t2 y2 z21 |

**What the counts do not say.** A tool's right cell is the reading present in its analysis, not a formalization
made from it: whether step 1 given the analysis writes the facts right is [S2.2f.3](../s2.2f.3_step1_with_the_analysis.md)'s.
Of the 68 cells, 26 are kinds and 13 orders; the eight number cells and the one *unstated* cell are ones a
sentence parser gets right almost by construction (it attaches a number to its noun, and reads one sentence at a
time). The cells where a tool and the model part are the last table's, cell by cell.

## 3. The cells

Each text's cells, the sentence in italics before the note; a note that says *judged* marks a cell that is a
judgement, and why.

#### `apples`

| id | s | class | the hand's step 1 | M1 | M2 | xfm | spring | ibm | stanza | spacy | coref | wsd | the sentence; note |
|---|---:|---|---|---|---|---|---|---|---|---|---|---|---|
| a1 | 1 | number | (has Bob (n z 5) Apple Start) | W | R | R | R | R | R | R | - | - | *Bob has 5 apples.* M1: (has Bob apples), 5 a membership (instance 5 Number), out of its fact. M2: (has Bob (n z 5) Apple). |
| a2 | 1 | kind | Bob a person; the five are apples | R | R | R | R | R | R | R | - | W | *Bob has 5 apples.* M1: (person Bob), apples the thing had. M2: Apple, Bob a Person by the declaration's label. WSD: Bob a former British monetary unit. |
| a3 | 2 | number | (gave Bob (n z 2) Apple Alice …) | W | R | R | R | R | R | R | - | - | *He gave 2 to Alice.* M1: (gave Bob Alice apples), 2 a membership. M2: (gave Bob (n z 2) Alice). |
| a4 | 2 | coreference | He is Bob | R | R | N | N | N | N | N | R | - | *He gave 2 to Alice.* Tools: every AMR keeps (h / he); XFM given the whole text still writes he. Stanza's coreference: He -> Bob. |
| a5 | 2 | kind | the two are apples (elided); Alice a person | R | N | N | N | N | N | N | - | N | *He gave 2 to Alice.* M1: apples in the fact, (person Alice). M2: (gave Bob (n z 2) Alice), the 2's kind dropped. Tools: AMR (t / thing :quant 2), IBM a bare 2; XFM given the whole text writes (a2 / apple :quant 2). WSD: Alice not in WordNet. |
| a6 | 3 | question | Bob's count of apples, after the giving | R | R | R | R | R | R | R | - | - | *How many apples does Bob have?* Both ask Bob's count (M1 over `amount`, no fact of it). Neither has a moment: time is none of the classes (L-M2.18's moment left out). |

#### `more`

| id | s | class | the hand's step 1 | M1 | M2 | xfm | spring | ibm | stanza | spacy | coref | wsd | the sentence; note |
|---|---:|---|---|---|---|---|---|---|---|---|---|---|---|
| m1 | 1 | number | (has Bob (n z 5) Apple Now) | W\|R | R | R | R | R | R | R | - | - | *Bob has 5 apples.* M1: (has Bob apples 5), the value in its fact, a name. |
| m2 | 1 | kind | Bob a person; the five are apples | R | R | R | R | R | R | R | - | W | *Bob has 5 apples.* WSD: Bob a monetary unit. |
| m3 | 2 | number | (has Alice (n (n z 1) 2) Apple Now) | W\|R | W\|R | R | R | R | R | R | - | - | *Alice has 12 apples.* M1: (has Alice apples 12), a name. M2: (has Alice (n z 12) Apple), 12 as one digit, a numeral no digit table has. |
| m4 | 2 | kind | Alice a person; the twelve are apples | R | R | R | R | R | R | R | - | N | *Alice has 12 apples.*  |
| m5 | 3 | question | two asks: who has more, and the difference | R | W | W | W | R | R | R | - | - | *Who has more apples, and by how many?* M1: (and (more ?who apples ?diff) (diff-of 5 12 ?diff)), who and by how many. M2: (and (has ?who-more (n ?diff) Apple) (gt (n ?who-more 1) (n ?who-less 1))): the difference read as a count of apples, the people put inside numerals. XFM: and(have-03 :ARG0 amr-unknown :ARG1 apple :mod more; have-quant-91 :ARG2 amr-unknown :ARG3 equal): the second ask an amount 'equal', not the difference (judgement). SPRING: no unknown for 'how many'. IBM: apple :quant more :quant amr-unknown, both asks. Stanza, spaCy: 'by how many' kept in the parse, attached to the verb or the noun (judged right). |
| m6 | 3 | order | more … than: who has more than whom | N | W | N | N | N | N | N | - | - | *Who has more apples, and by how many?* M1: (more ?who apples ?diff), no one it is more than. M2: gt over numerals made of the people's names. Tools: 'more apples', no one it is more than (as M1). |

#### `seating`

| id | s | class | the hand's step 1 | M1 | M2 | xfm | spring | ibm | stanza | spacy | coref | wsd | the sentence; note |
|---|---:|---|---|---|---|---|---|---|---|---|---|---|---|
| s1 | 1 | kind | Ann, Bob, Cy persons; First, Middle, Third chairs | R | W | R | R | R | R | R | - | W | *Ann, Bob and Cy sit in a row of three chairs.* M1: the people (person …), the chairs Place by sits-in's label (a kind apart from the people's; judged right). M2: (instance Ann Chair) … beside (instance First Chair): people as chairs (L-M2.18). WSD: Bob a racing sled; Ann, Cy not in WordNet. |
| s2 | 1 | number | three chairs | N | W | R | R | R | R | R | - | - | *Ann, Bob and Cy sit in a row of three chairs.* M1: no fact of the chairs, three places used. M2: six members of Chair, the three people among them. |
| s3 | 1 | order | a row: the chairs by their places, first, middle, third | R | R | R | R | R | R | R | - | - | *Ann, Bob and Cy sit in a row of three chairs.* M1: First, Second, Third (relations first/second/third declared). M2: First, Middle, Third. Tools: (r / row :consist-of (c / chair :quant 3)); the places are sentences 3 and 4's (judged right, as the models'). |
| s4 | 2 | order | (sits-next-to Ann Bob) | N | W | R | W | R | R | R | - | - | *Ann sits next to Bob.* M1: the sentence not stated (three placements in its place). M2: (sits Ann Bob), next to dropped. SPRING: (sit-01 :ARG2 (next-to :op1 Bob)), Ann dropped. |
| s5 | 2 | unstated | nothing but Ann next to Bob | W | R | R | R | R | R | R | - | - | *Ann sits next to Bob.* M1: (sits-in Ann First) (sits-in Bob Second) (sits-in Cy Third), each quoting this sentence. |
| s6 | 3 | negation | (not (sits-in Cy First)) | R | W | R | R | R | R | R | - | - | *Cy does not sit in the first chair.* M2: (sits Cy First) (L-M2.18). |
| s7 | 3 | order | the first chair: First | R | R | R | R | R | R | R | - | - | *Cy does not sit in the first chair.*  |
| s8 | 4 | negation | (not (sits-in Bob Middle)) | R | W | R | R | R | R | R | - | - | *Bob does not sit in the middle chair.* M2: (sits Bob Middle) (L-M2.18). |
| s9 | 4 | order | the middle chair: Middle | R | R | R | R | R | R | R | - | - | *Bob does not sit in the middle chair.* M1: Second, the middle of three (judged right: the place, named by its rank). |
| s10 | 5 | question | (sits-in ?who First) | R | R | R | R | R | R | R | - | - | *Who sits in the first chair?*  |

#### `together`

| id | s | class | the hand's step 1 | M1 | M2 | xfm | spring | ibm | stanza | spacy | coref | wsd | the sentence; note |
|---|---:|---|---|---|---|---|---|---|---|---|---|---|---|
| t1 | 1 | number | (has Bob (n z 5) Apple Start) | W | n/a | R | R | R | R | R | - | - | *Bob has 5 apples.* M1: (has Bob apples), no number anywhere. |
| t2 | 1 | kind | Bob a person; the five are apples | R | n/a | R | R | R | R | R | - | W | *Bob has 5 apples.*  |
| t3 | 2 | number | (gave Bob (n z 2) Apple Alice …) | W | n/a | R | R | R | R | R | - | - | *He gave 2 to Alice.* M1: (gave Bob Alice apples). |
| t4 | 2 | coreference | He is Bob | R | n/a | N | N | N | N | N | R | - | *He gave 2 to Alice.*  |
| t5 | 2 | kind | the two are apples (elided); Alice a person | R | n/a | N | N | N | N | N | - | N | *He gave 2 to Alice.*  |
| t6 | 3 | question | the count Bob and Alice have together, after | W | n/a | R | R | R | R | R | - | - | *How many apples do they have together?* M1: (total-apples ?who ?amount): asks a who the question does not ask. |
| t7 | 3 | coreference | they are Bob and Alice | N | n/a | N | N | N | N | N | N | - | *How many apples do they have together?* M1: they left an unknown, ?who. Tools: they unresolved; Stanza's coreference gives they a chain of its own. |

#### `youngest`

| id | s | class | the hand's step 1 | M1 | M2 | xfm | spring | ibm | stanza | spacy | coref | wsd | the sentence; note |
|---|---:|---|---|---|---|---|---|---|---|---|---|---|---|
| y1 | 1 | order | (older Ann Bob) | R | R | R | R | R | R | R | - | - | *Ann is older than Bob.* M2: (is-older-than Ann Bob). |
| y2 | 1 | kind | Ann, Bob persons | R | R | R | R | R | R | R | - | W | *Ann is older than Bob.* By the declarations' labels, Person; neither writes the hand's memberships (S2.2.2's record counts them missing). WSD: Bob a monetary unit. |
| y3 | 2 | order | (older Bob Cy) | R | R | R | R | R | R | R | - | - | *Bob is older than Cy.*  |
| y4 | 2 | kind | Cy a person | R | R | R | R | R | R | R | - | N | *Bob is older than Cy.* As y2. |
| y5 | 3 | question | (youngest ?who) | R | R | R | R | R | R | R | - | - | *Who is the youngest?* M2: (is-youngest ?who). |

#### `zebra`

| id | s | class | the hand's step 1 | M1 | M2 | xfm | spring | ibm | stanza | spacy | coref | wsd | the sentence; note |
|---|---:|---|---|---|---|---|---|---|---|---|---|---|---|
| z1 | 1 | number | five houses: five things of kind House | W | R | R | R | R | R | R | - | - | *There are five houses.* M1: (five (count 5)), the number a relation and a count of nothing. M2: House1 … House5. |
| z2 | 1 | kind | the five are houses | N | R | R | R | R | R | R | - | R | *There are five houses.* M1: (relation house Place) declared, no house. |
| z3 | 2 | kind | the Englishman a man (of a nationality); red a colour, of the house | R | W | R | R | R | R | R | - | R | *The Englishman lives in the red house.* M1: (instance Englishman person), (instance red color). M2: RedHouse a House: a description made a thing (L-M2.24's first defect). |
| z4 | 3 | kind | the Spaniard a man; the dog a pet | R | N | R | R | R | R | N | - | R | *The Spaniard owns the dog.* M1: person, animal. M2: Dog a Thing (no kind; L-M2.24's third defect). spaCy: Spaniard a PROPN with no entity type. |
| z5 | 4 | voice | coffee is what is drunk, in the green house | W | W | R | R | W | W | W | - | - | *Coffee is drunk in the green house.* M1: (drinks Coffee green), M2: (drinks Coffee GreenHouse): coffee the drinker. IBM: (drunk :domain (person :name Coffee) :location (house :name Green House)); Stanza, spaCy: drunk an adjective, coffee its subject (intoxicated). |
| z6 | 4 | kind | coffee a drink; green a colour, of the house | R | W | R | R | W | R | R | - | R | *Coffee is drunk in the green house.* M2: Coffee a Thing, GreenHouse a House. IBM: Coffee a person, Green House a named house. |
| z7 | 5 | kind | the Ukrainian a man; tea a drink | R | N | R | R | R | W | R | - | N | *The Ukrainian drinks tea.* M2: Tea a Thing. Stanza: a noun phrase with no verb (Ukrainian ADJ, drinks NOUN compound, tea root). WSD: Ukrainian the adjective (of Ukraine). |
| z8 | 6 | order | green immediately right of ivory, in that order | W | R | R | R | R | R | R | - | - | *The green house is immediately to the right of the ivory house.* M1: (right-of green ivory), its declaration's :why "{?4} is immediately to the right of {?3}": reversed by its own text (judgement). SPRING: right-04 (PropBank: be located next to the right side), :ARG1 green, :ARG2 ivory. XFM, IBM: be-located-at-91 / relative-position :direction right. |
| z9 | 6 | kind | green, ivory colours, of houses | R | W | R | W | R | R | R | - | N | *The green house is immediately to the right of the ivory house.* M2: GreenHouse, IvoryHouse Houses. SPRING: green-03 (PropBank: compatible with a healthy environment). WSD: ivory not in WordNet as an adjective. |
| z10 | 7 | name | Old Gold one name | W | R | W | W | W | R | R | - | - | *The Old Gold smoker owns snails.* M1: (owns Old Gold snails), two arguments. M2: OldGold. XFM: (gold :mod old); SPRING: smoke-02 :ARG1 (gold :mod old); IBM: smoke-02 :ARG1 gold, :rel Old. Stanza: ORG span Old Gold; spaCy: PRODUCT span. |
| z11 | 7 | kind | the smoker a man, Old Gold a cigarette, snails a pet | W | N | W | W | W | W | R | - | W | *The Old Gold smoker owns snails.* M1: (instance Old Gold smoker), a fact of three places. M2: OldGold, Snails Things. Stanza: Old Gold an ORG (judged wrong, as a company is). WSD: Gold coins made of gold. |
| z12 | 8 | voice | Kools are what is smoked, in the yellow house | W | W | R | R | R | R | R | - | - | *Kools are smoked in the yellow house.* M1: (smokes Kools yellow), M2: (smokes Kools YellowHouse): Kools the smoker. IBM: smoke-01 (PropBank: emit smoke, inhaling), not smoke-02 (smoke cigarettes); the roles right. |
| z13 | 8 | kind | Kools a cigarette; yellow a colour, of the house | W | W | N | R | W | N | N | - | N | *Kools are smoked in the yellow house.* M1: (instance Kools smoker). M2: Kools a Thing, YellowHouse a House. XFM: (k / kool), the word as its own concept (no kind). SPRING: (product :name Kools). IBM: (product :name Kool-Aid), a drink's brand. Stanza, spaCy: no entity. WSD: not in WordNet. |
| z14 | 9 | voice | milk is what is drunk, in the middle house | W | W | W | R | W | W | W | - | - | *Milk is drunk in the middle house.* M1: (drinks Milk middle), M2: (drinks Milk MiddleHouse). XFM: (drink-01 :ARG0 milk), milk the drinker. IBM: (drunk :domain milk-02), milk-02 'exploit'. Stanza, spaCy: drunk an adjective. |
| z15 | 9 | order | the middle house: the middle place of the five | R | W | R | R | R | R | R | - | - | *Milk is drunk in the middle house.* M1: middle. M2: MiddleHouse, a house beside the five (L-M2.24's first defect). |
| z16 | 9 | kind | milk a drink; the middle house a house | W | N | R | R | W | R | R | - | R | *Milk is drunk in the middle house.* M1: (instance middle number). M2: Milk a Thing. IBM: milk-02. WSD: milk a dairy product, food (judged right). |
| z17 | 10 | order | the first house: the first place | R | R | R | R | R | R | R | - | - | *The Norwegian lives in the first house.* M1: first. M2: House1. |
| z18 | 10 | kind | the Norwegian a man; the first house a house | W | R | R | R | R | R | R | - | N | *The Norwegian lives in the first house.* M1: (instance first number). WSD: Norwegian the adjective. |
| z19 | 11 | coreference | who is the man; the man with the fox another man | R | W | R | R | R | R | R | R | - | *The man who smokes Chesterfields lives in the house next to the man with the fox.* M1: (next-to Chesterfields fox), each man by what he has. M2: (lives-in ChesterfieldsHouse FoxOwner): the first man made a house. Tools: who the man (AMR's :ARG0-of, UD's acl:relcl); the man with the fox a second man. Stanza's UD attaches the fox to next, not to the man (outside this cell). Coreference: one mention each. |
| z20 | 11 | order | the two men's houses next to each other | R | W | R | R | R | R | R | - | - | *The man who smokes Chesterfields lives in the house next to the man with the fox.* M2: lives-in, next to dropped (L-M2.24's fourth defect). IBM: (house :location-of (next-to :op1 man)), the inverse role on a symmetric relation (judged right). |
| z21 | 11 | kind | Chesterfields a cigarette (or its smoker); the fox a pet | R | N | W | W | W | W | R | - | W | *The man who smokes Chesterfields lives in the house next to the man with the fox.* M1: (instance Chesterfields smoker), its name standing for the man as it does in its fact (judged right); fox an animal. M2: Things. XFM: Chesterfield a road; SPRING: smoke-02 :location (road :name Chesterfields); IBM: a local-region; Stanza: a PERSON; spaCy: a PRODUCT. WSD: Chesterfield the statesman, and the fox Charles James Fox, the statesman. |
| z22 | 12 | voice | Kools are what is smoked; the horse what is kept | R | W | R | R | R | R | R | - | - | *Kools are smoked in the house next to the house where the horse is kept.* M1: (next-to Kools horse), no voice to get wrong (judged right). M2: (smokes Kools HorseKeeperHouse): Kools the smoker. IBM: smoke-01 again. |
| z23 | 12 | order | the two houses next to each other | R | W | R | R | R | R | R | - | - | *Kools are smoked in the house next to the house where the horse is kept.* M1: next-to. M2: next to dropped. |
| z24 | 12 | coreference | where is the house; the horse's house | R | R | R | R | W | R | R | R | - | *Kools are smoked in the house next to the house where the horse is kept.* M2: HorseKeeperHouse, the right house under a name of its own. IBM: keep-01 placed on the house where Kools are smoked, not on the house next to it. |
| z25 | 12 | kind | Kools a cigarette; the horse a pet | W | N | N | R | W | N | N | - | N | *Kools are smoked in the house next to the house where the horse is kept.* M1: (instance Kools smoker). Kools: as z13. |
| z26 | 13 | name | Lucky Strike one name; orange juice one name | W | R | R | R | R | R | R | - | - | *The Lucky Strike smoker drinks orange juice.* M1: (drinks Lucky Strike orange). M2: LuckyStrike, OrangeJuice. |
| z27 | 13 | kind | the smoker a man, Lucky Strike a cigarette; orange juice a drink | W | N | W | R | W | R | R | - | W | *The Lucky Strike smoker drinks orange juice.* M1: (instance Lucky Strike smoker), three places; (instance orange drink). XFM, IBM: Lucky Strike a company (judged wrong: the brand, nothing smoked by XFM; IBM's man smokes the company). SPRING: a product; Stanza, spaCy: a PRODUCT. WSD: Strike a labour strike. |
| z28 | 14 | kind | the Japanese a man; Parliaments cigarettes | W | N | W | W | W | W | W | - | W | *The Japanese smokes Parliaments.* M1: (instance Parliaments smoker), the thing smoked a smoker. AMR: (parliament), the legislature; Stanza, spaCy: Parliaments an ORG; WSD: a legislative assembly. |
| z29 | 15 | order | the Norwegian next to the blue house | R | W | R | R | R | R | R | - | - | *The Norwegian lives next to the blue house.* M1: (next-to Norwegian blue). M2: (lives-in Norwegian BlueHouse), next to dropped. |
| z30 | 15 | kind | blue a colour, of the house | R | W | R | R | R | R | R | - | R | *The Norwegian lives next to the blue house.* M2: BlueHouse a House. |
| z31 | 16 | question | who drinks water | R | R | R | R | R | R | R | - | - | *Who drinks water?*  |
| z32 | 16 | kind | water a drink | R | N | R | R | R | R | R | - | R | *Who drinks water?* WSD: water the compound H2O (the liquid; its hypernyms say compound, not drink; judged right as the noun's concept). |
| z33 | 17 | question | who owns the zebra, another unknown | R | W | R | R | R | R | R | - | - | *Who owns the zebra?* M1: a query of its own. M2: the same ?who as water's (L-M2.21). |
| z34 | 17 | kind | the zebra a pet | R | N | R | R | R | R | R | - | R | *Who owns the zebra?*  |

## 4. Each text: what the tools got that the model did not, and the reverse

- **`apples`, `together`, `more`.** Every parser keeps each number in its fact with its noun (`apple :quant 5`),
  which the probe's step 1 did not on any of the three texts; and reads the questions as AMR's `amr-unknown` on the
  count. None resolves *He* in a sentence's parse, which the model did; Stanza's resolver does. *He gave 2 to
  Alice* leaves the 2's kind unsaid in every sentence parse (`thing :quant 2`); XFM and SPRING given the whole text
  write `apple :quant 2`, and still `he`. *by how many* is the parsers' hardest: XFM writes an amount `:ARG3 equal`,
  SPRING drops it, IBM keeps it (`apple :quant more :quant amr-unknown`); no tool says *than whom*, nor did the model.
- **`seating`.** Every tool states the two denials (`:polarity -`, UD's `Polarity=Neg`), where M2 wrote two
  statements (L-M2.18), and none invents a placement, where M1 wrote three; the chairs by their places (`ord 1`,
  `middle`) and the people as persons, where M2 made the people chairs. **SPRING drops Ann** from *Ann sits next to
  Bob* (`sit-01 :ARG2 (next-to :op1 Bob)`), the one wrong cell of this text.
- **`youngest`.** The model and every tool right on every cell: *older than* as `have-degree-91` with its two people
  in order and `more`, *the youngest* as `young` with `most` and `amr-unknown`.
- **The Zebra puzzle.** The tools read right what the model misread most: *five houses* (`house :quant 5`), the
  colours as properties of the houses and not houses of their own (L-M2.24's first defect), the three *next to*
  sentences with both their sides (its fourth), the passive of *Kools are smoked* and *the horse is kept*, and two
  questions with an unknown each (L-M2.21). Where the model was wrong they are often wrong otherwise: *Coffee is
  drunk* and *Milk is drunk*, which the model made active with the drink the drinker, IBM's AMR and both UD parses
  read as intoxication, XFM *milk* the drinker (SPRING both right); *Parliaments* every tool the legislature or an
  organisation, where the model called it a smoker; *Old Gold* every AMR parser gold and Stanza an organisation,
  where the model split it; *Kools* XFM a word and IBM Kool-Aid. The one brand the model read right,
  *Chesterfields*, every AMR parser makes a place and Stanza a person. spaCy's entities are the only analysis that
  gets *Old Gold*, *Chesterfields* and *Lucky Strike* all as products; SPRING gets *Kools* and *Lucky Strike*.

## 5. What the tools read wrong outside the cells

The cells hold the classes; these misreadings are in no class, and are counted nowhere but here.

- **Stanza parses *The Ukrainian drinks tea.* as a noun phrase with no verb** (*Ukrainian* an adjective, *drinks* a
  noun, *tea* the root); the word senses, reading Stanza's tags, take *drinks* as *the act of drinking alcoholic
  beverages to excess*.
- **Stanza attaches *with the fox* to *next***, not to the second man.
- **IBM renames *Kools* to *Kool-Aid*** (its name, not only its kind), and makes *Green House* a name.
- **IBM writes `smoke-01`** (PropBank: *emit smoke, inhaling*) for *Kools are smoked*, where XFM and SPRING write
  `smoke-02` (*smoke cigarettes*), and `milk-02` (*exploit*) for *milk*. **SPRING's *green* in sentence 6 is
  `green-03`** (*compatible with a healthy environment*), its `right-04` (*be located next to the right side*) right,
  and *immediately* a `:time`. The senses were read from PropBank's frame files, not assumed.
- **XFM reads *the Lucky Strike smoker* and *the Old Gold smoker* with nothing smoked** (`smoke-02` with no
  `:ARG1`, the brand a `:mod` of the man: a company, and gold); IBM has the one smoke a company and the other gold,
  SPRING the other gold.
- **The whole text as one input**: XFM's graph of the Zebra puzzle holds **8 of its 17 sentences**, SPRING's 13;
  on the short texts both keep every sentence and write *2* as 2 apples, and XFM makes *Bob* of the question the
  first sentence's Bob (one node reached twice), SPRING not.
- **IBM's document parser** is malformed on the four word problems (spurious `include-91`, `ask-01` and empty
  frames, sentences nested in sentences; in `together` a person named with three words none of which the text
  holds together, `they` among them) and writes no `:same-as` on them, not even *Bob* in `youngest`; on the seating
  its five `:same-as` are all right (Ann, Bob twice, Cy, the first chair); on the Zebra puzzle one of its three is
  right (*the green house* of sentences 4 and 6), and it writes *Kools* as `you` and `no` and *Coffee* as
  `caffeine`.
- **The word senses read every name as a word** where WordNet has one: *Bob* a former British monetary unit, a
  racing sled or a haircut (by sentence), *Strike* a labour strike, *Gold* coins, *Chesterfield* the statesman, *the
  fox* the statesman Charles James Fox; *Ann*, *Cy*, *Alice*, *Kools* and *ivory* are not in it.
- **spaCy gives *Spaniard* no entity type**, and both UD parses read *Parliaments* as an organisation.

## 6. The verdict per tool

By the stage's default rule: it goes on if it fits 8 GB and is right on more cells of the table than the model's
step 1 was (§ 2's second table; the best of the model's columns on the reading alone, M1's 43 of 68, M2's 28 of 61).

| tool | fits 8 GB | right cells, tool / M1 (reading) / M2 (reading) | wrong cells, tool / M1 / M2 | goes on |
|---|---|---|---|---|
| XFM | yes, 2.1 GB | 53 / 43 / 28 | 7 / 20 / 22 | **yes** |
| SPRING | yes, 2.1 GB | 55 / 43 / 28 | 7 / 20 / 22 | **yes**, the most right cells of the AMR parsers |
| IBM | yes, 3.8 GB | 50 / 43 / 28 | 12 / 20 / 22 | **yes**, by the rule; the most wrong cells of any parser, all on the Zebra puzzle |
| Stanza (UD and entities) | yes, 4.0 GB with its coreference | 54 / 43 / 28 | 6 / 20 / 22 | **yes** |
| spaCy (UPOS, dependencies, entities) | yes, on the CPU | 56 / 43 / 28 | 3 / 20 / 22 | **yes**, the fewest wrong cells |
| Stanza's coreference | yes (in Stanza's 4.0 GB) | 4 / 4 / 2 of 5 | 0 / 0 / 1 | **no**: a tie with M1 |
| WSD (WordNet and T5) | yes, 3.1 GB | 8 / 18 / 7 of 26 | 9 / 7 / 6 | **no** |
| IBM's document parser | yes, 3.9 GB | (no column: malformed on 4 of the 6 texts, § 5) | | **no** |

**So the phase goes on** ([the phase's order](../README.md#order)): at least one tool runs here and reads the texts
better than the model did. What S2.2f.2 takes is [the stage's record](../s2.2f.1_the_check.md#what-s22f2-takes).

## 7. How it is taken again

```sh
export EIN_NLP=/home/user/work/ein/build/nlp                   # in a worktree; the default is <checkout>/build/nlp
bash plans/m2_nl_to_ir/p2.2f_text_analysis/tools/setup.sh       # the two venvs and every model, from nothing
bash plans/m2_nl_to_ir/p2.2f_text_analysis/tools/run_all.sh     # every tool, one at a time, under gpumon; the graphs drawn
python3 plans/m2_nl_to_ir/p2.2f_text_analysis/check/tally.py    # § 2's counts from cells.tsv
python3 plans/m2_nl_to_ir/p2.2f_text_analysis/check/tally.py --cells   # § 3's tables
```

The runs (gitignored): [`runs/amr-xfm/`](runs/amr-xfm/index.md), [`runs/amr-spring/`](runs/amr-spring/index.md),
[`runs/amr-ibm/`](runs/amr-ibm/index.md), [`runs/amr-ibm-doc/`](runs/amr-ibm-doc/index.md) (each `index.md` a
sentence's PENMAN and its picture, `svg/`), [`runs/stanza/`](runs/stanza/) (`.conllu`, `.coref.json`, `.ner.json`),
[`runs/spacy-en_core_web_trf/`](runs/spacy-en_core_web_trf/), [`runs/wsd/`](runs/wsd/), [`runs/gpu/`](runs/gpu/)
(each tool's `nvidia-smi` before and its peaks), and [`runs/repeat/`](runs/repeat/), the AMR runs made again.
