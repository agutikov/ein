# S1f.21.25 — an activator that names no relation activates nothing, in silence

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-24 on the user's ruling of review item 82
([the README](README.md#the-answers-to-part-1s-items-66-to-82)); **built** 2026-11-24 (the record below), gate green.
**Writes:** `ein-ir/src/from_ir.rs`, `ein-infer/src/compile.rs` and its three callers, `ein-cli/src/common.rs`, their
tests; the decisions and pages that say a line is printed; the headers of the programs that show one.

## What it is

An activator that binds a rule's relation parameter to a fact, `(symmetric (H1 H2))` for a rule
`symmetric (?rel)` whose `?rel` heads a pattern, activates nothing: the engine compiles no plan for it. Today three
`warning:` lines say so:

| line | printed where | decided by |
|---|---|---|
| the load's, for one a rule *could* derive | read off the program's text, on every command | Q-M1f.1, with Q-M1f.41 and Q-M1f.42 |
| the written one's, for one the program writes | at load | Q-M1e2.18, its second half |
| the run's, for one a state holds | when a run ends, placed at the rule | Q-M1f.1 |

**The user ruled all three gone** (*"I think this warning is false by desing. We do use relations as arguments.
And there could be any number of not activated rules if program imports something but not use."*, and, asked back
of the other two, *"All three go (Recommended)"*). The first fires on valid programs: review item 82's
`zebra2-about-1.ein`, where it names `std.algebra.imply1`, which nothing activates; `imply1`'s own use toward a
rule, `(imply1 equivalence symmetric)`, which works; and item 81's programs, eighteen lines of rules with no plan.
The item's page and programs are in `git log`, last at `0f7190b9`.

So such an activator says nothing, as an activator of another count says nothing
([Q-M1f.115](../../../docs/decisions/language/Q-M1f.115-an-activator-is-a-fact-of-its-rules-name-and-one.md)).
**No answer and no exit code moves**: Q-M1f.1 already says none moves for a line.

## Tasks

1. **The lines go, the reading stays.**
   - `ein-ir/src/from_ir.rs`: `warn_derived_relation_activators` writes no warning. The positions it reads stay,
     for their two other readers: `refuse_eq_at_another_arity`
     ([Q-M1f.46](../../../docs/decisions/language/Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md)),
     and `RelationPlaces`, which `relations_written` takes into the relations of the program
     ([Q-M1f.522](../../../docs/decisions/language/Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md)).
     The function is renamed for what it does now. `validate_relation_parameters` writes no warning for a written
     activator; what else it does, if anything, stays.
   - `ein-infer/src/compile.rs`: `note_inert_activators` and `take_inert_activators` go, with their calls in
     `engine.rs`, `obligations.rs` and `hypothesis_rules.rs` and the printing in `ein-cli/src/common.rs`.
     `activators_for`, which hands no plan such an activator, stays.
   - The comments that cite a line: `ein-core`'s `entities.rs` and `program.rs`, `ein-infer`'s `plan.rs`.
2. **The tests hold the silence.** `ein-cli/tests/cli_semantics.rs`'
   `a_derived_activator_binding_a_relation_parameter_to_a_fact_is_warned_of_at_load` and its siblings, the unit
   test of that name in `from_ir.rs`, and `ein-infer/tests/compile_semantics.rs`'
   `a_relation_parameter_a_rule_binds_to_a_nested_fact_activates_nothing`: each holds that no line is printed, on
   every command, and keeps what it holds of the engine (no plan, the answer). `solve_lists.rs` and
   `stdlib_semantics.rs` where they cite the lines.
3. **The decisions.** Q-M1f.1 rewritten under its id, as S1f.21.17 rewrote it: such an activator activates nothing
   and nothing says so. Q-M1e2.18's second half the same; its first, the keyword pair refused at the parse, stays.
   Q-M1f.41 and Q-M1f.42 reworded for what reads the positions now. Q-M1f.580: when lines print stands for every
   line; the words it set for these go. Where Q-M1f.115, Q-M1f.567, Q-M1f.555, Q-M1f.559, Q-M1e2.26 and Q-M1f.46
   cite the lines; the language decisions' README.
4. **The pages.** The diagnostics page's § 2.9, *a rule shape that reads nothing*, and its rows; `std.negation`'s
   *A program the import warns of*, which goes; and every page `git grep -e 'activates nothing' -e 'Q-M1f\.1\b'
   -e 'Q-M1e2\.18' -- docs` finds, read for a claim that a line is printed: `08-read-out.md`, `02-syntax.md`,
   `05-rules.md`, the reasoning pages `02-rewriting-rules.md` and `02-saturation.md`, and `08-reasoning-in-code/`'s
   `01-static-ir.md` and `02-saturation-and-negation.md`.
5. **The programs' words.** The headers of `examples/features/19_derived_activator.ein` and
   `20_derived_activator_never.ein`, which stay as fixtures of an activator that activates nothing, and of
   `examples/ein-bugs/wrong-answers/derived-activator.ein`, `derived-activator-never.ein`,
   `nested-fact-activator.ein` and `kwpair-in-guard.ein`; their lines in `examples/README.md`,
   `corpus/corpus.toml` and `corpus/regressions.toml`; the comments of `stdlib/negation.ein`,
   `stdlib/math/decimal-arith.ein` and `stdlib/math/positional.ein`, and `stdlib/README.md` (a stdlib file that
   changes takes `utils/stdlib_manifest.py --write`); `examples/broken/compile/activator_arity.expected` and
   `examples/broken/load/README.md`, read for which shape they are about.
6. **The check over the corpus**: no corpus program's stderr, under `ein solve`, `ein saturate` and `ein show hyps`,
   holds a line of the three.

## What must hold

- **No answer moves**: $H$, $R$ and every exit code on every corpus program, by the corpus sweeps as they are.
- **Fixtures 19 and 20 stay flagged** where they are: the static monotone check is `ein-infer`'s and reads
  activators itself, not the loader's positions.
- **Q-M1f.46's refusal and the relations of the program do not move**: they read the positions, which stay.
- The gate on the batch's last commit.

## Record

Run 2026-11-24 in a worktree fast-forwarded to `master` at `0f962f04`. Scratch is the agent's own: the release binary
of `0f962f04` (`base`), of the stage's tip (`final`), and `sweep.py`, which runs `ein solve`, `ein saturate` and
`ein show hyps` on every program of `corpus/corpus.toml` and keeps each run's exit, its stdout's digest (times masked)
and its stderr.

### Commits

| commit | task | what |
|---|---|---|
| `a117e80a` | 1, 2 | the three lines go: `validate_relation_parameters` gone, `warn_derived_relation_activators` is `read_relation_places`, which reads the positions as before for Q-M1f.46's refusal and the program's relations and writes no line; `Program::warnings` gone; `note_inert_activators`/`take_inert_activators` gone with their three calls; `keep_warnings` gone and `say_warnings` is `say_notes`; the tests turned to hold the silence |
| `e802c611` | 3, 4 | the decisions (Q-M1f.1, Q-M1e2.18, Q-M1f.41, Q-M1f.42, Q-M1f.580, Q-M1f.555, Q-M1f.559, the language README) and the pages (diagnostics § 2.9 and its opening, § 2.6; the read-out's stderr table; `02-syntax.md`, `05-rules.md`; the reasoning-in-code rows; `std.negation`'s *A program the import warns of*, deleted) |
| `02bcddc3` | 5 | the fixtures' headers (19, 20, `derived-activator`, `derived-activator-never`, `nested-fact-activator`), their lines in `examples/README.md` and `corpus/corpus.toml`, `examples/broken/load/README.md`, the three stdlib comments and `stdlib/README.md`, the manifest |
| the next | | this record; plans only |

### 1. The lines, the reading and the semantics

- **The load's line for a written activator** was `validate_relation_parameters`' only output: the function is
  gone. **The load's line for a derived one** and the line for a fact written where a passed name is held were
  `warn_derived_relation_activators`' warnings: the function is `read_relation_places`, its positions read exactly
  as before. Its map of *why* a position holds names (`Holds`, read only by the lines' words) is a `BTreeSet` of the
  same keys, inserted in the same order of sites, ways and occurrences, so the fixpoint reaches the same set;
  `Occ::Inner`/`Occ::Head` are one `Occ::Value` (no reader told them apart but the lines), and a way no longer
  carries its disjunct. `refuse_eq_at_another_arity` reads `rp.contains` where it read `contains_key`, and
  `RelationPlaces` is built by the same two loops. `Lines`, `NAMED` and the `warnings` vector go.
- **The run's line** was `ein_infer::compile::note_inert_activators`, called before `activators_into` in
  `Engine::compile_all`, `obligations::plans_for` and `HypothesisRules::new`, read back by `take_inert_activators`
  in `ein-cli`'s `say_warnings`: all gone. `activators_for`/`activators_into` and `binds_a_relation_to_a_fact`
  are unchanged, so no such activator gets a plan.
- **The CLI**: `keep_warnings`, which printed `Program::warnings` as `warning: FILE: …`, and the pending list go;
  the load's `note:` lines are kept and printed at the run's end as before, by `say_notes` (`lib.rs` twice,
  `test.rs` once a file).
- No crate test changed a golden; no `EIN_BLESS` run.

### 2. The tests

| test | now holds |
|---|---|
| `ein-cli/tests/cli_semantics.rs`: `an_activator_binding_a_relation_parameter_to_a_fact_activates_nothing_in_silence` (was `…_is_warned_of_at_load`) | on 19, 20 and their two twins, under `saturate --dump`, `solve`, `solve --minimal`, `show hyps`, `oracle`, `render lattice`, `render kb`, `render rules` and `kb save`: exit 0, no `warning:` line, no compiler line; `saturate --dump` holds `(r (a b) :rule mk)` where root derives it and lists `r` with *no plan compiled*; zebra2 and the bijection test load with no `warning:` |
| `cli_semantics.rs`: `a_rule_shape_that_reads_nothing_is_refused_at_load` | `nested-fact-activator.ein` under `saturate --dump`, `solve`, `oracle`, `show hyps`: exit 0, no `warning:`; the dump holds `(r (a b))`, *no plan compiled* for `r`, no `hit` |
| `ein-ir/src/from_ir.rs`: `a_derived_activator_binding_a_relation_parameter_to_a_fact_loads` (was `…_is_warned_of_at_load`) | each of the sixteen shapes loads; and what the positions are read for: a name passed through a marker, or through a conclusion whose relation is a parameter, is a relation of the program, one written at a declared relation's argument is not (dropping the passed names from `RelationPlaces` fails it, checked by hand) |
| `from_ir.rs`: `a_rule_shape_that_reads_nothing_is_refused_at_load` | the written activator, a fact or a literal in an `:assert`, loads, and the fact is held |
| `ein-infer/tests/compile_semantics.rs`: `a_relation_parameter_a_rule_binds_to_a_nested_fact_activates_nothing` | root derives `(mirror (q A B))`, no `hit`, and `plan_shape_with` compiles no plan of `mirror` |
| `solve_lists.rs`, `stdlib_semantics.rs` | comments only |

### 3. The check over the corpus (task 6)

`sweep.py` on all 933 corpus programs × `ein solve`, `ein saturate`, `ein show hyps` = 2 799 runs, `base` against
`final`, both reading the tip's files: **every exit code and every stdout the same** (times masked); **stderr the
same but for the three lines**, which 15 runs of `base` printed (5 programs: features 19 and 20, `derived-activator`,
`derived-activator-never`, `nested-fact-activator`, each under the three commands) and **no run of `final`
prints; no run of `final` prints any `warning:` line**. 19 and 20 still print the `WARNING` line under `solve` and
`show hyps`: flagged as before.

### 4. The gate

`./run_tests.sh` once, on `02bcddc3`, the batch's last commit: **exit 0**, every step of the per-commit tier — the ten static checks (the manifest, the determinism lint, the link check with 0 findings, the release voice, the step lists, the tool mirrors, the doc blocks, `cargo fmt`, clippy `-D warnings`, rustdoc `-D warnings`), `cargo test --profile gate --workspace` (169 test binaries, none failed; no golden re-banked), the test scratch, `check_generated.py` (9 generators, every file as written), `check_region_routes.py`, `wrong_answers.py` (9 open rows, each still answered wrong), the review bundle's smoke, and the bench smoke. Before it, on the way: the affected tests alone, clippy and the generators' check, each green. No DOT writer changed, so no container run is owed (Q-M1f.77); no nightly run.

### 5. Defaults

| id | default | the other reading |
|---|---|---|
| Q-M1f.690 | `Program::warnings`, which held only these lines, goes with them, and with it `ein-cli`'s `keep_warnings` and the pending list; `say_warnings`, which now prints the load's `note:` lines alone, is `say_notes`. Q-M1f.580 is rewritten for every line a load or a run prints, the load's notes today, held by `non_monotone_warning.rs`' `the_warning_comes_before_a_loads_note` | keep the field and the channel, empty, for a later load warning |
| Q-M1f.691 | Q-M1f.1 keeps its file and slug: its title is rewritten to start as the old one, *A rule that could derive an activator binding a relation parameter to a fact loads, and such an activator activates nothing in silence*, so no link moves and the ledger's link to it (`open_questions.md`, a coordinator file this stage does not edit) stays good | a title led by the activator, *An activator that binds a relation parameter to a fact activates nothing, and nothing says so*, file `Q-M1f.1-an-activator-that-binds-a-relation-parameter.md`, with the ledger's link moved |
| Q-M1f.692 | the unit test that read the load's lines keeps its sixteen shapes as programs that load, and holds the positions through the relations of the program (three programs), which Q-M1f.41 and Q-M1f.42 now cite beside `eq_at_another_arity_is_refused_at_load` | the shapes alone, loading |
| Q-M1f.693 | the five fixtures' headers keep their line counts, so no position in them moves and no golden is re-banked | headers of any length |
| Q-M1f.694 | what cites no line stays: diagnostics § 2.9 keeps its positions paragraph (§ 2.14 and § 2.20 read *§ 2.9's positions*), retitled *Which positions hold relation names*; the reasoning pages `02-rewriting-rules.md` and `02-saturation.md`, Q-M1f.115, Q-M1f.567, Q-M1e2.26, Q-M1f.46, `entities.rs`, `plan.rs`, `corpus/regressions.toml` and `activator_arity.expected` unchanged; the reasoning-in-code row *a variable in a relation's place is a parameter* names `refuse_no_fact_holds`, which refuses one no parameter binds, for the removed `validate_relation_parameters` | — |

### 6. Findings

| id | what | where |
|---|---|---|
| V-101 | `kwpair-in-guard.ein`'s header says the loader refuses it with a `kb load error` naming the rule and the clause, *… a name, a string or an integer*; today it is a parse error at the keyword (`33:38: unexpected `:k``), as its corpus note and regression row say. The keyword half, not this stage's | `examples/ein-bugs/wrong-answers/kwpair-in-guard.ein` |
| V-102 | `std.math.positional`'s shared rules are named by their activators and not in a base's closing rule, a choice the comments justified by the load's warning, now gone; they now give the loader's reading (Q-M1f.42) as the reason. Whether a closing rule may name them now is open, and not tried: naming them would move the positions the loader reads, and so the relations of the program (Q-M1f.522) | `stdlib/math/positional.ein`, `decimal-arith.ein` |
| V-103 | `cli_semantics.rs`' `a_conclusion_no_fact_can_hold_is_one_line_in_eins_spelling` cites Q-M1f.1 for a conclusion no fact can hold refused at the parse, which Q-M1f.1 does not decide | `ein-cli/tests/cli_semantics.rs` |
| V-104 | `binds_a_relation_to_a_fact` stays `pub` and re-exported by `ein_infer`, with `activators_into` its one caller | `ein-infer/src/compile.rs`, `lib.rs` |
| V-105 | the ledger's Q-M1f.1 entry links the decision file, whose title is now Q-M1f.691's; the entry is the coordinator's to read again | `plans/m1f_hypothesis_and_documentation/open_questions.md` |
