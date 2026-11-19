# 38 · Q-M1f.559: the elimination of a double negation, and the module every program gets

**Option (c) is ruled, 2026-11-19, as a matter of syntax**: you, on reading why the program is refused: *":hrules argument looks bad. How do we activate rules normally? (rule R (?args) ....)  then somewhere (R args) right? So why :hrules look different? it has to be (solve :hrules ((guess chosen Drink) (guess served Drink)))"*
The clause is to list each activator whole ([Q-M1f.565](../../../../open_questions.md#q-m1f565--the-hrules-clause-lists-activators-as-a-rules-are-written),
S1f.21.16, planned). Each hrule then heads its own form and is no argument, so the four corpus programs load beside
`std.negation`. **What is left of this item**: whether the elimination goes to every program or stays a module a
program imports, its default; and option (d), whether anything is refused at load for what a rule could derive.

[← the topic](../README.md) · [← the phase](../../../README.md) ·
[the ledger](../../../../open_questions.md#q-m1f559--the-elimination-and-the-module-every-program-gets) ·
[V-12](../../../findings.md#v-12--the-elimination-cannot-be-given-to-every-program-as-it-stands)

**Part 1. Not ruled; a default is proposed.** It is nothing about `std.core` as S1f.21.10 builds it, with its one
rule; it is what [S1f.21.11](../../../s1f.21.11_kill_and_three_conventions.md) would add to it.

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
goes ([Q-M1f.42](../../../../../../docs/decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)).
It does not ask whether anything activates `eliminate-2` so, and here nothing can. The second `guess` of the list
counts as *a rule named as an argument*, the first does not, which is the difference between the second row and
the third ([V-13](../../../findings.md#v-13--the-check-on-a-parameters-conclusion-counts-an-hrule-from-its-second-naming)).

Four corpus programs meet it, `examples/lattice/02`, `03`, `15` and `16`. They import nothing today, so the
elimination in a module every program gets would refuse them.

## Why it is refused: where the check comes from

Your question, 2026-11-19, verbatim: *"Ok, so WHY in Q-M1f.559 …/c-neg-twice.ein refused at load ?"*

**Not for anything the program does.** Nothing in it activates `eliminate-2`, so that rule can never fire. It is
refused for what a static check cannot rule out, and the check is this coarse by a default, not by a ruling.

**The thing the check exists to prevent.** A rule's parameter that stands first in a pattern names a relation, and
an activator fills it. Rules can derive activators. So a rule could derive an activator that puts a **fact** where
a relation's name goes, `(guess (f a) T)`: a pattern whose relation is a fact, which no plan can be compiled for.

**How that reaches this program**, step by step:

1. `eliminate-2 (?R)` concludes `(?R ?a ?b)`: a fact of two arguments of whatever relation its activator names;
2. `guess (?R ?T)` is a rule of two parameters, so a fact `(guess x y)` would be an activator of `guess`, `x` in the
   place of its relation;
3. were `eliminate-2` activated with `guess`, it would conclude `(guess ?a ?b)` with `?a` taken out of a double
   negation, where it can be a fact;
4. the check does not ask whether anything activates `eliminate-2` with `guess`. It reads the conclusion as able to
   write into **every rule of that count whose name the program writes as an argument**;
5. and `guess` counts as written as an argument, because it stands a second time inside the `:hrules` list, which
   is read as one form headed by the first `guess`. Named once, it does not count, and the program loads.

**Where each step was decided:**

| when | the question | what was decided, and by whom |
|---|---|---|
| 2026-11-08 | An activator a rule **derives** that binds a relation parameter to a fact. Until then the run stopped where such a plan was compiled, and a program that never derived one answered. Is the shape refused at load, or only where it happens? | **You ruled (b), refused at load by a static check**, which refuses a program that never derives it too (*"Q1 - b"*). The agent had recommended (a), refused where compiled. [Q-M1f.1](../../../../../../docs/decisions/language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md) |
| 2026-11-09 | What *could* means: which positions can hold a fact | **A default**, under your instruction of that day to go on with defaults: an argument of a declared relation can hold a fact; a marker's argument holds what the program does with it. [Q-M1f.41](../../../../../../docs/decisions/language/Q-M1f.41-a-position-holds-a-fact-unless-the-program.md) |
| 2026-11-09 | A conclusion whose relation is a parameter, `(?T …)`: which rules can it write into? | **A default**, the same way: (a), every rule of that count whose name the program writes as an argument, in a fact, a rule's clause or the `(solve …)` form. Beside it stood (c), *followed through the relation names each parameter can hold*, called then *the precise form of (a)*, and not taken because it moved nothing on the corpus of that day. [Q-M1f.42](../../../../../../docs/decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md) |
| 2026-11-19 | `std.negation` is written, the first rule of the standard library whose conclusion is `(?R …)` over arguments that can be facts | the coarse default now refuses four corpus programs that import nothing; found by S1f.21.8 |

So step 4 is the default of Q-M1f.42 and step 5 an accident of how the `:hrules` list is read
([V-13](../../../findings.md#v-13--the-check-on-a-parameters-conclusion-counts-an-hrule-from-its-second-naming));
only that such a thing is refused **at load, even where it can never happen**, is a ruling, yours of 2026-11-08.

**What would make it load**, from the smallest change up:

- the `:hrules` list read as a list, so a rule named there is no argument: option (c) below;
- the precise reading of Q-M1f.42, its own option (c): `eliminate-2`'s `?R` holds only the names its activators
  give it, none of which is a rule, so nothing is refused here: option (b) below;
- the ruling of 2026-11-08 the other way, its option (a) or (c): nothing is refused at load for what a rule *could*
  derive; a run that does derive such an activator stops there, with the compiler's line, as it did before. That
  is a fourth option, (d) below, and the one in line with what you ruled today of *forbid*: a program loads, and is
  judged by what happens.


- **(a) The elimination stays a module a program imports** (`std.negation`, as built), and what every program gets
  of `not` is the contradiction alone. **Proposed default**: no program changes, and a program that reasons with
  double negations says so by its import.
- **(b) The load check reads what an activator can name.** It no longer refuses a conclusion nothing can aim at a
  rule. The check gets finer, and the elimination can go to every program. It touches a check two decisions rest
  on (Q-M1f.1, Q-M1f.42).
- **(c) The `:hrules` list is read as a list**, so a rule named there twice is no argument. It frees these four
  programs, and not a program that names a rule as an argument anywhere else; the elimination then goes to every
  program with that left open.
- **(d) Nothing is refused at load for what a rule could derive** (Q-M1f.1 the other way). A program loads; where a
  run does derive an activator that puts a fact in a relation's place, the run stops there with the compiler's
  line, or goes on with a warning and no plan for that activator. Every program the check refuses today loads.
