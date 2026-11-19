# 62 · What a fork drops, on the seats cube, node by node

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[your answers](../ANSWERS.md)

**What it shows.** Task 1.5 of the stage. You ruled that a fork drops every `open` fact and that they are made again
at the fixpoint (Q-M1f.566), every rule that concludes `(open …)` being read at every state (Q-M1f.569). Here, at
each of the 16 nodes of the seats program's cube, the `open` facts the pass would write there, and which of root's a
fork drops for good.

**The answer, short.** Root holds `(open seats)` and its four candidates. **Every node that owes holds `(open
seats)`** (root, the four of layer 1, the four of layer 2 that are dead and owe one), with the facts that would pay
what it owes there; **neither state of $R$ holds an `open` fact**, nor does any of the five nodes above them, which
owe nothing. So a fork from root drops the five and each node writes its own: at a node of layer 1 four of the five
come back and one does not, the candidate the node supposes.

## 1. How it was taken

The program is [`examples/lattice/05_owed_seats_bounded.ein`](../../../../../../examples/lattice/05_owed_seats_bounded.ein):
Ann and Bob, chairs C1 and C2, `total-owed` and `surjective-owed` on `seats`, and `functional` and `injective`. Its
cube is `ein oracle --dump --states` (`runs/oracle/region.json`): 16 nodes, two of them $R$.

Today's engine, `build/review-bin/ein-head-edbe6c3b`, with the checkout's `stdlib/`. `tools/run.sh` re-takes every
run into `runs/` (ignored by git; mirrored under `build/review-incoming/p1f21-s12-examples/62-what-a-fork-drops/`);
`runs/nodes.md` is the table below as the tool wrote it, with a column more, and `runs/nodes/` has every node's
programs and runs.

**Each node is made a root.** Its commitment is written into the program as facts, so the node's fixpoint is
reached afresh, as the pass reaches it once root's `open` facts are dropped. Two readings of it, which agree at every
node:

- **the kernel's own**: `ein show hyps` lists the candidates the obligations rung reads off the guard at that
  state, and `ein solve --monotone --json-summary` what it owes there;
- **a stand-in**, [`seats-asked.ein`](programs/seats-asked.ein): the program with `std.debts`, which states each
  requirement and its payment as facts, and three rules that conclude under `question` what the pass would write
  under `open`: `(question seats)` where an instance is owed, `(question (seats ?g ?c))` for each fact that would
  pay one. `ein saturate --dump` at the node lists them.

The stand-in is flagged by today's static check: its rules read the absence of a payment, which a commitment brings
(`runs/seats-asked.solve.txt`, three sites). That is the stand-in's; the pass is no rule.

## 2. The cube

`question` stands for `open`; the last column is what a fork from root drops and the node does not write again.

| layer | commitment | status | owes | the `open` facts the pass would write there | of root's five, gone |
|---:|---|---|---:|---|---|
| 0 | ∅ | owing | 4 | `(open seats)` `(open (seats Ann C1))` `(open (seats Ann C2))` `(open (seats Bob C1))` `(open (seats Bob C2))` | — |
| 1 | {Ann C1} | owing | 2 | `(open seats)` `(open (seats Ann C2))` `(open (seats Bob C1))` `(open (seats Bob C2))` | `(open (seats Ann C1))` |
| 1 | {Ann C2} | owing | 2 | `(open seats)` and Ann C1, Bob C1, Bob C2 | `(open (seats Ann C2))` |
| 1 | {Bob C1} | owing | 2 | `(open seats)` and Ann C1, Ann C2, Bob C2 | `(open (seats Bob C1))` |
| 1 | {Bob C2} | owing | 2 | `(open seats)` and Ann C1, Ann C2, Bob C1 | `(open (seats Bob C2))` |
| 2 | {Ann C1, Ann C2} | dead, owing | 1 | `(open seats)` and Bob C1, Bob C2 | Ann C1, Ann C2 |
| 2 | {Ann C1, Bob C1} | dead, owing | 1 | `(open seats)` and Ann C2, Bob C2 | Ann C1, Bob C1 |
| 2 | **{Ann C2, Bob C1}** | **$R$** | 0 | **none** | all five |
| 2 | **{Ann C1, Bob C2}** | **$R$** | 0 | **none** | all five |
| 2 | {Ann C2, Bob C2} | dead, owing | 1 | `(open seats)` and Ann C1, Bob C1 | Ann C2, Bob C2 |
| 2 | {Bob C1, Bob C2} | dead, owing | 1 | `(open seats)` and Ann C1, Ann C2 | Bob C1, Bob C2 |
| 3 | each of the four | dead | 0 | none | all five |
| 4 | all four | dead | 0 | none | all five |

*Ann C1* is `(seats Ann C1)`, and so on. The kernel's reading at each node, its owed count and its candidates there,
is the same as the stand-in's `question` facts at every one of the 16 (`runs/nodes.md`).

**At a dead node.** The ruling holds `(open R)` *where a state owes*, and the kernel counts what a dead node owes
today (the oracle's *dead+owing*), so the table writes `(open seats)` at the four dead nodes that owe one. The
stand-in holds them too: its dump at each of the four lists the `question` facts beside the `(false)` that kills the
state. If the pass is not to write at a state whose saturation has ended (Q-M1f.550), those four hold nothing, and
the rest of the table stands: the question below.

## 3. What the fork drops, in one sentence each

- **Root's candidates are not the node's.** At a node of layer 1 the candidate the node supposes is not written
  again, being held, and the three others are, being the payments of what the node still owes. At a state of $R$
  nothing is owed, so nothing is written.
- **`(open seats)` follows the debt**: written at the nine nodes that owe, at none of the seven that do not, $R$'s
  two among them. Read as a fact it would otherwise be the one fact that a larger state does not keep; dropped at the
  fork and made again, it never has to be taken out.
- **Nothing a rule concluded from an `open` fact is dropped.** This program has no such rule. Where one does, its
  conclusion stays above root while the `open` fact it read does not, which is why such a program is flagged
  ([item 61](../61-a-rule-that-reads-open/README.md)).

## 4. Your call

The fork is your ruling, node by node. One thing it does not say:

**Q-M1f.598, put here: the pass at a dead state.**

- **(a)** It writes there as at any state: a dead state that owes holds `(open R)` and the facts that would pay it,
  as the ruling's *where a state owes* reads and as the kernel counts today. **Default.**
- **(b)** It writes nothing at a state whose saturation has ended: the four dead nodes that owe hold no `open` fact;
  the kernel's count of what they owe is as today.
