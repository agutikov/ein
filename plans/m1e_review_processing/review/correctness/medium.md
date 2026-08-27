# Correctness — Medium

## The inter-layer alive-∅ path records root as a model with no contradiction re-check

**Severity:** Medium
**Confidence:** Medium
**Topic:** Correctness
**Classification:** code bug (plausible risk — soundness rests on an unchecked invariant)

**Locations**
- `ein.rs/crates/ein-infer/src/solve.rs:1528-1551`

### Finding

phase2's inter-layer path calls `record_node(root)` when `compute_alive` comes back empty, **without** `has_contradiction` — unlike phase1 (:1091) and the cascade (:2131), which both re-check.

### Evidence

Soundness currently rests on two facts: the writebacks are `(not h)` for h ∉ root, and root has not been re-saturated on this path — so no derived `(false)` can exist *to* detect. But that second fact is itself the gap: negative-completion or total-style rules that would derive `(false)` from the new negatives never run at root here unless the cascade happens to fire. With obligations declared, the re-read tally (:1548) catches the owing case as Open; a program that encodes totality as a saturation `(false)` rule rather than as an obligation could in principle have root recorded as a "model" whose falsity only a fork would have derived.

### Impact

A wrong `Solution`/`Ambiguity` on a specific (currently out-of-corpus) program shape. Plausible-by-invariant, but unchecked and undocumented — the exact category the project's method treats as needing either a check or a written argument.

### Recommendation

Either add the `has_contradiction` re-check on this path (cheap: the path is rare) or write the invariant argument down beside the code and add a fixture with a saturation-encoded totality rule to pin whichever behavior is intended.

### Cross-references

- `review/open-questions.md` Q5.

---

## The Solution verdict arm prints `stats.solution_nodes`, which diverges from `Verdict::k` in the defined mixed regime

**Severity:** Medium
**Confidence:** Medium (code facts High; the regime is defined but unexercised)
**Topic:** Correctness
**Classification:** code bug

**Locations**
- `ein.rs/crates/ein-render/src/answer.rs:419-433`
- `ein.rs/crates/ein-cli/src/solve.rs:648`
- `ein.rs/crates/ein-infer/src/solve.rs:2428-2441`

### Finding

`finalise` admits a state where one node is a discharged model and others are open states (`branches.len()==1` with `open_states` non-empty → `Verdict::Solution`; solve.rs:2429-2440 says "no corpus entry is in that regime today … defined rather than measured"). There `stats.solution_nodes > 1`, and ein-cli passes it into `render_solution_table`, whose Solution arm prints it as `solutions (k) N` — so the table would print `k=2` beside `verdict Solution`. The Ambiguity arm computes its own distinct count and Open prints 0; only the Solution arm inherits the raw node count.

### Impact

The one regime S1d.2.6 defined without a corpus witness prints a self-contradicting read-out the day a program reaches it. S1d.3.3's whole point was that every count says what it counts.

### Recommendation

Print `Verdict::k()` in the Solution arm (as the verdict event already does), and add a synthetic fixture for the mixed regime.

### Cross-references

- `review/semantics/medium.md` — the k vs solution_nodes vocabulary split.

---

## Value::UNBOUND leaks through the accessors

**Severity:** Medium
**Confidence:** High (facts verified; impact latent)
**Topic:** Correctness
**Classification:** code bug (latent)

**Locations**
- `ein.rs/crates/ein-core/src/value.rs:65-71, 94-120`
- `ein.rs/crates/ein-core/src/facts.rs:122-124`

### Finding

`UNBOUND.tag()` returns `Tag::Fact` (the `>>30==3` fallthrough) and `UNBOUND.as_fact()` returns `Some(FactId(0x3FFFFFFF))` — a FactId the store can legitimately assign (the capacity check allows ids up to CAPACITY−1). Any consumer that calls `as_fact()`/`tag()` on a register value without first testing `is_unbound()` silently treats the sentinel as a real, astronomically-high-numbered fact.

