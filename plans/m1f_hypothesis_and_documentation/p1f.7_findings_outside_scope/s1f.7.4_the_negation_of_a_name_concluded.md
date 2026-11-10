# S1f.7.4 — A conclusion `(not ?x)` whose variable binds a name

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 15. Not started.
**Depends on:** nothing.

## The finding

Recorded in [S1f.5.24's T2 record](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md#t2-record-2026-11-10)
on the release binary at `dbfd7862`:

> **A conclusion `(not ?x)` whose variable binds a name stores the negation of a name.** `(rule r () :match (a
> ?x) :assert (not ?x) …)` stores `(not k :rule r)`, which no program can write (`(not k)` is a parse error as a
> fact and as a conclusion). The grammar takes `(not Var)` as a conclusion, for a variable bound to a fact.

So the store holds a fact that the language cannot state. It reaches every surface that prints a state, and a
`--dump-answer` state file that holds it does not reload.

## What is open

The grammar takes `(not ?x)` because `?x` may be bound to a fact, and whether it is depends on the data. Three
directions:

- **A refusal at load** wherever the program shows that `?x` can bind a name, read from the positions that hold
  names in the way S1f.5.18 and S1f.5.9 read relation names (`refuse_derived_relation_activators`,
  `refuse_eq_at_another_arity`). This agrees with how the phase refused shapes that no run can answer, but it
  refuses a program in which only some bindings are names.
- **A false conclusion**: the firing that would conclude the negation of a name concludes `(false)` instead, as
  S1f.5.7 treats a false `(eq …)` conclusion (G-21, Q-M1e2.26).
- **A stop** with Ein's line where the firing meets a name. This breaks *exit non-zero only on error* unless the
  program is wrong, and it is answered only where a firing meets it.

T1 measures where the corpus and the stdlib write `(not ?x)` and what each variable binds. The choice is a Q-M1f
row. The first direction is the default if the census shows no program in which a variable binds both a name and a
fact.

## Tasks

### T1f.7.4.1 — confirmation, and the census

Re-take the finding on every command, including `--dump-answer` with its reload and a saved container. Census every
`(not ?x)` conclusion in the corpus, the stdlib and the Rust suites' inline programs, and what its variable can
bind.

### T1f.7.4.2 — the rows

One witness per way the engine answers wrongly: the stored `(not k)`, the state file that does not reload, and any
claim that holds of it. Each is filed `open` and held to today's engine.

### T1f.7.4.3 — the fix

The chosen direction. The rows are flipped. A test holds every witness on every command.

### T1f.7.4.4 — the pages

`02_patterns.md` (what `(not ?x)` concludes), `defined_behaviour.md`, and the stage's records.

## Acceptance

- No run stores the negation of a name, and every command answers a program that would store one the same way.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves, or
  each that moves is named and ruled.
- `./run_tests.sh` green.
