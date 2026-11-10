# S1f.5.28 — The findings taken from the stage records: confirmed, triaged, fixed

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 9, and given finding 7's three
stale counts by default ([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). Not
started.
**Depends on:** nothing. It runs last of the phase's stages, after S1f.5.27, so its re-checks are taken on the tree
the other seven leave.

## The ruling

The close review's finding 9, as written on 2026-11-10, is a list *"taken from the stage records and not re-checked
today"* (below). The user, 2026-11-10: *"new stage for investigation, confirmation, triage and fixes"*.

Finding 7, the stale slow counts, drew a question rather than a ruling: *"so what? what does mean slow here? slow
for what? compared to what?"*. The close review answers it under the finding. The fix is three prose edits and a
citation, so the coordinator placed it here by default, recorded as a Q-M1f row. The user may move it.

## The items

| # | item | from |
|---|---|---|
| 1 | `region_dump.md` never names seven fields: `reads.dynamic_*`, `absent_reads.dynamic_*`, `cycles[].negative`, `paths.dead_parents`, `paths.saturations`, `paths.unprinted.saturations`, `state.stale_nodes` | S1f.5.16 |
| 2 | `configuration.md` never names `leftover.states_by_relation` | S1f.5.16 |
| 3 | `-t`'s `route` row and the cube's counts print without digit groups | S1f.5.16 |
| 4 | Route 1's *stopped before layer L* is reached by no CLI run | S1f.5.16 |
| 5 | `--trace` prints a `:why` or `:source` as markdown paragraphs, where a single line break is a soft break | S1f.5.16 |
| 6 | Stale comments: `ein-core/src/terms.rs` (the SYMBOL exclusion and `LEXER_KEYWORDS`), `FIFO_GAP_PRIORITY` in `ein-infer/tests/detector.rs`, the headers of `form-in-guard.ein` and `guard-before-binder.ein`, the `corpus/fuzz_findings` notes | several |
| 7 | `player` is declared `T T` but used with one argument in two Rust tests | S1f.5.14 |
| 8 | `utils/guard_split.py` reports `synth/trigger_hole` as *does not load* | S1f.5.14 |
| 9 | `ein kb save` of a container writes 1 288 B where the `.ein` gives 1 248 B for the same KB | S1f.5.19 |
| 10 | The fuzz property costs about 27 s per 20 000 inputs | S1f.5.10 |
| 11 | Finding 7: the `corpus/corpus.toml` header says *"Two entries are"* slow and `corpus/README.md` § `slow` says *"Three"*, naming `features/04_open`, which is not slow since M1e2 S1e2.1a.5. That README and `ein-cli/tests/corpus_cli.rs` line 112 cite `the_slow_set_is_exactly_these_three`, a test renamed `…_these_fourteen` (`ein-corpus/src/manifest.rs`). 14 entries are slow (`grep -c '^slow *= *true' corpus/corpus.toml`, 2026-11-10) | the close review |

Items 12 to 16, the wrong answers S1f.5.21's and S1f.5.24's records found outside their scope, were given here by
the coordinator on 2026-11-10. **They moved the same day to a phase of their own,
[P1f.7](../p1f.7_findings_outside_scope/README.md), a stage each (S1f.7.1 to S1f.7.5)**, on the user's instruction:
*"new batch of findings … creates new phase p1f.7 with stage per finding to allow to close current phase"*. A wrong
answer this stage finds outside items 1 to 11 goes there too, as a new stage.

Item 10 is S1f.5.27's to measure (its T1 attributes the fuzz job's cost). This stage takes it only for what that
measurement leaves to fix.

## The method

**T1f.5.28.1, confirmation.** Each item is re-checked on the release binary at the tree S1f.5.27 leaves, with the
command or the read that shows it, and marked *confirmed*, *not reproduced* (with what changed) or *already fixed*
(with the commit).

**T1f.5.28.2, triage.** Each confirmed item gets one of three dispositions:
- **A wrong answer**: a register row, filed `open` before any fix. For example, item 9 if the two containers do
  not answer alike, or item 8 if `trigger_hole` loads and the script is wrong about a program.
- **A defect that moves no answer** (a page, a comment, a label, a script's report): fixed here.
- **A question that is the user's**: one that would reverse a ruling, or that has no stated criterion. Recorded as a
  Q-M1f row and put to the user. Anything else takes the stage's default and is recorded as defaulted.

**T1f.5.28.3 onward, fixes**, one commit per kind (pages, comments, tests, the script, the printer), each gated.
A test pins a page fix where the page has a test (`read_out_reference.rs`, `config_reference.rs`, the region dump's
reference). Prose that states a count either carries its command or test, or drops the count (Q-M1e.4).

## Acceptance

- Every item is confirmed, not reproduced or already fixed, with its evidence, in the stage's record.
- Every confirmed item is fixed, a register row, or a Q-M1f row the user has seen.
- `corpus/README.md`, the `corpus.toml` header and `corpus_cli.rs` say what `the_slow_set_is_exactly_these_fourteen`
  pins, by citing the test rather than restating its number.
- `./run_tests.sh` green.
