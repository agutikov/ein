# 38 · Q-M1f.559: the elimination of a double negation, and the module every program gets

[← the topic](../README.md) · [← the phase](../../README.md) ·
[the ledger](../../../open_questions.md#q-m1f559--the-elimination-and-the-module-every-program-gets) ·
[V-12](../../findings.md#v-12--the-elimination-cannot-be-given-to-every-program-as-it-stands)

**Part 1. Not ruled; a default is proposed.** It is nothing about `std.core` as S1f.21.10 builds it, with its one
rule; it is what [S1f.21.11](../../s1f.21.11_kill_and_three_conventions.md) would add to it.

**The question.** The meaning of `not` is two rules, the contradiction and the elimination, `(not (not g))` ⟹ `g`.
S1f.21.11 planned both in the module every program gets. The contradiction's rules load beside any program. The
elimination's are refused beside some programs that never use them.

## What today's engine shows

The gate build of master at `ca98dec2`, on one program in four forms, [`programs/`](programs/); `tools/run.sh`
re-takes them into `runs/` (both ignored). The program has an hrule of two parameters, `guess (?R ?T)`.

| program | what differs | `ein solve` |
|---|---|---|
| [`a-plain.ein`](programs/a-plain.ein) | no import; `guess` named twice in `(solve :hrules …)` | 16 states |
| [`b-neg-once.ein`](programs/b-neg-once.ein) | `(import std.negation)`; `guess` named once | 4 states |
| [`c-neg-twice.ein`](programs/c-neg-twice.ein) | `(import std.negation)`; `guess` named twice | **refused at load** |
| [`d-con-twice.ein`](programs/d-con-twice.ein) | `(import std.contradiction)`; `guess` named twice | 16 states |

The refusal: *rule 'std.negation.eliminate-2': its :assert writes (?R ?a ?b), whose ?a is rule 'guess''s parameter
?R, which names a relation, and (not (not (?R ?a ?b))) can bind ?a to a fact … a relation is named by a symbol, not
a fact*. `eliminate-2` concludes a fact of two arguments of whatever relation its activator names; `guess` has two
parameters, so the loader reads that conclusion as able to activate `guess` with a fact where a relation's name
goes ([Q-M1f.42](../../../../../docs/decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)).
It does not ask whether anything activates `eliminate-2` so, and here nothing can. The second `guess` of the list
counts as *a rule named as an argument*, the first does not, which is the difference between the second row and
the third ([V-13](../../findings.md#v-13--the-check-on-a-parameters-conclusion-counts-an-hrule-from-its-second-naming)).

Four corpus programs meet it, `examples/lattice/02`, `03`, `15` and `16`. They import nothing today, so the
elimination in a module every program gets would refuse them.

## The options

- **(a) The elimination stays a module a program imports** (`std.negation`, as built), and what every program gets
  of `not` is the contradiction alone. **Proposed default**: no program changes, and a program that reasons with
  double negations says so by its import.
- **(b) The load check reads what an activator can name.** It no longer refuses a conclusion nothing can aim at a
  rule. The check gets finer, and the elimination can go to every program. It touches a check two decisions rest
  on (Q-M1f.1, Q-M1f.42).
- **(c) The `:hrules` list is read as a list**, so a rule named there twice is no argument. It frees these four
  programs, and not a program that names a rule as an argument anywhere else; the elimination then goes to every
  program with that left open.
