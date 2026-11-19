# Decisions: the stdlib

Decisions about the standard library as a whole: its layout, and the modules a puzzle reads. The calculator's are
in [`math/`](math/README.md), Boolean functions' in [`bool/`](bool/README.md).

- [Q16](Q16-general-rule-families-are-stdlib-modules.md) — General rule families are stdlib modules, and a problem's own rules are in its own file
- [Q25](Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md) — Ein has no ordinal form: an order is a relation a program declares
- [Q-M1f.338](Q-M1f.338-the-calculator-is-std-math-and-boolean-functions.md) — The calculator is `std.math.*`, and Boolean functions are `std.bool.*`
- [Q-M1f.551](Q-M1f.551-std-core-is-loaded-into-every-program.md) — `std.core` is loaded into every program
- [Q-M1f.557](Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md) — The contradiction is a theory a program imports
