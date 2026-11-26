# P2.2b — Guided thinking: the model decomposes, Ein solves

**Status:** **planned 2026-11-26** on the user's word, *"add new phase p2.2b for guided thinking"*. **Not started**:
no stage runs, not even a survey, until the user says so.
**Depends on:** [P2.2](../p2.2_formalizer/README.md): the steps written by hand on six texts
([S2.2.1](../p2.2_formalizer/s2.2.1_contract.md), `einaf/contract/`); the runtime, its records of thinking on, and
step 1 with the model ([S2.2.2](../p2.2_formalizer/s2.2.2_model_runtime.md)); the grammars, a grammar begun at the
model's token `</think>`, and the narrowed root a model writes under ([S2.2.3](../p2.2_formalizer/s2.2.3_gbnf.md));
the steps' prompts, the catalogue, the contract's check and the steps' command, whose runs with thinking off are
this phase's baseline ([S2.2.4](../p2.2_formalizer/s2.2.4_passes.md)); `einaf from-text`
([S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md)).
**Informs:** [the arms](../README.md#the-arms) (B1, a chain of thought; B2 to B5, the filter with thinking off);
[P2.6](../p2.6_ablations/README.md)'s *thinking before the program*, which as planned measures unguided thinking;
[Q-M2.27](../p2.2_formalizer/open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md),
greedy thinking that loops.
**Research:** Ye et al., *SatLM* (2023), a declarative specification handed to a solver instead of a chain of
thought ([the reading list](../reading.md)).

## Goal

**The model's thinking is spent on decomposing the text into the parts an Ein program is made of, and not on
solving the problem**, which is the kernel's. The phase builds the means to guide the thinking (the questions it
answers, a scaffold it continues, a grammar over it, a budget), and measures what guided thinking gives against
thinking off and against thinking left alone: step 1 first, where the model did worst, then steps 2 and 3 and the
program written in one call.

## The user's words (2026-11-26)

> can we guide the model's thinking anyhow? recommend some questions to decide instead of solving the problem
> itself inside thinking? e.g. identify involved objects, relations, properties, attributes, types/classes/kinds,
> groups, sets, orders, processes, dependences. Like not solving the entire problem but decomposing it into parts
> suitable for conversion into Ein.

> add new phase p2.2b for guided thinking

The user was first offered guided thinking as a third condition of S2.2.4; the user made it a phase of its own, so
S2.2.4 runs as it is planned, thinking off, and its runs are this phase's baseline.

## What P2.2 found that this phase starts from

- **Thinking on, the model solves, and then loops.** Alone on the Zebra puzzle, `qwen3.5:4b` solved it in its
  thinking by about its 5 900th token and from its 6 459th repeated an 85-token cycle to the budget; with its own
  program as the example it decided to copy by about token 2 500 and looped from token 6 154. **36 of the probe's
  36 runs with thinking on** ended in a loop the budget cut, cycles of 11 to 911 tokens begun between tokens 249
  and 8 901 ([S2.2.2's report](../p2.2_formalizer/model_runtime/README.md)). Every run was greedy
  ([Q-M2.27](../p2.2_formalizer/open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md)).
- **A grammar after the thinking costs the thinking nothing, and never begins when the thinking loops.** It begins at
  the model's token `</think>` (`einaf::model::GrammarStart::Token`); on short prompts it began at tokens 268 and 310,
  on the steps' prompts the thinking looped and never closed ([S2.2.3's report](../p2.2_formalizer/grammars/README.md)).
