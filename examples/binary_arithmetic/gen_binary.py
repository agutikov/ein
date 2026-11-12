#!/usr/bin/env python3
"""Write the binary theory of `examples/binary_arithmetic/` from the shipped `std.arith`.

M1f P1f.6, S1f.6.16. The plan page, `plans/m1f_hypothesis_and_documentation/
p1f.6_calculator/binary_arithmetic.md`, tried three ways to have binary numbers
and recommends (b): a theory of its own, its own shape `(b2 PREFIX BIT)` and
every name its own, so that it stands beside decimal in one program. 51 of its
67 rules are `std.arith`'s, the same up to their names, so they are not written
twice: this script reads `stdlib/arith.ein`, keeps what does not know the base,
replaces what does, and renames.

    examples/binary_arithmetic/bits.ein   the three tables, 28 rows
    examples/binary_arithmetic/bin.ein    the theory, 67 rules and its closing rule
    examples/binary_logic/bits.ein        a copy of the tables: an import cannot
                                          reach a sibling directory

What it does to `std.arith`:

  - drops the division of base ten, the nine multiples and the bisection, and
    puts in a division of base two: one subtraction decides a quotient bit;
  - drops the digit multiplier, `md` and its walk: a row of a binary product is
    nothing or the multiplicand, so a step is one of four rules;
  - drops `nz`: the digit that is not zero is `1`, written in the rule;
  - drops the check of malformed numbers and of undeclared names (S1f.6.12):
    its rules name the ten digits and `nz`, so they are decimal's, and the
    binary theory answers a malformed request by silence, as `std.arith` did
    until that stage. Whether it gets a check of its own is the plan page's;
  - renames every relation, every rule and the tables with a `2`, the number
    shape `n` to `b2` and the accumulator `b` to `a2`; the entry is `bin`.

The names are the prototype's placeholders; which names the theory has is the
plan page's second question, and the user's.

    python3 examples/binary_arithmetic/gen_binary.py            # write the files
    python3 examples/binary_arithmetic/gen_binary.py --check    # exit 1 if one differs

The script is run by hand. It is not in the gate: whether the binary rules are
generated from `std.arith` and held to it by a test is the plan page's fourth
question, open. So an edit to `stdlib/arith.ein` does not fail a test here; it
makes `--check` exit 1 until the script is run again.

No dependency beyond the standard library.
"""
from __future__ import annotations

import copy
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
SHIPPED = REPO / "stdlib" / "arith.ein"

# ── a small S-expression reader and printer for Ein text ────────────────────

TOK = re.compile(r'\s+|;[^\n]*|("(?:[^"\\]|\\.)*")|([()])|([^\s()";]+)')


class Str(str):
    """A string literal, kept apart from a name so that renaming leaves it alone."""


def parse(text):
    stack = [[]]
    for m in TOK.finditer(text):
        s, p, a = m.groups()
        if s is not None:
            stack[-1].append(Str(s))
        elif p == "(":
            stack.append([])
        elif p == ")":
            x = stack.pop()
            stack[-1].append(x)
        elif a is not None:
            stack[-1].append(a)
    assert len(stack) == 1
    return stack[0]


def show(x):
    if isinstance(x, list):
        return "(" + " ".join(show(y) for y in x) + ")"
    return str(x)


def pretty(form):
    if isinstance(form, list) and form and form[0] == "rule":
        out = [f"({form[0]} {form[1]} {show(form[2])}"]
        i = 3
        while i < len(form):
            v = form[i + 1]
            if isinstance(v, list) and v and v[0] == "and" and len(show(v)) > 88:
                pad = " " * 14
                body = "(and " + ("\n" + pad).join(show(y) for y in v[1:]) + ")"
            else:
                body = show(v)
            out.append(f"  {form[i]:<7} {body}")
            i += 2
        return "\n".join(out) + ")"
    if isinstance(form, list) and form and form[0] == "relation":
        i = form.index(":why")
        return "(" + " ".join(show(y) for y in form[:i]) + "\n  :why " + show(form[i + 1]) + ")"
    return show(form)


