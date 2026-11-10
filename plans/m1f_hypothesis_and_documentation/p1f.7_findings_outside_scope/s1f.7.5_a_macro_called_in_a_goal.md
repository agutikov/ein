# S1f.7.5 — A macro called in a query's `:goal`

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 16. **Done 2026-11-11**, with S1f.7.11 and S1f.7.1 in one
subagent: the records are below ([T1](#t1-record-2026-11-11), [T2](#t2-record-2026-11-11),
[T3](#t3-record-2026-11-11), [T4](#t4-record-2026-11-11)). W77, four rows, filed and fixed; Q-M1f.117 and .118
defaulted; one row of F25.
**Depends on:** nothing. It runs first in the phase, since [S1f.7.1](s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md)
reads a goal after expansion.

## The finding

Recorded in [S1f.5.24's T2 record](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md#t2-record-2026-11-10)
on the release binary at `dbfd7862`:

> **A macro called in a query's `:goal` is not expanded.** `(macro pp (?a) (p ?a))` and `(query :goal (pp ?x))`:
> the goal reads a relation `pp`, which nothing declares, and *the goal has no row*, in silence, where the same
> call in a rule's `:match` is expanded (`expand_rule_clauses` reads rules only), and an unimported `std.macro`
> call in a rule is refused (`unimported_macro_errors`).

So the same call means one thing in a rule and another in a goal, and the goal's answer is wrong.

## The direction the rulings point to

A goal is a premise. The direction that agrees with *one tool, consistent answers* expands a goal's macro calls as
a rule's `:match` is expanded, and then holds the expanded goal to the checks a rule gets after expansion: S1f.5.24's
shapes, an unimported `std.macro` call refused, and an unused macro still loading (Q-M1f.67).

The other direction refuses a macro call in a goal at load. It is narrower, and it is a question for the user only
if T1 finds that expansion in a goal cannot mean what it means in a rule.

T1 also reads the query's other clauses that hold a pattern or a fact, such as `:expect`'s `(model …)` and
`(region …)` claims. A claim is a set of facts, and whether a macro call there is expanded, refused or read as a
fact is stated for each.

## Tasks

### T1f.7.5.1 — confirmation, and the class

Re-take the finding. List every clause of a query (and of a `(solve …)` form) that holds a pattern or a fact, and
what a macro call there does today on every command. That includes a `std.macro` call, imported and not, and a
macro whose expansion is malformed.

### T1f.7.5.2 — the rows

One witness per way the engine answers wrongly. Each is filed `open` and held to today's engine.

### T1f.7.5.3 — the fix

Expansion of a goal's macro calls in the loader, beside `expand_rule_clauses`, followed by the rule's checks after
expansion. The rows are flipped. A test holds every witness on every command.

### T1f.7.5.4 — the pages

[`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query), the macro page under
`docs/kernel/ir/03-ein-lang/`, `defined_behaviour.md`, and the stage's records.

## Acceptance

- A macro call in a goal means what it means in a rule's `:match`, on every command, or is refused with one line if
  the user so rules.
- The expanded goal is held to the checks a rule's `:match` gets after expansion.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.5 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `73a0c17d`. On a program holding `(p k)` and `(macro pp (?a) (p ?a))`,
`(query :goal (pp ?x))` prints *the goal has no row: undetermined here* and suggests `(model)`, where `(query :goal
(p ?x))` answers `?x = k`. With `:expect (model)` the claim **holds**, exit 0; with `:expect (model (p k))` the
program is refused at load, *:expect lists (p k), which no atom of the query's :goal matches*. The stdlib's
`unknown` in a goal behaves the same imported and not: `(and (is-a ?x Thing) (unknown (p ?x)))` has no row in any
state, where a rule's `:match` writing the unimported call is refused at load.

**Every clause of a query and of a `(solve …)` that holds a pattern or a fact**, and what a macro call there did:

| clause | holds | a macro call there, before the stage | since |
|---|---|---|---|
| `(query :goal …)` | a premise | never expanded: a pattern over a relation of the macro's name, no row | expanded, as a rule's `:match` |
| `(query :goal …)`, a `std.macro` call without its import | a premise | loads, no row, no word | refused, the rule's line with `query` for the rule |
| `(query :goal …)`, a call of another arity, or of a macro that expands to itself | a premise | loads, no row | refused, the expander's line |
| `(query :goal …)`, a call whose expansion is no premise (`(macro id (?x) ?x)`, `(id foo)`) or writes a primitive in another shape (`(eq ?x)`) | a premise | loads, no row | refused, the lines a rule's expansion gets (S1f.5.24's, `refuse_no_fact_holds`'s) |
| `(query :goal-text "…")` | text | its references are the goal's variables as written | the expanded goal's, which are the written goal's and the body's own |
| `(query :expect (model (pp k)))` | facts | refused, *names pp, which no declaration or fact makes a relation* | refused, not expanded (Q-M1f.118) |
| `(solve :expect (region (model (pp k))))` | facts | refused, the same line | refused, not expanded |
| `(solve :hrules …)`, the two relation lists | names | names, held to the program since S1f.7.8 | unmoved |

Each on every command: a query is read only where a state is answered from it, so before the stage `ein saturate`,
the views and `kb save` loaded every one of these, and `ein solve`, `ein test` and `ein oracle` answered. A macro
no rule and no goal calls still loads whatever its body (Q-M1f.67).

**Expansion in a goal means what it means in a rule.** A goal is compiled as a rule's `:match` is: `goal_bindings`
wraps it in a synthetic rule with no parameters. So the stage file's second direction, refusing a call in a goal,
was not needed. What a surface prints was the open choice
([Q-M1f.117](../open_questions.md#q-m1f117--a-goal-with-a-macro-call-printed-as-written-or-as-expanded)).

**Found beside it.** The read-out built a class from the goal's top-level conjuncts only, where the loader reads a
claim against the goal's atoms with every `(and …)` flattened. `unknown` expands to an `(and …)`, so the class of
`(and (is-a ?g Guest) (unknown (paid ?g)))` printed `(and (absent …) (absent …))` as one of its facts, which no
claim can write. A goal written with an `(and …)` inside an `(and …)` printed the same at `73a0c17d`; no corpus
program writes one. Fixed in T3.

## T2 record, 2026-11-11

Four witnesses under `examples/ein-bugs/wrong-answers/`, each a corpus entry (`solve`, `saturate`) and a register
row, W77:

| row | witness | answered at `73a0c17d` | right |
|---|---|---|---|
| `W77/goal-macro-unknown` | `goal-macro-unknown.ein` | refused at load: the claim listing what `unknown` means matches no atom of the goal | loads; one class, `?g = Bob`; the claim holds |
| `W77/goal-macro-expect-model` | `goal-macro-expect-model.ein` | the goal has no row and `:expect (model)` holds, exit 0 | the class is `(paid Ann)`; the claim is FAILED, exit 1 |
| `W77/goal-macro-unimported` | `goal-macro-unimported.ein` | loads, no row | refused at load |
| `W77/goal-macro-arity` | `goal-macro-arity.ein` | loads, no row | refused at load |

The ten rows of the three stages were filed `open` together and held to the unchanged engine: with the engine's
sources as at `73a0c17d` (`git stash`) and the ten rows open, `cargo test -p ein-corpus`, `cargo test -p ein-cli
--test wrong_answers` and `cargo test -p ein-infer --test register_region` are green, the register reading 254
fixed, 23 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). A row
whose right answer is a refusal had `now` the run `saturate --dump` answering; one whose right answer is an answer
and whose program was refused had `now` that refusal; `goal-macro-expect-model`'s had the claim held. The
register's schema is unmoved, `/21`.

**The goldens the fix moves**, named before `EIN_BLESS=1`, for the three stages: `ein-cli/tests/golden/
corpus_exits.txt` (the ten witnesses' two runs each) and `ein-render/tests/golden/corpus_shapes.md5` (the ten
witnesses' digests, and those of `examples/broken/load/expect_unknown_relation.ein`, whose header says what the
claim check reads since). The bless moved those two files and those eleven programs alone. One banked line moved
with S1f.7.11's rewording, `examples/broken/load/expect_unknown_relation.expected`.

## T3 record, 2026-11-11

**The expansion** is the loader's, `ein_ir::from_ir`'s `expand_goal`, where the program's queries are built: the
`:goal` pair's value is replaced by its expansion (`macros::expand_macros`, the expander a rule's clauses get), so
every reader of the query, the loader's checks, the compiler, `ein test`, the scope and the trace's relevance,
meets the expanded goal. `ein_core::Query::goal_written` keeps the goal as written, and `ein_infer::goal_written`
is what the read-out, the summary and `ein test`'s report print (Q-M1f.117).

**The checks** are the rule's, on the expanded goal, in `validate_queries`:

```text
kb load error: query: '(unknown …)' is a std.macro pattern macro used without importing it — add (import std.macro :symbols (unknown)); unexpanded it would silently never match
kb load error: query: its :goal: macro settled/1 invoked with 2 args at t.ein:28:15
kb load error: query: its :goal: macro expansion exceeded depth 50 — a macro likely expands to itself
kb load error: query: its :goal writes Ann as a premise of and, and Ann is no premise — a premise is a pattern (relation argument …), a guard (eq a b) or (neq a b), an (absent …), or an (and …) or (or …) of premises
kb load error: t.ein:4:18: query: its :goal writes (eq ?g), and eq takes exactly 2 arguments, not 1 — it compares two values, so write one guard per pair
```

`macros::unimported_macro_errors` takes who writes the premise, `rule 'r'` or `query`, so a rule's line is
unmoved. One walk reads a rule's clauses and a goal, `refuse_no_fact_holds`, which replaced the goal's two own
walks (`refuse_guard_patterns`, `refuse_goal_relation_variables`); `refuse_malformed_primitives` reads the goal as
a premise.

**The class** lists the goal's atoms as the loader reads them (`ein_ir::expect::goal_atoms`), so the read-out's
`as :expect` for `unknown` is `(model (absent (not (paid Bob))) (absent (paid Bob)) (is-a Bob Guest))`, which loads
and holds.

**The sweep**, for the three stages: every tracked `.ein` file, 612, under `ein saturate` (exit and stderr) and
`ein solve` (exit and both streams), the binary at `73a0c17d` against the stages' (2026-11-11). One file differs,
`examples/broken/load/expect_unknown_relation.ein`, whose refusal reads S1f.7.11's reworded line. No zebra
program's load, answer or exit moved: `ein solve` prints the same bytes on each of the six `examples/zebra*.ein`.
No fixture's program was rewritten. Three tests wrote a goal over a relation their program does not name and name
one since: `from_ir`'s two unit tests of several queries, and `solve_lists.rs`'s accepted list (S1f.7.11's T3).

**The rows** are `fixed`: 264 fixed, 13 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort |
uniq -c`, 2026-11-11), the count after the three stages.

**The pin**, `ein-cli/tests/goal_premises.rs`, for the three stages: nineteen shapes of a goal or a claim the
loader refuses and the six witnesses whose right answer is a refusal, each on twenty commands, one first line, the
same on each, and no container written; nine programs answered, the goal printed as written, `ein test` agreeing
with `ein solve` on each claim; and sixteen goals over relations of the program, each loading on five commands, a
container answering its queries as its text does. With the expansion, the walk and the relation check taken out,
four of its five tests fail.

**Found outside the stage**: [F25](../../followups/f25_roughness_found_outside_zebra.md)'s R-5, a class of a goal
whose `(absent …)` holds a variable nothing outside it binds prints the variable, which `forall` in a goal now
reaches.

## T4 record, 2026-11-11

- [`defined_behaviour.md` § 4.21](../../../docs/kernel/defined_behaviour.md#421-a-goal-is-held-to-what-a-rules-match-is-held-to--m1f-s1f75-s1f711-s1f71),
  new, for the three stages: what a goal is held to, the lines, what is not refused, the pin.
- [`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query): a goal is a premise, with the
  three consequences.
- `02_patterns.md`'s paragraph on a macro's expansion and `06_reserved_names.md`'s `macro` row: a goal's calls are
  expanded.
- `examples/README.md`'s witnesses, `AGENTS.md`, the phase README.
- `00_ebnf.md` is unmoved: the grammar did not change.

## Acceptance re-taken, 2026-11-11

- *A macro call in a goal means what it means in a rule's `:match`, on every command*: `goal_premises.rs`'s
  answered programs and W77's two answer rows.
- *The expanded goal is held to the checks a rule's `:match` gets after expansion*: the T3 record's lines, pinned
  on twenty commands.
- *Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves*: the
  T2 and T3 records.
- *`./run_tests.sh` green*: exit 0 on the stages' tree (`build/gate-s1f.7.5.log`, 2026-11-11).
