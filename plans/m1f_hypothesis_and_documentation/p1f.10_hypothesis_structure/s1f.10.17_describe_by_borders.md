# S1f.10.17 — Describe by borders

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-16, on the user's ruling of Q-M1f.311 that day. Not started.
**Depends on:** [S1f.10.3](s1f.10.3_the_restricted_join.md) (the memory `known_dead`, L8),
[S1f.10.5](s1f.10.5_ordering.md) (the tree's order), [S1f.10.14](s1f.10.14_exactly_one_spent.md) (L9; Q-M1f.311's
numbers), [S1f.10.13](s1f.10.13_the_price_list.md) (the rule for *go*).

## The ruling

[Q-M1f.311](../open_questions.md#q-m1f311--a-describe-the-budget-would-stop-answer-what-the-tree-can-reach-instead),
option (e), put by the main session on 2026-11-16 in answer to the user's *"how describe could work the same way as
--single but when there are potentially R contains many nodes?"* and *"what is more generic approach, independent
from k_c = k_s?"*. The user, the same day:

> YES! a describe that visits each predicate's borders instead of every alive state - lets pick this variant

## The ruling on the measurement (2026-11-16)

T2 measured the whole border route *no* for describe on the corpus, and the cheap part, the tree and the classes,
exact on R where the state route stops. The user, shown the table:

> S1f.10.17 - 2 modes: state route (existing) and tree plus classes, second by default, cmdline option to force one
> or another, verbose warning about budget so user knows about forecast and options.

So **T3 is redefined**: describe and `--count` have two routes, the state route as it is and **the tree and the
classes**, which is the default; a command-line option forces either; where a route's forecast passes the budget the
run says so, with the forecast and the options. N by dualize and advance, the part that did not pay, is not a route.
This ruling is the user's, and it supersedes the plan's *no lever chooses it*: the option chooses a route and never
R.

## Context

Describe takes the state route (`states.rs`). It visits every alive state, each once with its one-up extensions,
and reads every row of its description off what it met (`described.rs`: the states with their classes, generators
and line flags, `k_c`, the minimal commitments, N, dead alone, the shape, and an empty R's owing frontier). That is
correct and costs every alive state.

Where most alive states owe, nearly all of the work is states that are not in R. Q-M1f.311's record, S1f.10.14's
T4, measured it:

| program | describe | of it, on alive owing states | what the tree reaches |
|---|---:|---:|---|
| `send_more_money.ein` | stopped at the budget, 336 101 saturations | 98.3 % | `--single` proves R is one state in 18 030 |
| `zebra2-minus-15.ein` | stopped at the budget, 7 261 | 76.2 % | `-n 1000000` lists all 32 states in 104 |

## The design ruled, to be measured before it is built

Every quantity describe reports is a **border** of a monotone predicate over the cube, or a count over one. On a
monotone program:

| what describe reports | the predicate and its border | how it is found |
|---|---|---|
| R's **states** | *in R*, up-closed among the alive | the tree exhausted (L5, L9), then the alive states above its leaves (L2, L3): `-n N`'s route with no N |
| per state, **its class** `{C ⊆ s : cl(C) = s}`: its commitments, its **generators**, `k_c` | *generates s*, an up-set inside `2^s` because cl is monotone; its minimal elements are the minimal generators, the maximal elements outside it are the maximal closed proper subsets `m_j` of `s` | dualize and advance inside `2^s` with cl as the oracle, saturating subsets of `s` only; `|class(s)| = 2^|s| − |⋃_j 2^{m_j}|` |
| the **minimal line** | R's minimal commitments: the alive minimal discharged sets | duality against the maximal owing states, `--minimal`'s route |
| the **maximal line** | R's maximal states | the climb and dualize and advance, `--maximal`'s route |
| **N** | the minimal dead sets: the border of *alive* | dualize and advance between the maximal alive sets and the minimal dead sets |
| **dead alone** | the singletons of N | the singles |
| an empty R's **owing frontier** | the maximal owing states | the same duality as the minimal line |
| the **shape** | components and sub-cubes of R | read off R's states and lines |

Each border is found at a cost polynomial in its size and its dual's, quasi-polynomial at worst (Fredman–Khachiyan),
and not in the number of alive states. **That is the bet, and it is not measured**: the global borders, N and the
maximal owing states, can themselves be large, and on a program where L5 and L9 license no instance the tree
branches on every extension and is the state route again.

## Tasks

1. **T1f.10.17.1 — the inventory.** Every field of `Described` and every surface that reads it (the read-out, the
   summary, `--dump-answer`, `--dump-lattice`, `--count`, the region claim's check, `ein test`), mapped to the
   border it needs and the route that finds it, checked against the code. The goldens that will move named here,
   in a commit of its own, before any route changes: the `work` and `route` rows move, and with them
   `read_out_examples.md`, the shape digests, `cost_ms` and the slow set. A golden that moves and is not on the
   list is a stop.
2. **T1f.10.17.2 — the borders measured.** For every corpus program that searches: the size of each border (R's
   states, each class's `m_j` and generators, N, the maximal alive sets, the maximal owing states) and what finding
   each costs in saturations and facts beyond root, against the state route's describe, in an example beside
   `price_list`. On every program within the oracle's limit the borders are read off the cube as well, and the two
   readings agree. Within the default budget; a cell the budget stops is recorded as stopped, with what it had
   spent. **The rule for *go* is S1f.10.13's**, read as its *Defaults taken* read it, applied to describe and to
   `--count`, written in this file before the table is taken.
3. **T1f.10.17.3 — the route built**, where T2 says *go*: a border route in a module of its own, which `describe`
   and `--count` take on a monotone program, every node through the node function and the memory of what is known
   dead. Where it is *no* on some programs and *go* on the corpus, the stage says how describe chooses between the
   two routes, by something the run can see before it has the answer, and **no lever chooses it**. The state route
   stays, behind an internal switch on the calling thread (as `known_dead`'s with and without), as the reference.
   If T2 says *no* for describe on the corpus, **stop and report** with the table: the user picked this variant,
   and a *no* is theirs to see before anything else is decided.
4. **T1f.10.17.4 — held.** Every field of the description identical between the border route and the state route on
   every corpus program the state route finishes; both held to the oracle where it reaches (`ein-infer/tests/explore.rs`);
   the claims (`claim_routes.rs`); `lever_invariance`, `route_invariance`, `answer_round_trip`; a run the budget
   stops says what it established and no more, its counts lower bounds. A sweep through `ein_corpus::pool::map`,
   its slow half on a nightly tier within five minutes a shard.
5. **T1f.10.17.5 — the pages and the record.** [`read_out.md`](../../../docs/kernel/inference/read_out.md)'s `route` and
   `work` rows, `read_out_examples.md` regenerated,
   [`finding_r.md`](../../../docs/kernel/inference/finding_r.md) § 6 and the borders' formulas,
   `implementation.md`'s rows; the record here with every number's command; Q-M1f.311's *Built* paragraph.

## Acceptance

- The rule for *go* is written here before T2's table is taken.
- **Every field of the description is identical** to the state route's on every corpus program both finish, and to
  the oracle's where the oracle reaches.
- The work per program for describe and `--count` is recorded against the state route's, the programs made slower
  named, and the rule applied with the borders' cost included.
- Whether `send_more_money.ein`'s and `zebra2-minus-15.ein`'s describe now finish within the default budget is
  recorded, with what each costs.
- No lever. No relation, type or module read by name. No answer changes.
- `./run_tests.sh` and `./run_tests.sh --nightly` (in the runner's container) green on the batch's last commit,
  since a route changed.

## What it must not do

- **Not drop a field of the description.** A field the border route cannot find within the budget is reported as
  what was established, a lower bound or *not computed*, as a stopped run's are today.
- **Not take the border route on a flagged program.** A flagged program takes the cube, as every mode does.
- **Not count a saved saturation without the cost that bought it.**
- **Not run a test past the default budget.**
