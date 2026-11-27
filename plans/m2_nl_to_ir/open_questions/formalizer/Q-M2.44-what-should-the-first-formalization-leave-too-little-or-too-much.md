# Q-M2.44 — What should the first formalization leave: too little, or too much? And how does a statement carry its confidence?

**Status:** open; measured by [S2.2d.7](../../p2.2d_formalizer_corrected/s2.2d.7_first_iteration_and_relaxation.md), then the user's

**Raised 2026-11-27 by the user**, of [P2.4](../../p2.4_loop/README.md), the loop, as a question for
[P2.2d](../../p2.2d_formalizer_corrected/README.md), the formalizer corrected:

> plans/m2_nl_to_ir/p2.4_loop - related question, that is already should be started in
> plans/m2_nl_to_ir/p2.2d_formalizer_corrected - what is better to have after first formalizer iteration?
> underconstrained formalization with many states, or overconstrained inconsistent? We can delete constaints and
> automatically iterate over formalization variants, but can not generate new constaints. On the other side on
> second iteration LLM can add new constaints, but not easy to remove - will need to re-write whole program
> probably, right? Also maybe we can introduce some new kwarg for LLM to mark facts with some confidence level,
> e.g. it may write (instance Red    Color :confidence .99) and (slot-spatial   same-house next-to instance House
> :confidence .75) and for overconstarained program we then try to solve programs removing less confident facts

## The coordinator's reading, which the user may correct

**The asymmetry is real.** Taking statements out can be searched without a model, and putting a missing one in
needs the text read again.

**But a wrong statement is both.** The model's errors P2.2 recorded are mostly *wrong*, not *extra*: a polarity
reversed, a passive written as a second active relation, *Old Gold* split, a kind wrong
([L-M2.18](../../findings/model/L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md),
[L-M2.24](../../findings/model/L-M2.24-the-model-s-zebra-program-has-seven-defects-and-three-repairs-leave-r-empty.md)).
Deleting a wrong statement leaves its right reading missing. A puzzle whose every clue is needed, as the Zebra
puzzle's are (clue 15 dropped: 32 states), then has several states, and the right reading must be added by
reading again. So deletion turns *inconsistent* into *consistent*, not into *right*, unless the text says the
same thing twice.

**What each side tells the loop:**

- **Inconsistent** comes with a core: the facts that cannot hold together and, through their `:source`, the
  sentences (E-125, a strength: a dead root's core names the sentence the model misread). It is local. Two cautions:
  a core is an explanation, not the smallest set
  ([Q-M1f.6](../../../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md); F22 makes it exact), and a
  theory whose activating fact is missing reads as statements that contradict each other
  ([E-107](../../evaluation/diagnostics/E-107-a-theory-s-missing-activating-fact-is-reported-as-the.md)).
- **Several states** come with the answer query's classes, which say *what* is left open but not *which sentence*
  was dropped or misread. The contract's check sees one kind of omission: a sentence that no fact quotes.

**So the default target of the first iteration is coverage, not a side.** Every sentence written, each fact
quoting its sentence, and a reading the model is not sure of written as **alternatives** rather than committed one
way. An omission is what neither the kernel nor the check can see; a contradiction the kernel localises. That
leans toward *too much* in the sense of *everything the text states*, never in the sense of anything the text does
not state ([S2.4.2](../../p2.4_loop/s2.4.2_repair.md)'s E3 failure, *several → invent → one*).

**Removing without a model is already Ein's**, with no new keyword. A statement written as a hypothesis,
`(open (drinks Ann Tea))`, is one the kernel may take or leave. `ein solve --maximal` then lists the maximal sets
of statements that hold together. Probed on the current binary: three statements, two of which exclude each
other, give two maximal states, `{Ann Milk, Bob Tea}` and `{Ann Tea, Bob Tea}`. The pattern for one sentence's
readings is [S2.4.5](../../p2.4_loop/s2.4.5_alternatives_as_hypotheses.md)'s. **One caution, the coordinator's
and not probed**: in a puzzle whose own search is hypotheses too (`std.slots`' placements), one lattice mixes
statements left out with placements made, and the maximal line holds every solution of each relaxed puzzle. So
relaxing the statements is a level above the puzzle's search: by cores, one statement of each core left out and
the kernel asked again, or by the statements' hypotheses alone with the puzzle's question asked of each.

**Removing with a model need not mean rewriting**: S2.4.2 makes *repair in a kept context* one of its two
strategies, told where by the core. A program here is short, so writing it whole again is cheap. Ein's answer
does not depend on the order of its forms
([Q-M2.24](../kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md)), so a statement
taken out or put in anywhere is safe.

**Confidence**:

- **Not as a keyword of Ein.** `:confidence .99` would be a new keyword with a number. Ein has no numbers, a form's
  keywords are a closed list, and the user's principle is that Ein's language does not change to help a model.
- **EinAf can carry it.** It can be a word of the model's own dialect, admitted by the grammar the model writes
  under and stripped by EinAf before the kernel reads the program. Or it can be **measured** rather than stated:
  llama.cpp gives each token's probability, so a statement's probability under the model is a confidence the model
  did not have to write. A small model's stated numbers are a known weak signal, and that is the reason for the
  second.
- **Its use is an order**: which statement of a core is left out first, or which maximal set is kept, read by
  EinAf. A statement below a threshold can also be written as a hypothesis from the start.

## The default, until the user rules on S2.2d.7's measurement

The first iteration aims at coverage: every sentence written, uncertain readings as alternatives. An inconsistent
program is relaxed by cores, by EinAf with no model, in the order of the statements' measured probability, before
the model is asked again. No keyword is added to Ein.
