# S1f.6.20 — An application: elementary-school word problems, arithmetic among other relations

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day ([§ The instruction](#the-instruction)).
Not started. Its first task **stops for the user** with example programs before any theory is written.
**Depends on:** [S1f.6.6](s1f.6.6_division.md), the four operations. The protocol is ruled
([the rulings](design.md#the-rulings-2026-11-11)): request facts, result facts read by a goal with a variable in
the result's place, and named constants. **No expression layer is in the core:** every *"together means sum"* is a
rule of the problem's theory from facts to a request. How that reads, and how often a quantity needs two or three
operations, is the evidence [S1f.6.14](s1f.6.14_expressions.md)'s question to the user waits for. It does not need
the extensions, S1f.6.7 to S1f.6.9; where a problem would need a negative number it says so
([§ What it may find](#what-it-may-find)).

## The instruction

The user, 2026-11-11:

> example arithmetic application that involves another relations - add stage s1f.6.20 for it: solving simple
> elementary school problems like "Bob has 5 apples. He gave 2 to Alice. How many apples they have together?"
> where we interested in different thins: 1 - imported arithmetic works without support inside Ein engine; 2 -
> reasoning that infers something like "gave means subtraction", "together means sum", etc. and derives
> arithmetic solution from problem facts; and 3 - detection of not enought information and other facts and
> theory gaps, like "we don't know how many apples Alice had before, so we know that Bob have 3 now, but can't
> answer how many they have together, without silent assumption that Alice had 0 before"

## What it is

The first program in the repo where the calculator is a **library among other relations** and not the subject.
A word problem is written as Ein facts about people, things, events and a question. A theory of what those
words mean turns them into arithmetic requests, the calculator answers the requests, and the theory reads the
answers back as facts about the people. Three things are looked at, the user's three:

1. **The arithmetic is imported and the engine knows none of it.** The stage changes no engine source. A
   test holds that the programs load and answer with the calculator as an ordinary import.
2. **The meaning of the words is rules.** *"gave means subtraction"*, *"together means sum"*: a rule per
   reading, each with a `:why`, so that the trace of a solved problem reads as the reasoning a pupil would
   write: *Bob had 5; he gave 2 away, so he has 5 − 2 = 3; …*. The arithmetic solution is **derived** from
   the problem's facts, never written in the problem.
3. **What is missing is detected and said.** A quantity the problem does not state is not silently zero. On
   the user's example the program answers that Bob has 3 now, and that *together* cannot be answered,
   **naming the gap**: how many apples Alice had before is not known. The assumption that would close the gap,
   *Alice had 0*, is reported as an assumption, and the answer under it, 5, is given only as the answer under
   it.

## What it is not

- **Not natural-language parsing.** The problem's English is in `:source` strings and comments; its facts are
  written by hand. Turning the sentence into the facts is [M2](../../m2_nl_to_ir/README.md)'s, which links here: its
  [§ The reasoning as a program](../../m2_nl_to_ir/README.md#the-reasoning-as-a-program-the-users-idea-2026-11-11)
  takes this stage's programs as its first worked instances. The stage does
  record, per problem, the sentence beside the facts it was encoded as, since that pair is what M2 would have
  to produce.
- **Not engine arithmetic**, and no new built-in. Q-M1e2.24 stands.
- **Not part of the calculator's theory.** The word-problem theory is a module of its own that imports the
  calculator and is imported by a problem. Whether it lives in `stdlib/` or under `examples/` is the user's
  to rule at the stop.

## The tasks

**T1 — the examples, on a prototype, and the stop.** Under `build/s1f.6.20/`, as S1f.6.1 did: a prototype
theory and a set of problems, each run on the release binary, every answer computed. A page,
`p1f.6_calculator/word_problems.md`, shows them with every artefact a run writes (`ein solve`,
`ein saturate --dump`, `ein test`, `--trace`). The problems, at least:

| problem | what it shows |
|---|---|
| *Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have?* | a transfer read as a subtraction; the answer 3, and the trace as the reasoning |
| *… How many apples do they have together?* (the user's) | the gap: Alice's apples before are not stated. Bob's 3 is answered, *together* is not, and the gap is named |
| the same with *Alice had 4 apples* added | the gap closed by a fact: 3, 6 and 9 |
| the same with the assumption stated, *Alice had none* | the gap closed by an assumption, and the answer marked as resting on it |
| *Bob has 5 apples and gave 7 to Alice* | a contradiction in the problem's facts: what the program says, `(false)` or a flag, as the user ruled for division by zero |
| *Bob has 5 apples, Alice has 3 pears. How many apples do they have together?* | two kinds of thing: a sum only over the kind asked about |
| *12 boxes of 144 pencils, 250 go to the office, the rest shared among 31 classes* | several steps, a product, a difference, a quotient and a remainder ([`design.md`](design.md)'s word problem, with its words as facts rather than as `let` lines) |
| a *who has more, and by how much* problem | the comparison |
| *Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need?* (the user's, 2026-11-11) | **three numbers, two steps, and the program explains the plan**: which operations, in what order, and why. *They* fill the boxes together, so the apples are summed first, 5 + 7 = 12; a box *holds* 3, so the boxes are a division, 12 ÷ 3 = 4. See [§ The plan is explained](#the-plan-is-explained) |
| the same with *Alice has 8* (the coordinator's variant) | the division leaves a remainder, 13 ÷ 3 = 4 and 1 over: whether *need* means a fifth box is a reading of the word, and the program says which reading it took |
| a problem with a fact it does not need | an unused fact is not an error, and the trace does not use it |

### The plan is explained

The user, 2026-11-11:

> addition to word problemt s1f.6.20: still simplestic arithmetic problem with 3 numbers that is solved in 2
> steps, e.g. Bob has 5, Alice has 7, box volume is 3, how many boxes do they need? We see numbers 5,7,3, what
> operations we should use, in what order, why? - all this has to be explained by Ein program; and yes here we
> don't cover how this program is synthesized

A problem with three numbers has several computations a pupil could write down (5 + 7 then ÷ 3; 7 ÷ 3; 5 × 3 …),
and only one of them answers the question. The Ein program is the explanation of which, and it is read off
the run, not written beside it:

- **Which operations:** each is a rule of the word-problem theory firing on the problem's facts, and its
  `:why` says what in the problem called for it (*a box holds 3 and the question asks for boxes, so the
  apples are divided by 3*).
- **In what order:** the second step's request is derived from the first step's answer, so the order is the
  derivation's, visible in `--trace`, and nothing in the problem states it.
- **Why:** the trace of the answer, from the question back to the three stated numbers, with every step a
  rule and its `:why`. The page shows it for this problem in full, and shows that no rule fires for the
  computations that do not answer the question.

How such a program is synthesised from the sentence is not this stage's, as the user says: the program is
written by hand, and [M2](../../m2_nl_to_ir/README.md#the-reasoning-as-a-program-the-users-idea-2026-11-11)
takes it as an instance of the output it wants.

**The questions the stop puts to the user**, each with a recommendation computed on the prototype:

1. **How a problem is written.** A problem has states and events in an order (*has*, then *gave*, then
   *how many now*), and a fact carries no keyword of its own, so the order is written in the facts: named
   moments with a `before` relation, events as nested facts, or a chain. Each is shown on the same problem.
2. **How a gap is detected.** Three mechanisms, each tried:
   - **(a)** an obligation: the theory states that a quantity asked about is owed, so the answer is a state
     that **owes** and the read-out names what (`∅ not in R — root owes …`, the M1d mechanism);
   - **(b)** a derived fact, `(unknown-quantity …)`, by `absent` over the quantity's relation. That makes the
     word-problem module non-monotone, where the calculator stays monotone, and *forbid* may refuse a
     refutation resting on it;
   - **(c)** the hypothesis machinery: the unknown as a candidate. Numbers are not enumerable, so this is
     expected not to work, and the page says what happens.
3. **How an assumption is written and reported:** a fact with a marker the trace prints, an hrule, or a
   second program beside the first.
4. **Where the theory lives**, and what it is called.
5. **Which readings of a word the theory holds**, where a word has two (*gave* a number of things against
   *gave* all; *together* over the people named against everyone in the problem).

**T2 onward** are written after the stop, from the user's rulings: the theory, its problems as corpus
entries each with an `:expect`, a test that holds every answer to the harness's integers, and a test that
holds each gap problem to its named gap.

## Acceptance

- **The page**, with every problem's answer computed and every artefact linked, and the questions with their
  recommendations. The commit at the stop is docs only.
- After the rulings:
  - every problem is a corpus entry with an `:expect`, and `ein test` holds it;
  - **the engine is unchanged by the stage**, and a test holds that the problems answer through the imported
    calculator;
  - **the two-step problem's plan is explained by the program**: its operations, their order and the reason for
    each are read off the run, and a test holds the trace's steps;
  - **each derived answer's trace is the reasoning**: every step is a rule of the word-problem theory or of
    the calculator, each with its `:why` rendered;
  - **each gap problem names its gap**, and no problem's answer rests on a quantity the problem did not state
    without saying so;
  - whether a solved problem is search-free (H empty, R {∅}) is **reported per problem and not required**:
    the phase's principle is about arithmetic, and a gap or an assumption may be where search belongs.

## What it may find

- **A gap needs negation as failure**, which the calculator's theory avoids on principle. The stage keeps
  the two modules apart so the calculator's *one stratum* still holds of the calculator.
- **A transfer of more than one has** needs either a refusal or a negative number (S1f.6.7).
- **Engine findings.** Each is a row of the phase's [`findings.md`](findings.md), shown to the user with the
  page; a wrong answer is a register row before anything fixes it. Each row there a problem meets is named on
  the page.
