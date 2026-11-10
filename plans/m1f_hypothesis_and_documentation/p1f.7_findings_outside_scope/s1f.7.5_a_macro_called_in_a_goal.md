# S1f.7.5 — A macro called in a query's `:goal`

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 16. Not started.
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