def rename(x, m):
    if isinstance(x, Str):
        return x
    if isinstance(x, list):
        return [rename(y, m) for y in x]
    return m.get(x, x)


def name(f):
    return f[1]


def is_rule(f):
    return isinstance(f, list) and bool(f) and f[0] == "rule"


def is_rel(f):
    return isinstance(f, list) and bool(f) and f[0] == "relation"


# ── what replaces the rules that know the base ──────────────────────────────

# The division of base two, in std.arith's names (renamed below with the rest).
DIV2 = r'''
(relation div-digits Tag Number
  :why "the digits of the dividend, for {?1}, which is divided by {?2}")
(relation div-step Tag Number Digits Number Number
  :why "dividing, for {?1}, by {?2}: the digits {?3} are still to bring down, {?4} is left over, and the quotient so far is {?5}")
(relation div-trial Tag Number Digits Number Number
  :why "a trial, for {?1}: the divisor {?2} taken from {?5}")

(rule div-turn ()
  :match  (done (div-m1 ?k ?u) (n ?p ?a))
  :assert (top-first (div-digits ?k (n ?p ?a)) ?u z)
  :why    "the divisor is not zero. Now the digits of {?u}, the top one first")
(rule div-begin ()
  :match  (top-first-is (div-digits ?k ?d) ?t)
  :assert (div-step ?k ?d ?t z z)
  :why    "start at the top digit: nothing is left over yet, and the quotient is zero")
(rule div-bring ()
  :match  (div-step ?k ?d (b ?a ?rest) (n ?p ?c) ?w)
  :assert (sub-walk (div-trial ?k ?d ?rest ?w (n (n ?p ?c) ?a)) (n (n ?p ?c) ?a) ?d 0 z)
  :why    "bring down {?a} beside what is left over, and try to take the divisor from it")
(rule div-bring-first ()
  :match  (div-step ?k ?d (b 1 ?rest) z ?w)
  :assert (sub-walk (div-trial ?k ?d ?rest ?w (n z 1)) (n z 1) ?d 0 z)
  :why    "nothing was left over: bring down 1, and try to take the divisor from it")
(rule div-bring-zero ()
  :match  (div-step ?k ?d (b 0 ?rest) z (n ?p ?c))
  :assert (div-step ?k ?d ?rest z (n (n ?p ?c) 0))
  :why    "nothing was left over and the digit brought down is 0: the next digit of the quotient is 0")
(rule div-bring-zero-none ()
  :match  (div-step ?k ?d (b 0 ?rest) z z)
  :assert (div-step ?k ?d ?rest z z)
  :why    "nothing was left over, the digit brought down is 0 and the quotient so far is zero: no high zero is written")
(rule div-fits ()
  :match  (done (div-trial ?k ?d ?rest ?w ?u) ?t)
  :assert (div-step ?k ?d ?rest ?t (n ?w 1))
  :why    "the divisor fits, and leaves {?t}: the next digit of the quotient is 1")
(rule div-too-big ()
  :match  (short (div-trial ?k ?d ?rest (n ?p ?c) ?u))
  :assert (div-step ?k ?d ?rest ?u (n (n ?p ?c) 0))
  :why    "the divisor does not fit: the next digit of the quotient is 0")
(rule div-too-big-none ()
  :match  (short (div-trial ?k ?d ?rest z ?u))
  :assert (div-step ?k ?d ?rest ?u z)
  :why    "the divisor does not fit and the quotient so far is zero: no high zero is written")
(rule div-end ()
  :match  (div-step ?k ?d z ?t ?w)
  :assert (div-done ?k ?w ?t)
  :why    "no digit is left: the quotient is {?w} and the remainder {?t}")
'''

