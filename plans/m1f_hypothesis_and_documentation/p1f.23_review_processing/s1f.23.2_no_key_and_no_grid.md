# S1f.23.2 — `--models key` and `--models grid` removed

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word. It runs in the worktree `build/wt-s1f232` on the branch
`s1f.23.2-models-forms`, beside S1f.23.1, which edits the same CLI files on master; the main session merges it after
S1f.23.1's code, and one gate, `utils/ci_container.sh` and one nightly hold the two on the merged tree.

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

Not started.
