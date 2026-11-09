# The read-out speaks R — the page diff for the vocabulary

> **The record, not the reference.** Since M1f [S1f.5.16](../../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md) (`79fc14b0`, 2026-11-09) the read-out's live vocabulary is [`docs/kernel/inference/read_out.md`](../../../docs/kernel/inference/read_out.md), held to the printer by `ein-cli/tests/read_out_reference.rs`. This page records the rulings of 2026-10-16 and what M1e2's stages amended, and is not kept current; its examples are the frozen [`read_out_examples.md`](read_out_examples.md).

**Stage:** [S1e2.1a.5](s1e2.1a.5_the_read_out.md), task T1e2.1a.5.1.
**Status:** proposed 2026-10-16, and **ruled the same day**: the user
answered § 11's three questions, and § 12 stands as decided. Two of the
answers changed the page: every answer exits 0 (§ 9), and stdout's listings
stop at 100 lines, with the whole read-out written by `--dump-answer`
(§ 4.1). **Nothing
in `ein solve` has moved.** T1e2.1a.5.2 builds what this page says.
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
holds 29 runs on thirteen programs, re-generated after the rulings. **Since
T1e2.1a.5.2 each block is printed by `ein solve` itself**, written by
[`utils/read_out_examples.py`](../../../utils/read_out_examples.py) and held
to the binary by `ein-cli/tests/read_out_examples.rs`. Until then they were
printed by `read_out_preview`, an `ein-infer` example that printed this layout
from the library's own R, and was deleted when the printer replaced it
(`git log --diff-filter=D -- ein.rs/crates/ein-infer/examples/read_out_preview.rs`).
This page quotes them as `X1`…`X13`.

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

## 3. Six rules every surface follows

1. **It says what was asked, and what it lists.** Which part of R (all of R,
   the minimal line, the maximal line) and which question (describe, N
   states, exactly one; or how many, `--count`, since M1e2 T1e2.1c.4.6). That is the `asked` row on stdout, `asked` in the
   summary, and the line a dump's `index.json` names.
2. **The counts are R's, named k_s and k_c, and only R's.** They are printed
   when the run described R. A mode counts its own line, under the line's
   name: `maximal line  3 states — all of them`. A bound says it is one, and
   why.
3. **No verdict word without an expectation.** The four words are
   `--single`'s answer, and nowhere else (§ 11, ruling 1). A claim the program
   states is judged `holds`, `FAILED` or `NOT CHECKED`, as it is today.
4. **Every printed state is a member of R (C1). It is marked as a line member
   only where that is certified.** The `(region …)` form, which reads back as a
   claim that these are *all* of R, is printed only for an exhausted
   description. A line, an `-n` cut or a partial run is listed as states, each
   a `(model …)` with a comment, and never wrapped in `(region …)`. Neither is
   a region that stdout cut at its 100 lines (§ 4.1).
5. **Completeness is one row, `exhausted`**, and `decided` under `--single`.
   When it is *no*, the row gives the reason: stopped as asked (`-n`), at the
   budget (with what was spent and what the next step projects), past
   `--region-limit`, or `-T`, which aborts with exit 2.
6. **Every answer exits 0** (§ 11, ruling 1). R empty, a partial run and each
   of `--single`'s four words are answers. Only an error is not: usage, a load
   error, `-T`, and a claim the program states that the run refutes (§ 9).

