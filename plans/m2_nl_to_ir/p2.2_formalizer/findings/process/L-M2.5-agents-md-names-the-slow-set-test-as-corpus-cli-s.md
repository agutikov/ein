# L-M2.5 — AGENTS.md names the slow-set test as `corpus_cli`'s; it is `ein-corpus`'s

**Status:** closed: AGENTS.md says `ein-corpus`'s, by the coordinator's commit after S2.2.0a
**Found:** by S2.2.0a, 2026-11-26, adding four corpus entries and checking whether one of them is `slow`
**Owner:** none (AGENTS.md is the coordinator's to edit)

## What

AGENTS.md § Where things live (`corpus/`) says *"`corpus_cli`'s `the_slow_set_is_exactly_these_sixteen` names the
set"*, and the gate's command list says *"which the_slow_set_is_exactly_these_sixteen names"* beside a `corpus_cli`
command. The test is `ein-corpus`'s, in `ein.rs/crates/ein-corpus/src/manifest.rs`, as `corpus/README.md` and
`corpus/corpus.toml`'s header say; `corpus_cli` only cites it in a comment.

## Evidence

`grep -rn these_sixteen ein.rs/crates AGENTS.md corpus`: the `fn` is at `ein-corpus/src/manifest.rs`; AGENTS.md's two
lines.

## What it costs

A stage that makes an entry slow, and has to rename the test and every reference to it, is sent to the wrong crate
first. None of the four entries this stage added is slow, so nothing was renamed.
