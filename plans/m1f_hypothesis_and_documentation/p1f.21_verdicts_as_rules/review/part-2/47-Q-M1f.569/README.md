# 47 · Q-M1f.569: what may read an `open` fact, and when

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the ledger](../../../../open_questions.md#q-m1f569--what-may-read-an-open-fact-and-when)

**From your answer to item 44**, 2026-11-19: *"Now let's decide what to do with open context when fork. One option is
to erase it and re-generate after fixed point. Another options is that kernel will delete facts from it when they
proved or refuted, but … I'm not ready to refuse the only stable thing in the whole project - that we don't have
rules that delete facts. So lit's pick option when fork removes all open facts from KB."* And of item 32: *"btw,
absent see it the same way as other facts."*

**The gap.** A fork starts from root's fixpoint. Dropping the `open` facts there takes the facts out and leaves
everything a rule concluded from them. So the fork rule settles what the `open` facts are in a child, and not what
a rule that read one at root has left behind.

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
into `runs/` (both ignored, so on this machine only).

## 1. What the runs show

**The *must*, held and read** ([`r-held-and-read.ein`](programs/r-held-and-read.ein)). On the seats program,
`(unfinished seats)` stands for `(open seats)`: it is concluded where a requirement has no payment. A second rule
reads it and concludes `(noted seats)`. At the cube's 16 nodes (`runs/nodes.md`):

| | nodes that owe nothing | hold `(unfinished seats)` | hold `(noted seats)` |
|---|---:|---:|---:|
| today | 7, the two states of $R$ among them | 7 | 7 |
| with your fork rule | 7 | 0: dropped at the fork, and made again only where something is owed | **7**: it is root's conclusion, and no fact of `open` |

The static check flags the program as not monotone today, so nothing is claimed of its answer.

**The *may*, held and read** ([`f-held-and-read.ein`](programs/f-held-and-read.ein),
[`f-absent-read.ein`](programs/f-absent-read.ein)). `(question F)` stands for `(open F)`, concluded by a rule from
facts no commitment changes. A rule that reads one, and a rule that reads the absence of one, hold the same at
every node and neither program is flagged: such facts are the same in every state.

So the two kinds differ. What a rule concludes from `(open R)`, or from a candidate an obligation made, can be
false above; what it concludes from an `(open F)` made of root's own facts cannot.

## 2. The questions

**1. May a rule's `:match` read an `open` fact?**

- **(a)** Not in this phase. The kernel and the read-out read them; a pattern over `open` in a `:match`, under
  `absent` or not, is refused. Nothing can go stale, and *search steered from inside Ein* waits.
- **(b)** Yes. The `open` facts are written by a pass at each state's fixpoint, as obligations are read today; the
  rules that read them run after it, and the two repeat until nothing changes. A program in which a rule reads
  `(open R)`, or an `(open F)` an obligation made, or the absence of either, is flagged as not monotone, as
  `r-held-and-read.ein` is today: it loads, is warned of, and nothing is claimed. A rule that reads an `(open F)` a
  rule concluded from facts no commitment changes is not flagged. **Default**: it is your *"absent see it the same
  way"*, with one thing new, the order of the pass.
- **(c)** Yes, and at the fork the kernel also drops what was concluded from a dropped fact. It is deletion by the
  kernel and by no rule, and it is new machinery: every conclusion's support followed.

**2. A written `(open P)` at a fork.** You ruled that a program may write one (item 46, cell 4). No rule makes it
again.

- **(a)** It is the program's, and is there in every state, as a rule with an empty `:match` would make it.
  **Default.**
- **(b)** It is dropped with the rest, so it is a candidate at root and held nowhere above.

**3. To confirm: what *re-generate* means.** Every rule that concludes `(open …)`, a hypothesis among them, is read
by that pass at every state, where a hypothesis rule today is read once, at root. $H$ stays what root holds
([item 51](../51-Q-M1f.573/README.md), question 3). **Default: so.**

## 3. Your call

For 1, (a), (b) or (c); for 2, (a) or (b); 3, yes or otherwise. In [`ANSWERS.md`](../ANSWERS.md) under item 47.
Blank keeps the defaults.
