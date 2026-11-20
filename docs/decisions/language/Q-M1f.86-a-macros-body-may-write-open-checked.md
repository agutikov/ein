# Q-M1f.86 — A macro’s body may write `(open …)`, checked where it is called

A macro's body may write `(open ?R)` or `(open (p ?x))`, as any template: it is checked where a rule calls it,
as every primitive's shape is ([Q-M1f.67](Q-M1f.67-a-macros-body-is-checked-where-it-is-called.md)), and a macro
no rule calls is not read.

**Why.** `open` is a name a template may head; what the call writes is held to the shapes a rule's clause is held
to, where it is written.

**Held by.** `ein-ir/src/ein.y`'s `Template`; `ein_ir::from_ir`'s `refuse_malformed_primitives`;
`examples/ein-bugs/wrong-answers/open-macro-verdict.ein` and `open-macro-unused.ein`, which load.
