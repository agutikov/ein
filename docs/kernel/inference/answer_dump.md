# The answer dump — `ein solve --dump-answer DIR`

**What a solve answered, as files.** One directory per run: the read-out
uncut, the summary, one **loadable** program per state the run reached, the
owing frontier of an empty R, the minimal dead sets, and an index naming each.
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
  nogoods.json    N: each minimal dead set, its core and the rules that refuted it
```

- **`NN` is stdout's numbering.** `models/02.ein` is *state 2 of k* in the
  `(region …)` block, two or more digits, so the files list in that order.
  On a partial run it is every state the run reached, listed or not: a cut
  at 100 lines (`-n`, or the shared stdout limit) cuts stdout, not the
  directory, and `index.json` says which files stdout listed.
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
  computed (a line mode does not compute it).

What the walk's directory held and this one does not: `program.ein`,
`root.ein` and `root_writes.json` (root is never written: a region run
classifies nodes, it does not commit), `refuted/` (N replaces it) and
`alive_at_end.json` (there is no walk to end). The walk's `models/NN.json`
records are the summary's `region.states[]` now.

## Every state file loads, and is a fixpoint

A state file is the program as loaded, imports inlined, followed by every fact
of the state, sorted, each with the provenance keyword it carries (`:rule`,
`:source`, `:hypothesis`). The program's own `(relation …)` declarations are
not repeated, because the loader refuses a second one. So the file **loads as
a program**, and saturating it **derives no fact and no `(false)`**. Each state
is read again through the node function every route uses, so the file is the
state the read-out names and not a copy of one.

That is the check for the class M1e's
[`CO-M1`](../../history/m1e_review_processing/README.md#the-findings) fixed: a
state the engine reports that its own rules extend or refute. Per file, it is
`answer_dump::every_recorded_state_is_a_fixpoint_of_its_own_program`, over
every run `corpus/wrong_answers.toml` records and seven cases of its own, and
it checks one thing more: the file holds every candidate of its state's
closure and no other candidate.

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
  stdout**, on every question
  (`answer_dump::answer_json_is_the_json_summary_byte_for_byte`).
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
  `index.json` names this format, `models/` and `owing/` are cleared before
  writing. A directory holding anything else is written beside and nothing in
  it is deleted.
- **`--count` writes two files** (M1e2 T1e2.1c.4.6): `answer.txt`, its
  read-out uncut, every warning row among it, and `answer.json`, its summary,
  whose `asked.ask` is `count` and whose `region.states` and
  `region.frontier` are `null`. It reads no state, so it writes no state file,
  no `index.json` and no `nogoods.json`, as `--monotone` writes none; and
  where `index.json` names this format it removes what that dump left, the
  two top-level files with the subtrees
  (`region_modes.rs::the_count_question_is_describes_counts`).

## The run summary

`answer.json` is `--json-summary`'s object, `ein-summary/2`
([`read_out.md` § 8.1](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md)):
`asked`, `region`, `verdict` (under `--single` only), `queries`, `claim`,
`work`, `monotonicity`
([`region_dump.md` § Monotonicity](region_dump.md#monotonicity--the-static-check)),
`leftover`, `root`, `config`. Every bound in `region` carries `exhausted`
beside it, so a reader never has to know which numbers were complete.

The `work` block is what a reader asks *what did the run cost* of. Each
counter is defined here; the review bundle quotes these rows beside the
numbers rather than paraphrasing them, and
`answer_dump::every_summary_counter_is_defined_on_this_page` fails when a
counter appears in a summary without a row. The walk's twelve `stats`
counters went with it at the switch.

| counter | what it counts |
|---|---|
| `saturations` | node fixpoints computed, root's included: one per node route 1 visits, one per node of the cube under route 2, one per extension the state route saturates (M1e2 S1e2.1c.2), plus one per listed state a mode re-reads. The read-out's `work` row is this number |
| `facts` | facts those saturations derived beyond root's own fixpoint, summed over nodes — what `--region-budget` is spent against |
| `layers` | route 1's layers visited, root's layer 0 included. Under the state route, its layers: root's, then one per state size it extended. `null` under route 2, which visits the cube in mask order and has no layers |

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
