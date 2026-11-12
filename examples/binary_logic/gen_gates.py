#!/usr/bin/env python3
"""Write the generated modules of `examples/binary_logic/`: the sixteen functions and what is known of them.

M1f P1f.6, S1f.6.17. The plan page, `plans/m1f_hypothesis_and_documentation/
p1f.6_calculator/binary_logic.md`, recommends one table for the sixteen
functions of two inputs, `(gate2 NAME A B OUT)`, and the thirty relations and
properties known of them as generated facts, with the rules that derive each
from the 64 rows kept as their check. This script writes those four files
from sixteen four-bit columns:

    gates.ein         the table, 64 rows, each function's second name, and
                      three small tables the rules read: inv, le, bit
    props.ein         the thirty relations and properties, as facts
    props_rules.ein   the 31 rules that derive every one of them from the rows
    props_check.ein   a program: it imports the rules and claims, relation by
                      relation, exactly the facts of props.ein

So the facts are produced twice, here from the columns and in Ein from the
rows, and `ein test examples/binary_logic/props_check.ein` holds the two
equal: a claim that names a relation closes it, so it holds only if the rules
derive every listed fact and no other.

    python3 examples/binary_logic/gen_gates.py            # write the files
    python3 examples/binary_logic/gen_gates.py --check    # exit 1 if one differs

The script is run by hand and is not in the gate. What the gate holds is the
claim of `props_check.ein`, a corpus program with a `test` cell.

The gate names are the stage file's proposal; they and the layout are the plan
page's questions, open. No dependency beyond the standard library.
"""
from __future__ import annotations

import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]

# column (the outputs at a b = 00 01 10 11) -> the proposed gate name
NAMES = {"0000": "Zero", "0001": "And", "0010": "Andn", "0011": "Fst", "0100": "Randn", "0101": "Snd",
         "0110": "Xor", "0111": "Or", "1000": "Nor", "1001": "Xnor", "1010": "Nsnd", "1011": "Orn",
         "1100": "Nfst", "1101": "Rorn", "1110": "Nand", "1111": "One"}
COLS = {v: k for k, v in NAMES.items()}
F = list(NAMES.values())
B = (0, 1)


def f(name, a, b):
    return int(COLS[name][2 * a + b])


def named(fn):
    return NAMES["".join(str(fn(a, b)) for a in B for b in B)]


def properties():
    """Every relation and property, computed from the columns: {relation: set of tuples}."""
    P = {}
    P["column"] = {(n, *COLS[n]) for n in F}
    P["swap-of"] = {(n, named(lambda a, b: f(n, b, a))) for n in F}
    P["complement-of"] = {(n, named(lambda a, b: 1 - f(n, a, b))) for n in F}
    P["dual-of"] = {(n, named(lambda a, b: 1 - f(n, 1 - a, 1 - b))) for n in F}
    P["left-negated"] = {(n, named(lambda a, b: f(n, 1 - a, b))) for n in F}
    P["right-negated"] = {(n, named(lambda a, b: f(n, a, 1 - b))) for n in F}
    P["commutative"] = {(n,) for n in F if all(f(n, a, b) == f(n, b, a) for a in B for b in B)}
    P["associative"] = {(n,) for n in F
                        if all(f(n, f(n, a, b), c) == f(n, a, f(n, b, c)) for a in B for b in B for c in B)}
    P["idempotent"] = {(n,) for n in F if all(f(n, a, a) == a for a in B)}
    P["reads-first"] = {(n,) for n in F if any(f(n, 0, b) != f(n, 1, b) for b in B)}
    P["reads-second"] = {(n,) for n in F if any(f(n, a, 0) != f(n, a, 1) for a in B)}
    P["ignores-first"] = {(n,) for n in F} - P["reads-first"]
    P["ignores-second"] = {(n,) for n in F} - P["reads-second"]
    P["right-identity-of"] = {(n, str(e)) for n in F for e in B if all(f(n, a, e) == a for a in B)}
    P["left-identity-of"] = {(n, str(e)) for n in F for e in B if all(f(n, e, a) == a for a in B)}
    P["identity-of"] = P["right-identity-of"] & P["left-identity-of"]
    P["right-absorbing-of"] = {(n, str(e)) for n in F for e in B if all(f(n, a, e) == e for a in B)}
    P["left-absorbing-of"] = {(n, str(e)) for n in F for e in B if all(f(n, e, a) == e for a in B)}
    P["absorbing-of"] = P["right-absorbing-of"] & P["left-absorbing-of"]
    P["preserves-zero"] = {(n,) for n in F if f(n, 0, 0) == 0}
    P["preserves-one"] = {(n,) for n in F if f(n, 1, 1) == 1}
    P["monotone"] = {(n,) for n in F
                     if all(f(n, a, b) <= f(n, c, d) for a in B for b in B for c in B for d in B if a <= c and b <= d)}
    P["affine"] = {(n,) for n in F if sum(int(x) for x in COLS[n]) % 2 == 0}
    P["self-dual"] = {(n,) for n in F if (n, n) in P["dual-of"]}
    allf = {(n,) for n in F}
    P["escapes-zero"] = allf - P["preserves-zero"]
    P["escapes-one"] = allf - P["preserves-one"]
    P["escapes-monotone"] = allf - P["monotone"]
    P["escapes-affine"] = allf - P["affine"]
    P["escapes-self-dual"] = allf - P["self-dual"]
    P["complete-alone"] = (P["escapes-zero"] & P["escapes-one"] & P["escapes-monotone"]
                           & P["escapes-affine"] & P["escapes-self-dual"])
    return P


