# S1a.10.1 — Bank what only the oracle proves

**Phase:** P1a.10 (One implementation)
**Estimate:** 4 days
**Depends on:** [P1a.9](../p1a.9_bindings_release/README.md) — **for the
deletion, not for this stage.** The phase's hard dependency is `docs/api/`
losing its subject at [S1a.10.5](s1a.10.5_removal.md)/[.6](s1a.10.6_docs.md);
an inventory that deletes nothing has none, and running it *first* is the whole
point of the gate below. P1a.7 is paused and P1a.8/P1a.9 have not started.
**Gate:** nothing in this phase is deleted until this stage's ledger has an
owner for every row.

**Status: shipped 2026-08-20.** The ledger is
[`oracle_ledger.md`](oracle_ledger.md) — 9 sections, 4 dispositions, 4 accepted
losses. Three instruments landed with it, and each was checked against a
deliberate break before being trusted.

| finding | number |
|---|---|
| the corpus at T3, one last time, both engines | **503 same, 2 DIFF** (both [D2](../divergences.md)), 0 skip, 505 cells |
| **ein.rs integration tests that shell out to ein.py** | **42 of 91** — 41 skip and 1 panics when the oracle cannot start, so the suite reports 318 of 319 passing |
| how a skipped parity test looks in the gate | *exactly like a passing one* — `skip()` writes to stderr, which `cargo test` captures for passing tests |
| ein.rs tests that read files under `ein.py/` and would break on `git rm` alone | **5**, none of which the no-Python experiment could detect |
| the determinism sweep's successor: renderings that move under a permuted id space | **66 of 2 544** (495 of 20 352 at 8 seeds) |
| — of those, **answers** that move | **0** |
| — what does move | a dying fork's stopping point (44) and which of a fact's equally valid justifications was recorded first (22) — [D3](../divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)'s three observables, reached from inside one engine |
| T1 counter identities, measured over the corpus before being written down | **13**, holding on 365 cells |
| T1 counters that are **zero on every one of 176 solve cells** | **3** — and two of them had never been noticed |
| T3 renderings banked as digests | **4 228**, 345 KB, against **38.0 MB** of bytes |
| what the mutation floor costs a break | 138 of 4 228 renderings; 137 of 365 counter cells |
| and one the inventory was not looking for: `utils/check_hashmap_iteration.py` on `master` | **red** — six unannotated aggregate iterations, arriving one at a time since S1a.5.4. Repaired here |

