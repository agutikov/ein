# S1f.7.7 — A written or concluded `(absent k)`: a fact no premise can read

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from [S1f.5.23's T2 record](../p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md#t2-record-2026-11-10)
(its out-of-scope finding 2; S1f.5.24's T2 record notes the conclusion half), filed here under
[Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes). **Done 2026-11-11**: W80, five rows filed and
fixed; refused at the parse, and at load where a macro or a parameter writes it ([the records](#t1-record-2026-11-11)).
**Depends on:** nothing. It may edit `ein.y`'s `FactHead` and `ConclusionHead`, so it runs after
[S1f.7.6](s1f.7.6_a_false_fact_with_an_argument.md), not beside it.

## The finding

Recorded on the release binary at `f62e34e1`:

> **`(absent k)` written as a fact, or concluded, stores a fact no premise can read**: it loads, a fact of a
> relation `absent` no premise can read (`PremiseHead` takes no `absent`, which is negation as failure), as
> `:assert (absent (p ?x))` stores one.

So the program can state a fact that no rule can ever match. It is stored, printed and counted, so it can make two
states differ. S1f.5.23 called it the same case as `open`'s.

## The direction the rulings point to

S1f.5.23 (Q-M1f.66 (a)) refused `open` wherever it named a relation that no premise can read. The direction that
agrees is to refuse `absent` as a fact's, a nested fact's and a conclusion's head: `FactHead`, `ArgHead` and
`ConclusionHead` lose it, with rows in `00_ebnf.md`'s table. `absent` stays the negation-as-failure connective in a
premise.

The other direction would let a premise read a relation named `absent`. That clashes with the connective, so it is
a question for the user.

## Tasks

- **T1f.7.7.1, confirmation and the census.** Re-take the finding as a fact, a nested fact, a stored negation and a
  rule's, an hrule's and a macro's conclusion. Each is run on every command, with the states it can split.
- **T1f.7.7.2, the rows.** One witness per way the engine answers. Each is filed `open` and held to today's engine.
- **T1f.7.7.3, the refusal.** In the grammar, with the rows flipped and a test on every command.
- **T1f.7.7.4, the pages.** `01_grammar.md`, `02_patterns.md`, `00_ebnf.md` re-blessed, `06_reserved_names.md`,
  `defined_behaviour.md`, and the stage's records.

## Acceptance

- `absent` is written only as the connective of a premise, and every other place is refused on every command with
  one line.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.7 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.

## T1 record, 2026-11-11

**Confirmed**, on the release binary at `52e05fc1` (`ein saturate --dump` and `ein solve`). The class, with a
relation `p` and `(p a)`:

| the program writes | at `52e05fc1` |
|---|---|
| `(absent k)` | stored; `k` is a name of the program, so the blind enumerator's H grows: on `(relation paid Guest) (is-a Ann Guest)`, R is two states without the fact and four with `(absent Bob)` |
| `(p a) (absent (p a))` | both stored, root alive, R {∅}: the constraint the author meant by `(not (p a))` is ignored |
| `(q (absent k))`, `(not (absent k))` | stored |
| `:assert (absent (p ?x))` in a rule | `(absent (p a))` stored |
| `:assert (absent ?x)` in an hrule | a candidate `(absent a)`: R two states that differ in a fact nothing reads |
| `:assert (q (absent ?x))`, `:assert (not (absent ?x))` | `(q (absent a))`, `(not (absent a))` stored |
| a macro whose body is `(absent ?a)`, called in an `:assert` | `(absent a)` stored |
| `(mk absent)` beside `(rule mk (?R) :match (p ?x) :assert (?R ?x))` | `(absent a)` stored; `:assert (q (?R ?x))` stores `(q (absent a))`; a premise `(?R (q ?x))` is a pattern over a relation `absent` and matches nothing |
| a pattern's argument `(q (absent (q ?x)))` | loads, matches only a stored `(q (absent …))`: S1f.7.3's |
| a claim's `(model (absent (q a)) (p a))` | a class atom of the goal `(and (p ?y) (absent (q ?y)))`: this is right, and stays (Q-M1f.122) |

Every command that reads a program answered each: `saturate`, `solve` and its questions, `oracle`, `test`, the
`render` views, and `kb save`, whose container answers as its text. As a name `absent` is a value like any other,
`(h absent)`, and stays one.

## T2 record, 2026-11-11

Five witnesses under `examples/ein-bugs/wrong-answers/`, each with a corpus entry and a row of W80:
`absent-fact-for-not` (written where `(not …)` was meant, alive), `absent-fact-name` (H grows, R four states where
two), `absent-fact-nested` (a value), `absent-concluded` (a rule stores one) and `absent-hrule-concluded` (an hrule
splits R on one). Filed `open` with a `[case.region]` each and held to the unchanged engine: `ein-corpus`,
`ein-cli`'s `wrong_answers` and `ein-infer`'s `register_region` green on the sources at `52e05fc1`, with all
nineteen rows of this batch open, 264 fixed, 32 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml |
sort | uniq -c`, 2026-11-11).

Goldens named before blessing: `corpus_exits.txt` (two lines a witness), `corpus_shapes.md5` (the witnesses'
lines), `grammar_table.txt` (the stage's rows), `00_ebnf.md`'s §2 block (`FactHead`, `ValueHead`, `PremiseHead`,
`ConclusionHead`, `Fact`, `NotFact`, `NestedFact`, `EqFact`, `Premise`, `Negated`, `Arg`, `ArgHead`, `Pattern`,
`Conclusion`, `ClaimFact`, `ClaimAtom`), and the fixtures T3's record lists.

## T3 record, 2026-11-11

One fix for the four stages of this batch (S1f.7.7, S1f.7.6, S1f.7.10, S1f.7.3), in the grammar and the loader.

**The grammar** (`ein.y`): `ValueHead` is the one list of the relations a value may have, a fact inside a fact
(`NestedFact`), a pattern inside a pattern (`ArgHead`), a conclusion's or a guard's argument (`Pattern`), and it
holds no `absent`, `eq` or `false`. `FactHead` and `ConclusionHead` lose `absent`. A claim's class keeps `(absent
<fact>)` as `ClaimAtom` (Q-M1f.122). An error found at an `absent` that opens a form carries the note *absent is
negation as failure, a test a premise makes of a state, (absent <premise>) in a :match or a :goal, so no fact,
value or conclusion is of it: the stored negation of a fact is (not …)* (`ein_ir::grammar`'s `ABSENT_NOTE`).

**The loader** (`ein_ir::from_ir`): the `Shapes` walk refuses, after macro expansion, a conclusion headed `absent`
and an `(absent …)` where a value goes; and `refuse_eq_at_another_arity` runs once per word, `eq`, `absent` and
`false`, so a relation parameter an activator can bind to `absent` is refused for each form it heads (Q-M1f.126).

The lines, as printed on `examples/ein-bugs/wrong-answers/absent-fact-for-not.ein` and on the T1 probes:

```
examples/ein-bugs/wrong-answers/absent-fact-for-not.ein:27:2: unexpected `absent`
  note: absent is negation as failure, a test a premise makes of a state, (absent <premise>) in a :match or a :goal, so no fact, value or conclusion is of it: the stored negation of a fact is (not …)
kb load error: <file>:<l>:<c>: rule 'r': its :assert writes (absent ?x), and absent is negation as failure, a premise's test, and no fact is of it — a conclusion is a fact (relation argument …); the stored negation of a fact is (not …)
kb load error: rule 'mk': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (mk absent) binds ?R to absent — absent is negation as failure, a premise's test, and no fact is of it
```

W80's five rows flipped to `fixed`, with the fourteen of W81 to W83: 283 fixed, 13 open and 1 not wrong (`grep
'^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). `[case.fixed]` carries the null id in the
fixing commit and the hash in the next.

A sweep of the 622 `.ein` files tracked at `52e05fc1`, the binary there against the new one, `ein saturate
--dump`'s stored facts and exit and `ein solve`'s whole output (2026-11-11), moved no answer. Three files moved, each
refused before and after: `examples/broken/absent_arity_zero.ein` and one parser probe under `plans/` print two of
their ranked repairs in the other order, and another probe's nested `(neq a b)` gains the note. No zebra program
moved, `ein solve` printing the same bytes on each.

**Fixtures and tests moved, each named**: `examples/broken/absent_arity_zero.expected` (two repair lines in the
other order); `ein-ir`'s `fuzz_seeds.txt` (the same two lines of one seed) and `grammar_table.txt` (the thirty-two
new rows, the note on two rows that write `(false A)` in a claim, the repair order on two); `corpus_exits.txt`
(two cells a witness, thirty-eight) and `corpus_shapes.md5` (the nineteen witnesses' lines and
`absent_arity_zero`'s `ir[parse]`); `00_ebnf.md`'s §2 block, re-blessed. Two `from_ir` unit tests listed seven
shapes as loading that the rule refuses, `(absent (not (absent (p ?x))))`, `(p (eq k k))`, `(p (and (q ?x)))`, `(eq
?x (absent (p k)))`, a premise `(false ?x)`, `:assert (q (eq ?x k))` and `:assert (absent (p ?x))`, and two more,
a parameter heading a value under `(r eq)` and a nested `(box (eq a))`; each is held refused there since, the
last by the grammar's table. `dump_goldens`'s sanity bound on
refused corpus files is a third, where a quarter. No fixture's program was rewritten.

**The pin**: `ein-cli/tests/value_rule.rs`, fifty-five shapes and the nineteen witnesses on twenty commands,
nineteen programs that keep loading and one answered. Run against the binary at `52e05fc1`, two of its four tests
fail.

## T4 record, 2026-11-11

`defined_behaviour.md` § 4.22 (new, the four stages'; § 4.14 and § 4.17 restated where they said a nested `(eq
a)` and a written `(absent k)` load), `00_ebnf.md` (the blocks re-blessed, §1's fact 1 and §4), `01_grammar.md`
(§ Facts), `02_patterns.md`, `06_reserved_names.md` (`false` at arity 0, `absent` never a stored atom's head, `eq`
and `neq` no value), `01-ein-graph/03_ein_model.md` (no `(false <witness>)`), `examples/README.md`, `AGENTS.md`,
the phase README and this file. Gate green on the fixing tree.
