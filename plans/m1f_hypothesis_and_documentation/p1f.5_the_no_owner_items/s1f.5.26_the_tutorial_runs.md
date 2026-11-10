# S1f.5.26 — The tutorial's programs run on today's engine

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 6
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). Not started.
**Depends on:** nothing. It edits only `docs/guide/` and a test, so it may run first.

## The ruling

The close review's finding 6, as written on 2026-11-10:

> **The tutorial is broken.** `docs/guide/04_solving_the_whole_puzzle.md` line 13 writes the retired `(config
> :enable-pre-branch-lookahead true)`, so a reader who copies it gets a refusal.

The user, 2026-11-10: *"fix the tutorial, new stage"*.

## The case

[`docs/guide/04_solving_the_whole_puzzle.md`](../../../docs/guide/04_solving_the_whole_puzzle.md) shows what
`zebra2.ein` is made of in a `lisp` block whose first line is `(config :enable-pre-branch-lookahead true) ; an
engine knob`. That flag was retired at M1e2 S1e2.1a.5 ([`configuration.md`](../../../docs/kernel/configuration.md)
lists it and the stage that retired it), and `zebra2.ein` no longer writes it. A reader who copies the line gets the
retired-flag refusal.

What holds the guide today is [`guide_transcripts.rs`](../../../ein.rs/crates/ein-cli/tests/guide_transcripts.rs):
it re-runs each *marked* transcript (`<!-- transcript: … -->`) and diffs its bytes. Nothing reads the guide's
*program* blocks, which is how a retired flag survived three milestones. S1f.5.14 wrote `:why`s into the guide's
blocks by hand (`9c06414f`), and nothing checks those either.

## What it changes

- **The block.** Chapter 4's `(config …)` line goes, with the prose that explains it. The block shows `zebra2.ein`
  as it is.
- **Every program block in the guide, read.** Each `lisp` block in `docs/guide/` is either a whole program or an
  excerpt (it writes `…`). Every whole program is loaded and answered. In an excerpt, every complete top-level
  form is parsed and loaded. Whatever a block writes that today's engine refuses or answers differently from the
  prose is fixed in the page, never in the engine.
- **Pinned.** Default: a test beside `guide_transcripts.rs` that does the reading above on every `lisp` block of
  `docs/guide/`, so a retired flag, a keyword or a missing `:why` fails `cargo test`. It is held both ways: a
  planted retired flag in a scratch copy fails it. The precedent is S1f.5.14's doc-block migration, and
  [Q-M1e.4](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)'s
  rule that a page is pinned by something that runs. Recorded as a Q-M1f row, taken by default.

## Tasks

### Task T1f.5.26.1 — every block, read

List every `lisp` block of `docs/guide/`, classify it, and run it. Record what each does today.

### Task T1f.5.26.2 — the pages fixed, and the pin

Fix each block that fails; the test; `docs/guide/README.md`'s note on what is pinned and what is not.

## Acceptance

- No block in `docs/guide/` writes a form today's engine refuses, held by the test.
- Chapter 4's block matches `zebra2.ein`'s forms where it quotes them.
- `./run_tests.sh` green.
