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

**No default is taken.** (c) undoes part of your ruling, (b) reopens a decision, (a) and (d) differ in whether a
contradiction is something every fact has or something a program asks for.
