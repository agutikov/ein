# Maintainability — Medium

## `phase_2_done` is dead scaffolding with an explicit warning-suppressor

**Severity:** Medium
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-infer/src/solve.rs:1160, 1162, 1525, 1566`

### Finding

Declared `false`, tested by two `break`s, never assigned true anywhere (grep confirms only the four sites), and the loop body ends with `let _ = &mut phase_2_done;` — an explicit suppressor for a variable that cannot change. Both breaks are unreachable. It reads like the residue of a removed early-exit and either hides a missing feature or should be deleted; either way it misleads a reader of the solve loop's control flow.

### Recommendation

Delete it, or restore whatever early-exit it was for with a test.

---

## Stale rustdoc contradicting the code it documents

**Severity:** Medium
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-infer/src/commitment.rs:109-116` vs `solve.rs:790-799` (claims `resume` is never `Some` on shipping paths; the default path passes root's snapshot at four call sites)
- `ein.rs/crates/ein-infer/src/solve.rs:193-198` vs `ein.rs/crates/ein-cli/src/solve.rs:584` (claims `--dump-states` sets `store_lattice`; only `--trace` does — `--dump-states` builds a MonotonicDumper without a proof)

### Finding

Both are the kind of comment a reviewer would otherwise trust as the contract, and both state the opposite of the shipping behavior.

---

## `LatticeStats.state_key_merges` is a named counter that never counts, while the engine demonstrably merges

**Severity:** Medium
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-infer/src/solve.rs:156, 501, 627, 2468` vs `:2198-2208`

### Finding

Initialised to 0, copied into the proof, never incremented — while `record_node`'s replacement path *is* a state-key merge (a comment nearby records "calls this 1 221 times to keep 22 nodes" on branching/06). lattice_semantics.rs:27-29 frames the zero as deliberate port scope, but a named counter silently under-reporting a thing the engine does invites wrong conclusions from `proof_summary.json`.

### Recommendation

Increment it or remove the field from the emitted proof.

---

## Numeric drift across load-bearing in-code comments

**Severity:** Medium
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-infer/src/solve.rs:739-741, 2494-2500`; `verdict.rs:52-53`; `ein.rs/crates/ein-render/src/answer.rs:558-560`; `ein.rs/crates/ein-infer/src/expect.rs:264-265` vs `explain.rs:544-548` and the census documents

### Finding

119/146 vs 92/121 fixpoint-entry counts, eleven vs twelve entries moved to Open, 126/39 vs 123/38 zebra2-bad witnesses — the comments disagree with each other and with the censuses, and a reader cross-checking cannot tell which snapshot is current.

### Recommendation

Cite the census document instead of inlining its numbers, or date each number.

### Cross-references

- `review/documentation/medium.md` (the same rot class in READMEs).
