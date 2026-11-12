# M1j — `nlp/` and `smt/` as one proof of concept: Zebra sentences through link grammar to SMT

**Status:** planned 2026-11-12, on the user's note of that day. Not started. **A small milestone:** one
README, its tasks as sections, no phase directories. It carries no estimate (the user's ruling of
2026-11-08).
**Depends on:** nothing in the engine. It uses the two scratch areas, [`nlp/`](../../nlp/README.md) and
[`smt/`](../../smt/README.md), a link-grammar parser or an equivalent, a Lisp, Python, and an SMT solver.
**Blocks:** nothing.

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

- **[`nlp/`](../../nlp/README.md):** two throwaway scripts from the 2021 prototype's look at dependency
  parsing, `xxx.py` and `xxx-link.py`, and a reading list. `xxx-link.py` is the *linkgrammar script*: it
  imports the `linkgrammar` Python bindings and holds the Zebra puzzle's sentences as text (*The Englishman
  lives in the red house. The Spaniard owns the dog. …*). The `link-grammar` submodule it used was
  deinitialised at M1a S1a.10.5 and never checked out here; the directory's README has the one command that
  restores it.
- **[`smt/`](../../smt/README.md):** three hand-written encodings, `einstain-problem.smt`,
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

1. The merged folder's name and place (T1).
2. Link grammar restored, or which equivalent (T2).
3. Which Lisp (T3) and which solver (T5).
4. Whether an Ein program is also an output. The note says SMT code and nothing else, so it is **not
   planned**; it is recorded because the same sentences have an Ein encoding,
   [`examples/zebra.ein`](../../examples/zebra.ein), and the comparison is one a reader will ask for.
5. Where it sits against the other milestones. It depends on none of them.

## Connections

- [M2](../m2_nl_to_ir/README.md): natural language to a formal statement by a model in a loop; this is the
  same arrow by a parser and a hand-written converter, on one puzzle, as a baseline to look at.
- [M10](../m10_external_benchmarks/README.md): the solvers, installed and compared.
- [`docs/lib/02`](../../docs/lib/02-solvers-csp-sat-smt.md), the external-tech notes on solvers.
- [`c/README.md`](../../c/README.md): three plain-C baselines of the same puzzle, which are to the engine
  what this pipeline is to M2: the same problem by other means, wired into nothing.
