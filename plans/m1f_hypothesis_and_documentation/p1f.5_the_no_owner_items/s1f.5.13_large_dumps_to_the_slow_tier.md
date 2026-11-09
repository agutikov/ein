# S1f.5.13 — The round trip's four large dumps move to the slow tier, and the nightly runs it

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.13**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).

**Status:** specified 2026-11-08 on the user's ruling of that day on P-16. **Done 2026-11-09**, on the user's
go-ahead for P1f.5 of 2026-11-08 and the instruction of 2026-11-09 (*"just continue with default decisions till the end
of the phase"*), after S1f.5.8 and beside the second lane's S1f.5.9, in three commits: T1f.5.13.1, the distribution and
the slow tier's first run (`63245a11`, [its record](#t1-record-2026-11-09)); T1f.5.13.2, the field, its checks, the
selection and the nightly step (`87d18cf6`, [its record](#t2-record-2026-11-09)); and T1f.5.13.3, the pages ([its
record](#t3-record-2026-11-09)). Three choices were taken by default and recorded:
[Q-M1f.50](../open_questions.md#q-m1f50--where-the-heavy_dump-line-goes-and-which-entries-carry-the-flag), the line
and the five entries;
[Q-M1f.51](../open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs), three more tests whose slow
half no CI tier runs, parked for the phase's close; and
[Q-M1f.52](../open_questions.md#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve), the
nightly red at `utils/review_bundle.py` since `fc53818f`, reported and not fixed here. The acceptance is re-taken [below](#acceptance-re-taken-2026-11-09).
**Depends on:** nothing. [S1f.5.12](s1f.5.12_cores_for_n_only.md) made each of the four dumps cheaper, and
neither stage waited for the other.

## The ruling

The agent put P-16 as *leave it, move the four to the slow tier, or give it an owner*, and recommended
re-measuring after P-15. The user, 2026-11-08: *"P-16 - I already decide - move to slow tier"*.

## The case

[`answer_round_trip.rs`](../../../ein.rs/crates/ein-cli/tests/answer_round_trip.rs) runs every corpus program's
describe run with `--dump-answer`, whether or not its entry declares that run. It then reloads every state
file by value. Four programs past `--region-limit` write tens of thousands of state files each (the
inventory, 2026-11-06, release build):

| program | wall | dump size |
|---|---|---|
| `saturation/square-unique/corner-house` | 2.85 s | 268 MB, 65 214 state files |
| `saturation/square-unique/cul-de-sac` | 3.2 s | 275 MB |
| `saturation/square-unique/terminus` | 3.78 s | 334 MB |
| `features/04_open` | 3.63 s | 318 MB |

The test took 34.5 s and 19.6 s on two consecutive runs, the second with no compile in it.

**`slow` cannot carry them.** `slow = true` is a measured claim, *an entry whose declared runs cost 1 s or
more together*, and two tests hold it in both directions. These four declare cheap runs (`saturate`,
`render rules`, `render constraints`, and `render lattice` for `04_open`), and the round trip's dump is not
one of them. Marking them `slow` would be a false claim, and the check would fail.

**The test's slow tier runs nowhere.** `include_slow()` reads `EIN_CORPUS_SLOW`. Neither
[`nightly.yml`](../../../.github/workflows/nightly.yml) nor `./run_tests.sh --nightly` runs
`answer_round_trip` with it (`grep -n answer_round_trip .github/workflows/*.yml run_tests.sh`, 2026-11-08,
empty). Only `./run_tests.sh --slow`, run by hand, does. So the 14 `slow` entries' dumps,
`lattice/06_friends_tables` and `features/01_not_and_absent` among them, are round-tripped by no CI tier.
Moving four more there unchanged would leave them unchecked too.

**What stays per commit.** [`past-limit-lines.ein`](../../../examples/ein-bugs/wrong-answers/past-limit-lines.ein)
stops at the limit with three state files and 15 members of N in 1.23 s (`ein solve FILE --dump-answer D`,
2026-11-08, `916d97ac`, release). So the per-commit tier keeps a run past the limit, and `members_of_n`'s
past-the-limit check, after the four leave.

## What it changes

- **A manifest field** (default name `heavy_dump`): *the entry's describe dump writes at least T state files*.
  T is set at step 1 in the gap of the distribution `EIN_ROUND_TRIP_LIST=1` prints. It is a count, not a time,
  so it depends on no machine.
- **Held in both directions**, as `slow` is:
  - per commit, the test dumps every unflagged entry, and fails on one that writes T or more;
  - in the slow tier, it dumps the flagged ones too, and fails on one that writes fewer.
- **The test's selection.** Per commit it skips a flagged entry. Under `EIN_CORPUS_SLOW=1` it takes every
  entry, the `slow` ones included, as today.
- **A nightly step**: `cargo test -p ein-cli --release --test answer_round_trip` with `EIN_CORPUS_SLOW=1`, in
  `deep-corpus` beside `answer_dump`'s step, with a comment in the same style. `./run_tests.sh --nightly`
  gains the same step, which `ein-cli/tests/nightly_steps.rs` holds against the workflow.
- **The first run of the slow tier** is step 1's, by hand. A failure on an entry no CI tier has
  round-tripped is a finding, shown to the user and not fixed here.
- **What does not move.** No answer and no register row. Every program is still round-tripped, four of them
  nightly instead of per commit.

## The method

### Task T1f.5.13.1 — the distribution, and the slow tier's first run

`EIN_ROUND_TRIP_LIST=1 EIN_CORPUS_SLOW=1 cargo test -p ein-cli --release --test answer_round_trip`: the state
files per program, T chosen in its gap, and every failure listed.

### T1 record, 2026-11-09

**The slow tier's first run passes.** `EIN_ROUND_TRIP_LIST=1 EIN_CORPUS_SLOW=1 cargo test -p ein-cli --release
--test answer_round_trip -- --nocapture` at `7bc6ea72`: 9 of 9 tests pass, in 116.4 s, with the second lane's gate
running beside it (`build/s1f.5.13/t1_slow_list.log`, the machine state at its head). 296 of the 417 entries are
answered and write 485 020 state files; 39 of those write none, and the other 121 the round trip does not answer (a
file the loader refuses, or a run that writes no `index.json`). **No failure**, so there is no finding to show the
user: the 14 `slow` entries, whose dumps no CI tier had round-tripped, reload by value, every member of N with its
core and every node held to its deadness.

**The distribution** (`build/s1f.5.13/t1_distribution.txt`), the state files `check` reloads per program:

| state files | entries | today's tier |
|---:|---|---|
| 65 214 · 64 817 · 64 761 · 64 655 · 64 514 | `square-unique/corner-house`, `cul-de-sac`, `features/04_open`, `features/01_not_and_absent`, `square-unique/terminus` | per commit, but `features/01`, `slow` |
| 23 390 | `square-fwd` and `square-bwd`, `floors`, `houses` and `meetings`, six | `slow` |
| 11 339 | `type-exclusivity/pets` | per commit |
| 4 096 | `syntax/equality`, `syntax/constraint-scopes` | per commit |
| 169 | `type-exclusivity/colors`, `nationalities` | per commit |
| 34 or fewer | the other 280, 39 of them writing no state file | per commit, but seven `slow` ones: `lattice/06_friends_tables`, `lattice/07_two_pairs_two_tables`, `zebra`, `zebra2`, `zebra2-obligations` and the two `zebra2-minus-15` |

**What each costs the per-commit run**, by a scratch probe that timed each `check` in list mode, reverted after
(debug, as the gate builds it, at `7bc6ea72`, under the same load, `build/s1f.5.13/t1_debug_default_probe.log`): the
test took 62.8 s. `terminus` 57.7 s, `features/04_open` 56.6 s, `cul-de-sac` 51.9 s and `corner-house` 48.8 s,
each on a shard of its own, so the slowest shard is the test; then `pets` 8.1 s, `zebra2-hints` 4.3 s (six state
files; the oracle's cube and the lattice dump are its cost), `past-limit-lines` 3.3 s, the two `syntax/` programs
1.7 s and 1.3 s, and every other program 0.5 s or less. Summed per shard over the probe
(`build/s1f.5.13/shard_sums.py`): today the slowest shard is 62.2 s and the eight 244 s together; with the four left
out, 9.3 s and 29.3 s; with `pets` and the two `syntax/` programs out as well, 5.5 s and 18.2 s.

**T = 40 000** (`ein_corpus::manifest::HEAVY_DUMP_FILES`), taken by default
([Q-M1f.50](../open_questions.md#q-m1f50--where-the-heavy_dump-line-goes-and-which-entries-carry-the-flag)). The
distribution has two gaps a line could sit in: 23 390 to 64 514 (2.8×), which separates exactly the four the ruling
named, and 169 to 4 096 (24×), which would move `pets` and the two `syntax/` programs too and save about 4 s more of
the per-commit wall. Forty thousand is 1.6× under the five and 1.7× over the six. **The flag is held on every entry
the round trip dumps**, the slow tier's too, so it says one thing of every entry: `features/01_not_and_absent`, which
is `slow` and writes 64 655, carries it, and five entries are flagged.

**The spec's case, re-taken on today's build** (`utils/bench_env.sh --core 4 build/s1f.5.13/dumps.sh …`, release,
one pinned P-core, best of three, 2026-11-09, `build/s1f.5.13/t1_dumps.log`; the second lane started a release
`cargo test` of three targets during it, and the spread is 0.04 s or less): S1f.5.12 took no core from these runs
before or after, and their cost is the files.

| program | wall | state files | apparent size | on disk |
|---|---:|---:|---:|---:|
| `saturation/square-unique/corner-house` | 2.60 s | 65 214 | 255 MiB | 393 MiB |
| `saturation/square-unique/cul-de-sac` | 2.89 s | 64 817 | 262 MiB | 397 MiB |
| `saturation/square-unique/terminus` | 3.36 s | 64 514 | 318 MiB | 429 MiB |
| `features/04_open` | 3.18 s | 64 761 | 307 MiB | 430 MiB |
| `features/01_not_and_absent` (`slow`) | 3.03 s | 64 655 | 284 MiB | 402 MiB |
| `ein-bugs/wrong-answers/past-limit-lines` | 0.71 s | 3 | — | 1 MiB |
| `saturation/type-exclusivity/pets` | 0.55 s | 11 339 | — | 74 MiB |

**What stays per commit past the limit** is `past-limit-lines`: three state files, and `nogoods.json`'s 15 members of
N each with its core (S1f.5.12's T3 record), so `members_of_n`'s past-the-limit check keeps a per-commit witness.

### Task T1f.5.13.2 — the field, its check and the selection

The field in `ein-corpus`'s manifest and on the four entries. The two checks, the test's selection, and its
doc comment's `SHARDS` paragraph restated.

### T2 record, 2026-11-09

**The field.** `ein_corpus::Entry::heavy_dump`, with its line, `ein_corpus::manifest::HEAVY_DUMP_FILES = 40_000`,
whose doc carries T1's distribution; `corpus/corpus.toml` flags the five entries and says what the flag is in its
header. `ein-corpus`'s `the_heavy_dump_set_is_exactly_these_five` pins the set per commit, as
`the_slow_set_is_exactly_these_fourteen` pins `slow`'s: whether a flag is true is the round trip's to say, and it dumps
a flagged entry only in the slow tier, so a flag gained by an entry that writes few files would take it out of the
per-commit round trip with nothing per commit to notice.

**The checks and the selection.** `answer_round_trip.rs`' `shard` takes every entry under `EIN_CORPUS_SLOW=1`, and per
commit every entry that is neither `slow` nor `heavy_dump`; `heavy_dump_flag` holds each entry it dumps to its flag,
so an unflagged entry that writes 40 000 state files or more fails in either tier, and a flagged one that writes
fewer, none included, fails in the slow tier. The doc comment's new paragraph says what runs where, and `SHARDS`' says
what the shards now spread. **Mutation-checked** (2026-11-09, release, the manifest restored after): with
`corner-house` unflagged, the per-commit run fails naming it, 65 214 files, and so does the set's pin; with
`syntax/equality` flagged as well, the slow tier fails naming both, `equality`'s 4 096 under the line.

**The nightly step**, brought forward from T3 so that no commit leaves the four out of every CI tier:
`TMPDIR="${GATE_TMP}" cargo test -p ein-cli --release --test answer_round_trip` under `EIN_CORPUS_SLOW: "1"`, in
`nightly.yml`'s `deep-corpus` after `answer_dump`'s step and before the scratch check, with a comment in the same
style; `./run_tests.sh --nightly` runs the same step, which `nightly_steps.rs` holds against the workflow, and its new
`the_round_trips_slow_half_runs_with_the_flag` pins the flag, as `the_slow_cells_run_under_release_with_the_flag` pins
`corpus_cli`'s: with `EIN_CORPUS_SLOW` taken out of both copies the diff passes and the pin fails (a scratch mutation,
reverted). Two comments that counted the nightly's test steps, *four* in `nightly.yml` and `run_tests.sh` where there
were five and are now six, no longer count them, and `run_tests.sh`' `--slow` paragraph names `heavy_dump`.

**The per-commit time, before and after** (`build/s1f.5.13/before_after.sh debug default 2`, the two test binaries
built from `7bc6ea72`'s test and this one and run alternately, debug as the gate builds them, no other cargo process
running, 2026-11-09, `build/s1f.5.13/t2_per_commit_before_after.log`): **55.1 s and 56.5 s before, 8.60 s and
8.67 s after.** The run dumps 278 programs and 20 623 state files, where it dumped 282 and 279 929; `pets`' 11 339
are the most any program writes per commit.

**The slow tier**, on this commit's code (`WHICH=after build/s1f.5.13/before_after.sh release slow 1`, no other cargo
process at its start, 2026-11-09, `build/s1f.5.13/t2_slow_tier.log`): 9 of 9 tests pass in **121.4 s**. It is what the
nightly's new step runs.

**Nothing else moves.** The change is a test's selection, a manifest field and a CI step: no engine code, golden,
answer or register row.

### Task T1f.5.13.3 — the nightly step and the pages

The nightly step and its `--nightly` mirror. [`corpus/README.md`](../../../corpus/README.md)'s fields and
readers, `CLAUDE.md`'s gate block, and the inventory's P-16, which says what fixed it.

### T3 record, 2026-11-09

**The pages.** [`corpus/README.md`](../../../corpus/README.md) has a § `heavy_dump` beside § `slow`, the flag in its
format block, and a row for `answer_round_trip` among its readers (with `manifest.rs`'s row, whose *ten tests* no
longer counted them). `AGENTS.md` has the round trip's slow run in its gate block, a sentence in its nightly
paragraph, and the stage in its P1f.5 account. The inventory's P-16 says what fixed it, in its table and under its
item, which stays as taken; the phase README has P-16's row, the stage's row and § The order's item 10.

**Found outside the brief:**
- **Three more tests have a slow half no CI tier runs**: `route_invariance`, `ein-infer`'s `region` and
  `obligation_rung`, each passing at its slow depth (857.3 s, 37.6 s and 71.0 s in release, under load, 2026-11-09,
  `build/s1f.5.13/t3_other_slow_halves.log`). The same hole P-16's re-take found for the round trip; recorded, not
  scheduled, and parked for the phase's close
  ([Q-M1f.51](../open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs), defaulted).
- **The nightly is red at `utils/review_bundle.py`, and has been since `fc53818f`** (S1f.5.5's T13, the same day),
  behind a green gate, whose `--smoke` builds one case and no index. `./run_tests.sh --nightly` on `87d18cf6` ran every
  `deep-corpus` test step green, the new one in 123.3 s, then the scratch check and `stdlib_census.py --check`, and
  stopped at the bundle, 1 738 s in (`build/nightly-s1f.5.13.log`): its index reads `[case.wrong] verdict`, which the
  refusals of `W41/saturate-import` and `W43/einb-dump-answer` do not carry; and with that line guarded (tried, and
  reverted) seven cases fail to build, W40's six and W41's, on `case.py`'s *a register run is a solve*. What a review
  page shows for a run that is not a solve is a design this stage does not own: reported, not fixed
  ([Q-M1f.52](../open_questions.md#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve),
  defaulted). The steps after it, run apart on the same code (`build/s1f.5.13/nightly_after_bundle.sh`,
  `build/nightly-s1f.5.13-after-bundle.log`): `stdlib_mutants.py --check` green, 167 of 217 killed, in 42 s; the
  bundle's link check finds four dead links, every one in pages an earlier local bundle wrote on 2026-11-06 for F1's
  files, which `ccd1007f` deleted, so a fresh checkout has none; the fuzzer's seed 1 green in 1 507 s. The bench smoke
  is the gate's last step, green at T2, and the second seed was not run here.
- **The fuzz job costs about 25 minutes a seed now**: seed 1's million iterations took 1 507 s above, and ten thousand
  take 16.8 s (`EIN_FUZZ_ITERS=10000 EIN_FUZZ_SEED=1 cargo test -p ein-ir --release --test fuzz_properties`,
  2026-11-09, beside the million), where S1e2.3.6 measured both seeds and the bench smoke at 15 s together on
  2026-11-03, before S1f.5.5's swap. `run_tests.sh`' header still says the fuzz and bench jobs are in `--nightly`
  *because they are cheap*. Reported, not acted on: what a fuzz iteration costs is the parser's, Q-M1f.8's subject.
- **`corpus/README.md` § `slow` and `corpus/corpus.toml`'s header count the `slow` entries as three and two**, where
  `the_slow_set_is_exactly_these_fourteen` names fourteen (re-taken 2026-11-03); the section's cell count and
  seconds are of the same earlier day. Not restated here: its numbers want `utils/corpus_cost.py`'s re-take, which is
  not this stage's.

## Acceptance

- The per-commit tier writes no dump of T state files or more, and a test fails if one appears unflagged.
- The nightly round-trips every corpus program's dump, the flagged and the `slow` entries included, and
  `./run_tests.sh --nightly` runs the same step.
- A flagged entry whose dump falls under T fails the slow tier.
- `./run_tests.sh` is green, and `answer_round_trip`'s per-commit time is taken before and after.

### Acceptance, re-taken 2026-11-09

On the stage's last code commit, `87d18cf6`, item by item:

- **The per-commit tier writes no dump of T state files or more, and a test fails if one appears unflagged.** Per
  commit the round trip dumps 278 programs and 20 623 state files, the most `pets`' 11 339 (T2's run); its
  `heavy_dump_flag` fails an unflagged entry that writes 40 000 or more in either tier, as `corner-house` unflagged
  showed per commit (65 214), and `the_heavy_dump_set_is_exactly_these_five` fails on any change to the set. **Met.**
- **The nightly round-trips every corpus program's dump, the flagged and the `slow` entries included, and
  `./run_tests.sh --nightly` runs the same step.** `deep-corpus` runs `answer_round_trip` under `EIN_CORPUS_SLOW: "1"`,
  which selects every entry, the five flagged and the 14 `slow` among them, 296 programs answered and 485 020 state
  files (T1's run); `nightly_steps.rs`' diff holds `--nightly`'s copy to the workflow's, and
  `the_round_trips_slow_half_runs_with_the_flag` the flag. The step on this code: 9 of 9 in 121.4 s (T2's run).
  `./run_tests.sh --nightly` on this code ran it green too, 9 of 9 in 123.3 s, with every `deep-corpus` test step and
  the scratch check after them (`build/nightly-s1f.5.13.log`); the nightly as a whole is red at a later step this
  stage did not make ([§ T3 record](#t3-record-2026-11-09)). **Met.**
- **A flagged entry whose dump falls under T fails the slow tier.** `syntax/equality` flagged, the slow tier failed
  naming it, 4 096 under 40 000 (T2's mutation); a flagged entry the round trip does not answer counts as none and
  fails the same way. **Met.**
- **`./run_tests.sh` is green, and `answer_round_trip`'s per-commit time is taken before and after.** The gate at
  T1f.5.13.2, 16 min 12 s (`build/gate-s1f.5.13-T2.log`), the round trip 9.99 s in it; before and after, debug,
  alternated on a quiet machine: 55.1 s and 56.5 s, then 8.60 s and 8.67 s. **Met.**
- **What does not move.** No answer, golden or register row: the register reads 129 fixed, 22 open and 1 not wrong
  (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09), unmoved by the stage. Every program is
  still round-tripped, four of them nightly where they were per commit. **Met.**