- **Thinking off, step 1 fails on exactly the decisions the user's list names.** Six of six replies load, and only
  `youngest` matches the hand-written step 1. On the Zebra puzzle: no `:source` at all, "Old Gold" split into two
  arguments, the passives made active, none of the hand's 44 facts; on the seating, placements no sentence states;
  on the word problems, a number taken out of its fact or written as a name. Coreference was mostly right
  ([S2.2.2's record](../p2.2_formalizer/s2.2.2_model_runtime.md#record), T2.2.2.8).

## The questions

Each of the user's words is a question the thinking answers, and each answer decides a part of the program. **The
scaffold says it in so many words: decide what the text is made of; do not decide who or what, Ein does.**

| the user's word | the question | it decides | the part of an Ein program | step |
|---|---|---|---|---|
| (the text) | Which sentences state, and which ask? | statements, the question | facts with their `:source`; the answer query | 1 |
| objects | What things does the text name or describe? Which words name the same thing? | the objects, coreference | names | 1 |
| types, classes, kinds | What kinds are there, and which thing is of which? | the kinds | memberships (`instance`, `type`, `is-a`: names the program chooses) | 1, 2 |
| relations | Which words relate things, between how many, and which pairs say one relation actively and passively? | the relations, their places, converses | `(relation …)` with its `:why`; `converse` | 1, 2 |
| properties | What is said of one thing alone? | one-place facts | a relation of one place, as `zebra3.3.ein`'s adjectives | 1 |
| attributes | What does each thing have one value of (a colour, a drink, a number of apples)? | attributes and their values | a relation from a thing to a value; the values' kind | 1, 2 |
| quantities | Which values are numbers, and which operations does the text ask? | numerals and requests | `(n z 5)`; the calculator's requests | 1, 3 |
| groups, sets | What is grouped, and is a group given whole? | groupings, whether a set is closed | memberships; a partition (`slot-partition`); whether the text lists every member | 2, 3 |
| orders | Is anything in a row, a ranking, before and after? | an order and its properties | an order relation; `transitive`, `irreflexive`, `asymmetric`, `connex`; positions (`slot-spatial`) | 2 |
| (one-to-one) | Does each thing take exactly one value, and each value one thing? | the assignment's shape | `bijective`, `total`, `injective`, `functional`; `std.slots` | 2 |
| processes | Does anything change: a transfer, a moment before and after? | states over time | rules of the program's own: the library has no theory of time | 2b, 3 |
| dependencies | Which sentences say *if … then*, or that one thing follows from another? | rules | rules, after the marker of the property they supply | 2b, 3 |
| (the unknown) | What does the text leave open, and does it state every fact there is? | the search | `(solve :no-search)`, the selection's obligations, or `(open F)` | 3 |
| (the question) | What is asked, which relation answers it, and what is the answer sentence? | the answer | the answer query's `:goal` and `:goal-text` | 1 |

## The four ways to guide the thinking

EinAf renders the chat template itself and owns the loop over tokens
([`05-model.md`](../../../docs/einaf/05-model.md)), so it has four, from the weakest to the strongest:

1. **Instructed**: the prompt asks for the questions to be answered in the thinking, and says not to solve.
2. **Scaffolded**: the assistant's turn is begun for the model, `<think>` and then the first heading of the
   questions, and the model continues it.
3. **Held to a grammar**: the thinking itself under a grammar of the questions' sections, in order, each a bounded
   list of short lines, then `</think>`, then the step's own grammar inside its fence. A grammar over the whole output
   took the thinking in [the host probe](../analysis/host.md#what-the-model-does-through-llama-server-a-probe)
   because it left the thinking no room; this one is the thinking's shape. A bounded section has nowhere to loop.
4. **Budgeted**: at a stated count of thinking tokens EinAf writes `</think>` itself and the step's grammar begins.
   With any of the three above, and on its own.

## Design questions, each with its default

1. **Where the analysis lives.** In the thinking (default), or as a visible output of a step of its own before step
   1. Either way the record keeps it; in the thinking the step's output stays a program and nothing else.
2. **How an answer is written.** Under fixed headings, one item a line, **a thing written as the name the program
   will spell it**, with the text's words beside it in quotes, so that the analysis can be compared with the
   program mechanically (default).
3. **Which questions a step asks.** Step 1 the sentences, objects, coreference, kinds, relations, properties,
   attributes, quantities and the question; step 2 the structure (groups, sets, orders, one-to-one, processes,
   dependencies) against the catalogue; step 3 what is left open (default). The one call asks them all.
4. **Is the analysis handed on?** No: the program is what a step hands the next, as the contract has it, and the
   analysis is the step's own (default). Handing it on is a condition S2.2b.3 may add.
5. **Sampling.** Greedy, as the phase's runs are
   ([Q-M2.27](../p2.2_formalizer/open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md)'s
   default): whether a thinking with a shape still loops under greedy decoding is one of the measurements.
6. **The budget** counts the thinking's tokens ([the frame's default 6](../README.md#the-defaults-the-re-plan-took));
   a run is reported at its whole cost.
7. **The examples** follow S2.2.4's rule: never of the item's own family, and every program and analysis a prompt
   quotes is listed.

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.2b.1](s2.2b.1_questions_by_hand.md) | The questions, the analysis by hand, and the thinking's grammar | the questions by step as versioned prompt files; an analysis written by hand for each of S2.2.1's six texts, held to the hand-written steps; the scaffold; the thinking's grammar joined to each step's, accepting the analyses by hand, read alike by llama.cpp |
| [S2.2b.2](s2.2b.2_step1_guided.md) | Step 1 with guided thinking | step 1 on the Zebra text, the word problems and the six texts under thinking off, thinking left alone and the four ways, a plan fixed before the first generation: for each, whether it ends, what the thinking costs, whether it solved, how far its analysis and its step 1 are from the hand's |
| [S2.2b.3](s2.2b.3_steps_and_verdict.md) | Steps 2 and 3, the one call, and the verdict | the structure and the open questions before steps 2 and 3, the one call with every question, the assembled programs' outcomes and answers against S2.2.4's and S2.2.5's; the verdict for the user, a question with its default: where guided thinking enters the formalizer and the arms |

### Order

**The phase starts on the user's word, by default after P2.2 has run its stages**: its baseline is S2.2.4's runs
with thinking off and S2.2.5's `einaf from-text`. S2.2b.1 needs only S2.2.1 and S2.2.3 and can run before S2.2.4 if
the user says so. S2.2b.2 follows S2.2b.1 and S2.2.4; S2.2b.3 follows S2.2b.2 and S2.2.5.

**How the phase reaches the model and the kernel** is P2.2's: the runtime `einaf::model`, built with the feature
`cuda`, the model `qwen3.5:4b` and only it (the user, 2026-11-26: *"Ok, then continue with 4B"*); every program
through `einaf::kernel`'s bounded call; every plan of generations committed before its first generation.

## Acceptance

- The questions, the scaffold and the thinking's grammar exist, versioned, each step's part held by EinAf's tests
  with no model, and the analysis by hand of each of S2.2.1's texts accepted by them.
- For step 1, on the Zebra text and the sixteen word problems, under each condition: whether the generation ended
  or looped, its thinking's tokens, whether the thinking solved instead of decomposing, the analysis against the
  hand's, and step 1's program (parse, load, the contract's check, root's judgement, the hand's facts matched)
  against thinking off. Every run a record.
- For steps 2 and 3 and the one call, the same on the assembled program, with its outcome among the seven and its
  answer, against S2.2.4's and S2.2.5's.
- The verdict is the user's: a question in the phase's ledger, with its default and the runs it rests on.
- What the phase finds of Ein is in [the evaluation](../evaluation/README.md); of EinAf and the process, in its
  findings.

## Risks

- **A 4B model may answer the questions and solve anyway**, inside a line. The grammar bounds a line, not its
  content; the measure of solving is the answer's values in the thinking, read off every run and by hand on a
  sample.
- **A scaffold too tight forces nonsense**, as a grammar too tight did
  ([L-M2.14](../p2.2_formalizer/findings/model/L-M2.14-a-grammar-that-leaves-out-the-fence-a-prompt-asks-for-derails-the-model.md)):
  a model with nothing to say in a section still writes there. Each section allows *none*.
- **The analysis teaches the test**: an example analysis of a near text is a near answer. S2.2.4's family rule holds.
- **Better step 1, same program**: an analysis right and a program wrong says the hand-off from thinking to program
  is the gap, not the decomposition. The comparison is made at both.

## Connections

- [P2.2's notes on the steps](../p2.2_formalizer/README.md#formalization-in-steps-the-users-notes-on-the-whole-phase):
  the decisions of step 1 are the user's list there; this phase asks them of the thinking.
- [P2.6](../p2.6_ablations/README.md), *thinking before the program*: B2 and B5 with thinking on and the grammar
  after it. Guided thinking is a value of that ablation the plan did not have.
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md)'s three neural actions: reinterpretation is what the
  analysis makes explicit.
- [idea 08](../../ideas/08-human-style-deductive-trace.md), the human-style trace: the trace is the kernel's; the
  thinking's part is what the trace reasons about.

## Ledgers

[`findings/`](findings/README.md), [`open_questions/`](open_questions/README.md) and
[`evaluation/`](evaluation/README.md), one item a file under a topic, ids the milestone's next free.
