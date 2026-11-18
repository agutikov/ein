# The read-out — every row `ein solve` prints, and what it counts

**What this page is.** The reference for what `ein solve` writes on stdout and stderr for each of its questions, what
`ein oracle` writes, and the answer blocks of `--json-summary`: every row's label, when it is printed, what it says,
which kind of thing its numbers count, and the summary field that holds the same number. The values are on the
examples page, [`09-read-out-examples.md`](09-read-out-examples.md), every block of it one run of the binary. The test
suite holds this page to the binary:

1. every label the read-out can print is a row of the table its section names, and every row is a label, in printing
   order;
2. every label is printed, at its indent, by a block of the examples page, and the clock rows by one run with the
   times masked;
3. every field of `schema`, `source`, `asked`, `region`, `verdict`, `queries` and `claim` is named here, and every
   field named here is in a summary; the owners table names every top-level key;
4. on every example run, every row whose summary column names a field prints that field's number.

**What it is not.** What R, k_s, k_c, a line and a model are is
[`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md)'s. Exit codes, and which stream each byte
goes to, are [`07-diagnostics.md`](07-diagnostics.md)'s and [`05-cli.md`](05-cli.md)'s. The options and their defaults are
[`06-config.md`](06-config.md)'s. The `--dump-answer` directory and the summary's `work` and `root` blocks are
[`10-answer-dump.md`](10-answer-dump.md)'s; the `monotonicity` block and the warning's theory are
[`11-region-dump.md` § Monotonicity](11-region-dump.md#monotonicity--the-static-check)'s; the `--events` stream is
[`05-events.md`](../../ein.rs/05-events.md)'s. How each question is computed is
[`07-finding-r.md`](../../reasoning/02-kb-evolution/07-finding-r.md)'s. `--trace`'s file, `ein test`'s text and report, and the
event stream repeat rows this page defines, and § 12 says which.

---

## 1. How a row reads

The read-out opens with a header, the command and its arguments as typed, and a rule under it. Then come the rows,
each a label and a value: a row at the left margin at two spaces, a row of a block under it at four, every value at
column 20, or one space past a label as long as its column or longer, so that no label touches its value (only a
numbered label is that long, `class i of n` at a block's sixteen characters). A row's value that runs over a line
break of a text the program wrote (a `:goal-text`'s sentence, an obligation's `:why`) continues at the column that
text starts at ([Q-M1f.19](../../../decisions/read-out/Q-M1f.19-a-texts-line-break-on-the-read-out-continues-at.md)).
After the rows come the listing, the queries, the claims, and on request the forms (§ 6 to § 8).

**Every row says what its numbers count**, one of four kinds:

| kind | what it counts |
|---|---|
| **R** | states or commitments of R, or of the line or the part the question names; and whether $\emptyset$ is in R, or which kind of empty R is |
| **around R** | the cube and the universe: candidates, and nodes and sets of the cube. None of them is a state of R |
| **the run** | what the search did: saturations, facts, layers, nodes met, facts spent and projected, parents checked, the wall clock |
| **the listing** | what stdout shows and what it leaves out |

A row marked **—** counts nothing: it says what was asked, or a verdict on a claim.

**What the rows of kind R count, as formulas.** Write $H$ for the candidates, $C \subseteq H$ for a commitment and
$\operatorname{st}(C)$ for what root's facts and $C$'s saturate to. Then

$$
R = \lbrace C \subseteq H : \operatorname{st}(C) \text{ is alive and owes nothing} \rbrace
$$

$$
\begin{aligned}
k_c &= |R| \\
k_s &= |\lbrace \operatorname{st}(C) : C \in R \rbrace|
\end{aligned}
$$

`commitments (k_c)` counts the members of $R$ and `states (k_s)` the states they reach, so $k_s \le k_c$, with
equality exactly where the `merged` row reads *none*. The two lines are the states of the members of $R$ with no
member below them, and with none above:

$$
\begin{aligned}
\min R &= \lbrace C \in R : \nexists D \in R \colon D \subsetneq C \rbrace \\
\max R &= \lbrace C \in R : \nexists D \in R \colon C \subsetneq D \rbrace
\end{aligned}
$$

and `minimal line` and `maximal line` count $\lbrace \operatorname{st}(C) : C \in \min R \rbrace$ and
$\lbrace \operatorname{st}(C) : C \in \max R \rbrace$. `--minimal` and `--maximal` list those states, and
`--single` asks whether $k_s = 1$.

**One label, one number.** Each invocation prints one label for one number, and every number a row prints that the
summary also holds is the summary's field. The tables' **summary** column names that field: its number is the
integer, or an array's length, or each string of a list; a zero may print as a word (*none*, *in R*, *no hole*). In a
path, `[]` is every element, `[k]` the element paired with the k-th row of that label in the run (from 0), `[q]` the
query whose block the row is in, and `[i]` the row's own number less one. A field no row prints as itself is in
§ 11.2, with where it shows.

**No table quotes a printed value.** The values are the examples' (the **example** column), so what a table states is
what each row means, and the examples page shows what it prints.

---

## 2. Describe: a bare `ein solve`, `--count`, and `ein oracle`

A bare `ein solve FILE` describes all of R. On a monotone program it takes one of two routes, which
`--describe-route tree|states` chooses and which describe one R
([`07-finding-r.md` § 6](../../reasoning/02-kb-evolution/07-finding-r.md#6-the-routes-question-by-question)): **the tree and the
classes**, the default, where root owes an instance a licence takes
([Q-M1f.311](../../../decisions/answer/Q-M1f.311-describe-takes-the-tree-and-the-classes.md)), and **the state
route** elsewhere or when asked.
The tree and the classes compute neither N nor an empty R's owing frontier, and the `N` row and the listing say so
(below and § 6). `ein oracle FILE` prints the same rows over the cube, route 2 asked for by name, then `the cube`
(§ 9). `--count` prints `asked`, `states (k_s)`, `commitments (k_c)`, `exhausted`, `empty` or `∅` (without its `owed`
rows), `lattice` with its block, `strata`, `universe`, `route` and `work`, each describe's text, then `claims` (§ 7)
and no listing: it reads no state. It prints no `N` row either, and a `--count` that writes no artefact carrying N's
count does not take N: on the state route N is searched for after the route, as the minimal transversals of the maximal
alive states' complements, and on a search whose alive nodes are partial assignments that search is most of a
description's work. A surface that prints N's count needs the search as much as one that prints its sets, the count
being how many transversals there are; so `--json-summary`, `--events` or `--dump-answer` beside `--count` asks for N,
and `region.nogoods` is the description's; `--dump-lattice`'s files carry no N. `ein test` asks the same way: no line of
it prints N, and its `--json-summary` and `--events` ask for it. R, its counts, its states and every claim's outcome are
the same either way ([Q-M1f.224](../../../decisions/read-out/Q-M1f.224-a-run-takes-n-only-where-it-prints-n.md)).

<!-- rows: describe -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `asked` | every run, first | the question: all of R, described, or how many states and commitments it holds | — | `asked.n` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once) |
| `states (k_s)` | describe, `--count`, `ein oracle` | R's size in states; on a run that did not exhaust, a lower bound, and how far it is complete | R | `region.k_s` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X11](09-read-out-examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `commitments (k_c)` | as `states (k_s)` | R's size in commitments, qualified the same way, and exact where R is decided as the whole cube. **Counted exactly and reported up to $2^{63} - 1$**: at that value the row reads *9 223 372 036 854 775 807 or more   (2^63 − 1: the run counts no further)*, the run having classified every commitment, and `region.k_c` is that number, which is then a floor. The same *or more* stands wherever a count of commitments is printed: a component's size, a listed state's *n commitments*, the `(merged …)` comment, `index.json`'s `commitments`, `ein test -v`'s `k_c` ([Q-M1f.218](../../../decisions/answer/Q-M1f.218-a-count-is-exact-and-reported-up-to-2-63-1.md)) | R | `region.k_c` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X11](09-read-out-examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `exhausted` | as `states (k_s)` | whether the run classified every commitment, so that these are all of R; when not, why: the state route's, the tree and the classes' or route 1's budget, with the facts spent and what the next step projects, and for the tree and the classes the part it was finding (R's states, the candidates dead alone, or the classes and how many were found), or a flagged program past `--region-limit` | the run | `region.exhausted`, `region.stopped.spent`, `region.stopped.projected`, `region.stopped.commitments`, `region.stopped.n`, `region.stopped.limit` | [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit), [X11](09-read-out-examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `empty` | in place of `∅`, when R is exhausted and empty | which kind of empty: every node dead, root's own fixpoint deriving `(false)`, or every alive node owing | R | — | [X6](09-read-out-examples.md#x6--testsregion04_owingein-r-empty-alive-nodes-owe), [X7](09-read-out-examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) |
| `∅` | otherwise | where root stands: in R, not in R and how many instances it owes, or dead | R | `region.root_owes` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X5](09-read-out-examples.md#x5--latticesynthmerged_statesein-every-state-merged) |
| `owed` | under `∅`, where root is alive and owes: describe and `ein oracle`, at the block's indent, up to four | an obligation instance root's own fixpoint leaves undischarged, through its rule's `:why`: what the `∅` row counts, named (below the table) | around R | `region.root_owed[k]` | [X5](09-read-out-examples.md#x5--latticesynthmerged_statesein-every-state-merged), [X10](09-read-out-examples.md#x10--zebra2ein-the-puzzle) |
| `minimal line` | describe and `ein oracle`, unless R is exhausted and empty | the minimal line's size in states: where $\emptyset$ is in R, `1 state — ∅`, on every program and at every limit, the `∅` row above it saying why; on an exhausted R without $\emptyset$, how many minimal commitments reach it and the size of the smallest, *the least guess*; on a partial run, a lower bound and how far it is complete | R | `region.minimal.states` | [X5](09-read-out-examples.md#x5--latticesynthmerged_statesein-every-state-merged), [X10](09-read-out-examples.md#x10--zebra2ein-the-puzzle) |
| `maximal line` | as `minimal line` | the maximal line's size in states, $\top$ *trivial* when nothing dies; on a partial run what is certified, or *not known* where the line needs the whole cube | R | `region.maximal.states` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit), [X11](09-read-out-examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `components` | as `minimal line` | how many components R has under one-fact edges, and their sizes in commitments where there are several; *not known* on a partial run | R | `region.components` | [X4](09-read-out-examples.md#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes), [X5](09-read-out-examples.md#x5--latticesynthmerged_statesein-every-state-merged) |
| `sub-cubes` | as `minimal line` | how many maximal sub-cubes R has, and their dimensions; *not known* on a partial run or past the interval search's bound | R | `region.subcubes` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `merged` | as `minimal line` | how many states more than one commitment reaches, and how many minimal commitments they have between them; *none* where $k_c = k_s$; *not known* on a partial run | R | `region.merged.states`, `region.merged.minimal` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X5](09-read-out-examples.md#x5--latticesynthmerged_statesein-every-state-merged) |
| `N` | as `minimal line` | the minimal dead sets; on a partial run, a lower bound and which of them it holds. **Not computed** by the tree and the classes, which the row says, with the candidates dead alone as the members met and `--describe-route states` as what computes it, the summary's `region.nogoods` `null`; *not taken* where the run was asked by a surface that prints no N | around R | `region.nogoods` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `E` | describe on route 2, and `ein oracle` | the minimal discharging sets, which only the cube computes | around R | `region.discharging` | [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
| `lattice` | every run but `--monotone` | the static check's verdict; on a flagged program its sites, and the holes the cube shows or that they are not known; then the block of § 5.1 | around R | `monotonicity.sites`, `region.holes.count` | [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole), [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `strata` | when the rules' dependency graph has a cycle through an `absent` | how many such cycles; a diagnostic, not a refusal: root's fixpoint may depend on the boundary's order. The block of § 5.2 names each | around R | — | [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `minimal hole` | describe on route 2 and `ein oracle`, when the cube shows a hole; at the block's indent, up to four | a minimal hole, and why it is out: dead, with its core (`dead: ∅` where the core holds no fact), or what it owes | around R | `region.holes.minimal[k].owes`, `region.holes.minimal[k].core` | [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
| `universe` | every run | H's size in candidates and how many are dead alone, or that the run did not visit each alone; *no candidate* when H is empty, which decides R by root alone. The block of § 5.3 under it | around R | `region.universe.candidates`, `region.universe.dead_alone` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X6](09-read-out-examples.md#x6--testsregion04_owingein-r-empty-alive-nodes-owe), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `route` | every run | the route that answered, and the licences it used. A licence a run can do without is named only where an answer rested on it: `L5 an owed instance's alternatives are exhaustive` where a branch took an instance's alternatives; `L8 what a state denies is dead above it` where a set was taken for dead, with no saturation, because a state the run met stores the negation of one of its candidates; and `L9 a fact nothing proposes and nothing writes is in no state` where a branch took the alternatives of an instance L5 alone does not license: some of its witnesses are no candidate and not denied at root, and every such witness is of a relation no rule of the program writes and the kernel does not mirror, so no state holds it and the candidates among its witnesses are all that can pay ([`07-finding-r.md` § 4](../../reasoning/02-kb-evolution/07-finding-r.md#4-a-monotone-program-three-properties-and-the-licences) has each licence's argument). The tree and the classes, describe's default where root owes an instance a licence takes, read *the tree and the classes: R's states by the obligation tree, each state's class by dualise and advance*, then their licences, L4 and L5 among them. On route 2 describe and `ein oracle` say *no licence* and why, where the summary and a mode's row name `L7 the cube` ([Q-M1f.20](../../../decisions/read-out/Q-M1f.20-on-route-2-describe-says-no-licence-and-a-mode.md)) | the run | — | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
| `work` | every run | saturations, facts beyond root, and the route's layers: what the run cost, each counter defined in [`10-answer-dump.md` § The run summary](10-answer-dump.md#the-run-summary). A saturation is a node the run computed: a set a licence decided, dead above a dead set (L1), dead by a state's denial (L8) or a state already met (L3), is none, and costs none. The facts are each node's beyond root: an alive node's fixpoint, and on a monotone program a dead node's **to its first contradiction**, where its saturation stops ([`07-finding-r.md` § 1](../../reasoning/02-kb-evolution/07-finding-r.md#1-a-node)), but for the whole node of [§ 3.1](#31-what-order-the-obligation-tree-took), read to quiescence; on a flagged program every node's fixpoint. No option but a speed option moves it, and the wall clock is `-s`'s | the run | `work.saturations`, `work.facts`, `work.layers` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once) |
<!-- /rows -->

**What root owes.** Where the `∅` row reads *not in R — root owes n*, the `owed` rows under it name the n instances,
each through its obligation rule's `:why`, in the order the rules report them: the debt of a root beside the states
that pay it. The block is cut as the warning's is (§ 5.1): at most four `owed` rows, then one line at the values'
column that counts the rest and names what writes them, `--dump-answer DIR`, whose `answer.txt` has every one. It does
not take the listings' 100 lines, which are R's
([Q-M1f.220](../../../decisions/read-out/Q-M1f.220-what-root-owes-is-a-block-of-at-most-four-owed.md)). `--count`
prints the `∅` row alone, and a mode prints neither. `--trace`'s headline is these rows as stdout prints them, and the
summary's `region.root_owed` holds every instance on every description, `[]` where root owes none. Where R is
exhausted and empty the `empty` row stands in place of `∅`, and what is owed is under the frontier's `owing` rows
(§ 6): root's own instances are there where root is on the frontier, and in `region.root_owed` alone where it is not.

**A partial description** prints what it certified and bounds the rest. The state route stops between sizes, so it is
complete up to states of k candidates; route 1 stops between layers, complete up to commitments of L candidates.
Describe takes route 1 only on a flagged program whose root is dead, which reads root alone, so no `ein` run prints
route 1's *stopped before layer L*: layer 0 is root, which the route projects at one fact and the smallest
`--region-budget` the command takes, 1, admits, and a dead root leaves layer 1 empty
([Q-M1f.108](../../../decisions/read-out/Q-M1f.108-route-1s-stop-at-a-layer-is-the-librarys-and-no.md)).
**The tree and the classes stop in one of three parts**, in order, and the rows say which: in R's states, the tree
not exhausted, `states (k_s)` a lower bound and the maximal line those met with every extension dead; in the candidates
dead alone, every state of R found, `states (k_s)` exact, the maximal line exact, and `universe` not knowing who is dead
alone; in the classes, every state of R and its maximal line, the candidates dead alone, and the commitments of the
states whose class was found, `commitments (k_c)` a lower bound that counts one for each other state. The minimal line,
the components, the sub-cubes and the merged states need every class, and N is not computed by this route. A query
over all of R or its maximal line is exact past R's states (§ 7). Where $\emptyset$ and $\top$ are both in R it is decided as the
whole cube (L1, L2), and only its states are not all counted
([X11](09-read-out-examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube)). A flagged program
past `--region-limit` visits the cube in layer order up to $2^{\text{limit}}$ nodes and certifies the minimal line's members met
with nothing in R below them
([X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit)).

---

## 3. A mode: `--minimal`, `--maximal`, `-n N`, `--single`

A mode asks one question about R and prints its own count, never k_s or k_c, which are R's and which a mode did not
describe. `--single` prints `verdict` for the count row and `decided` for `exhausted`: the four words are its answer
and nowhere else.

<!-- rows: mode -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `asked` | every run, first | the question: which part of R, and every state on it, N of them (`-n N`), or whether it is one (`--single`) | — | `asked.n` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `verdict` | `--single` | the word, `Solution`, `Ambiguity`, `Contradiction` or `Open`, and why; *not decided* when the run stopped first. On the minimal line where $\emptyset$ is in R, `Solution` after one saturation, on every program and at every limit | R | `verdict.word` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X6](09-read-out-examples.md#x6--testsregion04_owingein-r-empty-alive-nodes-owe), [X7](09-read-out-examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) |
| `states of R` | `-n N` over all of R | how many states are listed, and whether they are all of R, as many as asked, or a lower bound | R | — | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X11](09-read-out-examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) |
| `minimal line` | `--minimal`, alone or with `-n N` | as `states of R`, of the minimal line: where $\emptyset$ is in R, $\emptyset$ alone and all of the line, after one saturation, on every program and at every limit; otherwise, past `--region-limit` on a flagged program, describe's `minimal line` row, from the same description | R | — | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `maximal line` | `--maximal`, alone or with `-n N` | as `minimal line`, of the maximal line | R | — | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit), [X10](09-read-out-examples.md#x10--zebra2ein-the-puzzle) |
| `decided` | `--single` | whether the word is certified: the part is empty, it holds no other state, or two states settle it; *no* and why when the budget or the limit stopped the run | the run | `region.stopped.spent`, `region.stopped.projected`, `region.stopped.n`, `region.stopped.limit` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `exhausted` | every other mode | whether the states listed are all of the part asked; *no* when stopped as asked, at the budget, or past the limit | the run | `region.exhausted`, `region.stopped.spent`, `region.stopped.projected`, `region.stopped.n`, `region.stopped.limit` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `lattice` | every mode | as describe's, with the holes only where the route knows them | around R | `monotonicity.sites` | [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `strata` | as describe's | as describe's | around R | — | [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `universe` | every mode | H's size in candidates, and § 5.3's block | around R | `region.universe.candidates` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `route` | every mode | the route and the licences it used: a mode on the state route meets each state at most once. **Where the route is the obligation tree, the order the run took**, in words, between the route's name and the licences ([§ 3.1](#31-what-order-the-obligation-tree-took)) | the run | `region.licences` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X10](09-read-out-examples.md#x10--zebra2ein-the-puzzle) |
| `work` | every mode | saturations and facts beyond root, a dead node's to its first contradiction on a monotone program, as describe's | the run | `work.saturations`, `work.facts` | [X10](09-read-out-examples.md#x10--zebra2ein-the-puzzle) |
<!-- /rows -->

A mode's count row counts the states the run lists: the summary's `region.states` whose `listed` is true. Past the
limit a line mode meets members it does not list, so its row is describe's, read off the same description, and the
other members met are counted on the listing's last line.

A run `--max-time` stops is no answer: it prints nothing on stdout, writes no summary and no answer directory's
files, and exits 2 (§ 10). So no row reads *stopped at `--max-time`*.

### 3.1 What order the obligation tree took

An owing state of the obligation tree branches on **one** owed instance, and which one decides how many nodes the run
saturates and nothing else: the verdict, R and what a stopped run claims are the same under every order, and only
which members of R a question that stops early lists first is the order's. The order is the route's own, a function of
the question, and no option, `(config …)` field or `EIN_*` name reaches it. A state chooses by three keys in turn, and
the `route` row says which of them this run came to use:

- ***branching on the owed instance with the fewest alternatives left***: no state's choice was left tied by the
  count, so nothing after it decided a branch. *Left* is not known dead: an alternative a saturation found dead under
  the state, or one the state denies, is not counted;
- ***…, the first by text among equals***: some choice was tied and the last key broke it, the instances taken in the
  order of their alternatives as rendered text. Under `-n N`, of all of R or of a line, and on the minimal line whole
  or under `--single`, the row says ***the first written***: there the last key is the order the obligation pass
  reports the instances in, which is the order the program states its facts in. The minimal line, whole or under
  `--single`, reads no coupling, so its row never says more than this;
- ***…; no coupling read, the whole node costing more than a twentieth of the run***: the same, the run having tried
  to read the couplings and stopped, as below;
- ***branching on the owed instance closest to closing a coupling, then the one with the fewest alternatives left,
  then the first by text: K couplings read off the whole node at saturation S***: the run read the couplings, and from
  then on a state branches first on the instance the fewest unpaid instances away from completing one.
  ***, the tree started again*** where a choice made before the couplings were read fell otherwise with them: every
  such state chooses again, by the counts it had when it first chose, and the tree is walked again from root over the
  nodes it has, each read from memory.

**A coupling** is a set of owed instances that some conflict or some derivation needs a member of each of: the
letters of one column of a sum, with the letters its carry comes from. It is read off **the whole node**, root with
every candidate committed, saturated once
([`05-hypothesis-space.md` § The coupling a route reads](../../reasoning/02-kb-evolution/05-hypothesis-space.md#the-coupling-a-route-reads-the-whole-node-alone)).
**That saturation is the run's**: it is one of the `work` row's saturations, with its facts, and one of the nodes a
`--dump-lattice` lists, the node of every candidate. So the couplings are read only **when a choice is left tied, the
run has saturated twenty nodes, and the node costs at most a twentieth of the facts the run has derived**: a question
a few nodes answer reads none, and its row has no word of them. A node that needs more is stopped at that twentieth,
what it derived is in the `work` row's facts, and it is tried again once the run has derived four times as much.

**Where $\emptyset$ is in R, every minimal question stops at root**: $\emptyset$ is below every commitment, so the minimal line is $\lbrace \emptyset \rbrace$,
and whether $\emptyset$ is in R is root's fixpoint alone. `--minimal`, `--minimal -n N` and `--minimal --single` list $\emptyset$ alone,
decided, after one saturation, on a monotone program and a flagged one, within `--region-limit` and past it; on a
flagged program the route row still names the cube, whose first node decides the line
([Q-M1f.23](../../../decisions/read-out/Q-M1f.23-where-root-decides-a-flagged-programs-minimal.md)). The run reads one
node, so the warning's `status` is not known and its `state` counts one node.

---

## 4. `--monotone` and `--no-search`: no question about R

Two runs build no node past root. `--monotone` asks about the program; `--no-search` declines to ask about R and reports
root. The summary's `region` is `null` on both.

### 4.1 `--monotone`

A question about the program, answered by root's fixpoint and the static check: no node is built and no state is
listed.

<!-- rows: monotone -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `asked` | first | the question, whether the program is monotone | — | — | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `monotone` | always | *yes*, with no site, or *no* and how many sites, with the warning's static rows and `bit` under it (§ 5.1) | around R | `monotonicity.sites` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `universe` | always | H's size, and § 5.3's block | around R | `root.hypgen.emitted` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `route` | always | none: a question about the program | the run | — | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
| `work` | always | one saturation, root's | the run | `work.saturations`, `work.facts` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode) |
<!-- /rows -->

### 4.2 `--no-search`

Root alone, and R not searched: the flag does not change R, it declines to discover it
([Q-M1f.255](../../../decisions/read-out/Q-M1f.255-ein-solve-no-search-prints-root-and-an-r-row.md)). The run
saturates root and reads root's node the way every route reads a node, and builds no other: it prints what root is,
says that R was not searched, and prints **no row that counts or lists R** (none of § 2's from `states (k_s)` to `E`,
and no listing). What root is is what the full run reaches there, the same `∅` row and `owed` block describe prints
where it prints them, and the summary's `root.dead`, `root.owes` and `root.owed` are the full run's `region.root`,
`region.root_owes` and `region.root_owed`. A dead root's core follows the rows, as describe prints it under `(false)`;
each query is read at $\emptyset$ where $\emptyset$ is in R, its classes *∅ alone*, and says so where root owes or is dead; each claim is
what root decides of it: on a dead root what the full run decides, every node being dead (L0); on an alive root a
`(false)` refuted where `ein test` asks it, a query whose class at $\emptyset$ its claim does not list refuted, and every other
NOT CHECKED, with whether $\emptyset$ is in R
([Q-M1f.256](../../../decisions/answer/Q-M1f.256-under-no-search-a-claim-is-decided-as-far-as.md)).
`--json-summary` writes `asked.ask` = `no-search`, `region` and `verdict` `null`; `--dump-answer DIR` writes `root.ein`,
root's state, beside `answer.txt` and `answer.json`; the forms (`-p`, `-P`, `-f`) draw root
whatever it is ([`10-answer-dump.md`](10-answer-dump.md)). A program whose `(solve …)` states `:no-search` is another thing:
its lattice is root alone, so a bare `ein solve` describes R, $\lbrace \emptyset \rbrace$ or empty, its `universe` row saying *the program
states :no-search*.

<!-- rows: no-search -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `asked` | first | the question: root alone, and R not searched | — | `asked.ask` | [X18](09-read-out-examples.md#x18--testsregion04_owingein---no-search-root-alone) |
| `∅` | always | where root stands: in R, not in R and how many instances it owes, or dead | R | `root.owes` | [X18](09-read-out-examples.md#x18--testsregion04_owingein---no-search-root-alone), [X19](09-read-out-examples.md#x19--features07_unstratifiableein---no-search-flagged-and-its-strata) |
| `owed` | under `∅`, where root is alive and owes, up to four | an instance root owes, through its rule's `:why`, as describe's | around R | `root.owed[k]` | [X18](09-read-out-examples.md#x18--testsregion04_owingein---no-search-root-alone) |
| `R` | always | *not searched*: no node past root was built, and a bare `ein solve FILE` describes R | — | — | [X18](09-read-out-examples.md#x18--testsregion04_owingein---no-search-root-alone) |
| `lattice` | always | the static check's verdict and, on a flagged program, its sites; the cube's `status` not known | around R | `monotonicity.sites` | [X18](09-read-out-examples.md#x18--testsregion04_owingein---no-search-root-alone) |
| `strata` | as describe's | as describe's | around R | — | [X19](09-read-out-examples.md#x19--features07_unstratifiableein---no-search-flagged-and-its-strata) |
| `universe` | always | H's size, built and *none visited*; *no candidate*, and *the program states :no-search* where it does | around R | `root.hypgen.emitted` | [X18](09-read-out-examples.md#x18--testsregion04_owingein---no-search-root-alone) |
| `route` | always | none: root's node, and no node past it | the run | — | [X18](09-read-out-examples.md#x18--testsregion04_owingein---no-search-root-alone) |
| `work` | always | one saturation, root's | the run | `work.saturations`, `work.facts` | [X18](09-read-out-examples.md#x18--testsregion04_owingein---no-search-root-alone) |
<!-- /rows -->

---

## 5. The blocks under a row

### 5.1 Under `lattice` and `monotone`: the warning

A monotone program prints `lattice`'s one row, and `paths` under it with `-y`. A flagged program prints what is
assumed, what each site puts at stake, and whether it bit, the status and the state apart, because the cube reads only
the status. At most four `site` and four `stale` rows are printed, each then a `…` row; `--dump-answer`'s `answer.txt`
has them all. The block does not take the listings' 100 lines. Its JSON is the summary's `monotonicity` block,
[`11-region-dump.md`](11-region-dump.md#monotonicity--the-static-check)'s.

<!-- rows: lattice -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `assumes` | on a flagged program | what a node is taken to be, and that the answer rests on that reading ([Q-M1e2.2](../../../decisions/answer/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md)) | — | — | [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
| `site` | per site, up to four | the rule, what it reads and where, what it concludes, and what it puts at stake | around R | `monotonicity.sites[k].rule` | [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole), [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `…` | past four sites | how many more sites, and what writes every one | the listing | — | none on the examples page: none of its programs has five sites |
| `cycle` | per cycle through an `absent` over what a commitment can change | its rules' negative edges, and what its parity allows | around R | `monotonicity.cycles[k].negative` | [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `order` | per order finding | which rules read under an `absent` what another concludes, and what orders them | around R | `monotonicity.order[k].reader`, `monotonicity.order[k].writer` | [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `bit` | `--monotone`, on a flagged program | that the cube's half is not computed, at what cost, and what computes it | the run | — | [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `status` | on a flagged program, under `lattice` | whether a site bit on the cube: exercised, by which rules, with the cube's counts of holes, dead nodes with an alive superset and discharged nodes with an owing superset; conservative; or not known, and why | around R | `monotonicity.status.holes`, `monotonicity.status.dead_not_upward_closed`, `monotonicity.status.discharged_not_upward_closed` | [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `state` | as `status` | how many of the nodes the run computed hold a stale fact, a fact whose `absent` guard the node's own fixpoint falsifies | around R | `monotonicity.state.nodes`, `monotonicity.state.stale_nodes` | [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole), [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
| `stale` | per stale node, up to four | the node, its class, and its stale facts with what derived them | around R | `monotonicity.state.stale[k].facts` | [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata), [X15](09-read-out-examples.md#x15--features01_not_and_absentein-the-warning-past-four-rows) |
| `…` | past four stale nodes | how many more, and what writes them | the listing | — | [X15](09-read-out-examples.md#x15--features01_not_and_absentein-the-warning-past-four-rows) |
| `paths` | on a flagged program, and with `-y` under a monotone `lattice` | what the parent-path spot check read, and whether a parent path reaches another state; *not checked* and its cost where the read-out prints no state to check | the run | `monotonicity.paths.parents` | [X2](09-read-out-examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode), [X8](09-read-out-examples.md#x8--lattice04_obligation_trigger_holeein-a-hole) |
<!-- /rows -->

### 5.2 Under `strata`

<!-- rows: strata -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `negative cycle` | per cycle through an `absent` of the rules' dependency graph | its relations, the rules on its negative edges, and what its parity may mean; [`04-rule-evolution.md`](../../ein.rs/08-reasoning-in-code/04-rule-evolution.md#where-the-diagnostic-is-shown) has every line it prints | around R | — | [X14](09-read-out-examples.md#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) |
<!-- /rows -->

### 5.3 Under `universe`

Every question prints these, since every question is over the same H.

<!-- rows: universe -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `commit` | when the program is a saved file with a `(commit …)`, a node of its program's lattice ([`10-answer-dump.md`](10-answer-dump.md#a-saved-file-is-a-node-of-its-programs-lattice)) | how many candidates of H the node commits, that R is the part of the program's R above it and the universe the rest of H, and the candidates as the file names them. `∅`, the run's root, is the node | around R | `root.commit` | [X21](09-read-out-examples.md#x21--testssaved04_a_saved_nodeein-a-saved-file-a-node-of-its-programs-lattice) |
| `later debt` | per debt root does not owe and a commitment can bring, where H holds a candidate for it that no debt of root's proposes | the activator, the rules whose `:assert` concludes it, how many such candidates would pay the debt, and the candidates, at most four named and the rest counted; `--dump-answer`'s `answer.txt` names them all. What H is on the obligations rung is [`06-solution-semantics.md` § 1.4](../../reasoning/02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe)'s | around R | `root.hypgen.later_debts[k].activator`, `root.hypgen.later_debts[k].added` | [X20](09-read-out-examples.md#x20--ein-bugswrong-answersoblig-derived-activatorein-a-debt-root-does-not-owe) |
| `outside` | when H leaves out a relation the program can conclude or ask about | the relations, why no rung proposes their facts, and what every state of R holds of them | around R | `root.hypgen.outside[].relation` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once) |
<!-- /rows -->

---

## 6. The listing

**What is listed.** An exhausted description lists R as one `(region …)`: every state, then each `(merged …)` for a
state more than one commitment reaches, read back as the claim that these are all of R. Everything else is listed as
members, each a `(model …)` under a comment and never wrapped in `(region …)`: a line, an `-n` cut, `--single`'s one or
two states, and on a partial run the line members the run certified, then how many other members it met. An empty R
is listed as `(false)` with the rows below, or as `(region (owing …))` under the frontier's rows. Where the route did
not compute the frontier (the tree and the classes, R empty, root owing an instance a licence takes), a comment says
so and that `--describe-route states` computes it, the `owing` rows are the nodes the run met, each owing with every
extension dead, and no `(region …)` follows; the summary's `region.frontier` is `null`, and a region claim is not
checked. `--count` and `--monotone` list nothing.

**Each comment** names a state's number in the listing, its size in candidates, the lines it is on, and the commitment
that reaches it, or how many do and how many of them are minimal. States are ordered by closure size, then by their
candidates, whatever the route, the option or `-j`.

**The 100 lines.** Stdout's listings share 100 lines
([Q-M1e2.34](../../../decisions/read-out/Q-M1e2.34-stdouts-listings-share-100-lines-and-a-dump.md)): R's states and
its `(merged …)`s, then a line's members, then each query's classes, then `as :expect`, then the forms of § 8. Each
prints whole items while the lines last, and at least one; a line a text's line break adds counts. A cut region is not
wrapped in `(region …)`: its first comment says how long the whole is, the states that fit keep their numbers, and its
last says what was left out and that `--dump-answer DIR` writes the rest to `DIR/answer.txt`, the read-out uncut
([X13](09-read-out-examples.md#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines)). `ein oracle`
writes no answer directory, so its lines name `ein solve FILE --dump-answer DIR`. The rows are never cut; a block under
a row prints at most four rows of a kind, and three kinds say where the rest is: the warning's `site` and `stale` rows
(§ 5.1) and `∅`'s `owed` rows (§ 2).

**Two things called *model*.** A listing's `(model …)`, and the summary's `region.states[].facts`, is a state's facts
over the candidates' relations, root's included: what tells R's states apart, and the region language's claim form.
[`13-solutions-and-models.md` § Model](../03-syntax-and-semantics/13-solutions-and-models.md#model)'s **model**, a state's positive
facts minus the positive initial KB, is what `-P` prints (§ 8). `-t`'s `read-out` row says *model* of the first. The
words are as they print
([Q-M1f.18](../../../decisions/read-out/Q-M1f.18-the-read-outs-model-is-the-claim-form-and-p.md)).

**Why R is empty.** Under an empty R, between the rows and the form:

<!-- rows: empty -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `unsat core` | R empty of the dead kind: one per fact of root's core, or one, `∅, the empty set: the refutation rests on no fact`, where the core holds no fact | a fact root's contradiction rests on, or that it rests on none | around R | `region.core[k]` | [X7](09-read-out-examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead), [X16](09-read-out-examples.md#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) |
| `core search` | under the core, where its search was cut at its budget | the cap the search stopped at, and that a smaller core may exist | the run | — | [X16](09-read-out-examples.md#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) |
| `source` | per fact of the core with a `:source` | the fact and its `:source`, quoted | around R | — | [X16](09-read-out-examples.md#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) |
| `refuted by` | under the core | for each fact of the contradiction, the rule of the step the core's derivation takes, `(given)` for a fact the core holds | around R | `region.refuted_by` | [X7](09-read-out-examples.md#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead), [X16](09-read-out-examples.md#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) |
| `owing` | R empty of the owing kind: one per node of the frontier | the node and how many instances it owes, each instance on a line under it through its rule's `:why` | around R | `region.frontier[k].owes`, `region.frontier[k].set` | [X6](09-read-out-examples.md#x6--testsregion04_owingein-r-empty-alive-nodes-owe) |
<!-- /rows -->

**A dead root's core where the contradiction rests on no fact, or on a computed one.** A rule whose `:match` is `()`
fires at root, so `(rule r () :match () :assert (false))` refutes root with the empty set for its core: one
`unsat core` row, `∅`, the text surfaces' spelling of the summary's `[]` (§ 11), then `refuted by  r`. A false
conclusion of `eq` or `neq` (computed and never stored) is `(false)` derived by its rule, so root's `empty` row reads
*every node is dead — root's own fixpoint derives (false)*, `unsat core` lists the premises the firing matched and
`refuted by` names the rule; a written false `(eq a w)` is a conclusion with no premise, `unsat core  (false)` and
`refuted by  (given)`.

---

## 7. Queries and claims

R is computed once and every `(query …)` is answered from it. A query's classes are over the part its `:over` names,
all of R by default, or over the states listed where the run asked another question, and one row says so.

<!-- rows: queries -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `query`, `query i of n` | per `(query …)`, after the listing; numbered where the program has several | the goal | — | `queries[k].goal` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once) |
| `over` | under each query | the set the classes are over, how many states it holds, and how many projection classes they fall into; *a lower bound* where they are; over an empty R, which kind of empty it is, *root is dead* or *root is alive and owes n instances*, and that there is no state to answer from | R | `queries[k].classes` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `class i of n` | per class, while stdout's lines last, at the block's indent | how many states give it, and which by their numbers in the listing when that is complete; then each binding row, sorted, with the `:goal-text` sentence filled for it, a variable an `eq` or `neq` guard binds among them; *the goal holds: it has no variable* where the goal has none, and *the goal holds: it binds no variable* where each of its variables is an `(absent …)`'s own; *undetermined here* where the goal has no row | R | `queries[q].classes[i].states` | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X4](09-read-out-examples.md#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes) |
| `as :expect` | under the classes | the claim that states exactly these classes, to paste into the query, each class the goal's instances under its rows, substituted at every depth, an `eq`, `neq` or `(absent …)` instance among them, and of an `(or …)` goal the instances of the disjunct each row holds by, never the `(or …)` ([Q-M1f.532](../../../decisions/read-out/Q-M1f.532-an-or-goals-class-lists-the-facts.md)); or *not printed*, and why: a lower bound, the states listed, a cut, or past what is left of the 100 lines | the listing | — | [X1](09-read-out-examples.md#x1--branching05_mini_zebraein-everything-at-once), [X9](09-read-out-examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) |
| `:expect` | under a query that states a claim | the claim checked against the classes: holds, FAILED and why, or NOT CHECKED and why | — | — | [X3](09-read-out-examples.md#x3--features11_expect_ambiguityein-a-query-over-the-maximal-line) |
<!-- /rows -->

<!-- rows: claims -->
| label | printed when | says | kind | summary | example |
|---|---|---|---|---|---|
| `(solve :expect)` | when the program states `(solve … :expect …)` | the claim checked against R: holds, FAILED or NOT CHECKED, and the reasons under it | — | — | [X12](09-read-out-examples.md#x12--testsregion03_merged_orein-a-program-that-states-its-region) |
| `claims` | `--count`, when the program states a claim | how many claims, which, and that `--count` checks none | — | — | [X12](09-read-out-examples.md#x12--testsregion03_merged_orein-a-program-that-states-its-region) |
<!-- /rows -->

A claim the run refutes writes one stderr line and exits 1; one it could not check writes a `note:` and exits 0
(§ 10). How a claim's models differ from the states or classes found, model by model and the near-miss *one fact
off* among them, is `ein test --diff`'s ([`05-cli.md` § 2.3](05-cli.md#23-ein-test)); `ein solve` prints no diff.

**A claim under a mode.** A claim is about a whole part of R, so `ein solve` checks it where the answer lists that part
whole: under describe when R is exhausted, under `--single` that says `Solution`, `Contradiction` or `Open`, and under
`-n N` that found fewer than N states. Any other answer leaves it *not checked*, a note. That is what lets `ein test`
ask a claim less than the description: it reads a file's claims by their shapes, asks `--single` or `-n k+1` where that
can decide them, and describes what is left ([`12-query.md`](../03-syntax-and-semantics/12-query.md) has the table). `ein test`
alone also refutes a claim from a part, by a class of a state found that the claim does not list.

**A query over an empty R.** The `over` row says which kind of empty R is, as root says it: *R is empty, root is dead*,
every node being dead with it, or *R is empty, root is alive and owes n instances*, an alive root of an empty R always
owing. Describe, `--single` and `-n` print it alike. The claim under it is the same on both, `as :expect (false)`: a
query's `(false)` says the query has no class, and holds on either empty, where the region's `(false)` holds on the dead
kind alone
([Q-M1f.219](../../../decisions/answer/Q-M1f.219-a-querys-false-holds-over-either-kind-of-empty-r.md)).

---

## 8. The forms after the read-out

Asked for by flag, each after the whole read-out, drawn from the states stdout showed, sharing what the listings left
of the 100 lines; a form's head is never cut, and its last line, when it cut, names what it left out and the file
`--dump-answer DIR` writes it to whole, `DIR/forms/<form>.txt`, drawn from every listed state.

| flag | its head | what it prints, per listed state | what it is about |
|---|---|---|---|
| `-H` | H's size, over how many relations, and how many dead alone | each relation's candidates, and those dead alone | H, the universe |
| `-p` | per state, its number, and the count of facts | every fact beyond the program as loaded: root's derivations, the commitment and what it derives | a state's facts beyond the program |
| `-P` | the same, `(not …)` omitted | the positive ones | [the model](../03-syntax-and-semantics/13-solutions-and-models.md#model) |
| `-f` | the relations, and the count of facts | every fact of the candidates' relations, given ones too | the candidates' relations' facts |

`-H` lists H after a read-out, by relation, within the 100 lines. **H alone, whole and as Ein facts, is
`ein show hyps FILE`**: one candidate a line and nothing else on stdout, at the cost of root's saturation and the
generator's pass, no route run
([hypotheses and the answer, in code § 1.3](../../ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md#13-h-listed)). The
candidates are the same in both.

---

## 9. `-s`, `-t`, and `ein oracle`'s line

The clock rows print after the read-out, `-s`'s `wall` first and then, after the forms, `-t`'s table under a line
`timing`, in its own layout (the phase at two spaces, the time right-aligned, then what the phase did). A clock differs
between two runs, so the rows are shown below by one run with each time masked.

**Every count prints in digit groups of three**, `2 048`, as every row above does: `-t`'s `route` row and `the cube`'s
two counts too. A `2^n` exponent and an echoed `--region-budget` print as written.

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

The run, each time masked to `…`:

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
| `the cube` | `ein oracle`, at the left margin after the read-out | the cube's dead nodes with an alive superset and discharged nodes with an owing superset, and whether the static verdict held: *contradicted* is an engine defect, exit 1 | around R | — | [X17](09-read-out-examples.md#x17--ein-oracle-the-cubes-line) |
<!-- /rows -->

---

## 10. Stderr

The shapes of the lines a question writes beside an answer; [`07-diagnostics.md`](07-diagnostics.md) has the exit codes and
every error's line.

| line | when | exit |
|---|---|---|
| `note: FILE: partial — …` | a run the budget or `--region-limit` stopped. Under describe and `--count` stopped by the budget the note says, by default and not only under `-v`, the forecast (the next step's projection, on top of what was spent, against `--region-budget`: the state route's next layer, or the tree and the classes' next step), what the run established and what it did not, and the options: `--region-budget` larger (the state route: at least the spent and projected facts, which takes the next layer), and the other route where the program has one | 0 |
| `note: FILE: query :expect NOT CHECKED — …` (`query i of n` where there are several) | a query's claim the run could not check, every claim under `--count`, every claim root does not decide under `--no-search`, and every claim of a saved file with a `(commit …)`, whose claims are its program's ([Q-M1f.403](../../../decisions/answer/Q-M1f.403-a-saved-file-with-a-commitment-checks-none.md)) | 0 |
| `note: FILE: (solve :expect) NOT CHECKED — …` | the solve claim, the same | 0 |
| `note: FILE:L:C: solve: its :no-hypothesis …`, `… :hypothesis-relations …`, `… :no-search …` | at load, on every command: a list beside an hrule, which filters no candidate, or a name another declaration already says; an hrule or a list beside `(solve :no-search)`, which proposes or filters nothing ([`07-diagnostics.md`](07-diagnostics.md), the hypothesis mechanisms) | 0 |
| `FILE: query :expect FAILED — …`, `FILE: (solve :expect) FAILED — …` | a claim the run refutes | 1 |
| `** aborted: --max-time S s reached before the question was decided **` | `-T`: no answer, so nothing on stdout, no summary, and no file in the answer directory | 2 |

---

## 11. `--json-summary`: the answer blocks

The summary is the same facts as stdout, from the same read-out, with every key on every run and `null` where it does
not apply (`ein-summary/2`). Every bound sits beside `exhausted`.

**`[]` and `null`**
([Q-M1e2.27](../../../decisions/read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md)): `[]` is a
set known to be empty, and `null` a thing that does not exist or that the run does not state, on every field of every
JSON surface. A core is one thing, the set of facts a dead node's contradiction rests on, so a dead node's core and
rules are lists, and a node that is not dead, whose core does not exist, has `null`: `region.core` and
`region.refuted_by` are root's lists where R is empty of the dead kind and `null` where root is not dead;
`region.holes.minimal[].core` is a dead hole's list and an owing hole's `null`. Under `--count` and a mode they are
`null` on a dead root too, since the question prints no core
([Q-M1f.29](../../../decisions/read-out/Q-M1f.29-a-question-that-prints-no-core-writes-a-dead.md)). `[]` is a core
that holds no fact, *the empty set suffices*: a rule whose `:match` is `()` and whose `:assert` is `(false)` refutes
root on it, and the summary writes `core: []` and `refuted_by: ["r"]`, where every text surface prints `∅`
([Q-M1f.48](../../../decisions/read-out/Q-M1f.48-a-text-surface-writes-the-empty-core.md)). `region.json`,
`nodes.json` and `--events`' `enter` lines follow the same rule.

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
| `work` | [`10-answer-dump.md` § The run summary](10-answer-dump.md#the-run-summary) |
| `monotonicity` | [`11-region-dump.md` § Monotonicity](11-region-dump.md#monotonicity--the-static-check) |
| `leftover` | [`06-config.md`](06-config.md)'s `EIN_LEFTOVER` row |
| `root` | [`10-answer-dump.md` § The run summary](10-answer-dump.md#the-run-summary) |
| `config` | [`06-config.md`](06-config.md)'s defaults block |
<!-- /owners -->

### 11.2 The fields no row prints as itself

Every other field of the seven keys is in a row's summary column above. Under a mode, R's own numbers are `null`, and
`region.root_owed` with `region.root_owes`; under `--count`, `region.states` and `region.frontier`; under `--monotone`,
`region` itself.

<!-- fields -->
| field | what it holds | where stdout shows it |
|---|---|---|
| `schema` | `ein-summary/2` | nowhere |
| `source` | the file, as given | the header |
| `asked.which` | `all`, `minimal` or `maximal`; `null` under `--monotone` | `asked`'s part |
| `asked.ask` | `describe`, `find`, `single`, `count` or `monotone` | `asked`'s question |
| `region.stopped.why` | `asked`, `budget` or `limit`; `stopped` is `null` where nothing stopped the run | `exhausted`'s or `decided`'s reason |
| `region.stopped.layer` | route 1: the commitment size it stopped before; the state route: its layer, a state's size plus one ([Q-M1f.21](../../../decisions/read-out/Q-M1f.21-region-stopped-layer-is-each-routes-own-unit.md)) | `exhausted`'s *layer L*, or *the states of k candidates*, k one less |
| `region.complete_up_to` | route 1's last complete commitment size; `null` on the state route, whose *k* no field holds | `states (k_s)`'s and `minimal line`'s *complete up to commitments of L candidates* |
| `region.root` | `in-r`, `owing` or `dead` | `∅` |
| `region.empty` | `dead`, `owing`, or `null` where R is not exhausted and empty | `empty` |
| `region.minimal.commitments` | the minimal commitments that reach the minimal line | `minimal line`'s *from m commitments*, on an exhausted R without $\emptyset$ |
| `region.minimal.least` | the smallest of them, in candidates | *the least guess* |
| `region.minimal.trivial` | $\emptyset$ is in R, so the line is $\lbrace \emptyset \rbrace$: true exactly where `region.root` is `in-r` | `minimal line`'s `— ∅` |
| `region.maximal.trivial` | the line is $\top$ because nothing dies | *⊤, the trivial line* |
| `region.route` | `1`, `2`, `states`, `tree` or `tree-classes` | `route`'s first words |
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

| surface | what it repeats |
|---|---|
| `--dump-answer DIR`'s `answer.txt` | the read-out, every listing uncut ([X10](09-read-out-examples.md#x10--zebra2ein-the-puzzle)'s `DIR/answer.txt` block) |
| `--dump-answer DIR`'s `answer.json` and `index.json` | the summary byte for byte; the `region` block |
| `--trace FILE.md`'s headline | the rows above the listing, from the same printer |
| `--events FILE`'s `region` event | `asked`, `region` without `states` and `frontier`, `verdict`, `work`: the summary's own blocks |
| `ein test -v` and `--json-report` | a solve row's `region.k_s` and `k_c`; a query row's classes, over the states of the question the claim was read off, which both name (`route`, and `fell_back_from` where the first question left the claim undecided) |

---

## Cross-references

- [`09-read-out-examples.md`](09-read-out-examples.md): every row in a real run.
- [`13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md): R, its two counts, its lines, a model.
- [`10-answer-dump.md`](10-answer-dump.md): the answer directory, and the `work` and `root` blocks.
- [`11-region-dump.md`](11-region-dump.md): the cube's dump and the `monotonicity` block.
- [`05-events.md`](../../ein.rs/05-events.md): the event stream.
- [`06-config.md`](06-config.md): every option.
- [`05-cli.md`](05-cli.md): every command.
