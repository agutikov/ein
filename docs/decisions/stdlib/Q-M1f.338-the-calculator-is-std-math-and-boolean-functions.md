# Q-M1f.338 — The calculator is `std.math.*`, and Boolean functions are `std.bool.*`

A module's name is its path under `stdlib/`, a dot a directory: `std.math.decimal-arith` is
`math/decimal-arith.ein`. The calculator's modules are under `std.math`: the tables `digits` and `bits`, `arith`,
what every base shares, the numerals `decimal` and `binary`, the shared rules `positional`, the arithmetic
`decimal-arith` and `binary-arith`, `convert`, `int`, `rat`, `gcd` and `num9`. Boolean functions and circuits are
under `std.bool`: `gates`, `props`, `circuit` and `inputs`. The modules a puzzle reads, `std.algebra`,
`std.bijection`, `std.closure`, `std.elim`, `std.macro`, `std.slots` and `std.typing`, are at the top. A module's
place renames no relation: the decimal requests keep their bare names, `add` and `sum-of`, and binary's carry
`bin-` ([Q-M1f.267](math/binary/Q-M1f.267-binarys-names-carry-the-prefix-bin.md)).

**Why.** One subject a directory: a reader finds the calculator's modules together, and the directory is the first
part of every full name a module declares.

**Held by.** `ein-ir/src/stdlib.rs`, which resolves a module's name to its file in the checkout and in the copy the
binary embeds; `ein-infer/tests/stdlib_coverage.rs`: `the_inventory_reads_every_module_the_manifest_lists`;
`ein-ir/tests/digit_tables.rs`: `a_flat_import_brings_a_table_from_the_embedded_copy`.
