# The numerals' check

[← the bundle](../README.md)

`written` is erased (Q-M1f.404): a numeral an operation takes from the program is checked once and labelled
`(valid N)` (`(bin-valid N)` in binary), and a number an operation makes is labelled as it is made. Q-M1f.405 to
.411 are the readings that build took, each with its default (a); L-101 is what it left in `std.math.int` and
`std.math.rat`. *Before* is `ein-0a4a97a3`, the calculator with `written`.

| # | item | the question | default |
|---|---|---|---|
| 04 | [Q-M1f.405](04-Q-M1f.405/README.md) | which numerals are taken from the program, and *once* | (a) |
| 05 | [Q-M1f.409](05-Q-M1f.409/README.md) | an integer's and a rational's check and label | (a) |
| 11 | [Q-M1f.406](11-Q-M1f.406/README.md) | the label, and where a made number gets it | (a) |
| 12 | [Q-M1f.407](12-Q-M1f.407/README.md) | a numeral no operation takes is not checked | (a) |
| 13 | [Q-M1f.408](13-Q-M1f.408/README.md) | where the check's rules live | (a) |
| 14 | [Q-M1f.410](14-Q-M1f.410/README.md) | no join on a number; a made number handed on is walked again | (a) |
| 15 | [Q-M1f.411](15-Q-M1f.411/README.md) | a numeral with two faults has one flag | (a) |
| 29 | [L-101](29-L-101/README.md) | `int-check-parts` / `rat-check-parts` ask non-numerals | as they are |

**Two answers moved under the defaults**, outside the corpus: a malformed numeral handed straight to a tagged call
(04, option (e)'s case) and a malformed natural as a rational's numerator (05, option (d)'s), each dead before and
alive today. Three corpus programs went from dead to R={∅} and were rewritten to claim it (12).

## Found while building

- **04**: the record's `states 1` holds only with `(__closed__ mine)`; without it the blind enumerator gives 32 768
  states. The new `tests/stdlib/math/decimal/07` is alive under the earlier binary: its `(false)` claim holds only
  under today's design.
- **05**: there is no `int-valid` or `rat-valid` label; the moved case leaves `(int-check (n z x))`, which no rule
  reads (L-101's third point), and the second rational is never looked up.
- **11**: Q-M1f.408 (a)'s *label at a walk's end … by `std.math.positional`* means the arithmetic walk; an operand's
  label is `std.math.decimal`'s `numeral-valid`. `div-end`'s label of the remainder is a second derivation.
- **12**: `ec9049b5` touches only `alive_invariant.rs`; the whole change is `bf9693dc`.
- **13**: `ein render rule --name` on a program that imports the module finds neither the full nor the short name
  (exit 1); it works only on the module's own file. The renderer's, not this question's.
- **14**: nothing differs from the ledger.
- **15**: it is one flag per place, not per numeral: `(n (n 1 x) 5)` gets two flags today as before.
- **29**: the ledger's *two rules more each* comes out as one more rule per module.
