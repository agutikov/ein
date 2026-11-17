# Q-M1e.18 — `eq`, `neq` and `absent` are read only at their own arity

A kernel primitive is never read at an arity other than its own, and never drops an argument in silence: `eq` and
`neq` compare exactly two objects, `absent` reads exactly one premise, and `false` takes none. Written directly, any
other arity is a parse error at the token where the shape stops being the primitive's. Written in a macro's body, it
is refused at load after expansion ([Q-M1f.80](Q-M1f.80-a-malformed-primitive-a-macros-expansion-writes.md));
reached through a relation parameter that can hold `eq`, it is refused at load too
([Q-M1f.46](Q-M1f.46-a-relation-parameter-that-can-hold-eq-heads-only.md)). The compiler's own arity checks are
invariants no program reaches.

**Why.** A guard that read fewer arguments than were written would evaluate a weaker condition than the one stated,
a wrong answer with a success exit code; `(eq ?x A B)` would read as `(eq ?x A)`. Refusing the shape where it is
written is the cheapest diagnostic and the one a generated program most needs.

**Held by.** The grammar, `ein-ir/src/ein.y` (`Guard`, `Connective`). `ein-cli/tests/primitive_arity.rs`:
`the_unpinned_primitives_are_refused_at_a_wrong_arity`, `the_shape_pinned_primitives_refuse_a_wrong_arity`,
`a_refused_arity_names_its_position`; `examples/broken/eq_arity_low.ein`, `eq_arity_high.ein`,
`absent_arity_zero.ein` and `absent_arity_high.ein`.
