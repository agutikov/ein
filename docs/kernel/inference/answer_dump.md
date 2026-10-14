# The answer dump — `ein solve --dump-answer DIR`

**What a solve answered, as files.** One directory per run: the program, root,
one **loadable** file per recorded model or open state with the commitment
that reached it, every refuted commitment, and the run summary. M1e2
[S1e2.1.2](../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.2_every_answer_a_file.md)
added it, because a person reviewing a wrong answer needs to see the answer,
and before this flag it existed only as partial copies:

| | what it holds | what it lacks |
|---|---|---|
| stdout | the verdict, and with `-p`/`-P` each model's facts | not loadable, and gone with the terminal |
| `--json-summary` | every model's facts and goal rows, the core, the counters | *how*: which commitment reached a model, at which layer, what each refuted commitment's core and learned clause were |
| `--dump-states` | root before and after each layer, and the timeline | the answer: it is the search's **frontier**, root per layer |

The engine held the rest in the `store_lattice` proof — the one `--trace`
renders — and nothing wrote it out.

```sh
ein solve examples/lattice/02_genuine_3set_death.ein -e --dump-answer out/
```

## The directory

```text
out/
  index.json          what is here — the verdict, k, exhausted, and every state file by name
  program.ein         the source as loaded, imports inlined
  root.ein            program + root's facts after its saturation, before any hypothesis
  root_writes.json    what the search wrote into root after that, and by what
  answer.json         byte-identical to --json-summary on the same run
  models/NN.ein       program + one recorded model's facts
  models/NN.json      {commitment, layer, reported, goal_bindings, owes, added, facts}
  open/NN.ein, .json  the same, per recorded state that still owes
  refuted/NN.json     {commitment, layer, kind, unsat_core, learned_clause}
  alive_at_end.json   the frontier the depth cap left — [] when the search exhausted
```

- **`NN` is stdout's numbering.** `models/02.ein` is *model 2/3* in the solve
  table and *solution 2/3* under `-P`. It is the verdict's branch order, and
  two or more digits, so the files list in that order.
- **`program.ein`** is the three calls
  [`stdlib/README.md`](../../../stdlib/README.md) documents for inlining an
  import (`parse` → `resolve_and_minimize` → `dump_canonical`).
  `imports_semantics::the_inlining_route_the_stdlib_readme_documents_round_trips`
  holds that the result is the same program. Comments do not survive it.
- **`root.ein`** is root at the moment its saturation reached a fixpoint —
  [`LatticeProof::root_initial`](../../../ein.rs/crates/ein-infer/src/solve.rs),
  the state the `root_initial` dumper hook is handed. It is *before* the
  search's own root writes, which is why those are a file of their own.
- **`root_writes.json`** lists every fact the final root holds and the
  saturated one did not, each with the rule its provenance names. Those are
  the kill cache's `<lookahead-dies-immediately>`, the singleton writeback,
  and a `<forced-positive>` promotion. They are the writes the M1e2 cases
  W1, W2 and W12 turn on, so they are not folded into a model's `added`
  silently.
- **`models/NN.json`**:
  - `commitment` is the facts the commitment asserts, in
    `00_timeline.jsonl`'s `{relation, args}` spelling, so a timeline row and
    this record compare as equal values;
  - `added` is the state's facts minus root's (positive and negative): *what
    this commitment made true that root did not*, plus whatever the search
    wrote into root after it started;
  - `facts` is the whole state, in `--json-summary`'s sorted s-expression
    spelling — the same strings as `verdict.solutions[].facts`;
  - `reported` is whether the read-out shows this state (below).
- **`open/`** holds the recorded states that still owe, in a program that
  states an obligation. When the search found both kinds, the answer reports
  only the models. The owing states are still written, with
  `"reported": false`, because a reviewer asking what the search found wants
  them and the read-out does not show them.
- **`refuted/NN.json`** has one file per refuted commitment, in the order the
  search refuted them. `kind` is `dead-pre` or `dead-post`, and
  `learned_clause` is the no-good the death emitted. Under
  `EIN_TRAVERSAL=tree` the proof has no layers and records only the dead
  branches it narrates, so this directory may be empty there. That is what the
  proof has, not a missing write
  ([`configuration.md` § What `EIN_TRAVERSAL=tree` reports](../configuration.md)).

## Every state file loads, and is a fixpoint

