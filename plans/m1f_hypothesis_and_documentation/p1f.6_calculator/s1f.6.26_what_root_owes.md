# S1f.6.26 — A read-out row for what root owes

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day; **running 2026-11-13**, the record below.
**An engine stage,** on the read-out.
**Depends on:** nothing in the phase. Independent of the stdlib layout.
[S1f.6.28](s1f.6.28_word_problems_second_pass.md) reads its row.

## The ruling

[Question 30](questions.md#q30) asked which fix to wait for where a named gap stands beside a supposition:
*(a) a read-out row for what root owes when R is not empty. A small change to the read-out*, or (b) F21's
ruling on non-monotone programs. The user, 2026-11-12:

> that simple problem couldn't have non-monotone solution, I mean the problem is too simple and too obvious,
> or not? or that means that simple problem, containing ambiguity, could already be non-monotone? Or that
> means ambiguity == non-monotone solution == required additional information? Again - those questions go
> into M1h. Here I don't care - so pick (a).

So (a) is built, and the user confirmed it in round 2 (`dc88520b`: *"OK (a)"*). The questions in the reply are
answered in [round 2, item 30](questions_round2.md#q30) and
are [M1h's P1h.3](../../m1h_beyond_word_problems/README.md#p1h3--gaps-assumptions-and-readings); nothing of
them is built here.

Row 65 of [section J](questions.md#j-findings-that-need-an-owner) (E-18) was left empty and so took its
suggestion, *decided with question 24 and question 30, as one read-out stage*. Question 24 was not ruled
(round 2, items 24, 28 and 37: *nothing is built from 24*), so **this stage takes of E-18 only what a row
can say, and changes no claim.**

## The two findings

- **E-19.** With R empty the `owing` rows print each owed instance with its rule's `:why`. With R not empty
  the `∅` row reads *not in R — root owes 1 instance* and neither the read-out nor `--trace` says which
  instance or why: it is in `--events` alone, the `owe` line. The witnesses are a word problem whose root
  owes a reading of *need* and one whose root owes a quantity. The count is right; the `:why` of the debt,
  which is the named gap, is not printed.
- **E-18.** A query cannot tell an R that is empty because root owes from one that is empty because root is
  dead: each query's row reads *R is empty: no state to answer from*, and a query's `:expect (false)`
  **holds** on a program that only owes, though root is alive and `ein solve --single` says `Open`. Only
  the region claim tells the two apart.

## What it builds

**The row.** Where root is alive and owes, the read-out names each instance root owes, with its rule's
`:why` rendered, whether R is empty or not: the same rows the empty case prints today, under the `∅` row.
[Round 2, item 30](questions_round2.md#q30) is the reason it matters: *the gap written as a debt is
monotone, because it is the engine and not a rule that says "missing"*, and with this row the monotone form
says what the non-monotone one says.

**A query's row over an empty R says which empty.** Where R is empty and root is alive, a query's row says
that root owes and how many instances, in place of the bare *no state to answer from*; where root is dead it
says so. No claim's outcome moves.

**What it does not build:** the N row for an R that is empty by owing (question 24's option (a)); a core for
an open state; any change to what `Open` or `(false)` mean. Those are
[M1g's P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md).

## Tasks

1. **T1f.6.26.1 — the rows shown first, on real programs,** before the printer moves: today's read-out and
   the proposed one side by side for E-19's two witnesses and E-18's (rebuilt from `build/s1f.6.20/` or
   taken from `examples/word_problems/` where banked), and for one zebra-family program where root owes many
   instances, so the row's length is seen. The listing shares the read-out's line budget (the user's rule of
   2026-10-16: stdout listings share 100 lines, and `--dump-answer` writes the whole read-out); the table
   shows where it is cut and what the cut line says.
2. **T1f.6.26.2 — the printer.** A variant of `ein_render::read_out::Row` for the owed instances under an
   alive root that is not in R, and the query row's two wordings. **In the same commit:**
   [`read_out.md`](../../../docs/kernel/inference/read_out.md)'s table,
   [`read_out_examples.md`](../../../docs/kernel/inference/read_out_examples.md) re-written by
   `utils/read_out_examples.py`, and `ein-cli/tests/read_out_reference.rs`, which fails otherwise.
3. **T1f.6.26.3 — the other surfaces agree.** `--trace` names the same instances; `--json-summary` already
   carries the owing holes, and the task checks that root's are among them and adds nothing if so;
   `--dump-answer`'s read-out file has the rows uncut. `--single`, `--minimal`, `--maximal` and `-n` print
   the row wherever they print the `∅` row.
4. **T1f.6.26.4 — E-18's claim, asked and not changed.** Whether a query's `:expect (false)` should *fail*
   on a program that only owes is a change to what a claim means, and it is question 24's subject. The task
   files it under the next free id with the two witnesses and the default *no change*, and points it at
   M1g's P1g.7.
5. **T1f.6.26.5 — the goldens.** Every corpus program whose root is alive and owes, with R not empty, gains
   rows: they are counted with a command and named in the record before blessing. `lever_invariance` holds
   the read-out under every lever.
6. **T1f.6.26.6 — the pages and the rows:** E-19 closed and E-18 marked *the row built, the claim asked* in
   [`findings.md`](findings.md); `AGENTS.md`; [`word_problems.md`](word_problems.md)'s § 3.5 gets a pointer
   from the stage that edits that page next, not from this one.

## Acceptance

- On E-19's witnesses the read-out names what root owes and why, with R not empty.
- A query's row over an empty R says whether root owes or is dead.
- R, N, H, every claim's outcome and every exit code are the same before and after, on every corpus program
  (the sweep).
- `read_out_reference.rs` holds the page to the printer.

## What it must not do

- **Not change a verdict word or a claim's outcome.**
- **Not print N** in the read-out of an R that is empty by owing: not ruled.
- **Not take a core** where no surface prints one (*minimum work per question*): the row prints the
  obligation's `:why`, which needs none.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. A change to what a surface prints shows
real computed examples first (T1). Config never changes an answer; every answer exits 0. The gate is
`./run_tests.sh`, then `./run_tests.sh --nightly` after the code commit; `utils/ci_container.sh` if the
review bundle's pages change with the read-out.

## Record, 2026-11-13

The baseline every run below compares with is the release binary built at `e9d692e0`, kept as
`build/s1f.6.26/ein-old`; *proposed* is the tree's. E-19's and E-18's witnesses are S1f.6.20's, in
`build/s1f.6.20/p/` and `build/s1f.6.20/witness/`, which are scratch and not tracked: they load today as they
did, and none is banked under `examples/word_problems/`. The tracked programs that show the same rows are
named beside each.

### T1f.6.26.1 — the rows shown first, done

**Where the row is missing, counted.** `python3 build/s1f.6.26/census.py` runs a bare `ein solve FILE
--json-summary` over every `.ein` file HEAD tracks and reads `region.root`, `region.root_owes`, `region.k_s`
and `region.empty` (2026-11-13, on `ein-old`): 849 files, 511 of which write a summary.

| root | programs | what the read-out says of it today |
|---|---:|---|
| alive and owing, R not exhausted and empty: the `∅` row reads *not in R — root owes n* | **36** | the count, and no instance |
| alive and owing, R empty: the `empty` row | 23, 20 of them with a query | the frontier's `owing` rows, each instance through its `:why`; root's own where root is on the frontier |
| dead | 79, 28 of them with a query | root's core |

The 36: two owe 46 instances (`examples/zebra2-minus-15.ein` and its `-obligations` twin), three owe 36
(`zebra.ein`, `zebra2.ein`, `zebra2-obligations.ein`), one owes 5
(`lattice/06_friends_tables_everyone_sits.ein`), six owe 4, seven owe 3, three owe 2 and fourteen owe 1. So
root owes one to four instances on 30 programs, five on one, and 36 or 46 on five.

**Today and proposed, side by side.** Each cell is what the run prints, the rest of the read-out unchanged.

| program | today | proposed |
|---|---|---|
| `p/gap_assume_ha.ein`, E-19: root owes a quantity, R is the one state that supposes it | `∅  not in R — root owes 1 instance` | the same row, and under it `owed  the problem owes [(amount Alice apples t0)] a value: no sentence states it` |
| `p/boxes13.ein`, E-19: root owes a reading of *need*, R is two states | `∅  not in R — root owes 1 instance` | the same row, and `owed  the problem does not say what need means, and it matters: (n z 4) boxes are filled and (n z 1) apples are left over` |
| `examples/zebra2-minus-15-obligations.ein`: root owes 46 | `∅  not in R — root owes 46 instances` | the same row, four `owed` rows, and one line for the other 42 |
| `p/gap_a.ein`, `witness/gap_a2x.ein`, E-18: R empty, root alive and owing | each query: `over  all of R — R is empty: no state to answer from` | `over  all of R — R is empty, root is alive and owes 1 instance: no state to answer from` |
| `examples/word_problems/contra.ein`: R empty, root dead | `over  all of R — R is empty: no state to answer from` | `over  all of R — R is empty, root is dead: no state to answer from` |

`p/gap_assume_ha.ein`, the rows from `exhausted` to `minimal line`:

```text
today      exhausted         yes — every commitment is classified: this is all of R
           ∅                 not in R — root owes 1 instance
           minimal line      1 state, from 1 commitment — the least guess is 1 candidate

proposed   exhausted         yes — every commitment is classified: this is all of R
           ∅                 not in R — root owes 1 instance
             owed            the problem owes [(amount Alice apples t0)] a value: no sentence states it
           minimal line      1 state, from 1 commitment — the least guess is 1 candidate
```

`p/boxes13.ein`, which the static check flags, so the rows are route 2's; and `ein oracle` prints the same:

```text
proposed   ∅                 not in R — root owes 1 instance
             owed            the problem does not say what need means, and it matters: (n z 4) boxes are filled and (n z 1) apples are left over
           minimal line      2 states, from 2 commitments — the least guess is 1 candidate
```

`examples/zebra2-minus-15-obligations.ein`, a partial run (`build/s1f.6.26/z-old.txt` and `z-new.txt`):

```text
today      ∅                 not in R — root owes 46 instances
           minimal line      ≥ 32 states — complete up to states of 2 candidates

proposed   ∅                 not in R — root owes 46 instances
             owed            color-loc owes Red a House.
             owed            color-loc owes Green a House.
             owed            color-loc owes Ivory a House.
             owed            color-loc owes Yellow a House.
                             … 42 more instances: --dump-answer DIR writes every one, to DIR/answer.txt
           minimal line      ≥ 32 states — complete up to states of 2 candidates
```

`witness/gap_a2x.ein`, whose first query claims `(false)` and whose solve claim is `(region (owing
(model)))`; the `owing` rows, the form and both claims' outcomes are as they were:

```text
today      owing             ∅ — owes 1
                               the problem owes [(amount Alice apples t0)] a value: no sentence states it

             (region (owing (model)))

           query 1 of 2      (and (asked ?q) (value ?q ?n))
           over              all of R — R is empty: no state to answer from
           as :expect        (false)
           :expect           holds

proposed   query 1 of 2      (and (asked ?q) (value ?q ?n))
           over              all of R — R is empty, root is alive and owes 1 instance: no state to answer from
           as :expect        (false)
           :expect           holds
```

`witness/gap_a2y.ein`, the same program claiming `(solve :expect (false))`, fails that claim and exits 1
before and after, with the same line. Three tracked programs have a query's `(false)` holding on an R that
is empty by owing, `examples/binary_logic/unsatisfiable.ein`, `equivalent.ein` and
`examples/beyond_arith/equation_no_solution.ein`: their row reads *root is alive and owes 3 instances*, and
2 on the third.

**The form, each part a default taken, the user's to revise**
([Q-M1f.220](../open_questions.md#q-m1f220--the-form-of-the-rows-for-what-root-owes)):

1. **The label is `owed`, a row of the block under `∅`,** one row an instance, its value the instance
   through its rule's `:why`: the text an empty R's `owing` row prints under itself. A `Row` variant, so the
   reference page has a row for it and `read_out_reference.rs` holds each printed instance to the summary.
2. **It is printed where the `∅` row reads *not in R — root owes n*:** under describe and `ein oracle`, on
   an exhausted R and a partial one. `--count` prints the `∅` row and no `owed` row, as it prints the `empty`
   row and no `owing` row. A mode prints no `∅` row, so none (T3).
3. **The cut is the warning's: at most four rows on stdout, then one line that counts the rest and names
   `--dump-answer DIR`, whose `answer.txt` has every one.** The block does not take the listings' 100
   lines. This departs from this file's T1, which has the listing share that budget, on the measurement
   below.
4. **A query's row over an empty R has two wordings,** *R is empty, root is dead* and *R is empty, root is
   alive and owes n instances*, each ending *no state to answer from*. Under describe, `--single` and `-n`
   alike, which print the row today; `as :expect (false)` under it is unchanged, and so is every claim's
   outcome (T4).

**The alternative measured: the rows charged to the listings' 100 lines.** The printer was built that way
first: every instance a listed item, first in the order the listings share the lines in, since the rows
print above R's states.

- On the corpus (`python3 build/s1f.6.26/sweep.py --diff`, 849 files, 2026-11-13,
  `build/s1f.6.26/sweep-proto-shared.txt`): 84 files moved, 82 by the new rows alone, and no summary, claim
  or exit code. The two others are `zebra2-minus-15.ein` and its twin: 46 `owed` rows, and then **four of the
  32 certified states listed where there were seven**, and two classes fewer (`build/s1f.6.26/z-shared.txt`).
  The other three zebra programs print 36 rows between `∅` and `minimal line`.
- No corpus program's root owes a hundred instances, so the cut line is printed on none. On
  `build/s1f.6.26/p/owes120.ein`, whose root owes 120 beside one state that pays them all, the rows take all
  100 lines, and R's listing, one state of one line, reads `;; R as one (region …) is 3 lines, past stdout's
  100: what fits is below, not wrapped in a (region …)` and `;; not shown:  — …`
  (`build/s1f.6.26/out/owes120-shared.txt`): the debts are charged before the answer, and the answer is
  the part cut.

Charged to the 100 lines, a root's debts displace R's own states; capped at four they move nothing but
themselves. The warning's block is the precedent: rows under a summary row, four of a kind and a line for
the rest, outside the 100 lines (`read_out.md` § 5.1, approved 2026-11-03).

**What the table does not cover.** An R that is empty by owing where root is not on the frontier: the
`owing` rows name the frontier's instances, the query row now counts root's, and no row names root's own
(E-44). `--json-summary` holds them on every description, T3's field.

**Found on the way:** [E-43](findings.md), the cut region's last line names nothing where every state was
shown, on the binary at `e9d692e0`.

### T1f.6.26.4 — E-18's claim, asked, done

Filed as [Q-M1f.219](../open_questions.md#q-m1f219--a-querys-false-on-a-program-that-only-owes), **defaulted
to no change, the user's to revise**, and pointed at
[M1g's P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md). Its witnesses: S1f.6.20's two in
scratch, `witness/gap_a2x.ein` (a query's `(false)` holds on a root that only owes) and `gap_a2y.ein` (the
region's `(false)` fails on it), and three tracked programs whose query claims `(false)` and holds on an R
that is empty by owing, `examples/binary_logic/unsatisfiable.ein`, `equivalent.ein` and
`examples/beyond_arith/equation_no_solution.ein`. Nothing a claim means is changed by this stage: the row
above the claim says which empty R is, and `as :expect (false)` under it is what it was.
