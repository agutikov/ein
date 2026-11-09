# S1f.5.16 — The read-out reference: every row and number `ein solve` prints, on one live page the printer is held to

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)
**Was S1f.5.3**, proposed in the old P1f.5 (*Documentation, and other*), until 2026-11-08, and **S1e.5.3**, proposed
in M1e's [P1e.5](../../../docs/history/m1e_review_processing/README.md#p1e5--documentation-and-other), until
2026-08-29 ([§ The ids](README.md#the-ids)).
**Status:** specified 2026-11-08 on the user's instruction: *"Ok, schedule S1f.5.16 also"*. **Done 2026-11-09**, in
five commits: T1f.5.16.1, the census ([its record](#t1-record-2026-11-09)), Q1 defaulted (a) and three findings;
T1f.5.16.5, S1f.5.2's line-break item fixed before T2 ([its record](#t5-record-2026-11-09)), ten read-out digests
moved, no answer; T1f.5.16.2, the page and the live examples ([its record](#t2-record-2026-11-09)); T1f.5.16.3, the
label list and the test ([its record](#t3-record-2026-11-09)); T1f.5.16.4, the pointers and the record ([its
record](#t4-record-2026-11-09)), with a fourth finding, Q-M1f.22, documented. The acceptance is re-taken [below](#acceptance-re-taken-2026-11-09).
**Depends on:** [S1f.5.5](s1f.5.5_the_grammar.md)'s swap and the non-grammar halves after it
([§ The order](README.md#the-order), item 8), so that the census is taken, and the examples are written, in the
final grammar; and [S1f.5.15](s1f.5.15_rule_priority.md), done, whose `strata` row the page documents.
**Runs before** every stage that changes what the read-out prints: [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md),
[S1f.5.6](s1f.5.6_the_empty_state_decides_the_minimal_line.md), [S1f.5.8](s1f.5.8_no_contradiction_no_core.md)
and [S1f.5.12](s1f.5.12_cores_for_n_only.md), and [S1f.5.14](s1f.5.14_nl_required.md) last. Each of them then
changes the live page in the commit that changes the output, and the page's test fails until it does.

## The proposal

The old phase's row, until 2026-11-08: *one page for every number `ein solve` prints, and which of them counts
models and which what the search did.* Three of M1e's findings are about that confusion
([the ledger](../../../docs/history/m1e_review_processing/README.md#the-findings)):

- **SE-M1**: `k` and `stats.solution_nodes` under one label. `--stats` printed `solutions (k)` above
  `solution_nodes`, so one `ein solve … --stats` printed `solutions (k) 0` and `solutions (k) 1` a screen apart.
- **AR-M2**: the count was read out by three crates, each choosing its own number. M1e
  [S1e.3.4](../../../docs/history/m1e_review_processing/README.md#s1e34--architecture-medium) gave it one
  builder, and since M1e2 R's counts have one too, `ein_infer::described`.
- **AR-M1**: hand-maintained parallel copies as the tree's recurring drift mechanism, whose rule is now
  [`architecture.md` § One artifact, one owner](../../../docs/kernel/architecture.md#one-artifact-one-owner).

A fourth item came to this stage by name. M1e's
[Q-M1e.7](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model),
ruled 2026-08-28, gave its read-out half (*a projection has to exist, `model`, and `ein solve` and
`--json-summary` print it under that name*) to *P1e.5's S1e.5.3*, which is this stage.

The walk and its `k` are gone since M1e2 S1e2.1c.5 (2026-11-03). The question is unchanged, asked of R: which
number counts R's states or commitments, which counts what lies around R (the cube, the universe), and which
counts what the run did.

## The case

### The vocabulary is a closed milestone's plan

[`read_out.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/read_out.md) was T1e2.1a.5.1's page diff,
proposed and ruled on 2026-10-16. Every M1e2 stage that changed the read-out amended it in place, and `AGENTS.md`
names it *the read-out's vocabulary*. Three things follow from where it lives.

1. **The live specification sits in a plan directory.** M1e2 closed on 2026-11-08 with its plans kept in place,
   and they are not frozen (the user, 2026-11-08: *"It is not frozed, I'm asking about the reason"*, `dc4c2946`):
   S1f.5.15 added and reworded the `strata` row there (`ccc4e8ca`, `49666600`). Four stage files of this phase
   name it as a page to edit: S1f.5.6 (T1f.5.6.4), S1f.5.7 (T1f.5.7.5), S1f.5.8 (T1f.5.8.3) and S1f.5.12
   (T1f.5.12.3) (`grep -n 'read_out\.md' s1f.5.*.md`, 2026-11-08). Editing it is allowed; the case for a page
   under `docs/kernel/` is that a reader looking for what `ein solve` prints looks there, and that nothing holds
   the plan's prose to the printer (2).
2. **Nothing holds its prose to the printer.** The prose is a hand copy, AR-M1's shape. Only the examples are
   held: [`read_out_examples.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/read_out_examples.md), which
   [`utils/read_out_examples.py`](../../../utils/read_out_examples.py) writes from the binary and
   [`read_out_examples.rs`](../../../ein.rs/crates/ein-cli/tests/read_out_examples.rs) compares block by block.
   That golden page lives in the same closed directory, so a stage that changes the read-out re-blesses a file
   there. S1f.5.6's T1f.5.6.4 does.
3. **The code and the kernel cite it as their specification.** There are eleven doc-comment links in nine files
   under `ein.rs/crates/`: `ein-render`'s `read_out.rs`, `dump/region_answer.rs` and `trace/region.rs`;
   `ein-infer`'s `described.rs`, `downset.rs` and `read_out.rs`; `ein-cli`'s `modes.rs`, `summary.rs` and
   `tests/region_answer.rs`. There are ten links on six kernel pages: `answer_dump.md`, `configuration.md`,
   `defined_behaviour.md`, `events.md`, `implementation.md` and `solution_semantics.md`
   (`grep -rnoE '\]\([^)]*p1e2\.1a_the_solution_region/read_out\.md[^)]*\)' ein.rs/crates docs/kernel`,
   2026-11-08).

### A first look at the drift

Taken on 2026-11-08 at `49666600`, with the runs on that day's release binary. T1 takes the whole list; these
rows are what a first read found.

| `read_out.md` says | the code prints | where |
|---|---|---|
| § 4, the `route` row: `1, the alive down-set — licence L1, dead is upward-closed` | `1, the alive down-set — L1 dead is upward-closed` | `Licence::label` in [`explore.rs`](../../../ein.rs/crates/ein-infer/src/explore.rs) |
| § 8.6, `-s`: *the wall clock per phase, and route 1's per-layer table (visited, dead, owing, in R, facts)* | one `wall` row. The per-phase clock is `-t`'s `timing` table (`load`, `route`, `read-out`, `print`, `total`), which the page does not list | `ein solve examples/lattice/02_genuine_3set_death.ein -s`, and `-t`; [`modes.rs`](../../../ein.rs/crates/ein-cli/src/modes.rs) |
| § 8.1, `route: "1"\|"2"` | `"1"`, `"2"`, `"states"` or `"tree"`; `branching/05_mini_zebra.ein`'s summary says `"states"` | [`region_answer.rs`](../../../ein.rs/crates/ein-render/src/dump/region_answer.rs) |
| § 8.1, a query's `over: all\|listed` | `all`, `minimal`, `maximal` or `listed` | the same |
| § 8.1's block names neither `schema` nor `source` | both are top-level keys of every summary | the same |

One kernel page also says something the read-out has disproved.
[`solution_semantics.md` § 3](../../../docs/kernel/inference/solution_semantics.md#3-model) says *"No surface
computes projection classes yet"*, and every query block has printed them since M1e2 S1e2.1a.5.

None of these is a wrong answer. Each is prose that fell behind the printer, which is what happens to a page
that nothing runs.

### What is printed today

At `49666600`, from a source scan of [`read_out.rs`](../../../ein.rs/crates/ein-render/src/read_out.rs) for the
first argument of `row`, `sub_row` and `row_text` (2026-11-08):

- **45 literal row labels**, and four computed ones: a mode's count row (`states of R`, `minimal line` or
  `maximal line`), `query` or `query i of n`, `class i of n`, and `decided` in place of `exhausted` under
  `--single`.
- **Rows printed outside it.** `-s`'s `wall` and `-t`'s `timing` table are printed in
  [`modes.rs`](../../../ein.rs/crates/ein-cli/src/modes.rs), and `ein oracle`'s `the cube` line in
  [`oracle.rs`](../../../ein.rs/crates/ein-cli/src/oracle.rs).
- **Lines that are not rows.** The listing's comments (`;; state i of k — …`, `;; not shown: …`, and the
  others), each form's head after the read-out (`H — the universe: …`, `models — …`, and the heads of `-p`, `-P`,
  `-f` and `--diff-models`), and the stderr lines (`note: FILE: partial — …`, and a claim's `FAILED` and
  `NOT CHECKED`).
- **10 of the 45 labels are printed by no block of `read_out_examples.md`**: `bit`, `core search`, `cycle`,
  `monotone`, `negative cycle`, `order`, `source`, `stale`, `strata` and `…`. (The page's 32 `text` blocks were
  scanned against the list above, 2026-11-08.)

### The word *model* names two objects

Q-M1e.7's ruling: *"the recorded object is the solution state. `model` is a projection of it, computed at the
read-out: the positive part minus the positive initial KB"*, and `ein solve` and `--json-summary` print it
*"under that name"*. [`solution_semantics.md` § 3](../../../docs/kernel/inference/solution_semantics.md#3-model)
states the definition. Here is what prints today (`ein solve examples/branching/05_mini_zebra.ein`, with and
without `-P`, 2026-11-08):

- **A listing's `(model …)`**: six facts for ∅, two of which the program states (`(co-located Alice Tea)` and
  `(co-located Alice Cat)`). It is the region language's claim form, a state's facts over the candidates'
  relations (`read_out.md` § 4), and § 3 calls it neither a state nor a model. `--json-summary`'s
  `region.states[].facts` is the same set.
- **`-P`**: four facts, which is § 3's model. Its head says *final-state facts (beyond the program as loaded,
  (not …) omitted …)*.
- **`-t`'s `read-out` row** says *each listed state's model*.

So the object § 3 names *model* is printed under another name, and the word `model` is printed over another
object. That is SE-M1's shape. This stage documents both objects by name, and [Q1](#q1--q-m1e7s-read-out-half)
asks whether anything printed moves.

## What it changes

### The page: `docs/kernel/inference/read_out.md`

The page is a current one, with no banner. It sits beside the pages of the other surfaces (`answer_dump.md`,
`region_dump.md`, `events.md`, `lattice_dump.md`) and is named after its printer, `ein_render::read_out`. This file
does not link to it, because it does not exist until T2 and the link check is a gate step.

**Its subject** is what `ein solve` writes on stdout and stderr in each mode, and what `ein oracle` writes, which
is the same printer plus the `the cube` line. It also covers the answer blocks of `--json-summary`. The modes are:

- describe (a bare `ein solve`), and `--count`;
- `--minimal` and `--maximal`;
- `-n N`, alone and with a line;
- `--single`, alone and with a line;
- `--monotone`;
- the runs that stop: `--region-budget` (under route 1, under the state route, and on a whole cube), past
  `--region-limit` (describe and the line modes), and `-T`, which exits 2.

**Every row says what its numbers count.** There are four kinds:

| kind | what it counts | rows, today |
|---|---|---|
| **R**, the answer | states or commitments of R, or of the line or the part the question names | `states (k_s)`, `commitments (k_c)`, `minimal line`, `maximal line`, `components`, `sub-cubes`, `merged`, a mode's count row, `verdict` |
| **around R**: the cube and the universe | candidates, and nodes and sets of the cube. None of them is a state of R | `N`, `E`, `minimal hole`, `universe` with `outside` and `types`, the `lattice` block's sites, `strata`'s cycles, `the cube` |
| **the run**: what the search did | saturations, facts, layers, nodes met, facts spent and projected, the wall clock | `work`, the reasons on `exhausted` and `decided`, the listing's *other members met*, `-s`, `-t` |
| **the listing**: what stdout shows | items printed and items left out | `;; state i of k`, `class i of n`, `… n more`, `not shown` |

The page states the rule that SE-M1 and AR-M2 ended in. Each invocation prints one label for one number. Every
number that a row prints and that the summary also holds is the summary's field.

**Each row table has these columns:** the label; when it is printed; what the row says; the kind; the summary
field that holds the same number, or none; and an example, by its block on the examples page. **A row table
quotes no printed value.** Values are the examples' job, and the examples are generated, so a stage that rewords a
value re-blesses an example and has no prose to chase. This is the rule that keeps the page from drifting where
its test cannot see.

**The sections**, as a default that T2 may reorder:

1. the header and the four kinds;
2. describe's rows, which also covers `--count`'s (a subset) and `ein oracle`'s;
3. a mode's rows;
4. `--monotone`'s rows;
5. the `lattice` block's sub-rows and what each counts, with the theory left to
   [`region_dump.md` § Monotonicity](../../../docs/kernel/inference/region_dump.md#monotonicity--the-static-check);
6. the listing, the 100-line cut, and which object each `(model …)` holds;
7. queries and claims;
8. the forms after the read-out, with the object each prints and the numbers in its head;
9. `-s` and `-t`;
10. stderr;
11. the summary's answer blocks, and the owner of every other block;
12. the surfaces that repeat these numbers.

**What the page links and does not restate:**

- what R, k_s, k_c, a line and a model are:
  [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md);
- exit codes, and which stream each byte goes to:
  [`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes) and
  [§ 5](../../../docs/kernel/defined_behaviour.md#5-the-cli-surface). The stderr section lists the shapes of the
  lines and links § 4 for the codes;
- the options and their defaults: [`configuration.md`](../../../docs/kernel/configuration.md);
- the `--dump-answer` directory, and the summary's `work` and `root` blocks:
  [`answer_dump.md`](../../../docs/kernel/inference/answer_dump.md);
- the `monotonicity` block and the warning's theory: `region_dump.md`;
- the `--events` stream: [`events.md`](../../../docs/kernel/inference/events.md);
- how each mode is computed, what it costs, and which command answers which question:
  [`solve_interface.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md) § 3 and § 4, until
  P1f.20 re-homes it;
- the rulings that made the vocabulary: `read_out.md`, linked once as the record.

**Not its subject:** `--trace`'s file, `ein test`'s text and its `--json-report`, and the `--events` stream.
Each repeats rows the page defines. The trace's headline is the `asked`, count and `exhausted` rows, and the
report's `region` carries `k_s` and `k_c`. One table lists these surfaces with the rows each repeats and the test
that holds it to the summary (for example `test_cli::the_verbose_header_and_the_report_row_agree_field_for_field`,
SE-M1's cross-surface test), or records that none does. The table defines nothing.

### `--json-summary`: which fields are this page's

The summary is *"the same facts as stdout"*, from the same `ReadOut` (`region_answer.rs`'s module doc). On
`branching/05_mini_zebra.ein` its top-level keys are `schema`, `source`, `asked`, `region`, `verdict`, `queries`,
`claim`, `work`, `monotonicity`, `leftover`, `root` and `config` (`ein solve FILE --json-summary J`, 2026-11-08).

- **On this page: `schema`, `source`, `asked`, `region`, `verdict`, `queries` and `claim`.** These blocks carry
  stdout's rows as data, and today they are defined only in `read_out.md` § 8.1, a plan that differs from the
  writer in three places (above). On this page each number is defined once, in the row that prints it, and the
  row names its field. The fields that no row prints get one table of their own: `stopped`'s members,
  `complete_up_to`, each minimal hole, each state's `closure`, `from`, `minimal_generators`, `commitments`,
  `lines` and `listed`, `frontier`, and the rest that T1 finds. Defining these fields on a page about the summary
  would split each number from the row that prints it, and that split is SE-M1's shape again.
- **Not on this page: `work`, `root`, `monotonicity`, `config` and `leftover`.** Each already has an owner.
  `work` and `root` are defined in [`answer_dump.md` § The run summary](../../../docs/kernel/inference/answer_dump.md#the-run-summary),
  held field by field by `answer_dump::every_summary_counter_is_defined_on_this_page`. `monotonicity` is
  `region_dump.md`'s. `config` and `leftover` are `configuration.md`'s, in its defaults block and its
  `EIN_LEFTOVER` row. A second definition here would be the very copy this stage exists to remove. The `work`
  row links answer_dump.md's three counters and does not restate them.
- **An owners table** lists every top-level key and the page that defines it. The test holds it to the keys a
  summary has. T1 checks that each owner defines every field of its block, and lists any field one does not.
- `answer_dump.md` § The run summary keeps its list of blocks, and points at the page for the seven keys where
  it points at `read_out.md` § 8.1 today.

### The examples: a live golden page beside it, and M1e2's frozen

- **`utils/read_out_examples.py` writes `docs/kernel/inference/read_out_examples.md`**, and
  `read_out_examples.rs` reads it. The page's prose is in the reference's voice. X1 to X13 keep their numbers and
  anchors, so the reference cites them as `read_out.md` does. The page gains a run for every label no block
  prints (the ten above, and whatever T1 adds), and an `ein oracle` block for `the cube`. The test's caption
  parser learns `ein oracle`.
- **M1e2's `read_out_examples.md` is frozen**, and it is not moved. One banner line goes under its title: it is
  frozen at the commit, no test reads it since, and the live golden page is the new one. Nothing else in it moves.
  There are 29 links to it in 15 files, and 23 of them are in nine M1e2 records
  (`grep -rnoE '\]\([^)]*read_out_examples\.md' plans docs AGENTS.md utils/README.md ein.rs`, and the same over
  `plans/m1e2_correctness`, 2026-11-08). The records keep their links, so each still cites what it saw. Only the
  live readers are re-pointed: `AGENTS.md`, `solution_semantics.md`, `utils/README.md`, the test's own doc
  comment, and P1f.20's two plan files.
- **The alternative is a move**: a `git mv`, with all 29 links re-pointed in one commit, as P1f.20's rule 1
  asks. It would leave one golden page and no frozen one. It is not the default for two reasons. It edits nine
  M1e2 records for link targets alone. And it points each record at blocks that later stages change, so a record
  that quotes X9 would cite an X9 it never saw. The frozen copy is the legitimate case of
  [One artifact, one owner](../../../docs/kernel/architecture.md#one-artifact-one-owner): two artifacts, a
  snapshot and a live page, annotated at both sites with why they differ.

### The test: `ein-cli/tests/read_out_reference.rs`

The precedent is [`configuration.md`](../../../docs/kernel/configuration.md) with
[`config_reference.rs`](../../../ein.rs/crates/ein-cli/tests/config_reference.rs). Its judgement table is
hand-written and diffed against `ein_core::config::FLAGS`, *"what makes the prose columns safe to hand-write"*,
and its defaults block is the binary's own output. A second precedent is [`docs/api/rust.md`](../../../docs/api/rust.md)
with [`embedding.rs`](../../../ein.rs/crates/ein-cli/tests/embedding.rs), the marker-guarded region. The test
has four parts:

1. **`every_row_label_has_a_row_and_no_row_is_orphaned`**: the page's row tables, marked so that the test finds
   them, against the printer's label list (below), both ways and in printing order. A label added to the printer
   with no row fails. So does a row the printer no longer has.
2. **`every_row_is_printed_by_an_example`**: every label appears at its indent in at least one block of the
   examples page, so every row the page defines is one the binary prints in a real run. `-s`'s `wall` and `-t`'s
   `timing` rows print a clock, which no golden can hold. Each is held by one run with the times masked.
3. **`every_answer_field_of_the_summary_has_a_row`**: every leaf path of the seven keys, over the summaries of
   the examples' runs, is named by the page, and every path the page names is in a summary. The owners table
   names every top-level key, and each key it names exists.
4. **`each_row_prints_its_summary_fields_number`**: on every example run, every row whose summary column names
   a path prints that path's value. This is *one label, one number*, held for every row.
   `read_out::each_count_is_printed_once_and_is_the_summarys` holds it today for `k_s` and `k_c` alone.

`read_out_examples.rs`, re-pointed, still holds every block of the examples page to what the binary prints.

**The printer's label list.** The 45 labels are literals spread over the 1 947 lines of `read_out.rs`
(`wc -l`, 2026-11-08), and four more are computed, so no list exists for the page to be diffed against. The
default is an enum, `ein_render::read_out::Row`, whose `label()` is the only way `Out::row` and `sub_row` get a
label. The computed labels are variants that carry their numbers. `wall`, the `timing` rows and `the cube` go in
it, or in a second list in `ein-cli` that the test also reads. With the enum, a label the page lacks cannot be
printed without being in the list, and the compiler enforces that at every print site. The alternative is a
source scan of each row call's first argument, as `config_reference.rs`'s fourth test scans for `EIN_*` names.
It needs no refactor, and it misses a computed label. T1's census decides between the two. **Either way, no
byte of output moves**: every read-out golden stays unmoved, including `corpus_shapes.md5`'s read-out
renderings and the examples page's blocks.

### The record, and the pointers

- **`read_out.md` stays where it is, as M1e2's record**, and gains one line under its title. The line says the
  live vocabulary is the kernel page since M1f S1f.5.16 (with the commit), and that this page records the rulings
  of 2026-10-16 and what M1e2's stages amended, and is not kept current. The line links the kernel page, which
  exists by T4. Nothing else in the file moves, so its 13 links to the frozen examples page stay as they are.
- **The live pointers move to the page**: `AGENTS.md`'s *"The read-out's vocabulary is …"* and the gate block's
  `read_out_examples.py` line; each kernel page and crate doc comment that cites `read_out.md` as the layout the
  code prints (T1 sorts these from the ones that cite a ruling, which stay); `solution_semantics.md` § 5.1's table,
  which becomes a pointer that keeps its definition-level sentences, and § 3's projection sentence, which is
  corrected; `answer_dump.md` § The run summary; `inference/README.md`'s tree; `docs/kernel/README.md`'s page
  count and its user and embedder rows; and `utils/README.md`.
- **The later stages' page lists name the live page**: S1f.5.6, S1f.5.7, S1f.5.8 and S1f.5.12 where they name
  `read_out.md` or the golden page, and S1f.5.14's read-out risk.

### P1f.20

This stage creates the page now. [P1f.20](../p1f.20_docs_refactor/README.md)'s disposition table,
[S1f.20.2](../p1f.20_docs_refactor/s1f.20.2_the_disposition_table.md), decides later where the page finally sits
in `docs/ein/`, and this stage does not choose. Its default is `user/reference/`: the read-out is the *result*
half of *problem → solve as a black box → result*. What S1f.20.2 then does differently is one line. Its row for
the read-out is the two kernel pages, each moved with its test, its generator's path and its inbound links in one
commit (rule 1), as [S1f.20.5](../p1f.20_docs_refactor/s1f.20.5_ein_rs.md) moves `rust.md`'s region. It is no
longer a page under `plans/` to be rewritten. `solve_interface.md` remains a user-facing reference under `plans/`
for S1f.20.2 to place. This stage takes none of its content.

### What does not move

No answer, exit code or summary field moves, and no byte of stdout or stderr. The only goldens touched are the
live examples page, which is written at its new path with new blocks, and the frozen page's banner. That holds
unless the user rules Q1 (b) or (c).

## Q1 — Q-M1e.7's read-out half

The ruling said *model* is printed under that name. Today the claim form owns the word, and § 3's model prints
as *final-state facts*. There are three options:

- **(a) Document it.** The page names the two objects apart: a listing's `(model …)` is the claim form, a state's
  facts over the candidates' relations, and § 3's model is what `-P` prints. `glossary.md` says the same. Nothing
  printed moves.
- **(b)** (a), and the heads move. `-P`'s head names § 3's model, and `-t`'s `read-out` row says *each listed
  state* instead of *each listed state's model*. The goldens that hold `-P`'s head move, and T1 names them first.
- **(c)** (b), and each state's model is a summary field, so `--json-summary` prints it under that name, as the
  ruling said.

**The agent recommends (a).** The claim form's name belongs to the language, and the grammar is final from
S1f.5.5's T3 on (the user's ruling of 2026-11-08). Printing *model* over § 3's object on the same screen as
`(model …)` blocks that hold another set would make SE-M1's collision worse, not better. (a) is also the stage's
default. T2 and T3 do not depend on Q1, because under every option the page names both objects. So Q1 is put to
the user with T1's census, if it is still open then, and only T4's head edits wait on (b) or (c).

## The method

### Task T1f.5.16.1 — the census: every row and number, from the code and the binary, not from `read_out.md`

- **From the code.** List every print site of `ein solve`'s and `ein oracle`'s stdout and stderr: `ein-render`'s
  `read_out.rs` and the forms' renderers, and `ein-cli`'s `modes.rs` (`-s`, `-t`, and the forms through
  `extras`) and `oracle.rs`. For each site, record its label or line shape, the condition it prints under, and the
  numbers it prints.
- **From the binary.** Run every corpus program under every mode the page covers: describe, `--count`,
  `--minimal`, `--maximal`, `-n 1`, `--single`, `--maximal --single`, `--monotone`, and `ein oracle` where the cube
  is within the limit. Write `--json-summary` beside each run, so that every label is tied to at least one program
  that prints it, and every numeric row to the summary field that holds the same number. The numbers must be
  equal on every run, or the difference is listed. The slow entries run under `EIN_CORPUS_SLOW=1`, as the
  corpus's other sweeps do.
- **Per mode**: which rows print, and which of the four kinds each numeric row counts.
- **Every word that two surfaces use for a number**, such as *saturations* on `work` and on `-t`'s `route`,
  and *model*: check that each is one number and one object, or list it.
- **Every difference with `read_out.md`**, each with a disposition. Either the prose fell behind, and the page
  states the code; or the printer says something false. A false line is a finding, shown to the user and not
  fixed here: a surface that misreports is P1e2.3's class, and a register row if it misstates an answer.
- **The summary.** List every leaf path of the seven keys over the runs above, against § 8.1. Check that each
  owner page defines every field of its block.
- **The inbound references.** Every link and doc comment that cites `read_out.md` or `read_out_examples.md`,
  sorted into *cites the vocabulary* (re-pointed at T4) and *cites a ruling or a record* (stays).
- **The label list's form**: the enum or the scan, from how many labels are computed.
- **The record**: the census and the raw runs under `build/s1f.5.16/` (gitignored), the tables in a § T1 record
  of this file, and Q1 put to the user if it is still open.

### T1 record, 2026-11-09

Taken at `dc4c0bdc` on that day's release binary, by `build/s1f.5.16/census.py` (gitignored, kept with its
`census.json`; its raw runs, 1.2 GB under `build/s1f.5.16/runs/`, were deleted at the stage's end, and a run of the
script re-takes them): every corpus entry of the `positive`, `regression` and `stdlib` groups (351) under
describe, `--count`, `--minimal`, `--maximal`, `-n 1`, `--single`, `--maximal --single` and `--monotone`, each with
`--json-summary` and `-s -t` (`-s` alone under `--monotone`), and under `ein oracle FILE --json-summary J -s`; the 62
negative entries under describe alone. **3 221 runs**, the slow entries among them (`EIN_CORPUS_SLOW` gates a test,
not a binary run), in 31 s at twelve jobs. 292 programs answer under every `ein solve` question and 59 are refused at
load or by *forbid* (exit 1, under each); `--monotone` answers 294; `ein oracle` answers 255 and refuses 37 past
`--region-limit` (exit 2). No run exited 0 with a row-shaped line the source scan does not name.

**From the code.** The scan of `read_out.rs` for the first argument of `row`, `sub_row` and `row_text` finds the
spec's 45 literal labels and four computed ones (the mode's count row, one of `states of R`, `minimal line` and
`maximal line`; `query` or `query i of n`; `class i of n`; `decided` in place of `exhausted`), and two that are not
labels: `""`, a row's continuation (a claim's further lines, an owed instance under `owing`, a class's binding rows
and its `:goal-text` sentences, a cut class's `…` line, `as :expect`'s further lines), and `"  {head}"`, the class
row. Printed outside it: `-s`'s `wall` (`modes.rs`, twice: a question and `--monotone`), `-t`'s `timing` table
(`modes.rs`'s `timing`, five rows: `load`, `route`, `read-out`, `print`, `total`, in its own `  {:<10}` layout) and
`ein oracle`'s `the cube` line (`oracle.rs`, at column 0). Lines that are not rows: the header and its rule; the
listing (`(region …)`, `(merged …)`, a member's `(model …)`, `(false)`, `(region (owing …))`) and its comments
(`;; state i of k — …`, `;; state i is reached by …`, `;; R as one (region …) is n lines, past stdout's 100 …`,
`;; not shown: …`, `;; … n more states: …`, `;; the line members this run certified …`, `;; n other members met are
not certified on a line …`); each form's head after the read-out (`H — the universe: …`, `models — the determining key
of …`, `models — …, each as a grid`, `── state i of k ──` and `final-state facts (…; n facts):` under `-p`, `-P` and
`-f`, `--diff-models`' head); and stderr (below).

**Every label, by the runs that print it** (a run counted once per label; `ein solve -y`'s `paths` under a monotone
`lattice` row is a lever and not in the sweep):

| label | runs that print it, by question | first program |
|---|---|---|
| `wall` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292, --monotone 294, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `asked` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292, --monotone 294, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `universe` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292, --monotone 294, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `work` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292, --monotone 294, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `route` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292, --monotone 294, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `lattice` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `-t print` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292 | `examples/branching/01_saturate_only.ein` |
| `-t load` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292 | `examples/branching/01_saturate_only.ein` |
| `-t read-out` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292 | `examples/branching/01_saturate_only.ein` |
| `-t total` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292 | `examples/branching/01_saturate_only.ein` |
| `-t route` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, --single 292, --maximal --single 292 | `examples/branching/01_saturate_only.ein` |
| `exhausted` | describe 292, --count 292, --minimal 292, --maximal 292, -n 1 292, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `query, query i of n` | describe 201, --minimal 201, --maximal 201, -n 1 201, --single 201, --maximal --single 201, oracle 167 | `examples/branching/01_saturate_only.ein` |
| `as :expect` | describe 201, --minimal 201, --maximal 201, -n 1 201, --single 201, --maximal --single 201, oracle 167 | `examples/branching/01_saturate_only.ein` |
| `over` | describe 201, --minimal 201, --maximal 201, -n 1 201, --single 201, --maximal --single 201, oracle 167 | `examples/branching/01_saturate_only.ein` |
| `class i of n` | describe 156, --minimal 156, --maximal 149, -n 1 156, --single 156, --maximal --single 149, oracle 126 | `examples/branching/01_saturate_only.ein` |
| `states (k_s)` | describe 292, --count 292, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `commitments (k_c)` | describe 292, --count 292, oracle 255 | `examples/branching/01_saturate_only.ein` |
| `maximal line` | describe 233, --maximal 292, oracle 203 | `examples/branching/01_saturate_only.ein` |
| `minimal line` | describe 233, --minimal 292, oracle 203 | `examples/branching/01_saturate_only.ein` |
| `∅` | describe 233, --count 233, oracle 203 | `examples/branching/01_saturate_only.ein` |
| `verdict` | --single 292, --maximal --single 292 | `examples/branching/01_saturate_only.ein` |
| `decided` | --single 292, --maximal --single 292 | `examples/branching/01_saturate_only.ein` |
| `components` | describe 233, oracle 203 | `examples/branching/01_saturate_only.ein` |
| `sub-cubes` | describe 233, oracle 203 | `examples/branching/01_saturate_only.ein` |
| `N` | describe 233, oracle 203 | `examples/branching/01_saturate_only.ein` |
| `merged` | describe 224, oracle 203 | `examples/branching/01_saturate_only.ein` |
| `monotone` | --monotone 294 | `examples/branching/01_saturate_only.ein` |
| `states of R` | -n 1 292 | `examples/branching/01_saturate_only.ein` |
| `outside` | describe 32, --count 32, --minimal 32, --maximal 32, -n 1 32, --single 32, --maximal --single 32, --monotone 32, oracle 22 | `examples/branching/05_mini_zebra.ein` |
| `the cube` | oracle 255 | `examples/branching/01_saturate_only.ein` |
| `site` | describe 29, --count 29, --minimal 29, --maximal 29, -n 1 29, --single 29, --maximal --single 29, --monotone 29, oracle 22 | `examples/features/01_not_and_absent.ein` |
| `assumes` | describe 29, --count 29, --minimal 29, --maximal 29, -n 1 29, --single 29, --maximal --single 29, --monotone 29, oracle 22 | `examples/features/01_not_and_absent.ein` |
| `status` | describe 29, --count 29, --minimal 29, --maximal 29, -n 1 29, --single 29, --maximal --single 29, oracle 22 | `examples/features/01_not_and_absent.ein` |
| `state` | describe 29, --count 29, --minimal 29, --maximal 29, -n 1 29, --single 29, --maximal --single 29, oracle 22 | `examples/features/01_not_and_absent.ein` |
| `paths` | describe 29, --count 29, --minimal 29, --maximal 29, -n 1 29, --single 29, --maximal --single 29, oracle 22 | `examples/features/01_not_and_absent.ein` |
| `E` | describe 21, oracle 203 | `examples/branching/01_saturate_only.ein` |
| `empty` | describe 59, --count 59, oracle 52 | `examples/ein-bugs/zebra2-bad.ein` |
| `claims` | --count 105 | `examples/branching/15_lookahead_two_step_on.ein` |
| `types` | describe 10, --count 10, --minimal 10, --maximal 10, -n 1 10, --single 10, --maximal --single 10, --monotone 11, oracle 4 | `examples/branching/02_one_dead_one_alive.ein` |
| `refuted by` | describe 39, oracle 32 | `examples/ein-bugs/zebra2-bad.ein` |
| `unsat core` | describe 39, oracle 32 | `examples/ein-bugs/zebra2-bad.ein` |
| `stale` | describe 11, --count 11, --minimal 11, --maximal 11, --single 3, --maximal --single 11, oracle 6 | `examples/features/01_not_and_absent.ein` |
| `:expect` | describe 7, --minimal 7, --maximal 7, -n 1 7, --single 7, --maximal --single 7, oracle 7 | `examples/features/11_expect_ambiguity.ein` |
| `negative cycle` | describe 6, --count 6, --minimal 6, --maximal 6, -n 1 6, --single 6, --maximal --single 6, oracle 6 | `examples/features/07_unstratifiable.ein` |
| `strata` | describe 6, --count 6, --minimal 6, --maximal 6, -n 1 6, --single 6, --maximal --single 6, oracle 6 | `examples/features/07_unstratifiable.ein` |
| `owing` | describe 20, oracle 20 | `examples/ein-bugs/wrong-answers/owed-two-step.ein` |
| `order` | describe 4, --count 4, --minimal 4, --maximal 4, -n 1 4, --single 4, --maximal --single 4, --monotone 4, oracle 4 | `examples/features/07_unstratifiable.ein` |
| `bit` | --monotone 29 | `examples/features/01_not_and_absent.ein` |
| `cycle` | describe 3, --count 3, --minimal 3, --maximal 3, -n 1 3, --single 3, --maximal --single 3, --monotone 3, oracle 3 | `examples/features/07_unstratifiable.ein` |
| `…` | describe 5, --count 5, --minimal 5, --maximal 5, --maximal --single 5 | `examples/features/01_not_and_absent.ein` |
| `minimal hole` | describe 6, oracle 6 | `examples/lattice/04_obligation_trigger_hole.ein` |
| `source` | describe 3, oracle 1 | `examples/ein-bugs/zebra2-bad.ein` |
| `core search` | describe 1, oracle 1 | `examples/ein-bugs/wrong-answers/core-search-cut.ein` |

Every label the scan names is printed by some run but `paths` under `-y`, which the examples page will print. The
census runs print `states of R` under `-n 1` only (`--single` prints `verdict`), and route 1's `exhausted` reason
(*route 1 stopped before layer L*) under none: describe takes route 1 only on a flagged program whose root is dead
([`explore.rs`](../../../ein.rs/crates/ein-infer/src/explore.rs)'s § Describe's route), which reads root alone and
never meets the budget.

**Per question, the rows and their kinds.** Describe and `ein oracle`: `asked`; R's counts (`states (k_s)`,
`commitments (k_c)`), `exhausted`; `∅` or `empty`; R's lines and shape (`minimal line`, `maximal line`,
`components`, `sub-cubes`, `merged`); around R, `N`, `E` (route 2 only), `lattice` and its block, `strata` and its
`negative cycle`s, `minimal hole`, `universe` with `outside` and `types`; the run's `route` and `work`; then the
listing, the queries, the claims, and `ein oracle`'s `the cube`. `--count`: describe's rows without the lines, the shape, `N`,
`E` and the holes, then a `claims` row where the program states a claim. A mode (`--minimal`,
`--maximal`, `-n N`): `asked`, its count row (the states listed), `exhausted`, then `lattice`, `universe`, `route`,
`work`; `--single`: `verdict` for the count row and `decided` for `exhausted`. `--monotone`: `asked`, `monotone` and
the warning's static rows with `bit`, `universe`, `route`, `work`. Of the kinds: R is the counts, `∅` and `empty`, the
lines and the shape, a mode's count row and `verdict`; around R is `N`, `E`, `lattice` and its block, `strata`,
`minimal hole`, `universe`, `the cube`; the run is `exhausted`'s and `decided`'s reasons, `route`, `work`, `wall` and
`timing`; the listing is its comments, `class i of n`, the `…` lines and `as :expect`'s *not printed*.

**One label, one number, on every run.** Each numeric row was held to the summary field the draft mapping names
(the integer printed, or an array's length, or each string of a list; a zero may print as a word, *none*, *in R*,
*no hole*): `states (k_s)` `region.k_s`; `commitments (k_c)` `region.k_c`; `exhausted` `region.exhausted` and
`region.stopped`'s numbers; `∅` `region.root_owes`; `minimal line` `region.minimal.states`; `maximal line`
`region.maximal.states`; `components`, `sub-cubes` the lengths of `region.components` and `region.subcubes`;
`merged` `region.merged.states` and `.minimal`; `N` `region.nogoods`; `E` `region.discharging`; `lattice`
`monotonicity.sites`' length and `region.holes.count`; `universe` `region.universe.candidates` and `.dead_alone`
(`root.hypgen.emitted` under `--monotone`, whose `region` is `null`); `route` `region.licences`; `work` the `work`
block; `verdict` `verdict.word`; `unsat core`, `owing` and `minimal hole` the k-th element of `region.core`,
`region.frontier[].owes` and `region.holes.minimal[].owes`; `refuted by` `region.refuted_by`; `query` the goal,
`over` the classes' count and `class i of n` its states; `outside`, `types`, `status`, `state` and `paths` their
`root.hypgen` and `monotonicity` fields; a mode's count row the `region.states` whose `listed` is true; `-t`'s
`route` saturations `work.saturations`. **346 of the checks differ, in three kinds, and none is a row printing
another number than its field**:

- 313 are `route` on route 2 (describe 29, `--count` 29, `ein oracle` 255): the row says *no licence*, and
  `region.licences` is `["L7 the cube"]`. Finding (a) below.
- 31 are `region.stopped`'s `n` printed as the exponent of `2^n` (the census's tokeniser read `2^118` as `2` and `118`
  joined), and its `layer` on the state route: `floors.ein`'s `layer` is 10 where the row says *the states of 9
  candidates*. The first is the census's; the second is finding (b).
- 2 are `paths` on `lattice/08_absent_witness_hole.ein`, whose *path-dependent* sentence prints the parents and no
  count of printed states: the row's number is `monotonicity.paths.parents` alone where a path differs.

**The words two surfaces use for a number**, each checked:

- *saturations*: `work`'s and `-t`'s `route` row are one number on all 2 044 runs that print both, `-t` printing it
  without digit groups. The `bit` row's *2^n = x saturations* and `paths`' *would cost up to x saturations* are their
  own numbers, each said by its row.
- *model*: a listing's `(model …)` is a state's facts over the candidates' relations (`ein_infer::read_out::Listed`'s
  `model`, the summary's `region.states[].facts`); `-P` prints [`solution_semantics.md` §
  3](../../../docs/kernel/inference/solution_semantics.md#3-model)'s model, under the head *final-state facts (beyond the
  program as loaded, (not …) omitted …)*; and `-t`'s `read-out` row, *each listed state's model*, means the listing's.
  Two objects, one word: [Q1](#q1--q-m1e7s-read-out-half), below.
- *states*: `states (k_s)` counts R's; a mode's count row the states listed; `over`'s *n states* the states its classes
  are over; `asked`'s *N states of R (-n N)* the request; a comment's *state i of k* the listing.
- *commitments*: `commitments (k_c)` is R's; `minimal line`'s *from m commitments* is `region.minimal.commitments`;
  `merged`'s *with m minimal* and a comment's *c commitments, g minimal* count one state's; `exhausted`'s *its n
  commitments* (route 1) and *their n extensions* (the state route) are `region.stopped.commitments`.
- *candidates*: `universe`'s n is H's; *the least guess is l candidates* is `region.minimal.least`; *complete up to …
  of L candidates* a size; a comment's *k candidates* a state's closure.
- *holes*: `lattice`'s count, `region.holes.count` and `monotonicity.status.holes` agree on all 66 runs that hold the
  last two (22 each under describe, `--count` and `ein oracle`).
- *members*: a listing's *other members met* and `over`'s *the n members met* are members of R a run met, one object.
- *layers*: `work`'s, route 1's *layer L* and the state route's `stopped.layer`: finding (b).
- Digit groups: every count but four prints them. `-t`'s `route` row, `the cube`'s two counts and a state's
  *2^n* exponent print plain digits, and `--region-budget` is echoed as typed.

**Every difference with `read_out.md`**, each disposed of. None is a wrong answer.

| `read_out.md` | the code | disposition |
|---|---|---|
| § 4 `route`: *licence L1, dead is upward-closed* | `L1 dead is upward-closed` (`Licence::label`) | the prose fell behind; the page quotes no value |
| § 4 and § 6.1 name `unsat core`, `refuted by`, `core search` | and `source`: each fact of root's core with its `:source`, `(fact) :source "…"` (3 programs) | the prose fell behind; the page has the row |
| § 4 has no `paths` under a monotone `lattice` | `-y` prints it there since M1e2 S1e2.1c.5 | the prose fell behind |
| § 4 `E`, *cube only* | route 2: describe on a flagged program (21 runs) and `ein oracle` | one statement; the page says route 2 |
| § 6.2, a partial route-1 run | no CLI run reaches it (above); the branch stands | the page states it, and that no corpus run prints it |
| § 8.1 `route: "1"\|"2"` | `"1"`, `"2"`, `"states"`, `"tree"` | the prose fell behind |
| § 8.1 `over: all\|listed` | `all`, `minimal`, `maximal`, `listed` | the prose fell behind |
| § 8.1 names neither `schema` nor `source` | both, on every summary | the prose fell behind |
| § 8.1 `expect: null` (until the switch) | `null`, or `{outcome, lines}` | the prose fell behind |
| § 8.1 `stopped: {why, …}` | `budget`: `spent`, `projected`, `layer`, `commitments`; `limit`: `n`, `limit` | elided; the page defines each |
| § 8.6 `-s`: *the wall clock per phase, and route 1's per-layer table* | `-s` one `wall` row; `-t` the per-phase `timing` table; the per-layer lines are `-v`'s and `--layer-progress`'s, on stderr | the prose fell behind |
| § 8.6 `ein oracle`: *every hole row* | at most four, as describe (§ 6.3 says so) | the prose fell behind |
| § 8.6 `ein oracle`: *today's `the cube` line* | `the cube   n dead nodes with an alive superset · m discharged nodes with an owing superset`, then whether the static verdict held; *contradicted* exits 1 | unspecified; the page states it |
| no mention of `-t` | five rows | the page has them |
| [`solution_semantics.md` § 3](../../../docs/kernel/inference/solution_semantics.md#3-model): *No surface computes projection classes yet* | every query block prints them since M1e2 S1e2.1a.5 | a kernel sentence the read-out disproved; T4 corrects it |

**Findings: a line that says something false or ambiguous.** None misstates an answer, so none is a register row;
each is P1e2.3's class (*a surface that misreports*), shown and not fixed here, as the method says:

- **(a) Route 2's licence, in two words.** Describe's and `ein oracle`'s `route` row on route 2 says *no licence*
  (`2, the cube — no licence: the static check flags the program`, `… asked for by name`), where the summary's
  `region.licences` and a mode's `route` row on the same route name `L7 the cube` (every mode run on route 2 prints
  `2, the cube — L7 the cube`), the licence [`solve_interface.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md)'s
  table calls *always*. [Q-M1f.20](../open_questions.md#q-m1f20--route-2s-licence-no-licence-on-one-row-l7-the-cube-on-another),
  defaulted: documented, not moved.
- **(b) `stopped.layer` in two units.** On route 1 it is the commitment size the route stopped before; on the state
  route it is the route's layer, a state's size plus one (`saturation/square-bwd/floors.ein`: `layer` 10, *the states
  of 9 candidates*), and the state route's *complete up to states of k candidates* is held by no field
  (`complete_up_to` is `null`). [Q-M1f.21](../open_questions.md#q-m1f21--regionstoppedlayer-on-two-routes),
  defaulted: documented per route, not moved.
- **(c) A text's line breaks** (S1f.5.2's item, handed to this stage): a `:goal-text` holding `\n` or `\r`, and an
  obligation's `:why` under `owing`, print raw, so the text's further lines start at column 0, outside every row, and
  a class block counts each sentence as one of stdout's 100 lines however many it prints
  (`examples/features/15_unicode_escapes.ein`, `tests/text/01_unicode.ein`). It is the read-out's own printing
  (`read_out.rs`'s `row_text`, the class block's `“{t}”`, the `owing` rows), so it is fixed, at
  [T1f.5.16.5](#task-t1f5165--a-texts-line-breaks-keep-the-rows-column):
  [Q-M1f.19](../open_questions.md#q-m1f19--a-texts-line-break-on-the-read-out), defaulted. `--trace` prints no
  `:goal-text`: it prints a rule's `:why` and a premise's `:source` as markdown paragraphs, where a line break is a soft
  break, and that is the trace's printer and not the read-out's (the page's *not its subject*). `ein saturate --dump`
  and a state file quote every text (`ein_core::text::quote`), and the `source` row is quoted too.

**The summary's seven keys.** Every leaf path over the 3 221 runs: `schema`, `source`; `asked.{which, ask, n}`;
`region.{k_s, k_c, exhausted, complete_up_to, root, root_owes, empty, nogoods, discharging, route, licences,
components, subcubes, core, core_status, refuted_by}`, `region.stopped.{why, spent, projected, layer, commitments, n,
limit}`, `region.minimal.{states, commitments, least, trivial}`, `region.maximal.{states, trivial}`,
`region.merged.{states, minimal}`, `region.holes.count`, `region.holes.minimal[].{set, dead, core, core_status, owes}`,
`region.universe.{candidates, dead_alone}`, `region.states[].{facts, closure, from, minimal_generators, commitments,
lines, listed}`, `region.frontier[].{facts, set, owes, owed}`; `verdict.{word, which}`; `queries[].{goal, over, exact}`,
`queries[].classes[].{facts, states}`, `queries[].classes[].rows[]`, a map from each goal variable to its value, and
`queries[].expect.{outcome, lines}`; `claim.{outcome, lines}`. Against § 8.1: `schema`, `source`, `stopped`'s
members, `route`'s and `over`'s other values and `expect`'s shape are new, as the table says; nothing § 8.1 names is
gone. **The other five keys' owners**: `work` and `root` are [`answer_dump.md` § The run
summary](../../../docs/kernel/inference/answer_dump.md#the-run-summary)'s, held field by field by
`answer_dump::every_summary_counter_is_defined_on_this_page`; `config` is
[`configuration.md`](../../../docs/kernel/configuration.md)'s defaults block, held by `config_reference`; `leftover`
is `configuration.md`'s `EIN_LEFTOVER` row, which names `taken` and `states` and not `states_by_relation`;
`monotonicity` is [`region_dump.md` §
Monotonicity](../../../docs/kernel/inference/region_dump.md#monotonicity--the-static-check)'s, which names the block's
keys (`concludes`, `at_stake`, `assumes`, `cycles`, `order`, `status`, `state`, `paths`) and not these leaves:
`reads` and `absent_reads`' `dynamic_monotone` and `dynamic_non_monotone`, `cycles[].negative`, `paths.dead_parents`,
`paths.saturations`, `paths.unprinted.saturations` and `state.stale_nodes`. Those gaps are their owners', listed here
and left there.

**The inbound references**, sorted.

- *Cite the vocabulary*, re-pointed at T4: `AGENTS.md`'s *"The read-out's vocabulary is …"* and its examples page;
  `configuration.md`'s *printed in `read_out.md`'s read-out*; `implementation.md`'s two module rows (`read_out.rs`,
  `described.rs`); `solution_semantics.md` § 5.1; `answer_dump.md` § The run summary's § 8.1; the crate doc comments of
  `ein-render`'s `read_out.rs` and `dump/region_answer.rs`, `ein-infer`'s `described.rs` (its first link) and
  `read_out.rs`, `ein-cli`'s `modes.rs`, `summary.rs` (two) and `tests/region_answer.rs`, and `read_out_examples.rs`'s
  and `region_modes.rs`'s mentions of the golden page; `utils/README.md`; `utils/review/run.py`'s § 8.1 label;
  [P1f.20](../p1f.20_docs_refactor/README.md)'s README and
  [S1f.20.2](../p1f.20_docs_refactor/s1f.20.2_the_disposition_table.md); the later stages' page lists (S1f.5.6,
  S1f.5.7, S1f.5.8, S1f.5.12, S1f.5.14).
- *Cite a ruling or a record*, kept: `defined_behaviour.md`'s three § 9 links (the exit-code ruling); `answer_dump.md`'s
  § 8.2 (where `ein-answer/2` was decided); `events.md`'s § 8.6 (where the `region` event was); `described.rs`'s §
  6.2 and `downset.rs`'s § 6.2 (the certification argument); `trace/region.rs`'s § 8.4 (the trace's design, not the
  page's subject); `utils/review/run.py`'s § 8.4 label; S1f.5.15's record; and the 23 links in M1e2's records.

**The label list's form: the enum** (the spec's default). Four labels are computed, but each is one of a closed set:
the mode's count row is one of three literals, `decided` is a literal chosen in place of `exhausted`, and `query i of
n` and `class i of n` carry their numbers at the print site. So `ein_render::read_out::Row` has a variant for each,
the two numbered ones printed through one function that adds the numbers; a scan would miss all four. Taken by
default on the user's instruction of 2026-11-09.

**Q1: (a), document it.** Taken by default on the user's instruction of 2026-11-09
([Q-M1f.18](../open_questions.md#q-m1f18--which-object-the-word-model-names-on-the-read-out)): the page names a
listing's `(model …)` and § 3's model as two objects and says which surface prints which; nothing printed moves, and
T4 has no head to edit.

### Task T1f.5.16.2 — the page, and the live examples

- `docs/kernel/inference/read_out.md`, written from the census, as [§ The page](#the-page-docskernelinferenceread_outmd)
  says.
- `utils/read_out_examples.py` re-pointed at `docs/kernel/inference/read_out_examples.md`, its prose re-voiced,
  and the runs added for every label no block prints. `read_out_examples.rs` re-pointed, and its caption parser
  extended to `ein oracle`.
- M1e2's `read_out_examples.md` frozen with its banner.
- No output moves.

### T2 record, 2026-11-09

**The page**, [`docs/kernel/inference/read_out.md`](../../../docs/kernel/inference/read_out.md), current and with no
banner, written from T1's census in the spec's twelve sections. Its row tables are marked, `<!-- rows: NAME -->`, one
per printer section: `describe` (a bare `ein solve`, `--count`'s subset and `ein oracle`), `mode`, `monotone`,
`lattice`, `strata`, `universe`, `empty` (why R is empty), `queries`, `claims`, `clock` (`-s` and `-t`) and `oracle`
(`the cube`). Each row has the spec's six columns, and no row table quotes a printed value. A **kind** of `—` marks a
row that counts nothing (`asked`, a verdict on a claim). The **summary** column names the field whose number the row
prints, in a path notation the page defines: `[]` every element, `[k]` the element paired with the k-th row of that
label, `[q]` the query a row is under, `[i]` a numbered row's own number less one. The fields no row prints as itself
(a word field, a state's listing fields, `stopped.layer` per route) are a marked table of their own, § 11.2, and the
owners of the other five keys a third, § 11.1. § 6 names the two objects called *model* (Q1 (a), Q-M1f.18), and
`glossary.md`'s Model entry says the same where it said the read-out prints *the recorded KB*. Findings (a) and (b)
are stated where their rows are (`route`, § 11.2's `region.licences` and `region.stopped.layer`). The clock rows'
masked run is a marked block in § 9, for T3's test to compare.

**The live examples**, [`docs/kernel/inference/read_out_examples.md`](../../../docs/kernel/inference/read_out_examples.md),
written by `utils/read_out_examples.py`, re-pointed: 41 blocks, 40 of them whole runs, where M1e2's page had 32 and 31. X1 to
X13 keep their numbers, headings and anchors, their prose re-voiced as reference and its plan links re-pointed; X2
gains `--monotone` and `-y`; four new sections print every label no block printed: X14
(`features/07_unstratifiable.ein`, describe and `--monotone`: `cycle`, `order`, `stale`, `strata`, `negative cycle`,
`monotone`, `bit`), X15 (`features/01_not_and_absent.ein`: `…` past four stale nodes), X16
(`ein-bugs/zebra2-bad.ein` and `ein-bugs/wrong-answers/core-search-cut.ein`: `source`, `core search`) and X17 (two
`ein oracle` runs: `the cube`). `read_out_examples.rs` reads the live page, and its caption parser takes `ein oracle`:
it checks the 40, holds that one is an oracle's, and passed in 21 s (release, 2026-11-09). The `…` past four sites
is printed by no corpus program (T1's census), so the label `…` is held by the stale rows' `…`, which shares it.

**M1e2's `read_out_examples.md` is frozen**: one banner line under its title, *frozen at `f1a71ec4`*, the commit
before this one, where `read_out_examples.rs` last held it; nothing else in it moved, so its 23 inbound links from
M1e2's records still cite what they saw.

No output moved: the generator wrote no block the binary does not print, and `read_out_examples.rs` holds each.

### Task T1f.5.16.3 — the label list, and the test

- The label list, as T1 chose it. A test holds that no golden moved.
- `ein-cli/tests/read_out_reference.rs`, with the four tests [above](#the-test-ein-clitestsread_out_referencers).
- **Mutation-checked**, as `gate_steps.rs` was. Each of these fails one test: a label added to the printer, a
  row deleted from the page, a summary field renamed, and a row's field pointed at the wrong number. Each is tried
  and reverted, and the record says so.

### T3 record, 2026-11-09

**The label list** is `ein_render::read_out::Row`, declared by a `rows!` macro that writes each variant with its label
and its layout and `Row::ALL` beside them, so no label exists outside the list: 56 variants, the 45 literal labels,
the three computed ones (`StatesOfR`; `Query` and `Class`, numbered where they print by `Row::numbered`; `Decided`),
the second `…` (`MoreSites` and `MoreStale` share the label), `-s`'s `Wall`, `-t`'s five `Timing*` rows and
`TheCube`. `Row::TABLES` is the page's eleven tables by marker, each in printing order. `Out::row` and `sub_row` take
a `Row`; a continuation is `Out::more` and `more_text`, under no label; `line` is the one public printer of a row,
which `ein-cli` uses for `wall`; `modes.rs`'s `timing` and `oracle.rs`'s `the cube` take their labels from `Row`. No
string reaches a label but through a variant (`padded` is private). No byte of output moved: every read-out golden,
`corpus_shapes.md5`'s 1 890 `read-out[…]` and `oracle` lines (`grep -c`, 2026-11-09) and the examples page's 40 blocks among them, is unmoved
in the gate below.

**The test**, `ein-cli/tests/read_out_reference.rs`, the spec's four, sharing one run of each example block with
`--json-summary` (23 s, release, 2026-11-09):

1. `every_row_label_has_a_row_and_no_row_is_orphaned`: each marked table is its `Row::TABLES` entry, label for label
   (a numbered row by its pattern, `query i of n`), and every `Row` is in a table.
2. `every_row_is_printed_by_an_example`: every label but the clock rows' is printed at its layout by a block of the
   examples page, each block's rows in their tables' order (a row's place never goes back but to its table's first
   row, the next query's); the clock rows by `ein solve examples/lattice/02_genuine_3set_death.ein -s -t`, its times
   masked to `…` and compared with the page's marked block, and `-t`'s `route` saturations held to `work.saturations`.
3. `every_answer_field_of_the_summary_has_a_row`: every leaf path of the seven keys over the examples' summaries (a
   binding row's variable read as `?var`) is named by a row's summary column or the fields table, every path the page
   names is in a summary, and the owners table names exactly the summary's top-level keys.
4. `each_row_prints_its_summary_fields_number`: 912 row fields on the 40 runs (2026-11-09), each by its JSON type: an
   integer, or an array's length, among the row's counts (a zero may print as a word); each string of a list, or a
   string, in the row; a flag as *yes* or *no*.

**Mutation-checked**, each tried on the tree of this commit and reverted, `cargo test --release -p ein-cli --test
read_out_reference` (2026-11-09):

| mutant | fails |
|---|---|
| a label added to the printer: a variant `Extra` printed under `asked` | test 1, *`Extra` is in no table* |
| a row deleted from the page: `merged` out of the `describe` table | tests 1, 3 and 4 |
| a summary field renamed: `region.nogoods` written `nogood` | test 3, *`region.nogood` is named nowhere*, *the page names `region.nogoods`, which no example's summary has* |
| a row's field pointed at the wrong number: `states (k_s)`'s cell `region.k_c` | test 4, on `05_mini_zebra.ein` (8) and `merged_states.ein` (16), and test 3 |

`read_out::each_count_is_printed_once_and_is_the_summarys` stays: it holds `k_s` and `k_c` on its own cells, which
test 4 now holds with every other row on the examples'.

### Task T1f.5.16.4 — the pointers, and the record

- `read_out.md`'s line, and every pointer [above](#the-record-and-the-pointers).
- Under Q1 (b) or (c), the heads and the summary field, with their goldens named before they move.
- The later stages' page lists, as above.
- This file's status, the phase README's row and [§ The order](README.md#the-order), and `AGENTS.md`'s P1f.5
  passage.
- `python3 utils/doc_audit.py --links --check` reports 0 findings, `RUSTDOCFLAGS="-D warnings" cargo doc` is
  green, and so is `./run_tests.sh`.

### Task T1f.5.16.5 — a text's line breaks keep the row's column

Added 2026-11-09 by T1's census, for the item S1f.5.2 handed this stage (finding (c),
[Q-M1f.19](../open_questions.md#q-m1f19--a-texts-line-break-on-the-read-out), defaulted (a)). It runs after T1 and
before T2, so that the examples are written by the fixed printer.

- **The printer.** A row's value that holds a line break (`\r\n`, `\n`, `\r`, or Unicode's other mandatory breaks:
  U+000B, U+000C, U+0085, U+2028, U+2029) prints each further line at the column the value's first character is
  printed at, past its indent and an opening `“`, so a `:goal-text`'s sentence and an obligation's `:why` under
  `owing` stay inside their row. A class block counts every line it prints against stdout's 100, so the cut and the
  forms after it see the lines the screen shows.
- **The goldens it moves, named before they move**: `natural_language_text`'s
  `the_read_out_prints_the_goal_text_as_written` and `a_container_holds_the_texts`, which look for the sentence as one
  run of characters; `corpus_shapes.md5`'s `read-out[describe]`, `[maximal]`, `[minimal]`, `[single]` and `[n2]` lines of
  `examples/features/15_unicode_escapes.ein` and `tests/text/01_unicode.ein`, the two corpus programs whose
  `:goal-text` holds a line break (`git grep -n 'goal-text' -- '*.ein'`, 2026-11-09), and their `oracle` lines if that
  rendering prints the query's classes; and no other. No answer,
  exit code or summary field moves, and the register is unmoved: no answer is misstated.
- **The pages**: the two programs' headers, and `natural_language_text.rs`'s table, say how the read-out prints a text
  with a line break.

### T5 record, 2026-11-09

`read_out.rs`'s `row_text` and `sub_row` print a value through `hang`, which starts each further line of a text at the
column its first character is printed at (past the value's indent and an opening `“`), splitting at Unicode's
mandatory breaks with `\r\n` one; and the listing counts the lines a block prints (`lines_of`), so `fit`, the cut and
`Text::listed` see the screen's. `read_out::tests::a_texts_line_breaks_keep_the_rows_column` holds both.

What moved is what the task named, less one: **ten** lines of `corpus_shapes.md5`, the five `read-out[…]` lines of
each of the two programs (`15_unicode_escapes.ein` 31 lines before and after, its second line now indented;
`01_unicode.ein` 36 to 38, its `\r` now a line break too). Their `oracle` lines did not move: that rendering is the
cube's, with no query block. `natural_language_text`'s two assertions now look for the sentence as the read-out prints
it, each line after a break at column 25 (`sentence`), and the first also holds that no read-out line starts outside a
row. The two programs' headers say where the second line goes. No answer, exit code, summary field or register row
moved; `read_out_examples.md`'s blocks print no text with a line break, and did not move.

### T4 record, 2026-11-09

**The record.** M1e2's `read_out.md` gains one line under its title: the record, not the reference, since `79fc14b0`,
the live vocabulary the kernel page; nothing else in it moves, and its 13 links to the frozen examples page stay.

**The live pointers**, each one T1 sorted as citing the vocabulary: `AGENTS.md`'s *"The read-out's vocabulary is …"*,
its gate block's `read_out_examples.py` line, its page-state count (45 pages, 42 current) and one sentence in its P1f.5
passage; `configuration.md`'s *printed in the read-out*; `implementation.md`'s two module rows; `answer_dump.md` §
The run summary, which points at the page's § 11 for the seven keys; `solution_semantics.md` § 5.1, its question
table now a pointer that keeps its definition-level sentences, and § 3's projection sentence corrected (every query
block prints the classes, since M1e2 S1e2.1a.5); `docs/kernel/README.md`'s page count and its user and embedder rows;
`inference/README.md`'s tree; `utils/README.md`; `utils/review/run.py`'s summary label; the crate doc comments of
`ein-render`'s `read_out.rs` and `dump/region_answer.rs`, `ein-infer`'s `described.rs` (its first link) and
`read_out.rs`, `ein-cli`'s `modes.rs`, `summary.rs` (two) and `tests/region_answer.rs`, and `region_modes.rs`'s mention
of the golden page (`read_out_examples.rs` was re-pointed at T2); P1f.20's README. The links T1 sorted as citing a
ruling or a record stay (`git grep -nE 'p1e2\.1a_the_solution_region/read_out(_examples)?\.md' -- ein.rs/crates docs
AGENTS.md utils`, 2026-11-09: `defined_behaviour.md`'s three, `answer_dump.md`'s § 8.2, `events.md`'s § 8.6,
`described.rs`'s and `downset.rs`'s § 6.2, `trace/region.rs`'s § 8.4, `run.py`'s § 8.4 label, and the new pages' own
links to the record).

**The later stages' page lists** name the live page where they named M1e2's: S1f.5.6's T1f.5.6.4, S1f.5.7's
T1f.5.7.5, S1f.5.8's T1f.5.8.3, S1f.5.12's page list, and S1f.5.14's read-out risk. S1f.20.2's disposition row
already named the two kernel pages.

**Under Q1 (a)** no head moves; § 6 of the page and the glossary name the two objects.

**Found outside the brief.** (d) A block's label that fills its sixteen characters is printed with no space before its
value: `class 100 of 277` and its *28 states* print as `class 100 of 27728 states` in
`examples/features/04_open.ein`'s `answer.txt`, the one corpus listing that reaches a hundredth class (T1's census:
277 classes; stdout's 100 lines never reach one). It garbles a class's count of states, so the stage that fixes it
files a register row first;
[Q-M1f.22](../open_questions.md#q-m1f22--a-class-label-of-sixteen-characters-glues-to-its-value), defaulted:
documented, not fixed here. `read_out_reference.rs`'s row parser reads a numbered label by its words, so it is not
misled.

### Acceptance, re-taken 2026-11-09

- The page exists, current, with no banner; every label `ein solve` and `ein oracle` print is a row and every row a
  label, in printing order: `read_out_reference::every_row_label_has_a_row_and_no_row_is_orphaned`, and
  `every_row_is_printed_by_an_example` for the order.
- Every row says what its numbers count and names the summary field where one holds the same number, and on every
  example run the number is the field's: `each_row_prints_its_summary_fields_number`, 912 fields.
- Every field of the seven keys is defined on the page, and the owners table is held to the keys a summary has:
  `every_answer_field_of_the_summary_has_a_row`. The gaps T1 found in the other owners' pages (`region_dump.md`'s
  `reads`, `absent_reads`, `cycles[].negative`, `paths.dead_parents`, `paths.saturations`,
  `paths.unprinted.saturations`, `state.stale_nodes`; `configuration.md`'s `leftover.states_by_relation`) are their
  owners', listed in T1's record.
- No row table quotes a printed value; the values are on the examples page, written by `utils/read_out_examples.py`
  and held by `read_out_examples.rs`, every label printed by a block but the clock rows, held by the masked run.
- The page names a listing's `(model …)` and § 3's model as two objects (§ 6), and Q1 is (a).
- M1e2's `read_out.md` and `read_out_examples.md` stay where they are, each with one line.
- `AGENTS.md`, the kernel pages, the crates' doc comments and the later stage files point at the live page where they
  cite the vocabulary.
- No answer, exit code or summary field moved; the read-out goldens moved only at T1f.5.16.5, the ten digests of the
  two programs whose `:goal-text` holds a line break, named first.
- `./run_tests.sh` green at each code commit, `doc_audit.py --links --check` 0 findings, `cargo doc` green.

## Acceptance

- `docs/kernel/inference/read_out.md` exists, current, with no banner. Every row label that `ein solve` and
  `ein oracle` print is a row of it, and every row of it is a label they print, in printing order
  (`read_out_reference.rs`).
- Every row says whether its numbers count R, what lies around R, the run, or the listing, and names the summary
  field that holds the same number where one does. On every example run, that number is the field's.
- Every field of `schema`, `source`, `asked`, `region`, `verdict`, `queries` and `claim` is defined on the page.
  Every other top-level key names its owner page, and the owners table is held to the keys a summary has.
- No row table quotes a printed value. The values are on the examples page, which `utils/read_out_examples.py`
  writes and `read_out_examples.rs` holds to the binary. Every label is printed by one of its blocks, apart from
  the two clock rows, which are held by a masked run.
- The page names a listing's `(model …)` and § 3's model as two objects, and says which surface prints which.
  Under Q1 (b) or (c), the heads say so too.
- M1e2's `read_out.md` and `read_out_examples.md` stay where they are as the record. Each gains one line, and
  nothing else in either moves.
- `AGENTS.md`, the kernel pages, the crates' doc comments and the later stage files point at the live page
  wherever they cite the vocabulary.
- No answer, exit code or summary field moves. The read-out goldens are unmoved, apart from the examples page,
  which is written at its new path.
- `./run_tests.sh` is green, `doc_audit.py --links --check` reports 0 findings, and `cargo doc` is green.

## Risks

- **A reference page is another hand-maintained copy.** That is the old phase's risk, and it still holds
  ([§ The two stages that were P1f.5's](README.md#the-two-stages-that-were-p1f5s)). Labels, summary paths and
  numbers are held both ways. A row's *meaning* is not: a stage that changes what a row says and keeps its label
  passes the test with stale prose. The mitigation is the rule that no row table quotes a value, so that only the
  meaning column can drift. It is not removed, because no test reads prose. The later stages' page lists name the
  page, so a stage that changes a row is pointed at its line.
- **The label list touches the printer that every later stage edits.** It moves no byte, and the goldens hold
  it. It lands before S1f.5.6, S1f.5.7, S1f.5.8 and S1f.5.12 so that each of them edits the list rather than a
  string.
- **Computed labels and clock rows are held more weakly**: by pattern, and by a masked run.
- **The page moves twice**: here, and at P1f.20. The test reads it at one path constant, as `embedding.rs` reads
  `rust.md`, and S1f.20.5's rule moves the page and its test together.
- **A frozen golden page can be mistaken for the live one.** Its banner, `AGENTS.md`'s pointer and
  `utils/README.md` say which page is live.
- **The census sweep is large**: every corpus program under nine questions. It writes to `build/` and moves
  nothing.

## Notes

**What a later stage does.** It changes the page in the commit that changes the output. A new or renamed label
fails test 1 until the list and the page agree. A row that changes its summary field fails test 3 or test 4. A
reworded value is re-blessed in the examples page by `python3 utils/read_out_examples.py`. A changed meaning is
in the page's prose, which the stage's page list names. S1f.5.14's NL rendering of a state's facts, *"the answer
table is on every `ein solve`"*, is the largest such change, and it lands last, on the live page.

**Why after the swap and before S1f.5.7**, and not sooner. The swap and its non-grammar halves change how names
and strings print, and S1f.5.5's T6 changes what `--diff-models` compares. If the census and the examples ran
before them, the swap would move both. Of the stages still to run before it, none adds, removes or renames a row
(S1f.5.15's `strata` row is already in), so running earlier gains nothing. Running later leaves S1f.5.6, S1f.5.7,
S1f.5.8 and S1f.5.12 writing their read-out changes into prose no test holds to the printer, and the census would
then have four more stages' drift to find.
