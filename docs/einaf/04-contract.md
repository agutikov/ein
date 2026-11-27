# The formalization contract

**What EinAf asks of a formalization**: a text in English goes in, one Ein program comes out, and the kernel does
everything else. Whether the program is consistent, how many states its region has and what its answer is are the
kernel's; the model writes no answer beside its program. The page states the research plan's Stage B, its items B1
to B4 (the task, the categories of what a program holds, what the model receives, what a run records), and the
three steps a formalization is made in.

Every clause names what enforces it: **the kernel** (a refusal, the static check, a read-out), **EinAf's code**
([`einaf::contract`](../../einaf/src/contract.rs), which computes the categories, and
[`einaf::check`](../../einaf/src/check.rs), the contract's check, [the steps § 4](07-steps.md#4-the-contracts-check)),
**EinAf's tests** ([`einaf/tests/contract.rs`](../../einaf/tests/contract.rs), which run this page's examples), or
**nothing yet**: the faithfulness judge and the record's layout are written to this page and do not exist yet. A
number on the page carries the test that holds it.

| section | what |
|---|---|
| [1. The task](#1-the-task) | the input; the output in five parts made in three steps; the order; what a program may not hold; the answer; the target encoding; what a formalization comes to |
| [2. The mechanism line](#2-the-mechanism-line-the-worked-example) | the worked example, in its three steps, and the answer turning on one line |
| [3. The steps](#3-the-steps) | each step's input, output and check, and the five design questions with the choice taken |
| [4. The categories](#4-what-each-form-is-the-categories) | each form of a program in one row, by step, and which row is a faithfulness event |
| [5. What the model receives](#5-what-the-model-receives) | the five items of a prompt, and what it never holds |
| [6. What a run records](#6-what-a-run-records) | the record's fields |
| [7. The steps by hand](#7-the-steps-by-hand) | the Zebra puzzle and four other texts through the three steps, each step's program judged |
| [8. Held by](#8-held-by) | the tests |

## 1. The task

### 1.1 The input

**A problem statement in English**: sentences, each ending at a `.`, `?` or `!`. A question is a sentence of the
text. Names are ASCII and a character that is not ASCII is written only in a string or a comment
([§ 1.6 of the diagnostics](../ein/user/04-reference/07-diagnostics.md#16-a-character-that-is-not-ascii-is-written-only-in-a-string-or-a-comment)),
so a text in another language is out until Ein's names are not ASCII only. *Enforced by* the kernel, which refuses
a name that is not ASCII; [`einaf::contract::sentences`](../../einaf/src/contract.rs) cuts a text into the sentences
a `:source` is read against.

### 1.2 The output: five parts, made in three steps

**One Ein program the kernel loads as it stands**, in five parts. A part is written by a step, and partial where a
later step completes it ([§ 3](#3-the-steps)).

| part | step | the forms | the rule | enforced by |
|---|---|---|---|---|
| **instance** | 1 | facts and stored negations with `:source`, in the text's own words | the `:source` is **a sentence of the input, verbatim**: a sentence that states several facts is quoted whole by each, as `There are five houses.` is by five; a quantity a theory computes with is a numeral of the calculator, `(n z 5)`, and any other number is a name | the categories: a `:source` that is no sentence of the input is the judge's event `unfound_source` ([§ 4](#4-what-each-form-is-the-categories)); a fact takes `:source`, `:rule` and `:using` and no other keyword, which the kernel refuses ([Q-M1e2.28](../decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)); a number written as a name: the contract's check, `number-as-name` |
| **query** | 1 | **the answer query, the one `query` that carries a `:goal-text`**; any other query carries none and is a check | its goal asks what the text's question asks, written as literally as a statement, over the facts' own relations, so that the theory has to make it answerable; its `:goal-text` is **a sentence that says the answer in the problem's words**, `"{?who} is the youngest"`, with references and no name of its own where a reference should be; its variables bind names and numerals | the kernel holds a goal to what a rule's `:match` is held to ([§ 2.21](../ein/user/04-reference/07-diagnostics.md#221-a-goal-is-held-to-what-a-rules-match-is-held-to)); [`einaf::contract::answer_query`](../../einaf/src/contract.rs) reads the first query with a `:goal-text`; a program with none or with two, a reference the goal does not bind, and a template that holds more than its references: the contract's check, finding 6 |
| **ontology** | 1, partial; 2, complete | `relation` forms; membership facts | every relation carries a `:why` that reads its declaration, `{?3}` its first label ([natural language § 1](../ein/user/03-syntax-and-semantics/14-natural-language.md#1-the-four-texts)); a name is a run of the characters [the syntax § 1](../ein/user/04-reference/02-syntax.md#1-lexical-rules) allows, taken from the text's own words; a label is no type ([Q-M1f.372](../decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)) | the kernel refuses a relation with no `:why` ([§ 2.10](../ein/user/04-reference/07-diagnostics.md#210-a-form-with-no-why)), and EinAf never loads with the requirement off; a fact of a relation the program does not declare, and a form of a declared relation at another count, load in silence ([Q-M1f.447](../decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)): the contract's check, `undeclared` and `wrong-count` |
| **theory** | 2, partial; 3, complete | `import` forms with `:symbols`, and the facts that select what they bring; a `rule` of the program's own where the library has nothing | a theory is written as the library's pages write it ([what to list](../ein/user/04-reference/04-stdlib/README.md#what-to-list)); a rule carries a `:why` that says what it concludes and from what | the kernel refuses a rule with no `:why`; a selection under a name the file does not list is inert, with no message, and a selected theory that activated nothing: the contract's check, `selection-not-listed` and `selection-inactive`; the catalogue each step 2 and 3 is shown is a table of programs the kernel ran ([the steps § 1](07-steps.md#1-the-catalogue)) |
| **hypothesis mechanism** | 3 | nothing, where a theory the program selects has obligations that propose; a rule that concludes `(open F)`, or a written `(open F)`; `(solve :no-search)`; `(solve :hypothesis-relations (R …))` | **the program states its mechanism**: one of the four, or the first two together, whose hypotheses the kernel takes as one set ([Q-M1f.570](../decisions/hypotheses/Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)); the writer of H is what the kernel reports, `obligations`, `hypothesis-rules`, `union`, `no-search` or `blind`, as `ein show hyps` prints it, and `blind` with no `(solve …)` list is a mechanism not stated | the kernel refuses mechanisms that contradict one another ([§ 2.26](../ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another)); **a program that states none is answered by the blind enumerator in silence** ([the writers](../ein/user/03-syntax-and-semantics/09-hypotheses.md#where-candidates-come-from-the-writers), [Q-M1f.576](../decisions/hypotheses/Q-M1f.576-the-blind-enumerator-writes-only-where-root.md)); the bounded call returns no writer, and `einaf::kernel::root` does; a mechanism not stated, or not the one the writer is: the contract's check, finding 3 |

### 1.3 The order of the program's text

**The program's text is the steps' outputs one after another**, its synthesis order: the statements in the text's
order with the answer query where the text asks it, then the ontology, then the theory with the hypothesis
mechanism. Ein reads a program's forms in any order: a fact may come before its relation's declaration, a selecting
fact before the import that brings its rule. *Enforced by* nothing of Ein's: no page of Ein states it and no test of
Ein holds it. On this page's programs it is held by
`each_complete_program_answers_alike_with_its_forms_reversed`: each complete program, its forms written the other
way round, has the same outcome, states and answer.

### 1.4 What a program may not hold

A `(config …)`, since no flag changes the answer and the harness sets what a run needs; a `(trace …)`; a saved
file's wrapper; a string outside `:why`, `:source` and `:goal-text`, which the kernel refuses. *Enforced by* the
kernel for the strings; for the rest, the grammars a model writes under, which admit none of them
([the grammars § 1](06-grammars.md#1-the-whole-grammar)), and with no grammar nothing. The categories put a `(config …)` and a `(trace …)` in the program's
frame ([§ 4](#4-what-each-form-is-the-categories)).

### 1.5 The answer is the program's

The model writes no answer beside its program. **What is reported is each row of the answer query rendered through
its `:goal-text`**, the sentence the kernel fills ([natural language](../ein/user/03-syntax-and-semantics/14-natural-language.md)),
*Cy is the youngest*, and not *Cy*. The kernel's line is the answer as it stands: a numeral prints as the
calculator writes it, *Bob has (n z 3) apples.*, until EinAf formats the values that do not read as text, by filling
the query's template again and never by editing the line. A line is printed per row and for nothing else: where the
answer query has no single class with a row, or the outcome gives no state to read, the answer is one of a few
fixed texts of EinAf's, chosen by the kernel's values and never written by the model. Every arm is asked for such a
sentence, and scored by the values it binds, not by its wording. *Enforced by* the kernel, which fills the template
(the feedback object's `queries[].classes[].lines`, [the feedback object § 2](03-feedback.md#2-the-keys)); the
formatters and the fixed texts, `einaf::format` and `einaf::answer`
([from a text to a short answer § 2 and § 3](08-from-text.md#2-the-answer)).

### 1.6 The target encoding

**For an attribute puzzle**, the Zebra puzzle's statements in the text's own words, one relation a verb in the
sentence's voice, a description written as the word it is built on, the theory selected from the library with no
rule of the program's own, H written by the library's obligations: [`examples/zebra3.1.ein`](../../examples/zebra3.1.ein),
which the steps by hand end at ([§ 7.2](#72-the-zebra-puzzle-step-by-step)). **For a word problem**, the statements
in the text's words over the calculator, `std.math.decimal-arith`, with the rules the library does not have.
*Enforced by* nothing: it is the shape the examples show.

### 1.7 What a formalization comes to

**One of the seven outcomes** under each question asked of it ([the outcomes](01-outcomes.md)): *invalid*,
*flagged*, *not decided*, *inconsistent*, *incomplete*, *several*, *one*. A program the static check finds not
monotone loads and prints an answer of which nothing is claimed
([Q-M1f.564](../decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)); to EinAf it is
*flagged*, never certified. Beside the outcome stand the findings of the contract's check, which are EinAf's and
never the kernel's, kept apart from what the kernel said: a `:source` that is no sentence of the input; a selected
theory that activated nothing; a mechanism not stated; a fact of a relation the program does not declare; a form of
a declared relation at another count; a number written as a name where the theory computes; a `:goal-text`
reference nothing binds; a `:goal-text` on no query, or on two. The kernel refuses none of them. *Enforced by* the
kernel and [the outcomes](01-outcomes.md) for the outcome; the findings by the contract's check,
[`einaf::check`](../../einaf/src/check.rs), each from the step that could have avoided it
([the steps § 4](07-steps.md#4-the-contracts-check)).

## 2. The mechanism line: the worked example

*Ann is older than Bob. Bob is older than Cy. Who is the youngest?* ([`text.txt`](../../einaf/contract/youngest/text.txt))
Its three steps, each a file under [`einaf/contract/youngest/`](../../einaf/contract/youngest/); the three blocks
in this order are the whole program, [`step3.ein`](../../einaf/contract/youngest/step3.ein).

**Step 1**, [`step1.ein`](../../einaf/contract/youngest/step1.ein): the two statements, the question as the answer
query, and the ontology: the relations they use, and who the text names, each membership quoting the sentence that
names the person.

```lisp
(older Ann Bob :source "Ann is older than Bob.")
(older Bob Cy  :source "Bob is older than Cy.")

;; *Who is the youngest?*
(query :goal (youngest ?who) :goal-text "{?who} is the youngest")

(relation person   Person        :why "{?3} is a person")
(relation older    Person Person :why "a {?3} is older than a {?4}")
(relation youngest Person        :why "a {?3} is the youngest")
(person Ann :source "Ann is older than Bob.")
(person Bob :source "Ann is older than Bob.")
(person Cy  :source "Bob is older than Cy.")
```

**Step 2**, [`step2.ein`](../../einaf/contract/youngest/step2.ein), adds the theory the library has, selected under
the name the file lists:

```lisp
(import std.algebra :symbols (transitive))
(transitive older)
```

**Step 3**, [`step3.ein`](../../einaf/contract/youngest/step3.ein), adds what the library does not have, after the
marker of the property it supplies, and the mechanism:

```lisp
;; missing: the youngest, the one older than nobody
(rule youngest-is-older-than-nobody ()
  :match  (and (person ?p) (absent (older ?p ?q)))
  :assert (youngest ?p)
  :why    "{?p} is older than nobody, so {?p} is the youngest")

;; The hypothesis mechanism: none; the text states every fact there is.
(solve :no-search)
```

**One theory and one set of facts, and the answer is right, unclaimed or wrong by the mechanism line alone**
(`the_worked_example_turns_on_its_mechanism_line`):

| the mechanism line | the file | `ein show hyps` | the static check | `--single` | the answer |
|---|---|---|---|---|---|
| `(solve :no-search)` | [`step3.ein`](../../einaf/contract/youngest/step3.ein) | from `no-search`: no candidate | monotone | *one*, `Solution` | *“Cy is the youngest”* |
| deleted | [`no-mechanism.ein`](../../einaf/contract/youngest/no-mechanism.ein) | from `blind`: 5 candidates | **flagged**: the rule's `absent` reads `older`, which a commitment can fill | *flagged* | nothing is claimed of it |
| `(__closed__ older)` in its place | [`closed.ein`](../../einaf/contract/youngest/closed.ein) | from `blind`: 2 candidates, `(youngest Ann)` and `(youngest Bob)` | monotone | *several*, `Ambiguity` | *“Cy is the youngest”* in one state, *“Ann is the youngest”* and *“Cy is the youngest”* in another; R holds 4 states |

The third row is the one a reader does not expect: with the relation the rule reads closed, the blind enumerator
still supposes facts of the relation the rule concludes. And the five forms a formalizer with no theory writes,
[`no-theory.ein`](../../einaf/contract/youngest/no-theory.ein) (two relations, the two statements, the answer
query): H from `blind`, 7 candidates, a region of 128 states, monotone, *several*; **nothing the kernel returns
says that the program stated no mechanism**.

## 3. The steps

**The formalization is a search for an ontology and a theory around facts that stay close to the text**, made in
three steps: the facts and a partial ontology, then the complete ontology and a partial theory, then the complete
theory. The facts are the part of a formalization closest to the text and the part that changes least; *partial*
means *expected to change in the next step*. Once the program is complete the kernel is run on it and its feedback
consumed, and a repair changes the theory before the facts.

### 3.1 Each step

| | step 1: the facts and a partial ontology | step 2: the complete ontology and a partial theory | step 3: the complete theory |
|---|---|---|---|
| **input** | the text | the text, step 1's program | the text, step 2's program |
| **output** | the statements, each a fact (or several) in the text's words with its sentence as `:source`; the answer query; the declarations its facts and query use, each with a `:why`; the memberships it sees | the complete ontology, which stands in place of step 1's: every relation and membership the theory reads; the imports and the facts that select what they bring; some rules of the program's own | the rest of the rules, each with a `:why`; the hypothesis mechanism |
| **what *partial* means** | the declarations' labels, the memberships and the kinds may be completed or renamed | the rules | nothing: the program is run |
| **what it decides** | **which words name the same object**; what is an object and what a relation; one fact a sentence or several; whether the objects are of one kind or of several | which library theory, which properties | which rules the library does not have; which mechanism |
| **what it may change** | nothing before it | step 1's ontology, completed or renamed; never a statement or the answer query | nothing of a statement or the answer query |
| **how it is checked** | the kernel at root, under `--no-search`: it loads, it is not flagged, root is alive; the contract's check, its findings that apply to a step-1 program, a fact held to a declaration among them | the kernel at root; the contract's check | the kernel under every question; the contract's check |
| **enforced by** | a step's grammar ([the grammars § 3](06-grammars.md#3-a-grammar-a-step)), the contract's check on the step's program and the run's record ([the steps](07-steps.md)); on this page's programs, `a_later_step_keeps_the_statements_and_writes_its_own_part` holds that no later step rewrites a statement or the answer query and that no step writes a form of a later step's part ([§ 4](#4-what-each-form-is-the-categories) gives each part its step) | as step 1 | as step 1 |

### 3.2 The five design questions

1. **Where the questions go.** Step 1 translates a question as literally as a statement, into the answer query over
   the facts' own relations with its `:goal-text` sentence, and the theory has to make that query answerable. On
   the Zebra puzzle, step 2's program has one state and its answer query no row; step 3's read-back gives it its
   row ([§ 7.1](#71-each-steps-program-judged)).
2. **What the kernel is asked between steps.** Each step's program is loaded and judged at root (`--no-search`), its
   feedback object kept in the record; only the complete program's feedback is the loop's. Whether the model is
   shown a step's judgement before the next step is an arm of the ablations. On this page's programs every step's
   judgement at root is *not decided*: it loads, it is not flagged and root is alive, which is all `--no-search`
   says; `--single` says more ([§ 7.1](#71-each-steps-program-judged)).
3. **The program's text is the steps' outputs one after another** ([§ 1.3](#13-the-order-of-the-programs-text)).
4. **What a later step may change.** A later step adds to the facts and does not rewrite them: step 2's complete
   ontology stands in place of step 1's partial one, and what it changed is recorded; a fact a later step writes for
   a sentence step 1 already stated otherwise is recorded as *rewritten*, with the step-1 fact it differs from, and
   counted. Changing a step-1 fact is the loop's. On this page's programs no step rewrote one.
5. **What EinAf checks where the kernel cannot.** The contract's check runs on each step's program with the findings
   that apply to it ([§ 1.7](#17-what-a-formalization-comes-to), [the steps § 4](07-steps.md#4-the-contracts-check)).

## 4. What each form is: the categories

*The formalizer may restructure information from the source but must not invent problem-specific facts.* **Each
top-level form of a program is in exactly one row**, with the step that wrote it. A row is **computed** by EinAf
from the program's forms, what its imports bring and the input text
([`einaf::contract::categories`](../../einaf/src/contract.rs), over
[`einaf::kernel::forms`](../../einaf/src/kernel/forms.rs), which parses and loads the program and asks it nothing),
and kept beside the program in the record, keyed by the form's canonical text (`ein_ir::dump_canonical`'s). Nothing
is added to the program, which stays a plain input of the kernel: a fact takes no keyword that could carry a
category.

| category | the forms | its part's step | to the faithfulness judge |
|---|---|---|---|
| **source-derived fact** | a fact, a stored negation or a written `(open F)` with a `:source`; a membership is one where it quotes the sentence that names the thing | 1 | nothing; **the event `unfound_source`** where the `:source` is no sentence of the input |
| **generic theory** | an `import`; a fact that activates a rule of an imported module, `(transitive older)`; a fact of a relation an imported module's rule reads and the program does not declare, `(slot-partition …)` | 2 | nothing |
| **generated auxiliary relation** | a `relation` form of which no source-derived fact is a fact and which the answer query does not read | 2 | the note `generated_relation` |
| **generated inference rule** | a `rule` or a `macro` of the program's own, one that concludes `(open F)` among them, and a fact that activates one | 3 | the note `generated_rule` |
| **generated assumption** | a fact or a stored negation stated **as holding**, with no `:source`, that no rule reads as its activator and no imported module reads: a membership no sentence names, a kind, a condition no sentence states | 2 | **the event `generated_assumption`** |
| *the program's frame* | what a program needs to be well formed and to state its lattice: a `relation` form a source-derived fact is a fact of, or the answer query reads, with its `:why`; the queries; `(solve …)`; `(__closed__ R)`; a written `(open F)` with no `:source`; and a `(config …)` or `(trace …)`, which § 1.4 does not allow | 1 for a declaration and a query, 3 for the rest | never an event |

**The line between an event and a note is this table**: an event counts against a program, and a final program is
*faithful* where it has none. Only a form that says something holds, with no sentence behind it, is an event, and a
`:source` that is no sentence. A membership no sentence names is an event, though step 2's ontology writes such
memberships by design: every attribute puzzle's kinds are ([§ 7.3](#73-what-each-step-decided-on-each-text)). The
texts the frame carries, `:why` and `:goal-text`, are the model's own words and in no row.

**The step a form is written in** is its part's ([§ 1.2](#12-the-output-five-parts-made-in-three-steps),
`einaf::contract::part_step`): a step may write a form of an earlier step's part and never one of a later step's.
For a program made in steps the row carries the step that actually wrote it, the first whose program holds its
canonical text (`einaf::contract::categories_by_steps`).

**On the reference programs** (`the_categories_partition_every_form_of_the_reference_programs`, which assigns
every form of the target shape, the sixteen word problems of [`examples/word_problems/`](../../examples/word_problems/README.md),
this page's examples and every step's program, and prints each program's counts under `--nocapture`):

| program | forms | source-derived | generic theory | generated relation | generated rule | generated assumption | frame |
|---|---:|---:|---:|---:|---:|---:|---:|
| [`youngest/step3.ein`](../../einaf/contract/youngest/step3.ein) | 13 | 5 | 2 | 0 | 1 | 0 | 5 |
| [`zebra/step3.ein`](../../einaf/contract/zebra/step3.ein) | 112 | 44 | 34 | 12 | 0 | **11** | 11 |
| [`seating/step3.ein`](../../einaf/contract/seating/step3.ein) | 33 | 9 | 10 | 4 | 1 | **5** | 4 |
| [`apples/step3.ein`](../../einaf/contract/apples/step3.ein) | 11 | 2 | 3 | 0 | 2 | 0 | 4 |
| [`together/step3.ein`](../../einaf/contract/together/step3.ein) | 16 | 2 | 3 | 0 | 6 | 0 | 5 |
| [`more/step3.ein`](../../einaf/contract/more/step3.ein) | 11 | 2 | 3 | 0 | 2 | 0 | 4 |
| [`examples/zebra3.1.ein`](../../examples/zebra3.1.ein) | 113 | 19 | 34 | 12 | 0 | **36** | 12 |

The Zebra puzzle's eleven events are its seven kinds and the four facts of the row, which no sentence states;
`zebra3.1.ein` as written has twenty-five more, the memberships of things a sentence names, which quote no
sentence. Of the sixteen word problems, which import their family's word theory, nine carry `unfound_source`
events, one to four each, and no other event: a clause quoted where a sentence states two facts (*"Bob has 5
apples,"*), and an assumption marked by a text no sentence has (*"ASSUMPTION, not in the problem: …"*).

## 5. What the model receives

The items of a prompt, in the order a prompt is assembled, each versioned and hashed; the first and the third are in
every prompt, of every step and every arm, and the tool's function description carries what the filter's prompt
holds of Ein, so the two arrangements differ by no knowledge of the language. *Enforced by* the steps' prompts,
versioned templates under [`einaf/prompts/`](../../einaf/prompts/) rendered by named slots, each naming the section of
this page it implements ([the steps § 2](07-steps.md#2-the-prompts)); the tool's description: nothing yet.

1. **An excerpt of Ein's reference**, cut from pages at a named commit: the forms of
   [the syntax](../ein/user/04-reference/02-syntax.md), the table from intent to word of
   [the reserved names](../ein/user/04-reference/03-reserved-names.md#quick-reference--from-intent-to-word), the
   concept pages of [syntax and semantics](../ein/user/03-syntax-and-semantics/README.md) and
   [natural language](../ein/user/03-syntax-and-semantics/14-natural-language.md) for the texts. Whole pages do not
   fit a prompt, so the item is an excerpt, a file of EinAf's: every Ein block in it loaded by a test, its hash and
   its length in the model's own tokens recorded.
2. **The catalogue of the library**: for each theory of `std.*` a program may select, the module, the import line
   with the names it lists, the facts that activate it with their arguments, the facts it needs beside them, what it
   gives the hypothesis mechanism, and a line of meaning from the rule's own `:why`; no digest of the library, which
   is the record's (§ 6). Each entry is a program the kernel runs. **It holds the library's modules and no family's own**: a
   family's module is its canonical program's, so that where the library has no theory the model's synthesis is
   what is measured; offering one is the widest condition of the library ablation.
3. **Examples: a text beside its program.** The sixteen word problems, each under its text as a comment, each stated
   fact carrying its sentence; [the walkthrough's table](../ein/user/02-guide/06-zebra-walkthrough.md#condition-cross-reference)
   of the fifteen conditions beside their Ein forms; the target shape; this page's examples and the steps by hand,
   each a text with the program each step wrote ([§ 7](#7-the-steps-by-hand)). **An example follows the contract**:
   one query with a `:goal-text`, a sentence that says the answer, and no import of a module the catalogue does not
   hold; a corpus program that differs is an example of the widest library condition only, or is edited, and the
   prompt's copy is what is listed. No item is shown an example of its own text family, but the first experiment, the
   Zebra puzzle shown a working Zebra program, which is labelled so and enters no table. **Never a benchmark instance
   of the validation or test split**: every program a prompt quotes is listed in the record, and a check holds every
   prompt's examples to the split file once there is one.
4. **The problem statement**, with what its instance says of the task: its family's answer line, given to every arm
   in the same words, and for an external item what its adapter tells the formalizer.
5. **Prior diagnostics, if iterative**: the feedback object ([the feedback object](03-feedback.md)) rendered at the
   run's level of the feedback ladder, and, when repairing, the current program. Absent in one shot.

**What the model does not receive**: a benchmark answer; a canonical program beyond the listed examples; any output
of the kernel but the rendered feedback object; a human edit between iterations.

## 6. What a run records

A prompt edit changes a hash, and a changed hash is a new experiment. The layout is the record's; the fields are
these. *Enforced by* the record of a run of the steps ([the steps § 5](07-steps.md#5-the-record)): every call with
its envelope, and the run with its digests, programs and judgements; the record's layout of the harness: nothing
yet.

| the research plan's B4 asks for | the field | today |
|---|---|---|
| model, model version | the file's SHA-256 and what its header says of it, the binding and its llama.cpp | each call's envelope ([the model's runtime § 4](05-model.md#4-the-envelope)) |
| system prompt hash, formalization prompt hash | the hash of each rendered prompt, one a call, and of each template it was rendered from | a run's `steps[].prompt_sha256` and `template_sha256` |
| Ein specification version | the hash of the excerpt, and the commit it was cut at | a run's `excerpt_sha256`; the commit: nothing yet |
| stdlib version | the hash of the catalogue, and the library's manifest digest, `ein --version`'s `stdlib` line | a run's `catalogue_sha256`; the manifest digest is a run's `stdlib`, each step's beside its template's, and the feedback object's `kernel.stdlib`, and no prompt shows it |
| generation parameters | the sampler chain in order, the seed, the context, thinking on or off, the grammar's hash and where it began | each call's envelope, and a run's `steps[].grammar_sha256` |
| *beyond B4* | the kernel's commit, read from git; the program, and each step's program with its judgement at root; the categories, with the step of each form; the writer of H the kernel reported; the feedback object | the kernel's version is `kernel.version`, its commit `kernel.commit`, which the record sets; a run's `steps[].program` and `steps[].judged`, its `program`, `complete` and `categories`; the writer of H is [`einaf::kernel::root`](../../einaf/src/kernel/root.rs)'s, `judged.root.writer` |

## 7. The steps by hand

Five texts beside the worked example, each through the three steps by hand, each rule after the marker of the
property it supplies, `;; missing: ` and its words, as a step's grammar asks: the Zebra puzzle to the target shape;
three word problems over the calculator alone, as the main arms see the library (a computed quantity, a comparison,
and a text that does not give enough to answer); and a seating. Each step's program is a file under
[`einaf/contract/`](../../einaf/contract/), `stepN.ein`, the text beside it as `text.txt`; a step's program is the
steps so far, in synthesis order.

### 7.1 Each step's program, judged

H as `ein show hyps` says it; every step's program at root (`--no-search`) is *not decided*: it loads, it is not
flagged and root is alive. Under `--single`, the outcome and the answer query's lines; *no row* is a class whose goal
has no row (`each_steps_program_is_judged_as_the_page_says`; the complete programs are *one* under every question
that searches, `each_complete_program_is_one_under_every_question`).

| text | step | H | `--single` | the answer |
|---|---|---|---|---|
| [youngest](../../einaf/contract/youngest/) | 1 | `blind`, 7 | *several* | no row |
| | 2 | `blind`, 6 | *several* | no row |
| | 3 | `no-search`, none | *one* | *“Cy is the youngest”* |
| [zebra](../../einaf/contract/zebra/) | 1 | `blind`, 9 896 | not asked | |
| | 2 | `obligations`, 112 | *one* | no row |
| | 3 | `obligations`, 112 | *one* | *“The Norwegian drinks water. The Japanese owns the zebra.”* |
| [seating](../../einaf/contract/seating/) | 1 | `blind`, 89 | *several* | no row |
| | 2 | `obligations`, 18 | *several* | no row |
| | 3 | `obligations`, none | *one* | *“Bob sits in the first chair.”* |
| [apples](../../einaf/contract/apples/) | 1 | `blind`, none | *one* | no row |
| | 2 | `blind`, none | *one* | no row |
| | 3 | `no-search`, none | *one* | *“Bob has (n z 3) apples.”* |
| [together](../../einaf/contract/together/) | 1, 2 | `blind`, none | *one* | no row |
| | 3 | `no-search`, none | *one* | no row: the text does not give Alice's apples, and the answer is one of EinAf's fixed texts |
| [more](../../einaf/contract/more/) | 1, 2 | `blind`, none | *one* | no row |
| | 3 | `no-search`, none | *one* | *“Alice has (n z 7) more apples than Bob.”* |

**What a partial program's root says depends on its relations' count of places, not on what it states**: the
blind enumerator guesses facts of a relation of one or two places, so the Zebra puzzle's step 1, nine verbs of two
places, has nearly ten thousand candidates and is not searched, and a word problem's step 1, whose relations have
four places or more, has none and is *one*, with no row. *One* says nothing of a partial program; the answer
query's row does.

### 7.2 The Zebra puzzle, step by step

The puzzle's fifteen statements and two questions, [`text.txt`](../../einaf/contract/zebra/text.txt). Step 3 is [`examples/zebra3.1.ein`](../../examples/zebra3.1.ein)
form for form and in its order, less its `(config …)`, with each membership of a thing a sentence names quoting the
first sentence that names it; the two have the same outcome, states, H and answer under every question
(`the_zebra_steps_end_at_the_target_shape`). The forms each step writes, by the steps' categories:

| step | it writes | of the steps before |
|---|---|---|
| 1, [`step1.ein`](../../einaf/contract/zebra/step1.ein), 55 forms | 19 statements: *There are five houses* as five memberships of the houses, named by their places (`First` … `Fifth`), and one fact a sentence for the other fourteen, its verb in the sentence's voice (`(drinks Ukrainian Tea)`, `(is-drunk-in Coffee Green)`); the answer query, `(and (drinks ?who_water Water) (owns ?who_zebra Zebra))`; 10 declarations, the nine verbs and `instance`; 25 memberships of the values, each quoting the first sentence that names it, `Water` and `Zebra` the questions | — |
| 2, [`step2.ein`](../../einaf/contract/zebra/step2.ein), 94 forms | 12 declarations: the four passives, the verb a pet's place is said in, `same-house`, `next-to`, the four relations of a kind and a house, and `type`; 7 kinds and their place, `(type Color Attribute)`; the row, four facts the text speaks of and never states; the theory of shared houses: 2 imports, 9 `includes` reading each verb into the link, 2 `symmetric`, `slot-partition` and 2 `slot-spatial`, whose obligations write H | adds to step 1's ontology and changes none of it |
| 3, [`step3.ein`](../../einaf/contract/zebra/step3.ein), 112 forms | the read-back that makes the answer query answerable, all selected: 4 `converse` (an active and its passive), 6 `compose` (a passive of place through the man who lives in the house), 4 `top` and 4 `meet` (a value in a house by its kind's verb); no rule; the mechanism is the obligations step 2 selected, and no form states it | changes nothing |

Step 2's program has one state, the grid placed in `same-house`, and its answer query, asked in `drinks` and `owns`,
no row: the puzzle is solved in the theory's relation and not yet in the question's words.

### 7.3 What each step decided, on each text

| text | which words name one object | sentences and facts | the kinds | step 2 | step 3 synthesised |
|---|---|---|---|---|---|
| youngest | none twice | one fact a sentence; the question's *youngest* a relation | one, `Person` | the library's `transitive` | 1 rule: the youngest is older than nobody, which the library has no rule for; the mechanism `(solve :no-search)` |
| zebra | *the Old Gold smoker* `Old_Gold`, *the man who smokes Chesterfields* `Chesterfields`, *the man with the fox* `Fox`, *the house where the horse is kept* `Horse`, *the green house* `Green`; *snails* `Snail`, *Parliaments* `Parliament`, *orange juice* `Juice` | one fact a sentence, five for *There are five houses* | six: the houses and five kinds of value | 12 declarations, 7 kinds, the row, and the library's theory of shared houses | none |
| seating | *the first chair* `First`, *the middle chair* `Middle`; the third chair no word names, `Third` | the first sentence six facts, three people and three chairs; *does not* a stored negation | two, `Person` and `Chair` | 4 declarations, 3 kinds, the row, the library's theory of shared places | 1 rule: a stored negation of the verb is one of the link, which the library's `meet` does not read back; and the read-back, selected |
| apples | *He* is Bob; *2* is 2 apples, the kind elided; the moments, which no word names: the sentences' order is their time, `Start` and `After` | one fact a sentence | none written | the calculator, which gives the numerals step 1 wrote their meaning | 2 rules: a giver has what was had less what was given, asked of the calculator and read back; `(solve :no-search)` |
| together | as apples, and *they* is Bob and Alice | as apples | none written | the calculator | 6 rules, for the giver, the receiver and the sum; the receiver's start is not stated, so the answer query has no row |
| more | none; one moment, `Now` | one fact a sentence; the question one relation of five places | none written | the calculator | 2 rules: the difference, and *more* where it is past nothing |

**Step 1 is not as simple off the Zebra puzzle as on it.** On the Zebra puzzle the hard part of step 1 is
coreference through descriptions, and every sentence is one fact but the first, which is five. On the word problems step 1 decides things no
word names: the moments a change implies, the kind an ellipsis drops, a pronoun; and a step-1 numeral is a fact of
the file's own `n` until step 2 imports the calculator, which loads in silence either way. On the seating a sentence
is six facts and a negation a stored negation. **Where the work lands moves with the text**: an attribute puzzle's
ontology is written in step 2 (the kinds, the link, the row) and its theory selected, with nothing to synthesise; a
word problem's ontology is complete at step 1, step 2 imports, and every meaning of its words is a rule of step 3,
since the library has the calculator and not the words.

## 8. Held by

[`einaf/tests/contract.rs`](../../einaf/tests/contract.rs), in EinAf's gate step, each program through the bounded
call and its forms through `einaf::kernel::forms`; H read off `ein show hyps` of the binary the gate built
(`$EIN_BIN`, by default `ein.rs/target/gate/ein`):

| test | what it holds |
|---|---|
| `the_worked_example_turns_on_its_mechanism_line` | [§ 2](#2-the-mechanism-line-the-worked-example)'s table and the five forms; no event in any of the example's files |
| `each_steps_program_is_judged_as_the_page_says` | [§ 7.1](#71-each-steps-program-judged)'s table: H, the judgement at root, `--single` and the answer |
| `each_complete_program_is_one_under_every_question` | each step 3 *one* under every reading of every question that searches, and its answer the same under each |
| `each_complete_program_answers_alike_with_its_forms_reversed` | [§ 1.3](#13-the-order-of-the-programs-text), on each complete program |
| `the_zebra_steps_end_at_the_target_shape` | [§ 7.2](#72-the-zebra-puzzle-step-by-step): step 3 against `zebra3.1.ein`, form for form and under every question |
| `the_writer_of_h_the_kernel_module_reads_is_ein_show_hyps` | `einaf::kernel::root`'s writer of H and H's size, which a run of the steps records, against `ein show hyps` on every program of `einaf/contract/` |
| `a_later_step_keeps_the_statements_and_writes_its_own_part` | [§ 3.1](#31-each-step): no statement or answer query rewritten, no form of a later step's part |
| `the_categories_partition_every_form_of_the_reference_programs` | [§ 4](#4-what-each-form-is-the-categories): every form of every reference program in one row with a step, the table's counts and the word problems' `unfound_source` events |
