# 74 · The defaults S1f.21.21 took

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.21_the_contradictions_cost.md#record) · [the cost page](../../../costs/README.md)

S1f.21.21 is done: what you ruled built of the four things recommended (Q-M1f.657). A rule of the pair's shape
is matched by one probe for each new fact, and the relation of a nested fact is a key of the indexes. **No firing
moved**: `ein solve -p` and the digest of the events are byte for byte the same on all 942 corpus files. These are
the six defaults it took.

## What you should know first

- **The pair's rule of no argument stays, and you have ruled it** (default 1, Q-M1f.658): *"this cost is
  absolutely acceptable"*. It comes from the membership fact every declaration stores, and still costs 0.5 to
  8.6 % of the instructions, as a plan held at every state and not as a match. You accepted the contradiction's cost that is left.
- **Two things built beyond the plan** (default 5): a pass no longer scans every plan for one never matched, and a
  resumed state no longer copies its engine's list of plans. On `equation_two_unknowns.ein` they saved more than
  the probe and the index together.
- **The set of negated facts is gone** (default 2), with the rows of the shape dump that printed it: the probe
  asks the fact store, which reads no name the kernel knows.
- **The largest cost at a state that is left is not the contradiction's**
  ([V-78](../../../findings.md#v-78--a-resumed-state-walks-every-fact-it-holds-to-find-what-is-new)): a resumed
  state walks every fact it holds to find what is new, about 17 % of `send_more_money.ein`. Not built, and
  not ruled: what you accepted is the contradiction's cost.

## Instructions executed

The stage's own counts, user space, one worker, pinned, each engine on the corpus of its own commit, in millions.
*Before* is the engine before the contradiction became rules; *after* the engine after the phase's builds;
*improved* the engine this stage leaves. The final table, with the times on an idle machine, is on
[the cost page](../../../costs/README.md).

| program | before | after | improved | improved / before |
|---|---:|---:|---:|---:|
| `equation_two_unknowns.ein` | 3 143 | 5 036 | 3 404 | 1.08× |
| `pets.ein` | 2 629 | 4 432 | 3 530 | 1.34× |
| `01_not_and_absent.ein` | 17 118 | 29 391 | 23 991 | 1.40× |
| `send_more_money.ein` | 10 665 | 19 841 | 11 511 | 1.08× |
| `zebra2.ein` | 1 117 | 1 342 | 1 117 | 1.00× |

## The six

| # | question | what was asked | the default taken | the other reading |
|---|---|---|---|---|
| 1 | [Q-M1f.658](../../../../open_questions.md#q-m1f658--the-pairs-rule-of-no-argument-for-a-relation-declared-with-labels) | `clash-0`'s instance for a relation declared with labels | stays, made free; measured, the three ways shown (§ 3) | one of the three ways |
| 2 | [Q-M1f.659](../../../../open_questions.md#q-m1f659--how-a-pairs-probe-asks-whether-the-state-holds-the-other-side) | how a pair's probe asks whether the state holds the other side | the fact store's own probe of `(W, [f])`, then `Kb::contains`; `Kb::is_negated`, its set and the dump's `NEG` rows go | a set of the facts a state holds under a wrapper, by wrapper relation: measured 0.1 to 0.4 % cheaper in the form specific to `not`, which reads a kernel name |
| 3 | [Q-M1f.660](../../../../open_questions.md#q-m1f660--which-rules-take-the-probe) | which plans take the probe | a plan whose whole match is one pair, one disjunct of two premises, and whose firing ends a state, by stating the key or concluding what a rule that does matches alone: `clash-N` and a program's own pair rule either way; a rule that states `:ein inconsistent` over anything but a pair, `std.core`'s, and a pair inside one of several disjuncts stay seeded | probe the pair's disjunct of a plan of several and seed the rest |
| 4 | [Q-M1f.661](../../../../open_questions.md#q-m1f661--the-key-of-a-nested-facts-relation) | the nested relation's key | `(context, argument, RELATION, relation)`, the argument part of the key; a plan keyed by its premise's first nested pattern; the probe last, after every argument that can be bound, and not listed after a constant | `(context, relation)` without the argument; or the relation's key before a later argument |
| 5 | [Q-M1f.662](../../../../open_questions.md#q-m1f662--what-a-rule-instance-costs-at-every-state-beside-the-two-tasks) | the costs of a plan at a state beside the two tasks | built: the watermark for the plan never matched, and the shared plan list | leave them for a stage of their own |
| 6 | [Q-M1f.663](../../../../open_questions.md#q-m1f663--a-position-inside-a-nested-fact-that-has-the-relation-keys-number) | a position inside a nested fact that is `RELATION`'s number (the last of a fact of 65 535 arguments) | not keyed, by the scan, the compiled list and the index alike (`SlotKey::keys_inner`), so a position's key and a relation's never share a bucket | a field of its own in `SlotKey`, past its 12 bytes |

## Your call

The first is ruled. A number and what you want otherwise for the other five, in [`ANSWERS.md`](../ANSWERS.md)
under item 74. Blank keeps them as built.
