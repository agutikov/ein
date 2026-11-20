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

## The entries

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

**The manifest is not changed here.** [S1f.21.20](../s1f.21.20_what_the_first_uses_of_open_found.md) has a task
on the pass's cost, so the costs are taken again on the engine it leaves, and the flags and the recorded costs
move once, then.

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