# Multiplication of base two: a row is nothing or the multiplicand.
MUL2 = r'''
(rule mul-turn ()
  :match  (mul-call ?k ?u ?v)
  :assert (top-first (mul-digits ?k ?u) ?v z)
  :why    "to multiply {?u} by {?v}: first {?v}'s digits, the top one first")
(rule mul-begin ()
  :match  (top-first-is (mul-digits ?k ?u) ?t)
  :assert (mul-step ?k ?u ?t z)
  :why    "nothing is summed yet")
(rule mul-one ()
  :match  (mul-step ?k ?u (b 1 ?rest) (n ?p ?a))
  :assert (add-walk (mul-sum ?k ?u ?rest) (n (n ?p ?a) 0) ?u 0 z)
  :why    "the next digit of the multiplier is 1: twice the sum so far, plus {?u}")
(rule mul-one-first ()
  :match  (mul-step ?k ?u (b 1 ?rest) z)
  :assert (add-walk (mul-sum ?k ?u ?rest) z ?u 0 z)
  :why    "the next digit of the multiplier is 1 and nothing is summed yet: the sum is {?u}, in normal form")
(rule mul-zero ()
  :match  (mul-step ?k ?u (b 0 ?rest) (n ?p ?a))
  :assert (mul-step ?k ?u ?rest (n (n ?p ?a) 0))
  :why    "the next digit of the multiplier is 0: twice the sum so far")
(rule mul-zero-first ()
  :match  (mul-step ?k ?u (b 0 ?rest) z)
  :assert (mul-step ?k ?u ?rest z)
  :why    "the next digit of the multiplier is 0 and nothing is summed yet")
(rule mul-next ()
  :match  (done (mul-sum ?k ?u ?rest) ?s)
  :assert (mul-step ?k ?u ?rest ?s)
  :why    "the sum so far is {?s}")
(rule mul-end ()
  :match  (mul-step ?k ?u z ?acc)
  :assert (done ?k ?acc)
  :why    "no digit of the multiplier is left: the product is {?acc}")
'''

# The divisor in normal form with no digit multiplier, and the strip rule with no `nz`.
NORM2 = r'''
(rule div-multiples ()
  :match  (div-call ?k ?u ?v)
  :assert (add-walk (div-m1 ?k ?u) z ?v 0 z)
  :why    "to divide {?u} by {?v}: first the divisor in normal form, zero plus {?v}")
(rule strip-top ()
  :match  (strip ?k (b 1 ?r))
  :assert (turn ?k (b 1 ?r) z)
  :why    "the top digit is 1: no high zero is left")
'''

DIV_KEEP = {"div-multiples", "div-ask", "div-ask-second", "div-start", "div-answer", "div-by-zero",
            "division-by-zero-is-false"}
DIV_REL_GONE = {"multiples", "div-choose", "div-digit", "div-digits", "div-step", "div-trial"} | \
               {f"div-m{i}" for i in range(2, 10)}
MUL_RULES_GONE = {"mul-column", "mul-walk-end", "mul-carry-out", "mul-row", "mul-first-row", "mul-add-row"}
MUL_REL_GONE = {"mul-walk", "mul-row-of"}
TABLES = ["fa", "fs", "cl"]
# S1f.6.12's check: the rules that flag a malformed number or an undeclared name, by the start of their names,
# their relations, and the two rules of the operands as they were before they handed a number to the check.
CHECK_RULES_GONE = ("check-", "bad-", "let-twice", "let-zero", "let-digit", "operand-digit", "operand-undeclared",
                    "malformed-is-false", "undeclared-is-false")
CHECK_REL_GONE = {"malformed", "undeclared", "check", "bad-operand", "bad-request", "not-a-number", "no-let"}
UNCHECKED = {"operand-number": '"a number stands for itself"', "operand-name": '"{?x} names {?v}"'}


