# S1f.7.1 — A goal's nested pattern whose relation is a variable: refused, as in a rule

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 12. **Done 2026-11-11**, with S1f.7.5 and S1f.7.11 in one
subagent: the records are below. W79, two rows, filed and fixed; Q-M1f.121 defaulted.
**Depends on:** [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md), so the census reads a goal after its macros are
expanded.

## The finding

Recorded in [S1f.5.21's T1 record](../p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md#t1-record-2026-11-10)
on the release binary at `864b16ed`, not re-checked since:

> A query's goal that writes a pattern inside a pattern whose relation is a variable, `(query :goal (p (?R ?x)))`,
> loads, and the compiler lowers the inner pattern as an opaque slot that matches nothing, so `(p (g k))` gives the
> goal no row, in silence (`goal-free-head-nested.ein`, … *the goal has no row: undetermined here*). In a rule the
> same shape is refused at load since M1e2 S1e2.2.24 (W24's `nested-unbound-head`).

So one shape gets two answers: refused in a rule, and in a goal answered as if no fact matched. That breaks
*one tool, consistent answers*, and the goal's answer is wrong, since `(p (g k))` is the fact the goal asks about.

## The direction the rulings point to

The user ruled on Q-M1e2.16 (1), and again on Q-M1e2.18, that this shape is refused at load in a rule
(S1e2.2.24, W24). The direction that agrees is the same refusal in a goal: one line in S1f.5.21's spelling, on
every command, naming the query and the clause.

The other direction would give the shape a meaning in both places: a nested pattern whose relation the match binds
from the fact. That reverses the W24 ruling, so it is a question for the user, not a default.

## Tasks

### T1f.7.1.1 — confirmation, and every check a goal skips

Re-take the finding. Then list every load-time check a rule's `:match` gets (`refuse_no_fact_holds`,
`refuse_nested_disjunctions`, `refuse_malformed_primitives`, the variable-width limit, `validate_open`, the
reserved names and any others the loader holds) and whether a goal's premise gets it, each with a witness. A
skipped check that changes an answer is part of this stage if it is the same shape, and becomes a new P1f.7 stage
if it is not.

### T1f.7.1.2 — the rows

One witness per way the engine answers, on every command that reads a goal (every `solve` question, `test` with a
claim, `oracle`, the views, `kb save` and a saved container). Each is filed `open` and held to today's engine.

### T1f.7.1.3 — the refusal

The goal is held to the rule's checks at load. The rows are flipped to `fixed`. A test holds every witness on every
command.

### T1f.7.1.4 — the pages

[`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query), `defined_behaviour.md` § 4,
and the stage's records.

## Acceptance

- A goal's nested pattern whose relation is a variable is refused on every command with the rule's line.
- Every check a rule's `:match` gets and a goal's premise skipped is listed, and each one that moves an answer is
  fixed here or is a P1f.7 stage.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.1 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `73a0c17d`. Beside the fact `(says (paid Bob))`, `(query :goal (says (?R
?g)))` loads and reports *the goal has no row: undetermined here*; `(query :goal (absent (says (?R ?g))))` loads
and reports *the goal holds: it binds no variable*, in every state. `(query :goal (?R ?g))`, the same variable at
the top level, is refused at load since S1f.5.21.

**Every load-time check a rule's `:match` gets, and whether a goal's premise got it** at `73a0c17d`:

| check | what it refuses | a goal's premise |
|---|---|---|
| `refuse_no_fact_holds`: a relation written as a variable | `(?R ?x)` with `?R` no parameter, at any depth | the top level only, by a walk of the goal's own (`refuse_goal_relation_variables`, W64). **Inside a pattern: not checked**, this stage's finding |
| `refuse_no_fact_holds`: a guard over a form holding a variable | `(eq ?x (f ?y))` | checked, by a walk of the goal's own (`refuse_guard_patterns`, Q-M1f.30) |
| `refuse_no_fact_holds`: a premise that is no form | a name or a variable a macro's expansion leaves | **not checked**; only a macro's expansion writes one, and a goal's was not expanded (S1f.7.5) |
| `refuse_nested_disjunctions` | an `(or …)` written as a premise | checked (S1f.5.21, W62) |
| `refuse_wide_matches` | more than 256 variables | checked (S1f.5.21, W63) |
| `refuse_malformed_primitives` | a kernel primitive in a shape only a macro's expansion writes | **not checked**; S1f.7.5's, for the same reason |
| `macros::unimported_macro_errors` | a `std.macro` call without its import | **not checked**; S1f.7.5's |
| `refuse_unbound_conclusions`, `validate_open` | a conclusion's variable; the verdict | a goal has no conclusion, and the grammar writes no `open` in one |
| `validate_relation_parameters`, `refuse_derived_relation_activators`, `refuse_eq_at_another_arity` | what an activator binds a parameter to | a query has no parameter |
| the reserved names, `:why` required, a `:why`'s references | a rule's name and text | a query has no name; its `:goal-text`'s references are checked |

So one check a goal skipped is this stage's shape, and three more are reachable only through a macro call, which
S1f.7.5 fixed with the expansion. None is left, and none became a stage.

## T2 record, 2026-11-11

Two witnesses under `examples/ein-bugs/wrong-answers/`, each a corpus entry (`solve`, `saturate`) and a register
row, W79:

| row | witness | answered at `73a0c17d` | right |
|---|---|---|---|
| `W79/goal-nested-relation-variable` | `goal-nested-relation-variable.ein` | loads, no row beside `(says (paid Bob))` | refused at load |
| `W79/goal-absent-nested-relation-variable` | `goal-absent-nested-relation-variable.ein` | loads, *the goal holds* | refused at load |

Filed `open` with the other two stages' rows and held to the unchanged engine
([S1f.7.5's T2 record](s1f.7.5_a_macro_called_in_a_goal.md#t2-record-2026-11-11)).

## T3 record, 2026-11-11

A goal is read by the walk a rule's clauses are read by, `ein_ir::from_ir`'s `refuse_no_fact_holds`, called with
`query` for the rule and `:goal` for the clause and no parameter, so every relation written as a variable is one
nothing binds. The walk descends into a pattern's arguments, which the goal's own walk did not, and under `(and
…)`, `(or …)`, `(absent …)` and a stored negation. `refuse_goal_relation_variables` and `refuse_guard_patterns`
are gone. The line is the goal's, at every depth
([Q-M1f.121](../open_questions.md#q-m1f121--the-line-a-goals-nested-relation-variable-is-refused-with)):

```text
kb load error: query: its :goal writes (?R ?g) with ?R as its relation, which nothing binds — a relation is named by a name, or by a rule's parameter its activator binds, and a query has no activator
```

The sweep, the rows' count and the pin are in
[S1f.7.5's T3 record](s1f.7.5_a_macro_called_in_a_goal.md#t3-record-2026-11-11): no corpus goal writes the shape.
`ein_infer::compile`'s lowering of the inner pattern as a slot that matches nothing stays as the backstop a direct
caller of `compile_rule` reaches.

## T4 record, 2026-11-11

`defined_behaviour.md` § 4.21, `01_grammar.md` § Query, and the records;
[S1f.7.5's T4 record](s1f.7.5_a_macro_called_in_a_goal.md#t4-record-2026-11-11) lists the pages.

## Acceptance re-taken, 2026-11-11

- *A goal's nested pattern whose relation is a variable is refused on every command with the rule's line*:
  `goal_premises.rs`, the line being the goal's own (Q-M1f.121).
- *Every check a rule's `:match` gets and a goal's premise skipped is listed, and each one that moves an answer is
  fixed here or is a P1f.7 stage*: the T1 record's table; the three reachable through a macro are fixed by S1f.7.5.
- *Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves*: the
  T2 and T3 records.
- *`./run_tests.sh` green*: exit 0 on the stages' tree (`build/gate-s1f.7.5.log`, 2026-11-11).
