# S1f.21.26 — the key of a state at its own size

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-25 on the user's answer to review item 78, *"(b)"*
([the README](README.md#the-answers-to-part-2s-items-and-item-87)); **built** 2026-11-25 on its own branch, the
code in `24aeebbe`, the costs re-taken and the record below; merged into master by `6ddef90e`.
**Writes:** `ein-infer/src/region.rs` (the node function) and what reads a state's key whole; their tests; the
costs page's final table, re-taken.

## What it is

[V-86](findings.md#v-86--a-states-key-sorts-every-fact-the-state-holds-roots-among-them). At every state the node
function takes the state's key, the sorted list of its facts beyond root's, which is how two commitments that reach
one state are told from two that do not (`region.rs`, `classes.entry(out.state)`; `canon::distinct_by_state`). It
makes the key the long way (`region.rs`, `node_in_region`):

```rust
let state = crate::canon::state_key(&fork);          // every fact the state holds, root's fixpoint too, sorted
let delta: Vec<FactId> = state.iter().copied()
    .filter(|f| !axes.root_facts.contains(f))        // root's filtered back out
    .collect();                                      // the key
```

On `send_more_money.ein` the key is 38 to 39 % of the instructions and the filter 9 to 10 %; on
`equation_two_unknowns.ein` about 11 % and 2 to 3 % (review item 78's profile, in `git log` at `bc5146bb`). The engine
did so before the phase.

**The move is S1f.21.23's** ([Q-M1f.674](../open_questions.md#q-m1f674--the-facts-a-resumed-state-added-read-off-its-own-layers)):
a fork's own facts are its layers above root's mark, `Kb::facts_above`, which `Saturator::new_facts_of` already reads,
filtering the few against root's set (an `open` fact the fork holds as root did is root's). So the key is

```rust
let mut delta = snapshot.new_facts_of(&fork);        // the fork's own facts, at their own size
delta.sort_unstable();                               // still sorted: derivation order differs between commitments
```

**the same set in the same order**, so every key, every class of states, every answer and every firing is as it was.

## Tasks

1. **Who reads a key whole**, root's facts in it, settled first: the describe route's splits (`region.rs`,
   `split_in_fork`), the read-out's check of paths (`read_out.rs`, `crate::canon::state_key(kb)`), `ein-einb`'s
   solutions, `canon::distinct_by_state` and its callers. Each either takes the key above root, which tells states
   apart as exactly, every state holding root's facts, or keeps the whole one where it is no state's cost; recorded,
   one line each.
2. **The node function** takes its key above root's mark as above, the filter gone. On a build with debug assertions
   (the gate's) the new key is held to the old at every state, as S1f.21.23's read is held to the walk.
3. **What holds it**, as S1f.21.21 and S1f.21.23 held theirs: `ein solve -p` and the digest of the events on every
   corpus file, before and after, identical; the gate.
4. **The cost taken again**: the profile of item 78 on both programs, and the final table of
   [the costs page](costs/README.md#the-final-table-before-after-improved-after) re-taken with a column for this
   stage, on an idle machine on AC, through `utils/bench_env.sh`.

## What must hold

- **No key moves**, so no class, no answer, no firing: task 3.
- The defaults a stage takes are recorded with ids the coordinator gives, and its findings likewise.

## Record

Run 2026-11-25 in a worktree fast-forwarded to `master` at `9994ac94`; `master` moved during the stage (S1f.21.27's
pictures, `e207bc5d`) and was not merged in. Scratch named `build/s26/…` is the worktree's.

### Commits

| commit | task | what |
|---|---|---|
| `24aeebbe` | 1, 2 | the node function's key read above root's mark and sorted, held to the whole key on the gate's build; the oracle's listing read at the same point; the comments of `canon.rs`, the solution store and its test; three design pages |
| `2460b536` | 4 | `costs/three_engines.py` with a fourth point, `instructions.py`, `key_profile.sh`, `key_profile_figures.py`, their outputs, and the costs page's final table re-taken |
| this record | | plans only |

### 1. Who reads a key whole

| reader | what it does now |
|---|---|
| the node function (`region::node_in_region`) | the key above root's mark, at its own size (task 2) |
| `Nodes::by_parent` and `by_split` (`split_in_fork`) | **the whole key**, kept: read once a parent path of a printed state (the read-out's spot check, bounded on a flagged program, every path under `-y`) and by `detector.rs`, compared with the whole key of the direct fixpoint and, in that test, of a state built from scratch, which has no mark; each costs a fresh saturation over the whole KB, beside which the key is small; no search reads it at a node |
| the read-out's check of paths (`read_out.rs`, `crate::canon::state_key(kb)`) | **the whole key**, kept: the other side of the comparison above, one a printed state of two commitments or more |
| `ein-einb`'s solutions (`SolutionNode::state_key`) | **the whole key**, kept: the identity a stored state carries in the file, compared with a state rebuilt from the file (`SolutionNode::reconstitute`), which has no mark; written by `Solutions::of`, which no command calls (the round-trip test does); its comment, which said the search deduplicated nodes by it, now says the search keys a node by the part beyond root's |
| `canon::distinct_by_state` | reads nothing: **no caller** in the workspace (V-106) |
| the oracle's listing of a node (`--states`, `Axes::keep_states`) | no key: every fact the node holds, read over `Kb::facts` in any order (the text is sorted), against root's set, at the same point of the node as before |
| the examples `owed_census.rs` and `rung_census.rs`; the tests `explain_semantics`, `search_semantics`, `acceptance`, `detector`, `ein-einb`'s `roundtrip` | whole keys compared with whole keys, outside the engine; unchanged |

### 2. The node function

`node_in_region` takes `snapshot.new_facts_of(&fork)`, the fork's facts above root's mark less those the snapshot
held (an `open` fact the fork holds as root did is root's), and sorts them with `sort_unstable`, as `state_key` sorts;
the whole key and the filter against `Axes::root_facts` are gone from the release build. On a build with debug
assertions (the gate's) a `debug_assert_eq!` holds the key at every node to `whole_key_beyond_root`, the old
computation, `state_key(&fork)` less root's facts. **Checked live**: a gate build whose key dropped its last fact
panicked on `zebra2.ein` with the check's message; the restored source builds the same gate binary, byte for byte.

The oracle's listing read the sorted whole key; it now reads `fork.facts()` with the same filter, taken right after
the key, where the old one was taken, before a stopped dead node is saturated on for its core. `Axes::root_facts`
stays for the listing and the check (Q-M1f.698).

**Which read the key takes**: a probe build that printed a line wherever `new_facts_of` walks a KB whole, over the 944
corpus files under 13 modes (`solve -p`, `--minimal`, `--maximal`, `--single`, `-n 3`, `--count`, `--describe-route
states`, `--in-flight -p`, `-y`, `--dump-answer`, `--dump-lattice`, `oracle --dump --states`, `test FILE`; 30 s a
run): **the walk never ran**; every key is read above the mark (`build/s26/probe.sh`).

The design pages say how a node is keyed: `07-search.md` § 5, `01-determinism-and-order.md` § 5, the identity row of
`08-reasoning-in-code/03-hypotheses-and-the-answer.md`; `canon.rs`' module comment too.

### 3. No key moved

`build/s26/diffbins.sh`: for every `.ein` under `examples/`, `tests/`, `stdlib/` and `corpus/` (**944 files**), `ein
solve FILE -p` (stdout, stderr, exit code) and the md5 of `ein solve FILE --events E`, `E` the same path for every
build: **byte for byte the same** as `9994ac94`'s release build on three builds: the release build of the change, its
gate build, whose check ran at every state and never fired, and the release build of `24aeebbe`, which is the first
byte for byte. 191 files write no events, the same 191 on every build. And `build/s26/oracle_states.sh`: `ein oracle
FILE --dump D --states` on the 944 files, stdout, stderr, exit code and the dump, region.md's line of times aside: the
same on both release builds (699 exit 0, 200 exit 1, 45 exit 2, on both).

**Tests**: `cargo test --profile gate -p ein-infer -p ein-einb`: 66 test binaries, 592 tests, none failed (the
modes against the oracle, `claim_routes.rs`, `region.rs`, `tree_classes.rs` among them, every node through the
check), nothing left in `TMPDIR`. `cargo fmt --all --check`, `cargo clippy -p ein-infer -p ein-einb --all-targets
-- -D warnings` and `RUSTDOCFLAGS=-D warnings cargo doc -p ein-infer -p ein-einb --no-deps`: clean.
`utils/doc_audit.py --links --check --tracked`: 0 findings; the release voice and `check_doc_blocks.py` on the edited
pages: clean. The gate is the coordinator's, after the merge.

### 4. The cost

**Item 78's profile, again**: `costs/key_profile.sh`, three `perf record --call-graph lbr -e instructions:u` of
`taskset -c 0-15 ein solve FILE --jobs 1` a program and an engine, `9994ac94` (`master` before the stage) and
`24aeebbe`, each reported as item 78 reported it; `costs/key_profile_figures.py` reads the shares of the P-core
instructions off each report (`costs/key_profile.tsv`). *The key* is `canon::state_key` with what it calls; *the
filter* the vector of the key's facts root does not hold, its self time; *the read above the mark* the vectors
`new_facts_of` builds, both its calls (a resumed state's delta and the key); the sort of the few facts left has no
symbol of its own on `send_more_money.ein` and 0.2 to 0.3 % on `equation_two_unknowns.ein`.

| program | instructions, before → after | the key, before | the filter, before | the read above the mark, after |
|---|---:|---:|---:|---:|
| `send_more_money.ein` | 10 438 M → 5 537 M | 38.8 %, 38.5 %, 38.2 % | 9.4 %, 9.1 %, 9.6 % | 0.5 %, 0.3 %, 0.8 % |
| `equation_two_unknowns.ein` | 3 290 M → 2 860 M | 10.1 %, 9.9 %, 10.7 % | 2.8 %, 3.0 %, 2.2 % | 0.2 %, 0.2 %, 0.3 % |

The instructions are the reports' P-core counts, the mean of three. Taken on AC, the load 2.3 when it started
(instructions, not time, so the load does not move them).

**The final table**, [the costs page](costs/README.md#the-final-table-before-after-improved-after), re-taken with a
fourth column, `24aeebbe`, every engine on the corpus and the standard library of its own commit (`improved after`
on `build/costs/tree-fa2c2914`, unpacked by `git archive`, where it read the main checkout before). Instructions
(`costs/instructions.py`, `perf stat`, three runs, which agree to within 0.03 %), with `master` at `9994ac94` on the
same tree as the key's, so the stage's own effect is the ratio of the two:

| program | `master` | the key | key / master |
|---|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 3 292 M | 2 867 M | 0.871 |
| `saturation/type-exclusivity/pets.ein` | 3 517 M | 3 495 M | 0.994 |
| `features/01_not_and_absent.ein` | 23 928 M | 23 776 M | 0.994 |
| `beyond_arith/send_more_money.ein` | 10 441 M | 5 539 M | 0.531 |
| `zebra2.ein` | 1 112 M | 1 105 M | 0.994 |

**Time**, milliseconds, an entry's declared runs summed (`costs/three_engines.py`, every column re-taken; the
whole table is on the costs page). Taken through `utils/bench_env.sh --cores PT` on AC (ACAD online=1, BAT1 full),
governor `powersave`, turbo on, load 1.46 / 3.69 / 6.12 as it started and 1.02 as it ended, no agent running (the
other stage had ended), only a browser in the background. The entries the key moves, and three it does not:

| entry | before | improved after | key | key / improved | work |
|---|---:|---:|---:|---:|---|
| `beyond_arith/send_more_money.ein` | 2 090 | 2 112 | 1 238 | 0.59× | 18 129 |
| `beyond_arith/send_more_money_written.ein` | 1 404 | 1 410 | 819 | 0.58× |  |
| `beyond_arith/equation_two_unknowns.ein` | 461 | 491 | 439 | 0.89× | 1 191 |
| `zebra2.ein` | 1 837 | 1 874 | 1 821 | 0.97× | 120 |
| `features/01_not_and_absent.ein` | 2 376 | 2 952 | 2 995 | 1.01× | 65 299 |
| `saturation/type-exclusivity/pets.ein` | 404 | 474 | 471 | 0.99× | 11 339 |

Everywhere but these first three the key is within 4 % of *improved after*, either way.

### Defaults taken (Q-M1f.695 to Q-M1f.699)

| id | question | default | the other reading |
|---|---|---|---|
| Q-M1f.695 | how the oracle's `--states` listing is read, which read the sorted whole key | over every fact the node holds, in any order, against root's set, at the point the key was taken: the text sorts it, and the oracle alone asks it | read above the mark, with root's `open` facts the node holds added from the snapshot's set: exact only while root's stack holds no `open` fact |
| Q-M1f.696 | whether the read-out's check of parent paths and `by_parent`/`by_split` take the key above root | kept whole: a printed state's paths, each a fresh saturation, compared with a whole key and, in `detector.rs`, with a state built from scratch, which has no mark | both sides above root's mark, the test's state from scratch read whole |
| Q-M1f.697 | whether `ein-einb`'s stored key takes the part above root | kept whole: the file's identity of a state, compared with a state rebuilt from the file; the format unchanged | the part beyond root's, the format's version raised |
| Q-M1f.698 | whether `Axes::root_facts` goes with the filter | kept: the listing reads it, and the gate's check of the key | dropped: the listing read off the snapshot's set and the check off `root.facts()` at each node |
| Q-M1f.699 | what becomes of `canon::distinct_by_state`, which nothing calls | left as it is, recorded (V-106) | removed with its doc |

### Findings outside the stage (V-106, V-107)

- **V-106 — `canon::distinct_by_state` has no caller.** Nothing in the workspace calls it, the examples and the
  tests included; its doc says it is *the one implementation of same model* that three sites (the solution table,
  `expect`, the key table) deduplicate a model list by, which none does any more. Not changed (Q-M1f.699).
- **V-107 — an instruction count of a program of many small states moves with how it is run.** Re-taken by
  `costs/instructions.py`, `pets.ein` and `01_not_and_absent.ein` count about 420 M and 3 180 M less than the
  rows of 2026-11-20 on every engine alike, where `equation_two_unknowns.ein`, `send_more_money.ein` and
  `zebra2.ein` agree to 0.1 %. One binary on one input moves too: `fa2c2914` on its own tree's `pets.ein`, one
  worker, counts 3 509 M under `bench_env.sh`, 3 642 M from a plain shell pinned the same way, and 4 230 M unpinned.
  The three runs of one session agree to 0.03 %, so rows taken together compare; rows of two sessions do not. The
  cause is not settled (a worker pool's idle threads, which `instructions:u` counts, would fit). **Open.**

### What the user must see

- **A state's key costs its own size**: read above root's mark and sorted, where every state sorted root's whole
  fixpoint and filtered it back out. The same key at every state: 944 corpus files, `solve -p` and the digest of
  `--events`, byte for byte as before on the release build and on the gate build that checks the key against the
  whole one at every node; the oracle's listing the same.
- **What it saves**: 47 % of `send_more_money.ein`'s instructions and 13 % of `equation_two_unknowns.ein`'s, 0.6 %
  of the other three; in time, 41 to 42 % of the two `send_more_money` programs' (1 238 ms where 2 112) and 11 % of
  `equation_two_unknowns.ein`'s, the rest within 4 % either way; the key and its filter were 47 to 48 % and about 13 % of the two programs' profiles, and what
  is left of them is under 1 %.
- **Five defaults** (Q-M1f.695 to Q-M1f.699) and two findings (V-106, V-107).
