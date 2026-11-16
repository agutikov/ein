# Q-M1f.190 — Each circuit example takes the simplest form its answer holds under

The programs of `examples/binary_logic/` are each written in the simplest form under which their answer holds:

- **a truth table by search** proposes its candidates from the obligations rung alone, so the program has no hrule
  and no `(solve …)` form: `std.bool.inputs`' obligation says an input has exactly one value, and R is every total
  assignment (`truth_table.ein`);
- **Post's criterion** is checked on seven written sets of functions, a set complete by a rule over the class facts
  `std.bool.props` derives, and inside a class by `forall` over its members (`post.ein`);
- **the simplifier** reads three relations of `std.bool.props`, `identity-of`, `absorbing-of` and `commutative`
  (`simplify.ein`).

**Why.** An example is read to learn the mechanism it shows, and a second mechanism beside it that changes neither
the answer nor the cost is something more to read that teaches nothing.

**Held by.** Each program's claims, computed outside Ein by enumeration and held by its `test` run in
`corpus/corpus.toml`; `examples/binary_logic/README.md`.
