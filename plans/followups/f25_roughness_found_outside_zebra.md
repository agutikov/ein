# F25 — Roughness found outside the zebra programs

**Filed 2026-11-11**, on the user's instruction of that day, at the start of M1f
[P1f.7](../m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md):

> I don't want to review any new findings that does not affect existing zebra problems - so if find anything -
> either fix in place if it is quick or record into followups; I accept that language and engine MAY still contain
> some impurities, roughness, it's OK, we can return to them later when have real applications and corpus of real
> Ein programs (that starts from plans/m1f_hypothesis_and_documentation/p1f.6_calculator)

**What it owns.** Every finding a stage makes outside its own scope that moves no answer of an existing zebra
program (`examples/zebra*.ein` and the files generated from them) and is not quick to fix where it was found. A
finding is a row below: what, the smallest witness, where it was found, and the answer today. Nothing here is a
question for the user, and nothing here is a stage. A finding that is a wrong answer also gets a row in
`corpus/wrong_answers.toml`, `open` and owned by this followup, when its stage can file one at no cost; otherwise
the row here is the record.

**Trigger.** A corpus of real Ein programs exists (from [P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md)'s
calculator onward) and one of them meets a row; or the user asks for the list.

## The items

| id | what | witness | found by | today |
|---|---|---|---|---|
| R-1 | **`(not ?x)` concluded where the loader cannot foresee that `?x` binds a name.** S1f.7.4 refuses the shape where a name is provable at load; a binding that arrives under a commitment or through an undeclared relation is not. What the firing does with it is not decided | S1f.7.4's records | the analysis of 2026-11-11 | to be taken by S1f.7.4's T1 |
