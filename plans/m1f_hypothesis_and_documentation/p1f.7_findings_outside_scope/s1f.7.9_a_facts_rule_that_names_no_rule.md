# S1f.7.9 — A fact's `:rule` that names no rule

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from [S1f.5.23's T2 record](../p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md#t2-record-2026-11-10)
(its out-of-scope finding 4), filed here under [Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes).
Not started. **Reduced to its rows on 2026-11-11** (the user: *"yes S1f.7.9 moves to a new F24"*): the fix and the
question under it are [F24](../../followups/f24_written_provenance.md)'s, and this stage does T1 and T2 below and
files its rows `open`, owned by F24. The sections after § The finding are the plan as it stood, kept as record.
**Depends on:** nothing.

## What the analysis of 2026-11-11 found

A written `:rule` fact is a derivation with no premises, whether or not its rule is declared, so the fact leaves
every core: `(p k :rule zzz)` with `p → q` and `q ∧ p → (false)` prints `unsat core ∅`, where the program without
`:rule` gives `(p k)`. `:rule r`, a declared rule, and `:using ((nope x))`, a fact never held, do the same. R is not
moved. A `--dump-answer` state file writes no `:rule` (its provenance is a comment), so no dump depends on the form.
Refusing an unknown name would leave the empty core, and would move `examples/syntax/unary-and-nary.ein`,
`examples/broken/load/derivation_cycle.ein` and four test files that write provenance by hand. The witnesses this
stage files are the three rows of F24's table.

## The finding

Recorded on the release binary at `f62e34e1`:

> **A fact's `:rule zzz` names no rule, in silence**: `(p k :rule zzz)` loads and `p k` is stored as derived by a
> rule the program does not declare (`ein saturate --dump`: `(p k :rule zzz)`).

So a written fact can claim a provenance the program does not have. Every surface that names a fact's rule (the
trace, the cores' `refuted by`, the natural-language rendering) then names a rule that does not exist.

## What T1 has to settle first

**Who writes `:rule` on a fact.** `ein saturate --dump` prints derived facts with `:rule`. If such a dump is meant
to load on its own, without the rules that derived it, then a `:rule` that names no rule of the reloaded program is
the dump's normal case and not a mistake. T1 lists every writer and every reader of a fact's `:rule`, and whether
anything reloads a dump.

**Then the direction.** If nothing reloads a dump without its rules, the direction that agrees with S1f.7.8 is to
refuse at load a `:rule` that names no rule. If something does, the choice is between that refusal, a provenance
the engine marks as not checked, and keeping it as it is but saying so on every surface. That choice is a question
for the user.

## Tasks

- **T1f.7.9.1, confirmation and the census.** Re-take the finding. List the writers and readers of a fact's
  `:rule`, and which surfaces print it.
- **T1f.7.9.2, the rows.** One witness per surface that names a rule the program lacks. Each is filed `open` and
  held to today's engine.
- **T1f.7.9.3, the fix.** The chosen direction, with the rows flipped and a test on every command.
- **T1f.7.9.4, the pages.** `01_grammar.md` (a fact's clauses), `defined_behaviour.md`, the dump's page, and the
  stage's records.

## Acceptance

- No surface names a rule the program does not declare, unless it says the provenance was not checked.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.
