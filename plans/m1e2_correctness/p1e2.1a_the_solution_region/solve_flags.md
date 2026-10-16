# `ein solve`'s default and its flags — the page diff for the ruling

**Stage:** [S1e2.1a.9](s1e2.1a.9_solve_describes_the_region.md), task
T1e2.1a.9.2.
**Status:** proposed 2026-10-16. **Questions 1, 2 and 4 were ruled the same
day**, and this page's § 9 has the rulings; § 10 is what the stage decides
unless the user objects. **Question 3 waits** for the analysis the user asked
for first, [`walk_and_region.md`](walk_and_region.md). **Nothing in the parser
has moved.**
**What it stands on:** route 1 exists in the library since T1e2.1a.9.3
([`ein_infer::downset`](../../../ein.rs/crates/ein-infer/src/downset.rs)) and is held
to the oracle node for node. How far it reaches is
[`route1_reach.md`](route1_reach.md) (T1e2.1a.9.1).

---

## 1. What is being decided

The user, 2026-10-16:

> change solve behaviour; the default simple `solve` without flags we get
> exhausive certifying full region description; while with some flags can
> simplify the task - e.g. look for any solution, or expect there is a single
> solution - so k>=2 is a contradiction; or look for only maximal solutions,
> or only for minimal solutions

Four things follow. Each is decided here before the parser changes:

1. the **names** of the four flags, and how they compose;
2. **today's 38 `ein solve` options**, one by one: kept, re-read or retired;
3. **where today's walk goes**. It is the search under review, and
   [the register](../../../corpus/wrong_answers.toml) reproduces 46 answers
   with it, all of them `solve` runs;
4. **when the default switches**, which decides how many times the goldens
   move.

## 2. The routes the default takes

From the stage file, unchanged, with what T1e2.1a.9.1 measured:

| route | when | what it computes | cost | certified |
|---|---|---|---|---|
| **1**, the alive down-set | the static check finds no site | R, both lines, k_s, k_c, components, sub-cubes, merges, `N` | \|alive\| + \|N\| saturations | yes: the static verdict, `N` and the minimal line |
| **2**, the oracle | flagged, or route 1 declined, and n ≤ `--region-limit` (16) | the same, over all 2ⁿ nodes, plus holes and `E` | 2ⁿ saturations | yes, under the stub's *assumes a monotone lattice* when flagged |
| **3**, the walk | neither | what the walk records, as a lower bound | the walk's | no: `exhausted = false` by definition |

Route 1's budget is § 6.

## 3. The flags: two axes

The user's four, as the stage file reads them: **which nodes** and **what is
asked**.

| | **describe** (default) | **`-n N`** (any, N times) | **`--single`** |
|---|---|---|---|
| **all of R** (default) | R in full | the first N states of R the route finds | is k_s = 1? |
| **`--minimal`** | the minimal line | the first N states on it | is the minimal line one state? |
| **`--maximal`** | the maximal line | the first N states on it | is the maximal line one state? |

- **`-n 1` is *any solution*.** Today's `-n N` already means *stop after N*,
  and the eight corpus `solve -n 3` runs keep their spelling. Only its default
  moves: from 1 to unset, which is *describe*. There is no separate `--any`,
  because a second spelling of `-n 1` would be a parallel copy.
- **`--single`** expects exactly one state in the chosen set. It answers in
  today's three words, and they judge that expectation (ruling (i)). **1** is
  `Solution`, exit 0. **≥ 2** is `Ambiguity`, exit 1. **0** is
  `Contradiction`, or `Open` when every alive node owes, exit 1. It exits the
  way a false `:expect` does, with the same one stderr line
  ([M1d S1d.4.3](../../../docs/history/m1d_satisfiability/the_vocabulary.md)).
- **`--minimal` / `--maximal`** restrict the set. They are mutually exclusive,
  and so are `-n` and `--single`.
- **`--maximal --single`** is the uniqueness question for a program that
  states no obligation. There ∅ ∈ R, so plain `--single` fails whenever a
  candidate is alive (ruling (a)).

What each cell costs on route 1. This is why the flags *simplify* the task
rather than only filtering the output:

