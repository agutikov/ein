# S1f.6.11 — A flat import brings a module's facts

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's rulings of that day. **It runs first:** the theory stages write their
tables as plain facts and import them flat. An engine stage.
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
