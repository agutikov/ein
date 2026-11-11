# S1f.6.2 — The digits and the hardcoded tables, as a library of facts

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-11** (`3bd448a5`): `stdlib/digits.ein`, 1 664 rows in seven tables, generated, held to Rust's
integers, importable flat and closed; four decisions taken by default, Q-M1f.145 to .148, and two findings, E-9
and E-10 ([§ T1 record](#t1-record-2026-11-11) onward). Planned 2026-11-08; re-planned 2026-11-11 to
[the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.11](s1f.6.11_importing_facts.md), so a library of facts can be imported flat.

## What it builds

The theory's gates, as **plain facts** (the user, 2026-11-11: *"yes plain facts"*), in a module of their own,
`stdlib/digits.ein`, imported as `std.digits`. It is the stdlib's first library of facts, the *ontology* half of
[`solution_semantics.md` § 1.1](../../../docs/kernel/inference/solution_semantics.md#11-context-and-problem), and
both arithmetic modules import it: `std.arith` (S1f.6.3 to S1f.6.6) and `std.num9`
([S1f.6.13](s1f.6.13_num9.md)).

| table | reads | gives | rows |
|---|---|---|---:|
| `fa` | a, b, carry in (0 or 1) | sum digit, carry out | 200 |
| `fs` | a, b, borrow in (0 or 1) | difference digit, borrow out | 200 |
| `md` | a, b, carry in (0 to 8) | product digit, carry out | 900 |
| `cl` | the order below (`lt`, `eq`, `gt`), a, b | the order: a higher differing digit overrules | 300 |
| `nz` | a digit | it is not zero | 9 |
| `mid` | two bounds from 0 to 10, at least two apart | the digit to try between them | 45 |
| `adj` | two bounds | nothing lies between them | 10 |

These are the tables of the prototype's `build/s1f.6/prototype/r3/k.ein`
([design.md § Round 3](design.md#round-3-2026-11-11-several-representations-at-once)). The digit product with a
carry in replaces the plan's separate product and short-division tables: division bisects the divisor's
multiples and needs neither. `std.num9` adds its step counter in its own module.

## Tasks

1. **The generator.** `utils/gen_arith_tables.py` writes the module; `--check` is a gate step and fails when the
   file differs from what the script writes (the user's ruling on question 7, (a)).
2. **The module.** Every relation declared with a `:why`, every row a fact, and the module closes its own
   relations with written `(__closed__ R)` facts (question 6, (a)).
3. **The test that holds the rows:** each agrees with the harness's integers, none is missing, none is twice.
4. **The corpus and the stdlib's gates.** A module of facts has no rule for `stdlib_coverage.rs` to ask about;
   the stage says so on `tests/README.md` and adds one program under `tests/stdlib/digits/` that imports the
   module flat and claims a row of each table, which is also S1f.6.11's fix seen from the stdlib.
5. **The page:** the module's section of `stdlib/README.md`, as the first facts library.

## Acceptance

- Every table, its rows held to the integers by a test, and `--check` in the gate.
- A program that imports `std.digits` and nothing else: H empty, k_s 1, R {∅}.
- A core that passes through a table names its rows, which is the measured cost of plain facts
  ([design.md § G](design.md#g-the-flat-import-finding-a-specification-for-importing-facts)); the stage records
  one such core.
- No engine change in this stage.

## T1 record, 2026-11-11

**The generator** is [`utils/gen_arith_tables.py`](../../../utils/gen_arith_tables.py): it writes
`stdlib/digits.ein`, `--check` exits 1 when the file is not what it writes, `--stdout` prints it. The module is
a header, seven `(relation …)` declarations each with a `:why`, one line of seven written `(__closed__ R)`
facts, and the rows, 1 664 of them (`python3 utils/gen_arith_tables.py`, 2026-11-11): `fa` 200, `fs` 200, `md`
900, `cl` 300, `nz` 9, `mid` 45, `adj` 10. The file is 28.8 KB, and `stdlib/MANIFEST.sha256` lists eight modules.

**How `--check` is in the gate** ([Q-M1f.145](../open_questions.md#q-m1f145--how-the-table-generators-check-is-in-the-gate),
defaulted): by a cargo test, `ein-ir/tests/digit_tables.rs`'s `the_file_is_what_the_generator_writes`, the way
`examples/gen_zebra2_variants.py --check` is in it through `cli_semantics.rs`. The per-commit job already
installs a `python3` for that one; no step was added to `run_tests.sh`, `per-commit.yml` or `gate_steps.rs`.

## T2 record, 2026-11-11

**The module closes its own tables with written facts,** as task 2 says, and what that means for an importer
is [Q-M1f.146](../open_questions.md#q-m1f146--how-stddigits-closes-its-tables-for-a-flat-importer) (defaulted).
By S1f.6.11's Q-M1f.142 a `(__closed__ R)` fact is a fact of the relation `__closed__`, and a selection brings
it when that name is in the closure. Probed on the release binary (2026-11-11):

| the import | H | `ein solve` |
|---|---|---|
| `(import std.digits :symbols (fa nz __closed__))` | no candidate | k_s 1, R {∅}; all seven closing facts arrive, the five about tables left behind harmless |
| `(import std.digits :symbols (fa nz adj))` | `nz` and `adj` are open | exit 1, *a state's commitments pass 2^128: the state route cannot count them* |
| `(import std.digits :as d)` | no candidate | k_s 1, R {∅} |
| `ein solve stdlib/digits.ein` | no candidate | k_s 1, R {∅} |
| a theory whose closing rule asserts `(__closed__ fa)` and is the one name listed | no candidate | k_s 1, R {∅} (S1f.6.3's `arith`) |

So a program that imports the tables directly lists `__closed__` beside them, and the module's header, its
section of `stdlib/README.md` and the test program say so. A theory that reads the tables closes the ones it
reads in its closing rule, so its importer lists one name. Only `nz` and `adj` are ever candidates: a relation
of three or more arguments is outside H either way.

**The acceptance's program** is `tests/stdlib/digits/01_tables.ein`: it imports `std.digits` and nothing else,
all seven tables and `__closed__`, states no `:hypothesis-relations` list, and claims R = {∅} on its `(solve …)`
form: *universe: no candidate*, k_s 1, k_c 1 (`ein solve tests/stdlib/digits/01_tables.ein`, 2026-11-11).

## T3 record, 2026-11-11

`ein-ir/tests/digit_tables.rs`, four tests:

- `every_row_agrees_with_the_integers`: each table computed again from Rust's integers and compared as sorted
  lists, so a wrong row, a missing one, an extra one and one written twice each fail, naming the first row that
  differs. Tried: `(fa 7 5 0 2 1)` changed to `(fa 7 5 0 3 1)` fails it, and the two below.
- `the_module_is_seven_closed_tables_and_nothing_else`: seven declarations, one `(__closed__ R)` for each, the
  rows, and no other form.
- `the_file_is_what_the_generator_writes`: the script's `--check`.
- `a_flat_import_brings_a_table_from_the_embedded_copy`: `(import std.digits :symbols (fa nz __closed__))`
  resolved against `Source::Embedded`, the tier a release binary uses: `fa`'s 200 rows and `nz`'s 9, the seven
  closing facts, and no row of a table not listed.

## T4 record, 2026-11-11

- **`stdlib_coverage.rs`** has no rule to ask about: `std.digits` is named in `RULE_FREE_MODULES`, beside
  `std.macro`, so a module that parses to no rule by accident still fails. `utils/stdlib_mutants.py` mutates
  rules and has none here: its table is unmoved, 167 of 217 killed and the bank's 50 lines as they were
  (`python3 utils/stdlib_mutants.py --check`, 2026-11-11).
- **Two programs** under `tests/stdlib/digits/`, each a corpus entry of group `stdlib` with the five runs, and
  `stdlib/digits.ein` an entry with the modules' four. `01_tables.ein` is the acceptance's program, with a
  lookup of each table claimed and `nz` and `adj` claimed whole. `02_a_core_names_its_rows.ein` is the
  acceptance's core:

  ```text
    unsat core        (column 7 5 3)
    unsat core        (fa 7 5 0 2 1)
    refuted by        column-wrong
  ```

  `(column 7 5 3)` claims that 7 + 5 has the digit 3, a rule looks the column up in `fa` and concludes
  `(false)`, and the core names the claim and the row (`ein solve
  tests/stdlib/digits/02_a_core_names_its_rows.ein`, 2026-11-11).

**What the module costs.**

| | before | after | re-take |
|---|---:|---:|---|
| a program that does not import it | — | nothing: a module is read when an import names it | — |
| `ein test` of a `std.macro` test, for scale | 5.8 ms | | best of ten, release, 2026-11-11 |
| `ein test tests/stdlib/digits/01_tables.ein` (all seven tables) | — | 19.2 ms | the same |
| `ein test tests/stdlib/digits/02_a_core_names_its_rows.ein` (`fa` alone) | — | 17.9 ms | the same |
| `ein test tests/` | 143 ms | 171 ms, the two programs | the same |
| `dot_wellformed.rs`, per commit | 135.9 s | 171.9 s | `cargo test -p ein-render --test dot_wellformed`, 2026-11-11; the *before* is `build/gate-s1f.6.11.log`'s |
| `corpus_shapes.rs`, per commit | 78.6 s | 80.9 s | the same logs |

An importer pays the parse of the whole 28.8 KB file whichever tables it lists, about 12 ms; a module per table
would cut that for a program that reads one, and is not what the stage file says
([Q-M1f.148](../open_questions.md#q-m1f148--one-module-for-the-seven-tables), defaulted).

**No picture of a table can be drawn, and the per-commit DOT sweep had to be told**
([Q-M1f.147](../open_questions.md#q-m1f147--the-dot-sweep-and-a-view-no-layout-finishes), defaulted; E-10 of
[`findings.md`](findings.md)). Every view of `stdlib/digits.ein`, the program views and the six KB views, is
some 7 900 edges onto eleven shared names. `dot` 16.0.0 had not drawn one after five minutes, `fdp`, which a KB
view suggests, not after 100 s, and `sfdp` dumps core; `dot_wellformed.rs` puts every view of every corpus file
through `dot`, so the gate did not finish. Since this stage a view of more than 1 500 edges is put to Graphviz
under `fdp` stopped after one iteration, which reads the same grammar, renders the same attributes and gives
the same warnings, and takes two seconds on the largest; no view the corpus had before is past the bound, the
largest having 361 edges. `dot`'s own layout of a table is what the check no longer asks for. On the runner
image's Graphviz, 14.1.2 in its x86-64-v3 build, the bounded layout draws the same views in 2.1 s and 0.8 s
with no warning (`ein-ci:ubuntu-26.04-amd64v3`, 2026-11-11), and the whole sweep is green there on the stage's
commit, 126.8 s (`utils/ci_container.sh --tests-only -p ein-render --test dot_wellformed` on `3bd448a5`,
2026-11-11). The review bundle's region gallery draws corpus
programs' KB views with a 60 s limit and was not re-run here: it is the dev host's.

**A second drawing Graphviz would not take** is E-9: the first spelling of `02`'s rule, the `column` premise
before the `fa` one, drew with *Warning: Unable to reclaim box space in spline routing for edge "?claimed_L" ->
"?digit_L"* on `dot` 16.0.0, which `dot_wellformed.rs` counts as a refusal. The premises are written the other
way round, which draws, on 16.0.0 and on the runner image's 14.1.2, where the first spelling warns too.

**The goldens that move,** named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt`, 14 new cells
(the module's four runs, `render rules` exiting 1, *no rule forms*, as `std.macro`'s does, and the two
programs' five each), and `ein-render/tests/golden/corpus_shapes.md5`, the three new files' lines alone. No
line of an existing entry.

**The sweep.** `python3 build/s1f.6.2/sweep.py` (2026-11-11): the release binary built at `ed398cc5`, kept as
`build/s1f.6.2/ein-old`, against the tree's, `ein saturate --dump` and `ein solve`, over the 672 `.ein` files of
the tree: 0 files moved. The engine did not change, and both binaries read the checkout's stdlib, so what the
sweep holds is that adding the module and re-taking the manifest moved no existing program.

## T5 record, 2026-11-11

Pages: `stdlib/README.md` (§ `std.digits`, new, the modules table and § Importing), `tests/README.md` (the
directory's row, and § A module of facts has no rule to activate, new), `docs/kernel/ir/03-ein-lang/07_stdlib_api.md`
(§ `std.digits`, new), `utils/README.md`, `corpus/corpus.toml`, the root `README.md`, `AGENTS.md`,
[`findings.md`](findings.md)'s E-9 and E-10, the questions Q-M1f.145 to .148, and this record.

## Acceptance re-taken, 2026-11-11

- **Every table, its rows held to the integers by a test, and `--check` in the gate:** `digit_tables.rs`.
- **A program that imports `std.digits` and nothing else: H empty, k_s 1, R {∅}:**
  `tests/stdlib/digits/01_tables.ein`, its claim held by `ein test` and by the corpus sweep.
- **A core that passes through a table names its rows:** `02_a_core_names_its_rows.ein`, above.
- **No engine change:** no file under `ein.rs/crates/*/src` moved; the two test files that did are
  `ein-infer/tests/stdlib_coverage.rs` (the rule-free list) and `ein-render/tests/dot_wellformed.rs` (the bound).
