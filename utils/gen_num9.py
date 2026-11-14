#!/usr/bin/env python3
"""Write `stdlib/math/num9.ein`, school arithmetic on numbers of nine digits (M1f S1f.6.13).

`std.math.num9` is the calculator's second module: the user's own number,
`(num9 0 0 0 1 2 3 0 7 8)`, nine digit names, most significant first
(the instruction of 2026-11-08, and the ruling of 2026-11-11: *"yes, ship num9
in stdlib, why not"*). Its rules are **wide**: a sum is one rule with a premise
per column, so a rule has as many table premises as the number has digits, and
some have twice or three times as many. Nobody should type nine columns of
variables twenty times, and nobody should review them by eye, so the module is
written by this script, as `std.math.digits` is.

    python3 utils/gen_num9.py                      # write stdlib/math/num9.ein
    python3 utils/gen_num9.py --check              # exit 1 if the file differs
    python3 utils/gen_num9.py --stdout             # print it
    python3 utils/gen_num9.py --width 2 --stdout   # the same theory at another width

After a write, `python3 utils/stdlib_manifest.py --write` re-takes the
manifest: the module is part of the stdlib and is embedded in the binary.

**One width ships: nine.** `--width W`, 2 to 9, writes the same theory over
`(numW d… d)` with every name's 9 replaced by W, to stdout only; it is how
`ein-infer/tests/num9_theory.rs` holds the theory exhaustively, at a width
small enough to exhaust, and it is not a shipped module. The width stops at
nine because the steps of a product and a quotient are counted by the digit
names through `std.math.digits`' `adj`.

**A wide rule is wide in registers.** The engine binds a rule's variables to
a file of 256 (`ein_infer::MAX_REGS`), and `div9-start` binds eighteen a
digit, 162 at width nine. `ein-infer/tests/compile_limits.rs` pins that
number: a change here that widens a rule fails there, on purpose (Q-M1f.173).

Two things hold the file besides `--check`:

- `ein-infer/tests/num9_theory.rs` runs this script's `--check` inside
  `cargo test`, which is how the check is in the gate, and holds every answer
  to Rust's integers: every pair of operands at width 2, a sample at width 9.
- `tests/stdlib/math/num9/` has a program for every rule.

No dependency beyond the standard library.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
OUT = REPO / "stdlib" / "math" / "num9.ein"
SHIPPED = 9


class Theory:
    def __init__(self, w: int):
        assert 2 <= w <= 9, "the steps are counted by the digit names, so the width is 2 to 9"
        self.w = w
        self.rels: list[tuple[str, str, str]] = []     # name, types, why
        self.rules: list[tuple[str, str]] = []         # name, text
        self.sections: list[tuple[str, int, int]] = []  # comment, first relation, first rule

    # ── names ──────────────────────────────────────────────────────────────
    def nm(self, stem: str) -> str:
        """`add9`, `sum9-of`: a name of the module, the width in it."""
        return stem.replace("#", str(self.w))

    # ── the number ─────────────────────────────────────────────────────────
    def num(self, prefix: str) -> str:
        """`(num9 ?p8 … ?p0)`, most significant digit first."""
        return self.numl([f"?{prefix}{i}" for i in range(self.w - 1, -1, -1)])

    def numl(self, items: list[str]) -> str:
        assert len(items) == self.w
        return f"({self.nm('num#')} " + " ".join(items) + ")"

    def lo(self, prefix: str) -> list[str]:
        """The digits' variables, the units first: `?p0 … ?p8`."""
        return [f"?{prefix}{i}" for i in range(self.w)]

    @property
    def zero(self) -> str:
        return self.numl(["0"] * self.w)

    def chain(self, table: str, a: list[str], b: list[str], first: str, out: list[str], last: str,
              c: str = "c") -> list[str]:
        """A premise a column, the units first: `(table a_i b_i c_i out_i c_{i+1})`."""
        ps = []
        for i in range(len(a)):
            ci = first if i == 0 else f"?{c}{i}"
            co = last if i == len(a) - 1 else f"?{c}{i + 1}"
            ps.append(f"({table} {a[i]} {b[i]} {ci} {out[i]} {co})")
        return ps

    # ── emitting ───────────────────────────────────────────────────────────
    def section(self, comment: str) -> None:
        self.sections.append((comment, len(self.rels), len(self.rules)))

    def rel(self, stem: str, types: str, why: str) -> str:
        name = self.nm(stem)
        self.rels.append((name, types, self.nm(why)))
        return name

    def rule(self, stem: str, match: str | list[str], assert_: str, why: str) -> None:
        name = self.nm(stem)
        if isinstance(match, list):
            m = "(and " + "\n               ".join(match) + ")" if len(match) > 1 else match[0]
        else:
            m = match
        self.rules.append((name, f"(rule {name} ()\n  :match  {m}\n  :assert {assert_}\n"
                                 f"  :why    \"{self.nm(why)}\")"))