def core(shipped):
    """std.arith without its import, its closing rule, the division of base ten and the digit multiplier."""
    out = []
    for f in shipped:
        if f[0] in ("import", "arith-rule"):
            continue
        if (is_rule(f) and name(f) == "arith") or (is_rel(f) and name(f) == "arith-rule"):
            continue
        if is_rule(f) and name(f).startswith("div") and name(f) not in DIV_KEEP:
            continue
        if is_rel(f) and name(f) in DIV_REL_GONE:
            continue
        if is_rule(f) and name(f) in MUL_RULES_GONE:
            continue
        if is_rel(f) and name(f) in MUL_REL_GONE:
            continue
        if is_rule(f) and name(f).startswith(CHECK_RULES_GONE):
            continue
        if is_rel(f) and name(f) in CHECK_REL_GONE:
            continue
        f = copy.deepcopy(f)
        if is_rule(f) and name(f) in UNCHECKED:
            # `(and (operand-is …) (check …))` back to the one conclusion, and the sentence with it.
            i = f.index(":assert") + 1
            assert f[i][0] == "and" and len(f[i]) == 3 and f[i][2][0] == "check", f
            f[i] = f[i][1]
            f[f.index(":why") + 1] = Str(UNCHECKED[name(f)])
        out.append(f)
    return out


def replace(forms, text):
    new = parse(text)
    names = {name(f) for f in new}
    return [f for f in forms if not ((is_rule(f) or is_rel(f)) and name(f) in names)] + new


def closing(forms):
    rels = [name(f) for f in forms if is_rel(f)]
    rls = [name(f) for f in forms if is_rule(f)]
    body = ["and"] + [["__closed__", t] for t in TABLES] + [["__closed__", r] for r in rels] + \
           [["__closed__", "arith-rule"]] + [["arith-rule", r] for r in rls]
    return [parse('(relation arith-rule Rule :why "{?1} is a rule of the theory")')[0],
            ["arith-rule", "arith"],
            ["rule", "arith", [], ":match", [], ":assert", body, ":why",
             Str('"binary arithmetic is computed and never guessed: every relation of the theory is closed"')]]


# ── the tables ──────────────────────────────────────────────────────────────

WHY = {
    "fa": ("Bit Bit Bit Bit Bit", "{?1} + {?2} + carry {?3} is digit {?4}, carry {?5}"),
    "fs": ("Bit Bit Bit Bit Bit", "{?1} - {?2} - borrow {?3} is digit {?4}, borrow {?5}"),
    "cl": ("Order Bit Bit Order", "the order below is {?1}; {?2} against {?3} above it makes it {?4}"),
}


