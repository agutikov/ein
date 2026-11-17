# S1f.20.10 — the user's review of the tree

**Phase:** [P1f.20](README.md)
**Status:** in progress since 2026-11-17.
**Source:** the user's review notes of 2026-11-17 on `docs/ein/`, verbatim in [§ The notes](#the-notes). They
override every earlier stage file and ruling of this phase where they differ.

---

## The notes

> enumerate folders and files in docs/ein, like ein/reasoning/01-static-ir/01-names-and-atoms.md
>
> No ASCII diagrams - use graphviz or plantuml
>
> docs/ein/reasoning - is abstract Ein reasoning theory, not even concrete algorithms,
> so it does not contain references to code or tests, no tables with tests, tests belong to implementation
> docs/ein/ein.rs
>
> Lanuages allowed in docs for description and examples:
> - docs/ein/reasoning:
>     - mathematic formulas - main language here
>     - graphical diagrams / ein graphs - for facts, rules, lattice, etc.
>     - NL - what arrow on the graph means, what formula means, what rule means, what are lattice nodes, etc.
> - docs/ein/user:
>     - ein language - main language here, because user writes programs in ein
>     - graphical diagrams / ein graphs - for viaualisation
>     - mathematic formulas - additional, for explanation, where it explains
>     - NL - explain what ein code say
> - docs/ein/ein.rs:
>     - diagrams for algorithms, state mashines, C4, processes, and other
>     - it is software project documentation, use whatever needed language (pseudocode, Rust, Python, shell, ...)
>
> docs/ein/reasoning - remove code and references to code, add more mathematic formulas and graphs
>
> docs/ein/user - all 4 representatins required for explaining core sematics - facts, rules,
>     - but it is not needed to explain just special syntax, like in
>       docs/ein/user/syntax-and-semantics/natural-language.md
>     - e.g. there is no graphical or mathematical representation of keyword `:why`, it is a built-in connection
>       between NL and ein
>     - there is no graphical and mathematical representation of macros special form, but there is both graphical
>       and mathematical representation of speific macro, like forall
>
> docs/ein/ein.rs - add more diagrams
>
> docs/ein/user/reference/grammar.md - ebnf in plantuml, links to lexer and parser grammars
>
> docs/ein/standard-of-proof.md - what is this? haven't decided where it goes?
>
> docs/ein/user/syntax-and-semantics/objects.md - there is NO TYPES in Ein, (relation is-instance Object Type) is
> almost an ordinary fact, and `Object` and `Type` here are ordinary objects, just labels, if no rule use them in
> `:match` they don't have any meaning and not affect program. the idea is that you may write `:match (and
> (relation ?R ?T1 ?T2) (?R1 ?x1 ?x2) (is-instance ?T1 ?x1))` - it checks that there is a fact of relation R, and
> it's first argument is connected with `is-instance` arrow with Object, that we may call `x1 type is Object`
>
> Math formulas must look like this:
>
>     $$
>     \operatorname{CyclicProof}(G,\varphi) \land \operatorname{GlobalTraceValid}(G) \implies \operatorname{Valid}(\varphi)
>     $$
>
> and not like this:
>
>     ```text
>     owes(o, a, θ, S)  ⟺  S ⊨ T(θ)  ∧  ¬∃ ȳ. S ⊨ W(θ, ȳ)
>
>     owes(S)        = | { (o, a, θ) : owes(o, a, θ, S) } |
>     discharged(S)  ⟺  owes(S) = 0
>     ```
>
> docs/ein/user/syntax-and-semantics/macros.md - this example doesn't show how to use macro, rewrite
>
> docs/ein/user/reference/stdlib.md - split into subfolder with files
>     - show more usage examples and what library symbols produce
>     - separate usage description from module internal implementation
>     - if list rules - show them rendered

## How the notes are read

Each is a rule of the documentation and becomes a decision under `docs/decisions/docs/` (part A writes them);
the readings below are the stage's, and a default among them is marked.

1. **Numbering.** Every folder and page under `docs/ein/user/`, `docs/ein/reasoning/`, `docs/ein/ein.rs/` and
   `docs/ein/overview/` is named `NN-kebab-name`, `NN` its place in its folder's reading order, from `01`, with no
   gap. A `README.md` keeps its name, as does a folder of a page's assets (`diagrams/`). The four top-level
   folders keep theirs, as the note's example does (`ein/reasoning/01-static-ir/…`). The guide's `01_…`
   underscore names become `01-…`. **Default:** the order is the one each folder's README lists its pages in;
   `user/` reads `01-install.md`, `02-guide/`, `03-syntax-and-semantics/`, `04-reference/`.
2. **Diagrams.** No diagram is drawn in characters. A diagram is a fenced `dot` block (Graphviz) or a fenced
   `plantuml` block in the page, the way the pages have carried `dot` blocks since the first ones.
   **Default:** a graph the engine draws that is too large to read as source stays a `.dot` with its `.svg`
   beside the page (`diagrams/`), as the eight now there. What a program prints (a read-out's table, a grid) is
   output, not a diagram, and stays a block of text.
3. **Formulas.** A formula is TeX: display math between `$$` lines, inline math between `$`. No formula in a
   fenced `text` block, and none spelt in Unicode operators.
4. **The languages of a perspective.**
   - `reasoning/`: mathematics is the main language; graphs (of facts, rules, the lattice) beside it; words say
     what a formula, an arrow, a node means. **No Ein source, no shell, no Rust, no file path, no test name, no
     table of tests, no algorithm**: what an implementation does to compute a thing is `ein.rs/`'s. A page of
     `reasoning/` links other pages of `docs/ein/` and decisions, and nothing else.
     **Default:** an example program appears in `reasoning/` as its graph and its formulas; where a reader needs
     its Ein spelling the page links the `user/` page that has it.
   - `user/`: Ein is the main language; graphs visualise it; mathematics where it explains; words say what the
     Ein says.
   - `ein.rs/`: software documentation, any language it needs; more diagrams (algorithms, state machines, C4,
     processes).
5. **Four representations.** A page of `user/` on a core semantic notion (objects, relations, facts, rules,
   constraints, hypotheses, obligations, the answer) carries all four: Ein, graph, words, mathematics. A page on
   special syntax (`:why` and the natural-language texts, the macro form, imports) does not owe a graph or a
   formula; a specific macro that means something (`forall`) has both.
6. **No types.** Ein has no types. `(relation is-instance Object Type)` is almost an ordinary fact, and `Object`
   and `Type` are ordinary names, labels with no meaning a rule does not give them. `objects.md`, and every page
   that says *type* of a declaration's slots, says so; the example of the note (a rule matching `(relation ?R ?T1
   ?T2)`) is run before it is written, and what the engine does otherwise is a finding, not a sentence.
7. **`grammar.md`**: the grammar as EBNF in `plantuml` blocks (`@startebnf`), with links to the lexer's and the
   parser's grammar files. The page's blocks stay generated, so the generator (the test that blesses them) writes
   the new shape.
8. **`macros.md`**: its example is rewritten to show a macro being used: defined, called, and what the call
   expands to.
9. **`stdlib.md`** becomes a folder, a page a module family: usage first (what a program imports and writes, and
   what the symbols produce, with examples that run), the module's internals apart; a rule that is listed is
   shown rendered (`ein render`'s graph of it).
10. **`standard-of-proof.md`.** It is the page ruling 1 of S1f.20.2's review put under `docs/ein/`: what it takes
    to call a claim about Ein settled. It belongs to none of the three perspectives. Its two rules and the rule
    on numbers are already decisions (`Q-M1e.1`, `Q-M1e.2`, `Q-M1e.4`). **Default:** the page is dissolved: the
    decisions carry the rules, every reference to the page is re-aimed at the decision it means, and its table of
    premises (what the answer rests on, and what holds each) is split, the premise and its argument to
    `reasoning/`, what enforces it to `ein.rs/`. The user's to reverse.

## Parts

- **A. The names and the check** (one lane, first, on master): the decisions; `utils/check_doc_blocks.py`, a gate
  step (every fenced `dot` block of the project's pages compiles; the numbering; `reasoning/`'s fences and links;
  no formula in a `text` block; `--plantuml` checks the `plantuml` blocks where a PlantUML that knows them is
  installed, the dev host's tier, since the runner's package predates `@startebnf`); the renumbering, every
  reference moved with its file; `standard-of-proof.md` dissolved.
- **B. `reasoning/`** in two lanes: `static-ir` with `rule-evolution`, and `kb-evolution`. What a page sheds
  (commands, tests, algorithms) goes to new pages of `ein.rs/` the lane writes, so no lane edits another's file.
- **C. `user/`** in three lanes: the syntax-and-semantics pages (four representations, no types, the macro
  example); the standard library's folder; the guide and the reference (diagrams, formulas, the grammar page).
- **D. `ein.rs/`**, after B merges: the diagrams, and what B shed set in its place.
- **E. The gates**: the link check, the release-voice check and the new check on each merge; the gate once on the
  last commit; `utils/ci_container.sh` if a workflow changed.

The phase still stops for the user's review at the end; it is not closed.

## Record

(each part writes what it did here)

### Part A

Five commits on master, `aa861dfa` to the one that carries this record; the gate's result is the last paragraph.

**The renumbering** (`aa861dfa`). 70 pages and 7 folders renamed; 4 READMEs and 16 diagram files moved with
their folders: 90 files. The map is the commit's renames, `git show -M30% --diff-filter=R --name-status aa861dfa`
(`-M30%` because three READMEs changed more than half their lines, every one a link). Per folder, in the order
its README lists:

- `user/`: `01-install.md`, `02-guide/`, `03-syntax-and-semantics/`, `04-reference/`.
- `user/02-guide/`: the six chapters `01-objects-and-relations.md` to `06-zebra-walkthrough.md`, then
  `07-examples.md`, `08-word-problems.md`.
- `user/03-syntax-and-semantics/`, by the README's § The pages: `01-objects`, `02-relations`, `03-facts`,
  `04-ontology`, `05-rules`, `06-constraints`, `07-properties-and-operators`, `08-obligations`, `09-hypotheses`,
  `10-theory`, `11-macros`, `12-query`, `13-solutions-and-models`, `14-natural-language`.
- `user/04-reference/`: `01-grammar`, `02-syntax`, `03-reserved-names`, `04-stdlib`, `05-cli`, `06-config`,
  `07-diagnostics`, `08-read-out`, `09-read-out-examples`, `10-answer-dump`, `11-region-dump`, `12-rendering`,
  `13-glossary`.
- `reasoning/`: `01-static-ir/` (`01-names-and-atoms`, `02-relations-and-facts`, `03-values-and-order`,
  `04-rules-as-statements`), `02-kb-evolution/` (`01-firing`, `02-saturation`, `03-absent-and-naf`,
  `04-hypothesis-generation`, `05-hypothesis-space`, `06-solution-semantics`, `07-finding-r`, `08-monotonicity`,
  `09-cores`, `10-satisfiability`; `diagrams/` keeps its name), `03-rule-evolution/` (`01-rules-as-data`,
  `02-analysis-of-rules`, `03-transformations`). The README's list ran 1 to 17 across the three parts; it now
  starts at 1 in each, so a page's number in the list is its file's.
- `ein.rs/`: `01-architecture`, `02-algorithms`, `03-design/`, `04-api`, `05-events`, `06-lattice-dump`,
  `07-reserved-engine-strings`; `03-design/` by its README's reading order: `01-determinism-and-order`,
  `02-shared-assets`, `03-data-model`, `04-ir-frontend`, `05-matcher`, `06-saturation`, `07-search`,
  `08-parallelism`, `09-binary-format`, `10-toolchain-and-layout`.
- `overview/`: `01-inspirations.md`.

*The references.* One script (kept outside the repository) built the map, moved the files and rewrote every
path-like token of every tracked text file: 2 259 relative paths (a page's links, a Rust comment's), 345 paths
from the repository's root, 224 short paths of two or more components (`design/parallelism.md`), 690 bare page
names (`read-out.md`), in 579 files. By hand after it: the release workflow's URL of the install page, one
ambiguous `saturation.md` label, `utils/read_out_examples.py`'s two relative links, a shell line of the
monotonicity page, `guide_programs.rs`' `starts_with("04_")` twice, one link of `docs/lib/`.
**This phase's own records keep their short names** (`disposition.md`'s tables map an older tree onto this one in
them): there the script re-aimed links and full `docs/ein/…` paths only. This stage file is not rewritten at all,
so § The notes stays verbatim. An output file named `theory.md` that `ein-infer/examples/pair_layer.rs` writes is
not the page and keeps its name.

*The citations of a design page by number.* The old numbers (02 to 12, with no 09) map to the new by page:
02→01, 11→02, 03 to 08 unchanged, 10→09, 12→10. Five citations outside `plans/`, each now the page's path and
a section that says what is cited: `per-commit.yml` (no hash-map iteration: `01-determinism-and-order.md` § 2.3,
where § 9 is now the tests), `release.yml` (snmalloc default-on: `10-toolchain-and-layout.md` § 4),
`utils/bench_env.sh` (resolution order: `02-shared-assets.md` § 3), `ein-ir/tests/grammar_decisions.rs` (the
message of a refusal: `04-ir-frontend.md` § 6, where § 4 is now positions and strings), `AGENTS.md` (`unsafe`:
`10-toolchain-and-layout.md` § 2). `utils/check_hashmap_iteration.py` cited a section for *a `dylint` rule is
allowed for*, which no page says; its two sentences say it of themselves now. The plans' citations of a design
page by number are records of the pages as they were and are left.

*What the binary prints.* `ein`'s help and the region dump name pages by path, and a state file's header names
`answer-dump.md`: `ein-cli/tests/golden/help_shape.txt` moved with the paths, and
`ein-render/tests/golden/corpus_shapes.md5` was re-banked over every corpus program
(`EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance`): 637 lines, every one an
`oracle` rendering, the same programs before and after.

**The decisions** (`0694c726`), in `docs/decisions/docs/`: Q-M1f.367 (numbering), Q-M1f.368 (diagrams),
Q-M1f.369 (formulas), Q-M1f.370 (the languages of the three perspectives, in one decision), Q-M1f.371 (four
representations). Q-M1f.368 also states the default on output: a block of what `ein` printed that could be read as
a drawing stands in a region the tests compare or under `<!-- output -->`.

**The check** (`ad0d16ee`), `utils/check_doc_blocks.py`, the gate's tenth static step. In `per-commit.yml` it is
in the `rust` job, after the Graphviz install, since the `shared-assets` job installs none. Its rules, and today's
findings (`python3 utils/check_doc_blocks.py --counts`):

| rule | what fails | today |
|---|---|---|
| 1 | a fenced `dot` block Graphviz refuses | 0 of 41 blocks; none needed a fix |
| 2 | the numbering | 0 |
| 3 | `reasoning/`: a fenced block not `dot` or `plantuml`; a link out of `docs/`; inline code that is a file path, a Rust path, a crate, test, function or field name, a command line | 704, all `reasoning/`: 27 blocks (17 `text`, 6 `lisp`, 4 `sh`), 42 links, 297 file paths, 306 test or field names, 32 command lines |
| 4 | a formula in a `text` block | 31: `reasoning/` 11, `user/` 20 |
| 5 | a diagram drawn in characters | 6: `ein.rs/` 3, `reasoning/` 1, `user/` 2 |
| 6 | `--plantuml`: a `plantuml` block PlantUML refuses | 0 of 0 blocks |

The gate step fails on rules 1 and 2. **`--all` adds 3 to 5, and the gate step becomes `--all` when the lanes
have driven them to zero**: the coordinator changes the step in `run_tests.sh`, `per-commit.yml` and
`check_gate_steps.py`'s `TESTS_ONLY_SKIPS`. `--plantuml` is a dev-host step, named in `DEV_HOST_ONLY`. Choices
the docstring states in full: rule 4's operators are those in no golden under `ein.rs/crates/*/tests/golden/` and
not on the generated read-out page (`⟺ ⇔ ∀ ∃ ⊨ ∨ ∈ ∉ ⊆ ∪ ∩ ≤ ≠ ≡ ↦ ⊥ ⇒` and a few more); a trace prints the
stdlib's `⟹` and `∧`, so a formula spelt only in `∧ ¬ → ⟹` is not caught. A block is output, and rules 4 and 5
skip it, on a page generated whole, inside a region a test compares (`<!-- name … -->` to `<!-- /name -->`), or
under `<!-- output -->`. Rule 3 does not read an option written alone (`--minimal`) or inline Ein (`(false)`).

**`standard-of-proof.md`** (`95df1737`), dissolved. Its rules: Q-M1e.1 gains *whatever states the claim* and
*the refusal* among the things that check an absence; Q-M1e.2 already said the rest (the premise enforced, the
test that asserts something next to it, the argument beside the code). Its table of four premises: the
statement and the argument of each were already in `reasoning/` (`03-values-and-order.md`, § 1.6 of
`06-solution-semantics.md` with `03-absent-and-naf.md`'s refutation section, `02-saturation.md`'s alive-set
invariant, `02-relations-and-facts.md` § Equality), so nothing was added there; what enforces each went to
`ein.rs/`: `03-design/README.md` § What the answer rests on (the four in one table, each with the page that
states it and the test or refusal that enforces it), and `07-search.md`, which now names the alive invariant's
two tests and the refusal's test and probe. The worked example's program is
`examples/ein-bugs/naf-upward-closure.ein`, which `03-absent-and-naf.md` already describes. References
re-aimed: the root README, `docs/ein/README.md`, `user/README.md`, five Rust comments (bare id), AGENTS.md,
three plans. Nothing was left without a home.

**Left to the lanes, seen on the way.** `reasoning/README.md` § What holds a page says each page ends with its
tests, which Q-M1f.370 takes out of `reasoning/` (lane B). `user/03-syntax-and-semantics/README.md` says *in
plain notation* of the mathematics (lane C). The `stdlib.md` page became `04-reference/04-stdlib.md` and is still
one page (lane C makes it `04-stdlib/`).

**Outside the scope**: L-51 to L-55 of [`findings.md`](findings.md).

**The gates.** On `053fdf1d`: `./run_tests.sh` exit 0, its seventeen steps, 160 test binaries and none failed;
`python3 utils/doc_audit.py --links --check --tracked` 0 links and 0 decision ids, `check_release_voice.py` none,
`check_gate_steps.py` and `check_doc_blocks.py` pass, on that commit and after each of the four before it (the
links, the voice). The new check was also run in the runner's image on the working tree (Graphviz 14.1.2,
x86-64-v3): 41 `dot` blocks, rules 1 and 2 hold. `utils/ci_container.sh` was not run: the stage's part E runs it,
a workflow having changed.

