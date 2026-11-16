# Q-M1f.186 — What the calculator's tables do besides arithmetic is beside `examples/arith/`

The programs that use the calculator's tables for something other than a calculation, tables read backwards,
equations solved by search, bounded theorem checking, number theory by saturation, live in
`examples/beyond_arith/`, beside `examples/arith/` and not under it.

**Why.** Every program under `examples/arith/` is held to *search-free*: no candidate, R {∅}, one saturation. Some
of these search, by design: an equation's unknown digits are hypotheses. A directory of their own keeps the
search-free claim one that holds of a whole directory, with no list of exceptions to keep in step with it.

**Held by.** `ein-cli/tests/arith_search_free.rs`: `every_arithmetic_program_is_search_free`, over `examples/arith/`
and the calculator's test programs; the programs of `examples/beyond_arith/`, each with its claim, in
`corpus/corpus.toml`.
