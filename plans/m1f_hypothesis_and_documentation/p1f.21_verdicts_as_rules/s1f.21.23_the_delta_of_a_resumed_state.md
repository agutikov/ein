# S1f.21.23 — the delta of a resumed state, read off its own layers

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator on the user's word, *"of course build"*
([Q-M1f.674](../open_questions.md#q-m1f674--the-facts-a-resumed-state-added-read-off-its-own-layers)); **not
started**.
**Writes:** `ein-infer/src/saturator.rs` (`Snapshot::new_facts_of` and what it reads), its callers in
`region.rs`, `ein-core/src/kb.rs` if the read needs one, their tests; two comments of `obligations.rs`.

## What it is

[V-78](findings.md#v-78--a-resumed-state-walks-every-fact-it-holds-to-find-what-is-new). A state of a
search is root's fixpoint branched, the commitment's facts written, and the saturation resumed from root's
snapshot. To resume, the saturator takes the facts the state holds and root did not. `Snapshot::new_facts_of`
finds them by walking every fact the state holds and asking the snapshot's set for each: root's whole fixpoint,
at every state, to find the handful the commitment wrote a moment before. On `send_more_money.ein` root holds
1 867 facts and the search builds 18 129 states; a profile puts the walk at about 17 % of its instructions. The
engine did so before the phase.

## Tasks

1. **The delta read off the layers above the snapshot's.** The facts a state added are in its own layers, above
   root's sealed ones, in the order they were written. `Kb` already reads its facts from a position on and skips
   whole layers below it. The delta costs its own size.
   - **The same facts in the same order**: the order of the delta is the order a state's derivations arrive in,
     which a core and an explanation read. A debug check on the gate's build holds the new read to the walk it
     replaces, a check a state and bounded as the earlier stages' are.
   - **Every caller**: the node function, the trace's, the saved node's, and the tests' own.
   - A fact the commitment names that root already held is no part of the delta, as today.
2. **[V-84](findings.md#v-84--two-comments-of-the-obligations-reader-name-forms-the-parser-refuses)**:
   the two comments of `obligations.rs` that name forms the parser refuses, reworded.
3. **Measured**: instructions executed, one worker, pinned, on the cost page's five programs, before and after.

## What must hold

Every corpus answer as before, and the order of every firing: `ein solve -p` and the digest of `--events`
identical on every corpus file, as S1f.21.21 checked its own. No golden moves. The gate on the last commit, and
the two slow render sweeps.

## Record

Run 2026-11-20 in a worktree fast-forwarded to `master` at `0c8887d6`; `master` did not move during the stage.
Scratch named `build/s23/…` is the worktree's.

### Commits

| commit | task | what |
|---|---|---|
| `e211bde3` | 1 | a layer's stamp, `Kb::mark`, `Kb::facts_above`; `Snapshot::new_facts_of` reads the delta above root's mark, the walk kept for a KB that did not grow from it and, on the gate's build, held to the read; two tests; the three design pages |
| `30371a5e` | 2 | the comments of `obligations.rs` that named `:priority` and a bare `(open)` (V-84) |
| this record | | plans only |

### 1. The delta read above the snapshot's layers

**Built.** A `Layer` has a `stamp`, its identity: a number from a process-wide counter, drawn where a layer is made
(`Default`) and where one is copied (`Clone`), and kept where one is moved, as `Kb::seal` moves the top behind an
`Arc`. Two layers with one stamp are one layer at two moments, and since a layer only appends, the facts it held at
the first are the first it holds at the second. `Kb::mark` records each sealed layer, and the top where it holds a
fact, by stamp and length (`ein_core::Mark`); `Kb::facts_above(&mark)` compares the KB's first layers with the
mark's (the same stamps, each as long, the last at least as long) and, where they match, reads
`Kb::facts_from(the mark's count)`: the facts the KB wrote since the mark, then its `open` facts, as `Kb::facts`
reads them. It answers `None` for any other KB: a copy whose top is its own (`Kb::clone` gives the copy's top a new
stamp), a KB flattened or rebuilt (a new layer), another KB. What it costs is the mark's layers compared and the
facts read.

`Snapshot` keeps root's mark beside its fact set, both taken at the same moment (`Saturator::snapshot`).
`Snapshot::new_facts_of` reads the facts above the mark less any the snapshot held, where it walked every fact the
state holds and asked the set for each. The two reads are the same list: a KB that matches the mark holds root's
stack at the snapshot first, every one of those facts in the snapshot's set, so the walk dropped exactly them, and
past them both reads ask the same set of the same facts in the same order.

The cases the brief named:

- **A fact the commitment names that root held** is not written into the fork (`Kb::add_and_index_fact` finds it
  held and appends nothing), as before; and were it written above the mark, the set would drop it, as the walk did.
- **The program's written `(open F)` facts**, which the empty commitment's fork holds as root does
  (`region::fork_of`): they are in the fork's `open` layer, above the mark, and the snapshot's set holds them (root's
  `open` layer is written before the snapshot is taken), so the read drops them exactly as the walk did. This is why
  the set stays: the mark covers root's stack, not root's `open` facts, which a fork does not inherit.
- **The snapshot's KB written since**: its top is the mark's last layer, longer, and the read is what it grew by.
- **A KB not grown from the snapshot's** (a copy of root taken before root was sealed, which a test writes): the
  read answers `None` and the walk runs, as before.

**Every caller is unchanged and reads through `new_facts_of`**: the node function (`region::saturated`, which every
route's node, the saved node of a `(commit …)` file through `region::above`, `region::fixpoint` and so the oracle's
node, `split_in_fork`'s first half and the traced reader go through), the coupling's reader
(`Nodes::with_fixpoint_supported_within`), `saturator.rs`' tests, `naf_semantics.rs`, and the examples
`owed_census.rs` and `rung_census.rs`.

**Which path the engine takes**: a probe build that printed a line wherever the walk ran, over the 942 corpus files
(`examples/`, `tests/`, `stdlib/`, `corpus/`) under `solve -p`, `solve --events`, `--minimal`, `--maximal`,
`--single`, `-n 3`, `--count`, `--describe-route states`, `--in-flight -p`, `--json-summary`, `--dump-answer`,
`--dump-lattice`, `ein oracle`, `ein show hyps`, `ein render h-space` (the read-out, `--dump`, `--supports --view
coupling`, `--view one-step`), `ein test FILE`, `ein test FILE --diff full`, and `ein test examples tests stdlib`:
**the walk never ran** (`build/s23/modes.sh`, `supports.sh`; a 30 s budget a run, none reached). Only the tests'
copy takes it.

**The debug check**: on a build with debug assertions (the gate's), `new_facts_of` asserts the read equal to the
walk at every state where the read is taken: the walk the release build did before, and nothing more. Checked live:
a build whose read skipped one fact (`facts_from(n + 1)`) panicked on `zebra2.ein` with the check's message. Its
cost, in instructions of the gate build, one `ein solve --jobs 1` pinned to the P-core threads, `0c8887d6`'s gate
build built from its own tree (`git archive` into `build/s23/tree-before`), one run each:

| program | gate, `0c8887d6` | gate, after | after / before | gate / release, after |
|---|---:|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 6 574 M | 6 591 M | 1.003 | 1.99 |
| `saturation/type-exclusivity/pets.ein` | 4 484 M | 4 521 M | 1.008 | 1.29 |
| `features/01_not_and_absent.ein` | 28 995 M | 29 221 M | 1.008 | 1.22 |
| `beyond_arith/send_more_money.ein` | 18 617 M | 18 660 M | 1.002 | 1.78 |
| `zebra2.ein` | 1 334 M | 1 336 M | 1.001 | 1.20 |

Tests: `kb.rs`' `the_facts_above_a_mark_are_what_a_kb_grown_from_it_holds_after_them` (a fork, the marked KB written
since, the `open` facts last, a mark whose top holds nothing, and `None` for a copy, a flattened KB and another KB);
`saturator.rs`' `a_delta_holds_nothing_the_snapshot_held_on_either_read` (a fact root held, written again, and an
`open` fact the fork holds as root did, on the read and on the walk of a copy). The design pages say how the delta
is read: `03-data-model.md` § 5 (the stamp, the mark, `Kb::facts_above`), `06-saturation.md` § 4,
`01-determinism-and-order.md`'s table.

### 2. Two comments of `obligations.rs` (V-84)

The module's comment said `:priority` keeps a meaning in the report order, and `Owes`' said report order begins
with `:priority`: both now say the order is the rules' load order, then the activator's, then the match's. Three
comments said `None` is "the bare `(open)`": `Owed::relation`'s says `None` is an argument that is no name, which no
activator gives (one that binds `?R` to a fact activates nothing: the loader's warning, checked on a program of the
kind); `Owes::by_relation`'s and `open_argument`'s say the same without the form. `// Load order, the registry's: no
rule has a priority.` stays: it is true. The code is unchanged (V-88 below).

### No firing moved

`build/s23/diffbins.sh`: for every `.ein` under `examples/`, `tests/`, `stdlib/` and `corpus/` (942 files), `ein solve
FILE -p` (stdout, stderr, exit code) and the md5 of `ein solve FILE --events E`, `E` the same path for every build,
since the `run` line echoes argv: **byte for byte the same** as `0c8887d6`'s release build on the release build of
`30371a5e` and on its gate build, whose debug check ran at every state and never fired (no panic, every exit code as
before). 191 of the files write no events, the same 191 on every build: the load refusals of `examples/broken/`,
`examples/ein-bugs/` and three fuzz findings. `ein test examples tests stdlib`: the same output on both release
builds, the time aside (834 held, 9 FAILED, 2 not checked, 187 errors, as at `0c8887d6`).

### 3. Measured

**Instructions executed, user space**: `perf stat -x, -e instructions:u <ein> solve FILE --jobs 1`, pinned with
`taskset -c 0-15`, the two core types' lines summed (the E-core counter counts nothing there), three runs each,
which agree to within 0.012 %; the mean, millions. Both builds read the worktree's `stdlib/`, which the stage does
not change. The script is `build/s23/instr.sh`.

| program | before, `0c8887d6` | after | difference | after / before |
|---|---:|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 3 413 | 3 304 | −109 | 0.968 |
| `saturation/type-exclusivity/pets.ein` | 3 526 | 3 509 | −17 | 0.995 |
| `features/01_not_and_absent.ein` | 23 992 | 23 900 | −92 | 0.996 |
| `beyond_arith/send_more_money.ein` | 11 498 | 10 479 | −1 019 | 0.911 |
| `zebra2.ein` | 1 117 | 1 114 | −3 | 0.997 |

A call-graph profile (`perf record -e instructions:u --call-graph lbr`, one worker) of `send_more_money.ein`: at
`0c8887d6` the delta's walk is 8.2 % inclusive, the loop over the layers (8.1 %) inside it; after, the read above
the mark is 0.24 %. What the program saves, 8.9 %, is that walk and the set's lookups (V-87). No time is given here:
the coordinator takes the timed table on an idle machine.

### Defaults taken (Q-M1f.675 to Q-M1f.678)

| id | question | default | the other reading |
|---|---|---|---|
| Q-M1f.675 | how a KB grown from the snapshot's is recognised | a stamp a layer, fresh where a layer is made or copied and kept where it is moved, and a mark of stamps and lengths, compared at every read (the count of root's layers) | a fact count alone, `Kb::facts_from(n)`, trusting every caller to hand a fork of the snapshot's KB (a copy of root would read a wrong delta, in silence); or root sealed when the snapshot is taken and its layers compared by `Arc` |
| Q-M1f.676 | whether the snapshot keeps its fact set | kept: the facts above the mark are asked of it, so the `open` facts the empty commitment's fork holds as root did, and any fact root held in its `open` layer that a fork writes into its stack, stay out exactly as the walk had them; the set is built once a run and copied with the snapshot, as before | the set dropped and root's `open` facts kept apart, the read above the mark taken as it stands |
| Q-M1f.677 | what a KB that did not grow from the snapshot's gets | the walk, as before: no engine path in the corpus takes it (§ 1), the tests' copy does | refused, a panic: a caller legal today would fail |
| Q-M1f.678 | where a stamp comes from | a process-wide atomic counter, relaxed: two draws where a KB branches, one where a layer is sealed; a stamp is compared and nothing else, and its debug form is a constant, so no output reads the number | a counter per thread with the thread's id; or the stamps kept by the KB beside each `Arc` |

### Findings outside the stage (V-86 to V-88)

- **V-86 — a node's state key sorts every fact the state holds, root's among them.** `region::node_in_region` takes
  `canon::state_key(&fork)`, which collects every fact of the node and sorts them by id, and then filters root's
  out against `axes.root_facts` to get the node's `state`. On `send_more_money.ein` after this stage the key is
  37.7 % of the instructions (34.1 % at `0c8887d6`) and the filter 9.3 % (9.8 %); on `equation_two_unknowns.ein`
  10.9 % and 2.1 %. The node's own facts are above root's mark, which this stage reads, so the part of the key above
  root could be read and sorted at its own size; whether anything reads the key whole, root's facts in it, is what a
  stage would have to settle. **Open; the largest cost at a state left on `send_more_money.ein`; the user's call.**
- **V-87 — V-78's 17 % was one cost counted twice.** In the profile of `0c8887d6`, the delta's walk is 8.2 %
  inclusive and the walk of the layers (8.1 % of self time) is inside it; V-78 added the two. What this stage takes
  off `send_more_money.ein` is 8.9 % of the instructions (§ 3).
- **V-88 — `Owed::relation`'s `None` is reached by no program.** An obligation's `(open R)` takes a name or a
  parameter; an activator that binds the parameter to a fact activates nothing (the loader's warning, *a relation is
  named by a symbol, not a fact*), so every instance has a relation, and the `Option`, the `None` that
  `Owes::by_relation` skips and the empty relation `narrate` writes for it are dead code. Not changed.

### The gate

Not run yet.

### What the user must see

- **The delta of a resumed state costs its own size**: read above root's mark, where it walked root's fixpoint at
  every state. The same facts in the same order: 942 corpus files, `solve -p` and the digest of `--events`, byte for
  byte as before, on the release build and on the gate build that checks the read against the walk at every state.
  No golden moved.
- **What it saves**: 8.9 % of `send_more_money.ein`'s instructions, 3.2 % of `equation_two_unknowns.ein`'s, 0.3 to
  0.5 % of the other three (§ 3). V-78's 17 % was the walk counted twice (V-87).
- **The next cost of the same kind is larger** (V-86): every node sorts all its facts, root's among them, for its
  key, and filters root's away: about 47 % of `send_more_money.ein` after this stage.
- **Four defaults** (Q-M1f.675 to Q-M1f.678): a stamp a layer and a mark, the set kept, the walk for a KB not grown
  from the snapshot's, a process-wide counter.
