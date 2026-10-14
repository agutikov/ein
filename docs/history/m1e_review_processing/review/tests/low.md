# Tests — Low

## Wall-clock-sensitive assertions inside the deterministic gate

**Severity:** Low
**Confidence:** High
**Topic:** Tests

**Locations**
- `ein.rs/crates/ein-cli/tests/test_cli.rs:203-225` (features dir must finish < 20 s)
- `ein.rs/crates/ein-cli/tests/corpus_cli.rs:335-412` (dev-profile wall clock at 4× tolerance)

### Finding

Deliberately generous, but these are the only tests in the workspace that can fail on machine load rather than behavior; on a badly overloaded CI runner they become flakes that will be read as engine regressions.

### Recommendation

Gate on an env var or raise the margins and label the failure message "machine load?".

---

## Hard-coded world anchors couple four crates' tests to two puzzle files with no single registry

**Severity:** Low
**Confidence:** High
**Topic:** Tests

**Locations**
- `ein.rs/crates/ein-cli/tests/embedding.rs:126-139`; `ein-ir/tests/kb_semantics.rs:1101`; `ein-cli/tests/cli_semantics.rs:156-176, 304-330`; `ein-infer/tests/obligation_reports.rs:263-285`

### Finding

Deliberate anchor tests (their docs say so), but any legitimate edit to zebra.ein/zebra2.ein fans into at least four crates' test files plus docs/api/rust.md, and only embedding.rs documents the cost. A reviewer changing zebra2 has no list of the anchors that will fire.

### Recommendation

One comment block (or doc section) listing the anchors, referenced from both puzzle files.

---

## `run_tests.sh --tests-only` also skips the bench smoke, contradicting its own header

**Severity:** Low
**Confidence:** High
**Topic:** Tests

**Locations**
- `run_tests.sh:7, 186-189`

### Finding

The header says "skip the static checks"; the flag also skips CI's last step (the bench compile/run smoke). A green `--tests-only` is a strict subset of CI — the precise property the script's own header warns about, in miniature; a bench that stops compiling is invisible to `cargo test`.

### Recommendation

Run the bench smoke under `--tests-only`, or amend the header.

---

## stdlib_census.py --check is wired to no gate or workflow

**Severity:** Low
**Confidence:** High
**Topic:** Tests

**Locations**
- `utils/stdlib_census.py`; `ein-infer/tests/stdlib_coverage.rs:28-35`

### Finding

By design the in-gate check is scoped to `tests/stdlib/`; the corpus-wide census (the numbers, the sole-activator table) is re-taken only when someone remembers — the same "runs when somebody remembers it" failure the coverage test's own doc names for scripts.

### Recommendation

A nightly step, or an explicit statement that the census is milestone-cadence only.

---

## The release workflow's cross-platform legs have never run

**Severity:** Low
**Confidence:** High
**Topic:** Tests

**Locations**
- `.github/workflows/release.yml:19-25`

### Finding

The file is honest about it ("a green badge here is read as: the first tag passed"), but until a tag exists the four-platform matrix, jobs-cross-diff and no-default-features legs are untested promises; a reviewer should not read the workflow as evidence the binary matrix works.

### Recommendation

Run the matrix once on a pre-release tag.
