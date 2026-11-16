# Q-M1f.166 — The generated arithmetic examples are sized for every command

Three programs of `examples/arith/` are generated, each long in a different way, and each at a size every command
answers and every corpus sweep reads: `long_numbers.ein`, few requests on long operands, a sum, a difference and a
comparison of two 100-digit numbers, a product of two of 10 digits and a quotient of 20 digits by 10;
`many_calculations.ein`, 100 requests on numbers of one to six digits, 20 an operation; and `factorial.ein`, 25! by
a chain of the program's own rules from one written fact. A larger calculation is not part of the corpus.

**Why.** `ein solve` writes a state as text, and a calculation's state grows as its facts times their digits, so a
program the corpus solves, dumps and reloads must be one whose state stays small. At these sizes each still shows
what it is for: where a long program's time goes, on long operands, on many requests, and on a long chain.

**Held by.** The three entries in `corpus/corpus.toml`, none of them `slow`; `ein-cli/tests/arith_search_free.rs`:
`every_arithmetic_program_is_search_free`, each of the three with no candidate, R {∅} and one saturation, and the
same answer under `ein solve`, `ein saturate` and `ein test`.
