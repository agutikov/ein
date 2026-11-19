# 35 · Q-M1f.557: how the contradiction's rules reach every relation and count

[← the topic](../README.md) · [← the phase](../../../README.md) ·
[the ledger](../../../../open_questions.md#q-m1f557--how-the-contradictions-rules-reach-every-relation-and-count)

**Part 1. Not ruled, and no default.** It is the one choice
[S1f.21.11](../../../s1f.21.11_kill_and_three_conventions.md) cannot take by itself: its task 2.1 waits on it.

**The question.** `not` is ruled a convention of the standard library: a fact beside its stored negation is to be
dead because rules say so, `std.contradiction`'s, and not because the kernel reads `not`. Those rules are written
one a count of arguments, 0 to 6, and switched on a relation at a time, by an activator that `clash-of-N` derives
from the relation's declaration. A relation is any atom first in a tuple, declared or not
([Q-M1f.522](../../../../../../docs/decisions/language/Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md)).
So once the kernel's own check goes, **a relation no file declares, and one of seven arguments or more, has no
contradiction**, where today every fact of every relation and count has one.

## What today's engine shows

The gate build of master at `ca98dec2` (`build/review-bin/ein-head-ca98dec2`), on the four programs of
[`programs/`](programs/); `tools/run.sh` re-takes them into `runs/` (both ignored). Each holds `(p a)` beside
`(not (p a))`, or the same of seven arguments, and asks for the rules by `(clash-of-declared)`.

| program | the relation | what the rules derive (`ein saturate --dump`) | `ein solve` today |
|---|---|---|---|
| [`reach-declared.ein`](programs/reach-declared.ein) | declared, one argument | `(clash-1 p)`, then `(false :rule clash-1)` | dead, `refuted by clash-1` |
| [`reach-undeclared.ein`](programs/reach-undeclared.ein) | not declared | **nothing** | dead, `refuted by (given)`: the kernel's check alone |
| [`reach-undeclared-by-hand.ein`](programs/reach-undeclared-by-hand.ein) | not declared, `(clash-1 p)` written | `(false :rule clash-1)` | dead, `refuted by clash-1` |
| [`reach-seven.ein`](programs/reach-seven.ein) | declared, seven arguments | no `(false)` | dead, `refuted by (given)`: the kernel's check alone |

The second and the fourth are dead today only by the kernel. With the kernel's check gone and nothing else
changed, both are alive: a held fact beside its negation, in $R$.

**How much of the corpus that is** is not counted yet: it needs a build with the check off, which is S1f.21.11's
first task. What is counted: at root, the corpus negates facts of 0, 1 and 2 arguments and none of more
(S1f.21.8's census), so the count of arguments bites no corpus program today, and the undeclared relation is the
case to count.

## Why not just one rule with the relation a variable

Your question, 2026-11-19, verbatim: *"why not just (rule () :match (and (?R ?a) (not (?R ?a))) :assert (false) ) ?"*

**Because today's loader refuses it; and it is a good answer if that refusal is lifted for this shape.**

[`reach-free-relation.ein`](programs/reach-free-relation.ein) is the rule as you wrote it, with a name and a
`:why`, beside `(p a)` and `(not (p a))`; [`reach-free-relation-not-first.ein`](programs/reach-free-relation-not-first.ein)
is the same with the stored negation written first. Both are refused the same way:

```text
kb load error: rule 'clash-1': its :match writes (?R ?a) with ?R as its relation, which no parameter binds — a relation is named by a name or by a parameter its activator binds
```

In Ein today a variable may stand first in a pattern only where it is a **parameter** of the rule, which an
activator fills before anything is matched: `(rule clash-1 (?R) …)` with `(clash-1 p)`. That is why the standard
library's rule set needs an activator a relation, and why an undeclared relation is not reached: nothing writes
its activator. Two things stand behind the refusal:

- **The matcher.** A plan is compiled a premise at a time from the premise's relation: the relation says which
  held extent to walk and which index to use. A premise whose relation is a variable is one of the *shapes no plan
  can be compiled for*
  ([Q-M1f.72](../../../../../../docs/decisions/diagnostics/Q-M1f.72-a-nested-or-a-wide-match-and-a-goals-relation.md)).
- **The analyses.** The strata, the monotonicity check, *forbid* and the invariant that a state names no relation
  the program lacks each ask which relations a rule reads and concludes, and read them off the rule's text or off
  the activator that fills a parameter
  ([Q-M1f.393](../../../../../../docs/decisions/language/Q-M1f.393-a-rule-unpacks-a-fact-by-its-count.md)).

**Neither bites this rule hard.** Its two premises share the variable, and the one under `not` binds it: walk the
stored negations, take the fact each denies, read its relation and its argument off it, and ask whether that fact
is held. That is the kernel's own check today, line for line (`contradiction::detect`), so the plan exists; what
is missing is the loader letting a relation be a variable **that another premise of the same match binds**, and
the compiler ordering the two premises so. And for the analyses, *this rule reads every relation of one argument*
is simply true of a contradiction: it reads them positively and concludes only `(false)`, so the monotonicity
check has nothing to flag, and *forbid* refuses nothing since your ruling of 2026-11-19 (Q-M1f.564).

**What it would take, and what it leaves.**

- A change to the language: a relation may be a variable of the pattern where another premise of the match binds
  it; the loader's refusal stays for a relation variable nothing binds. One row of the grammar page's table and of
  the diagnostics; the matcher reads the relation at match time for such a premise; each analysis reads such a
  premise as reading every relation of its count.
- **No activator, no declaration, no fact the loader makes**: the rule reaches every relation a program writes,
  declared or not. `clash-of-N`, `(clash-of-declared)` and the activators go.
- **It is still one rule a count of arguments.** The pattern spells its arguments, `(?R ?a)`, `(?R ?a ?b)`, …, so
  a relation of more arguments than the rules are written for has no contradiction. The corpus negates facts of 0,
  1 and 2 arguments at root and nothing wider, and its widest declared relation has 10 labels.

It is option (e) below. Of the five it is the one that takes nothing from the ruling on `not`, makes the loader
write nothing, and reaches the undeclared relation.

## Where the refusal comes from

Your question, 2026-11-19, verbatim: *"explain (in readme) where this bahaviour came from? what feature, what
decision, maybe what were the reasons to refuse this at load. Because now I don't see why we must have it."*

**Nobody decided that a relation may not be a variable of a pattern. The refusal follows from how a rule is run,
and it was put at load to end a silent wrong answer.** In order:

| when | what | where it is |
|---|---|---|
| the engine's design | **A rule is compiled, once per activator.** The activator's arguments are written into the plan before any match, so a premise's relation is a constant of its plan: it says which held extent to walk and which index to use. The one variable a relation can be is a parameter, which is a name by the time the plan exists. The reason given: *the plan is a pure function of that binding; a matcher that walked the rule's tree would rediscover the binding at every match* | [Q29](../../../../../../docs/decisions/engine/Q29-a-rule-compiles-once-per-activator.md); `ein-infer/src/plan.rs`, a pattern's `rel` is a `Symbol` |
| 2026-08-20 | The engine's fuzzer writes `(?R ?x)` with `?R` no parameter. The compiler had no plan for it and rejected the rule, *unbound relation head ?R* | [`unbound-relation-head.ein`](../../../../../../examples/ein-bugs/unbound-relation-head.ein) |
| 2026-11-05 | **The same shape inside a pattern, `(box (?r b))`, was not rejected.** The compiler left the inner pattern as a slot that unifies with nothing, so the program loaded, the rule never fired, and nothing said so: a wrong answer, *a rule shape that reads nothing, in silence*. Fixed that day on your ruling: such shapes are refused at load, each with a line in Ein's words, today's line among them (`3ab9a231`) | [`nested-unbound-head.ein`](../../../../../../examples/ein-bugs/wrong-answers/nested-unbound-head.ein); its row of [`corpus/regressions.toml`](../../../../../../corpus/regressions.toml); [Q-M1e2.18](../../../../../../docs/decisions/language/Q-M1e2.18-a-keyword-pair-is-no-guards-argument-and-a-fact.md), the ruling's page for the sibling shapes: *each shape loads and matches nothing in silence otherwise* |
| 2026-11-10 | The compiler's own rejection of the top-level shape, and of a goal's, become the loader's lines too, in the same spelling: *shapes no plan can be compiled for* | [Q-M1f.72](../../../../../../docs/decisions/diagnostics/Q-M1f.72-a-nested-or-a-wide-match-and-a-goals-relation.md), [Q-M1f.121](../../../../../../docs/decisions/diagnostics/Q-M1f.121-a-goals-relation-variable-is-refused-with.md) |
| 2026-11-17 | Nested facts. *A rule unpacks a fact by its count*: the count is fixed by the text and **the relation may be a parameter**. It is the first page that gives a reason of the language's and not of the engine's: *every reading of a rule set asks which relations a rule reads and which it concludes … each reads the relation off the text, or off the activator that fills a parameter … a premise of no fixed relation has no held extent to be matched in* | [Q-M1f.393](../../../../../../docs/decisions/language/Q-M1f.393-a-rule-unpacks-a-fact-by-its-count.md) |

So the feature is **the activator**: Ein's one way to write a rule over many relations is a parameter that a fact
headed by the rule's name fills, and everything the engine does with a rule starts after that filling. The refusal
is the loader saying, in place of the matcher saying nothing, that a relation it would have to find *during* a
match is something the engine does not do.

**The reasons, and what each is worth against your rule.**

| the reason given | where | against `(and (?R ?a) (not (?R ?a)))` |
|---|---|---|
| no plan can be compiled for it | Q29, Q-M1f.72 | true of the engine as built, and a property of the engine, not of the language. For this rule the plan is short: under `not` the pattern meets a fact, and a fact carries its relation, so `?R` and `?a` are read off it; the other premise is then one whole fact, asked for by membership. It is the loop `contradiction::detect` runs today |
| otherwise it matches nothing in silence | Q-M1e2.18 | the reason for refusing **at load** rather than letting it load. It is a reason to keep refusing what cannot be run, and none against running what can |
| the analyses read a rule's relations off its text or its activator | Q-M1f.393 | such a premise reads *every relation of its count*, which each analysis can take as it stands: the strata and the monotonicity check see a positive read of everything, and this rule concludes only `(false)`; *forbid* refuses nothing since Q-M1f.564. It costs precision only for a rule that concludes facts from such a premise. A **conclusion** whose relation is a variable of the match is another matter, since it could write any relation, an activator among them ([Q-M1f.42](../../../../../../docs/decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)), and nothing here asks for it |
| a premise of no fixed relation has no held extent to be matched in | Q-M1f.393 | true of such a premise **alone**: `(?R ?a)` with nothing else would walk every fact of one argument. Not true once another premise has bound the relation |

**So you are right: nothing says we must have it.** What has to stay is the thing the refusal was made for, no
rule that loads and silently reads nothing. The smallest change that keeps it:

- in a `:match`, a relation may be a variable **that another premise of the match binds**, by standing first in a
  pattern under an arrow of that premise, as `?R` does under `not`; the compiler orders the premises so, whatever
  order they are written in;
- a relation variable nothing binds stays refused, with today's line, and so does one in a conclusion.

That is option (e) below, said as a change to the language.

## And a quantifier over the relation

Your question, 2026-11-19, verbatim: *"foreach ?R also wouldn't work, right?"*

**Right, it would not.** There is no `foreach` in Ein; the nearest things are a premise that takes the relation
from a fact that lists it, and `std.macro`'s `forall`. Both are refused with the same line, since each still writes
`?R` first in a pattern, and the loader asks only whether `?R` is a parameter:

| program | how the relation is reached | `ein solve` |
|---|---|---|
| [`reach-relation-from-a-fact.ein`](programs/reach-relation-from-a-fact.ein) | `(and (relation ?R ?l) (?R ?a) (not (?R ?a)))`: `?R` bound by a declaration, then written first | refused: *its :match writes (?R ?a) with ?R as its relation, which no parameter binds* |
| [`reach-forall-relation.ein`](programs/reach-forall-relation.ein) | `(forall ?R (relation ?R ?l) (absent (and (?R ?a) (not (?R ?a)))))` | refused, the same line |

`forall` is no loop either: it is the macro `(absent (and G (absent B)))`, *nothing satisfies the guard and not the
body*, one premise of one rule.

**What Ein has for *each relation* today is a rule that derives an activator**: `clash-of-1` matches a declaration
`(relation ?R ?l)` and concludes `(clash-1 ?R)`, and `clash-1` is then compiled once for that `?R`. That is the
loop, and it shows where the gap is: it runs over what some fact lists, and nothing lists a relation no file
declares. So a quantifier over declarations, were it allowed, would reach exactly what `clash-of-N` reaches and no
more. Only a relation read off the denied fact itself, option (e), reaches every relation, since every stored
negation carries the relation of what it denies.

## Or it is no gap: the contradiction as the standard library's feature

Your remark, 2026-11-19, verbatim: *"Mybe this is not a bug... Maybe it's a feature of stdlib. e.g. negation elimination and contradiction introduction works only if we import std.contradiction and only for declared relations. Again return to the nature of Ein - it is a graph. Do graphs have true or false nodes? or do they have positive or negative? No, here we can separate kernel engine machinery from semantic."*

**It holds together, and it is option (d) said as what it is.** A knowledge base is a graph: `(not F)` is a fact
that points at `F`, as `(think Ann F)` is. Nothing in a graph is true, false, positive or negative; that a fact and
its negation may not stand together is a statement of a logic, and a logic is a theory a program takes. The kernel
then knows two things only: a rule may state `:ein inconsistent`, and what a rule matches. The page on contexts
already drew the table this gives ([`open_facts.md` § 10.6](../../../open_facts.md#106-not-as-a-context)): of one
fact, *asserted*, *negated*, *open* or *contradicted*, the fourth a state a program may live in if it does not take
the classical theory.

**What it comes to, concretely.**

- **The contradiction is `std.contradiction`'s**, on for the relations a program declares, by a rule that derives
  each relation's activator from its declaration, which is how the module is built today (`clash-of-N`). A relation
  no file declares has none, and a program that wants one writes its activator. The elimination of a double
  negation is `std.negation`'s, imported the same way
  ([item 38](../38-Q-M1f.559/README.md), its default).
- **`std.core` stays one rule**, from `(false)` to the key. No rule of `not` is every program's.
- **Nothing is added to the language**: no relation variable, no activator made at load. Option (e) is not needed.
- **A program that does not import the module may hold a fact beside its negation and be alive.**

**What it costs, counted on the corpus as it is** (`grep` over `examples/` and `tests/`, 893 files, at `946ee788`):

- three modules of the standard library conclude a stored negation, `std.algebra` (3 rules), `std.bijection` (2)
  and `std.slots` (9); each would import `std.contradiction`, and a program that imports one of them, 107 files,
  gets it with them, an import bringing its module whole;
- 123 files write a `(not (…))` themselves, and **65 of them import nothing from the standard library**: those are
  the programs whose answer can move, each needing the import, or the activator where its relation is not declared.

How many do move is not counted by a `grep`: it needs a build with the kernel's check off, which is S1f.21.11's
first task.

**Three readers of `not` that are not the death, and what each becomes.** The kernel reads a stored negation for
more than killing, and each of these is sound only where the contradiction is on:

- **a candidate root denies is no candidate.** Without the kernel's check, committing such a candidate gives a state
  that holds both, alive unless the rule is on for its relation. So the filter either goes, and $H$ grows by what
  root denies, or stays for the relations the rule is on for;
- **what a state denies is dead above it** (licence L8): the same;
- **the agenda's first half, *forbid* and the static check** read the rules that lead to a death: with the
  contradiction a rule, they read it as any rule; *forbid* refuses nothing since your ruling of 2026-11-19
  (Q-M1f.564), so what is left of it is the warning's reading of the same rules.

**One thing to say with it**: whether importing the module is the switch, or the fact `(clash-of-declared)` beside
the import, as built. Under *works only if we import*, the import is the switch and the fact goes; an import is
whole, so a program that imports a module that imports it has it on too.

## The options

- **(a) An activator for every head, made at load.** The loader knows every atom a program writes first in a
  tuple and at which counts; it writes what switches the rule on for each. Every relation is reached, at every
  count up to the rules'. It costs the loader a word of the module's to write, and one fact a relation and count in
  every program.
- **(b) One rule for every fact**: a premise that is a variable for a whole fact, `(and ?f (not ?f))`. No
  activator, no count. [Q-M1f.393](../../../../../../docs/decisions/language/Q-M1f.393-a-rule-unpacks-a-fact-by-its-count.md)
  refuses such a premise today, for a reason every analysis of a rule set rests on: each reads which relations a
  rule reads off its text.
- **(c) The kernel keeps its check, as its way of running the module's rule.** Nothing moves and nothing is
  missed. It leaves the kernel reading `not`, which the ruling took from it.
- **(d) A relation is checked where a program asks.** The rules reach what is declared, or activated by hand, and
  the rest has no contradiction: a program that wants one of an undeclared relation writes its activator. The
  simplest; it changes what some programs mean, and how many is the count above.
- **(e) One rule a count, the relation a variable of the pattern**, your rule:
  `(and (?R ?a) (not (?R ?a)))` ⟹ `(false)`, and its siblings of 0 and of 2 to 6 arguments. Refused at load today;
  it asks the loader to admit a relation variable that another premise of the match binds
  ([above](#why-not-just-one-rule-with-the-relation-a-variable)). Every relation is reached at every count the
  rules are written for, with no activator and nothing made at load.

**No default was taken when the item was put.** With (e) in it I proposed (e), the smallest change that reaches
every relation. **Since your remark the proposal is (d)**, read as you put it: reaching every relation is not the
aim; the contradiction is a feature a program takes by importing the module, for the relations it declares. It adds
nothing to the language and takes the last reading of `not` out of the kernel. (c) undoes part of your ruling, (b)
and (e) change the language for a reach that is then not wanted, and (a) makes the loader write a module's word.
