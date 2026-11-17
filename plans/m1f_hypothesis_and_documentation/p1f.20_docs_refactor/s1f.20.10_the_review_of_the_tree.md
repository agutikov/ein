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

## The second notes: `reasoning/01-static-ir/` (2026-11-17)

After the lanes merged and the user accepted the review, the user read `reasoning/01-static-ir/` and wrote,
verbatim (the close of M1f, started on the acceptance, is paused on the user's word until this is done):

> review docs/ein/reasoning/01-static-ir
>
> the very first diagram in docs/ein/reasoning/01-static-ir/01-names-and-atoms.md is already not correct
>
> maybe we don't need to split names,atoms,relations,facts,values into multiple files?
>
> docs/ein/reasoning/01-static-ir/03-values-and-order.md already tell about rules, but rules introduced in
> docs/ein/reasoning/01-static-ir/04-rules-as-statements.md
>
> docs/ein/reasoning/01-static-ir/03-values-and-order.md already talks about candidates, but hypothesis are
> introdyced in docs/ein/reasoning/02-kb-evolution
>
> we probably need only 2 files in docs/ein/reasoning/01-static-ir
>
> first describes knowledge base of facts of relations
> starting from Levi-bipartite hypergraph representation
> and then given already some graph, name what is where: named objects, relations, facts
> then their properties - identity, order, and other
>
> second is about the graph rewriting rules
> same facts structures in :match and :assert but with variable names
> pattern matching
> first - we do graph rewriting and then how it is declared
>
> we don't talk here about rules firings
> e.g. "state only grows" belongs to docs/ein/reasoning/02-kb-evolution

**How they are read.** `01-static-ir/` becomes two pages: the knowledge base, and the rewriting rules. The first
starts from the graph, the Levi-bipartite hypergraph, and only then names what is where in it (named objects,
relations, facts) and gives their properties (identity, order, the rest). The second starts from rewriting a graph
(the same fact structures with variables, matching a pattern) and only then says how a rule is declared. Nothing in
the part speaks of a firing, of how a state grows, or of a candidate: that is `02-kb-evolution/`'s, and what the
four pages said of it moves there. No page uses a notion before the page that introduces it. **The first diagram**:
it is the engine's `--levi` drawing, which labels a fact's node with its relation and draws no node for the
relation, so `left-of` stands on two nodes and is one name; in the canonical Levi form every name is one node, a
relation among them, and a fact's node has an edge to its relation beside the numbered edges to its arguments
(the stage's reading, from the form the knowledge base was first documented in; the user's to correct).

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

### Parts B, C and D: the lanes

Seven lanes, each in a worktree of its own on a branch of its own, merged into master as each reported. Every
worktree was created at the commit before part A and fast-forwarded to master by its lane before it started.

**B1** (merge `6e545615`). `reasoning/README.md`, `01-static-ir/` and `03-rule-evolution/` are theory: 94
displayed formulas and 21 `dot` graphs, no Ein source, command, path, test name or table of tests. The README
says what a page states and has § Notation. The graphs the engine can draw are the DOT `ein render
program|kb|rules|rule` and `ein solve --trace` write for a small program; schematics are drawn by hand. What the
pages shed is `docs/ein/ein.rs/08-reasoning-in-code/`: a README, `01-static-ir.md` and `04-rule-evolution.md`,
each with a code map, the test tables (87 names, each checked to exist) and the commands, the example programs as
Ein source. *No types*: four kinds of node, a declaration's argument names are ordinary names. `03-transformations.md`
was not only a plan: it defines what preserves the answer and argues three invariances.

**B2a** (merge `8a82ad98`). `01-firing`, `02-saturation`, `03-absent-and-naf`, `08-monotonicity`, `09-cores` and
`10-satisfiability`: every definition and claim a `$$` formula first (86 where there were none), 30 `dot` blocks
beside the seven images of the monotonicity page, seven propositions and the theorem with their arguments, no
algorithm. Headings other files anchor on are kept, with every § number of the monotonicity and satisfiability
pages. What they shed is `08-reasoning-in-code/02-saturation-and-negation.md`. Two of the seven images had
drifted from the binary and are re-drawn; `monotonicity_three_components` is refused at load and kept as drawn.
One statement was corrected, not carried: on a stratified program the written order of the rules, and the
agenda's first half, can decide root's fixpoint where two absences chain over relations no commitment changes
(the finding is in [`findings.md`](findings.md)).

**B2b** (merge `2ba3db8f`). `04-hypothesis-generation`, `05-hypothesis-space`, `06-solution-semantics` and
`07-finding-r`: H as a definition by cases, the space as the lattice of commitments with every object over it a
formula, R and its counts, the questions asked of R named mathematically (06's new § 5), and in 07 the reference,
the licences as lemmas with their arguments, the propositions each question rests on and the agreement theorem;
fifteen `dot` blocks; no procedure. What they shed is `08-reasoning-in-code/03-hypotheses-and-the-answer.md`:
where each notion lives, the routes as procedures, the commands, `h-space.json` in full, the test tables.
`ein-cli/tests/show_hyps.rs` and `h_space_view.rs` read the generated blocks from the new page. One anchor was
renamed (06 § 2.5).

**C1** (merge `a29a8c33`). Every page of `user/03-syntax-and-semantics/` on a core notion (01 to 10, 12, 13)
carries *In Ein*, *As a graph*, *In words* and *In mathematics* of one example: the program runs, the graph is the
DOT the named command wrote, every formula is TeX, and every rule of a page's example is drawn. 11 and 14 are
special syntax: 11 shows a macro defined, called, expanded and answering as the written-out program does, then
`forall` in all four; 14 lost its graph and its formula. *No types* is Q-M1f.372, and 01, 02, 04, 07, 09 and 10
say *label* of a declaration's argument names. The user's rule was run: with no parameter it is refused (a
variable in a relation's place is a parameter's); with `?R` a parameter and a second rule deriving its activator
from `(relation ?R ?T1 ?T2)` it loads and fires, and is 01's example.

**C2** (merge `1d23f9e6`). `user/04-reference/04-stdlib.md` is the folder `04-stdlib/`: an index and nineteen
pages, a page a family of modules. A usage page says what a program imports and lists and what every symbol
produces, by small programs each run with the binary and its output pasted, the failures among them; how a family
works is apart. All 499 rules of the library are shown, each as its module writes it and as `ein render rules`
draws it, in a region `utils/gen_stdlib_rule_pages.py` writes; its `--check`, run by `check_generated.py`, fails
on a region that differs, on a rule no page shows and on one shown twice. Seven rules whose drawing has more than
120 nodes are shown as Ein alone. Q-M1f.375 states the rule. One sentence of the old page was false and is gone.

**D** (merge `e2bee982`). `docs/ein/ein.rs/` (but `08-reasoning-in-code/`) and `docs/ein/overview/` had no `dot`
or `plantuml` block and have 54, each read off the code it draws: the C4 context, the crates' dependency edges
from the manifests, a component graph each for `ein-ir`, `ein-infer` and `ein-render`, one `ein solve` as a
sequence; `02-algorithms.md` as the page of the engine's algorithms, nine activity diagrams with pseudocode; in
`03-design/`, the layered KB and a fork, the load path, the matcher's plan and a register's life, the saturation
loop and a commitment's life as state machines, a fan-out batch, the `.einb` layout as records, the build; the
`--events` stream of one run and its legal orders; the lattice dump. The three character-drawn diagrams are gone.
Eight statements were corrected to the code. `overview/README.md` states no plan (L-53).

**C3** (merge `0f607471`). The guide and the reference. `01-grammar.md`'s two generated regions are `ein.l` and
`ein.y` as EBNF in `plantuml` blocks, nineteen diagrams, each group under a generated heading with a link to its
file; the generator is `ein-ir/tests/grammar_pages.rs` (a production's alternatives, a left-recursive list as a
repetition, a lexer rule's regular expression translated from the tree `regex-syntax` gives, a dev-dependency of
`ein-ir` already in the lock), and a construct with no translation fails the test; Q-M1f.378. Start conditions
and error names are a generated table. No drawing in characters is left in the guide or the reference: the rule
and KB drawings are what `ein render` writes, the walkthrough's two grids what `--models grid` prints. Formulas
are TeX; `08-read-out.md` states what its counts count. A declaration's argument names are labels throughout; the
word *type* stays where a program uses it for itself and where the binary prints it.

### Part E: the integration, and the gates

**The integration lane** (merge of `ab1fc97d` to `84a585c6`). *One notation in `reasoning/`*: `N` and `E` are the
minimal dead and the minimal discharging sets on every page, as the read-out prints them; the program's names
are `names(P)`, its objects `obj(P)`; `K` is a knowledge base and `S` the state a saturation ends in; a rule is
`(p̄, φ, ψ)`, a firing `ξ = (ρ, a, θ)`, an activator the fact `a`, a derivation `δ` in `J(f)`. `reasoning/README.md`
§ Notation is the table of every symbol with the page that defines it, and a second table of each page's own
letters. *One word*: a declaration's argument names are *labels* on every page, with Q-M1f.372 cited where a page
says Ein has no types. *An algorithm is told once*: the routes' procedures, the tree's order, `--single` and the
oracle are `02-algorithms.md` § 8's, and the pages of `08-reasoning-in-code/` name the code, the tests and the
commands and link the flow. Four statements were corrected to the code or the binary: a monotone program's
describe of a dead root takes the state route; `ein-ir` resolves imports before it expands macros; the order the
rules fire in decides no answer only where no rule's `absent` reads what another rule waiting on an `absent`
derives (`AGENTS.md`, the saturation contract, the rules page, the syntax reference, finding R, and the
transformations' Propositions 2 and 3); a hypothesis rule's activators are those the program lists. The
decisions' index lines, fifteen links to the standard library's reference aimed at a module's usage page, the
two commands' documentation cited where it is (the binary's help too, `help_shape.txt` re-blessed), `AGENTS.md`
and `utils/README.md` in place. Findings L-56 to L-75 of [`findings.md`](findings.md).

**The coordinator** (`e0892c68` and the commit of this record). Two links followed what the reasoning shed, one
through `utils/read_out_examples.py`. Rule 6 of `check_doc_blocks.py` draws each `plantuml` block and reads the
drawing, since PlantUML's `-syntax` passes EBNF it then draws as a syntax error. **The gate step holds rules 1 to
5** now that the pages do: the script's default, so no step list changed.

**The checks on the merged tree**: `python3 utils/check_doc_blocks.py --plantuml`, 482 pages, 673 `dot` and 50
`plantuml` blocks, rules 1 to 6 hold; `python3 utils/doc_audit.py --links --check --tracked`, 0 links and 0
decision ids; `python3 utils/check_release_voice.py`, none; `python3 utils/check_gate_steps.py` passes.

### The second notes: what the user ruled while the lane ran, and the lane

**The user's rulings, 2026-11-17, in the order given** (verbatim):

> let's review following definitions: objects are nodes in levi-bipartite representations, atoms are labelled nodes
> w/o outgoing arrows, names are labels, facts are nodes with out arrows, relations are objects under first arrow,
> relation desclaration is a fact with first arrow pointing to object with name="relation" - then second arrow
> points to the name of relation
>
> so effectively we got the obejct identity is either name or tuple
>
> Points to settle: "{?1}" and :why are both ein languate syntax and is out of scope here; 2 - yes, object is a
> levi-bipartite graph node; 3 - say that levi-bipartite is an initial idea, the final graph is not strictly it, but
> still has nodes with and without arrows; 4 - yes; Then - the graph is not just drawing, it's a canonical
> representation, because in the list of tuples names can be repeated, and in the graph there is only one node with
> that name and only one node with this set of arrows; Additionally can say that facts in general are directed
> hyper-edges, and facts with 3 arrows natively can be represented as labelled arrow, but in this case drawing
> loose relation name identity preperty, because there are many arrows with same label, but in canonical graph form
> there are many facts but the same relation mean their first arrow goes to the same atom; 3 - again, hypotheses
> are out of scope here
>
> I forgot to mention! We are also talking about relations as: entire set of relation facts, as separate objects -
> relation fact; there is a potential ambiguity between them; relation declaration say about entire relation as a
> single object, so do facts where relation name is used as argument
>
> docs/ein/reasoning/01-static-ir/01-knowledge-base.md - add identity graph, or definition graph into "What is
> where", maybe like class diagram?
>
> unfortunately inline dot and plantuml sometimes doesn't work - so need to put all diagrams in subfolder, render
> to svg and insert svg; also long multi-statement math should be written in multiple lines

The points the user answers by number were the coordinator's: (1) how a fact's arrows are numbered; (2) *object*
taking the place of *value* on every page; (3) the graph not being bipartite once a fact points at a fact; (4) a
relation by its role against a declared relation.

**The lane** (`5eaa0acc`, `1af644e5`). `reasoning/01-static-ir/` is two pages. `01-knowledge-base.md` starts from
the graph: the Levi graph of a hypergraph as the idea, then the graph a knowledge base is, nodes without outgoing
arrows and nodes with ordered ones, one node for a name and one for a list of arrows, with the set of facts held.
It names what is where by the user's definitions (objects, atoms and names, facts, relations under the first
arrow, declarations and declared relations; the relation, its extent, a fact of it), carries a class diagram of
the definitions, and gives the properties: identity as name or tuple, the total order with its proof, the text and
the quoted form, the count of arguments, nesting, the status of a fact in a set. The first drawing has one node
for `left-of`. The labelled-arrow form stands beside the graph as a drawing convention, with what it loses.
`02-rewriting-rules.md` does the rewriting first (a pattern graph, a match as a homomorphism, the conditions, the
conclusion, the result of rewriting, the span) and then the declaration (parts, activators, conditions, kinds, the
four levels). Neither page speaks of a firing, of growth or of a hypothesis; what the four old pages said of those
is in `02-kb-evolution/`: a firing's identity, a fork and the independence of outputs from numbering in
`01-firing`; the activator-order corollary in `02-saturation`; state identity and the candidates' order in
`06-solution-semantics`; provenance, origin and what a fact reaches in `09-cores`. Across `reasoning/` and
`ein.rs/08-reasoning-in-code/`, *object* is a node of the graph, the set is `O`, *value* in that sense is gone,
and the blind enumerator's objects are the guessable atoms. The review corrected thirteen statements or diagrams
against the binary, among them the pattern grammar (a disjunction is a whole match), the variables a guard binds
(they range over the program's names), the scope of an absence (by written order), checks in a conclusion,
`names(P)`, and the pushout (it holds where one arrow list is one node). The part's seventeen diagrams are files
under `01-static-ir/diagrams/`, each with the SVG drawn from it, and the pages embed the SVGs; its multi-statement
formulas are one statement a row. Every reference to the four old pages is re-aimed; no printed path and no
golden moved.

**Left by these rulings, for the lanes that follow**: every other diagram of `docs/ein/` as a file with its SVG,
with the tool that draws them and the check that holds them, the generated pages among them; every other
multi-statement formula one statement a row; *object* for *value* in `user/`, the rest of `ein.rs/`, the
decisions and `AGENTS.md`; *relation* said of an extent on other pages of the reasoning.
