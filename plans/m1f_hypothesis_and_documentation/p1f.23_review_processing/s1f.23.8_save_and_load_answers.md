# S1f.23.8 — the user's answers on save and load, built

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word (*"process plans/m1f_hypothesis_and_documentation/review/01-save-and-load/ANSWERS.md"*),
in a worktree beside S1f.23.6; merged after it, and one gate, `utils/ci_container.sh` and one nightly hold both.
**The answers:** the user's file as written, `35719115`
(`git show 35719115:plans/m1f_hypothesis_and_documentation/review/01-save-and-load/ANSWERS.md`); the topic left the
bundle with every item answered (`eaabeada`).

## The answers

| item | the question | the user's answer |
|---|---|---|
| L-106 | `answer_round_trip`'s nightly shards near five minutes | (a), as it is |
| Q-M1f.413 | `ein kb save` of a saved file | **(c)**: *give the container a section for the commitment, so a node's container opens as that node* |
| Q-M1f.415 | `ein show hyps` / `ein render h-space` of a saved file | (a), the program's H |
| Q-M1f.416 | what `ein test` asks of a saved node | **(b)**: *no run at all: each claim NOT CHECKED, no run's data in `--json-report`* |
| Q-M1f.417 | the `commit` row, `root.commit`, `ein-answer/3` | (a) |
| Q-M1f.418 | the nesting limit and a saved file's wrappers | (a) |
| L-104 | `:hypothesis 0` in `ein saturate --dump` | (a), as it is |
| F23 D-24 | the note on a carried standard-library module | *"Drop the note entirely, since the file is self-contained and comparing libraries isn't the loader's business"* |

## What it builds

1. **Q-M1f.413 (c)**: `ein kb save` of a saved file stores its `(commit …)` in a section of the container, and a
   node's container opens as that node: `ein solve x.einb` of a node's container answers what `ein solve` of the
   saved file answers, byte for byte but the path (the container's contract). A root's container stays as it is. The
   container's version moves if its format does (with R-30, the 24 bytes S1f.23.4 took out at the same `einb/2.0`):
   the stage says what the version is and why.
2. **Q-M1f.416 (b)**: `ein test` of a saved file with a `(commit …)` runs nothing: each claim is NOT CHECKED, no exit
   code moves, and `--json-report` carries no run's data for it.
3. **D-24**: the `note:` a saved file's load writes for a carried `std.*` module that differs from the binary's
   library goes, with its code, its tests and its pages; a saved file loads from what it carries and compares nothing.
4. **The ledger**: each answer filed against its question as ruled, the user's words verbatim (`open_questions.md`,
   the milestone's `findings.md` for L-104 and L-106, F23's D-24 row); the decisions Q-M1f.413 and .416 amend
   (Q-M1f.403's file, `ein kb save`'s page, `10-answer-dump.md`, `05-cli.md`, `07-diagnostics.md`) brought to the
   rulings.

## Acceptance

- No answer moves; a saved node's container answers as its saved file, held by a test over the corpus's saved
  nodes (`answer_round_trip`'s or `einb_cli`'s); the claims of a saved node NOT CHECKED with no run (a test).
- The gate, `utils/ci_container.sh` and the nightly, once, with S1f.23.6, on the merged tree.

## Record

Not started.
