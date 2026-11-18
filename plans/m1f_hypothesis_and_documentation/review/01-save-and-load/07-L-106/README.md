# L-106 — `answer_round_trip`'s nightly shards come close to the five minutes

## 1. The question

`ein-cli/tests/answer_round_trip.rs` holds every state file `ein solve --dump-answer` writes to the state it saved: for
each corpus program it dumps the answer, then loads each state file twice and checks it three ways (the first load is
the state saved; save → load → save is byte for byte; load → save → load is dump for dump). Per commit it leaves out
the five `heavy_dump` programs, whose dumps write 40 000 state files or more; the nightly runs them, in four shards
(`EIN_TEST_SHARD=k/4`), each a job that must stay within five minutes on the runner (Q-M1f.102). **A program's whole
round trip is one unit of the test's worker pool**, so a shard cannot finish before its heaviest program does. On the
measured run the shards took 180, 194, 55 and 211 s here; at the runner's 1.13 to 1.34 times (Q-M1f.103), up to about
283 s. The call: leave it; deal a heavy program's files to the pool in parts; or run the second load on a share of a
heavy program's files.

Recorded: [`findings.md`](../../../findings.md#the-release-voice-where-the-check-does-not-read-and-the-gate), row
**L-106**.

## 2. What is so today

**The measurements** were taken before this review, at `a5de14a1` (an ancestor of HEAD, on the save/load branch), on
four P-core threads (`--cores PT:4`: two physical cores), `EIN_TEST_JOBS=4`, `EIN_CORPUS_SLOW=1`, on AC; copied in raw:
[`runs/s1.log`](runs/s1.log) to [`runs/s4.log`](runs/s4.log) (each with `bench_env.sh`'s fingerprint and the test's
own lines) and [`runs/units1.tsv`](runs/units1.tsv) to [`runs/units4.tsv`](runs/units4.tsv) (`EIN_TEST_UNITS`: one
line per program, its unit's milliseconds). **No `cargo test` was run for this review.** Every table below is
[`runs/shard-tables.md`](runs/shard-tables.md), written by [`tools/shard_tables.py`](tools/shard_tables.py) from them.

| shard | wall here (s) | × 1.13 | × 1.34 | units summed (s) | summed ÷ 4 (s) | heaviest unit (s) | heavy_dump programs in it |
|---|---:|---:|---:|---:|---:|---:|---|
| 1/4 | 179.6 | 203 | 241 | 421.7 | 105.4 | `terminus` 172.8 | `terminus` |
| 2/4 | 193.6 | 219 | 259 | 387.8 | 97.0 | `04_open` 193.6 | `04_open` |
| 3/4 | 54.9 | 62 | 74 | 175.2 | 43.8 | `square-fwd/floors` 48.2 | none |
| 4/4 | 211.2 | 239 | 283 | 827.3 | 206.8 | `01_not_and_absent` 211.2 | `01_not_and_absent`, `corner-house`, `cul-de-sac` |

*Summed ÷ 4* is the least any dealing of these units to four workers could take. In shards 1 and 2 the wall clock is
the one heavy program's unit, and the other workers sit idle for most of it. **Shard 4/4 holds three of the five
heavy programs**, which ran side by side on three of its four workers: its wall clock is its heaviest unit and also
its work divided by four.

**The five heavy programs**, today's binary, `ein solve FILE --dump-answer DIR`
([`tools/heavy-dumps.sh`](tools/heavy-dumps.sh); one file each, `runs/heavy.NAME.dump-answer.count.txt`, with the
read-out and stderr beside it):

| program | shard | unit in the test (s) | state files | ms a file in the test | the dump (bytes) | of which `models/` | one state file (bytes) | the dump alone, wall (s) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `features/01_not_and_absent` | 4/4 | 211.2 | 64 655 | 3.27 | 354 577 878 | 189 677 693 | 2 817 to 2 976 | 2.2 |
| `square-unique/corner-house` | 4/4 | 210.4 | 65 214 | 3.23 | 286 187 689 | 135 235 865 | 1 984 to 2 099 | 2.0 |
| `square-unique/cul-de-sac` | 4/4 | 208.1 | 64 817 | 3.21 | 290 955 813 | 134 715 837 | 2 001 to 2 091 | 2.1 |
| `features/04_open` | 2/4 | 193.6 | 64 761 | 2.99 | 525 882 278 | 334 033 596 | 5 070 to 5 175 | 2.5 |
| `square-unique/terminus` | 1/4 | 172.8 | 64 514 | 2.68 | 349 017 219 | 156 814 505 | 2 337 to 2 455 | 2.4 |

The unit is the test's at `a5de14a1`; the file count is today's binary's (the logs count files per test function,
not per program), so *ms a file* divides one by the other.
The three heaviest by the test's time are `01_not_and_absent`, `corner-house` and `cul-de-sac`; `04_open` and
`terminus` are added because the ledger names `04_open` and the set is five. Each run is a partial answer: the program
is not monotone and the question needs the whole cube, past `--region-limit 16`, so each lists the 64 514 to 65 214
members it met (`exhausted no`, a `note: … partial` on stderr). A saved file is the whole program text and its
`(commit …)`, so its size is the program's. Writing the dump takes about two seconds of the 170 to 211; the rest is
the round trip of 64 to 65 thousand files, 2.7 to 3.3 ms of a worker each. (The `.count.txt` files' wall clock is
written with a decimal comma, the shell's locale.)

**One such file through the CLI**, for scale, on one pinned P-core thread
([`runs/one-state-file-through-the-cli.txt`](runs/one-state-file-through-the-cli.txt),
[`tools/one-state-file-through-the-cli.sh`](tools/one-state-file-through-the-cli.sh), every 200th file of
`01_not_and_absent`'s dump): `ein --version` 1.2 ms, `ein saturate --dump FILE` 3.2 ms, `ein solve FILE --no-search
--dump-answer D` 3.4 ms; so one load of such a file is about 2 ms beyond the process's start. The test's three checks
run in process, with no process start; how a file's 3.3 ms splits among them is not measured here.

**Inside each shard**, the test's eight functions `_0` to `_7` take the programs whose index is their number modulo 8,
all on one pool ([`runs/shard-tables.md` § The eight tests](runs/shard-tables.md#the-eight-tests-inside-each-shard)):
shard 4/4's `_0`, `_1` and `_2` read 90 123, 65 969 and 64 854 state files, the other five 20 to 4 143.

## 3. The options

What each would change in [`answer_round_trip.rs`](../../../../../ein.rs/crates/ein-cli/tests/answer_round_trip.rs)
(read, not run): `shard()` (line 667) deals the selected entries to the eight tests and hands each test's entries to
`ein_corpus::pool::map` (line 693), one unit an entry; the unit is `check()` (line 186), which runs the dump, loads the
program and runs the describe in process, checks `nogoods.json`, the eager cores and the cores' deadness, builds the
node function, and then loops over every state file: the run's own state through the node function (line 257), the
first load (262), save → load → save (279), load → save → load (289), and the binary on the first and last file (302).
The counts are asserted equal to the files at line 749.

**(a) As it is.** Nothing changes. Shard 4/4 is at 211 s here, 239 to 283 s at the runner's factor, under 300 s.

**(b) A heavy program's files dealt to the pool in parts.** `check()` splits in two: a unit that dumps and runs the
describe, then units of a part of the files each, every one loading the program and building its own node function,
folded back in order. The pool then spreads one heavy program over several workers. MOCK, the test's units file as
it would read with four parts (each part a quarter of the measured unit):
[`runs/units4.MOCK.option-b.tsv`](runs/units4.MOCK.option-b.tsv) ([diff](runs/units4.MOCK.option-b.diff)). MOCK, the
arithmetic on the measured units (the longest unit first, onto the worker that frees first; a part costs its share of
the measured unit; [`runs/shard-tables.md` § MOCK — option (b)](runs/shard-tables.md#mock--option-b-a-heavy-programs-files-dealt-to-the-pool-in-parts)):

| shard | wall here (s) | parts of 2 (s) | parts of 4 (s) | parts of 8 (s) |
|---|---:|---:|---:|---:|
| 1/4 | 179.6 | 105.4 | 105.4 | 105.4 |
| 2/4 | 193.6 | 97.0 | 97.0 | 97.0 |
| 3/4 | 54.9 | 48.2 | 48.2 | 48.2 |
| 4/4 | 211.2 | 209.3 | 210.2 | 206.8 |

Parts halve shards 1 and 2 and **do not move shard 4/4**: its three heavy programs already keep its workers busy, and
its work is 827 s for four workers. Parts reach shard 4/4 only with the dealing changed too, the five heavy programs
one to a shard first: MOCK, about 98 to 139 s a shard with parts of four
([`runs/shard-tables.md` § dealt to four shards first](runs/shard-tables.md#mock--option-b-with-the-five-heavy_dump-programs-dealt-to-four-shards-first)).
Today the dealing (`ein_corpus::shard::assign`) follows the pool's start order, `pool::HEAVY`, then `pool::LONG`, then
the manifest's `cost_ms`; `04_open` is on `HEAVY` and `corner-house` on `LONG`, the other three heavy dumps on neither.
The unit costs were measured with three heavy programs side by side on two physical cores; another mix costs
otherwise.

**(c) The second load on a share of a heavy program's files.** In `check()`, the load → save → load block (line 289)
runs on every *k*-th file of a `heavy_dump` program, and on every file of the others; the assert at line 749 counts the
files it ran on. Each heavy file keeps its first load (the state saved, origin for origin) and its byte-for-byte save;
the third check holds on one in *k*. MOCK, shard 4/4's log with *k* = 10 (the three heavy programs placed in tests
`_0`, `_1`, `_2` by their file counts, which fit no other way):
[`runs/s4.MOCK.option-c.log`](runs/s4.MOCK.option-c.log) ([diff](runs/s4.MOCK.option-c.diff)) (output, one line):

```text
shard 2: 22 programs, 64854 state files: 64854 loaded as the state saved, 64854 saved again byte for byte, 6665 loaded again to the same dump (every 10th file of `01_not_and_absent`, 6466 of its 64655), 27 through the binary; …
```

It removes one of a file's two loads on nine files in ten of the five heavy programs; the time that saves is that
load's share of a file's 3.3 ms, not measured here. It cuts each heavy unit, so it moves shard 4/4 as much as the
others.

## 4. The default

None: the row is **the user's call** (*as it is; a heavy program's files dealt to the pool in parts; or the second
load on a share of a heavy program's files*). Today it is (a).

## 5. Files

- [`runs/s1.log`](runs/s1.log), [`s2.log`](runs/s2.log), [`s3.log`](runs/s3.log), [`s4.log`](runs/s4.log) — the four shards' runs, raw (copied from the session's scratch).
- [`runs/units1.tsv`](runs/units1.tsv) to [`units4.tsv`](runs/units4.tsv) — each shard's units, raw.
- [`runs/shard-tables.md`](runs/shard-tables.md) — every table: per shard, its heaviest programs, the eight tests, the heavy programs, the two MOCK sections.
- `runs/heavy.NAME.dump-answer.count.txt` (five) — each heavy program's dump: files, `du`, sizes, wall clock.
- `runs/heavy.NAME.dump-answer.solve-stdout.txt`, `…stderr.txt` (five each) — that run's read-out and its `note: … partial`.
- [`runs/one-state-file-through-the-cli.txt`](runs/one-state-file-through-the-cli.txt) — one heavy state file through the CLI, with `bench_env.sh`'s fingerprint.
- [`runs/units4.MOCK.option-b.tsv`](runs/units4.MOCK.option-b.tsv) / [`.diff`](runs/units4.MOCK.option-b.diff) — **MOCK** of (b).
- [`runs/s4.MOCK.option-c.log`](runs/s4.MOCK.option-c.log) / [`.diff`](runs/s4.MOCK.option-c.diff) — **MOCK** of (c).
- [`tools/shard_tables.py`](tools/shard_tables.py) — writes `shard-tables.md`.
- [`tools/heavy-dumps.sh`](tools/heavy-dumps.sh) — the five dumps, counted (1.8 GB of files, 2.3 GB on disk; give it a scratch directory).
- [`tools/one-state-file-through-the-cli.sh`](tools/one-state-file-through-the-cli.sh) — the CLI timing.

The dumps themselves are not kept here (1.8 GB); the script re-takes them.

## 6. Re-run

From the repo root (no `cargo test`):

```sh
D=plans/m1f_hypothesis_and_documentation/review/01-save-and-load/07-L-106
python3 -I $D/tools/shard_tables.py
bash $D/tools/heavy-dumps.sh /tmp/ein-heavy            # a scratch directory; then copy its *.count.txt
utils/bench_env.sh --core 2 bash $D/tools/one-state-file-through-the-cli.sh /tmp/ein-heavy/01_not_and_absent/models /tmp/ein-cli-scratch
# the shards themselves, as measured (not run for this review):
# utils/bench_env.sh --cores PT:4 env EIN_TEST_JOBS=4 EIN_CORPUS_SLOW=1 EIN_TEST_SHARD=k/4 EIN_TEST_UNITS=u.tsv \
#   cargo test --profile gate -p ein-cli --test answer_round_trip
```

## 7. Found while building

- **Shard 4/4 is bound by three programs, not one.** The ledger says each shard is bound by one program; shard 4/4
  holds `01_not_and_absent`, `corner-house` and `cul-de-sac`, which ran side by side, and its 827 s of work for four
  workers is about its wall clock. So option (b) as worded leaves shard 4/4 where it is (by the arithmetic above); it
  needs the dealing of the heavy programs changed as well.
- **The ledger's third-heaviest is `04_open` (194 s); by the units it is `cul-de-sac` (208 s)**, which the row does
  not name. The row's numbers otherwise reproduce from the units and logs: 180, 194, 55, 211 s; 211 and 210 s.
- **All five heavy dumps are partial runs** at `--region-limit 16`, each listing 64 to 65 thousand members met; the
  number of files the round trip reads is set by that limit, not by the program.

## 8. Your call

- **(a)** as it is: shard 4/4 at about 239 to 283 s on the runner. Today's state.
- **(b)** a heavy program's files dealt to the pool in parts: shards 1 and 2 halve; shard 4/4 moves only with the
  heavy programs dealt one to a shard as well.
- **(c)** the load → save → load check on one in *k* files of the five heavy programs: every heavy unit shorter, by
  that load's share, not measured.
- No default.
