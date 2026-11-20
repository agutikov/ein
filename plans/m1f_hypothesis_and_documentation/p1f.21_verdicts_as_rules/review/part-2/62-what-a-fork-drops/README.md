# 62 · What a fork drops, on the seats cube, node by node

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[your answers](../ANSWERS.md)

**What it shows.** Task 1.5 of the stage. You ruled that a fork drops every `open` fact and that they are made again
at the fixpoint (Q-M1f.566), every rule that concludes `(open …)` being read at every state (Q-M1f.569). Here, at
each of the 16 nodes of the seats program's cube, the `open` facts the pass would write there, and which of root's a
fork drops for good. The same cube on the final binary, node by node, is [On the built engine](#on-the-built-engine),
at the foot of the page.

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

## On the built engine

**In short.** The final binary writes, at each of the 16 nodes, exactly the `open` facts the stand-in's table of § 2
gives, the four dead nodes that owe among them: what Q-M1f.598's default (a) says. The stand-in itself, written with
`open`, does not agree above root, for a reason that is not the fork's (the last paragraph).

**How it was taken.** The final binary, `build/review-bin/ein-final-7b72bd84`, the release build of 7b72bd84 (its
five lines: `runs/built/version.txt`), with the checkout's `stdlib/`. `tools/run-built.sh` re-takes every run into
`runs/built/`, and `tools/nodes-built.py` the cube node by node into `runs/built/nodes.md`, `nodes.json` and `nodes/`.
The programs: the seats program as the corpus has it; the stand-in as it was,
[`seats-asked.ein`](programs/seats-asked.ein), and in the spelling the built engine reads,
[`seats-asked.built.ein`](programs/seats-asked.built.ein), `open` where it wrote `question` and nothing else; and a
probe written for this section, [`probe-hypothesis-rule-absent.ein`](programs/probe-hypothesis-rule-absent.ein).
The seats program and the two stand-ins have the full set of runs: the read-out, `-p`, `ein show hyps`, `ein saturate
--dump`, the oracle with its dump and `--states`, the events and the summary, and the knowledge base, the lattice
(both views) and the region drawn; the probe has its read-out, `ein show hyps`, `ein saturate --dump` and the oracle.

**How a node's `open` facts are read.** The built engine writes a state's `open` facts at root, at a state that is
printed or saved, and at every state under `--in-flight`. On this program the readings of a search's nodes do not
show them:

- `ein oracle --dump --states --in-flight` keeps each node's facts *beyond root's*, and every `open` fact a node of
  this cube holds, root holds too: an owing node's `state` is its commitment alone
  (`runs/built/seats.oracle-in-flight/region.json`). The dump is the one taken without the flag
  (`runs/built/seats.oracle/region.json`), and nothing is found in flight (`"in_flight": null`,
  `runs/built/seats.in-flight.summary.json`).
- `ein solve -p --in-flight` prints the two states of $R$, which hold no `open` fact
  (`runs/built/seats.solve-p-in-flight.txt`).
- `ein render lattice --view full --in-flight` draws each node's commitment, for the 10 nodes the describing run
  visits (`runs/built/seats.lattice-full-in-flight.dot`), as it does without the flag.

So each node is read as a **saved state**: the answer's first saved file (`runs/built/seats.dump-answer/models/01.ein`)
with its `(commit …)` replaced by the node's commitment (none for ∅). `ein saturate --dump` of that file forks root,
commits the node's facts and prints the node's fixpoint, with each `open` fact the pass writes there and the rule that
wrote it (`runs/built/nodes/saved-NN.saturate-dump.txt`, `NN` the node's mask in the oracle's dump); `ein solve
--json-summary` of it says what the node owes and whether it is dead (`saved-NN.summary.json`). Each node is also
read as a root of its own, its commitment written as facts, as the stand-in's tool read it (`runs/built/nodes/
plain-NN.*`, with `ein show hyps`): the two readings give the same `open` facts at every node (`runs/built/nodes.md`).

**The cube on the built engine** (`runs/built/nodes.md`; status and owes are the oracle's, and the saved node's
summary gives the same owes and the same death at each):

| layer | commitment | status | owes | what the built engine holds at the node | the stand-in's row of § 2 |
|---:|---|---|---:|---|---|
| 0 | ∅ | owing | 4 | `(open seats)` and Ann C1, Ann C2, Bob C1, Bob C2 | the same |
| 1 | {Ann C1} | owing | 2 | `(open seats)` and Ann C2, Bob C1, Bob C2 | the same |
| 1 | {Ann C2} | owing | 2 | `(open seats)` and Ann C1, Bob C1, Bob C2 | the same |
| 1 | {Bob C1} | owing | 2 | `(open seats)` and Ann C1, Ann C2, Bob C2 | the same |
| 1 | {Bob C2} | owing | 2 | `(open seats)` and Ann C1, Ann C2, Bob C1 | the same |
| 2 | {Ann C1, Ann C2} | dead, owing | 1 | `(false)` beside `(open seats)` and Bob C1, Bob C2 | the same |
| 2 | {Ann C1, Bob C1} | dead, owing | 1 | `(false)` beside `(open seats)` and Ann C2, Bob C2 | the same |
| 2 | **{Ann C2, Bob C1}** | **$R$** | 0 | **no `open` fact** | the same |
| 2 | **{Ann C1, Bob C2}** | **$R$** | 0 | **no `open` fact** | the same |
| 2 | {Ann C2, Bob C2} | dead, owing | 1 | `(false)` beside `(open seats)` and Ann C1, Bob C1 | the same |
| 2 | {Bob C1, Bob C2} | dead, owing | 1 | `(false)` beside `(open seats)` and Ann C1, Ann C2 | the same |
| 3 | each of the four | dead | 0 | `(false)`, no `open` fact | the same |
| 4 | all four | dead | 0 | `(false)`, no `open` fact | the same |

One thing the stand-in could not show: which writer writes each fact. At root all five are written by `total-owed`
(two obligations write some of them, and identical facts are one); at a node of layer 1 the payment of the chair's
debt is `surjective-owed`'s, `(open (seats Ann C2))` at {Ann C1} (`runs/built/nodes/saved-01.saturate-dump.txt`).

**The page's numbers beside the built engine's:**

| | the stand-in, on edbe6c3b | the built engine | where |
|---|---|---|---|
| nodes of the cube; of them in $R$ | 16; 2 | 16; 2 | `runs/built/seats.oracle.txt`: `16 saturations`, `states (k_s) 2` |
| root's `open` facts | 5 | 5 | `runs/built/nodes/saved-00.saturate-dump.txt` |
| nodes that hold `(open seats)` | 9 | 9 | `runs/built/nodes.md` |
| nodes that hold no `open` fact | 7: $R$'s two and the five above them | 7, the same | `runs/built/nodes.md` |
| of root's five, gone at a node of layer 1 | 1, the candidate the node supposes | 1, the same | `runs/built/nodes.md`, last column |
| of root's five, gone at a state of $R$ | 5 | 5 | `runs/built/nodes.md` |
| dead nodes that owe and hold `open` facts | 4, the `question` facts beside `(false)` | 4, the `open` facts beside `(false)` | `runs/built/nodes/saved-NN.saturate-dump.txt`, `NN` 03, 05, 10 and 12 |
| the node's candidates, the node a root of its own | the `question` facts, at all 16 | the saved node's `open` facts, at all 16 | `runs/built/nodes/plain-NN.show-hyps.txt` |
| the stand-in `seats-asked.ein` flagged | yes, 3 sites | yes, 3 sites, and stale facts at 15 of the 16 nodes, as on edbe6c3b | `runs/built/seats-asked.solve.txt` |
| found in flight | — | nothing, with `--in-flight` or without | `runs/built/seats.in-flight.summary.json` |

**Q-M1f.598 on the built engine.** The binary writes at a dead state as at any, the default (a): each of the four dead
nodes that owe holds `(false)` beside `(open seats)` and the two facts that would pay what it owes, and owes 1, as the
oracle counts; the dead nodes that owe nothing hold `(false)` and no `open` fact
(`runs/built/nodes/saved-03.saturate-dump.txt` and its kin). A dead node is read here as a saved state, where the
pass runs; no command prints a dead node of the search with its `open` facts.

**The stand-in in built spelling agrees at all 16 nodes, since a wrong answer these runs found was fixed.**
[`seats-asked.built.ein`](programs/seats-asked.built.ein) loads, is monotone by the static check, and answers as the
seats program: $H$ the same four candidates, from `union`, and $R$ the same two states
(`runs/built/seats-asked.built.show-hyps.txt`, `seats-asked.built.solve.txt`). At every node it holds the `open`
facts the stand-in's `question` facts were (`runs/built/nodes.md`, column *the built stand-in*: *yes* at all 16).

**As first run it did not**, and that is how the wrong answer was found (V-89; `runs/built-before-the-fix/` has
those runs). Above root it held all four candidates at every node, at $R$'s two states too: its rules
`asked-total` and `asked-surjective` read `(absent (debt-paid …))`, and a rule that concludes `(open F)` wrote its
hypothesis for every match of its other premises, its `(absent …)` premise not read. The probe shows it alone,
[`probe-hypothesis-rule-absent.ein`](programs/probe-hypothesis-rule-absent.ein): `(q a)` is written, so the match
`(and (p ?x) (absent (q ?x)))` holds of `b` alone, and the rule that concludes a fact concludes `(s b)` alone.

| the probe | before the fix | since |
|---|---|---|
| what the rule that concludes a hypothesis writes | `(open (r a))` and `(open (r b))` | `(open (r b))` |
| $H$ | 2 candidates | 1 |
| $R$ | 4 states, the program monotone, so claimed | 2 states |
| the runs | `runs/built-before-the-fix/probe-hypothesis-rule-absent.*` | `runs/built/probe-hypothesis-rule-absent.*` |

The engine before the phase, the rule written as an `hrule`, proposed the same two
(`runs/built/probe-standin/probe-hypothesis-rule-absent.hrule.show-hyps.txt`): the wrong answer was older than the
build. [S1f.21.24](../../../s1f.21.24_a_hypothesis_rule_reads_its_absent.md) filed it and fixed it; what it took
by default is [item 86](../86-a-hypothesis-rule-reads-its-absent/README.md).
