# S1f.7.3 — A connective as a premise pattern's argument

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 14. **Done 2026-11-11**: W83, five rows filed and fixed; refused at
load, after macro expansion ([the records](#t1-record-2026-11-11)).
**Depends on:** nothing. It extends [S1f.5.24](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s
walk, `ein_ir::from_ir::refuse_malformed_primitives`.

## The finding

Recorded in [S1f.5.24's T2 record](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md#t2-record-2026-11-10)
on the release binary at `dbfd7862`:

> **A connective written as a premise pattern's argument reads nothing, in silence.** `(rule r () :match (and (p
> ?x) (q (and (p ?x)))) …)`: the grammar's `Arg` takes a connective, so a macro can be called there, and the
> loader and the compiler read `(and (p ?x))` as a pattern inside a pattern whose relation is `and`, which no fact
> holds, so `r` never fires (written, and through a macro's argument, `(macro m (?a) (q ?a))` called with a
> conjunction). The same shape is a parse error as a conclusion's or a guard's value, and S1f.5.24 refuses it
> there; in a premise pattern it is the grammar's own, so not this stage's.

So the grammar accepts a shape that has no meaning, and the program loads with a rule that can never fire.

## The direction the rulings point to

S1f.5.24 refused the same shape after macro expansion as a conclusion's value and as a guard's value (W67, W68,
[Q-M1f.81](../open_questions.md#q-m1f81--what-a-malformed-primitive-is)). The direction that agrees is to
refuse it as a premise pattern's argument too, after expansion, with the same line. That covers `and`, `or`, `not`,
`absent` and a guard as an argument, each one T1 shows means nothing there.

The grammar keeps `Arg` taking a connective only if a macro call needs it. T1 reads why `Arg` takes one. If no
macro call needs it, the grammar refuses the written shape at the parse, and the loader refuses only what an
expansion produces.

## Tasks

### T1f.7.3.1 — confirmation, and the class

Re-take the finding. List every connective and primitive that `Arg` admits in a premise pattern, written and
through a macro's body or argument, in a rule, an hrule, an obligation and a goal, each on every command. Find out
why `ein.y`'s `Arg` takes a connective.

### T1f.7.3.2 — the rows

One witness per way the engine answers. Each is filed `open` and held to today's engine.

### T1f.7.3.3 — the refusal

The grammar, the walk, or both, as T1 decides, then the rows flipped. A row is added to `00_ebnf.md`'s table if the
grammar takes the refusal. A test holds every witness on every command.

### T1f.7.3.4 — the pages

[`02_patterns.md`](../../../docs/kernel/ir/03-ein-lang/02_patterns.md), `00_ebnf.md` (re-blessed if the grammar
moves), `defined_behaviour.md` § 4.16, and the stage's records.

## Acceptance

- A connective or primitive with no meaning as a premise pattern's argument is refused on every command, written or
  produced by a macro, with S1f.5.24's line.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.3 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `52e05fc1`, with `(p a)`:

| the `:match` holds | at `52e05fc1` |
|---|---|
| `(q (and (p ?x)))` | loads; the rule never fires |
| `(absent (q (and (p ?x))))`, `(absent (q (or (p ?x) (q ?x))))`, `(absent (q (neq ?x b)))` | loads; the dead pattern makes the `absent` hold, and the rule **fires**: `(won a)` derived |
| the same in an hrule | proposes `(won a)` under the `absent`, R two states; without it H is empty |
| the same in a goal | *the goal holds*, a row for every binding of the rest |
| a macro `(macro m (?a) (q ?a))` called `(m (and (p ?x)))`, at the top of a premise and inside a pattern | the same, through the expansion |
| `std.macro`'s `forall` called inside a pattern, `(h (forall ?y (p ?y) (q ?y)))` | `(h (absent (and …)))`, a pattern no fact matches |
| `(q (absent (q ?x)))`, `(q (eq ?a ?b))` | matched a stored `(q (absent …))` or `(q (eq …))`, which S1f.7.7 and S1f.7.10 leave no fact able to be |
| `(q (not (p ?x)))` | matches a stored `(q (not (p a)))`: right, and stays |
| `(q (not (and (p ?x))))` | a parse error already |

**Why `Arg` takes a connective**: a macro is called as a pattern, and a call's argument may be a premise.
`std.macro`'s `forall` is `(forall ?b G B)` with `G` and `B` premises: the stdlib's `total` writes `(forall ?b (?isa
?b ?B) (not (?R ?a ?b)))`, and `examples/ein-bugs/wrong-answers/forall-core-elim.ein` `(forall ?s (and (slot ?s) (neq
?s ?t)) (not (in ?x ?s)))`; the grammar reads the call as a pattern headed `forall` and its arguments as `Arg`s.
A context-free grammar cannot tell a macro's name from a relation's, so the written shape cannot be refused at the
parse without refusing those calls: the refusal is the loader's, after expansion (Q-M1f.125).

## T2 record, 2026-11-11

Five witnesses and rows of W83: `pattern-argument-and` (never fires), `pattern-argument-and-under-absent` (fires for
every guest), `pattern-argument-hrule` (an `(or …)`, H empty), `pattern-argument-goal` (a `(neq …)` under `absent`,
the goal holds of every guest) and `pattern-argument-macro` (through a macro's argument). Filed `open` and held to
the unchanged engine with S1f.7.7's ([its T2 record](s1f.7.7_an_absent_fact.md#t2-record-2026-11-11)).

## T3 record, 2026-11-11

Fixed with S1f.7.7, S1f.7.6 and S1f.7.10, one commit
([S1f.7.7's T3 record](s1f.7.7_an_absent_fact.md#t3-record-2026-11-11) has the sweep, the goldens and the
fixtures). The refusal is the loader's (Q-M1f.125): `ein_ir::from_ir`'s `Shapes::arg`, the walk S1f.5.24 gave a
rule's, an hrule's and an obligation's clauses and S1f.7.5 a goal, reads a pattern's argument after macro
expansion. Until this stage it read a connective there as a premise, the grammar's reading. It refuses `and`,
`or` and `neq` with S1f.5.24's line, and `eq`, `absent` and `false` with an argument with the value rule's
(`Shapes::no_value`); `(not <pattern>)` is read as a stored negation, whose pattern is a value's. The grammar is
unmoved for this stage: `Arg` takes a `Connective`.

```
kb load error: examples/ein-bugs/wrong-answers/pattern-argument-and-under-absent.ein:27:53: rule 'greet': its :match writes (says (and (paid ?g))), and (and (paid ?g)) is a connective where a value goes — and heads no fact, and a value is a name, a variable or a fact (relation argument …)
kb load error: examples/ein-bugs/wrong-answers/pattern-argument-goal.ein:26:44: query: its :goal writes (says (neq ?g Bob)), and (neq ?g Bob) is a connective where a value goes — neq heads no fact, and a value is a name, a variable or a fact (relation argument …)
kb load error: <file>:<l>:<c>: rule 'r': its :match writes (q (absent (q ?x))), and (absent (q ?x)) is written where a value goes — absent is negation as failure, a premise's test that no fact is of, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation; the stored negation of a fact is (not …)
```

W83's five rows flipped to `fixed`. The rule of `pattern-argument-and-under-absent`, written as it was meant,
`(absent (says (paid ?g)))`, welcomes Bob alone (`value_rule.rs`'s `the_meant_program_answers`). No stdlib rule
and no corpus program wrote the shape: the sweep moved none.

## T4 record, 2026-11-11

`defined_behaviour.md` § 4.22, `02_patterns.md`, `00_ebnf.md` § 4 (a connective as a pattern's argument is the
loader's), `06_reserved_names.md`, and this file.
