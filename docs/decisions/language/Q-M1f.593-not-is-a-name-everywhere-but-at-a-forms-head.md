# Q-M1f.593 — `not` is a name everywhere but at a form's head

`false` and `not` are the standard library's names: what a held `(false)` does is `std.core`'s rule
([Q-M1f.551](../stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)), and what `(not X)` beside `X` means is
`std.contradiction`'s and `std.negation`'s ([Q-M1f.557](../stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md),
[Q-M1f.559](../stdlib/Q-M1f.559-the-elimination-of-a-double-negation-is-a-module.md)). Each is a position word: a
name wherever a name goes, an argument, an activator's argument, a label, so `(clash-1 not)` and
`(relation not Claim)` are written and print bare. Where a form's head is read, the grammar gives each its shape:
`(not X)` is the stored negation of one fact `X`, `(false)` takes no argument, and `(not a)` and `(false k)` are
refused at the parse. A relation, a rule, a hypothesis rule and an import's alias may be named `false` or `not`; a
macro may not, since every `(false)` and every `(not X)` would be its call, `std.core`'s own among them.

The names a declarator may not bind are `eq`, `neq`, `and`, `or`, `absent`, `open`, `relation` and `__closed__`.
`eq` and `neq` stay the kernel's computed relations.

**Why.** The kernel reads neither to judge a state, so neither is a word of the kernel's vocabulary
([Q-M1f.436](../engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)); a theory that gives a meaning to a negation
has to name the relation `not` as any relation is named, to activate a rule for it or to declare it. Kept reserved,
the name was written quoted and drawn as two nodes of a knowledge base.

**Held by.** `ein-ir/src/ein.y`'s `Name`, `HeadName` and `DeclName`; `ein_core::LIBRARY_NAMES` and
`ein_core::binds_no_reserved`; `ein-ir/tests/imports_semantics.rs`'
`the_librarys_two_names_are_bound_by_a_relation_a_rule_and_an_hrule`; `ein-ir/tests/grammar_pages.rs`, a row of the
grammar's table per shape the parse refuses; `examples/beliefs/04_liar_denial_contradicted.ein`, which declares
`(relation not Claim)`. The page is [the reserved names](../../ein/user/04-reference/03-reserved-names.md).
