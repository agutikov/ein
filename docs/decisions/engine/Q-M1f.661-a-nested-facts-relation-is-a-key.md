# Q-M1f.661 — A nested fact's relation is a key of the participation index

The participation index keys a fact by each name at an argument, `(rel, slot, DIRECT, name)`, and, where the
argument is a fact, one level inside it: each name at position `inner` of that fact, `(rel, slot, inner, name)`, and
that fact's relation, `(rel, slot, RELATION, R)`, the context, the argument and the relation. So a premise
`(not (R ?a ?b))` with nothing bound walks the stored negations of `R` and not every negation the state holds, and the
saturator hands a new `(not (R …))` to the plans that read a negation of `R` and to those whose premise leaves the
relation open, and to no other, a plan listed under the key of its premise's first nested pattern. Among a step's
compiled probes the relation comes last, after every argument that can be bound, and only where no constant came
before it.

A position inside a fact whose number is `RELATION`'s, which a fact of as many arguments as a `u16` counts has, is
not keyed, by the index, the compiled probe list and the scan alike, so a position's key and a relation's never
share a bucket.

**Why.** With no name bound, a premise over a stored negation, or over a context whose fact holds names, would walk
the whole held extent of its relation; the relation of the fact at the argument is the narrowest key it has, and a
bound argument a narrower one, tried first. The index only narrows: the matcher unifies every slot of every
candidate, and a bucket keeps the extent's order, so the matches and their order are the scan's. The relation is
keyed by value, whatever its name ([Q-M1f.436](Q-M1f.436-the-kernel-reads-no-theorys-name.md)). A position kept out
of the relation's number keeps each bucket to what its key says.

**Held by.** `ein_core::SlotKey::relation` and `SlotKey::keys_inner`, which `ein_core::Kb::index_fact` reads; the
compiler's probe list (`compile`'s `probes_for`) and `ein_infer::match_::Matcher::probe_key`; the saturator's
`PlanIndex`; `kb.rs`' `the_participation_index_keys_the_join_types_and_one_level_in`; `compile.rs`'
`the_nested_relation_is_the_last_probe` and `a_nested_pattern_with_no_keyable_slot_has_no_probe`; `match_.rs`'
`narrowing_never_changes_the_match_sequence`. The design pages:
[`03-data-model.md` § 6.1](../../ein/ein.rs/03-design/03-data-model.md#61-the-participation-key) and
[`05-matcher.md` § 5.2](../../ein/ein.rs/03-design/05-matcher.md#52-a-probe-into-the-participation-index).
