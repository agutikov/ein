# S1f.7.10 — `(eq k j)` as a value: a nested fact headed `eq`

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-11, from the [analysis of 2026-11-11](README.md#the-analysis-of-2026-11-11), on the user's
ruling of that day (*"yes - it become S1f.7.10 onward"*). **Done 2026-11-11**: W82, four rows filed and fixed;
`(eq …)` as a value refused at the parse, and at load as a pattern's argument
([the records](#t1-record-2026-11-11)).
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

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `52e05fc1`: the finding's table re-taken row for row, and the probe that
settled the ruling. The class beyond the table:

| the program writes | at `52e05fc1` |
|---|---|
| `(h (eq k j)) (h (eq k k))` and `:match (h ?s) :assert (not ?s)` | `(not (eq k j))` and `(not (eq k k))` stored, no `(false)`, root alive: the probe |
| `(q (not (eq k j)))` | stored |
| `:assert (q (not (eq ?x b)))` | `(q (not (eq a b)))` stored |
| a goal `(q (eq ?a ?b))` beside `(q (eq k j))` | a row, `?a = k · ?b = j`, matched by shape |
| a macro whose body is `(q (eq ?a ?a))` called in an `:assert` | `(q (eq a a))` stored |
| `(mk eq)` beside `:assert (q (?R ?x ?x))` | `(q (eq a a))` stored; § 4.14's check reads a parameter that heads a premise or a conclusion, not one inside a pattern |
| a claim's `(model (p a) (q (eq a b)))` | refused at load already, *which no atom of the query's :goal matches* |
| `(eq a b)`, `(not (eq a b))` as a fact; `(not (eq ?x b))` as a premise and a conclusion; `(eq a b)` in a claim | checked, S1f.5.7's: these stay (Q-M1f.124) |

No tracked `.ein` file writes `(eq …)` as a value: the sweep of T3 moved none for it.

## T2 record, 2026-11-11

Four witnesses and rows of W82: `eq-value-fact` (stored), `eq-value-concluded` (a rule stores four),
`eq-value-matched-by-shape` (`(same Ann Bob)` derived of two names that are not equal) and
`eq-value-negation-concluded` (the probe, word for word). Filed `open` and held to the unchanged engine with
S1f.7.7's ([its T2 record](s1f.7.7_an_absent_fact.md#t2-record-2026-11-11)).

## T3 record, 2026-11-11

Fixed with S1f.7.7, S1f.7.6 and S1f.7.3, one commit
([S1f.7.7's T3 record](s1f.7.7_an_absent_fact.md#t3-record-2026-11-11) has the sweep, the goldens and the
fixtures). **The grammar**'s `ValueHead` holds no `eq`, so `(eq …)` inside a fact, a conclusion, a guard's value
and a claim's fact is a parse error, as `(neq …)` there was, each with the note *eq is computed and never stored,
so (eq a b) is no value: write it as a premise or a conclusion of its own, where it is checked, and never inside
another fact or pattern*. `(eq a b)` stays where S1f.5.7 checks it: a fact (`FactHead`), `(not (eq a b))` as a
fact, a premise and a conclusion (`EqFact`, `Negated`), and a claim's (Q-M1f.124). **The loader** refuses `(eq
…)` as a pattern's argument, which the grammar reads as a guard written where a macro's argument could be
(S1f.7.3's walk), the same left by a macro's expansion, `(not (eq …))` where a value goes, and a relation parameter
an activator can bind to `eq` heading a form where a value goes, at any arity.

```
examples/ein-bugs/wrong-answers/eq-value-fact.ein:25:12: unexpected `eq`
  note: eq is computed and never stored, so (eq a b) is no value: write it as a premise or a conclusion of its own, where it is checked, and never inside another fact or pattern
kb load error: <file>:<l>:<c>: rule 'r': its :match writes (q (eq ?a ?b)), and (eq ?a ?b) is written where a value goes — eq is computed and never stored, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation
kb load error: rule 'mk': its parameter ?R heads (?R ?x ?x), written where a value goes, and the fact (mk eq) binds ?R to eq — eq is computed and never stored, and a value is a name, or a fact a state could hold
```

W82's four rows flipped to `fixed`. The probe's program, `eq-value-negation-concluded`, is refused at its first
`(h (eq k j))`, so no state holds an `(eq …)` for `(not ?s)` to negate.

## T4 record, 2026-11-11

`defined_behaviour.md` § 4.22 and § 4.14 (a parameter heading a value is read since), `06_reserved_names.md`
(§ Computed predicates), `01_grammar.md`, `00_ebnf.md` re-blessed, and this file.
