# The steps

**A text becomes an Ein program in three calls of the model**, the facts first, then the ontology, then the theory,
as [the contract § 3](04-contract.md#3-the-steps) states them, or in one call, the whole program, a mode kept for
comparison. This page is what EinAf runs: the catalogue the steps select from, the prompts, a run, the contract's
check on each step's program, the record, the commands. The code is
[`einaf::formalize`](../../einaf/src/formalize.rs), [`einaf::catalogue`](../../einaf/src/catalogue.rs),
[`einaf::prompts`](../../einaf/src/prompts.rs) and [`einaf::check`](../../einaf/src/check.rs).

| section | what |
|---|---|
| [1. The catalogue](#1-the-catalogue) | the library's theories a program may select, each an entry program the kernel runs |
| [2. The prompts](#2-the-prompts) | the templates, their slots, the excerpt, the examples, the answer lines |
| [3. A run](#3-a-run) | each step's request, grammar and reading; the assembly; each step's judgement at root; the whole program in one call |
| [4. The contract's check](#4-the-contracts-check) | the seven findings, each from the step that could have avoided it |
| [5. The record](#5-the-record) | what a run writes, and replay |
| [6. The commands](#6-the-commands) | `einaf steps run`, `einaf steps check`, `einaf catalogue` |
| [7. Held by](#7-held-by) | the tests |

## 1. The catalogue

**The theories of Ein's standard library a program may select**, and nothing of a family's own: a family's module
is its canonical program's, and offering one is the widest condition of the library ablation. Steps 2 and 3 are
shown it; step 1 is not.

**Its written part** is a table of entries, [`einaf::catalogue::ENTRIES`](../../einaf/src/catalogue.rs), each an
**entry program** under [`einaf/catalogue/`](../../einaf/catalogue/): the import line with the names it lists, the
facts that select the theory with their arguments, a fact or two and the answer query. Beside it the entry says what
the selection means, what it gives the hypothesis mechanism, the library's rules root fires or owes once it is
selected, the writer of H the kernel reports, the outcome under `--single` and the answer.

| group | entries |
|---|---|
| `std.algebra` | `(transitive R)`, `(symmetric R)`, `(includes R S)`, `(converse R S)`, `(compose R S T)`, `(top R isa Dom Ran)`, `(meet R S T)`, the checks `(functional R)` and `(injective R)`, the obligation `(total-owed R isa)` |
| `std.slots` | `(slot-partition R isa sub Super Index)`, whose obligations write H; `(slot-spatial R S isa Index)` |
| `std.bijection` | `(bijective R)` with `(bijection-hierarchy isa)`, whose obligations write H |
| `std.contradiction` | the import alone: a fact beside its stored negation is a contradiction |
| the calculator | the requests `(add X Y)`, `(sub X Y)`, `(mul X Y)`, `(div X Y)`, `(cmp X Y)`, each stated by a rule and read back from its result fact, over `std.math.decimal-arith`, `std.math.decimal` and `std.math.arith` |

**The text a prompt shows** (`Catalogue::text`) is the library's manifest digest, how a theory is selected, and
each entry: its selection, its meaning, its rule's own `:why` where a rule says it, what it gives the hypothesis
mechanism, the kernel's answer, and the entry program less its header, so that the model is shown text the kernel
ran. **The minimal catalogue** is the closure of `std.algebra`, its entries and the contradiction's.

**Its generated part** (`catalogue::generated`) is every module the kernel resolves, each rule with its parameters,
its own `:why` and what it concludes, read off the module's parse, the manifest digest at its top; the check reads
from it which rule is a check, and the test holds each entry's quoted rule to it.

**The catalogue a step's grammar is rendered on** is narrower than the library's
([the grammars § 3](06-grammars.md#3-a-grammar-a-step)): the modules the entries import, each with the names the
entries list of it, the relations among those names, the facts the entries select with at their counts, and the
names the entries write as values that the library reads and no module declares (the calculator's orders and the
ten digits) (`Catalogue::grammar`).

## 2. The prompts

**Every prompt is a versioned template under [`einaf/prompts/`](../../einaf/prompts/), rendered by filling named
slots** (`einaf::prompts::Template`): a slot is `{{name}}`, a line that begins with `%%` is the template's own note,
which names the section of [the contract](04-contract.md) it implements and is not rendered, and rendering refuses a
slot left unfilled or a value for a slot the template has not. A template's digest and the rendered prompt's are the
record's: a prompt edit changes a digest, and a changed digest is a new experiment.

| template | the request | its slots |
|---|---|---|
| `steps/system.v1.md` | the system message of every request: one sentence of instructions and the excerpt | `excerpt` |
| `steps/step1.v1.md` | step 1: the statements, the answer query, the ontology partial | `examples` (each to step 1), `text`, `answer_line` |
| `steps/step2.v1.md` | step 2: the ontology whole, the imports, the selection, a marker where a property is missing | `catalogue`, `examples` (to step 2), `text`, `program` (step 1's) |
| `steps/step2b.v1.md` | step 2's second call: the rules of the property each marker names | `markers`, `catalogue`, `examples` (to step 3), `text`, `program` |
| `steps/step3.v1.md` | step 3: the rest of the selection, the rules each after its marker, the hypothesis mechanism | `catalogue`, `examples` (to step 3), `text`, `program` |
| `steps/whole.v1.md` | the whole program in one call | `catalogue`, `examples` (whole), `text`, `answer_line` |

**The excerpt** is [`ein-excerpt.v2.md`](../../einaf/prompts/ein-excerpt.v2.md): Ein's words, forms, facts and
relations, rules, the answer, the hypothesis mechanism, the library in brief and what the kernel refuses, every
program it shows loaded by a test; names of letters, digits and `_ . * + = -` alone, as the grammars allow; `config`,
`trace` and `macro` named as forms a program does not write; its examples of no family the phase's texts are of.

**The examples** (`einaf::prompts::EXAMPLES`, [`einaf/prompts/examples/`](../../einaf/prompts/examples/)) are three
texts, each with what each step wrote for it, a step's own output, never the program so far: **three stalls in a
row**, an attribute puzzle over the Zebra puzzle's theory (`std.slots`, the relation algebra's read-back); **a
purchase**, a product over the calculator; **an age**, a difference over the calculator. A request shows each text
with its steps up to the request's step, so that the model sees what it is handed and what it hands on. None is of a
family the phase's texts are of, and every program a prompt quotes is listed in the run's record (`quoted`). No
example program holds a backtick, `<`, `>` or `|` in a comment or a string, which no grammar a model writes under
allows ([the grammars § 1](06-grammars.md#1-the-whole-grammar)).

**The answer lines** (`einaf/prompts/answer-lines/`) are what a text is asked to answer with, the same words for
every arm of a family: `zebra.v1.md` (who drinks water, who owns the zebra), `word.v1.md` (how many),
`generic.v1.md` (a sentence that answers the question). Step 1 and the one call are given one with the text.

## 3. A run

**Each request** is the system message, then the user message the step's template renders, thinking off, greedy,
seed 1, a budget of generated tokens (12 288 by default); under the step's grammar
([the grammars § 3](06-grammars.md#3-a-grammar-a-step)), rendered on the text's sentences (step 1) or on the
catalogue and the program of the steps before it (steps 2, 2b and 3), inside the fenced block the prompt asks for
and held to by the chain's choice first, or under no grammar (`--no-grammar`). How a request is made beside its
prompt and grammar is a **request mode**, `thinking-off` today, which a later stage extends. **What the step wrote**
is its answer's first fenced block, or the whole answer where it wrote none, parsed alone through `einaf::kernel::parse`.

**The steps, in order**: step 1; step 2, on step 1's output; **step 2's second call**, only where step 2's output
holds a marker line, `;; missing: ` and the property's words, with the markers in its prompt; step 3, on the program
so far. A step whose output does not parse, or is empty, ends the run, and the record says where.

**The program is assembled in synthesis order** (`einaf::formalize::assemble_parts`): step 1's statements and
answer query, everything step 1 wrote before its first `relation` form (`split_step1`); then step 2's output, whose
ontology stands in place of step 1's; then step 2's second call and step 3; each part under a comment line that names
its step, each form as the model wrote it. What step 2 changed of step 1's ontology is recorded (the forms kept,
dropped and added, the relations declared otherwise), and is no finding.

**Each step's program is judged at root**: the program assembled up to that step, through the bounded call under
`--no-search` with a clock (60 s by default); root and H through `einaf::kernel::root`, which builds them as `ein show
hyps` does and returns **the writer of H the kernel reports** and root's narration, each firing and each obligation
owed, as values; and the contract's check, its findings that apply to the step. **The complete program** is asked
`--no-search` and `--single`, and each of its forms is put in the contract's categories with the step whose program
first holds it ([the contract § 4](04-contract.md#4-what-each-form-is-the-categories)).

**The whole program in one call** (`--mode whole`): one request, the whole template, under the whole grammar's root
`root` inside the fence or under none; the same reading, the same check, on the complete program alone.

## 4. The contract's check

**EinAf's, over a step's program or the complete one, the input text and the catalogue**
(`einaf::check::check`). It reports and refuses nothing; its findings are kept apart from the kernel's diagnostics. A
finding applies to a step's program from the step that could have avoided it.

| | finding | its kinds | from step |
|---|---|---|---|
| 1 | a `:source` that is no sentence of the input | `source-not-a-sentence` | 1 |
| 2 | a selection that is not active: an activating fact under a name the file does not list from the module that declares its rule, or of a module no import brings, read off the program's text; a selected rule root names no firing of with the fact's arguments and a selected obligation no `owe` of, read off root's narration, a check of the library's aside, which fires only where a state breaks it; an import or a selecting fact the catalogue has no entry for | `selection-not-listed`, `selection-inactive`, `selection-not-in-catalogue` | 2 |
| 3 | the mechanism: the writer of H is `blind` and the program states none, or the writer does not agree with every mechanism the program states (`(solve :no-search)`, a `(solve :hypothesis-relations …)` list, a written `(open F)` or a rule that concludes one, a selected obligation): `(solve :no-search)` beside a selected obligation is the writer `no-search`, and takes the obligations' candidates away | `mechanism-not-stated`, `mechanism-not-the-one-stated` | 3 alone |
| 4 | a fact or a pattern of a relation the program does not declare and no imported module has, or at a count it is not declared at | `undeclared`, `wrong-count` | 1 for a top-level fact and a goal, 2 for a fact inside a fact, 3 for a rule's |
| 5 | a number written as a name, outside a numeral's digit | `number-as-name` | 1 |
| 6 | of the answer query: none, or more than one; a `:goal-text` reference the goal does not bind; a template with no reference, or one that writes a name of the program's facts the goal does not hold | `no-answer-query`, `several-answer-queries`, `unbound-reference`, `answer-in-template` | 1 |
| 7 | a step-1 statement written again otherwise: a fact a later step writes with the `:source` of a sentence step 1 stated, of no relation step 1's memberships are of, and not among step 1's statements | `rewritten` | 2 |

**What the second reading of finding 2 says and does not**: a rule that derives only from facts a state adds above
root fires at no root, so a selection whose premises root does not hold is `selection-inactive` though a state
fires it (the Zebra puzzle's steps by hand: two read-backs of a pet's place). The first reading, the text's, is the
one that names a misspelling.

## 5. The record

`einaf steps run … --out DIR` writes:

| file | what |
|---|---|
| `calls.jsonl` | every call, `einaf-generation/1`, a line a call: the request whole and the generation or the failure ([the model's runtime § 9](05-model.md#9-replay)) |
| `run.json` | the run, `einaf-steps/1`: the options; the digests of the text, the answer line, the excerpt and the catalogue; every program a prompt quoted; each step's template and its digest, the rendered prompt's digest, the grammar's, the request's, the tokens, what it wrote, its parse, its markers, its program, and its judgement (each feedback object, the writer of H and root's firings counted, the check's findings and the mechanism); where the run stopped; the complete program, its judgement and its categories; what step 2 changed of step 1's ontology |
| `stepN.out.ein` | what each step wrote |
| `program.ein` | the complete program |

**A run replays**: `--replay DIR/calls.jsonl` answers each request from the record by its digest, needs no model,
and gives the run it recorded; a template edited since is a request no record answers.

## 6. The commands

```sh
einaf steps run TEXT --out DIR --model PATH [--id ID] [--answer-line zebra|word|generic]
     [--mode steps|whole] [--no-grammar] [--request-mode thinking-off]
     [--catalogue whole|minimal] [--max-tokens N] [-T SECONDS]   # with --replay FILE in place of --model
einaf steps check PROGRAM [--text TEXT] [--step 1|2|3] [--catalogue whole|minimal]
einaf catalogue [--minimal] [--generated] [--grammar] [--tokens MODEL]
```

`einaf steps run` prints a line a step (its prompt's and generation's tokens, how it stopped, its parse, its
judgement at root, the writer of H, its findings) and the complete program's outcome under each question, the answer
query's lines, and the findings by kind. `einaf steps check` prints the writer of H, the mechanism the program states
and each finding with its form. `einaf catalogue` prints the catalogue as a prompt shows it; `--generated` the
library's rules; `--grammar` the catalogue the steps' grammars are rendered on; `--tokens MODEL` its length and the
templates' in the model's tokens (the `llama` feature).

## 7. Held by

In EinAf's gate step, with no model:

| test | what it holds |
|---|---|
| [`catalogue.rs`](../../einaf/tests/catalogue.rs) `every_entry_is_run_by_the_kernel_and_shows_its_stated_effect` | each entry program through the bounded call and `einaf::kernel::root`: its outcome and answer, the writer of H, the rules root fires or owes, every selection active by the check |
| `catalogue.rs`, the rest | the text cut from the entry programs; the minimal catalogue; the grammars' catalogue narrower than the library's; the generated part |
| [`steps.rs`](../../einaf/tests/steps.rs) `the_examples_answer_as_their_texts_ask_under_their_steps_grammars` | each example's program answered by the kernel; each step of it accepted by its step's grammar |
| `steps.rs` `the_steps_by_hand_replay_through_the_steps_as_one_generation_a_step` | [the contract § 7](04-contract.md#7-the-steps-by-hand)'s texts through a run, a generation a step: the writer of H of each step, root at each step before the last, the answer, the program the hand's, the categories, and the check's findings |
| `steps.rs` `each_hand_written_step_is_accepted_by_the_grammar_a_run_renders` | each part of the steps by hand under its step's grammar, rendered on the catalogue's entries |
| `steps.rs` `the_check_finds_what_the_kernel_loads_in_silence` | each finding on a program that holds it, and the findings of a step 1 |
| `steps.rs` `a_recorded_run_replays_to_the_run_it_recorded` | recorded runs of the model under [`einaf/tests/fixtures/steps/`](../../einaf/tests/fixtures/steps/), replayed: the same requests, outputs, programs, writers, findings and categories; each template's digest the record's |
| [`prompts.rs`](../../einaf/tests/prompts.rs) | the excerpt's programs load; each template renders with its slots and no other |
| [`contract.rs`](../../einaf/tests/contract.rs) `the_writer_of_h_the_kernel_module_reads_is_ein_show_hyps` | `einaf::kernel::root`'s writer and H's size against `ein show hyps` on every program of `einaf/contract/` |
