# P1f.22 — nested facts

**Milestone:** [M1f](../README.md)
**Status:** created 2026-11-17 on the user's instruction (below), and **started 2026-11-17 on the user's word**
(*"start p1f.22"*). [S1f.22.1](s1f.22.1_what_we_have.md) is done: [`analysis.md`](analysis.md) is what the engine
and the pages do, the explanation drafted, and the six points for the user (Q-M1f.391 to .396);
[`findings.md`](findings.md) is what it found outside its scope. The user answered the six with notes, which
[`contexts.md`](contexts.md) works out, with the engine run on what they ask and the drawings they ask for;
[`q392_state_file_and_written.md`](q392_state_file_and_written.md) and [`q394_words.md`](q394_words.md) are the
two explanations the user asked for.
**Where the questions stand** (2026-11-17, after the user's third notes): **ruled** Q-M1f.391 ((c), the
elimination a rule), Q-M1f.392 (**a bug**, to be fixed and held by a unit test), Q-M1f.393 ((a)), Q-M1f.394 (the
words), Q-M1f.395 (every object drawn once), Q-M1f.396 (sections), Q-M1f.397 (the reserved words), Q-M1f.398
(**a frame around the facts in a context**, the filling for held; re-ruled on the first trial, tried again in
[`contexts.md` § 14](contexts.md#14-the-frame-tried-twice)). **P1f.21's**: Q-M1f.399 and `(open)` as facts, with
the user's direction recorded in its V-5. **[S1f.22.2](s1f.22.2_the_explanation_written.md) is done**: the explanation is on the pages, with the framed
drawings and five decisions. **Q-M1f.392 is ruled** in full: an invariance in both
directions, a saved solution the program unchanged with the choices made, in one file. **Every question of the
phase is ruled.** S1f.22.2, S1f.22.3 and S1f.22.4 are done. The work stopped there on the user's word
(2026-11-17, *"stop after current stage"*), and **S1f.22.5, save and load, is started on the user's word**
(2026-11-17, *"start s1f.22.5"*); its first task ends with the user's look at the format, and the close is not
started without the user's word. What waits for the user is in [`findings.md`](findings.md): N-15, N-22 and N-23
to N-28.
**Depends on:** nothing now. Its second stage writes into pages of `docs/ein/reasoning/` that S1f.20.10, the
last stage of P1f.20, was still moving; that stage is done and P1f.20 is closed (2026-11-17). What P1f.20 found
on those pages and left is [the milestone's findings page](../findings.md); its N-14 here came from there. Its
first stage reads the engine and the pages and writes here.
**Blocks:** the close of M1f, paused on the user's word; it resumes when this phase and
[P1f.21](../p1f.21_verdicts_as_rules/README.md) are done.

---

## The instruction

The user, 2026-11-17, verbatim:

> another new phase in M1f - nested facts, we have to decide at least how to represent them in
> docs/ein/reasoning and how they interact with other facts via rules; because obviously (= 5 (+ 2 2)) is not the
> same as (think Alice (= 5 (+ 2 2))) ; but while rules can pack and unpack facts - then the same fact can be in
> KB and in another fact context or can be only in KB or only in fact context; need to analyze what we have right
> now and add consistent explanation of how nested facts work

## What it is

A fact may point at a fact ([the knowledge base](../../../docs/ein/reasoning/01-static-ir/01-knowledge-base.md):
an arrow of a fact points at an object, and an object is an atom or a fact). One list of arrows is one node, so
the fact `(= 5 (+ 2 2))` is the same node wherever it is written: stated by itself, or under an arrow of
`(think Alice (= 5 (+ 2 2)))`. What differs is what the knowledge base **holds**. That gives three cases for one
fact, and the instruction names them:

| | the knowledge base holds it | another held fact points at it |
|---|---|---|
| **only in the knowledge base** | yes | no |
| **only in a fact's context** | no | yes |
| **both** | yes | yes |

*Alice thinks that five is two plus two* says nothing of whether five is two plus two, so the second case must not
slide into the first. But a rule can **pack** a fact (conclude a fact that points at one it matched) and
**unpack** one (match a fact under an arrow and conclude it, or read its parts), so a program moves facts between
the cases, and what holds then is a matter of what the rules say and of nothing else, if the engine is
consistent about it.

The phase decides, at least, **how nested facts are represented in `docs/ein/reasoning/`** and **how they
interact with other facts through rules**, after analysing what the engine and the pages do now, and writes one
consistent explanation.

## Stages

| stage | what | state |
|---|---|---|
| [S1f.22.1](s1f.22.1_what_we_have.md) | **What we have.** What the engine does with a fact under an arrow of another, run and shown: what is held, what a pattern can match inside a fact, what a rule can pack and unpack, what the engine itself reads or states of nested facts (`written`, a hypothesis, a negation, a claim), what is refused; what every page and decision says of it, and where they disagree with the engine or with one another. Then the explanation, drafted: the representation in the graph, the three cases, packing and unpacking as rewriting. Ends with what is the user's to decide | **done 2026-11-17**: [`analysis.md`](analysis.md), [`findings.md`](findings.md) (N-1 to N-13), Q-M1f.391 to .396 not ruled |
| [S1f.22.2](s1f.22.2_the_explanation_written.md) | **The explanation, written.** In `docs/ein/reasoning/` (the knowledge base, the rewriting rules, and the pages of `02-kb-evolution/` that fire rules over nested facts), every figure of the graph with the frame, the user's pages, and the decisions | **done 2026-11-17**: `9eda1a04`, `729094b8`, `27baf9f2`, record `a9f54d25`; findings N-17 to N-21 |
| [S1f.22.3](s1f.22.3_the_negation_of_a_negation.md) | **The negation of a negation is a fact** (Q-M1f.391): the grammar admits `(not (not g))`, the kernel is not changed, the witnesses are corpus programs, the pages follow | **done 2026-11-17**: `b1a9a635`, record `9519208a`; the gate and the parser's fuzz on both seeds exit 0; finding N-22 |
| [S1f.22.4](s1f.22.4_every_object_drawn_once.md) | **`ein render` draws every object once** (Q-M1f.395, Q-M1f.398): a fact that a fact points at is its node, filled where held, in a frame named for the context's relation | **done 2026-11-17**: `811846ce`, record `e506bfd9`; the gate here and `utils/ci_container.sh` on the commit exit 0; findings N-23 to N-28, each the user's call |
| [S1f.22.5](s1f.22.5_save_and_load.md) | **A saved solution loads back as itself, in both directions** (Q-M1f.392): the program unchanged with the choices made, in one file; the two invariances as tests over the corpus | **started 2026-11-17** on the user's word (*"start s1f.22.5"*); its first task ends with the user's look at the format |
| S1f.22.6 | **The close.** | last |

## Rules of the phase

The standing ones ([`AGENTS.md`](../../../AGENTS.md)): a stage per subagent; a claim about the engine is a run,
not an argument; what a stage finds outside its scope goes to a findings page of the phase's own; no estimate is
written.
