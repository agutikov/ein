# Q-M1f.579 — An obligation whose guard or trigger reads what a commitment can change writes at root, and is flagged

An obligation whose guard, beside its witness, or whose trigger reads a relation a commitment can change never
steps aside: it writes its guard's candidates at root as any obligation does, and the program is one the static
check flags, by the trigger's site, `obligation-trigger`, or the guard's, `obligation-guard`. Nothing is claimed of
its answer.

**Why.** A value the guard gains at a node is in no candidate root wrote, so the answer can miss what it licenses;
detected, the program is answered and warned of, and nothing reads a different generator in its place.

**Held by.** `ein_infer::monotone`'s `obligation-guard` site; `ein_infer::oblgen`, which reads every activated
obligation's candidates; `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein` and its siblings, each flagged.
