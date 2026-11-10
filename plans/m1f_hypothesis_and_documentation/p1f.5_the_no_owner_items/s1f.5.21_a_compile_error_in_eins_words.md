# S1f.5.21 — A refusal the compiler makes reads the same on every command, in Ein's words

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 1
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). **Done 2026-11-10**, in one
working tree for the coordinator to commit: T1f.5.21.1 ([its record](#t1-record-2026-11-10)), every refusal and every
command censused, every move listed, W62 to W65's sixteen rows filed `open` and held to the unchanged engine;
T1f.5.21.2 ([its record](#t2-record-2026-11-10)), the three refusals a program reached at load, the compiler's lines in
Ein's words, the four views reading a program through the loader, the sixteen rows `fixed`; T1f.5.21.3
([its record](#t3-record-2026-11-10)), the pages; the acceptance re-taken ([§ Acceptance,
re-taken](#acceptance-re-taken-2026-11-10)). Six questions taken by default, Q-M1f.71 to Q-M1f.76. The register reads
177 fixed, 13 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10).
**Depends on:** nothing. It runs before [S1f.5.24](s1f.5.24_a_macro_body_checked_where_a_rule_is.md), which moves
the one refusal this stage leaves where it is, the one a macro's body reaches.
**Takes:** F23's [D-1](../../followups/f23_diagnostics_in_eins_words.md), its `CompileError` half (the Python class
prefix and the IR's `SForm(…)` repr). The rest of D-1 is re-taken at T1 and taken here if it is the same code path,
or left to F23 with a line saying so.

## The ruling

The close review's finding 1, as written on 2026-11-10:

> **A compile error's first line differs by command, and prints the IR's Python spelling.** On
> `examples/broken/compile/nested_or.ein`: `ein solve`, `ein oracle` and `ein render lattice` print the message
> bare; `ein saturate` puts `ein.inference.compile.CompileError:` in front, a Python class name; `ein render kb`
> puts the file name in front; `ein render program` and `ein render rules` draw the rule and exit 0. Every one
> prints `SForm(head=Atom(name='or'), …)` and cites "S1.8.A13" and "the M1 compiler", and the fixture's
> `nested_or.expected` pins that text. This breaks the one-tool rule.

The user, 2026-11-10: *"new stage in p1f.5"*.

## The case

Re-taken on 2026-11-10, release binary at `21c24c07`, on `examples/broken/compile/nested_or.ein`:

| command | exit | stderr's first line, cut at 120 characters |
|---|---:|---|
| `ein solve` | 1 | ``nested `(or …)` in a `:match` premise: SForm(head=Atom(name='or'), args=(SForm(head=Atom(name='p'), …`` |
| `ein oracle` | 1 | the same |
| `ein render lattice` | 1 | the same |
| `ein saturate` | 1 | ``ein.inference.compile.CompileError: nested `(or …)` in a `:match` premise: SForm(…`` |
| `ein render kb` | 1 | ``examples/broken/compile/nested_or.ein: nested `(or …)` in a `:match` premise: SForm(…`` |
| `ein render program` | 0 | none: the rule is drawn |
| `ein render rules` | 0 | none: the rule is drawn |
| `ein kb save` | 0 | none: a container is written that every command above refuses |
| `ein test` | 2 | *nothing to check*: a query with no `:expect` is never solved, which is `ein test`'s contract |

The `(macro one (?a) (eq ?a) …)` case, finding 4, takes the same line (`` `(eq …)` takes exactly 2 arguments and was
given 1: SForm(head=Atom(name='eq'), …) at Loc(file=…, line=2, col=18) ``) and is
[S1f.5.24](s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s.

## What it changes

- **One line, one spelling.** Every refusal the compiler makes that a program can reach prints the same first line
  on every command that reads the program: `<file>:<line>:<col>: rule '<name>': <what>`, as the loader's refusals
  since S1f.5.9, .10 and .18 print. The form is written as Ein text (`ein_core::text`), never as the IR's repr. No
  Python class name, no stage id (`S1.8.A13`), and no *the M1 compiler*.
- **Where.** Default (T1 confirms it per refusal): each such refusal moves to the loader, the precedent of S1f.5.9's
  `eq` at another arity, S1f.5.10's unbound conclusion variable and S1f.5.18's derived activator. Then the
  compiler's line is an invariant no program reaches, and a `.einb` cannot store a program every command refuses.
  **Not** the refusals a macro's body reaches: those are S1f.5.24's, where the user rules where they belong.
- **Every command agrees.** A program refused at load is refused by every command that reads a program, with the
  same line and exit code (`ein test`'s 2). That includes `kb save`, and `render program` and `render rules`,
  which today draw a program every other command refuses. Default (T1 records it as a Q-M1f row): the two views
  refuse it too, because the one-tool rule is *no two commands may disagree*, and a view that exits 0 on a refused
  program disagrees about whether the program is one. The alternative, drawing it and printing the line as a
  `note:`, is the row's (b).
- **What a refusal replaces is a register row** where it moves an answer
  ([Q-M1f.36](../open_questions.md#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row)): `kb save`'s
  and the two views' exit 0, each a command run under the register's schema `/16`.

## Defaults

Each is taken as the stage's default and recorded in its file and in `open_questions.md` as a Q-M1f row:
- the line's shape, `<file>:<line>:<col>: rule '<name>': …`, with the form as Ein text;
- the views refuse a refused program (above);
- the D-1 lines outside the compiler (`KeyError`, `FileNotFoundError`): taken here if T1 finds them on the same path,
  else left to F23 with a line in F23's table.

**As taken (2026-11-10)**, each a row of [`open_questions.md`](../open_questions.md): the line's shape is the
precedent's, which has no position, the compiler's remaining line keeping one
([Q-M1f.72](../open_questions.md#q-m1f72--the-line-a-moved-refusal-prints)); the views refuse
([Q-M1f.71](../open_questions.md#q-m1f71--does-a-view-draw-a-program-the-loader-refuses)); `FileNotFoundError` is
left to F23 and `loc_repr`'s `Loc(…)` taken
([Q-M1f.75](../open_questions.md#q-m1f75--the-compilers-arms-no-program-reaches-and-d-1s-and-d-2s-other-lines)).
Three more T1 met: how the loader counts the register ceiling
([Q-M1f.73](../open_questions.md#q-m1f73--how-the-loader-counts-the-register-ceiling)), a nested `(or …)` a macro's
expansion writes ([Q-M1f.74](../open_questions.md#q-m1f74--a-nested-or--a-macros-expansion-writes)), and which moves
are rows, with `features/02`'s rewrite
([Q-M1f.76](../open_questions.md#q-m1f76--which-of-what-the-move-replaces-is-a-register-row)).

## Tasks

### Task T1f.5.21.1 — every refusal the compiler can make, and every command's line

Read `ein-infer/src/compile.rs` and `error.rs` for every `CompileError` a program can reach, and build a witness for
each. Run every witness on every command that reads a program (`solve` with each mode, `oracle`, `saturate`,
`test` with an `:expect`, `render lattice`, `render kb`, `render program`, `render rules`, `kb save`, and a
container of it where `kb save` writes one), and record the exit and stderr's first line for each. List every
golden, fixture `.expected` and register row the change moves before any edit. File a register row for each
command run whose exit moves. Record D-1's other lines.

### Task T1f.5.21.2 — the refusals at load, in Ein's words

Move each refusal to the loader, one line per slot, and retire the compiler's arm into an invariant naming the
loader check that makes it unreachable (Q-M1f.9 (a)'s pattern). Move `examples/broken/compile/` fixtures that
are now load refusals to `examples/broken/load/`, rewrite their `.expected`, and flip the rows.

### Task T1f.5.21.3 — the pages

[`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes) (the refusal lines and exit codes), `02_patterns.md`'s
*where a malformed primitive is caught*, F23's D-1 row (what this stage took), and the stage's records.

## Acceptance

- No command prints `CompileError`, `SForm(`, `Atom(name=`, `Loc(` or `S1.8.` on stderr for any corpus program,
  held by a test over the corpus and the witnesses, every command.
- Every witness is refused by every command with one first line and its load-error exit code, held by a test.
- Every exit that moved is a register row, `fixed`.
- `./run_tests.sh` green.

## T1 record, 2026-11-10

Taken on the release binary at `864b16ed` (`./build.sh --engine`), before any engine edit. The witnesses and the scripts
are under the session's scratch (`census.py`, `summ.py`, `sweep1.sh`); every program below that a register row reads is
banked under `examples/ein-bugs/wrong-answers/`.

**Every `CompileError` the compiler can raise** (`ein-infer/src/compile.rs`; `error.rs` only wraps it), and whether a
program reaches it:

| arm | what it refuses | reached by a program? | witness |
|---|---|---|---|
| `Space::push` | more than 256 distinct variables in one register file (`MAX_REGS`) | **yes**: a rule's, an hrule's or an obligation's `:match`, an `(absent …)`'s sub-plan, a query's `:goal` | `wide-match-stops`, `wide-match-never-compiled`, `wide-goal`; an `(absent …)` of 257 |
| `bind_activator` | an activator whose arity is not the rule's | **no**: `activators_for` and the hrule driver drop it first; a direct caller of `compile_rule` reaches it | `examples/broken/compile/activator_arity.ein`, which every command answers |
| `premise`, `(absent …)` arity | `absent` at any arity but 1 | **only through a macro's body**: the grammar's `Connective` takes one premise | `(macro two (?a ?b) (absent ?a ?b))`; S1f.5.24's |
| `premise`, nested `(or …)` | an `(or …)` written as a premise of an `(and …)`, an `(absent …)` or a top-level `(or …)` | **yes**: in a rule, an hrule, an obligation and a query's goal, written or produced by a macro's expansion (`forall`'s, a program's own) | the six `nested-or-*` witnesses; `examples/broken/compile/nested_or.ein` |
| `premise`, `eq` / `neq` arity | `eq` at any arity but 2 | **only through a macro's body**: the grammar's `Guard` takes two terms, and a relation parameter bound to `eq` is refused at load since S1f.5.9 | `(macro one (?a) (eq ?a))`; S1f.5.24's |
| `head_not_a_name` | a relation head its activator binds to a fact | **no**: S1f.5.18's checks refuse a written, a nested-slot and a derived activator at load, and a candidate is a name (`head-fact-hyp*`, H holds no nested fact) | four probes, each refused at load or answering |
| `relation`, unbound head variable | a premise whose relation is a variable no activator binds | **yes, in a goal**: a query has no activator; in a rule the loader refuses it since M1e2 S1e2.2.24 (W24) | `examples/ein-bugs/query-goal-free-head{,-unsat}.ein`, `examples/features/02_star_in_identifiers.ein` |
| `relation`, any other head | a head that is no name or variable | **no**: the grammar's `PremiseHead` | — |
| `guard_arg` | a guard comparing a form that holds a variable | **no**: refused at load in a rule (`refuse_no_fact_holds`) and in a goal (`refuse_guard_patterns`) since S1f.5.7 | `goal-form-var`, refused at load |

The `assert!` on an empty `(absent …)` sub-plan (`empty_sub_plan`) is an invariant since S1f.5.5's T5 and no program
reaches it.

**Every command, on every witness** (`census.py`: `saturate`, `saturate --dump`, `solve` bare, `--minimal`,
`--maximal`, `--single`, `--count`, `-n 1`, `oracle`, `test` with and without an `:expect`, `render lattice`, `render
kb`, `render program`, `render rules`, `render rule --name`, `render constraints`, `kb save`, and the same on the
container `kb save` wrote where it wrote one). The first line, by command:

| witness | stops, exit 1 | answers, exit 0 | the first line where it stops |
|---|---|---|---|
| `nested-or-stops` (and `nested_or.ein`, an `(absent (or …))`, a `forall` over an `(or …)`, a program's macro expanding to one) | every `solve` mode, `oracle`, `render lattice`, `saturate` (behind `ein.inference.compile.CompileError:`), `render kb` (behind `<file>: `); `ein test` with a claim, exit 2 | `render program`, `render rules`, `render rule`, `render constraints`, `kb save` | ``nested `(or …)` in a `:match` premise: SForm(head=Atom(name='or'), args=(…)). Only a TOP-LEVEL `(or …)` is supported (S1.8.A13 splits it into one plan per disjunct); a nested one needs DNF expansion, which the M1 compiler does not do. …`` |
| `nested-or-under-commitment` | describe, `--maximal`, `--count`, `oracle`, `render lattice`; `test` 2 | `saturate`, `--minimal`, `--single`, `-n 1`, `render kb`, the views, `kb save` | the same |
| `nested-or-hrule` | every `solve` mode (behind the prefix), `oracle`, `render lattice`; `test` 2 | `saturate`, `render kb`, the views, `kb save` | the same |
| `nested-or-never-compiled` | — | every command; `ein test` with a claim 1, the claim's | — |
| `nested-or-goal` | every `solve` mode (behind the prefix), `oracle` (bare); `test` 2 | `saturate`, `render lattice`, `render kb`, `render program`, `render constraints`, `kb save` | the same |
| `nested-or-goal-no-state` | — | every command (R empty, the goal never compiled) | — |
| `wide-match-stops` (and an `(absent …)` of 257) | as `nested-or-stops` | as `nested-or-stops` | `more than 256 distinct variables in one `:match` — ein.rs numbers a rule's variables into a fixed register file (256 slots) …` |
| `wide-match-never-compiled` | — | every command | — |
| `wide-goal` | as `nested-or-goal` | as `nested-or-goal` | the same |
| `query-goal-free-head`, `features/02` | as `nested-or-goal` | as `nested-or-goal` | ``ein.inference.compile.CompileError: unbound relation head ?R in a premise of `SForm(head=Var(name='R'), …)` — M1 matches relations per activator (Q29), …`` (`oracle` bare) |
| `query-goal-free-head-unsat` | — | every command | — |
| `eq-macro` (`(macro one (?a) (eq ?a))`) | as `nested-or-stops` | the views, `kb save` | ``(eq …)` takes exactly 2 arguments and was given 1: SForm(head=Atom(name='eq'), args=(Var(name='x'),)) at Loc(file='…', line=2, col=18) — …`` |
| `absent-macro` | as `nested-or-stops` | the views, `kb save` | ``(absent …)` takes exactly 1 argument and was given 2: SForm(…) at Loc(…) — …`` |
| `activator_arity.ein` | — | every command | — |

So the stage file's case holds and is wider: one refusal printed four ways (bare, behind the Python class, behind the
file's name, and `solve` behind the class on a goal while `oracle` prints it bare); and for each moved refusal there
are programs on which some commands answer and others stop (a commitment, an hrule, a goal) or every command answers (no
activator, no listed state).

**The corpus** (`sweep1.sh` over the 558 `.ein` files under `examples tests stdlib corpus plans`, `solve`, `saturate`,
`oracle` and `render kb`, release at `864b16ed`): a compiler spelling reaches stderr on three corpus programs only,
`examples/broken/compile/nested_or.ein`, `examples/ein-bugs/query-goal-free-head.ein` and
`examples/features/02_star_in_identifiers.ein` (a `positive` entry whose `(query :goal (?R Rex Animal))` `solve` and
`oracle` refuse, and whose declared runs avoid them); and the loader's `at Loc(file=…, line=…, col=…)` on one,
`examples/broken/load/macro_arity_mismatch.ein` (D-2). A static scan of the same files (`scan.py`) finds no other nested
`(or …)` in a `:match` or a `:goal`, no other goal written with a relation variable, and no `:match` within 50 variables
of 256; the widest corpus rule binds 7 (`compile_limits::the_corpus_is_nowhere_near_the_ceiling`).

**Where each refusal goes.** Moved to the loader, the default: the nested `(or …)` (a rule's, an hrule's, an
obligation's `:match`, and a query's `:goal`, read after macro expansion, Q-M1f.74), the register ceiling (Q-M1f.73) and
a goal's relation variable. Their compiler arms become invariants naming the loader's check (Q-M1f.9 (a)'s pattern).
Left where they are, their line in Ein's words: the two arity lines, reached only through a macro's body (S1f.5.24's,
Q-M1f.67); and the three a program does not reach and a direct caller of `compile_rule` does, `bind_activator`'s,
`head_not_a_name` and a rule's unbound head with no activator (Q-M1f.75). The `_` head arm is an invariant (the
grammar's `PremiseHead`).

**D-1's and D-2's other lines.** `FileNotFoundError: [Errno 2] …` (`solve`, `saturate`, `oracle`; `ein test` prints
`error: nope.ein: No such file or directory (os error 2)`) is the read path's (`common::read_program`), not the
compiler's: left to F23 with a line in D-1's row. `KeyError` is gone since S1f.5.18. The loader's `at Loc(file=…)`
(D-2's second half) is the compiler's `Loc(` too, one function, `ein_ir::loc_repr`: taken here, printed `<file>:<line>:<col>`;
`at None` stays F23's (Q-M1f.75).

**What will move**, listed before any edit:

- *Register rows*, filed `open` by this task (Q-M1f.76, Q-M1f.36's reading: a row per way the engine answers today, a
  stop where a refusal is right among them), each `owner` S1f.5.21, `right` the refusal, witnesses under
  `examples/ein-bugs/wrong-answers/` with a corpus entry, `wrong` and `now` taken at `864b16ed`; held against the
  unchanged engine by `wrong_answers.rs` (`cargo test -p ein-cli --test wrong_answers`, 2026-11-10, green):
  - **W62**, the nested `(or …)`: `nested-or-stops`, `-under-commitment`, `-hrule`, `-never-compiled`, `-goal`,
    `-goal-no-state`;
  - **W63**, the register ceiling: `wide-match-stops`, `wide-match-never-compiled`, `wide-goal` (under a commitment
    and in an hrule it is met one plan per activator, as W62's rows meet the disjunction);
  - **W64**, a goal's relation variable: `goal-free-head` and `goal-free-head-no-state`, on S1a.6.6's two fixtures;
  - **W65**, a command that answers a program every other command refuses: `render-program`, `render-rules`,
    `render-rule`, `render-constraints`, `kb-save`, on `nested-or-stops.ein`.

  Sixteen rows; the register reads 161 fixed, 29 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml |
  sort | uniq -c`, 2026-11-10). Schema `/20` (`ein-corpus`'s `REGISTER_SCHEMA`): a recorded answer can be that a
  command's run answered (`Answer::answered`, W65's `wrong`), and a row whose right answer is a refusal states no
  `[case.region]`, a command's `now` included.
- *Exits*: every command on the nine witnesses and on `nested_or.ein`, `query-goal-free-head{,-unsat}.ein` → 1 (`ein
  test` 2); and, by the views' default (Q-M1f.71), `render program`, `render rules`, `render rule` and `render
  constraints` on **every** program the loader refuses, 0 → 1.
- *`ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt`*: `broken/compile/nested_or.ein`'s two cells leave and
  `broken/load/nested_or.ein`'s three arrive (`solve`, `saturate`, `render rules`, each 1); the twelve `render rules`
  cells of the load-negatives the loader refuses that today exit 0 (`derived_activator`, `derived_activator_never`,
  `hrule_duplicate_name`, `import_conflicting_definitions`, `macro_arity_mismatch`, `open_compound_witness`,
  `rule_duplicate_name`, `rule_half_declarators`, `rule_missing_assert`, `rule_missing_match`, `unbound_head`,
  `unimported_std_macro`) 0 → 1; `query-goal-free-head.ein :: saturate` 0 → 1; `query-goal-free-head-unsat.ein ::
  solve`, `:: saturate`, `:: render rules` 0 → 1; the nine witnesses' cells and the new compile-negative fixture's
  arrive.
- *`ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5`*: `nested_or.ein`'s lines renamed into `broken/load/`, its
  `load` op the refusal and the rest a refused program's; the same for `query-goal-free-head{,-unsat}.ein`;
  `features/02`'s renderings of its rewritten goal; `macro_arity_mismatch.ein`'s `load` op (its `Loc(`); and the nine
  witnesses' and the new fixture's lines, added.
- *`ein.rs/crates/ein-cli/tests/golden/help_shape.txt`*: `--no-require-why` on the four views, and `render program`'s
  about line.
- *Fixtures*: `examples/broken/compile/nested_or.{ein,expected}` → `examples/broken/load/`, its `.expected` the loader's
  line; `activator_arity.expected` in Ein's words; `examples/broken/load/macro_arity_mismatch.expected`'s position;
  `examples/broken/compile/eq_in_a_macro_body.{ein,expected}` new, the one compile refusal a corpus program still
  reaches (the group `compile-negative` would otherwise be empty).
- *Corpus programs rewritten*: `examples/features/02_star_in_identifiers.ein`'s goal, `(?R Rex Animal)` →
  `(is-a* Rex Animal)`, the one `positive` program the move refuses (Q-M1f.76, Q-M1f.39's reading); its declared runs
  answer as before.
- *Tests* that pin today's lines: `ein-infer`'s `compile_semantics` (`every_compile_error_says_what_its_expected_file_says`,
  `a_nested_or_is_refused_in_a_premise_and_inside_an_absent`, `the_compile_and_firing_lines_a_macro_still_reaches`,
  `an_arity_mismatched_activator_is_refused_and_never_reaches_the_compiler`), `compile_limits`, `ein-ir`'s
  `ast::tests::a_loc_renders_as_file_line_col_or_none`, `ein-cli`'s `primitive_arity`, `program_read`
  (`a_containers_positions_name_its_stored_program`), `render_doors` (`render_program_draws_what_the_loader_refuses`),
  `config_reference` (`require_why_decides_loading_only`, `the_cli_counts_come_from_the_golden`).

**Found outside the case, not filed**: a query's goal that writes a pattern inside a pattern whose relation is a
variable, `(query :goal (p (?R ?x)))`, loads, and the compiler lowers the inner pattern as an opaque slot that matches
nothing, so `(p (g k))` gives the goal no row, in silence (`goal-free-head-nested.ein`, release at `864b16ed`: *the goal
has no row: undetermined here*). In a rule the same shape is refused at load since M1e2 S1e2.2.24 (W24's
`nested-unbound-head`). It is no `CompileError`, so it is not this stage's; reported to the coordinator. (Given to
[S1f.5.28](s1f.5.28_the_unchecked_findings_triaged.md) as item 12, then to
[P1f.7](../p1f.7_findings_outside_scope/README.md) as S1f.7.1, both on 2026-11-10.)

## T2 record, 2026-11-10

**The loader** (`ein-ir/src/from_ir.rs`), three refusals, each one line in the precedent's shape (Q-M1f.72), read after
macro expansion (Q-M1f.74):

- `refuse_nested_disjunctions`, for a rule's, an hrule's and an obligation's `:match` (`ingest_rules`, after
  `refuse_no_fact_holds`) and a query's `:goal` (`validate_queries`). Its walk is the compiler's `premise`: from the
  clause's top, through `and`, `absent` and a top-level `or`'s disjuncts, an `(or …)` met as a premise is refused, and
  it never descends into a pattern, a guard or a stored negation. *`rule 'r': its :match writes (or (p ?x) (q ?x)) as a
  premise of and — an (or …) is a whole :match, each of its premises one way it holds: lift it to the top of the :match,
  or split the rule`*; a goal's ends *… a whole :goal, … or ask two queries*.
- `refuse_wide_matches`, the same places, against `MAX_MATCH_VARIABLES` (256, exported as
  `ein_ir::MAX_MATCH_VARIABLES`; `ein_infer::MAX_REGS` is it): every distinct variable the clause writes, and a rule's
  parameters (Q-M1f.73). *`rule 'wide': its :match writes 257 distinct variables, and a rule binds at most 256 — split
  the rule`*, *its parameters among them* where it has any.
- `refuse_goal_relation_variables`, a query's `:goal` premise whose relation is a variable, under `and`, `or` and
  `absent`: *`query: its :goal writes (?R Rex Animal) with ?R as its relation, which nothing binds — a relation is named
  by a name, or by a rule's parameter its activator binds, and a query has no activator`*.

And `ein_ir::loc_repr` prints `<file>:<line>:<col>` (Q-M1f.75): `macro_arity_mismatch.expected` reads `… at
{FILE}:6:20`.

**The compiler** (`ein-infer/src/compile.rs`). The nested `(or …)` arm and the `_` head arm are `unreachable!`, and
`Space::push`'s bound an `assert!`, each naming what makes it so (Q-M1f.9 (a)'s pattern). Every remaining line is in
Ein's words and names the rule (`rule 'r'`) or the query, its clause (`:match`, `:goal`), the form as Ein text
(`dump_compact`), and where the form has a position, the file, the line and the column first: the arity lines a macro's
body reaches (*`<file>:21:18: rule 'r': its :match writes (eq ?x), and eq takes exactly 2 arguments, not 1 — it
compares two values, so write one guard per pair`*), and, for a direct caller of `compile_rule` (Q-M1f.75), an
activator of another arity (*`rule 'pairwise': its activator (pairwise r) has 1 argument, and it declares 2 parameters
(?R ?T) — …`*), a relation head bound to a fact and a rule's relation variable with no activator. `node_repr` is no
longer read by the compiler, only by the plan-shape golden (`shape.rs`, D-5's).

**The CLI** (`ein-cli`): `common::compile_error_line` is gone, so `ein saturate` and `ein solve` print the compiler's
line bare, as `oracle` did; `render kb` prints a saturation error as `saturate` does, with no `<file>: ` in front; and
`render program`, `rules`, `rule` and `constraints` read the program through the loader first (`render::refused`) and
take `--no-require-why` (Q-M1f.71), so a program the loader refuses is refused with its line, exit 1, and one it accepts
is drawn from its parsed forms as before.

**The census, re-taken on the build** (`census.py` over the nine witnesses, `nested_or.ein`, the two goal fixtures, an
`(absent (or …))`, a `forall` over an `(or …)`, a program's macro expanding to one and an `(absent …)` of 257, release
build of the working tree, 2026-11-10): on each, all seventeen commands exit 1 (`ein test` 2) with one first line, the
loader's, behind `kb load error: `, and none prints a Python spelling; `kb save` writes no container. The macro bodies'
`(eq ?x)` and `(absent (p ?x) (q ?x))` print one line on every command that compiles, the container's naming `<program
stored in x.einb>`, and the views and `kb save` accept them (Q-M1f.67). `activator_arity.ein` answers on every command,
as before.

**What moved**, every one listed by T1:

- `corpus/wrong_answers.toml`: W62's six, W63's three, W64's two and W65's five rows `fixed`, each `now` the refusal,
  `[case.fixed]` the null id (the stage's commit not yet written), `wrong` the record T1f.5.21.1 took at `864b16ed`;
  schema `/20`. The register reads **177 fixed, 13 open and 1 not wrong** (`grep '^status' corpus/wrong_answers.toml |
  sort | uniq -c`, 2026-11-10).
- `corpus_exits.txt`, re-blessed (`EIN_BLESS=1 cargo test -p ein-cli --test corpus_cli`): 18 lines out and 42 in
  (`git diff --numstat`, 2026-11-10), exactly T1's list: `nested_or`'s two cells out and its three in under
  `broken/load/`, the twelve load-negatives' `render rules` 0 → 1, the four `query-goal-free-head{,-unsat}` cells
  0 → 1, the nine witnesses' twenty-one cells and `eq_in_a_macro_body`'s two in; 1 504 cells (`wc -l`).
- `corpus_shapes.md5`, re-blessed (`EIN_BLESS=1 cargo test -p ein-render --test corpus_shapes`): 130 lines out and 286
  in, on exactly these programs: `nested_or` (42 out under `broken/compile/`, 20 in under `broken/load/`, a refused
  program's), `eq_in_a_macro_body` (42 in), `query-goal-free-head` and its `-unsat` sibling (29 out and 7 in each, now
  refused), `macro_arity_mismatch` (8, its position), `features/02` (22, its goal), and the nine witnesses (20 in each).
  No other corpus program's rendering moved, its read-outs among them, so no other answer did.
- `help_shape.txt`, re-blessed (`EIN_BLESS=1 cargo test -p ein-cli --test help_surface`): the four views'
  `--no-require-why`, `render program`'s about line; 82 options (`configuration.md`, AGENTS.md, `docs/kernel/README.md`).
- Fixtures: `nested_or.{ein,expected}` under `examples/broken/load/`, its header restated and its `.expected` the
  loader's line; `activator_arity.expected` and `macro_arity_mismatch.expected` restated; `eq_in_a_macro_body.{ein,expected}`
  new, `compile-negative`; `features/02_star_in_identifiers.ein`'s goal `(is-a* Rex Animal)`, its five declared runs
  exiting 0 as before (Q-M1f.76); corpus entries for the nine witnesses and the new fixture, the moved one's
  `load-negative` with `render rules`.

**The tests.** New: `ein-ir`'s `from_ir::tests::the_compilers_refusals_a_program_reached_are_refused_at_load` (each
shape, each place, written and through a `forall` and a program's macro, the ceiling at 256 and 257, and what loads);
`ein-cli`'s `cli_semantics::a_compile_refusal_reads_the_same_on_every_command` (twelve programs on seventeen commands,
and the macro body's line); `corpus_cli::no_cell_prints_the_python_spelling`; `ein-infer`'s
`compile_semantics::no_corpus_program_is_refused_in_the_python_spelling` (every corpus program loaded, every plan and
goal compiled, the one compile refusal `eq_in_a_macro_body`'s); `fuzz_properties::the_compiler_refuses_nothing_the_loader_accepts`
(the seventh property). Turned around: `render_doors::every_view_refuses_what_the_loader_refuses` (was
`render_program_draws_what_the_loader_refuses`) and `cli_semantics::render_reports_a_load_error_as_every_command_does`
(was `render_views_the_ir_and_never_reports_a_load_error`). Restated: `compile_semantics`' four, `compile_limits`'
`one_variable_past_the_ceiling_is_refused_at_load`, `program_read::a_containers_positions_name_its_stored_program`,
`config_reference::require_why_decides_loading_only` (eleven commands), `help_surface`'s counts, `register_region`
(a command's refusal where the right answer is one is the loader's) and `region_modes::one_verdict_on_every_corpus_program`
(its two unread programs, `--monotone` answering where the read-out stopped on the goal, are none since: one refused,
one rewritten).

## T3 record, 2026-11-10

**The pages.**
- [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) § 4: the compile row is the macro body's line,
  naming no class; the paragraph on class names says `CompileError` went and `FileNotFoundError`, the read path's, is
  F23's D-1; § 4.3's two rows in the new spelling, the position first, the same on every command that compiles, the
  views and `kb save` accepting the program (Q-M1f.67); a new
  [§ 4.15](../../../docs/kernel/defined_behaviour.md#415-the-compilers-refusals-a-program-reached-at-load--m1f-s1f521),
  the three refusals at load, what each replaced (W62 to W64), the views' refusal (W65, Q-M1f.71), what pins them and
  what is not refused; § 1.5's count (14 of 34 fixtures end `at None`, `grep -L 'at None'
  examples/broken/load/*.expected` names twenty, 2026-11-10) and `macro_arity_mismatch`'s position in the new spelling;
  § 4.8's history of the `_` head names the compile row as it read then.
- [`02_patterns.md`](../../../docs/kernel/ir/03-ein-lang/02_patterns.md)'s *where a malformed primitive is caught*:
  the macro body's arity lines are the only compile refusals a program reaches, their line in Ein's words and what it
  was, where a macro body's primitive is refused is S1f.5.24's, and a nested `(or …)` is the loader's.
- [`00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) § 4: what the grammar does not refuse gains *where an
  `(or …)` sits, how many variables a clause writes, and a goal's relations*, each the loader's since.
- [`04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md): `render program` reads the program
  through the loader first; [`configuration.md`](../../../docs/kernel/configuration.md) § 3.1 and § 3.4, 82 options,
  `--no-require-why` on eleven commands; [`inference/implementation.md`](../../../docs/kernel/inference/implementation.md),
  the register ceiling refused at load.
- [F23](../../followups/f23_diagnostics_in_eins_words.md)'s D-1 row: its `CompileError` half taken here,
  `FileNotFoundError` left with the line T1 re-took; D-2's: its `Loc(…)` half taken here, `at None` left.
- The fixture READMEs (`examples/broken/compile/`, `examples/broken/load/`), `examples/README.md`'s catalogue (177
  witnesses under `ein-bugs/wrong-answers/`, `ls examples/ein-bugs/wrong-answers | wc -l`, 2026-11-10), the close
  review's link to `nested_or.ein`, and the review bundle's program page (`utils/review/run.py`), which says the views
  draw what the loader read.

`utils/doc_audit.py --links --check` over the gate's scope: 399 pages, 0 findings (2026-11-10).

## Acceptance, re-taken (2026-11-10)

Each item, and what holds it:

- *No command prints `CompileError`, `SForm(`, `Atom(name=`, `Loc(` or `S1.8.` on stderr for any corpus program,
  held by a test over the corpus and the witnesses, every command*: **met**. `corpus_cli::no_cell_prints_the_python_spelling`
  holds every corpus cell's stderr (those five, `Var(name=` and *M1 compiler*); `compile_semantics::no_corpus_program_is_refused_in_the_python_spelling`
  every corpus program's load error and every plan and goal it compiles, which is the text every command prints behind
  at most `kb load error: `; `cli_semantics::a_compile_refusal_reads_the_same_on_every_command` the witnesses on
  seventeen commands. `Loc(` is the loader's too, D-2's, taken here (Q-M1f.75): `macro_arity_mismatch` was its one
  corpus program (`sweep1.sh`, T1).
- *Every witness is refused by every command with one first line and its load-error exit code, held by a test*:
  **met** for the refusals moved to load: the twelve programs of
  `cli_semantics::a_compile_refusal_reads_the_same_on_every_command` (the nine witnesses, `nested_or.ein`, the two goal
  fixtures), each on seventeen commands, the views and `kb save` among them, exit 1 (`ein test` 2), one first line,
  `kb save` writing no container. The macro body's line, which the user's ruling on Q-M1f.67 leaves at compile and
  S1f.5.24 places, is one first line on every command that compiles, held by the same test, and the views and `kb save`
  accept that program (the ruling: none executes it).
- *Every exit that moved is a register row, `fixed`*: **met**. W62's six, W63's three, W64's two and W65's five,
  `fixed`, filed `open` first and held to the unchanged engine (T1); every other exit that moved is a load-negative's
  `render rules` (twelve cells) under Q-M1f.71, the views' default W65's rows stand for, or `nested_or.ein`'s move
  between groups. `wrong_answers.rs` and `ein-corpus`'s `the_register_is_well_formed` hold them.
- *`./run_tests.sh` green*: **met**, `build/gate-s1f.5.21.log` (green, 17 min 20 s, `./run_tests.sh`, 2026-11-10, exit 0: the six static checks, `cargo test --workspace`, the scratch check, the review bundle's smoke and the bench smoke).
