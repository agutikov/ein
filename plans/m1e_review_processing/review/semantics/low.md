# Semantics — Low

## The two entering-timeline emitters write the same event with different key orders

**Severity:** Low
**Confidence:** High
**Topic:** Semantics
**Classification:** design ambiguity

**Locations**
- `ein.rs/crates/ein-render/src/dump/state.rs:129-143`
- `ein.rs/crates/ein-render/src/dump/lattice.rs:140-152`

### Finding

MonotonicDumper emits `layer/outcome/commitment/kind/firings/facts_merged/unsat_core_size/nogood_*`; LatticeDumper emits `layer/outcome/commitment/facts_merged/nogood_*/kind/firings/unsat_core_size`. The JSON writer preserves insertion order as a document property (its whole reason to exist), so `00_timeline.jsonl` records for the same conceptual event differ in shape between the two dumpers, and nothing says why at either site.

### Recommendation

Pick one order (or extract a shared emitter) unless a parity constraint requires the difference — in which case say so at both sites.

---

## Two different sets are both named RESERVED

**Severity:** Low
**Confidence:** High
**Topic:** Semantics
**Classification:** design ambiguity (terminology)

**Locations**
- `ein.rs/crates/ein-ir/src/lex.rs:128` (11 SYMBOL-excluded lexer words)
- `ein.rs/crates/ein-core/src/terms.rs:191` (9 shadow-check names, incl. `open`)
- `docs/kernel/ir/03-ein-lang/00_ebnf.md:51-62`, `docs/kernel/defined_behaviour.md:326-332`

### Finding

Both docs are individually accurate, but a reader of 00_ebnf's "eleven RESERVED words" plus defined_behaviour's "`open` joined RESERVED" would conclude the lexer set grew to twelve — it did not (`(open ?R)` must still lex as a SYMBOL). Same name, two different semantic sets, three copies total (imports.rs carries a third, drifted one — see `review/correctness/high.md`).

### Recommendation

Rename one set (e.g. LEXER_KEYWORDS vs SHADOW_GUARDED) in both code and docs.