# relation -> (its signature, its sentence)
WHYS = {
    "column": ("Function Bit Bit Bit Bit", "{?1} gives {?2}, {?3}, {?4} and {?5} at 00, 01, 10 and 11"),
    "swap-of": ("Function Function", "{?2} is {?1} with its inputs exchanged"),
    "complement-of": ("Function Function", "{?2} is {?1} with its output negated"),
    "dual-of": ("Function Function", "{?2} is De Morgan's dual of {?1}"),
    "left-negated": ("Function Function", "{?2} is {?1} with its first input negated"),
    "right-negated": ("Function Function", "{?2} is {?1} with its second input negated"),
    "commutative": ("Function", "{?1} is commutative"),
    "associative": ("Function", "{?1} is associative"),
    "idempotent": ("Function", "{?1} of a bit and itself is that bit"),
    "reads-first": ("Function", "{?1} depends on its first input"),
    "reads-second": ("Function", "{?1} depends on its second input"),
    "ignores-first": ("Function", "{?1} does not depend on its first input"),
    "ignores-second": ("Function", "{?1} does not depend on its second input"),
    "right-identity-of": ("Function Bit", "{?2} as the second input of {?1} gives the first"),
    "left-identity-of": ("Function Bit", "{?2} as the first input of {?1} gives the second"),
    "identity-of": ("Function Bit", "{?2} is neutral for {?1}"),
    "right-absorbing-of": ("Function Bit", "{?2} as the second input of {?1} gives {?2}"),
    "left-absorbing-of": ("Function Bit", "{?2} as the first input of {?1} gives {?2}"),
    "absorbing-of": ("Function Bit", "{?2} decides {?1}"),
    "preserves-zero": ("Function", "{?1} of 0 and 0 is 0"),
    "preserves-one": ("Function", "{?1} of 1 and 1 is 1"),
    "monotone": ("Function", "raising an input of {?1} never lowers its output"),
    "affine": ("Function", "{?1} is a sum modulo 2 of inputs and a constant"),
    "self-dual": ("Function", "{?1} is its own dual"),
    "escapes-zero": ("Function", "{?1} of 0 and 0 is 1"),
    "escapes-one": ("Function", "{?1} of 1 and 1 is 0"),
    "escapes-monotone": ("Function", "{?1} is not monotone"),
    "escapes-affine": ("Function", "{?1} is not affine"),
    "escapes-self-dual": ("Function", "{?1} is not its own dual"),
    "complete-alone": ("Function", "every function can be built from {?1} alone"),
}


def decls():
    return [f'(relation {n} {t}\n  :why "{w}")' for n, (t, w) in WHYS.items()]


def wrap(items, width=100, indent=""):
    lines, cur = [], indent
    for it in items:
        if len(cur) + len(it) + 1 > width and cur.strip():
            lines.append(cur.rstrip())
            cur = indent
        cur += it + " "
    if cur.strip():
        lines.append(cur.rstrip())
    return "\n".join(lines)


