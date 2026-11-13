# S1f.6.35 — Imports by full name: one copy of a module, short names local to a file

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-13, on the user's ruling of that day. Not started. **An engine stage and a
language change.**
**Depends on:** [S1f.6.30](s1f.6.30_name_clashes_between_imported_theories.md), whose page is its
specification: [`import_clashes.md` § 7](import_clashes.md#7-the-ruling-2026-11-13-h-full-names-and-short-names),
the user's model (h) and the defaults taken for what it leaves open. It runs after
[S1f.6.34](s1f.6.34_hypothesis_mechanisms_in_conflict.md) and **before
[S1f.6.31](s1f.6.31_the_stdlib_layout.md)** (the user: *"6. before"*), so the layout move renames full names
once. It is S1f.6.30's T6, planned as a stage of its own.

## The ruling

The user's words are quoted in full in `import_clashes.md` § 7. In one line each:

- a **full name** is a module's canonical name and the short name, one per declaration, global, the same through
  every import path, so a module is one copy per program;
- a **short name** is an alias local to the file whose import lists it; it renames nothing, and is not
  re-exported;
- a **selection** brings the listed names and what they require, following a relation to the rules that read or
  conclude it as well as a rule to its relations (*"1 - yes (ii)"*); a whole-module import, bare or `:as`,
  brings everything; the program holds the union;
- one short name bound to two full names in one file is a load error; a short name no import lists is the
  file's own, shadowing in silence;
- the stdlib has one root, other modules resolve inside the program's file tree, `..` stays refused.

## Tasks

1. **T1f.6.35.1 — the examples first.** The user's rule for a language change is real programs, answers
   computed, before the change is built. On a prototype (module text rewritten to full names, the S1f.6.30
   prototypes' method, `build/s1f.6.30/`): the probes of `import_clashes.md` § 2 and the dependency table of
   § 7, each under (h), with what it prints; the 133 corpus programs that bring the calculator, each import line
   as it must be written under (h) (the short names the program writes, listed), and what moves in `ein solve`,
   `ein saturate --dump` and the shape digests; what (ii)'s closure brings for each shipped module's typical
   selections, against today's closure. Recorded as the record's first section and committed before the
   resolver moves.
2. **T1f.6.35.2 — the resolver.** `ein_ir::imports`: each module resolved once per program, by its canonical
   identity; every declared name rewritten to its full name inside the module's forms; a per-file table of
   short names, from the file's own declarations and its imports' lists and aliases, applied to that file's
   forms only; the closure (ii); the union of edges, deduplicated by full name; the load errors of § 7 (one
   short name to two full names; a full name of a module no import brings; a declaration under an imported
   module's full name), each a section of [`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md)
   and a row of the grammar page's table where it is a shape. The grammar is unchanged: an import keeps its three
   shapes.
3. **T1f.6.35.3 — what prints.** A state, a trace, a core and every read-out row print the main file's short
   name where it has one, the full name otherwise (Q-M1f.237's default). A `--dump-answer` state file re-writes
   the program with full names and loads back to the same answer (`answer_round_trip.rs`); a `.einb` the same.
4. **T1f.6.35.4 — the corpus and the stdlib.** Every program's and every module's imports rewritten to list the
   short names it writes; entry rules and `__closed__` lists stay. Every corpus answer is held: R is the same on
   every tracked program before and after (the sweep), and what moves in printed output is counted before any
   golden is blessed and named in the commit. A register row first for any answer that moves.
5. **T1f.6.35.5 — the findings and the pages.** E-20 (a selection leaves rules behind) and E-48 (a closing rule
   names its obligation) re-read under (ii) and closed or restated; E-49 closed as accepted shadowing; E-50
   closed where full names dissolve it; E-21 (*conflicting definitions … (at None)*) re-read; E-32 restated as
   ruled. `01_grammar.md` § Import rewritten to the model, `defined_behaviour.md`, `stdlib/README.md` §
   Importing and every module header that says how to import it, `AGENTS.md`'s rules on imports, the
   read-out reference where a printed line moves.

## Acceptance

- The user's dependency graph loads one copy of each module, and every row of `import_clashes.md` § 7's table
  behaves as its *under (h)* column says, held by a test.
- A program that lists `add` and `sum-of` from the calculator, and no entry rule, computes; one that writes its
  own `done`, `b` or a rule `add-column` beside the calculator loads and is unaffected by it.
- `:as` makes no copy: the calculator imported flat and `:as` holds one set of tables.
- No tracked program's R moves; every moved output is counted and named.
- Every page that says how an import works says (h).

## What it must not do

- **Not change the grammar's three import shapes**, and not add an export list: the importer chooses
  (Q-M1f.232 as ruled).
- **Not allow `..`**, and not follow symlinks specially (*"out of the scope"*).
- Not rename any relation of a shipped module: the full names are made by the resolver; S1f.6.31 moves the
  modules after this stage.
- Not decide a printing rule beyond Q-M1f.237's default without showing the user.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Config never changes an answer; every
answer exits 0. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly` on the stage's last commit;
`utils/ci_container.sh` if a DOT writer's output or the review bundle moves.
