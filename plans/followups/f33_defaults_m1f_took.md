# F33 — The defaults M1f's phases took that no question carries

**Filed 2026-11-25**, at the close of M1f, on the user's word of 2026-11-25, *"after p1f.22 closed, hand over all
remain open findings to followups and then close M1f"* (P1f.21, the milestone's last phase, closed that day). The
milestone's findings page kept, beside its findings, the choices three of its phases' stages made that the user never
ruled on one by one and that no question of the milestone's ledger carries, so that they were not lost with the stage
records. They are here, as the page had them; the page went with the milestone's folder (`git log --diff-filter=D --
plans/m1f_hypothesis_and_documentation`).

**What it owns.** Three lists of defaults: P1f.20's (the documentation tree), P1f.22's (nested facts) and P1f.23's
(the review's rulings, built). **A default stands until the user revises it**: none is a finding, none is a question
waiting, and nothing is to be built for any. What the user ruled is a decision under
[`docs/decisions/`](../../docs/decisions/README.md) or a rule of [`AGENTS.md`](../../AGENTS.md), and is not listed;
the readings those phases put as questions, each with its default, are decisions too (a file of their own, or
another's that carries them), the root ledger's where they rule the process, or were spent with the milestone's
ledger, which said which in each row (the ledger's close, the same day).

**A question a row names.** Its id is a decision under [`docs/decisions/`](../../docs/decisions/README.md) or a
section of the root ledger, [`plans/open_questions.md`](../open_questions.md); one with neither is carried by another
decision's file or was spent, as its row of the milestone's ledger says, in the parent of the commit that deleted it
(`git log --diff-filter=D -- plans/m1f_hypothesis_and_documentation`).

**Trigger.** The user revising one; a stage that changes what a default describes, which then names it.

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

## Defaults nested facts took

Choices a stage of P1f.22 made that the user did not rule on and that no question of the ledger carries, each with
where it shows. A default stands until the user revises it. What the user ruled is a decision (Q-M1f.391 to .398,
.400 to .404); the readings the stages put as questions with a default are the ledger's Q-M1f.405 to .411 and .413
to .418.

**The pages**
- The knowledge-base page writes the context of a fact $\operatorname{ctx}_K$, in its notation table, and has the
  sections *The global context, and the context of a fact* and *A context gives a fact no truth*; the rules page
  *Packing and unpacking*; the user's pages *A fact inside a fact* (`03-facts.md`) and *Rules and contexts*
  (`05-rules.md`), and the glossary *Held, global context, context*. The explanation is told on the user's example
  beside the page's `says` (Q-M1f.396's default for the example).
- The refusal *a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation*
  is as it was: it reads true of a negation of a negation.

**The drawings**
- A digraph that holds a frame sets `newrank=true` (`dot_util::FRAMED_RANKING`), in the KB view, the program view
  and the constraint view, and only where a frame is left: a view whose every fact under an arrow is held has no
  frame and ranks as before. A rule's drawing does not set it.
- In the rule view, a literal of the conclusion that is no premise loses its fill and its frame and keeps its dashed
  outline: in the overlay it is the one mark of what the conclusion adds.
- The filling went from every hand-written drawing that had it, framed or not, so that no page fills a held
  octagon beside one that does not: 20 sources.
- The extent's $g$, no fact of any knowledge base, is drawn grey (L-107).
- The views of a derivation (the slice, the derivation graph, a written trace's graph) are as they were (L-99).

**The saved file**
- Its first line is one, `;;; ein-saved/1 — the program and every module it imports, each as written, and the
  choices made: this file reads no other.`, where the format's proposal had two.

**The corpus**
- `examples/beliefs/06_double_negation.ein` declares `=`, which it writes only under negations and in an activator,
  so that the invariant's sweep passes it (V-7 of P1f.21).

## Defaults the review's rulings took

Choices a stage of P1f.23 made that no question of the ledger carries, each with where it shows. Every one is in
what the user ruled on 2026-11-19, *"Go with all defaults to the end of this phase p1f.23 and stop"*, and stands until
the user revises it. The readings P1f.23's stages put as questions are the ledger's (Q-M1f.421 to .548); these are
S1f.23.7's build's, which its Record kept and the ledger does not.

**The slice of a knowledge base** (S1f.23.7's build, `519fadd2`; Q-M1f.523 to .532)
- *A relation of the program*, for the slice, is the whole list a goal and a claim are held to: the kernel's words
  the program writes and an activator's relations among it, so Q-M1f.524's *one notion with the goal's and the
  claim's* is one list, kept on the program (`relations_of`). The alternative, the registry with `eq` and `neq`, would
  drop the corpus's one `__closed__` fact the view keeps.
- The decisions: a file each for Q-M1f.523, .527, .529, .530, .531 and .532; .524 to .526 are parts of .523's
  definition and .528 of .527's drawing, each carried by that file.
- `ein render constraints` of a saved file draws its program's root, its `(commit …)` not committed; a root that
  does not saturate is refused with `ein saturate`'s line, exit 1.
- `--goal` is read as `(query :goal GOAL)` under the source name `--goal`, so a position counts the wrapper's 13
  columns; a refusal is the loader's line after `kb load error: `, a parse error the parser's, a goal the compiler
  refuses its line, exit 1; nothing is drawn on any refusal.
- A goal's slice is the digraph `goal`, the constraint view keeps `constraints`, and an empty slice says `// the
  slice keeps no fact`.
- `--query N` counts from 1, as `query i of n`; a query the program has not is exit 1 with one line; `--query 0`, and
  `--goal` beside `--query`, are usage errors, exit 2.
- The facts that answer a goal are its matches' premises: a stored negation a `(not …)` pattern matches is kept, and
  a check keeps none.
- `import_facts.rs` runs `render constraints` among the commands that load, since it loads now; `dot_wellformed`
  skips the two constraint views whose root does not saturate.
- A claim may list a fact of a kernel word a goal reads, `(relation next-to)`, so that an `(or …)` goal's class
  pasted back as `:expect` loads (R-39); the claim's relation check takes the kernel's words as the goal's walk does,
  as Q-M1f.119 states. This resolved R-43.
