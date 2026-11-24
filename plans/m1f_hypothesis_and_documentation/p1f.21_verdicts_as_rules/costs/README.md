# What the phase's builds cost: four engines, one corpus each

[← the phase](../README.md) · [the findings](../findings.md): [V-39](../findings.md#v-39--the-cost-of-the-contradiction-as-rules-is-not-measured),
[V-64](../findings.md#v-64--the-pass-that-writes-a-states-open-facts-costs-at-every-state),
[V-65](../findings.md#v-65--the-gates-checked-build-costs-four-times-the-release-on-one-program)

Taken 2026-11-20 by the coordinator, on AC, the machine idle, no agent running. Nineteen corpus entries, each
entry's declared runs, on a release build of the engine at four commits, **each engine on the corpus and the
standard library of its own commit**:

| point | commit | what it is |
|---|---|---|
| A | `edbe6c3b` | before the contradiction became rules: the kernel's own pair check |
| B | `ddf09492` | after S1f.21.11: the contradiction as rules of `std.contradiction` |
| C | `08ed0ce7` | after S1f.21.12's build: `open` facts, written at every state by the pass; $H$ a union |
| D | `1655f19d` | after S1f.21.18: the `hrule` form removed |

Each cell is run four times, the first discarded where it costs under two seconds, as `utils/corpus_cost.py`
does; a figure is the mean, an entry's the sum over its declared runs. The script is
[`four_engines.py`](four_engines.py), its rows [`four_engines.json`](four_engines.json), run as
`utils/bench_env.sh --cores PT python3 four_engines.py OUT ENTRY…` with the four binaries built by
`cargo build --release -p ein-cli` at each commit. The machine, as `bench_env.sh` printed it:

```text
  cpu             Intel(R) Core(TM) i9-14900HX
  core spec       --cores PT
  governor        powersave (epp balance_performance)
  turbo           on (no_turbo=0)
  power           AC (ACAD online=1); BAT1 Not charging 100 %
  loadavg         0.35 0.62 1.63
```

## The final table: before, after, improved after

The user's word, 2026-11-20: *"plan to build everything recommended, then measure again and get final table with 3
cols: before, after, improved after"*. Taken by the coordinator once every build of the phase was in master, no
agent running, on AC.

| column | commit | what it is |
|---|---|---|
| **before** | `edbe6c3b` | the engine before the contradiction became rules: point A above |
| **after** | `1655f19d` | the engine after the phase's builds and before any of the fixes: point D above |
| **improved after** | `fa2c2914` | the engine with the fixes in: the pass where a state is used and the agenda's tables kept (S1f.21.20), a pair matched by a probe and the index over contexts (S1f.21.21), a resumed state's delta read off its own layers (S1f.21.23) |
| **key** | `24aeebbe` | after [S1f.21.26](../s1f.21.26_the_key_of_a_state_at_its_own_size.md), on the user's answer to item 78: a state's key read above root's mark and sorted at its own size |

Each engine runs the corpus and the standard library of its own commit: the old commits' trees unpacked by `git
archive` into `build/costs/tree-<hash>`, *improved after* among them since this table was first taken (it read the
main checkout then), and the key's the checkout of its branch. The script is [`three_engines.py`](three_engines.py),
its rows [`three_engines.json`](three_engines.json), run as the four engines' was. Taken first 2026-11-20 with three
columns, and **again 2026-11-25 with the fourth**, every column re-taken, no agent running, the machine as
`bench_env.sh` printed it:

```text
  cpu             Intel(R) Core(TM) i9-14900HX
  pinned to       cpu0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 — 16 cpu(s), 8 physical core(s), all P
  core spec       --cores PT
  governor        powersave (epp balance_performance)
  turbo           on (no_turbo=0)
  power           AC (ACAD online=1); BAT1 Full 100 %
  loadavg         1.46 3.69 6.12
```

### Time

Milliseconds, an entry's declared runs summed, each run four times and the first discarded under two seconds.
*Work* is the count of saturations `ein solve` reports, one figure where the four engines agree.

| entry | before | after | improved after | key | key / before | key / improved | work |
|---|---:|---:|---:|---:|---:|---:|---|
| `zebra.ein` | 366 | 885 | 782 | 764 | 2.09× | 0.98× | 120 → 353 |
| `features/04_open.ein` | 515 | 1 608 | 560 | 556 | 1.08× | 0.99× |  |
| `beyond_arith/equation_two_unknowns.ein` | 461 | 774 | 491 | 439 | 0.95× | 0.89× | 1 191 |
| `beyond_arith/send_more_money.ein` | 2 090 | 4 298 | 2 112 | 1 238 | 0.59× | 0.59× | 18 129 |
| `beyond_arith/send_more_money_written.ein` | 1 404 | 2 866 | 1 410 | 819 | 0.58× | 0.58× |  |
| `features/01_not_and_absent.ein` | 2 376 | 3 869 | 2 952 | 2 995 | 1.26× | 1.01× | 65 299 |
| `lattice/06_friends_tables.ein` | 4 847 | 7 212 | 4 960 | 4 881 | 1.01× | 0.98× | 65 299 |
| `lattice/07_two_pairs_two_tables.ein` | 4 168 | 4 993 | 4 289 | 4 263 | 1.02× | 0.99× | 149 |
| `lattice/14_seats_nothing_proposes.ein` | 257 | 5 022 | 4 356 | 4 300 | 16.73× | 0.99× | 69 → 149 |
| `zebra2.ein` | 1 837 | 1 965 | 1 874 | 1 821 | 0.99× | 0.97× | 120 |
| `zebra2-obligations.ein` | 1 733 | 1 804 | 1 752 | 1 691 | 0.98× | 0.97× | 120 |
| `zebra2-hints.ein` | 491 | 506 | 505 | 492 | 1.00× | 0.97× | 62 |
| `saturation/square-fwd/meetings.ein` | 1 596 | 1 872 | 1 594 | 1 588 | 1.00× | 1.00× |  |
| `saturation/square-fwd/houses.ein` | 1 576 | 1 833 | 1 560 | 1 568 | 1.00× | 1.01× |  |
| `branching/06_lookahead_on.ein` | 31 | 62 | 39 | 39 | 1.25× | 1.01× | 133 |
| `branching/07_lookahead_off.ein` | 45 | 57 | 47 | 45 | 1.00× | 0.96× | 133 |
| `arith/rat.ein` | 655 | 652 | 656 | 651 | 0.99× | 0.99× | 1 |
| `saturation/type-exclusivity/pets.ein` | 404 | 632 | 474 | 471 | 1.17× | 0.99× | 11 339 |
| `zebra2-minus-15.ein` | 10 933 | 13 482 | 10 737 | 10 728 | 0.98× | 1.00× | 11 222 |

- **The key at its own size takes 41 to 42 % off the two `send_more_money` programs**, which build 18 129 states of
  1 867 facts at root: 0.59 and 0.58 times their time before the phase; and 11 % off `equation_two_unknowns.ein`.
  Everywhere else it is within 4 % of *improved after*, in either direction.
- **At the same work the engine is at or below where it was**: 0.95 to 1.02 times its time before the phase on
  eleven of the nineteen entries, and 0.58 and 0.59 on the two `send_more_money` programs.
- **Two entries do more work, by the union the user ruled** (Q-M1f.570): `zebra.ein` describes $R$ in 353
  saturations where it took 120, and `14_seats_nothing_proposes.ein`'s oracle enumerates a cube of 16 candidates
  where it had 12. Their time is the union's and no fix's to take back.
- **Three are left above**: `01_not_and_absent.ein` at 1.26, `pets.ein` at 1.17 and `04_open.ein` at 1.08. The
  first two build many small states, 65 299 and 11 339, of a program with few facts, where what a rule instance
  costs at a state is most of a state's cost and the key was little of it: the contradiction's cost that is left,
  which the user accepted. `branching/06_lookahead_on.ein`'s 1.25 is eight milliseconds over six runs.

### Instructions executed

User space, `perf stat -e instructions:u`, one `ein solve` with one worker, pinned to the performance cores, in
millions, the mean of three runs, which agree to within 0.03 %. The counts of [the section
above](#why-the-contradiction-as-rules-costs) were taken at the default count of workers and are not to be set
beside these (V-80). The script is [`instructions.py`](instructions.py), its rows
[`three_engines_instructions.tsv`](three_engines_instructions.tsv), taken 2026-11-25 with the time above: each
engine on its own commit's tree, and **master**, `9994ac94`, the engine just before the key's stage, on the same tree
as the key's, so that *key / master* is that stage's own effect.

| program | before | after | improved after | master | key | key / master | key / before | key / improved |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 3 143 | 5 036 | 3 304 | 3 292 | 2 867 | 0.871 | 0.91× | 0.87× |
| `saturation/type-exclusivity/pets.ein` | 2 629 | 4 432 | 3 509 | 3 517 | 3 495 | 0.994 | 1.33× | 1.00× |
| `features/01_not_and_absent.ein` | 17 118 | 29 391 | 23 900 | 23 928 | 23 776 | 0.994 | 1.39× | 0.99× |
| `beyond_arith/send_more_money.ein` | 10 665 | 19 840 | 10 479 | 10 441 | 5 539 | 0.531 | 0.52× | 0.53× |
| `zebra2.ein` | 1 117 | 1 342 | 1 114 | 1 112 | 1 105 | 0.994 | 0.99× | 0.99× |

- **The key at its own size takes 47 % off `send_more_money.ein`** and 13 % off `equation_two_unknowns.ein`, 0.6 %
  off the other three, which build few facts at a state: `send_more_money.ein` now executes half what it did before
  the phase.
- **`pets.ein` and `01_not_and_absent.ein` count less on every engine than the rows taken 2026-11-20** (in
  `git log`), by about the same amount on each, 420 M and 3 180 M, where the other three programs agree to 0.1 %:
  the count of a program of many small states moves with how it is run, not only with the engine and the input
  (V-107). Rows taken in one session by one script, as these are, stand beside each other.

### The key's profile

Review item 78's profile taken again, before and after the key's stage: [`key_profile.sh`](key_profile.sh), three
`perf record --call-graph lbr -e instructions:u` of `taskset -c 0-15 ein solve FILE --jobs 1` a program and an
engine, `9994ac94` and `24aeebbe`, each reported with `perf report --stdio --children --sort symbol -F
overhead_children,overhead,symbol -g none`; [`key_profile_figures.py`](key_profile_figures.py) reads the shares of
the P-core instructions off each report, its rows [`key_profile.tsv`](key_profile.tsv). *The key* is
`ein_infer::canon::state_key` with what it calls; *the filter* the vector the node function built of the key's facts
root does not hold, its self time; *the read above the mark* the vectors `Snapshot::new_facts_of` builds, with what
they call, both its calls (a resumed state's delta and the key). Taken on AC, the load 2.3 as it started; a share is
of instructions, which the load does not move.

| program | instructions, before → after | the key, before | the filter, before | the read above the mark, after |
|---|---:|---:|---:|---:|
| `send_more_money.ein` | 10 438 M → 5 537 M | 38.8 %, 38.5 %, 38.2 % | 9.4 %, 9.1 %, 9.6 % | 0.5 %, 0.3 %, 0.8 % |
| `equation_two_unknowns.ein` | 3 290 M → 2 860 M | 10.1 %, 9.9 %, 10.7 % | 2.8 %, 3.0 %, 2.2 % | 0.2 %, 0.2 %, 0.3 % |

The key and its filter were 47.5 to 48.2 % of `send_more_money.ein` and 12.8 to 13.0 % of
`equation_two_unknowns.ein`, as item 78 had them; after, the key has no symbol of its own, and the sort of the few
facts a state adds is no symbol on `send_more_money.ein` and 0.2 to 0.3 % on `equation_two_unknowns.ein`.

## The four engines: the entries

Milliseconds. *Work* is the count of saturations `ein solve` reports, one figure where the four engines agree.

| entry | A | B | C | D | B / A | C / B | D / A | work |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `zebra.ein` | 360 | 382 | 866 | 880 | 1.06× | 2.27× | 2.45× | 120 → 353 |
| `features/04_open.ein` | 544 | 758 | 1606 | 1614 | 1.39× | 2.12× | 2.97× | no `solve` run |
| `beyond_arith/equation_two_unknowns.ein` | 459 | 732 | 759 | 771 | 1.59× | 1.04× | 1.68× | 1191 |
| `beyond_arith/send_more_money.ein` | 2093 | 2858 | 4227 | 4288 | 1.37× | 1.48× | 2.05× | 18129 |
| `beyond_arith/send_more_money_written.ein` | 1397 | 1919 | 2821 | 2854 | 1.37× | 1.47× | 2.04× | no `solve` run |
| `features/01_not_and_absent.ein` | 2551 | 3204 | 4023 | 4023 | 1.26× | 1.26× | 1.58× | 65299 |
| `lattice/06_friends_tables.ein` | 4832 | 5595 | 7171 | 7209 | 1.16× | 1.28× | 1.49× | 65299 |
| `lattice/07_two_pairs_two_tables.ein` | 4188 | 4478 | 4991 | 5023 | 1.07× | 1.11× | 1.20× | 149 |
| `lattice/14_seats_nothing_proposes.ein` | 251 | 293 | 5029 | 5058 | 1.17× | 17.19× | 20.15× | 69 → 149 |
| `zebra2.ein` | 1839 | 1910 | 1998 | 1962 | 1.04× | 1.05× | 1.07× | 120 |
| `zebra2-obligations.ein` | 1740 | 1781 | 1816 | 1823 | 1.02× | 1.02× | 1.05× | 120 |
| `zebra2-hints.ein` | 502 | 513 | 509 | 521 | 1.02× | 0.99× | 1.04× | 62 |
| `saturation/square-fwd/meetings.ein` | 1680 | 1679 | 1894 | 1927 | 1.00× | 1.13× | 1.15× | no `solve` run |
| `saturation/square-fwd/houses.ein` | 1586 | 1604 | 1819 | 1871 | 1.01× | 1.13× | 1.18× | no `solve` run |
| `branching/06_lookahead_on.ein` | 30 | 51 | 54 | 47 | 1.73× | 1.07× | 1.59× | 133 |
| `branching/07_lookahead_off.ein` | 44 | 47 | 55 | 50 | 1.08× | 1.17× | 1.14× | 133 |
| `arith/rat.ein` | 655 | 673 | 683 | 688 | 1.03× | 1.01× | 1.05× | 1 |
| `saturation/type-exclusivity/pets.ein` | 411 | 526 | 643 | 644 | 1.28× | 1.22× | 1.57× | 11339 |
| `zebra2-minus-15.ein` | 10985 | 12610 | 13501 | 13511 | 1.15× | 1.07× | 1.23× | 11222 |

## What each step costs

- **A to B, the contradiction as rules (V-39).** The work is the same on every entry, saturation for saturation.
  The time is 1.0 to 1.06 times on the zebras and the squares, 1.15 to 1.4 on the programs that search a wide
  cube, and 1.6 on `equation_two_unknowns.ein`, the calculator under a search.
- **B to C, the `open` facts.** Two things, apart:
  - **The pass, at the same work (V-64)**: 1.02 to 1.13 times on the zebras and the squares, 1.2 to 1.3 on
    `01_not_and_absent.ein`, `06_friends_tables.ein` and `pets.ein`, 1.5 on the two `send_more_money` programs. A
    profile of `send_more_money.ein` at D has `Kb::write_open` first, at 13 % of the samples, and
    `Saturator::end_step`, the drop of an `Engine` and the allocator's free among the next five, where B's profile
    has none of them.
  - **The union, more work**: `zebra.ein` describes $R$ in 353 saturations where it took 120, and costs 2.3 times;
    `14_seats_nothing_proposes.ein`'s oracle run enumerates a cube of 16 candidates where it had 12, and costs 23
    times.
  - `04_open.ein` has no `solve` run; its lattice render costs 2.1 times.
- **C to D, the `hrule` form removed.** Nothing: every entry within two percent.

## Why the contradiction as rules costs

The user asked, 2026-11-20: *"contradiction as rules - investigate and explain why it costs, because I see only one
more step added rule firing from (false) to :ein inconsistent. Why the rule match P and not P is slower than
previously existing built-in detection?"*

**The firing chain is one step longer, and that step costs nothing that can be measured.** A contradiction is
rare, and the cost is not in firing one. It is in what the engine does at every state and for every fact so that a
rule *could* fire, where the kernel's check was one probe of a hash table.

**Before** (`ein_infer::contradiction::contradicts` at A): as each derived fact lands, *is its negation held?*,
one probe. **After**: the pair is a rule, `clash-N`, with one instance for each declared relation, and each
instance is a compiled plan the matcher runs.

Instructions executed, user space, `perf stat`, one `ein solve`, the same saturations and the same facts on both
engines:

| program | A | B | B / A | saturations | facts |
|---|---:|---:|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 3.61 G | 6.67 G | 1.85× | 1 191 | 80 701 |
| `saturation/type-exclusivity/pets.ein` | 3.61 G | 4.94 G | 1.37× | 11 339 | 76 683 |
| `features/01_not_and_absent.ein` | 23.65 G | 32.09 G | 1.36× | 65 299 | 390 225 |
| `beyond_arith/send_more_money.ein` | 11.84 G | 14.06 G | 1.19× | 18 129 | 161 086 |
| `zebra2.ein` | 1.38 G | 1.30 G | 0.94× | 120 | 5 409, 5 411 |

**The rule instances at root** (`root.plans` of the summary): `pets.ein` 2 before and 13 after,
`send_more_money.ein` 20 and 63, `equation_two_unknowns.ein` 153 and 338. Each declared relation takes two
instances, `clash-0` and the rule of its count (V-37), and an imported module's declarations take theirs.

**Where the added instructions go**, by a profile with call stacks (`perf record --call-graph lbr`, one worker),
each function's cost with what it calls, in millions of instructions added from A to B:

| part | `pets` | `equation_two_unknowns` | `send_more_money` |
|---|---:|---:|---:|
| all that is added | 309 | 578 | 1 078 |
| **1. the agenda's tables, built again at every state** (`Saturator::resume`'s `refresh_first` and `refresh_at_once`) | 132, 43 % | 214, 37 % | 451, 42 % |
| **2. the rules that end a state, matched ahead of the others after each step** (`end_step`) | 11, 4 % | 144, 25 % | 120, 11 % |
| **3. every new fact run through its relation's instances, and every pass over more instances** (`enqueue_pass`, `Matcher::run_seeded`) | 111, 36 % | 177, 31 % | 119, 11 % |
| the rest: the module parsed once a run, a larger engine copied for each state, a larger key of a state | 55, 18 % | 43, 7 % | 388, 36 % |

1. **The tables.** The agenda fires first a rule that ends a state or refutes a pair, then what leads to one
   (Q-M1f.313). Which rules those are was the kernel's to know; it is now read off the compiled rules and the
   negations the state holds (`contradiction::Pairs::of`, `monotone::writes_of`, `monotone::leads_to_an_end`). A
   resumed saturation starts with the tables empty and builds them again, over every plan, at every state a search
   builds, though they are the same at every state but for what the state's own negations add. **This is no cost
   of the rule: it is the tables not being kept**, and it is the largest part.
2. **Ahead of the others.** So that a contradiction ends a state before another rule fires, the rules that fire at
   once are matched against the new facts after each step. Before, there was one such rule, `std.core`'s, over
   `(false)`; now there are two for each declared relation, and their index is built again at every state too.
3. **A match where there was a probe.** A new fact `(R a b)` is handed to the instances of `R`: bindings built,
   the fact's negation made ground and looked up, and what is found given a key and queued. That is the matcher's
   ordinary work, many times one probe, done for every fact though it finds a pair almost never. **This part is
   what a contradiction as a rule costs**, and its size follows the count of instances.

`zebra2.ein` executes no more than before: it builds 120 states, and the three parts are paid by the state and by
the fact.

**What can go with no change to any answer or any printed line**: part 1 whole, and part 2's index, by keeping
the tables with the snapshot a state resumes from. [S1f.21.20](../s1f.21.20_what_the_first_uses_of_open_found.md)
has it as a task. **Part 3** is [S1f.21.21](../s1f.21.21_the_contradictions_cost.md)'s, on the user's ruling of Q-M1f.657: a
rule of the pair's shape matched by one probe, and an index over contexts, analysed below.

## An index over contexts, analysed

The user, 2026-11-20, on the last option above: *"I hope we already have index of all facts by relation. Here we
have to improve this index by adding index over contexts. Analyze if it would work"*.

**What the engine has.** Facts by relation (`Kb::facts_of`). A participation index, `SlotKey { rel, slot, inner,
value }`: the facts of `rel` with a name at an argument, or at an argument **of the fact** in that argument, one
level in. So `(not (R ?a ?b))` with `?a` bound is a bucket lookup. It keys the nested fact's arguments and **not
its relation**, and no key has a fact as its value: `(not (R ?a ?b))` with nothing bound walks every negation the
state holds. And the saturator's index of rule instances, `pos_index`, is by the premise's relation alone: a new
`(not (S x y))` is handed to every instance with a `not` premise, the clash instances of every relation among
them. Last, `Kb::is_negated`, a set of the facts a state holds the negation of: kept up to date, and read by
nothing since the kernel's check went (V-36).

**Read as (context, relation) → facts**: the nested fact's relation becomes one more key of the participation
index, and `pos_index` gains the same key. It works. It only narrows what is offered, in the extent's order, so no
firing moves. It costs one more key for each fact that holds a fact. What it saves:

- a new negation goes to the instances of its own relation, and to rules whose premise leaves the relation a
  variable, and not to every other relation's;
- a rule's `(not (R …))` premise with nothing bound walks the negations of `R` and not all of them, which is
  every program's gain and not the contradiction's.

**It does not recover the cost measured above.** Facts the firings derive, by `--events`, one worker:

| program | rule instances | positive facts | negations | clash instances |
|---|---:|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 338 | 77 884 | 0 | 185 |
| `beyond_arith/send_more_money.ein` | 63 | 50 046 | 0 | 43 |
| `features/01_not_and_absent.ein` | 14 | 9 | 0 | 4 |
| `saturation/type-exclusivity/pets.ein` | 13 | 4 | 8 208 | 4 |
| `zebra2.ein` | 160 | 1 689 | 3 842 | 18 |

Where the contradiction costs most, no negation is derived: every new fact is positive, goes to the two instances
of its own relation and to no other, and what is paid is the matcher's work for each. `pets.ein` derives
negations and has two relations. `zebra2.ein` is the shape the index is for, 3 842 negations each tried against
18 instances with 16 of them of another relation, and it is the program that shows no added cost.

**Read as (context, fact) → held**: *is this fact held under `not`?*, one probe by the fact's id, with no term
built. That is `Kb::is_negated`, which exists. Used to match a rule of the pair's shape, a fact beside one holding
it as its one argument, which `contradiction::Pairs` already tells by shape and by no name: a new fact costs one
probe, and the rule's instance is queued only where the probe hits. The rule still fires as a rule, with its
premises and its `:why`. This is the reading that would take parts 2 and 3 down to about what the kernel's check
cost; it is a change to how the saturator matches such rules, and it must keep every firing where it is.
**Ruled by the user 2026-11-20: built**, with the reading above it, by [S1f.21.21](../s1f.21.21_the_contradictions_cost.md).

## The runs behind the entries that moved most

| entry | run | A | B | C | D |
|---|---|---:|---:|---:|---:|
| `zebra.ein` | `solve` | 100 | 112 | 318 | 324 |
| `zebra.ein` | `render lattice` | 181 | 184 | 458 | 467 |
| `features/04_open.ein` | `render lattice` | 523 | 740 | 1584 | 1594 |
| `beyond_arith/send_more_money.ein` | `solve` | 700 | 958 | 1413 | 1433 |
| `beyond_arith/send_more_money.ein` | `solve --single` | 694 | 955 | 1402 | 1423 |
| `beyond_arith/send_more_money.ein` | `test` | 699 | 945 | 1412 | 1433 |
| `lattice/14_seats_nothing_proposes.ein` | `oracle --dump {out}/region` | 192 | 211 | 4952 | 4986 |

## Against the manifest

`utils/bench_env.sh --cores PT python3 utils/corpus_cost.py --check`, taken twice at D, finds:

| entry | recorded | measured, twice | |
|---|---:|---:|---|
| `features/04_open.ein` | not slow | 1 684, 1 635 ms | over the line: slow |
| `zebra.ein` | 500 ms, not slow | 1 010, 1 000 ms | on the line |
| `beyond_arith/send_more_money.ein` | 2 150 ms | 4 406, 4 408 ms | past twice its record |
| `beyond_arith/send_more_money_written.ein` | 1 430 ms | 2 937, 2 938 ms | past twice its record |
| `branching/06_lookahead_on.ein`, `07_lookahead_off.ein` | 40, 30 ms | 105, 104 and 70, 98 ms | past twice their record; far under the line |

**The manifest was not changed then.** On the engine of the final table's last column the costs were taken
again: no flag moves, the slow set is the same sixteen entries, `zebra.ein` at 889 ms and `04_open.ein` at 609 ms
under the line, and twenty-one recorded costs are written to what was measured.

## The gate's build

The per-commit sweep runs the gate profile's binary, release codegen with the debug checks on. One `solve` of
three programs, release against gate, at D:

| program | release | gate | |
|---|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 398 ms | 1 513 ms | 3.8× |
| `zebra.ein` | 323 ms | 478 ms | 1.5× |
| `features/04_open.ein` | 5 220 ms | 5 700 ms | 1.1× |

`equation_two_unknowns.ein` declares two runs, so its cells cost three seconds in the per-commit sweep against a
ceiling of four, which a loaded machine passes (V-60, V-65).
