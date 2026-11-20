# Q-M1f.573 — Under `open` a relation is owed and a fact is a hypothesis, and H is root's

Under `open`, read at load: **a relation's name, or a parameter that names one, is what an obligation owes**, and
**a fact, or a variable the `:match` binds, is a hypothesis**; a variable bound to a name gives none. A hypothesis
root holds or denies is no candidate, and $H$ is read once, at root: one first held above root is held and not
proposed.

**Why.** A rule's conclusion says which: the relation it owes, or the fact it supposes.

**Held by.** `ein_ir::from_ir`'s `validate_open`; `ein_infer::hypothesis_rules`, which reads a rule's `(open F)` conjuncts.
