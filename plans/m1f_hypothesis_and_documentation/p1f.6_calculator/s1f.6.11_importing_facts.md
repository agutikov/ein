# S1f.6.11 — A flat import brings a module's facts

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-11** (`5b252d50`): W88's four rows filed and fixed, the sweep moved the four witnesses
alone, and the six design points stand as defaults taken, Q-M1f.135 to .140, with four more the build met,
Q-M1f.141 to .144 ([§ T1 record](#t1-record-2026-11-11) onward). Planned 2026-11-11, on the user's rulings of
that day. **It ran first:** the theory stages write their tables as plain facts and import them flat. An engine
stage.
**Depends on:** nothing in the phase. Its specification is
[design.md § G](design.md#g-the-flat-import-finding-a-specification-for-importing-facts).

## The rulings

The user, 2026-11-11: *"the flat import - together with theory libs that are rule libs we probably would want to
have ontology libs that are facts libs, does it make sense? and then importing facts is mandatory"*, and on the
fix, *"yes"*.

Today `(import M :symbols (…))` keeps the listed declarations and their closure and drops every fact of M, in
silence; a qualified import keeps the facts and renames each declared relation, so a library's `is-a` facts land
in `M.is-a` and do not join the program's.

## Is it a wrong answer?

Yes, under the ruling. A program that imports a facts library flat and asks a goal over one of its relations is
answered *the goal has no row*, exit 0, where the ruled answer has the library's rows. That is a well-formed
program and an answer that is false under a ruling, which is what the register holds. T2 files it before the
fix. A solve run with a query's classes is what `ein-wrong-answers/21` already records, so the default is no
schema change; T1 confirms that on the witnesses.

## The design points, each with its default

The defaults are the recommendations of `design.md` § G. Each stands unless the user overrules it; T1 lists
them again with their probes re-run.

| point | default |
|---|---|
| (a) which facts `:symbols` brings | a listed name selects the module's declaration of it, if any, and every top-level fact whose relation it is; the closure follows names as today, so a kept rule that reads a relation brings that relation's facts. A listed name may be a relation the module only states facts of |
| (b) what a qualified import does to a fact's names | unchanged: it renames declared names. The page says a library of facts is imported flat |
| (c) imported constants | they are names of the program, as an imported fact is a fact of the program; a library closes its own relations if it does not want them guessed |
| (d) provenance | unchanged: a fact keeps its own `:source` and is otherwise background. A module stamp is F24's |
| (e) duplicates | identical facts are one fact; the same fact with two `:source`s keeps its first statement, as today, named on the page. This is E-6 of [`findings.md`](findings.md): the stage states the behaviour and hands the question to [F24](../../followups/f24_written_provenance.md) unless the user rules it here |
| (f) an unlisted rule | still dropped: a selection is a selection. Not part of this fix |

## Tasks

1. **T1, read-only, on the release binary.** The probes of `build/s1f.6/prototype/g/imp/` re-run and banked as
   the stage's table: what each import form does with a module's relations, rules, macros and facts. **The
   census:** every tracked `.ein` file (`build/s1f.6/prototype/scan_imports.py` found 663 files and no import
   of a module that holds a fact), and what round 2 did not scan, the programs and modules written inline in
   the Rust test suites (`ein-ir/tests/imports_semantics.rs` and every test that calls the resolver). The
   census decides what the fix can move.
2. **T2, the register rows, filed `open` before any fix.** Witnesses under `examples/ein-bugs/wrong-answers/`:
   a library of facts and a program that imports it flat and asks a goal over it (no row today); the same with
   a rule of the library that reads its facts (the rule kept, never fired); the calculator's case, a table as
   facts and a rule that looks it up. Each `now` is today's answer and each `right` the answer under the
   defaults above. Held to the unchanged engine by `wrong_answers.rs`.
3. **T3, the fix,** in `ein_ir::imports`' `select`: a kept name keeps the top-level facts of that relation,
   unrenamed, in the module's order. The resolver's tree-shake after it already keeps every fact.
4. **T4, the pin:** a case per row of T1's table in `imports_semantics.rs`, and a test that a diamond of flat
   imports of one facts library yields each fact once.
5. **T5, the sweep.** Old release binary against new over every tracked `.ein` file, `ein saturate --dump` and
   `ein solve`: by the census nothing moves but T2's witnesses. Anything else that moves is shown before a
   golden is blessed.
6. **T6, the pages.** `01_grammar.md`'s import row and § import; `defined_behaviour.md`, a section of its own;
   `stdlib/README.md` § Importing, which today says only *"the listed names plus their dependency closure"*;
   `solution_semantics.md` § 1.1, that an import can bring ontology as well as theory; `examples/README.md` for
   the witnesses; `AGENTS.md`.
7. **T7, the rows flipped to `fixed`** in the commit that fixes them, and the gate green.

## Acceptance

- A library of facts imported flat brings its facts, on every command that loads a program.
- The register's rows are fixed, and no other corpus answer moved, by the sweep.
- Each design point is recorded with the user's ruling or as a default taken.
- `./run_tests.sh` green; `utils/ci_container.sh` is not needed, since no DOT writer or installed package
  changes.

## T1 record, 2026-11-11

Read-only, on the release binary built at `7f574423` and kept as `build/s1f.6.11/ein-old`.

**The probes of `build/s1f.6/prototype/g/imp/` re-run** (`ein saturate --dump` on each): every cell of
[design.md § G](design.md#1-what-each-import-form-does-today)'s table reproduced. The table as the stage banks
it, *before* that binary and *after* the fix, a cell that moved in bold:

| of the module | `(import onto)`, `(import onto :as o)` | `:symbols`, its name in the closure, before | after | `:symbols`, its name not in the closure |
|---|---|---|---|---|
| a declared relation | kept, renamed `onto.is-a` / `o.is-a` | kept, `is-a` | the same | dropped |
| a rule | kept, renamed | kept | the same | dropped, though the program's facts, and since the fix the imported ones, would fire it |
| a macro | kept, renamed, called `(o.both …)` | kept | the same | dropped |
| a fact of a declared relation | kept, its relation renamed: `(o.is-a Tea Drink)` | dropped | **kept as written** | dropped |
| a fact of a relation the module does not declare, `(likes Tea Milk)` | kept as written | the name could not be listed: *not provided by the module* | **the name may be listed, and the fact is kept** | dropped |
| a stored negation, `(not (likes Milk Tea))` | kept | — | **kept, a fact of the relation it negates** | dropped |
| a fact's `:source` | kept: the fact is *given* | — | **kept** | — |
| a constant in a fact, `Tea` | unrenamed | — | unrenamed, a name of the program | — |
| the module's `(query …)`, `(solve …)`, `(config …)` | **kept** (E-8 of [`findings.md`](findings.md)) | dropped | dropped | dropped |

The qualified columns did not move. The last row is this record's own probe, not § G's: a module that holds a
`(solve …)` imported under two aliases is refused, *2 (solve …) forms — a program states its lattice once*, and
one that holds a `(query …)` or a `(config …)` gives both to its importer (`ein solve`, `--dump-config`). It is
outside the stage and filed as E-8.

**The census.** `python3 build/s1f.6/prototype/scan_imports.py` (2026-11-11): 663 tracked `.ein` files, 173 flat
imports, 3 aliased, 6 bare, and no import under any form of a module that holds a top-level fact; the seven
stdlib modules hold none. What round 2 did not scan, read by hand (`grep -rnE '\(import ' ein.rs/crates`,
2026-11-11): every module a Rust test writes and imports **flat** holds declarations alone, `ein-cli`'s
`program_read.rs` (`mod.ein`, one relation), `unicode_only_in_a_string.rs` (modules that are refused),
`ein-ir`'s `imports_semantics.rs` (`m.ein`, two relations and one declaration, and the override stdlib's
`sibling` and `hasrel`, one relation each) and `kb_semantics.rs` (`lib.ein`, two macros). Every other inline
import is of a `std.*` module, or qualified (`kb_semantics.rs`, `cli_semantics.rs`, `open_reserved.rs`), which
the fix does not touch. No page under `docs/` shows a flat import of a module with a fact. **So the fix can move
no existing program's answer**, and T5's sweep and the gate hold it.

**The register's schema** needs no change: a solve's run with the sentences a query's `:goal-text` prints
(`goal_text`, schema /15) records what the import drops, and a refusal the fourth row (Q-M1f.144).

**The design points, each as taken.** No ruling overrode a default, and none proved unimplementable:

| point | taken | id |
|---|---|---|
| (a) which facts `:symbols` brings | a3, as stated: a name in the closure selects the module's declaration of it and every top-level fact whose relation it is; a listed name may be a relation the module only states facts of | [Q-M1f.135](../open_questions.md#q-m1f135--which-facts-a-flat-import-brings) |
| (b) a qualified import and a fact's names | unchanged; the page says a library of facts is imported flat | [Q-M1f.136](../open_questions.md#q-m1f136--what-a-qualified-import-does-to-the-names-inside-a-fact) |
| (c) imported constants | names of the program | [Q-M1f.137](../open_questions.md#q-m1f137--are-an-imported-facts-constants-names-of-the-program) |
| (d) provenance | unchanged, no module stamp: F24's | [Q-M1f.138](../open_questions.md#q-m1f138--the-provenance-of-an-imported-fact) |
| (e) duplicates | identical facts are one; two `:source`s keep the first statement, stated on the page and handed to F24 (E-6) | [Q-M1f.139](../open_questions.md#q-m1f139--the-same-fact-from-two-places) |
| (f) an unlisted rule | still dropped | [Q-M1f.140](../open_questions.md#q-m1f140--a-rule-of-the-module-the-selection-does-not-name) |

Four more the build met, each taken by default:

| what | taken | id |
|---|---|---|
| does the closure follow the names inside a kept fact? | no: declarations only, as before the stage | [Q-M1f.141](../open_questions.md#q-m1f141--does-a-selections-closure-follow-the-names-inside-a-kept-fact) |
| a stored negation, and a fact of a kernel relation (`__closed__`, a written `eq`) | a negation is a fact of the relation it negates; a kernel relation's fact comes as any fact does | [Q-M1f.142](../open_questions.md#q-m1f142--a-stored-negation-and-a-fact-of-a-kernel-relation-under-a-selection) |
| a fact that arrives twice through flat imports | one statement, kept where it first arrived | [Q-M1f.143](../open_questions.md#q-m1f143--a-fact-that-arrives-twice-through-flat-imports) |
| what the register rows record | the goal's sentences and a refusal, no schema change | [Q-M1f.144](../open_questions.md#q-m1f144--what-w88s-rows-record) |

## T2 record, 2026-11-11

Four witnesses and two modules under `examples/ein-bugs/wrong-answers/`, each with a corpus entry, case **W88**:

| row | the witness | at `7f574423` | right |
|---|---|---|---|
| `W88/import-facts-flat` | `(import import-facts-library :symbols (is-a))` and the goal `(is-a ?x Drink)` | loads with no `is-a` fact; *the goal has no row*, exit 0 | two rows, *Milk is a drink*, *Tea is a drink* |
| `W88/import-facts-rule` | `:symbols (liquids)`, the library's rule that reads its `is-a` facts, and the goal `(liquid ?x)` | the rule and the declaration kept, the facts dropped; the rule never fires; no row | `(liquid Tea)` and `(liquid Milk)` derived; two rows |
| `W88/import-facts-undeclared` | `:symbols (likes)`, a relation the library only states facts of | refused at load on every command, *not provided by the module: likes* | loads; root holds `(likes Tea Milk)` and `(not (likes Milk Tea))` |
| `W88/import-facts-table` | `(import import-facts-table-module :symbols (add2))`, the calculator's case: two `fa` rows as facts and the rule that looks them up, asked 9 + 1 | the rule kept, the table dropped; no row | `(sum-of (num2 0 9) (num2 0 1) (num2 1 0))` |

`import-facts-library.ein` and `import-facts-table-module.ein` are the modules, programs of their own too. Each
row was filed `open`, its `now` today's answer: a solve whose `goal_text` is empty for three, and the refusal for
the fourth. With them the register read 301 fixed, 20 open and 1 not wrong (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11), and `ein-corpus`'s tests, `ein-cli`'s
`wrong_answers.rs` and `ein-infer`'s `register_region.rs` were green on the engine sources at `7f574423`.

**The goldens the fix moves**, named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt` (16 new
cells: the two modules' two runs and the four witnesses' three, each exit 0) and
`ein-render/tests/golden/corpus_shapes.md5` (the six new files' lines alone). No other golden, and no line of an
existing entry.

## T3 record, 2026-11-11

The fix is `ein_ir::imports`' `select` and nothing else of the resolver's logic:

- a top-level form is a declaration, a **fact**, or one of `query`, `solve`, `config`, `trace`, `import`
  (`DIRECTIVES`); `fact_relation` gives a fact's relation, a stored negation's being the relation it negates;
- `select` collects the closure as before, now keeping every name it reaches and not only the declared ones,
  and keeps a declaration whose name, or a fact whose relation, is in it, in the module's order;
- a listed name is *provided* when the module declares it or states a fact of it;
- `dedup_declarations` keeps a fact that arrives a second time, the same node, once.

The grammar is untouched, and so is the loader: an imported fact is ingested as a written one.
`resolve_and_minimize`'s tree-shake already kept every form that is no declaration.

On the new binary each witness answers its `right` on every command, and the four rows are `fixed`.

## T4 record, 2026-11-11

- `ein-ir/tests/imports_semantics.rs`: `what_each_import_form_brings_of_a_module`, a case per cell of T1's
  table, each import form's resolved forms held line for line;
  `a_selection_follows_declarations_and_not_the_names_inside_a_fact` (Q-M1f.141, .142); and
  `a_diamond_of_flat_imports_yields_each_fact_once`, a diamond through two modules that each re-export the
  library, the library imported flat twice by one program, and the diamond loaded, each fact held once.
- `ein-cli/tests/import_facts.rs`: the four witnesses, each given the claim its goal's rows make, on twelve
  commands (`saturate`, `saturate --dump`, `solve` and its four modes, `oracle`, `test`, `render lattice`,
  `render kb`, `kb save`): the import answers what the selected forms written in its place answer, and the
  saved container what the file answers; and `ein test` holds each claim. The four views that parse and never
  load are left out, since each draws a file's imports unresolved (Q-M1f.17).

## T5 record, 2026-11-11

`python3 build/s1f.6.11/sweep.py` (2026-11-11): `build/s1f.6.11/ein-old` against the fixed release binary,
`ein saturate --dump` and `ein solve`, stdout, stderr and exit, over the 669 `.ein` files of the tree (the 663
tracked at `7f574423` and the stage's six): 1 338 runs a binary, and **4 files moved, the four witnesses**, each
under both commands. The two modules and every other file print the same bytes, the zebra programs among them.

## T6 and T7 record, 2026-11-11

Pages: `01_grammar.md` (an `import` row in § 2's table, the `:symbols` keyword row, and § Import, new),
`defined_behaviour.md` § 4.25, `stdlib/README.md` § Importing, `solution_semantics.md` § 1.1,
`examples/README.md`, `AGENTS.md`; [`findings.md`](findings.md)'s E-6 restated and E-8 added; the questions
Q-M1f.135 to .144.

The register reads 305 fixed, 16 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq
-c`, 2026-11-11). `./run_tests.sh` green (`build/gate-s1f.6.11.log`). The fixing commit is `5b252d50`; the commit after it wrote that hash into W88's four `[case.fixed]` rows, where the fixing commit carried the null id.

## Acceptance re-taken, 2026-11-11

- **A library of facts imported flat brings its facts, on every command that loads a program:** `import_facts.rs`,
  twelve commands and the container.
- **The register's rows are fixed, and no other corpus answer moved:** W88's four, and T5's sweep.
- **Each design point is recorded as a default taken:** T1's table, Q-M1f.135 to .140, and .141 to .144.
- **`./run_tests.sh` green.**
