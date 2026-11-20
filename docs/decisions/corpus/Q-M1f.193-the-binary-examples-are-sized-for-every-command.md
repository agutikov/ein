# Q-M1f.193 — The binary examples are sized for every command

The programs of `examples/binary_arithmetic/` and `examples/binary_logic/` are written at sizes every command
answers and every corpus sweep reads: three inputs for every search over a circuit, so that describing R is 3³ + 3,
thirty saturations; binary operands of up to seventeen bits; and conversions of five numbers to binary, the largest
255, and of seven to decimal, the largest 1000. None of them is a `slow` entry.

**Why.** Describing R over n inputs as hypotheses is 3ⁿ + n saturations, a node being a partial assignment, so a
search over a circuit stops at the default budget near twelve inputs; and every corpus program is solved, dumped and
reloaded. Three inputs are enough to show each thing a circuit program does; where every input assignment is to be
tried at a larger size, test vectors written as facts are one saturation (`adder_against_table.ein`).

**Held by.** The programs' entries in `corpus/corpus.toml`, none `slow`, and `ein-corpus`'s
`the_slow_set_is_exactly_these_sixteen`; `ein-cli/tests/arith_search_free.rs`, which holds the binary arithmetic and
the conversions search-free.