A state file is the program followed by every fact of the state, sorted,
each with the provenance keyword it carries (`:rule`, `:source`,
`:hypothesis`). The program's own `(relation …)` declarations are not
repeated, because the loader refuses a second one. So the file **loads as a
program**, and saturating it **derives no fact and no `(false)`**. It holds
exactly the recorded state and is a fixpoint of the rules that produced it.

That is the check for the class M1e's
[`CO-M1`](../../history/m1e_review_processing/README.md#the-findings) fixed: a
state the engine recorded that its own rules extend or refute. Per file, it is
`answer_dump::every_recorded_state_is_a_fixpoint_of_its_own_program`. That
test runs over every run `corpus/wrong_answers.toml` records and over `zebra2
-e`, plus `zebra2-minus-15 -e -m 3`'s 32 models under `EIN_CORPUS_SLOW=1`.
One exception is the definition, not a waiver: a `root.ein` whose root was
contradictory before any hypothesis holds `(false)` and saturates to it again
(`alt-cap-core.ein`).

**The stronger claim was measured and holds** (2026-10-14, the same runs plus
`zebra2 -e`, release build). Re-solving a model file with the run's own flags
answers `Solution`, `k = 1`, with exactly the file's state as the model, for
**63 of 63** model files. Re-solving `root.ein` gives the original run's
verdict and `k` for **42 of 42**. It is a measurement and not a test, because
the generator proposes again on a loaded state.
`plans/m1e2_correctness/probes/answer_resolve.py` takes it again.

**`--dump-states`' snapshots are not this.** Its `00_root_initial.ein` and
`layers/layer_NN_{pre,post}.ein` write both of a relation's schema facts
(`(relation R T…)` and `(relation R)`) and no rules, and the loader refuses
them: *duplicate relation*. Found at S1e2.1.1. They are snapshots to read,
not programs to load.

## The contract

- **Additive.** stdout, stderr, the exit code, `--json-summary` and the
  `--events` stream are identical with the flag and without —
  `answer_dump::the_flag_moves_no_answer`, on three programs, one of them
  under `EIN_TRAVERSAL=tree`. The flag sets `store_lattice`, which is the
  search every corpus `--trace` cell already runs.
- **`answer.json` is `--json-summary`**, byte for byte, on every arm, an
  aborted run included. One summary is built and written to both
  (`answer_dump::answer_json_is_the_json_summary_byte_for_byte`). With
  `--dump-answer` alone, the summary is built with the event stream off, so
  the stream does not narrate a summary nobody asked for.
- **The directory is made before the search**, and a run that cannot make it
  exits 1 without solving. That is `--dump-states`' arm of
  [`defined_behaviour.md` § 4.4](../defined_behaviour.md#44-an-artefact-flags-failure--m1e-s1e35),
  for its reason. The files are written once the answer exists, and a failure
  there takes the additive arm: `error: --dump-answer <path>: <os error>`, and
  the exit code unchanged. An empty path is exit 2.
- **One path, one question.** Like `--events`, `--trace`, `--json-summary` and
  `--dump-states`, it is refused on a file with several `(query …)` blocks.
- **A `.einb` is refused**, exit 2. The container carries no program source,
  and every state file starts with one.
- **A budget cut** (`-E`, `-T`) writes `answer.json`, `program.ein` and an
  `index.json` whose `verdict` is `Aborted` with the reason. There is no proof,
  so there are no states.
- **A second dump into the same directory leaves nothing stale.** When
  `index.json` names this format, `models/`, `open/` and `refuted/` are
  cleared before writing. A directory holding anything else is written beside
  and nothing in it is deleted.

## What it costs

Nothing on the search beyond `store_lattice`. On `zebra2-minus-15 -e -m 3`
(25.5 s, 32 models, release, 2026-10-14) the dump added ~0.2 s and
**56 MB**, nearly all of it `refuted/`'s 11 577 files. Its `models/` and
`root.ein` are 35 KB apiece, because each carries the program. A corpus-wide
sweep that wants only the models can delete `refuted/` after the fact. The
flag writes everything, because the reviewer it exists for asked for every
artefact.

## Where it is read

[M1e2 P1e2.1](../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/README.md)'s
review bundle reads it. The grid and the model-set diff are drawn from
`models/NN.json`, and the KB view of a model from `models/NN.ein`. The
writer is
[`ein_render::dump::answer`](../../../ein.rs/crates/ein-render/src/dump/answer.rs):
a function over a finished `Solved`, not a `Dumper`, so it cannot perturb
the search it reports.
