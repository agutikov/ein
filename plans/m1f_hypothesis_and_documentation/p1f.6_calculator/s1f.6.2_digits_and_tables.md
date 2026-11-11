# S1f.6.2 — The digits and the hardcoded tables, as a library of facts

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08; re-planned 2026-11-11 to [the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.11](s1f.6.11_importing_facts.md), so a library of facts can be imported flat.

## What it builds

The theory's gates, as **plain facts** (the user, 2026-11-11: *"yes plain facts"*), in a module of their own,
`stdlib/digits.ein`, imported as `std.digits`. It is the stdlib's first library of facts, the *ontology* half of
[`solution_semantics.md` § 1.1](../../../docs/kernel/inference/solution_semantics.md#11-context-and-problem), and
both arithmetic modules import it: `std.arith` (S1f.6.3 to S1f.6.6) and `std.num9`
([S1f.6.13](s1f.6.13_num9.md)).

| table | reads | gives | rows |
|---|---|---|---:|
| `fa` | a, b, carry in (0 or 1) | sum digit, carry out | 200 |
| `fs` | a, b, borrow in (0 or 1) | difference digit, borrow out | 200 |
| `md` | a, b, carry in (0 to 8) | product digit, carry out | 900 |
| `cl` | the order below (`lt`, `eq`, `gt`), a, b | the order: a higher differing digit overrules | 300 |
| `nz` | a digit | it is not zero | 9 |
| `mid` | two bounds from 0 to 10, at least two apart | the digit to try between them | 45 |
| `adj` | two bounds | nothing lies between them | 10 |

These are the tables of the prototype's `build/s1f.6/prototype/r3/k.ein`
([design.md § Round 3](design.md#round-3-2026-11-11-several-representations-at-once)). The digit product with a
carry in replaces the plan's separate product and short-division tables: division bisects the divisor's
multiples and needs neither. `std.num9` adds its step counter in its own module.

## Tasks

1. **The generator.** `utils/gen_arith_tables.py` writes the module; `--check` is a gate step and fails when the
   file differs from what the script writes (the user's ruling on question 7, (a)).
2. **The module.** Every relation declared with a `:why`, every row a fact, and the module closes its own
   relations with written `(__closed__ R)` facts (question 6, (a)).
3. **The test that holds the rows:** each agrees with the harness's integers, none is missing, none is twice.
4. **The corpus and the stdlib's gates.** A module of facts has no rule for `stdlib_coverage.rs` to ask about;
   the stage says so on `tests/README.md` and adds one program under `tests/stdlib/digits/` that imports the
   module flat and claims a row of each table, which is also S1f.6.11's fix seen from the stdlib.
5. **The page:** the module's section of `stdlib/README.md`, as the first facts library.

## Acceptance

- Every table, its rows held to the integers by a test, and `--check` in the gate.
- A program that imports `std.digits` and nothing else: H empty, k_s 1, R {∅}.
- A core that passes through a table names its rows, which is the measured cost of plain facts
  ([design.md § G](design.md#g-the-flat-import-finding-a-specification-for-importing-facts)); the stage records
  one such core.
- No engine change in this stage.