**Where a surface shows a value, it shows it as Ein text the loader reads
back to the same value** (M1e2 S1e2.3.8, P1e2.3's N5): the one printer,
`ein_core::text` through `Terms::display`, on every surface below, the
read-out, the summary, the dump, the trace, the events, the forms, the DOT
views and the diagnostics. A name prints bare only where it lexes as one
`SYMBOL`, and quoted everywhere else, so `(holds "x y")` is one argument,
`(holds "1")` is the name and `(holds 1)` the int, and `(pick "?v")` and
`(holds "not")` are names. A state's `(model …)` and the `as :expect` line
load as written. Until S1e2.3.8 every surface printed a name as its raw text,
and since a claim and a query's classes compare that text, a claim about the
int `1` held of a state holding the name `"1"`, and two classes that differ
by them were one (W28).

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
| `lattice` | the static check, and on a flagged program the detector's warning under it and the holes (§ 6.3) | `monotone (the static check) — R is convex: no hole` |
| `strata` (M1f S1f.5.15's T4, 2026-11-08) | on a program with no stratification only: how many negative cycles the rules' dependency graph has over every relation, and under it a `negative cycle` row each, its relations, the rules on its negative edges and what its parity may mean. A diagnostic, not a refusal, and a statement of what the graph licenses, never of this run: root's fixpoint *may* depend on the boundary's admission order and *need not* be unique (`syntax/rule-guards.ein`'s cycle never fires, and its fixpoint is unique); [`absent_semantics.md` § The stratification diagnostic](../../../docs/kernel/inference/absent_semantics.md#the-stratification-diagnostic) has every line it prints | `not stratifiable: 1 cycle through an absent — root's fixpoint may depend on the boundary's admission order, and need not be unique` (`features/07_unstratifiable.ein`) |
| `universe` | n, and how many candidates are dead alone, which no state can hold | `9 candidates, 6 dead alone`; X6: `no candidate: R is {∅} or empty, decided by root alone` |
| `route` | the route, and the licences it used | `1, the alive down-set — licence L1, dead is upward-closed`; describe on a monotone program since M1e2 S1e2.1c.2: `the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local`; a mode on one since S1e2.1c.4: `the obligation tree, over states — … · L5 an owed instance's alternatives are exhaustive` where a branch used L5, `the state route, each state at most once — …` where none did |
| `work` | saturations, facts beyond root and layers, as the interface's § 2.1 asks of every run. They do not depend on the machine or on `-j` (`every_mode_at_four_jobs_is_every_mode_at_one`). A speed lever may move them, which C4 allows (*cost counters aside*), so the lever sweeps compare every row but this one (§ 12). The wall clock stays behind `-s` and `-t` | `14 saturations · 66 facts beyond root · 4 layers` |

**Amended 2026-11-04, M1e2
[S1e2.2.8](../p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md)**, on the
user's ruling of 2026-11-03 on Q-M1e2.3, (a) *scoped, and the read-out says
so*: under `universe`, every question that prints it (describe, `--count`, the
lines, `--single`, `-n`, `--monotone`, `ein oracle`, and so `--trace`'s
headline and `answer.txt`) prints an **`outside`** row when H leaves out a
relation the program can conclude or ask about and does not scope out
itself: X1's `is-a — no candidate: no hrule proposes it. R holds of it only
what the program states and saturation derives`. It says what the relation is,
why the rung that built H proposes none of its facts, and what every state of R
then holds of it, as S1d.3.3's *a lower bound* says what a count licenses.
`--json-summary` carries it on every run as `root.hypgen.outside` (§ 8.1's
`root` block, `ein-summary/2` kept), `ein test` beside each claim it prints, as
the warning is (§ 8.5), and `ein oracle --dump`'s `region.md` and
`region.json` too.

**Amended 2026-11-04, M1e2
[S1e2.2.10](../p1e2.2_wrong_answers/s1e2.2.10_types_as_objects.md)**, on the
user's ruling of 2026-11-03 on Q-M1e.12, (a) *the fixture, with the
warning*: under `universe`, after `outside`, the same questions print a
**`types`** row when the blind enumerator built H and a candidate names an
object on the right of an `is-a`: X13's `Color — on the right of an is-a, and
6 candidates of co-located, is-a name it as an object: the blind enumerator is
untyped, and no signature names it`. It names the objects, the fact about the
KB that makes them types, how many candidates of which relations name them,
and why, and moves nothing else. `--json-summary` carries it on every run as
`root.hypgen.types_as_objects` (`{objects, relations, candidates}`,
`ein-summary/2` kept), `ein test` beside each claim it prints, and `ein
oracle --dump`'s `region.md` and `region.json` too.

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

### 4.1 Past 100 lines, stdout lists what fits

The user's ruling (§ 11, ruling 2). **Stdout's listings share 100 lines.**
A listing is R's states and their `(merged …)`s, a line's or a cut's members,
a query's classes, or a query's `as :expect`, and they are printed in that
order. Each prints whole items while the 100 lines last, and at least one.
The summary rows are never cut, and neither is a query's `over` row or a
claim's outcome.

- **A cut region is not wrapped in `(region …)`** (rule 4). Its first comment
  says how long the whole `(region …)` is; the states that fit follow with the
  numbers they have in the whole, so *state 37* on the screen is *state 37* in
  the file; the last comment names what was left out.
- **`--dump-answer DIR` writes the read-out uncut, as `DIR/answer.txt`**
  (§ 8.2). That is the flag the user asked after: it already writes the answer
  as files, and since S1e2.1a.4 none of them is the read-out. Without it
  nothing is written, and stdout is the same with it and without, as for every
  file a run writes.
- **A cut query** lists the classes that fit, says how many more there are,
  and prints no `as :expect` on stdout, since the claim would be about classes
  the screen does not show. An `as :expect` longer than what is left says how
  long it is and where it is; one of a single line is always printed, since
  the note would be as long.

[X13](read_out_examples.md#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines)
is `colors.ein`: R is 169 states, 537 lines as one `(region …)`, and stdout
lists states 1–37. [X10](read_out_examples.md#x10--zebra2ein-the-puzzle)'s
exhausted zebra2 is cut by its `(merged …)` alone: the one state fits, and its
63 minimal commitments are in `answer.txt`.

**Measured** with the preview over the 198 corpus programs that load, at the
default budget (2026-10-16): of the 178 exhausted runs, 15 are cut, and the
longest read-out is 126 lines with its summary rows (`nationalities.ein`;
uncut, `pets.ein` ran to 44 457). Of the 20 partial runs, 5 are cut, and the
longest is 124. The median exhausted read-out is 27 lines. One limit per
listing instead of a shared one let a read-out reach 186 lines
(`lattice/synth/trivial_line.ein`: a 50-line region, 93 lines of classes and
a 40-line `as :expect`).

**Amended 2026-11-03, M1e2
[S1e2.3.3](../p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.3_the_forms_past_100_lines.md)**,
on the user's ruling (a) of that day: **the forms printed after the read-out
are listings too, and share the same 100 lines.** They are `-H`'s candidates,
`--models key`'s rows, `--models grid`'s, `-p`'s, `-P`'s and `-f`'s states,
and `--diff-models`' entries, in that order, after every listing above. Each
is drawn from the states stdout showed, so the screen and the forms agree.
Each prints whole items while what the read-out left lasts, and at least one.
A form's head is never cut, as the summary rows are not: its title, the
key's size and column heads, `-H`'s line per relation, the diff's counts. Its
last line, `;; not shown: …`, names what it left out and the file that holds
it. **`--dump-answer DIR` writes each form asked for uncut**, drawn from every
listed state, as `DIR/forms/<form>.txt` (`hyp-stats`, `key`, `grid`,
`print-final-state`, `print-final-positive`, `print-final-hfacts`,
`diff-models`), named in `index.json`'s `forms`. **`answer.txt` stays the
read-out uncut.** `ein oracle` writes no answer directory, so its forms' line
names `ein solve FILE --dump-answer DIR`. `--count` lists no state and still
refuses every form (T1e2.1c.4.6). On `pets.ein` the forms went from up to
178 858 stdout lines (`-p`) to at most 134, and their files hold what they
printed before, byte for byte (`ein solve
examples/saturation/type-exclusivity/pets.ein FORM | wc -l`, 2026-11-03;
`ein-cli/tests/forms_cut.rs`).

## 5. The modes

[X2](read_out_examples.md#x2--lattice02_genuine_3set_deathein-one-program-every-mode)
runs one program under each.

| mode | `asked` | count row | listing |
|---|---|---|---|
| `--maximal` | `the maximal line — every state on it` | `maximal line  3 states — all of them` | each state as `(model …)`, `;; … · maximal` |
| `--minimal` | `the minimal line — every state on it` | `minimal line  1 state — all of them` | each state, with its minimal commitments |
| `-n N` | `N states of R (-n N)` | `states of R  2 states, as asked — R may hold more` | the first N in the route's order |
| `--maximal -n N`, `--minimal -n N` | the same, of the line | `maximal line  1 state, as asked — the maximal line may hold more` | the same |
| `--minimal`, `--maximal`, past `--region-limit` on a flagged program (M1e2 S1e2.2.26) | as above | describe's line row: `minimal line  1 state — ∅`, `maximal line  not known — without a licence it needs the whole cube` | describe's certified members of the line, none on the maximal line, then `;; N other members met are not certified on a line, and not listed: --dump-answer DIR writes each to DIR/models/` (§ 6.2) |
| `--single` | `is R one state? (--single)` | `verdict  Ambiguity — R holds two states or more; two of them below` and `decided  yes — two states settle it; …` | the one state, or the two |
| `--count` (M1e2 T1e2.1c.4.6, ruled 2026-11-03) | `how many states and commitments R holds (--count)` | describe's `states (k_s)` and `commitments (k_c)`, each qualified as describe qualifies it, then `exhausted`, `∅` or `empty`, `lattice` with a flagged program's warning, `universe`, `route` and `work`: describe's text, row for row, but the warning's `paths`, which says the spot check was not run | none: no state is read, so no query is answered. Each claim the program states is not checked, and one `claims` row says how many, with a `note:` per claim. Over all of R only |
| `--monotone` (M1e2 S1e2.1b.5, approved 2026-11-03) | `is the program monotone? — the static check, on root's fixpoint` | `monotone  yes — no site: …`, or `monotone  no — 1 site` with the warning's static rows (`assumes`, `site`, `cycle`, `order`) and a `bit` row, `not computed — …`, naming what computes the cube | none: it asks about the program, builds no node and lists no state; `route  none — a question about the program`, `work  1 saturation, root's · 0 facts beyond root` |

- **k_s and k_c are not printed under a mode.** They are R's, and a mode did
  not describe R (rule 2). `--count` is the exception that proves it: it asks
  for R's two counts and nothing else, takes describe's route to them, and
  is over all of R only, so beside `--minimal` or `--maximal` it is a usage
  error that names a bare `ein solve`. `--single`'s `Solution` certifies that the part it
  asked about is one state, and its row says which: `Solution — R is one
  state`, or `— the maximal line is one state`.
- **`route`, `lattice`, `universe` and `work` are printed under every mode**,
  as the interface's § 2.1 asks. The cost of a question is visible on its
  own run: zebra2's `--maximal -n 1` is 42 saturations, and `--single` is
  45 (X10), since M1e2 S1e2.1c.4's routes. They were 54 and 8 616 on route
  1.
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
| no alive node | `empty  every node is dead — root's own fixpoint derives (false)` | `unsat core` and `refuted by`, root's, and `core search` where the core's search was cut (M1e2 S1e2.2.23). `refuted by` names the rules of the last step of the core's derivation: for each fact of the contradiction the core was taken over, the first justification the store records (the primary, the alternatives, the smallest-closure arrival) whose premises the core derives, `(given)` for a fact the core holds itself (M1e2 S1e2.3.10, Q-M1e2.21 (a)); until S1e2.3.10 the first contradiction's primaries, which need not be a step the core's derivation takes (`alt-cap-core.ein`: `narrow` beside `(w1 X) (w2 X)`, which only `wide` refutes) | `(false)` |
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
| the listing | **the certified line members only**, as `(model …)`s, cut at 100 lines like any listing (§ 4.1), with one comment saying how many other members were met and that `--dump-answer` writes each to `models/` | rule 4; the floors run met 10 075 other members (X11), and they are not the answer to *describe* |
| queries | over every member met, marked *a lower bound*, cut at 100 lines, and **no `as :expect`** | each class shown is real; there may be more, and a pasted claim would say there are not |

**The state route's partial run** (describe on a monotone program, since M1e2
[S1e2.1c.2](../p1e2.1c_the_region_search/s1e2.1c.2_the_lines_by_duality.md)).
It takes the alive states by size and stops between sizes, so what it
certifies is *complete up to states of k candidates*, not commitments:

| row | on a partial state-route run | why it holds |
|---|---|---|
| `states (k_s)`, `commitments (k_c)` | `≥`, *complete up to states of k candidates*. k_c counts every commitment of a state it extended (L3) and those met of the rest | every alive state of at most k candidates was met and extended |
| `minimal line` | `≥ m — complete up to states of k candidates` | a generator of an extended state of R has every member of R below it among the extended states; a commitment met whose every one-down subset is a known owing state is minimal (L2) |
| `maximal line` | ⊤'s line as route 1's; otherwise `≥ m — all those of up to k candidates` | an extended state whose every extension is dead (L4) |
| `N` | `≥ x — those the run met with every one-down subset alive`; `0` when ⊤ is alive | a dead set saturated whose one-down subsets are all known alive is minimal |
| **R is the whole cube** | when ∅ and ⊤ are both in R: k_c = 2ⁿ, both lines, one component, one sub-cube and N = 0, exact; `merged` *not known*; the `exhausted` row says R is decided and only its states are not all counted | L1 and L2: ⊤ alive makes every node alive, ∅ discharged every node discharged |

[X10](read_out_examples.md#x10--zebra2ein-the-puzzle) is zebra2 at the
default budget. Until S1e2.1c.2 it was partial there, with the puzzle answered:
its one state certified minimal (from the single fact
`(drink-loc Juice House-4)`) and maximal. The state route describes it
exhausted at the default budget since.
[X11](read_out_examples.md#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube)
is a cube that is all of R, partial at the default budget: R is decided, its
states are counted in part.

**A program the static check flags, past `--region-limit`**
([X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit)):
the run visits the cube in layer order up to 2^limit nodes. One line is still
certified without a licence: a member met in layer order with nothing in R
below it is minimal, since every subset was met first, and when ∅ is in R the
minimal line is exactly {∅}. Those are what it lists. The maximal line needs
the whole cube, and the row says so. The `exhausted` row names the route that
declined and why: `no — not monotone, and the question needs the whole cube:
2^19, past --region-limit 16`. **It runs no walk.** It keeps each node it
met as its two bits, so a member of N it met has no core: `nogoods.json`
writes it with a `null` core and `null` rules beside `core_status:
"not-kept"`, and `--trace` says *Core: not kept: the run stopped at
`--region-limit`.* (M1e2
[S1e2.3.9](../p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md),
`b6139e62`; until then an empty core with no rule, 68 of 68 on X9's program).

**The line modes answer from the same description** (M1e2
[S1e2.2.26](../p1e2.2_wrong_answers/s1e2.2.26_line_modes_past_the_limit.md),
W27, the user's ruling of 2026-11-05 on
[Q-M1e2.19](../open_questions.md#q-m1e219--what-do---minimal-and---maximal-answer-past-the-limit)).
Past the limit `--minimal` lists the members describe certifies on the
minimal line, {∅} when ∅ is in R, and every `--maximal` question lists none:
the maximal line is not known. Each line row is describe's, the other members
met are counted on the listing's last line and not listed, `--dump-answer`
writes each, the summary's `lines` mark the listed states alone, and
`--maximal --single` is *not decided*. Until then each line mode labelled
every member it met with its line: 65 214 states `· minimal` on
`square-unique/corner-house.ein`, where describe certifies one, and on
`lattice/10_fifo_gap.ein --region-limit 1` two states on each line the
oracle puts on neither ([X9](read_out_examples.md#x9--lattice06_friends_tablesein-flagged-and-past-the-limit)
shows both modes).

**Why a partial run lists only certified line members.** Measured with the
preview over every corpus program that loads, 198 of them, at the default
budget (2026-10-16): listing every member they met, the flagged programs past
the limit printed up to 64 761 states and 648 435 lines
(`square-unique/terminus.ein`). Listing only the certified line members
lists one state on each of the six flagged corpus programs past the limit,
∅ (`ein solve FILE`, 2026-11-05). Until M1e2 S1e2.2.26 this paragraph said
that all 64 761 members `features/04_open.ein` meets are minimal, which is
585 192 lines: that was `--minimal`'s label (W27), not the line, which is
{∅}. The 100-line rule (§ 4.1) bounds every listing, and every member is in
`--dump-answer`.

### 6.3 Holes, and the warning: never silent

On a flagged program that the cube reaches
([X8](read_out_examples.md#x8--lattice04_obligation_trigger_holeein-a-hole)), the
`lattice` row was S1e2.1a.1's stub until M1e2 S1e2.1b.5 replaced it with the
detector's warning, in the wording the user approved on 2026-11-03
([`detector_proposal.md`](../p1e2.1b_non_monotone_programs/detector_proposal.md)
§ 4.2):

```
  lattice           not monotone: 1 site (the static check) — 1 hole
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            bad: absent reads q at absent depth 1, concludes (false) — at stake: the state, and dead with it
    status          non-monotone here (exercised, by bad) — the cube: 1 hole · 1 dead node with an alive superset · 0 discharged nodes with an owing superset
    state           no stale fact in the cube's 4 nodes
    paths           every parent path checked reaches the same state — 1 parent of 1 printed state
    minimal hole    {(p A)} — dead: (p A)
```

The rows say three things: **what is assumed** (`assumes`), **which property
is at stake** (each `site`, with each `cycle` and `order` finding), and
**whether it bit**, the status and the state apart, because the cube reads
only the status (`status`, `state` with up to four `stale` rows, `paths`).
On stdout at most four `site` and four `stale` rows are named, each then a
`…` row naming `--dump-answer`, whose `answer.txt` has them all; the block is
bounded on its own and does not take the listings' 100 lines. A monotone
program's one row is unchanged. Every minimal hole is still named with why it
is out (its core, or what it owes), up to four, and the count is always
printed: the hole rows' form is F21's D4, open. The `(region …)` listing is the
cube's R either way: it lists states, so a hole needs no special form.

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
asked     {which: all|minimal|maximal, ask: describe|find|single|count, n}
region    {k_s, k_c, exhausted, stopped: null | {why: asked|budget|limit|deadline, …},
           complete_up_to, root: in-r|owing|dead, root_owes, empty: null|dead|owing,
           minimal: {states, commitments, least, trivial}, maximal: {states, trivial},
           components: [sizes], subcubes: [dimensions], merged: {states, minimal},
           nogoods, discharging, holes: {count, minimal: [{set, dead, core, core_status, owes}]},
           universe: {candidates, dead_alone}, route: "1"|"2", licences,
           states:   [{facts, closure, from, minimal_generators, commitments,
                       lines: [minimal, maximal], listed}],
           frontier: [{facts, set, owes, owed}],               (empty R, owing kind)
           core:     [facts], core_status, refuted_by: [rules]}  (empty R, dead kind)
verdict   null | {word, which}                                (--single only)
queries   [{goal, over: all|listed, exact, classes: [{facts, rows, states}], expect: null}]
claim     null | {outcome: held|failed|not-checked, lines}   (the solve claim)
work      {saturations, facts, layers}
```

**Built in T1e2.1a.5.2's fifth step**, for the region modes; the walk keeps
`ein-summary/1` until the switch removes it. Under a mode R was not described,
so R's numbers are `null` and `states` holds the mode's own. Under
`--count` (T1e2.1c.4.6) R's numbers are describe's, and `states` and
`frontier` are `null`: no state was read. Its queries are listed with no
class (`over: listed`, none listed) and each claim `not-checked`. A query's
`expect` is `null` until its `:expect` lands at the switch, so that the shape
does not move then. `region_answer.rs` holds the key set to one shape on
every arm, nine of them.

`monotonicity`, `root` and `config` stay as they are. Every bound carries
`exhausted = false` beside it: a reader never has to know which numbers were
complete.

### 8.2 `--dump-answer DIR`: `ein-answer/2`

- **`answer.txt`**, the read-out uncut: stdout with no listing cut (§ 4.1).
  Its `(region …)` reads back as the claim, `(merged …)`s included.
- **`models/NN.ein`**, one loadable program per state the run reached, as
  today: the program, then the state's facts, one a line, each printed as
  text the loader reads back to the same value and its provenance a comment
  (since M1e2 S1e2.3.8: `answer_dump.md` § Every state file loads,
  `answer_round_trip.rs` reloading every one by value). Its header comment gives the
  state's number, its lines, its commitment and its minimal generators. On a
  partial run that is every member found, listed or not, which is where
  X11's 10 075 go. Each is read again through the node function, and
  `region_answer.rs` loads every one and finds it a fixpoint that holds its
  state's model.
- **`index.json`** names what `models/` holds: `asked`, the same `region`
  block as the summary, and per file its `lines`. That is the acceptance's
  *`models/` per line, named in `index.json`*.
- **`owing/NN.ein`** holds the frontier on an empty R of the owing kind, and
  **`nogoods.json`** holds N with each core. A core, wherever a surface
  prints one, is the facts its contradiction rests on: since M1e2 S1e2.2.21
  both sides of a pair and the facts a `forall` read (Q-M1e2.15 (b)), and a
  run takes one only where a surface prints it, which moves no line of any
  surface (`a_run_that_prints_no_core_takes_none_and_answers_alike`).
  **Since M1e2 S1e2.2.23 a core carries its status wherever it is printed**
  (W23): `core_status` beside it in every JSON, `within-budget`, `cut-` and
  the caps its search stopped at, `not-kept` beside a `null` core for a
  member of N a run past `--region-limit` met (S1e2.3.9, § 6.2), or `null`
  where no core was taken; and on
  every text surface, for a cut core alone, the cap: the read-out's `core
  search` row, *cut at its budget of 64 environments per fact: a smaller
  core may exist*, a minimal hole's *(core search cut at its budget of …)*,
  the trace's *Core search: …*. A core inside its budget prints nothing
  new.
- **`root_writes.json` and `alive_at_end.json` go.** Root is never written,
  and there is no walk to end.
- `answer.json` stays the summary byte for byte, as `answer_dump.rs` checks
  for the walk and `region_answer.rs` for the region modes.
- **The flags are additive**: stdout, stderr and the exit code are the same
  with them and without, so the header leaves them out, as it leaves out
  `-j`.
- **Under `--count`** (T1e2.1c.4.6) the directory holds `answer.txt` and
  `answer.json` and nothing else, as under `--monotone`: no state was read,
  so there is no state file to write. What a previous `ein-answer/2` dump
  left, its `index.json` and `nogoods.json` among it, is removed.

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
  core and refuting rule, or, past `--region-limit`, *not kept: the run
  stopped at `--region-limit`* (§ 6.2), in place of *Refuted hypotheses*;
  then **owing** when R is empty of that kind.
- **Cost**: one saturation per listed state, recorded. A description is
  traced as far as it lists.

### 8.5 `ein test`

- **Text**: unchanged, `ok` / `FAILED` / `NOT CHECKED` per file.
- **Report**: `ein-test-report/3`. A query row's `ran` (the walk's counters)
  becomes `answer {over, exact, classes, states}`. Solve rows are unchanged.
  `ein-test-report/4` since M1e2 S1e2.3.10, additively: every row carries
  `outside` and `types_as_objects`, what the universe leaves out, as
  `--json-summary`'s `root.hypgen` writes them, `null` where no run checked
  the row's claim; the lines `ein test` prints beside each claim it checked,
  *outside the universe* and *a type as an object*, as data (P1e2.3's N8).
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

**Amended 2026-11-03, M1e2
[S1e2.3.7](../p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.7_four_lines_that_misname.md)**,
on the user's ruling of that day (N1, N2, N4). Two rows above name a set
other than the one that prints, the forms' headers named a third, and `ein
oracle`'s read-out named a flag it refuses:

- **`-p` and `-P` print each listed state's facts beyond the program as
  loaded**, every one or the positive ones. `-P`'s set is the state's model as
  [`solution_semantics.md` § 3](../../../docs/kernel/inference/solution_semantics.md#3-model)
  defines it, and that model holds root's own derivations. *Facts beyond
  root* is the `work` row's set, which leaves them out. 148 corpus programs
  list ∅ under `ein solve FILE -p`, and on 77 of them ∅'s block holds facts,
  though ∅ has none beyond root (T1e2.3.7.1, 2026-11-03). The headers said
  *derived facts*, and a commitment is not derived. They now say what prints:
  `beyond the program as loaded: root's derivations, the commitment and what
  it derives`, and `-P`'s adds `(not …) omitted`. What the forms print did
  not move. Whether `-p` should print the facts beyond root instead is
  parked in the stage's notes.
- **`-f` prints every fact of the candidates' relations, given ones too**,
  which is more than *its closure*. Its header named `:hrules ['color-loc',
  …]`, in a Python list's spelling, on programs that declare no `:hrules`
  (166 of 186 runs, the same sweep). It now names the relations as Ein spells
  names: `of the candidates' relations, given ones too: color-loc drink-loc
  nation-loc pet-loc smoke-loc`.
- **`ein oracle`'s read-out names `ein solve FILE --dump-answer DIR`**
  wherever `ein solve`'s names `--dump-answer DIR`: a cut region, a cut
  listing, a query's cut classes, a long `as :expect`, the warning's rows
  past four. `ein oracle` refuses `--dump-answer`, exit 2, and writes no
  answer directory, so the line names the command that writes one, as its
  forms' line has since S1e2.3.3. `as :expect`'s *it is in DIR/answer.txt*
  adds *which ein solve FILE --dump-answer DIR writes*. Over the corpus
  (`ein oracle FILE` on the 279 files, 2026-11-03), 10 lines on 7 files
  moved and nothing else, and on each of the 7 the named command exits 0
  with the oracle's count of states. `ein oracle` gained no flag, and `ein
  solve`'s lines did not move.

Held by `forms_cut::the_fact_dumps_headers_say_what_they_print` and
`read_out::the_oracles_read_out_names_the_command_that_writes_its_rest`.

## 9. Exit codes

**An answer exits 0, and only an error does not** (§ 11, ruling 1): *"any
solution of ein program is successful execution of ein tool"*.

| run | 0 | 1 | 2 |
|---|---|---|---|
| every mode: describe, `-n`, `--minimal`, `--maximal`, `--single`, `--count` | **every answer**: R empty of either kind, a partial run, `Solution`, `Ambiguity`, `Contradiction`, `Open`, and `--single` not decided | a claim the program states that the run **refutes** (`FAILED`) | usage, `-T` |
| a load error | — | `ein solve` | `ein test`, unchanged |
| `ein test` | every claim `holds` | a claim `FAILED` or `NOT CHECKED`, unchanged | a load error, usage |

- **A refuted claim is the one answer that is an error.** The program asserts
  something about its own answer, and the run shows it false: the program is
  wrong, as a failed `assert` is. That is the reading the ruling needed, and
  it keeps M1c's `ein solve` behaviour for a false `:expect`.
- **A claim the run could not check is a note under `ein solve`**, not an
  error. The run answered a narrower question than the claim (an `-n` cut, a
  line, a budget), and asking one is the user's call. `ein test` keeps exit 1
  for it, since checking is all it does.
- **`--single`'s exit codes, ruled in
  [`solve_interface.md` § 2.1](solve_interface.md#21-ein-solve)
  as 1 for `Ambiguity`, `Contradiction`, `Open` and *not decided*, are 0.**
  That rule was there so a shell could test uniqueness. A program that
  expects one solution can say so as a claim, and a refuted claim exits 1.

On stderr there is one line per refuted claim, and one `note:` per claim not
checked and per partial run, so that a pipeline sees each:

```
examples/f.ein: (solve :expect) FAILED — the claim lists 1 state, R has 2
note: examples/zebra2.ein: partial — route 1 stopped before layer 5 at --region-budget 524288
```

## 10. Which of the user's questions the default answers

The user's list and the plan's additions, from
[§ What `solve` can answer about R](../open_questions.md#what-solve-can-answer-about-r-2026-10-16).
That page says *"which of them `ein solve` answers by default, and which
behind a flag, is S1e2.1a.5's to propose and the user's to rule"*. **Ruled as
it stands** (§ 11, ruling 3): the *by default* column is what bare `ein solve`
prints, and nothing in it moves behind `-s`.

| question | by default (describe) | by a mode or flag | not built |
|---|---|---|---|
| does R have a node? | `states (k_s)`; `empty` and its kind | `-n 1` | |
| is it the only one? | k_s = 1; `maximal line` 1 | `--single`; `--maximal --single` | |
| how many? | `states (k_s)`, `commitments (k_c)` | `--count`: the two rows and what qualifies them, nothing listed (T1e2.1c.4.6) | |
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

## 11. The user's rulings (2026-10-16)

The three questions this page asked, and the answers, in the user's words.

1. **Where the four words appear.** Proposed: only as `--single`'s answer,
   with empty R described by its kind (§ 6.1) and `-n N` a request, not an
   expectation. The alternatives were the words in a false claim's reason (B)
   and `Contradiction`/`Open` as the names of empty R's kinds (C).

   > yes, Solution/Ambiguity/Contradiction/Open appear only as --single's
   > answer, ein program exit not 0 only on error, any solution of ein program
   > is successful execution of ein tool - why it should be other way?

   **(A)**, and **every answer exits 0** (rule 6, § 9). The *other way* was
   [`solve_interface.md`](solve_interface.md)'s `--single`, which exited 1 on
   anything but `Solution` so that a shell could test uniqueness. A claim the
   program states does that job, and a refuted claim is the one answer that
   exits 1 (§ 9).
2. **How much of an exhausted R stdout lists.** Proposed: all of it (A), or
   64 states and then *n more* (B).

   > How much of an exhausted R to print - maybe make sense to save to file
   > if exceeds some limit e.g. 100 lines, I think flags to explicitely ask to
   > output into file already exist?

   **Cut at 100 lines, and the whole in a file** (§ 4.1). The flag exists:
   `--dump-answer DIR` writes the answer as files, and it gains `answer.txt`,
   the read-out uncut (§ 8.2). The other file flags write a different thing:
   `--json-summary` the summary as data, `--trace` the derivation. The 100
   lines are shared by every listing on stdout, and the partial run's 64 is
   gone. Over the corpus, 20 of 198 read-outs are cut, and none is longer
   than 126 lines (§ 4.1).
3. **The default's answers, § 10's *by default* column.** Proposed as it
   stands.

   > yes, bare ein solve prints as much details as possible, except full
   > lattice

   **As it stands.** Every row of it is computed by route 1, or by the cube
   only where the static check flags the program, so the default never asks
   for the cube by itself; `ein oracle` is the cube.

## 12. Decided here, and not objected to

- **The labels**: `states (k_s)` and `commitments (k_c)`, mirroring today's
  `solutions (k)`. `exhausted` everywhere, `decided` under `--single`. `asked`
  first.
- **A `(region …)` is printed only for an exhausted description** (rule 4).
  Lines, cuts and partial runs list states.
- **The state order is canonical**: closure size, then candidate order.
- **A printed `(model …)` closes the candidates' relations**, as
  `claim::print` does. A claim a program writes may name more: X6's fixture
  pins `(closed-here r)` where the printed form is `(model)`.
- **A partial run lists its certified line members**, under the 100-line rule
  (§ 4.1), and certifies ⊤ with one saturation when ⊤ is in R: L6's trivial
  line, for the price of one fixpoint.
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
- **A partial run writes one `note:` to stderr** and exits 0, and so does a
  claim the run did not check (§ 9).
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
  `test_cli.rs`, `region_modes.rs` (whose
  `single_exits_like_a_false_expectation` pinned `--single`'s exit 1, which
  ruling 1 makes 0; moved in T1e2.1a.5.2's first step), `model_set_report.rs`
  and `models_grid.rs`;
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
