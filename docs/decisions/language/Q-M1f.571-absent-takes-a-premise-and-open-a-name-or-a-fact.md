# Q-M1f.571 — `absent` takes a premise and `open` a name or a fact; under an ordinary relation both are names

`absent` takes a premise: a pattern, or a conjunction, which may hold an `absent`, the universal; only the direct
`(absent (absent P))` is refused. `open` takes a relation's name or a fact. **Under an ordinary relation both are
ordinary names**: `(think Ann (open (p A)))` and `(think Ann (absent (p A)))` are facts whose inner relation is so
called, and a premise `(think ?w (absent (p ?x)))` matches such a stored fact and tests no absence. A written or
concluded `(absent P)` at the top stays refused: there it is the kernel's word, and no fact.

**Why.** The kernel's words have their meaning where the kernel reads them, at the head of a premise, a written fact
or a conclusion; inside a fact of an ordinary relation nothing reads them, and a word there is what any word is.

**Held by.** `ein-ir/src/ein.y`'s `ValueHead`, `OpenHead` and `OrdinaryHead`; `ein_ir::from_ir`'s
`refuse_malformed_primitives`; `examples/ein-bugs/wrong-answers/absent-fact-nested.ein`, which loads.
