# Q-M1f.167 — The names of `std.math.num9` are decimal's with a 9

`std.math.num9`'s number is `(num9 D8 … D0)`, nine digit names, the most significant first. Its requests and
results are the decimal arithmetic's with a 9: `add9` answered by `sum9-of`, `sub9` by `diff9-of`, `cmp9` by
`cmp9-of`, `mul9` by `product9-of` and `div9` by `quot9-of`; the entry is `arith9`, the conversions `to9`/`as9` and
`from9`/`of9`, and the flags `overflow` and `big`.

**Why.** A reader who knows one module reads the other, and the two stand in one program.

**Held by.** `stdlib/math/num9.ein`; `tests/stdlib/math/num9/12_both_modules.ein`.
