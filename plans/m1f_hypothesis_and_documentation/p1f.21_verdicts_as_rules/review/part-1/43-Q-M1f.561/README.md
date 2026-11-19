# 43 · Q-M1f.561: one way to say *must* and one to say *may* — are hypothesis rules removed?

[← the part](../README.md) · [← the phase](../../../README.md) ·
[the ledger](../../../../open_questions.md#q-m1f561--one-way-to-say-must-and-one-to-say-may-hypothesis-rules-removed) ·
**[the analysis](../../../one_way_each.md)**

**Part 1. Not ruled.** Raised 2026-11-19 by you: *"analyze semantic of hrules and obligation rules, I think they are
100% equivalent, so we can remove hrules completely … And replace them with obligation rules. Can we? … Let's leave
only one valid way for each. Obligations is positive way, what would be the negative (filtering) one?"*

## What the analysis found

The page is [`one_way_each.md`](../../../one_way_each.md); this is its result.

- **Not equivalent, by one thing.** As a way to build $H$ an obligation is a hypothesis rule. As a judge it adds
  *at least one of these must hold*; a hypothesis rule requires nothing. *Must* against *may*.
- **So the rules can go, but not all into obligations.** *Must* is the obligation's. *May* is said the other way:
  nothing proposes, and every unsettled fact of a relation that is not taken out is a candidate.
- **The filtering way's one mechanism is the closure**, the fact `(__closed__ R)`. The two lists of `(solve …)`
  are closures written twice more.

## The six programs

The gate build of master at `ca98dec2`, on [`programs/`](programs/); `tools/run.sh` re-takes them into `runs/` (both
ignored), `ein show hyps`, `ein solve` and `ein oracle` of each. Two things `a` and `b`, an atom `c` that is no
thing.

| | program | $H$ | $R$ |
|---|---|---|---|
| 1 | [`1-may-hrule.ein`](programs/1-may-hrule.ein): a hypothesis rule | `(s a)`, `(s b)` | 4 states, `∅` among them |
| 2 | [`2-must-one.ein`](programs/2-must-one.ein): an obligation, `s` of some thing is owed | the same | 3 states; `∅` is out, root owes |
| 3 | [`3-must-each.ein`](programs/3-must-each.ein): each thing owes its own `s` | | refused: an obligation with no choice *is a plain `absent` check* |
| 4 | [`4-may-blind.ein`](programs/4-may-blind.ein): nothing proposes | and `(s c)` | 8 states: every guessable atom fills the place |
| 5 | [`5-may-blind-denied.ein`](programs/5-may-blind-denied.ein): the same, and a rule denies `s` of what is no thing | `(s a)`, `(s b)` | 4 states, program 1's |
| 6 | [`6-may-as-must-decide.ein`](programs/6-may-as-must-decide.ein): each thing owes a decision | four decisions | 4 states, none partial |

## The corpus

113 programs declare a hypothesis rule, 80 are answered. Run with every hypothesis rule deleted:

- **21 have an obligation that owes at root**: 12 answer with the same $R$ (`zebra2`, `send_more_money`, the
  equations: the rule is redundant there); `zebra.ein` keeps its one state over a universe twice the size; 7 change,
  the rule proposing facts no debt asks for; 1 is refused.
- **59 owe nothing at root**, the rule their only *may*: 23 answer the same with the rule deleted, or deleted and a
  list of relations added; 25 need what the rule's pattern kept out denied or filtered; the rest have nothing
  proposed without it.

And every program that states a list of `(solve …)`, with the list replaced by closure facts: 80 of 81 for
`:no-hypothesis`, 19 of 19 for `:hypothesis-relations`, the same $H$ and $R$.

## The options

Three calls, each with what is proposed.

**A. The hypothesis rules.**

- **(a) Removed**: the declarator, the clause, the first rung. *Must* is an obligation; *may* is the filtering way.
  66 corpus programs that say *may* with a rule are rewritten, each against the shape it was written to have, and
  the guide's Zebra program loses its rule. **Proposed**, if one way each is wanted: it is what your sentence asks,
  and the count says what it costs.
- **(b) Kept**, as the one way to say *may* of facts picked by a pattern.
- **(c) Left for the second part**, where a candidate may become a fact a rule concludes (Q-M1f.549), which is a
  hypothesis rule again with no declarator.

**B. The filtering way's mechanism.**

- **(a) The closure alone**: `(__closed__ R)`, stated or derived; the two lists of `(solve …)` are removed and
  rewritten as closures; a single fact is taken out by a rule that denies it. **Proposed.**
- **(b) The two lists stay** beside the closure, as now.

**C. *May* beside *must* in one program** (8 corpus programs today, by a hypothesis rule beside an obligation).

- **(a) One way a program**: under an activated obligation $H$ is what the debts ask for, as today; such a program
  writes its *may* as a decision that is owed (program 6). **Proposed**: it keeps the reason the ladder takes one
  rung (Q-M1f.243).
- **(b) The two ways add up**: $H$ is the debts' payments and every unsettled fact of a relation that is not closed.

**What waits on it.** Nothing of S1f.21.10 or S1f.21.11. A ruling for A (a) or B (a) is a stage of its own in
part 1, planned when it is given.
