# S1f.6.2 — The digits and the hardcoded tables

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08.
**Depends on:** [S1f.6.1](s1f.6.1_design_proposal.md) ruled; P1f.5 closed.

## What it builds

The theory's gates, as facts: the electronics the user asked for. A rule only looks them up.

| table | reads | gives | rows |
|---|---|---|---|
| digit | — | the ten digit names `0` … `9` | 10 |
| full adder | a, b, carry-in (0 or 1) | sum digit, carry-out | 10 × 10 × 2 |
| full subtractor | a, b, borrow-in (0 or 1) | difference digit, borrow-out | 10 × 10 × 2 |
| digit product | a, b | high digit, low digit | 10 × 10 |
| digit comparator | a, b | `lt`, `eq` or `gt` | 10 × 10 |
| short division | a remainder r and a digit a, by a divisor d with r < d | quotient digit, remainder digit | Σ over d of 10 d |

The short-division table divides a single digit's worth of the dividend at a time by a one-digit divisor. It is
the school method for a one-digit divisor, and S1f.6.6 decides whether long division uses it or only the
comparator. Each table's relation names, argument order and file are S1f.6.1's ruling.

## Tasks

1. **The tables, in the form S1f.6.1 ruled.** Generated with a `--check` in the gate, or hand-written. Every
   relation is closed if that is how H stays empty (S1f.6.1's question 6).
2. **The test that holds them:** every row agrees with the harness's integers, and no row is missing or
   duplicated.
3. **The programs under `tests/stdlib/arith/`** that the stdlib's coverage gate needs. A table is facts, not
   rules, so this stage adds no rule to cover unless the ruled form derives a table from another.
4. **The page:** the theory's section of `stdlib/README.md`, and the module's header.

## Acceptance

- Every table, its rows held to the integers by a test.
- An empty program that imports the theory: H empty, k_s 1, R {∅}.
- No engine change.
