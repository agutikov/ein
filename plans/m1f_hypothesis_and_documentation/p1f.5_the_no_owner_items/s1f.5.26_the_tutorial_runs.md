# S1f.5.26 — The tutorial's programs run on today's engine

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** **done 2026-11-10** ([its records](#t1-record-2026-11-10)), specified the same day on the user's ruling on
the close review's finding 6 ([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)).
T1 read the guide's sixteen `lisp` blocks and found three refused, one answering otherwise than its page says and one
misquoting its source, and four sentences of prose beside them false; T2 fixed each in the page, and
`ein-cli/tests/guide_programs.rs` reads every block, held both ways. No engine code, answer or register row moved.
[Q-M1f.69](../open_questions.md#q-m1f69--what-the-guides-pin-reads-in-a-program-block) and
[Q-M1f.70](../open_questions.md#q-m1f70--what-the-stage-does-with-what-t1-found-false-beside-the-blocks) defaulted.
**Depends on:** nothing. It edits only `docs/guide/` and a test, so it may run first.

## The ruling

The close review's finding 6, as written on 2026-11-10:

> **The tutorial is broken.** `docs/guide/04_solving_the_whole_puzzle.md` line 13 writes the retired `(config
> :enable-pre-branch-lookahead true)`, so a reader who copies it gets a refusal.

The user, 2026-11-10: *"fix the tutorial, new stage"*.

## The case

[`docs/guide/04_solving_the_whole_puzzle.md`](../../../docs/guide/04_solving_the_whole_puzzle.md) shows what
`zebra2.ein` is made of in a `lisp` block whose first line is `(config :enable-pre-branch-lookahead true) ; an
engine knob`. That flag was retired at M1e2 S1e2.1c.5, on 2026-11-03
([`configuration.md`](../../../docs/kernel/configuration.md) lists it and the stage that retired it; this page said
S1e2.1a.5 until T1 read the refusal's note), and `zebra2.ein` no longer writes it. A reader who copies the line gets
the retired-flag refusal.

What holds the guide today is [`guide_transcripts.rs`](../../../ein.rs/crates/ein-cli/tests/guide_transcripts.rs):
it re-runs each *marked* transcript (`<!-- transcript: … -->`) and diffs its bytes. Nothing reads the guide's
*program* blocks, which is how a retired flag survived (the line was written on 2026-06-17, `b4d51580`, while the
flag was live, and was refused from 2026-11-03 until this stage). S1f.5.14 wrote `:why`s into the guide's
blocks by hand (`9c06414f`), and nothing checks those either.

## What it changes

- **The block.** Chapter 4's `(config …)` line goes, with the prose that explains it. The block shows `zebra2.ein`
  as it is.
- **Every program block in the guide, read.** Each `lisp` block in `docs/guide/` is either a whole program or an
  excerpt (it writes `…`). Every whole program is loaded and answered. In an excerpt, every complete top-level
  form is parsed and loaded. Whatever a block writes that today's engine refuses or answers differently from the
  prose is fixed in the page, never in the engine.
- **Pinned.** Default: a test beside `guide_transcripts.rs` that does the reading above on every `lisp` block of
  `docs/guide/`, so a retired flag, a keyword or a missing `:why` fails `cargo test`. It is held both ways: a
  planted retired flag in a scratch copy fails it. The precedent is S1f.5.14's doc-block migration, and
  [Q-M1e.4](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)'s
  rule that a page is pinned by something that runs. Recorded as a Q-M1f row, taken by default.

## Tasks

### Task T1f.5.26.1 — every block, read

List every `lisp` block of `docs/guide/`, classify it, and run it. Record what each does today.

### T1 record, 2026-11-10

Each block was taken out of its page to a file of its own and run on the release binary, whose code is `52e759e7`'s:
a whole program loaded and solved (`ein solve FILE`), an excerpt's whole forms loaded together where they stand, as
T2's test does ([Q-M1f.69](../open_questions.md#q-m1f69--what-the-guides-pin-reads-in-a-program-block)). A block is
an **excerpt** where it writes `…` outside a string, and a top-level form with a `…` in it is elided. There are
**sixteen** blocks, thirteen whole programs and three excerpts (2026-11-10); a line below is the page's, at the
opening fence, as it stood at `52e759e7`.

| block | kind, whole forms / elided | what it did | the file it quotes |
|---|---|---|---|
| `01_objects_and_relations.md:24` | whole, 2 / 0 | loads, answered: R is one state, no candidate | — |
| `01_objects_and_relations.md:47` | whole, 1 / 0 | the same | — |
| `01_objects_and_relations.md:75` | whole, 1 / 0 | the same (`nation-loc` undeclared, which loads) | — |
| `01_objects_and_relations.md:102` | whole, 1 / 0 | the same | — |
| `01_objects_and_relations.md:123` | whole, 4 / 0 | answered: R is 32 states over 5 candidates, the blind enumerator's pairs of the program's names, `(nation-loc House-1 Englishman)` among them, since the kernel has no type system ([`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)); the page claims no answer | its four forms are `zebra2.ein`'s |
| `02_first_rules.md:11` | excerpt, 0 / 1 | the template `(rule NAME (params…) …)`: nothing to load | — |
| `02_first_rules.md:28` | whole, 5 / 0 | answered: one state, the goal holds, as the page's run of the file says | `saturation/symmetric/neighbours.ein`, form for form |
| `02_first_rules.md:85` | whole, 5 / 0 | **answered otherwise than the page says**: one state, where the page's run of the file it quotes says four. It leaves out the file's `(relation is-a T T …)`, so no `is-a` fact is a candidate | `saturation/transitive/taxonomy.ein`, without its `relation` line and its two facts' `:source` |
| `02_first_rules.md:140` | excerpt, 1 / 0 | its one form, `(rule co-located (?rel) …)`, loads; **the file it names has no such rule**: there `co-located` is a relation that `(symmetric co-located)` and `(transitive co-located)` make an equivalence | `branching/05_mini_zebra.ein`, misquoted |
| `03_rule_families.md:15` | whole, 2 / 0 | answered, one state | `zebra2.ein`'s two imports |
| `03_rule_families.md:22` | whole, 7 / 0 | answered, one state | `zebra2.ein`'s activators |
| `03_rule_families.md:87` | whole, 1 / 0 | answered, one state | `zebra2.ein`'s `co-located-negative` |
| `03_rule_families.md:110` | whole, 1 / 0 | answered, one state | `zebra2.ein`'s `co-located` |
| `03_rule_families.md:135` | whole, 1 / 0 | **refused at load**: *rule 'adjacent-via-fwd' has no :why "…"*, since S1f.5.14's flip (`4b231abd`) | `zebra2.ein`'s, without its `:why` |
| `03_rule_families.md:165` | whole, 1 / 0 | **refused at load**: *rule 'disjunctive-prune-fwd' has no :why "…"* | `zebra2.ein`'s, without its `:why` |
| `04_solving_the_whole_puzzle.md:12` | excerpt, 13 / 4 | **refused at the parse**, twice: line 13's `(config :enable-pre-branch-lookahead true)`, *config flag :enable-pre-branch-lookahead was retired at M1e2 S1e2.1c.5*, and line 20's `… (drink-loc, smoke-loc, pet-loc, right-of, next-to)`, prose in parentheses that reads as a form | `zebra2.ein`: every other whole form is the file's; the `(hrule guess …)` and `(solve :hrules …)` the file is made of are not shown, and the `std.bijection` import is elided where chapter 3 writes it whole |

Chapter 3's two refusals are S1f.5.14's: its migration wrote the guide's missing `:why`s by hand (`9c06414f`) and
missed these two, which nothing loaded, so the flip refused them. The three `zebra2.ein` blocks of chapter 3 that
load quote the file form for form, whitespace aside (each form, comments stripped and whitespace collapsed, found in
the file the same way, 2026-11-10).

**The prose beside the blocks**, read as the blocks were. Four sentences are false today:

- chapter 1: *"`is-a` (used above) is a built-in two-slot relation"* (`b4d51580`, 2026-06-17). `is-a` is an ordinary
  relation since S1.7.23 ([`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)), which
  is why chapter 2's transitive block, lacking the declaration, answers one state;
- chapter 4: a bare `ein solve examples/zebra2.ein` *"stops at its default budget before it has shown there is no
  other, and says so on stderr"* (`fbcae3b2`, 2026-10-18). Today it prints `exhausted yes — every commitment is
  classified: this is all of R`, one state, and nothing on stderr, exit 0 (2026-11-10);
- chapter 4: `-s` gives *"the wall clock per phase"*; it prints one `wall` row, *"print the wall clock under the
  read-out"* (`ein solve --help`);
- chapter 4: `--trace` *"emits the step-by-step "it follows that…" narrative"*; the trace writes each step's rule,
  premises and what it derives, and never that phrase (`ein solve examples/zebra2.ein --trace FILE`, 2026-11-10).

**The `sh` commands**, run, none pinned: chapter 2's three runs print the lines its excerpts show
(`neighbours.ein` and `taxonomy.ein` byte for byte; `05_mini_zebra.ein --maximal` binds `?n = Bob · ?p = Dog`, and a
bare run lists five states, four with *the goal has no row*); chapter 4's three next moves exit 0; the README's
`ein render rule --name <R> <file>` is the command's usage. No block or command showed an engine answer wrong, so no
register row is filed.

### Task T1f.5.26.2 — the pages fixed, and the pin

Fix each block that fails; the test; `docs/guide/README.md`'s note on what is pinned and what is not.

### T2 record, 2026-11-10

**The pages**, each fix the page's, never the engine's
([Q-M1f.70](../open_questions.md#q-m1f70--what-the-stage-does-with-what-t1-found-false-beside-the-blocks) (a)):

- **Chapter 4's block** shows `zebra2.ein` as it is: the `(config …)` line goes with its comment, the `std.bijection`
  import is written whole, the parenthesised prose is a comment (`;; … drink-loc, smoke-loc, pet-loc, right-of,
  next-to, is-a and is-a* likewise`), and the hypothesis layer the file is made of is shown, `(hrule guess …)` and
  `(solve :hrules …)` as the file writes them; the diagram gains the node and the prose a sentence for it. Its 14
  whole forms are `zebra2.ein`'s and its 3 elided ones are the three spatial rules (2026-11-10). After the
  transcript, the bare `ein solve` sentence says what the run prints (`exhausted yes`, and `--single`'s `work` row
  smaller), `-s` gives *the wall clock*, and `--trace` *the derivation step by step, each step's premises and what it
  derives*.
- **Chapter 3**: `adjacent-via-fwd` and `disjunctive-prune-fwd` carry `zebra2.ein`'s `:why`s; `co-located` is
  *Chapter 2's `co-located`, now a rule of its own*, since chapter 2 now shows it as a relation.
- **Chapter 2**: the transitive block writes `taxonomy.ein`'s `(relation is-a T T …)` and its facts' `:source`, so it
  is the file's program and answers the page's four states; the co-located block quotes `05_mini_zebra.ein`, the
  relation, its two activators and the two clues, ending in a `;; …` comment, and the section is *the first
  puzzle-shaped relation*, the chapter's opening *the two gentlest rules … and then the relation they make the
  puzzle's engine*.
- **Chapter 1**: `is-a` *is not built in*, and a puzzle whose `is-a` facts Ein may guess declares it.
- **`docs/guide/README.md`**: *Running the examples* no longer says *every snippet is runnable*, and a new section,
  *Which program blocks are read*, says what the test reads, what it does not (the prose, the `sh` commands outside a
  marked transcript, whether an excerpt quotes its file), and that this stage checked the quotes by hand.

**The pin**, [`ein-cli/tests/guide_programs.rs`](../../../ein.rs/crates/ein-cli/tests/guide_programs.rs), beside
`guide_transcripts.rs`, whose doc now points at it. It reads every fenced `lisp` block of every page in
`docs/guide/`, as the lexer reads strings and comments (`;` and `#| … |#`): a whole program is loaded with
`ein_ir::load_file` under the default options, so a `:why` is required, and `ein solve FILE` must exit 0; an
excerpt's whole forms are loaded together, the elided forms and the `…` between forms blanked character for
character, so a position the loader prints is mapped to the page's line. Two tests:

- `every_program_block_in_the_guide_runs`: no finding on `docs/guide/`, and it read a whole program, an excerpt with
  a form to load, and chapter 4's block as one, so a renamed fence cannot leave it green over nothing.
- `a_planted_refusal_in_a_copy_of_the_guide_fails_it`: the pages copied to a `Scratch` directory and four plants put
  after each page's first fence, each one finding on its page and none elsewhere: chapter 4's retired flag, placed
  on its page line; a relation with no `:why` in chapter 2's template, an excerpt; a `:priority` in chapter 1's first
  block, a whole program; and in chapter 3's first, a whole program that loads and whose `:expect` the engine
  refutes, which only the solve can fail.

**Held both ways on the real pages**: with `52e759e7`'s pages put back (`git checkout HEAD -- docs/guide`, the edits
restored after), `every_program_block_in_the_guide_runs` fails with T1's three refusals and no other, chapter 4's at
its lines 13 and 20; on the fixed pages it passes, and so does the planted test
(`cargo test -p ein-cli --test guide_programs`, 2026-11-10). The test reads the sixteen blocks in 0.15 s (debug,
2026-11-10).

## Acceptance

- No block in `docs/guide/` writes a form today's engine refuses, held by the test.
- Chapter 4's block matches `zebra2.ein`'s forms where it quotes them.
- `./run_tests.sh` green.

## Acceptance, re-taken (2026-11-10)

- **No block in `docs/guide/` writes a form today's engine refuses, held by the test**: met.
  `every_program_block_in_the_guide_runs` loads every whole form of the sixteen blocks and solves the thirteen whole
  programs, and fails on `52e759e7`'s pages with T1's three refusals; `a_planted_refusal_in_a_copy_of_the_guide_fails_it`
  holds a retired flag, a missing `:why`, a keyword and a refuted claim each to a finding.
- **Chapter 4's block matches `zebra2.ein`'s forms where it quotes them**: met. Each of its 14 whole forms, comments
  stripped and whitespace collapsed, is found in the file the same way (2026-11-10), and so is every whole form of
  chapter 3's blocks; chapter 2's match `neighbours.ein`, `taxonomy.ein` and `05_mini_zebra.ein` likewise. The test
  does not hold this, as the guide's README says.
- **`./run_tests.sh` green**: met, 17 min 26 s (`build/gate-s1f.5.26.log`, 2026-11-10), the two new tests among the
  suite's and the scratch check finding nothing they left.
