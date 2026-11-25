# Q-M1f.609 — Above root, the blind enumerator and a later debt write what root proposed, held to the state

Where the pass writes a state's `open` facts above root
([Q-M1f.655](../engine/Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md)), two of the writers are not
asked again. **The blind enumerator** writes, at a state above root, the facts it proposed at root before any filter,
less those the state holds or denies and those of a relation the state closes; it writes nothing above root where it
wrote nothing at root ([Q-M1f.576](Q-M1f.576-the-blind-enumerator-writes-only-where-root.md)). **A later debt**,
what root's obligations propose for a debt only a commitment brings
([Q-M1e.11](Q-M1e.11-h-holds-what-a-debt-only-a-commitment-brings.md)), is written the same way: what root proposed for it, less what the state holds or denies. A rule that concludes a
hypothesis and an activated obligation are read at the state itself.

```lisp
(relation happy P :why "{?2} is happy")
(thing ann)
(thing bob)
```

Root holds `(open (happy ann))` and `(open (happy bob))`, the blind enumerator's; `ein solve FILE -p` lists the
state of the commitment `{(happy ann)}` with `(happy ann)` and `(open (happy bob))`, and the state of both with
neither `open` fact.

**Why.** What the blind enumerator guesses about, the program's names and its declared relations, is the same at every
state ([saturation § The alive-set invariant](../../ein/reasoning/02-kb-evolution/02-saturation.md#the-alive-set-invariant)),
so its list at a state is root's list held to the state, and a second enumeration per state would cost and say
nothing more. H is root's ([Q-M1f.573](Q-M1f.573-under-open-a-relation-is-owed-and-a-fact.md)), so a later debt
proposes above root what it proposed at root, and no state is told of a candidate H does not hold.

**Held by.** `ein_infer::open`: `Writers`, whose `blind` and `later` root reads once, and `hypotheses_at`, which holds
them to each state, the one reading at root and at every state above it; the example, by `ein solve FILE -p`.
