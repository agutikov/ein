# S1f.6.35 — Imports by full name: one copy of a module, short names local to a file

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-13, on the user's ruling of that day. **T1 done 2026-11-14, and stopped for the
coordinator there**: on the prototype, (h) as ruled moves R on tracked programs (§ Record, *The stop*). The
user ruled
[Q-M1f.247](../open_questions.md#q-m1f247--what-the-closure-ii-follows-a-relation-of-any-kind-or-a-declared-one)
and [Q-M1f.248](../open_questions.md#q-m1f248--a-closing-rule-under-the-closure-ii) the same day, the reading
V2, and asked to see the moves first
([Q-M1f.249](../open_questions.md#q-m1f249--the-answers-h-moves-under-the-recommended-reading)): shown in §
Record, *The moves under V2, shown*, with E-53's fix to `std.algebra`. **Stopped again there**, T2 not started;
Q-M1f.249 to .253 open. No engine source has moved. **An engine stage and a language change.**
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
  brings everything; the program holds the union. **Made precise 2026-11-14** (Q-M1f.247, .248): *"the
  behaviour must not change - when import std.math.arith and use std.math.arith.sum must work identical to sum
  from import std.math.arith :symbols (sum); So I guess that any touched relation must be pulled in, otherwise
  semantic of imported symbold could be incomplete"*, and *"Yes, closing counts"*: **a selected short name
  means exactly what its full name means under a whole-module import**, so the closure follows any relation, a
  rule's activators' too, and a rule that closes a kept relation concludes it;
- one short name bound to two full names in one file is a load error; a short name no import lists is the
  file's own, shadowing in silence;
- the stdlib has one root, `std.`; every other module name is a path relative to **the program file's
  directory**, a dot per directory, for the imports a module writes too: `(import word-theory.gaps :as gaps)`
  from `examples/word_problems/` names `word-theory/gaps.ein`, whose `gaps` is `word-theory.gaps.gaps` (the
  user, the same day); `..` stays refused; a declared short name holds no dot (Q-M1f.241, defaulted).

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
   forms only; every local module located against the program file's directory, a module's own imports
   included (today they resolve against the module's directory); the closure (ii); the union of edges, deduplicated by full name; the load errors of § 7 (one
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

## Record, 2026-11-14

Every run below is the release binary of `4676a5de`, the last commit that moved engine source, kept as
`build/s1f.6.35/ein-before` (`ein --version`: stdlib `sha256:aad4cec7…`). The scratch is `build/s1f.6.35/`,
which is not tracked; every program and output this section relies on is named or quoted. **No engine source,
stdlib module, corpus program or golden moves in this section.**

### T1f.6.35.1 — the examples first, on a prototype, done; and the stop

**The prototype.** `build/s1f.6.35/h.py` is (h) written as a resolver in Python: it takes a program and writes
the import-free program (h) makes of it, and the release binary loads that text, so every answer below is the
engine's. What it does, point by point of § The ruling:

- every module once, by its canonical name: `std.x` under the stdlib, any other a path from **the program
  file's directory**, a dot per directory, for the imports a module writes too; a cycle refused as today;
- every name a module declares under its full name, `std.arith.strip`; the main file's own declarations keep
  their short names (the program has no module name); a name no file declares (`z`, the digits, a type, a
  relation stated and never declared) has no full name and is one name in every file, as today;
- per file, a table of short names — its own declarations, its imports' lists, its aliases — applied to that
  file's forms only; one short name bound to two full names in one file refused; a full name of a module the
  program brings read as itself, of one it does not brings refused (Q-M1f.238), a declaration under an imported
  module's full name refused (Q-M1f.239), a module's declared short name with a dot refused (Q-M1f.241);
- the closure (ii), per module and transitively across modules by full name, and **the union of every import
  edge of every file the program reads**;
- the main file's short names printed short, the full name otherwise (Q-M1f.237): the resolved text is renamed
  once, full name to the main file's spelling, before the loader sees it.

The drivers: `rewrite2.py` writes every tracked file's imports as (h) asks them (`build/s1f.6.35/tree/`, the
stdlib's manifest re-taken), `t1.py` loads every tracked `.ein` three ways — **A** as tracked, **B** the rewritten
file under today's resolver, **C** the rewritten file flattened by `h.py` — and compares R by
`--json-summary` (the states as sets of facts, k_s, k_c, root's verdict and debts, the lines, the components,
the universe, every query's classes and claim) with every module's prefix taken off C's names, and the
read-out by its text; `probes.sh` runs the probes and the dependency table; `calc.py` the 133; `closure.py` the
selections.

**How the imports must be written** (`rewrite2.py`, read off every program that holds the file). Today a
program is one flat list and one name is one name; under (h) a name a file writes is the file's own unless it
declares it or an import of it lists it. So wherever a file F writes a short name that, in some program, today
meets a declaration another file D wrote, F lists it in an import of D; a name F lists from a module that
neither declares it nor states a fact of it (a re-export, which (h) does not have) moves to an import of its
declaring module; where D is the program's own file, a module's undeclared name is already the main file's.
**162 of the 854 tracked files are rewritten** (four of them stdlib modules: `std.int` and `std.rat` list the 12 and 15 names of the modules below them that they write, `std.closure` and `std.typing` gain an import), every other one left byte for byte; no file declares a name an
import also brings (0 *variant copies* met), and no file needs one short name from two modules. Beside the
programs' lists, two shapes the rule meets:

- **Four modules write another module's names without importing it**, today joined only by the program's one
  flat namespace, and under (h) each needs an import of its own: `std.typing` writes `std.algebra`'s
  `converse`, `converse-illtyped-dom` and `converse-illtyped-ran`; `std.closure` writes `functional` and
  `total`; `examples/binary_logic/full_adder.ein`, a library of facts, writes `circuit`'s `gate`;
  `examples/word_problems/readings.ein` writes `std.arith`'s `add`, `n` and `sum-of`. Without the four imports
  `tests/stdlib/typing/02` loses its contradiction and `tests/stdlib/closure/02` its closure, 1 state to 9
  (the first sweep, before they were found).
- **One program declares its own copy of a module's relation**: `examples/binary_logic/adder_against_table.ein`
  declares `gate` as `circuit` does and imports `full_adder`'s facts of it, where every other program reads
  those facts as `circuit`'s; under (h) `full_adder` imports `circuit`, and the program, which lists `gate`
  from `full_adder` and declares it too, is refused (*one short name, two full names*). It is the one program
  whose text moves beyond its imports: its `(relation gate …)` replaced by `(import circuit :symbols (gate))`,
  R and read-out unchanged, under today's resolver and under (h).

And four precisions the prototype had to take, each a default of this section: a listed name the module states
facts of and does not declare is bound to the relation those facts are of, as the module reads it
(`full_adder`'s `gate`, `inputs`' `total-owed`: Q-M1f.250); an alias-prefixed or full name that names nothing
the module declares is refused (`i.n` under `(import std.int :as i)`, `std.int` declaring no `n`: Q-M1f.251);
a module name with an empty segment or a `/` is refused (Q-M1f.252); the main file's alias spelling, `a.sum-of`,
is the name it prints under `:as` (Q-M1f.253). And one reconciliation of two defaults: Q-M1f.241 (*a declared
short name holds no dot*) is read of a **module's** declarations, and the main file's are Q-M1f.239's, so a
`--dump-answer` state file, which declares `std.arith.strip` and imports nothing, loads as today (Q-M1f.239's
own words).

**Three readings of the closure (ii), computed** (`build/s1f.6.35/variants.sh`, three sweeps of all 854 files,
2026-11-14). *Literal*: a kept name of the module's own, of any kind, to the module's rules that read it in
their `:match` or conclude it in their `:assert`. *Closing*: and a rule whose `:assert` writes
`(__closed__ R)` of a kept `R` concludes it (Q-M1f.248). *Relations*: the closure follows a name the module
declares by `(relation …)`, not a rule's name read as its activators' relation (Q-M1f.247).

| reading | of the 514 programs that load today, R or exit moves on | the read-out byte-equal / once prefixes are off |
|---|---|---|
| V1, literal | **22**: 16 `std.num9` programs refused by *forbid* (`stdlib/num9.ein`, `examples/arith/num9.ein`, `num9_overflow.ein`, `tests/stdlib/num9/` all but `12`); `tests/stdlib/typing/03` refuted; `adder_against_table`, `full_adder`, `gaps`, `listed_is_all` past the budget or the region limit; the four of V3 | 461 / 482 of 492 |
| V2, literal and closing | **5**: `tests/stdlib/typing/03` refuted; the four of V3 | 477 / 499 of 509 |
| V3, relations and closing | **4**: `tests/imports/02_nested_import_left_behind.ein`, `examples/word_problems/gaps.ein`, `listed_is_all.ein`, and `examples/features/09`'s universe row | 478 / 500 of 510 |

No program refused today loads under any reading; of the 336 refused both ways, 304 print the same first line
once places and prefixes are off, and the 32 others are the prototype's own text (its reader, its error
lines) and `examples/broken/load/import_conflicting_definitions.ein`, whose refusal is now (h)'s.

Why each moves, and what it says:

- **`std.num9`'s 16, V1.** `std.num9` lists `division-by-zero` from `std.arith`, since it writes it. Literal
  (ii) follows the relation to the rules that conclude it, `div-by-zero` and `div-call-by-zero`, which read
  `done`, and from `done` to nearly the whole circuit, 195 declarations, **without the closing rule `arith`**,
  which writes every `(__closed__ …)` of the calculator as values: so `let` is open, and *forbid* refuses
  (*rule 'std.arith.malformed-is-false' concludes (false) from std.arith.malformed, which rule
  'std.arith.bad-request-malformed' derives resting on (absent …) over std.arith.let, a relation a commitment
  can still extend*). **The acceptance's own program fails the same way**: `(import std.arith :symbols (add
  sum-of n))` and `(add (n z 2) (n z 3))` is refused under V1 with the same two lines, and computes under V2
  and V3, `(sum-of (n z 2) (n z 3) (n z 5))` (`build/s1f.6.35/acc/`). The closing rule is what makes a
  selection of a computed theory compute, and nothing in (ii)'s words reaches it.
- **`tests/stdlib/typing/03_type_hierarchy_welltyped.ein`, V1 and V2.** It imports `std.algebra`'s
  `converse-illtyped-dom` and `-ran`, and writes `(converse owns owned-by)` over a subtype hierarchy (`Pet` is
  an `Animal`): well-typed, `(solve :expect (region (model …)))` held. `converse` is a rule of `std.algebra`
  and the relation of its activators; literal (ii) follows it to the rules that read or conclude it, nine
  lemmas (`converse-pair-symmetric`, `symmetric-is-self-converse`, `compose-contravariant`, …), and
  `converse-pair-symmetric` derives `(converse owned-by owns)`, whose range check under the hierarchy fails:
  root dead, the claim refuted. **The same refutation today**, with the lemma listed by hand
  (`build/s1f.6.35/e53/welltyped_with_lemma.ein`: *refuted by converse-illtyped-ran*): [E-53](findings.md),
  two library rules that do not compose, met by (ii) and not made by it.
- **`tests/imports/02_nested_import_left_behind.ein`, every reading.** E-20's witness: `upper_without_entry`
  imports `lower` by its entry, and the program `upper_without_entry` by its; today the closure of the
  program's selection reaches `lower`'s declarations and none of its rules, and the loader's `note:` says so.
  Under (h) `upper_without_entry`'s own import of `lower` is an edge of the program, and brings `lower` whole:
  `(got a aa)` is derived, the claim `(model)` refuted. R is {∅} either way; root's state and the query's
  answer move. **E-20 dissolves**, as the stage file expects of it, and the witness answers what its sibling
  `01_nested_import_whole` answers.
- **`examples/word_problems/gaps.ein` and `listed_is_all.ein`, every reading.** Two modules a problem imports,
  read alone as programs; their corpus entries run `saturate` and `render rules` and **no solve**, because,
  in their notes' words, *alone `asked` and `wanted` are open and a solve describes the blind enumerator's
  `(asked __closed__)` (E-1), not the module*. Each lists relations of `words`; (ii) follows them to `words`'
  rules, and V2 and V3's closing to its closing rule, so `ein solve` now describes the module: R {∅} where it
  was 4 states of `(asked __closed__)`, and `ein saturate --dump` 172 lines to 2 609 (`words` and the
  calculator whole).
- **`examples/features/09_adjacent_via_same_house.ein`, every reading: R unmoved, the universe row moves**, 82
  candidates, each dead alone, to none. The program lists `typecheck-arg-0` and `-1` from `std.bijection`;
  under (h) `std.bijection`'s own import of `std.algebra`, which lists `total-owed` and `surjective-owed`, is an
  edge of the program, so an obligation rule is loaded, and by E-51 the ladder takes the obligations rung,
  which owes nothing: no candidate. Today's selection reached neither.

### The stop

The coordinator's instruction for T1: *"this is not a stop, unless the prototype shows that (h) as ruled cannot
be built, or would move R on a program"*. It shows both halves of that sentence in part:

1. **(h) as ruled, read literally (V1), moves R on 22 programs** and refuses the stage's own acceptance program;
   the two readings that hold R, Q-M1f.247 and Q-M1f.248, read the ruling's *"a listed or kept relation to the
   rules that read or conclude it"* narrowly (a declared relation) and widely (closing a relation concludes
   it). Each is a reading of the user's words, not a default of what they left open, so it is the
   coordinator's to take or to put to the user, not this stage's.
2. **Under both readings, R or an answer still moves on three tracked programs**, each by the ruling itself and
   not by a defect: the E-20 witness (one copy, the union of edges), and the two modules' `ein solve`, which the
   corpus does not run (Q-M1f.249). And one program's text must move beyond its imports.

Nothing irreversible was done; no source moved. The resolver (T2) waits on Q-M1f.247 to .249; Q-M1f.250 to
.253 are defaults T2 would take.

### The probes of `import_clashes.md` § 2, under (h)

Each written as (h) asks (`build/s1f.6.35/p/`; the importer lists the short names it writes), flattened under
V3 and solved; the today column is § 2's.

| probe | today | under (h) |
|---|---|---|
| **P1a**, `examples/arith/num9_both_ways.ein` | `:expect holds` | holds; `std.num9`'s `n` is `std.arith`'s by full name: one `n` |
| **P1b**, `succ` and `order` over `std.digits`, joined | nine `step-up`; `fa` 200 rows, 522 facts | the same: `std.digits` once, `std.digits.fa` 200 rows, 522 facts |
| **P2a**, `std.arith` and `chores`, each declaring `done`; the program lists `chores`' | refused, *conflicting definitions of 'done' … (at None)* | **loads**: `(sum-of (n z 2) (n z 3) (n z 5))`, `(to-do dishes)` |
| P2a, `done` listed from both | refused, as above | **refused**: *'done' is bound to two full names, std.arith.done and chores.done* |
| **P2b**, `std.arith` and E-21's `arith2`, one listed, one `:as t` | refused, *… of 'div-m1' …* | **loads**: 5 + 7 = `(n (n z 1) 2)`, and base two's `(t.sum-of (t.n (t.n z 1) 0) (t.n (t.n z 1) 1) (t.n (t.n (t.n z 1) 0) 1))` |
| P2b as S1f.6.16 wrote it, `arith` listed from both | refused | refused: *'arith' is bound to two full names, std.arith.arith and arith2.arith* |
| **P3a**, `std.arith :as a`, `chores` flat | loads; `(a.sum-of … (a.n z 5))` | the same, printed the same |
| **P3b**, `succ` flat, `std.arith :as a` | `fa` and `a.fa`; 2 044 facts | **one table**: 1 842 facts, 119 rules; `(bumped 6)` |
| **P3c**, `std.num9` flat, `std.arith :as a` | `(a.add (n z 2) …)` silent: two numbers | **connected**: `(to9 (a.n (a.n z 4) 2))` gives `(as9 … (std.num9.num9 0 0 0 0 0 0 0 4 2))`, `num9` printed full since the program does not list it |
| **P4a**, the program's `done` | refused | **loads**: `(done laundry)` and the sum |
| **P4b**, the program's `sum-of` | refused | **loads**: its `(sum-of ann (n z 2) (n z 3))`, and the calculator's as `(std.arith.sum-of ?x ?y ?s)`, `(n z 5)` |
| **P4d**, `(done laundry today)`, undeclared | joins the calculator's `done` | the program's own: one row, `?a = today · ?t = laundry` (shadowing, Q-M1f.236) |
| **P4e**, the program's rule `add-column` | refused | **loads**: `(col units 2 1)`, `(sum-of (n z 7) (n z 5) (n (n z 1) 2))` |
| **P4f**, the program's relation `b` | refused | **loads**: `(b one)` and the sum |
| **P5**, `std.arith` flat and `:as a` | two calculators, 3 656 facts, 232 rules | **one copy**: 1 829 facts, 116 rules; both requests answered by one `sum-of`, `(sum-of (n z 4) (n z 3) (n z 7))` printed short |
| **P7a**, `(a.add-walk …)`, `(a.done …)` | `(total tea (a.n (a.n z 1) 0))` | the same |
| **P7b**, the call written bare | silence | silence: the program's own `add-walk` and `done` (shadowing, Q-M1f.236) |
| **P8**, a core under `:as a` | `refuted by a.division-by-zero-is-false` | the same; listed instead, `refuted by std.arith.division-by-zero-is-false`, the circuit's rule by its full name |
| **P10**, `std.int :as i`, `std.rat :as r`, `i`'s sum to `r` | 3 943 facts, 432 rules; no answer | **connected**: 2 020 facts, 245 rules; `(r.rat-simple-of (r.rat (i.int (n z 3) z) (n z 1)) …)`, with `n` listed from `std.arith` |
| P10 written `i.n` | — | **refused**: `std.int` declares no `n` (Q-M1f.251) |
| **P11**, `std.algebra`'s rule `difference` beside `std.arith` | refused by the count | **loads**, the claim held: `std.algebra.difference` and `std.arith.difference` are two names |
| `(relation std.arith.mine …)` beside `std.arith` | loads | **refused** (Q-M1f.239) |
| `(std.arith.add …)` with no import of `std.arith` | loads, a name of its own | **refused** (Q-M1f.238) |
| `(std.arith.sum-of ?x ?y ?s)` beside `std.arith :symbols (arith add n)` | no row | `(n z 5)`: a full name of a module the program brings |

### The user's dependency table, under (h)

The coordinator's scratch modules rebuilt (`build/s1f.6.35/dep/`: `A` with `a-in`, `a-step` concluding
`a-out`, the entry `a` and the fact `(a-in seed)`; `B`, `C`, `E` each over the one below by its entry and
`a-out` or `b-out`, as § 7 describes them), each program written as (h) asks.

| program | § 7's *under (h)* | computed |
|---|---|---|
| all flat, the user's graph | one copy; the program writes `a-in` only if its import of A lists it | one copy: `a-step`, `a`, one `(a-in seed)`; the program lists `a-in`, and `a-out`, `c-out`, `e-out` are each `mine` and `seed` |
| C alone, writing `a-in`, reading `a-out` | the program's own | the program's own: **refused at load**, the query's check, *the program names no relation a-out*, since the program writes `a-out` in its goal alone |
| its own `a-out`, C alone | loads | loads; its `a-out` is its own and open (`(a-out __closed__)` among its 4 states, E-1), and C's chain reads A's, `A.a-out` |
| A qualified, the rest flat | one copy | one copy, printed `qa.a-in`, `qa.a-step`, B's rule reading `qa.a-out` |
| `(import C :as cc)` beside a flat A | one copy; `cc.` aliases C's own names | `cc.c-out`, `cc.c-step`; A once, under the program's flat names |
| a qualified import inside a qualified one | the defining module's: `A.a-in` | `(A.a-out ?x)` answers `seed`; `Bq`'s own names print `bb.bq-out` |
| a cycle X → Y → X | refused | refused, *import cycle: X -> Y -> X* |
| `(A.a-in x)` with A flat | A's own `a-in` | `(a-out x)` beside `(a-out seed)` |
| `(relation std.arith.mine …)` | refused | refused (Q-M1f.239) |

### The 133 programs that bring the calculator

The 133 of S1f.6.30's sweep (`calc.py`, every tracked file that is `stdlib/arith.ein` or imports it at any
depth), under V3. **R equal on 128**; the other 5 are three refused today and under (h) alike (the arity
witnesses) and `gaps` and `listed_is_all` above. **`ein solve`'s read-out**: byte-equal on 97, a name only on 21
(a circuit's tag or rule the program does not list, printed full: `refuted by std.arith.division-by-zero-is-false`,
`(words.capacity cartons pencils)`), and a name and the order it sorts in on 10 (an unsat core's facts, a
class's facts, listed by name: `(std.digits.fs 7 0 0 7 0)` sorts after `(nz 7)` where `(fs …)` sorted
before). **`ein saturate --dump`**: byte-equal on 1 of 130 that load; equal as a set of lines, every prefix and
the padding taken off, on 112; 15 `std.num9` programs gain 580 lines, the calculator whole, which their
selection of `division-by-zero` brings under V3's closing (R unmoved); `readings`, `gaps` and `listed_is_all`
gain 2 435, `words` and the calculator. **`corpus_shapes.md5`**, which cannot be taken under a prototype
(the test refuses `$EIN_STDLIB`): the 300 tracked files that import hold 12 065 of its 24 479 digests, and
those that render the resolved program, a state, a trace or a rule can move with a name.

Each program's imports as (h) asks them, the names (h) adds in bold (129 rewritten, 4 already listing what
they write), and what `ein solve` moves:

| program | its imports under (h), the names (h) adds in bold | `ein solve` moves |
|---|---|---|
| `examples/arith/add.ein` | `std.arith` (arith **add** **n** **let** **sum-of**) | nothing |
| `examples/arith/div.ein` | `std.arith` (arith **div** **n** **mul** **product-of** **quot-of**) | nothing |
| `examples/arith/div_by_zero.ein` | `std.arith` (arith **let** **n** **add** **div**) | names |
| `examples/arith/factorial.ein` | `std.arith` (arith **let** **n** **add** **cmp** **cmp-of** **sum-of** **mul** **product-of**) | nothing |
| `examples/arith/int.ein` | `std.int` (int-arith **int-add** **int** **int-sub** **int-mul** **int-simplify** **int-neg** **int-div-euclid** **int-div-trunc** **int-div-floor** **int-cmp** **int-cmp-of** **int-sum-of** **int-diff-of** **int-product-of** **int-simple-of** **int-neg-of** **int-quot-euclid-of** **int-quot-trunc-of** **int-quot-floor-of**); `std.arith` (**n** **let**) | nothing |
| `examples/arith/long_numbers.ein` | `std.arith` (arith **let** **n** **b** **add** **sub** **cmp** **mul** **div** **sum-of** **diff-of** **cmp-of** **product-of** **quot-of**) | nothing |
| `examples/arith/many_calculations.ein` | `std.arith` (arith **add** **n** **sub** **cmp** **mul** **div** **sum-of** **diff-of** **cmp-of** **product-of** **quot-of**) | nothing |
| `examples/arith/mul.ein` | `std.arith` (arith **mul** **n** **let** **product-of** **add** **sum-of**) | nothing |
| `examples/arith/num9.ein` | `std.num9` (arith9 **add9** **num9** **sub9** **cmp9** **mul9** **div9** **product9-of** **sum9-of** **diff9-of** **cmp9-of** **quot9-of**) | nothing; dump 2084 → 2663 lines |
| `examples/arith/num9_both_ways.ein` | `std.arith` (arith **add** **n** **sum-of** **cmp** **cmp-of**); `std.num9` (arith9 **to9** **as9** **add9** **sum9-of** **num9** **from9** **of9**) | nothing |
| `examples/arith/num9_overflow.ein` | `std.num9` (arith9 **add9** **num9** **mul9** **overflow** **sum9-of** **product9-of**) | nothing; dump 2039 → 2619 lines |
| `examples/arith/rat.ein` | `std.rat` (rat-arith **rat-add** **rat** **rat-sub** **rat-div** **rat-simplify** **rat-mul** **rat-product-of** **rat-cmp** **rat-cmp-of** **rat-sum-of** **rat-diff-of** **rat-quot-of** **rat-simple-of**); `std.int` (**int**); `std.arith` (**n** **let**) | nothing |
| `examples/arith/sub.ein` | `std.arith` (arith **sub** **n** **let** **cmp** **diff-of** **cmp-of**) | nothing |
| `examples/beyond_arith/casting_out_nines.ein` | `std.arith` (arith **n** **cmp-walk** **order-is** **add-walk** **done** **div-call** **div-done**) | nothing |
| `examples/beyond_arith/collatz.ein` | `std.arith` (arith **n** **cmp-walk** **order-is** **div-call** **div-done** **add-walk** **mul-call** **done**) | nothing |
| `examples/beyond_arith/equation_two_unknowns.ein` | `std.arith` (arith **mul** **n** **product-of** **add** **sum-of**); `std.algebra` (total-owed functional) | nothing |
| `examples/beyond_arith/fibonacci.ein` | `std.arith` (arith **n** **cmp-walk** **order-is** **add-walk** **done**) | nothing |
| `examples/beyond_arith/gcd.ein` | `std.arith` (arith **n** **div-call** **div-done** **mul-call** **done**) | nothing |
| `examples/beyond_arith/laws.ein` | `std.arith` (arith **n** **add-walk** **mul-call** **done** **div-call** **div-done** **cmp-walk** **order-is**); `std.digits` (**nz**) | nothing |
| `examples/beyond_arith/perfect.ein` | `std.arith` (arith **n** **cmp-walk** **order-is** **add-walk** **done** **div-call** **div-done**) | nothing |
| `examples/beyond_arith/primes.ein` | `std.arith` (arith **n** **cmp-walk** **order-is** **add-walk** **done** **div-call** **div-done**) | nothing |
| `examples/beyond_arith/pythagorean.ein` | `std.arith` (arith **n** **cmp-walk** **order-is** **add-walk** **done** **mul-call**) | nothing |
| `examples/binary_arithmetic/both_bases.ein` | `std.arith` (arith **add** **n** **sum-of**); `bin` (bin **add2** **b2** **sum-of2**) | nothing |
| `examples/binary_arithmetic/conv.ein` | `std.arith` (arith **n** **div-call** **div-done** **add-walk** **done** **arith-rule**); `bin` (bin **a2** **strip2** **done2** **top-first2** **top-first-is2** **arith-rule2**) | nothing |
| `examples/binary_arithmetic/conversions.ein` | `conv` (conv **to-bin** **to-dec** **bin-of** **dec-of**); `std.arith` (**n**); `bin` (**b2**) | nothing |
| `examples/ein-bugs/wrong-answers/arity-nested-fact.ein` | `std.arith` (arith **add** **n** **sum-of**) | refused today and under (h) |
| `examples/ein-bugs/wrong-answers/arity-num9-eight-digits.ein` | `std.num9` (arith9 **add9** **num9** **sum9-of**) | refused today and under (h) |
| `examples/ein-bugs/wrong-answers/arity-request.ein` | `std.arith` (arith **add** **n** **sum-of**) | refused today and under (h) |
| `examples/word_problems/before.ein` | `words` (words **gave** **has** **asked** **amount** **value**); `gaps` (gaps **not-stated** **unanswered**); `std.arith` (**n**) | nothing |
| `examples/word_problems/bob1.ein` | `words` (words **has** **gave** **asked** **amount** **value**); `std.arith` (**n**) | nothing |
| `examples/word_problems/boxes12.ein` | `words` (words **has** **holds** **asked** **needed** **together** **value** **model-counts-only**); `readings` (readings **read-need** **reads-as** **pack-all** **whole-only**); `std.arith` (**n**) | nothing |
| `examples/word_problems/boxes13.ein` | `words` (words **has** **holds** **asked** **needed** **together** **value** **over** **model-counts-only**); `readings` (readings **read-need** **reads-as** **pack-all** **whole-only**); `std.arith` (**n**) | nothing |
| `examples/word_problems/bystander.ein` | `words` (words **has** **gave** **asked** **amount** **value**); `std.arith` (**n**) | nothing |
| `examples/word_problems/contra.ein` | `words` (words **has** **gave** **asked** **amount** **value**); `std.arith` (**n**) | names and lines |
| `examples/word_problems/gap.ein` | `words` (words **has** **gave** **asked** **together** **value** **amount**); `gaps` (gaps **unanswered**); `std.arith` (**n**) | nothing |
| `examples/word_problems/gap_assume.ein` | `words` (words **has** **gave** **asked** **together** **assumed** **amount** **value**); `gaps` (gaps **rests-on** **unanswered**); `std.arith` (**n**) | nothing |
| `examples/word_problems/gap_fact.ein` | `words` (words **has** **gave** **asked** **amount** **together** **value**); `std.arith` (**n**) | nothing |
| `examples/word_problems/gaps.ein` | `words` (asked wanted needs value assumed) | **R moves**; dump 172 → 2609 lines |
| `examples/word_problems/kinds.ein` | `words` (words **has** **asked** **together** **value** **amount**); `gaps` (gaps **unanswered**); `std.arith` (**n**) | nothing |
| `examples/word_problems/kinds_assume.ein` | `words` (words **has** **asked** **together** **assumed** **amount** **value**); `gaps` (gaps **rests-on**); `std.arith` (**n**) | nothing |
| `examples/word_problems/kinds_full.ein` | `words` (words **has** **asked** **together** **value**); `std.arith` (**n**) | names and lines |
| `examples/word_problems/kinds_listed.ein` | `words` (words **has** **asked** **together** **value** **amount** **assumed**); `gaps` (gaps **rests-on**); `listed_is_all` (listed-is-all); `std.arith` (**n**) | nothing |
| `examples/word_problems/listed_is_all.ein` | `words` (has amount assumed) | **R moves**; dump 150 → 2589 lines |
| `examples/word_problems/more.ein` | `words` (words **has** **asked** **apart** **has-more** **value**); `std.arith` (**n**) | nothing |
| `examples/word_problems/pencils.ein` | `words` (words **has** **holds** **packed-in** **gave** **there-are** **asked** **each-of** **amount** **rest-of** **value**); `std.arith` (**n**) | names and lines |
| `examples/word_problems/readings.ein` | `words` (wanted value left-over needed over); `std.arith` (**add** **n** **sum-of**) | names and lines; dump 174 → 2609 lines |
| `examples/word_problems/unused.ein` | `words` (words **has** **holds** **gave** **asked** **amount** **value**); `std.arith` (**n**) | nothing |
| `examples/word_problems/words.ein` | `std.arith` (arith **sub** **diff-of** **add** **sum-of** **cmp** **cmp-of** **mul** **product-of** **div** **quot-of**) | nothing |
| `stdlib/arith.ein` | `std.digits` (fa fs md cl nz mid adj) | nothing |
| `stdlib/int.ein` | `std.arith` (arith **operand** **operand-is** **sub-walk** **done** **add-walk** **mul-call** **cmp-walk** **order-is** **division-by-zero** **n** **div-call** **div-done**) | nothing |
| `stdlib/num9.ein` | `std.digits` (fa fs md cl nz adj); `std.arith` (n division-by-zero division-by-zero-is-false) | nothing; dump 1986 → 2566 lines |
| `stdlib/rat.ein` | `std.arith` (arith **operand** **operand-is** **cmp-walk** **order-is** **division-by-zero** **n** **div-call** **div-done** **mul-call** **done** **add-walk**); `std.int` (int-arith **int** **int-norm** **int-done** **int-mul-call**) | nothing |
| `tests/stdlib/arith/01_add.ein` | `std.arith` (arith **add** **n** **sum-of**) | nothing |
| `tests/stdlib/arith/02_carry_out.ein` | `std.arith` (arith **add** **n** **sum-of**) | nothing |
| `tests/stdlib/arith/03_lengths.ein` | `std.arith` (arith **add** **n** **sum-of**) | nothing |
| `tests/stdlib/arith/04_zero.ein` | `std.arith` (arith **add** **n** **sum-of**) | nothing |
| `tests/stdlib/arith/05_high_zeros.ein` | `std.arith` (arith **add** **n** **sum-of**) | nothing |
| `tests/stdlib/arith/06_named.ein` | `std.arith` (arith **let** **n** **add** **sum-of**) | nothing |
| `tests/stdlib/arith/07_only_what_is_asked.ein` | `std.arith` (arith **let** **n** **b** **add** **done** **sum** **sum-of**) | nothing |
| `tests/stdlib/arith/08_sub.ein` | `std.arith` (arith **sub** **n** **diff-of** **short** **shortfall** **difference**) | nothing |
| `tests/stdlib/arith/09_sub_borrow_and_zeros.ein` | `std.arith` (arith **sub** **n** **diff-of**) | nothing |
| `tests/stdlib/arith/10_sub_named.ein` | `std.arith` (arith **diff-of** **n** **let** **sub**) | nothing |
| `tests/stdlib/arith/11_cmp.ein` | `std.arith` (arith **cmp** **n** **cmp-of**) | nothing |
| `tests/stdlib/arith/12_cmp_lengths_and_names.ein` | `std.arith` (arith **let** **n** **cmp** **cmp-of**) | nothing |
| `tests/stdlib/arith/13_mul.ein` | `std.arith` (arith **mul** **n** **product-of**) | nothing |
| `tests/stdlib/arith/14_mul_zeros_and_carries.ein` | `std.arith` (arith **mul** **n** **product-of**) | nothing |
| `tests/stdlib/arith/15_mul_long_and_named.ein` | `std.arith` (arith **let** **n** **mul** **product-of** **done** **product**) | nothing |
| `tests/stdlib/arith/16_div.ein` | `std.arith` (arith **div** **n** **quot-of**) | nothing |
| `tests/stdlib/arith/17_div_every_digit.ein` | `std.arith` (arith **div** **n** **quot-of**) | nothing |
| `tests/stdlib/arith/18_div_small_and_zeros.ein` | `std.arith` (arith **div** **n** **quot-of**) | nothing |
| `tests/stdlib/arith/19_div_named_and_chained.ein` | `std.arith` (arith **let** **n** **mul** **div** **product-of** **quot-of**) | nothing |
| `tests/stdlib/arith/20_div_by_zero.ein` | `std.arith` (arith **add** **n** **div**) | names |
| `tests/stdlib/arith/21_div_by_zero_spelt_and_named.ein` | `std.arith` (arith **let** **n** **div**) | names |
| `tests/stdlib/arith/22_malformed_digit.ein` | `std.arith` (arith **add** **n**) | names |
| `tests/stdlib/arith/23_malformed_fact_as_digit.ein` | `std.arith` (arith **add** **n**) | names |
| `tests/stdlib/arith/24_malformed_ends_in_digit.ein` | `std.arith` (arith **add** **n** **sub**) | names |
| `tests/stdlib/arith/25_malformed_ends_in_zero.ein` | `std.arith` (arith **add** **n** **sub**) | names |
| `tests/stdlib/arith/26_malformed_ends_in_a_name.ein` | `std.arith` (arith **let** **n** **add** **cmp**) | names |
| `tests/stdlib/arith/27_malformed_digit_as_number.ein` | `std.arith` (arith **add** **n** **cmp**) | names |
| `tests/stdlib/arith/28_malformed_zero_digit_as_number.ein` | `std.arith` (arith **add** **n** **mul**) | names |
| `tests/stdlib/arith/29_undeclared_name.ein` | `std.arith` (arith **let** **n** **add** **mul**) | names |
| `tests/stdlib/arith/30_declared_name.ein` | `std.arith` (arith **let** **n** **add** **mul** **product-of** **undeclared** **malformed**) | nothing |
| `tests/stdlib/arith/31_malformed_through_a_name.ein` | `std.arith` (arith **let** **n** **add** **div**) | names |
| `tests/stdlib/arith/32_malformed_name_for_a_name.ein` | `std.arith` (arith **let** **n** **add** **div**) | names |
| `tests/stdlib/arith/33_let_twice.ein` | `std.arith` (arith **let** **n** **add**) | names |
| `tests/stdlib/arith/34_let_twice_the_same.ein` | `std.arith` (arith **let** **n** **add** **sum-of** **malformed**) | nothing |
| `tests/stdlib/arith/35_let_zero.ein` | `std.arith` (arith **let** **n**) | names |
| `tests/stdlib/arith/36_let_digit.ein` | `std.arith` (arith **let** **n** **add**) | names |
| `tests/stdlib/arith/37_let_zero_digit.ein` | `std.arith` (arith **let** **n** **add**) | names |
| `tests/stdlib/arith/38_what_stays_silent.ein` | `std.arith` (arith **n** **add** **b** **sum-of** **malformed** **undeclared**) | nothing |
| `tests/stdlib/arith/39_div_call_by_zero.ein` | `std.arith` (arith **div-call** **div-done** **n**) | names |
| `tests/stdlib/arith/40_div_call_by_zero_spelt.ein` | `std.arith` (arith **div-call** **div-done** **n**) | names and lines |
| `tests/stdlib/arith/41_div_call_not_zero.ein` | `std.arith` (arith **div-call** **div-done** **n**) | nothing |
| `tests/stdlib/int/01_simplify.ein` | `std.int` (int-arith **int-simplify** **int** **int-simple-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/02_neg.ein` | `std.int` (int-arith **int-neg** **int** **int-neg-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/03_add.ein` | `std.int` (int-arith **int-add** **int** **int-sum-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/04_sub.ein` | `std.int` (int-arith **int-sub** **int** **int-diff-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/05_mul.ein` | `std.int` (int-arith **int-mul** **int** **int-product-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/06_cmp.ein` | `std.int` (int-arith **int-cmp** **int** **int-cmp-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/07_unsimplified.ein` | `std.int` (int-arith **int-add** **int** **int-sub** **int-mul** **int-cmp** **int-sum-of** **int-diff-of** **int-product-of** **int-cmp-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/08_named_and_chained.ein` | `std.int` (int-arith **int** **int-sub** **int-mul** **int-cmp** **int-diff-of** **int-neg** **int-neg-of** **int-product-of** **int-cmp-of**); `std.arith` (**let** **n**) | nothing |
| `tests/stdlib/int/09_div_euclid.ein` | `std.int` (int-arith **int-div-euclid** **int** **int-quot-euclid-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/10_div_trunc.ein` | `std.int` (int-arith **int-div-trunc** **int** **int-quot-trunc-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/11_div_floor.ein` | `std.int` (int-arith **int-div-floor** **int** **int-quot-floor-of**); `std.arith` (**n**) | nothing |
| `tests/stdlib/int/12_div_euclid_by_zero.ein` | `std.int` (int-arith **int-add** **int** **int-div-euclid**); `std.arith` (**n**) | names and lines |
| `tests/stdlib/int/13_div_trunc_by_zero.ein` | `std.int` (int-arith **int-add** **int** **int-div-trunc**); `std.arith` (**n**) | names and lines |
| `tests/stdlib/int/14_div_floor_by_zero.ein` | `std.int` (int-arith **int** **int-add** **int-div-floor**); `std.arith` (**let** **n**) | names and lines |
| `tests/stdlib/num9/01_add.ein` | `std.num9` (arith9 **add9** **num9** **sum9-of**) | nothing; dump 2002 → 2582 lines |
| `tests/stdlib/num9/02_add_overflow.ein` | `std.num9` (arith9 **add9** **num9** **overflow** **sum9-of**) | nothing; dump 2004 → 2584 lines |
| `tests/stdlib/num9/03_sub.ein` | `std.num9` (arith9 **sub9** **num9** **diff9-of**) | nothing; dump 2005 → 2585 lines |
| `tests/stdlib/num9/04_cmp.ein` | `std.num9` (arith9 **cmp9** **num9** **cmp9-of**) | nothing; dump 2006 → 2586 lines |
| `tests/stdlib/num9/05_mul.ein` | `std.num9` (arith9 **mul9** **num9** **product9-of** **overflow**) | nothing; dump 2040 → 2620 lines |
| `tests/stdlib/num9/06_mul_overflow.ein` | `std.num9` (arith9 **mul9** **num9** **overflow** **product9-of**) | nothing; dump 2053 → 2633 lines |
| `tests/stdlib/num9/07_div.ein` | `std.num9` (arith9 **div9** **num9** **quot9-of**) | nothing; dump 2016 → 2595 lines |
| `tests/stdlib/num9/08_div_every_digit.ein` | `std.num9` (arith9 **div9** **num9** **quot9-of**) | nothing; dump 2037 → 2616 lines |
| `tests/stdlib/num9/09_div_by_zero.ein` | `std.num9` (arith9 **div9** **num9** **add9**) | names; dump 2010 → 2589 lines |
| `tests/stdlib/num9/10_to9.ein` | `std.num9` (arith9 **to9** **as9** **num9** **big**); `std.arith` (**n**) | nothing; dump 2034 → 2614 lines |
| `tests/stdlib/num9/11_from9.ein` | `std.num9` (arith9 **from9** **num9** **of9**); `std.arith` (**n**) | nothing; dump 2029 → 2609 lines |
| `tests/stdlib/num9/12_both_modules.ein` | `std.arith` (arith **add** **n** **sum-of** **cmp** **cmp-of**); `std.num9` (arith9 **to9** **as9** **add9** **sum9-of** **num9** **from9** **of9**) | nothing |
| `tests/stdlib/num9/13_chain.ein` | `std.num9` (arith9 **mul9** **num9** **product9-of** **div9** **quot9-of**) | nothing; dump 2042 → 2621 lines |
| `tests/stdlib/rat/01_simplify.ein` | `std.rat` (rat-arith **rat-simplify** **rat** **rat-simple-of**); `std.int` (**int**); `std.arith` (**n**) | nothing |
| `tests/stdlib/rat/02_neg.ein` | `std.rat` (rat-arith **rat-neg** **rat** **rat-neg-of**); `std.int` (**int**); `std.arith` (**n**) | nothing |
| `tests/stdlib/rat/03_add.ein` | `std.rat` (rat-arith **rat-add** **rat** **rat-sum-of**); `std.int` (**int**); `std.arith` (**n**) | nothing |
| `tests/stdlib/rat/04_sub.ein` | `std.rat` (rat-arith **rat-sub** **rat** **rat-diff-of**); `std.int` (**int**); `std.arith` (**n**) | nothing |
| `tests/stdlib/rat/05_mul.ein` | `std.rat` (rat-arith **rat-mul** **rat** **rat-product-of**); `std.int` (**int**); `std.arith` (**n**) | nothing |
| `tests/stdlib/rat/06_div.ein` | `std.rat` (rat-arith **rat-div** **rat** **rat-quot-of**); `std.int` (**int**); `std.arith` (**n**) | nothing |
| `tests/stdlib/rat/07_cmp.ein` | `std.rat` (rat-arith **rat-cmp** **rat** **rat-cmp-of**); `std.int` (**int**); `std.arith` (**n**) | nothing |
| `tests/stdlib/rat/08_unsimplified.ein` | `std.rat` (rat-arith **rat-add** **rat** **rat-sub** **rat-mul** **rat-div** **rat-cmp** **rat-sum-of** **rat-diff-of** **rat-product-of** **rat-quot-of** **rat-cmp-of**); `std.int` (**int**); `std.arith` (**n**) | nothing |
| `tests/stdlib/rat/09_named_and_chained.ein` | `std.rat` (rat-arith **rat** **rat-add** **rat-sum-of** **rat-mul** **rat-product-of** **rat-cmp** **rat-cmp-of**); `std.arith` (**let** **n**); `std.int` (**int**) | nothing |
| `tests/stdlib/rat/10_div_by_zero_rational.ein` | `std.rat` (rat-arith **rat-add** **rat** **rat-div**); `std.int` (**int**); `std.arith` (**n**) | names and lines |
| `tests/stdlib/rat/11_zero_denominator_first.ein` | `std.rat` (rat-arith **rat-add** **rat**); `std.int` (**int**); `std.arith` (**n**) | names |
| `tests/stdlib/rat/12_zero_denominator_second.ein` | `std.rat` (rat-arith **rat** **rat-add** **rat-mul**); `std.arith` (**let** **n**); `std.int` (**int**) | names and lines |
| `tests/stdlib/rat/13_zero_denominator_one_operand.ein` | `std.rat` (rat-arith **rat-add** **rat** **rat-neg**); `std.int` (**int**); `std.arith` (**n**) | names |

### What a selection brings: today's closure, (h)'s without (ii), and (h)'s

Over the 168 distinct selections of a `std.*` module the rewritten corpus writes (`closure.py`), the
declarations each brings, every module counted; the most written, and every row where the three differ in an
instructive way:

| selection | written | today | (h), no (ii) | (h), V3 | V3 by module |
|---|---:|---:|---:|---:|---|
| `std.arith` (n) | 42 | 1 | 8 | 197 | `std.arith` 190, `std.digits` 7 |
| `std.arith` (arith add n sum-of) | 8 | 197 | 197 | 197 | the same |
| `std.arith` (add n sum-of), the acceptance's | — | 3 | 10 | 197 | the same |
| `std.arith` (n division-by-zero division-by-zero-is-false), `std.num9`'s | 1 | 3 | 10 | 197 | the same |
| `std.int` (int) | 14 | 1 | 198 | 315 | `std.int` 118, `std.arith` 190, `std.digits` 7 |
| `std.num9` (arith9 add9 num9 sum9-of) | 2 | 78 | 79 | 266 | `std.num9` 69, `std.digits` 7, `std.arith` 190 |
| `std.macro` (forall) | 22 | 1 | 1 | 1 | |
| `std.algebra` (total-owed) | 18 | 1 | 2 | 2 | `std.algebra` 1, `std.macro` 1 |
| `std.algebra` (symmetric transitive includes bijective-properties), zebra2's | 5 | 9 | 9 | 9 | V1: 20, with nine lemmas |
| `std.algebra` (converse-illtyped-dom converse-illtyped-ran converse) | 4 | 3 | 4 | 4 | V1: 13, `converse-pair-symmetric` among them |
| `std.bijection` (bijective-setup typecheck-setup) | 6 | 15 | 16 | 16 | `std.bijection` 8, `std.algebra` 7, `std.macro` 1 |
| `std.bijection` (typecheck-arg-0 typecheck-arg-1) | 3 | 2 | 10 | 10 | the union of edges: `std.bijection`'s own imports |
| `std.slots` (slot-partition-setup slot-spatial-setup) | 2 | 21 | 21 | 21 | |
| `std.typing` (type-hierarchy-converse) | 2 | 4 | 5 | 5 | with its new import of `std.algebra` |

So for the calculator a selection is the whole theory as soon as it lists a relation the circuit reads or
writes, its number `n` included, and for the zebra modules (rules only, no declared relation) (ii) under V3
brings nothing, the union of edges what their own imports list. Under V1 the zebra modules' lemmas come with any
activator name.

### Defaults and questions

Filed in [`open_questions.md`](../open_questions.md), each with the evidence above. **Open, the coordinator's or
the user's** (the stop): Q-M1f.247 (what (ii) follows), Q-M1f.248 (a closing rule), Q-M1f.249 (the moves that
remain). **Defaults T2 would take**, the user's to revise: Q-M1f.250 (a listed stated relation), Q-M1f.251 (a
prefixed name that names nothing), Q-M1f.252 (`..` and `/` in a module name), Q-M1f.253 (the alias spelling
under `:as`). Finding [E-53](findings.md): `std.algebra`'s converse lemma and the subtype-directional converse
check do not compose, today, with the lemma listed. No register row: no answer of the engine is wrong; every
move above is the language's.

### The moves under V2, shown

**The ruling, 2026-11-14**, on Q-M1f.247 and .248, quoted in § The ruling: the closure follows **any** relation
and a closing rule concludes what it closes, the prototype's **V2**; the rule the closure satisfies is that **a
selected short name means exactly what its full name means under a whole-module import**. On Q-M1f.249 the user
asked *"Show me first"*: this section shows every program V2 moves, and nothing is restated or accepted.

**Re-taken under V2 exactly** (`build/s1f.6.35/v2.sh`, 2026-11-14: `t1.py` with `H_II_RELATIONS_ONLY=0`, all
854 tracked files, then `t1test.py`, `ein test` on every file today and on its (h) text). Of the 514 programs
that load today, R or exit moves on **5**, the five of V2 above, and nothing else: R equal on 509, the
read-out byte-equal on 477 and equal once module prefixes are off on 499. Under `ein test`, of the 267 files
that load and state a claim, the verdicts move on **2**, the same two as in `ein solve`
(`tests/imports/02_nested_import_left_behind.ein`, `tests/stdlib/typing/03_type_hierarchy_welltyped.ein`),
and on none other. No file refused today loads.

*Before* is `ein solve FILE` (and `ein test FILE`) in the checkout on the release binary of `4676a5de`;
*after* is the same command on the same binary in `build/s1f.6.35/flat/`, where FILE is the program's (h)
text, written by `h.py` under V2 from the program with its imports as (h) asks them. Each output is trimmed to
the lines that differ, verbatim.

| program | its imports today | under (h) | why it moves |
|---|---|---|---|
| `tests/stdlib/typing/03_type_hierarchy_welltyped.ein` | `(import std.algebra :symbols (converse-illtyped-dom converse-illtyped-ran))` `(import std.typing :symbols (type-hierarchy-converse))` | `std.algebra` lists `converse` too, which the program writes; `std.typing` unchanged | `converse` brings the rules that read or conclude it, `converse-pair-symmetric` among them, which derives `(converse owned-by owns)`; `std.typing`'s knob checks that pair too, and its range check fails under the hierarchy: E-53, below |
| `tests/imports/02_nested_import_left_behind.ein` | `(import upper_without_entry :symbols (upper))` | `(import upper_without_entry :symbols (upper wanted got))` | `upper_without_entry`'s own `(import lower :symbols (lower …))` is an edge of the program and brings `lower` whole, so the request is answered; E-20 dissolves |
| `examples/word_problems/gaps.ein` | `(import words :symbols (asked wanted needs value assumed))` | the same | the five relations bring `words`' rules that read or write them and, by the closing, `words`' closing rule, which closes `asked` and `wanted`: the blind enumerator has nothing left to propose |
| `examples/word_problems/listed_is_all.ein` | `(import words :symbols (has amount assumed))` | the same | the same: `assumed` closed by `words`' closing rule |
| `examples/features/09_adjacent_via_same_house.ein` | `(import std.algebra :symbols (symmetric transitive includes))` `(import std.bijection :symbols (typecheck-arg-0 typecheck-arg-1))` | the same | R unmoved. `std.bijection`'s own `(import std.algebra :symbols (… total-owed surjective-owed))` is an edge of the program, so an obligation rule is loaded, and by E-51 the ladder takes the obligations rung, which owes nothing here |

**`tests/stdlib/typing/03_type_hierarchy_welltyped.ein`.** `ein solve`, before and after:

```text
<   states (k_s)      1
<   commitments (k_c) 1
---
>   states (k_s)      0
>   commitments (k_c) 0
<   ∅                 in R — root is alive and owes nothing
<   minimal line      1 state — ∅
<   maximal line      1 state
<   components        1
<   sub-cubes         1, of dimension 0
<   merged            none — every state is reached by one commitment, so k_c = k_s
<   N                 0 minimal dead sets
---
>   empty             every node is dead — root's own fixpoint derives (false)
<   work              1 saturation · 0 facts beyond root · 2 layers
---
>   work              1 saturation · 0 facts beyond root · 1 layer
<   (region
<     ;; state 1 of 1 — ∅, root itself · minimal and maximal
<     (model))
---
>   unsat core        (converse owns owned-by)
>   unsat core        (relation owned-by Animal Person)
>   unsat core        (relation owns Person Pet)
>   refuted by        converse-illtyped-ran
>
>   (false)
<   over              all of R — 1 state → 1 projection class
<     class 1 of 1    1 state (1)
<                       ?what = Rex · ?who = Ann
<   as :expect        (model (owns Ann Rex))
---
>   over              all of R — R is empty, root is dead: no state to answer from
>   as :expect        (false)
<   (solve :expect)   holds
---
>   (solve :expect)   FAILED
>                     the claim lists 1 state and R is empty
> tests/stdlib/typing/03_type_hierarchy_welltyped.ein: (solve :expect) FAILED — the claim lists 1 state and R is empty
```

`ein test`, before (exit 0) and after (exit 1):

```text
ok           tests/stdlib/typing/03_type_hierarchy_welltyped.ein
1 file, 1 expectation: 1 held, 0 FAILED, 0 not checked, 0 errors  (0.01 s)
```

```text
FAILED       tests/stdlib/typing/03_type_hierarchy_welltyped.ein
  solve · :expect (region …) — FAILED
    the claim lists 1 state and R is empty
1 file, 1 expectation: 0 held, 1 FAILED, 0 not checked, 0 errors  (0.00 s)
```

**`tests/imports/02_nested_import_left_behind.ein`.** `ein solve`, before and after:

```text
< note: tests/imports/02_nested_import_left_behind.ein:25:9: (import upper_without_entry :symbols (upper)) brings the declaration of double-of and no rule that derives it: upper_without_entry has such a rule, its own or one it imports, and the selection names none — a selection brings the names it lists and what their declarations write, so list the rule, or the entry that names it, if double-of is to be derived
<     class 1 of 1    1 state (1) — the goal has no row: undetermined here
<   as :expect        (model)
<   :expect           holds
---
>     class 1 of 1    1 state (1)
>                       ?x = a · ?y = aa
>                         “a's double is aa”
>   as :expect        (model (got a aa))
>   :expect           FAILED — the claim lists 1 class, the query has 1: (model (got a aa)) is not listed
>                     (model (got a aa)) is a class the claim does not list
>                     the claim lists (model), which is no class
> tests/imports/02_nested_import_left_behind.ein: query :expect FAILED — the claim lists 1 class, the query has 1: (model (got a aa)) is not listed
```

`ein test`, before (exit 0) and after (exit 1):

```text
note: tests/imports/02_nested_import_left_behind.ein:25:9: (import upper_without_entry :symbols (upper)) brings the declaration of double-of and no rule that derives it: upper_without_entry has such a rule, its own or one it imports, and the selection names none — a selection brings the names it lists and what their declarations write, so list the rule, or the entry that names it, if double-of is to be derived
ok           tests/imports/02_nested_import_left_behind.ein
1 file, 2 expectations: 2 held, 0 FAILED, 0 not checked, 0 errors  (0.01 s)
```

```text
FAILED       tests/imports/02_nested_import_left_behind.ein
  :goal (got ?x ?y) — FAILED
    the claim lists 1 class, the query has 1: (model (got a aa)) is not listed
    (model (got a aa)) is a class the claim does not list
    the claim lists (model), which is no class
1 file, 2 expectations: 1 held, 1 FAILED, 0 not checked, 0 errors  (0.01 s)
```

Its solve claim, `(region (model))`, holds before and after. The `note:` line goes because the prototype's
text has no import left to note; whether the resolver keeps E-20's note at all is T2's (nothing is left behind
of a module's own relations under V2).

**`examples/word_problems/gaps.ein`.** No claim; its corpus entry runs `saturate` and `render rules`, no
solve. `ein solve`, before and after:

```text
<   states (k_s)      4
<   commitments (k_c) 4
---
>   states (k_s)      1
>   commitments (k_c) 1
<   maximal line      1 state — ⊤, the trivial line: no restriction bites
---
>   maximal line      1 state
<   sub-cubes         1, of dimension 2
---
>   sub-cubes         1, of dimension 0
<   universe          2 candidates, 0 dead alone
---
>   universe          no candidate: R is {∅} or empty, decided by root alone
<   work              4 saturations · 7 facts beyond root · 4 layers
---
>   work              1 saturation · 0 facts beyond root · 2 layers
<     ;; state 1 of 4 — ∅, root itself · minimal
<     (model)
<     ;; state 2 of 4 — 1 candidate · from {(asked __closed__)}
<     (model (asked __closed__))
<     ;; state 3 of 4 — 1 candidate · from {(wanted __closed__)}
<     (model (wanted __closed__))
<     ;; state 4 of 4 — 2 candidates · maximal · from {(asked __closed__), (wanted __closed__)}
<     (model (asked __closed__) (wanted __closed__)))
---
>     ;; state 1 of 1 — ∅, root itself · minimal and maximal
>     (model))
```

**`examples/word_problems/listed_is_all.ein`.** No claim; its corpus entry runs `saturate` and `render
rules`, no solve. `ein solve`, before and after:

```text
<   states (k_s)      4
<   commitments (k_c) 4
---
>   states (k_s)      1
>   commitments (k_c) 1
<   maximal line      1 state — ⊤, the trivial line: no restriction bites
---
>   maximal line      1 state
<   sub-cubes         1, of dimension 2
---
>   sub-cubes         1, of dimension 0
<   universe          2 candidates, 0 dead alone
---
>   universe          no candidate: R is {∅} or empty, decided by root alone
<   work              4 saturations · 4 facts beyond root · 4 layers
---
>   work              1 saturation · 0 facts beyond root · 2 layers
<     ;; state 1 of 4 — ∅, root itself · minimal
<     (model)
<     ;; state 2 of 4 — 1 candidate · from {(assumed __closed__ z)}
<     (model (assumed __closed__ z))
<     ;; state 3 of 4 — 1 candidate · from {(assumed z __closed__)}
<     (model (assumed z __closed__))
<     ;; state 4 of 4 — 2 candidates · maximal · from {(assumed __closed__ z), (assumed z __closed__)}
<     (model (assumed __closed__ z) (assumed z __closed__)))
---
>     ;; state 1 of 1 — ∅, root itself · minimal and maximal
>     (model))
```

**`examples/features/09_adjacent_via_same_house.ein`.** No claim. `ein solve`, before and after:

```text
<   universe          82 candidates, 82 dead alone
---
>   universe          no candidate: R is {∅} or empty, decided by root alone
>     outside         pet-loc — no candidate: no obligation owes it. R holds of it only what the program states and saturation derives
```

### E-53 under the ruling: the fix to `std.algebra`

**It is no longer hypothetical.** Under the rule the closure satisfies, a program that writes `(converse R1
R2)` lists `converse`, and the selection holds every rule of `std.algebra` that reads or concludes it, as a
whole-module import would: `converse-pair-symmetric` among them. So `tests/stdlib/typing/03`, a pair
well-typed through the hierarchy, is refuted once (h) lands, as it is today by a program that lists the lemma
(`build/s1f.6.35/e53/welltyped_with_lemma.ein`). **That is a wrong answer of the shipped `std.algebra`, and its
register row is filed `open`, with its witness and corpus entry, in a commit of its own, before any fix: with
T2, once the user has seen the moves.** Nothing in the stdlib or the register moves in this section.

**Why the two rules do not compose.** The header reads `(converse R1 R2)` as an equality, *R2 = R1°*, and
`converse-pair-symmetric` makes it one: `(converse R1 R2) ⟹ (converse R2 R1)`. The two checks read it one way:
`converse-illtyped-dom` refutes unless `range(R1)` is `domain(R2)` or a subtype of it, `-ran` unless
`domain(R1)` is `range(R2)` or a subtype of it, which is right for the rule `converse` alone, R2 ⊇ R1°. Under a
proper subtype the pair passes and its mirror cannot: `owns : Person → Pet`, `owned-by : Animal → Person`,
`(is-a* Pet Animal)`; the mirror asks `Animal ⊑ Pet`. `std.typing`'s knob derives the check's activators for
every `(converse …)`, the lemma's mirror among them; the per-pair activators of `tests/stdlib/algebra/16` and
`17` name one orientation and never meet the lemma.

**Three ways, each tried on the prototype** (`build/s1f.6.35/e53/`, 2026-11-14: the stdlib of `tree/` with
`algebra.ein` edited, `F1/` and `F3/`; the twins under V2 by `run.sh`; every tracked program that brings
`std.algebra`, 104, by `sweep.py`, R against today's). The twins: `t03` is `typing/03`; `t03r` its mirror
(`owns : Person → Animal`, `owned-by : Pet → Person`), well-typed only read backwards; `t03eq` exact both ways
(`owned-by : Pet → Person`); `t02`, `a16`, `a17` the three ill-typed fixtures (`Pet` against `Place`, `Person`
against `Place`).

| | `t03` | `t03r` | `t03eq` | `t02`, `a16`, `a17` | the 104 programs that bring `std.algebra` |
|---|---|---|---|---|---|
| today | 1 state | refuted, `converse-illtyped-dom` | 1 state | refuted, each by its own check | — |
| (h), the stdlib as shipped | **refuted, `converse-illtyped-ran`** | refuted, `-dom` | 1 state | refuted, each by its own check | R moves on `typing/03` (and `features/09`'s universe row, E-51's, not this) |
| **F1**, the checks direction-free | 1 state | **1 state** | 1 state | refuted, each by its own check | **none moves** but `features/09`'s universe row |
| F3, the lemma mirrors edges and derives no `(converse R2 R1)` | 1 state | refuted, `-dom` | 1 state | refuted, each by its own check | **`tests/stdlib/algebra/05_tag_lemmas.ein` fails**: its claim lists `(converse q p)`, which the lemma no longer derives |

- **F1, the checks read the pair as the equality the header states.** Each check refutes only where the two
  types are neither equal nor one a subtype of the other: `converse-illtyped-dom` gains `(absent (?isR* ?D2
  ?Rn1))` beside `(absent (?isR* ?Rn1 ?D2))`, `-ran` gains `(absent (?isR* ?Rn2 ?D1))`, and each `:why` says
  *neither equal nor one a subtype of the other*. A pair and its mirror check alike, so the lemma and the knob
  compose, and *up to the supplied subtype relation* holds of both orientations. What it gives up: `t03r`, a
  pair whose rule `converse` alone could put an `Animal` where `owns` takes a `Pet`, passes, where today's
  one-way check refutes it; no tracked program is such a pair. Two rules' `:match` and `:why` move; no
  fixture.
- **F2, nothing in the library moves; `typing/03` is restated.** Under the equality the header states, a pair
  is well-typed only where each signature is the other's reversed both ways, and the shipped checks with the
  lemma say exactly that: `typing/03`'s claim was written for the rule `converse` without its lemma, which a
  selection could leave out until (h). Its pair made exact (`t03eq`) or its claim `(false)`. The library's
  *"up to the supplied subtype relation"* then holds of no pair with a proper subtype wherever the lemma is
  loaded, which is every program that writes `converse` under (h).
- **F3, the lemma mirrors the edges and not the pair.** `converse-pair-symmetric` derives the activator of a
  new rule `converse-back (?R1 ?R2)`, `(?R2 ?b ?a) → (?R1 ?a ?b)`, and no `(converse R2 R1)`, so the knob checks
  each pair as written, one way, as today (`t03r` still refuted). A parameterless rule cannot do it in one step:
  a relation written as a variable must be a parameter its activator binds (the loader's refusal, tried). It
  changes the lemma's own promise, *(converse R1 R2) ⟹ (converse R2 R1)*, and
  `tests/stdlib/algebra/05_tag_lemmas.ein`, its conformance test, fails.

**Recommended: F1.** It is the one that makes the library agree with itself — the header's equality, the lemma
that states it and the checks' *up to the subtype relation* — without restating a fixture or a lemma's
promise, and the one tracked answer it touches is `typing/03`'s, which it returns to today's. Its cost, `t03r`, is the mirror of a
pair it accepts, which under an equality is the same pair. With T2: the register row `open` first (its witness
`typing/03` under (h), or the lemma listed by hand, today), then F1 in the fixing commit, the row flipped, and
the stdlib's manifest re-taken (`utils/stdlib_manifest.py --write`) and `utils/stdlib_mutants.py --check` run.
