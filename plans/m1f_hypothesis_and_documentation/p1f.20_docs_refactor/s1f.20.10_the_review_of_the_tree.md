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
