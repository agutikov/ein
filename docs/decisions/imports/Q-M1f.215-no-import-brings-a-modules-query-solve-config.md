# Q-M1f.215 — No import brings a module's query, solve, config or trace

A module may be a program of its own, with a `(query …)`, a `(solve …)`, a `(config …)` and a `(trace …)`. No
import brings any of the four, whatever its form: they are dropped where the module is read. The importer's
queries, lattice, flags and trace are the ones it writes.

**Why.** Those four forms state a question about a program, not knowledge a program shares. Brought in, the
importer would answer a query it never asked, under a lattice and flags it never stated, which is false of the
program as written.

**Held by.** `ein-ir/tests/imports_semantics.rs`: `no_import_brings_a_modules_query_solve_or_config`, the four
forms under a bare import, `:as` and `:symbols`, through a module that imports the module, and loaded.
