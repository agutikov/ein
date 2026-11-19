# S1f.21.6 — the surfaces

**Phase:** [P1f.21](README.md)
**Status:** **done 2026-11-19**, on the user's word to start it (*"start s1f.21.6 and s1f.21.8"*); every reading's
default taken; the batch's gate and `utils/ci_container.sh` are the coordinator's to run ([the record](#record)).
Planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*).
**Group:** [A](modifications.md#a-surfaces-no-answer-and-no-word-of-the-language-moves), items 1 to 5.
**Waits on:** nothing. It is the first stage to run.
**Writes:** the printers and their strings under `ein.rs/`; the pages that describe them,
[`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md),
[`09-read-out-examples.md`](../../../docs/ein/user/04-reference/09-read-out-examples.md) (written by the binary) and
[`10-answer-dump.md`](../../../docs/ein/user/04-reference/10-answer-dump.md); the goldens that hold a line it
moves; for item 5, this folder's own pages.

## What it does

No answer and no word of the language moves: each task changes what a surface prints, or a number in the plan.

1. **`-p` prints a fact of no argument as it is written** ([V-3](findings.md#v-3---p-prints-a-fact-of-no-argument-with-a-space-inside)).
   `ein solve -p` and `-P` print `(alive )` where the same run's `stale` row, `ein saturate --dump` and `ein oracle
   --dump --states` print `(alive)`. One printer for a fact, and a test that the surfaces agree on a fact of no
   argument.
2. **The saturated dump lists every kind of rule** ([V-4](findings.md#v-4--the-saturated-dump-lists-no-obligation-rule)).
   `;; Rules (N)` is the agenda's rules; an obligation rule and a hypothesis rule are in no section. **Reading 1**,
   default: a section each, `;; Obligation rules (N)` and `;; Hypothesis rules (N)`, in the dump's form for a rule,
   and the first heading says it is the agenda's. The other reading: the heading's words alone.
3. **A dead state by a pair is not said to derive `(false)`** ([V-2](findings.md#v-2--the-surfaces-say-derives-false-of-a-state-that-holds-none)).
   Four places: the read-out's `empty` row, `10-answer-dump.md`'s `root.dead` row, the legend of `ein render
   region`, the comment on `DEAD` in `ein-infer/src/region.rs`. **Reading 2**, default: *holds a contradiction*,
   true of a held `(false)` and of a pair today, and of a held `(ein.kill)` after
   [S1f.21.11](s1f.21.11_kill_and_three_conventions.md). The other reading: left until that stage makes the words
   literally true.
4. **An owing node below a non-empty $R$ has no saved file** ([item 32 § 7](review/part-2/32-Q-M1f.390/README.md#7-found-while-building)).
   Shown on a program and put. **Reading 3**, default: not built, and entered in [`findings.md`](findings.md) as a
   finding of its own; the other reading: `--dump-answer` writes the owing frontier whether $R$ is empty or not.
5. **The plan's own numbers**, no project file: the five corrections
   [`modifications.md`](modifications.md#a-surfaces-no-answer-and-no-word-of-the-language-moves) lists under its
   item 5, each in the page it is of, with the command that re-takes it.

## What must hold

- **No answer moves**: no field of `region` in any corpus program's summary, by the corpus sweep through the binary.
- A change to what `ein solve` prints changes `08-read-out.md` in the same commit, and the examples page is written
  again by the binary (`ein-cli/tests/read_out_reference.rs`).
- A golden that holds a moved line is re-banked and its diff read: only the lines of tasks 1 to 3.
- `ein render region`'s legend is a DOT writer's: `utils/ci_container.sh` is part of done.

## Acceptance

Each of V-2, V-3 and V-4 no longer shows on the run its finding gives, and the finding says so; the three readings
are recorded with what was taken; one gate on the stage's last commit.

## Record

Run 2026-11-19 by one agent, in the main tree, on `master`; its commits stand on `c569ee08`. `ein` is the gate
build, `ein.rs/target/gate/ein`; *the binary before* is that build of the code before the stage's first commit, kept
aside for the comparisons.

### The readings

| reading | of | taken |
|---|---|---|
| **1** | task 2, the dump's rules | **the default**: a section for each kind, `;; Obligation rules (N)` and `;; Hypothesis rules (N)`, in the dump's form for a rule, and the first heading says it is the agenda's, `;; Rules — the saturation agenda's (N)` |
| **2** | task 3, the words for a dead state | **the default**: *holds a contradiction* |
| **3** | task 4, a saved file for an owing node below a non-empty $R$ | **the default**: not built; entered as [V-9](findings.md#v-9--an-owing-node-below-a-non-empty-r-has-no-saved-file) |

### The commits

| commit | task | what is now true |
|---|---|---|
| `d35408cb` | 1 | `ein solve -p`, `-P` and `-f` print each fact as the one printer does, so a fact of no argument is `(alive)`. A second printer had the same space, `Terms::compact`, behind the loader's refusal of a written derivation cycle (`(p ) -> (q ) -> (p )`): it is gone, and the message prints `Terms::fact_text`. `ein-cli/tests/fact_of_no_argument.rs`, three tests, holds the read-out and its forms, the saturated dump, `show hyps`, the oracle and its dump, the events, the trace, the answer dump, `ein test --diff full`, the `stale` row and the cycle's refusal to one text |
| `8d8327ce` | 2 | `ein saturate --dump` has a section for each kind of rule the program loads. Beside an agenda's rule its stratum, as before; beside an obligation rule and a hypothesis rule with parameters, how many activators bind them (`no activator` where nothing activates it), and nothing beside one without parameters. The count for a hypothesis rule is `ein_infer::hrule::activator_counts`, which shares the `:hrules` reader and its skip with `Hrules::new`. `ein-cli/tests/saturate_dump_rules.rs`, four tests |
| `c9ea9597` | 3 | the read-out's `empty` row and its `∅` row of a dead root (describe and `--no-search`), the legend of `ein render region`, the sentence of `ein render h-space` and `h-space.json`'s `says`, `10-answer-dump.md`'s `root.dead` and the comment on `DEAD` say *holds a contradiction*; so do the pages that quote the rows and the eight region drawings that carry the legend |
| `d1c9d4b3` | 5 | the plan's five corrections, in [`open_facts.md`](open_facts.md) and [`design.md`](design.md) |
| this one | 4, the record | V-9 entered; V-2, V-3 and V-4 marked fixed in [`findings.md`](findings.md); this record |

### Task 3: every place that said it

**Re-worded**, the four the stage names and those that are the same sentence:

- printed: the `empty` row; the `∅` row of a dead root, under describe and under `--no-search`; the region's
  legend; the h-space's sentence (`ein render h-space`, `h-space.json`'s `says`);
- comments: `DEAD` in `ein-infer/src/region.rs`, and *Root's own fixpoint derives `(false)`* on the fields and the
  variant that hold a dead root or node, in `region.rs` (four), `explore.rs` (two), `described.rs`, `structure.rs`
  (two) and `structure/coupling.rs`;
- pages: `08-read-out.md` (§ 2's `empty` row, § 6's paragraph); `09-read-out-examples.md`, by its generator (four
  rows; the generator's sentence over X16); `10-answer-dump.md`'s `root.dead`;
  `08-reasoning-in-code/03-hypotheses-and-the-answer.md`'s `root_dead` key; the rows quoted in
  `02-guide/06-zebra-walkthrough.md` (three) and `08-word-problems.md` (two), in `04-stdlib/02-relation-algebra.md`,
  `04-bijections.md`, `05-slots.md` and `07-calculator.md` (two), and in `stdlib/README.md`;
- definitions of *dead* that had the one clause: `13-solutions-and-models.md` (its first paragraph and its
  table), `13-glossary.md`'s *State*, and the premise *dead is upward-closed* in `03-design/README.md`;
- `examples/features/09_adjacent_via_same_house.ein`'s comment: its root is dead by a pair and holds no `(false)`
  at quiescence (`ein saturate --dump`);
- the eight drawings under `docs/ein/reasoning/02-kb-evolution/diagrams/` that carry the legend: seven sources
  written again by the commands their pages give
  (`08-reasoning-in-code/02-saturation-and-negation.md` and `03-hypotheses-and-the-answer.md`), each differing from
  what was there in the legend's line alone; `08-region-three-components.dot`, which no command rebuilds, edited at
  that line; the eight pictures drawn again (`utils/render_diagrams.py`).

**Left as they are**, each read:

- true as written, a rule of the program concluding `(false)`: the comments of the calculator's programs under
  `tests/stdlib/math/` and `tests/README.md`'s paragraph on them, `examples/zebra2-hints.ein` and
  `examples/ein-bugs/zebra2-hints.ein` (root holds `(false)` at quiescence, by `std.algebra.functional`),
  `04-stdlib/02-relation-algebra.md`'s *a state that violates the property derives `(false)`* (every check there
  concludes it), the notes of `corpus/corpus.toml`, the messages of `arith_search_free.rs` and `num9_theory.rs`;
- an implication, true and one-sided: *a state that holds `(false)` is dead* in `08-obligations.md` and
  `03-reserved-names.md`; *`(false)` is stored, since … a dead state stays dead* in `02-syntax.md`; `monotone.rs`'
  *`dead` is upward-closed, `(false)` being a fact* and `obligations.rs`' twin;
- another sentence of the same kind, **not changed, for the pages' stage**: `06-zebra-walkthrough.md`'s *a
  candidate is dead alone when assuming it, and nothing else, saturates to `(false)`*;
- the plan's own drawing, `diagrams/region-owed-seats.dot` and its picture, which carry the old legend.

### Goldens

- `ein-render/tests/golden/trace_dead.md`: the `empty` row, one line.
- `ein-render/tests/golden/corpus_shapes.md5`, re-banked with `EIN_BLESS=1 EIN_CORPUS_SLOW=1` on
  `id_order_invariance`: 294 lines, the `read-out[describe]` and the `region-trace` line of each of the 147 programs
  whose root is dead; every line keeps its count of lines. `ein solve` of each of the 147, the binary before
  against the binary after, differs in the re-worded row and nowhere else.
- `docs/ein/user/04-reference/09-read-out-examples.md`, written by `utils/read_out_examples.py`: four rows and the
  generator's one paragraph.
- No golden moves by task 1 or task 2.

### What held

- **No answer moves.** `ein solve FILE --json-summary` over the 899 `.ein` files of `examples/`, `tests/` and
  `stdlib/`, the binary before against the binary after: the same `region`, `queries` and `root.facts` in each of
  the 655 summaries, and the same exit code on the 244 the loader refuses.
- **Task 1 by invariance**: `ein solve FILE -p` over the same 899, before against after, byte for byte the same,
  the two binaries pointed at one standard library (`EIN_STDLIB`); on the new test's program the binary before
  prints `(lit )` and `(on )`.

### Checks

Each on the stage's final tree, the gate profile where a test:

| check | result |
|---|---|
| `cargo test --profile gate --workspace`, **once** | it stopped at `ein-cli`'s last binary: the 62 before it ok, and `world_anchors` **failed**. The new dump test named `examples/zebra2.ein`, and `examples/README.md`'s anchor list names every test that does. The test was re-written over a program of its own, so the list stands as it was, and the run was finished by the packages it had not reached (`--no-fail-fast -p ein-core -p ein-corpus -p ein-einb -p ein-infer -p ein-ir -p ein-narration -p ein-render`): 100 binaries ok, none failed. Then `world_anchors`, `saturate_dump_rules` and `ein-cli`'s doc tests: ok |
| `EIN_BLESS=1 EIN_CORPUS_SLOW=1` … `--test id_order_invariance` | ok, the shape golden re-banked as above |
| `cargo fmt --all --check` | ok |
| `cargo clippy --workspace --all-targets -- -D warnings` | ok |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | ok |
| `utils/doc_audit.py --links --check --tracked` | 0 findings in links and in decision ids |
| `utils/check_release_voice.py` | ok |
| `utils/check_doc_blocks.py` | ok, rules 1 to 5 |
| `utils/render_diagrams.py --check` | ok, after the eight pictures were drawn again |
| `utils/check_generated.py --bin ein.rs/target/gate/ein` | ok, 9 generators |

**Not run**: `./run_tests.sh`, `./run_tests.sh --nightly`, `utils/ci_container.sh`; the calculator's batch, no rule
of it having changed.

### Not done

- **The gate and `utils/ci_container.sh`**: not run by this stage's agent, on the coordinator's word, who runs one
  gate for the batch. The stage's acceptance, *one gate on the stage's last commit*, and its *`utils/ci_container.sh`
  is part of done* (task 3 changes a DOT writer's legend) wait on that run.
- **Task 4 is not built**, by Reading 3's default.
- The plan pages this stage does not own are as they were: the phase's README and overview, the list of
  modifications, the ledger. What should change in them is in the agent's report.
