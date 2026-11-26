# P2.2 — the phase against its goals: an analysis

**What this is.** A reading of [the phase report](README.md) against the goals the phase set itself, written on
the user's request of 2026-11-26 (*"What were the phase goals, were they met, full yes/no or partially, how and
why"*), on `60964047`. It read the report, [the phase's plan](../README.md), the six stage records, the stage
reports and tables, the model's programs and the prompts; it re-ran five commands
([§ 7](#7-what-was-checked-here-and-what-was-not)). It rules on nothing and closes nothing.
[§ 8](#8-the-one-who-for-two-questions) and [§ 9](#9-proposals-for-the-other-issues) were added the same day, on
the user's question about the one `?who` and the word *"propose solutions for this and other issues and add to the
same file"*: what to do about each issue, as proposals. [§ 10](#10-the-model-solves-the-puzzle-and-cannot-state-it)
was added after them, on the user's question whether Ein interferes with a reasoning that was right, and
[§ 11](#11-the-three-steps-against-the-one-call-and-how-the-steps-are-cut) on the user's question about the three
steps against one call and a strict split of them. [§ 12](#12-what-a-library-says-of-its-rules-to-whoever-selects-them)
was added on 2026-11-27, on the user's note that a library should carry a description of its rules for whoever
selects them.

## Summary

**Partially met. What the phase was to build and to measure is there and holds when re-run; the one thing it was
to achieve with the model is not.**

- **Of the phase's twelve acceptance items: 8 met, 3 partially, 1 not met** ([§ 2](#2-the-acceptance-item-by-item)).
- **Not met: the Zebra puzzle from its text to its answer, one shot.** `einaf from-text` prints *The statements
  contradict each other.*, not the Norwegian and the Japanese. This is also the first attempt at the milestone's
  minimal goal, which stays open for the loop.
- **The machinery is not what failed.** The steps written by hand go through the same code to *The Norwegian
  drinks water. The Japanese owns the zebra.*, scored right, in the gate. What failed is the model's reading of
  the text.
- **The miss is far, not near.** The model's program shares 18 of the reference's 113 forms and has at least
  seven independent defects across all three steps. With the two faults the report names removed *and* the
  search allowed, R is still empty ([§ 3](#3-why-the-zebra-one-shot-missed); my run).
- **On the word problems no program computes a right answer.** The three texts read *right* each print the
  undetermined sentence because of a slip, and the same slip prints the same sentence where the gold is a number.
- **Why:** a 4.2B model, thinking off and greedy, copies the shape its examples show and gets the theory's
  selection right; it does not make the reading decisions of step 1 (what a description names, *next to*, the
  kinds), and those errors are silent until step 2, in a one shot that by design never looks back. The plan's own
  risk list predicted the miss and assigned the second chance to [P2.4](../../p2.4_loop/README.md).
- **Five places where the evidence reads harder than the report says** are in
  [§ 5](#5-where-this-reading-differs-from-the-reports).
- **What to do about it**: seventeen proposals, [§ 9](#9-proposals-for-the-other-issues). The first answers the
  user's question on the one `?who` ([§ 8](#8-the-one-who-for-two-questions)): **a query for each question is
  right, and the model wrote it so itself** in S2.2.2's probe, before the contract asked for one query. Most of
  the rest move a decision from the prompt, where the model ignored it, into a grammar or into EinAf's check.
- **The model solves the puzzle in its thinking and cannot state it**
  ([§ 10](#10-the-model-solves-the-puzzle-and-cannot-state-it)). The filter as run, one shot, does lose to the
  model's own reasoning on this puzzle: it switches that reasoning off and asks for a translation instead. That is
  one arm at zero feedback, not the kernel. The model recognised the puzzle by name, so the Zebra text is a poor
  place to look for help; and what it lacked alone was a verdict on its grid, which Ein gives today in 0.04 s,
  naming the clue a wrong grid breaks. Seven further steps, the first to measure the model alone properly.
- **The three steps against the one call** ([§ 11](#11-the-three-steps-against-the-one-call-and-how-the-steps-are-cut)):
  no better answers; with no grammar the steps load fewer programs (11 against 14), from slips between steps. Their
  gain is the grammars a step boundary allows. The *partial* ontology is written twice and never revised, and the
  one time a step did revise it the program kept both versions; the *partial* theory writes an import's list a
  step before its uses. **A strict split was not investigated**; proposal 25 measures it, and the steps by hand
  cut strictly here load at every cut but one.
- **What a library says of its rules** ([§ 12](#12-what-a-library-says-of-its-rules-to-whoever-selects-them)): the
  user's ranking of the rules by how hard they are to understand is the model's record, rule for rule (`converse`
  four written and four wrong, `compose` never, `top` and `meet` a copied shape). The catalogue does describe each
  rule, but by what it derives, not by what it is for or what its arguments need; those words are EinAf's, not the
  library's; and the modules given beside the library, shown with no description, were imported by no run.
  Proposal 26: a rule described for its use, by the library, with a first draft for the eight rules.

## 1. The goals, and the verdict on each

The goals are the phase README's [§ Goal](../README.md#goal), the user's
[notes on the steps](../README.md#formalization-in-steps-the-users-notes-on-the-whole-phase), the research
plan's [Stage B](../../EinAf.md#stage-b--define-autoformalization-as-an-explicit-task) (B1 to B4), and the
milestone's minimal goal ([Q-M2.17](../../open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md)),
whose first attempt this phase holds.

| | the goal | verdict | on what |
|---|---|---|---|
| G1 | `einaf from-text`: a text becomes a program and the short answer that program prints, with the run's record | **met** | the command, the answer rule, seven fixed texts, the formatters, a slim record that replays: 29 of 29 programs byte for byte (4 re-run here) |
| G2 | the formalization made in three steps, the facts first (the user's notes) | **met as built; not shown to help** | steps, a prompt and a grammar each, each step judged at root. Against one call: no better on the word problems (none right against one), better on Zebra only in that a program exists |
| G3 | the Zebra encodings compared, the text's own words written, a target shape | **met**, as a default | `zebra3.1.ein` to `zebra3.4.ein`, six encodings on every axis; the shape is the user's to rule |
| G4 | the cheapest early measurement: does the model write loadable Ein with no grammar | **met: answered** | yes, shown an excerpt and examples, thinking off (13 of 14 parse, 8 load); no, told about Ein in a paragraph (1 of 15 parses); no, thinking on and greedy (0 of 36: every run loops). And what loads is not right |
| G5 | B1 to B4: the contract, source facts apart from generated knowledge, what the model is given, versioned prompts | **met, two gaps** | the contract page with its enforcers; categories computed, but not for a refused program ([§ 2](#2-the-acceptance-item-by-item), item 4); the model, the templates, the excerpt, the catalogue and the library hashed in `config.json`, but `kernel.commit` is `null` |
| G6 | the Zebra puzzle, one shot, to its answer: the minimal goal's first attempt | **not met** | *inconsistent*, root dead at step 2 |

G6 is the only goal that asks the model to be right, and the plan said beforehand that a miss there *is no failure
of the stage* ([S2.2.5](../s2.2.5_from_text.md#acceptance)): the phase was to report where it stopped, and did.
So the phase did its work; its result is that **one shot is a low baseline**, on Zebra and off it.

**Stage by stage**, each against its own acceptance ([the records](../README.md#stages)):

| stage | verdict | what is short |
|---|---|---|
| S2.2.0a three Zebras | met | nothing; the target shape is a default |
| S2.2.1 the contract | met | *the judge finds no event in any example* holds of the worked example only (item 3 below) |
| S2.2.2 the runtime and the probe | met, 10 of 10 | nothing |
| S2.2.3 the grammars | met, 6 of 6 | Q-M2.6 is a default, not a decision; amended once on the user's word |
| S2.2.4 the steps | met as measurements | step 3's clause **not met on 4 of 14** programs (the writer of H is not the one stated); Q9 a default |
| S2.2.5 from a text to an answer | 8 of 10 | the Zebra clause not met; the record's layout a stand-in for [S2.5.1](../../p2.5_harness/s2.5.1_experiment_record.md)'s |

## 2. The acceptance, item by item

The twelve items of [the phase's acceptance](../README.md#acceptance), in its order.

| # | the item | verdict | evidence, and what is short |
|---:|---|---|---|
| 1 | Zebra: `einaf from-text` yields a program answered *one*, naming the Norwegian and the Japanese | **not met** | *inconsistent*; *The statements contradict each other.* One shot, the pinned model, the theory selected from the library, with the record: all as asked, but the answer |
| 2 | the sixteen word problems, one shot, the library alone: parsed, loaded, outcome, the answer beside the README's; numbers from a record | **met** | [the tables](../from_text/tables.md): 13 texts for the 16 problems; 11 load; *one* 8, *several* 3, *invalid* 2; read right 3. The phase says itself this is no result: the prompts were developed on these problems |
| 3 | a right program is scored right; the judge finds no event in any reference program | **partially** | scored right: yes, the reference word problem (*3*) and the Zebra steps by hand through the one shot, both gate tests. No event: only in the worked example; Zebra by hand carries 11 and the seating 5 under the open [Q-M2.22](../open_questions/answer/Q-M2.22-is-a-membership-no-sentence-states-a-faithfulness-event.md) |
| 4 | every form of every recorded program is in exactly one category row | **partially** | held by test for the reference programs. Of the 29 one-shot programs the sidecar has it for 22; **the 7 the kernel refuses carry an empty table** (the categories are computed after a load) |
| 5 | the steps by hand: Zebra to `zebra3.1.ein` and two or more other texts, each step judged by a test | **met** | six texts, eighteen programs, `einaf/tests/contract.rs` |
| 6 | the user's first experiment has a record; step 1 alone against the hand's | **met** | shown `zebra3.1.ein`, the model copies it whole and prints the right sentence (a floor); step 1 alone: 0 of the hand's 44 Zebra facts |
| 7 | every one-shot record holds each step's program and judgement | **met** | a replay writes `steps/` and `kernel.jsonl` again from the slim calls (seen here) |
| 8 | a record replays to the same program | **met** | 29 of 29; four re-run here, byte for byte |
| 9 | a program with no fixpoint comes back *not decided* within the bound | **met as worded** | the cap on firings holds. The bound leaks beside it: a dead root took 72 s under a 60 s clock, and under `-T 5` ([E-126](../evaluation/bounds/E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md)) |
| 10 | no prompt holds an instance's solution; the quoted programs listed and handed to S2.3.4 | **met in the record** | `config.json`'s `quoted` lists them; the hand-over waits for [S2.3.4](../../p2.3_benchmark/s2.3.4_splits.md), which has not run |
| 11 | the probe's record exists; Q-M2.6 and Q9 decided in the ledger with their measurements | **partially** | the record exists; both questions carry a **default** with its measurement and are still open for the user |
| 12 | the evaluation of Ein holds what each stage found, and is shown | **met** | E-104 to E-127, 24 entries, two of them bugs with no owner |

## 3. Why the Zebra one shot missed

**The pipeline reaches the answer when the reading is right.** The test
`the_zebra_puzzles_steps_by_hand_go_through_the_one_shot_to_its_answer` feeds the hand's three steps through
`einaf::from_text` and gets the canonical sentence, scored right. So the grammars, the assembly, the kernel call,
the answer rule and the formatters are not where the run was lost.

**The model's program** ([`program.ein`](../from_text/runs/library/zebra/program.ein), 104 forms) has these defects,
each independent of the others. The first two are the report's (L-M2.23); the rest are read off the program here.

| | the defect | step | what it does | what tells the model |
|---:|---|:---:|---|---|
| 1 | a description made a thing: `RedHouse`, `GreenHouse`, `IvoryHouse`, `YellowHouse`, `BlueHouse`, `MiddleHouse` are houses, eleven in all | 1 | with defect 4, the Norwegian is in two houses: `slot-no-room`, root dead | the kernel's core, one fault at a time |
| 2 | four kinds with no member under the partition (`Smoker`, `Drinker`, `Owner`, `Color`) | 2 | `slot-no-fill` at every place: root dead | the core, naming one house's sentence of eleven, by accident *Coffee is drunk in the green house.* |
| 3 | drinks, pets and cigarettes are one kind, `Thing`: fifteen members for five places | 1, 2 | with 1 and 2 removed and the search allowed, **R is still empty** (below) | only *Open: every alive node owes* |
| 4 | the three *next to* sentences written as `lives-in` and `smokes`, two of them over names that are nothing (`ChesterfieldsHouse`, `FoxOwner`, `HorseKeeperHouse`); no sentence is stated as a neighbour | 1 | three clues lost; a program alive without them could not be *one* | nothing |
| 5 | `(solve :no-search)` beside the obligations | 3 | H is empty: nothing is searched | EinAf's check, not the kernel |
| 6 | the answer query asks one person both questions (`?who` twice) | 1 | no row on the right grid either | nothing |
| 7 | the read-back selected over the empty kinds, and no `compose` | 3 | the query has no row | EinAf's check (`selection-inactive`) |

**Defect 3, measured here.** The report's last variant, the program *less both* faults, is alive and owes 28
instances, and the report stops there because `(solve :no-search)` proposes nothing. With that line dropped too:

<!-- output -->
```text
$ grep -v '^(solve :no-search)' plans/m2_nl_to_ir/p2.2_formalizer/report/out/ein/model-zebra-less-both.ein > /tmp/less-both-search.ein
$ ein solve /tmp/less-both-search.ein --single                                   # 0.13 s, exit 0
  verdict           Open — R is empty: every alive node owes
  universe          190 candidates
  work              171 saturations · 27 487 facts beyond root
  over              all of R — R is empty, root is alive and owes 28 instances: no state to answer from
```

That it is the one kind `Thing` that leaves every node owing is my reading of the program (fifteen values of one
kind cannot each have one of five places); the run shows only that three repairs are not enough.

**What transferred from the examples, and what did not.** Every prompt of the run shows the same three worked
examples, and the first, [`shops`](../../../../einaf/prompts/examples/shops/step1.ein), is the Zebra puzzle's theory
in miniature, written the hand's way: *the blue stall* is `Blue`, a `Color`; the passive stays
(`are-sold-at Apples Green`); *next to* is a relation of its own; the query has two variables; step 3 ends on the
comment *The hypothesis mechanism: the obligations of the theory step 2 selected.* and no `solve` form. The model's
step 2 and step 3 repeat that example's comments nearly word for word and its selections with the names changed,
which is why the theory is *selected and spelled right*. Then it appends `(solve :no-search)`, the last line of the
other two examples. Each reading decision the example also shows, and that has to be made again on new sentences,
went wrong: the description, the voice, *next to*, the kinds, the query. So the model follows a shape and does not
carry over a reading.

**So step 1 is where it is lost**, the step the notes call simple: 4 of its 53 forms are the hand's
([the report § 2](README.md#2-where-the-minimal-goal-stands)), and alone in the probe 0 of the hand's 44 facts.
The plan's risk was written the other way round (*a model of 4.2B parameters may not select a theory in one
shot*); the selection is the part it got. The risk that did come true is the fourth of
[the list](../README.md#risks): *a wrong coreference in step 1 is costly to undo*.

**Why nothing caught it on the way**, each a choice of the plan and none a fault of a stage:

- **Step 1's judgement at root is silent.** A program of facts with no theory loads, root is alive, H is the blind
  enumerator's 7 840 candidates: *not decided*, whatever the facts say. The first signal is step 2's dead root.
- **The one shot never looks back.** The model is not shown a step's judgement before the next step (default 39:
  that is P2.6's arm), and no later step rewrites a step-1 fact (default 37: that is the loop's).
- **The model ran without its reasoning.** Thinking on, greedy, every run of the probe loops to its budget (36 of
  36), so every result is thinking off, where the model *thinks in comment lines*
  ([L-M2.16](../findings/model/L-M2.16-with-thinking-off-the-model-thinks-in-comment-lines.md)).
- **Most of the defects are silent in Ein.** Of the seven, the kernel names two, and those one at a time; two more
  are seen only by EinAf's check; three have nothing that points at them ([E-123](../evaluation/diagnostics/E-123-no-value-says-whether-a-selecting-fact-was-taken-up.md),
  [L-M2.18](../findings/model/L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md)).

## 4. Why the word problems read as they do

[The tables](../from_text/tables.md), the library alone, thirteen texts:

- **No right answer is computed.** `gap` and `kinds` print *The statements do not determine the answer.* because
  the program declares the calculator's `sum-of` as its own and does not list it in the import, so nothing derives
  it; `boxes13` prints it because two of its steps ran to the budget and wrote nothing. The gold happens to be
  *undetermined* on all three. **The same slip prints the same sentence on `gap_fact` and `kinds_full`, where the
  gold is 9 and 8.** The sentence follows the slip, not the text.
- **The dominant failure is a missing notion, not a misspelling.** A change over time has no word in the text and
  no theory in the library, so the model's giving rule reads what it concludes and one state holds every pass:
  `bob1` prints 1, 3 and 5; `bystander` gives Carol five counts; `before` three
  ([E-124](../evaluation/answer/E-124-one-state-may-answer-a-question-of-one-value-with-a-row-a-pass.md),
  [E-110](../evaluation/stdlib/E-110-the-library-has-no-reading-of-a-literal-vocabulary.md)).
- **The contradictory text is not seen as one.** `contra` prints *Bob has 5 apples.*; `zebra-contradiction` is
  scored right on a contradiction of the model's own making
  ([L-M2.22](../findings/model/L-M2.22-a-fixed-text-scored-right-can-rest-on-a-contradiction-of-the-model-s-own.md)).
  The three classes Level B asks to be told apart are, on this evidence, hit only by accident.
- **Given the family's own modules, the model imports none**, loads less (8 against 11) and prints one right
  number, `bystander`'s *Carol has 9 apples.*, which is a sentence of the text read back.
- **Two refusals are Ein's**, not the model's: `unused` declares `has` twice and is refused against Q-M1f.447
  ([E-104](../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md)), in both
  conditions.

## 5. Where this reading differs from the report's

The report is accurate on what it states; every number checked here agrees. These are matters of weight.

1. **The Zebra miss is not two faults and a variable.** [§ 4.1](README.md#41-the-zebra-puzzle-from-its-text-to-the-kernels-answer)
   says *the repair the loop has to find is two … and a third before the answer prints*. The table of
   [§ 3](#3-why-the-zebra-one-shot-missed) has seven, and the run above shows that three repairs leave R empty.
   The stage records first read the death as *ten houses*; the report's re-run corrected that to two faults; this
   re-run finds a third behind them. **The nearest miss the risk list asks for has not been measured yet**: the
   least edit of the model's program that answers.
2. **Read right, 3 of 13 overstates.** Counted by programs that answer for the text's reason, the library
   condition has **none**, and the Zebra row's one *right* is the same accident. The tables say so line by line;
   the summary rows do not.
3. **The Zebra run had a worked miniature of its theory in the prompt.** S2.2.4's default 7 calls the three
   examples *families no text of the phase has*, and in the same sentence `shops` *an attribute puzzle over the
   Zebra puzzle's theory*; `buying` and `ages` are word problems over the calculator. No solution leaks, so item 10
   holds; but the one shot is not a run on an unseen kind of text, and it missed all the same.
4. **The other side of the minimal goal rests on the decoding.** The model alone, thinking on, **reached the
   right answer in its thinking** by about its 5 900th token and then looped
   ([the probe § 4](../model_runtime/README.md#4-the-model-alone-on-the-zebra-text)). *The model alone does not
   solve it* is so far *the model alone, decoded greedily, prints no answer*. Under the model's own sampler, which
   is [S2.5.2](../../p2.5_harness/s2.5.2_baselines.md)'s default for the arms, it may solve the puzzle in text, and
   the minimal goal's premise would then need a second look
   ([Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md)).
   Not run here.
5. **Two acceptance items are short in ways no record names**: item 4 (the seven refused programs have no
   categories) and item 11 (*decided* is a default taken, on both questions).

Beside them, smaller: the record's `kernel.commit` is `null`, which Level B's fourth criterion will want; and the
phase's numbers were read by a stand-in for [S2.5.3](../../p2.5_harness/s2.5.3_metrics.md)'s reader, where the plan
had that stage's first part run before them ([§ Order](../README.md#order)).

## 6. What it comes to

Inference, for the user's review; none of it is a ruling. What to do about each point is
[§ 9](#9-proposals-for-the-other-issues).

- **The phase can be judged done as a phase**: its deliverables exist, are tested in the gate (1 427 of Ein's
  tests and 86 of EinAf's on `d124b7b2`; no code has changed since), and replay. Its one unmet item was planned as
  reportable, and the two partial items that are real gaps (4, 11) are small.
- **As a baseline, B2 is near zero** on these texts: no Zebra, no word problem answered for the text's reason.
  That is a finding the plan allowed for, and it is the number the loop will be measured against.
- **The loop inherits more than a repair.** Of the seven Zebra defects the kernel speaks of two, one at a time,
  its core naming a sentence by the accident of which house it took
  ([E-125](../evaluation/diagnostics/E-125-a-dead-root-s-core-names-the-sentence-the-model-misread.md)); the other
  five it cannot point at. Feedback made of the kernel's word alone will not carry the model from this program to
  the reference; the contract's check and the faithfulness judge have to carry most of it, and the principle *the
  theory before the facts* meets a case where the facts are what is wrong.
- **The steps are not yet shown to earn their cost** (B1: *evaluate whether this intermediate representation
  helps rather than assuming that it does*): three calls of 4 600 to 13 900 prompt tokens against one, for no
  better answers on the word problems.
- **Every number is conditional on defaults nobody reviewed**: 147 of the stages' and 40 of the re-plan's. The
  ones the results turn on are greedy decoding with thinking off, the target shape `zebra3.1`, and the three
  examples.
- **What Ein owes this application** is already in the evaluation: two bugs with no owner
  ([E-104](../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md), which cost a
  run, and [E-105](../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md)), a
  clock that does not stop a dead root (E-126, with
  [Q-M2.34](../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md)),
  and the silences of § 3.

## 7. What was checked here, and what was not

**Re-run**, on the release builds of this checkout (`ein.rs/target/release/ein`, `einaf/target/release/einaf`):

- `einaf from-text … --replay` on four committed records (`zebra-contradiction`, `gap` and `bob1` of the library
  condition, `bystander` with the modules given): each prints the report's answer and verdict, and each replayed
  `program.ein` is the committed one byte for byte.
- `ein solve --single` on the report's *less both* program with `(solve :no-search)` dropped: the output of
  [§ 3](#3-why-the-zebra-one-shot-missed).
- `git diff --stat d124b7b2 HEAD -- einaf ein.rs stdlib examples tests corpus` is empty: the gate's commit is
  still the code's.

**Read, not re-run**: the 29 sidecars (22 with categories, 7 empty, the 7 *invalid* programs); the model's Zebra
program and the three prompt examples; a replayed run's `config.json`.

**Not done**: the gate (taken from the record of `d124b7b2`); any run with the model; the 236 s Zebra replay; the
sidecars of S2.2.4's 64 runs; the cause of the empty R of § 3 beyond the run itself.

**For §§ 8 and 9**, added later the same day. Re-run: `ein solve --single`, `einaf steps check` and `einaf feedback`
on `examples/zebra3.1.ein` with its query written as two; `einaf grammar render` of steps 1, 2 and 3, read for how
each holds the answer query, an import's list and the mechanism. Read: the probe's record of step 1 alone on the
Zebra text; the step-1 template, the excerpt, the answer lines and the three examples; step 1's output of
`steps/runs/steps/` and `steps/runs/steps-free/`. Measured by throwaway scripts, kept nowhere: proposal 2's rule
over the recorded step 1 of sixteen texts and the steps by hand, and proposal 9's share of the generation time
from [the tables](../from_text/tables.md). **No proposal was implemented and none was run with the model.**

**For § 10.** Read: the model's thinking in the probe's record `model-alone.on.zebra` (its opening, its grid, how
it reads *next to*, its backtracking). Re-run: `ein solve` on `examples/zebra3.1.ein` with twenty cells of its own
answer added as facts and `(solve :no-search)`, and the same with two cells exchanged, twice (the zebra and the
fox; green and ivory); the three files were scratch and are not kept. Nothing was run with the model.

**For § 11.** Read: S2.2.4's report and tables, the model's programs of `gap` (steps), `kinds_full` (one call),
the seating and the Zebra puzzle, the plan of P2.6's S2.6.4. Counted by throwaway scripts: the forms of each
step 2 that are step 1's, and the tokens generated in calls that ran to the budget, a condition. Re-run:
`einaf grammar render 3` on the model's and on the hand's Zebra program up to step 2 (which selecting facts step 3
is offered); `ein solve` on the model's seating program, as written and less the three memberships step 2
restated; `einaf outcome --no-search` and `ein show hyps` on four of the steps by hand cut into three strict
parts, and `ein solve --single` on the Zebra one whole. Scratch files, not kept; nothing was run with the model.

**For § 12.** Read: `einaf catalogue` and `einaf catalogue --generated` as a prompt shows them; the rules and
their comments in `stdlib/algebra.ein` and `stdlib/slots.ein`; the reference's pages on the relation algebra and
the slots; `einaf::catalogue::Given::text`; the selecting facts of the model's and the hand's Zebra and seating
programs, counted by `grep`. Nothing was run with the model, and § 12.3's draft is checked by nothing.

## 8. The one `?who` for two questions

**The user's words** (2026-11-26), on defect 6 of [§ 3](#3-why-the-zebra-one-shot-missed), kept as stated:

> On single variable `?who`.
> Qwen can write programs in programming languages AFIAK.
> So it knows same name means same variable can mean same object in the same context.
> So LLM either does not understand the Ein semantics,
> or our `close to text` instruction may be confusing.
> Or we should have separate query for each question - then same `?who` works perfectly. Right?

**Right. And it is what the model wrote by itself, before the contract told it to write one query.** In S2.2.2's
probe of step 1 alone (`step1.off.zebra` in [`probe.jsonl.gz`](../model_runtime/probe.jsonl.gz): no grammar, one
example, a text with one question) the model's answer to the Zebra text holds:

```lisp
(query :goal (drinks ?who water) :goal-text "{?who} drinks water")
(query :goal (owns ?who zebra)   :goal-text "{?who} owns the zebra")
```

A query a question, `?who` in each, and correct Ein. S2.2.4's prompt then asks for *one query*, and from that
stage on the model folds its two queries into one `and` and keeps its variable, **with the grammar and without it**
(`steps/runs/steps/zebra/` and `steps/runs/steps-free/zebra/` write the same query). The three explanations, each
against the records:

| the explanation | what the records say |
|---|---|
| the model does not understand Ein's semantics | **not the cause.** Its own rules join on a repeated variable on purpose: `bob1`'s `(and (gave ?giver ?k ?receiver) (has ?giver ?a ?thing) (diff-of ?a ?k ?d z))`. But the reference it is given never states the rule: the excerpt's one line on variables is *A variable used once stands for anything*, and its one joined goal, `(and (is-parent-of ?p Cy) (has-no-child ?q))`, shares none |
| *close to the text* is confusing | **in part.** Both questions say *Who*, so the text's own word for each unknown is `?who`; the example's `?who_bread` and `?who_red` are a device, not the text's words. The same pull makes *the red house* `RedHouse` ([proposal 10](#9-proposals-for-the-other-issues)). But the instruction that decides here is another one: *one query* |
| a query for each question | **right.** Three places ask for one: the step-1 template (*The answer query: one query whose `:goal` asks what the text's question asks*), the excerpt (*The answer query is the one query with a `:goal-text`*), and step 1's grammar (`# the answer query, once`); and the `shops` example shows two questions in one query. Two questions, one form allowed: the model joined them |

**In Ein it works as the user says.** [`examples/zebra3.1.ein`](../../../../examples/zebra3.1.ein) with its one
query replaced by two, the same variable in each (a variable is a form's own):

<!-- output -->
```text
$ ein solve zebra3.1-two-queries.ein --single
  query 1 of 2      (drinks ?who Water)
    class 1 of 1    1 state (1)
                      ?who = Norwegian
                        “Norwegian drinks water.”
  query 2 of 2      (owns ?who Zebra)
    class 1 of 1    1 state (1)
                      ?who = Japanese
                        “Japanese owns the zebra.”
```

`einaf feedback` already returns both queries with their lines. What stands in the way is EinAf's own rule:
`einaf steps check` on that file reports *`several-answer-queries`: 2 queries carry a `:goal-text`; the first is the
answer query*, and `einaf::answer` would print the first line alone.

**The Zebra text is not the only one it costs.** `gap_fact` asks three questions (*How many apples does Bob have?
How many does Alice have? How many apples do they have together?*), and the model joins them:
`(and (has Bob ?b Apple) (has Alice ?a Apple) (sum-of ?b ?a ?t))`. The third conjunct reads a relation nothing
derives (the `sum-of` slip), so the whole goal has no row and the first two questions go unanswered with it. Its
variables are named apart there, after the people; on the Zebra text they are named after the question's word.

**The cause is [the phase's default 4](../README.md#defaults-taken-at-the-re-plan)**, *the answer query is the one
query that carries a `:goal-text`*, taken so that the benchmark and the metrics read one answer. It is a default,
not a ruling.

**Proposal 1: an answer query for each question of the text.** A question is a sentence, and a sentence is a form
of its own, as a statement already is: this is the notes' *literal translation*, applied to the questions. EinAf
only; Ein is untouched.

- **The contract**: every query that carries a `:goal-text` is an answer query, one for each sentence of the text
  that ends in `?`, in the text's order ([`04-contract.md`](../../../../docs/einaf/04-contract.md), default 4).
- **The prompts**: the excerpt's § 5 and the step-1 template say so; the `shops` example's query becomes two; and
  the excerpt gains the one sentence it lacks whatever is decided here: *a variable written twice in one form is
  the same object in both places; two forms share no variable.*
- **Step 1's grammar**: it is rendered on the text, so it can hold the count: as many answer queries as the text
  has questions, where today it holds one.
- **The answer** (`einaf::answer`): each answer query's line in order, and the fixed text chosen a query, so that
  one question can be determined where another is not. A joint query cannot say that: its classes are over the
  pair, and one conjunct with no row leaves the whole goal with none.
- **The check**: `several-answer-queries` goes; *fewer answer queries than questions* takes its place.
- **The readers**: a gold claim a question, which the stand-in already reads (`Norwegian water|Japanese zebra`);
  [P2.3](../../p2.3_benchmark/README.md)'s instance and [S2.5.3](../../p2.5_harness/s2.5.3_metrics.md) follow.
- **The steps by hand**: `einaf/contract/zebra/` takes the two queries; `zebra3.1.ein` may keep its joint query,
  which is Ein as good as any, the test comparing the two files less the query.

**Held by an invariance**, not by the old answer: the Zebra steps by hand with the joint query and with a query a
question print the same sentences under every question of `ein solve`; a test of EinAf's gate step.

## 9. Proposals for the other issues

Added on the user's word, *"propose solutions for this and other issues and add to the same file"*. **Proposals,
none run with the model**: what *would have* happened is read off the phase's records, and where a number is
measured here the text says so. Each names where it lands and whose word it needs; a default changed is the
user's to say.

**One caution over all of them.** Every proposal is drawn from the Zebra text and the thirteen word texts, so
after any of them those texts are a development set by construction (B4: *a prompt change is an experimental
change*). What a change is worth is shown only on texts no prompt was tuned against, which is
[S2.3.4](../../p2.3_benchmark/s2.3.4_splits.md)'s split.

### 9.1 At a glance

| # | the issue | the proposal | lands in | needs |
|---:|---|---|---|---|
| 1 | one `?who` for two questions (defect 6) | an answer query for each question ([§ 8](#8-the-one-who-for-two-questions)) | the contract, the prompts, step 1's grammar, `einaf::answer` | the user: default 4 |
| 2 | the voice folded, *next to* read as *in* (5 Zebra statements) | a statement's relation is made of words of its own sentence: first a finding of the check, then step 1's grammar | `einaf::check`, `einaf::grammar` | nothing: the contract says it already |
| 3 | *the red house* made a house (defect 1) | the rule said in the template, with its counter-example | `steps/step1` | nothing |
| 4 | empty kinds, kinds of unequal size, names of no kind, read-backs over nothing (defects 1, 2, 3, 4, 7) | each catalogue entry states what it needs of its arguments, and the check evaluates it at root | `einaf::catalogue`, `einaf::check` | nothing |
| 5 | `(solve :no-search)` beside obligations; no mechanism at all (4 of 14 programs) | step 3's grammar offers only the mechanism step 2's program can have | `einaf::grammar` | nothing |
| 6 | a calculator's relation not listed, declared as the program's own (4 of 13 texts) | an import lists an entry's names together; no declaration of a name a module has | `einaf::grammar` | nothing |
| 7 | a rule that reads what it concludes: 1, 3 and 5 apples (3 texts; `contra`) | an example with a change over time; a finding for the rule; then a reading in the library | the prompts, `einaf::check`; `stdlib/` | the user: Q-M2.23, Q-M2.33 |
| 8 | step 3's read-back: 0 of the hand's 18 forms | one selecting fact a verb | `stdlib/`, the catalogue | the user: Q-M2.23 |
| 9 | 8 calls of 88 loop to the budget: 82 % of all generation time | stop a generation that repeats; a bound on comment lines; a repetition sampler ahead of greedy | `einaf::model`, `einaf::grammar` | the user: Q-M2.27, Q-M2.31 |
| 10 | *close to the text* against the target shape's abbreviations | say the abbreviation as a rule now; the literal shape in the library later | the contract; `stdlib/` | the user: the target shape |
| 11 | a step's judgement reaches nobody; the one shot never looks back | a step is shown its own findings once before the next | [P2.4](../../p2.4_loop/README.md)'s first rung | the user: *the theory before the facts* |
| 12 | *the model alone does not solve it* rests on greedy decoding | the model alone on Zebra under its own sampler, a few seeds, before the loop | [S2.5.2](../../p2.5_harness/s2.5.2_baselines.md)'s B1, early | the user |
| 13 | a dead root costs 72 s past a 60 s clock | EinAf's call sets the lever; the kernel's clock covers a dead root | `einaf::kernel`; `ein.rs` | the user: Q-M2.34 |
| 14 | two bugs of Ein with no owner | fix E-104 and E-105 before the loop spends turns on them | `ein.rs`, `stdlib/` | the user (Q-M2.16 allows it) |
| 15 | right for a reason not the text's (3 of 13; `zebra-contradiction`) | two counts: right, and right with nothing of the check on the answer's path | the reader, S2.5.3 | nothing |
| 16 | the nearest miss is not measured | the seven Zebra defects mended one at a time by hand, the kernel's word after each | a record, no model | nothing |
| 17 | the small gaps of [§ 2](#2-the-acceptance-item-by-item) | categories from the parse where a load is refused; Ein's commit in the record; Q-M2.6 and Q9 ruled | `einaf::contract`, S2.5.1; the ledger | the user, for the two questions |

### 9.2 What a prompt or a grammar can prevent

The phase's own measurement is the argument for the grammar over the prompt: the `shops` example shows every one
of these decisions made right, and the model made them wrong; a step's grammar kept out 11 misspelt selections and
7 `:source` that are no sentence and changed no answer. Its cost is known too: a refusal led 7 calls into a loop
([L-M2.19](../findings/model/L-M2.19-a-grammar-s-refusal-sends-greedy-decoding-into-a-loop-it-does-not-enter-free.md)),
which proposal 9 is for, so 9a goes in before any grammar is narrowed further.

**2. A statement's relation is made of words of its own sentence, in their order.** The contract's rule (*the
relation is the sentence's verb in its own voice*) has no enforcer. Measured here, by a throwaway script over the
recorded step 1 of all sixteen texts: of the model's 57 statements the rule refuses 8, and **the five on the Zebra
text are exactly the misread ones**:

<!-- output -->
```text
drinks    ⟵  Coffee is drunk in the green house.
smokes    ⟵  Kools are smoked in the yellow house.
drinks    ⟵  Milk is drunk in the middle house.
smokes    ⟵  Kools are smoked in the house next to the house where the horse is kept.
lives-in  ⟵  The Norwegian lives next to the blue house.
```

The other three are two of an inflection (`sits` for *does not sit*, the seating) and one invented word (`count`,
`pencils`). On the steps by hand and the three prompt examples it passes every statement but the same inflection
(and `youngest`'s three memberships, written as a relation `person` with a sentence, which are not statements).
So: a finding of the check first, with a verb's forms allowed, measured on the steps by hand; then step 1's grammar,
an alternative a sentence, which a context-free grammar can state because the `:source` is already held to the
text's sentences. The same rule on an answer query (its relations are the statements', or words of its question)
refuses `sum-of` in step 1, where `gap`'s slip begins. It does not catch a name made up for a description
(`ChesterfieldsHouse`): proposal 4 does.

**3. A description is not a new thing.** One paragraph of the step-1 template, with the wrong form beside the
right one: *the red house* is one of the five houses, described by its colour: write `Red`, a `Color`, never
`RedHouse`, a sixth house. The template has the right form only, inside a parenthesis.

**5. Step 3's grammar offers only the mechanism the program can have.** It is rendered on step 2's program, and
today it offers `(solve :no-search)` whatever step 2 selected. The catalogue knows which writer of H each entry
leaves; where a selected entry's obligations write H, the `:no-search` alternative is not rendered, and where
nothing writes H a mechanism form is required. That removes `mechanism-not-the-one-stated` and
`mechanism-not-stated` by construction, as step 2's grammar removed the misspelt selections.

**6. An import lists the catalogue's names an entry at a time.** Step 2's grammar lets the model compose a
`:symbols` list name by name, and `gap` wrote `(add product-of)`: the request of one entry, `(add sum-of)`, and
the answer of another, `(mul product-of)`. The grammar offers an entry's names together, a request with its
answer, and refuses a `relation` declaration of a name an offered module declares. Until then `name-not-listed` is
the loop's cheapest repair: the finding names the line and the word.

**7, the first part. An example with a change over time.** None of the three examples has one (`buying` multiplies,
`ages` subtracts, each once), and seven of the thirteen word texts hold a giving. The steps by hand write
the moments (`Start`, `After`); an example of another text that does the same is the cheapest thing to try against
*Bob has 1 apples. Bob has 3 apples. Bob has 5 apples.*

### 9.3 What EinAf's check can see where the kernel is silent

**4. Each catalogue entry states what it needs, and the check evaluates it at root.** An entry already carries its
effect, held by `einaf/tests/catalogue.rs`; its preconditions are data of the same kind. For
`(slot-partition R isa sub Super Index)`: every kind under `Super` has a member, each as many as `Index` has, and
every name standing in a relation read into `R` is of one of those kinds. For `(top R isa Dom Ran)`: both kinds
have members. On the model's Zebra program one finding would then say what three of the seven defects are, in
counts read off the program: *`House` 11, `Person` 5, `Thing` 15, `Smoker`, `Drinker`, `Owner` and `Color` 0, where
the partition needs one of each kind a house; `ChesterfieldsHouse`, `FoxOwner` and `HorseKeeperHouse` are of no
kind.* Today the same program gives a dead root and a core that names one house of eleven. The kernel is not asked
to read a theory's name: the entries are EinAf's. Held by: no finding on any entry program or step by hand, a
finding on the model's steps 2 and 3.

**7, the second part.** A finding for a rule that concludes the relation it matches with nothing that stops it,
reported where the answer query's class holds several rows; it is a note, since a transitive closure is such a
rule. And a note where the answer's row is a statement of the text read back while the program has rules, which is
`contra`'s *Bob has 5 apples.* and, rightly, `bystander`'s 9. With them
[Q-M2.33](../open_questions/answer/Q-M2.33-is-a-class-with-several-rows-an-answer-of-several-values-or-the-undetermined-text.md)
has a third answer beside *its lines* and *the undetermined text*: several rows for a question of one value is
reported as a fault of the program, not printed as an answer.

**15. Right, and right for the program's reason.** The reader scores *The statements do not determine the answer.*
right on `gap` while the check holds `name-not-listed` on the very relation the query reads. Two counts: right, and
right with no finding of the check on a relation the answer query reads (for a contradiction: a core that holds two
statements of the text). On this phase's runs of the library condition the second count is 0 of 13 word texts and
0 of 3 Zebra texts.

### 9.4 What the library and the kernel owe

**8. One selecting fact a verb for the read-back.** `zebra3.1.ein` reads an answer back through `converse`,
`compose`, `top` and `meet`, 18 of its 32 selecting facts, and the catalogue had to say what no page says (*a value
is read back through its place*). The model's step 3 has none of the hand's 18 forms. A rule with parameters, or a
macro over the four, that says *a person drinks the drink of their house* in one fact a verb is a library change
that leaves Ein's syntax and semantics alone. It reopens
[Q-M2.23](../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md), whose
default is *no module*. **7, the third part**, is the same question for the word problems: the moments of a change
are a theory the library lacks ([E-110](../evaluation/stdlib/E-110-the-library-has-no-reading-of-a-literal-vocabulary.md)),
and the family's own modules, given, were imported by no run, so a module needs a catalogue entry to be taken up.

**10. The shape and the instruction disagree.** *In the text's own words* is the instruction; the target shape
writes *the green house* as `Green` and *the Old Gold smoker* as `Old_Gold`, which is an abbreviation and not the
text's words. The model's `GreenHouse` is the literal reading. Two ways: keep `zebra3.1` and state the abbreviation
as the rule it is (proposal 3, now); or make the literal reading the right one, a description a term of its own,
`zebra3.2`'s `(the Green House)`, with its six rules moved into the library, so that step 1 is translation and what
a description names is found by the theory. The second is closer to the notes (*facts as close to the original
text as possible*, the theory searched for); today it is six rules of the program's own, and
[E-106](../evaluation/answer/E-106-the-static-check-reads-a-relation-not-a-shape.md) flags its `same` form where
it would be most literal. Which axis weighs most was left to the user by S2.2.0a; this is evidence for that choice.

**13. The clock and the lever.** [Q-M2.34](../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md)'s
default is *no*. The case for *yes*: `:record-alternative-justifications false` is the one lever exempt from *no
lever changes R*, it moves only a dead node's core, and a core is an explanation; 72 s against 0.07 s a question on
the model's Zebra program. Beside it, an item for Ein: a clock that a dead root's bookkeeping does not outrun
([E-126](../evaluation/bounds/E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md)).

**14. E-104 and E-105 before the loop.** [E-104](../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md)
refuses what Q-M1f.447 allows and cost `unused` in both conditions; the loop would spend its turns repairing a
program that is right. [E-105](../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md)
lets two neighbours share a house, in the theory every attribute puzzle of the benchmark will select.

### 9.5 The runtime and the decoding

**9. Stop a generation that loops.** Measured from [the tables](../from_text/tables.md): the 8 calls that ran to
the budget of 12 288 tokens took 1 758 s of the 2 138 s the 88 calls generated, **82 %**, each a loop inside a form
or in comment lines. Three things, in order of cost: (a) a detector in `einaf::model` that ends a generation whose
tail repeats one cycle, with a stop reason of its own; **held by an invariance on the eight recorded calls**, with
no model: the forms kept are the forms the budget's cut keeps. (b)
[Q-M2.31](../open_questions/model/Q-M2.31-does-a-step-s-grammar-bound-the-comment-lines-the-model-writes.md): a bound
on consecutive comment lines in a step's grammar, where the model thinks and loops
([L-M2.16](../findings/model/L-M2.16-with-thinking-off-the-model-thinks-in-comment-lines.md)). (c) a repetition
sampler ahead of greedy in the chain, which stays one run a request; whether the binding exposes llama.cpp's is to
be looked up. It bears on [Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md)
and on [P2.2b](../../p2.2b_guided_thinking/README.md), whose thinking runs meet the same loop.

**12. The model alone, under its own sampler.** A handful of runs of about six thousand tokens each. If the model
then solves the Zebra puzzle in text, the minimal goal as worded has no other side, and that is better known
before the loop is built towards it than after.

### 9.6 The loop's first rung, and the nearest miss

**11. A step is shown its own findings once before the next step.** It is planned as an arm of P2.6 (default 39).
With proposals 2 and 4 the findings at steps 1 and 2 are specific and mechanical (*`lives-in` is not in the words
of this sentence*; *`House` has 11 members and `Person` 5*), which is what a 4B model can act on. It also asks
something of the principle *the theory before the facts*: on the Zebra run the facts are what is wrong, and a
theory repaired around them cannot answer. A clause for it is the user's: a statement whose relation is not its
sentence's words is repaired before any theory.

**16. The repair ladder, by hand, once.** The model's Zebra program with its seven defects mended one at a time,
the kernel's outcome and core and the check's findings recorded after each. No model, an afternoon's programs. It
is the nearest miss [the risk list](../README.md#risks) asks for, and it shows what the loop's feedback ladder
would have to say at each rung before [S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md) builds one.

### 9.7 The order

- **First, EinAf only, no ruling but default 4**: 1 (a query a question), 9a (the loop detector), 5 and 6 (two
  grammar renderings), 2 as a finding, 4 (the entries' preconditions), 3 (a paragraph). Each is small, each removes
  a defect by construction or names it in a sentence, and four of them are held by a test with no model.
- **Before P2.4 starts**: 16 (the ladder by hand), 12 (the other side under its own sampler), 13 and 14 (the lever,
  the two bugs), 15 (the second count), 17.
- **The user's rulings they wait on**: default 4 (proposal 1); Q-M2.23 and the target shape (7, 8, 10); Q-M2.33
  (7); Q-M2.27 and Q-M2.31 (9); Q-M2.34 (13); *the theory before the facts* (11).
- **Then one run of the plan's 29 again**, the same texts, as a development number and labelled so.

## 10. The model solves the puzzle and cannot state it

**The user's words** (2026-11-26), kept as stated:

> While it was shown that model solve zebra puzzle during thinking, but not able to formalize it into programs.
> What does it mean? Looks like Ein does not "help" model but does the opposite - "interferes" it's correct
> reasoning?

**In short.** For the filter as it was run, one shot, the answer is yes: on this puzzle the route through Ein is
worse than the model alone, because it switches off what the model does well and asks for what it does badly. For
Ein it is not shown either way: the kernel and the model's reasoning never met in this phase. And what the model
alone lacked was not a search but a verdict, which is the thing a kernel gives cheaply
([§ 10.3](#103-what-the-model-lacked-and-what-ein-gives-today)).

### 10.1 What the records show

The same model on the same seventeen lines, in two modes
([the probe § 4](../model_runtime/README.md#4-the-model-alone-on-the-zebra-text), `model-alone.on.zebra`; the one
shot's [`program.ein`](../from_text/runs/library/zebra/program.ein)):

| | reasoning in text, thinking on | writing the program, thinking off |
|---|---|---|
| the houses | *There are 5 houses (1, 2, 3, 4, 5)* | eleven: `House1` to `House5`, five coloured ones, `MiddleHouse` |
| the kinds | *Attributes to fill: Color, Nationality, Drink, Pet, Cigarette*, before it reads a clue | `Person`, `House`, and one kind `Thing` for every drink, pet and cigarette; four kinds with no member |
| *the blue house* | *Since H1 is Norwegian, H2 must be Blue*: a colour of one of the five | `BlueHouse`, a house of its own |
| *next to* | used as a neighbour, in both directions | `(lives-in Norwegian BlueHouse)` |
| the result | the right grid by about its 5 900th token; then *Wait, I need to double check*, a second branch, a loop to the budget: **no answer given** | 104 forms, root dead: *The statements contradict each other.* |

Three facts beside the table:

- **It recognised the puzzle.** Its thinking opens: *Here's a thinking process that leads to the solution of the
  Einstein's Riddle (Zebra Puzzle)*. The five kinds it names are in no sentence of the text: *pet*, *cigarette* and
  *nationality* are its own words. It knows this puzzle from its training.
- **The two modes never met.** No run of the phase has the model reason and then write a program: thinking on
  beside a program, every run of the probe looped (36 of 36). Every program was written with thinking off.
- **The model alone was run on this one text**, not on its two edits and not on a puzzle it cannot have seen.

### 10.2 What it means

1. **Solving and stating are two skills, and for this model on this puzzle stating is the harder one.** To solve,
   it uses a form it was trained on: a grid, prose, thousands of puzzles of this kind and this one by name, with
   what the text leaves unsaid (one of each kind a house, the row, *next to* both ways) applied without being said.
   To state it in Ein it has to write all of that down, in a language it knows from a reference of a few thousand
   tokens and three examples, with its reasoning off, in one pass. The plan asks the model for *one difficult thing* on the premise that it is the
   thing a model can be asked for. Here it is the other way round.
2. **It is not that the model does not know.** The left column is knowledge the right column needed. The
   formalizer's mode did not reach it: thinking off is the mode in which the model did not reason; the template
   says *in the text's own words* and *as far as they are seen*; and the design tells it not to solve. The same
   weights wrote *H2 must be Blue* and `BlueHouse`.
3. **"It solves the Zebra puzzle" is weaker than it sounds.** The working is real: it fills the grid clue by clue,
   takes a wrong turn and catches it (*CRITICAL ERROR FOUND. Let's backtrack.*). But it is working a puzzle whose
   name, kinds and shape it brought with it. Whether this model solves a puzzle of this size it has not seen is
   not measured. So the user's first observation (the model
   does not solve it alone) is overturned on this text for a reason that says little about its reasoning, and the
   Zebra text as written is a poor place to look for help: there is nothing on it to help with.
4. **Does Ein interfere?** Three things the pipeline did turn from right to wrong: the two queries the model wrote
   became one ([§ 8](#8-the-one-who-for-two-questions)); the kinds it knows became `Thing`; and its reasoning cannot
   run beside a program without looping. Each is the filter's arrangement, the prompts' and the decoding's; none is
   the kernel's. What can be said: **the filter, one shot, built so that the model does not use its reasoning,
   loses to that reasoning where the reasoning works.** Its answer can be no better than its translation, and the
   translation is the weak skill. That is a finding about one arm, B2, at zero feedback. The user's core idea has
   two halves (*on one hand ein works like a tool for LLM, on other hand it works like filter of thoughts*), and
   this phase built one shot of the second.
5. **On this puzzle Ein cannot better the answer; it can only add what the answer lacks**: that it is the only
   one, proved and not asserted; the two other classes, which a model that recalls will get wrong (the same
   seventeen lines less one clue do not determine the answer); an explanation tied to sentences; and a stop.
6. **Where Ein can better the answer is where the model's reasoning ends**: puzzles it has not seen, larger ones,
   undetermined and contradictory ones. The cost of stating a puzzle is nearly the same for five houses and for
   eight (the theory is the same, the facts a few more); the cost of solving it in text is not. Where the two
   cross for this model is unknown, and it is the number the milestone turns on.
7. **For the research question** (*can structured feedback from a symbolic reasoner improve an LLM's ability to
   construct correct formal theories*): the ladder F0 to F8 is what the kernel says. This phase points at a
   witness the ladder does not have, **the model's own solution**. Where the model can reason at all, its answer
   set against its program's is feedback with no gold in it.

### 10.3 What the model lacked, and what Ein gives today

The model alone found the grid and then lost it: it went to double-check, opened a second branch and never came
back. It had no way to know it was done. **A candidate grid checked against the clues is the cheapest thing a
kernel does**, and Ein does it now. [`examples/zebra3.1.ein`](../../../../examples/zebra3.1.ein), the clues by
hand, with twenty cells of the grid added as facts, `(same-house Norwegian First)` and the like, and
`(solve :no-search)`, so that nothing is searched and root alone is judged:

<!-- output -->
```text
$ ein solve grid-right.ein                    # 0.04 s, one saturation
  states (k_s)      1
  universe          no candidate: the program states :no-search, so R is {∅} or empty, decided by root alone
                        “The Norwegian drinks water. The Japanese owns the zebra.”

$ ein solve grid-wrong.ein                    # the zebra and the fox in each other's house: 0.05 s
  empty             every node is dead — root's own fixpoint holds a contradiction
  unsat core        (lives-next-to Chesterfields Fox)
  unsat core        (same-house Chesterfields Second)
  unsat core        (same-house Fox Fifth)
  …                 five more: three houses, the row between two of them, the kind of a house
  source            (lives-next-to Chesterfields Fox) :source "The man who smokes Chesterfields lives in the house next to the man with the fox."
  refuted by        std.slots.slot-adjacent-fwd, std.slots.slot-occupied
```

With green and ivory exchanged the core is `(is-drunk-in Coffee Green)`, *Coffee is drunk in the green house.*, and
the two cells that break it. **The clue and the two cells, by name**: that is a reply a model reasoning over a
grid can use, in either direction. If the grid is wrong, it mends its reasoning. If the grid is right and the clue
was written wrong, it mends the program, clause by clause, where a dead root's core today names one fault of seven.
The kernel cannot tell which of the two it is; the record keeps which side moved.

The second question is the certificate: the same clues without the grid, searched. *One*, and the grid: two routes
agree. *Several*: Ein shows a second grid, and the question to the model is which sentence rules it out, which is
how a lost clue (defect 4) would be found.

### 10.4 Further steps

Proposals, continuing [§ 9](#9-proposals-for-the-other-issues)'s numbers; none is run, and each waits for the
user's word.

| # | the step | what it answers | cost, and where it lands |
|---:|---|---|---|
| 18 | **The model alone, measured properly**: thinking on, its own sampler, several seeds, on the Zebra text, its two edits, the same puzzle with every noun renamed and the sentences reordered, and one generated puzzle of the same size | does it reason or recall; on which texts it fails alone, which is where help can be measured at all | about six thousand tokens a run; extends proposal 12, [S2.5.2](../../p2.5_harness/s2.5.2_baselines.md)'s B1 |
| 19 | **Ein as the checker of the model's own answer**: the model reasons, fills a table under a grammar (a row a place, a column a kind), the clues are written once, and the kernel answers the two questions of § 10.3 | whether Ein helps the reasoning it now replaces | a sketch, not a design: an arm between B1 and B6, or the tool's first form ([S2.4.6](../../p2.4_loop/s2.4.6_ein_as_a_tool.md)) |
| 20 | **Think, then state**: step 1 written after the model's own analysis, beside [P2.2b](../../p2.2b_guided_thinking/README.md)'s guided decomposition | whether the left column of § 10.1 reaches the right one | a condition of S2.2b.2; needs a generation that stops when it loops (proposal 9a) |
| 21 | **The model's answer as a level of the ladder**: *your program prints X; you concluded Y*, and the clause-by-clause check | whether a second witness repairs what the kernel's word cannot | [S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md), with its guard below |
| 22 | **The tool probed early**: a few episodes on the texts of 18, before the filter's loop is built | does the model call Ein, what does it write, does the reply change its answer | the runtime has the turns and the function; S2.4.6 is planned last |
| 23 | **Where the model stops solving**: the model alone on generated puzzles of three to eight places | the region where Ein can better an answer, or that there is none for this model | a small form of [P2.7](../../p2.7_failure_scaling_generalization/README.md)'s scaling, early |
| 24 | **The minimal goal's instance**, the user's: read on a puzzle the model does not solve alone, chosen by 18 | keeps the goal's sense where its premise failed | [Q-M2.17](../../open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md) |

**19, said once more plainly.** It keeps the kernel's part (the verdict, the only-one, the classes) and gives back
to the model the part it does well. It also makes most of [§ 3](#3-why-the-zebra-one-shot-missed)'s defects have
no place to occur: the kinds and the places are the table's, which the model sets up right when it reasons; the
check at root needs no hypothesis mechanism and no read-back, since the answer is a cell of the table. What stays
is the reading of each clue, defect 4, and proposal 2 is for that. It fits problems whose answer is an assignment;
for a word problem the model's number is only a second witness (21).

**The guard on 19 and 21.** The milestone's most dangerous failure is a loop *succeeding by its own measure while
failing the task*, and a model that bends its program until it prints what the model believes is that failure
with a new cause. So: the grid tests a program and is never part of the one that answers; the answer is the
program's, run without it; a fact with no sentence behind it is still a generated assumption; and the arm is
measured on texts whose gold is *undetermined* or *contradictory*, where a model that recalls believes the wrong
thing and the kernel's word has to win.

**The order.** 18 first: it is the cheapest, and 19, 23 and 24 are read off it. Then 22 and 19 on the texts where
the model fails alone, beside the filter's loop and not after it. 20 belongs to P2.2b as it is planned.

**What this does not say.** That Ein is of no use to the model: no arm in which the kernel answers the model has
run. That the filter is finished: its loop has not run, and one shot was always its floor. That the model reasons:
on a puzzle it knows by name, that is not shown either.

## 11. The three steps against the one call, and how the steps are cut

**The user's words** (2026-11-26), kept as stated:

> analyze 3-steps proposal results, compared to one shot; Does it make sense of partial ontology and partial
> theory, or better strictly split - 1 - facts and queries only, 2 - ontology only, 3 - theory only; Was it
> investigated or not? If not - add proposal;

**In short.** Against the one call the three steps gave no better answers; what they gave is a place to hang
grammars, and that is where their measured gain comes from. With no grammar the steps load *fewer* programs than
the one call, and the reasons are slips between steps. The two *partial* parts are where those slips are: an
ontology written twice and never revised, and a theory whose import list is written a step before its uses. **A
strict split was not investigated and is planned nowhere**; it is [proposal 25](#115-proposal-25-the-cut-measured).
The steps by hand, cut strictly here, load at every cut but one, and show what a strict split would lose: the
kernel's one informative judgement in the middle.

### 11.1 The three steps against the one call

S2.2.4's four conditions on the sixteen texts ([its report §§ 8 and 9](../steps/README.md#8-the-one-call-against-the-steps-and-b2s-mode),
[the tables](../steps/tables.md)):

| | the steps, each under its grammar | the one call, under the whole grammar | the steps, no grammar | the one call, no grammar |
|---|---|---|---|---|
| a complete program; it loads | 16; **14** | 15; **14** | 16; **11** | 15; **14** |
| `--single` | one 9, several 3, inconsistent 2, invalid 2 | one 11, inconsistent 3, invalid 1 | one 8, several 1, inconsistent 2, invalid 5 | as under its grammar |
| the Zebra puzzle | a program, root dead | no program: a loop to the budget | a program, root dead | no program |
| a word text answered right | 0 | 1 (`kinds_full`) | 0 | 1 |
| the check's findings in the complete programs | 34 | 111 (`undeclared` 63, `wrong-count` 16, 16 `:source` no sentence) | 47 | 111 |
| calls; generation | 49; 1 745 s; 7 calls to the budget | 16; 376 s; 1 to the budget (Zebra) | 48; none to the budget | 16; 1 |
| prompt tokens, a word text | about 28 600 (4 600, 11 600, 12 400) | about 11 700 | | |

What the table says, read column against column:

- **No gain in answers.** Loads are equal, and the one call has more *one* and the one right word answer. That
  answer is not clean either: its query is `(have-together (n z 8) Apple)`, the model's own sum written into the
  goal (`answer-in-template`), which the calculator then confirms.
- **The split alone costs.** With no grammar the steps load 11 programs and the one call 14. Four programs are
  lost to slips between steps: step 2 binds the calculator's `sum-of` twice, once in the ontology it completes and
  once in the import (`gap`, `kinds`), and step 3 writes a second `(solve …)` (`boxes12`, `boxes13`); one the
  grammar refuses loads without it (`more`). One call has no later step to contradict an earlier one.
- **The grammars buy it back, and they are what the steps are for.** A step's grammar is rendered on what is
  already written, so it can quote the text's sentences and close the vocabulary; under the whole grammar's root
  the one call's output is its free output but for two tokens. The findings fall from 111 to 34 for that reason.
  The step boundary is what lets a grammar know the program; that gain would stay under any cut.
- **The time is the loops.** Seven of the steps' 49 calls ran to the budget: 86 016 of the 96 412 tokens the
  steps generated, 89 % (counted here from the tables). The other 42 calls wrote 10 396 tokens, against 9 482 for
  the one call's fifteen that ended: the same work. A step's grammar refusing a token is what starts each loop ([L-M2.19](../findings/model/L-M2.19-a-grammar-s-refusal-sends-greedy-decoding-into-a-loop-it-does-not-enter-free.md)).
- **The Zebra puzzle in one call is no program at all**, with the grammar and without. A dead program in steps is
  more to work from.

So B1's caution (*evaluate whether this intermediate representation helps rather than assuming that it does*) has
been evaluated once, and the answer so far: **the steps help EinAf hold the output; they have not helped the model
formalize.** The default for B2's mode, *the steps* ([Q-M2.32](../open_questions/formalizer/Q-M2.32-which-mode-does-baseline-b2-run-with.md)),
rests on the Zebra row and on the findings, not on answers.

### 11.2 What the two partial parts cost

**The partial ontology of step 1.**

- **It is written twice.** Step 2 writes *the ontology whole*, step 1's again: of the 208 forms the sixteen step 2s
  wrote, 75 are step 1's form for form, 36 %, and 38 of the Zebra puzzle's 69 (a throwaway count here, white
  space folded). On the thirteen word texts step 1's partial ontology is one to four declarations and no
  membership at all.
- **It is not revised.** *Partial* was to mean *expected to change*. Step 2 dropped or changed no declaration on
  any of the sixteen texts: *additions only* ([the steps' report § 5](../steps/README.md#5-step-by-step-under-the-steps-grammars)).
  What step 1 decides of the kinds, with no theory in view, stands. On the Zebra puzzle that is `Thing` for every
  drink, pet and cigarette, and the six extra houses; step 2 then added `Smoker`, `Drinker`, `Owner` and `Color`
  beside them and left them empty, which is one of the two faults that kill root.
- **Where a step did revise, the program holds both versions.** The seating's step 1 wrote the three people as
  chairs, `(instance Ann Chair)`; step 2 wrote them as persons, which is right. Step 1's facts are never
  rewritten, so the assembled program says Ann is a chair and a person, the check counts `rewritten` 3, and the
  kernel's core is made of the stale ones: `(instance Ann Chair)`, `(instance Bob Chair)`, `(sits Ann Bob)`
  (re-run here; with the three stale lines removed the program is dead of its theory instead). The one time the
  model used *partial* as it was meant, the design had no way to take the change.
- **It makes a fault of what is no fault.** Step 1 is asked to declare what it uses, so a fact of a relation it
  did not declare is `undeclared` at step 1 (9 findings). Asked for no declaration, it could not miss one.

**The partial theory of step 2.**

- **The import's list is written a step before its uses.** The Zebra puzzle's step 2 lists `(symmetric includes
  converse top meet)`, where the hand and the `shops` example list `compose` too. Step 3's grammar, rendered on
  that program, then offers `(converse …)` short and `compose` only as `(std.algebra.compose …)`, which no example
  shows (rendered here on both programs). Step 3 wrote no `compose`, and the read-back needs it.
- **The mechanism is settled in one step and stated in the next.** Step 2's `slot-partition` makes the
  obligations the writer of H; step 3 states `(solve :no-search)`. Two steps, two answers to one question.
- **The query is written before the relation it reads exists, and the ontology then claims it.** `gap`'s step 1
  asks `(sum-of (n z 5) (n z 2) ?n)`; step 2, completing the ontology, declares `sum-of` as the program's own; the
  import then cannot list it; step 3's rules read a `sum-of` nothing derives. Four texts (`name-not-listed`).
- **The second call of step 2 was used once in sixteen texts**, and wrote comments to the budget.

### 11.3 What the partial parts buy

- **A step 1 that loads alone.** A query may read only a relation the program names. Step 1's declaration of the
  query's relation is what makes its program load where no statement uses that relation, and the model's query is
  of that kind on 8 of 16 texts.
- **The kernel's one judgement in the middle.** Today's step 2 is an ontology with the theory of shared places
  over it, and that is the first program the kernel has something to say of: by hand, the obligations' 112
  candidates and one state with no row; for the model, a dead root with a core. Of an ontology alone it says
  nothing ([§ 11.4](#114-the-strict-split-by-hand)).
- **The notes' sense**: facts change least, the ontology and the theory are searched for around them.

Nobody reads the middle judgement in a one shot (default 39), and the loop would read the same thing off the
complete program. So what the partial parts buy is real and small; what they cost is in the two dead roots and
four of the word texts.

### 11.4 The strict split, by hand

**Not investigated.** S2.2.4's conditions vary the grammar and steps against one call, never what a step may
write. The first re-plan's four passes (ontology, theory, instance, query) were a strict split in another order,
replaced by the notes' steps before any run. S2.2.1 decided *what a later step may change of an earlier one* by
default (*adds, does not rewrite*), unmeasured. [P2.6](../../p2.6_ablations/README.md)'s G8 puts a JSON model in
front of the steps, and its oracle arms give the model parts of the canonical program
([S2.6.4](../../p2.6_ablations/s2.6.4_representation_ablations.md)); neither cuts the steps another way. The
steps by hand exist in the partial cut only.

**Cut here, with no model.** Four of the steps by hand, each complete program dealt into three strict parts
(statements and the query; declarations, memberships and kinds; everything else), and the kernel asked at each
cut (`einaf outcome --no-search`, `ein show hyps`); scratch files, not kept:

| text | 1: statements and the query | 2: and the ontology | 3: and the theory |
|---|---|---|---|
| zebra (20, 54, 38 forms) | loads, root alive; H `blind`, **no candidate** | root alive; H `blind`, **20 441 candidates** | H `obligations`, 112; *one*, *The Norwegian drinks water. The Japanese owns the zebra.* |
| seating (10, 10, 13) | root alive; no candidate | `blind`, 209 | `obligations` |
| apples (3, 2, 6) | root alive; no candidate | `blind`, none | `no-search` |
| youngest (3, 3, 7) | **refused at the load**: the goal writes `(youngest ?who)` and the program names no such relation | `blind`, 10 | `no-search` |

So a strict split is ordinary Ein and answers the same; a strict step 1 is not always a program on its own; and
the kernel has nothing to say of facts alone or of an ontology alone (its H is the blind enumerator's, set by how
many places the declarations have, E-114). In a strict split the kernel is first worth asking when the theory is
in.

### 11.5 Proposal 25: the cut, measured

**Each kind of form is written once, by one step, and no step writes a list for a later one.** That much the
records support. Whether the ontology is then a step of its own is not settled by them: an ontology exists for
the theory (which kinds go under the partition, which places the calculator reads), so written before the theory
is chosen it is written blind, which is today's step 1 moved one step on. So the proposal is the measurement, of
two cuts beside the two the phase has:

| the cut | step 1 | step 2 | step 3 |
|---|---|---|---|
| today's | statements, the answer query, a partial ontology | the ontology whole, imports, selection | the rest of the selection, rules, the mechanism |
| **strict** (the user's) | statements and the queries | the ontology: declarations, memberships, kinds | the theory: imports, selection, rules, the mechanism |
| **two steps** | statements and the queries | the ontology and the theory, one call | |
| the one call | everything | | |

- **What a strict step's grammar can hold.** Step 2's is rendered on step 1's facts: one declaration for each
  relation in use, at the count it is used at, and one membership for each name; only the labels, the kinds and
  the `:why` are the model's. `undeclared` and `wrong-count` cannot be written, nothing is written twice, and a
  name with no kind cannot be left ([proposal 4](#93-what-einafs-check-can-see-where-the-kernel-is-silent)).
  Step 3's has the import and its uses in one call, and renders the mechanism from the selection made in the same
  call ([proposal 5](#92-what-a-prompt-or-a-grammar-can-prevent)).
- **The queries**: with the statements, in the question's own words (proposals 1 and 2); a relation only a query
  reads is declared by step 2 and concluded by step 3, never imported under the same name. Step 1's program is
  judged by the parse and the check; the kernel is asked from the step that brings the theory.
- **If a middle judgement is wanted**, step 3 is two calls, the selection and then the rules a marker asks for,
  and the kernel is asked between them: today's step 2, with nothing written twice.
- **A later step may not restate an earlier one's fact.** The seating's two versions cannot arise: memberships are
  step 2's alone.
- **The oracle form**, which separates a step's own difficulty from what it inherits: step 2 given the hand's
  step 1; step 3 given the hand's steps 1 and 2. It is S2.6.4's oracle axis, taken early and on six texts.
- **What is compared**: programs that load, outcomes, the check's findings by kind, the Zebra and seating programs
  against the hand's step by step, calls to the budget, tokens. On the sixteen texts, as a development number
  (the caution of [§ 9](#9-proposals-for-the-other-issues) holds); about a hundred calls and a dozen oracle ones.
- **First, with no model**: the six steps by hand cut strictly, as files a test of EinAf's gate step judges, each
  cut loading or refused for the reason stated, each complete program the same forms as today's and so the same
  answer; and the step grammars of the two cuts, each accepting its part by hand.
- **Where it lands**: a mode beside `--mode steps|whole` in `einaf::formalize`, the grammars in
  `einaf::grammar::steps`, two templates; the contract's § 3 says which cut is the formalizer's once measured.
  **It needs the user's word**: the three steps are the user's notes.

**What I expect, as a guess to be checked and no more**: the strict cut removes the four `name-not-listed`, the
seating's stale memberships, the mechanism's contradiction and the missing `compose`, and it does not touch what
step 1 gets wrong in the statements, which is most of the Zebra puzzle's distance. The two-step cut tests whether
anything but taking the statements out first matters.

## 12. What a library says of its rules to whoever selects them

**The user's words** (2026-11-27), kept as stated:

> Probably stdlib, or other lib the LLM can import, should carry NL description/documentation of existing rules
> when it selects what to use.
> Because, if I take my own understanding:
>     symmetric - is quite simple
>     includes, compose, converse - is harder to understand
>     top, meet - absolute ambiguity
>     slot-partition, slot-spatial - may look simple, but at the same time exact semantic is not obvious

**In short.** The user's ranking is the model's record, rule for rule. The model is not without descriptions:
EinAf's catalogue has one for each of its nineteen entries. But they say what a rule *derives*, not what it is
*for* or what its arguments must be; they are EinAf's and not the library's, whose own words for a rule are its
`:why`, a sentence about one firing; and a library other than the standard one has none at all, which is the one
condition of the phase in which the model took up nothing. So: yes, and [proposal 26](#124-proposal-26-a-rule-described-for-its-use-by-the-library)
is what such a description holds, where it lives, and how it is kept true.

### 12.1 The ranking against the model's record

The two attribute puzzles under the steps ([`steps/runs/steps/`](../steps/runs/steps/)), against the steps by hand:

| the user | the rule | what the model wrote |
|---|---|---|
| *quite simple* | `symmetric` | the Zebra puzzle's two, right; the seating's two, left out |
| *harder* | `includes` | five on the Zebra puzzle, each a verb read into the right relation (the hand has nine); on the seating `(includes sits next-to)`, one verb for *sits in* and *sits next to* |
| | `converse` | four written, **four wrong**: a verb paired with a rectangle, `(converse smokes smoker-and-house)`, where it pairs a verb with its passive, `(converse drinks is-drunk-by)` |
| | `compose` | **none**, of the hand's six; step 2 left it off the import ([§ 11.2](#112-what-the-two-partial-parts-cost)) |
| *absolute ambiguity* | `top`, `meet` | five and five on the Zebra puzzle: the example's pattern with the names changed, four rectangles over kinds with no member, and `(meet same-house color-and-house lives-in)`, `(meet same-house thing-and-house has)`, which read back nothing the query asks |
| *looks simple, the exact meaning not obvious* | `slot-partition` | its form right both times; on the seating its link is `next-to`, the neighbour relation; on the Zebra puzzle the kinds under it are empty or of unequal size |
| | `slot-spatial` | one of the hand's two on the Zebra puzzle, none of its one on the seating |

Where a description is plain the selection is right; where the user finds it ambiguous the model reproduces a
shape and misses the meaning.

### 12.2 What a selector is told today

**Three texts say what a rule is, and all three say what it derives.** For `(top R isa Dom Ran)`:

| | its words | whose |
|---|---|---|
| the rule's `:why` | *(top {?R} {?isa} {?Dom} {?Ran}): {?a}∈{?Dom},{?b}∈{?Ran} → {?R}({?a},{?b}).* | the library's; a sentence of one firing |
| the reference page | `(R a b)` for every member `a` of `Dom` and `b` of `Ran`: R ⊇ Dom × Ran; *extensive* | Ein's documentation |
| the catalogue's entry | *R holds between every member of Dom and every member of Ran, a member being what the membership relation isa says.* With a program: *Ann and Rex are a person and a pet.* | EinAf's, by hand ([07-steps § 1](../../../../docs/einaf/07-steps.md#1-the-catalogue)) |

- **None says what it is for.** In every program of the phase `top` is half of one idiom: `top` makes the
  rectangle of a kind and the places, `meet` cuts the link by it, and what is left is a verb read back (*a drink
  is drunk in the house it shares*). That use is written once, as a comment inside `slot-partition`'s example.
  The names are the lattice's, ⊤ and ∩; the slots page of the reference does not mention a read-back.
- **A `:why` is written for a firing.** For a check it is the complaint: `functional`'s is *{?R} not functional:
  {?a} has both {?b} and {?c}*. For `slot-partition` it is a list of ten activators. Read as a description it
  says the wrong thing or nothing.
- **`slot-partition`'s one sentence holds five roles and four conditions**: *R is an equivalence whose classes
  are the places: each place holds exactly one member of every kind that is a sub of Super, and its member of the
  kind Index names it.* The two misuses of the runs are each a condition inside it that nothing sets apart: the
  link is the same-place relation (the seating), and every kind under `Super` has a member for every place (the
  Zebra puzzle). The reference adds a trap in passing: transitivity holds of R *but must not be declared*.
- **Neither module declares its selecting facts**, so no form of the library names an argument's role.
- **The model reads what it is given.** Its steps 2 and 3 repeat the examples' comments nearly word for word
  ([§ 3](#3-why-the-zebra-one-shot-missed)). Given mechanics, it wrote mechanics.

**For a library that is not the standard one there is nothing.** A directory of modules given beside the library
is shown *each by its forms, no comment of its file*: its declarations, and each rule's name, parameters and
`:why` (`einaf::catalogue::Given::text`). Thirteen runs had the word problems' own modules shown so, offered to
the grammars and loadable: **no program imported one**. The standard library, shown with a description and a
worked program an entry, was selected on every text as its entries spell it. It is the phase's nearest
measurement of the user's point, and it was not designed as one: the two differ in more than the description.

The milestone's evaluation has the neighbour of this, [E-27](../../evaluation/stdlib/E-27-no-one-catalogue-says-how-each-library-property-is-spelled.md)
(*no one catalogue says how each library property is spelled to be active*): that is the spelling. This is the
meaning, and it is a candidate for the same ledger: **a library cannot say, in the library, what a selecting
fact is for or what its arguments must be.**

### 12.3 The eight rules, in the words of their use

A first draft of what is asked for, from the reference page, the catalogue and the steps by hand, whose programs
the gate runs; the wording is not checked by anything, and its first test is the user's own reading.

| the fact | what it is for | each argument, and what it needs |
|---|---|---|
| `(symmetric R)` | R holds both ways: *next to* | R, a relation of two places |
| `(includes R S)` | every R is also an S: a verb is one way of standing in a wider relation. *Lives in* is a way of being in the same house: `(includes lives-in same-house)` | R the narrower, S the wider; nothing comes back from S to R |
| `(converse R S)` | S is R said from the other end: a verb and its passive, `(converse drinks is-drunk-by)` | two relations over the same pairs, each place exchanged; never a relation and a rectangle |
| `(compose R S T)` | follow R, then S, and call the result T: what is drunk by a man who lives in a house is drunk in that house, `(compose is-drunk-by lives-in is-drunk-in)` | R's second place and S's first are the same kind of thing; `compose` is listed in the import with the rest |
| `(top R isa Dom Ran)` | nothing about the problem: it makes a mask, every pair of a `Dom` and a `Ran`, for `meet` to cut with | both kinds have members, or the mask is empty and what is cut with it is empty |
| `(meet R S T)` | T is where R and S both hold. With a mask: of all the pairs in one house, those of a drink and a house are *is drunk in*: `(top drink-and-house instance Drink House)`, `(meet same-house drink-and-house is-drunk-in)` | T is the verb whose two places are the mask's two kinds, in that order |
| `(slot-partition R isa sub Super Index)` | things in places: every place holds exactly one thing of each kind. It is what proposes the candidates | **R** the same-place relation, stated symmetric and never transitive, the verbs of one place read into it by `includes`; **isa** the program's membership relation, **sub** its kind-of relation; **Super** the kind whose sub-kinds are dealt out; **Index** the sub-kind whose members are the places. Every sub-kind has as many members as `Index`; and everything that stands in R is a member of one (this last is my reading, not run) |
| `(slot-spatial R S isa Index)` | a relation between places holds between the things in them, and back: *the green house is right of the ivory one* exactly where their places are | S is stated between the places themselves (the row, which a text rarely states), and the sentences' own relations are read into S by `includes`; one fact for each such relation |

What the kernel shows when a need is not met, from the re-runs of [§ 3](#3-why-the-zebra-one-shot-missed): a
kind with no member, root dead by `slot-no-fill`; a thing in two places, `slot-no-room`; and, as § 3 reads its
last run, more members than places, every node owing. A description that names its needs is also what
[proposal 4](#93-what-einafs-check-can-see-where-the-kernel-is-silent) evaluates: one list, read by the model as
words and by the check as conditions.

### 12.4 Proposal 26: a rule described for its use, by the library

**What a description holds**, for each fact a program may select, and for each idiom of several (the read-back is
one thing to a selector and three rules to the library):

1. what it is for, in a sentence of the problem's kind and not of the rule's algebra;
2. each argument's role by name, and what must hold of it;
3. what it gives the hypothesis mechanism;
4. what the kernel shows when a need is not met;
5. what it is written with (`slot-partition` with `symmetric` and `includes`; `top` with `meet`);
6. a program that uses it, run by the kernel, with its answer.

The catalogue has 3 and 6 and part of 1 today.

**Where it lives.** Four places, each with its cost under Ein's rules:

| the place | what it takes | what it costs |
|---|---|---|
| the module's comments, read by EinAf | nothing of Ein: the comment above a rule is where the library's authors already say what it is; EinAf drops it today | words nothing checks, in the lattice's vocabulary as they stand, and no form for an argument's role |
| a declaration of the selecting fact in the module, its labels the roles, its `:why` the sentence | no new syntax: a declaration's `:why` already reads its own arrows | a declaration is a fact others read: the blind enumerator guesses at every declared relation and `std.contradiction` activates from every declaration, so it would have to be shown to leave every answer as it is |
| **a described program beside the module**, one a selectable fact or idiom, in the library's tree | the catalogue's entries moved from `einaf/catalogue/` to the library, as corpus programs the gate runs; a library is then its modules and their described programs, and a directory given beside it is read the same way | a place and a layout to choose in Ein's tree; `tests/stdlib/` has a program a rule already, written to break and not to teach |
| a keyword of the language on `rule` and `macro`, beside `:why` | the description a part of the form, read by every tool | a change to Ein's syntax, which M2 does not make |

**The proposal**: the third now, with the first's words where they serve; the fourth written into the evaluation
for the iteration of Ein that follows, where it belongs with E-27. Then:

- **EinAf's catalogue is generated from the library's described programs**, not written beside them, so the two
  cannot part; a module with no described program is shown as undescribed, or not offered.
- **`--module-dir` reads the same layout**, so the word problems' modules come with what they are for, and the
  run that imported none can be made again.
- **Each description's program is run**, as the catalogue's entries are today (`einaf/tests/catalogue.rs`), and
  its needs are the conditions proposal 4 evaluates.
- **For `top` and `meet` a description is the lesser repair.** The greater is that a selector never composes
  them: one fact a verb, named for its use ([proposal 8](#94-what-the-library-and-the-kernel-owe),
  [Q-M2.23](../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md)).

**How it is measured**, with the model, on the two attribute puzzles and a few like them, in the oracle form of
[proposal 25](#115-proposal-25-the-cut-measured) (steps 2 and 3 given the hand's step 1, so that no misread
statement hides the selection): the catalogue as it stands; with descriptions of use; with the names alone.
What is counted is each selecting fact against the hand's. B3 has the catalogue as an input of its own, and
[P2.6](../../p2.6_ablations/README.md)'s G9 varies which library is listed, not how it is described.

**It needs the user's word**: on where a library's descriptions live, which is Ein's and not EinAf's to settle;
and the first reader of § 12.3's draft is the user.
