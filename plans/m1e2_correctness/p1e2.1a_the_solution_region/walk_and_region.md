# The walk and the region: why they differ — 2026-10-16

**For:** [`solve_flags.md`](solve_flags.md)'s third question, route 1's budget.
The user, 2026-10-16:

> 3 - explain first the difference between walk and oracle/solve; then
> analyse and report the reasons why they differ; because this conflicts with
> my assumption that thay are equivalent

```sh
cargo run --release -p ein-infer --example walk_vs_region -- --jobs 12 > rows.jsonl   # 12 min 32 s
WVR_SHOW=1 cargo run --release -p ein-infer --example walk_vs_region -- FILE          # the states, by name
```

Taken 2026-10-16 on `90ca5418` plus the example, release build.
[`walk_vs_region.rs`](../../../ein.rs/crates/ein-infer/examples/walk_vs_region.rs)
takes every program whose R is known: route 1 at 2²⁰ visits (172 monotone
programs), or the oracle at `n ≤ 16` for the 13 the static check flags. It
runs the walk six ways, and puts every state the walk records, and every
commitment it enters, on R. A state is compared by its **closure**, the
candidates it holds, which in a monotone program is the state.

| variant | the walk |
|---|---|
| `solve` | today's `ein solve`: `-n 1`, `-m 5`, the program's config |
| `e` | `ein solve -e` |
| `e_m` | `-e -m n`: no depth cap |
| `e_m_f` | and `enable-forced-positive` off: no promotion into root |
| `e_m_f_k` | and the lookahead kill cache off (`-K`) |
| `e_m_f_k_w` | and the singleton writeback off: **the walk writes nothing into root or a node** |

---

## 1. The short answer

They are two different computations, and one assumption under the design is
right while the other is not.

- **What is right: the walk is a search for one line of R.** On a monotone
  program, an untruncated walk that writes nothing and whose lookahead sees
  every death records exactly the states of **R's maximal line** (§ 3).
  Measured, the walk without a depth cap gives the maximal line on **156 of
  172** monotone programs.
- **What is not: the walk computes R.** It never did. R is every alive,
  discharged commitment. The walk looks for *complete* ones, never records ∅
  while a candidate exists, and stops at the first complete commitment of a
  state. On 43 of the 156 where the uncapped walk gives the maximal line
  exactly, R holds more than that line: **706 517** states against **135**.

Five reasons account for every disagreement on the 172 monotone programs, and
four of them are already register rows:

| # | reason | what it does | programs where it changes the models (`-e`) |
|---|---|---|---:|
| 1 | **a different question** | the walk reports the maximal line (and under the obligations rung, roughly the minimal one), never R | by design: every program where R ≠ its maximal line |
| 2 | **the cuts** | `-n 1` stops at the first model, and `-m 5` stops at size 5 | 13 at `-e` (the depth cap); 13 more for `-n 1` |
| 3 | **a different node** | the walk saturates `root ∪ K ∪ P ∪ C`: its own `(not h)` writes `K`, its promotions `P`. R saturates `root ∪ C` | 3 (W9, W11), and it hides `zebra2-hints`' second state (reason 4 also does) |
| 4 | **`complete` is approximate** | a candidate that needs two firings to die keeps a maximal node from being recorded, and root's completeness is never asked when every singleton dies (W1) | 9 |
| 5 | **the obligations rung** | there `complete` means *nothing owed is left*, so a discharged node that is not maximal is recorded (W5, W6) | 4 |

143 + 13 + 9 + 3 + 4 = 172. The walk never calls a node alive that R calls
dead: 0 of 6.9 M enterings (§ 4.3).

**Why the costs differ** (§ 5), on `zebra.ein`: the walk enters 111
commitments, route 1 visits 270 912, and there are **27** alive states.
The walk stops at the first complete commitment of a state. Its stronger node
kills 33 of its 111 enterings that R keeps alive. Route 1 has to count every
commitment, because k_c and the merged form are defined over commitments.

## 2. What each computes

