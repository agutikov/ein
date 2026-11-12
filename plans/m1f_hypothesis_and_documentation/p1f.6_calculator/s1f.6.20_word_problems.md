# S1f.6.20 — An application: elementary-school word problems, arithmetic among other relations

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day ([§ The instruction](#the-instruction)).
**T1 done 2026-11-11, and stopped for the user:** the prototype and the page, [`word_problems.md`](word_problems.md),
29 programs under `build/s1f.6.20/` on the release binary, every problem of the table answered, the three gap
mechanisms and the three forms of a missing reading each run, twelve questions with recommendations, and E-17 to
E-20 in [`findings.md`](findings.md). Nothing shipped; T2 onward wait for the rulings.
**The prototype was banked on 2026-11-12**, on the user's instruction of that day, with the variants the page
recommends: [`examples/word_problems/`](../../../examples/word_problems/README.md), three modules and thirteen
problems ([§ T2 record](#t2-record-2026-11-12-the-prototype-banked)). The remainder variant is not among them: a
test of the gate does not end on it (E-33). The twelve questions stay open, and the tests T2 lists wait for the
rulings.
Its first task **stops for the user** with example programs before any theory is written.
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

### The problem is a model, and says where it ends

The user, 2026-11-11, on the variant with a remainder (13 apples, a box holds 3):

> of course, 13 doesn't fit into 4 boxes of 3, so there are still potentially the gap, do we have to fill all
> boxes and leave the remainder or we have to pack everything and accept partial box occupation? The problem is
> intentionally ignores other real-world expansions like: every item has dimentions that may fit or not into
> boxes, boxes can cost but we still can put every item in separate box - is "need" means minimal or not?
> Because the real world is potentially infinite, while the problem and theory is a model.

So the variant stays, and it is a **gap of a second kind**. The first kind is a missing quantity (Alice's
apples before). This one is a missing **reading**: every number is stated, and the question's word, *need*,
does not say which computation answers it. The page shows, on a prototype, what the program says for it:

- **The readings the theory holds are named**, each a rule with its `:why`: *fill whole boxes and leave the
  rest* (4 boxes, 1 apple over), *pack everything and accept a box that is not full* (5 boxes), and that
  *need* is the least number of boxes and not any number that suffices (13 boxes, one apple each, also packs
  everything).
- **Where the problem does not choose between them, the answer says so**, as it does for a missing quantity:
  either both answers, each under its named reading, or no answer and the reading named as what is missing.
  Which of the two is a question for the stop. Where 12 ÷ 3 leaves nothing over the readings agree, and the
  program says that the answer does not depend on the choice.
- **What the model leaves out is the theory's to state, once, and not each problem's.** The theory counts
  things and boxes by number alone: no sizes, no cost, no shape. That is a list of assumptions the theory
  carries as text a run can print, so an answer can be read as *4 boxes, in a model where a box holds any 3
  apples*. The stage does not try to detect what the world would add: the world is not bounded and the model
  is, which is the user's point. It detects only a gap **inside** the model, a quantity or a reading the
  theory's own rules ask for and the problem does not give.

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
6. **A missing reading: both answers, or none.** Whether the two readings are written as hypotheses, so that
   the answer is a region of two states and the read-out's projection classes are the two answers, or as two
   derived answers each marked with its reading, or as a named gap and no answer. Each is tried.
   **The user leans to the first** (2026-11-11): *"yes, readings as hypotheses - it looks like search of
   ontology, not yes as search of theories, but already very valuable"*. So it is the one built first and
   shown in full; the other two are shown beside it for comparison. The user's distinction is kept: what is
   searched here is **which facts about the words hold** (an ontology: *need* is this reading or that one),
   under rules that stay fixed. A search over the **rules** themselves, theories as hypotheses, is not this
   stage's and has no stage yet; [F7](../../followups/f7_rule_induction.md) and
   [M2](../../m2_nl_to_ir/README.md) are where it would be taken up.
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

## T2 record, 2026-11-12: the prototype banked

**The instruction.** The user, 2026-11-12: *"bank word problems into examples/word_problems with recommended
variants"*. It is an instruction for the examples. **It is not a ruling on the twelve questions of
[`word_problems.md` § 11](word_problems.md#11-the-questions-for-the-user), which stay open**; a ruling that
differs from a recommendation moves the banked files. No stdlib module and no engine source moved.

**What is banked**, under [`examples/word_problems/`](../../../examples/word_problems/README.md): sixteen
`.ein` files and a README.

| the page's question | the variant banked | the files |
|---|---|---|
| 1, how a problem is written | form 1, named moments | every problem |
| 2, the direction of the theory | backwards from the question, then forwards | `words.ein` |
| 3, a missing quantity | (b), a derived fact by `absent`, in a module of its own | `gaps.ein`; `gap.ein`, `kinds.ein` |
| 4, an assumption | a marked fact, `assumed`, with `rests-on` beside the answer | `gap_assume.ein`, `kinds_assume.ein` |
| 5, a named gap with a supposition | nothing: neither combination is banked | — |
| 6, a missing reading | readings as hypotheses with nothing owed, the page's `readings_plain`; `need-exact` answers with no reading where nothing is left over | `readings.ein`; `boxes12.ein`. **Its 13-apple problems are not banked** (below) |
| 7, two kinds | *has 3 pears* does not say *no apples*: a gap | `kinds.ein`, `kinds_assume.ein`, `kinds_full.ein` |
| 8, the model's boundary | a fact with a `:source`, a premise of the rules that depend on it | `words.ein`'s `(model-counts-only)` |
| 9, where and what it is called | `examples/word_problems/`, `words`, `gaps`, `readings` | the directory |
| 10, more readings of a word | none added | — |
| 11, the trace as the pupil's page | nothing built; `boxes12.ein` declares a `--trace` cell | — |
| 12, the evidence for an expression layer | the rules as the prototype has them, two a word | `words.ein` |

The other problems of the stage's table: `bob1.ein`, `gap_fact.ein`, `contra.ein`, `pencils.ein`, `more.ein`,
`unused.ein`, `bystander.ein`.

**Every answer re-taken**, on the release binary and the stdlib of the banking commit (the prototype ran on the
stdlib of `a621dc75`, before `std.int`, `std.rat` and `std.num9`): `ein test examples/word_problems/` holds 21
claims in 13 files, none failed and none not checked, and each read-out was set beside the page's (`ein solve`
on each file, 2026-11-12).

| file | the answer | universe | k_s |
|---|---|---|---:|
| `bob1` | 3 | no candidate | 1 |
| `gap` | Bob's 3 and 5 known; *together* no row; `(unanswered (together Bob Alice apples t1) (amount Alice apples t0))` | no candidate | 1 |
| `gap_fact` | 3, 6 and 9 | no candidate | 1 |
| `gap_assume` | 5, and `(rests-on (together Bob Alice apples t1) (amount Alice apples t0) z)` | no candidate | 1 |
| `contra` | root dead, R empty; the core: the two sentences, the question, `(model-counts-only)`, `(fs 7 5 0 2 0)` and `(nz 2)`; refuted by `gave-more-than-had-is-false` | no candidate | 0 |
| `kinds` | Bob's 5 known; no answer; Alice's apples not stated | no candidate | 1 |
| `kinds_assume` | 5, resting on the assumption | no candidate | 1 |
| `kinds_full` | 8 | no candidate | 1 |
| `pencils` | 47 each, 21 left over; 1728 and 1478 on the way | no candidate | 1 |
| `more` | Alice has 7 more | no candidate | 1 |
| `boxes12` | 4; no reading proposed | no candidate | 1 |
| `unused` | 3; the values known are Bob's two | no candidate | 1 |
| `bystander` | 9 | no candidate | 1 |

**No answer differs from the page's.** Root's firings are the page's too: 20 for `bob1`, 140 for `boxes12` (the
page's `boxes12_plain`), 298 for `pencils`, 21 for `contra` (`ein saturate` on each, 2026-11-12).

**The two programs left out, and the test that did not end.** The page's `boxes13_plain` and `boxes13_stated`
answer on the banking commit's binary as § 4 reports: three states, *undetermined here*, 4 and 5, with
`reading-matters` in each, 2 candidates and 4 saturations; and one state, 5 boxes under `pack-all`, 1 candidate
dead alone (`ein solve` on each, with the banked `readings.ein`, 2026-11-12; the second's `lattice` row reads
*monotone*, where the page's file imported the owed `readings` and was flagged for its obligation). They were
banked first, and the gate's workspace run stayed inside `ein-infer/tests/region.rs`'s
`fact_grain_never_calls_a_violated_cube_monotone` for 13 263 s on one thread until it was stopped. The test runs
a prototype static analysis (`ein-infer/examples/shared/fact_grain.rs`, read by no route) over every corpus
file. It closes the candidates under the rules over shapes with unbound arguments as *any*: a supposed reading
concludes a `value` of any quantity, the word rules read a `value` of any quantity and conclude a request to the
calculator with any operands, and the closure then follows every shape of every circuit. The stack was in
`Shape::subsumes` under `classify`. What was tried:

| the corpus | the test |
|---|---|
| without the two programs | passes, 20 s |
| with them | not ended in 300 s, and not in 13 263 s |
| with them, and the request for the fifth box made at root whatever the reading | not ended in 300 s: the `value` under a reading is the cause, not the request under it |
| with them, the reading rules concluding `(answer-under Q N OVER)` in place of a `value` | passes, 20 s |

(`cargo test -p ein-infer --test region fact_grain_never`, 2026-11-12.) The corpus has no way to keep a file out
of that sweep, which walks the files. The test's meaning and the engine were not touched; the two programs are
not banked; the engine does not loop and gives no wrong answer. It is **E-33** of [`findings.md`](findings.md),
and what to do is [Q-M1f.198](../open_questions.md#q-m1f198--the-remainder-variant-of-the-word-problems-and-the-gate),
defaulted to *not banked* and the user's. `readings.ein` stays, since `boxes12.ein` imports it and it proposes
nothing there; the directory's README shows the 13-apple program for a reader to run.

**What moved in the files**, beside header comments: `readings.ein` is `readings_plain.ein` under the page's
proposed name, and its import of `words` lost `wanted`, which it read nowhere (below); `boxes12.ein` is two of
the page's files in one ([Q-M1f.195](../open_questions.md#q-m1f195--one-file-for-the-two-step-problem)). No rule,
relation or claim of the prototype was otherwise edited.

**A note on a row that exists**, [`findings.md`](findings.md): E-11 with a relation in place of a rule.
`readings.ein` as first banked listed `wanted` in its import and read it nowhere; the gate's first run failed in
`answer_round_trip.rs`, *examples/word_problems/readings.ein: models/01.ein: 2 facts lost*, the declaration the
re-written program dropped (`build/gate-s1f.6.20-first.log`). The module imports what it reads. `gaps.ein` alone
meets E-1, `(asked __closed__)`. **No wrong answer of the engine was found** and no row of
`corpus/wrong_answers.toml` was filed: 305 fixed, 16 open and 1 not wrong (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-12).

**The decisions taken by default**, each the user's to revise:
[Q-M1f.194](../open_questions.md#q-m1f194--which-of-the-word-problem-prototypes-programs-are-banked-and-under-what-names),
which programs and under what names;
[Q-M1f.195](../open_questions.md#q-m1f195--one-file-for-the-two-step-problem), one file for the two-step
problem; [Q-M1f.196](../open_questions.md#q-m1f196--the-word-problem-modules-as-corpus-entries), the modules as
corpus entries; [Q-M1f.197](../open_questions.md#q-m1f197--what-holds-the-banked-word-problems), the corpus
holds them and no test of their own is written before the rulings;
[Q-M1f.198](../open_questions.md#q-m1f198--the-remainder-variant-of-the-word-problems-and-the-gate), the
remainder variant not banked.

**The corpus:** sixteen entries, none `slow`. Thirteen problems under `solve`, `test` and `saturate`,
`boxes12.ein` under `solve --trace` too; `words.ein` and `readings.ein` under `solve`, `saturate` and `render
rules`; `gaps.ein` under `saturate` and `render rules`.

**The goldens that move,** named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt`, 48 new cells,
the sixteen new files'; `ein-render/tests/golden/corpus_shapes.md5`, 672 new lines, their 42 renderings each;
and no other line of either (`git diff` on each, every added line naming `examples/word_problems/`, none
removed). One test's list was edited by hand, as S1f.6.10's was: `ein-infer/tests/alive_invariant.rs`'s `SEEDED`
gains thirteen of the files, the `z` inside a nested number they write or the calculator's own, and for
`readings.ein` and `boxes12.ein` the hrule's name its closing rule concludes. No engine source moved.

**Pages:** `examples/word_problems/README.md`, new; `examples/README.md`, a row for the directory;
[`word_problems.md`](word_problems.md), *Where it is banked* and the table of names; `findings.md`;
`open_questions.md`; the phase README's row; `corpus/corpus.toml`; `AGENTS.md`. `questions.md`, which holds the
user's replies in progress, was not touched: its section F says the problems were banked, and Q-M1f.194 to .198
and E-33 are not yet in its sections I and J.

