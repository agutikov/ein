# Q-M1f.20 — On route 2 describe says no licence, and a mode names `L7 the cube`

On route 2, the cube, the `route` row of a description and of `ein oracle` reads *no licence*, with why: *the static
check flags the program*, or *asked for by name*. A mode's `route` row on the same route, and the summary's
`region.licences` on every question, name `L7 the cube`. *No licence* there means no licence that lets the route skip
a node; L7 is the cube's own, which holds of every program.

**Why.** The two say one thing in two registers: describe's row tells a reader that nothing was skipped, the summary
lists the licence the route stands on. Neither misstates an answer.

**Held by.** `ein_render::read_out`'s `describe_rows`, which writes describe's row;
[read_out.md](../../kernel/inference/read_out.md) § 2's `route` row and § 11's `region.licences`, held to the printer
by `ein-cli/tests/read_out_reference.rs`.
