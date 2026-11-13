# M1j — `nlp/` and `smt/` as one proof of concept: Zebra sentences through link grammar to SMT

**Status:** **done 2026-11-13**, built on branch `m1j` and merged into master the same day as `3d941d81`
([§ The merge](#the-merge)); planned 2026-11-12, on the user's note of that day. **A
small milestone:** one README, its tasks as sections, no phase directories. It carries no estimate (the
user's ruling of 2026-11-08).
**Depends on:** nothing in the engine. It uses the two scratch areas, [`nlp/`](../../nl2smt/README.md) and
[`smt/`](../../nl2smt/README.md), a link-grammar parser or an equivalent, a Lisp, Python, and an SMT solver.
**Blocks:** nothing.

**How to read this page.** Down to § Connections it is the plan as written on 2026-11-12, and `nlp/` and
`smt/` in it are the two directories as they were then: both are [`nl2smt/`](../../nl2smt/README.md) since
T1. § Open questions carries each answer and its source, and [§ Record](#record-2026-11-13) is what was
built and run. The folder's own page, [`nl2smt/README.md`](../../nl2smt/README.md), is where the pipeline
is described for a reader who wants to use it.

## The instruction

The user, 2026-11-12, in
[`questions.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md) of the calculator's
phase, in full:

> TODO: small milestone M1j about nlp/ and smt/ - put both into one folder, reincarnate linkgrammar script or
> replace with equivalent, write small not necessarily universal program in lisp that converts linkgrammar
> output into smt code, one direction
> combine all together with python wrapper, allow only limited set of words in NL required only for this
> zebra puzzle. Not a solution, just PoC, don't make it degenerate, not just output hardcoded text, but make
> it simple. The requirement is that I add/remove input NL lines, change order, swap things in NL (e.g. swap
> Norwegian and Ukrainian) - it can change the solution or break it.

Everything below is drawn from that note. Where the note does not say, the page says *not said* and asks.

## What exists

- **[`nlp/`](../../nl2smt/README.md):** two throwaway scripts from the 2021 prototype's look at dependency
  parsing, `xxx.py` and `xxx-link.py`, and a reading list. `xxx-link.py` is the *linkgrammar script*: it
  imports the `linkgrammar` Python bindings and holds the Zebra puzzle's sentences as text (*The Englishman
  lives in the red house. The Spaniard owns the dog. …*). The `link-grammar` submodule it used was
  deinitialised at M1a S1a.10.5 and never checked out here; the directory's README has the one command that
  restores it.
- **[`smt/`](../../nl2smt/README.md):** three hand-written encodings, `einstain-problem.smt`,
  `einstain-problem-minus-15.smt` and `4-queens.smt`. The Zebra one is the **target's shape**: integer
  constants such as `house-red` and `nation-england`, the helper functions `order` and `near`, and an
  assertion a clue. Its `CVC4` submodule was deinitialised at the same stage.
- **Two plans already name these directories,** and this milestone is neither:
  - [M2's S2.6.4](../m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md) starts from `nlp/`'s
    two scripts for a *measured* A/B, whether link-grammar output in the prompt improves an LLM's
    formalization; it re-adds the submodule when it runs.
  - [M10](../m10_external_benchmarks/README.md) counts `smt/`'s three files among its benchmark corpus's
    encodings ([S10.1](../m10_external_benchmarks/s10.1_problem_corpus.md)).
- **M3, SMT integration, was dropped on 2026-08-18** as an idea and not only a schedule: Ein has no solver
  back-end ([`plans/README.md`](../README.md)). This milestone does not revive it. Nothing in the note
  connects the pipeline to the Ein engine: its output is SMT code, and it is run by an SMT solver.

## What it is

A pipeline, one direction, small on purpose:

```
NL lines (a limited vocabulary)  →  link grammar  →  a Lisp program  →  SMT code  →  a solver  →  the solution
                 └────────────────────── a Python wrapper runs all of it ──────────────────────┘
```

*"Not a solution, just PoC"*: it shows that the Zebra puzzle's sentences can be carried by a parser and a
converter to a solver's input, on this puzzle's words alone. *"don't make it degenerate, not just output
hardcoded text, but make it simple"*: the SMT code is a function of the sentences given, and nothing more
general than this puzzle needs is built.

## The tasks

### T1 — One folder for `nlp/` and `smt/`

*"put both into one folder"*. The two directories become one. **Not said:** its name and where it sits. To
decide with the user before anything moves; and what moves with it: the two READMEs' reading lists, the
three `.smt` files, the two scripts. Every link to the old paths is restated in the same commit:
`AGENTS.md`'s paragraph on `nlp/` and `smt/`, `plans/README.md`'s note on M3, M2's S2.6.4, M10's S10.1 and
S10.2, and the two submodule notes. M10 reads the `.smt` files as benchmark encodings, so they keep their
content byte for byte.

### T2 — The link-grammar script revived, or an equivalent

*"reincarnate linkgrammar script or replace with equivalent"*. `xxx-link.py` was written for Python 3.6 and
bindings that are not installed here. The task first finds out which is less work on this host: restoring
link grammar (the system's package, or the submodule) and making the script run, or an equivalent parser
that gives a comparable structure for these sentences. **Not said:** what counts as equivalent. The
criterion taken from the rest of the note: its output is something T3's converter can read, and it is a
real parse of the sentence, not a table of the puzzle's sentences. The choice and the reason are recorded;
if link grammar is restored, M2's S2.6.4 is told, since it plans to re-add the same submodule.

### T3 — A Lisp program from link-grammar output to SMT code

*"write small not necessarily universal program in lisp that converts linkgrammar output into smt code, one
direction"*. It reads the parser's output for one sentence and emits the assertion that sentence states, in
the shape `smt/einstain-problem.smt` has by hand; the declarations and the helper functions come from the
vocabulary. *"not necessarily universal"*: it knows this puzzle's sentence shapes (*X lives in the Y house*,
*the X owns the Y*, *next to*, *immediately to the right of*, *the middle house*, *the first house*) and no
others. *"one direction"*: nothing goes from SMT back to language. **Not said:** which Lisp; chosen by
what is on the host, and recorded.

### T4 — A limited vocabulary

*"allow only limited set of words in NL required only for this zebra puzzle"*. The word list is written
down: the five nationalities, colours, pets, drinks and cigarette brands, and the few verbs and relations
the clues use. A sentence with a word outside it is refused by name, which is the pipeline's one error
besides a sentence the parser or the converter cannot read.

### T5 — A Python wrapper that runs all of it

*"combine all together with python wrapper"*. One command: NL lines in (a file, a line a sentence), the
parser, the converter, the solver, and the result out. **Not said:** which solver. `smt/`'s files were
written for CVC4 1.8; M10 notes that the installed `cvc4` and the CVC5 it benchmarks are different
programs. The wrapper takes whichever solver the host has that reads the emitted code, and names it in its
output.

### T6 — The acceptance, as the user states it

> The requirement is that I add/remove input NL lines, change order, swap things in NL (e.g. swap Norwegian
> and Ukrainian) - it can change the solution or break it.

So the proof that it is not degenerate is behavioural, and each is a test with its input file:

- **Add or remove a line.** Removing a clue changes the answer: the puzzle with one clue fewer has more
  solutions, or another. `smt/einstain-problem-minus-15.smt` is the hand-written twin of one such removal,
  and the pipeline's output for the same sentences is compared with it.
- **Change the order of the lines.** The emitted assertions change order with them; the solution does not,
  and the test says so. (*"it can change the solution or break it"* is read as covering all three edits
  together: reordering clues is the one that should leave the answer as it was.)
- **Swap things,** *"e.g. swap Norwegian and Ukrainian"*: the solution changes accordingly, or the puzzle
  becomes unsatisfiable, and the solver says which.
- **No hardcoded output.** A check that the SMT text differs for each of the edits above, and that a
  sentence built from the vocabulary which is in no version of the puzzle is still converted.

## What it must not do

- **Not grow.** *"Not a solution, just PoC … make it simple."* No general English, no second puzzle, no
  second direction.
- **Not become M3.** Nothing here links a solver into Ein.
- **Not pre-empt M2's S2.6.4,** whose question is measured and about an LLM's formalization; this
  milestone's parser choice informs it and decides nothing for it.
- **Not join the gate unasked.** The directories are wired into nothing today; whether the pipeline's tests
  run per commit depends on a parser and a solver being installed where the gate runs, and that is the
  user's call.

## Open questions

All five are answered. A default is the user's to revise.

1. The merged folder's name and place (T1). **`nl2smt/`, at the repository's root**: the user's answer,
   2026-11-13.
2. Link grammar restored, or which equivalent (T2). **Restored, not replaced**, and the user confirmed it on
   2026-11-13 (*"ok, continue with link-grammar"*). The host has `link-parser` 5.13.0, the system package,
   with no Python bindings for the installed Python 3.14, so the wrapper runs the command line as a
   subprocess and no submodule is re-added. The same day the user asked whether the parser emits only one
   tree. It does not: it gives every linkage it finds, cheapest first. The converter therefore walks a
   sentence's linkages in that order and takes the first whose shape it knows.
3. Which Lisp (T3) and which solver (T5). **Racket 9.3**: the user's choice, 2026-11-13. The host's only
   Lisps had been GNU Guile and Emacs Lisp, and the user installed Racket after being told so. **The solver
   is whichever is installed**, the first of `cvc5`, `z3` and `cvc4`, named in the output, with `--solver`
   to choose: by what is on the host, where all three are and all three were run. The order of the three is
   a default.
4. Whether an Ein program is also an output. The note says SMT code and nothing else, so it is **not
   planned**, and none was written; it is recorded because the same sentences have an Ein encoding,
   [`examples/zebra.ein`](../../examples/zebra.ein), and the comparison is one a reader will ask for.
5. Where it sits against the other milestones. It depends on none of them, and **it ran on a branch of its
   own, `m1j`, beside M1f**: the user's instruction of 2026-11-13.

## Connections

- [M2](../m2_nl_to_ir/README.md): natural language to a formal statement by a model in a loop; this is the
  same arrow by a parser and a hand-written converter, on one puzzle, as a baseline to look at.
- [M10](../m10_external_benchmarks/README.md): the solvers, installed and compared.
- [`docs/lib/02`](../../docs/lib/02-solvers-csp-sat-smt.md), the external-tech notes on solvers.
- [`c/README.md`](../../c/README.md): three plain-C baselines of the same puzzle, which are to the engine
  what this pipeline is to M2: the same problem by other means, wired into nothing.

## Record (2026-11-13)

Built on branch `m1j`, three commits and this one, a task or a group of tasks each. Everything is under
[`nl2smt/`](../../nl2smt/README.md); nothing under `ein.rs/`, `stdlib/`, `examples/`, `tests/` or `corpus/`
moved, and no gate file. The tools, as run: `link-parser` 5.13.0 with its English dictionary 5.12.4,
Racket 9.3 [cs], `cvc5` 1.3.0, `z3` 4.16.0, `cvc4` 1.8, Python 3.14.7.

```sh
python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra.txt     # the pipeline
python3 nl2smt/test_pipeline.py                        # its tests
```

### T1 — one folder

`nlp/` and `smt/` are `nl2smt/`. The three hand-written encodings moved to `nl2smt/smt/` with `git mv`,
each a 100 % rename with the blob id it had, so M10 reads what it read. The two 2021 scripts moved to
`nl2smt/prototype/`: both are kept, since [M2's
S2.6.4](../m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md) names them as its starting point.
The two READMEs are one, with both submodule notes and both reading lists.

Restated in the same commit: `AGENTS.md`'s paragraph, the root `README.md`'s layout row and History
sentence, `plans/README.md`'s note on M3, M2's S2.6.4, M10's README and S10.1, and this page's links.
**Left as written**, each a record of its time and none a link: M10 README's and S10.2's `smt/CVC4`, the
path the deinitialised submodule had; `docs/history/m1a_rust/README.md`'s quoted acceptance line; and
P1f.6's `questions.md`, `questions_round2.md` and `replies_2026-11-12.md`.

### T2 — link grammar, by its command line

```sh
link-parser en -postscript=1 -graphics=0 -echo=1
```

with each sentence followed by 99 empty lines on stdin, since the parser prints a sentence's cheapest
linkage and the next one for every empty line. The postscript form is what the converter reads: the words
by position, and each link as *left right height (label)*. The constituent tree, which the parser also
offers, is not used: the links say what is linked to what, by type, and the tree does not.

The seventeen sentences have between 2 and 129 linkages. Two of them, the *is drunk* clues, are read off
linkage 2, the passive, because linkage 1 reads *drunk* as an adjective; the other fifteen off linkage 1.
The table of ranks and the two commands that re-take it are in
[`nl2smt/README.md`](../../nl2smt/README.md#the-parser-gives-several-linkages-and-the-first-that-is-read-is-taken).
**No sentence of the puzzle has two linkages that convert to different assertions**: every linkage of a
sentence that is read is read alike, which test 6 holds. About sixty further sentences over the vocabulary
were tried by hand that day while the converter was written, and none showed two readings either; those
are not banked.

M2's S2.6.4 is told, in one sentence where it speaks of adding the submodule, that the system package's
command line was enough here. Nothing is decided for it.

### T3 — the converter, in Racket

[`nl2smt/lg2smt.rkt`](../../nl2smt/lg2smt.rkt), plain `#lang racket`, no package. It reads the linkages on
stdin and writes the SMT-LIB script, and under `--readings` it prints what each linkage of each sentence is
read as, or why it is not read.

A noun, or a preposition with its noun, is read as a house: `(at x)`, the house of a constant or a number;
`(near x)`; `(right x)`. A sentence puts its subject's house and what is said of it together into one
term: `(= x y)`, `(near x y)`, `(order y x)`. The shapes are the plan's and compose from a dozen link
types: the subject and its verb (`W`, `S`), an object (`O`), a place (`MV`, `J`), the passive (`Pv`), a
noun named by the value beside it (`A`, `AN`), the first house (`L`), *next to*, *to the right of* (`Pp`,
`M`, and `EB` for *immediately*), *the man who* (`B`), *the man with* and *the house next to* (`M`), *the
house where* (`C`), *There are N houses* (`PF`, `SI`, `D`) and *Who* (`W`).

The framing comes from the vocabulary: a constant for every value, each in `1..houses`, each kind's
`distinct`. *There are five houses.* is a real input line: it becomes `(define-fun houses () Int 5)`, and
with *six* in its place the puzzle has more solutions.

**Where it is less driven by the parse than the acceptance asks**, said plainly:

- A name of several words, *Old Gold*, *Lucky Strike*, *orange juice*, is joined by position from the
  vocabulary before any link is read. The parser has no complete linkage of a sentence with *Old Gold*
  inside it and leaves *Old* linked to nothing.
- The words of a shape are matched as words (*in*, *with*, *to*, *right*, *where*, *is*, *are*, *there*,
  *houses*), and *next to* is *next* before *to* with a link between them, whatever the link's label.
- The header, the two functions `order` and `near`, `check-sat` and the model's `get-value` are fixed
  text.

### T4 — the vocabulary

[`nl2smt/vocabulary.rktd`](../../nl2smt/vocabulary.rktd), forms `(kind tag word ...)`, the one file both
programs read. The wrapper takes the words from it and refuses any other by name and line. The converter
takes the kinds and tags: which constant a word is, what a verb is said of, what a noun is named by, what
*who* asks for.

```
$ python3 nl2smt/nl2smt.py nl2smt/puzzles/refused-word.txt
nl2smt: line 2: the word "cat" is not in the vocabulary (vocabulary.rktd)
$ python3 nl2smt/nl2smt.py nl2smt/puzzles/refused-parse.txt
nl2smt: line 2: cannot read "The dog the the lives.": the parser links "the" to nothing (its cheapest linkage; none of the 7 is read)
$ python3 nl2smt/nl2smt.py nl2smt/puzzles/refused-shape.txt
nl2smt: line 2: cannot read "The Spaniard drinks the dog.": "drinks" wants a drink, which "dog" is not (its cheapest linkage; none of the 2 is read)
```

### T5 — the wrapper

[`nl2smt/nl2smt.py`](../../nl2smt/nl2smt.py), standard library only. It reads the lines, checks the words,
runs `link-parser`, hands its output to `lg2smt.rkt` and that program's script to a solver, and prints the
model as a table and each question's answer. It reads no English and writes no SMT. A solution is counted
by passing the model the solver printed back to the converter, which writes the script again with that
model ruled out; one solution and then `unsat` is *1 solution, and no other*.

```
$ python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra.txt
solver: cvc5 (This is cvc5 version 1.3.0 [git tag 1.3.0 branch HEAD])
1 solution, and no other

  house   1          2              3           4             5
  nation  Norwegian  Ukrainian      Englishman  Spaniard      Japanese
  colour  yellow     blue           red         ivory         green
  pet     fox        horse          snails      dog           zebra
  drink   water      tea            milk        orange juice  coffee
  smoke   Kools      Chesterfields  Old Gold    Lucky Strike  Parliaments
  Who drinks water?    Norwegian
  Who owns the zebra?  Japanese
```

`--solver z3` and `--solver cvc4` print the same table under their own names.

### T6 — the acceptance

[`nl2smt/test_pipeline.py`](../../nl2smt/test_pipeline.py), nine tests, each on a file under
`nl2smt/puzzles/`:

```
$ python3 nl2smt/test_pipeline.py
...
test_7_every_solver_installed_gives_the_same_solution ... solvers run: cvc5, z3, cvc4 ... ok

----------------------------------------------------------------------
Ran 9 tests in 38.830s

OK
```

- **A line removed.** `zebra-minus-15.txt` is the puzzle without *The Norwegian lives next to the blue
  house.* The default run says *more than 2 solutions*, and `--solutions 40` says *32 solutions, and no
  other*. Against the hand-written `einstain-problem-minus-15.smt` the test makes two comparisons and says
  which is which: the clue assertions are the same set once the constants are renamed by a table in the
  test and the sides of `=` and `near` are unordered; and the 32 solutions are the same 32.
- **The order changed.** `zebra-reversed.txt` and `zebra-shuffled.txt`: the assertions come in the lines'
  order, each sentence's assertion what it was, and the solution is the same.
- **A swap.** `zebra-swapped.txt` has *Norwegian* and *Ukrainian* exchanged. The puzzle does not break: it
  has one solution still, the classic one with the two exchanged.

  ```
    house   1          2              3           4             5
    nation  Ukrainian  Norwegian      Englishman  Spaniard      Japanese
    ...
    Who drinks water?    Ukrainian
    Who owns the zebra?  Japanese
  ```

  The hand-written file with the two constants exchanged gives the same model, which the test checks.
- **A line added.** `zebra-contradiction.txt` is the puzzle with *The Norwegian drinks milk.*:
  *no solution: the sentences cannot all be true*.
- **No fixed text.** The seven inputs' scripts are pairwise different. `new-sentences.txt`, six statements
  and a question that are in no version of the puzzle, becomes `(= nation-Japanese pet-zebra)`,
  `(= drink-tea colour-blue)`, `(near nation-Spaniard colour-red)`, `(= drink-water 1)`,
  `(< colour-yellow colour-ivory)`, `(= smoke-Kools pet-dog)` and a `get-value` over
  `(= nation-X colour-green)`.

That the tests bite was checked once by hand: with `order`'s two arguments exchanged in the converter,
five of the nine tests fail (`failures=7`, test 7 once for each solver).

### Decisions taken by default

Each is the user's to revise.

- **The layout inside the folder**: `smt/` for the hand-written encodings, `prototype/` for the 2021
  scripts, `puzzles/` for the inputs.
- **The converter reads links, not the constituent tree**, and reads the passive only by its `Pv` link.
  If it also took the adjective reading, the two *is drunk* clues would be read off linkage 1, to the same
  assertions.
- **A verb is held to its kind**, so *The Spaniard drinks the dog.* is refused where it could have been
  read as the two being in one house.
- **Every value of the vocabulary is declared**, whether or not a sentence names it.
- **`four` and `six` are in the vocabulary**, the two words that are not the puzzle's, so that the count
  can be changed. With no line that counts the houses there are as many as a kind has values. An even
  count and *the middle house* is refused.
- ***To the right of* without *immediately*** is `<`, where with it it is `order`.
- **The solver order** `cvc5`, `z3`, `cvc4`, and **`--solutions`' default of 2**.
- **Exit codes**: an answer exits 0, *no solution* included, and a refused sentence exits 1.
- **No clue is reworded.** `puzzles/zebra.txt` is `xxx-link.py`'s list as it stood.

### Left out, and why

- **The gate.** The plan's own rule: whether these tests run per commit is the user's call, since they
  need a parser, Racket and a solver where the gate runs. `run_tests.sh`, the workflows and
  `gate_steps.rs` are untouched.
- **An Ein program as a second output** (open question 4).
- **The submodules.** Neither is re-added; each one command is in `nl2smt/README.md`.
- **Anything sent upstream.** The parser's unlinked *Old* and the unset heights in its postscript output
  are worked around here and noted on the folder's page.

### The merge

The branch was merged into master from the root as `3d941d81` (2026-11-13), a merge commit over the four
above, and the branch and its worktree were removed.

**Checked on the branch before the merge**, by the coordinator, on the tools above (2026-11-13):
`python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra.txt`, one solution and no other, the two answers right;
`python3 nl2smt/test_pipeline.py`, 9 tests OK, cvc5, z3 and cvc4 each run; and inputs the branch does not
bank, each read from its parse:

| the edit | what the pipeline says |
|---|---|
| *red* and *green* exchanged through the text | one solution and no other, the two colours exchanged |
| the clue *The Norwegian lives in the first house.* dropped | more than 20 solutions (`--solutions 20`) |
| *The Japanese lives in the house next to the man who drinks tea.* | `(assert (near nation-Japanese drink-tea))` |
| *The man who owns the zebra smokes Kools.* | `(assert (= pet-zebra smoke-Kools))` |
| *The blue house is immediately to the right of the house where the fox is kept.* | `(assert (order pet-fox colour-blue))` |
| *There are four houses.* with *Milk is drunk in the middle house.* | refused: *line 2: 4 houses have no middle one* |
| *The dog owns the Spaniard.* | refused: *"owns" wants a pet, which "Spaniard" is not* |
| the two questions replaced by *Who lives in the green house?* and *Who smokes Kools?* | *Japanese* and *Norwegian* |

**The gate on the merged tree**: `./run_tests.sh` on `3d941d81` in a detached worktree, exit 0
(2026-11-13, `build/m1j/gate-merge.log`), its link check over 508 pages with no finding. The pipeline's own
tests are not among its steps, as § Left out says.
