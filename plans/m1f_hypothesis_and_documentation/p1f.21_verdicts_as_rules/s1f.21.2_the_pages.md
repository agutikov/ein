# S1f.21.2 — the pages of the first part: *dead*, the contradiction, the key

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned with the phase (2026-11-17), its plan written 2026-11-19; **cut in two the same day** on the
user's word (*"leave open facts and hypotheses to the second part of the phase and finishe everything else
before"*): this stage keeps the pages of the first part, and
[S1f.21.14](s1f.21.14_the_pages_of_open.md) has those of `open` and the hypotheses. **Done 2026-11-20** ([§ Record](#record)).
**Group:** [B](modifications.md#b-the-pages-todays-kernel-stated-as-rules), items 6, 7, 9 and 12, and item 10's
half on the contradiction.
**Waits on:** the builds of the first part, [S1f.21.8](s1f.21.8_rules_the_standard_library_states.md),
[S1f.21.10](s1f.21.10_the_kernels_prefix.md) and [S1f.21.11](s1f.21.11_kill_and_three_conventions.md). A page
states the kernel as built
([Q-M1f.341](../../../docs/decisions/docs/Q-M1f.341-the-project-documents-only-its-current-state.md)), so it is the
last stage of the first part.
**Writes:** pages of `docs/ein/reasoning/` and `docs/ein/user/`, their diagrams, the glossary, and the decisions
of the first part's rulings under `docs/decisions/`.

## What it does

| item | page | what |
|---|---|---|
| 6 | `09-cores.md` § A contradiction, a pointer from `01-firing.md` | the contradiction rule: the schema, its pattern graph, why it is a schema, the rules the standard library states for it, and that the kernel judges no pair by itself |
| 7 | `06-solution-semantics.md` § 1.5 | *dead* as a rule's doing: a state is dead where a rule that states `:ein inconsistent` fired; *alive* as its absence, and why that is read on a state and never inherited |
| 9 | `03-absent-and-naf.md` § Stratification | where the rule that ends a state stands among a program's rules |
| 10, its first half | the user's `06-constraints.md` and `05-rules.md` | the same in Ein, each a program with its answer: the contradiction rule for one relation and for one count; a rule that states the key |
| 12 | the pages that introduce `(false)` | that it is a proposition of the standard library's and no truth value, what infers it and what reads it |

And, with them:

- **The key on the pages that state a rule's form**: `01-static-ir/02-rewriting-rules.md` (a conclusion is facts, or
  a word addressed to the engine), the user's `05-rules.md`, `02-syntax.md` and the grammar page, the glossary's
  *dead*, *inconsistent*, *State*.
- **`std.core`**, the module every program gets, where the standard library's pages and the page on imports say what
  a program has without importing it.
- **Each ruling of the first part as a decision**, one file under its id: Q-M1f.399, Q-M1f.550, Q-M1f.551,
  Q-M1f.447 (the count check dropped, a relation overloaded by its count), Q-M1f.557 (the contradiction a module a
  program imports), Q-M1f.559, Q-M1f.563 and Q-M1f.565 where the stage that builds one has not written it, and the
  readings the stages took; a decision the part amends
  (Q-M1f.313, the agenda; Q-M1f.436, the kernel reads no theory's name) is rewritten in place.
- **The sentence S1f.21.6 left for the pages**: `06-zebra-walkthrough.md`'s *a candidate is dead alone when assuming
  it … saturates to `(false)`*.

## What must hold

- Each perspective in its languages: `reasoning/` has no Ein source, command or file path, so a rule there is its
  formula and its graph, and the Ein is the user's page.
- A diagram is a file with its picture, a formula is TeX, a folder's numbering has no gap.
- Every command a page shows is run, and its output is the page's.
- `utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`, `utils/render_diagrams.py --check`
  and `utils/check_release_voice.py` pass.

## Acceptance

Each item is on its page; `docs/ein/` says nothing of the first part that the engine no longer does (searched for
by the old words: the kernel's reading of `(false)` and of `not`, *a rule concludes a conjunction of literals* with
no key); the checks above.

## Record

Run 2026-11-20 in a worktree, its branch fast-forwarded to `master` at `621c2ab1` first. No engine code, no stdlib
rule and no corpus program was changed; the binary of the worktree (`cargo build --release -p ein-cli`) ran every
command a changed page shows. The shared plan files (the ledger, the phase's README, `overview.md`, `findings.md`,
`review/`) and `AGENTS.md` were not edited.

### The commits

| commit | what |
|---|---|
| `c4f84f83` | the reasoning pages and the decisions |
| `8a86ac68` | the user's pages, the standard library's pages, the module tables, the reasoning-in-code pages |
| `e3a98135` | the pages that still read a stored negation as a denial by itself |
| `e2f68bad` | a fact of more than six arguments has no activator to reach it; the pattern's shapes are the language's; the semantics index |
| this one | the record, before its gate paragraph |

### Each row of the plan

- **Item 6, the contradiction rule** — [`09-cores.md` § A contradiction](../../../docs/ein/reasoning/02-kb-evolution/09-cores.md#a-contradiction):
  dead as a match of a rule that states the engine's word, recorded and writing no fact; *The contradiction rule, and
  why it is a schema*: $f \land \lnot f \implies \bot$ drawn as a pattern graph (`diagrams/09-contradiction-rule.dot`,
  the variable $f$ a filled diamond that is a premise), why no rule is written so (no premise is a variable for a
  whole fact; it would range over every relation and count), the schema one rule a relation and count, the
  library's `clash_n` and the rule that concludes its activator from a declaration, drawn for $n = 2$
  (`diagrams/09-contradiction-rule-per-count.dot`), that a program takes the fourteen rules or not, and *The kernel
  judges no pair by itself*, with the formula of a pair that is alive where no rule reads it; *The contradictions of a
  world, as sets of facts* keeps `contr(K)`. The pointer from
  [`01-firing.md`](../../../docs/ein/reasoning/02-kb-evolution/01-firing.md#firing-a-rule): a firing of a rule that states the
  word adds nothing and its knowledge base is dead, the one way one becomes dead.
- **Item 7, dead and alive** — [`06-solution-semantics.md` § 1.5](../../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#15-commitment-node-state):
  *Dead is a rule's doing, and alive is its absence*: dead where a rule of $P_{\mathsf{inc}}$ was applied in the
  saturation, the record a fork inherits, upward-closed on a monotone program; alive as the formula of no match,
  read on the finished state; *Why alive is read on a state, and never inherited*: a fact *alive* would be stale in a
  fork that dies, and as a rule from the absence of $\bot$ it is the shape the static check finds (run: a program
  with `alive-from-no-false` is flagged, one site, and its dead node holds the stale `(alive)`).
- **Item 9, the stratum of the end** — [`03-absent-and-naf.md` § Stratification](../../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md#stratification):
  *The end of a state stands last*: the end a node with edges in and none out, $\sigma(\mathrm{end}) = 1 + \max
  \sigma$, a program stratified with its ending rules exactly when without them, the strata drawn
  (`diagrams/03-the-end-stands-last.dot`), alive read above the last stratum, and why the end may still fire first.
  The same page's *A refutation resting on an absent* reads a refutation as the static check does,
  $\operatorname{refutes}(\rho)$, where it read $\bot$ or a stored negation in the conclusion.
- **Item 10, its first half** — [`06-constraints.md`](../../../docs/ein/user/03-syntax-and-semantics/06-constraints.md):
  the page's program gains `(import std.contradiction)` (its answer rested on the negation as a denial: without it
  `ein show hyps` lists three candidates and R has a third state, run), and a new *The contradiction is a rule*: the
  rule over a whole fact, refused at the parse (its output whole), the rule for one relation, `in-and-not`, and for
  one count, `clash` with `(clash in)`, each run with what `ein solve` prints, then `std.contradiction` doing the
  same with no rule written. The trace excerpt reads as the trace prints it (`<summary>`). *The negation of a negation*
  says `(not P)` beside `(not (not P))` is a contradiction where the contradiction reaches `not`.
  [`05-rules.md`](../../../docs/ein/user/03-syntax-and-semantics/05-rules.md): *Ending a state: `:ein inconsistent`*,
  a program (`one-king` over two given kings), what `ein solve` prints, the trace's step, and `(false)` as the usual
  way to the word (the same program with `:assert (false)` answers alike, one step more, run); the order rules fire
  in and the refusals as built (an activator that binds a relation parameter to a fact is a warning, Q-M1f.1).
- **Item 12, `(false)`** — the knowledge base's *Falsity* (a proposition and no truth value, read by a rule) and *A
  contradiction* (what concludes $\bot$ and what reads it); `06-constraints.md`'s *Two shapes*;
  `04-ontology.md`'s table, where `not` and `false` are the library's names a program may declare;
  the glossary's new *`(false)`, stored negation, the contradiction*.
- **The key on the pages that state a rule's form** — [`02-rewriting-rules.md`](../../../docs/ein/reasoning/01-static-ir/02-rewriting-rules.md):
  *Or a word to the engine*, $\mathsf{inconsistent}$ and $P_{\mathsf{inc}}$, the conditions of a declaration, and
  *By its conclusion* with a row for the word and $\operatorname{dead}(K)$ as a match of a rule of $P_{\mathsf{inc}}$;
  `05-rules.md` (above); `02-syntax.md` (the rule's template takes `:ein <word>`, the keyword table a row, the agenda's
  order, `(false)` a fact and no verdict, the activator a warning); the grammar page's notes (*a rule's word to the
  engine is no `Conclusion`*, outside the generated regions); the glossary's *Rule*, *Inconsistent, `:ein`*, *State*
  and *Dead, alive*. The reasoning notation table has the word, $P_{\mathsf{inc}}$, $\operatorname{refutes}$,
  $\operatorname{denies}$ and $w(r(\bar x))$, and $\operatorname{dead}$ as a match.
- **`std.core`** — `02-syntax.md` § Import and `11-macros.md` § What an import brings: the one module a program has with
  no import; the standard library's index (*What a program has without importing*, and its row: an import brings no
  second copy, where it said *imported by none*); `stdlib/README.md` and the root `README.md` gain a `std.core` row
  (V-31), and the former's *Importing* says it.
- **The decisions** — written: [Q-M1f.552](../../../docs/decisions/read-out/Q-M1f.552-ein-saturate-dump-lists-each-kind-of-rule-in.md),
  [Q-M1f.553](../../../docs/decisions/read-out/Q-M1f.553-a-dead-state-holds-a-contradiction-in-the-words.md) (with .591's words),
  [Q-M1f.555](../../../docs/decisions/stdlib/Q-M1f.555-the-rules-of-a-stored-negation-are-modules.md),
  [Q-M1f.556](../../../docs/decisions/stdlib/Q-M1f.556-the-rules-of-a-stored-negation-are-written-for.md),
  [Q-M1f.559](../../../docs/decisions/stdlib/Q-M1f.559-the-elimination-of-a-double-negation-is-a-module.md),
  [Q-M1f.580](../../../docs/decisions/diagnostics/Q-M1f.580-a-loads-and-a-runs-warnings-are-printed-when.md),
  [Q-M1f.589](../../../docs/decisions/engine/Q-M1f.589-the-static-check-reads-not-as-any-relation.md),
  [Q-M1f.590](../../../docs/decisions/hypotheses/Q-M1f.590-a-hypothesis-rule-is-matched-as-it-is-reached.md),
  [Q-M1f.593](../../../docs/decisions/language/Q-M1f.593-not-is-a-name-everywhere-but-at-a-forms-head.md), each in
  its folder's index. Rewritten in place: Q-M1f.391 (the kernel judged a fact beside its negation dead; now that is
  the contradiction's where it reaches the relation, and the elimination is `std.negation`'s), Q-M1f.447 (where a
  relation of several counts is named once, .594's reading), Q-M1f.557 (a fact of more than six arguments is reached
  by a rule the program writes, not by an activator; one clause said *what read the kernel's check*). Already as
  built and left: Q-M1f.399, .550, .551 (with .563's ruling and .584's reading), .313, .436, .564, .1. **Not
  written**: Q-M1f.565, as the brief says; the ids whose content another decision carries (Q-M1f.601 below).
- **The sentence S1f.21.6 left** — [`06-zebra-walkthrough.md`](../../../docs/ein/user/02-guide/06-zebra-walkthrough.md):
  *dead alone* ends the state, a rule concluding `(false)` and the rule every program gets stating the word; the
  copy of `zebra2.ein` with the candidate stated still reports `∅ dead — root's own fixpoint holds a contradiction`
  (run).

### Beside the rows

- **V-20**: [`10-theory.md`](../../../docs/ein/user/03-syntax-and-semantics/10-theory.md)'s trace shows steps 17, 18 and
  19, the last `std.core.inconsistent` stating the word; its block said 15 and 16 where the drawings beside it said
  17 and 18. A refutation rule concludes `(false)` or states the word.
- **V-16**: *The words they share* on the relation algebra's and the bijections' pages, *The words it shares* on the
  slots' page, and a paragraph in the library's index pointing at them and at `std.debts`' bullet, which already
  listed its words. The words are what `ein saturate --dump` lists as auto-vivified and unprefixed for a program
  that is one bare import of the module, read with the rules that read them.
- **What else was untrue and is now said as built**: the knowledge base's reserved atoms (`not` and `false` are not
  among them); the agenda's first band and its `lead` (the saturation page); the denial read wherever H, L8 or a
  branch's alternatives are defined (`denies`: hypothesis generation § 2, solution semantics § 1.4, finding R's
  known-dead family, satisfiability's `alt`); `08-monotonicity.md` § 3.2's argument; the objects page's table (a
  declaration fixed the count and was written once); the facts page (the context of `not` is the one the engine
  reads); the reserved-names page's opening (the kernel read a held `(not P)`); the natural-language page's kernel
  register; the reasoning-in-code pages (`contradicts`, gone; the dead rows; the L8 rows); `09-binary-format.md`'s
  `FORMAT_MINOR`, 2.

### The acceptance search

Over `docs/ein/` and `docs/decisions/`, by `grep -rn -i`: *beside its negation*, *with its negation*, *and its
negation* (38 lines, each read); *reserved* beside `false` or `not`; *kernel* beside *negation*, `not`, *contradict*,
`false`, *pair*; *conjunction of literals*, *concludes a conjunction*; *derives (false)*, *saturates to `(false)`*;
*as many arguments as*, *declared once*, *declared twice*, *duplicate*; `clash-of-declared`; `contradicts`,
`is_negated`, `wrong_counts`, `Refutes::Not`, `negated_relation`; *engine reads*, *kernel reads*; *denies*, *denied*,
*negation root*; `\lnot f \notin`, `\lnot h \in`; *more than six*; the lisp blocks of every page that write `(not (`
beside a rule or a declaration, with their imports (a script). What each found that the engine no longer does is
fixed above. What is left on purpose: *derives `(false)`* where a rule literally derives it (the calculator's flags,
the algebra's checks); `Kb::is_negated` on the data-model page, which exists (V-36); `Kw` of hypothesis generation
§ 5, the words the blind rung never guesses, `not` and `false` among them, as `hypgen` reads them.

### Defaults taken

| id | default |
|---|---|
| Q-M1f.601 | **Which readings become decision files.** A reading about Ein that no decision carries gets a file under its id: .552, .553, .555, .556, .559, .580, .589, .590, .593. One whose content another decision states gets none, and that decision says it: .562 (Q-M1f.399, .313, .551, .553), .563 (Q-M1f.551, rewritten by S1f.21.17; the older saved file's refusal is the diagnostics page's row), .584 (Q-M1f.551), .585's modules, .586, .588 (Q-M1f.557), .587 (Q-M1f.313), .591 (Q-M1f.553), .594 (Q-M1f.447). Those about which corpus file holds what, .581, .582, .583, .585's programs and .592, get none: the corpus and its notes carry them. |
| Q-M1f.602 | **The reasoning pages' notation for the key**: the word as $\mathsf{inconsistent}$, a conclusion in the place of literals with $\mathsf{inconsistent}\,\theta = \emptyset$; $P_{\mathsf{inc}}$ the rules that state it; $\operatorname{dead}(K)$ a match of one of them; $\operatorname{denies}(S, f)$ a stored side beside which a rule refutes $f$. No Ein spelling on those pages. |
| Q-M1f.603 | **The constraints page's running program imports `std.contradiction`**, since its answer and its *dead* candidate rest on the negation being a constraint; the page says what the program answers without the line. |
| Q-M1f.604 | **V-16 is listed per page from a run**, the auto-vivified relations of a program that is one bare import, with the rules that read each; `std.elim`'s `functional` and `total` are listed as its own positional markers, no name of `std.algebra`'s. |
| Q-M1f.605 | **Sentences of the hypothesis and search pages that state part 1's denial were changed**, the definition of a settled candidate among them, though those pages are S1f.21.14's: each is a statement of what the contradiction is, and was untrue. Nothing else of those pages was touched. |

### Found outside the stage

- **V-43 — A declared `not`'s facts still read as written.** `examples/beliefs/04_liar_denial_contradicted.ein`
  declares `(relation not Claim :why "it is not so that {?3}")`; its trace reads the declaration's fact by that
  `:why` and every fact of `not` as written, `not(=(5, +(2, 2)))`. `ein-cli/tests/natural_language_page.rs` holds the
  page's *kernel* register to `ein_core::RESERVED`, which still lists `false` and `not` beside
  `ein_core::LIBRARY_NAMES`. The page now says the two are the library's and read as written; whether a declared
  `not` should read by its `:why` is open.
- **V-44 — A trace quoted on a page is held by nothing but the natural-language page's.** `10-theory.md`'s block had
  drifted two steps from the trace (15 and 16 for 17 and 18) while its drawings were redrawn; the excerpts on
  `05-rules.md`, `06-constraints.md` and `21-core.md` are as unheld. `natural_language_page.rs`' `<!-- trace: FILE -->`
  check could hold them.
- **V-45 — `09-binary-format.md` § 4.1 named `FORMAT_MINOR = 1`** where `ein_einb::header::FORMAT_MINOR` is 2, with
  nothing to hold it; fixed here, and still unheld.

### For the user

- The constraints page's program now imports `std.contradiction`; its answer and drawings are what they were.
- Five sentences of the hypothesis pages changed (Q-M1f.605): the stage beside this one edits those pages, so a
  merge may meet them.
- The grammar page's notes say *six things* where they said five: a note on `:ein`, outside the generated regions,
  which the stage beside this one also edits.

### The gate

Run on this record's commit; its result is the next commit's paragraph.
