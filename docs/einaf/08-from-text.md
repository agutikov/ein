# From a text to a short answer

**`einaf from-text` is the one shot**: a text becomes an Ein program in the three steps ([the steps](07-steps.md)),
each step's program judged at root and kept; the complete program is asked the least questions its answer needs,
each through the bounded call ([the feedback object § 1](03-feedback.md#1-the-bounded-call)); and what is printed
is **the short answer the program prints**, the answer query's `:goal-text` filled per row, its values formatted, or
one of EinAf's fixed texts where no program prints one. No repair, and no step generated a second time. The code is
[`einaf::from_text`](../../einaf/src/from_text.rs), its command [`from_text_cmd.rs`](../../einaf/src/from_text_cmd.rs);
the answer [`einaf::answer`](../../einaf/src/answer.rs); the formatters [`einaf::format`](../../einaf/src/format/mod.rs).
The loop's GENERATE and the harness's run call the same function, `einaf::from_text::run`.

| section | what |
|---|---|
| [1. A run](#1-a-run) | the steps, the questions asked, the bounds, a directory of modules, an output the budget cut |
| [2. The answer](#2-the-answer) | which text is printed, chosen by the answer query's classes; the fixed texts |
| [3. The formatters](#3-the-formatters) | a value read back as a term, its names resolved, and written; the template filled again |
| [4. The record](#4-the-record) | what a run writes; a call slim, with no request body; replay |
| [5. The command](#5-the-command) | `einaf from-text`, its options and its output |
| [6. Held by](#6-held-by) | the tests |

## 1. A run

**The formalization** is [the steps' run](07-steps.md#3-a-run), `einaf::formalize::run_with`: the three steps under
their grammars, thinking off, greedy; each step's program assembled in synthesis order and judged at root
(`--no-search`, the writer of H, the contract's check); the complete program asked `--no-search` and `--single` and
put in the categories. The model is not shown a step's judgement before the next step.

**The questions** are the least the answer needs. `--single` says whether R is one state; where it found two, the
answer query's classes are over the two states it listed and not over R, a lower bound. **A description of R is
asked only where that bound decides the text**: the answer query has one class, with a row, and is not over a
complete set (`einaf::answer::needs_description`). Where `--single`'s two states already give two classes, the text
is decided, and the alternatives printed are those two, said to be the states found. `--no-search`'s object gives
what root owes, in the kernel's sentences, which a description alone would otherwise carry.

**The bounds** are every kernel call's, and the record keeps them: the kernel's facts budget (`--region-budget`,
2²⁰) and cube limit (`--region-limit`, 16), and the two of EinAf's call, the cap on one saturation's firings
(`--max-steps`, 2²⁰) and the clock (`-T`, 60 s by default). A run a bound stopped is *not decided*. **The check's
narration of root is EinAf's own cost**: a root whose saturation has no fixpoint narrates each firing with a term one
deeper, so its narration grows with the square of its firings; what is kept of it is bounded
(`einaf::kernel::NARRATION_LIMIT`, 64 MiB), a `selection-inactive` finding is not read off a narration cut so,
and its time is the clock's.

**A directory of modules given beside the library** (`--module-dir DIR`): the escape hatch of the ontology's provenance, a theory given, the widest condition of
the library ablation, and no part of the main arms. Its modules are its `.ein` files that state no `(query …)` and
no `(solve …)`, a problem's file being no module (`einaf::catalogue::Given`). Steps 2 and 3 are shown each module by
its forms, its relations as declared with their `:why` and its rules with their parameters and `:why`, and no comment
of its file; the steps' grammars are offered its names as a module of the catalogue's; and every program is loaded
with its local imports found from the directory.

**An output the budget of generated tokens cut** (`--cut`, `keep-forms` by default here): the forms written before
the cut, every top-level form the output completed, less the one the cut stopped in and every form that repeats one
written before it, which is the loop greedy decoding runs into under a grammar; a step whose output keeps no form
ends the run, as one that does not parse does. `--cut end` reads the output as it stands, `einaf steps run`'s
reading. The step's record says what was kept and dropped.

## 2. The answer

**The text is chosen by the answer query's classes, not by the kernel's word alone** (`einaf::answer::read`). The
answer query is the program's first query with a `:goal-text`. The object that chooses is the last question asked
that is not `--no-search`: `--single`'s, or the description's where one was asked.

| the kernel found | `einaf from-text` prints | whose words | an answer |
|---|---|---|---|
| a state in R, and the answer query one class with a row, over a complete set | each row through the `:goal-text`, a line each, its values formatted | the program's `:goal-text`; a value's text EinAf's | yes |
| a state in R, and two classes or more | **The statements do not determine the answer.** Beneath it, each class's lines, as alternatives | EinAf's; the alternatives the program's | yes: *ambiguous* |
| a state in R, and one class with no row | **The statements do not determine the answer.** | EinAf's | yes: *ambiguous* |
| *inconsistent* | **The statements contradict each other.** | EinAf's | yes: *unsatisfiable* |
| *incomplete* | **No answer: the program is incomplete.** Beneath it, what root owes, in the kernel's sentences | EinAf's; the kernel's beneath | no |
| *not decided* | **No answer: not decided.** Beneath it, the bound that stopped the run and the kernel's lines | EinAf's; the kernel's beneath | no |
| *flagged* | **No answer is claimed: the program is not monotone.** Beneath it, the kernel's warning and the lines it printed | EinAf's; the kernel's beneath | no |
| *invalid*, or no program | **No answer: the program was refused.** Beneath it, each diagnostic with its kind and its place, or where the steps stopped | EinAf's; the kernel's beneath | no |
| a state in R, and no answer query | **No answer: the program states no answer query.** | EinAf's | no |

**The fixed texts are EinAf's, never the model's**, constants of `einaf::answer`. The two that are answers are the
strings every arm may give and the gold short answers of an *ambiguous* and an *unsatisfiable* instance; each reads
as an answer to a question of *who*, *how many*, *which* or *can*. The other five are lines of the command, scored as
no answer. **This is the one place the answer is not the program's own output**: an empty R has no state to print
from.

**A sentence is printed per row.** One class whose rows are several prints each row: a program whose rule reads what
it concludes may give one value a row a pass, and the answer states each (a program for `bob1`'s text whose rule
asks the difference of what it concluded prints *Bob has 1 / 3 / 5 apples*).

**A stand-in reader of the answer's comparison** (`einaf::answer::score`, `--gold-answer`): whether the answer
states the values a gold short answer binds. The gold is `@undetermined` or `@contradiction`, the fixed text of that
name, or claims separated by `|`, each a few words one line must hold together, case aside; where the gold states a
number, no line may state another. A fixed text against a gold of values is wrong, and a line that is no answer is
*no answer*. The comparison every arm is scored by is the harness's, which this stands in for.

## 3. The formatters

**A value of a row is its Ein text**, in the spelling the program's main file gives its names: one numeral prints
`(n z 5)`, `(d.n ar.z 5)` or `(std.math.decimal.n std.math.arith.z 5)` by the form of the import that brought it. So a
formatter **reads the text back as a term** (`einaf::format::read`) and **matches its names by their full names**,
which `einaf::format::Names` resolves from the program's imports: a name an import lists is that module's, `a.x` is
the module aliased `a`'s `x`, any other name as printed.

**A formatter is a kind of value's** (`einaf::format::Format`), tried in order, the first that takes a term writing
it; a value none takes prints as written:

| formatter | the kind | written as |
|---|---|---|
| `decimal` ([`decimal.rs`](../../einaf/src/format/decimal.rs)) | `std.math.arith.z`, or `(std.math.decimal.n PREFIX DIGIT)` with a digit `0` to `9` and a numeral prefix | its digits, by value: `(n (n z 4) 2)` is `42`, a high zero dropped, zero `0`; a term headed like a numeral that is not one is not taken |
| `words` ([`facts.rs`](../../einaf/src/format/facts.rs), [`cases/words.txt`](../../einaf/src/format/cases/words.txt)) | a fact of one of the word theory's quantities, `words.amount` and the eight others, at its count | its case's words, the theory's own declarations' `:why`, each argument formatted in its turn: `(amount Bob apples t1)` is *how many apples Bob has at t1* |

**A fact that stands as a value is written by cases that are data**: a file a theory, a case a line, the relation's
full name, its count of arguments and its words, `{1}` the first argument. A fact with no case prints as written: no
words are invented for a relation the cases do not name, which is Ein's own rule for a fact. A kind added (an integer,
a rational, a binary numeral) is a formatter, its cases and its tests.

**The template is filled again, the kernel's line is not edited** (`einaf::format::fill`): a reference `{?v}` a row
binds is replaced by the formatted value, one nothing binds is left as written, by the kernel's rule for a reference.
A value's text is not unique inside a line, `boxes13`'s holding `(n z 5)` and a bare `z` together, and a row keeps each
value apart, under its variable. **With no formatter the line is the kernel's, byte for byte**, which the corpus test
holds; the record keeps both lines.

**The module is built alone**: it uses the standard library and nothing of EinAf's or Ein's, and its test compiles it
with `rustc` on its own. It is written to be the starting point of Ein's own formatting of a value: a formatter is a
kind's and nothing else's, its cases are data beside it, its tests are beside it. The trace `--explain` writes is the
kernel's and is not formatted.

## 4. The record

`--record DIR` writes, in the layout the harness's record will keep:

| file | what |
|---|---|
| `config.json` | `einaf-from-text-config/1`: the text's and the answer line's digests, the options, the module directory, the bounds, the questions asked, the kernel, the model, its runtime and context, the chat template and the sampler from the first call's envelope, the excerpt's and the catalogue's digests, `stdlib`, the library's manifest digest, which no prompt shows, each template and its digest, every program a prompt quoted; `analysis`, where step 1 was given one ([the text's analysis § 5](09-analysis.md#5-the-record)) |
| `generations.jsonl` | every call **slim**, `einaf-call/1` (`einaf::model::Slim`): the request's digest, and the generation, its envelope whole and the answer as the model wrote it, the generated tokens' ids replaced by their digest (`tokens_sha256`); or the failure. **No request body**: replay renders the request again from the versioned prompts and finds the call by its digest |
| `program.ein` | the complete program, as assembled |
| `program.sidecar.json` | `einaf-sidecar/1`: each form's category with the step that wrote it, the check's findings of each step's program and the complete one's, the writer of H at each, and what step 2 changed of step 1's ontology |
| `steps/stepN.ein` | each step's program, assembled up to it |
| `kernel.jsonl` | every feedback object, a line each, with the step it judged (`1`, `2`, `2b`, `complete`, `answer`) |
| `answer.json` | `einaf-answer/1`: the answer's case, the question and the outcome it was chosen by, the template, each line as the kernel filled it and as formatted with each value and its formatter, the fixed text, the alternatives, what stands beneath, how many values still print as a term; the text scored, the kernel's text, and the score against the gold answer where one was given |
| `run.json` | the run whole, `einaf-from-text/1`: the formalization's record ([the steps § 5](07-steps.md#5-the-record)), the objects asked for the answer, the answer, the bounds |

**A record replays**: `--replay DIR/generations.jsonl` with the run's text and options gives the same calls, each
step's program, the same complete program and the same answer, with no model. A prompt edited since is a request no
call answers. **The library's digest is the record's and no prompt's** ([the steps § 1](07-steps.md#1-the-catalogue)),
so a record replays under any library that leaves the catalogue's text as it was, `$EIN_STDLIB` naming it, and the
record of the replay names that library.

## 5. The command

```sh
einaf from-text TEXT (--model PATH | --replay FILE) [--id ID] [--answer-line zebra|word|generic]
     [--mode steps|whole] [--no-grammar] [--request-mode thinking-off] [--catalogue whole|minimal]
     [--module-dir DIR] [--max-tokens N] [-T SECONDS] [--seed N] [--cut keep-forms|end] [--no-format]
     [--out FILE] [--explain FILE.md] [--record DIR] [--gold-answer GOLD]
     [--analysis FILE [--analysis-parser P] [--analysis-form F] [--analysis-entities]]
```

**stdout is the answer**: the formatted lines, or the fixed text with what stands beneath it, indented, and the
alternatives. **stderr is the run**: each call with its prompt's and its generation's tokens, how it stopped and its
time; each step's parse, its judgement at root, the writer of H and the check's findings; the complete program's
writer of H and findings; each question asked of it with its outcome and time; the answer's case; the whole time; the
score against `--gold-answer`. `--out` writes the program, `--explain` the kernel's trace of the answer under the
deciding question, as `ein solve --trace FILE.md --no-diagrams` writes it, `--no-format` prints the kernel's lines as
they stand. `--seed` is the requests' seed, recorded; greedy decoding draws nothing. Every answer exits 0; a usage
error exits 2, a model that does not load or a record not written 1.

<!-- output -->
```text
$ einaf from-text einaf/tests/fixtures/steps/bob1/text.txt --answer-line word \
    --replay einaf/tests/fixtures/steps/bob1/calls.jsonl
Bob has 5 apples.
```

The record replayed is a run of `einaf steps run` on the same text, whose requests are the one shot's request for
request; its program has no rule that asks the difference, so the stated *5* is the one row.

## 6. Held by

In EinAf's gate step, with no model:

| test | what it holds |
|---|---|
| [`from_text.rs`](../../einaf/tests/from_text.rs) `the_formatters_module_builds_and_is_tested_alone` | the module's files name nothing of EinAf or Ein; `rustc` builds it alone and its tests pass |
| `from_text.rs` `every_outcome_has_its_line_and_einaf_from_text_replay_prints_it` | a fixture a row of § 2's table (each of the seven outcomes, one class with no row, two classes, a class a description decides, no answer query), the one shot's program, its record written slim and `einaf from-text --replay` printing the table's line; the questions asked |
| `from_text.rs` `the_reference_word_problem_prints_a_sentence_whose_number_reads_3` | the word problem over the calculator prints *Bob has 3 apples*, scored right, and with no formatter the kernel's line |
| `from_text.rs` `the_word_problems_print_no_numeral_as_a_term_under_every_import_form` | the sixteen word problems' own programs under each of the three forms of an import: every numeral its digits, every quantity in its words, every line filled again the kernel's |
| `from_text.rs` `the_zebra_puzzles_steps_by_hand_go_through_the_one_shot_to_its_answer` | the Zebra puzzle's steps by hand, a generation a step, through the one shot: each step at root, the answer *The Norwegian drinks water. The Japanese owns the zebra.*, scored right; the library's rules root fired |
| `from_text.rs` `a_recorded_run_of_the_model_replays_step_by_step_from_its_slim_record` | the recorded runs of the model under [`einaf/tests/fixtures/from_text/`](../../einaf/tests/fixtures/from_text/), slim, replayed: each step's program, the complete program and the answer printed |
| `from_text.rs` `a_recorded_run_replays_under_a_library_whose_digest_moved_and_whose_catalogue_did_not` | a copy of the library with a comment added to a module no entry imports and its manifest written again: under it (`$EIN_STDLIB`) `einaf catalogue` prints what it prints under the checkout's library, and each recorded run replays through `einaf from-text --replay` to its program, its sidecar and its answer, its record naming the copy's digest |
| `from_text.rs` `a_directory_of_modules_is_shown_offered_to_the_grammars_and_loaded` | `--module-dir`: the modules and no problem shown by their forms, offered to step 2's grammar, and loaded |
| [`corpus.rs`](../../einaf/tests/corpus.rs) `the_answer_line_with_no_formatter_is_the_kernels_byte_for_byte` | on every corpus program that is not `slow`, under every question, every row of every query with a `:goal-text` filled again with no formatter is the kernel's line |