FOOT = ";;; M1f P1f.6 (S1f.6.17): `plans/…/p1f.6_calculator/binary_logic.md`.\n"

GATES_HEADER = """\
;;; The sixteen functions of two inputs, one table: 64 rows.
;;; GENERATED by `examples/binary_logic/gen_gates.py`: edit the script and
;;; run it, never the rows.
;;;
;;;   (gate2 NAME A B OUT)       NAME of A and B is OUT; four rows a function
;;;   (column-name NAME fABCD)   the function's second name: its outputs at
;;;                              a b = 00, 01, 10 and 11, so Xor is f0110
;;;   (inv A B)                  negation, two rows
;;;   (le A B)                   the order of the bits, three rows
;;;   (bit A)                    the two bits
;;;
;;; A function of two bits is its column of four outputs, and there are
;;; sixteen columns; every one has a name here, exactly once. The names are
;;; capitalised because `and`, `or` and `not` are Ein's own words and cannot
;;; be a value. `Andn` is a and not b, `Orn` is a or not b, and `Randn` and
;;; `Rorn` are those two with the inputs exchanged; `Fst` and `Snd` give back
;;; one input, `Nfst` and `Nsnd` its negation.
;;;
;;; A function is a value here, so a rule about any function matches
;;; (gate2 ?f …) with no parameter, and a search can make the function a
;;; hypothesis. The tables are closed by written facts: a program brings them
;;; with (import gates :symbols (gate2 column-name inv le bit __closed__)).
;;;
;;; The names and the layout are the stage file's proposal and the plan
;;; page's recommendation; both are the page's questions, open.
;;;
"""

PROPS_HEADER = """\
;;; What is known of the sixteen functions, as facts: thirty relations, each
;;; function's column, (column F A B C D), and twenty-nine relations and
;;; properties read off it. GENERATED by `examples/binary_logic/gen_gates.py` from the
;;; functions' columns: edit the script and run it, never the rows.
;;;
;;; Relations between two functions, sixteen facts each: swap-of (the inputs
;;; exchanged), complement-of (the output negated), dual-of (De Morgan's
;;; dual), left-negated and right-negated (one input negated).
;;;
;;; Properties of one function: commutative, associative, idempotent; which
;;; inputs it reads or ignores; its identity and absorbing bits, two-sided and
;;; one-sided; and Post's five classes, from inside (preserves-zero,
;;; preserves-one, monotone, affine, self-dual) and from outside (the five
;;; `escapes-` relations). `complete-alone` holds of Nand and of Nor, the two
;;; outside every class.
;;;
;;; A program reads them by a flat import of the relations it wants, for
;;; instance (import props :symbols (commutative identity-of __closed__)).
;;; Nothing here is derived when a program loads: `props_rules.ein` derives
;;; every one of these facts from the 64 rows of `gates.ein`, and
;;; `props_check.ein` holds the two equal.
;;;
"""

RULES_HEADER = """\
;;; The rules that derive what is known of the sixteen functions from the 64
;;; rows of `gates.ein`. GENERATED by `examples/binary_logic/gen_gates.py`,
;;; which holds this text: edit the script and run it.
;;;
;;; 31 rules and a closing rule, all monotone: no `absent` and no `not`. A
;;; function has four rows and all four are written, so *for every input* is a
;;; conjunction of premises, and *not in a class* is one witness row or pair
;;; of rows. `affine-r` reads Xor's own rows: the table states a property of
;;; the table.
;;;
;;; This module is the check of `props.ein`, which holds the same facts
;;; written out: `props_check.ein` imports the rules and claims exactly those
;;; facts. A program that wants the properties imports `props.ein` and pays
;;; nothing for them at load; one that imports this module instead derives
;;; them, in some four hundred firings.
;;;
;;;   (import props_rules :symbols (props-derived))   every rule, by the entry
;;;
"""

