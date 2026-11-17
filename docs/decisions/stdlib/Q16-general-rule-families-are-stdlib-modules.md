# Q16 — General rule families are stdlib modules, and a problem's own rules are in its own file

The rules that hold of any problem of a kind ship as modules of the standard library, each imported by name: the
relation algebra and its closures (`std.algebra`), closed-world elimination (`std.elim`, `std.bijection`,
`std.slots`), membership and what a declaration's labels are held to (`std.typing`), the closure of a functional and
total relation (`std.closure`) and the pattern macros (`std.macro`). A problem's own rules are ordinary declarations
in its own file. Nothing is loaded that a program neither writes nor imports, and the program's text is the one
place its rules are configured.

**Why.** A rule family written once and imported is one text for every problem that uses it, and one that a test
holds; a rule only one problem needs stays where it is read. A bundle the engine loaded by itself would be a
second, hidden source of a program's rules.

**Held by.** The modules under `stdlib/`, each with its programs under `tests/stdlib/`;
`ein-infer/tests/stdlib_semantics.rs`; `examples/zebra.ein` and `examples/zebra2.ein`, one puzzle over `std.slots`
and over `std.bijection`, each with its own rules in its own file.
