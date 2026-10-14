# Architecture — Medium

*Caveat: the dedicated architecture-review agent did not complete (see `review/summary.md` § Method); these findings come from the system-reconstruction pass and are correspondingly conservative.*

## Hand-maintained parallel copies of single-semantic artifacts are the project's recurring drift mechanism — and it has now bitten

**Severity:** Medium
**Confidence:** High (three observed instances, one with a behavioral consequence)
**Topic:** Architecture
**Classification:** architectural drift

**Locations**
- `ein.rs/crates/ein-ir/src/imports.rs:49-51` vs `ein.rs/crates/ein-core/src/terms.rs:184-193` (reserved names — drifted, behavioral consequence: see `review/correctness/high.md`)
- `ein.rs/crates/ein-ir/src/macros.rs:50-76,197-240` vs `ein.rs/crates/ein-ir/src/from_ir.rs:474-539` (two macro pipelines — divergent error semantics)
- `ein.rs/crates/ein-render/src/dump/state.rs:129-143` vs `dump/lattice.rs:140-152` (two timeline emitters — divergent key order)
- `run_tests.sh:56-62` vs `.github/workflows/per-commit.yml:5-9` (gate vs CI step lists — convention only, and the repo already lived through the failure once per both files' own headers)

### Finding

Four instances of one semantic artifact maintained as two hand-synchronized copies. One has already produced a real behavioral divergence (the reserved-name bypass), one produced divergent event shapes, and one reproduced an incident the repo documents (three red CI commits). In two of the four cases a comment *predicting the unification* exists and the unification never happened (imports.rs:46-48).

### Impact

Each copy-pair is a standing invitation for the next milestone to update one side. The project's own method (checks over convention, "a local gate that is a subset of the remote one is a local gate that lies") argues for mechanical unification or mechanical comparison in every case.

### Recommendation

One constant for reserved names; one macro-ingestion path; one timeline emitter (or a stated parity reason); a test that diffs run_tests.sh's step list against per-commit.yml (both are simple enough to parse).

---

## Verdict read-out ownership is split across three crates, and each split point has already produced an inconsistency

**Severity:** Medium
**Confidence:** Medium
**Topic:** Architecture
**Classification:** architectural drift

**Locations**
- `ein.rs/crates/ein-infer/src/solve.rs:2386-2445` (`finalise` — the one verdict constructor)
- `ein.rs/crates/ein-render/src/answer.rs:419-487` (the table recomputes/qualifies counts per arm)
- `ein.rs/crates/ein-cli/src/solve.rs:648`, `ein.rs/crates/ein-cli/src/test.rs:784-838` (each passes/prints its own choice of count)

### Finding

`Verdict` is computed once in ein-infer, but what a user *reads* is assembled downstream: the Ambiguity arm computes its own distinct count, the Solution arm inherits the raw node count, `ein test`'s header prints the search count under the verdict's letter, and the summary emits both numbers. Every observed k-vs-solution_nodes inconsistency (see `review/correctness/medium.md`, `review/semantics/medium.md`) lives at one of these seams — none inside `finalise` itself.

### Impact

Adding the next verdict word or count qualifier requires touching all three crates coherently; S1d.2.6/S1d.3.3 demonstrably missed two sites.

### Recommendation

Have `Verdict` (or a read-out struct beside it) carry the printable counts and qualifiers, so downstream surfaces render rather than recompute.
