# S1f.23.2 — `--models key` and `--models grid` removed

**Phase:** [P1f.23](README.md)
**Status:** run to its end 2026-11-18 on the branch `s1f.23.2-models-forms` (`377e3761`), in the worktree
`build/wt-s1f232`, beside S1f.23.1, which edits the same CLI files on master; it waits for the main session's merge
after S1f.23.1's code, and one gate, `utils/ci_container.sh` and one nightly hold the two on the merged tree.

## The ruling

The user, 2026-11-18, after asking what `--models key|grid|list` and `--diff-models` are and why they are neither Ein
nor DOT (*"--models grid is a "table a person draws for a puzzle" - what if the program is not a 2D puzzle? --models
key - I just don't understand what is it. Do we need it?"*), and the main session's answer (neither is read by an
answer, a claim or a summary; R already lists each state with its commitment, in Ein, and `ein render region` draws
the set; both rest on names, `functional`, `injective`, `bijective`, `is-a`, `T` and a declaration's labels read as
types; both serve one genre, the zebra puzzle's author), verbatim:

> remove key and grid, add a stage for it and look at --diff-models

So: **`--models key` and `--models grid` go**, with everything that exists only for them. `--diff-models` is not
part of this stage (the main session's look at it is in the conversation and waits for the user's word).

## What changes

1. **The CLI**: `--models`' values `key` and `grid` on every command that takes them (`ein solve`, and the second
   parser that has `--models`, `cmdline.rs`); the forms' order in `modes.rs`, `oracle.rs`' forms, the `--count`
   refusal's words; `--dump-answer`'s `forms/key.txt` and `forms/grid.txt`.
2. **The code that exists only for them**: `ein-render/src/grid.rs` whole; of `ein-render/src/models.rs`, the key
   (the decision variables, the pair table, its budgets, `KeyOutcome`, `PAIR_TABLE_BOUND`), keeping what
   `--diff-models` uses (`Expected`, `parse_expected`, the diff); `ModelsForm`.
3. **The tests**: `key_declines.rs`, `models_grid.rs` and `model_set_report.rs` where they test only these; the forms
   in `forms_cut.rs`, `closed_stdout.rs`, `region_modes.rs`; `help_surface.rs` and the help golden; any golden or
   corpus run that asks for either (`git grep -- '--models'`).
4. **The process**: `utils/review/run.py` and `case.py` run both forms for the wrong-answer review bundle (its smoke
   is a gate step); `utils/read_out_examples.py` and the examples page it writes; `utils/README.md`; any other script
   (`git grep -- 'models key\|models grid\|"key"\|"grid"' -- utils`).
5. **The pages**: `08-read-out.md` § 8 (the table's two rows and the words), `05-cli.md`, `06-config.md` (its
   generated block regenerated), `07-diagnostics.md`, `10-answer-dump.md`, `09-read-out-examples.md` (regenerated),
   the guide's `06-zebra-walkthrough.md`, `ein.rs/08-reasoning-in-code/` pages, `ein.rs`'s architecture page where
   it names the modules. The reasoning page `02-kb-evolution/10-satisfiability.md` § 6 defines the *determining
   key*: kept where it is mathematics a page uses, else taken out with the term.
6. **The decisions** that state either form (Q-M1d.5, Q-M1e2.34, and every one `git grep -l 'models key\|models
   grid\|determining key\|--models' -- docs/decisions` finds): rewritten to the answer that stands, a slug with its
   title, every citation moved.

## Readings this stage takes (defaults, written in its Record; the main session files them as questions on merge)

- **`--models` itself**: with `key` and `grid` gone, its one value `list` *"adds nothing"*; the flag goes whole,
  where it is a flag of nothing. If a reader still needs it (a script, a config row), say so and keep `list`.
- **Q-M1f.412's finding that `--models grid` reads `is-a`**, and P1f.23's finding on the grid's names, end with the
  code.

## Acceptance

- No answer moves: the shape golden's lines that move are the forms' alone, if any; every corpus run otherwise as
  before.
- The crate tests it touches, `cargo test --profile gate -p ein-render -p ein-cli` in the worktree, the review
  bundle's smoke (`python3 utils/review_bundle.py` as the gate runs it), `utils/doc_audit.py --links --check
  --tracked`, `utils/check_doc_blocks.py`, `utils/check_generated.py`, `utils/check_release_voice.py`,
  `utils/check_gate_steps.py`, all green in the worktree. The gate, `utils/ci_container.sh` and the nightly run once,
  on the merged tree, by the main session.

## Record

### What went

- **The flag, whole.** `--models` is gone from both parsers that had it (`ein solve`, `ein oracle`), with its help
  (`models_help`), its id in `--monotone`'s conflicts, its `--count` refusal (`COUNT_REFUSES` 12 → 11), `Shown`'s
  `form`, the key's and the grid's arms of `modes::extras` and the two locals only they read, and its name in
  `--dump-answer`'s help. `ein solve` takes 35 options and `ein oracle` 10; the help golden lost the two `--models`
  lines and `--dump-answer`'s help line changed (`help_shape.txt`, re-banked by `EIN_BLESS=1`); `help_surface.rs`'
  counts and its total, 73 → 71.
- **`ein-render/src/grid.rs`, whole**, and its `pub mod`.
- **Of `ein-render/src/models.rs`, the key**: `ModelsForm`, `KEY_NODE_BUDGET`, `KEY_TABLE_BUDGET`,
  `key_pair_table_bytes`, `key_over_bound`, the decision variables (`Var`, `Vars`, `variables`, `single_valued`, the
  labels), the minimum determining set (`Mask`, `separating`, `Budget`, `min_key_size`, `hardest_first`, `hits`,
  `Keys`, `all_keys`, `walk_keys`), the form (`KeyOutcome`, `key_table`, `KeyOf`, `KeyParts`, `key_parts_of`,
  `key_table_of`, `key_build`, `pad_row`, `binomial`) and `key_tests`; the crate root's
  `pub use models::{KeyOutcome, ModelsForm, key_table}`. 1 519 lines → 601.
- **The tests that tested only these**: `model_set_report.rs`' four key tests (its two guarantee-rule tests stay);
  `models_grid.rs`' grid tests and `list_stays_the_default`; `key_declines.rs`' price and key tests; the key and grid
  rows of `forms_cut.rs` (`FORMS` 6 → 4; the stale-form check now dumps `-f`), `closed_stdout.rs` (four cells) and
  `region_modes.rs` (the two key checks and the grid check of `the_listed_states_print_as_asked`, the oracle's key
  check, now made on its `-P`, and the two `--count` refusal rows).
- **`ein_corpus::ZEBRA_GRID`** existed in `ein-corpus` only so the grid's renderer and the solver could share it; it
  is `acceptance.rs`' own `GRID` now, its one reader, and Q-M1e.25 is held as before.
- **The process**: `utils/review/run.py`'s `grid` and `key` invocations, `FORM_DIRS`' two entries, `grids_from`,
  the answer page's key section and per-state grid, the two summaries cross-check 1 compared, and the words;
  `case.py`'s grid in § 4; `utils/read_out_examples.py`'s X18 prose (`-f` for the grid), the examples page
  regenerated (two prose lines moved, no block); `utils/README.md`'s bundle row; `AGENTS.md`'s command line.
- **The pages**: `08-read-out.md` § 4.2 and § 8 (the two rows), `05-cli.md` (the oracle's synopsis, § 4.3's key row,
  § 5's sentence), `06-config.md` (the counts 92 → 90, 36 → 35, 11 → 10; § 3.3's presentation list and its forms
  passage, now five forms and the diff's bound alone; § 5's knob is *the presentation options*),
  `10-answer-dump.md`, the guide's `06-zebra-walkthrough.md` (the two grids are `-f`'s output, the minus-15 bullet
  says `--maximal` lists the states), `01-architecture.md`'s `ein-render` row and its module diagram (redrawn),
  `08-reasoning-in-code/01-static-ir.md` (*the two orders*, which existed for the forms), `02-saturation-and-negation.md`
  § 6.1–6.4 (the key's row, sentence, tests and command; § 6.4 shows `--maximal -n 5` instead), `examples/README.md`'s
  generated anchor list (`world_anchors.rs`, re-banked). `07-diagnostics.md` named neither form: unchanged.
- **The reasoning page** `10-satisfiability.md` § 6: no other page uses the determining key, so it went with the term,
  and with it *what models differ in* (the variables, which only the key read), the drawing `diagrams/10-key.dot`/`.svg`,
  the intro's *compact exact form*, and `key` from the notation row of `reasoning/README.md`. The model and the count's
  guarantee stay.
- **The decisions**: Q-M1d.5 is rewritten to the answer that stands, *the read-out lists the states, each count
  qualified*; its slug is the same (the new title cuts to it), so no citation moves. Q-M1e2.34, Q-M1f.255 and
  Q-M1f.258 lose the flag from their lists. `git grep -l 'models key\|models grid\|determining key\|--models' --
  docs/decisions` finds nothing.

### What stayed, and why

`--diff-models` and `models.rs` as its module, under its name: `PAIR_TABLE_BOUND` (the near pairing is priced against
it; its doc now speaks of the diff alone), `mib` and `group` (the decline's numbers), `wrap` (now private: `diff_parts`
wraps the decline), `Near`, `SetDiff`, `diff_model_sets`, `Expected`, `ExpectedError`, `parse_expected`,
`model_facts`, `first_loc`, `comparable_facts` (a state's model, which `03-hypotheses-and-the-answer.md` maps
$\operatorname{model}(C)$ to), `diff_lines`, `diff_parts` and `diff_tests`. Two test files were renamed for what they
hold now: `models_grid.rs` → `diff_models.rs` (the CLI's diff tests) and `key_declines.rs` → `diff_declines.rs` (the
diff's two allocation readings, each now asserting its side of the bound).

### Readings (defaults; the main session files them)

1. **`--models` goes whole**: with `key` and `grid` gone its one value `list` added nothing, and nothing reads it: no
   script (`utils/review` ran only `key` and `grid`), no corpus run, no config row (`06-config.md` named it in § 3.3's
   presentation list and § 5 only), and neither `ein test` nor any artefact. *Default: removed.*
2. **Removed, not retired**: the flag is not added to `ein solve`'s retired options (`cmdline.rs`' `retired(…)`,
   `solve::RETIRED`, refused with what answers instead); clap refuses it as an unknown argument, exit 2. The retired
   table holds what the switch to R replaced. *Default: not retired*; the alternative is a retired entry naming
   `--maximal` and `-f`.
3. **Q-M1f.412's finding that `--models grid` reads `is-a`, and P1f.23's finding on the grid's names, end with the
   code** (`grid.rs` is deleted). *Default: closed by this stage.*
4. **The determining key leaves the reasoning page** (§ 6), with *what models differ in*, since no page uses either.
   *Default: taken out.*
5. **The walkthrough shows `-f`'s output** where it showed the grids (root after step 1, the final state): Ein facts
   of the five relations a guess is made of, as printed, and no hand-drawn table. *Default: `-f`.*
6. **`models.rs` keeps its name** for `--diff-models`. *Default: kept*; the alternative is `diff.rs`.

### Findings (for this phase's `findings.md`, R-n, on merge)

- **(a) The link check in a fresh checkout, known.** `utils/doc_audit.py --links --check --tracked` in the
  worktree reports 2 961 findings, every one in a `README.md` under `plans/m1f_hypothesis_and_documentation/review/`
  (33 of them) linking a run's outputs (`renders/`, `runs/`, `tools/`), which the root tree has and a clean checkout
  does not: what `c8b26770`'s message records, on the user's word, to be fixed before a push. Nothing of this stage
  is among them.
- **(b) Process plans that name the forms**, left as written (process, the main session's): the review item
  `review/04-language-and-decisions/16-Q-M1f.412/` re-runs `--models grid` on its two programs (its README's re-run
  and § 2.4), which now refuse; `p1f.21_verdicts_as_rules/findings.md` lists `grid.rs` among the readers of `is-a`
  by name; `review/05-verdicts-as-rules/33-Q-M1f.399/` cites `grid.rs:317`; M1g's
  `s1g.2.2_valuations_as_hypotheses.md` plans `--models key` on the letters; M1h's
  `s1h.2.4_what_used_means_and_the_stop.md` names `--models` as a form; `plans/README.md`'s M1d line and
  `open_questions.md`'s Q-M1f.255/.258 records name it (that file is not this stage's to edit).
- **(c) A census names a deleted test**: `utils/censuses/corpus-cost.md` § 8, a dated re-take, names
  `models_grid.rs` among the per-commit runs on `zebra2`; left as the record it is. The runs it lists (`--single`, a
  bare run, `-n 1`) stay per-commit in `acceptance_cli.rs` and `answer_dump.rs`; what went is the CLI-level check
  that a renderer drew the 25 published cells, with the renderer.

### Checks

- `cargo build --profile gate --workspace --all-targets`, `cargo fmt --all -- --check`, `cargo clippy -p ein-render
  -p ein-cli -p ein-corpus -p ein-infer --all-targets -- -D warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc
  --no-deps` on the same four: clean.
- The crate tests touched, each alone: `forms_cut` 4, `closed_stdout` 4, `region_modes` 22, `model_set_report` 2,
  `diff_models` 2, `config_reference` 9, `help_surface` 4, `world_anchors` 2, `ein-render`'s lib 21, `diff_declines`
  2, `ein-infer`'s `acceptance` 8: all pass.
- `cargo test --profile gate -p ein-render -p ein-cli`, with `TMPDIR` a fresh directory, left empty: exit 0, 77 test
  binaries, 452 tests passed, none failed or ignored.
- The shape golden: no op digests a `--models` form (its ops are `load`, `saturate`, the views, the read-outs of the
  five questions, `oracle`, …), so no line moved and none was re-banked; `id_order_invariance` in the run above
  holds it.
- `python3 utils/review_bundle.py --smoke`, as the gate runs it (the worktree's `target/gate/ein`): 2 cases, 1
  program, 0 failed, 25 SVGs, link check exit 0.
- `utils/check_doc_blocks.py`: ok, rules 1–5. `utils/check_generated.py --bin ein.rs/target/gate/ein`: 9 generators,
  every file as written. `utils/check_release_voice.py`: ok. `utils/check_gate_steps.py`: ok.
- `utils/doc_audit.py --links --check --tracked`: exit 1, every finding (a)'s, known; none in a file this stage
  touched, and its decision-id pass 0 findings.