| | the oracle (route 2) and route 1 | the walk (`ein solve` today; `ein walk` after T1e2.1a.9.4) |
|---|---|---|
| **the question** | R = { C : alive ∧ discharged }, all of it | the states of nodes the generator calls **complete**: *the active rung proposes nothing here* |
| **the universe** | H, the config-free candidate list at root's fixpoint | `alive₀`: H after the lookahead's root kills and the forced-positive cascade (≠ H on 26 of the 172) |
| **a node** | `sat(root ∪ C)`, saturation alone | `sat(root ∪ K ∪ P ∪ C)`, where root holds every `(not h)` the search has written so far and every promotion |
| **what is visited** | route 1: every alive commitment and `N`. The oracle: all 2ⁿ | every alive commitment that is **not complete**, and their joins, minus what a learned clause or the kill cache rules out |
| **what is recorded** | every node, with both bits | an alive complete node: a model if discharged, an open state if it owes. Open states are reported only when there is no model |
| **when it stops** | when the down-set is exhausted, or at the budget | at `-n` models (1 by default), at `-m` (5), or when the frontier empties |
| **`exhausted`** | R is complete | the walk ran out of frontier inside `-m` ([Q-M1e.8](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)) |
| **cost** | \|alive\| + \|N\| saturations | enterings up to completion, typically far fewer |

