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
check has nothing to flag, and what *forbid* makes of it is item [37](../37-Q-M1f.558/README.md) either way.

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

**No default was taken when the item was put. With (e) in it, (e) is what I would propose**: it is the smallest
change that leaves the kernel reading no `not`, and its one cost is a shape the loader refuses today and the
matcher already runs for the kernel's own check. (c) undoes part of your ruling, (b) reopens a decision for more
than is needed, (a) makes the loader write a module's word, and (d) changes what programs with an undeclared
relation mean.