### Evidence

The type's own test proves only that `pack()` can never *produce* UNBOUND, not that the accessors reject it. The sentinel's safety therefore rests on call-site discipline in ein-infer rather than on the type.

### Impact

No current misuse was found, but the failure mode is silent (a phantom fact id in a match), and the hardening is a few instructions.

### Recommendation

Make `as_fact()` (and `tag()`) reject the sentinel, or at minimum add a test asserting `UNBOUND.as_fact() == None` — which currently would fail, making the gap visible.

---

## macros.rs carries a second, laxer macro pipeline that is not what the loader runs

**Severity:** Medium
**Confidence:** High
**Topic:** Correctness
**Classification:** code bug (divergent duplicate semantics)

**Locations**
- `ein.rs/crates/ein-ir/src/macros.rs:50-76, 197-240`
- `ein.rs/crates/ein-ir/src/from_ir.rs:474-539, 685-695`

### Finding

`collect_macros` (first-declaration-wins, silently skips malformed forms) + `expand_rule_clauses` carry a doc comment claiming "what the loader does, and therefore the shape the parity gate compares" — while the actual loader path is `from_ir::ingest_macros` (duplicate = error, reserved = error) + `expand_pair` per rule. A reader trusting the comment models the wrong duplicate/reserved semantics; a non-loader caller (the dump/golden path) gets different macro registration than a load does.

### Impact

Duplicate implementations of one load-time semantics with divergent error behavior, plus a stale contract comment pointing at the wrong one.

### Recommendation

Route the secondary consumers through the loader's ingestion (or delete the parallel pipeline), and fix the comment either way.

### Cross-references

- `review/architecture/medium.md` — duplicated mechanisms as the drift generator.

---

## Resolver::locate derives module identity from the display string; embedded-root modules degrade silently

**Severity:** Medium
**Confidence:** Medium
**Topic:** Correctness
**Classification:** code bug (latent, environment-dependent)

**Locations**
- `ein.rs/crates/ein-ir/src/imports.rs:271-311, 179-186`

### Finding

`locate` canonicalizes the *display* string (`std::fs::canonicalize(&display)` at :304), which fails silently for the embedded root (identity falls back to `<embedded>/name`) and for any transiently-unreadable path; `base_dir` for a module's own nested imports is then `None` for embedded modules. Works today because stdlib modules only import `std.*` — but a stdlib module using a file-relative import would resolve under checkout/override roots and fail only under the embedded root, i.e. only in an installed binary.

### Impact

An environment-dependent behavior difference between the three stdlib sources that no test exercises (the test harness always sets `$EIN_STDLIB` per stdlib.rs:183); the failure would ship silently in release binaries only.

### Recommendation

Either forbid file-relative imports in stdlib modules (a load-time check) or give the embedded root a real base identity; add a test that loads through the embedded source.

---

## Saturator::is_stalled is not read-only: it leaves delta untaken and advances the tiebreaker

**Severity:** Medium
**Confidence:** High
**Topic:** Correctness
**Classification:** code bug (API-contract hazard)

**Locations**
- `ein.rs/crates/ein-infer/src/saturator.rs:627-641`

### Finding

`is_stalled` runs a full `enqueue_pass(s, None)` without taking `self.delta` (so stale delta facts can be re-seeded on a later `closure_step` — harmless via seen-dedup, but wasted work), and it advances the tiebreaker as a documented-intentional side effect — so merely *asking* whether the saturator is stalled perturbs subsequent enqueue ordering.

### Impact

An ostensibly read-only predicate is order-relevant. Inert for current callers; a trap for any new caller (an embedder probing quiescence mid-drive), and invisible outside a code comment.

### Recommendation

Take the delta or rename/document the method as effectful at the API level (the embedding page is where an embedder would look, and it says nothing).
