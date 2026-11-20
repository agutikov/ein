# Q-M1e2.10 — An obligation whose domain can grow writes its candidates at root, and its program is flagged

An obligation reads its candidates off its guard at the state. Where its guard, beside its witness, or its trigger
reads a relation a commitment can change, the set can grow above root: the obligation writes what root's guard
gives, as any obligation does, and the program is one the static check flags
([Q-M1f.579](Q-M1f.579-an-obligation-whose-guard-or-trigger-reads-what.md)). Nothing is claimed of its answer.

**Why.** An atom of the domain that only a commitment derives is in no candidate scanned at root, so the answer can
miss the model it licenses; such a program is detected and warned of, not answered by another generator.

**Held by.** `ein_infer::monotone`'s `obligation-guard` and `obligation-trigger` sites;
`examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein` and `oblig-stdlib-total-owed.ein`, each flagged.
