# Maintainability — Low

## DEFAULT_PRIORITY's doc comment is arithmetically self-contradicting

**Severity:** Low
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-infer/src/saturator.rs:42-45`

### Finding

"Rules with no :priority sit between the eliminate band (300) and the hypothesis band (900)" — for a constant of 1000, which sorts after both; the stdlib's real bands span 90-500 and no 900 band exists. The only in-code statement of where undeclared-priority rules schedule is wrong in the direction that matters (they fire last, not mid-band).

---

## A literal run of ~22 spaces inside the non-exhausted Contradiction headline

**Severity:** Low
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-render/src/answer.rs:232` (vs the single-space wording at `:507`)

### Finding

Looks like a lost line-continuation when the string was reflowed (every sibling string in the file uses `\` continuations). It reaches output through `render_answer` on every non-exhausted k=0 run; no test or golden pins it (the corpus banks only an md5 digest, which cannot reveal the oddity).

---

## summary.rs's write() doc comment contradicts the JSON writer's tested behavior

**Severity:** Low
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-cli/src/summary.rs:634-638` vs `ein.rs/crates/ein-render/src/dump/json.rs:153-184, 219-225`

### Finding

The comment claims `json.dumps(summary, indent=2, ensure_ascii=False)`; the writer escapes all non-ASCII and its own test asserts "ensure_ascii=True is CPython's default and no caller overrides it". One of the two is wrong about the parity target; if ein.py really passed ensure_ascii=False for the summary, the ported bytes differ for any non-ASCII `:why` content.

---

## sanity -y re-saturates parents with a fresh memo, polluting the live event stream

**Severity:** Low
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-infer/src/sanity.rs:137-152`

### Finding

`check_commutativity` builds the parent-path Session with `SharedMemo::default()` while the direct path uses the run's shared memo — every checked commitment recompiles all plans per parent (cost only), and the recompiles narrate `compile` events into the run's live stream, so a `-y` run's stream differs from a plain run's by more than the check itself. Diagnostic-only flag; the inconsistency looks unintentional.

---

## imports.rs predicts a refactor that never happened, above the list that then drifted

**Severity:** Low
**Confidence:** High
**Topic:** Maintainability

**Locations**
- `ein.rs/crates/ein-ir/src/imports.rs:42-51`

### Finding

":46-48 — 'P1a.3 brings the registries over and this becomes a query against them'" — it never did, and the hand copy below it is the one that missed `open` (see `review/correctness/high.md`). The comment now actively misleads: it explains why the duplication is temporary, and it isn't.
