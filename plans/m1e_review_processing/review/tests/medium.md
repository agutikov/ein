# Tests — Medium

## The zebra2-variant byte check silently skips when python3 is absent

**Severity:** Medium
**Confidence:** High
**Topic:** Tests
**Classification:** test gap

**Locations**
- `ein.rs/crates/ein-cli/tests/cli_semantics.rs:269-293`

### Finding

`gen_zebra2_variants.py --check` is invoked from a cargo test, but when python3 is absent or exits 127 the test eprintlns and passes — the exact skip-on-stderr pattern the repo's own history denounces (oracle ledger §2: "41 tests passing on a SKIP line nobody read"; `dot_wellformed` was converted to fail for the same reason). CLAUDE.md flatly says "`--check` is in the gate". The structural diff still runs, and CI installs python — but a rule-body drift in a generated variant would pass a local gate on a machine without python3.

### Recommendation

Fail (as dot_wellformed does) rather than skip.

---

## Non-vacuity floors have drifted far below the corpus they guard

**Severity:** Medium
**Confidence:** High
**Topic:** Tests
**Classification:** test gap

**Locations**
- `ein.rs/crates/ein-infer/tests/lattice_semantics.rs:272-293`
- `ein.rs/crates/ein-cli/tests/corpus_cli.rs:451-458`

### Finding

`assert_census` requires `checked >= 55` over what is now 197 corpus files; corpus_cli's `every_positive_entry_answers` floor is `>= 60` against ~143 eligible entries (its inline comment is triply stale). Roughly half the positive/stdlib corpus could stop being swept before either assertion fired. The floor-not-exact pattern exists so the corpus can grow — but nothing re-tightens it, so sensitivity decays monotonically.

### Recommendation

Derive the floor from the manifest (e.g. `eligible_count - small_slack`) instead of a constant.

---

## The "no longer slow" direction of the slow-flag check runs only nightly

**Severity:** Medium
**Confidence:** High
**Topic:** Tests
**Classification:** test gap

**Locations**
- `ein.rs/crates/ein-cli/tests/corpus_cli.rs:350-412`

### Finding

The default sweep contains no slow entries, so the wall-clock floor check is skipped for them in every per-commit run and every default local gate; a `slow = true` flag that stops being true is visible only to nightly plus a reader of nightly. Documented as intentional in the test (:358-361) — but this is the exact rot S1a.9.0 existed to end, reintroduced with a one-day-plus-attention detection lag.

### Recommendation

At least assert in the default run that the two slow entries still *exist* and their `cost_ms` is present; leave the timing direction nightly.

---

## no_cell_crashes hard-codes "exit 2 = the CLI refused the argv", which is no longer the only meaning of 2

**Severity:** Medium
**Confidence:** High
**Topic:** Tests
**Classification:** test gap (latent coupling)

**Locations**
- `ein.rs/crates/ein-cli/tests/corpus_cli.rs:303-333` vs `cli_semantics.rs:943-964`, `test_cli.rs:332-396`

### Finding

`ein solve` legitimately exits 2 for a budget abort and `ein test` for load errors / nothing-to-check. Safe today only because no corpus run declares `-E`/`-T` and test runs are declared only on files that load and claim; adding a budgeted run or a test cell on a loader-negative would make an intended exit-2 fail the crash check with a misleading "usage error" message. The coupling is unstated in corpus/README's run-vocabulary section.

### Recommendation

State the constraint in corpus/README or make the check consult the entry's group.

---

## expect_semantics' or-matcher tests assert almost nothing

**Severity:** Medium
**Confidence:** High
**Topic:** Tests
**Classification:** test gap

**Locations**
- `ein.rs/crates/ein-infer/tests/expect_semantics.rs:354-380`

### Finding

`two_identical_disjuncts_do_not_cover_two_models` asserts only `!lines.is_empty()` after failure is already established — any failure text passes, so the doc-claimed property (the augmenting-path matcher produces a specific, non-confusing report) is untested; `one_wrong_disjunct_fails` only checks a line starts with "expectation ". A regression to the greedy pairing's confusing message would pass.

### Recommendation

Assert the decisive substring of the report in both tests.

---

## The stdlib mutation survivor has no re-take instrument

**Severity:** Medium
**Confidence:** High
**Topic:** Tests
**Classification:** test gap

**Locations**
- `tests/README.md:154-166`; `stdlib/slots.ein:408-415`

### Finding

`slot-adjacent-bwd-neg` with its two structure operands exchanged is killable by nothing in the suite, and the 51-mutant sweep it comes from is hand-taken with no `utils/` script — unlike every other census. A real stdlib soundness defect of that exact shape would pass `ein test tests/`, stdlib_coverage, the corpus sweep and all goldens, and the recorded 50/51 sensitivity number will silently rot as rules are added.

### Recommendation

Bank the sweep as `utils/stdlib_mutants.py` (the mutation vocabulary is already written down in tests/README) and add a fixture that kills the survivor.

---

## The NAF boundary's exactness machinery has no direct unit test

**Severity:** Medium
**Confidence:** High
**Topic:** Tests
**Classification:** test gap

**Locations**
- `ein.rs/crates/ein-infer/src/saturator.rs:1112-1116, 1230-1252`
- gap relative to `ein-infer/tests/naf_semantics.rs`

### Finding

Nothing asserts that a parked candidate whose watched relations did not grow is actually skipped (the judged_at/gs_epoch fast path), nor that watched_sizes carried across a resume behaves at the fork boundary; the guarantee rides indirectly on corpus event goldens and counter tests. This is the invalidation machinery that replaced a per-candidate stamp, and the one place a missed re-judgement would silently under-derive (a forall flip never noticed) rather than crash — a weaker localization than the rest of the boundary enjoys (naf_dropped, one-admission and retirement all have named tests).

### Recommendation

Two unit tests: a stalled-guard candidate is not re-judged when an unrelated relation grows; a fork-inherited candidate is re-judged exactly when its watched extent grew in the fork.

---

## Gate = CI is enforced only by convention, and the convention already failed once

**Severity:** Medium
**Confidence:** High
**Topic:** Tests
**Classification:** test gap

**Locations**
- `run_tests.sh:56-62`; `.github/workflows/per-commit.yml:5-9`

### Finding

Both files instruct "keep the two lists the same", but nothing diffs them; the repo's own headers record the failure mode happening (three red commits on findings the local run reported as a pass). The mechanism that failed — hand-maintained parallel lists — is unchanged.

### Recommendation

A tiny test that parses both step lists and diffs them.

### Cross-references

- `review/architecture/medium.md` (the parallel-copies pattern).