[`solution_semantics.md` § 6](../../../docs/kernel/inference/solution_semantics.md#6-what-the-engine-implements-and-where-it-differs)
describes each difference. This page measures them.

## 3. Where they are equivalent

**On a monotone program, the walk's models are R's maximal line** when four
conditions hold:

- the walk is not cut, so `-e` with `-m ≥ n`;
- it writes nothing: no forced positive, no kill cache, no singleton
  writeback;
- its lookahead sees every death of a one-candidate extension;
- its rung is the hrule or the blind one.

The argument has four steps:

1. **Same node.** With nothing written, the walk's node is `sat(root ∪ C)`,
   which is R's. Measured: under `e_m_f_k_w`, 0 of the 6.9 M enterings
   disagree with R's status.
2. **Complete is maximal among alive.** `complete(C)` says every candidate
   outside C's closure is negated or dies in one step. With a perfect
   lookahead, that means every one-up superset of the closure is dead, so the
   closure is maximal among alive nodes.
3. **Maximal among alive is the maximal line.** *Discharged* is upward-closed
   in a monotone program. So an alive superset of a node of R is in R, and a
   discharged node is maximal in R exactly when it is maximal among alive
   nodes.
4. **Nothing maximal is skipped.** The walk expands every alive node that is
   not complete, and every subset of an alive node is alive. Walk up from ∅
   towards a maximal state T through its subsets. The first complete one, D,
   has a closure that is maximal among alive nodes and lies inside T, so it
   is T, and the walk records T at D.

So the assumption of equivalence is right **for the maximal line under
those four conditions**, and § 4 is each condition failing. It was never
right for R.

## 4. The reasons, measured

### 4.1 A different question

R is the region, and the walk reports one line of it.

- **∅ is in R on 118 of the 172**: every program whose root is alive and
  discharged. The walk records ∅ only when the generator proposes nothing
  at all ([§ 6.1](../../../docs/kernel/inference/solution_semantics.md#61-the-walk-looks-for-the-maximal-line)).
- **R equals its maximal line on 79 of the 132 non-empty regions**: there,
  the walk's question and R coincide. On the other 53, R has states the walk
  never reports by design. The largest:

  | program | k_s (R) | maximal line | k_c |
  |---|---:|---:|---:|
  | `syntax/arg-kinds` | 524 288 | 1 | 524 288 |
  | `square-*` (six) | 27 559 | 1 | 1 048 576 |
  | `type-exclusivity/pets` | 11 339 | 39 | 11 339 |
  | `syntax/constraint-scopes` | 4 096 | 1 | 4 096 |
  | `features/02_star_in_identifiers` | 728 | 1 | 32 768 |

This difference is the ruling of 2026-10-16, *the default describes R*, and
not a defect.

### 4.2 The cuts

| variant | the models equal R's maximal line on |
|---|---:|
| `solve` (`-n 1`, `-m 5`) | 130 of 172 |
| `-e` (`-m 5`) | 143 |
| `-e -m n` | **156** |

- **`-n 1`** reports one model where the maximal line has more. 22 programs
  have a maximal line of more than one state.
- **`-m 5`** truncates 13 programs: the six `square-*`, `arg-kinds`,
  `features/02`, `constraint-scopes`, `equality`, and three
  `type-exclusivity` files. On every one of them the maximal state is larger
  than 5 commitments, the walk at `-m 5` reports `k = 0` or a subset with
  `exhausted = false`, and at `-m n` it reports the maximal line exactly.
  The six `square-*` say `Contradiction`, *no model found*, about a region
  that holds **every** commitment of their cube, ∅ included.

### 4.3 A different node

The walk writes into root as it goes
([§ 6.2–6.3](../../../docs/kernel/inference/solution_semantics.md#62-what-is-recorded-is-s--k-and-the-criteria-were-checked-against-s)):

- the singleton writeback stores `(not h)` for every singleton that died;
- the kill cache stores `(not h)` for every candidate the lookahead killed;
- the cascade promotes a lone surviving candidate.

Every later node is saturated over those facts. A rule that reads a stored
negative then concludes more than R's node does. In the zebra family these
are the totality rules, which *fire only when every partner is explicitly
excluded*.

Each entering against R's status of the same commitment, summed over the
corpus:

| variant | enterings | walk says dead, R says alive | walk says alive, R says dead |
|---|---:|---:|---:|
| `solve` | 169 952 | 3 on 2 programs | 0 |
| `e` | 178 061 | **92 on 5** | 0 |
| `e_m`, `e_m_f` | 6 888 662 | 92 on 5 | 0 |
| `e_m_f_k` (no kill cache) | 6 888 723 | 90 on 4 | 0 |
| `e_m_f_k_w` (nothing written) | 6 914 575 | **0** | 0 |

The five programs, under `-e`:

| program | enterings dead by the walk's writes only | what it changes |
|---|---:|---|
| `zebra.ein` | 33 of 111 | nothing: R is one state, and the walk records it |
| `zebra2.ein`, `zebra2-obligations.ein` | 26 of 101 each | nothing, for the same reason |
| `zebra2-hints.ein` | 5 of 36 | **a maximal state of R is never recorded** (below) |
| `corpus/fuzz_findings/unsat-core-id-space.ein` (W11's witness) | 2 of 2 | **the walk says `Contradiction`, and R has two maximal states.** `-K` gives them back |

On two more programs the writes change the *recorded* state rather than an
entering. `record_node` re-saturates a state the kill cache wrote into, and
refuses it (M1e `CO-M1`): `complete-records-stale` and `alive-empty-phase1`.
`-K` gives each its maximal line. **These are W9's two witnesses, and the
register records them the other way round.** W9 calls the `-K` run wrong,
*"with the kill cache off, an unnoticed entailment makes a state a model"*,
and the default its control. Under R's node, the `-K` run is the one on the
maximal line (§ 7).

**`zebra2-hints.ein` is not in the register.** R's maximal line has two
states. The walk records one, with `exhausted = true`:

```
max   {(drink-loc Juice House-4) (drink-loc Tea House-2) (nation-loc Japanese House-5) … (smoke-loc Parliament House-5)}   13 facts, recorded
max   {(nation-loc Spaniard House-5) (pet-loc Dog House-5) (pet-loc Fox House-4) (pet-loc Snail House-3)
       (pet-loc Zebra House-1) (smoke-loc Chesterfields House-5) (smoke-loc Old_Gold House-3)}                       7 facts, missed
```

Two causes stack:

- **The walk's node.** With the writeback on, `{(nation-loc Spaniard
  House-5)}` is `dead-post` after 33 firings, with a core about where the
  Japanese can live. In R it is alive: it lies under the 7-fact state. With
  every write off, all 127 non-empty subsets of the 7-fact state are entered
  and alive.
- **`complete`.** Even then, none of the 127 is called complete, the 7-fact
  state included. A candidate whose death needs more than one firing is still
  proposed there, so the state is expanded, every extension dies, and nothing
  is recorded. That is W1's shape (§ 4.4).

What the 7-fact state is in R's terms: a partial assignment no candidate can
extend, which the program's rules do not refute. `zebra2-hints` states its
totality as a *restriction* that fires on explicit exclusions, not as an
obligation. So under the ruled definition (*alive* is decided by saturation
alone) the state is in R, and the walk's refutation of it comes from
knowledge the walk gathered from other nodes. Had the program stated
totality as an obligation (`total-owed`), the state would owe, and the same
search would call it open.

### 4.4 `complete` is approximate (W1)

`complete` asks the lookahead, which sees deaths **one firing** deep. A
maximal node with a candidate that needs two firings to die is expanded
instead of recorded. Its extensions all die, so it is never recorded. And when
every singleton dies at layer 1, the walk ends without asking whether root
itself is complete.

| programs | what the walk says | R | register |
|---|---|---|---|
| `branching/15_lookahead_two_step_on`, `/16_…_off` | one model, or `Contradiction` | two maximal states | W1 |
| `branching/07_lookahead_off` (n = 204, lookahead off in the file) | `Contradiction`, at every `-m` | 22 maximal states | none. W1's shape with `-L`, where `complete` needs every candidate asserted or negated |
| `forced-2-two`, `forced-2-off`, `fresh-name-nospawn`, `alive-set-fresh-name` | `Contradiction` | root, ∅, is R's one state | W1 at layer 0, and W4 on the last |
| `forced-2` | `Contradiction` | ∅ | W2, which falls into W1 with the cascade off |
| `zebra2-hints` | one model | two | none (§ 4.3) |

`owed-two-step` is the same at layer 0 for an open state: R is empty, its one
maximal alive node owes, and the walk says `Contradiction` where the ruled
word is `Open` (W1).

### 4.5 The obligations rung (W5, W6)

Under the obligations rung the generator proposes what pays a debt. A
discharged node is therefore complete whether or not it is maximal.

| program | walk (`-e`) | R |
|---|---|---|
| `owed-seats`, `owed-seats-renamed` (W5) | 3 models | maximal line: 1 state; minimal line: 2 |
| `uncovered`, `uncovered-knows` (W6) | the same 3 | the same |

On all four the walk's three models are **R's minimal line plus one
superset**, `{(seats Ann C1) (seats Ann C2) (seats Bob C1)}`. That superset is
on neither line, and it is W5's row: *a superset of a model, chosen by name
order*. So on these programs the walk looks for the minimal line and misses
it by one state. That is the case ruling (f) and the user's *"relevant only
in special case with no obligations"* were about.

### 4.6 What is not in these numbers

- **The 13 flagged programs.** Under `-e` the walk gives the oracle's maximal
  line on 7 of 13. What R is there is P1e2.1b's.
- **What neither route sees.** Both share the saturator (W7) and the
  generator's candidate list (W4, W6's uncovered relations), so a defect
  there is invisible to this comparison
  ([§ 6.6](../../../docs/kernel/inference/solution_semantics.md#66-what-the-oracle-does-not-check)).
- **Open states beside models.** On six programs, among them the zebra
  family, the walk finds a model and R also has maximal alive nodes that owe.
  The walk reports open states only when it has no model, so they are not
  printed. That is the read-out's choice, and S1e2.1a.5's to change.

## 5. Why the walk is cheap and route 1 is not

| program | n | route 1 visits | alive states | k_c | k_s | walk `solve` | walk `-e` | walk `-e -m n` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `zebra.ein` | 56 | 270 912 | 27 | 261 982 | 1 | 13 | 111 | 111 |
| `zebra2.ein` | 56 | 270 579 | 20 | 261 982 | 1 | 11 | 101 | 101 |
| `branching/06_lookahead_on` | 204 | 19 192 | 74 | 18 622 | 74 | 67 | 5 173 | 5 659 |
| `features/02_star_in_identifiers` | 15 | 32 768 | 728 | 32 768 | 728 | 4 943 | 4 943 | 32 704 |
| `square-bwd/floors` | 20 | 1 048 576 | 27 559 | 1 048 576 | 27 559 | 21 699 | 21 699 | 1 047 552 |
| `syntax/arg-kinds` | 19 | 524 288 | 524 288 | 524 288 | 524 288 | 16 663 | 16 663 | 524 287 |
| **corpus, monotone** | | **7 732 752** | **707 253** | 7 704 189 | 706 740 | 169 952 | 178 061 | 6 888 662 |

Three mechanisms, in order of size:

1. **The walk stops at the first complete commitment of a state.** Zebra's R
   is one state reached by 261 982 commitments. The walk records it from the
   first few that determine it and never enters the rest. Route 1 enters all
   of them because k_c counts them, and the merged form names their minimal
   generators.
2. **The walk's node is stronger.** On zebra, 33 of the 111 enterings die
   only because of what the walk wrote into root (§ 4.3). Route 1 keeps them
   alive and expands them.
3. **The lookahead and the cascade shrink the universe.** `alive₀` < n on 26
   programs, and the cascade promoted a fact on 4. Learned clauses prune what
   route 1 prunes too: route 1 never generates a superset of a dead
   commitment.

**Where nothing completes early, the walk costs what route 1 costs.** On the
`square-*` cubes every commitment is alive and none is complete before ⊤.
The uncapped walk enters 1 047 552 commitments against route 1's 1 048 576.
Its cheapness there today is the `-m 5` cut and nothing else, and the cut is
why it says *no model found*.

## 6. What this means for question 3

Route 1's budget exists because R is counted in commitments. The walk avoids
the cost in two ways:

- it answers a smaller question, one line of R and not R;
- it uses a stronger node, which under the ruled definition is a deviation
  and not an optimisation.

Neither is open to route 1 without changing what R is. So no budget makes the
default as cheap as the walk on every program, and the walk's 111 enterings
on zebra are not a price route 1 can reach by tuning.

What can change is **the unit route 1 enumerates**. Zebra has 270 454 alive
commitments but **27 alive states**. A route that visits each alive state once
would cost at most *alive states × n* saturations, and k_c would be counted
over the state lattice rather than enumerated:

- **`f(T) = 2^|T| − Σ f(S)`**, over the states S strictly below T;
- **a state's minimal generators** are the minimal transversals of its
  complements against the states below it. That is
  [P1e2.1c](../p1e2.1c_the_region_search/README.md)'s S1e2.1c.2, *the lines by
  duality*.

Its bound, against route 1's visits:

| program | route 1 visits | alive states × n, an upper bound |
|---|---:|---:|
| `zebra.ein` | 270 912 | 1 512 |
| `zebra2.ein` | 270 579 | 1 120 |
| `square-*` | 1 048 576 | 551 180 |
| `features/02` | 32 768 | 10 920 |
| `type-exclusivity/pets` | 11 351 | 170 085 |
| `syntax/arg-kinds` | 524 288 | 9 961 472 |

It wins wherever commitments collapse into few states, the zebra family most,
and it loses where every commitment is its own state.

The two answers to question 3 are therefore:

- **(A), as proposed:** route 1 enumerates commitments under a budget of 2¹⁹
  facts. Zebra's R is then described by `--region-budget`, not by default.
- **(B), as well:** a state route, which would describe zebra by default in
  about 1 500 saturations. It is S1e2.1c.2's work brought forward, and it
  needs the k_c count and the generators above.

[`solve_flags.md` § 9](solve_flags.md#9-what-needs-the-users-ruling) carries the
question.

## 7. What it leaves for the stages that follow

- **[S1e2.1a.6](s1e2.1a.6_engine_against_oracle.md)** turns this comparison
  into its test. `walk_vs_region.rs` is its first version. Two disagreements
  have no register row:
  - `zebra2-hints.ein`: W1's shape, and the walk's node (§ 4.3);
  - `branching/07_lookahead_off`: W1 under `-L` (§ 4.4).
- **[S1e2.1a.7](s1e2.1a.7_the_bundle_shows_r.md)** restates every register
  `right` block under R. **W11's witness** has `right = Contradiction, k = 0`,
  and R has two maximal states there, so the restatement moves it.
- **The walk's stronger node** is a fourth way a search write reaches an
  answer, after the three in
  [§ 6.2](../../../docs/kernel/inference/solution_semantics.md#62-what-is-recorded-is-s--k-and-the-criteria-were-checked-against-s).
  It is not a row of its own: its answer-changing witnesses are W9's two,
  W11's, and the `zebra2-hints` row above. On zebra it changes no answer and
  is a third of the walk's enterings.
- **W9 and W11 change direction** at S1e2.1a.7 if § 7 holds: their `right`
  blocks are the default's answer, and R's is the one the kill cache removes.

## 7. Two rulings meet here

**Q-M1e.7, 2026-08-28**, as `record_node`'s comment carries it:

> the criteria are evaluated on the **state**, including `K`, what the search
> itself wrote into it: the singleton writeback's `(not h)` and the lookahead
> kill cache's. Those are consequences of the state rather than bookkeeping

**R's node, 2026-10-15/16**: *alive* is decided by saturation alone, the oracle
never writes, and *root is never written* is proposed (S1e2.1a.9's acceptance
assumes it).

They give different answers on three programs: W9's two witnesses and W11's.
They give different node verdicts on four more: the zebra family and
`zebra2-hints`. **The config principle decides between them.**

- `K` is written by two `(config …)` flags, `enable-lookahead-kill-cache` and
  `enable-singleton-writeback`.
- If `K` were part of R's node, those flags would change R. Measured: they
  move 92 node verdicts, and the answer on those three programs.
- The ruling of 2026-10-15 is that no lever changes the answer.

So R's node is `sat(root ∪ C)`. Q-M1e.7's reading stays true of what the
**walk** records, which is the walk's business until S1e2.1c.5 decides its
fate. The consequences:

- W9's and W11's `right` blocks are restated at S1e2.1a.7, in the direction
  above;
- `zebra2-hints` gets a register row at S1e2.1a.6, which it needs either
  way (W1's shape);
- *root is never written* holds for R by the same argument, since a promotion
  is a write a flag turns off (`enable-forced-positive`).

This page states the consequence. It does not rule on it: the user confirms
it, or says otherwise, beside question 3.
