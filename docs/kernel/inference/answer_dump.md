# The answer dump — `ein solve --dump-answer DIR`

**What a solve answered, as files.** One directory per run: the read-out
uncut, the summary, one **loadable** program per state the run reached, the
owing frontier of an empty R, the minimal dead sets, each form asked for
uncut, and an index naming each.
M1e2
[S1e2.1.2](../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.2_every_answer_a_file.md)
added the flag for the walk, because a person reviewing a wrong answer needs
to see the answer. Since
[S1e2.1a.5](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
switch it writes the **region** read-out's answer, `ein-answer/2`
([`read_out.md` § 8.2](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md)):
R, or the part of it the question asked, as [`solution_semantics.md`](solution_semantics.md)
defines it.

```sh
ein solve examples/ein-bugs/wrong-answers/owed-seats.ein --dump-answer out/
ein solve examples/zebra2.ein --maximal --dump-answer out/
```

## The directory

```text
out/
  index.json      what is here — the question, the summary's region block, and every state file by name
  answer.txt      the read-out uncut: stdout with no listing cut
  answer.json     byte-identical to --json-summary on the same run
  models/NN.ein   program + one state's facts, one per state the run reached
  owing/NN.ein    program + one owing node's facts, on an empty R of the owing kind
  nogoods.json    N: each minimal dead set, its core, the core's status and the rules that refuted it
  forms/F.txt     each form asked for, uncut: -H, --models key|grid, -p, -P, -f, --diff-models
```

- **`NN` is stdout's numbering.** `models/02.ein` is *state 2 of k* in the
  `(region …)` block, two or more digits, so the files list in that order.
  On a partial run it is every state the run reached, listed or not: a cut
  at 100 lines (`-n`, or the shared stdout limit) cuts stdout, not the
  directory, and `index.json` says which files stdout listed. A line mode
  past `--region-limit` on a flagged program lists the states describe
  certifies on its line and writes every other member it met unlisted, with
  no line (M1e2 S1e2.2.26).
- **`index.json`** carries `asked` and the same `region` block as the summary,
  and per state file its `path`, `state` number, `lines`
  (`minimal` / `maximal`) and `listed`.
- **A state file's header** gives its number, the commitment it was read from,
  its lines, how many commitments reach it and its minimal generators.
- **`answer.txt`** reads back: its `(region …)` is the claim a `(solve …
  :expect …)` states, `(merged …)`s included
  ([`01_grammar.md` § Solve](../ir/03-ein-lang/01_grammar.md#solve)).
- **`owing/`** holds the frontier of an empty R whose nodes all owe, each with
  what it owes. **`nogoods.json`** is `[]` where N is empty or was not
  computed (a line mode does not compute it). Each member is `{set, core,
  core_status, refuted_by}`; `refuted_by` (M1e2 S1e2.3.10, P1e2.3's N7)
  names the rules of the last step of the core's derivation: for each fact of the contradiction the core was taken over, the first justification the store records (the primary, the alternatives, the smallest-closure arrival) whose premises the core derives, `(given)` for a fact the core holds itself. Until S1e2.3.10 it named the first
  contradiction's primaries, a rule no derivation from the core uses on
  65 559 distinct printed cores per commit (2026-11-06, held since by
  `ein-infer/tests/corpus_cores.rs`). Where the core derives a fact
  through two rules, the store's order picks one, so two programs one
  rule's written place apart (one `:priority` apart until M1f S1f.5.15) can
  name two rules beside one core, the accepted cost
  of the user's ruling (Q-M1e2.21 (a)). `core_status` (M1e2 S1e2.2.23, W23) says
  whether the core's search was cut at its budget: `within-budget`, `cut-`
  and the caps it stopped at (`cut-environments`, `cut-rounds`,
  `cut-facts`, joined). **Every member has its core, past `--region-limit`
  too** (M1f S1f.5.12, the user's ruling of 2026-11-08 on P-15, (c)): the
  run takes no core during its walk and, after it, each member's, through the
  node function against the snapshot the walk resumed every node from, so
  each is the core the walk takes eagerly where it visits the member
  (`--dump-lattice`'s `nodes.json` writes that one), and a member the state
  route reads off its maximal alive states, without visiting it, has the
  core a first preparation of root gives it. Until S1f.5.12 a member a run
  past the limit met was written with a `null` core and rules beside
  `core_status: "not-kept"` (M1e2 S1e2.3.9, `b6139e62`), and before
  S1e2.3.9 with an empty core, which reads as *refuted by nothing*:
  `lattice/06_friends_tables.ein`'s 68 of 68 (`ein solve FILE --dump-answer
  DIR`, 2026-11-06); and a member the state route did not visit was taken
  through a second preparation of root, which gave 19 of `zebra2.ein`'s 366
  another core, status or rules (2026-11-09). No member is written without
  its core, and each is the eager one, which
  `ein-cli/tests/answer_round_trip.rs` holds over every corpus program it
  dumps.
- **`forms/`** (M1e2
  [S1e2.3.3](../../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.3_the_forms_past_100_lines.md),
  the user's ruling (a) of 2026-11-03) holds each form the run was asked
  for, uncut and drawn from every listed state, the states `answer.txt`
  lists: `hyp-stats.txt` (`-H`), `key.txt` or `grid.txt` (`--models`),
  `print-final-state.txt`, `print-final-positive.txt` and
  `print-final-hfacts.txt` (`-p`, `-P`, `-f`) and `diff-models.txt`
  (`--diff-models`). stdout prints what fits of each in what the read-out
  left of its 100 lines, drawn from the states it showed, and its last line
  names the file. A file is what the form prints with nothing cut: where
  nothing is cut it is stdout's form byte for byte, and on `pets.ein` it is
  what the form printed before the stage
  (`ein-cli/tests/forms_cut.rs`). `index.json`'s **`forms`** lists each as
  `{form, path}`, in the order stdout prints them, and is `[]` where no form
  was asked for.

What the walk's directory held and this one does not: `program.ein`,
`root.ein` and `root_writes.json` (root is never written: a region run
classifies nodes, it does not commit), `refuted/` (N replaces it) and
`alive_at_end.json` (there is no walk to end). The walk's `models/NN.json`
records are the summary's `region.states[]` now.

## Every state file loads, and is a fixpoint

A state file is the program as loaded, imports inlined, followed by every fact
of the state, one a line, sorted by its text. The program's own `(relation …)`
declarations are not repeated, because the loader refuses a second one. So the
file **loads as a program**, and saturating it **derives no fact and no
`(false)`**. Each state is read again through the node function every route
uses, so the file is the state the read-out names and not a copy of one.

That is the check for the class M1e's
[`CO-M1`](../../history/m1e_review_processing/README.md#the-findings) fixed: a
state the engine reports that its own rules extend or refute. Per file, it is
`answer_dump::every_recorded_state_is_a_fixpoint_of_its_own_program`, over
every run `corpus/wrong_answers.toml` records and seven cases of its own, and
it checks one thing more: the file holds every candidate of its state's
closure and no other candidate.

**With no exception since M1e2 S1e2.3.8.** From S1e2.2.16 (2026-11-04) the
test named the state files the fact printer could not write rather than
checking them (`printed_alike`): W16's two witnesses, each holding `(noted 1
1)` and `(noted 1 "1")`, which the printer wrote as one line twice, and from
S1e2.3.8's T1e2.3.8.2 W28's `classes-int-name`, whose name `"1"` was written
as the int. The one printer writes every state, and the list is gone.

**It loads to the state** (M1e2 S1e2.3.8, P1e2.3's N5). Each fact is written
by the one printer (`ein_core::text`, `Terms::display`), which prints a value
as text the loader reads back to the same value: a name bare only where it
lexes as one `NAME` (`House-1`, `neqw`; one `SYMBOL` until M1f S1f.5.5's
swap), and quoted everywhere else, a
nested fact `(rel …)`; until M1f S1f.5.5's swap a program could write a name
that printed quoted (`"x y"`, `"not"`, `"?x"`, `"Åsa"`), and since it a string
is natural-language text and no value (S1f.5.2), so the quoted case is a name
the loader made up, `"#empty"`, which does not load back (W34's); since M1f S1f.5.5's
swap `1`, `07` and `1..5` are names that print bare, and since S1f.5.4 there is
no int, which printed bare. So loading `models/NN.ein` gives that state's facts **by value**:
`ein-cli/tests/answer_round_trip.rs` reloads every state file `ein solve
--dump-answer` writes, on every corpus program it answers (272 programs and
392 712 files per commit, 2026-11-06; the slow entries under
`EIN_CORPUS_SLOW=1`), and holds each fact's `repr` to the state the library
computes for that commitment, with no exception. Until S1e2.3.8 the printer
wrote a name as its raw text, so `(holds "x y")` was written as the
two-argument `(holds x y)`, the int `1` and the name `"1"` as one line, and a
name `#|` opened a block comment: 112 800 files over five corpus programs did
not reload to their state (`syntax/arg-kinds`' 112 793 among them).

**What a reloaded state's facts rest on: nothing; each reads as given.** A
fact's provenance is written as a **comment** after it, which the loader
skips: `; source "…"` for a fact the program states with a source, `; rule r`
for one a rule derived, `; hypothesis N` for the commitment. A state file is
the state's facts, not a second kind of program that re-states where each
came from, so a reloaded state is its facts as given, and the comment is for
a reader. Until S1e2.3.8 the provenance was written as keywords, `:source
"…"`, `:rule "r"` and `:hypothesis N`, of which the loader read the first,
took `:rule` only as an atom (the writer wrote a string) and had no third, so
a reloaded state read as given then too, while the file said otherwise (the
inventory of 2026-11-05's S-5). `answer_round_trip.rs` holds every reloaded
fact as given.

**`--dump-states`' and `--dump-lattice`'s snapshots are a lesser thing.**
They carry a state's facts and no rules. They load and reload to the same
facts (`ein-render/tests/dump_states_load.rs`), so `ein render kb` draws them,
but with no rules, saturating one derives nothing whether or not it is a
fixpoint of the program.

## The contract

- **Additive.** stdout, stderr, the exit code, `--json-summary` and the
  `--events` stream are identical with the flag and without —
  `answer_dump::the_flag_moves_no_answer`, on three questions — so the
  read-out's header leaves the flag out, as it leaves out `-j`.
- **`answer.json` is `--json-summary`**, byte for byte, and **`answer.txt` is
  stdout's read-out uncut**, on every question
  (`answer_dump::answer_json_is_the_json_summary_byte_for_byte`). The forms
  printed after the read-out are not in it: each is its own file under
  `forms/`, and the read-out and the summary are the same with any form and
  without (`forms_cut.rs`).
- **The directory is made before the run**, and a run that cannot make it
  exits 1 without solving — [`defined_behaviour.md` §
  4.4](../defined_behaviour.md#44-an-artefact-flags-failure--m1e-s1e35). The
  files are written once the answer exists, and a failure there takes the
  additive arm: `error: --dump-answer <path>: <os error>`, and the exit code
  unchanged. An empty path is exit 2.
- **One run, one directory.** A file with several `(query …)` blocks is one
  run, and each query's classes are in `answer.json`'s `queries`. Under the
  walk, which solved once per query, the flag was refused on such a file.
- **A `.einb` is refused**, exit 2. The container carries no program source,
  and every state file starts with one.
- **A budget stop is an answer**: `--region-budget` and `--region-limit`
  write the whole directory, and `index.json`'s `region` says
  `exhausted: false` and `stopped: {why, …}`. **A `-T` abort is not**: it
  exits 2, and the directory it made first stays empty
  (`answer_dump::a_stopped_run_still_says_what_it_is`).
- **A second dump into the same directory leaves nothing stale.** When
  `index.json` names this format, `models/`, `owing/` and `forms/` are
  cleared before writing. A directory holding anything else is written beside
  and nothing in it is deleted.
- **`--count` writes two files** (M1e2 T1e2.1c.4.6): `answer.txt`, its
  read-out uncut, every warning row among it, and `answer.json`, its summary,
  whose `asked.ask` is `count` and whose `region.states` and
  `region.frontier` are `null`. It reads no state, so it writes no state file,
  no form, no `index.json` and no `nogoods.json`, as `--monotone` writes none
  (every form is refused beside `--count`, exit 2); and
  where `index.json` names this format it removes what that dump left, the
  two top-level files with the subtrees
  (`region_modes.rs::the_count_question_is_describes_counts`).

## The run summary

`answer.json` is `--json-summary`'s object, `ein-summary/2`:
`schema`, `source`, `asked`, `region`, `verdict` (under `--single` only),
`queries` and `claim`, each field defined by
[`read_out.md` § 11](read_out.md#11---json-summary-the-answer-blocks) beside
the row that prints it; `work`, `monotonicity`
([`region_dump.md` § Monotonicity](region_dump.md#monotonicity--the-static-check)),
`leftover`, `root`, `config`. Every bound in `region` carries `exhausted`
beside it, so a reader never has to know which numbers were complete.

The `work` block is what a reader asks *what did the run cost* of. Each
counter is defined here, and so is every field of the `root` block below; the
review bundle quotes these rows beside the numbers rather than paraphrasing
them, and `answer_dump::every_summary_counter_is_defined_on_this_page` fails
when a counter or a field appears in a summary without a row. The walk's
twelve `stats` counters went with it at the switch.

| counter | what it counts |
|---|---|
| `saturations` | node fixpoints computed, root's included: one per node route 1 visits, one per node of the cube under route 2, one per extension the state route saturates (M1e2 S1e2.1c.2), plus one per listed state a mode re-reads. The read-out's `work` row is this number |
| `facts` | facts those saturations derived beyond root's own fixpoint, summed over nodes — what `--region-budget` is spent against |
| `layers` | route 1's layers visited, root's layer 0 included. Under the state route, its layers: root's, then one per state size it extended. `null` under route 2, which visits the cube in mask order and has no layers |

The `root` block is **root's fixpoint and how H was built, as every route
starts from them**: the KB `region::prepare_with` saturates in place and the
counts its own pass took, read and not recomputed, so a summary adds no work
and no lever moves the block. That is M1e2
[S1e2.3.1](../../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md)'s
ruling (a), the user's of 2026-11-03. Until then the block ran a pass of its
own on a fork of root (`emit_closed`'s markers, a second saturation and the
walk's generator with its lookahead and kill cache on) and counted what that
pass wrote: 27 facts where root's fixpoint holds 22 on
`examples/branching/02_one_dead_one_alive.ein`, and 30 candidates where H holds
48. `summary_properties::the_description_is_coherent_on_every_corpus_cell`
holds `root.facts` to root's fixpoint and `root.hypgen.emitted` to
`region.universe.candidates` on every corpus cell, and `--monotone`'s block to
the region run's.

| field | what it holds |
|---|---|
| `root.facts` | the facts of root's fixpoint, `sat(program)`: what `ein saturate FILE` prints as *facts (total)* after saturation, and what state ∅'s file holds when ∅ ∈ R. No route writes into root, so it is root before any commitment and after every one |
| `root.facts_by_relation` | those facts by relation name, sorted, only the relations that have one. They sum to `root.facts` |
| `root.plans` | the plans root's saturator compiled, the derived activators' included. Measured equal to `monotonicity.plans.saturation` on all 218 corpus programs that write a summary under `ein solve -n 1` (2026-11-03, S1e2.3.1's census) |
| `root.saturator.naf_rounds` | how many times root's saturation reached the NAF boundary with candidates parked on an `absent` guard, and judged them against the quiesced world |
| `root.saturator.naf_admitted` | parked candidates the boundary admitted: every guard passed, and the firing was queued |
| `root.saturator.naf_retired` | parked candidates the boundary retired as dead: an anti-monotone guard found a match, which a growing KB keeps |
| `root.saturator.naf_dropped` | structurally 0 since S1.21.8: a guard is judged once, at the boundary, so no admitted firing is dropped at dequeue. Kept so a boundary rebuilt wrong shows here |
| `root.hypgen.raw` | candidates the generator built at root's fixpoint, config-free (`hypgen::generate_config_free`: no lookahead filter, no kill cache, the obligations rung in report order), before any filter. It is `emitted` plus the sum of `filtered` |
| `root.hypgen.emitted` | the candidates that survived every filter: **H**, the universe every route reads, so it equals `region.universe.candidates` and is the universe `ein solve --monotone` prints, whose `region` is `null`. A repeat is dropped as `seen_in_call` before it counts |
| `root.hypgen.filtered` | why a built candidate is not in H, sparse, only the keys that fired: `fact_already_exists` (root holds it), `negated_fact` (root holds its `(not …)`), `seen_in_call` (proposed again, from another focal object). `lookahead_killed` is the walk's filter, which this pass does not run, so it never appears |
| `root.hypgen.pre_candidate` | relation and slot skips before any candidate exists, sparse: `closed_relation` (a `(__closed__ R)` root holds, which the program states or derives through `std.closure`: no pass writes one), `no_hypothesis_relation` and `relation_not_whitelisted` (the query's `:no-hypothesis` and `:hypothesis-relations`), `self_edge` (both slots the same object) |
| `root.hypgen.outside` | what the universe leaves out (M1e2 S1e2.2.8, the user's ruling of 2026-11-03 on Q-M1e2.3, (a) *scoped*): one `{relation, arity, why}` per relation the program can conclude or ask about, does not scope out itself, and of which the rung that built H proposes no fact, sorted by name, `[]` when none. `why` is `not-owed` (the obligations rung: no obligation owes it), `no-hrule` (the hrules: none asserts it) or `arity` (the blind enumerator, which proposes arity 1 and 2 only). The read-out's `outside` row says the same in words ([`solution_semantics.md` § 1.4](solution_semantics.md#14-candidate-and-universe)) |
| `root.hypgen.types_as_objects.objects` | the names the untyped rung bound as objects (M1e2 S1e2.2.10, the user's ruling of 2026-11-03 on Q-M1e.12, (a) *the fixture, with the warning*): where the blind enumerator built H, each name that is the second argument of an `(is-a x y)` fact at root's fixpoint and an argument of a candidate, sorted, `[]` when none. The read-out's `types` row under `universe` says the same in words ([`solution_semantics.md` § 1.4](solution_semantics.md#14-candidate-and-universe)) |
| `root.hypgen.types_as_objects.relations` | the relations of the candidates that name one, sorted, `[]` when none |
| `root.hypgen.types_as_objects.candidates` | how many candidates of H name one, `0` when none |

## What it costs

Little beyond the run: each state file is one more saturation of a state the
run already classified. On `examples/zebra2.ein` (bare describe, partial at
the default budget, 1 state) the run took 3.61 s with the flag and without, and
wrote 144 KB; on `zebra2-minus-15.ein`, 28 state files and 1.5 MB, 4.67 s
against 4.68 s (release build, 2026-10-18, `ein solve FILE [--dump-answer
DIR]`). A state file is mostly the program, so the directory's size is about
*k* × the inlined program.

## Where it is read

[M1e2 P1e2.1](../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/README.md)'s
review bundle reads it: the KB view of a state from `models/NN.ein`, and the
read-out from `answer.txt`. `ein render region CUBE --answer DIR` reads
`index.json`, and draws each state on the oracle's cube against the part of R
the run asked
([§ Region rendering](../ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run),
S1e2.1a.7). The writer is
[`ein_render::dump::region_answer`](../../../ein.rs/crates/ein-render/src/dump/region_answer.rs):
a function over a finished `ReadOut`, so it cannot perturb the run it reports.
