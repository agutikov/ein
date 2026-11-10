# S1f.7.10 — `(eq k j)` as a value: a nested fact headed `eq`

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-11, from the [analysis of 2026-11-11](README.md#the-analysis-of-2026-11-11), on the user's
ruling of that day (*"yes - it become S1f.7.10 onward"*). Not started.
**Depends on:** [S1f.7.7](s1f.7.7_an_absent_fact.md) and [S1f.7.6](s1f.7.6_a_false_fact_with_an_argument.md), which
edit the same production, `ein.y`'s `ArgHead`. It runs before [S1f.7.3](s1f.7.3_a_connective_as_a_patterns_argument.md),
whose walk refuses as a pattern's argument what no fact can hold.

## The finding

Taken on the release binary at `4bf6da73`, 2026-11-11 (`ein saturate --dump`):

| program | stored |
|---|---|
| `(eq k j)` | `(false)`: a written equality is checked, and this one is false (S1f.5.7) |
| `(q (eq k j))` | `(q (eq k j))`, alive, R {∅} |
| `(q (neq k j))` | refused at the parse: `unexpected neq` |
| `(a k) (a j)`, `(rule r () :match (and (a ?x) (a ?y)) :assert (q (eq ?x ?y)))` | `(q (eq k k))`, `(q (eq k j))`, `(q (eq j k))`, `(q (eq j j))` |
| `(q (eq k j)) (q (eq k k))`, `:match (q (eq ?a ?b)) :assert (hit ?a ?b)` | `(hit k j)` and `(hit k k)` |

S1f.5.7 ruled `eq` and `neq` *the relations of equality and non-equality, computed and never stored*. As a value
`eq` is neither: `(eq k j)` inside a fact is stored and nothing computes it. So one spelling has two meanings. As a
premise `(eq ?a ?b)` holds only where the two are equal. As a pattern's argument it matches any stored `(eq x y)`
by its shape, so the last row derives `(hit k j)`, of two names that are not equal. And `neq` in the same place is
a parse error, so the two relations of one ruling differ.

## The direction

A nested fact is a value and is not asserted: `(q (p a))` does not make `(p a)` hold. So `(q (eq k j))` can be read
as *q of the statement that k equals j*, which is not false. Two directions:

- **(a) Refused, as `neq` is** (the default): `ArgHead` loses `eq`, so no fact, conclusion or pattern writes
  `(eq …)` as an argument. It agrees with *never stored* and with what `neq` already does.
- **(b) A value like any other**: `eq` and `neq` both allowed as a nested head, matched by shape, and the pages say
  that a nested `eq` is not the guard.

**Ruled 2026-11-11 (the user): (a)**, *"adopt the rule"*, as [the value rule](README.md#the-value-rule): a value
is a name or a fact a state could hold. The probe that settled it is a row of this stage's T2: with
`(h (eq k j)) (h (eq k k))` and `:match (h ?s) :assert (not ?s)`, `(not (eq k j))` and `(not (eq k k))` are stored
and no `(false)` is derived, where S1f.5.7 has a concluded `(not (eq …))` checked.

## Tasks

- **T1f.7.10.1, confirmation and the census.** Re-take the table. Every site: a fact's argument at any depth, a
  rule's and an hrule's conclusion, a premise pattern's argument, a goal's, a claim's, a macro's body and argument,
  and a parameter that binds `eq` (S1f.5.9's `refuse_eq_at_another_arity`). Every corpus program that writes one.
- **T1f.7.10.2, the rows.** One witness per way the engine answers: the stored value, the conclusion, the pattern
  that matches by shape. Each filed `open` and held to today's engine.
- **T1f.7.10.3, the fix**, the rows flipped, and a test on every command.
- **T1f.7.10.4, the pages.** `00_ebnf.md`'s table, `06_reserved_names.md`, `defined_behaviour.md`, the records.

## Acceptance

- `eq` and `neq` are treated alike as a nested head, on every command.
- Every witness is a register row, `open` before the fix and `fixed` after it. No corpus answer moves but the
  witnesses' and the fixtures the stage names.
- `./run_tests.sh` green.
