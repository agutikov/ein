# S2.2.4's steps: the catalogue, the prompts, the check, and the model through the three steps

[S2.2.4](../s2.2.4_passes.md)'s report. **A text becomes an Ein program in three calls of the model**, the facts
first, then the ontology, then the theory, each under its step's grammar, each step's program judged at root, the
complete program asked `--no-search` and `--single`, and **the contract's check** read beside each; or **in one
call**, the whole program, the mode kept for comparison. Steps 2 and 3 select from **a catalogue of the library**,
each entry a program the kernel runs. The code is `einaf::catalogue`, `einaf::prompts`, `einaf::check` and
`einaf::formalize` ([the steps' page](../../../../docs/einaf/07-steps.md)); the records are this folder's
([§ 13](#13-the-files)). Everything was run on 2026-11-26 on the machine the work runs on, on AC power, the GPU free
of any other model; the model `qwen3.5:4b` alone, thinking off, greedy, seed 1, a context of 65 536 tokens and a budget
of 12 288 generated tokens a call, on `7aaefe6f`'s binary; the plan was committed before its first generation
(`cde1100b`, its script fixed in `7ae5bb97` before any call). **The tables are the runs replayed on the final code**
(`replay.sh`): the generations as recorded, the check as it stands at the stage's end.

**What it comes to.**

- **The pieces hold with no model.** Nineteen catalogue entries each show the effect they state under the kernel; the
  three prompt examples answer as their texts ask and are accepted by their steps' grammars; the six texts of the
  steps by hand replay through the steps as one generation a step to the program by hand, each step's writer of H
  and judgement as S2.2.1's table has them; and a recorded run of the model replays to the run it recorded.
- **The model writes a program in three steps that loads, mostly, and the grammars hold every step to its forms.**
  Under the steps' grammars, 16 texts gave 16 complete programs (one of them steps 1 and 2's, step 3 having run into
  its budget), and 14 load; 42 of the 49 calls ended their turn, 7 ran into the budget, the model thinking in comment
  lines (L-M2.16) or repeating a form, each after the grammar refused one of its tokens or with such a loop in its
  prompt (L-M2.19). **Step 1 is what
  the grammar makes it**: every statement quotes a sentence of the text, no stating sentence is left without a fact,
  every relation has its `:why`, one answer query each.
- **What loads is rarely right, and the check sees part of why.** Of the 13 word texts, two print the right count
  among others (a rule that reads what it concludes, with no moment, fires again on its own result: E-124), two a
  wrong count, the rest no line; the Zebra puzzle and the seating are dead at step 2, their cores naming the sentence
  the model misread (E-125). In the complete programs the check finds a calculator's name written as the program's
  own (4 texts), selections that fire nothing at root (the Zebra puzzle's 5), a mechanism stated beside another (1)
  or not stated (3), a relation undeclared or at another count (4 texts), and step-1 statements written again
  otherwise (the seating's people, stated as chairs, written as persons): 10 of the 14 programs that load carry a finding, each loaded in silence. What it does not see:
  a denial written as a statement, a person as a chair, a moment left out (L-M2.18, E-124).
- **The grammars change a run only where they refuse a token** (§ 9): with no grammar the steps give the same answers,
  6 of the 16 texts the same outputs byte for byte, three programs fewer that load, a selection misspelt 11 times (on
  two texts) and a `:source` that is no sentence 7 times (on four), and no call that runs into its budget.
- **The one call does as well on the word problems and worse on the Zebra puzzle**: under the whole grammar's root it
  gives one right answer of 13 word texts (`kinds_full`) where the steps give none alone, and on the Zebra puzzle
  repeats a form from its ninth line to the budget, no program. **B2's mode, by the stage's rule, is the steps**
  (Q-M2.32's default).
- **Q9's default**: the vocabulary the text's own (B), begun in step 1 and completed in step 2, the theory selected
  from the catalogue (C), a module given (A) the escape hatch: every word problem's step 2 selected the calculator
  in its three imports, and no run under A in S2.2.2's probe answered better ([§ 10](#10-q9-the-ontologys-provenance)).

## 1. What was built

| what | where |
|---|---|
| **the catalogue**: nineteen entry programs, each with its meaning, what it gives H, the rules root fires or owes, the writer of H and the answer; the generated part; the minimal catalogue; the catalogue the steps' grammars are rendered on | [`einaf::catalogue`](../../../../einaf/src/catalogue.rs), [`einaf/catalogue/`](../../../../einaf/catalogue/), held by [`einaf/tests/catalogue.rs`](../../../../einaf/tests/catalogue.rs) |
| **root and H, read by the kernel module**: the writer of H the kernel reports and root's firings and owes as values | `einaf::kernel::root` ([`root.rs`](../../../../einaf/src/kernel/root.rs)), held to `ein show hyps` on every program of `einaf/contract/` |
| **the prompts**: the excerpt v2, a template a step and one for the whole program, rendered by named slots; three answer lines; three examples in three steps | [`einaf::prompts`](../../../../einaf/src/prompts.rs), [`einaf/prompts/`](../../../../einaf/prompts/) |
| **the contract's check**: the seven findings, each from the step that could have avoided it | [`einaf::check`](../../../../einaf/src/check.rs) |
| **the steps**: steps 1, 2 (and its second call on a marker) and 3, or the whole program in one call; the assembly in synthesis order; each step's program judged at root with the writer of H and the check; the complete program's outcome and categories | [`einaf::formalize`](../../../../einaf/src/formalize.rs) |
| **the command**: `einaf steps run` (`--model` or `--replay`), `einaf steps check`, `einaf catalogue` | [`steps_cmd.rs`](../../../../einaf/src/steps_cmd.rs) |
| **the page** | [`docs/einaf/07-steps.md`](../../../../docs/einaf/07-steps.md); the contract's, the grammars' and EinAf's index name it |
| **the tests**, EinAf's gate step | [`catalogue.rs`](../../../../einaf/tests/catalogue.rs) (4), [`steps.rs`](../../../../einaf/tests/steps.rs) (5), and one each in `contract.rs` and `prompts.rs` (2) |
## 2. The catalogue

**Nineteen entries, each a program the kernel runs** ([`einaf/catalogue/`](../../../../einaf/catalogue/)), in five
groups; what each selection gives the hypothesis mechanism is the entry's own line, and the writer of H the kernel
reports on its program:

| group | entries | what the selection gives H | writer of H of the entry program |
|---|---|---|---|
| `std.algebra`, derivations | `(transitive R)`, `(symmetric R)`, `(includes R S)`, `(converse R S)`, `(compose R S T)`, `(top R isa Dom Ran)`, `(meet R S T)` | nothing | `no-search` (each entry states it) |
| `std.algebra`, checks | `(functional R)`, `(injective R)` | nothing; a state that breaks one is dead | `hypothesis-rules` (each entry writes an `(open F)` the check kills) |
| `std.algebra`, an obligation | `(total-owed R isa)` | the obligation's candidates are H | `obligations` (2 candidates) |
| `std.slots` | `(slot-partition R isa sub Super Index)`; `(slot-spatial R S isa Index)` | the obligations propose every placement | `obligations` |
| `std.bijection` | `(bijective R)` with `(bijection-hierarchy isa)` | the obligations propose R's facts | `obligations` |
| `std.contradiction` | the import | nothing | `no-search` (its entry is `inconsistent`) |
| the calculator | `(add X Y)` → `(sum-of X Y S)`, `(sub X Y)` → `(diff-of X Y D R)`, `(mul X Y)` → `(product-of X Y P)`, `(div X Y)` → `(quot-of X Y Q R)`, `(cmp X Y)` → `(cmp-of X Y O)`, each stated by a rule and read back | nothing | `no-search` |

**Its sizes** (`einaf catalogue --tokens MODEL`, the model's vocabulary): **the whole catalogue 19 416 bytes, 5 906
tokens**, `sha256:93d7869f…acd8b3`; **the minimal one** (`--minimal`, `std.algebra`'s ten entries and the
contradiction's) **8 640 bytes, 2 634 tokens**, `sha256:584f827b…dc1dd7`; both carry the library's manifest digest
`sha256:292894be…ff83d681` on their first lines. **The catalogue the steps' grammars are rendered on**
(`einaf catalogue --grammar`): seven modules, `std.algebra`'s ten selecting heads and the relation `bijective` its
rules read, `std.slots`' two, `std.bijection`'s two, the calculator's ten names; where S2.2.3's stand-in offered every
module and every rule (the calculator's `std.math.positional` alone 97 activators).

**Its test** ([`einaf/tests/catalogue.rs`](../../../../einaf/tests/catalogue.rs), 4 tests, 8.6 s on the gate's
profile): each entry program through the bounded call (its outcome under `--single` and its answer query's lines)
and `einaf::kernel::root` (the writer of H, each rule it says root fires or owes), every selection of it active by the
contract's check; its text cut from the programs, with none of `` ` `` `<` `>` `|`; the minimal catalogue a part of
the whole; the grammars' catalogue narrower than the library's, module by module; the generated part.

## 3. The prompts

Versioned files under [`einaf/prompts/`](../../../../einaf/prompts/), each template's `%%` lines naming the section
of [the contract](../../../../docs/einaf/04-contract.md) it implements; lengths in the model's tokens:

| file | what it holds | bytes | tokens |
|---|---|---:|---:|
| `ein-excerpt.v2.md` | the excerpt: words (the narrower name set), forms (`config`, `trace`, `macro` named as forms a program does not write), facts and relations, rules, the answer, the hypothesis mechanism, the library in brief, what the kernel refuses; its examples a kinship and a product of seats | 9 967 | 2 907 |
| `steps/system.v1.md` | the system message of every request: one sentence and the excerpt | 416 | 102 |
| `steps/step1.v1.md` | step 1: the statements, the answer query, the ontology partial; the examples to step 1, the text, the answer line | 2 564 | 659 |
| `steps/step2.v1.md` | step 2: the ontology whole, the imports, the selection, the marker; the catalogue, the examples to step 2, the text, step 1's output | 1 957 | 492 |
| `steps/step2b.v1.md` | step 2's second call: the rules of each marker; the markers, the catalogue, the examples to step 3, the text, the program | 1 056 | 275 |
| `steps/step3.v1.md` | step 3: the rest of the selection, the rules each after its marker, the mechanism; the catalogue, the examples to step 3, the text, the program | 1 852 | 461 |
| `steps/whole.v1.md` | the whole program in one call; the catalogue, the examples whole, the text, the answer line | 1 633 | 392 |
| `answer-lines/{zebra,word,generic}.v1.md` | who drinks water and who owns the zebra; how many; a sentence that answers the question | 84 to 113 | |
| `examples/{shops,buying,ages}/` | three stalls in a row (an attribute puzzle over `std.slots`, the read-back through the stall), a purchase (`mul`), an age (`sub`), each step's own output; their programs 1 293, 514 and 512 tokens | | |

**What a request comes to** (the envelopes' prompt tokens, the chat template rendered): step 1 about 4 640 tokens,
step 2 about 11 600, step 3 about 12 400 for a word problem and 13 900 for the Zebra puzzle (the program so far in it),
the one call about 11 700; a step that follows one that ran to its budget carries that output, up to 25 300. All fit
the context of 65 536.

## 4. The runs

**The plan** ([`plan.tsv`](plan.tsv), 64 runs, committed before the first generation): the sixteen texts
([`texts/`](texts/): the Zebra puzzle, the ordering and the seating of the steps by hand, the thirteen texts of the
sixteen word problems, `gap_assume`'s and the `kinds_*` that share a text asked once) under four conditions: **steps**,
the three steps each under its step's grammar inside the fence; **whole**, the program in one call under the whole
grammar's root inside the fence; **steps-free** and **whole-free**, the same two with no grammar, which the room
allowed. Each run is one command, `einaf steps run TEXT --id ID --answer-line LINE --out DIR --model MODEL [flags]`,
which [`run.sh`](run.sh) issues a line of the plan; the answer line is `zebra` for the Zebra puzzle, `word` for the
word problems and `generic` for the ordering and the seating. **Each run's record** is a folder under
[`runs/`](runs/), `CONDITION/ITEM/`: `calls.jsonl.gz`, every call whole (the request, the generation, its envelope);
`run.json.gz`, the run (§ 5 to 9 are read from it); each step's output, `stepN.out.ein`; the complete program,
`program.ein`; `summary.txt`, what the command printed. [`tables.md`](tables.md) is every table, written by
[`analyse.py`](analyse.py) from the records; [`replay.sh`](replay.sh) writes the runs again from their calls with no
model.

**What the calls cost.** Under the steps' grammars, 49 calls (16 texts, one step 2's second call), 1 745 s of
generation; under the one call, 16 calls, 376 s. A call that runs into its budget, 12 288 tokens, takes about four
minutes.

## 5. Step by step, under the steps' grammars

The condition **steps**; every row of every table is in [`tables.md`](tables.md#steps).

| step | calls | its output parses | at root (`--no-search`) | the writer of H at root |
|---|---:|---:|---|---|
| 1 | 16 | 16 | not decided 14, invalid 2 | `blind` 14 |
| 2 | 16 | 16 | not decided 12, inconsistent 2, invalid 2 | `blind` 12, `obligations` 1, `stuck` 1 |
| 2's second call | 1 (`boxes13`) | 1 (comments alone, to the budget) | not decided | `blind` |
| 3 | 16 | 15 (`boxes12`'s cut by the budget, L-M2.17) | not decided 11, inconsistent 2, invalid 2 | `no-search` 10, `obligations` 1, `blind` 2 |

**Step 1** (the acceptance's checks; the table *Step 1, what the acceptance asks* in `tables.md`). Every output parses
and holds only step 1's forms: no import, no rule, no `solve`. **Every statement's `:source` is a sentence of the
input** (the grammar quotes them), and **no sentence that states something is left without a fact** on any of the
sixteen texts. **Every relation carries a `:why`**. **One answer query each**, every reference its goal binds; its
goal is written in the statements' own relations on 8 of 16 (on the others it reads a relation only a rule will
conclude, `is-youngest`, `need-boxes`, or the calculator's `sum-of` and an unnamed `gt`). Two step-1 programs are
refused at root: their answer query reads a relation the program never names (`more`'s `(gt …)`, `boxes13`'s
`need-boxes`). **Against the steps by hand** (forms compared with their `:source` and layout taken out): the Zebra
puzzle's 2 of 19 statements and none of its 25 memberships are the hand's (the model names the coloured houses
`RedHouse`, `GreenHouse`, and makes them houses; it writes one verb for the active and the passive, `drinks` for *is
drunk in*); the ordering's 0 of 2 (`is-older-than` for `older`); the seating's 3 of 9; the word problems' none (the
model writes no moment: `(has Bob (n z 5) Apple)` for the hand's `(has Bob (n z 5) Apple Start)`). **The
coreferences** the hand found: *he* is Bob in every word text; the Zebra puzzle's descriptions are names of their
own (`ChesterfieldsHouse`, `FoxOwner`, `HorseKeeperHouse`), where the hand reads each as the thing it is built on; the
seating's people are written as chairs (L-M2.18).

**Step 2.** Every import is an entry's module, and every selecting fact an entry's (the check's
`selection-not-in-catalogue`, 0); every word text imports the calculator in its three lines, as the entries write
them, and the Zebra puzzle and the seating `std.algebra` and `std.slots`. **What step 2 changed of step 1's
ontology** is additions only: no declaration dropped or declared otherwise on any text (the Zebra puzzle kept 33
forms and added 25: `type`, the kinds, the row, the read-back's pairs). **The marker**: one step 2 wrote one
(`boxes13`), and its second call wrote comments to the budget. **Selections not active at step 2**: none of the text's
reading; of the narration's, none at step 2 (the Zebra puzzle's are step 3's). The writer of H after step 2:
`obligations` for the seating, `stuck` for the Zebra puzzle (root dead: § 7), `blind` for every word text.

**Step 3.** Every rule a step 3 wrote stands after its marker (the grammar requires it): 16 rules on 10 texts, the
calculator's request and its read-back, two a text on six of them, one on `kinds`, `kinds_full`, `more` and
`boxes12` (whose second was cut by the budget). **The
writer of H is the one the program states on 10 of the 14 programs that load**: `no-search` where it wrote `(solve
:no-search)` (9), `obligations` for the seating, which wrote nothing for the mechanism as its example does; **the Zebra puzzle stated
`(solve :no-search)` beside its selected obligations**, and the writer is `no-search` (`mechanism-not-the-one-stated`);
`boxes12`, `boxes13` and `pencils` state none, and **the blind enumerator wrote H** (`mechanism-not-stated`), their
`--single` *several*.

## 6. The assembled programs

The complete program of each text, in synthesis order ([`runs/steps/ITEM/program.ein`](runs/steps/)), asked
`--no-search` and `--single`:

| text | `--single` | the answer query's lines | the check's findings |
|---|---|---|---|
| zebra | inconsistent | none | `selection-inactive` 5, `mechanism-not-the-one-stated` 1 |
| youngest | one | no row | `undeclared` 4 |
| seating | inconsistent | none | `rewritten` 3 |
| bob1 | one | *Bob has 1 / 3 / 5 apples.* | none |
| gap | one | no row | `name-not-listed` 1 |
| gap_fact | one | no row | `name-not-listed` 1 |
| contra | one | *Bob has 5 apples.* (the text gives no answer) | none |
| unused | invalid (`has` declared twice, E-104) | — | — |
| bystander | one | *Carol has 1 / 3 / 5 / 7 / 9 apples.* | none |
| before | one | *Bob had 1 / 3 / 5 apples before.* (the gold is 7) | none |
| kinds | one | no row | `name-not-listed` 1 |
| kinds_full | one | no row | `name-not-listed` 1 |
| more | invalid (a goal on `gt`) | — | — |
| boxes12 | several (H `blind`; step 3 cut, the program steps 1 and 2's) | no row | `mechanism-not-stated` 1, `wrong-count` 1 |
| boxes13 | several | no row | `mechanism-not-stated` 1, `undeclared` 7, `wrong-count` 1 |
| pencils | several | no row | `mechanism-not-stated` 1, `undeclared` 3, `number-as-name` 2 |

**The outcomes among the seven**: *one* 9, *several* 3, *inconsistent* 2, *invalid* 2. **The answers**, read by hand
against the word problems' README (the answer is S2.2.5's acceptance, not this stage's): none right alone; `bob1` and
`bystander` hold the right count among others (E-124), `before` and `contra` a wrong one. **The check's findings in each step's program** (the table at the end of `tables.md`; a finding a step's program
holds is held again by every later step's): step 1's programs, `undeclared` 9, `wrong-count` 1, `number-as-name` 1;
step 2's, `undeclared` 14, `name-not-listed` 4, `rewritten` 3, `wrong-count` 2, `number-as-name` 2; the second call's
(`boxes13`), `undeclared` 7, `wrong-count` 1; step 3's, as the table above, with `selection-inactive` 5
and the mechanism 3 (`boxes12`'s judged on its complete program). **Every one of them the kernel loads in silence**:
of the 14 complete programs that load, 10 carry a finding.

**The categories** (each form in one row with the step that wrote it, `tables.md`): the word texts' programs are 2 to
5 source-derived facts, the calculator's three imports, one or two generated rules and the frame, and no generated
assumption but `boxes13` and `pencils`'; the Zebra puzzle's 50 / 25 / 9 / 0 / 12 / 8 (source-derived, generic theory,
generated relations, rules, assumptions, frame), against the hand's 44 / 34 / 12 / 0 / 11 / 11.

## 7. The Zebra puzzle in detail

**Under the steps' grammars** ([`runs/steps/zebra/`](runs/steps/zebra/)): step 1 wrote 53 forms (19 statements, the
answer query, 7 declarations, 26 memberships), step 2 69 (the ontology whole, 33 of step 1's forms kept and 25 added;
`std.algebra` and `std.slots`; 9 selecting facts), step 3 15 (four `top`, five `meet`, four `converse`, and `(solve
:no-search)`). **How far each step is from the hand's** (forms compared with `:source` and layout taken out, and the
relations each writes):

| step | forms | as the hand's | the hand's | the relations the model writes | the hand's |
|---|---:|---:|---:|---|---|
| 1 | 53 | 4 | 55 | `drinks`, `instance`, `is-immediately-to-the-right-of`, `lives-in`, `owns`, `smokes` | the same and `are-smoked-in`, `are-smoked-next-to`, `is-drunk-in`, `lives-next-to` |
| 2 | 69 | 16 | 74 | `includes`, `instance`, `is-immediately-to-the-right-of`, `slot-partition`, `slot-spatial`, `symmetric`, `type` | the same |
| 3 | 15 | 0 | 18 | `converse`, `meet`, `top`, `solve` | `compose`, `converse`, `meet`, `top` |
| the program, against `zebra3.1.ein` | 104 | 18 | 113 | | |

**Where it went wrong, step by step.** **Step 1** folds the passives into the actives (*coffee is drunk in the green
house* is `(drinks Coffee GreenHouse)`), and *next to* into *lives in* (`(lives-in Norwegian BlueHouse)` for *the
Norwegian lives next to the blue house*), so two of the clues say the wrong thing in the right words; and it names a
coloured house as a house of its own (`GreenHouse`, `RedHouse`, five more house names beside `House1` to `House5`).
**Step 2** selects the right theory, spelled right: `(slot-partition same-house instance type Attribute House)`, two
`slot-spatial` lines, `includes` of the verbs into `same-house`, `symmetric`; the kinds it adds are `Person`, `Thing`,
`Smoker`, `Drinker`, `Owner`, `Color` and `House`, and only `Person`, `Thing` and `House` have members. **Root is dead
at step 2**: eleven houses for five places and four kinds with no member, either fault alone deadly (L-M2.23, found by
the phase report's re-runs), the core naming `(instance GreenHouse House)` with its sentence, *Coffee is
drunk in the green house.*, refuted by `std.slots.slot-no-fill` (E-125). **Step 3** writes the read-back over the kinds
with no members (`selection-inactive` 5, E-123) and `(solve :no-search)` beside the obligations it selected, which
leaves H empty (`mechanism-not-the-one-stated`). The complete program is *inconsistent* under both questions.

**In one call** ([`runs/whole/zebra/`](runs/whole/zebra/)): five memberships of the houses, then the fact
`(source-instance House-1 :source "The Norwegian lives in the first house.")` from the output's ninth line, 718 times,
to the budget; the text cut mid-form does not parse (L-M2.17). The model also copied the assembled program's heading
(*;; ---- step 1: the statements and the answer query*) from the examples, which the one call shows whole.

## 8. The one call against the steps, and B2's mode

Both under a grammar (the steps' grammars; the whole grammar's root):

| | the steps | the one call |
|---|---|---|
| a complete program | 16 of 16 | 15 of 16 (the Zebra puzzle's loops to the budget) |
| it loads | 14 | 14 |
| `--single` | one 9, several 3, inconsistent 2, invalid 2 | one 11, inconsistent 3, invalid 1 |
| the Zebra puzzle | a program, dead at root, its core naming the misread sentence | no program |
| the word texts' answers, by hand | none right alone; 2 the right count among others, 2 a wrong one | 1 right (`kinds_full`, *They have 8 apples together.*); 1 among others, 2 wrong |
| complete programs with a finding | 10 of 14 that load | 12 of 14 |
| findings in the complete programs | 34 | 111 |
| `:source` that is no sentence | 0 (step 1's grammar quotes the text) | 16, on 5 texts |
| calls, generation | 49, 1 745 s | 16, 376 s |

**The default taken for B2's mode: the steps**
([Q-M2.32](../open_questions/formalizer/Q-M2.32-which-mode-does-baseline-b2-run-with.md)). The stage's rule: the
steps, unless the one call is no worse on both materials. On the word problems it is no worse (one right answer
against none, the rest alike: no line or a wrong one); **on the Zebra puzzle it is worse**, no program against a
program whose judgement names what to repair. The steps also hold every `:source` to a sentence and close steps 2b
and 3's vocabulary, which the one call's findings show it does not have.

## 9. With no grammar

The same requests with no grammar (the conditions **steps-free** and **whole-free**; the grammar is the one difference
from **steps** and **whole**):

| | steps | steps-free | whole | whole-free |
|---|---|---|---|---|
| a complete program | 16 | 16 | 15 | 15 |
| it loads | **14** | 11 | 14 | 14 |
| `--single` | one 9, several 3, inconsistent 2, invalid 2 | one 8, several 1, inconsistent 2, invalid 5 | one 11, inconsistent 3, invalid 1 | the same |
| the answers | as § 6 | the same four lines (`bob1`, `bystander`, `before`, `contra`) | as § 8 | the same |
| calls that ran to the budget | 7 of 49 | **0 of 48** | 1 of 16 (the Zebra puzzle) | 1 of 16 (the Zebra puzzle) |
| texts whose every output is the free run's | 6 of 16 | | 15 of 16 | |

**A grammar held by the chain's choice first changes a generation only at the tokens it refuses**: the envelope's
`chosen_again` is 0 on the Zebra puzzle's, the seating's, `before`'s, `bystander`'s, `contra`'s and `gap_fact`'s
three steps, and their outputs are the free runs' byte for byte; under the whole grammar's root it refused a token
in two calls of sixteen. **Where the steps' grammars refused, they did two things.** They kept four programs from a
refusal: with no grammar, step 2 binds the calculator's `sum-of` twice, the program's own declaration and the import's
list (`full-name`, `gap`, `kinds`), and step 3 writes a second `(solve …)` (`claim`, `boxes12`, `boxes13`); `unused`'s
two `has` are refused under both (E-104), and `more`'s goal on `gt` loads with no grammar only because its free step 3
writes `(gt ?a ?b)` in a rule, which step 3's grammar, closed over the declared relations, does not allow: five
complete programs refused with no grammar, two under it. **And they sent seven calls into a loop to the budget** that the free runs did not enter
(L-M2.19): `youngest`'s step 2, `unused`'s steps 2 and 3, `boxes12`'s step 3, `boxes13`'s second call and step 3,
`pencils`' step 3, each ending its turn with no grammar in 27 to 607 tokens. **What the steps' grammars kept out**, by the check's findings in
the free steps' programs and none in the steps' (each counted once a run): an activating fact under a name the
import does not list (`selection-not-listed`, 11, on two texts), a `:source` that is no sentence (7, on four) and a
second query with a `:goal-text` (3, on three). **What a grammar does not change**: the answers. The four
lines the steps print are the free steps' four; the one call's answers are the free call's.

## 10. Q9, the ontology's provenance

**The default taken: B with C in the main arms, A the escape hatch**, written into
[Q9](../../open_questions/formalizer/Q9-ontology-provenance.md) as S2.2.4's default and left open. **B**, the vocabulary
from the text: every word text's step 1 names its things and verbs from the text and quotes every sentence; what it
misses is no word of the text but the moment a change implies, which the hand makes up and the model leaves out
(E-124). **C**, the catalogue: every word text's step 2 imports the calculator as the entries write it, the Zebra
puzzle's selects `std.slots` and `std.algebra`, spelled right, and no run of the steps imports or selects outside it;
what fails is a calculator's name the import does not list (four texts, `name-not-listed`). **A**, a module given: no
run of this stage; S2.2.2's probe gave the thirteen word texts the corpus's word theory (*a theory given*): 13 parsed,
11 loaded, 10 *one*, and none printed the answer the corpus's README states. A is not better where it was tried, and
B with C reaches every part but the moments.

## 11. What the stage found

**Of Ein**, in [the phase's evaluation](../evaluation/README.md):

- **E-123** (gap, `diagnostics`): no value says whether a selecting fact was taken up; a misspelt selection, one over
  labels with no members and a read-back that fires only above root look alike at root, no firing.
- **E-124** (limit, `answer`): one state may answer a question of one value with a row for each pass of a rule that
  reads what it concludes; `bob1`, `bystander`, `before` are *one* with three, five and three rows.
- **E-125** (strength, `diagnostics`): a dead root's core names the sentence the model misread and the rule that found
  it (the Zebra puzzle's coloured house, the seating's people as chairs, a number written as a name).
- **Confirmed**, a line each: **E-11** on a relation (the calculator's `sum-of` written as the program's own, eight
  runs); **E-12** (an undeclared relation or another count in 4 of 16 complete programs of the steps and 10 of 15 of the
  one call); **E-104** at one count (`unused`, `has` declared with two label lists, refused once step 2 imports);
  **E-27** answered from EinAf's side by the catalogue, with what it had to say that no page says in one place
  (`top` and `meet` read a value back through its place; a read-back's names are listed by step 2's import); **E-39**
  read around by `einaf::kernel::root`.
- **Which selections the model misspelt, and whether the kernel could have told it**: none of the catalogue's
  activating facts was misspelt in a run of the steps (the step 2 grammar offers only the entries' heads); the misses
  are a calculator's relation the import does not list (`sum-of`, eight runs in the two conditions) and selections
  over kinds with no members (the Zebra puzzle). The kernel said nothing of either: the programs load, and the answer
  query has no row.
- **What the judgement of a partial program at root said**: *not decided* for every step-1 program that loads (root
  alive, nothing more: E-114); *inconsistent*, with a core naming the misread sentence, for the Zebra puzzle's and the
  seating's step 2 (E-125); a refusal, with its place, for a step-1 goal on a relation the program never names. It
  said nothing of a step-2 program whose selection waits for step 3's read-back.

**Of EinAf and the process**, in [the phase's findings](../findings/README.md): **L-M2.16**, with thinking off the
model thinks in comment lines and loops there to the budget; **L-M2.17**, an output the budget cuts under a grammar
does not parse and ends the run; **L-M2.18**, a sentence's polarity and a thing's kind go wrong in step 1 with
nothing to see them; **L-M2.19**, a grammar's refusal can send greedy decoding into a loop it does not enter with no
grammar (7 of 49 calls under the steps' grammars ran to the budget, none of 48 with none). **Closed**: L-M2.6 (the writer of H, `einaf::kernel::root`), L-M2.9 (the excerpt's family),
L-M2.12 (the grammars' catalogue), L-M2.13 (the marker in the steps by hand). **For the user**, in
[the phase's open questions](../open_questions/README.md): **Q-M2.31** (does a step's grammar bound the comment lines?
default: no) and **Q-M2.32** (B2's mode; default: the steps); and Q9's default, above. **Q-M2.6** (a grammar a step)
and **Q-M2.29** (step 2's facts held by the check) gain this stage's measurements, their defaults standing.

## 12. The commands, with their outputs

From the repository's root, `CARGO_HOME` set where the host's rules say, `M` the model's file
(`/var/lib/ollama/blobs/sha256-a9d0a3e1d8c91732a3812becd31f8d4a8ff57414903f226306069f0437e2ac50`). Each output below is
what the command printed on `6953c6f5`'s binaries; the two runs with the model were made again after the plan's, and
their outputs and programs are the records' byte for byte.

**Build EinAf with the model's runtime** (the gate's build, `--profile gate`, needs neither CUDA nor the model):

```sh
CMAKE_CUDA_ARCHITECTURES=89 cargo build --manifest-path einaf/Cargo.toml --release --features cuda
```

<!-- output -->
```text
    Finished `release` profile [optimized] target(s) in 13.71s
```

**The steps on the Zebra text, with the model** (6 min 1 s, the gate running beside it):

```sh
einaf/target/release/einaf steps run plans/m2_nl_to_ir/p2.2_formalizer/steps/texts/zebra.txt \
    --id zebra --answer-line zebra --out /tmp/zebra --model $M
```

<!-- output -->
```text
zebra: steps mode, grammar on, whole
  step 1     4787 prompt tokens, 1032 generated (end-of-turn), 53 forms; at root not decided; H from blind (7840); 0 findings
  step 2     12627 prompt tokens, 1160 generated (end-of-turn), 69 forms; at root inconsistent; H from stuck (0); 0 findings
  step 3     13869 prompt tokens, 308 generated (end-of-turn), 15 forms; at root inconsistent; H from no-search (0); 6 findings
  complete, --no-search: inconsistent
  complete, --single: inconsistent
  findings: mechanism-not-the-one-stated 1, selection-inactive 5
```

**The steps on a word problem, with the model** (25 s):

```sh
einaf/target/release/einaf steps run plans/m2_nl_to_ir/p2.2_formalizer/steps/texts/bob1.txt \
    --id bob1 --answer-line word --out /tmp/bob1 --model $M
```

<!-- output -->
```text
bob1: steps mode, grammar on, whole
  step 1     4636 prompt tokens, 142 generated (end-of-turn), 5 forms; at root not decided; H from blind (0); 0 findings
  step 2     11589 prompt tokens, 122 generated (end-of-turn), 5 forms; at root not decided; H from blind (0); 0 findings
  step 3     12351 prompt tokens, 230 generated (end-of-turn), 3 forms; at root not decided; H from no-search (0); 0 findings; 2 markers
  complete, --no-search: not decided
    Bob has (n z 1) apples.
    Bob has (n z 3) apples.
    Bob has (n z 5) apples.
  complete, --single: one
    Bob has (n z 1) apples.
    Bob has (n z 3) apples.
    Bob has (n z 5) apples.
  findings: none
```

**The same run replayed, with no model** (from the gate's fixture; any run's `calls.jsonl` replays the same way):

```sh
einaf/target/gate/einaf steps run plans/m2_nl_to_ir/p2.2_formalizer/steps/texts/bob1.txt \
    --id bob1 --answer-line word --replay einaf/tests/fixtures/steps/bob1/calls.jsonl --out /tmp/bob1-replay
```

It prints `einaf: replaying 3 recorded calls` on stderr and the same lines as above, and writes `calls.jsonl`,
`run.json`, `step1.out.ein` to `step3.out.ein` and `program.ein`.

**The contract's check on a program**, the Zebra puzzle's steps by hand and the model's `gap`:

```sh
einaf/target/gate/einaf steps check einaf/contract/zebra/step3.ein --text einaf/contract/zebra/text.txt
einaf/target/gate/einaf steps check plans/m2_nl_to_ir/p2.2_formalizer/steps/runs/steps/gap/program.ein \
    --text plans/m2_nl_to_ir/p2.2_formalizer/steps/texts/gap.txt
```

<!-- output -->
```text
einaf/contract/zebra/step3.ein: H from obligations (112 candidates); the program states obligations
selection-inactive (2, from step 2): root's saturation names no firing of compose with these arguments and no owe of it
    (compose is-kept-in is-lived-in-by is-owned-by)
selection-inactive (2, from step 2): root's saturation names no firing of compose with these arguments and no owe of it
    (compose is-owned-by lives-in is-kept-in)
2 findings
plans/m2_nl_to_ir/p2.2_formalizer/steps/runs/steps/gap/program.ein: H from no-search (0 candidates); the program states no-search
name-not-listed (2, from step 2): the program writes sum-of, which std.math.decimal-arith declares and its import does not list: the module's facts are std.math.decimal-arith.sum-of, and the program's sum-of is its own
1 findings
```

**The catalogue**, as a prompt shows it (`einaf catalogue`, 19 416 bytes; `--minimal`, 8 640), the one the steps'
grammars are rendered on, and its test:

```sh
einaf/target/gate/einaf catalogue --grammar
cargo test --manifest-path einaf/Cargo.toml --profile gate --test catalogue
```

<!-- output -->
```text
std.algebra: 10 names; selected by compose/3 converse/2 functional/1 includes/2 injective/1 meet/3 symmetric/1 top/4 total-owed/2 transitive/1; read, declared by none: bijective/1; values: none
std.bijection: 0 names; selected by none; read, declared by none: bijection-hierarchy/1 bijective/1; values: none
std.contradiction: 0 names; selected by none; read, declared by none: none; values: none
std.math.arith: 1 names; selected by none; read, declared by none: none; values: 1 2 4 5 6 7 9 Number gt
std.math.decimal: 1 names; selected by none; read, declared by none: none; values: 0 1 2 3 4 5 6 7 8 9 Number gt
std.math.decimal-arith: 10 names; selected by none; read, declared by none: none; values: 1 2 4 5 6 7 9 Number gt
std.slots: 0 names; selected by none; read, declared by none: slot-partition/5 slot-spatial/4; values: none

running 4 tests
test the_generated_part_holds_every_module_and_its_rules ... ok
test the_catalogue_s_text_is_cut_from_the_entry_programs ... ok
test the_grammars_catalogue_is_the_entries_and_narrower_than_the_library ... ok
test every_entry_is_run_by_the_kernel_and_shows_its_stated_effect ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.89s
```

**The plan's runs, and the tables**: `EINAF_MODEL=$M sh plans/m2_nl_to_ir/p2.2_formalizer/steps/run.sh` (`ONLY=steps`
for one condition; a run recorded is not made again), `sh replay.sh` (every run again from its calls, no model), and
`python3 analyse.py` (writes `tables.md`).

## 13. The files

| file | what |
|---|---|
| [`plan.tsv`](plan.tsv), [`run.sh`](run.sh) | the plan of the 64 runs, committed before the first generation, and the script that makes each |
| [`texts/`](texts/) | the sixteen texts |
| [`runs/CONDITION/ITEM/`](runs/) | a run's record: `calls.jsonl.gz` (every call whole, `einaf-generation/1`), `run.json.gz` (the run, `einaf-steps/1`, as replayed on the final code), `stepN.out.ein`, `program.ein`, `summary.txt`, `stderr.txt` (the model's load), `replay.stderr.txt` |
| [`replay.sh`](replay.sh) | every run again from its calls, with no model (it unpacks `calls.jsonl.gz`) |
| [`analyse.py`](analyse.py), [`tables.md`](tables.md) | every table, read off the records |
