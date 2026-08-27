# Code ↔ doc consistency — Medium

## Three current-kernel pages attribute the two-phase loop to a nonexistent `Engine::step()`

**Severity:** Medium
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `docs/kernel/inference/absent_semantics.md:124-127`
- `docs/kernel/inference/architecture_and_algorithms.md:157-159`
- `docs/kernel/inference/implementation.md:66`
- actual: `ein.rs/crates/ein-infer/src/saturator.rs:540` (`Saturator::step`)

### Finding

All three hyperlink `engine.rs`, which has no such method — engine.rs is a compile cache; the loop is `Saturator::step` and it is queue-based, not queue-less. Survived from ein.py's `Engine`; the S1a.10.6 doc pass missed it in all three places.

### Recommendation

Fix the symbol and the links in all three pages.

---

## events.md misdocuments payloads and omits an emitted event

**Severity:** Medium
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `docs/kernel/inference/events.md:129, 133, 172-183`
- `ein.rs/crates/ein-infer/src/saturator.rs:1127-1135`; `ein.rs/crates/ein-infer/src/engine.rs:184-190`; `ein.rs/crates/ein-infer/src/solve.rs:908-913`

### Finding

The schema table claims `watched` on all of park/admit/retire — `admit` carries none; `compile.n_guards` is actually the disjunct count, a knowingly wrong number kept only as a comparison surface (the truth lives in an engine.rs comment); and the `traversal` event the tree-decline path emits (`{kind:'tree', verdict:'declined', reason:…}`) has no row at all, on a page that claims to be "every step the engine took". events.md sells itself as the schema for external observers (M20's likely feed); a consumer coding to the table will look for a field admit never carries and misread n_guards.

### Recommendation

Fix the two payload rows; add the traversal row (marking it experimental if T1d.10.6.4 wants the freedom).

---

## features.md's own corrections were not propagated to the prose that cites them

**Severity:** Medium
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `docs/kernel/inference/features.md:170-171, 175-179, 199-203`
- `docs/kernel/inference/architecture_and_algorithms.md:683-688`

### Finding

§Two corrections establishes 3 557 (not 3 831) enterings / 54.5× and 101 (not 134) / 1.1×, and claims "the two conclusions that rested on them are amended where they stand" — yet the per-lever notes still read "101 → 3 831 (38×) … 56.6×" and "134 commitments … exactly the 1.2× that implies … 33 more dead ends", and architecture_and_algorithms.md still quotes 3 831/56.6×. The correction section and the corrected-by section disagree inside one file.

### Recommendation

Apply the amendment where the numbers stand, as the corrections section claims was already done.

---

## docs/api/rust.md has rotted outside its marker-guarded region

**Severity:** Medium
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `docs/api/rust.md:266-267`
- `ein.rs/crates/ein-cli/tests/embedding.rs:149`

### Finding

The page says the other verdict arms are exercised by `the_other_two_verdicts_are_reachable` and "the match is not three arms"; the test was renamed `the_other_three_verdicts_are_reachable` when the Open arm arrived, and the match now has five arms (four verdicts + Aborted). The prose sits outside the marker-delimited region the page-quotes-this-file test diffs — the one class of drift the mechanism structurally cannot catch, demonstrated on the mechanism's own page.

### Recommendation

Fix the prose; consider widening the marker or adding a second marker around the test-name sentence.

---

## stdlib/README documents a CLI command that does not exist

**Severity:** Medium
**Confidence:** High (verified: "error: unrecognized subcommand 'ir'")
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `stdlib/README.md:212-217`

### Finding

`ein ir parse --resolve` is offered as the way to inline imports; the CLI refuses it. The removal is even recorded elsewhere (utils/README.md:35-36), and the two docs disagree on when it happened (P1.7c vs render_examples.sh:26's P1.11).

### Recommendation

Replace with the current route (`ein render`-adjacent or the library call) and reconcile the two removal dates.

---

## examples/README points at the deleted Python engine, and the two-encodings agreement claim has no named owner

**Severity:** Medium
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug + possible test gap

**Locations**
- `examples/README.md:5, 27, 28-29`

### Finding

"see docs/api/ to drive them from Python" (no Python module exists — docs/api's whole point), and `acceptance/test_zebra_two_ontologies.py` (no acceptance/ directory anywhere). The claim that dead pointer carried — zebra.ein and zebra2.ein agree cell by cell — now has no named enforcement; whether anything in cargo test still pins the two encodings to the same model was not established by this review (see `review/open-questions.md` Q10). Also the dangling "C2" reference (:28-29; same in stdlib/README.md:139-140) — bare link text whose target is gone.

### Recommendation

Fix the pointers; name (or add) the test that pins the two encodings to one model.

---

## utils/README attaches the wrong reason to the 29 no-solve corpus entries

**Severity:** Medium
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `utils/README.md:51` vs `utils/openness_census.py:130-134`, `corpus/README.md:151-162`

### Finding

The README says "29 entries drop solve because it does not terminate on them"; the script and corpus/README say non-termination covers four — the other 25 drop solve because a solve does not ask their question. The count is right; the causal claim is false for 86 % of the set.

### Recommendation

Split the sentence into the 4 + 25.

---

## architecture_and_algorithms.md mixes as-built ein.rs facts with as-was ein.py vocabulary without marking which is which

**Severity:** Medium
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `docs/kernel/inference/architecture_and_algorithms.md:62-73, 193, 434-439`
- `ein.rs/crates/ein-infer/src/firing.rs` (no `classes.union` call)

### Finding

The deductive-layer file list names saturator.rs and firing.rs twice each; O4 says EqClasses is "wired into the API so firing can call kb.classes.union" — firing.rs contains no such call (equality is a stub; matching does not resolve classes, pinned by `naf_semantics::matching_does_not_resolve_equality_classes`); section 3's type table uses ein.py names (JoinPlan, World, Scan/Join opcodes) the Rust engine renders differently. implementation.md:69 marks the World divergence; this page doesn't — the two designated as-built references disagree on how faithfully to describe the port.

### Recommendation

Dedupe the file list, fix O4 to "stub", and mark the ein.py vocabulary as historical where kept.
