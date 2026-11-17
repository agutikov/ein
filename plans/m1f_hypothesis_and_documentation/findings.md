# M1f — what the documentation tree found and left

**Milestone:** [M1f](README.md). Written 2026-11-17, at the close of **P1f.20, the documentation tree**, on the
user's word (*"go ahead to close p1f.20"*).

P1f.20 kept what its stages found outside their scope in a findings page of its own, L-1 to L-75, and showed it.
The phase's folder is deleted
(`git log --diff-filter=D -- plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor`; its findings page is
`findings.md` there), and this page is every finding of it that is **open and has no other owner**, each under
its L-number, with its evidence. L-76 to L-90 were filed at the close: what the lanes that ran after the phase's
page stopped at L-75 reported (the second notes on `reasoning/01-static-ir/`, the diagrams as files, the three
lanes on the ruled words and the formulas), and three things earlier stage records held and no row did.

A row marked **the user's call** needs a ruling before anything is built; the others are small and need none.
None is a known wrong answer: the one the phase found, L-61, is a row of the register.
[§ Defaults the documentation tree took](#defaults-the-documentation-tree-took) is the choices the stages made
that the user never ruled on, so that they are not lost with the stage records.

Every command is run from the repository's root, with `ein` the gate build, `ein.rs/target/gate/ein`.
**Re-taken at the close** on the gate binary of `186b248e`: L-19, L-35, L-56, L-57, L-62, L-63, L-68, L-76 and
every `grep` and count below. Read and not re-run: L-74, L-78, L-82, L-83, L-84.

## Where each finding is

**Closed, and gone with the folder**: L-1, L-3 to L-10, L-12 to L-18, L-20, L-23 to L-25, L-27 to L-29, L-31 to
L-34, L-36 to L-40, L-43, L-46, L-52, L-53, L-58, L-67, L-69, L-70 and L-73. L-2 was withdrawn; L-55 was fixed at
the close (`utils/README.md`'s row for `check_hashmap_iteration.py` carries no count). L-60, the decisions whose
wording said *type*, was closed by the lane that put the decisions into the ruled words, but for what L-57, L-80
and F23's D-20 carry. Of the later lanes' findings, closed without a number: the extent as held facts only (the
user ruled it: every fact of the relation, held or not, with the held extent beside it); the old words outside
`01-static-ir/` and *relation* said of an extent on other pages (the three lanes on the words did them); Q1's
*typed, labelled hypergraph* (reworded); Q-M1f.34 worded for guards (its *Why* states the rule for every premise);
`written` leaving out the fact a negation denies, and `(not (false))` loading (both are what the pages and
Q-M1f.123 say, and P1f.22's analysis has them); `ein saturate --dump`'s census reading the top level only (stated
on `08-reasoning-in-code/01-static-ir.md`); and the thirteen statements of the old static-IR pages corrected
against the binary.

**Moved to another owner:**

| finding | where it is |
|---|---|
| L-48, M2's plans mean to write into a page that is gone | [M2's P2.1](../m2_nl_to_ir/p2.1_kernel_as_instrumentation/README.md#the-page-this-phase-writes-into) |
| L-59, the engine's own words say *type*; and what the lanes on the ruled words listed of *value*, *atom*, *object* and *relation* in what the binary prints | [F23](../followups/f23_diagnostics_in_eins_words.md#the-items)'s D-20, **the user's call** |
| L-61, rule order decides root's fixpoint under a *monotone* verdict | the register's `W8/fifo-gap-root`, and [F21 § Handed over by M1f, 2026-11-17](../followups/f21_non_monotone_programs_decided.md#handed-over-by-m1f-2026-11-17-the-fifo-gap-at-root), with **the user's three calls** |
| L-71, a list of activators written with the hrule's name twice is refused with a message about something else | [F23](../followups/f23_diagnostics_in_eins_words.md#the-items)'s D-21 |
| L-87, an import of a module that is not there is refused with a line that ends `(None)` (a stage's record held it; no row did) | [F23](../followups/f23_diagnostics_in_eins_words.md#the-items)'s D-22 |
| L-88, the analysis of rules gives the dependency graph an edge for a premise a file cannot write | [P1f.22's findings](p1f.22_nested_facts/findings.md), N-14 |

**Here**, by subject: [the language and the engine](#the-language-and-the-engine) (L-19, L-30, L-56, L-57, L-62);
[the engine's renderers](#the-engines-renderers) (L-35, L-63, L-76);
[the engine's words against the definitions](#the-engines-words-against-the-definitions) (L-80, L-81);
[the reasoning pages](#the-reasoning-pages-reading-order-and-two-sense-words) (L-68, L-75, L-78, L-82, L-83);
[the user's pages and the standard library's](#the-users-pages-and-the-standard-librarys) (L-64, L-72, L-77, L-79);
[weak or missing tests](#weak-or-missing-tests) (L-22, L-42, L-66, L-74, L-84, L-86);
[dead code, and comments](#dead-code-and-comments) (L-11, L-26, L-41, L-45, L-47, L-65);
[examples](#examples) (L-44, L-49);
[the release voice where the check does not read, and the gate](#the-release-voice-where-the-check-does-not-read-and-the-gate)
(L-2, L-50, L-51, L-54, L-85, L-89); [decisions](#decisions) (L-21, L-90).

## The language and the engine

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-56 | **The rule of the user's note does not load as written.** *`:match (and (relation ?R ?T1 ?T2) (?R ?x1 ?x2) (is-instance ?x1 ?T1))`* writes a variable in a relation's place that no parameter binds, which the loader refuses: a relation variable is always a parameter of its rule. With `?R` a parameter and a second rule that concludes the activator from `(relation ?R ?T1 ?T2)`, it loads and does what the note says. The pages write that form and say so: `user/03-syntax-and-semantics/01-objects.md` § No types, Q-M1f.372, `reasoning/01-static-ir/01-knowledge-base.md`, `ein.rs/08-reasoning-in-code/01-static-ir.md` | `(relation is-instance Object Type :why "{?1} is a {?2}")` `(relation lives-in Person House :why "{?1} lives in {?2}")` `(relation fits Object Type :why "{?1} fits {?2}")` `(rule first-argument-fits () :match (and (relation ?R ?T1 ?T2) (?R ?x1 ?x2) (is-instance ?x1 ?T1)) :assert (fits ?x1 ?T1) :why "{?x1} fits {?T1}")`. `ein solve FILE`: *kb load error: rule 'first-argument-fits': its :match writes (?R ?x1 ?x2) with ?R as its relation, which no parameter binds — a relation is named by a name or by a parameter its activator binds* | **The user's call**: whether a relation variable may range free in a `:match`. Until then the pages' parameter form stands |
| L-57 | **A label is not inert: the blind enumerator reads which names are labels.** It leaves a name some declaration writes as a label out of the guessable atoms (Q-M1e.12, Q-M1f.306), and fills every argument place of every declared relation of one or two arguments with every guessable atom, filtering no place by its label (`hypgen::candidate_objects`, `raw_candidates`). So two programs that differ only in the labels of one declaration, with no rule reading a label, have two candidate sets and two regions. That is against the user's note, *"if no rule use them in `:match` they don't have any meaning and not affect program"*. It is written as what the engine does, in one wording, on `01-objects.md` § No types and Q-M1f.372 (*the one place the engine itself reads which names are labels*), `reasoning/02-kb-evolution/04-hypothesis-generation.md` § 5, `06-solution-semantics.md` § 1.2, `ein.rs/03-design/07-search.md` § 1 and `ein.rs/08-reasoning-in-code/01-static-ir.md` | `a.ein`: `(relation is-instance Object Type :why "{?1} is a {?2}")` `(relation lives-in Person House :why "{?1} lives in {?2}")` `(is-instance Norwegian Person)` `(is-instance House-1 House)`. `b.ein`: the same with `(relation lives-in A B :why "{?1} lives in {?2}")`. `ein solve a.ein --count`: *universe 4 candidates*, `commitments (k_c) 16`. `ein solve b.ein --count`: *universe 22 candidates*, `commitments (k_c) 4 194 304`; `ein show hyps b.ein` lists `(lives-in Person Norwegian)` | **The user's call**: keep Q-M1e.12 and Q-M1f.306 (a label is a name not to guess about, a device of tractability), or make a label an atom like any other, which changes H and R of every program on the blind rung |
| L-62 | **Macro expansion is not hygienic**: a variable a macro's body writes and its parameters do not name is the same variable in every call of the macro in one rule, so two calls are joined on it. No decision says which is meant. The macros page (`11-macros.md`) shows a macro defined, called and expanded, and does not state a rule of capture | `(relation lives-in Person House :why "{?1} lives in {?2}")` `(relation linked Person Person :why "{?1} is linked to {?2}")` `(macro housemates (?a ?b) (and (lives-in ?a ?h) (lives-in ?b ?h) (neq ?a ?b)) :why "{?a} and {?b} share a house")` `(rule link () :match (and (housemates ?x ?y) (housemates ?y ?z) (neq ?x ?z)) :assert (linked ?x ?z) :why "{?x} and {?z} share a housemate")` `(lives-in Ann Elm) (lives-in Bob Elm) (lives-in Bob Oak) (lives-in Cy Oak)` `(solve :no-search)`. `ein saturate --dump FILE` prints `origin = DERIVED 0`: no `(linked Ann Cy)`, because both calls share `?h` | **The user's call**: rename a body's own variables per call (hygiene), or keep capture and state it as the rule, with a decision either way |
| L-19 | **A rule may conclude `(relation a)`**, which declares nothing: a rule can forge the membership fact that `(relation ?R)` in a `:match` reads as *declared relation*. The pages say what holds (`02-syntax.md`, `02-relations.md` and `03-rule-evolution/01-rules-as-data.md` state that a rule may derive the membership fact and that it declares nothing); the knowledge-base page's definition of a declared relation, read literally, makes `a` declared | `(relation p T :why "{?1} is p")` `(p a)` `(rule r () :match (p ?x) :assert (relation ?x) :why "{?x} is made a relation")` `(solve :no-search)`. `ein saturate --dump FILE` derives `(relation a :rule r)` | **The user's call**: whether a rule should be allowed to; refusing it at load is a language change. Either way the knowledge-base page's definition wants the word *written* |
| L-30 | **`ein_ir::load_file_with` reads with `fs::read_to_string`**, not `ein-cli`'s `common::read_program`, so a `.einb` or a non-UTF-8 input may be handled otherwise than by every command (the standing rule: every command reads a program by one path) | `grep -n 'read_to_string' ein.rs/crates/ein-ir/src/from_ir.rs` (line 123) | A probe first: if a command reaches it with a `.einb`, it is a wrong answer and a register row before any fix |

One question a closed finding leaves, nobody's: L-69 found that a region claim is a bijection between the models
it lists and R's states, and `10-satisfiability.md` § 3 says so; whether a claim over a *part* of the models
should be writable is [M1g](../m1g_logic/README.md)'s, if anyone's.

## The engine's renderers

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-35 | **Four renderings say less or other than the program**: `ein render program` draws a written `(not …)` fact as a dashed arrow with no ¬, as it draws a hypothetical edge; a declaration of three labels is drawn as an arrow between its first two; `ein render constraints` draws an import as a node; the region trace prints a given fact's re-derivation as a *Derives* step | `r.ein`: `(relation owns Person Pet :why "{?1} owns {?2}")` `(relation gives Person Person Pet :why "{?1} gives {?2} the {?3}")` `(owns Ann Rex)` `(not (owns Bob Rex))` `(gives Ann Bob Rex)` `(rule again () :match (owns ?p ?x) :assert (owns ?p ?x) :why "{?p} still owns {?x}")` `(solve :no-search)`. `ein render program r.ein` prints `"Bob" -> "Rex" [label="owns", … style=dashed]` and `"Person" -> "Person" [label="gives", style=dashed]`; `ein solve --trace t.md r.ein` writes *Step 1 — `again`* … *Derives `owns(Ann, Rex)`*; `ein render constraints examples/zebra2.ein` prints `"std.algebra" [shape=rectangle, label="std.algebra\\n«import»"]` | **The user's call**, with L-63 and L-76: a stage on the renderers. None misreports an answer |
| L-63 | **What `ein render` and its options show, beyond L-35**: (a) `ein render rule --name N FILE` finds only a rule FILE itself writes, so an imported rule is drawn only from its module's file; (b) no command prints what a macro's call expands to, and `ein render rule` draws a call of a program's macro as an arrow labelled with the macro's name, as if a relation; (c) `ein render rule` draws an `(absent P)` nested in an `(absent …)` exactly as it draws a `(not P)` there; (d) `ein render program` draws a declaration's labels as boxes only where no fact names them; (e) `ein render kb --no-types` and `--no-instances` drop the node lines and keep the edges; (f) `ein render constraints` writes a doubled backslash in a label, so a literal `\n` is drawn; (g) `ein render program` prints a fifth graph, `digraph solve`, whose note carries the loader's made-up name, `:no-search (#empty)`; (h) `ein render kb` draws no stored negation and no activator (`12-rendering.md` says so), and a nested fact as one box labelled with its text, which is [P1f.22](p1f.22_nested_facts/findings.md)'s N-3 | (a) `ein render rule --name total-owed examples/zebra2-obligations.ein`: *no rule named 'total-owed'*. (b) L-62's program: `ein render rule --name link FILE` prints `"?x_L" -> "?y_L" [label="housemates", …]`. (c) two files that differ in one premise, `(rule bag-is-ready () :match (and (is-a ?bag Bag) (absent (and (goes-in ?i ?bag) (absent (packed ?i))))) :assert (ready ?bag) :why "…")` and the same with `(not (packed ?i))`, over declared `is-a`, `goes-in`, `packed`, `ready`: `ein render rule --name bag-is-ready` writes the same bytes for both (`cmp`). (e) L-35's `r.ein` with `(relation is-a Thing Kind :why "…")` `(is-a Ann Person)`: `ein render kb --no-types r.ein` still prints `"Ann" -> "Person" [style=dashed, arrowhead=empty, label="is-a"]`. (f) `ein render constraints examples/zebra2.ein \| grep -c '\\\\n'` prints 10. (g) `ein render program examples/beliefs/01_common_belief.ein \| grep -A1 'digraph solve'`. (h) `r.ein` with `(doubts Dane (left-of House-1 House-9))`: `ein render kb r.ein` prints `"Dane" -> "(left-of House-1 House-9)" [label="doubts", …]` and no line for `(not (owns Bob Rex))`. (d) as the lane on the user's pages reported it, not re-run | With L-35: a stage on the renderers. (g) is a made-up name on a user's surface, which Q-M1f.339 rules out |
| L-76 | **No command draws the graph, and a fact's arrows are numbered three ways.** The knowledge base is a graph in which every name is one node, a relation among them, and a fact's first arrow points at its relation (Q-M1f.379 to .381). `ein render program --levi FILE` writes the relation's name in each fact's octagon and draws arrows to the arguments only, so `left-of` stands on as many nodes as it has facts and is no node; its help calls that *Levi-bipartite octagons*. `ein render kb` draws a fact of two arguments as a labelled arrow. Both number a fact's arrows by argument, from 1 (`label="1"`; `label="#1"`), where the graph's first arrow is the relation's; the guide's chapter 5 draws the graph by hand (`02-guide/diagrams/05-jack-levi.dot`) and numbers by argument too. `01-objects.md` and `12-rendering.md` say how each drawing stands to the graph; the engine is unchanged | L-35's `r.ein` with `(relation left-of House House :why "…")` `(left-of House-1 House-2)` `(left-of House-2 House-3)`: `ein render program --levi r.ein` prints `"h_3_left-of" [shape=octagon, label="left-of"]`, `"h_4_left-of" [shape=octagon, label="left-of"]`, `"h_3_left-of" -> "House-1" [label="1"]`; `ein render kb r.ein` prints `f_… -> "Ann" [label="#1" …]` for `(gives Ann Bob Rex)`; `ein render program --help \| grep levi` | **The user's call**: a view that draws the graph (a node a relation, the first arrow drawn), one numbering in every drawing, or the pages' account of the difference as it stands. With P1f.22's [Q-M1f.395](open_questions.md#q-m1f395--what-ein-render-draws-of-a-fact-under-an-arrow) |

## The engine's words against the definitions

What the binary prints, and the code's identifiers, are [F23](../followups/f23_diagnostics_in_eins_words.md#the-items)'s
D-20. Two rows are of the pages and the decisions.

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-80 | **Old words left on purpose, where a change would rename an id's file or break an anchor.** Decisions still say *name* for the node outside a sentence that classifies: the titles of Q-M1f.127 (*… can hold a name*) and Q-M1f.128 (*the negation of a name*). Q-M1f.372's title says *a declaration's slot names are labels*, where the pages say *label* alone. Headings of `docs/ein/ein.rs/03-design/` keep the code's words, because Rust doc comments and `08-reasoning-in-code/01-static-ir.md` link their anchors: *3. Values and fact ids*, *7. Per-relation extent counts* (`03-data-model.md`), *7. The value renderer* (`01-determinism-and-order.md`), *5.3 The extent*, *5.4 Extent counts* (`05-matcher.md`); each page says the mapping. `04-api.md`'s generated `rust` block says *symbols and values* in a comment, which is `embedding.rs`'s | `head -1 docs/decisions/language/Q-M1f.127-*.md docs/decisions/language/Q-M1f.128-*.md docs/decisions/language/Q-M1f.372-*.md`; `grep -n '^## 3\. Values\|^## 7\. The value\|^### 5\.3\|^### 5\.4\|^## 7\. Per-relation' docs/ein/ein.rs/03-design/*.md`; `grep -n 'symbols and values' docs/ein/ein.rs/04-api.md` | **The user's call**: retitle the three decisions (two files renamed, every reference with them) and the five headings (their anchors re-aimed in the crates' comments), or leave them |
| L-81 | **The user's pages keep three shorthands the decisions spell out.** *Closed relation* is a defined term on the user's pages (a relation a `(__closed__ R)` fact names; `09-hypotheses.md`, `03-reserved-names.md`, the glossary), where the decisions and the reasoning say *closes the held extent of R to hypotheses*. *Value* is kept for what an attribute takes in the `std.slots` pages and the Zebra guide, with one sentence saying it is an atom (`04-stdlib/05-slots.md`). `stdlib/README.md` says *kind* for an atom that membership facts point at, a word no definition has | `grep -rln 'closed relation' docs/ein/user \| wc -l` (3), against `grep -rn 'closed relation' docs/decisions \| wc -l` (0); `grep -n 'A \*\*value\*\*' docs/ein/user/04-reference/04-stdlib/05-slots.md`; `grep -n -w 'kind' stdlib/README.md` | **The user's call**: keep each as a defined shorthand, or spell it out everywhere |

## The reasoning pages: reading order and two-sense words

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-82 | **Notions used before the page that introduces them**, against the user's second notes (*"03-values-and-order.md already tell about rules, but rules introduced in 04"*), which the lanes carried out for `01-static-ir/` and for `01-firing.md` and not for the rest. Left: **monotone program**, used on `02-saturation`, `03-absent-and-naf` and `05-hypothesis-space`, defined in `06-solution-semantics` § 1.6 and `08-monotonicity`; the **derivation record** $\delta$ with `prem`, `all`, `abs`, used on `03` and `08`, defined on `09-cores` (a real fix moves it to saturation); **strata, the dependency graph, negative cycles**, used on `03` § Stratification and `08` § 8.2 to § 11.1, defined on `03-rule-evolution/02-analysis-of-rules`; **core**, used on `02`, `03`, `05`, `07` before `09`; **order-free at the boundary**, used in analysis-of-rules § 4, defined in `03-transformations` § 3; **query, goal, membership fact, module, import, `rows_g`**, used and defined on no page of the reasoning. **Words with two senses**: *candidate* (a firing with its disjunct $\kappa$ on `02`, a member of $H$ everywhere else), *closure* (the marker's, and $\operatorname{cl}(C)$), *open* (a fact's status, and `05`'s open candidate), *guard* (the absences of a disjunct on `02`, an obligation's pattern on `04`), *witness* (of an obligation, and of an explanation on `09`), *restriction* (`06`, `10`) for the rules page's *constraint*; the letters $X$ twice on `02` and $\beta$ twice on transformations | the report of the lane that put `reasoning/02-kb-evolution/` and `03-rule-evolution/` into the ruled words; read, each use a forward link today: `grep -n -i 'monotone' docs/ein/reasoning/02-kb-evolution/02-saturation.md docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md \| head` | **The user's call**: reorder (the derivation record to saturation, the strata before absence, `06` § 1 earlier), or keep the forward links; one word a sense. S1f.21.2 and S1f.22.2 write into these pages next. With L-90 |
| L-83 | **Small disagreements among the reasoning pages**, read and not run: `05-hypothesis-space.md` § The domain, row *the blind enumerator's universe*, says a label and a relation's name are among $\operatorname{names}(H)$, where `04-hypothesis-generation.md` § 5 says neither is a guessable atom; `04` § 3 filters $\operatorname{rel}(f) \notin \lbrace \operatorname{eq}, \operatorname{neq} \rbrace$ out of a set of facts, though no fact has those relations (Q-M1f.32 is about an hrule's instance); `08-monotonicity.md` counts a stored negation under a pseudo-relation $\lnot r$, so $\operatorname{rels}$, as $D$ uses it, is not literally the relations of the facts; `01-rules-as-data.md`'s example states $\operatorname{is-a}(\mathsf{Rule}, \mathsf{Object})$, a rule under `Object`, where the pages say a rule is no node (the prose says it is the program's own vocabulary) | `grep -n "blind enumerator's universe" docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md`; `sed -n 129,134p docs/ein/reasoning/02-kb-evolution/04-hypothesis-generation.md`; `grep -n 'pseudo-relation' docs/ein/reasoning/02-kb-evolution/08-monotonicity.md`; `grep -n 'mathsf{Rule}' docs/ein/reasoning/03-rule-evolution/01-rules-as-data.md` | Whoever next writes on those pages: S1f.21.2 or S1f.22.2 |
| L-75 | **The user's pages write a set of facts `S` and the reasoning writes a knowledge base `K`.** The reasoning has one notation (`reasoning/README.md` § Notation), and the user's pages have its spelling of the state of a commitment and of saturation (`st(C)`, `sat`); the letter of an arbitrary set of facts on the user's pages is still `S` (`ext_S(r)`, `dead(S)` defined through `sat(S)`), which the reasoning keeps for the state a saturation ends in | `grep -rn 'ext}_S\|inst}_S\|dead}(S)' docs/ein/user/03-syntax-and-semantics \| wc -l` (12) | **The user's call**: one letter in both perspectives, or the user's pages' lighter notation said to be its own |
| L-78 | **The definitions' diagram is crowded.** The class diagram of the definitions on the knowledge-base page (`01-static-ir/diagrams/01-definitions.puml`, which the user asked for: *"add identity graph, or definition graph into 'What is where', maybe like class diagram?"*) draws its associations' labels over one another where several meet one class | `docs/ein/reasoning/01-static-ir/diagrams/01-definitions.svg`, looked at; the source is 32 lines | **The user's call**: lay it out again (fewer labelled associations, or two diagrams), or accept it |
| L-68 | **One drawing of the monotonicity page cannot be re-drawn**: `diagrams/08-region-three-components.svg`, of `examples/lattice/synth/three_components.ein`, because `ein oracle` refuses the program (*forbid*), so the page keeps the drawing it had | `ein oracle examples/lattice/synth/three_components.ein`: *rule 'dead-3': concludes (false) from (absent …) over h, a relation a commitment can still extend …* | Whoever next touches `utils/region_synth.py`'s programs: a program the oracle answers, or the drawing dropped |

## The user's pages and the standard library's

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-64 | **The example outputs of the standard library's usage pages are re-run by nothing.** 118 blocks of what `ein` prints, on the usage pages of `04-stdlib/`; the generator (`utils/gen_stdlib_rule_pages.py`) holds the rule regions alone. The lane that wrote the pages built them from templates with a script; what it left is banked in [`stdlib_usage_builder/`](stdlib_usage_builder/): its 95 programs (`ex/`), the script (`build.py`, kept for what its directives mean: it does not run without the templates) and, per page, the command and filters behind each program and output block in order (`directives.txt`: `@@ein FILE` a program, `@@run ARGS :: FILTER…` an output, `@@dot` a drawing, `repo:` a file of the repository). The pages were edited after they were built, so the templates' prose is not kept | `grep -c '<!-- output -->' docs/ein/user/04-reference/04-stdlib/*.md`, the outputs a page; no test runs those pages' programs: `grep -rn '04-stdlib' ein.rs/crates/*/tests` finds links in comments only | **The user's call**: a generator beside `read_out_examples.py` that writes the outputs from the binary and joins `check_generated.py`, or a test that runs each page's programs as `guide_programs.rs` does the guide's |
| L-77 | **Thirty-six rule drawings of the standard library are wider than 2000 px**, a strip when fitted to a page, the nine `div9-step` rules of `std.math.num9` the widest (11 051 pt): the engine lays a rule's premises in one row. Not changed: the source is the engine's (`ein render rules`), and which rules are drawn is Q-M1f.375's (a rule whose drawing has more than 120 nodes is shown as Ein alone) | `python3 -c "import re,glob; print(sum(1 for f in glob.glob('docs/ein/user/04-reference/04-stdlib/diagrams/*.svg') if (m:=re.search(r'<svg width=\"(\d+)pt\"', open(f).read(600))) and int(m.group(1))*4/3 > 2000))"` prints 36 | **The user's call**: lower the threshold above which a rule is shown as Ein alone, or lay a rule's premises in rows in the engine's drawing (which moves every rule drawing and its goldens) |
| L-79 | **The grammar page shows the grammar as pictures only.** `04-reference/01-grammar.md`'s two generated regions are nineteen railroad diagrams; the EBNF text each is drawn from is the linked `.puml` source under `04-reference/diagrams/`, and no page holds the productions as text a reader can search or copy | `grep -c '^\x60\x60\x60' docs/ein/user/04-reference/01-grammar.md` (0 fenced blocks); `grep -c 'puml' docs/ein/user/04-reference/01-grammar.md` | **The user's call**: the picture alone (the ruling's letter: a diagram is a file, and the page shows its picture), or the EBNF as text beside it, which the generator in `ein-ir/tests/grammar_pages.rs` can write |
| L-72 | **`stdlib/README.md` says `std.slots` *needs `forall`, `symmetric` and `symmetric-negative-setup`***, as if a program had to list the last: the symmetric mirror of a stored negative needs no listing by the program. The reference's page no longer says it | `grep -o 'Needs .forall.[^\|]*' stdlib/README.md`; four of the programs under `tests/stdlib/slots/` list no `symmetric-negative-setup`: `grep -L symmetric-negative-setup tests/stdlib/slots/*.ein \| wc -l` | A pass over `stdlib/README.md` against the import model as it is; quick |

## Weak or missing tests

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-22 | **Two claims are held thinly**: *exactly one of `p`, `q` is derived* on `examples/features/07_unstratifiable.ein` is held only by the `corpus_shapes.md5` digest; *the cube checks the static check* only by `monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged` (six fixtures) and the oracle-equality tests | `grep -rn 'monotone_programs_show_no_violation' ein.rs/crates/ein-infer/tests` | **The user's call**: a test each, if wanted |
| L-42 | **Tests weaker than they read**: `ein-ir/src/stdlib.rs`' `the_embedded_copy_matches_the_manifest` asserts `checked >= 7`, with 24 modules today; `ein-cli/tests/primitive_arity.rs` never sweeps `false` | `grep -n 'checked >= ' ein.rs/crates/ein-ir/src/stdlib.rs`; `find stdlib -name '*.ein' \| wc -l` | **The user's call**: a small code stage |
| L-66 | **The diagrams of `04-api.md` and `05-events.md` are pinned by no test**: the sequence of an embedding and the order of a run's events are drawn by hand, and the events page's example program stands only on the page. `embedding.rs` holds the `rust` block and the prose region; `events_reference.rs` holds the kinds and their fields | `grep -n 'diagrams/' docs/ein/ein.rs/04-api.md docs/ein/ein.rs/05-events.md`; neither test reads a diagram's source | **The user's call**: hold the events page's example to the binary (its program in the corpus, its stream compared), or leave the diagrams as explanation |
| L-74 | **The running example of the static-IR pages is no corpus file, and is not monotone**: the three-house program stands as source only on `08-reasoning-in-code/01-static-ir.md`, and no test loads it; its drawings on the reasoning's pages are the binary's for that source | `sed -n '/^## The running example/,/^## The knowledge base/p' docs/ein/ein.rs/08-reasoning-in-code/01-static-ir.md` | **The user's call**: make it a corpus program, or keep it as the page's own |
| L-84 | **What the review's lanes did not read or check**, each as its lane reported it. The claims of the design pages (`ein.rs/03-design/`) were checked against the code only where a new diagram draws them. The reference pages `02-syntax`, `05-cli`, `06-config`, `07-diagnostics`, `08-read-out`, `10-answer-dump` and `11-region-dump` were searched for the old words and the forbidden shapes and not read line by line; the Zebra walkthrough has no graph. The lane on `reasoning/02-kb-evolution/` ran as seven parts and its diffs were spot-read, not read whole. The mathematics new on the reasoning pages is held by no test and by no second reader: *settled by the closure* and $D^{+}$ as a least fixpoint on `03-absent-and-naf.md`, and Proposition 2 of `03-transformations.md`, argued in full only without absence | the lanes' reports; nothing to run | **The user's call**: a second read of those pages, by the user or by a lane with nothing else to do |
| L-86 | **The witnesses that moved to the process leave the project's sweeps.** Thirteen programs under `plans/wrong_answers/witnesses/` are held by the register's recorded run and region alone; the corpus sweeps (oracle agreement, the routes, the claims, the agenda, import invariance and the rest) no longer read them. Five witnesses of open rows stayed in the project because a test or a page reads each for a claim of its own | `ls plans/wrong_answers/witnesses \| wc -l`; `python3 utils/wrong_answers.py` | **The user's call**, among the defaults of the register's split below: nothing, or a process step that runs the sweeps' properties over the witnesses too |

## Dead code, and comments

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-41 | **Code nothing runs** (with L-11 and L-26): `Kb::rebuild_closures` has no caller; `Kb::flatten` only a test's; `Kb::cites_fork_provenance` and `ProvArena::promoted()` no caller outside their own crate's comments, the test that held their claim deleted with the walk; `EqClasses` in `ein-core/src/kb.rs` (L-11: the store's equality classes exist, nothing writes them, no matcher reads them, and the KB copies them on every fork; `03-design/03-data-model.md` § 10 says so); `ein-narration`'s `NAMES` table and `events.rs`' header describe the walk's surfaces, and it relaxes on an event kind no emitter writes, an `enter` of kind `dead-post` (L-26) | `git grep -n 'rebuild_closures\|cites_fork_provenance\|promoted()' -- ein.rs/crates`; `grep -rn 'EqClasses\|eq_classes' ein.rs/crates --include='*.rs' \| grep -v '^ein.rs/crates/ein-core/src/kb.rs'`; `grep -rn dead-post ein.rs/crates` matches only `ein-narration` | **The user's call**: delete each, or give it a test |
| L-45 | **Measured numbers in comments with no command or test behind them** (with L-65): `ein-corpus/src/pool.rs`' `HEAVY` figures (*`send_more_money` 529 MB*, *`04_open` 266 MB*), `manifest.rs` (*at `-T 0.001`*), `ein-render/src/grid.rs` (*`zebra2`'s unique model is 444 facts*) | `grep -n 'MB' ein.rs/crates/ein-corpus/src/pool.rs \| head`; `grep -n '444 facts' ein.rs/crates/ein-render/src/grid.rs`; `grep -rn 'T 0.001' ein.rs/crates` | **The user's call**, with L-65: Q-M1e.4 and Q-M1f.349 say it of a page; whether a comment is held to the same |
| L-65 | **Crate comments carry history and measurements** (with L-45): `saturator.rs`' `Snapshot` doc (*94.6 % of a fork's firings*, on a command line that is gone, `-e`), and `try_commitment_set` and the kill cache it speaks of; `obligations.rs` (*`:priority` keeps one residual meaning*, *150 of the corpus's 173 files*); `hypgen.rs` (*an eight-stage filter pipeline*, where the pipeline has three filters; *~18 k raw candidates per call on zebra2*; *10.7 % of solve*); `states.rs` (*zebra.ein has 270 454 alive commitments*) | `grep -n 'eight-stage\|18 k\|94.6\|150 of the\|residual meaning\|270 454\|10.7 %' ein.rs/crates/ein-infer/src/*.rs` (seven lines) | **The user's call**, with L-45: a comment states the current state, and a number carries its command or goes |
| L-47 | **Long runs of empty `;;;` lines** kept to hold a line count after a header shrank: `examples/ein-bugs/closed-and-owing.ein` (44), `examples/branching/13_lookahead_naf_world.ein` (14), `14_lookahead_unjudgeable.ein` (18), `examples/ein-bugs/binding-key-both.ein` | `grep -c '^;;;$' examples/ein-bugs/closed-and-owing.ein` | Cosmetic: the cost of the rule that a comment edit moves no recorded position. Nothing to do unless that rule changes |

## Examples

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-44 | **Three pairs of identical programs named for retired levers**: `examples/branching/06_lookahead_on` and `07_lookahead_off`, `10_kill_cache_on` and `11_kill_cache_off`, `15_lookahead_two_step_on` and `16_lookahead_two_step_off` differ only in comments | `diff <(grep -v '^\s*;' examples/branching/06_lookahead_on.ein) <(grep -v '^\s*;' examples/branching/07_lookahead_off.ein)` prints nothing, as for the other two pairs | **The user's call**: keep one of each pair |
| L-49 | **Witness headers narrate their defect's history by commit hash**: 293 lines in 264 files under `examples/ein-bugs/` (*"At `6a775f3d`, …"*); no check reads a hash, and a backticked 7 to 12 hex pattern would catch exactly these | ``git grep -cE '`[0-9a-f]{7,12}`' examples/ein-bugs \| wc -l`` (264); ``git grep -hE '`[0-9a-f]{7,12}`' examples/ein-bugs \| wc -l`` (293) | **The user's call**: restate each as what the regression holds (line counts kept), and add the pattern to the voice check |

## The release voice where the check does not read, and the gate

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-2 | **The release-voice check reads comments and pages, and not a string**: a string literal of a `.rs` file, which the binary or a test prints, and the strings of a `.ein` file (`:why`, `:source`, `:goal-text`). The phase swept both by hand (its L-12, L-37): no `.ein` string carries an id or a date today and the binary prints none; L-54's test messages are what is left. One written later would pass | `grep -rnE ':(why\|source\|goal-text) +"[^"]*(20[0-9]{2}-[0-9]{2}\|M1\|S1[a-z]*\.)' --include='*.ein' stdlib examples tests` prints nothing | **The user's call**, with L-49, L-54 and L-85: whether the check reads strings, commit hashes and case ids |
| L-54 | **Test messages name the process**, in strings: `ein-cli/tests/guide_programs.rs` asserts with *"chapter 4's block, the one finding 6 was about"*; `ein-ir/tests/digit_tables.rs` says *S1f.6.2's table*. And `ein-cli/tests/config_reference.rs`' module doc says *the 17 `(config …)` flags*, where there are 2 flags and 15 retired names | `grep -n 'finding 6' ein.rs/crates/ein-cli/tests/guide_programs.rs`; `grep -n 'S1f.6.2' ein.rs/crates/ein-ir/tests/digit_tables.rs`; `sed -n 3p ein.rs/crates/ein-cli/tests/config_reference.rs` | Reword each to what the thing is; quick |
| L-85 | **Bare case ids of the wrong-answer register remain in the project**: `W62`, `C1` and the like, none with a slug, in 694 lines of 122 files. A case is several rows, so no map renames them, and the check does not read them | `git grep -cE '\b[WC][0-9]{1,3}\b' -- ein.rs stdlib examples tests corpus docs README.md ':!docs/lib' \| wc -l` (122 files); the same with `-h` in place of `-c` (694 lines) | **The user's call**: a pass that restates each as what the regression holds ([`fixed_rows.tsv`](../wrong_answers/fixed_rows.tsv) maps a fixed row's id to its regression's name), with the pattern added to the check |
| L-50 | **Census instruments ship in a crate's examples**: `ein-infer/examples/site_census.rs` and `refutation_census.rs` among ten `*_census.rs`, whose output a census under `utils/censuses/` reads | `ls ein.rs/crates/ein-infer/examples/*census*` | **The user's call**: an example is the project's, the census the process's |
| L-51 | **`utils/ci_container.sh --dev-host` cannot run the dev host's PlantUML step**: `run_tests.sh --dev-host` runs `check_doc_blocks.py --plantuml`, and the runner-like image installs no PlantUML that knows `@startebnf`, so the step exits 127 there; on the host it runs. `check_gate_steps.py`'s `DEV_HOST_ONLY` says why the step is the dev host's | `grep -n plantuml utils/ci_container.sh run_tests.sh` | Install such a PlantUML in the image (not Ubuntu 26.04's package), or state in `ci_container.sh` that the step is the host's alone |
| L-89 | **Two process tools still count a tree that is gone**: `utils/doc_audit.py --inbound` prints a `hist` column and its docstring speaks of `docs/history/`; `utils/history_census.py` keeps a `docs/history` row, now 0 | `grep -n 'docs/history' utils/doc_audit.py utils/history_census.py` | Drop the column and the row; quick |

## Decisions

| id | what | witness, and the command | the call |
|---|---|---|---|
| L-21 | **No decision states the static check's grain** (the relation, not the fact). `08-monotonicity.md` § 3.3 states it | `grep -rl grain docs/decisions` prints nothing | **The user's call**: a decision file, if one is wanted |
| L-90 | **Two rules of the user's second notes on the reasoning are carried out on the pages and stated in no decision**: that no page uses a notion before the page that introduces it (*"03-values-and-order.md already tell about rules, but rules introduced in 04-rules-as-statements.md"*; *"already talks about candidates, but hypothesis are introdyced in 02-kb-evolution"*), and that `01-static-ir/` speaks of no firing, no growth and no candidate (*"we don't talk here about rules firings; e.g. 'state only grows' belongs to 02-kb-evolution"*), starting from the graph and then naming what is where (*"first - we do graph rewriting and then how it is declared"*). The decisions of the documentation (Q-M1f.367 to .371) do not say either, and L-82 is where the first is not yet met | `grep -rln 'introduces\|before the page' docs/decisions/docs` prints nothing | **The user's call**: a decision under `docs/decisions/docs/` (the next free id is Q-M1f.397), proposed and not written: *a page of the reasoning uses no notion before the page that introduces it, and the first part states the knowledge base and its rules as they stand, with no firing* |

## Defaults the documentation tree took

Choices a stage or a lane of P1f.20 made that the user did not rule on, each with where it shows. A default
stands until the user revises it. What the user did rule is a decision (Q-M1f.341 to .349, .367 to .372, .375,
.378 to .382) or a rule of `AGENTS.md`, and is not listed.

**The tree and its names**

- The order of the numbering is the order each folder's README listed its pages in; `user/` reads `01-install.md`,
  `02-guide/`, `03-syntax-and-semantics/`, `04-reference/` (Q-M1f.367 states the rule, not the order).
- `standard-of-proof.md` is dissolved: Q-M1e.1, Q-M1e.2 and Q-M1e.4 carry its rules, and its table of the four
  premises the answer rests on is `ein.rs/03-design/README.md` § What the answer rests on. The user had asked
  *"what is this? haven't decided where it goes?"*.
- What the reasoning shed (the code map, the tests, the commands, the programs as source) is a folder of the
  engine's pages, `ein.rs/08-reasoning-in-code/`, one page a part of the reasoning; `h-space.json` is described
  there alone, and no page of `user/04-reference/` has it.
- The standard library's reference is an index and nineteen pages, a page a family of modules, with *Using …*
  headings, the outputs excerpted with `…`, a rule and its drawing side by side, a rule whose drawing has more
  than 120 nodes shown as Ein alone (seven), and a table's size written as a product where no command re-takes
  it. A new rule fails `check_generated.py` until a page shows it.
- `ein.rs/` draws sequences, activities, state machines and classes in PlantUML and structure in Graphviz, with
  pseudocode in `text` blocks; `02-algorithms.md` tells each algorithm once. The plan of a comparison with other
  systems left `overview/README.md`, which says what the folder holds.
- The diagrams' files are named `NN-name` for what each shows; `utils/render_diagrams.py --check` draws nothing
  and holds each picture to the stamp of its source.
- The grammar page: `regex-syntax` is a dev-dependency of `ein-ir`; four rule names are the page's own; the
  grouping of the productions is a table in `grammar_pages.rs`.

**The user's pages**

- The four representations' headings and order: *In Ein*, *As a graph*, *In words*, *In mathematics*; one running
  example (two people, two chairs) across obligations, hypotheses, query and solutions.
- `01-objects.md` is titled *Objects: atoms and facts*, uses `(relation is-instance Object Type)` as its example,
  draws the graph by hand (`01-the-graph`) before the engine's two drawings, and states L-57 as what the engine
  does. The guide's chapter 1 has sections *Atoms — the things* and *Facts — a relation and its arguments*.
- The diagnostics page's § 2.22 is titled *An object is an atom, or a fact a state could hold*; it numbers the old
  § 4 as § 2. `05-cli.md` numbers its sections as the old interface page did. Patterns are `02-syntax.md` § 3 and
  no page.
- The glossary dropped the walk's terms; *Value* is a redirect. The natural-language page has no graph.
- `08-word-problems.md` carries a cut-down copy of a word-problem theory and one example program whole, so that
  its blocks run under the guide's test; they can drift from the files they copy.

**The reasoning**

- The objects are $O$ (not $V$), so the observation of `01-firing.md` is $\mathrm{Obs}$; the guessable atoms are
  $\operatorname{guessable}(P)$, written with $\operatorname{rels}(\mathrm{root})$; `05-hypothesis-space.md` says
  *part* where it said *object*; *atom* is never an atomic formula.
- A held fact is a filled octagon and one only pointed at an empty one; a fact's node carries a reference letter
  ($f_1$) the page says is no label; the four levels stay on the rules' page as parts an atom plays; the quoted
  form of an object stays on the knowledge-base page.
- A formula breaks by its rendered width, not its source length. `01-firing.md` § Two layers introduces root, a
  candidate, a commitment, a node and its state; the obligation's home is `04-hypothesis-generation.md` § 4.1;
  *eligible relation* is what the blind enumerator fills, and `10-satisfiability.md` § 5 says *closed to
  derivation*; `05` has the symbols *given* and *clue*.
- `06-solution-semantics.md` keeps its numbered sections; `08-monotonicity.md`'s causes table dropped the old c11
  and renumbered; `03-transformations.md` states propositions, as far as today holds; the engine's drawings on
  the reasoning pages carry facts as the engine labels them, and corpus programs appear as shapes.
- Plans that cite a dropped section of the monotonicity page cite it as text at a commit, not as a link.

**The decisions**

- *Closes the held extent of R* for *closes R*, titles among them (Q-M1c.2, Q-M1f.244, .265, .365); *the goal has
  no row* and *a state holds no fact of a relation* for an empty extent (Q-M1e.13, Q-M1c.6); *a member of a
  claim's class* (Q-M1f.122); *object* never said loosely of a thing (Q-M1e2.27's title); Q-M1f.369 also states
  one statement a row.
- Fifteen decisions' files were renamed for their new titles, every reference with them.

**The process**

- The register's split is the coordinator's reading of two rulings: an open row and its witness are the
  process's, a fixed row a regression of the project named `regressions`, carrying no account of its right answer
  beyond its title and decision ids. Five witnesses of open rows stay in the project, each read by a test or a
  page for a claim of its own. `fixed_rows.tsv` stays for the ids the project still cites. `branching/06`'s
  not-wrong row is a regression. The review bundle reads the open register alone.
- A crate test that read a process file became a `utils/` check and a gate step; the runner of the held answers
  is an `ein-cli` example; `region.py` and `region_synth.py` are read as the oracle's second route.
- `utils/structure_bundle.py` and `utils/decision_census.py` were retired, not re-pointed; F21's probes are a
  folder beside its page; the release workflow's notes quote no figure for the musl build, and the workflows'
  headers cite no page; `rust-toolchain.toml` keeps *MSRV = stable-2* with no page behind it; `docs/lib/03` was
  touched for one link.
- The link check reads every comment line of a Rust file and every `.ein` comment, a bare path only with an
  extension or a trailing `/`, and neither a git-ignored target nor `ein.rs/vendor/`.