RULES = r'''
(relation assoc-at Function Bit Bit Bit
  :why "{?1} associates at {?2}, {?3}, {?4}")
(relation props-rule Rule
  :why "{?1} is a rule that derives what is known of the sixteen")
(props-rule props-derived)

(rule column-of ()
  :match  (and (gate2 ?f 0 0 ?a) (gate2 ?f 0 1 ?b) (gate2 ?f 1 0 ?c) (gate2 ?f 1 1 ?d))
  :assert (column ?f ?a ?b ?c ?d)
  :why    "the four rows of {?f}")
(rule swap ()
  :match  (and (column ?f ?a ?b ?c ?d) (column ?g ?a ?c ?b ?d))
  :assert (swap-of ?f ?g)
  :why    "{?g}'s rows at 01 and 10 are {?f}'s at 10 and 01")
(rule complement ()
  :match  (and (column ?f ?a ?b ?c ?d) (inv ?a ?a1) (inv ?b ?b1) (inv ?c ?c1) (inv ?d ?d1)
               (column ?g ?a1 ?b1 ?c1 ?d1))
  :assert (complement-of ?f ?g)
  :why    "every output of {?g} is the negation of {?f}'s")
(rule dual ()
  :match  (and (column ?f ?a ?b ?c ?d) (inv ?a ?a1) (inv ?b ?b1) (inv ?c ?c1) (inv ?d ?d1)
               (column ?g ?d1 ?c1 ?b1 ?a1))
  :assert (dual-of ?f ?g)
  :why    "{?g} is {?f} with both inputs and the output negated")
(rule left-neg ()
  :match  (and (column ?f ?a ?b ?c ?d) (column ?g ?c ?d ?a ?b))
  :assert (left-negated ?f ?g)
  :why    "{?g} is {?f} with its first input negated")
(rule right-neg ()
  :match  (and (column ?f ?a ?b ?c ?d) (column ?g ?b ?a ?d ?c))
  :assert (right-negated ?f ?g)
  :why    "{?g} is {?f} with its second input negated")
(rule commutative-r ()
  :match  (column ?f ?a ?b ?b ?d)
  :assert (commutative ?f)
  :why    "{?f} gives the same at 01 and at 10")
(rule assoc-point ()
  :match  (and (gate2 ?f ?a ?b ?x) (gate2 ?f ?x ?c ?l) (gate2 ?f ?b ?c ?y) (gate2 ?f ?a ?y ?l))
  :assert (assoc-at ?f ?a ?b ?c)
  :why    "({?a} {?f} {?b}) {?f} {?c} and {?a} {?f} ({?b} {?f} {?c}) are both {?l}")
(rule associative-r ()
  :match  (and (assoc-at ?f 0 0 0) (assoc-at ?f 0 0 1) (assoc-at ?f 0 1 0) (assoc-at ?f 0 1 1)
               (assoc-at ?f 1 0 0) (assoc-at ?f 1 0 1) (assoc-at ?f 1 1 0) (assoc-at ?f 1 1 1))
  :assert (associative ?f)
  :why    "{?f} associates at all eight inputs")
(rule idempotent-r ()
  :match  (and (gate2 ?f 0 0 0) (gate2 ?f 1 1 1))
  :assert (idempotent ?f)
  :why    "{?f} of 0 and 0 is 0, and of 1 and 1 is 1")
(rule reads-first-r ()
  :match  (and (gate2 ?f 0 ?b ?x) (gate2 ?f 1 ?b ?y) (inv ?x ?y))
  :assert (reads-first ?f)
  :why    "with the second input {?b}, changing the first changes {?f}")
(rule reads-second-r ()
  :match  (and (gate2 ?f ?a 0 ?x) (gate2 ?f ?a 1 ?y) (inv ?x ?y))
  :assert (reads-second ?f)
  :why    "with the first input {?a}, changing the second changes {?f}")
(rule ignores-first-r ()
  :match  (column ?f ?a ?b ?a ?b)
  :assert (ignores-first ?f)
  :why    "{?f}'s rows do not change with the first input")
(rule ignores-second-r ()
  :match  (column ?f ?a ?a ?c ?c)
  :assert (ignores-second ?f)
  :why    "{?f}'s rows do not change with the second input")
(rule right-identity ()
  :match  (and (gate2 ?f 0 ?e 0) (gate2 ?f 1 ?e 1))
  :assert (right-identity-of ?f ?e)
  :why    "{?f} of a bit and {?e} is that bit")
(rule left-identity ()
  :match  (and (gate2 ?f ?e 0 0) (gate2 ?f ?e 1 1))
  :assert (left-identity-of ?f ?e)
  :why    "{?f} of {?e} and a bit is that bit")
(rule identity ()
  :match  (and (right-identity-of ?f ?e) (left-identity-of ?f ?e))
  :assert (identity-of ?f ?e)
  :why    "{?e} is neutral for {?f} on both sides")
(rule right-absorbing ()
  :match  (and (gate2 ?f 0 ?e ?e) (gate2 ?f 1 ?e ?e))
  :assert (right-absorbing-of ?f ?e)
  :why    "{?f} of any bit and {?e} is {?e}")
(rule left-absorbing ()
  :match  (and (gate2 ?f ?e 0 ?e) (gate2 ?f ?e 1 ?e))
  :assert (left-absorbing-of ?f ?e)
  :why    "{?f} of {?e} and any bit is {?e}")
(rule absorbing ()
  :match  (and (right-absorbing-of ?f ?e) (left-absorbing-of ?f ?e))
  :assert (absorbing-of ?f ?e)
  :why    "{?e} decides {?f} on both sides")
(rule preserves-zero-r ()
  :match  (gate2 ?f 0 0 0)
  :assert (preserves-zero ?f)
  :why    "{?f} of 0 and 0 is 0")
(rule preserves-one-r ()
  :match  (gate2 ?f 1 1 1)
  :assert (preserves-one ?f)
  :why    "{?f} of 1 and 1 is 1")
(rule monotone-r ()
  :match  (and (column ?f ?a ?b ?c ?d) (le ?a ?b) (le ?a ?c) (le ?b ?d) (le ?c ?d))
  :assert (monotone ?f)
  :why    "{?f}'s outputs never fall as an input rises")
(rule affine-r ()
  :match  (and (column ?f ?a ?b ?c ?d) (gate2 Xor ?a ?b ?x) (gate2 Xor ?c ?d ?x))
  :assert (affine ?f)
  :why    "{?f}'s four outputs have an even number of ones")
(rule self-dual-r ()
  :match  (dual-of ?f ?f)
  :assert (self-dual ?f)
  :why    "{?f} is its own dual")
;; Post's five classes from outside: each *is not in the class* is one witness row or pair of rows
(rule escapes-zero-r ()
  :match  (gate2 ?f 0 0 1)
  :assert (escapes-zero ?f)
  :why    "{?f} of 0 and 0 is 1")
(rule escapes-one-r ()
  :match  (gate2 ?f 1 1 0)
  :assert (escapes-one ?f)
  :why    "{?f} of 1 and 1 is 0")
(rule escapes-monotone-r ()
  :match  (and (gate2 ?f ?a ?b 1) (le ?a ?c) (le ?b ?d) (gate2 ?f ?c ?d 0))
  :assert (escapes-monotone ?f)
  :why    "{?f} is 1 at {?a}{?b} and 0 at {?c}{?d}, which is not lower")
(rule escapes-affine-r ()
  :match  (and (column ?f ?a ?b ?c ?d) (gate2 Xor ?a ?b ?x) (gate2 Xor ?c ?d ?y) (inv ?x ?y))
  :assert (escapes-affine ?f)
  :why    "{?f}'s four outputs have an odd number of ones")
(rule escapes-self-dual-r ()
  :match  (and (gate2 ?f ?a ?b ?x) (inv ?a ?a1) (inv ?b ?b1) (gate2 ?f ?a1 ?b1 ?x))
  :assert (escapes-self-dual ?f)
  :why    "{?f} gives {?x} at {?a}{?b} and at its opposite")
(rule complete-alone-r ()
  :match  (and (escapes-zero ?f) (escapes-one ?f) (escapes-monotone ?f) (escapes-affine ?f) (escapes-self-dual ?f))
  :assert (complete-alone ?f)
  :why    "{?f} is outside each of Post's five classes")
'''