def theory(w: int) -> Theory:
    t = Theory(w)
    nm, num, numl, lo, chain = t.nm, t.num, t.numl, t.lo, t.chain
    N = "Number"
    D = "Digit"
    X, Y = num("x"), num("y")
    x, y = lo("x"), lo("y")
    digits = "".join("{?%d}" % i for i in range(1, w + 1))

    # ── the number, the protocol ───────────────────────────────────────────
    t.section("The number: " + f"{w} digits, the most significant first")
    t.rel("num#", " ".join([D] * w), f"the number written {digits}")

    t.section("The protocol: a request fact, answered by a result fact or by a flag")
    t.rel("add#", f"{N} {N}", "asked: {?1} + {?2}")
    t.rel("sum#-of", f"{N} {N} {N}", "{?1} + {?2} = {?3}")
    t.rel("sub#", f"{N} {N}", "asked: {?1} - {?2}")
    t.rel("diff#-of", f"{N} {N} {N} {N}", "{?1} - {?2} = {?3}, short by {?4}")
    t.rel("cmp#", f"{N} {N}", "asked: {?1} compared with {?2}")
    t.rel("cmp#-of", f"{N} {N} Order", "{?1} against {?2}: {?3}")
    t.rel("mul#", f"{N} {N}", "asked: {?1} x {?2}")
    t.rel("product#-of", f"{N} {N} {N}", "{?1} x {?2} = {?3}")
    t.rel("div#", f"{N} {N}", "asked: {?1} divided by {?2}")
    t.rel("quot#-of", f"{N} {N} {N} {N}", "{?1} divided by {?2} = {?3}, remainder {?4}")
    t.rel("overflow", "Request", f"the answer to {{?1}} does not fit {w} digits")
    t.rel("to#", "Nested", f"asked: {{?1}} as a number of {w} digits")
    t.rel("as#", f"Nested {N}", f"{{?1}} is {{?2}} in {w} digits")
    t.rel("big", "Nested", f"{{?1}} is written with more than {w} digits")
    t.rel("from#", N, "asked: {?1} as a number of any length")
    t.rel("of#", f"{N} Nested", "{?1} is {?2}")

    t.section("The circuit's own facts: the states of a product and of a quotient")
    t.rel("mul#-at", f"{N} {N} Step {N} {N}",
          "multiplying {?1} by {?2}, {?3} digits to go: of the multiplier {?4} is left, and the sum so far is {?5}")
    t.rel("row#", " ".join([D] * (w + 1)),
          "the multiple written " + "".join("{?%d}" % i for i in range(1, w + 2)))
    t.rel("multiples#", " ".join(["Row"] * 9),
          "the divisor's multiples: {?1}, {?2}, {?3}, {?4}, {?5}, {?6}, {?7}, {?8} and {?9}")
    t.rel("div#-at", f"{N} {N} Multiples Step {N} {N} {N}",
          "dividing {?1} by {?2}, {?4} digits to go: of the dividend {?5} is left, {?6} is left over, "
          "and the quotient so far is {?7}")

    # ── addition ───────────────────────────────────────────────────────────
    t.section("Addition: one rule, a premise a column")
    t.rule("add#-fits",
           [f"({nm('add#')} {X} {Y})"] + chain("fa", x, y, "0", lo("s"), "0"),
           f"({nm('sum#-of')} {X} {Y} {num('s')})",
           "each column's digit and carry from the table, the units first; no carry out of the top column")
    t.rule("add#-overflows",
           [f"({nm('add#')} {X} {Y})"] + chain("fa", x, y, "0", lo("s"), "1"),
           f"(overflow ({nm('add#')} {X} {Y}))",
           "each column's digit and carry from the table: a carry out of the top column, so the sum has "
           f"{w + 1} digits")

    # ── subtraction ────────────────────────────────────────────────────────
    t.section("Subtraction: two subtractors, and the one that does not borrow out of the top answers")
    t.rule("sub#-fits",
           [f"({nm('sub#')} {X} {Y})"] + chain("fs", x, y, "0", lo("d"), "0"),
           f"({nm('diff#-of')} {X} {Y} {num('d')} {t.zero})",
           "the second taken from the first, a column at a time: no borrow out of the top, so the first is "
           "not the smaller")
    t.rule("sub#-short",
           [f"({nm('sub#')} {X} {Y})"] + chain("fs", y, x, "0", lo("r"), "0"),
           f"({nm('diff#-of')} {X} {Y} {t.zero} {num('r')})",
           "the first taken from the second, a column at a time: no borrow out of the top, so the first is "
           "short by what is left")

    # ── comparison ─────────────────────────────────────────────────────────
    t.section("Comparison: one rule, the order of the digits below overruled by each higher column")
    order = ["eq"] + [f"?o{i}" for i in range(1, w)]
    t.rule("cmp#-columns",
           [f"({nm('cmp#')} {X} {Y})"]
           + [f"(cl {order[i]} {x[i]} {y[i]} {'?o' if i == w - 1 else f'?o{i + 1}'})" for i in range(w)],
           f"({nm('cmp#-of')} {X} {Y} ?o)",
           "the comparator's table, a column at a time from the units: a higher column overrules the order below")

    # ── multiplication ─────────────────────────────────────────────────────
    t.section("Multiplication: the school method from the multiplier's top digit. A step shifts the sum so\n"
              ";;    far one place and adds the multiplicand times the next digit, in one rule")
    ytop = numl([f"?y{i}" for i in range(w - 1, -1, -1)])
    yshift = numl([f"?y{i}" for i in range(w - 2, -1, -1)] + ["0"])
    acc = numl(["0"] + [f"?a{i}" for i in range(w - 2, -1, -1)])     # top digit 0: a shift loses nothing
    accs = ["0"] + [f"?a{i}" for i in range(w - 1)]                    # shifted, the units first
    acc_any = numl([f"?a{i}" for i in range(w - 1, -1, -1)])
    top = f"?y{w - 1}"
    row = lambda last: chain("md", x, [top] * w, "0", lo("p"), last, c="m")
    at = nm("mul#-at")
    t.rule("mul#-start",
           f"({nm('mul#')} {X} {Y})",
           f"({at} {X} {Y} {w} {Y} {t.zero})",
           f"start at the multiplier's top digit: {w} digits to go, and nothing summed yet")
    t.rule("mul#-step",
           [f"({at} {X} ?y ?k {ytop} {acc})", f"(nz {top})", "(adj ?j ?k)"] + row("0")
           + chain("fa", accs, lo("p"), "0", lo("s"), "0"),
           f"({at} {X} ?y ?j {yshift} {num('s')})",
           f"the multiplicand times the digit {{{top}}}, a column at a time, added to the sum so far shifted "
           "one place")
    t.rule("mul#-step-zero",
           [f"({at} ?x ?y ?k {numl(['0'] + [f'?y{i}' for i in range(w - 2, -1, -1)])} {acc})", "(adj ?j ?k)"],
           f"({at} ?x ?y ?j {yshift} " + numl([f"?a{i}" for i in range(w - 2, -1, -1)] + ["0"]) + ")",
           "the multiplier's next digit is 0: the sum so far shifts one place, and nothing is added")
    t.rule("mul#-done",
           f"({at} ?x ?y 0 ?left ?p)",
           f"({nm('product#-of')} ?x ?y ?p)",
           "no digit of the multiplier is left: the sum is the product")
    t.rule("mul#-overflows-shift",
           [f"({at} ?x ?y ?k ?left {acc_any})", "(adj ?j ?k)", f"(nz ?a{w - 1})"],
           f"(overflow ({nm('mul#')} ?x ?y))",
           f"the sum so far has a top digit, {{?a{w - 1}}}, and must still shift one place")
    # A top digit of the multiplier's rest that is not zero is a digit still to come: no step is counted.
    t.rule("mul#-overflows-row",
           [f"({at} {X} ?y ?k {ytop} ?acc)", f"(nz {top})"] + row("?c") + ["(nz ?c)"],
           f"(overflow ({nm('mul#')} {X} ?y))",
           f"the multiplicand times the digit {{{top}}} carries {{?c}} out of the top column")
    t.rule("mul#-overflows-add",
           [f"({at} {X} ?y ?k {ytop} {acc})", f"(nz {top})"] + row("0")
           + chain("fa", accs, lo("p"), "0", lo("s"), "1"),
           f"(overflow ({nm('mul#')} {X} ?y))",
           "adding the row to the sum so far carries out of the top column")

    # ── division ───────────────────────────────────────────────────────────
    t.section("Division: school long division. The divisor's multiples by 1 to 9 are computed once, by one\n"
              ";;    rule, and travel in the state; a step brings down a digit, and the one multiple that\n"
              ";;    subtracts without a borrow and leaves less than the divisor is the quotient's digit.\n"
              ";;    Ten step rules, one a digit, alike but for the multiple they read")
    Dv = num("d")
    d = lo("d")
    Nn = num("n")
    ntop = numl([f"?n{i}" for i in range(w - 1, -1, -1)])
    nshift = numl([f"?n{i}" for i in range(w - 2, -1, -1)] + ["0"])
    rem = num("r")
    quo = numl(["?qq"] + [f"?q{i}" for i in range(w - 2, -1, -1)])
    dat = nm("div#-at")
    rowf = nm("row#")
    mults = nm("multiples#")

    def quos(q: int) -> str:
        return numl([f"?q{i}" for i in range(w - 2, -1, -1)] + [str(q)])

    def mrow(q: int) -> str:
        """`(row9 ?m{q}c ?m{q}p8 … ?m{q}p0)`: q times the divisor, ten digits, the carry first."""
        if q == 1:
            return f"({rowf} 0 " + " ".join(f"?d{i}" for i in range(w - 1, -1, -1)) + ")"
        return f"({rowf} ?m{q}c " + " ".join(f"?m{q}p{i}" for i in range(w - 1, -1, -1)) + ")"

    start = [f"({nm('div#')} {Nn} {Dv})"]
    for q in range(2, 10):
        start += chain("md", d, [str(q)] * w, "0", [f"?m{q}p{i}" for i in range(w)], f"?m{q}c", c=f"m{q}c")
    t.rule("div#-start",
           start,
           f"({dat} {Nn} {Dv} ({mults} " + " ".join(mrow(q) for q in range(1, 10)) + f") {w} {Nn} {t.zero} {t.zero})",
           f"the divisor's multiples by 2 to 9, each a column at a time from the table. Start at the "
           f"dividend's top digit: {w} digits to go, nothing left over, the quotient zero")
    # The digit 0: what is left over, with the next digit beside it, is less than the divisor.
    rem0 = numl(["0"] + [f"?r{i}" for i in range(w - 2, -1, -1)])
    cur0 = [f"?n{w - 1}"] + [f"?r{i}" for i in range(w - 1)]            # W columns, the units first
    t.rule("div#-step-0",
           [f"({dat} ?n {Dv} ?ms ?k {ntop} {rem0} {quo})", "(adj ?j ?k)"]
           + chain("fs", cur0, d, "0", lo("u"), "1", c="e"),
           f"({dat} ?n {Dv} ?ms ?j {nshift} "
           + numl([f"?r{i}" for i in range(w - 2, -1, -1)] + [f"?n{w - 1}"]) + f" {quos(0)})",
           f"bring down {{?n{w - 1}}}: the number is less than the divisor, since taking the divisor from it "
           "borrows out of the top. The quotient's next digit is 0")
    cur = [f"?n{w - 1}"] + lo("r")                                       # W + 1 columns
    tt = [f"?t{i}" for i in range(w)]
    for q in range(1, 10):
        ms = f"({mults} " + " ".join(
            (f"({rowf} ?pc " + " ".join(f"?p{i}" for i in range(w - 1, -1, -1)) + ")") if k == q else f"?m{k}"
            for k in range(1, 10)) + ")"
        t.rule(f"div#-step-{q}",
               [f"({dat} ?n {Dv} {ms} ?k {ntop} {rem} {quo})", "(adj ?j ?k)"]
               + chain("fs", cur, lo("p") + ["?pc"], "0", tt + ["0"], "0", c="b")
               + chain("fs", tt, d, "0", lo("u"), "1", c="e"),
               f"({dat} ?n {Dv} {ms} ?j {nshift} "
               + numl([f"?t{i}" for i in range(w - 1, -1, -1)]) + f" {quos(q)})",
               f"bring down {{?n{w - 1}}}: {q} times the divisor goes into the number, with no borrow, and "
               f"what is left is less than the divisor. The quotient's next digit is {q}")
    t.rule("div#-done",
           f"({dat} ?n ?d ?ms 0 ?left ?r ?q)",
           f"({nm('quot#-of')} ?n ?d ?q ?r)",
           "no digit of the dividend is left: the quotient is {?q} and the remainder {?r}")
    t.rule("div#-by-zero",
           f"({nm('div#')} ?n {t.zero})",
           f"(division-by-zero ({nm('div#')} ?n {t.zero}))",
           "the divisor of {?n} divided by zero is zero")

    # ── conversions ────────────────────────────────────────────────────────
    t.section("Conversions to and from `std.math.decimal-arith`'s numbers, `z` and `(n PREFIX DIGIT)`: a request each\n"
              ";;    way, one firing, a rule for each count of digits")

    def nested(vs: list[str], inner: str = "z") -> str:
        s = inner
        for v in vs:
            s = f"(n {s} {v})"
        return s

    t.rule("to#-zero", f"({nm('to#')} z)", f"({nm('as#')} z {t.zero})", "zero is written with zeros")
    for k in range(1, w + 1):
        vs = [f"?a{i}" for i in range(k, 0, -1)]            # ?ak … ?a1, the top digit first
        t.rule(f"to#-{k}",
               f"({nm('to#')} {nested(vs)})",
               f"({nm('as#')} {nested(vs)} {numl(['0'] * (w - k) + vs)})",
               f"a number written with {k} digit{'s' if k > 1 else ''}: {w - k} zero{'s' if w - k != 1 else ''} "
               "in front" if k < w else f"a number written with {w} digits: as it stands")
    vs = [f"?a{i}" for i in range(w + 1, 0, -1)]
    t.rule("to#-big",
           f"({nm('to#')} {nested(vs, '?rest')})",
           f"(big {nested(vs, '?rest')})",
           f"a number written with more than {w} digits has no number of {w}")
    t.rule("from#-zero", f"({nm('from#')} {t.zero})", f"({nm('of#')} {t.zero} z)", "every digit is 0: zero")
    for h in range(w):
        vs = [f"?a{i}" for i in range(w - h, 0, -1)]
        W = numl(["0"] * h + vs)
        t.rule(f"from#-{w - h}",
               [f"({nm('from#')} {W})", f"(nz {vs[0]})"],
               f"({nm('of#')} {W} {nested(vs)})",
               f"the top digit that is not zero is {{{vs[0]}}}: a number of {w - h} digit{'s' if w - h > 1 else ''}, "
               "with no high zero")
    return t


