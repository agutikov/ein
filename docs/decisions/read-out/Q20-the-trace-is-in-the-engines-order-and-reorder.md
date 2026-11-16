# Q20 — The trace is in the engine's order, and `--reorder` groups it by entity

`--trace` writes each section's steps in the order the engine took them. `--reorder` groups each section's steps by
the entity they are about, under *About X*, every step once, keeping its number and its order within the group. It
moves no line of stdout.

**Why.** The engine's order is the derivation as it happened, which every other surface reports; a reader who
follows one entity at a time asks for the grouping, and the steps keep their numbers so the two views can be read
against each other.

**Held by.** `ein_render::trace::region`'s `steps`; `ein-cli/tests/region_modes.rs`:
`the_trace_takes_its_three_shaping_flags`, the grouped trace holding every step of the plain one and stdout unmoved.
