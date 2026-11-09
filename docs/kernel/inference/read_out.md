# The read-out — every row `ein solve` prints, and what it counts

**What this page is.** The reference for what `ein solve` writes on stdout and stderr in each of its questions,
what `ein oracle` writes, and the answer blocks of `--json-summary`: every row's label, when it is printed, what it
says, which kind of thing its numbers count, and the summary field that holds the same number. Its printer is
[`ein_render::read_out`](../../../ein.rs/crates/ein-render/src/read_out.rs), whose `Row` enum is the list of labels.
**Current since M1f [S1f.5.16](../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md)
(2026-11-09, `79fc14b0`; its test since `d01c6588`).** The values are on the examples page, [`read_out_examples.md`](read_out_examples.md), every block of
it one run of the binary, written by [`utils/read_out_examples.py`](../../../utils/read_out_examples.py) and held to
the binary by `ein-cli/tests/read_out_examples.rs`. This page is held to the printer by
`ein-cli/tests/read_out_reference.rs`:

1. every label the printer has is a row of the table its section names, and every row is a label, in printing order;
2. every label is printed, at its indent, by a block of the examples page, and the clock rows by one run with the
   times masked;
3. every field of `schema`, `source`, `asked`, `region`, `verdict`, `queries` and `claim` is named here, and every
   field named here is in a summary; the owners table names every top-level key;
4. on every example run, every row whose summary column names a field prints that field's number.