| cell | route 1 stops | why it is sound |
|---|---|---|
| describe, all | when no alive commitment is left | the definition |
| `--minimal` | it expands **owing** alive nodes only, and stops when none is left | a superset of a node of R is not minimal. Every subset of a minimal node is alive and owes, so it is reached through owing nodes alone. When ∅ ∈ R that is one saturation |
| `--maximal` | as describe | a node is maximal only when its supersets are known. Doing better is P1e2.1c's duality stage, S1e2.1c.2 |
| `-n N` | at the N-th state of the chosen set, in layer order | layer order finds the smallest commitments first. `-n 1` on a program with ∅ ∈ R is root, one saturation |
| `--single`, all or `--minimal` | at the second state found, or when the set is exhausted | two states decide `Ambiguity`, and `Contradiction` needs exhaustion |
| `--single --maximal` | as describe | as `--maximal` |

On route 2 every cell costs 2ⁿ. On route 3 every cell is the walk's, and is
partial.

## 4. Today's 38 options

Read off [`help_shape.txt`](../../../ein.rs/crates/ein-cli/tests/golden/help_shape.txt).

| option | today | proposed |
|---|---|---|
| `-n`, `--solutions N` | stop after N models, default 1 | **re-read**: N states of the chosen set; default unset (describe) |
| `-e`, `--exhaustive` | do not stop at the first model | **retired**: the default is exhaustive. A usage error that says so (§ 10) |
| `-m`, `--max-set-size` | the walk's depth cap, default 5 | **kept, the walk's only**: read by route 3 and `ein walk`. Routes 1 and 2 have no depth cap, so a description they give is never cut by `-m` |
| `-E`, `--max-enterings` | abort after N enterings | **kept, the walk's only**, as `-m` |
| `-T`, `--max-time` | abort after N seconds | **kept, every route**: a wall-clock abort, exit 2, as today |
| `-L`, `-K`, `-o`, `-y`, `-z`, `-d` | the six shadows of a `(config …)` flag | **kept, the walk's levers**. Under routes 1 and 2 they have nothing to change, and one stderr `note:` line says so (§ 10) |
| `-j`, `--jobs` | threads | **kept, every route**. Routes 1 and 2 fan out with the walk's contract |
| `--region-limit N` | `--dump-region`'s n | **kept, and read by the default too**: route 2's n |
| *new* `--region-budget N` | — | route 1's budget, in facts beyond root (§ 6) |
| *new* `--minimal`, `--maximal`, `--single` | — | § 3 |
| `--dump-region DIR`, `--region-states` | the oracle on its own load | **kept**. The dump is the oracle's drawing input, and a default that took route 1 does not change it |
| `-p`, `-P`, `-f`, `-s`, `-t`, `-v`, `-H`, `-c`, `-g`, `--layer-progress`, `--models`, `--diff-models` | presentation | **S1e2.1a.5's**: what each prints under routes 1 and 2 is the read-out's page diff, T1e2.1a.5.1 |
| `--events`, `--events-level`, `--json-summary`, `-r`/`--trace` with `-F` `-G` `-l` `-R`, `-D`/`--dump-states`, `--dump-lattice`, `--dump-answer` | artefacts | **S1e2.1a.5's**, as above. `--trace`, `--dump-states` and `--dump-lattice` describe a *walk*, and under routes 1 and 2 there is none |

`ein solve` goes from 38 options to **41**: four new (`--minimal`,
`--maximal`, `--single`, `--region-budget`) and one retired (`-e`). The
census's total depends on § 5. Under (a) `ein walk` carries today's 38
unchanged, built by the same function, and the total goes from 65 to 106.
Under (b) it is 69, and under (c) 68. `configuration.md` § 3.1's table,
`help_surface.rs`, `help_shape.txt` and `config_reference.rs` move in the
same commit.

## 5. Where today's walk goes

The walk is route 3, so its code stays whatever is decided here. What is
being decided is how a person, the register and the corpus reach it **as the
whole answer**, the way `ein solve` does today. The register needs that until
[P1e2.1c](../p1e2.1c_the_region_search/README.md)'s S1e2.1c.5 decides whether the
walk is kept.

| option | reads as | the config principle | the register's 46 runs |
|---|---|---|---|
| **(a) `ein walk FILE`**, a fifth subcommand that is today's `ein solve` byte for byte | *what does the walk find?*: a different question from *what is R?* | holds for `solve`: no lever of `solve` chooses a route | `solve -e` → `walk -e` |
| (b) `ein solve --walk` | a flag of `solve` that changes its answer | broken by construction: a lever that changes R | `solve -e` → `solve --walk -e` |
| (c) `EIN_WALK=1 ein solve` | `EIN_TRAVERSAL=tree`'s precedent: an environment lever whose fate is undecided | broken as (b), and out of `--help` | `env = ["EIN_WALK=1"]` on each |

