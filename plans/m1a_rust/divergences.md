# M1a — divergence ledger

Differences between `ein` (Python, the oracle) and `ein.rs` that are
**accepted** rather than fixed. Empty is the goal at the
[P1a.5](p1a.5_presentation/README.md) byte gate; a non-empty ledger is
allowed only with a written reason per entry.

The precedent for this shape is
[`docs/kernel/inference/parity_baselines.md`](../../docs/kernel/inference/parity_baselines.md),
which recorded the tree-vs-monotonic divergences explicitly rather than
treating them as failures or hiding them behind `xfail`. The difference
is the standard: that comparison was between two *different engines*;
this one is between an engine and its port, so the bar for an entry is
much higher.

## Rules

1. An entry needs **"what would make this unacceptable"** — a stated
   condition under which it becomes a bug. An entry without one is not a
   decision, it is a shrug.
2. An entry needs a **fixture** in the corpus that demonstrates it, so it
   cannot silently widen.
3. Anything on the [normalisation list](design/01_parity_contract.md) §5
   is *not* a divergence — that list is closed and lives in the design
   doc. Adding to it requires an [open question](open_questions.md).
4. When an entry is fixed, keep it with `**Status:** fixed in <stage>`.
   The trail is the memory.

## Template

```markdown
### D<n> — <one-line title>

**Found:** <date>, <phase/stage>
**Tier:** T<k>
**Status:** accepted | fixed in S1a.<p>.<s>
**Fixture:** <corpus entry>

**What.** <the observable difference, both sides quoted>

**Why it is acceptable.** <argument>

**What would make it unacceptable.** <the condition>
```

## Entries

*(none yet)*
