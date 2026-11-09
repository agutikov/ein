# S1f.5.6 — Where ∅ is in R, every minimal-line question stops at root, and says only ∅

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.6**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1.5 days.
- A quarter of a day to re-take the moves on the build and list every golden.
- Half a day for the rule in the flagged route, and its tests.
- A quarter of a day for the clause dropped, the summary's field and their goldens (S-17, F1).
- Half a day for the pages.

**Status:** specified 2026-11-07 on the user's rulings of that day on S-16, which closes P-14 with it,
and on S-17 and F1 ([§ The clause](#s-17-and-f1-the-clause-goes)).
**Not started, and not to start until the user says so.** No engine code, test, golden or fixture has
moved.
**Depends on:** nothing. It touches `explore.rs` and the read-out, and none of the grammar stages do.

## The ruling

The agent put two choices for S-16: (a) leave `--minimal --single` and `--minimal -n N` weaker than
describe, or (b) use the ∅ rule in both, so that where root is alive and owes nothing, `--single` answers
*Solution* and `-n N` says ∅ is the whole minimal line, each after one saturation. The user, 2026-11-07:
*"b, use the ∅ rule in both"*.

## The rule

The minimal line is the members of R with no smaller member of R. ∅ is a subset of every commitment, so
**where ∅ is in R, the minimal line is {∅}**, and no other state can be on it. Whether ∅ is in R is root's
fixpoint alone: root alive and owing nothing, one saturation. The rule holds on every program, monotone or
not, within `--region-limit` or past it. It needs no assumption about what a node is
([Q-M1e2.2](../../m1e2_correctness/open_questions.md), F21), since ∅ is root itself.

## The case

Taken on 2026-11-07 at `916d97ac`, on the release binary: every `.ein` under `examples/`, `tests/`,
`stdlib/` and `corpus/`, 390 files, under `ein solve`, `--minimal -n 1`, `--minimal -n 2`,
`--minimal --single` and `--minimal`.

- **213 have ∅ in R**, by describe's `∅` row.
- **190 of them already answer at root.** Every minimal question costs one saturation and says so:
  `minimal line  1 state — all of them`, `exhausted  yes — this is all of the minimal line`, and
  `--single`'s `verdict  Solution — the minimal line is one state`.
- **The other 23 are the programs the static check flags**, answered by `flagged` in
  [`explore.rs`](../../../ein.rs/crates/ein-infer/src/explore.rs) (line 1358), which walks the cube in layer
  order and stops when its question is decided. It does not see that root decides it.
  - **All 23:** `-n 1` stops at root and says `1 state, as asked — the minimal line may hold more`.
  - **16 within the limit:** `-n 2`, `--single` and `--minimal` answer right, after 2, 4 or 8 saturations.
  - **7 past the limit:** `--single` says `not decided — the run stopped first` and `-n 2` says
    `1 state   (a lower bound: the run stopped first)`. `--minimal` lists ∅, from describe's partial
    description, and says `exhausted  no`. Each runs the cube to the limit:

| program | `--single`, `-n 2` and `--minimal`: saturations each |
|---|---:|
| `ein-bugs/wrong-answers/past-limit-lines` | 65 536 |
| `saturation/square-unique/corner-house` | 65 389 |
| `lattice/06_friends_tables` | 65 299 |
| `saturation/square-unique/terminus` | 65 029 |
| `saturation/square-unique/cul-de-sac` | 64 923 |
| `features/04_open` | 64 761 |
| `features/01_not_and_absent` | 64 655 |

Over the 213, each of the three questions cost 455 848 saturations. With the rule, 213.

## What it changes

**The route.** In `flagged`, before the layer walk: where the question is on the minimal line (`Which::Minimal`,
under any `Ask`) and root is alive and owes nothing, the answer is {∅}, complete for the line, after one
saturation. No other route moves, since the 190 already stop there.

**The words.** The 23 print what the 190 print:

| question | today, on the 23 | after |
|---|---|---|
| `--minimal -n N` | `1 state, as asked — the minimal line may hold more`; past the limit under `-n 2`, `1 state   (a lower bound: the run stopped first)` | `1 state — all of them`, `exhausted  yes — this is all of the minimal line` |
| `--minimal --single` | past the limit, `verdict  not decided — the run stopped first` | `verdict  Solution — the minimal line is one state`, `decided  yes — the minimal line holds no other state` |
| `--minimal` | past the limit, ∅ listed and `exhausted  no — not monotone, and the question needs the whole cube` | `1 state — all of them`, `exhausted  yes — this is all of the minimal line` |

`--minimal` is not named in the ruling, but it is the same question without a count, and the rule answers
it the same way. Describe's own row (`1 state — ∅, the trivial line: no obligation is stated`) words the
same line differently on 206 of the 213. That sentence is [S-17 and F1](README.md)'s, and this stage leaves
it alone.

**What else moves, and what does not.** On the 23, `--json-summary`'s verdict, `exhausted` and `stopped`,
`--trace`, and `--dump-answer`, which writes no other members met, since one node is met. No R moves, and
no state is listed that was not: every answer is still {∅}, as the oracle has it.

**The register.** W27's two minimal rows ask `--minimal` on two of the 23, past the limit, and record more
than the states. `W27/past-limit-lines-minimal` holds `exhausted = false` and `others = 2` in its `right` and
its `now`, and `W27/fifo-gap-limit-1-minimal` (`--region-limit 1`) holds `exhausted = false` and
`others = 1`. Their `right` is restated on this ruling in the same commit, `exhausted = true` and
`others = 0`, and both stay `fixed`. The register counts do not move.

**The corpus.** Two runs ask a minimal question on a flagged program: `fifo_gap`'s `solve --minimal
--region-limit 1` and `past-limit-lines`' `solve --minimal --region-limit 8`. Their `exhausted` and `work`
rows move. Step 1 lists every golden before it moves.

## S-17 and F1: the clause goes

**The ruling.** The agent put three choices: (a) drop the clause, (b) keep one made true, *root owes
nothing*, or (c) keep *no obligation is stated* only where the program states none. The user, 2026-11-07:
*"a, drop the clause"*.

**The case.** Taken on 2026-11-07 at `916d97ac`. Describe's minimal-line row reads
`1 state — ∅, the trivial line: no obligation is stated` where ∅ is in R and no node the run met owes
(`d.owing_met`, [`read_out.rs`](../../../ein.rs/crates/ein-render/src/read_out.rs) line 836), whatever the
program states:

| program | states an obligation | the row today |
|---|---|---|
| 8 `tests/stdlib/` programs, among them `algebra/24_total_owed_satisfied` | yes, and root discharges it | `1 state — ∅, the trivial line: no obligation is stated` |
| the inventory's [F1 probe](../../m1e2_correctness/inventory_2026-11-06/findings/F1_trivial_line_stated_obligation.ein) | yes, `(total-owed likes is-a)` | the same |
| its control, the same program without that line | no | the same |
| `past-limit-lines` at `--region-limit 1` | yes | the same |
| `past-limit-lines` at `--region-limit 2` | yes | `1 state — ∅` |

The `--json-summary` field `minimal.trivial` is read off the same bit
([`region_answer.rs`](../../../ein.rs/crates/ein-render/src/dump/region_answer.rs) line 250).

**What it changes.**
- **The row.** Where ∅ is in R, describe's minimal-line row reads `1 state — ∅` on every program, at every
  limit. The `∅` row above it, `in R — root is alive and owes nothing`, is the reason, and is unchanged.
- **The summary.** `minimal.trivial` reads *∅ is in R* (`root.in_r()`), so the field and the row agree.
  The field keeps its name and place, and the page that lists the summary's fields says what it means.
- **What stays.** The maximal line's clause, `⊤, the trivial line: no restriction bites`, is printed only
  on an exhausted run, where no node is dead in the whole cube, so it is true. No answer, R or register row
  moves.
- **What moves.** Describe's row on most of the 213 programs with ∅ in R (206 print the clause today), the
  two trace goldens that print it (`ein-render/tests/golden/trace_ambiguous.md`, `trace_unconditional.md`)
  and the read-out's golden page. Step 1 lists each.

## The method

### Task T1f.5.6.1 — every move listed

The sweep above re-taken on the build, with `--json-summary` and `--dump-answer`, and `cargo test
--workspace` run on a prototype. Every move must be one of the 23's rows above.

### Task T1f.5.6.2 — the rule

The check in `flagged`, and three tests:
- **`ein-render/tests/line_modes_past_the_limit.rs`:** where describe says ∅ is in R, `--minimal -n N`,
  `--single` and `--minimal` list ∅ alone, decided, after one saturation. Today the file leaves `-n N` and
  `--single` out of its row check.
- **`ein-infer/tests/explore.rs`**, which holds every mode to the oracle: every minimal question on a
  program whose root is in R costs one saturation, on every route. That is *minimum work per question*,
  held where the corpus can break it.
- **`ein-cli/tests/region_modes.rs`:** the three rows of the words table, through the binary, on
  `past-limit-lines.ein`.

The register's two rows are restated in the same commit, and `ein-cli/tests/wrong_answers.rs` holds them.

### Task T1f.5.6.3 — the clause (S-17, F1)

`minimal_line` in the read-out and `minimal.trivial` in the summary, as above, in their own commit. A test
in `ein-cli/tests/region_modes.rs` holds the five rows of the case to `1 state — ∅`, and the summary's field
to *∅ is in R*, on the F1 probe moved into `examples/` beside its control.

### Task T1f.5.6.4 — the pages

The live read-out reference, [`read_out.md`](../../../docs/kernel/inference/read_out.md) (S1f.5.16): its rows for `-n N` and `--single` past the
limit, its `minimal line` row (no name for {∅}) and its summary fields, in the commit that changes the output;
`solve_interface.md` § 3.3; and the read-out's golden page, re-written by `utils/read_out_examples.py`. M1e2's
`read_out.md` is a record, and is not edited. The inventory's S-16, P-14, S-17 and F1 say what
fixed them. The probe outputs under `p1e2.1b_non_monotone_programs/detector_proposal/outputs/` are a record,
and do not change.

## Acceptance

- On every corpus program with ∅ in R, `--minimal -n N` for every `N`, `--minimal --single` and `--minimal`
  answer {∅}, decided, after one saturation.
- `--single` says *Solution* on the 7 past the limit, and `-n N` says ∅ is all of the minimal line on the 23.
- Every mode still agrees with the oracle within the limit, and with describe past it.
- Where ∅ is in R, describe's minimal-line row reads `1 state — ∅` on every program and at every
  `--region-limit`, and `minimal.trivial` is true exactly where ∅ is in R.
- No R moves. W27's two minimal rows have their `right` restated, and stay fixed.
- `./run_tests.sh` is green.
