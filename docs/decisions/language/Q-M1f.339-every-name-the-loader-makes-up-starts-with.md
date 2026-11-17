# Q-M1f.339 — Every name the loader makes up starts with `#`

A name the parser, the loader or the engine makes up for something a program writes no name for starts with `#`:
`#empty` for the empty form `()`, `#nested` for a nested form whose head is not a name, `#params` for a parameter
list, `#list` for a fact's `:using` list, and `#query` for the rule a query's goal compiles to. `#` is no name
character, so no program can spell one of them, and a name a program writes is never one the engine made up.

**Why.** Were a made-up name spellable, a program's own name could collide with it: `(p ())` and a fact written
with the made-up name would load as one fact.

**Held by.** `ein_core::text::INTERNAL_NAMES` and `INTERNAL_MARK`; `ein-ir/tests/generated_parser.rs`:
`no_name_the_loader_makes_up_is_a_name`.