def tables():
    """The rows of `utils/gen_arith_tables.py`'s three tables at base two."""
    B = 2
    t = {}
    t["fa"] = [(a, b, c, (a + b + c) % B, (a + b + c) // B) for a in range(B) for c in range(2) for b in range(B)]
    t["fs"] = [(a, b, c, (a - b - c) % B, 1 if a - b - c < 0 else 0) for a in range(B) for c in range(2) for b in range(B)]
    t["cl"] = [(o, a, b, "lt" if a < b else "gt" if a > b else o)
               for o in ("lt", "eq", "gt") for a in range(B) for b in range(B)]
    return t


BITS_HEADER = """\
;;; The tables of binary arithmetic: 28 rows, where decimal's `std.digits` has
;;; 1 664. GENERATED by `examples/binary_arithmetic/gen_binary.py`: edit the
;;; script and run it, never the rows.
;;;
;;;   (fa2 A B CARRY-IN DIGIT CARRY-OUT)     the full adder, 8 rows
;;;   (fs2 A B BORROW-IN DIGIT BORROW-OUT)   the full subtractor, 8 rows
;;;   (cl2 BELOW A B ORDER)                  comparison, a column at a time, 12 rows
;;;
;;; The two bits are the names `0` and `1`, which decimal's digits already
;;; are. The module holds facts and no rule; `bin.ein` beside it imports the
;;; three tables and closes them in its closing rule. A program that reads a
;;; table without that theory closes it itself, `(__closed__ fa2)`.
;;;
;;; M1f P1f.6 (S1f.6.16): `plans/…/p1f.6_calculator/binary_arithmetic.md`.
"""

BIN_HEADER = """\
;;; Binary arithmetic, a theory of its own, computed by saturation alone.
;;; GENERATED by `examples/binary_arithmetic/gen_binary.py` from the shipped
;;; `stdlib/arith.ein`: edit the script and run it, never this file.
;;;
;;;   (b2 PREFIX BIT)        a binary number, in reading order: twice the
;;;                          prefix, plus the bit; `z` is zero. Five, 101, is
;;;                          (b2 (b2 (b2 z 1) 0) 1)
;;;   (add2 X Y)             a request, answered by (sum-of2 X Y S)
;;;   (sub2 X Y)             answered by (diff-of2 X Y D R): D is X - Y where X
;;;                          is not the smaller, R what X is short by otherwise
;;;   (cmp2 X Y)             answered by (cmp-of2 X Y O), O one of lt, eq, gt
;;;   (mul2 X Y)             answered by (product-of2 X Y P)
;;;   (div2 X Y)             answered by (quot-of2 X Y Q R); a divisor that is
;;;                          zero derives (division-by-zero2 (div2 X Y)) and
;;;                          then (false)
;;;   (let2 NAME NUMBER)     a named constant; a request may name its operands
;;;
;;; One name brings the theory: (import bin :symbols (bin)). `bin` is the
;;; closing rule, which closes every relation and names every rule, so no fact
;;; is ever guessed: R is {∅}, or empty for a division by zero.
;;;
;;; This is way (b) of the plan page: `std.arith`'s rules of the operands, of
;;; addition, subtraction and comparison, the same up to their names (nothing
;;; in them says ten); a multiplication with no digit multiplier, since a row
;;; is nothing or the multiplicand; and a division in which one subtraction
;;; decides a quotient bit, where decimal bisects nine multiples. 67 rules
;;; where decimal has 88, three tables where it has seven.
;;;
;;; The names are placeholders, the shipped ones with a `2`; the shape, the
;;; names and whether binary arithmetic ships at all are the plan page's
;;; questions, open. This is an example, not a stdlib module.
;;;
;;; M1f P1f.6 (S1f.6.16): `plans/…/p1f.6_calculator/binary_arithmetic.md`.
"""


def build():
    shipped = parse(SHIPPED.read_text())
    forms = core(shipped)
    forms = replace(forms, MUL2)
    forms = replace(forms, NORM2)
    forms = forms + parse(DIV2)
    forms = forms + closing(forms)
    names = {name(f) for f in forms if is_rule(f) or is_rel(f)} | set(TABLES)
    m = {x: x + "2" for x in names}
    m["n"] = "b2"
    m["b"] = "a2"
    m["arith"] = "bin"
    forms = rename(forms, m)
    forms = [parse(f"(import bits :symbols ({' '.join(m[t] for t in TABLES)}))")[0]] + forms
    bin_text = BIN_HEADER + "\n".join(pretty(f) for f in forms) + "\n"

    t = tables()
    out = [BITS_HEADER.rstrip("\n")]
    for n in TABLES:
        ty, why = WHY[n]
        out.append(f'(relation {m[n]} {ty}\n  :why "{why}")')
    for n in TABLES:
        out.append(f";; {m[n]}: {len(t[n])} rows")
        rows = [f"({m[n]} " + " ".join(str(x) for x in r) + ")" for r in t[n]]
        for i in range(0, len(rows), 4):
            out.append(" ".join(rows[i:i + 4]))
    bits_text = "\n".join(out) + "\n"
    return {
        HERE / "bits.ein": bits_text,
        HERE / "bin.ein": bin_text,
        REPO / "examples" / "binary_logic" / "bits.ein": bits_text,
    }, forms


def main(argv):
    files, forms = build()
    if "--count" in argv:
        rules = [f for f in forms if is_rule(f)]
        print("rules", len(rules) - 1, "and the closing rule; relations", len([f for f in forms if is_rel(f)]),
              "; rules with a second premise",
              sum(1 for f in rules if isinstance(f[4], list) and f[4] and f[4][0] == "and"))
        return 0
    if "--check" in argv:
        stale = [p for p, text in files.items() if not p.exists() or p.read_text() != text]
        for p in stale:
            print(f"stale: {p.relative_to(REPO)} — run `python3 examples/binary_arithmetic/gen_binary.py`",
                  file=sys.stderr)
        return 1 if stale else 0
    for p, text in files.items():
        p.write_text(text)
        print("wrote", p.relative_to(REPO))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
