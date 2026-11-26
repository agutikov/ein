# S2.2.2's probe: Ein written with no grammar

**The plan, fixed before the first generation.** This file is the probe's report once it has run; until then it
states what was fixed, and [`plan.jsonl`](plan.jsonl) holds every run whole, its request among it: the prompt's
every message, the sampler chain, the seed, the budget. A changed prompt is a new experiment, so the plan is
committed before any of its runs is made.

`plan.jsonl`: 87 runs, a JSON line each, written by `einaf probe plan` (`einaf::probe::plan`); SHA-256
`d469a581f541a9ecc04a1fa7b5628259c51b629ee10be249fa355bba1edbc34c`. The prompt files it is made from, under
[`einaf/prompts/`](../../../../einaf/prompts/):

| file | SHA-256 | in |
|---|---|---|
| `ein-excerpt.v1.md`, the excerpt of Ein | `72a24095858f7867ca6651d2ab4438fcd0e56b2cbc03a0d426b7c96170bd2556` | every prompt but the model alone's and the described condition's |
| `ein-paragraph.v1.md`, Ein in a paragraph | `99cfe20aff320f160f19832f6bd2cab93128ee8b1ea901a6167ae4029f71640f` | the described condition |
| `word-theory.v1.md`, the word theory's vocabulary | `afc56add926c278fc4c28773961237e7baa01600c2ac49d41d8cec4110308511` | the theory-given condition |
| `step1.v1.md`, the statement of step 1 | `c49fea444aacf8e155ecb306d6bba287f4e2ee438b62f376a99f5c6cbb2eafb2` | step 1 |

## The conditions, in the order they run

| condition | thinking | runs | the prompt | the budget |
|---|---|---:|---|---|
| **the first experiment** (the user's: *"original zebra problem and with working zerba program as example - test for dummies"*) | off, on | 2 | the excerpt; the Zebra text beside `examples/zebra3.1.ein`, the one example; the Zebra text | 16 384; 55 000 |
| **the model alone** | on, off | 2 | the Zebra text, and nothing else | 65 000; 8 192 |
| **a whole program over the library** (the main arms' condition) | off, on | 15 each | the excerpt; the examples of the library condition but the item's family; the text | 8 192; 16 384 |
| **a theory given** (the library ablation's widest; no arm's) | off, on | 13 each | the excerpt and the word theory's vocabulary; the corpus's word problems but the item's family | 8 192; 16 384 |
| **described** (Datalog and Lisp alone) | off | 15 | one paragraph on Ein, no example; the text | 8 192 |
| **step 1 alone** | off, on | 6 each | the excerpt and the statement of step 1; one worked step 1 of another text | 8 192; 16 384 |

Every run is greedy (one sampler, `greedy`, seed 1), in a context of 65 536 tokens, through the binding.

## The items and their families

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

## The readings, and what is measured

An answer is read **whole** (the answer is the program) and **fenced** (its first fenced block). Each reading is
parsed by `einaf::kernel::parse` (every error with its kind and place); a reading that parses is loaded and asked
through the bounded call, `--no-search` and `--single`, each under a clock of 30 seconds; a reading that loads has
its forms read and set against the program it is measured from (`examples/zebra3.1.ein` for the first
experiment, the hand-written step 1 for step 1, the reference program for the small texts and the word problems
under a theory given). Each kind of error is read with what would have prevented it: any grammar, a grammar that
knows the program's names, the catalogue, or nothing a grammar can do.
