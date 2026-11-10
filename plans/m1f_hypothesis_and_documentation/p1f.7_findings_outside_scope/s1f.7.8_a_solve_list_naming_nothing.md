# S1f.7.8 — A `(solve …)` list that names nothing the program declares

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from [S1f.5.23's T2 record](../p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md#t2-record-2026-11-10)
(its out-of-scope finding 3), filed here under [Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes).
Not started.
**Depends on:** nothing.

## The finding

Recorded on the release binary at `f62e34e1`:

> **A `(solve …)` list naming nothing the program declares loads in silence.** `(solve :hrules (zzz))` beside a
> declared hrule `h` answers as the program without the form (two states), `(solve :hypothesis-relations (zzz))`
> leaves H empty and `:no-hypothesis (zzz)` restricts nothing, each with no word that the name matches no hrule or
> relation; a query's `:over zzz` and a claim's undeclared relation are refused at load.

So a misspelt name changes the answer in silence (H empty, or no restriction), while the same mistake in a query
is refused.

## The direction the rulings point to

A query's `:over` and a claim's relation are refused at load when they name nothing. The direction that agrees is
the same refusal for every name a `(solve …)` list holds: one line in S1f.5.21's spelling naming the form, the list
and the name. That is the default, since it moves no program that names what it declares.

## Tasks

- **T1f.7.8.1, confirmation and the census.** Re-take the finding. List every clause of `(solve …)`, `(query …)` and
  `(config …)` that names a relation, an hrule, a rule or a type, and what each does with a name the program does
  not declare, on every command.
- **T1f.7.8.2, the rows.** One witness per list whose answer moves. Each is filed `open` and held to today's engine.
- **T1f.7.8.3, the refusal.** At load, with the rows flipped and a test on every command.
- **T1f.7.8.4, the pages.** `01_grammar.md` § Solve, `defined_behaviour.md`, and the stage's records.

## Acceptance

- Every name a `(solve …)` list holds names something the program declares, or the program is refused on every
  command with one line.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.8 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.
