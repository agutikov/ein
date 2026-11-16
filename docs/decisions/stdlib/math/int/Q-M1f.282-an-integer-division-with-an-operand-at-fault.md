# Q-M1f.282 — An integer division with an operand at fault holds two flags

The three divisions share their operands' look-up under the tag `(int-div-of REQUEST)`, so an operand's fault
reaches that tag first, `(bad-request (int-div-of REQUEST) FAULT)`, and `int-bad-div` takes it to the request as
written. Such a request holds two flags, the tag's and its own, and `(false)` from each.

**Why.** The request as written is what a program reads and a core names, as a public division by zero holds its
request's flag beside its call's ([Q-M1f.211](../decimal/Q-M1f.211-a-public-division-by-zero-holds-two-flags.md)).

**Held by.** `stdlib/math/int.ein`'s `int-bad-div`; `tests/stdlib/math/int/35_undeclared_divisor.ein`.
