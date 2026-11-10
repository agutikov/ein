# S1f.7.11 — A goal over a relation nothing in the program makes

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-11, from the [analysis of 2026-11-11](README.md#the-analysis-of-2026-11-11), on the user's
ruling of that day (*"yes - it become S1f.7.10 onward"*). Not started.
**Depends on:** [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md), so that a goal's macro call is expanded before its
relations are read, and [S1f.7.8](s1f.7.8_a_solve_list_naming_nothing.md), whose check of a name against the
program this stage reuses.

## The finding

Taken on the release binary at `4bf6da73`, 2026-11-11: `(p k)` with `(query :goal (zzz ?x))` loads, and every state
reports *the goal has no row*, exit 0. No declaration, fact, rule or hrule names `zzz`, so no state can hold a
`zzz` fact. A relation misspelt in a goal reads the same as a goal that is undetermined.

A claim is already checked: `:expect` naming such a relation is refused, *names att, which no declaration or fact
makes a relation*. That check is too strict the other way: it refuses a relation that only a rule's `:assert`
derives (the analysis's `d1.ein`: the claim lists `(won a)`, which rule `r` derives, and the load is refused).

## The direction

- **(a) Refused at load** (the default): a goal's premise over a relation that no declaration, no fact, and no
  rule's, hrule's or obligation's conclusion names. A relation a conclusion can make is accepted.
- **(b) A warning row** in the read-out, the goal loaded.

The claim check's over-refusal is the same list read short. T1 takes it, and the stage fixes it with (a)'s list if
T1 confirms it, as a row of its own.

## Tasks

- **T1f.7.11.1, confirmation and the census.** Re-take both. Every site a relation is read and never made: a goal,
  a goal under `absent` (where it holds vacuously), a claim, a rule's `:match` (a rule that can never fire).
  Whether the rule's site is this stage's is a Q-M1f row; the default is that it is reported and not refused,
  because a library rule may read a relation its importer supplies.
- **T1f.7.11.2, the rows**, `open`, held to today's engine.
- **T1f.7.11.3, the fix**, the rows flipped, a test on every command.
- **T1f.7.11.4, the pages**: `01_grammar.md` § Query, `defined_behaviour.md`, the records.

## Acceptance

- A goal over a relation nothing makes is refused on every command, in S1f.5.21's spelling; a claim over a relation
  a rule derives loads.
- Every witness is a register row. No corpus answer moves but the witnesses' and the fixtures the stage names.
- `./run_tests.sh` green.
