# 36 · Q-M1f.447: the argument-count check, once `relation` is a convention

[← the topic](../README.md) · [← the phase](../../README.md) ·
[the ledger](../../../open_questions.md#q-m1f447--the-loaders-argument-count-check-once-relation-is-a-convention)

**Part 1. Not ruled, and no default** (yours: *"argument-count check - will think about it in p1f.21, record as open
question"*). Only the last build step of [S1f.21.11](../../s1f.21.11_kill_and_three_conventions.md) waits on it,
its task 2.4.

**The question.** `relation` is ruled a convention of the standard library: a declaration is an optional fact a
module's rules may read. The loader still reads it for one thing of its own: a form headed by a declared relation
must have as many arguments as the declaration has labels. That is the loader reading a theory's form.

## What today's engine does

The gate build of master at `ca98dec2`, on the three programs of [`programs/`](programs/); `tools/run.sh` re-takes
them into `runs/` (both ignored).

| program | what it writes | `ein solve` |
|---|---|---|
| [`count-declared-other.ein`](programs/count-declared-other.ein) | `(relation owns Person Pet)`, and the fact `(owns Bob)` | refused at load: *the fact (owns Bob) gives owns 1 argument, and (relation owns Person Pet) declares 2* |
| [`count-rule-other.ein`](programs/count-rule-other.ein) | the same declaration, and a rule that concludes `(owns ?p)` | refused at load: *rule 'everyone-owns': its :assert writes (owns ?p), which gives owns 1 argument …* |
| [`count-undeclared-two.ein`](programs/count-undeclared-two.ein) | `(likes Ann)` and `(likes Ann Rex)`, nothing declared | loads; each query answers its own count |

So the check holds only what is declared, and a program that wants a relation at two counts leaves it undeclared,
which the standing rules already say.

## The options

- **(a) Kept at load**, as the loader's reading of a declaration. Nothing moves. `relation` is then a convention
  with one reader in the kernel left.
- **(b) A rule's verdict at run time**, in the module that states the convention: a fact of a declared relation at
  another count is a contradiction the module's rule concludes. The kernel reads nothing; a miscount is found when
  a state holds it and no longer when the file loads, and it kills the state where it refused the program. It
  cannot be run today: the loader refuses the program before a rule could judge it.
- **(c) Dropped.** Ein has no types and a label means what a rule makes it mean
  ([Q-M1f.372](../../../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)),
  so a relation may be written at any count. A slip of the hand in a fact, one argument short, then loads in
  silence.

**No default is taken.** S1f.21.11's examples show each on the corpus programs the check refuses today and on
those that write a relation at two counts, before it asks.
