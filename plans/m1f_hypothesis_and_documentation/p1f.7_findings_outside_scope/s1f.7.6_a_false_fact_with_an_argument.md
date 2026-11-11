# S1f.7.6 — A written `(false k)`: `false` with an argument

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from [S1f.5.23's T2 record](../p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md#t2-record-2026-11-10)
(its out-of-scope finding 1), filed here under [Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes).
**Done 2026-11-11**: W81, five rows filed and fixed; refused at the parse, and at load where a macro or a parameter
writes it ([the records](#t1-record-2026-11-11)).
**Depends on:** nothing. It may edit `ein.y`'s `FactHead`, so it does not run beside
[S1f.7.7](s1f.7.7_an_absent_fact.md).

## The finding

Recorded on the release binary at `f62e34e1`:

> **`(false k)` written as a fact kills root**: root dead, R empty, the argument read as nothing. S1f.5.24 refuses
> `(false ?x)` a macro's expansion concludes (*false takes no argument, not 1*) and the grammar a rule's written
> `(false k)`, so a written fact is the one place `false` with an argument is taken.

So `false` with an argument is refused in two places and taken in a third. Where it is taken, the argument is
dropped in silence and the program is a contradiction.

## The direction the rulings point to

The grammar and S1f.5.24 (W68, [Q-M1f.81](../open_questions.md#q-m1f81--what-a-malformed-primitive-is)) refuse
`false` with an argument wherever a rule writes it. The direction that agrees is to refuse it as a written fact too:
`FactHead` takes `false` only at no argument, with a row in `00_ebnf.md`'s table. The census also reads `false` as a
nested fact's head and in a claim.

The other direction would make `(false k)` a fact of an ordinary relation `false`. That gives the word two meanings
by arity, so it is a question for the user.

## Tasks

- **T1f.7.6.1, confirmation and the census.** Re-take the finding. Read `false` with arguments in every place a fact
  or a value is written (a top-level fact, a nested fact, a stored negation, a claim, a `--diff-models` file), each on
  every command.
- **T1f.7.6.2, the rows.** One witness per way the engine answers. Each is filed `open` and held to today's engine.
- **T1f.7.6.3, the refusal.** In the grammar where it can, with the rows flipped and a test on every command.
- **T1f.7.6.4, the pages.** `01_grammar.md`, `00_ebnf.md` re-blessed, `06_reserved_names.md`, `defined_behaviour.md`,
  and the stage's records.

## Acceptance

- `false` with an argument is refused in every place, on every command, with one line.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.6 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.

## T1 record, 2026-11-11

**Confirmed, with the finding's second sentence corrected** (the analysis of 2026-11-11): on the release binary at
`52e05fc1` the argument is not dropped. `(false k)` is stored and printed with it, `k` is a name of the program,
root is dead and R is empty. The class, with `(p a)`:

| the program writes | at `52e05fc1` |
|---|---|
| `(false k)` | stored, root dead, R empty |
| `(q (false k))`, `(not (false k))` | stored, root alive |
| a premise `(false ?s)`, a goal `(false ?s)` | loads, a pattern over stored `(false …)` facts: no row |
| `(absent (false ?s))` in a `:match` | holds, so the rule fires: `(won a)` derived |
| a pattern's argument `(q (false ?s))`, a premise `(not (false k))` | loads, matches nothing |
| `:assert (q (false ?x))`, `:assert (not (false ?x))` | `(q (false a))`, `(not (false a))` stored |
| `:assert (false ?x)` written | a parse error already; through a macro refused since S1f.5.24 (W68) |
| a macro whose body is `(false ?a)` called as a premise; `(q (false ?a))` | loads; under `(absent …)` the rule fires |
| `(mk false)` beside `(rule mk (?R) :match (p ?x) :assert (?R ?x))` | `(false a)` stored, root dead; as a premise `(?R ?x)` matches nothing |
| a claim's `(model (false k))` | refused at load already: *the program names no relation false* |
| `(false)`, `(q (false))`, `(not (false))`, a premise `(false)` | load, and stay (Q-M1f.123; F25's R-7) |

Every command answered each, as in S1f.7.7's record.

## T2 record, 2026-11-11

Five witnesses and rows of W81: `false-fact-argument` (stored, root dead), `false-fact-nested`, `false-fact-negated`,
`false-premise-argument` (`(absent (false ?g))` holds, the rule fires for every guest) and `false-goal-argument`
(the goal holds of every guest). Filed `open` and held to the unchanged engine with S1f.7.7's
([its T2 record](s1f.7.7_an_absent_fact.md#t2-record-2026-11-11)).

## T3 record, 2026-11-11

Fixed with S1f.7.7, S1f.7.10 and S1f.7.3, one commit
([S1f.7.7's T3 record](s1f.7.7_an_absent_fact.md#t3-record-2026-11-11) has the sweep, the goldens and the
fixtures). **The grammar** writes `false` only as `(false)`: `FactHead`, `PremiseHead` and `ArgHead` lose the word,
and `Fact`, `NestedFact`, `Premise`, `Arg` and `Pattern` take `'(' 'false' ')'`, a top-level one with its
`:source` or `:rule`. An argument written after `(false` is a parse error with the note *false takes no argument:
(false) says that the state is impossible, and names nothing*. **The loader**'s `Shapes` walk refuses a premise
and a value `(false ?x)` a macro's expansion writes, as it refused a conclusion's since S1f.5.24, and a relation
parameter an activator can bind to `false` heading a form of one argument or more.

```
examples/ein-bugs/wrong-answers/false-fact-argument.ein:24:8: unexpected `Bob`: a name
  note: false takes no argument: (false) says that the state is impossible, and names nothing
kb load error: <file>:<l>:<c>: rule 'r': its :match writes (false ?x), and false takes no argument, not 1 — (false) holds where the state is impossible, and names nothing
kb load error: <file>:<l>:<c>: rule 'r': its :match writes (q (false ?x)), and (false ?x) is written where a value goes — false takes no argument, and a value is a name, a variable, or a fact a state could hold, (relation argument …) or its negation
kb load error: rule 'mk': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (mk false) binds ?R to false — false takes no argument: (false) says that the state is impossible, and names nothing
```

W81's five rows flipped to `fixed`. `(false)`, `(q (false))` and `(not (false))` load as before (Q-M1f.123, F25's
R-7).

## T4 record, 2026-11-11

`defined_behaviour.md` § 4.22, `06_reserved_names.md` (`false`'s arity, 0 where it said 0+),
`01-ein-graph/03_ein_model.md` § 6 (the sentence that planned `(false <witness>)`), `01_grammar.md`, `00_ebnf.md`
re-blessed, and this file.
