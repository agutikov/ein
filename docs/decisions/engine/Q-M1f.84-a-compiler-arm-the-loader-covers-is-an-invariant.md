# Q-M1f.84 — A compiler arm the loader covers is an invariant

Where the loader refuses a shape, the compiler does not refuse it again: its arm for that shape is an `assert!` or
`unreachable!` naming the loader's check. The nested `(or …)`, a `:match` past the register file, a goal's relation
variable, `absent`'s and `eq`'s arities and a conclusion that is no fact are each the loader's, and the compiler's
arms for them are invariants. The compiler refuses nothing the loader accepts.

**Why.** A refusal stated twice is a refusal one of whose lines no program can print; stated once, at load, it
reaches every command alike, whether or not a state ever activates the rule.

**Held by.** `ein-ir/tests/fuzz_properties.rs`: `the_compiler_refuses_nothing_the_loader_accepts`, every plan of
every accepted program compiled, one per activator, every query's goal too; `ein-infer/tests/compile_semantics.rs`:
`a_nested_or_is_refused_in_a_premise_and_inside_an_absent`, `the_lines_a_macro_body_reaches_are_the_loaders`. The
errors a library caller can still reach: [Q-M1f.75](Q-M1f.75-the-compilers-errors-are-a-library-callers.md).
