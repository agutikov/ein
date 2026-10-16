# `ein solve` and `ein oracle`: the interface, and which command answers which question

**Stage:** [S1e2.1a.9](s1e2.1a.9_solve_describes_the_region.md), task
T1e2.1a.9.2, second version.
**Status:** **ruled 2026-10-16, all of it**, § 7's decisions included
(§ 0.1). It was proposed the same day, after the ruling in § 0. It replaces
[`solve_flags.md`](solve_flags.md) § 5 (`ein walk`) and § 10's route-3 bullet,
and keeps every other ruling on that page. T1e2.1a.9.4 builds it. **Nothing
in the parser has moved yet.**
**How to read it:** § 1 is the contract every mode is held to. §§ 2–3 are the
reference: the commands and flags, then how each mode is computed and what it
costs. § 4 is the guide, question → command. §§ 5–7 say why two modes cannot
disagree, what moves, and what is decided here unless the user objects.

---

## 0. The ruling

The user, 2026-10-16, verbatim:

> 1. - yes, (A) now, with (B) as the first step of P1e2.1c. Next - my concern
> about `walk` subcommand. I don't want to have two different commands that
> give different and inconsistent results. I want to have a comprehensive tool
> for Region exploration under `solve` command. And it has to be optimized in
> all possible ways for every mode. If I ask to find any model - then ein has
> to reach the solution the shortest way - visiting 111 commitments, or meaybe
> even less if possible, not exploring whole cube. If I ask to find out if the
> solution is singleton - then search stops on second model, or stops when
> find that owing intersects with dead for monotone problems, or doing
> something else for non-monotone. If I request full range information - then
> do minimal required work to get it. And this is a general principle of
> optimization - do minimum required work for explicitely requested task. And
> if I request full cube oracle - only then do it. So I would prefer solve
> with subcommands + oracle command for full oracle, where walk would be
> `solve --max`. And results must be consistent: region is a region of oracle,
> maxline is part of region, any model also part of region, independent of
> monotonicity. Show me new disigned interface with explanation how to answer
> any kind of questions - one part is commands and flags reference, another is
> guide in a form of question -> command mapping.

What it settles:

