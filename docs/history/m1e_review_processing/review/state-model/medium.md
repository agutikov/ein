# State model — Medium

## The M1 alive-set invariant — which licenses state-key dedup and per-KB alive recompute — is enforced nowhere

**Severity:** Medium
**Confidence:** High
**Topic:** State model
**Classification:** implementation gap (documented honestly, still the largest unenforced soundness warrant)

**Locations**
- `docs/kernel/inference/README.md:140-187`
- `docs/kernel/inference/implementation.md:190-192`

### Finding

The invariant — rules assert no new objects, no new relations, hypotheses connect existing names only, so `alive` is a pure function of the closed KB — is what licenses both the per-KB alive recompute and state-key dedup soundness (and, since M1d, the tree traversal's exhaustiveness-by-discharge argument). The docs state outright it should be "promote[d] to a typed invariant check when F5 lands"; until then a rule library that asserts a new `(relation …)` or introduces new names silently invalidates the soundness warrant with no diagnostic.

### Evidence

`docs/kernel/inference/README.md:184-187` (the admission); no check exists in ein-infer (the saturator will happily store a derived fact naming a fresh symbol — nothing compares the derived name set against the load-time one).

### Impact

The entire model-counting story (`k`, dedup, exhaustion) is conditional on a property only the stdlib's conventions maintain. A third-party rule module is exactly the input M2 plans to generate.

### Recommendation

A cheap post-fixpoint check (derived facts' symbols ⊆ load-time symbol set, derived relations ⊆ declared∪auto-vivified) behind a debug assertion or a diagnostic; it need not wait for the F5 typed form.

### Cross-references

- `review/correctness/high.md` (the tree traversal leans on the same warrant).