def render(w: int) -> str:
    t = theory(w)
    nm = t.nm
    entry, rule_rel = nm("arith#"), nm("num#-rule")
    tables = ["fa", "fs", "md", "cl", "nz", "adj"]
    arith = ["n", "division-by-zero", "division-by-zero-is-false"]
    ex = lambda v: t.numl(list(str(v).rjust(w, "0")))
    out: list[str] = []
    emit = out.append
    emit(f""";;; Ein stdlib — `std.math.{nm('num#')}`: school arithmetic on numbers of {w} digits, computed by
;;; saturation alone (M1f P1f.6, S1f.6.13).
;;;
;;; GENERATED by `utils/gen_num9.py` — do not edit. `python3 utils/gen_num9.py
;;; --check` is in the gate (`ein-infer/tests/num9_theory.rs`).
;;;
;;; The calculator's second module (`std.math.decimal-arith` is the first): the user's own
;;; number, {w} digit names in one fact, the most significant first.
;;;
;;;     {ex(123078) if w >= 6 else ex(7)}        {'123078' if w >= 6 else '7'}
;;;
;;; Ein has no numbers and no arithmetic (the user's ruling of 2026-11-06).
;;; This is a theory written in Ein over the digit names: the tables of
;;; `std.math.digits` are the gates and the rules below are the wiring. Nothing is
;;; guessed: on a program that imports the theory and states requests the
;;; universe has no candidate and the answer is root's fixpoint. No `absent`,
;;; no `not`, one stratum.
;;;
;;; ── Import ─────────────────────────────────────────────────────────────────
;;;
;;;     (import std.math.{nm('num#')} :symbols ({entry} {nm('add#')} {nm('num#')} {nm('sum#-of')}))
;;;
;;; An import brings the whole theory, the tables and `std.math.decimal-arith` with it,
;;; under full names (M1f S1f.6.35); a program lists the names it writes.
;;; `{entry}` is the theory's entry, the closing rule at the end of this file:
;;; it names every rule and closes every relation of the theory and the tables
;;; it reads. `n` and `division-by-zero` are `std.math.decimal-arith`'s, which this module
;;; lists, and a program that writes them lists them from `std.math.decimal-arith`.
;;;
;;; ── The protocol: a request fact, answered by a result fact or by a flag ───
;;;
;;;     ({nm('add#')} X Y)      ({nm('sum#-of')} X Y S)         or (overflow ({nm('add#')} X Y))
;;;     ({nm('sub#')} X Y)      ({nm('diff#-of')} X Y D R)      D is X - Y or zero, R is Y - X or zero
;;;     ({nm('cmp#')} X Y)      ({nm('cmp#-of')} X Y O)         O is `lt`, `eq` or `gt`
;;;     ({nm('mul#')} X Y)      ({nm('product#-of')} X Y P)     or (overflow ({nm('mul#')} X Y))
;;;     ({nm('div#')} X Y)      ({nm('quot#-of')} X Y Q R)      X = Q x Y + R, and R less than Y
;;;
;;; A program reads an answer with a goal that has a variable in the result's
;;; place, `({nm('sum#-of')} X Y ?s)`. An operand is a number, written out: this
;;; module has no named constants.
;;;
;;; **Overflow is a fact, and no number.** A sum or a product that does not
;;; fit {w} digits derives `(overflow REQUEST)`, the request as written, and no
;;; result fact: never a truncated number. Root stays alive, so the program's
;;; other requests are answered, and a rule of the program may match the flag.
;;;
;;; **Division by zero is a contradiction,** as in `std.math.decimal-arith` and by its
;;; rule: the request derives `(division-by-zero ({nm('div#')} X ZERO))`, and
;;; `std.math.decimal-arith`'s `division-by-zero-is-false` derives `(false)` from the flag.
;;; Root is dead and R is empty.
;;;
;;; ── Conversions: a request each way ────────────────────────────────────────
;;;
;;;     ({nm('to#')} X)         ({nm('as#')} X W)   X a number of `std.math.decimal-arith`, `z` or `(n PREFIX DIGIT)`
;;;                      or (big X)   where X is written with more than {w} digits
;;;     ({nm('from#')} W)       ({nm('of#')} W X)   X in normal form, with no high zero
;;;
;;; One firing each. Nothing converts a number because it exists, and nothing
;;; routes a request of one module to the other: a program that wants both
;;; writes the rule that asks (`examples/arith/num9_both_ways.ein`).
;;;
;;; ── How it computes: wide rules ────────────────────────────────────────────
;;;
;;; **A sum is one firing.** `{nm('add#-fits')}` has a premise for the request and
;;; one for each column, a row of the full adder's table, the carry handed
;;; from column to column by a variable; the carry out of the top column is
;;; written `0`, and its twin `{nm('add#-overflows')}` writes `1` there. A
;;; difference is two such rules, one each way round, and the one that does
;;; not borrow out of the top answers: the borrow is the comparison. A
;;; comparison is one rule over the comparator's table.
;;;
;;; **A product is {w + 2} firings:** a start, a step for each digit of the
;;; multiplier from its top, and an end. A step is one rule with {2 * w}
;;; table premises: the multiplicand times the digit, a column at a time, and
;;; that row added to the sum so far shifted one place; where the digit is 0
;;; a second rule shifts and reads no table. Three rules derive the
;;; overflow, one for each place a digit can fall off: the shift, the row's
;;; carry, the addition's carry.
;;;
;;; **A quotient is {w + 2} firings.** `{nm('div#-start')}` computes the divisor's
;;; multiples by 2 to 9 in one firing, {8 * w} rows of the digit multiplier's
;;; table, and puts all nine into the state. A step brings down one digit of
;;; the dividend; ten step rules, one for each digit the quotient can have,
;;; each read their own multiple from the state and hold two subtractors: the
;;; multiple taken from the number must not borrow, and the divisor taken from
;;; what is left must. Exactly one of the ten fires. Nothing is guessed: the
;;; tables decide each.
;;;
;;; **No rule joins two facts on a number,** `std.math.decimal-arith`'s rule, kept here: a
;;; rule has one premise that holds numbers, and its other premises are table
;;; rows looked up by digits. So a batch of requests costs in proportion to
;;; its count (`ein-infer/tests/arith_batch.rs`).
;;;
;;; ── When to use which module ───────────────────────────────────────────────
;;;
;;; `std.math.num9` where every number fits {w} digits and one firing a sum
;;; matters, or where a fixed width with an overflow flag is what is modelled.
;;; `std.math.decimal-arith` for numbers of any length, for named constants, and for
;;; batches: its circuits are longer chains of cheaper steps
;;; (`stdlib/README.md` § `std.math.num9` has both measured).
;;;
;;; ── The names this module occupies ─────────────────────────────────────────
;;;
;;; Its own: {', '.join('`' + r[0] + '`' for r in t.rels)}, `{rule_rel}`, the entry
;;; `{entry}`, and its rules' names. From `std.math.decimal-arith`: `n`, with `z`, for the
;;; conversions, and `division-by-zero` with the rule from it to `(false)`.
;;; From `std.math.digits`: {', '.join('`' + x + '`' for x in tables)}, with the orders `lt`, `eq`,
;;; `gt`. The interface is the number, the requests, the results, the two
;;; flags, the conversions and `big`; the states `{nm('mul#-at')}` and
;;; `{nm('div#-at')}`, `{nm('multiples#')}` and `{nm('row#')}` are in the state for a query
;;; or a `--trace` to see, and are not a promise.
""")
    emit(f"(import std.math.digits :symbols ({' '.join(tables)}))")
    emit(f"(import std.math.decimal-arith :symbols ({' '.join(arith)}))")
    marks = t.sections + [("", len(t.rels), len(t.rules))]
    for (comment, r0, u0), (_, r1, u1) in zip(marks, marks[1:]):
        emit(f"\n;; ── {comment} ──")
        for name, types, why in t.rels[r0:r1]:
            emit(f"(relation {name} {types}\n  :why \"{why}\")")
        for i, (_, text) in enumerate(t.rules[u0:u1]):
            emit(("\n" if i or r1 > r0 else "") + text)

    def wrap(items: list[str], indent: str = "               ") -> str:
        lines, cur = [], ""
        for it in items:
            if cur and len(indent) + len(cur) + 1 + len(it) > 79:
                lines.append(cur)
                cur = it
            else:
                cur = f"{cur} {it}" if cur else it
        lines.append(cur)
        return ("\n" + indent).join(lines)

    closed = tables + ["n", "division-by-zero"] + [r[0] for r in t.rels] + [rule_rel]
    emit(f"""
;; ── The closing rule, and the theory's one entry ──
;; Arithmetic is computed and never guessed: every relation of the theory, and
;; each table it reads, is closed, so the search has no candidate among them.
;; The rule also names every rule of the module, which until M1f S1f.6.35 was
;; what made `(import std.math.{nm('num#')} :symbols ({entry}))` bring the whole theory;
;; every import brings its module whole since.
(relation {rule_rel} Rule
  :why "{{?1}} is a rule of std.math.{nm('num#')}")

;; The closing rule's own name, as a written fact: it kept `{entry}` in a
;; program that is inlined and tree-shaken, as the program a `--dump-answer`
;; state file re-writes is (E-11 of P1f.6's `findings.md`). No longer needed
;; for that since M1f S1f.6.25, a name a selection lists being live; it stays,
;; since removing it would move every state of every program.
({rule_rel} {entry})

(rule {entry} ()
  :match  ()
  :assert (and {wrap([f'(__closed__ {r})' for r in closed])}
               {wrap([f'({rule_rel} {name})' for name, _ in t.rules] + [f'({rule_rel} division-by-zero-is-false)'])})
  :why    "arithmetic is computed and never guessed: every relation of the theory is closed")""")
    return "\n".join(out) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--check", action="store_true", help="exit 1 if stdlib/math/num9.ein differs")
    ap.add_argument("--stdout", action="store_true", help="print the module")
    ap.add_argument("--width", type=int, default=SHIPPED, help="2 to 9; another width than 9 prints only")
    args = ap.parse_args()
    text = render(args.width)
    if args.stdout or args.width != SHIPPED:
        if args.check:
            print("--check holds the shipped width alone", file=sys.stderr)
            return 2
        sys.stdout.write(text)
        return 0
    if args.check:
        if not OUT.exists() or OUT.read_text() != text:
            print(f"{OUT}: differs from `python3 utils/gen_num9.py`", file=sys.stderr)
            return 1
        return 0
    OUT.write_text(text)
    print(f"wrote {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