**Proposed: (a).** The memory of 2026-10-15 is *levers and config affect speed
only, never solutions*. The walk does change solutions: that is what the
register records. So it can be a command, but not a lever. `ein render
lattice` and `ein test` run the walk today. `render lattice` keeps it, since
it draws a walk. `ein test` checks expectations, which is S1e2.1a.4's.

## 6. Route 1's budget

A count, never a wall clock. A machine-dependent budget turns *too slow here*
into an answer, which `lever_matrix.rs` already says of the process matrix. It
is checked **before each layer**, so a declined run has saturated whole layers
`0 … k` and nothing past them.

[`route1_reach.md` § 4](route1_reach.md#4-the-budget) has the curve. What the
choice trades:

- **a budget too small sends programs route 1 finishes to route 3.** Past the
  oracle's limit, route 3 is the walk, so they lose their certificate;
- **a budget too large makes the default pay for a decline.** Every
  saturation route 1 spends on a program it then declines is spent before
  route 3 starts. At 2¹⁶ visited commitments that is 23–72 s on the zebra
  family, which today's `solve` answers in milliseconds.

**Proposed: the unit is facts beyond root, summed over the visited nodes, and
the default is 2¹⁹.** Per node, the cost spreads 400× across the corpus. Per
fact derived it spreads 28×, so a fact budget bounds the wall clock and a node
budget does not. 2¹⁹ is the smallest budget that loses nothing route 1
describes short of the zebra family:

- 162 of 195 programs described, 15 of them past the oracle's limit;
- the worst decline is 5.7 s at one job;
- all twelve declines together cost 24 s.

A layer's facts are known only after it runs, so the budget is checked
against a projection: the next layer's size, from the join, times the facts
per node of the layer before. The numbers above are that rule's.

The zebra family itself fits at about 2²⁵ facts. That is 25 s on 12 jobs for
`zebra.ein`, whose R is one state of 261 982 commitments: a reader asks for it
with `--region-budget`, and the default does not pay for it.

## 7. When the default switches

| option | the commits | goldens move | `:expect` meanwhile |
|---|---|---|---|
| **(A) at S1e2.1a.5** | T1e2.1a.9.4 lands the routes, the flags and `ein walk`, and the default stays the walk. S1e2.1a.5 switches the default in the same commit as the read-out that prints it, after S1e2.1a.4 has said what an `:expect` claims about R | **once** | unchanged until S1e2.1a.4 |
| (B) at T1e2.1a.9.4 | the default switches now, with an interim read-out (route, k_s, k_c, `exhausted`) that S1e2.1a.5 replaces | **twice**: the corpus exits, summaries and `read_out.rs` at .9, and again at .5 | undefined. Today's `:expect (model …)` states one model set, and R has no single model set. Every corpus `solve` cell whose program states one changes exit code until S1e2.1a.4 lands |

**Proposed: (A).** Under (A), T1e2.1a.9.4 still reaches every route and flag
from the CLI. Until the switch they run where they cannot be mistaken for the
default: under `--minimal`, `--maximal` and `--single`, which do not exist
today, and with an explicit `-n`. Bare `ein solve FILE` keeps today's output
until S1e2.1a.5.

## 8. What moves, named before it moves

Counted 2026-10-16 from `corpus/corpus.toml` and `corpus/wrong_answers.toml`.
At the switch (S1e2.1a.5 under (A), T1e2.1a.9.4 under (B)):

| what | how many | why |
|---|---:|---|
| corpus `solve` cells | 214 | describe instead of `-n 1`: stdout, the summary and possibly the exit code |
| corpus `solve -e` cells | 94 | `-e` is retired. Under the new default each asks what `solve` asks, so the run is dropped from its `runs` column (the corpus's rule: a run is dropped only when it does not ask the fixture's question) |
| corpus `solve -e --dump-region` cells | 10 | → `solve --dump-region` |
| corpus `solve -n 3`, `-m 2`, `-p -s`, `--trace`, `--dump-states` cells | 8 each | re-read as § 4 says. `-m 2` on a route-1 program is inert |
| corpus `solve -E 1` | 1 | the walk's budget. → `walk -E 1` |
| register answers | 46 | 37 `solve -e`, 3 `solve -e -L`, 3 `solve -e -K`, 2 `solve`, 1 `solve -m`: each → `walk …`, byte-identical answers, so no row changes status |
| `golden/corpus_exits.txt` | one line per cell above | renamed runs, and any exit a describing run changes |
| `golden/help_shape.txt`, `help_surface.rs`, `config_reference.rs`, `configuration.md` § 3 | — | the option census, 65 → 68, and a fifth subcommand |
| `read_out.rs`, `summary_properties.rs`, `corpus_shapes.md5` | — | S1e2.1a.5's list. The read-out is the read-out stage's |

**A corpus `:expect` that held only because the search stopped early** is
listed before anything is re-blessed. The stage's acceptance asks for it,
and S1e2.1a.4's migration table is where each one goes.

## 9. What needs the user's ruling

The user, 2026-10-16, verbatim:

> 1 - yes; 2 - yes; 4 - yes; 3 - explain first the difference between walk and
> oracle/solve; then analyse and report the reasons why they differ; because
> this conflicts with my assumption that thay are equivalent

1. **The names.** `-n N` (with `-n 1` as *any*), `--single`, `--minimal`,
   `--maximal`, `--region-budget N`. **Ruled: yes.**
2. **Where today's walk goes.** (a) `ein walk`, (b) `ein solve --walk`, or (c)
   `EIN_WALK=1`. **Ruled: (a), `ein walk`.**
3. **Route 1's budget**: its unit and its default
   ([`route1_reach.md` § 4](route1_reach.md#4-the-budget)). **Open.** The user
   asked first why the walk and the oracle differ at all, since the question
   assumes they compute the same thing: [`walk_and_region.md`](walk_and_region.md).
   Its § 6 puts the question again with what the analysis found:
   - **(A)** route 1 enumerates commitments, under a budget of 2¹⁹ facts;
   - **(B)** a route that enumerates **states** as well. Zebra has 27 alive
     states against 270 454 alive commitments.

   Its § 7 asks the user to confirm one consequence: R's node is
   `sat(root ∪ C)`, without the search's writes. That follows from the config
   principle, and it moves W9's and W11's `right` blocks.
4. **When the default switches.** (A) at S1e2.1a.5, once, or (B) now, twice.
   **Ruled: (A).** The default switches in S1e2.1a.5's commit, with the
   read-out that prints it.

## 10. Decided here unless the user objects

- **`-e` is retired, not kept as a no-op.** A flag that does nothing is a
  claim nobody checks. The usage error names the replacement: *the default is
  exhaustive; for the walk's `-e`, use `ein walk -e`*.
- **A flag and a stated expectation are both checked**, and neither overrides
  the other. `--single` is the run's expectation and the program's
  `(solution …)` is the program's (S1e2.1a.4). Each is judged against what the
  run computed. A claim the computed set cannot decide, such as `--maximal`
  beside a stated full region, is `NOT CHECKED` with its stderr line, as M1d
  S1d.4.3 requires.
- **The walk's levers under routes 1 and 2 print one `note:` line**, never a
  refusal: *`-L` changes the walk, and route 1 took this run*. A refusal would
  make a corpus `levers` cell an error on every route-1 program. Silence would
  let a reader believe the lever was applied.
- **Route 3 runs the walk exhaustively** within `-m` (default 5), under an
  enterings budget of 2¹⁶ when `-E` is not given. Measured: `zebra.ein -e` is
  111 enterings, `features/04_open` 27 373 977 (98 s), and `zebra2-minus-15
  -e` 17 204 592 (25 min). When that budget is hit, the walk
  stops and reports what it found as partial, with `exhausted = false`. It
  does not abort with exit 2, because a route that declines is a description
  that says *partial*, not a refusal.
- **Root is never written by routes 1 and 2.** `ein-infer/tests/region.rs`
  holds route 1's root equal to `sat(program)` on every program it runs.
  Route 3 is the walk, which keeps its root writes until S1e2.1c.5
  ([`solution_semantics.md` § 6.3](../../../docs/kernel/inference/solution_semantics.md#63-the-search-writes-into-root-and-its-universe-is-not-h)).