**What it is not.** What R, k_s, k_c, a line and a model are is
[`solution_semantics.md`](solution_semantics.md)'s. Exit codes, and which stream each byte goes to, are
[`defined_behaviour.md` § 4](../defined_behaviour.md#4-errors-and-exit-codes) and
[§ 5](../defined_behaviour.md#5-the-cli-surface). The options and their defaults are
[`configuration.md`](../configuration.md)'s. The `--dump-answer` directory and the summary's `work` and `root` blocks
are [`answer_dump.md`](answer_dump.md)'s; the `monotonicity` block and the warning's theory are
[`region_dump.md` § Monotonicity](region_dump.md#monotonicity--the-static-check)'s; the `--events` stream is
[`events.md`](events.md)'s. How each question is computed and what it costs is
[`solve_interface.md`](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md) § 3 and § 4.
The rulings that made this vocabulary, on 2026-10-16, and what M1e2's stages amended, are the record in M1e2's
[`read_out.md`](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md), which is not kept current.
`--trace`'s file, `ein test`'s text and report, and the event stream repeat rows this page defines, and § 12 says
which.

---

## 1. How a row reads

The read-out opens with a header, the command and its arguments as typed, and a rule under it. Then come the rows,
each a label and a value: a row at the left margin at two spaces, a row of a block under it at four, every value at
column 20, or one space past a label as long as its column or longer, so that no label touches its value (M1f
S1f.5.6, W42: only a numbered label is that long, `class i of n` at a block's sixteen characters, which glued to its
count until then). A row's value that runs over a line break of a text the program wrote (a `:goal-text`'s sentence, an
obligation's `:why`) continues at the column that text starts at (M1f S1f.5.16's T1f.5.16.5). After the rows come the
listing, the queries, the claims, and on request the forms (§ 6 to § 8).

**Every row says what its numbers count**, one of four kinds:

| kind | what it counts |
|---|---|
| **R** | states or commitments of R, or of the line or the part the question names; and whether ∅ is in R, or which kind of empty R is |
| **around R** | the cube and the universe: candidates, and nodes and sets of the cube. None of them is a state of R |
| **the run** | what the search did: saturations, facts, layers, nodes met, facts spent and projected, parents checked, the wall clock |
| **the listing** | what stdout shows and what it leaves out |

A row marked **—** counts nothing: it says what was asked, or a verdict on a claim.

**One label, one number.** SE-M1 printed `solutions (k)` above `solution_nodes`, so one run printed two numbers under
one word, and AR-M2 had three crates each choosing which number to print. The rule they ended in: each invocation
prints one label for one number, and every number a row prints that the summary also holds is the summary's field.
The tables' **summary** column names that field: its number is the integer, or an array's length, or each string of a
list; a zero may print as a word (*none*, *in R*, *no hole*). In a path, `[]` is every element, `[k]` the element
paired with the k-th row of that label in the run (from 0), `[q]` the query whose block the row is in, and `[i]` the
row's own number less one. A field no row prints as itself is in § 11.2, with where it shows.

**No table quotes a printed value.** The values are the examples' (the **example** column), so a stage that rewords a
value re-blesses an example and has no prose here to chase. What a table states is what each row means.

---

## 2. Describe: a bare `ein solve`, `--count`, and `ein oracle`

A bare `ein solve FILE` describes all of R. `ein oracle FILE` prints the same rows over the cube, route 2 asked for by
name, then `the cube` (§ 9). `--count` prints `asked`, `states (k_s)`, `commitments (k_c)`, `exhausted`, `empty` or
`∅`, `lattice` with its block, `strata`, `universe`, `route` and `work`, each describe's text, then `claims` (§ 7) and
no listing: it reads no state.

<!-- rows: describe -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `asked` | every run, first | the question: all of R, described, or how many states and commitments it holds | — | `asked.n` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once) |
| `states (k_s)` | describe, `--count`, `ein oracle` | R's size in states; on a run that did not exhaust, a lower bound, and how far it is complete | R | `region.k_s` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X11](read_out_examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `commitments (k_c)` | as `states (k_s)` | R's size in commitments, qualified the same way, and exact where R is decided as the whole cube | R | `region.k_c` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X11](read_out_examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `exhausted` | as `states (k_s)` | whether the run classified every commitment, so that these are all of R; when not, why: the state route's or route 1's budget, with the facts spent and what the next step projects, or a flagged program past `--region-limit` | the run | `region.exhausted`, `region.stopped.spent`, `region.stopped.projected`, `region.stopped.commitments`, `region.stopped.n`, `region.stopped.limit` | [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit), [X11](read_out_examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `empty` | in place of `∅`, when R is exhausted and empty | which kind of empty: every node dead, root's own fixpoint deriving `(false)`, or every alive node owing | R | — | [X6](read_out_examples.md#x6--testsstdlibclosure03_closed_and_owingein-r-empty-alive-nodes-owe), [X7](read_out_examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) |
| `∅` | otherwise | where root stands: in R, not in R and how many instances it owes, or dead | R | `region.root_owes` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X5](read_out_examples.md#x5--latticesynthmerged_statesein-every-state-merged) |
| `minimal line` | describe and `ein oracle`, unless R is exhausted and empty | the minimal line's size in states: where ∅ is in R, `1 state — ∅`, on every program and at every limit, the `∅` row above it saying why (M1f S1f.5.6: until then a clause beside it, *the trivial line: no obligation is stated*, read whether a node the run met owed); on an exhausted R without ∅, how many minimal commitments reach it and the size of the smallest, *the least guess*; on a partial run, a lower bound and how far it is complete | R | `region.minimal.states` | [X5](read_out_examples.md#x5--latticesynthmerged_statesein-every-state-merged), [X10](read_out_examples.md#x10--zebra2ein-the-puzzle) |
| `maximal line` | as `minimal line` | the maximal line's size in states, ⊤ *trivial* when nothing dies; on a partial run what is certified, or *not known* where the line needs the whole cube | R | `region.maximal.states` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit), [X11](read_out_examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `components` | as `minimal line` | how many components R has under one-fact edges, and their sizes in commitments where there are several; *not known* on a partial run | R | `region.components` | [X4](read_out_examples.md#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes), [X5](read_out_examples.md#x5--latticesynthmerged_statesein-every-state-merged) |
| `sub-cubes` | as `minimal line` | how many maximal sub-cubes R has, and their dimensions; *not known* on a partial run or past the interval search's bound | R | `region.subcubes` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `merged` | as `minimal line` | how many states more than one commitment reaches, and how many minimal commitments they have between them; *none* where k_c = k_s; *not known* on a partial run | R | `region.merged.states`, `region.merged.minimal` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X5](read_out_examples.md#x5--latticesynthmerged_statesein-every-state-merged) |
| `N` | as `minimal line` | the minimal dead sets; on a partial run, a lower bound and which of them it holds | around R | `region.nogoods` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `E` | describe on route 2, and `ein oracle` | the minimal discharging sets, which only the cube computes | around R | `region.discharging` | [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
| `lattice` | every run but `--monotone` | the static check's verdict; on a flagged program its sites, and the holes the cube shows or that they are not known; then the block of § 5.1 | around R | `monotonicity.sites`, `region.holes.count` | [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole), [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `strata` | when the rules' dependency graph has a cycle through an `absent` | how many such cycles; a diagnostic, not a refusal: root's fixpoint may depend on the boundary's order. The block of § 5.2 names each | around R | — | [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `minimal hole` | describe on route 2 and `ein oracle`, when the cube shows a hole; at the block's indent, up to four | a minimal hole, and why it is out: dead, with its core, or what it owes | around R | `region.holes.minimal[k].owes`, `region.holes.minimal[k].core` | [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
| `universe` | every run | H's size in candidates and how many are dead alone, or that the run did not visit each alone; *no candidate* when H is empty, which decides R by root alone. The block of § 5.3 under it | around R | `region.universe.candidates`, `region.universe.dead_alone` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X6](read_out_examples.md#x6--testsstdlibclosure03_closed_and_owingein-r-empty-alive-nodes-owe), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `route` | every run | the route that answered, and the licences it used. On route 2 describe and `ein oracle` say *no licence* and why, where the summary and a mode's row name `L7 the cube` ([Q-M1f.20](../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f20--route-2s-licence-no-licence-on-one-row-l7-the-cube-on-another)) | the run | — | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
| `work` | every run | saturations, facts beyond root, and the route's layers: what the run cost, each counter defined in [`answer_dump.md` § The run summary](answer_dump.md#the-run-summary). No lever but a speed lever moves it, and the wall clock is `-s`'s | the run | `work.saturations`, `work.facts`, `work.layers` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once) |
<!-- /rows -->

**A partial description** prints what it certified and bounds the rest. The state route stops between sizes, so it
is complete up to states of k candidates; route 1 stops between layers, complete up to commitments of L candidates.
Describe takes route 1 only on a flagged program whose root is dead, which reads root alone, so no corpus run prints
route 1's *stopped before layer L* (the census of 2026-11-09,
[S1f.5.16 § T1 record](../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md#t1-record-2026-11-09)).
Where ∅ and ⊤ are both in R it is decided as the whole cube (L1, L2), and only its states are not all counted
([X11](read_out_examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube)). A flagged program past
`--region-limit` visits the cube in layer order up to 2^limit nodes and certifies the minimal line's members met with
nothing in R below them
([X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit)).

---

## 3. A mode: `--minimal`, `--maximal`, `-n N`, `--single`

A mode asks one question about R and prints its own count, never k_s or k_c, which are R's and which a mode did not
describe. `--single` prints `verdict` for the count row and `decided` for `exhausted`: the four words are its answer
and nowhere else.

<!-- rows: mode -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `asked` | every run, first | the question: which part of R, and every state on it, N of them (`-n N`), or whether it is one (`--single`) | — | `asked.n` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `verdict` | `--single` | the word, `Solution`, `Ambiguity`, `Contradiction` or `Open`, and why; *not decided* when the run stopped first. On the minimal line where ∅ is in R, `Solution` after one saturation, on every program and at every limit (M1f S1f.5.6) | R | `verdict.word` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X6](read_out_examples.md#x6--testsstdlibclosure03_closed_and_owingein-r-empty-alive-nodes-owe), [X7](read_out_examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) |
| `states of R` | `-n N` over all of R | how many states are listed, and whether they are all of R, as many as asked, or a lower bound | R | — | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X11](read_out_examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `minimal line` | `--minimal`, alone or with `-n N` | as `states of R`, of the minimal line: where ∅ is in R, ∅ alone and all of the line, after one saturation, on every program and at every limit (M1f S1f.5.6); otherwise, past `--region-limit` on a flagged program, describe's `minimal line` row, from the same description | R | — | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `maximal line` | `--maximal`, alone or with `-n N` | as `minimal line`, of the maximal line | R | — | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit), [X10](read_out_examples.md#x10--zebra2ein-the-puzzle) |
| `decided` | `--single` | whether the word is certified: the part is empty, it holds no other state, or two states settle it; *no* and why when the budget or the limit stopped the run | the run | `region.stopped.spent`, `region.stopped.projected`, `region.stopped.n`, `region.stopped.limit` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `exhausted` | every other mode | whether the states listed are all of the part asked; *no* when stopped as asked, at the budget, past the limit, or at `--max-time` | the run | `region.exhausted`, `region.stopped.spent`, `region.stopped.projected`, `region.stopped.n`, `region.stopped.limit` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `lattice` | every mode | as describe's, with the holes only where the route knows them | around R | `monotonicity.sites` | [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `strata` | as describe's | as describe's | around R | — | [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `universe` | every mode | H's size in candidates, and § 5.3's block | around R | `region.universe.candidates` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `route` | every mode | the route and the licences it used: a mode on the state route meets each state at most once | the run | `region.licences` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X10](read_out_examples.md#x10--zebra2ein-the-puzzle) |
| `work` | every mode | saturations and facts beyond root | the run | `work.saturations`, `work.facts` | [X10](read_out_examples.md#x10--zebra2ein-the-puzzle) |
<!-- /rows -->

A mode's count row counts the states the run lists: the summary's `region.states` whose `listed` is true. Past the
limit a line mode meets members it does not list, so its row is describe's, read off the same description, and the
other members met are counted on the listing's last line (M1e2 S1e2.2.26, W27).

**Where ∅ is in R, every minimal question stops at root** (M1f S1f.5.6, the user's ruling of 2026-11-07 on S-16): ∅
is below every commitment, so the minimal line is {∅}, and whether ∅ is in R is root's fixpoint alone. `--minimal`,
`--minimal -n N` and `--minimal --single` list ∅ alone, decided, after one saturation, on a monotone program and a
flagged one, within `--region-limit` and past it; on a flagged program the route row still names the cube, whose
first node decides the line. The run reads one node, so the warning's `status` is not known and its `state` counts
one node.

---

## 4. `--monotone`

A question about the program, answered by root's fixpoint and the static check: no node is built and no state is
listed. The summary's `region` is `null`.

<!-- rows: monotone -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `asked` | first | the question, whether the program is monotone | — | — | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `monotone` | always | *yes*, with no site, or *no* and how many sites, with the warning's static rows and `bit` under it (§ 5.1) | around R | `monotonicity.sites` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `universe` | always | H's size, and § 5.3's block | around R | `root.hypgen.emitted` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `route` | always | none: a question about the program | the run | — | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `work` | always | one saturation, root's | the run | `work.saturations`, `work.facts` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
<!-- /rows -->

---

## 5. The blocks under a row

### 5.1 Under `lattice` and `monotone`: the warning

A monotone program prints `lattice`'s one row, and `paths` under it with `-y`. A flagged program prints what is
assumed, what each site puts at stake, and whether it bit, the status and the state apart, because the cube reads
only the status. At most four `site` and four `stale` rows are printed, each then a `…` row; `--dump-answer`'s
`answer.txt` has them all. The block does not take the listings' 100 lines. Its JSON is the summary's `monotonicity`
block, [`region_dump.md`](region_dump.md#monotonicity--the-static-check)'s.

<!-- rows: lattice -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `assumes` | on a flagged program | what a node is taken to be, and that this is open | — | — | [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
| `site` | per site, up to four | the rule, what it reads and where, what it concludes, and what it puts at stake | around R | `monotonicity.sites[k].rule` | [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole), [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `…` | past four sites | how many more sites, and what writes every one | the listing | — | none: no corpus program has five sites (2026-11-09) |
| `cycle` | per cycle through an `absent` over what a commitment can change | its rules' negative edges, and what its parity allows | around R | `monotonicity.cycles[k].negative` | [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `order` | per order finding | which rules read under an `absent` what another concludes, and what orders them | around R | `monotonicity.order[k].reader`, `monotonicity.order[k].writer` | [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `bit` | `--monotone`, on a flagged program | that the cube's half is not computed, at what cost, and what computes it | the run | — | [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `status` | on a flagged program, under `lattice` | whether a site bit on the cube: exercised, by which rules, with the cube's counts of holes, dead nodes with an alive superset and discharged nodes with an owing superset; conservative; or not known, and why | around R | `monotonicity.status.holes`, `monotonicity.status.dead_not_upward_closed`, `monotonicity.status.discharged_not_upward_closed` | [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `state` | as `status` | how many of the nodes the run computed hold a stale fact, a fact whose `absent` guard the node's own fixpoint falsifies | around R | `monotonicity.state.nodes`, `monotonicity.state.stale_nodes` | [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole), [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `stale` | per stale node, up to four | the node, its class, and its stale facts with what derived them | around R | `monotonicity.state.stale[k].facts` | [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata), [X15](read_out_examples.md#x15--features01_not_and_absentein-the-warning-past-four-rows) |
| `…` | past four stale nodes | how many more, and what writes them | the listing | — | [X15](read_out_examples.md#x15--features01_not_and_absentein-the-warning-past-four-rows) |
| `paths` | on a flagged program, and with `-y` under a monotone `lattice` | what the parent-path spot check read, and whether a parent path reaches another state; *not checked* and its cost where the read-out prints no state to check | the run | `monotonicity.paths.parents` | [X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
<!-- /rows -->

### 5.2 Under `strata`

<!-- rows: strata -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `negative cycle` | per cycle through an `absent` of the rules' dependency graph | its relations, the rules on its negative edges, and what its parity may mean; [`absent_semantics.md`](absent_semantics.md#the-stratification-diagnostic) has every line it prints | around R | — | [X14](read_out_examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
<!-- /rows -->

### 5.3 Under `universe`

Every question prints these, since every question is over the same H.

<!-- rows: universe -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `outside` | when H leaves out a relation the program can conclude or ask about | the relations, why no rung proposes their facts, and what every state of R holds of them | around R | `root.hypgen.outside[].relation` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once) |
| `types` | when the blind enumerator binds a type as an object | the objects, how many candidates of which relations name them, and why | around R | `root.hypgen.types_as_objects.objects`, `root.hypgen.types_as_objects.candidates` | [X13](read_out_examples.md#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines) |
<!-- /rows -->

---

## 6. The listing

**What is listed.** An exhausted description lists R as one `(region …)`: every state, then each `(merged …)` for a
state more than one commitment reaches, read back as the claim that these are all of R. Everything else is listed as
members, each a `(model …)` under a comment and never wrapped in `(region …)`: a line, an `-n` cut, `--single`'s one
or two states, and on a partial run the line members the run certified, then how many other members it met. An empty
R is listed as `(false)` with the rows below, or as `(region (owing …))` under the frontier's rows. `--count` and
`--monotone` list nothing.

**Each comment** names a state's number in the listing, its size in candidates, the lines it is on, and the
commitment that reaches it, or how many do and how many of them are minimal. States are ordered by closure size, then
by their candidates, whatever the route, the lever or `-j`.

**The 100 lines.** Stdout's listings share 100 lines (the user's ruling of 2026-10-16): R's states and its
`(merged …)`s, then a line's members, then each query's classes, then `as :expect`, then the forms of § 8. Each prints
whole items while the lines last, and at least one; a line a text's line break adds counts. A cut region is not
wrapped in `(region …)`: its first comment says how long the whole is, the states that fit keep their numbers, and its
last says what was left out and that `--dump-answer DIR` writes the rest to `DIR/answer.txt`, the read-out uncut
([X13](read_out_examples.md#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines)). `ein oracle`
writes no answer directory, so its lines name `ein solve FILE --dump-answer DIR`. The rows are never cut.

**Two objects called *model*.** A listing's `(model …)`, and the summary's `region.states[].facts`, is a state's facts
over the candidates' relations, root's included: what tells R's states apart, and the region language's claim form.
[`solution_semantics.md` § 3](solution_semantics.md#3-model)'s **model**, a state's positive facts minus the positive
initial KB, is what `-P` prints (§ 8). `-t`'s `read-out` row says *model* of the first. The words are as they print
([Q-M1f.18](../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f18--which-object-the-word-model-names-on-the-read-out),
defaulted to documenting both).

**Why R is empty.** Under an empty R, between the rows and the form:

<!-- rows: empty -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `unsat core` | R empty of the dead kind: one per fact of root's core | a fact root's contradiction rests on | around R | `region.core[k]` | [X7](read_out_examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead), [X16](read_out_examples.md#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) |
| `core search` | under the core, where its search was cut at its budget | the cap the search stopped at, and that a smaller core may exist | the run | — | [X16](read_out_examples.md#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) |
| `source` | per fact of the core with a `:source` | the fact and its `:source`, quoted | around R | — | [X16](read_out_examples.md#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) |
| `refuted by` | under the core | for each fact of the contradiction, the rule of the step the core's derivation takes, `(given)` for a fact the core holds | around R | `region.refuted_by` | [X7](read_out_examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead), [X16](read_out_examples.md#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) |
| `owing` | R empty of the owing kind: one per node of the frontier | the node and how many instances it owes, each instance on a line under it through its rule's `:why` | around R | `region.frontier[k].owes`, `region.frontier[k].set` | [X6](read_out_examples.md#x6--testsstdlibclosure03_closed_and_owingein-r-empty-alive-nodes-owe) |
<!-- /rows -->

---

## 7. Queries and claims

R is computed once and every `(query …)` is answered from it. A query's classes are over the part its `:over` names,
all of R by default, or over the states listed where the run asked another question, and one row says so.

<!-- rows: queries -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `query`, `query i of n` | per `(query …)`, after the listing; numbered where the program has several | the goal | — | `queries[k].goal` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once) |
| `over` | under each query | the set the classes are over, how many states it holds, and how many projection classes they fall into; *a lower bound* where they are | R | `queries[k].classes` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `class i of n` | per class, while stdout's lines last, at the block's indent | how many states give it, and which by their numbers in the listing when that is complete; then each binding row, sorted, with the `:goal-text` sentence filled for it; *undetermined here* where the goal has no row | R | `queries[q].classes[i].states` | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X4](read_out_examples.md#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes) |
| `as :expect` | under the classes | the claim that states exactly these classes, to paste into the query; or *not printed*, and why: a lower bound, the states listed, a cut, or past what is left of the 100 lines | the listing | — | [X1](read_out_examples.md#x1--branching05_mini_zebraein-everything-at-once), [X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `:expect` | under a query that states a claim | the claim checked against the classes: holds, FAILED and why, or NOT CHECKED and why | — | — | [X3](read_out_examples.md#x3--features11_expect_ambiguityein-a-query-over-the-maximal-line) |
<!-- /rows -->

<!-- rows: claims -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `(solve :expect)` | when the program states `(solve … :expect …)` | the claim checked against R: holds, FAILED or NOT CHECKED, and the reasons under it | — | — | [X12](read_out_examples.md#x12--testsregion03_merged_orein-a-program-that-states-its-region) |
| `claims` | `--count`, when the program states a claim | how many claims, which, and that `--count` checks none | — | — | [X12](read_out_examples.md#x12--testsregion03_merged_orein-a-program-that-states-its-region) |
<!-- /rows -->

A claim the run refutes writes one stderr line and exits 1; one it could not check writes a `note:` and exits 0
(§ 10).

---

## 8. The forms after the read-out

Asked for by flag, each after the whole read-out, drawn from the states stdout showed, sharing what the listings left
of the 100 lines; a form's head is never cut, and its last line, when it cut, names what it left out and the file
`--dump-answer DIR` writes it to whole, `DIR/forms/<form>.txt`, drawn from every listed state.

| flag | its head | what it prints, per listed state | the object |
|---|---|---|---|
| `-H` | H's size, over how many relations, and how many dead alone | each relation's candidates, and those dead alone | H, the universe |
| `--models key` | the set it keys, and the key's size | the smallest set of slots that tells the states apart, and the combinations that occur | the listed states, as a key |
| `--models grid` | the set it draws | a grid per state where a relation says what a row is, or why none | each listed state |
| `-p` | per state, its number, and the count of facts | every fact beyond the program as loaded: root's derivations, the commitment and what it derives | a state's facts beyond the program |
| `-P` | the same, `(not …)` omitted | the positive ones | [§ 3's model](solution_semantics.md#3-model) |
| `-f` | the relations, and the count of facts | every fact of the candidates' relations, given ones too | the candidates' relations' facts |
| `--diff-models FILE` | the listed states against the file's, and what they are compared over | per state, what differs | the listed states |

---

## 9. `-s`, `-t`, and `ein oracle`'s line

The clock rows print after the read-out, `-s`'s `wall` first and then, after the forms, `-t`'s table under a line
`timing`, in its own layout (the phase at two spaces, the time right-aligned, then what the phase did). A clock is no
golden's: `read_out_reference.rs` holds them by the run below, its times masked.

<!-- rows: clock -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `wall` | `-s` | the run's wall clock | the run | — | the masked run |
| `load` | `-t` | parsing, imports and the program | the run | — | the masked run |
| `route` | `-t` | root, H and every node visited, and how many saturations | the run | `work.saturations` | the masked run |
| `read-out` | `-t` | each listed state's `(model …)` and the queries; under `--count`, none | the run | — | the masked run |
| `print` | `-t` | the text | the run | — | the masked run |
| `total` | `-t` | the sum | the run | — | the masked run |
<!-- /rows -->

The run, each time masked to `…`, as the test compares it:

<!-- masked: ein solve examples/lattice/02_genuine_3set_death.ein -s -t -->
```text
  wall … ms

timing
  load … ms   parse, imports, the program
  route … ms   root, H and every node visited: 8 saturations
  read-out … ms   each listed state's model, the queries
  print … ms   the text
  total … ms
```

<!-- rows: oracle -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `the cube` | `ein oracle`, at the left margin after the read-out | the cube's dead nodes with an alive superset and discharged nodes with an owing superset, and whether the static verdict held: *contradicted* is an engine defect, exit 1 | around R | — | [X17](read_out_examples.md#x17--ein-oracle-the-cubes-line) |
<!-- /rows -->

---

## 10. Stderr

The shapes of the lines a question writes beside an answer; [`defined_behaviour.md` §
4](../defined_behaviour.md#4-errors-and-exit-codes) has the exit codes and every error's line.

| line | when | exit |
|---|---|---|
| `note: FILE: partial — …` | a run the budget or `--region-limit` stopped | 0 |
| `note: FILE: query :expect NOT CHECKED — …` (`query i of n` where there are several) | a query's claim the run could not check, and every claim under `--count` | 0 |
| `note: FILE: (solve :expect) NOT CHECKED — …` | the solve claim, the same | 0 |
| `FILE: query :expect FAILED — …`, `FILE: (solve :expect) FAILED — …` | a claim the run refutes | 1 |
| `** aborted: --max-time S s reached before the question was decided **` | `-T` | 2, with nothing on stdout |

---

## 11. `--json-summary`: the answer blocks

The summary is the same facts as stdout, from the same read-out, with every key on every run and `null` where it does
not apply (`ein-summary/2`). Every bound sits beside `exhausted`.

**`[]` and `null`** (the user's ruling of 2026-11-07 on
[Q-M1e2.27](../../../plans/m1e2_correctness/open_questions.md#q-m1e227--what-do--and-null-mean-in-eins-json), built by
M1f [S1f.5.8](../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md)):
`[]` is a set known to be empty, and `null` an object that does not exist or that the run does not state, on every
field of every JSON surface. A core is one object, the set of facts a dead node's contradiction rests on, so a dead
node's core and rules are lists, and a node that is not dead, whose core does not exist, has `null`: `region.core` and
`region.refuted_by` are root's lists where R is empty of the dead kind and `null` where root is not dead;
`region.holes.minimal[].core` is a dead hole's list and an owing hole's `null`. Under `--count` and a mode they are
`null` on a dead root too, since the question prints no core
([Q-M1f.29](../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f29--a-dead-roots-core-under---count)).
`[]` would be a core that holds no fact, *the empty set suffices*, which no corpus program reaches (S1f.5.9). Until
S1f.5.8 an owing hole's core was written `[]`; `region.json`, `nodes.json` and `--events`' `enter` lines, which follow
the same rule, wrote `[]` on every node that is not dead.

### 11.1 Which page owns which key

<!-- owners -->
| key | owner |
|---|---|
| `schema` | this page, § 11.2 |
| `source` | this page, § 11.2 |
| `asked` | this page |
| `region` | this page |
| `verdict` | this page |
| `queries` | this page |
| `claim` | this page |
| `work` | [`answer_dump.md` § The run summary](answer_dump.md#the-run-summary) |
| `monotonicity` | [`region_dump.md` § Monotonicity](region_dump.md#monotonicity--the-static-check) |
| `leftover` | [`configuration.md`](../configuration.md)'s `EIN_LEFTOVER` row |
| `root` | [`answer_dump.md` § The run summary](answer_dump.md#the-run-summary) |
| `config` | [`configuration.md`](../configuration.md)'s defaults block |
<!-- /owners -->

### 11.2 The fields no row prints as itself

Every other field of the seven keys is in a row's summary column above. Under a mode, R's own numbers are `null`; under
`--count`, `region.states` and `region.frontier`; under `--monotone`, `region` itself.

<!-- fields -->
| field | what it holds | where stdout shows it |
|---|---|---|
| `schema` | `ein-summary/2` | nowhere |
| `source` | the file, as given | the header |
| `asked.which` | `all`, `minimal` or `maximal`; `null` under `--monotone` | `asked`'s part |
| `asked.ask` | `describe`, `find`, `single`, `count` or `monotone` | `asked`'s question |
| `region.stopped.why` | `asked`, `budget`, `limit` or `deadline`; `stopped` is `null` where nothing stopped the run | `exhausted`'s or `decided`'s reason |
| `region.stopped.layer` | route 1: the commitment size it stopped before; the state route: its layer, a state's size plus one ([Q-M1f.21](../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f21--regionstoppedlayer-on-two-routes)) | `exhausted`'s *layer L*, or *the states of k candidates*, k one less |
| `region.complete_up_to` | route 1's last complete commitment size; `null` on the state route, whose *k* no field holds | `states (k_s)`'s and `minimal line`'s *complete up to commitments of L candidates* |
| `region.root` | `in-r`, `owing` or `dead` | `∅` |
| `region.empty` | `dead`, `owing`, or `null` where R is not exhausted and empty | `empty` |
| `region.minimal.commitments` | the minimal commitments that reach the minimal line | `minimal line`'s *from m commitments*, on an exhausted R without ∅ |
| `region.minimal.least` | the smallest of them, in candidates | *the least guess* |
| `region.minimal.trivial` | ∅ is in R, so the line is {∅}: true exactly where `region.root` is `in-r` (M1f S1f.5.6; until then, where no node the run met owed) | `minimal line`'s `— ∅` |
| `region.maximal.trivial` | the line is ⊤ because nothing dies | *⊤, the trivial line* |
| `region.route` | `1`, `2`, `states` or `tree` | `route`'s first words |
| `region.licences` | each licence the route used, by its label | `route`, after the route; on route 2 describe and `ein oracle` say *no licence* where this holds `L7 the cube` |
| `region.core_status` | the core's search: `within-budget`, `cut-` and its caps, or `null` | `core search`, where cut |
| `region.holes.minimal[].set` | the hole's commitment | `minimal hole` |
| `region.holes.minimal[].dead` | dead, or owing | `minimal hole`'s *dead:* or *owes* |
| `region.holes.minimal[].core_status` | as `region.core_status`, for the hole's core | `minimal hole`'s *core search cut* |
| `region.states[].facts` | a state's facts over the candidates' relations | the listing's `(model …)` |
| `region.states[].closure` | the candidates its fixpoint holds | a comment's size |
| `region.states[].from` | the first commitment met that reaches it | a comment's *from* |
| `region.states[].minimal_generators` | its minimal commitments | `(merged …)`, and a member's *minimal* |
| `region.states[].commitments` | how many commitments reach it | a comment's *commitments* |
| `region.states[].lines` | `minimal`, `maximal` | a comment's lines |
| `region.states[].listed` | whether stdout lists it | a mode's count row counts these |
| `region.frontier[].facts` | a frontier node's facts over the candidates' relations | `(region (owing (model …)))` |
| `region.frontier[].owed` | each instance it owes, through its rule's `:why` | the lines under `owing` |
| `verdict.which` | the part `--single` asked about | `verdict` |
| `queries[].over` | `all`, `minimal`, `maximal`, or `listed` where the run listed another set | `over` |
| `queries[].exact` | whether the classes are all of the set | `over`'s *a lower bound* when false |
| `queries[].classes[].facts` | the class's facts | its `(model …)` in `as :expect` |
| `queries[].classes[].rows[].?var` | each binding row, a map from each goal variable to its value | the rows under `class i of n` |
| `queries[].expect.outcome` | `held`, `failed` or `not-checked`; `expect` is `null` where the query states no claim | `:expect` |
| `queries[].expect.lines` | why | `:expect`'s value and the rows under it |
| `claim.outcome` | as `queries[].expect.outcome`, for `(solve … :expect …)`; `claim` is `null` where the program states none | `(solve :expect)` |
| `claim.lines` | why | the rows under it |
<!-- /fields -->

---

## 12. The surfaces that repeat these numbers

They define nothing; each repeats rows defined above.

| surface | what it repeats | held by |
|---|---|---|
| `--dump-answer DIR`'s `answer.txt` | the read-out, every listing uncut | `read_out_examples.rs` (X10's `DIR/answer.txt` block) |
| `--dump-answer DIR`'s `answer.json` and `index.json` | the summary byte for byte; the `region` block | `region_answer::the_answer_directory_is_the_answer` |
| `--trace FILE.md`'s headline | the rows above the listing, from the same printer | `golden_trace::every_real_solve_reproduces_its_golden` |
| `--events FILE`'s `region` event | `asked`, `region` without `states` and `frontier`, `verdict`, `work` | by construction: `modes::emit_region` writes the summary's own blocks |
| `ein test -v` and `--json-report` | a solve row's `region.k_s` and `k_c`; a query row's classes | `test_cli::the_verbose_header_and_the_report_row_agree_field_for_field` (the header against the report row); none holds them to a summary |

---

## Cross-references

- [`read_out_examples.md`](read_out_examples.md): every row in a real run.
- [`solution_semantics.md`](solution_semantics.md): R, its two counts, its lines, a model.
- [`answer_dump.md`](answer_dump.md): the answer directory, and the `work` and `root` blocks.
- [`region_dump.md`](region_dump.md): the cube's dump and the `monotonicity` block.
- [`events.md`](events.md): the event stream.
- [`../configuration.md`](../configuration.md): every option.
- M1e2's [`read_out.md`](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md) and its frozen
  examples, the record of the rulings of 2026-10-16.
