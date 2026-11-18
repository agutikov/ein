#!/usr/bin/env python3
"""Write the calculator's tables: `stdlib/math/digits.ein` (M1f S1f.6.2) and `stdlib/math/bits.ein` (S1f.6.16).

`std.math.digits` is a library of **facts**: the seven tables the decimal
theories look up, one fact a row, and one `(__closed__ R)` fact per table.
Nobody should type 1 664 rows, and nobody should review them by eye, so the
module is written by this script (the user's ruling of 2026-11-11 on
`design.md`'s question 7: generated, with a `--check` in the gate, as plain
facts). `std.math.bits` is binary's four, 29 rows, written by the same script
the same way (S1f.6.16's T1f.6.16.6).

    python3 utils/gen_arith_tables.py            # write both modules
    python3 utils/gen_arith_tables.py --check    # exit 1 if either file differs
    python3 utils/gen_arith_tables.py --stdout   # print std.math.digits
    python3 utils/gen_arith_tables.py --stdout --bits   # print std.math.bits

After a write, `python3 utils/stdlib_manifest.py --write` re-takes the
manifest: the modules are part of the stdlib and are embedded in the binary.

Two things hold the files besides `--check`:

- the gate runs this script's `--check` (`utils/check_generated.py`), and
  `ein-ir/tests/digit_tables.rs` holds every row of both modules to Rust's
  integers, none missing and none twice. The rows are computed twice, here
  and there, in two languages.
- `tests/stdlib/math/digits/01_tables.ein` and `tests/stdlib/math/bits/01_tables.ein`
  import a module flat and claim a row of each table.

No dependency beyond the standard library.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
OUT = REPO / "stdlib" / "math" / "digits.ein"
OUT_BITS = REPO / "stdlib" / "math" / "bits.ein"

ORDERS = ("lt", "eq", "gt")


def tables() -> list[tuple[str, str, str, str, list[list[tuple]]]]:
    """`(name, types, why, comment, lines)`, a line being the rows printed on it."""
    fa = [[(a, b, c, (a + b + c) % 10, (a + b + c) // 10) for b in range(10)]
          for a in range(10) for c in range(2)]
    fs = [[(a, b, c, (a - b - c) % 10, 1 if a - b - c < 0 else 0) for b in range(10)]
          for a in range(10) for c in range(2)]
    md = [[(a, b, c, (a * b + c) % 10, (a * b + c) // 10) for c in range(9)]
          for a in range(10) for b in range(10)]
    cl = [[(o, a, b, "lt" if a < b else "gt" if a > b else o) for b in range(10)]
          for o in ORDERS for a in range(10)]
    nz = [[(k,) for k in range(1, 10)]]
    mid = [[(lo, hi, (lo + hi) // 2) for hi in range(lo + 2, 11)] for lo in range(9)]
    adj = [[(k, k + 1) for k in range(10)]]
    return [
        ("fa", "Digit Digit Digit Digit Digit",
         "{?3} + {?4} + carry {?5} is digit {?6}, carry {?7}",
         "The full adder. a + b + carry in (0 or 1) = 10 x carry out + digit.",
         fa),
        ("fs", "Digit Digit Digit Digit Digit",
         "{?3} - {?4} - borrow {?5} is digit {?6}, borrow {?7}",
         "The full subtractor. a - b - borrow in (0 or 1) = digit - 10 x borrow out.",
         fs),
        ("md", "Digit Digit Digit Digit Digit",
         "{?3} x {?4} + carry {?5} is digit {?6}, carry {?7}",
         "The digit multiplier. a x b + carry in (0 to 8) = 10 x carry out + digit;\n"
         ";; the carry out is at most 8, since 9 x 9 + 8 = 89.",
         md),
        ("cl", "Order Digit Digit Order",
         "the order below is {?3}; {?4} against {?5} above it makes it {?6}",
         "The digit comparator. Two numbers are compared from the units up: the order\n"
         ";; of the digits below, then one digit of each; a differing digit overrules\n"
         ";; the order below it, equal digits keep it.",
         cl),
        ("nz", "Digit",
         "{?3} is not zero",
         "The digits that are not zero.",
         nz),
        ("mid", "Bound Bound Digit",
         "between {?3} and {?4}, try {?5}",
         "The bisection of 0 to 10. For two bounds at least two apart, the digit to\n"
         ";; try between them, (lo + hi) / 2 rounded down.",
         mid),
        ("adj", "Bound Bound",
         "nothing lies between {?3} and {?4}",
         "The bounds with nothing between them: the bisection has ended.",
         adj),
    ]


HEADER = """\
;;; Ein stdlib — `std.math.digits`: the digit tables of the calculator.
;;;
;;; GENERATED: an edit by hand is lost the next time the file is written.
;;; Every row is computed where it is generated, and again from Rust's
;;; integers by `ein-ir/tests/digit_tables.rs`, which holds the file to them,
;;; none missing and none twice (Q-M1f.366).
;;;
;;; A library of **facts**, the stdlib's first: no rule, no macro. Ein has no
;;; numbers and no arithmetic (Q-M1e2.24), and the ten
;;; digits `0` to `9` are plain names. What school arithmetic knows about them
;;; is written here once, the way hardware holds it: as tables, one fact a
;;; row. `std.math.decimal-arith` and `std.math.num9` are the wiring; every step of theirs is a
;;; lookup in one of these tables, so nothing is ever guessed.
;;;
;;;   table  reads                               gives                      rows
;;;   fa     a, b, carry in (0 or 1)             sum digit, carry out        200
;;;   fs     a, b, borrow in (0 or 1)            difference digit, borrow    200
;;;   md     a, b, carry in (0 to 8)             product digit, carry out    900
;;;   cl     the order below (lt eq gt), a, b    the order                   300
;;;   nz     a digit                             it is not zero                9
;;;   mid    two bounds, 0 to 10, two apart      the digit to try             45
;;;   adj    two bounds                          nothing lies between them    10
;;;
;;; A row's arguments are digits, except `cl`'s orders, the names `lt`, `eq`
;;; and `gt`, and the bound `10` of `mid` and `adj`, a name like any digit.
;;;
;;; Importing. An import brings every table and its rows (Q-M1f.247); a
;;; program lists the tables it writes, and the others print by their full
;;; names, `std.math.digits.fs`:
;;;
;;;     (import std.math.digits :symbols (fa nz __closed__))
;;;
;;; **Closed.** Each table is closed by a written `(__closed__ R)` fact below:
;;; a table is complete, and a row is never a hypothesis. Those facts come
;;; with every import of the module, as its rows do, whether or not the
;;; import lists `__closed__` (Q-M1f.146). A theory that
;;; reads the tables closes the ones it reads in its own closing rule too, as
;;; `std.math.decimal-arith` does.
"""


def bits_tables() -> list[tuple[str, str, str, str, list[list[tuple]]]]:
    """`std.math.bits`: the binary tables, as `tables()` writes decimal's, at base two."""
    fa = [[(a, b, c, (a + b + c) % 2, (a + b + c) // 2) for b in range(2)]
          for a in range(2) for c in range(2)]
    fs = [[(a, b, c, (a - b - c) % 2, 1 if a - b - c < 0 else 0) for b in range(2)]
          for a in range(2) for c in range(2)]
    cl = [[(o, a, b, "lt" if a < b else "gt" if a > b else o) for b in range(2)]
          for o in ORDERS for a in range(2)]
    nz = [[(1,)]]
    return [
        ("bin-fa", "Bit Bit Bit Bit Bit",
         "{?3} + {?4} + carry {?5} is bit {?6}, carry {?7}",
         "The full adder. a + b + carry in = 2 x carry out + bit.",
         fa),
        ("bin-fs", "Bit Bit Bit Bit Bit",
         "{?3} - {?4} - borrow {?5} is bit {?6}, borrow {?7}",
         "The full subtractor. a - b - borrow in = bit - 2 x borrow out.",
         fs),
        ("bin-cl", "Order Bit Bit Order",
         "the order below is {?3}; {?4} against {?5} above it makes it {?6}",
         "The bit comparator, a column at a time from the units: a differing bit\n"
         ";; overrules the order below it, equal bits keep it.",
         cl),
        ("bin-nz", "Bit",
         "{?3} is not zero",
         "The bit that is not zero.",
         nz),
    ]


BITS_HEADER = """\
;;; Ein stdlib — `std.math.bits`: the bit tables of binary arithmetic, a
;;; module of their own (Q-M1f.192).
;;;
;;; GENERATED: an edit by hand is lost the next time the file is written.
;;; Every row is computed where it is generated, and again from Rust's
;;; integers by `ein-ir/tests/digit_tables.rs`, which holds the file to them,
;;; none missing and none twice (Q-M1f.366).
;;;
;;; A library of **facts**, as `std.math.digits` is: binary's tables, one fact
;;; a row, which `std.math.binary-arith` and `std.math.positional`'s rules
;;; activated for binary look up. The two bits are the names `0` and `1`,
;;; which are decimal's digits too.
;;;
;;;   table   reads                               gives                   rows
;;;   bin-fa  a, b, carry in                      sum bit, carry out         8
;;;   bin-fs  a, b, borrow in                     difference bit, borrow     8
;;;   bin-cl  the order below (lt eq gt), a, b    the order                 12
;;;   bin-nz  a bit                               it is not zero             1
;;;
;;; 29 rows, where decimal's seven tables have 1 664: binary needs no digit
;;; multiplier, since a row of a product is nothing or the multiplicand, and
;;; no bisection, since one subtraction decides a bit of a quotient.
;;;
;;; Importing. An import brings every table and its rows (Q-M1f.247); a
;;; program lists the tables it writes, and the others print by their full
;;; names, `std.math.bits.bin-fs`:
;;;
;;;     (import std.math.bits :symbols (bin-fa))
;;;
;;; **Closed.** Each table is closed by a written `(__closed__ R)` fact below:
;;; a table is complete, and a row is never a hypothesis. A theory that reads
;;; the tables closes the ones it reads in its own closing rule too, as
;;; `std.math.binary-arith` does.
"""


def render_bits() -> str:
    return render_module(BITS_HEADER, bits_tables())


def cell(v) -> str:
    return str(v)


def render() -> str:
    return render_module(HEADER, tables())


def render_module(header: str, ts) -> str:
    out = [header]
    for name, types, why, _, _ in ts:
        out.append(f'(relation {name} {types} :why "{why}")')
    out.append("")
    out.append(";; Every table is complete: no row is ever a hypothesis.")
    out.append(" ".join(f"(__closed__ {name})" for name, *_ in ts))
    for name, _, _, comment, lines in ts:
        rows = sum(len(l) for l in lines)
        out.append("")
        out.append(f";; ── {name}: {rows} {'row' if rows == 1 else 'rows'} ──")
        out.append(f";; {comment}")
        for line in lines:
            out.append(" ".join("(" + " ".join([name] + [cell(v) for v in row]) + ")" for row in line))
    return "\n".join(out) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true", help="exit 1 if stdlib/math/digits.ein or bits.ein differs from what this script writes")
    ap.add_argument("--stdout", action="store_true", help="print a module instead of writing it")
    ap.add_argument("--bits", action="store_true", help="with --stdout, print std.math.bits")
    args = ap.parse_args()
    files = [(OUT, render(), tables()), (OUT_BITS, render_bits(), bits_tables())]
    if args.stdout:
        sys.stdout.write(files[1 if args.bits else 0][1])
        return 0
    if args.check:
        stale = 0
        for out, text, _ in files:
            have = out.read_text(encoding="utf-8") if out.exists() else None
            if have != text:
                print(f"{out.relative_to(REPO)} is not what utils/gen_arith_tables.py writes.", file=sys.stderr)
                stale += 1
        if stale:
            print("if intentional: python3 utils/gen_arith_tables.py && python3 utils/stdlib_manifest.py --write", file=sys.stderr)
            return 1
        return 0
    for out, text, ts in files:
        out.write_text(text, encoding="utf-8")
        print(f"wrote {out.relative_to(REPO)}: {sum(len(l) for *_, lines in ts for l in lines)} rows")
    return 0


if __name__ == "__main__":
    sys.exit(main())
