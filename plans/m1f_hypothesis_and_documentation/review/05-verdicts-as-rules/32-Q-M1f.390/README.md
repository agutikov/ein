# 32 — Q-M1f.390: what an obligation emits

## 1. The question

An obligation is a rule whose conclusion is `(open ?R)`: *the relation R still owes something here*. Today that
conclusion is never a fact of the knowledge base. The kernel reads every obligation once on a state's fixpoint,
counts the matches whose witness is missing (the **instances** owed), and a state that owes none is
*discharged*. You asked whether `(open R)` should instead be emitted as a fact, `(open R <justification>)`. Read as
facts, today's `(open R)` would be the only facts that **(1) duplicate** (two debts of one relation are one tuple)
and **(2) can be removed** (a larger state pays what a smaller one owes, while every other fact a state holds,
every larger state holds). The survey found one justification that ends (1), the instance itself (the rule, its
activator's arguments, its trigger's bindings), and none that ends (2). It offers four options, (a) to (d), and
recommends (b). Your second instruction (*"instead of emitting (open ?R) emit (open (?R ?a ?b)) - the actual
hypothesis"*) stands beside (d): the open facts are the hypotheses themselves, `(open F J)`, moved out of *open*
per node as a state decides them.

Recorded in [`open_questions.md` § Q-M1f.390](../../../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact)
(**put, not ruled**); the page to rule from is [`design.md` § 5](../../../p1f.21_verdicts_as_rules/design.md#5-the-question-and-a-recommendation);
your form is [`open_facts.md` § 4](../../../p1f.21_verdicts_as_rules/open_facts.md#4-open-f-the-hypothesis-itself) and
[§ 5](../../../p1f.21_verdicts_as_rules/open_facts.md#5-the-move-open-held-not).

Every run below is today's binary, `build/review-bin/ein-head-b5fb8d70` (master `b5fb8d70`,
[`runs/ein-version.txt`](runs/ein-version.txt)).

## 2. What is so today

### 2.1 The three programs, and the counts the ledger gives, re-taken

| program | from | root owes | H (`ein show hyps`) | `(open R)` tuples at root | instances as terms (J3) | `(open F J)` pairs, F open |
|---|---|---:|---:|---:|---:|---:|
| [`seats.ein`](programs/seats.ein) | `examples/lattice/05_owed_seats_bounded.ein` | 4 | 4 | 1 | 4 | 8, over 4 F |
| the state {(seats Ann C1)} ([`seats1.ein`](programs/seats1.ein), [`seats-AnnC1.saved.ein`](programs/seats-AnnC1.saved.ein)) | seats, one commitment | 2 | 3 | 1 | 2 | 4, over 3 F |
| [`zebra2-obligations.ein`](programs/zebra2-obligations.ein) | `examples/zebra2-obligations.ein` | 36 | 56 | 5 | 36 | 112, over 56 F (68 more payments already denied) |
| the node z1 = {(color-loc Green House-5)} ([`zebra2-obligations-z1.ein`](programs/zebra2-obligations-z1.ein)) | zebra2, one commitment | 26 | 36 | 4 | 26 | 72, over 36 F (58 denied) |
| [`total-owed.ein`](programs/total-owed.ein) | `tests/stdlib/algebra/23_total_owed.ein` (`:no-search`) | 1 | 0 (`:no-search`) | 1 | 1 | — |
| [`total-owed-search.ein`](programs/total-owed-search.ein) | the same, its `(solve :no-search …)` removed | 1 | 2 | 1 | 1 | 2, over 2 F |

Where each number is read:
- *root owes*: the `∅` row of each `ein solve` ([`seats.solve.txt`](runs/seats.solve.txt),
  [`seats1.no-search.solve.txt`](runs/seats1.no-search.solve.txt),
  [`zebra2-obligations.solve.txt`](runs/zebra2-obligations.solve.txt),
  [`zebra2-obligations-z1.no-search.solve.txt`](runs/zebra2-obligations-z1.no-search.solve.txt),
  [`total-owed.solve.txt`](runs/total-owed.solve.txt), [`total-owed-search.solve.txt`](runs/total-owed-search.solve.txt)).
- *H*: the `note:` line of `ein show hyps` ([`seats.show-hyps.txt`](runs/seats.show-hyps.txt),
  [`seats1.show-hyps.txt`](runs/seats1.show-hyps.txt),
  [`zebra2-obligations.show-hyps.txt`](runs/zebra2-obligations.show-hyps.txt),
  [`zebra2-obligations-z1.show-hyps.txt`](runs/zebra2-obligations-z1.show-hyps.txt),
  [`total-owed.show-hyps.txt`](runs/total-owed.show-hyps.txt), [`total-owed-search.show-hyps.txt`](runs/total-owed-search.show-hyps.txt)).
- *tuples* and *J3*: [`seats.owe-counts.md`](runs/seats.owe-counts.md),
  [`zebra2-obligations.owe-counts.md`](runs/zebra2-obligations.owe-counts.md).
- *pairs*: [`seats.open-F-J.root.md`](runs/seats.open-F-J.root.md), [`seats1.open-F-J.md`](runs/seats1.open-F-J.md),
  [`zebra2-obligations.open-F-J.root.md`](runs/zebra2-obligations.open-F-J.root.md),
  [`zebra2-obligations-z1.open-F-J.md`](runs/zebra2-obligations-z1.open-F-J.md),
  [`total-owed-search.open-F-J.root.md`](runs/total-owed-search.open-F-J.root.md). Each pairs an owed instance
  with the `hyp` lines the obligations rung wrote for its payments, every pairing checked.

`open_facts.md` § 4's numbers all hold: seats 4 debts and 4 candidates; zebra2 36 debts and 56 candidates; at
{(seats Ann C1)} 2 debts (Bob's and C2's) and 3 candidates neither held nor denied. Not given in the ledger: at
that state the user's form holds **4** `(open F J)` facts, since `(seats Bob C2)` would pay both debts.

### 2.2 The verdict is counted, never held; its events and its counts

An obligation never fires. Its matches are `owe` lines, root's twice (once as root's pass after H is built, once
as the node of the empty commitment). Seats' root, from [`seats.events.md`](runs/seats.events.md) (every event as a
table) and [`seats.owe-events.md`](runs/seats.owe-events.md) (the owe lines node by node):

<!-- output -->
```text
| 22 | owe | `total-owed` [seats is-a] ?R=seats · ?isa=is-a · ?A=Guest · ?B=Chair · ?a=Ann — *seats owes Ann a Chair.* |
| 31 | enter | layer 0 · ∅ · **owing** · owes 4 |
| 35 | enter | layer 1 · {(seats Ann C1)} · **owing** · owes 2 |
| 46 | enter | layer 2 · {(seats Ann C1), (seats Bob C2)} · **in-r** · owes 0 |
```

`ein saturate --dump` lists no obligation rule and no derived fact at root
([`seats.saturate-dump.txt`](runs/seats.saturate-dump.txt): `;; Rules (38)`, neither `total-owed` nor
`surjective-owed` among them).

**(1) duplicates, re-taken over every node a run enters.** The owe lines carry the relation, the activator and the
bindings; the tables count how many distinct tuples the debts would be under each justification:

| program | nodes entered | nodes where `(open R)` keeps the count | J1, the activator | J3, the instance |
|---|---:|---:|---:|---:|
| seats ([`seats.owe-counts.md`](runs/seats.owe-counts.md)) | 10 | 5 | 9 | **10** |
| zebra2 ([`zebra2-obligations.owe-counts.md`](runs/zebra2-obligations.owe-counts.md)) | 120 | 64 | 70 | **120** |

(64 of zebra2's 120 nodes owe nothing, where every count is 0.) The zebra2 stream is 30 MB, so it is kept
gzipped ([`zebra2-obligations.events.jsonl.gz`](runs/zebra2-obligations.events.jsonl.gz)) beside its run, load,
hyp, rung, owe, enter and region lines ([`.jsonl`](runs/zebra2-obligations.events.owe-enter-hyp-rung.jsonl),
[`.md`](runs/zebra2-obligations.events.owe-enter-hyp-rung.md)) and its owe lines node by node
([`zebra2-obligations.owe-events.md`](runs/zebra2-obligations.owe-events.md)).

**(2) removal, on the whole cube of seats.** `ein oracle --dump --states` saturates all 16 nodes
([`seats.oracle.txt`](runs/seats.oracle.txt), [`seats.oracle/region.md`](runs/seats.oracle/region.md)); root owes 4,
each node of layer 1 owes 2, the two states of R owe 0, and every node's facts are its parent's and more. Drawn:
[`renders/seats.region.svg`](renders/seats.region.svg) (the cube, with the 10 nodes `ein solve` visited marked) and
[`renders/seats.lattice.svg`](renders/seats.lattice.svg) (those 10 nodes as a DAG). The tree route's visit order is
[`seats.dump-lattice/nodes.md`](runs/seats.dump-lattice/nodes.md); zebra2's 120 nodes are
[`zebra2-obligations.dump-lattice/nodes.md`](runs/zebra2-obligations.dump-lattice/nodes.md).

**What the summary carries** ([`summaries.owed.md`](runs/summaries.owed.md), from the six `--json-summary` files):
`root.owes`, `root.owed` (the `:why` sentences), `region.root_owes`, `region.root_owed`, and the generator's counts.
No summary, dump or read-out prints an instance as a term today.

### 2.3 One owing state: {(seats Ann C1)}

The state is taken three ways, and all three say the same:
- **the node of the run**: `enter` n=35 above, owes 2; [`seats.dump-lattice/nodes.json`](runs/seats.dump-lattice/nodes.json) node 2;
- **the saved node**, [`programs/seats-AnnC1.saved.ein`](programs/seats-AnnC1.saved.ein): the engine's saved file
  of a state of R ([`seats.dump-answer/models/01.ein`](runs/seats.dump-answer/models/01.ein)) with its last block
  cut to `(commit (seats Ann C1))` by hand, since a `--dump-answer` writes a file for a state of R and for the
  owing frontier of an empty R, and this node is neither. It loads as that node;
- **the commitment written as a fact**, [`programs/seats1.ein`](programs/seats1.ein), so that root is that state
  (open_facts.md § 4's run).

`ein solve programs/seats-AnnC1.saved.ein --no-search -p`
([`seats-AnnC1.saved.no-search.solve.txt`](runs/seats-AnnC1.saved.no-search.solve.txt)):

<!-- output -->
```text
  ∅                 not in R — root owes 2 instances
    owed            seats owes Bob a Chair.
    owed            seats owes C2 a preimage in Guest.
  …
  universe          3 candidates, none visited: R was not searched
    commit          1 candidate of H, the node this saved file names: R is the part of its program's R above it, and the universe is the rest of H — (seats Ann C1)
  …
final-state facts (beyond the program as loaded: root's derivations, the commitment and what it derives; 1 fact):
  (seats Ann C1)
```

Its events ([`seats-AnnC1.saved.no-search.events.md`](runs/seats-AnnC1.saved.no-search.events.md)): four owe lines
for the program's root (H is the program's), then two for the node. Its `--dump-answer`
([`seats-AnnC1.saved.no-search.dump-answer/`](runs/seats-AnnC1.saved.no-search.dump-answer/answer.txt)) and
`seats1`'s ([`seats1.no-search.dump-answer/root.ein`](runs/seats1.no-search.dump-answer/root.ein)) save root:
the program and its modules, and the choices. **A saved file holds no fact of the state**, only the choices
(`(commit …)`), so nothing owed is written in one. The engine's own dump of an owing state, the owing frontier of
an empty R, is `total-owed`'s ([`total-owed.dump-answer/owing/01.ein`](runs/total-owed.dump-answer/owing/01.ein),
listed in [`index.json`](runs/total-owed.dump-answer/index.json) with its path and its empty commitment; `ein
test` holds its claim, [`total-owed.test.txt`](runs/total-owed.test.txt)).

`ein show hyps`: of `seats1.ein`, the three candidates the state leaves open
([`seats1.show-hyps.txt`](runs/seats1.show-hyps.txt)); of the saved node, the program's four, the node's choice
among them ([`seats-AnnC1.saved.show-hyps.txt`](runs/seats-AnnC1.saved.show-hyps.txt); Q-M1f.415's default (a):
*both read the program's H*).

The KB, drawn by `ein render kb` (`fdp`): [`renders/seats-AnnC1.saved.kb.svg`](renders/seats-AnnC1.saved.kb.svg)
(the commitment dashed), against root with `--since`
([`renders/seats-AnnC1.saved.kb.since-root.svg`](renders/seats-AnnC1.saved.kb.since-root.svg)); seats1's, the
commitment solid ([`renders/seats1.kb.svg`](renders/seats1.kb.svg)); root's ([`renders/seats.kb.svg`](renders/seats.kb.svg)).
No debt has a node in any of them.

## 3. The options

At the state {(seats Ann C1)}, what each would hold and print. Every file named `.MOCK.` is hand-edited from the
binary's real output, the file it was edited from and the edits written in its own first lines; each `.MOCK.diff`
is `diff -u` of the real output against the mock. Today's three surfaces, as the mocks start from:
[`AnnC1.readout.today.txt`](runs/AnnC1.readout.today.txt), [`AnnC1.state.today.txt`](runs/AnnC1.state.today.txt),
[`AnnC1.show-hyps.today.txt`](runs/AnnC1.show-hyps.today.txt).

| | the state holds | the read-out prints | `ein show hyps` prints |
|---|---|---|---|
| **(a)** as today | `(seats Ann C1)`; the 2 debts are the one tuple `open(seats)` ([mock](runs/AnnC1.state.a.MOCK.txt)) | today's ([real](runs/AnnC1.readout.today.txt)) | the 3 candidates ([mock: unchanged](runs/AnnC1.show-hyps.a-b-c-d.MOCK.txt)) |
| **(b)** the verdict with its instance, not held | the same; beside it `open(seats, j)` twice, held by nothing ([mock](runs/AnnC1.state.b.MOCK.txt)) | each `owed` row with its term ([mock](runs/AnnC1.readout.b.MOCK.txt), [diff](runs/AnnC1.readout.b.MOCK.diff)); the `owe` line gains `term` ([mock](runs/AnnC1.owe-events.b.MOCK.jsonl)); the dump's index gains the terms ([mock](runs/total-owed.dump-answer.index.b.MOCK.json), [diff](runs/total-owed.dump-answer.index.b.MOCK.diff)) | unchanged |
| **(c)** requirement and payment as facts | `(seats Ann C1)`, 4 `due`, 2 `paid`, all inherited upward ([mock](runs/AnnC1.state.c.MOCK.txt)) | a `requirements` row, the `due`/`paid` facts under `-p` ([mock](runs/AnnC1.readout.c.MOCK.txt), [diff](runs/AnnC1.readout.c.MOCK.diff)) | unchanged |
| **(d)** `(open R j)` held, not inherited | `(seats Ann C1)`, and in the node's own part 2 `(open seats j)` ([mock](runs/AnnC1.state.d.MOCK.txt)) | a second listing under `-p`, the node's verdict part ([mock](runs/AnnC1.readout.d.MOCK.txt), [diff](runs/AnnC1.readout.d.MOCK.diff)) | unchanged |
| **your form** `(open F J)` | `(seats Ann C1)`, and in the node's own part 4 `(open F J)` ([mock](runs/AnnC1.state.user-open-F-J.MOCK.txt)) | an `open` row, and the open facts under `-p` ([mock](runs/AnnC1.readout.user-open-F-J.MOCK.txt), [diff](runs/AnnC1.readout.user-open-F-J.MOCK.diff)) | the 4 open facts ([mock](runs/AnnC1.show-hyps.user-open-F-J.MOCK.txt), [diff](runs/AnnC1.show-hyps.user-open-F-J.MOCK.diff)) |

Over the whole cube of seats, node by node, the kernel's `owes` against each option's count:
[`seats.options-per-node.md`](runs/seats.options-per-node.md). Root 4 debts: (a) 1, (b)/(d) 4, (c) 4 due · 0 paid,
your form 8 pairs over 4 F; {(seats Ann C1)}: 1, 2, 4 · 2, 4 over 3; the two states of R: 0 everywhere and
`(discharged)` held under (c).

**(a) As today.** Nothing is built; the pages state the verdicts as rules and say of `(open R)` what your first
instruction says. Duplicates stay: 4 debts are 1 tuple at seats' root, 36 are 5 at zebra2's.

**(b) The verdict carries its justification and stays a verdict.** The pass's result is the set of `open(r, j)`,
j the instance (J3), true of the state and held by no knowledge base. The engine already computes j and prints it in
three fields of every `owe` line ([`seats.owe-events.md`](runs/seats.owe-events.md)'s last column is j built from
those fields); printing it as one term is additive. Nothing held moves, so no licence, check, golden or state moves.
(2) stays, as *alive* has it: a verdict is read on each state.

**(c) The requirement and the payment are facts; owed is their difference.** This one runs today as ordinary rules:
[`programs/seats-emu.ein`](programs/seats-emu.ein) is seats with `design.md` Appendix A.2's four rules and
`discharged`, the kernel's obligations kept beside them.
- The emulation's unpaid `due` equals the kernel's `owes` at **16 of 16** nodes, and `(discharged)` is held exactly
  where nothing is owed ([`seats.options-per-node.md`](runs/seats.options-per-node.md), from
  [`seats-emu.oracle/region.json`](runs/seats-emu.oracle/region.json)).
- Every node's status and owes are the same as without the rules; the read-out differs in the `outside` row and
  the fact count only ([`seats.solve.vs-emu.diff`](runs/seats.solve.vs-emu.diff),
  [`seats.oracle.vs-emu.diff`](runs/seats.oracle.vs-emu.diff)): 19 → 37 facts beyond root (design.md's 35 is
  without `discharged`).
- At the state: [`seats1-emu.no-search.solve.txt`](runs/seats1-emu.no-search.solve.txt), 4 `due` and 2 `paid`; drawn,
  [`renders/seats1-emu.kb.svg`](renders/seats1-emu.kb.svg), the debts as nested facts in a context.

What a build adds: the facts made by the kernel (no rules written), two reserved relations, the listings, every
golden of a program with an obligation re-banked.

**(d) `(open R j)` a fact of the state that owes it**, written after the fixpoint into a part of the node no fork
inherits, matched by no rule. Why the part must not be inherited, run:
[`programs/seats-stored.ein`](programs/seats-stored.ein) derives the debt as an ordinary fact, and
[`seats-stored.solve.txt`](runs/seats-stored.solve.txt) says *stale facts — 15 of the cube's 16 nodes hold one*;
under `-p` both states of R still hold `(owing seats …)` for Ann and Bob
([`seats-stored.solve-p.txt`](runs/seats-stored.solve-p.txt)). The KB with the two facts, drawn by the engine from a
stand-in relation `open-debt` ([`programs/seats1-open-R-j.standin.ein`](programs/seats1-open-R-j.standin.ein),
[`renders/seats1-open-R-j.standin.kb.svg`](renders/seats1-open-R-j.standin.kb.svg)), and renamed to `open` by hand:
[`renders/seats-AnnC1.kb.d-open-R-j.MOCK.svg`](renders/seats-AnnC1.kb.d-open-R-j.MOCK.svg).

**Your form, `(open F J)`.** The open facts are H itself, each with the debt it would pay; a state holds
`(open F J)` where F is a candidate it holds neither of nor denies and J is a debt it has not paid; moved per node
after its fixpoint, never inherited (open_facts.md § 5). It is (d) with the hypothesis in the fact and rules
reading it (§ 6 there). At {(seats Ann C1)}:

<!-- output -->
```text
(open (seats Ann C2) (surjective-owed seats is-a Guest Chair C2))
(open (seats Bob C1) (total-owed seats is-a Guest Chair Bob))
(open (seats Bob C2) (total-owed seats is-a Guest Chair Bob))
(open (seats Bob C2) (surjective-owed seats is-a Guest Chair C2))
```

(a mock; the pairs are the run's, [`seats1.open-F-J.md`](runs/seats1.open-F-J.md)). Drawn by the engine from a
stand-in relation `open-fact` ([`programs/seats1-open-F-J.standin.ein`](programs/seats1-open-F-J.standin.ein),
[`renders/seats1-open-F-J.standin.kb.svg`](renders/seats1-open-F-J.standin.kb.svg); it loads and answers as seats1,
[`seats1-open-F-J.standin.no-search.solve.txt`](runs/seats1-open-F-J.standin.no-search.solve.txt)), then renamed by
hand: **[`renders/seats-AnnC1.kb.user-open-F-J.MOCK.svg`](renders/seats-AnnC1.kb.user-open-F-J.MOCK.svg)**. The
four `open` arrows (red, bold) go from each candidate to the debt it would pay, both nested facts in the `open`
context; `(seats Bob C2)` has two. The counts: 8 at seats' root, 112 at zebra2's (and 68 payments denied, which
have none), 72 at z1. The candidates and the debts are as many only by chance: 4 and 4 at seats' root, 56 and 36
at zebra2's, 3 and 2 at {(seats Ann C1)}; a debt is a disjunction of open facts, not one of them.

## 4. The default

**(b)**, recommended in [`design.md` § 5](../../../p1f.21_verdicts_as_rules/design.md#the-recommendation-b): it is all of
the *maybe* that can be had (the duplicate goes; the removal is what a verdict is, as *alive* is), it moves no
licence, check, golden or state, and it does not close (c). The ledger's own strongest argument against it: the
debt stays outside the knowledge base, so no rule and no claim reads one; (c) makes owed and discharged ordinary
rewriting, at the price of its build. Your second instruction is not a ruling among the four: it takes (d) further,
and S1f.21.4 is to put it again on programs.

## 5. Files

**`programs/`**
- [`seats.ein`](programs/seats.ein) — copy of `examples/lattice/05_owed_seats_bounded.ein`.
- [`seats1.ein`](programs/seats1.ein) — seats with `(seats Ann C1)` written as a fact: root is that state.
- [`seats-AnnC1.saved.ein`](programs/seats-AnnC1.saved.ein) — the engine's saved file of a state of R, its `(commit …)` cut by hand to `(seats Ann C1)`: the owing node.
- [`seats-emu.ein`](programs/seats-emu.ein) — option (c) emulated: seats with `due`, `paid`, `discharged` as ordinary rules (design.md A.2).
- [`seats1-emu.ein`](programs/seats1-emu.ein) — the same with `(seats Ann C1)` written.
- [`seats-stored.ein`](programs/seats-stored.ein) — the debt as a fact the agenda derives (design.md § 2.3 (iii)): stale.
- [`seats1-open-F-J.standin.ein`](programs/seats1-open-F-J.standin.ein) — seats1 with your four open facts under the stand-in `open-fact`.
- [`seats1-open-R-j.standin.ein`](programs/seats1-open-R-j.standin.ein) — seats1 with (d)'s two facts under the stand-in `open-debt`.
- [`zebra2-obligations.ein`](programs/zebra2-obligations.ein) — copy of `examples/zebra2-obligations.ein`.
- [`zebra2-obligations-z1.ein`](programs/zebra2-obligations-z1.ein) — the same with `(color-loc Green House-5)` written: the node z1.
- [`total-owed.ein`](programs/total-owed.ein) — copy of `tests/stdlib/algebra/23_total_owed.ein`.
- [`total-owed-search.ein`](programs/total-owed-search.ein) — the same, its `(solve :no-search …)` removed.

**`runs/`** (`.txt` is stdout and stderr with `[exit N]` last; `.jsonl` events; `.md` a table rendered from the file it names)
- [`ein-version.txt`](runs/ein-version.txt) — the binary.
- seats: [`solve`](runs/seats.solve.txt), [`show-hyps`](runs/seats.show-hyps.txt), [`events.jsonl`](runs/seats.events.jsonl) / [`.md`](runs/seats.events.md), [`owe-events.md`](runs/seats.owe-events.md), [`owe-counts.md`](runs/seats.owe-counts.md), [`open-F-J.root.md`](runs/seats.open-F-J.root.md), [`summary.json`](runs/seats.summary.json), [`trace.md`](runs/seats.trace.md), [`saturate-dump`](runs/seats.saturate-dump.txt), [`oracle.txt`](runs/seats.oracle.txt) and [`oracle/`](runs/seats.oracle/region.md) (`region.json`, `region.md`), [`dump-lattice/`](runs/seats.dump-lattice/nodes.md) (`nodes.json`, `nodes.md`), [`dump-answer/`](runs/seats.dump-answer/answer.txt) (`answer.txt`, `answer.json`, `index.json`, `nogoods.json`, `models/01.ein`, `models/02.ein`, `forms/print-final-state.txt`), [`options-per-node.md`](runs/seats.options-per-node.md).
- seats1: [`solve`](runs/seats1.solve.txt) (R above the state), [`events.jsonl`](runs/seats1.events.jsonl) / [`.md`](runs/seats1.events.md), [`summary.json`](runs/seats1.summary.json), [`show-hyps`](runs/seats1.show-hyps.txt), [`saturate-dump`](runs/seats1.saturate-dump.txt), [`no-search.solve`](runs/seats1.no-search.solve.txt), [`no-search.events.jsonl`](runs/seats1.no-search.events.jsonl) / [`.md`](runs/seats1.no-search.events.md), [`no-search.summary.json`](runs/seats1.no-search.summary.json), [`no-search.dump-answer/`](runs/seats1.no-search.dump-answer/answer.txt) (`answer.txt`, `answer.json`, `root.ein`, `forms/print-final-state.txt`), [`open-F-J.md`](runs/seats1.open-F-J.md).
- the saved node: [`solve`](runs/seats-AnnC1.saved.solve.txt) (R above it), [`no-search.solve`](runs/seats-AnnC1.saved.no-search.solve.txt), [`show-hyps`](runs/seats-AnnC1.saved.show-hyps.txt), [`no-search.events.jsonl`](runs/seats-AnnC1.saved.no-search.events.jsonl) / [`.md`](runs/seats-AnnC1.saved.no-search.events.md), [`no-search.summary.json`](runs/seats-AnnC1.saved.no-search.summary.json), [`no-search.dump-answer/`](runs/seats-AnnC1.saved.no-search.dump-answer/answer.txt) (`answer.txt`, `answer.json`, `root.ein`, `forms/print-final-state.txt`).
- option (c): [`seats-emu.solve`](runs/seats-emu.solve.txt), [`seats-emu.oracle.txt`](runs/seats-emu.oracle.txt) and [`seats-emu.oracle/`](runs/seats-emu.oracle/region.md) (`region.json`, `region.md`), [`seats.solve.vs-emu.diff`](runs/seats.solve.vs-emu.diff), [`seats.oracle.vs-emu.diff`](runs/seats.oracle.vs-emu.diff), [`seats1-emu.no-search.solve`](runs/seats1-emu.no-search.solve.txt).
- stale: [`seats-stored.solve`](runs/seats-stored.solve.txt), [`seats-stored.solve-p`](runs/seats-stored.solve-p.txt).
- stand-ins: [`seats1-open-F-J.standin.no-search.solve`](runs/seats1-open-F-J.standin.no-search.solve.txt), [`seats1-open-R-j.standin.no-search.solve`](runs/seats1-open-R-j.standin.no-search.solve.txt).
- zebra2: [`solve`](runs/zebra2-obligations.solve.txt), [`show-hyps`](runs/zebra2-obligations.show-hyps.txt), [`events.jsonl.gz`](runs/zebra2-obligations.events.jsonl.gz) (whole stream), [`events.owe-enter-hyp-rung.jsonl`](runs/zebra2-obligations.events.owe-enter-hyp-rung.jsonl) / [`.md`](runs/zebra2-obligations.events.owe-enter-hyp-rung.md), [`owe-events.md`](runs/zebra2-obligations.owe-events.md), [`owe-counts.md`](runs/zebra2-obligations.owe-counts.md), [`open-F-J.root.md`](runs/zebra2-obligations.open-F-J.root.md), [`summary.json`](runs/zebra2-obligations.summary.json), [`dump-lattice/`](runs/zebra2-obligations.dump-lattice/nodes.md) (`nodes.json`, `nodes.md`).
- z1: [`no-search.solve`](runs/zebra2-obligations-z1.no-search.solve.txt), [`show-hyps`](runs/zebra2-obligations-z1.show-hyps.txt), [`no-search.events.jsonl`](runs/zebra2-obligations-z1.no-search.events.jsonl) / [`.md`](runs/zebra2-obligations-z1.no-search.events.md), [`open-F-J.md`](runs/zebra2-obligations-z1.open-F-J.md).
- total-owed: [`solve`](runs/total-owed.solve.txt), [`show-hyps`](runs/total-owed.show-hyps.txt), [`test`](runs/total-owed.test.txt), [`events.jsonl`](runs/total-owed.events.jsonl) / [`.md`](runs/total-owed.events.md), [`owe-events.md`](runs/total-owed.owe-events.md), [`owe-counts.md`](runs/total-owed.owe-counts.md), [`summary.json`](runs/total-owed.summary.json), [`dump-answer/`](runs/total-owed.dump-answer/answer.txt) (`answer.txt`, `answer.json`, `index.json`, `nogoods.json`, `owing/01.ein`, `forms/print-final-state.txt`).
- total-owed-search: [`solve`](runs/total-owed-search.solve.txt), [`show-hyps`](runs/total-owed-search.show-hyps.txt), [`events.jsonl`](runs/total-owed-search.events.jsonl) / [`.md`](runs/total-owed-search.events.md), [`owe-events.md`](runs/total-owed-search.owe-events.md), [`owe-counts.md`](runs/total-owed-search.owe-counts.md), [`open-F-J.root.md`](runs/total-owed-search.open-F-J.root.md), [`summary.json`](runs/total-owed-search.summary.json).
- [`summaries.owed.md`](runs/summaries.owed.md) — the debt keys of the six summaries.
- today at the state: [`AnnC1.readout.today.txt`](runs/AnnC1.readout.today.txt), [`AnnC1.state.today.txt`](runs/AnnC1.state.today.txt), [`AnnC1.show-hyps.today.txt`](runs/AnnC1.show-hyps.today.txt).
- mocks: [`AnnC1.state.a.MOCK.txt`](runs/AnnC1.state.a.MOCK.txt), [`AnnC1.state.b.MOCK.txt`](runs/AnnC1.state.b.MOCK.txt), [`AnnC1.state.c.MOCK.txt`](runs/AnnC1.state.c.MOCK.txt), [`AnnC1.state.d.MOCK.txt`](runs/AnnC1.state.d.MOCK.txt), [`AnnC1.state.user-open-F-J.MOCK.txt`](runs/AnnC1.state.user-open-F-J.MOCK.txt); [`AnnC1.readout.b.MOCK.txt`](runs/AnnC1.readout.b.MOCK.txt) / [`.diff`](runs/AnnC1.readout.b.MOCK.diff), [`AnnC1.readout.c.MOCK.txt`](runs/AnnC1.readout.c.MOCK.txt) / [`.diff`](runs/AnnC1.readout.c.MOCK.diff), [`AnnC1.readout.d.MOCK.txt`](runs/AnnC1.readout.d.MOCK.txt) / [`.diff`](runs/AnnC1.readout.d.MOCK.diff), [`AnnC1.readout.user-open-F-J.MOCK.txt`](runs/AnnC1.readout.user-open-F-J.MOCK.txt) / [`.diff`](runs/AnnC1.readout.user-open-F-J.MOCK.diff); [`AnnC1.show-hyps.a-b-c-d.MOCK.txt`](runs/AnnC1.show-hyps.a-b-c-d.MOCK.txt), [`AnnC1.show-hyps.user-open-F-J.MOCK.txt`](runs/AnnC1.show-hyps.user-open-F-J.MOCK.txt) / [`.diff`](runs/AnnC1.show-hyps.user-open-F-J.MOCK.diff); [`AnnC1.owe-events.b.MOCK.jsonl`](runs/AnnC1.owe-events.b.MOCK.jsonl); [`total-owed.dump-answer.index.b.MOCK.json`](runs/total-owed.dump-answer.index.b.MOCK.json) / [`.diff`](runs/total-owed.dump-answer.index.b.MOCK.diff).

**`renders/`** (each `.dot` the engine's, drawn as the `.svg` beside it; KB views by `fdp`, the rest by `dot`)
- [`seats.kb`](renders/seats.kb.svg) ([dot](renders/seats.kb.dot)) — root.
- [`seats1.kb`](renders/seats1.kb.svg) ([dot](renders/seats1.kb.dot)) — the state, the commitment written.
- [`seats-AnnC1.saved.kb`](renders/seats-AnnC1.saved.kb.svg) ([dot](renders/seats-AnnC1.saved.kb.dot)) — the saved node, the commitment dashed.
- [`seats-AnnC1.saved.kb.since-root`](renders/seats-AnnC1.saved.kb.since-root.svg) ([dot](renders/seats-AnnC1.saved.kb.since-root.dot)) — the same, what it adds to root bold.
- [`seats1-emu.kb`](renders/seats1-emu.kb.svg) ([dot](renders/seats1-emu.kb.dot)) — option (c) emulated at the state.
- [`seats1-open-R-j.standin.kb`](renders/seats1-open-R-j.standin.kb.svg) ([dot](renders/seats1-open-R-j.standin.kb.dot)) — (d) under the stand-in `open-debt`.
- [`seats-AnnC1.kb.d-open-R-j.MOCK`](renders/seats-AnnC1.kb.d-open-R-j.MOCK.svg) ([dot](renders/seats-AnnC1.kb.d-open-R-j.MOCK.dot)) — the same renamed `open`: **mock**.
- [`seats1-open-F-J.standin.kb`](renders/seats1-open-F-J.standin.kb.svg) ([dot](renders/seats1-open-F-J.standin.kb.dot)) — your form under the stand-in `open-fact`.
- [`seats-AnnC1.kb.user-open-F-J.MOCK`](renders/seats-AnnC1.kb.user-open-F-J.MOCK.svg) ([dot](renders/seats-AnnC1.kb.user-open-F-J.MOCK.dot)) — the same renamed `open`: **mock**.
- [`seats.region`](renders/seats.region.svg) ([dot](renders/seats.region.dot)) — the cube, the run's 10 nodes marked.
- [`seats.lattice`](renders/seats.lattice.svg) ([dot](renders/seats.lattice.dot)) — the run's 10 nodes.

**`tools/`** — the scripts that re-take everything: [`run.sh`](tools/run.sh) (every command), [`tables.py`](tools/tables.py) (the `.md` tables), [`mocks.py`](tools/mocks.py) (the mocks), [`extra.py`](tools/extra.py) (the summaries table, the dump-index mock).

## 6. Re-run

From the repo root, with `E=build/review-bin/ein-head-b5fb8d70` (the scripts name it by its absolute path):

```sh
bash plans/m1f_hypothesis_and_documentation/review/05-verdicts-as-rules/32-Q-M1f.390/tools/run.sh
```

It runs, from the item's folder, every command below and then the three scripts and Graphviz. The engine commands, for each program `P`:

```sh
$E solve programs/P.ein
$E show hyps programs/P.ein
$E solve programs/P.ein --events runs/P.events.jsonl --json-summary runs/P.summary.json
$E solve programs/seats.ein --dump-lattice runs/seats.dump-lattice          # and zebra2-obligations
$E solve programs/seats1.ein --no-search -p                                 # and --events, --json-summary, --dump-answer
$E solve programs/seats-AnnC1.saved.ein --no-search -p                     # and --events, --json-summary, --dump-answer
$E solve programs/seats.ein --dump-answer runs/seats.dump-answer -p
$E solve programs/total-owed.ein --dump-answer runs/total-owed.dump-answer -p
$E test programs/total-owed.ein
$E solve programs/seats.ein --trace runs/seats.trace.md
$E oracle programs/seats.ein --dump runs/seats.oracle --states              # and seats-emu.ein
$E saturate programs/seats.ein --dump                                      # and seats1.ein
$E solve programs/seats-stored.ein [-p]
$E solve programs/seats1-open-F-J.standin.ein --no-search -p               # and seats1-open-R-j.standin.ein
$E solve programs/zebra2-obligations-z1.ein --no-search [--events …]
$E render kb programs/seats1.ein                                           # seats, the saved node (and --since programs/seats.ein), seats1-emu, both stand-ins
$E render region runs/seats.oracle --visited runs/seats.dump-lattice
$E render lattice programs/seats.ein --view full
fdp -Tsvg renders/X.kb.dot -o renders/X.kb.svg                              # dot -Tsvg for region and lattice
```

## 7. Found while building

- **`design.md` § 4.4 and § 4.5 read a state file that lists facts; today's does not.** Their (d) column says *a
  dumped owing state then holds `(open R j)` lines, and a file that writes a debt writes a stale one: the loader
  refuses them or drops them*. That was written against the state file before save and load were merged. Today's
  saved file (`ein-saved/1`) holds the program, its modules and the choices, `(commit …)`, and no fact of the
  state ([`seats.dump-answer/models/01.ein`](runs/seats.dump-answer/models/01.ein) ends `(commit (seats Ann C1)
  (seats Bob C2))`). So under (d) or your form a saved file need not change at all, and the stale debt in a file
  does not arise; what would list open facts is `-p` and the forms, as the mocks show.
- **The owing node has no saved file of the engine's own.** `--dump-answer` writes one for each state of R and,
  where R is empty, for the owing frontier ([`total-owed.dump-answer/owing/01.ein`](runs/total-owed.dump-answer/owing/01.ein));
  {(seats Ann C1)}, an owing node below a non-empty R, gets none. The one here is cut by hand from a model file and
  loads as that node ([`seats-AnnC1.saved.solve.txt`](runs/seats-AnnC1.saved.solve.txt): *commit 1 candidate of H,
  the node this saved file names*).
- Every count the ledger gives re-took exactly: seats 4/4, zebra2 56/36 and its 180 payments (56 emitted, 56 seen
  again, 68 denied), {(seats Ann C1)} 3 candidates against 2 debts, z1 26 debts, the tuples per node (seats 5/9/10
  of 10; zebra2 64/70/120 of 120), and (c)'s unpaid against `owes` (16 of 16).

## 8. Your call

- **(a)** as today: the verdict counted, nothing built; the pages say why `(open R)` is no fact.
- **(b)** the verdict carries the instance as its justification and stays a verdict; at most a term printed on
  `owe` and in the dump. **Default.**
- **(c)** the requirement and the payment as facts, *owed* their difference, `(discharged)` a fact; runs today as
  rules, a build makes them the kernel's.
- **(d)** `(open R j)` held by the state that owes it, in a part of the node no fork inherits, read by no rule.
- **your form** `(open F J)`: (d) with the hypothesis in the fact, H held under `open` and moved per node, rules
  reading it in a stratum of their own; S1f.21.4 is to put it on programs.
