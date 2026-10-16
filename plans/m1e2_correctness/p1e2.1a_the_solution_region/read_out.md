# The read-out speaks R — the page diff for the vocabulary

**Stage:** [S1e2.1a.5](s1e2.1a.5_the_read_out.md), task T1e2.1a.5.1.
**Status:** proposed 2026-10-16, for the user to read before any surface
moves. **Nothing in `ein solve` has moved.** Three questions are the user's
(§ 11). The rest is decided here unless the user objects (§ 12).
**What it stands on:**

- the rulings (a), (e), (g) and (i) of
  [Q-M1e2.7](../open_questions.md#the-rulings-and-the-questions-back-2026-10-16),
  and the third answer's `(region …)` form
  ([§ The third answer](../open_questions.md#the-third-answer-2026-10-16));
- the approved interface ([`solve_interface.md`](solve_interface.md)), whose
  § 2.1 left *"how it is laid out"* and *"exit codes other than `--single`'s"*
  to this page;
- the region language as built ([`region_language.md`](region_language.md),
  [`region_examples.md`](region_examples.md)), whose query `:expect` lands at
  this stage's switch.

**The examples are real.** [`read_out_examples.md`](read_out_examples.md)
holds 25 runs on twelve programs. Each block is printed by
[`read_out_preview`](../../../ein.rs/crates/ein-infer/examples/read_out_preview.rs),
an `ein-infer` example that prints the layout proposed here from the
library's own R, states and goal projections. This page quotes them as
`X1`…`X12`.

---

## 1. What is being decided

The words every surface uses for an answer that is now a region. The stage's
acceptance lists the surfaces:

- the verdict block;
- `--json-summary`;
- `--dump-answer`;
- `--models list|key|grid`;
- `--trace`'s headline;
- `ein test`'s report.

This page adds the four the acceptance implies: the query's answer, the event
stream, `-s`/`-p`/`-P`/`-f`/`--diff-models`, and the exit codes.

Three rulings fix most of it:

- **(a), (e): k_s and k_c replace `k`**, both printed;
- **(i): a verdict word judges a stated expectation.** Without one, `ein
  solve` explores and describes R;
- **(g): `exhausted` means the reported region is all of R.** A limited run
  is partial and says so.

What is left to decide is how each surface says it.

## 2. Today's surfaces

| surface | what it prints today | what it counts | why it cannot stay |
|---|---|---|---|
| verdict block (`ein solve`) | `solutions (k)`, `verdict` and one block per recorded node (`query bindings`, `query facts \| rendered`, `result`) | **the walk's recorded nodes** | it names no line of R; a word is printed with no expectation; *(no query goal to project)* for a state where the goal has no row (P1e2.3 S6); one goal row of several, unsorted (S7) |
| the region modes' interim layout (T1e2.1a.9.4) | `region`, `route`, `program`, `licences`, `k_s`, `k_c`, `∅ in R`, the shape counts, `answer`, `exhausted`; states as candidate lists, at most 64 | R or one line | interim by design: no claim form, no query, a cap |
| `ein oracle` | the interim layout, plus `holes` and the cube's violations | R | the same |
| `--json-summary` (`ein-summary/1`) | `verdict {type, k, exhausted, reason, unsat_core, solutions, open_states}`, `stats` (the walk's counters), `owes`, `monotonicity`, `leftover`, `root`, `config` | the walk | `verdict.type = "Contradiction"` on a truncated run (S3); `k` |
| `--dump-answer` (`ein-answer/1`) | `index.json {verdict, k, exhausted, models, open, refuted, alive_at_end}`, `models/NN.ein`, `root_writes.json` | the walk | `k`; root writes, which no mode makes |
| `--models list\|key\|grid` | the recorded models as blocks, a determining key, or tables | the walk's | which set it is drawn from is not said |
| `--trace` | `# Solution trace` / `# Open trace`, headline *"Solved."*, *"Ambiguous — k …"*, *"No solution — the constraints are contradictory"*; root's steps, the hypothesis, refuted hypotheses | the walk's proof | *No solution* on a run with 39 models (S2) |
| `ein test` report (`ein-test-report/2`) | query rows: `ran {verdict, k, solution_nodes, exhausted, layers, enterings, …}`; solve rows: `region {route, monotone, k_s, k_c}` | both | the query half is the walk's |
| events | `enter`, `layer`, `nogood`, `writeback`, `traversal`, `verdict` | the walk | the walk goes |
| exit codes | 0 for all four words; 1 for a false `:expect` or a load error; 2 for usage or an abort | — | a word with no expectation cannot fail, so nothing can exit 1 for it |

## 3. Five rules every surface follows

1. **It says what was asked, and what it lists.** Which part of R (all of R,
   the minimal line, the maximal line) and which question (describe, N
   states, exactly one). That is the `asked` row on stdout, `asked` in the
   summary, and the line a dump's `index.json` names.
2. **The counts are R's, named k_s and k_c, and only R's.** They are printed
   when the run described R. A mode counts its own line, under the line's
   name: `maximal line  3 states — all of them`. A bound says it is one, and
   why.
3. **No verdict word without an expectation.** The four words are
   `--single`'s answer (§ 11, question 1). A claim the program states is
   judged `holds`, `FAILED` or `NOT CHECKED`, as it is today.
4. **Every printed state is a member of R (C1). It is marked as a line member
   only where that is certified.** The `(region …)` form, which reads back as a
   claim that these are *all* of R, is printed only for an exhausted
   description. A line, an `-n` cut or a partial run is listed as states, each
   a `(model …)` with a comment, and never wrapped in `(region …)`.
5. **Completeness is one row, `exhausted`**, and `decided` under `--single`.
   When it is *no*, the row gives the reason: stopped as asked (`-n`), at the
   budget (with what was spent and what the next step projects), past
   `--region-limit`, or `-T`, which aborts with exit 2.

## 4. The default read-out, `ein solve FILE`

[X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once)
is every row on one program. The block keeps today's header: `solve · ARGS`
and the rule under it, so a transcript still starts the same way.

| row | says | example (X1 unless named) |
|---|---|---|
| `asked` | the question, rule 1 | `all of R — describe` |
| `states (k_s)` | \|R\| in states. *A lower bound* on a partial run, with what is complete | `5`; zebra2 at the default budget: `1   (a lower bound: complete up to commitments of 4 candidates)` (X10) |
| `commitments (k_c)` | \|R\| in commitments, the same way | `8` |
| `exhausted` | rule 5 | `yes — every commitment is classified: this is all of R` |
| `∅` | where root stands: in R, owing n, or dead | `in R — root is alive and owes nothing`; X5: `not in R — root owes 1 instance` |
| `empty` | instead of `∅`, when R is empty: which kind (§ 6.1) | X6: `every alive node owes — …`; X7: `every node is dead — …` |
| `minimal line` | its size in states. Named *trivial* when it is {∅} because no obligation is stated (ruling (f)); otherwise how many minimal commitments reach it and the smallest, which is *the least guess* | `1 state — ∅, the trivial line: no obligation is stated`; zebra2: `1 state, from 63 commitments — the least guess is 1 candidate` |
| `maximal line` | its size in states, and ⊤ named trivial when nothing dies | `1 state`; floors: `1 state — ⊤, every candidate: no node is dead, the trivial line, certified by one saturation` (X11) |
| `components` | how many, and their sizes in commitments when there are several | X4: `8, each a single commitment`; X5: `2, with 8 + 8 commitments` |
| `sub-cubes` | how many maximal sub-cubes, and their dimensions | X2: `3, each of dimension 2`; zebra2: `63, 4 of dimension 17, 59 of dimension 16` |
| `merged` | how many states more than one commitment reaches, and their minimal generators | `1 state reached by more than one commitment, with 3 minimal` |
| `N` | the minimal dead sets | `6 minimal dead sets` |
| `E` | the minimal discharging sets. **Cube only**: route 1 computes only E's alive members, which are the minimal line | X8: `1 minimal discharging set` |
| `lattice` | the static check, and through S1e2.1a.1's stub the holes (§ 6.3) | `monotone (the static check) — R is convex: no hole` |
| `universe` | n, and how many candidates are dead alone, which no state can hold | `9 candidates, 6 dead alone`; X6: `no candidate: R is {∅} or empty, decided by root alone` |
| `route` | the route, and the licences it used | `1, the alive down-set — licence L1, dead is upward-closed` |
| `work` | saturations, facts beyond root and layers, as the interface's § 2.1 asks of every run. They do not depend on the machine or on `-j` (`every_mode_at_four_jobs_is_every_mode_at_one`). A speed lever may move them, which C4 allows (*cost counters aside*), so the lever sweeps compare every row but this one (§ 12). The wall clock stays behind `-s` and `-t` | `14 saturations · 66 facts beyond root · 4 layers` |

**Then the region itself**, in the form S1e2.1a.4 reads back:

```lisp
  (region
    ;; state 1 of 5 — ∅, root itself · minimal
    (model …)
    ;; state 5 of 5 — 3 candidates · maximal · 4 commitments, 3 minimal
    (model …)
    ;; state 5 is reached by 4 commitments; its 3 minimal
    (merged (model …)
            (or (model (co-located Bob Coffee) (co-located Bob Dog))
                …)))
```

- **Every state of R, then every `(merged …)`**, the third answer's form,
  printed by the same rules as `claim::print`: each `(model …)` lists the
  positive facts of the **candidates' relations**, root's included, which is
  what tells R's states apart. `claim_round_trip.rs` already holds that
  `claim::print`'s text reads back as its own claim on 165 corpus programs.
  The preview's layout, comments and order included, was read back the same
  way on the nine examples that print a region or `(false)`: all nine hold.
- **One comment per state**, not part of the claim: its number, its size in
  candidates, the lines it is on, and either the commitment that reaches it or
  its class size and minimal count.
- **States are ordered by closure size, then by their candidates** in the
  universe's order, which is canonical text order. The order does not depend
  on the route, a lever or `-j` (C4).

**Then each query** (§ 7), then **each claim the program states**, checked
against what the run computed (§ 7.3).

## 5. The modes

[X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode)
runs one program under each.

| mode | `asked` | count row | listing |
|---|---|---|---|
| `--maximal` | `the maximal line — every state on it` | `maximal line  3 states — all of them` | each state as `(model …)`, `;; … · maximal` |
| `--minimal` | `the minimal line — every state on it` | `minimal line  1 state — all of them` | each state, with its minimal commitments |
| `-n N` | `N states of R (-n N)` | `states of R  2 states, as asked — R may hold more` | the first N in the route's order |
| `--maximal -n N`, `--minimal -n N` | the same, of the line | `maximal line  1 state, as asked — the maximal line may hold more` | the same |
| `--single` | `is R one state? (--single)` | `verdict  Ambiguity — R holds two states or more; two of them below` and `decided  yes — two states settle it; …` | the one state, or the two |

- **k_s and k_c are not printed under a mode.** They are R's, and a mode did
  not describe R (rule 2). `--single`'s `Solution` certifies that the part it
  asked about is one state, and its row says which: `Solution — R is one
  state`, or `— the maximal line is one state`.
- **`route`, `lattice`, `universe` and `work` are printed under every mode**,
  as the interface's § 2.1 asks. The cost of a question is visible on its
  own run: zebra2's `--maximal -n 1` is 54 saturations, and `--single` is
  8 616 (X10).
- **A line is not a claim.** No `(region …)` wraps it, because read back it
  would say that these states are all of R. The one form that names a line is
  a query's `:over`, and its answer is printed (§ 7).

## 6. Empty R, a partial run, a hole

### 6.1 Empty R: two kinds, described

The 2026-08-25 ruling named them `Contradiction` and `Open`. Under (i) a
word judges an expectation, so the exploring read-out **describes** them and
prints no word (§ 11, question 1):

| kind | row | why, printed under it | the form |
|---|---|---|---|
| no alive node | `empty  every node is dead — root's own fixpoint derives (false)` | `unsat core` and `refuted by`, root's | `(false)` |
| alive nodes, all owing | `empty  every alive node owes — no commitment pays every debt` | `owing`: each node of the frontier (alive, owing, nothing alive one fact up), with every instance it owes through the rule's `:why` | `(region (owing (model …) …))` |

[X6](read_out_examples.md#x6--testsstdlibclosure03_closed_and_owingein-r-empty-alive-nodes-owe)
and [X7](read_out_examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead)
show both. `--single` on each says `Open` and `Contradiction`: there the word
judges the run's own expectation of one state.

### 6.2 A partial run (C3)

A run a budget stopped prints what it **certified**, and bounds for the rest.
It prints nothing it merely did not reach. Route 1 stops between layers, so
everything up to its last complete layer L is exact. Each row is certified as
follows:

| row | on a partial route-1 run | why it holds |
|---|---|---|
| `states (k_s)`, `commitments (k_c)` | `≥`, *complete up to commitments of L candidates* | every member of R with a generator of size ≤ L was visited |
| `minimal line` | exact when ∅ ∈ R (it is {∅}). Otherwise `≥ m — complete up to commitments of L candidates` | a minimal commitment's subsets are all visited before it |
| `maximal line` | `1 state — ⊤ …` when one saturation of ⊤ finds it in R. Otherwise the states certified maximal, `— all those a commitment of up to L − 1 candidates reaches`, or `not reached` | L1 and L2 for ⊤; L4 for the rest: `C ∪ {x}` with x outside the closure is visited dead or never generated |
| `components`, `sub-cubes`, `merged` | `not known` | they need all of R |
| `N` | `≥ … — complete up to size L` | a minimal dead set's subsets are all alive, so it is generated |
| the listing | **the certified line members only**, as `(model …)`s, **at most 64**, with one comment saying how many more there are and that `--dump-answer` writes them | rule 4; the floors run met 10 075 other members (X11), and they are not the answer to *describe* |
| queries | over every member met, marked *a lower bound*, at most 64 classes, and **no `as :expect`** | each class shown is real; there may be more, and a pasted claim would say there are not |

[X10](read_out_examples.md#x10--zebra2ein-the-puzzle) is zebra2 at the
default budget. It is partial, and it has answered the puzzle: its one state is
certified minimal (from the single fact `(drink-loc Juice House-4)`) and
maximal, and the query's class is printed with its label.
[X11](read_out_examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube)
is a cube that is all of R. Both trivial lines are certified, the first by
root and the second by one saturation of ⊤.

**A program the static check flags, past `--region-limit`**
([X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit)):
the run visits the cube in layer order up to 2^limit nodes. One line is still
certified without a licence: a member met in layer order with nothing in R
below it is minimal, since every subset was met first, and when ∅ is in R the
minimal line is exactly {∅}. Those are what it lists. The maximal line needs
the whole cube, and the row says so. The `exhausted` row names the route that
declined and why: `no — not monotone, and the question needs the whole cube:
2^19, past --region-limit 16`. **It runs no walk.**

**Why the cap applies to partial runs only.** Measured with the preview over
every corpus program that loads, 198 of them, at the default budget
(2026-10-16): without a cap, the flagged programs past the limit listed every
member they met, up to 64 761 states and 648 435 lines
(`square-unique/terminus.ein`). Listing only the certified line members does
not help everywhere: all 64 761 members `features/04_open.ein` meets are
minimal, which is 585 192 lines. With the cap the longest partial run is 646
lines. A partial listing is evidence, and every member is in `--dump-answer`.
An exhausted description is the answer, and question 2 of § 11 asks whether
it is ever capped.

### 6.3 Holes: the stub, never silent

On a flagged program that the cube reaches
([X8](read_out_examples.md#x8--ein-bugsnaf-upward-closureein-a-hole)), the
`lattice` row is S1e2.1a.1's stub, extended with what the cube saw:

```
  lattice           not monotone: 1 site (the static check) — 1 hole; assumes a monotone lattice for what a node is (P1e2.1b)
    minimal hole    {(p A)} — dead: (p A)
```

Every minimal hole is named with why it is out (its core, or what it owes),
up to four, and the count is always printed. The `(region …)` listing is the
cube's R either way: it lists states, so a hole needs no special form. What
*assumes a monotone lattice* is the node itself, root's fixpoint plus the
commitment, which P1e2.1b's S1e2.1b.2 may change. S1e2.1b.3 rules on (d) and
replaces the stub.

## 7. Queries: projection classes

### 7.1 One solve, every query

R is computed once, and every `(query …)` is answered from it (the diff's
§ 3, *several queries share one solve*). Today `run_query` loads and solves
once per query, and refuses `--json-summary` and its siblings beside two
queries (`ONE_PATH_FLAGS`). Both go: one run has one summary.

### 7.2 What a query block says

```
  query             (and (co-located ?n Coffee) (co-located ?n ?p) (is-a ?p Pet))
  over              all of R — 5 states → 2 projection classes
    class 1 of 2    4 states (1, 2, 3, 4) — the goal has no row: undetermined here
    class 2 of 2    1 state (5)
                      ?n = Bob · ?p = Dog
  as :expect        (or (model)
                        (model (co-located Bob Coffee) (co-located Bob Dog)
                               (is-a Dog Pet)))
```

- **`over`** names the set and how many states it holds, then how many
  classes they fall into. The default set is all of R. A query's `:over`
  picks a line, which lands at the switch (the diff's § 5). Where the run
  listed a different set (a mode, an `-n` cut), the query is answered over
  what was listed, and one line says so.
- **One class per row block**: how many states give it, and which, by their
  numbers in the listing above. Then every binding row, sorted, with
  `:goal-text` rendered per row when the query has one. The class where the
  goal has no row says so: *the goal has no row: undetermined here*. That is
  P1e2.3's S6, and printing every row, sorted, is its S7.
- **`as :expect`** is the claim that states exactly these classes, in the
  query's `:expect` form. It is the second direction of *one form, both
  directions*, so it can be pasted into the query. It is printed only when the
  set it is over is complete: on a partial run or an `-n` cut the classes are
  a lower bound, and the row says *not printed* and why.
- **Today's `query facts | rendered` table and the `(query has no
  :goal-text template)` line go.** A class's facts are its `as :expect`
  line, and `:goal-text` renders each row.

**A class is the goal's atoms, so a goal that names a fact cannot hide it.**
The user's example of hidden seats needs a goal without the seat, which is a
derived relation in today's language
([X4](read_out_examples.md#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes):
`(sits ?p ?s)` gives 8 classes, and `(pair-at ?g ?t)` gives 2 classes of 4).
A select-list on the query is a new form, and not built here (§ 12).

### 7.3 Claims, checked by `ein solve`

Since S1e2.1a.4 `ein test` checks a `(solve … :expect …)`, and `ein solve` does
not until the switch. After it, `ein solve` checks every claim the program
states against what the run computed. Both a flag and a claim are checked,
and neither wins ([`region_language.md` § 6](region_language.md#6-a-flag-beside-a-form)).
The preview prints the first row below (X12). The other two are sketches of
the reasons T1e2.1a.5.2 writes, since the query's new `:expect` does not
parse yet:

```
  (solve :expect)   holds
  :expect           FAILED — the claim lists 1 class, the query has 2: (model) is not listed
  :expect           NOT CHECKED — the claim is about all of R, and this run listed the maximal line
```

Each row sits under the block it is about: the solve claim after the region,
and a query's `:expect` under its query. A claim that is not `holds` writes its
one stderr line and exits 1, as M1d S1d.4.3 requires.

## 8. Every other surface

### 8.1 `--json-summary`: `ein-summary/2`

The same facts as stdout, as data. One summary per run, and the fields below
replace `verdict` (with its `solutions` and `open_states`), `stats` and
`owes`:

```
asked     {which: all|minimal|maximal, ask: describe|find|single, n}
region    {k_s, k_c, exhausted, stopped: null | {why: asked|budget|limit|deadline, …},
           root: in-r|owing|dead, empty: null|dead|owing,
           minimal: {states, commitments, least, trivial}, maximal: {states, trivial},
           components: [sizes], subcubes: [dimensions], merged, nogoods, discharging, holes,
           route, licences,
           states:   [{facts, closure, from, minimal_generators, commitments, lines: [minimal, maximal], owes}],
           frontier: [{facts, owes: [{relation, why}]}],      (empty R, owing kind)
           core:     [facts], refuted_by: [rules]}             (empty R, dead kind)
verdict   null | {word, which}                                (--single only)
queries   [{goal, over, exact, classes: [{facts, rows, states}], expect: null | {outcome, lines}}]
claim     null | {outcome, lines}                             (the solve claim)
work      {saturations, facts, layers}
```

`monotonicity`, `root` and `config` stay as they are. Every bound carries
`exhausted = false` beside it: a reader never has to know which numbers were
complete.

### 8.2 `--dump-answer DIR`: `ein-answer/2`

- **`models/NN.ein`**, one loadable program per **listed** state, as today:
  the program, then the state's facts. Its header comment gives the state's
  number, its lines, its commitment and its minimal generators. On a partial
  run it also writes every other member found, which is where X11's 10 075
  go.
- **`index.json`** names what `models/` holds: `asked`, the same `region`
  block as the summary, and per file its `lines`. That is the acceptance's
  *`models/` per line, named in `index.json`*.
- **`owing/NN.ein`** holds the frontier on an empty R of the owing kind, and
  **`nogoods.json`** holds N with each core.
- **`root_writes.json` and `alive_at_end.json` go.** Root is never written,
  and there is no walk to end.
- `answer.json` stays the summary byte for byte, as `answer_dump.rs` checks.

### 8.3 `--models list|key|grid`

All three draw from **the listed states**, and the block's first line says
which set that is:

- **`list`** is the default: the `(region …)` form or the members;
- **`key`** is the determining key. Since the interface's § 4.3 handed it the
  certain core, it also prints the facts every listed state shares;
- **`grid`** is unchanged per state.

### 8.4 `--trace FILE.md`

- **Title and headline**: `# Region trace`, and the `asked`, count and
  `exhausted` rows as the headline. *No solution* can no longer be printed
  over a run with models (S2).
- **Sections**: root's derivation; then each listed state's derivation from
  its first minimal commitment; then **N**, each minimal dead set with its
  core and refuting rule, in place of *Refuted hypotheses*; then **owing**
  when R is empty of that kind.
- **Cost**: one saturation per listed state, recorded. A description is
  traced as far as it lists.

### 8.5 `ein test`

- **Text**: unchanged, `ok` / `FAILED` / `NOT CHECKED` per file.
- **Report**: `ein-test-report/3`. A query row's `ran` (the walk's counters)
  becomes `answer {over, exact, classes, states}`. Solve rows are unchanged.
- **Each claim runs the mode that decides it, and no more**, as the
  interface's § 2.3 says.

### 8.6 The rest

| surface | after the switch |
|---|---|
| events (`--events`) | `verdict` becomes **`region`**: the summary's `region` block without `states`. `enter` is one per node visited and `layer` one per route-1 layer, with route 1's counters. `nogood`, `writeback` and `traversal` go with the walk. [`events.md`](../../../docs/kernel/inference/events.md) is rewritten in T1e2.1a.5.2 |
| `-s` | the wall clock per phase, and route 1's per-layer table (visited, dead, owing, in R, facts). The walk's counters go |
| `-p`, `-P` | each listed state's facts beyond root, all of them or the positive ones: its **model**, by the definition. `(model …)` above names only the candidates' relations |
| `-f` | each listed state's candidates, its closure |
| `--diff-models FILE` | the listed states against the file's, with the set named on its first line |
| `-H` | H, the universe: the candidates per relation and those dead alone, read from the oracle's list. That is P1e2.3's S4, checked in T1e2.1a.5.2 |
| `ein oracle` | the describe layout, with `route  2, the cube — no licence: asked for by name`, the `E` row, every hole row, and today's `the cube` line of violations. It gains `-T`, `--json-summary`, `--models`, `-p` and `-P` (interface § 6) |

## 9. Exit codes

| run | 0 | 1 | 2 |
|---|---|---|---|
| describe, `-n`, `--minimal`, `--maximal` | always, **empty R and partial included**: a request is not an expectation, and a stopped run is an answer that says *partial* ([`solve_flags.md` § 10](solve_flags.md#10-decided-here-unless-the-user-objects)) | a claim the program states is `FAILED` or `NOT CHECKED` | usage, `-T` |
| `--single` | `Solution` | `Ambiguity`, `Contradiction`, `Open`, not decided | usage, `-T` |
| a load error | — | `ein solve` | `ein test`, unchanged |

On stderr there is one line per claim that is not `holds`, and one per
`--single` that did not hold. A partial run adds one `note:` line, so that a
pipeline sees it:

```
note: examples/zebra2.ein: partial — route 1 stopped before layer 5 at --region-budget 524288
```

## 10. Which of the user's questions the default answers

The user's list and the plan's additions, from
[§ What `solve` can answer about R](../open_questions.md#what-solve-can-answer-about-r-2026-10-16).
That page says *"which of them `ein solve` answers by default, and which
behind a flag, is S1e2.1a.5's to propose and the user's to rule"*, which is
question 3 of § 11.

| question | by default (describe) | by a mode or flag | not built |
|---|---|---|---|
| does R have a node? | `states (k_s)`; `empty` and its kind | `-n 1` | |
| is it the only one? | k_s = 1; `maximal line` 1 | `--single`; `--maximal --single` | |
| how many? | `states (k_s)`, `commitments (k_c)` | | |
| is a line trivial? | *the trivial line* on `minimal line` / `maximal line` | | |
| how many minimal / maximal, and which? | the line rows; `· minimal` / `· maximal` on each state | `--minimal`, `--maximal` | |
| components; is it one? | `components` | | |
| sub-cube dimensions | `sub-cubes` | | |
| enumerate all / min / max | the `(region …)` | `--minimal`, `--maximal` | |
| monotone? | `lattice` | `ein oracle` checks the verdict against the cube | |
| if R is empty, which kind and why? | `empty`, and the core or the owing frontier | | |
| holes? | `lattice` and the minimal holes, on the cube | `ein oracle` | |
| which commitments give one state? | `(merged …)` | | |
| how much must be guessed? | *the least guess*; each state's `from` | `--minimal` | |
| which facts hold in every solution, in some, in none? | | `--models key`: the certain core and the key | the skeptical / credulous / impossible sets as such |
| which facts decide the others? | | `--models key` | |
| what does a query's goal see? | each query's classes | `:over` | |
| which rule shapes which part? | `N`'s count | `--trace`: N with cores | a rule chain per minimal owing node |
| which rules or facts can be dropped? | | | R per rule |
| what would make R a single state? | | | |
| is R symmetric? | | | W5's question |
| how far apart are the solutions? | | | |
| is the answer complete? | `exhausted` | | N and E printed as a certificate (E is the cube's) |
| what is the universe? | `universe` | `-H` | |

The *not built* column stays where Q-M1e2.7's table filed it. The phase's
risk list makes any of them a followup.

## 11. What needs the user's ruling

1. **Where the four words appear.**
   - **(A), proposed: only as `--single`'s answer.** The exploring read-out
     describes empty R by its kind (§ 6.1). `-n N` is a request, not an
     expectation: it exits 0 and says *R is empty* when it is. A program's
     claims are judged `holds` / `FAILED` / `NOT CHECKED`, with the reason
     in plain counts.
   - (B) As (A), and also in a false claim's reason when the claim stated one
     state or some: `FAILED — Ambiguity: the claim lists 1 state, R has 2`.
   - (C) Keep `Contradiction` and `Open` as the names of the two kinds of
     empty R in the exploring read-out too.

   (A) keeps (i) literal: one place where an expectation of one is stated,
   one place where the words judge it.
2. **How much of an exhausted R stdout lists.**
   - **(A), proposed: every state**, in the `(region …)` form, after the
     summary rows, and every class of every query. `--models key|grid` are the
     compact forms, and `| head` is a glance.
   - (B) The same cap as a partial run's: 64 states and 64 classes, then *… n
     more*, and the whole `(region …)` in `--dump-answer` only.

   Measured over the 198 corpus programs that load, at the default budget
   (2026-10-16): 178 exhaust. Their read-outs are 29 lines at the median and
   107 at the 90th percentile. Seven have more than 64 states, and three are
   long: `type-exclusivity/pets.ein` (11 339 states, 44 457 lines),
   `syntax/constraint-scopes.ein` (4 096 states, 17 430 lines) and
   `features/02_star_in_identifiers.ein` (728, 15 957). (B) would make the
   default describe those three in part, which is what the default was ruled
   not to do. The partial runs are capped either way (§ 6.2).
3. **The default's answers: § 10's *by default* column.** Every row of it is
   computed by route 1 or the cube anyway, so printing it costs no work.
   Proposed as it stands. A row the user does not want moves behind `-s`.

## 12. Decided here unless the user objects

- **The labels**: `states (k_s)` and `commitments (k_c)`, mirroring today's
  `solutions (k)`. `exhausted` everywhere, `decided` under `--single`. `asked`
  first.
- **A `(region …)` is printed only for an exhausted description** (rule 4).
  Lines, cuts and partial runs list states.
- **The state order is canonical**: closure size, then candidate order.
- **A printed `(model …)` closes the candidates' relations**, as
  `claim::print` does. A claim a program writes may name more: X6's fixture
  pins `(closed-here r)` where the printed form is `(model)`.
- **A partial run lists its certified line members, at most 64**, and
  certifies ⊤ with one saturation when ⊤ is in R: L6's trivial line, for the
  price of one fixpoint.
- **`work` is printed by default**, since it does not depend on the machine.
  The wall clock stays behind `-s` and `-t`. It is the one row C4 lets a
  speed lever move, so `jobs_invariance`, `id_order_invariance` and
  S1e2.1a.6's lever test compare every other row.
- **No select-list on a query.** Hiding a fact takes a derived relation
  (§ 7.2). A `:show (?var …)` form would be new, and goes in the followups
  if a program needs it.
- **`--json-summary` is `ein-summary/2`, `--dump-answer` is `ein-answer/2`,
  and the test report is `ein-test-report/3`**, with § 8's fields. Every
  reader moves in the switch commit: the Rust tests, and the `utils/` scripts
  that read `verdict` or `stats`.
- **The `verdict` event becomes `region`**, and the walk's event kinds go.
- **A partial run writes one `note:` to stderr** and exits 0.
- **P1e2.3, in passing.** The switch closes S1 (its hint goes with `-e`), S2,
  S3, S5 (with `-m` and `-E`), S6, S7 and S9 (with the tree). S4 and S8 are
  checked in T1e2.1a.5.2. P1 is not a read-out. Q-M1e2.6 gets this note when
  the switch lands, as the stage's notes ask.
- **`leftover` (`EIN_LEFTOVER=1`) stays, re-read per listed state**: the
  blind rung's facts outside H, which is a question about the universe (h),
  not about the walk.

## 13. What moves, and the re-estimate

**Named here so that T1e2.1a.5.3 can count it, not yet counted:**

- **the stdout block**: `read_out.rs`'s seven cells, `corpus_exits.txt`
  (where a program's claims are now checked by `solve`), `guide_transcripts.rs`
  (`docs/guide/04` and the top-level `README.md`), `embedding.rs` and
  `docs/api/rust.md`, `acceptance_cli.rs`, `cli_semantics.rs`, `expect_cli.rs`,
  `test_cli.rs`, `region_modes.rs`, `model_set_report.rs` and `models_grid.rs`;
- **the summary and the dump**: `summary_properties.rs`, whose identities
  become the region's (k_c ≥ k_s; line sizes ≤ k_s; Σ layer visits =
  saturations), `answer_dump.rs`, `wrong_answers.rs`, `leftover_probe.rs`,
  and five `utils/` readers (`feature_matrix.py`, `region_census.py`,
  `closure_census.py`, `openness_census.py`, `model_set_census.py`), plus
  `review_bundle.py`;
- **the trace**: the six `trace_*.md` goldens in `ein-render/tests/golden/`
  and `corpus_shapes.md5`'s trace and solve renderings;
- **the pages**: `solution_semantics.md` § 5, `answer_dump.md`, `events.md`,
  `configuration.md` (options, the schema names), `defined_behaviour.md` and
  the guide.

**Re-estimate: 5 days, from 2.**

| task | days | what |
|---|---:|---|
| T1e2.1a.5.1 | 0.5 | this page, the preview, the examples |
| T1e2.1a.5.2 | 2.5 | the printer in `ein-render`, from the preview: the block, the region and members, the queries and their classes; the summary, dump, report, trace, events, `-p/-P/-f`, `--models`, `--diff-models`; `ein oracle`'s layout and options |
| T1e2.1a.5.3 | 1.5 | the switch: retirements, the corpus cells, the option census, the query `:expect` and `:over`, every claim's migration, every golden named first and moved |
| T1e2.1a.5.4 | 0.5 | the register: 46 runs re-pointed, rows flipped against S1e2.1a.10's `right` blocks |

The preview is deleted when T1e2.1a.5.2's printer replaces it.