- **Question 3 of [`solve_flags.md` § 9](solve_flags.md#9-what-needs-the-users-ruling)**:
  (A). Route 1 enumerates commitments under a budget of 2¹⁹ facts beyond
  root. The state route, (B) of
  [`walk_and_region.md` § 6](walk_and_region.md#6-what-this-means-for-question-3),
  is P1e2.1c's first step.
- **There is no `ein walk`.** Ruling 2 of that page is withdrawn. Today's walk
  stops being reachable when the default switches (§ 6). The sound parts of
  it survive as speed levers inside the modes.
- **One command explores R, and one computes the cube.** `ein solve` has
  modes, and `ein oracle` saturates all 2ⁿ commitments, only when asked. The
  modes are flags, as the user's `solve --max` and ruling 1's names have them.
  `--max` is spelt `--maximal`: that is ruling 1's name, and `--max` is a
  prefix of `--max-time`.
- **The answers are consistent**, independent of monotonicity: R is the
  oracle's R, the maximal line is a part of it, and any reported model is a
  member of it (§ 1).
- **Minimum work for the question asked** (§ 3). A mode stops as soon as its
  question is decided, and skips only what a licence proves cannot change the
  answer.
- **[`walk_and_region.md` § 7](walk_and_region.md#7-two-rulings-meet-here)
  follows.** If R is the oracle's R, a node is `sat(root ∪ C)` with nothing
  the search wrote into it. So W9's and W11's `right` blocks are restated at
  S1e2.1a.7, and root is never written.

### 0.1 The approval

The user, 2026-10-16, on this page, verbatim:

> All - yes, walk main principle was mining and reusing all possible
> information in favor of speed. Now we put the correctness and consistency as
> an absolute requirement and switch from searching the solution to
> exploration of the region in general. So everything you said make sense.

That is the change of principle the page follows from, in the user's words:

- **the walk's principle** was *mine and reuse all possible information, for
  speed*;
- **the principle now** is that correctness and consistency are absolute, and
  the task is to explore the region rather than to search for a solution.

Speed is still sought, but only through licences (§ 3.1), the information a
theorem proves safe to reuse. The walk's writes, its `complete()` and its
depth cap reused information no theorem covered, and that is where
[`walk_and_region.md`](walk_and_region.md)'s five reasons came from.

## 1. The contract

Four rules, held for every mode and every program. *The oracle's* means what
`ein oracle FILE` prints for the same program.

| # | rule | what it rules out today | held by |
|---|---|---|---|
| **C1** | **Every state a run prints is a member of the oracle's R.** It is printed *as* minimal or maximal only when it is on that line of the oracle's R | the walk's spurious superset on `owed-seats` (W5); a state printed as maximal while an alive extension exists | § 5's test |
| **C2** | **A claim printed as certified equals the oracle's**: k_s, k_c, a line, *unique*, *none* | `zebra2-hints` at `exhausted = true` with one of its two maximal states missing | § 5's test |
| **C3** | **A run a budget stopped prints bounds, never a different answer**: k_s ≥ …, a line ⊇ what it printed, no verdict word, `exhausted = false` | *"no model found"* for the six `square-*` regions, which hold every commitment of their cube | § 5's test, at a small budget |
| **C4** | **No lever changes what is printed**, cost counters aside. That includes *which* states `-n` prints | a lever that changes R (the config principle, 2026-10-15), or that picks a different witness | S1e2.1a.6's lever test, `jobs_invariance.rs` |

**Why it holds whether or not the program is monotone:**

1. **There is one node function.** Every mode evaluates a commitment with the
   oracle's own `sat(root ∪ C)`: root's snapshot from `region::prepare`,
   saturation alone, and no write, ever. A mode and the oracle therefore
   cannot disagree about a node. They can only visit different nodes.
2. **A mode skips a node, or concludes a line, only through a licence**
   (§ 3.1) that the program's static class proves. A program the static check
   flags gets no licence beyond the cube. Its modes are certified up to
   `--region-limit` and partial past it, and C1–C3 still hold.

## 2. Reference: the commands

| command | answers | cost | changes from today |
|---|---|---|---|
| **`ein solve FILE`** | a question about R, chosen by flags (§ 2.1) | the least the question needs (§ 3) | new default, new flags; the walk goes |
| **`ein oracle FILE`** | R by brute force: every one of the 2ⁿ commitments saturated | 2ⁿ saturations, always | new. It is today's `solve --dump-region` |
| `ein test PATH…` | the claims programs state about themselves | per claim, the cheapest mode that decides it | re-pointed |
| `ein saturate FILE` | root, the node ∅ | one saturation | unchanged |
| `ein render …` | drawings | — | `render region` reads an oracle dump and overlays a solve run |
| `ein kb …` | the `.einb` cache | — | unchanged |

### 2.1 `ein solve`

```
ein solve FILE  [WHICH]  [ASK]  [--region-budget N] [-T SECONDS] [-j JOBS]
                [presentation] [artefacts] [speed levers]

WHICH   (all of R, the default) | --minimal | --maximal
ASK     (describe, the default) | -n N | --single
```

**The two axes**, ruling 1's names, kept from `solve_flags.md` § 3:

| | **describe** (default) | **`-n N`** | **`--single`** |
|---|---|---|---|
| **all of R** (default) | R in full: k_s, k_c, both lines, components, maximal sub-cubes, merged classes, `N` | N states of R | is k_s = 1? |
| **`--minimal`** | the minimal line: its commitments, their states, their sizes | N of them | is the minimal line one state? |
| **`--maximal`** | the maximal line: the states nothing can be added to | N of them | is the maximal line one state? |

- **`-n N` prints the same N states under every lever and every `-j`** (C4).
  Which N is fixed by the route's own order (§ 3.2), not by a lever. A later
  release may change the route, and with it which N are printed, but never
  whether they are in R. `-n 1` is *any*.
- **`--single`** judges the expectation it states, in today's three words
  (ruling (i)). **1** is `Solution`, exit 0. **≥ 2** is `Ambiguity`, exit 1,
  and prints two of the states. **0** is `Contradiction`, or `Open` when every
  alive node owes, exit 1. **Undecided within the budget** is `NOT CHECKED`,
  exit 1, with its stderr line, as a false `:expect` has
  ([M1d S1d.4.3](../../../docs/history/m1d_satisfiability/the_vocabulary.md)).
- **`--minimal` and `--maximal`** are mutually exclusive, and so are `-n` and
  `--single`.
- **What every run prints beside its answer** is the route it took, the
  licences it used, `exhausted`, and the work done (saturations, facts). How
  it is laid out is S1e2.1a.5's page diff. Exit codes other than `--single`'s
  are S1e2.1a.5's too.

**Every option**, today's 38 and the four new ones. Today's list is
[`help_shape.txt`](../../../ein.rs/crates/ein-cli/tests/golden/help_shape.txt).

| option | today | proposed |
|---|---|---|
| *new* `--minimal`, `--maximal`, `--single` | — | the axes above (ruling 1) |
| `-n`, `--solutions N` | stop after N models, default 1 | **kept**, re-read: N states of the chosen set. Default unset, which is describe (ruling 1) |
| *new* `--region-budget N` | — | **the one count budget, for every mode**: facts beyond root, summed over the saturations. Default 2¹⁹ (question 3, ruled). Checked before each step of work. When it is reached, the run prints what it has certified and bounds for the rest (C3) |
| `-T`, `--max-time S` | wall-clock abort | **kept**: abort, exit 2. A machine-dependent limit never becomes an answer |
| `-j`, `--jobs` | threads | **kept**, every route |
| `--region-limit N` | `--dump-region`'s n, default 16 | **kept**: the largest n for which a program that no faster route licenses gets the cube inside `solve` (route 2). `ein oracle` reads the same option |
| `-L`, `--no-lookahead` | the walk's one-step lookahead off | **kept as a speed lever.** The lookahead becomes a pre-filter on a monotone program, where a one-firing death is a real refutation. A node it does not kill gets the full saturation. It no longer decides what is recorded, since `complete()` goes (§ 3.2) |
| `-K`, `--no-kill-cache` | the `(not h)` cache off | **kept as a speed lever.** The cache becomes bookkeeping, a remembered dead extension that the join skips, and it writes no fact |
| `-y`, `--lattice-sanity-check` | re-saturates every parent path | **kept**: a check. It can fail a run, and never changes an answer |
| `-o`, `--lattice-order` | within-layer order | **retired.** An order that is a lever could change which states `-n` prints (C4). Each route's order is part of its definition |
| `-z`, `--shuffle`; `-d`, `--seed` | permute each layer | **retired**, as `-o`. Order invariance is tested inside the test suite, without a CLI lever |
| `-e`, `--exhaustive` | do not stop at the first model | **retired**: the default is exhaustive. The usage error names the replacements, `--maximal` for the walk's maximal line and nothing for R |
| `-m`, `--max-set-size` | the walk's depth cap, default 5 | **retired.** A depth cap changes answers: 13 programs at `-m 5`, the six `square-*` among them. The question it approximated, *solutions within k guesses*, is answered by `--minimal`'s sizes |
| `-E`, `--max-enterings` | abort after N enterings | **retired**, in favour of `--region-budget`: one count, one unit, and a partial answer rather than an abort |
| `-D`, `--dump-states` | root's snapshot per layer | **retired.** Root is never written, so every snapshot would be root. A node's state is in `--dump-answer` and `--dump-lattice` |
| `--dump-region DIR`, `--region-states` | the oracle beside the walk | **moved** to `ein oracle --dump DIR` and `--states`. The usage error names the new spelling |
| `--dump-lattice DIR` | every commitment the walk tried | **kept**, re-pointed: every node this run visited, with its status, closure and firings, whatever the route |
| `--dump-answer DIR` | the answer as files | **kept**: every printed state as a loadable file |
| `-v`, `--layer-progress`, `-g` | per-layer progress | **kept**, re-pointed to the route that runs |
| `-H`, `--hyp-stats` | the root hypothesis preview | **kept**, re-read as H, the universe. The lookahead's filter report becomes a speed counter |
| `-c`, `-t`, `-s` | config, timing, counters | **kept**. `-s` gains the route, the licences and the facts derived |
| `--models`, `--diff-models`, `-p`, `-P`, `-f` | how states are printed | **kept**, S1e2.1a.5's |
| `--events`, `--events-level`, `--json-summary` | the stream, the summary | **kept**. The `writeback` event goes. `enter` and `layer` describe whichever route runs (S1e2.1a.5) |
| `-r`, `--trace` with `-G`, `-F`, `-R`, `-l` | the derivation trace | **kept**: the derivation of each printed state from its commitment |

**`ein solve` goes from 38 options to 33**: four new, nine retired or moved.

### 2.2 `ein oracle`

```
ein oracle FILE  [--region-limit N] [--dump DIR] [--states] [-j JOBS] [-T SECONDS]
                 [-s] [--json-summary FILE] [--models FORM] [-p] [-P]
```

- **What it does:** saturates every subset of H, with no licence and no
  pruning. It prints `ein solve FILE`'s description, in the same layout, plus
  what only the cube has: holes, `E`, and the static verdict checked against
  the cube.
- **Why it is a separate command:** it is the reference. It costs 2ⁿ on every
  program, including those where a mode needs one saturation, so it runs only
  when asked by name.
- **`--region-limit N`** (default 16): above it the command refuses with the
  projected cost. Passing a larger N is the request.
- **`--dump DIR`** writes today's `region.json` and `region.md`, and
  `--states` keeps every node's state, as `--dump-region` and
  `--region-states` do now ([`region_dump.md`](../../../docs/kernel/inference/region_dump.md)).
  `walk.json` goes, since there is no walk to write.

10 options. The census goes from 65 to 71: `solve` −5, `oracle` +10, and
`render region --visited` +1.

### 2.3 The other commands

- **`ein test`** runs, for each claim, the mode that decides it and no more.
  A claim of exactly one state runs `--single`, a claim about the maximal
  line runs `--maximal`, and a stated region runs describe. What a claim can
  state is S1e2.1a.4's grammar. Today it runs the walk at `-e -m 5` for every
  claim.
- **`ein saturate FILE`** is root, the node ∅. That is unchanged, and it is
  also how one commitment is checked by hand (§ 4.4).
- **`ein render region DIR [--visited DIR2]`** draws an oracle dump, and with
  `--visited` overlays the nodes a `solve` run visited, from its
  `--dump-lattice`. Today it overlays the walk that the same run wrote.
- **`ein render lattice FILE [solve flags]`** draws what that solve run
  visited.

### 2.4 The `(config …)` fields

The config principle is that a field changes speed and never an answer. Under
it, the 17 fields:

| field | proposed | why |
|---|---|---|
| `enable-pre-branch-lookahead`, `enable-lookahead-kill-cache`, `enable-path-nogoods`, `enable-fail-fast-fork` | **kept**, speed only | each prunes or stops work that a licence or a refutation already decides |
| `lattice-sanity-check`, `warn-derived-naf`, `record-alternative-justifications` | **kept** | a check, a diagnostic, and the shape of an explanation |
| `enable-symmetric-mirror` | **unchanged here** | it changes R. Its question is S1e2.1a.6's |
| `enable-singleton-writeback`, `enable-forced-positive` | **retired** | they write into root, which no mode does. What they knew survives as licence L1: a candidate dead alone leaves the universe, always |
| `lattice-order`, `lattice-order-seed`, `hypgen-scoring`, `hypgen-rel-weight`, `hypgen-obj-weight` | **retired** | order levers, as `-o` (C4) |
| `print-alive`, `candidate-order-seed` | **retired** | inert ([Q-M1e.10](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)) |

A retired field is a load error that names why. Six corpus files set one:
`examples/zebra.ein`, `syntax/config.ein`, and four register witnesses
(`forced-2`, `forced-2-off`, `naf-nowb`, `f07-fpoff`). They are edited in the
same commit.

## 3. Reference: how each mode is computed

### 3.1 The licences

A licence is a theorem that lets a mode skip work without changing its
answer. Each holds for a class of program, and the static check decides the
class.

| | licence | holds when | what it lets a mode do |
|---|---|---|---|
| **L1** | dead is upward-closed | monotone | never visit a superset of a dead node. A candidate dead alone leaves the universe |
| **L2** | discharged is upward-closed | monotone | every alive superset of a node of R is in R. So `--minimal` does not expand a node of R, and when ∅ ∈ R, R is the whole alive set |
| **L3** | a commitment's state is its closure's | monotone | visit states, not commitments, and count k_c as f(T) = 2^\|T\| − Σ f(S) over the alive states S ⊊ T |
| **L4** | the lines are local | monotone, so no hole | a node of R is maximal exactly when every one-up extension is dead, and minimal exactly when every one-down subset owes. A line member is certified with at most n saturations |
| **L5** | an owed instance's alternatives are exhaustive | monotone, and every fact that can witness the instance is in H | branch on one owed instance's alive alternatives. **An alive node owing an instance whose every alternative is dead has no member of R above it.** That is the user's *owing meets dead*, and it ends a branch |
| **L6** | trivial lines | the program's shape | restrictions only: the minimal line is {∅} when root is alive, one saturation. Obligations only: the maximal line is {⊤}, one saturation |
| **L7** | the cube | always | 2ⁿ saturations decide everything. That is `ein oracle`, and route 2 inside `solve` |

L1–L4 and L6 are [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)
§§ 1.5, 1.6 and 2.4, and the corpus agrees with them on every monotone cube
(§ 8 there). **L5 is new.** Its argument: at a node C owing instance I, every
discharged D ⊇ C holds a witness a of I. Since a ∈ H, a is in D's closure,
so D's state is above C ∪ {a}; if that node is dead, so is D. Its
precondition is checkable when the program loads. It is proved and held to
the oracle at P1e2.1c before any mode relies on it.

### 3.2 A monotone program: each mode's route

*Now* is what T1e2.1a.9.4 builds: route 1
([`ein_infer::downset`](../../../ein.rs/crates/ein-infer/src/downset.rs)),
with a cut per mode on which nodes it expands and when it stops, plus one new
step, the **climb**. *At P1e2.1c* is the cheapest sound route the phase is
for.

| mode | stops when | now: route 1 with a cut | at P1e2.1c | licences |
|---|---|---|---|---|
| `-n N` | the N-th state | layer order from ∅, stopping at the N-th distinct state. When ∅ ∈ R, root is the first, one saturation | the obligation tree's leaves, in its own order, where L5 holds | L1, L2 |
| `--maximal -n N` | the N-th maximal state | the first member of R, then the **climb**: add the lowest-index candidate whose extension is alive, until none is. For N > 1, `--maximal` stopped at N | the same, over states | L1, L4 |
| `--minimal -n N` | the N-th state on the minimal line | layer order, expanding owing nodes only. The first node of R it meets has every proper subset owing, so it is minimal | a tree leaf, then shrink it while it stays discharged | L1, L2 |
| `--single` | a second state, or decided | **∅ ∈ R:** R is the alive set (L2), so k_s = 1 exactly when no candidate outside root's closure is alive alone, at most n saturations. **∅ ∉ R:** `--minimal`'s exploration. `Ambiguity` as soon as two minimal nodes reach different states. Otherwise the one state s must be maximal (L4), at most n − \|s\| more | the tree: owing-meets-dead closes branches, then the same maximality check | L1, L2, L4, L5 |
| `--maximal --single` | a second maximal state, or decided | climb to s. Then route 1 over the alive nodes **not under s**. A discharged one has a maximal extension other than s (L2), which is `Ambiguity`. If there is none, s is unique | the state route | L1, L2, L4 |
| `--maximal` | no alive node is left outside the maxima found | as above, climbing from every discharged node found outside the maxima so far | the state route: at most alive states × n | L1–L4 |
| `--minimal` | no owing node is left | route 1 expanding owing nodes only. Restrictions only: {∅}, one saturation | duality, S1e2.1c.2: output-sized | L1, L2, L6 |
| describe | the down-set is exhausted | route 1, unchanged | the state route, k_c by L3, the generators by minimal transversals (S1e2.1c.2) | L1, L3 |

Why the cuts are sound, one line each:

- **The climb** ends at a node of R whose every one-up extension is dead,
  which L4 makes maximal.
- **`--maximal`** never misses a maximal state m. If m is under no maximum
  found, its subsets that lie outside those maxima are expanded, so the
  restricted join reaches m. Its subsets under a found maximum are alive
  without a visit, by L1.
- **`--single`** with ∅ ∉ R: every node of R contains a minimal one. If all
  minimal ones reach s and s has no alive extension, every node of R reaches
  s.
- **The budget** is checked before each step: a layer for route 1, a climb
  step, a node for the tree. A stopped run has its certified part and its
  bounds, and nothing half-done (C3).

### 3.3 A program the static check flags

No licence holds but L7, so nothing can be skipped and a line is defined only
over the cube.

| mode | what it does | certified |
|---|---|---|
| `-n N` | visits commitments in layer order and prints each node of R it meets | each printed state is a member of R, and is verified because it is the oracle's node. *There are only k < N* needs the cube |
| `--single` | the same, and stops at the second state | `Ambiguity` is certified at once. `Solution` and `Contradiction` need the cube |
| `--minimal`, `--maximal`, describe | the cube | within `--region-limit`. Past it: partial, with states printed as members of R and not as line members |

What P1e2.1b decides a non-monotone node *is* may license more, such as
S1e2.1b.6's guard split. That is S1e2.1c.3's.

### 3.4 What it costs

Saturations, i.e. nodes evaluated. *Measured* numbers carry their command.
*Derived* ones are arithmetic over route 1's layer counts for `zebra.ein`
([`route1_reach.md` § 6](route1_reach.md#6-where-route-1-declines-and-zebras-region-counted),
2²⁰ visits, 2026-10-16), and each is an upper bound.

**`examples/zebra.ein`**: n = 56. R is one state s of 18 candidates and
261 982 commitments. There are 27 alive states and 8 472 alive commitments
that owe. ∅ ∉ R. Four of R's 63 minimal generators are single facts.

| question | mode | the walk today | the walk, writes off | **now**, derived | **at P1e2.1c** |
|---|---|---:|---:|---:|---:|
| any solution | `-n 1` | 13 | 24 | ≤ 57: root, then layer 1 holds four members of R | 2, the tree |
| a complete solution | `--maximal -n 1` | 13 | 24 | ≤ 95: + 38 extensions of s, all dead | ≤ 95 |
| unique? | `--single` | 111 | 8 891 | ≤ 9 031: 8 472 owing + 63 minimal + 458 nogoods + 38 | ≈ 47: the tree's 9, + 38 |
| unique among complete? | `--maximal --single` | 111 | 8 891 | ≤ 8 863: the climb + the 8 310 alive nodes not under s + 458 | ≤ 1 512, the state route; ≈ 47 here since k_s = 1 |
| all complete | `--maximal` | 111 | 8 891 | ≤ 8 863 | as above |
| the least guessed | `--minimal` | — | — | ≤ 8 993 | output-sized: 63 generators |
| everything | (none) | — | — | 270 912. That is 2²⁴·⁹ facts, so it stops at the default budget with a partial answer that already prints s | ≤ 1 512 + the transversals |

- **The walk's 111 is not reachable soundly.** It rests on the walk's
  stronger node: with every search write off, the same walk needs 8 891
  enterings. Measured 2026-10-16 with a copy of `zebra.ein` ending in
  `(config :enable-forced-positive false :enable-lookahead-kill-cache false
  :enable-singleton-writeback false)`, then `ein solve -s -m 56 -n 1` (24
  enterings) and `-e` (8 891, 5.3 s). `zebra2.ein` gives 11 and 8 577.
- **The tree beats it soundly.** Its column is measured on
  `zebra2-obligations.ein`, zebra2 with the hrule deleted, because the tree
  declines on the hrule rung today. `EIN_TRAVERSAL=tree ein solve -s -n 1`
  is 2 enterings, and `-n 1000` exhausts the tree in 9 (4 alive, 5 dead,
  11 ms), with writes on or off. On `zebra2-minus-15-obligations.ein`, `-n 2`
  is 5 enterings and the whole tree 86, giving 32 states. The 38 is s's
  maximality check, n − |s|. Reaching `zebra.ein` itself needs L5's
  precondition on the hrule rung, which is P1e2.1c's to prove.
- **At the default budget, every *now* row from `--single` down stops
  partial on zebra.** About 112 facts per node puts 9 000 saturations near
  2²⁰. `--region-budget 2^21` certifies them. At P1e2.1c they fit with room
  to spare.

**`examples/saturation/square-bwd/floors.ein`**: n = 20. Every commitment is
alive and in R: k_c = 2²⁰, k_s = 27 559, the maximal line is {⊤}, ∅ ∈ R.

| question | mode | the walk today | **now** |
|---|---|---|---:|
| any solution | `-n 1` | *no model found*, 21 699 enterings at `-m 5` | 1: root |
| a complete solution | `--maximal -n 1` | the same; 1 047 552 enterings uncapped | ≤ 21: the climb |
| unique? | `--single` | — | ≤ 2: the first alive singleton outside root's closure, `Ambiguity` |
| unique among complete? / all complete | `--maximal --single` / `--maximal` | as above | ≤ 21: ⊤ holds every candidate, so no node lies outside it |
| the least guessed | `--minimal` | — | 1: ∅ ∈ R |
| everything | (none) | — | 2²⁰ visits, stopping at the budget in 0.3 s. Duality gives one sub-cube [∅ … ⊤] and k_c. k_s needs its 27 559 states |

This is the minimum-work principle at its plainest. The questions a person
asks first cost 1–21 saturations. Only *describe everything* pays for 27 559
states, and it says so.

## 4. Guide: which command answers which question

`F` is the program. Each answer is a statement about the oracle's R (§ 1).
*Cost* names the work in § 3's terms.

### 4.1 Is there a solution, and what is one?

| question | command | what you get | cost |
|---|---|---|---|
| Does the program have a solution at all? | `ein solve F -n 1` | one state of R. Or *R is empty*, with why: root's core when root is dead, or what every alive node owes | up to the first member; proving *none* needs the alive down-set (monotone) or the cube |
| Give me one complete solution, one nothing can be added to | `ein solve F --maximal -n 1` | one state, certified maximal | the above, plus one saturation per candidate outside it |
| Give me several | `ein solve F -n 5`, or `--maximal -n 5` | five states, the same five under every lever | stops at the fifth |
| What is the least I have to guess? | `ein solve F --minimal -n 1`; all of them: `--minimal` | a minimal commitment and its state; or the whole minimal line, with sizes | owing nodes only |
| Does a solution need more than k guesses? | `ein solve F --minimal` | the sizes of the minimal commitments. Today's `-m k` answered this wrongly where it cut | as above |

### 4.2 Is it unique?

| question | command | what you get |
|---|---|---|
| Exactly one solution, for a program that states what it requires (obligations) | `ein solve F --single` | `Solution`, exit 0; `Ambiguity` with two states, exit 1; `Contradiction` or `Open`, exit 1 |
| Exactly one complete solution, for any program, restrictions only included | `ein solve F --maximal --single` | the same words, about the maximal line |
| Is the minimal guess unique? | `ein solve F --minimal --single` | the same, about the minimal line |
| The same check, stated in the program | `(solution …)` in F, then `ein test F` | S1e2.1a.4's form. The runner picks `--single`'s route |

Plain `--single` on a program with restrictions only fails whenever any
candidate is alive, because ∅ ∈ R there and every alive extension is a
second state. That is why the second row exists (ruling (a)).

### 4.3 How many, and what shape?

| question | command | what you get |
|---|---|---|
| How many solutions? | `ein solve F` | k_s (states) and k_c (commitments) |
| Every complete solution | `ein solve F --maximal` | the maximal line |
| Every minimal one | `ein solve F --minimal` | the minimal line |
| Everything | `ein solve F` | R in full: both counts, both lines, components, maximal sub-cubes and their dimensions, the merged classes, `N` |
| Which facts every complete solution shares, and which vary | `ein solve F --maximal --models key` | the determining key, the slots that tell the states apart. The certain core is S1e2.1a.5's to print |
| Every state as a file I can load | `ein solve F … --dump-answer DIR` | one loadable file per printed state, with the commitment that reached it |
| What does a query's goal take, across the solutions? | `(query :goal … :expect …)` in F | the goal's projection classes, S1e2.1a.4 |

### 4.4 Checking

| question | command |
|---|---|
| Is the answer I stated in the program right? | `ein test F` |
| Is this partial assignment consistent, and is it a solution? | add its facts to a copy of F, then `ein saturate COPY` for that node's state (dead? owing?), or `ein solve COPY -n 1` for what extends it |
| Does `ein solve` agree with the reference? | `ein oracle F` beside `ein solve F`: identical where both finish. The test suite holds this for the corpus (§ 5) |
| Which shortcuts did the run use, and is the program monotone? | the run's `route` line names the licences. `ein oracle F` checks the static verdict against the cube |

### 4.5 Cost and inspection

| question | command |
|---|---|
| How much would describing it cost? | `ein solve F`. Past the budget it stops, prints what is certified, and projects the facts the rest needs |
| Spend more, or less | `--region-budget N` for a count, `-T S` for a wall-clock abort |
| Faster, same answer | `-j N`; the speed levers `-L` and `-K` |
| What did the run visit? | `-s` for counts; `--dump-lattice DIR` for every node; `ein render region ORACLE_DIR --visited DIR` to draw them on the cube |
| The whole lattice, drawn | `ein oracle F --dump D`, then `ein render region D` |
| Why does a solution look like this? | `--trace FILE.md`: the derivation of each printed state from its commitment |

## 5. Why two modes cannot disagree, and the test that holds it

By construction (§ 1): one node function, and licences only where the static
class proves them. What makes it a fact rather than an argument is one test,
`every_mode_is_the_oracle` in
[`ein-infer/tests/region.rs`](../../../ein.rs/crates/ein-infer/tests/region.rs).
It extends `route_1_is_the_oracle_on_every_monotone_program_it_reaches`,
which becomes its describe row:

- **for every program whose R is known**: the oracle at n ≤ 16, and route 1
  at a raised budget past that (the zebra family);
- **every cell of § 2.1's table**, with `-n` at 1, 2 and k_s + 1;
- **checks** C1 on every printed state, C2 on every certified claim, and C3
  at a budget small enough to stop every non-trivial run;
- **and C4**, the same output under every kept speed lever and under `-j`
  1 and 8. That is S1e2.1a.6's lever test, extended to the modes.

The flagged programs are in it. They get the cube within the limit, so C2
binds them there, and C1 and C3 bind them everywhere.

## 6. What moves

**When.** Ruling 4 stands: the default switches once, in S1e2.1a.5's commit.

- **T1e2.1a.9.4** lands `ein oracle`, the flags, the cuts of § 3.2, the
  retired options as usage errors, the retired config fields, and § 5's
  test.
- **Until S1e2.1a.5, bare `ein solve FILE` is still today's walk.** It can
  disagree with the new modes, and only where the register says it is wrong.
- **S1e2.1a.5** switches the default to describe and removes the last CLI
  path to the walk. The walk's library code goes in the same commit, or is
  kept with no caller until P1e2.1c measures what of it the cheap modes
  reuse.

| what | how many | what happens |
|---|---:|---|
| corpus `solve -e` cells | 94 | dropped from their `runs` column: each asks what bare `solve` asks |
| corpus `solve -e --dump-region` cells | 10 | → `ein oracle --dump` |
| corpus `-m`, `--dump-states`, `-E` cells | 8, 8, 1 | removed or re-pointed one by one. Each is named before it moves |
| corpus `levers` columns | 12 | `-o score-sum` goes. `-L`, `-K` and `-y` stay, now with C4's guarantee |
| register runs | 46 | re-pointed to the mode that asks the row's question, `--maximal` for a model-set row |
| register rows whose wrong answer is the walk's | 15 of 26: W1 (7 rows), W2 (2), W3 (2, the walk's pruning under `absent`, where a mode takes the cube), W5, W9 (2), and W11's `unsat-core-id-space`. Then `zebra2-hints` and `branching/07`, once S1e2.1a.6 files them | **expected to flip to fixed** at the switch. Each is confirmed by § 5's test against its `right` block restated under R at S1e2.1a.7, which therefore lands first |
| register rows the oracle shares, or that are about a read-out | W4 and W6 (the candidate list), W7 (the saturator), W8 (the node under `absent`, P1e2.1b's), W10, W11's `d3-unsat-core` and W12 (which core is printed), C1 (the blind rung) | unchanged by this interface ([`solution_semantics.md` § 6.6](../../../docs/kernel/inference/solution_semantics.md#66-what-the-oracle-does-not-check)) |
| option census | 65 → 71 | `configuration.md` § 3, `help_surface.rs`, `help_shape.txt`, `config_reference.rs` |
| config fields | 17 → 8 | six corpus files edited (§ 2.4) |

**P1e2.1c** then takes each *now* column of § 3.2 to its *at P1e2.1c*
column:

- the state route first (ruled);
- L5 and the tree, proved and tested against the oracle;
- duality at S1e2.1c.2;
- the non-monotone search at S1e2.1c.3.

Its S1e2.1c.4, *the flags at scale*, is § 3.2's right-hand column. Its
S1e2.1c.5 no longer asks whether the walk is kept: it is not. What is left of
that stage is P1e2.2's re-scoping.

## 7. Decided here unless the user objects

**Ruled 2026-10-16: yes, every item** (§ 0.1).

- **`-n` prints the same states under every lever** (C4), so every order
  lever is retired: `-o`, `-z`, `-d`, and the five order fields. The
  alternative was a witness that changes with a lever, which the config
  principle reads as a lever changing what is printed.
- **One budget, in facts, for every mode**, default 2¹⁹. A mode the user asked
  for explicitly still stops at it. Otherwise `--single` on a flagged program
  with n = 40 would ask for 2⁴⁰ saturations without saying so.
- **`-m`, `-E`, `-e`, `-D` and `--dump-region` are usage errors that name the
  replacement**, not silent no-ops. A flag that does nothing is a claim
  nobody checks.
- **The interim cost on zebra is accepted.** Between S1e2.1a.5 and P1e2.1c,
  zebra's `--single`, `--maximal` and `--minimal` need `--region-budget 2^21`
  to certify (§ 3.4). They answer correctly or say *partial*, where the walk
  answered fast and, on `zebra2-hints`, wrongly.
- **No `--count`.** Until the state route exists, counting costs what
  describing costs, so a flag for it would only filter the output. S1e2.1c.4
  adds it if the transversals turn out to dominate describe.
- **`solve_flags.md` § 10 stands**, except its route-3 bullet. A run past
  every route's reach now prints its certified part and its bounds (C3), and
  runs no second search of a different kind.
