# Q-M1f.219 — A query's `(false)` holds over either kind of empty R

A query's `:expect (false)` claims that the query has no class, and a query has no class over an empty R of either
kind: every node dead, or every alive node owing. The query's `over` row says which empty it is, as root says it:
*R is empty, root is dead* or *R is empty, root is alive and owes n instances*. A program that means *every node is
dead* states it over the region, `(solve :expect (false))`, which fails where alive nodes owe.

**Why.** A query asks about the states of R and their projection; with no state there is no class, whatever made R
empty. Telling a contradiction from a debt is the region claim's job, and the row under the query keeps the two
apart for the reader.

**Held by.** `ein-cli/tests/read_out.rs`: `a_query_over_an_empty_r_says_which_empty`, the row on both kinds of
empty under describe, `--single` and `-n`, and a query's `(false)` holding on
`examples/binary_logic/unsatisfiable.ein`, whose root is alive and owes.