The stage also opened
[Q-M1a.22](../open_questions.md#q-m1a22--is-einbs-id-remap-order-preserving-enough-for-its-own-gate),
because the same measurement bears on
[P1a.8](../p1a.8_binary_container/README.md)'s gate.

## Context

The harness is not one check, it is four tiers over ~505 cells plus a fuzzer
plus a determinism sweep, and they do not all prove the same *kind* of thing.
Some of it is already duplicated by ein.rs's own tests. Some of it is a claim
about ein.py that stops mattering when ein.py does. And some of it is a claim
about **the semantics** that ein.rs currently gets right only because
something else was checking.

Sorting those three apart is the whole stage, and it has to happen before the
delete because afterwards there is no way to find out which row was which.

## Acceptance

- A **ledger**, one row per behaviour the harness asserts, each with exactly
  one disposition:
  - **covered** — an ein.rs test already asserts it; the test is named;
  - **banked** — a new ein.rs test asserts it; the test is named and lands in
    this stage;
  - **retired** — it was a claim *about ein.py* (its exception classes, its
    `argparse` text, its `sorted()` raising) and dies with it; the reason is
    written down;
  - **accepted loss** — nothing will assert it again. Every row here needs a
    sentence saying what regression could now pass unnoticed. **A short list
    is a result; an empty list is a claim to be suspicious of.**
- The four tiers are each accounted for, not the corpus as a whole:
  **T0** verdict, **T1** every counter, **T2** the event log, **T3** the bytes.
- The **determinism sweep** (`--env-a PYTHONHASHSEED=0 --env-b
  PYTHONHASHSEED=42`, which found hazards H1 and H4) has a successor that does
  not need two engines — ein.rs against itself under a shuffled interner is
  the same question, and
  [S1a.7.1](../p1a.7_parallelism/s1a.7.1_sync_shared_state.md) T1a.7.1.6
  already wants that instrument.
- The **fuzzer** ([S1a.6.6](../p1a.6_performance/s1a.6.6_differential_fuzzer.md))
  keeps every property that is self-checkable — no panic, dump→parse→dump
  round-trip, hash-seed determinism, `--jobs` invariance — and the acceptance
  states plainly that the differential arm, which found all four of its bugs,
  is gone.
- The **divergence ledger** ([D1–D3](../divergences.md)) is re-read: each entry
  either becomes an ein.rs-side fixture asserting *ein.rs's* behaviour, or is
  marked historical.

## Tasks

### Task T1a.10.1.1 — Inventory the tiers

Walk `conformance/` and classify. The mechanical part is cheap — the corpus
manifest lists entries and runs — and the judgement is per *tier*: T3 on a DOT
file is a golden ein.rs can own outright, T1 on a counter is a property, and
T0 on a verdict is the thing P1a.11's stdlib corpus is about to assert from
the outside.

### Task T1a.10.1.2 — Bank the T3 bytes as ein.rs goldens

The pattern exists: [S1a.6.11](../p1a.6_performance/s1a.6.11_fixture_goldens.md)
already did this for what the contract stopped comparing, and
`ein.rs/crates/*/tests/golden*` is where they live. Extend it to the cells
that only the harness covers. Byte goldens are cheap to generate and their
weakness is well known — they pin *behaviour*, not *intent* — so record which
ones are pinning something nobody can otherwise state.

### Task T1a.10.1.3 — Bank the T1 counters as properties

A counter golden rots into "whatever it was last time". Prefer a property
where one exists: `enterings_total = alive + dead_pre + dead_post`,
`nogoods_emitted + nogoods_subsumed = deaths under path-nogoods`, the
`--jobs` invariants from
[S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md). Where no
property exists, a golden with a comment saying *why* the number is that
number.

### Task T1a.10.1.4 — The determinism successor

`utils/check_hashmap_iteration.py` and the two-seed sweep, re-aimed: one
engine, interner pre-seeded in a random order, whole corpus, output must not
move. This is the invariant [design/08](../design/08_parallelism.md) §1 calls
the one that makes determinism affordable, and it needs no oracle.

### Task T1a.10.1.5 — The accepted-loss list

Written last, from what is left over, and reviewed rather than filed. This is
the honest part of the stage: the harness caught four parity bugs on a surface
five phases had signed off, and whatever class those came from is exactly the
class that will not be caught again.

## What it found

### 1. The gate is already half differential — 42 of 91

Not in the plan, and the largest row in the ledger.
[P1a.10's acceptance](README.md#acceptance-for-the-phase) is "`cargo test
--workspace` is the whole gate", and **42 of that gate's 91 integration tests
start a Python process**. Measured by putting a `python3` that exits 127 first
on `PATH`:

```text
$ PATH=<a python3 that exits 127> \
    cargo test --workspace --no-fail-fast -- --nocapture --test-threads=1
318 passed; 1 failed
```

**41 of those 318 passes asserted nothing.** `--test-threads=1` is not
decoration: without it the skip lines interleave and cannot be attributed to a
test, which is how the count was first read as 40.

Three problems, and only the first is the one anybody would predict:

1. **They skip, and a skip is invisible.** `ein_oracle::skip` writes to stderr
   *because* `cargo test` swallows stdout for passing tests — but it captures
   stderr too, so the 40 skips appear only under `--nocapture`. A gate that
   silently stops asserting is worse than one that fails.
2. **One test panics instead**, `help_parity::the_surface_matches_argparse`,
   and it is the only honest one in the set by accident.
3. **Five crates carry `ein-oracle` as a dev-dependency**, so the delete is
   five compile errors before it is a test failure.

### 2. Five tests die on `git rm` alone, and no experiment above finds them

`ein-ir/tests/dump_parity.rs`, `ein-render/tests/golden_trace.rs`,
`derivation_dot.rs`, `golden_dot.rs` and `ein-conformance/src/corpus.rs` read
**files** under `ein.py/` — the 19 checked-in goldens and the package's stdlib
copy. They run Python not at all, so removing `python3` proves nothing about
them, and they are green right up to the commit that deletes the tree.

The ledger's [§4](oracle_ledger.md#4-what-the-removal-must-relocate) is the
defect list, and it carries the one instruction that matters: **move the files,
do not regenerate them.** All 19 are ein.py's own output. Carried across they
keep saying "ein.rs reproduces what the other implementation produced"; blessed
afresh from ein.rs they would say "ein.rs reproduces itself".

### 3. The answer does not depend on which integer a name got. The proof does.

The determinism sweep was `PYTHONHASHSEED=0` against `=42`, and it has no
successor of that shape: ein.rs has no salted hash to perturb. What it has
instead is an **id space**, assigned in first-seen order, which
[`intern`](../../../ein.rs/crates/ein-core/src/intern.rs) and
[design/08 §1](../design/08_parallelism.md) both make claims about —
`Symbol` has no `Ord`, observable sorts go through `Interner::rank`, no
observable iterates a hash map. Every one of those claims is falsified by the
same experiment.

`ein-render/tests/id_order_invariance.rs` runs each corpus file twice: once
ordinarily, once from a `Terms` where every name it will intern has already
been interned **in a shuffled order**, every integer literal likewise, and every
fact re-interned in an order shuffled *within its nesting depth* — depth-blind
would be wrong, since `(not X)` cannot precede `X`, and a uniform offset would
be no permutation at all.

| | 1 seed | 8 seeds |
|---|---:|---:|
| `(file, op)` pairs permuted | 2 544 | 2 544 |
| pairs with no ids to permute — `dot`'s parse views and the `ir[*]` ops, which answer off the AST | 1 684 | 1 684 |
| renderings that moved | **66** | **495** |
| — only where a dying fork stopped | 44 | 310 |
| — only in the body of a rendered derivation | 22 | 185 |
| **answers that moved** | **0** | **0** |
| wall clock | 10.5 s | 48.0 s |

What moves is *exactly* the three observables the D3 row of
[design/01 §5](../design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list)
already calls narration, and every name in the 22 —
`dot[slice]`, `trace[trace|answer|no-proof]`, `dump[snapshot]` — is already on
`ein_parity::is_narration`'s closed list. **Nothing had to be added to it.**
D3 therefore stops being a statement about two engines and becomes a statement
about what a derivation is, which is why `ein-parity` outlives the harness that
motivated it.

The first perturbation was *wrong* and the negative control caught it: shifting
every `FactId` by a constant is a translation, not a permutation, so an
identity-order sort survived it. The instrument is only worth its 10 seconds
because it was made to fail first.

### 4. Two T1 counters the harness compared 505 times and learned nothing from

`stats.enterings_dead_pre` and `stats.nogoods_subsumed` are **0 on all 176
solve cells**, alongside the already-documented `naf_dropped`. Two zeroes agree
for the wrong reason, and a T1 tier that compared them on every cell of every
run since P1a.4 never once separated a right answer from a wrong one.

The first is *structural* and the argument is short enough to check: a fork is
`dead-pre` when `contradiction::detect` fires on the hypothesis facts alone,
which needs a commitment holding some `X` **and** `(not X)`; `hypgen::generate`
only proposes positives and drops any whose negation is believed
(`negated_fact`), and `apriori::filter_candidate` re-drops any candidate whose
element left `alive`, which is exactly what the singleton writeback does. No
commitment can carry a pair, so **every death in this engine is `dead-post`**.
The second is a claim about the *corpus*, not the engine, and is a growth item.

`summary_properties.rs` asserts all three as zeroes *with their reasons*, and
asserts that the set of never-firing counters is exactly those three — so a
fourth one going quiet is a failure rather than a fact nobody noticed.

### 5. The bytes are 38.0 MB, so they are banked as 345 KB of digests

The shape functions over the corpus are **4 228 renderings and 38.0 MB** —
`ein-bugs/zebra2-bad`'s `trace[trace]` alone is 6 MB. A golden tree that size
is not more reviewable than a hash, so
`ein-render/tests/golden/corpus_shapes.md5` carries one line per rendering:
16 hex of `md5` — the same digest and truncation `render::hashed_id` already
uses — plus the line count, which is what turns "it moved" into "it grew by
40 lines".

It was blessed **here**, in a tree where `cargo test --workspace` was green
with the differential half still running. That timing is the whole provenance
argument and it is the reason this stage is the gate: every line in the
manifest is a byte string ein.py had signed off on.

## What it changes

- **[S1a.10.2](s1a.10.2_port_the_suite.md) gains a second subject.** It was
  written about 1 517 pytest tests; it also has to un-differential the 42
  ein.rs tests, and the ledger says what each of them still owes. The
  disposition table it asks for exists for those 42 already.
- **[S1a.10.3](s1a.10.3_corpus_without_an_oracle.md) T1a.10.3.3 has an
  answer for `ein-parity`.** Finding 3 makes the crate a statement about
  derivations rather than about two engines, so it survives; and
  `ein-conformance/src/corpus.rs`'s completeness check now carries all nine of
  the manifest's invariants, not five, which is what has to move when the crate
  is retired.
- **[S1a.10.5](s1a.10.5_removal.md) gets a defect list** — five tests and one
  `stdlib` fallback path, none of which fail before the delete.
- **[P1a.8](../p1a.8_binary_container/README.md) gets a question**
  ([Q-M1a.22](../open_questions.md#q-m1a22--is-einbs-id-remap-order-preserving-enough-for-its-own-gate)):
  its gate is byte-identity between `x.einb` and `x.ein`, `.einb`'s remap is a
  permutation of the id space, and finding 3 prices exactly that. The fast path
  saves it — and design/10 §3 states the fast path's condition as "the live
  interner is empty", which it never is, because `Terms::new` interns eighteen
  kernel names first.
- **The determinism lint is green again.** `utils/check_hashmap_iteration.py`
  is a per-commit CI step and it was reporting six findings on `master` — all
  false positives (`.sum()`s and a histogram), all needing the one-line
  `determinism-ok:` reason the check is designed around. The lint landed at
  S1a.0.4 (2026-08-17) and the first unannotated iteration at S1a.5.4 the next
  day, so it has been red since — and the two that followed, at T1a.6.2.2 and
  S1a.6.1, each landed against a gate that was already failing. That is how a
  gate stops being one: not by being removed, but by nobody being able to tell
  which finding is theirs.
- **The fuzzer's surviving arm is decided**, not implemented:
  [oracle_ledger §6 L1](oracle_ledger.md#6-accepted-loss) lists the four
  properties one engine can still check and says plainly that none of them
  would have found any of the four bugs the differential arm found.

## Notes

- The temptation is to bank everything as byte goldens because it is
  mechanical. Resist it in proportion to how much the golden would be
  *explaining*: a DOT file has no argument to make, a counter does.
- Anything that cannot be banked is a candidate for
  [P1a.11](../p1a.11_stdlib_conformance/README.md), which checks rules against
  stated expectations rather than against a second engine — the one kind of
  check that gets *stronger* when the oracle leaves.
- **The three new instruments cost 18 seconds** — 5.4 s for the digests, 10.5 s
  for the id sweep, 2.3 s for the counters — against the 9 m 13 s the workspace
  suite takes *with* the oracle. Most of that 9 minutes is Python: `dot_parity`
  alone is 166 s, and its ein.rs half is under three. The suite gets
  dramatically faster when the oracle goes, and that is worth saying out loud
  because it is the one thing about this phase that will feel like an
  improvement rather than a loss.
- **`EIN_ID_SEEDS=8`** is the deeper sweep and takes 48 s; one seed is the
  default because a single permutation already displaces every non-kernel id.
  Eight seeds found nothing the first did — 495 movements, all in the same two
  classes.
