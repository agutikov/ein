# Q-M1f.9 — A shape the grammar refuses is a loader invariant

The loader does not refuse a shape the grammar already refuses. Where it matches on the tree's shape, its arm for a
shape no parsed program has is an `expect`, `unreachable!` or `assert!` naming the production of `ein.y` that states
what was wanted, and the readers that can no longer fail say so in their types. A tree built by hand through the
library, past the grammar, panics there; no `ein` command reaches such a panic, since every path into the loader goes
through the parser, a `.einb`'s stored program included. A shape a macro's expansion can produce stays a load error.

**Why.** The grammar is the one statement of a shape; a second refusal of it in the loader would be a line no
program can print. A panic naming the production is louder than a load error for the one caller that can reach it.

**Held by.** The arms of `ein-ir/src/from_ir.rs` that name `ein.y`'s productions (`Form`, `Relation`, `Rule`,
`FactHead`, `FactArg`, `Bool`); `ein-ir/tests/fuzz_properties.rs`: `generated_input_is_always_parsed_or_diagnosed`.
The same pattern one layer down: [Q-M1f.84](Q-M1f.84-a-compiler-arm-the-loader-covers-is-an-invariant.md).