RULE_NAMES = ["column-of", "swap", "complement", "dual", "left-neg", "right-neg", "commutative-r", "assoc-point",
              "associative-r", "idempotent-r", "reads-first-r", "reads-second-r", "ignores-first-r",
              "ignores-second-r", "right-identity", "left-identity", "identity", "right-absorbing",
              "left-absorbing", "absorbing", "preserves-zero-r", "preserves-one-r", "monotone-r", "affine-r",
              "self-dual-r", "escapes-zero-r", "escapes-one-r", "escapes-monotone-r", "escapes-affine-r",
              "escapes-self-dual-r", "complete-alone-r"]

CHECK_HEADER = """\
;;; The check of `props.ein`: the rules of `props_rules.ein` derive, from the
;;; 64 rows of `gates.ein`, exactly the facts `props.ein` writes out.
;;; GENERATED by `examples/binary_logic/gen_gates.py`, with `props.ein`, from
;;; the same columns.
;;;
;;; One query a relation, each with its claim. A claim that names a relation
;;; closes it, so each holds only if the rules derive every listed fact and
;;; no other: Python's count from the columns and Ein's derivation from the
;;; rows agree on all thirty relations and on the sixteen columns.
;;;
;;;   ein test examples/binary_logic/props_check.ein
;;;
;;; Search-free: every relation is closed, so there is no candidate, one
;;; state and one saturation.
;;;
"""


