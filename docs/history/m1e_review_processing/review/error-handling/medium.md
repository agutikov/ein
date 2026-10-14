# Error handling — Medium

## Artefact-write failures never affect the exit code

**Severity:** Medium
**Confidence:** High
**Topic:** Error handling
**Classification:** design ambiguity (needs a deliberate ruling)

**Locations**
- `ein.rs/crates/ein-cli/src/solve.rs:314-319, 613-618, 699-705`
- `ein.rs/crates/ein-cli/src/test.rs:489-493, 809-815`

### Finding

A failed `--events` open, a failed `--json-summary` write, and a failed `--trace` write each print one stderr line and the run exits as if the artefact existed. A pipeline that asked for `--json-summary` can get exit 0 with no file (e.g. an unwritable path).

### Evidence

The corpus tests only assert artefacts appear on successful writes; nothing pins the failure path's exit code. This is arguably the strict reading of "additive: exit code unchanged" — but the one signal a machine consumer has is a stderr line it may not be reading.

### Impact

Silent data loss for machine consumers of the summary/report/event artefacts — the surfaces M1d built specifically for machine consumption.

### Recommendation

Rule deliberately: either a distinct exit code for "the run succeeded but a requested artefact could not be written", or document exit-0-with-stderr as the contract and add a test pinning it.

---

## $EIN_STDLIB is accepted with no validation while the checkout walk requires the manifest marker

**Severity:** Medium
**Confidence:** High
**Topic:** Error handling
**Classification:** code bug (diagnosis-cost / silent-misresolution hazard)

**Locations**
- `ein.rs/crates/ein-ir/src/stdlib.rs:113-136`

### Finding

The `$EIN_STDLIB` override is accepted without any check — no `MANIFEST.sha256` presence, no existence check (stdlib.rs:126-128) — while the checkout walk requires the marker. A typo'd or stale override yields "module not found at <path>" once per import rather than one "that is not a stdlib" root-cause error. Additionally, `resolve_default()` walking from `current_exe()` means a binary copied under an unrelated checkout containing `stdlib/MANIFEST.sha256` silently prefers that tree.

### Evidence

The marker exists precisely because "a directory called stdlib/ proves nothing" (stdlib.rs:32-34) — yet the highest-precedence source skips the proof entirely.

### Impact

Wrong-stdlib resolution is silent; a user diagnoses N import errors instead of one.

### Recommendation

Require the marker (or at least existence) on the override path, with a single readable refusal naming the env var.
