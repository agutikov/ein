# S1f.6.26 — A read-out row for what root owes

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. **Done 2026-11-13:** [the record](#record-2026-11-13).
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

### T1f.6.26.2 — the printer, done

**Built** (`ein-render`'s `read_out.rs`, `ein-infer`'s `explore.rs`).

- `Row::Owed`, label `owed`, a block's row, in the `describe` table after `∅`. `owed_rows` prints one an
  instance of `Described::root.owed`, the strings every route already kept for root's node, under the `∅`
  row where it reads *not in R — root owes n*; past `OWED_ROWS`, four, one line at the values' column, *… n
  more instances: `--dump-answer DIR` writes every one, to DIR/answer.txt*, and no cut where the read-out is
  written uncut. `ein oracle`'s line names `ein solve FILE --dump-answer DIR`, as its other cut lines do.
  `--count` is not given the rows.
- `empty_words`: a query's `over` row over an empty R reads *R is empty, root is dead: no state to answer
  from* or *R is empty, root is alive and owes n instances: no state to answer from*. A mode holds no
  description, so `Answer` gained `root_owes`, the count `region::prepare` already takes for every route
  (`Prepared::root_owes`, M1f S1f.5.6): no saturation and no tally is added. An alive root that owes
  nothing is in R, so no run reaches a third wording.
- No core is taken: an instance's text is its obligation rule's `:why`, which the node function renders
  where it tallies what a node owes.

**The pins** (`ein-cli/tests/read_out.rs`):

- `the_read_out_names_what_root_owes`: a program written in the test, 120 things that each owe a mark and
  one supposition that marks them all. Stdout has the `∅` row, four `owed` rows in report order and the line
  for the other 116, then `minimal line`; R is still printed as its `(region …)`, so the rows took none of
  the listings' lines; `answer.txt` has all 120 and no such line; the trace's headline is stdout's rows;
  `region.root_owed` is the 120 strings; `ein oracle` prints the same four and names the command that
  writes the rest; `--count` prints the `∅` row and no `owed` row; each of `--single`, `--minimal`,
  `--maximal` and `-n 2` prints neither row and writes `root_owed: null`.
- `a_query_over_an_empty_r_says_which_empty`: the row under describe, `--single` and `-n 2` on two programs
  whose root owes (`tests/stdlib/slots/03_fill.ein`, 10 instances, and
  `tests/stdlib/closure/03_closed_and_owing.ein`, 1) and two whose root is dead
  (`examples/ein-bugs/zebra2-bad.ein`, `tests/stdlib/algebra/09_irreflexive_violated.ein`), `as :expect
  (false)` under each; and on `examples/binary_logic/unsatisfiable.ein` the query's `:expect (false)` holds
  as it did, the solve claim with it.
- `an_owing_node_is_not_a_state`, which held the `∅` row of `features/13_mixed_solution_and_open.ein`, holds
  its `owed` row and the summary's string too.
- `summary_properties.rs`' `the_description_is_coherent_on_every_corpus_cell`, two identities more, on every
  corpus cell: `region.root_owed` names exactly `region.root_owes` instances and is `null` with it, and a
  root in R owes nothing.

**The pages, in the same commit.** [`read_out.md`](../../../docs/kernel/inference/read_out.md): the `owed`
row in § 2's table, its summary column `region.root_owed[k]`, and a paragraph under the table on what is
printed where, the cut and where the rest is; § 7's `over` row and a paragraph on the two wordings and the
claim under them; § 6's sentence on what is never cut; § 11.2's sentence on a mode's `null`.
[`read_out_examples.md`](../../../docs/kernel/inference/read_out_examples.md), re-written by
`utils/read_out_examples.py`: 69 lines came and 17 went (`git diff --numstat`), the `owed` rows of X4, X5
and X10 (four and the line on stdout, all 36 in X10's `answer.txt` block), the `over` rows of X6, X7 and
X16, and the script's prose for them. `answer_dump.md`: `answer.txt` holds every `owed` row.
`read_out_reference.rs` holds the page to the printer, six tests passing: X5 and X10 print the new label,
and its fourth test holds each printed `owed` row to `region.root_owed[k]`.

**The generator did not run** ([E-45](findings.md)). `utils/read_out_examples.py` writes one program that
is no corpus file, X4's variant of `lattice/07_two_pairs_two_tables.ein` with a relation and a rule added,
neither with a `:why`, which every load has required since S1f.5.14's flip (`4b231abd`, 2026-11-10): the
variant's run printed a load error and the script raised at it, `ValueError: substring not found`, on the
binary at `e9d692e0` too. The relation and the rule have a `:why` now, in the script and so in the page's
block. The page's other blocks were current, `read_out_examples.rs` holding each to the binary: the
re-write moved nothing but the rows above.

### T1f.6.26.3 — the other surfaces, done

| surface | what it carries | held by |
|---|---|---|
| `--trace FILE.md` | its headline is the read-out's rows above the listing, from the same printer, so it names the instances stdout names, four and the line | `the_read_out_names_what_root_owes`; the 36 `region-trace` digests |
| `--json-summary` | **`region.root_owed`, new**: every instance root owes, through its rule's `:why`, on every description, `[]` where root owes none, `null` under a mode as `region.root_owes` is | the same test; `read_out_reference.rs`' third and fourth tests |
| `--dump-answer DIR` | `answer.txt` has every `owed` row; `answer.json` is the summary, and `index.json` its `region` block | the same test; `region_answer.rs` |
| `--events`' `region` event | the summary's `region` block, so the field with it | by construction (`modes::emit_region`) |
| `--single`, `--minimal`, `--maximal`, `-n N` | no `∅` row, so no `owed` row; the query's `over` row under `--single` and `-n` | the two tests |
| `ein oracle` | describe's rows over the cube: the `owed` rows and the `over` row | `the_read_out_names_what_root_owes` |
| `ein test -v` | unchanged: its line for a query over an empty R, *0 classes over 0 states*, names neither kind | not in the stage |

**The check the task asks for.** The summary did not carry root's instances where R is not empty.
`region.frontier` is `null` there, and a hole is a node between two members of R, which ∅ never is, so root
is not among `region.holes.minimal` either: the summary held `region.root_owes`, the count. So the field was
added, in `ein-summary/2` as it stands: a key on every run, as `region.core_status` and
`root.hypgen.types_as_objects` were added. On an R that is empty by owing with root off the frontier, four
corpus programs, it is the one place root's own instances are ([E-44](findings.md)).

### T1f.6.26.5 — the goldens, done

**Counted before the commit, and no other golden moved.** `EIN_BLESS=1 cargo test -p ein-render --test
corpus_shapes` wrote the manifest, and `python3 build/s1f.6.26/shapes_moved.py` compares it with
`e9d692e0`'s (2026-11-13): **2 726 of 24 269 digests moved, none new and none gone**
(`build/s1f.6.26/t5-moved.txt`).

| kind | moved | by how many lines | why |
|---|---:|---|---|
| `cores[describe]` | 502 | 443 by +1; 59 by n + 2, n from 1 to 46 | the summary's `region` block: `"root_owed": []`, or the n instances a root owes |
| `cores[single]`, `[n2]`, `[minimal]`, `[maximal]` | 502 each | +1 | `"root_owed": null` under a mode |
| `read-out[describe]` | 84 | 36 by +1 to +5; 48 by 0 | the `owed` rows of the 36 programs of T1's census; the `over` row of 48 programs whose R is empty and that ask a query |
| `read-out[single]`, `read-out[n2]` | 48 each | 0 | the `over` row, the same 48 |
| `region-trace` | 36 | +1 to +5 | the headline's `owed` rows |

502 is every program of the sweep that reaches a region, and 59 the programs whose root is alive and
owes. No digest of `read-out[minimal]`, `read-out[maximal]`, `states`, `region-lattice`, `oracle` or any
other kind moved. `read_out_examples.md` is the read-out's golden page, above. `corpus_exits.txt`,
`help_shape.txt` and the trace goldens did not move.

**The sweep** (`python3 build/s1f.6.26/sweep.py --diff`, 2026-11-13, `build/s1f.6.26/sweep-t2.txt`): the
release binary at `e9d692e0` against the tree's, over the 849 `.ein` files `e9d692e0` tracks; per file
`ein saturate --dump`, `ein solve`, `ein solve --single`, `ein solve -n 2` and `ein test`, stdout, stderr
and exit, and the two summaries of a bare solve compared with `region.root_owed` taken out of the new one.

- **84 files moved, each by the new rows alone:** 36 gained `owed` rows under a bare solve (six of them the
  line for the rest), and 48 had the `over` row reworded, under a bare solve, `--single` and `-n 2`.
- **No summary differs** once the new field is taken out: R (`k_s`, `k_c`, every state, the frontier), N
  (`region.nogoods`), H (`region.universe`, `root.hypgen`), every claim's outcome (`queries[].expect`,
  `claim`) and every other field are the same on all 511 programs that write one.
- **No exit code moved,** and no line of `ein saturate --dump` or `ein test`.

`lever_invariance.rs` holds the read-out under every lever, unchanged and passing.

### T1f.6.26.4 — E-18's claim, asked, done

Filed as [Q-M1f.219](../open_questions.md#q-m1f219--a-querys-false-on-a-program-that-only-owes), **defaulted
to no change, the user's to revise**, and pointed at
[M1g's P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md). Its witnesses: S1f.6.20's two in
scratch, `witness/gap_a2x.ein` (a query's `(false)` holds on a root that only owes) and `gap_a2y.ein` (the
region's `(false)` fails on it), and three tracked programs whose query claims `(false)` and holds on an R
that is empty by owing, `examples/binary_logic/unsatisfiable.ein`, `equivalent.ein` and
`examples/beyond_arith/equation_no_solution.ein`. Nothing a claim means is changed by this stage: the row
above the claim says which empty R is, and `as :expect (false)` under it is what it was.

### T1f.6.26.6 — the pages and the rows, done

- [`findings.md`](findings.md): **E-19 closed**; **E-18 marked *the row built, the claim asked***; E-43, E-44
  and E-45 filed, shown and not fixed, but for E-45's script, which the stage had to run.
- The phase [README](README.md)'s row.
- **`AGENTS.md` is not edited.** This file's T6 lists it; the user ruled on 2026-11-13 that it holds
  standing rules, locations and commands and no stage's history, and none of the three changed: a new row
  of the read-out is `read_out.md`'s.
- [`word_problems.md`](word_problems.md) § 3.5 is not edited: its pointer is the next stage's that edits the
  page, as the task has it.

### The stage, closed

**The commits**, in order:

| commit | what |
|---|---|
| `38323f06` | T1 and T4: the rows shown first, and Q-M1f.219 and Q-M1f.220 filed. No source |
| `b9e03aa4` | T2, T3 and T5: the printer, `region.root_owed`, the pages, the pins, the goldens |
| this section's | T6: the findings, the phase's row, this record |

**The acceptance.**

- *On E-19's witnesses the read-out names what root owes and why, with R not empty*: T1's table, on
  `p/gap_assume_ha.ein` and `p/boxes13.ein`; pinned on a program of the test's and on
  `features/13_mixed_solution_and_open.ein`, the witnesses being scratch.
- *A query's row over an empty R says whether root owes or is dead*: the two wordings, pinned on four
  corpus programs under three questions.
- *R, N, H, every claim's outcome and every exit code are the same on every corpus program*: the sweep, 849
  files, 84 moved and each by the new rows alone.
- *`read_out_reference.rs` holds the page to the printer*: six tests passing.

**Defaults taken**, each the user's to revise: Q-M1f.219, a query's `(false)` on a program that only owes
is not changed; Q-M1f.220, the form of the rows: the label `owed`, the block under `∅`, the cut at four
with a line for the rest and outside the listings' 100 lines, `--count` printing none, and the two wordings
of the query's row. **The cut departs from this file's T1**, which has the rows share the 100 lines; T1's
record has the measurement it rests on.

**Found outside the tasks**: E-43, a cut region's last line names nothing where every state was shown;
E-44, on an R that is empty by owing root's own debts are on no row where root is not on the frontier;
E-45, the script that writes the read-out's examples page had stopped running and nothing runs it (its
variant fixed here, the missing check not).

**What moved of what already existed**, each intended: the read-out of 36 programs by their `owed` rows,
and of 48 by the wording of a query's row; the `--trace` headline of the 36; every summary, by one key;
`corpus_shapes.md5`'s 2 726 digests for those; the examples page. No verdict word, claim's outcome, exit
code, state, count or core moved.

**Not built, as this file rules**: the N row of an R that is empty by owing, a core for an open state, and
any change to what `Open` or `(false)` mean.

**Not run by this stage**: `./run_tests.sh`, `./run_tests.sh --nightly` and `utils/ci_container.sh`. The
first two run on `b9e03aa4` in detached worktrees, their outcome written under this line by the records
commit. No DOT writer, review bundle script or CI install changed; the bundle's pages carry the read-out
as the binary prints it, so they carry the new rows, and its scripts read none of the rows that changed
(`utils/review/run.py` reads `-H`'s form and the summary).
