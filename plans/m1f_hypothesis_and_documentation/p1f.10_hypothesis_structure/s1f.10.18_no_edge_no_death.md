# S1f.10.18 — A state that closes no coupling cannot die

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's *"Yes, both"* of that day. **T1 and T2 done 2026-11-16**
([Record](#record-2026-11-16-t1-and-t2)): the claim as stated is false, banked as
[`examples/lattice/15_union_label_dead.ein`](../../../examples/lattice/15_union_label_dead.ein); a narrowed claim
holds by argument, on programs whose whole node gives no fact a label of two classes, and needs a reading of a
pair the coupling does not keep ([`16_pair_through_closure.ein`](../../../examples/lattice/16_pair_through_closure.ein)).
**T3 waits for S1f.10.17**, and what it would need is listed.
**Depends on:** [S1f.10.11](s1f.10.11_coupling.md) (the coupling, its edges), [S1f.10.5](s1f.10.5_ordering.md)
(`structure::NodeCoupling`, the edges read off the whole node in one saturation), [S1f.10.13](s1f.10.13_the_price_list.md)
(the rule for *go*). **Built after [S1f.10.17](s1f.10.17_describe_by_borders.md) is merged**, since both change which
nodes the routes saturate; its measurement does not wait.

**T3 is not built: a written *no* for this way, decided by the main session on 2026-11-16.** The claim as stated is
false ([`15_union_label_dead.ein`](../../../examples/lattice/15_union_label_dead.ein),
[`16_pair_through_closure.ein`](../../../examples/lattice/16_pair_through_closure.ein)). The narrowed claim holds by
argument on premises nothing enforces today: the node read whole, no label a union, no `__symmetric__`, the
candidate alive alone known, and a pair met through a closure read (P\*). Its conclusion cannot be checked where it
is used, since a node wrongly called alive is one whose supersets are dead too. Read on every program it is *no* on
every question. Read when the tree's order reads the node it is *go* on the work, but that is an upper bound no
route can take: every route reads an alive extension's closure and debts, which *alive* alone does not give.
**The next way**, in the record's *What T3 would need*: a route that defers an alive extension's saturation, or
reads its closure and debts without one, with the premises enforced at the read and P\* kept by
`NodeCoupling`. The border route of [S1f.10.17](s1f.10.17_describe_by_borders.md) asks only *dead or alive* of the
sets near a border, and is where such a licence would first have a use.

## The idea, as the user took it up

The main session, 2026-11-16, answering *"It really make sense that we don't need to saturate owed states if no rule
can discharge it … saturating it to false may prune sub-lattice under it, right?"*: a saturation of an owing state
buys only its *dead* bit, which is what prunes, so the useful states to saturate are those that **can** die; and

> a state `S ∪ {b}` that closes no edge through `b`'s question cannot die, so it is alive without a saturation.

The user, the same day: *"Yes, both"*.

## The claim, and what it must survive

On a monotone program, let `S` be an alive state and `b` a candidate alive alone. Every contradiction `st(S ∪ {b})`
holds and `st(S)` does not is derived with `b` among its supports, and its supports' questions form an edge of the
coupling read off the whole node (`st` of every candidate alive alone, which holds `st(S ∪ {b})` by monotonicity). If
no edge through `b`'s question has every one of its questions met by `S ∪ {b}`, no such contradiction exists, and
`S ∪ {b}` is alive. **This is an argument, not yet a licence.** What it must survive, each checked against the code
and the corpus:

- the edges are read with bounds (`NODE_APPLICATIONS`, the ways and descriptions a derivation is cut at) and are
  coarse where a derived fact's label is a union of classes: a cut reading proves nothing;
- a contradiction the node derives through a fact a debt or an absent reads, or the kernel's mirror;
- a candidate dead alone, which the whole node does not commit;
- L8's denials, which are contradictions of two facts;
- a closure that adds candidates, so that `S ∪ {b}`'s state is not the set itself.

If it holds, its premise is checked on every node a route computes, a node that breaks it refusing the run, as L5's,
L8's and L9's are.

## Tasks

1. **T1f.10.18.1 — the argument**, written in this file under
   [`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md)'s rule 2, with every item above settled, or
   the program that breaks it banked under `examples/` with a corpus entry, which ends the stage at a written *no*.
2. **T1f.10.18.2 — measured with no route changed**: on the visited nodes of every corpus program that searches,
   every question, the nodes that close no edge through their last candidate; how many of them were alive (the
   saturations the licence would save) and **how many were dead (each one a counterexample)**; the cost of the
   whole node where a route did not already read it. Against the oracle on every cube within its limit. The rule
   for *go* written here before the table.
3. **T1f.10.18.3 — built where T2 says *go***, after S1f.10.17 is merged: a licence of its own name in the style of
   L1–L9, through `known_dead`'s owner of what a route may skip, with the internal with/without switch and its test,
   the goldens named before they move, and `read_out.md`'s `route` row, `finding_r.md` § 4 and the routes' headers.
4. **T1f.10.18.4 — the record.**

## Acceptance

- The argument is written whether it holds or not; a counterexample is banked.
- R and every field of the description identical with and without, every mode against the oracle.
- The work delta per program and question, losers named, the whole node's cost included.
- No lever; no relation read by name.

## What it must not do

- **Not skip a node whose state a surface prints** without saying where its content comes from.
- **Not ship a licence whose premise is argued and not checked at every node.**

## The rule for *go*, written before the table (2026-11-16)

Written and committed before T2's corpus table was taken. It is
[S1f.10.13's](s1f.10.13_the_price_list.md#the-rule-for-go-written-before-the-table-is-taken), read as
[its defaults](s1f.10.13_the_price_list.md#defaults-taken) read it, applied to one way of spending: **read the whole
node, and do not saturate an extension `S ∪ {b}` of an alive state that closes no edge through `b`.**

For a question:

- **go** when, over the corpus programs that search, the total work of that question with the licence taken (the
  whole node's cost added, the nodes it saves taken off) is lower, **and no program's cell is higher by more than
  one tenth**;
- **no** otherwise, with the programs that lose named.

Read as S1f.10.13's defaults read it, each where the rule does not say:

1. **Both units, and a *go* needs both**: saturations, and facts beyond root.
2. **A stopped cell counts with what it spent**, and the licence is priced on the nodes that cell visited.
3. **A flagged program is in every total**, and the licence is applied to none: its routes are the cube's.
4. **A unit in which a cell is 0 loses by any positive net.**
5. **The whole node is charged to the way that reads it**: one saturation and its facts beyond root, added to every
   cell of a monotone program whose run did not visit that node itself (the tree's coupled order reads it inside
   some runs, and there it is in the cell already). It is charged whether or not it is read whole; a node the
   reading stops is charged what it derived, and saves nothing.

And three that are this stage's, each the user's to revise:

6. **Soundness before work.** The rule prices work. A test under which some dead node closes no edge is no licence,
   whatever its price: the plan's *not ship a licence whose premise is argued and not checked*. So each question's
   verdict is *no* wherever T1 banks a program or T2 finds a node that breaks the test, and the rule's verdict on
   the work is given beside it, as what the licence would have been worth.
7. **The saving is an upper bound.** An alive node that closes no edge is counted saved, its saturation and its
   facts, though the licence decides the node's `dead` bit and not its closure or its debts, which every route but
   `--single`'s last step reads off it. The record says where a route could do without them.
8. **Three tests are priced**, each with the same node: *as stated* (no edge through `b` closed); *narrowed* (and
   `b` in no pair with a member of `S`, `S` holding no other member of a question of `b`, and `b` not dead alone);
   and *narrowed, unmerged* (the narrowed test, taken only on a program whose whole node is read whole and gives no
   fact a label of two classes, the node paid on every monotone program all the same).

## Record (2026-11-16): T1 and T2

**The claim as stated is false, and a narrower one holds.** A derived fact's label in the whole node is the union of
the classes its ways need, so a death that needs one of two candidates is read as needing both, and an extension
dies that meets no edge: [`examples/lattice/15_union_label_dead.ein`](../../../examples/lattice/15_union_label_dead.ein),
four candidates, banked with a corpus entry and an `:expect` of today's R, and held to the oracle by
[`ein-infer/tests/no_edge.rs`](../../../ein.rs/crates/ein-infer/tests/no_edge.rs). It breaks both readings of the
edges, the structure's ([S1f.10.11](s1f.10.11_coupling.md#record-2026-11-15)) and the one the tree reads
(`structure::NodeCoupling`, [S1f.10.5](s1f.10.5_ordering.md)), and each one application back. **What survives** is a
claim narrowed four ways (below): on a program whose whole node is read whole and gives every fact a label of one
class, which SEND's does and the zebras' do not. It needs one reading the coupling does not keep, a pair met through
a member the closure derives, without which
[`16_pair_through_closure.ein`](../../../examples/lattice/16_pair_through_closure.ein) breaks it, banked beside. On the
corpus, T2 finds **no counterexample to the narrowed test on any program whose labels are one class**, but 16, and
**3 899 dead nodes that close no edge, with their last candidate alive alone, on 41 programs, under describe alone**.
By the rule, the narrowed licence is *no* on every question read on every program, and *go* on every question read
at the moment the tree's order reads the node; but **no route today can take that saving**: every route reads an
alive extension's closure and debts, which the licence does not give. No route and no answer changed.

### T1: the argument

**Notation.** `W` is the whole node `NodeCoupling` reads: root with every candidate of `H` committed, saturated to
quiescence, every application read. A **question** is met by a set that holds one of its members. An extension
`S ∪ {b}` **closes** an edge when the edge holds a question of `b` and every question of the edge is met by
`S ∪ {b}`.

**The claim as stated, refuted.** `15_union_label_dead.ein`: `(flag X)` comes from `(a-prop X)` alone or from
`(b-prop X)` alone, and `(flag X)` with `(c-prop X)` and `(d-prop X)` is `(false)`. N is `{a, c, d}` and `{b, c, d}`.
The label of `(flag X)` is `{a, b}`, a union of two classes (`NodeWork::merged` 1), so the conflict's one edge is
`{a, b, c, d}`, and the derivation's edge is `{a, b}`. `{c, d}` is an alive state, `{a, c, d}` is dead, and it closes
neither edge: b is not met. The structure's coupling, with the cover and the pair layer, reads the same two edges
(`ein render h-space`), and so does each reading one application back. The test asserts all of it against the
oracle's cube (`a_union_label_reads_a_death_as_needing_a_question_it_does_not`).

**The narrowed claim.** On a program the static check calls monotone, let the whole node be read with nothing
stopped and nothing cut but descriptions (`NodeCoupling::node_stopped` none, `Cut::ways` 0, labels settled), let
**no derived fact's label be a union** (`NodeWork::merged` 0), and let **no relation be `(__symmetric__ …)`**. Let `S`
be an alive state and `b ∉ S` a candidate **alive alone**. If `S ∪ {b}` is dead, then one of:

- (E) an edge through a question of `b` is closed by `S ∪ {b}`;
- (Q) `S` holds a member, other than `b`, of a question of `b`;
- (P\*) a pair of the node is met by `S ∪ {b}`, a member met where it is committed in `S ∪ {b}`, or where `W` derives
  it and its label is met by `S ∪ {b}`.

So where none of the three holds, `S ∪ {b}` is alive.

**The argument**, under [`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md)'s rule 2.

1. *Every application of `st(S ∪ {b})` is an application of `W`.* The program is monotone: no read of a plan can
   take a conclusion away as a commitment grows, an `absent` reads only relations no commitment extends, and so the
   premises and guards that hold in `st(S ∪ {b})` hold in `W ⊇ st(S ∪ {b})`. `W` is read with every application,
   redundant ones and what their universals read among them (`Nodes::with_fixpoint_supported_within`), and every
   justification the store kept. The one derivation that is neither is a native mirror's of a fact already in the
   KB: no firing is returned for it, and the store keeps it only as an alternative justification, which it refuses
   for a fact whose primary is a commitment and keeps nowhere under `:record-alternative-justifications false`. Hence
   the premise on `__symmetric__` ([K-41](findings.md#from-the-stages-of-tracks-s-and-r)).
2. *Every fact `f` of `st(S ∪ {b})` beyond root is committed in `S ∪ {b}`, or its label is met by `S ∪ {b}`.* By
   induction on the derivation of `f` in `st(S ∪ {b})`. Its last application is a step of `W`'s graph; choose for
   each premise the candidate it is where it is committed in `S ∪ {b}`, and its label otherwise (met, by induction;
   a candidate `W` commits is read both ways, which is how a member of the closure is read). That is one way of the
   step, and each of its descriptions is met: a description is the derived premises' labels with one question of
   each committed candidate read, which holds it. The whole node's reading filters no way (`Clash::none`), so the
   way is among the items of `f`. The label is one class, so it covers every item, the way among them, and is one
   of the way's descriptions: met. A description cut past `DESCRIPTIONS` is still a description.
3. *The contradiction is read, and it is through `b`.* `S` is alive, so the derivation of a contradiction of
   `st(S ∪ {b})` reads `b`; `b` is alive alone, so it reads another candidate or a fact derived with one. Its last
   step is `W`'s: a `(false)` step, a fact beside its negation, or one beside a negation at root, each read as a way
   (`node_edges`). By 2 that way's descriptions are met, and the cover of the contradictions chooses one of them,
   `κ`. A question of `b` is in `κ`: `b` is committed where the derivation reads it, so the label of every fact on
   the way from `b` holds a question of `b` (a label is one description of the way the derivation takes), and `κ`
   holds those labels. If `κ` has two questions or more it is an edge, and it is closed: (E). If it has one, every
   candidate the derivation reads committed is in that question of `b`, and one of them is not `b`, since `b` is
   alive alone, so `S` holds it: (Q).
4. *The one way not read.* `NodeCoupling` takes out of its graph a `(false)` step whose two premises are both
   committed in `W`, and counts it as a pair (`node.rs`). Where both are committed in `S ∪ {b}` the pair is met; where
   one is derived there, the step's way through its label is gone from the graph, and only (P\*) reads it: the
   member's label is met by 2. That is `16_pair_through_closure.ein`.

So the narrowed claim holds where its premises do. **None of them is enforced today by anything that fails**: the
merged count and the cuts are reported by the reading and read by no route, the singles are the state route's and
not the tree's, and P\* reads a label `NodeCoupling` does not keep. T3 would have to enforce each. **And its
conclusion cannot be checked where it is used.** L1's check refuses a run that meets an alive node above a dead one;
a node the licence wrongly calls alive is dead, and so is everything above it, which that check finds consistent. A
wrong *alive* shows only where the node itself is saturated after all, so a licence that never saturates the nodes
it passes checks nothing of them, and its premises are all the evidence there is.

**Each item of the plan, settled.**

| item | what it does to the claim | settled by |
|---|---|---|
| **a cut reading** | the node stopped (`NodeStop`): no edge, so no claim. Labels unsettled past `ROUNDS`: a fact can be left with no label, its contradiction with no way, and no edge for a death. Ways cut past `WAYS`: one union way, coarse as a label is. Descriptions cut past `DESCRIPTIONS`: harmless, every description kept is met | premise: read whole, no way cut, labels settled. No corpus program that searches meets a cut (T2: all 202 read whole) |
| **coarse labels** | **breaks it**: a union label reads a death as needing a question it does not, so the edge is larger than the death and is not met. This is the direction that keeps the components honest (too coarse claims no false independence) and the wrong one for this claim, which needs a death read no larger than it is | **banked: `15_union_label_dead.ein`**. Narrowed to `NodeWork::merged` 0. On the corpus, every counterexample to the narrowed test but 16's is on a program with a union label, 12 of them: the zebras, `pets.ein`, three `branching/` programs, `tests/region/10_denied_extensions.ein` and 15 |
| **a debt** | none: an obligation rule derives nothing, and owing is not dead | argued: `owes` reads the quiescent KB and asserts no fact |
| **an `absent`** | none on a monotone program: its relation is static, so it reads at `W` what it reads at `S ∪ {b}`, and the application is `W`'s | argued; the premise is the static check's, enforced by every route's refusals |
| **the kernel's mirror** | a native mirror's first derivation is a firing and is read; a redundant one only as a stored justification, which the store refuses for a fact committed in `W` and keeps nowhere under `:record-alternative-justifications false`. A candidate whose mirror twin is a candidate is then read only as committed, never as derived | premise: no `__symmetric__` relation. Probed out of scope: K-41, where it breaks the structure's own claim that every member of N lies inside one edge |
| **`written`** | none: the loader states every `(written T)` and nothing else does, so each is at root, in every node | argued: `defined_behaviour.md` § 4.30 |
| **a candidate dead alone** | `W` commits it; its own death is a class of its own question and no edge, so `S ∪ {b}` closes none and is dead | premise: `b` alive alone. On the corpus 840 dead nodes under describe, 541 under `--single`, are of this kind. The state route has every single before it extends a larger state; the tree has none it did not visit |
| **L8's two-fact contradictions** | a fact beside its negation is read as a way through both (`Graph::pairs`, `half_pairs`), never taken out as a pair; and L8 decides those nodes before any saturation | covered by 3; and the licence is asked only where `known_dead` is not |
| **a closure that adds candidates** | a candidate the closure derives is read through its label (2), so the state's members beyond the commitment are seen; but not through a pair (4) | narrowed by (P\*): **banked: `16_pair_through_closure.ein`**, every label one class. No corpus program meets it |
| **a conflict of exactly two candidates** | `NodeCoupling` makes no edge of it, so the claim as stated misses every such death: *different letters, different digits* on SEND | narrowed by (P\*), and by (Q) for a conflict inside one question that a derivation reaches. On the corpus, of the dead nodes the claim as stated misses with `b` alive alone, 1 822 of 3 899 under describe and 2 338 of 2 345 under `--single` are of this kind |
| **which reading** | **the whole node's, followed back to the candidates.** One application back is not complete: a contradiction more than one application past what the candidates derive has no way in its first round (SEND's columns `c2` to `c4`), so its death has no edge. And neither is right where a label is a union | the narrowed claim's premise |

**What it would take where labels are unions.** The claim needs a death read **no larger** than it is: per fact, the
questions **every** way to it needs, given that the way reads `b`, an intersection where the coupling keeps a union.
That is a reading of its own, the dual of the label, not built or measured here.

### T2: measured

```sh
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example no_edge
ein.rs/target/release/examples/no_edge --jobs 6        # build/no_edge/: no_edge.jsonl, counterexamples.jsonl, no_edge.md
ein.rs/target/release/examples/no_edge --report        # the tables again
```

The example is [`ein-infer/examples/no_edge.rs`](../../../ein.rs/crates/ein-infer/examples/no_edge.rs) (its header
has the protocol, `utils/README.md` a row). On the final tree, 2026-11-16: **202 programs that search, every one
read, 173 monotone**; the whole node read whole on all 202, with no label a union on 156 (129 of the monotone ones).
The cube checked on the 142 monotone programs within the oracle's limit, 22 more being flagged; no cell in error and
no text unmapped. An **extension** is a visited node `S ∪ {b}`, `S` the closure of an alive node visited before it;
the four tests are E, P (P\* read over commitments, since the reading keeps no label of a candidate), Q, and `b`
dead alone. Every count is a node; *facts* are beyond root.

| question | extensions | alive / dead | alive closing no edge, as stated: nodes · facts | dead closing no edge | of those: `b` dead alone / a pair / one question / none of these | alive, narrowed: nodes · facts | dead, narrowed |
|---|---:|---|---|---:|---|---|---:|
| describe | 1 234 469 | 1 218 232 / 16 237 | 1 216 767 · 9 662 650 | 4 739 | 840 / 1 822 / 0 / 2 077 | 1 216 730 · 9 662 552 | 2 077 |
| `--count` | 1 234 469 | 1 218 232 / 16 237 | 1 216 767 · 9 662 650 | 4 739 | 840 / 1 822 / 0 / 2 077 | 1 216 730 · 9 662 552 | 2 077 |
| `--single` | 37 493 | 4 928 / 32 565 | 3 617 · 32 050 | 2 886 | 541 / 2 338 / 0 / 7 | 3 613 · 32 042 | 7 |
| `-n 1` | 34 414 | 5 316 / 29 098 | 3 339 · 28 941 | 2 949 | 232 / 2 713 / 0 / 4 | 3 335 · 28 933 | 4 |
| `--minimal` | 366 457 | 277 652 / 88 805 | 271 888 · 2 721 402 | 66 634 | 4 867 / 58 585 / 0 / 3 182 | 271 856 · 2 721 322 | 3 182 |
| `--maximal` | 40 654 | 5 727 / 34 927 | 4 359 · 45 475 | 4 515 | 1 730 / 2 487 / 0 / 298 | 4 346 · 45 437 | 298 |
| **the cube** | 196 392 | 159 672 / 36 720 | 157 556 | 34 604 | 754 / 1 991 / 90 / 31 769 | 157 444 | 31 769 |

**Nearly every alive extension closes no edge**, under every question: what the edges say of an alive node is
almost nothing, which is the claim's appeal. **And so do the dead ones the edges cannot see.** By kind:

- **As stated, `b` alive alone** (the plan's premise), the dead nodes that close no edge are 3 899 under describe on
  41 programs, 2 345 under `--single` on 10, 2 717 under `-n 1` on 8, 61 767 under `--minimal` on 24, 2 785 under
  `--maximal` on 29, and 33 850 in the cubes of 28: **44 programs in all**, each a counterexample to the claim as the
  plan states it.
- **Narrowed** (all four tests), 2 077, 7, 4, 3 182, 298 and 31 769: **13 programs**, the zebras
  (`zebra.ein`, `zebra2.ein` and `-obligations`, `zebra2-minus-15.ein` and `-obligations`, `zebra2-hints.ein`),
  `saturation/type-exclusivity/pets.ein`, `branching/02_one_dead_one_alive.ein`, `04_two_levels.ein`,
  `08_hypothesis_relation_whitelist.ein`, `tests/region/10_denied_extensions.ein`, and the two banked here.
- **Narrowed, on a program with no union label: none but `16_pair_through_closure.ein`'s**, one node under describe,
  `--count` and `--maximal` and two in its cube. That is the narrowed claim's only counterexample, and (P\*) is what
  it lacks.

**Every counterexample** is a line of `build/no_edge/counterexamples.jsonl` (121 066 lines, those with `b` dead alone
among them: program, question, `S`, `b`, and the first other test it meets); by program and question, a cell *as
stated with `b` alive alone (narrowed)*, `·` for none:

| program | union labels | describe | `--count` | `--single` | `-n 1` | `--minimal` | `--maximal` | cube |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `examples/beyond_arith/cryptarithm.ein` | 0 | 135 | 135 | · | · | 135 | · | · |
| `examples/beyond_arith/equation_by_search.ein` | 0 | 90 | 90 | · | · | 90 | · | · |
| `examples/beyond_arith/equation_no_solution.ein` | 0 | 45 | 45 | · | · | 45 | · | · |
| `examples/beyond_arith/equation_two_unknowns.ein` | 0 | 135 | 135 | · | · | 135 | · | · |
| `examples/beyond_arith/send_more_money.ein` | 0 | 599 | 599 | 1 134 | 1 241 | 1 840 | 1 134 | · |
| `examples/beyond_arith/send_more_money_written.ein` | 0 | 599 | 599 | 1 134 | 1 412 | 55 994 | 1 134 | · |
| `examples/binary_logic/equivalent.ein` | 0 | 3 | 3 | · | · | 3 | · | 22 |
| `examples/binary_logic/knights.ein` | 0 | 3 | 3 | · | · | 3 | · | 14 |
| `examples/binary_logic/not_equivalent.ein` | 0 | 3 | 3 | · | · | 3 | · | 22 |
| `examples/binary_logic/satisfiable.ein` | 0 | 3 | 3 | · | · | 3 | · | 22 |
| `examples/binary_logic/synthesis.ein` | 0 | 1 | 1 | · | · | · | 1 | 2 |
| `examples/binary_logic/truth_table.ein` | 0 | 3 | 3 | · | · | 3 | · | 22 |
| `examples/binary_logic/unsatisfiable.ein` | 0 | 3 | 3 | · | · | 3 | · | 22 |
| `examples/branching/02_one_dead_one_alive.ein` | 52 | 44 (44) | 44 (44) | · | · | · | 38 (38) | · |
| `examples/branching/04_two_levels.ein` | 46 | 16 (16) | 16 (16) | · | · | · | 8 (8) | · |
| `examples/branching/08_hypothesis_relation_whitelist.ein` | 52 | 92 (92) | 92 (92) | · | · | · | 114 (114) | · |
| `examples/branching/15_lookahead_two_step_on.ein` | 0 | 1 | 1 | · | · | · | 1 | 2 |
| `examples/branching/16_lookahead_two_step_off.ein` | 0 | 1 | 1 | · | · | · | 1 | 2 |
| `examples/ein-bugs/complete-records-stale.ein` | 0 | 2 | 2 | · | · | · | 3 | 5 |
| `examples/ein-bugs/unsat-core-id-space.ein` | 0 | 1 | 1 | · | · | · | 1 | 2 |
| `examples/ein-bugs/wrong-answers/einb-dump-answer.ein` | 0 | 1 | 1 | · | · | · | 1 | 2 |
| `examples/features/11_expect_ambiguity.ein` | 0 | 4 | 4 | · | · | · | 4 | 12 |
| `examples/features/13_mixed_solution_and_open.ein` | 0 | 1 | 1 | 1 | · | 1 | 1 | 2 |
| `examples/lattice/01_subset_pruned.ein` | 0 | 1 | 1 | · | · | · | 1 | 8 |
| `examples/lattice/05_owed_seats_bounded.ein` | 0 | 4 | 4 | 1 | 1 | 4 | 3 | 12 |
| `examples/lattice/06_friends_tables_everyone_sits.ein` | 0 | 67 | 67 | 31 | 27 | 143 | 87 | · |
| `examples/lattice/07_two_pairs_two_tables.ein` | 0 | 64 | 64 | 25 | 23 | 120 | 76 | 1 296 |
| `examples/lattice/14_seats_nothing_proposes.ein` | 0 | 36 | 36 | 12 | 9 | 56 | 26 | 484 |
| `examples/lattice/15_union_label_dead.ein` | 1 | 2 (2) | 2 (2) | · | · | · | 2 (2) | 6 (6) |
| `examples/lattice/16_pair_through_closure.ein` | 0 | 2 (1) | 2 (1) | · | · | · | 2 (1) | 4 (2) |
| `examples/lattice/synth/one_model_per_component.ein` | 0 | 6 | 6 | · | · | · | 6 | 12 |
| `examples/saturation/type-exclusivity/pets.ein` | 6 | · | · | · | · | · | 84 (84) | 31 743 (31 743) |
| `examples/word_problems/boxes12.ein` | 0 | 1 | 1 | · | · | · | 1 | 2 |
| `examples/word_problems/boxes13.ein` | 0 | 1 | 1 | · | · | · | 1 | 2 |
| `examples/zebra.ein` | 159 | 76 (76) | 76 (76) | 1 (1) | · | 78 (78) | 1 (1) | · |
| `examples/zebra2-hints.ein` | 63 | · | · | · | · | · | 4 (4) | · |
| `examples/zebra2-minus-15-obligations.ein` | 169 | 865 (865) | 865 (865) | 3 (3) | 2 (2) | 1 492 (1 492) | 23 (23) | · |
| `examples/zebra2-minus-15.ein` | 169 | 865 (865) | 865 (865) | 3 (3) | 2 (2) | 1 492 (1 492) | 23 (23) | · |
| `examples/zebra2-obligations.ein` | 103 | 58 (58) | 58 (58) | · | · | 60 (60) | · | · |
| `examples/zebra2.ein` | 103 | 58 (58) | 58 (58) | · | · | 60 (60) | · | · |
| `tests/region/09_query_over_maximal.ein` | 0 | 4 | 4 | · | · | · | 4 | 12 |
| `tests/region/10_denied_extensions.ein` | 9 | · | · | · | · | · | · | 108 (18) |
| `tests/stdlib/bool/inputs/01_in_drives.ein` | 0 | 2 | 2 | · | · | 2 | · | 4 |
| `tests/stdlib/bool/inputs/02_required.ein` | 0 | 2 | 2 | · | · | 2 | · | 4 |

One of each kind, as the file has it: SEND, describe, `S = {(val D 1)}`, `b = (val D 0)`, a pair (and one question);
`zebra2.ein`, describe, `S = {(pet-loc Zebra House-1)}`, `b = (drink-loc Tea House-2)`, none, its one edge being 31
of its 36 questions; `zebra2-minus-15.ein`, describe, `S = {(color-loc Blue House-1)}`, `b = (nation-loc Japanese
House-2)`, none, its one edge 45 of 46.

**The instances**, a cell *saturations · facts* and the counts of the narrowed test (`python3` over
`no_edge.jsonl`). The whole node: SEND's is 1 · 166, five edges (its columns and `{D, E}`), 640 pairs, no label a
union; `zebra2.ein`'s 1 · 182, one edge of 31 of its 36 questions, 103 labels a union; `zebra2-minus-15.ein`'s
1 · 222, one edge of 45 of 46, 169 a union.

| program | question | the cell | alive extensions closing nothing, narrowed | dead extensions closing nothing, narrowed |
|---|---|---|---|---:|
| SEND | `--single` | 18 030 · 221 803, the node read in the run | 1 571 · 13 295 of 2 207 | 0 of 15 821 |
| SEND | describe | 336 101 · 1 422 943, stopped | 329 638 · 1 371 486 of 330 328 | 0 of 5 771 |
| `zebra2.ein` | `--single` | 7 · 666 | 3 · 120 of 3 | 0 of 3 |
| `zebra2.ein` | describe | 263 · 22 503 | 183 · 8 125 of 183 | 58 of 79 |
| `zebra2-minus-15.ein` | describe | 7 261 · 545 818, stopped | 6 394 · 353 566 of 6 394 | 865 of 865 |

On the zebras the whole node's edge is every question the clues reach, as S1f.10.11 found, so every alive extension
and nearly every dead one closes nothing: the edges say nothing there.

### The rule, per question

Applied as [written before the table](#the-rule-for-go-written-before-the-table-2026-11-16) (`no_edge --report`),
totals over the 202 programs *saturations · facts*, and the cells higher by more than a tenth:

| question | without | as stated | narrowed | narrowed, unmerged | narrowed, unmerged, read at the order's moment |
|---|---|---|---|---|---|
| describe | 1 690 462 · 12 423 231 | 473 787 · 2 781 503, 31 higher | 473 824 · 2 781 601, 31 | 1 014 531 · 9 612 518, 40 | 1 019 059 · 9 602 319, **0** |
| `--count` | the same | the same | the same | the same | the same |
| `--single` | 37 790 · 481 001 | 34 311 · 470 745, 114 | 34 315 · 470 753, 114 | 34 462 · 475 574, 124 | 35 468 · 458 431, **0** |
| `-n 1` | 34 634 · 423 046 | 31 466 · 415 944, 157 | 31 470 · 415 952, 157 | 31 572 · 420 066, 162 | 32 372 · 400 816, **0** |
| `--minimal` | 367 177 · 4 361 641 | 95 461 · 1 662 406, 140 | 95 493 · 1 662 486, 140 | 119 058 · 3 105 907, 142 | 120 586 · 3 089 479, **0** |
| `--maximal` | 496 948 · 2 710 351 | 492 674 · 2 686 212, 39 | 492 687 · 2 686 250, 39 | 493 324 · 2 703 370, 52 | 494 632 · 2 688 566, **0** |

**The verdicts.**

- **As stated and narrowed: *no* on every question**, twice: counterexamples on the corpus (rule 6), and on the work
  the whole node costs more than a tenth on the small cells (31 to 157 programs, K-26's and K-27's kind), among them
  `beyond_arith/equation_two_unknowns.ein`, describe 1 466 · 60 223 → 1 137 · 77 213, a node of more facts than the
  nodes it saves, and `branching/01_saturate_only.ein`, 17 · 56 → 18 · 85.
- **Narrowed, unmerged: *no* on every question**, by the same small cells (40 to 162), and by 16 under rule 6 until
  (P\*) is read.
- **Narrowed, unmerged, read at the order's moment: *go* on the work on every question**, no cell higher; **it was
  not in the rule as committed**, and is given as what T3 would weigh. It was added after the first table was seen,
  because the third way sank only on cells the node costs more than (K-27), and it reads the node where the tree's
  order would (`order::READ_AFTER` 20 saturations, `READ_SHARE` a twentieth of the facts), saving only the nodes
  visited after. Its saving is mostly SEND's: under describe 328 474 · 1 369 236 on each SEND file of
  671 416 · 2 822 222 in all, the 14 468 saturations left on 21 other cells (`syntax/equality.ein` and
  `constraint-scopes.ein` the largest, 3 985 each); under `--single` and `--maximal` 1 161 · 11 285 on each, of 18 030 · 221 803 (6.4 %); under `-n 1` 1 126 and
  1 136; under `--minimal` 182 901 on SEND, 63 384 on SENDW, and 224, 64 and 17 on three seating programs and 12 on
  `cryptarithm.ein`; nothing on any zebra.
- **And every one of those savings is an upper bound no route can take today** (rule 7). The licence decides a
  node's `dead` bit; every route reads an alive extension's closure (the state it reaches, `k_s`, the next layer, the
  printed model) and its debts (owing or in R). `--single`'s last step reads only *alive*, and then prints the second
  state it found. So the saving T3 can take is what a route can do with *alive* alone, which is zero on the routes as
  built.

### Defaults taken

Each the user's to revise.

1. **The reading measured is `structure::node_coupling`'s**, over L5's table as `Space::alternatives` reads it; the
   tree reads the route's table, L9 included (`route_alternatives`), so the questions can differ where an instance
   only L9 licenses (`lattice/14_seats_nothing_proposes.ein`).
2. **An extension is a visited node `S ∪ {b}`, `S` the closure of an alive node visited before it.** A node with
   several such readings closes nothing when one of them does: the licence may take any. A node that is no such
   extension (a shrink's subset, a transversal, ⊤) is not tested.
3. **The singles are the node function's own**, a saturation a candidate, the reader's and no cell's.
4. **The cube is read over alive states** (closed sets), as the claim is stated, not over every alive commitment.
5. **The watched runs take no core** (`Cores::READ_OUT`): a core is no work, and the price list showed a watched
   run's work to be the plain run's.
6. **(P\*) is measured as P**, a pair read over commitments, since `NodeCoupling` keeps no candidate's label; the
   narrowed savings are an upper bound on (P\*)'s.
7. **Two programs banked, not three.** A contradiction inside one question (Q) was probed and is argued; it is no
   counterexample to the narrowed claim. The native mirror's probe is K-41's, out of scope.
8. **The fourth way of the rule's table was priced after the first table was seen**, and is marked so.
9. **The narrowed claim's verdict is written as *holds by argument, premises not enforced***, not as a licence, and
   T3 is left to the main session: whether a licence of *alive* alone is worth a route that can use it is a
   question this stage measures and does not settle.

### What T3 would need

- **A use for *alive* alone.** No route today proceeds from an alive node without its closure and debts. Either a
  route that defers an alive extension's saturation until a descendant needs it (a descendant's state holds it,
  monotone), or a reading of `cl(S ∪ {b})` and of its debts without the node: `st(S) ∪ st({b})` where no derivation
  edge through `b` is met either, a second claim with the same coarseness to settle.
- **(P\*)**: `NodeCoupling` keeping each candidate the node derives with its label, or not taking a pair out of the
  graph where a member is derived.
- **The premises enforced at the read**: read whole (no stop, no way cut, labels settled), `NodeWork::merged` 0, no
  `__symmetric__` relation (or the mirror's redundant derivations read, K-41), and `b` alive alone known, which the
  tree does not compute.
- **The moment the order uses**, or the small cells sink every question.
- **A check of the conclusion.** L1's refusal does not catch a wrong *alive* (T1); a route that defers a passed node
  and saturates it later, when it needs its state, checks it then and can refuse, and one that never does checks
  nothing.

### Verification

On the final tree: `cargo build` of the example; `cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p
ein-infer --lib --test no_edge`; `-p ein-cli --test corpus_cli --test world_anchors` and `-p ein-render --test
id_order_invariance`, with `corpus_exits.txt` gaining the two programs' ten cells and `corpus_shapes.md5` their 82
lines and nothing else (`EIN_BLESS=1`, the diff checked); the sweeps the two programs join, `-p ein-infer --test
coupling --test node_coupling --test explore --test region --test structure_invariance --test theory_share --test
cover --test exactly_one --test pair_layer`; `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, and `utils/doc_audit.py
--links --check` with the gate's argument list. No `./run_tests.sh`, nightly or container: the batch's gate is the
coordinator's. Walls are not measurements here; the machine was shared.