def build():
    PR = properties()
    assert RULES.count("(rule ") == len(RULE_NAMES) == 31
    for n in RULE_NAMES:
        assert f"(rule {n} ()" in RULES, n

    g = [GATES_HEADER + FOOT.rstrip("\n"),
         '(relation gate2 Function Bit Bit Bit\n  :why "{?1} of {?2} and {?3} is {?4}")',
         '(relation column-name Function Column\n  :why "{?1} is the function whose outputs at 00, 01, 10 and 11 spell {?2}")',
         '(relation inv Bit Bit\n  :why "not {?1} is {?2}")',
         '(relation le Bit Bit\n  :why "{?1} is at most {?2}")',
         '(relation bit Bit\n  :why "{?1} is a bit")',
         "(__closed__ gate2) (__closed__ column-name) (__closed__ inv) (__closed__ le) (__closed__ bit)",
         "(bit 0) (bit 1)", "(inv 0 1) (inv 1 0)", "(le 0 0) (le 0 1) (le 1 1)"]
    for n in F:
        g.append(" ".join(f"(gate2 {n} {a} {b} {f(n, a, b)})" for a in B for b in B) + f"   (column-name {n} f{COLS[n]})")
    gates = "\n".join(g) + "\n"

    p = [PROPS_HEADER + FOOT.rstrip("\n")] + decls()
    p.append(wrap([f"(__closed__ {n})" for n in WHYS]))
    for n in WHYS:
        p.append(f";; {n}: {len(PR[n])}")
        p.append(wrap([f"({n} {' '.join(t)})" for t in sorted(PR[n])]))
    props = "\n".join(p) + "\n"

    closed = ["(__closed__ %s)" % n for n in list(WHYS) + ["assoc-at", "props-rule"]]
    named_rules = ["(props-rule %s)" % n for n in RULE_NAMES]
    closing = ("(rule props-derived ()\n  :match  ()\n  :assert (and "
               + wrap(closed + named_rules, 100, " " * 15).lstrip()
               + ')\n  :why    "what is known of the sixteen is derived from the rows and never guessed")')
    r = [RULES_HEADER + FOOT.rstrip("\n"),
         "(import gates :symbols (gate2 column-name inv le bit __closed__))"] + decls() + [RULES.strip(), closing]
    rules = "\n".join(r) + "\n"

    c = [CHECK_HEADER + FOOT.rstrip("\n"), "(import props_rules :symbols (props-derived))"]
    for n, (t, _) in WHYS.items():
        vs = " ".join(f"?{chr(ord('a') + i)}" for i in range(len(t.split())))
        c.append(f"(query :goal ({n} {vs})\n       :expect (model "
                 + wrap([f"({n} {' '.join(x)})" for x in sorted(PR[n])], 100, " " * 22).lstrip() + "))")
    check = "\n".join(c) + "\n"
    return {HERE / "gates.ein": gates, HERE / "props.ein": props, HERE / "props_rules.ein": rules,
            HERE / "props_check.ein": check}, PR


def main(argv):
    files, PR = build()
    if "--count" in argv:
        for n in WHYS:
            print(f"{n:20} {len(PR[n])}")
        return 0
    if "--check" in argv:
        stale = [p for p, text in files.items() if not p.exists() or p.read_text() != text]
        for p in stale:
            print(f"stale: {p.relative_to(REPO)} — run `python3 examples/binary_logic/gen_gates.py`", file=sys.stderr)
        return 1 if stale else 0
    for p, text in files.items():
        p.write_text(text)
        print("wrote", p.relative_to(REPO))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
