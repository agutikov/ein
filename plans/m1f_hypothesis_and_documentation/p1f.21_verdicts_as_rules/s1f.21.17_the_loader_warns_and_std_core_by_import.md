# S1f.21.17 — a warning where the loader refused for what a rule could derive; `std.core` by import and by `$EIN_STDLIB`

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's answers to review items 38 and 41
([the rulings](README.md#the-rulings-of-the-answers-file)); **done 2026-11-20** (in master as `24491e80`, which its record names `8acb1b55`, the same change on the stage's branch); started the same day on the user's word (*"Start stages, if
anything can be done in parallel - do it."*), beside S1f.21.11 and the examples of S1f.21.12. **Built 2026-11-20**
in `8acb1b55`, on the defaults Q-M1f.580 to Q-M1f.584 ([the record](#record)).
**Waits on:** nothing. Task 1's reading is confirmed by the user, 2026-11-20: *"a - yes"*. In
order it is the next stage of part 1, before [S1f.21.11](s1f.21.11_kill_and_three_conventions.md);
S1f.21.16, which stood before it, is dropped.
**Writes:** the loader's check on an activator a rule could derive and the compiler's reading of such an
activator; the loader's reading of `(import std.core)` and of `$EIN_STDLIB`; the diagnostics' page, the page of
`std.core`, the install page; the decisions it amends; `AGENTS.md` where it states either.

## The answers

The user, 2026-11-19, in part 1's answers file, verbatim.

- Item 38 ([Q-M1f.559](../open_questions.md#q-m1f559--the-elimination-and-the-module-every-program-gets)): *"forbid becomes a warning, fix :hrulse activators syntax"*
- Item 41 ([Q-M1f.563](../open_questions.md#q-m1f563--what-a-module-every-program-gets-brings)): *"import std.core works like duplicated import; $EIN_STDLIB allows to replace std.core is in $EIN_STDLIB there is no std.core - print error about std.core not found in $EIN_STDLIB; There is still only a single user of ein - me, so I don't care about backward compatibility. Forget about it until I explicitely ask. Containers not load - delete and create new."*

The second half of item 38's was S1f.21.16's, which is dropped: the user's
later ruling removes the clause it spelled (Q-M1f.567). The rest is here.

## Task 1 — the refusal for what a rule could derive becomes a warning

**The reading, the coordinator's.** *Forbid* names, everywhere else, the refusal of a refutation under an `absent`,
which the user removed the same day ([Q-M1f.564](../open_questions.md#q-m1f564--a-non-monotone-program-loads-with-a-warning-forbids-refusal-goes),
S1f.21.15). Item 38 is about another refusal: at load, of a program in which a rule *could* derive an activator
that puts a fact where a relation's name goes, whether or not any run derives one. The answer stands under that
item, and stood there when the user was told that the two differ and asked which was meant; so it is read as that
item's option (d): **this refusal becomes a warning, and the program loads**. It reverses
[Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b), the user's of 2026-11-08. **The user confirmed it, 2026-11-20**: *"3 readings: a - yes"*.

What the check is, and where each step of it was decided, is in review item 38's page (in `git log`, last at
`bedbfbd4`) and in the ledger's Q-M1f.559.

1. **What it refuses today**, by the binary: five corpus programs, with the words *a relation is named by a symbol,
   not a fact* (`ein show hyps` over `examples/` and `tests/`, on `946ee788`):
   `examples/broken/load/derived_activator.ein` and `derived_activator_never.ein`, and under
   `examples/ein-bugs/wrong-answers/`, `derived-activator.ein`, `derived-activator-never.ein` and
   `nested-fact-activator.ein`. Each is shown loading, with what it answers, before anything is banked.
2. **At load**: each line the refusal printed is printed as a `warning:` on stderr, and the program loads.
3. **At run, where a state does derive such an activator.** No fact has a fact under its first arrow, so a pattern
   whose relation is a fact matches nothing, and a conclusion whose relation is a fact is no object of the
   knowledge base. **Reading 1**, default: the activator is held as the fact it is and **activates nothing**, no
   plan compiled for it, and a `warning:` names the rule and the activator where a state derives it. The other
   reading: the run stops there with the compiler's line, as it did before the check was at load.
4. **What expected the refusal**: the two fixtures under `examples/broken/load/` with their expected lines, the
   three witnesses' rows of `corpus/regressions.toml`, the diagnostics' section, the grammar page's table if it has
   the shape, and the decisions Q-M1f.1, Q-M1f.41 and Q-M1f.42, each rewritten in place to what is true: what the
   warning reads, and that nothing is refused.

## Task 2 — `(import std.core)` is an import like any other

Today it is refused: *std.core is the module every program gets, with no import line: no import names it*. The
user: *"import std.core works like duplicated import"*. A second import of a module a program already has loads
today in silence, one copy, in the same form or another (`(import std.negation)` twice; `(import std.negation)`
beside `(import std.negation :as n)`: each run on `946ee788`).

1. The refusal goes. `(import std.core)`, in a program or a module, in any of the three forms, binds the module's
   names as an import does and brings no second copy.
2. **Reading 2**, default: where the module stands is not moved by an import of it: after the program's forms and
   its imports, as [Q-M1f.551](../../../docs/decisions/stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)
   has it, so a program lists the same rules in the same order with the line and without.
3. `ein-cli/tests/import_invariance.rs` holds a program to its answer with the line added.

## Task 3 — `$EIN_STDLIB` may replace `std.core`

The user: *"$EIN_STDLIB allows to replace std.core is in $EIN_STDLIB there is no std.core - print error about
std.core not found in $EIN_STDLIB"*. It is the first answer to how the module is customised, which Q-M1f.551 left
for later. Today, on `946ee788`, with a copy of the library under `$EIN_STDLIB`:

| the copy | today |
|---|---|
| its `core.ein` edited, the manifest left as it was | loads; the edited module is every program's |
| no `core.ein` | *kb load error: std.core, the module every program gets, cannot be read: (import std.core) — module not found at …/core.ein (None)* |
| a `core.ein` and no `MANIFEST.sha256` | *… $EIN_STDLIB names …, which has no MANIFEST.sha256 — a directory is the stdlib only if it carries the marker …* |

1. **The error where the library `$EIN_STDLIB` names has no `std.core`** says so in those words: that `std.core`
   was not found in `$EIN_STDLIB`, with the directory.
2. **Reading 3**, default: the manifest stays what identifies a directory as the library, so the third row stays
   a refusal, with its own words; the other reading takes any directory that has a `core.ein`.
3. The page of `std.core`, the install page's section on `$EIN_STDLIB` and the configuration reference say that
   the module is replaced this way, with a program run under a library whose `std.core` is another.

## Not built: backward compatibility

The user: *"There is still only a single user of ein - me, so I don't care about backward compatibility. Forget
about it until I explicitely ask. Containers not load - delete and create new."* A saved file or a container of an
older format does not load, and nothing is added to read one. It is a rule of the phase from here on
([the README](README.md#rules-of-the-phase)).

## What must hold

- **Every answer** of a program that loaded before: the corpus sweep, each mode against the oracle.
- A program task 1 newly loads is shown with its answer and what warns, and nothing is claimed of it past what
  the run shows.
- A refusal that goes takes its section of
  [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md) with it, in the same commit; the
  standing rules of `AGENTS.md` that state either refusal are rewritten in place.

## Acceptance

`(import std.core)` loads; a library with no `core.ein` under `$EIN_STDLIB` gives the error in the user's words;
and, once the reading is the user's, the five programs of task 1.1 load under a warning. The gate on the stage's
last commit.

## Record

The stage ran on 2026-11-20 in its own worktree, branched from `master` at `669d7d43`, beside S1f.21.11 and the
examples of S1f.21.12, in two commits: the three tasks in one, since their edits share the corpus manifest, the
goldens, the diagnostics page and two READMEs, and this record.

| commit | what |
|---|---|
| `8acb1b55` | tasks 1, 2 and 3 |
| this one | the record |

### Task 1.1: the five programs, loaded, before anything was banked

Taken on the build of task 1 alone (the gate profile): `ein solve FILE`, exit 0, and `saturate`, `oracle`, `show
hyps`, `render rules`, `render h-space` and `kb save` exit 0 too (`ein test` exits 2: none states a claim, *nothing
to check*). The refusal before it, on `669d7d43`'s build, was the line the plan quotes, at each command's load-error
exit. **Four of the five are flagged
by the static check**, one site each, *r: its activator relation is one a commitment can change; root compiled no
plan of it*: a commitment to a candidate `(item b)` derives `(r b)`, a name, an activator root has no plan of.
Nothing is claimed of what those four answer (Q-M1f.564); the counts below are what the run printed.

| program | static check | `ein solve` | stderr |
|---|---|---|---|
| `examples/broken/load/derived_activator.ein` (now `examples/features/19_derived_activator.ein`) | flagged | `states (k_s) 64`, the cube of 6 candidates, root alive | the `WARNING` line, the load's warning (`mk`, `(r ?f)`, `?R`, `(item ?f)`), the run's (`r`, `(r (a b))`) |
| `examples/broken/load/derived_activator_never.ein` (now `examples/features/20_derived_activator_never.ein`) | flagged | `states (k_s) 64`, root alive | the `WARNING` line, the load's warning; no run's: no state derives the activator |
| `examples/ein-bugs/wrong-answers/derived-activator.ein` | flagged | `states (k_s) 64`, root alive, the query's goal `(hit ?x)` no row in root | as `19` |
| `examples/ein-bugs/wrong-answers/derived-activator-never.ein` | flagged | `states (k_s) 64`, root alive | as `20` |
| `examples/ein-bugs/wrong-answers/nested-fact-activator.ein` | monotone | as written, `states (k_s) 65 536`, the cube of 16 candidates, root holding no `hit`; with `(solve :no-search)` added (below), `states (k_s) 1`, R = {∅} | the load's warning (`r`, `?R`, `(r (a b))`), the run's (`r`, `(r (a b))`) |

### Task 1: the refusal is a warning, and such an activator activates nothing

**Reading 1**, the plan's default, taken: an activator that binds a relation parameter to a fact is held as the
fact it is and activates nothing.

- **The loader** (`ein-ir`'s `from_ir`): the lines of `validate_relation_parameters` (where the activator is
  written) and of `refuse_derived_relation_activators`, renamed `warn_derived_relation_activators` (where a rule
  could derive it), go to `Program::warnings` instead of the refusal; `eq` at another arity, which the second
  function also checks, stays a refusal. Each line keeps its text and its tail is now *— a relation is named by a
  symbol, not a fact, so such an activator activates nothing*.
- **The engine** (`ein-infer`'s `compile`): `Rule::relation_params`, the parameters that head a form (the loader's,
  a name written twice at its last position, the one the compiler seeds); `activators_for` filters an activator
  that binds one of them to a fact, as it filters one of another arity, so no plan is compiled for it and the
  compiler's line (`head_not_a_name`) is reached by a direct caller of `compile_rule` alone;
  `note_inert_activators`, called where a state's activators get plans (`Engine::compile_all`, the obligations'
  plans), records one line per such activator a state holds, placed at the rule's `:match`.
- **The CLI**: every command prints the load's lines as `warning: FILE: <line>` and the run's as
  `warning: <place>: <line>`, when its run of the program ends (`ein test`: after each file), so that a flagged
  program's `WARNING` line stays stderr's first (`non_monotone_warning.rs`). No exit code moves. The read-out page's
  § 10, the lines on stderr, has a row for each.
- **What expected the refusal**: the two fixtures moved out of `examples/broken/load/` to
  `examples/features/19_derived_activator.ein` and `20_derived_activator_never.ein`, their `.expected` deleted,
  headers rewritten, their corpus entries now `positive`; the regression rows `derived-activator/derived-activator`
  and `derived-activator-never` **removed**, their witnesses flagged (below); `nested-fact-activator`'s row rewritten
  to the program loading, `solve --minimal`, root with no `hit`; the decisions Q-M1f.1 (its title too), Q-M1f.41,
  Q-M1f.42 and Q-M1e2.18 rewritten in place; the diagnostics' § 2.6 and § 2.9 (the three rows of the refusal's table
  became the warning's table, with the run's line and a run shown), a paragraph in § 2's head; the syntax page, the
  reasoning pages `02-rewriting-rules.md` and `02-saturation.md`, the `std.negation` page's section (now *A
  program the import warns of*, its output re-taken), the reasoning-in-code tables, `examples/broken/load/README.md`,
  `examples/README.md`; comments in `stdlib/negation.ein`, `stdlib/README.md`, `std.math.positional`,
  `std.math.decimal-arith` (the manifest regenerated) and three tests. The grammar's table has no row of the shape.
  The tests: `ein-ir`'s and `ein-cli`'s `a_derived_activator_binding_a_relation_parameter_to_a_fact_is_warned_of_at_load`,
  `compile_semantics.rs`' `a_relation_parameter_a_rule_binds_to_a_nested_fact_activates_nothing`,
  `cli_semantics.rs`' `a_rule_shape_that_reads_nothing_is_refused_at_load` (the nested fact now loads, its two lines
  read); `answer_round_trip.rs` lets `ein kb save`'s stderr carry a load's `warning:` beside its `note:`. Moved by
  the programs that now load, each read before re-banking: the exit-code golden and the shape golden (only the
  five programs' lines and the moved fixtures'; the four flagged bank no answer op), `alive_invariant.rs`' list of
  seeded names (five programs, `b` and `z` each), `detector.rs`' count of flagged corpus programs (47 → 51), and
  `test_cli.rs`' count of `examples/features/` files with no expectation (8 → 11, with task 2's file).

### Task 2: `(import std.core)` is an import like any other

The refusal is gone from `ein_ir::imports`. **Reading 2**, the default, taken: the module is spliced only by the
resolver's own import of it, the main file's last form, so an import a file writes binds the names and moves the
module from nowhere. Two things the shapes needed: the names the loader marks `Rule::implicit` are the ones that
print (`c.inconsistent` under `:as c`, `inconsistent` under `:symbols (inconsistent)`), so `refuted by` still reads
through the rule; and the list a container stores keeps the module's names in full, since it holds none of its
forms. `examples/broken/load/import_of_core.ein` moved to `examples/features/21_import_of_core.ein`, its
`.expected` deleted. Held by `ein-ir/tests/imports_semantics.rs`'
`an_import_of_the_module_every_program_gets_is_an_import_like_any_other` (three shapes, first and last, a module
that imports it, what a container stores) and `ein-cli/tests/import_invariance.rs`'
`an_import_of_the_module_every_program_gets_changes_no_answer` (six programs, three shapes, first and last: R, the
queries, the claims, `refuted by` and the rule list of `saturate --dump`). Q-M1f.551 rewritten in place; the
`std.core` page, the diagnostics' § 2.7 (retitled, *The module every program gets cannot be read*), `stdlib/core.ein`'s
header and the `stdlib/README.md` section, `AGENTS.md`'s sentence; `tests/saved/04_a_saved_node.ein` saved again,
since it carries the module's header.

### Task 3: `$EIN_STDLIB` replaces `std.core`

Replacing already worked (the plan's first row). New: a library `$EIN_STDLIB` names, its manifest there and no
`core.ein`, refuses every program, and a file's import of the module, with *std.core was not found in $EIN_STDLIB:
DIR holds no core.ein — every program gets std.core, and a library $EIN_STDLIB names is where it is read from
(Q-M1f.551): write core.ein there, or unset $EIN_STDLIB to use the checkout or the embedded copy*. **Reading 3**,
the default, taken: the manifest stays what identifies the library, so a directory with no `MANIFEST.sha256` keeps
its own line, checked first. The `std.core` page (*Replacing it*, a run of `tests/stdlib/core/01_false_ends_the_state.ein`
under a copy whose `core.ein` reads `(absurd)`: root alive, the claim refuted, exit 1), the install page and the
configuration reference say so. Held by `imports_semantics.rs`' `a_library_with_no_std_core_is_refused_and_one_with_another_is_read`
and `cli_semantics.rs`' `ein_stdlib_replaces_std_core_and_a_library_without_one_is_refused`.

### Defaults taken

- **Q-M1f.580** — the warning's text: each line the refusal printed, its tail *a relation is named by a symbol, not
  a fact* followed by *, so such an activator activates nothing*; the run's line names the rule, the activator, the
  parameter and the fact, placed at the rule's `:match` (a top-level form has no position, diagnostics § 1.5). All
  `warning:` lines are printed when a command's run of the program ends, after a flagged program's `WARNING`, each
  load line once a process, each run line once a run (`ein test`: a file).
- **Q-M1f.581** — the two regression rows whose witnesses the static check now flags,
  `derived-activator/derived-activator` and `derived-activator-never`, are removed, not rewritten: no row holds a
  flagged program's answer (Q-M1f.564), as S1f.21.15 removed eleven. The witnesses stay in the corpus.
- **Q-M1f.582** — `nested-fact-activator.ein` states `(solve :no-search)`: its question is root's, and as written its
  blind cube of 16 candidates put 2^16 saturations into every sweep that describes R or runs the oracle.
- **Q-M1f.583** — the moved fixtures' place: `examples/features/19_…`, `20_…` and `21_import_of_core.ein`, the
  programs unchanged but for their headers; none states a claim, so `ein test examples/features` counts eleven files
  with no expectation.
- **Q-M1f.584** — under `(import std.core :as c)` or `:symbols`, the module's rule prints as the import spells it,
  as any module's does, and stays the module's (`Rule::implicit`); a container stores those names in full.

### Found outside the stage's scope

- **V-32 — The load's warning is one line per site.** A program that names a rule of a copier's count beside
  `std.algebra` now loads with seventeen lines, one per generic rule of the module (`converse`, `imply2-fwd`, …;
  the `std.negation` page's program with `std.algebra` in the import's place). Each says something true; whether a
  module's lines are to be gathered is open.
- **V-33 — The JSON summary carries no warning of the load or the run.** `--json-summary` has
  `monotonicity.warning` for a flagged program and nothing for these lines, which are on stderr alone.
- **V-34 — L-109's program loads.** `examples/zebra2.ein` with `about-1`, `(about-1 symmetric)` and its query
  appended (the milestone's L-109, *the user's call*): `ein saturate` exits 0 under one warning, naming
  `std.algebra.imply1`, and root derives `(in-slice-1 symmetric next-to)`. The refusal L-109 met is gone; whether a
  slice is had by such a rule is still the user's.

### For the user

- Four of the five programs Q-M1f.1 refused are flagged by the static check once they load; nothing is claimed of
  their answers, and their two regression rows went (Q-M1f.581).
- `nested-fact-activator.ein` gained `(solve :no-search)` (Q-M1f.582).

### The gate

`./run_tests.sh` on `63aff226` (this record before its gate paragraph; the project's tree is `8acb1b55`'s), at the
default job counts, exit 0: the ten static checks (the manifest, hash-map iteration, the link check, 0 findings, the
release voice, the gate steps, the tool mirrors, the doc blocks, `cargo fmt`, `clippy -D warnings`, `cargo doc -D
warnings`), `cargo test --profile gate --workspace` (1 375 tests passed, 0 failed, by the sum of its `test result`
lines), `check_test_scratch.py`, `check_generated.py`, `check_region_routes.py`, `wrong_answers.py`, the review
bundle's smoke (0 DOTs refused, its link check exit 0) and the bench smoke. Its last lines:

```text
Testing region_n1/zebra2
Success

Testing region_single/zebra2
Success
```

Not run, as the brief said: `--nightly`, `--dev-host`, `utils/ci_container.sh`. This stage changes no DOT writer,
no review bundle and nothing CI installs.
