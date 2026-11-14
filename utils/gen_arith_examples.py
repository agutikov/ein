#!/usr/bin/env python3
"""Write the three generated examples of `examples/arith/` (M1f S1f.6.15), or one program of a stated size.

The user, 2026-11-11: *"add examples with long numbers and many calculations
so we can compare load time with calculation time"*. A small arithmetic
program spends its run reading the tables of `std.digits`, not calculating.
These three are the other end, each a different way a program can be long:

    examples/arith/long_numbers.ein        few requests, long operands
    examples/arith/many_calculations.ein   many requests, short operands
    examples/arith/factorial.ein           one written fact and a long chain:
                                           the numbers are derived, not written

Nobody should type a number of a hundred digits, and nobody should check one
by eye, so the files are written by this script, operands drawn with a fixed
seed and every claimed answer computed here, by Python's integers. Python is
the oracle for the files and enters no program.

    python3 utils/gen_arith_examples.py            # write the three files
    python3 utils/gen_arith_examples.py --check    # exit 1 if one differs

`ein-cli/tests/arith_search_free.rs` runs `--check` inside `cargo test`, which
is how the check is in the gate, and `ein test examples/arith` holds every
claim the files make to the engine's own answers.

**The sizes are the corpus's, not the theory's.** Every corpus program is
solved, dumped and reloaded per commit, and `ein solve` writes every fact of
the state as text: a circuit's state shares its numbers in the store and not
on the page (E-24 of P1f.6's `findings.md`). So the shipped sizes are the ones
every surface affords, and `utils/arith_cost.py` runs the sweep past them, to
operands of 900 digits and 10 000 requests, with programs it writes itself
through the three functions below (`long_numbers`, `many_calculations`,
`factorial`), which take the sizes as arguments:

    python3 utils/gen_arith_examples.py --out DIR --digits 300 --short 60 \\
        --requests 2000 --factorial 120

**One program of a stated size, on demand** (M1f S1f.6.29). The user,
2026-11-12: *"Let's have biggest solvable arithmetic calculation in let's say
100+-50 seconds on this host. make a generator in python that takes some size
arguments and generate an example"*, and, told that memory ends a calculation
before time does, *"Ok, then pick the one that fits into memory, say ~16GB;
do not call it in tests and in CI, just for manual solve call"*. `--op` writes
one program to the path `-o` names:

    python3 utils/gen_arith_examples.py --op mul --digits 300 -o build/big/mul.ein
    python3 utils/gen_arith_examples.py --op div --digits 300 --dividend 600 -o build/big/div.ein
    python3 utils/gen_arith_examples.py --op mix --digits 9 --count 1000 -o build/big/mix.ein
    python3 utils/gen_arith_examples.py --op factorial --n 200 -o build/big/chain.ein

    --op        add, sub, cmp, mul, div; mix, the five in turn; factorial, the chain
    --digits N  the digits of each operand, exactly, at most 999 (a written
                number nests a level a digit); a division's divisor
    --dividend  a division's dividend, in digits (default twice the divisor's)
    --count N   requests in all (default 1); one request names its operands with
                `let`, many write theirs
    --n N       the factorial chain's last number
    --seed S    the operands' seed (default the shipped files'), so a size and a
                seed name one program, whatever the path
    --no-expect leave every `:expect` out, for a size where the claim's text
                would weigh more than the program

A claim is stated where Python computes the answer, which is everywhere, and
written where it nests within the parser's limit of 1 000 parentheses
(`ein_ir::grammar::NESTING_LIMIT`); a claim past it is left out, and the
file's header says so. Such a program is **not checked in and is run by no
test, gate step, nightly step or dev-host step**: it is written under `build/`
and solved by hand. `utils/arith_cost.py --biggest` finds the sizes that fit in
about 16 GB, and `stdlib/README.md` § The biggest calculation has them. With no
`--op`, nothing of this mode runs: `--check` holds the three shipped files and
nothing else.

No dependency beyond the standard library.
"""
from __future__ import annotations

import argparse
import math
import random
import re
import sys
import textwrap
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
OUT = REPO / "examples" / "arith"

SEED = 20261111

# The shipped sizes (see the docstring for why they are these).
DIGITS = 100      # long_numbers: the operands of +, - and the comparison
SHORT = 10        # long_numbers: the operands of x, and the divisor; the dividend has twice as many
REQUESTS = 20     # many_calculations: requests an operation, five operations
OPERAND = 6       # many_calculations: an operand has 1 to this many digits
FACTORIAL = 25    # factorial: the last number of the chain

OPS = ("add", "sub", "cmp", "mul", "div")
# Under M1f S1f.6.35's imports, a program lists the short names it writes from the module that declares
# them: the entry, and every name of `std.arith`'s the text writes, in the order it first writes them.
ARITH_NAMES = None


def arith_names() -> set:
    """Every name `stdlib/arith.ein` declares."""
    global ARITH_NAMES
    if ARITH_NAMES is None:
        text = (REPO / "stdlib" / "arith.ein").read_text()
        ARITH_NAMES = set(re.findall(r"^\((?:relation|rule|hrule|macro)\s+(\S+)", text, re.M))
    return ARITH_NAMES


def import_line(body: str) -> str:
    """`(import std.arith :symbols (arith …))`: the entry and the names `body` writes, wrapped at 100
    columns under the list's first name."""
    names = ["arith"]
    code = re.sub(r'"(?:\\.|[^"\\])*"|;[^\n]*', " ", body)
    for tok in re.findall(r"[^\s()]+", code):
        if tok in arith_names() and tok not in names:
            names.append(tok)
    head = "(import std.arith :symbols ("
    lines = [head]
    for n in names:
        last = lines[-1]
        if last.endswith("(") or len(last) + 1 + len(n) + 2 <= 100:
            lines[-1] = last + ("" if last.endswith("(") else " ") + n
        else:
            lines.append(" " * len(head) + n)
    return "\n".join(lines) + "))\n"
NESTING_LIMIT = 1000  # `ein_ir::grammar::NESTING_LIMIT`: a `(` deeper than this is refused


def enc(v: int) -> str:
    """A natural number in `std.arith`'s spelling: `z`, or `(n PREFIX DIGIT)`."""
    s = "z"
    for ch in str(v) if v else "":
        s = f"(n {s} {ch})"
    return s


def wrapped(text: str, indent: int) -> str:
    """A long form on lines of at most 100 columns, broken at spaces."""
    pad = " " * indent
    return ("\n" + pad).join(textwrap.wrap(text, 100 - indent, break_long_words=False))


def answer(op: str, x: int, y: int) -> str:
    """The result fact's arguments after the operands, as the theory answers."""
    if op == "add":
        return enc(x + y)
    if op == "sub":
        return f"{enc(max(x - y, 0))} {enc(max(y - x, 0))}"
    if op == "cmp":
        return "lt" if x < y else "gt" if x > y else "eq"
    if op == "mul":
        return enc(x * y)
    return f"{enc(x // y)} {enc(x % y)}"


RESULT = {"add": "sum-of", "sub": "diff-of", "cmp": "cmp-of", "mul": "product-of", "div": "quot-of"}
SIGN = {"add": "+", "sub": "-", "cmp": "compared with", "mul": "x", "div": "divided by"}
VARS = {"add": "?s", "sub": "?d ?r", "cmp": "?o", "mul": "?p", "div": "?q ?r"}
TEXT = {"add": "= {?s}", "sub": "= {?d}, short by {?r}", "cmp": "is {?o}", "mul": "= {?p}",
        "div": "= {?q}, remainder {?r}"}


def claim(op: str, x: str, y: str, xv: int, yv: int, said: str) -> str:
    """A query that reads one answer, and the claim of what it is."""
    fact = f"({RESULT[op]} {x} {y} {answer(op, xv, yv)})"
    return (f"(query :goal ({RESULT[op]} {x} {y} {VARS[op]})\n"
            f"       :goal-text \"{said} {TEXT[op]}\"\n"
            f"       :expect (model {wrapped(fact, 22)}))\n")


def draw(rnd: random.Random, digits: int) -> int:
    """A number of exactly `digits` digits."""
    return rnd.randrange(10 ** (digits - 1), 10 ** digits)


def long_numbers(digits: int = DIGITS, short: int = SHORT, seed: int = SEED) -> str:
    rnd = random.Random(seed)
    a, b = draw(rnd, digits), draw(rnd, digits)
    c, d = draw(rnd, short), draw(rnd, short)
    e = draw(rnd, 2 * short)
    head = f"""\
;;; Long numbers: few requests, long operands. M1f P1f.6 (S1f.6.15).
;;;
;;; GENERATED by `utils/gen_arith_examples.py` — do not edit; `--check` is in
;;; the gate (`ein-cli/tests/arith_search_free.rs`). Seed {seed}.
;;;
;;; ── What this program is ───────────────────────────────────────────────
;;;
;;; One request an operation of `std.arith` (`add.ein` beside this file is
;;; the introduction), on numbers nobody would type: `a` and `b` have
;;; {digits} digits each, `c` and `d` have {short}, and `e` has {2 * short}.
;;;
;;;   (add a b)   (sub a b)   (cmp a b)      {digits} digits against {digits}
;;;   (mul c d)                              {short} digits by {short}
;;;   (div e d)                              {2 * short} digits by {short}
;;;
;;; The numbers are named with `let`, so a request and its answer repeat the
;;; names and not the digits.
;;;
;;; ── What it measures ───────────────────────────────────────────────────
;;;
;;; The user, 2026-11-11: "add examples with long numbers and many
;;; calculations so we can compare load time with calculation time".
;;;
;;;     ein saturate examples/arith/long_numbers.ein
;;;
;;; prints three times: `parse:`, the program's own text; `kb load:`, mostly
;;; the 1 664 rows of the tables in `std.digits`, the same for every program
;;; that imports the theory; and `saturate:`, the calculation, with its
;;; firings. `python3 utils/arith_cost.py` prints them for this file and for
;;; a sweep of sizes, and `stdlib/README.md` § `std.arith` has the table.
;;;
;;; A sum, a difference and a comparison cost a few firings a digit. A
;;; product costs the two lengths multiplied, and a quotient more: each of
;;; its digits is up to four trial subtractions of the divisor's length.
;;;
;;; The operands are this long and no longer because every corpus program is
;;; also solved, dumped and reloaded per commit, and a circuit's state, which
;;; shares its numbers in the store, shares nothing when it is written out.
;;; The generator takes the sizes: `--digits 300 --short 60` writes the
;;; program the stage's plan asked for.
"""
    let = lambda name, v: f"(let {name}\n  {wrapped(enc(v), 2)})\n"
    lets = (f";; {short} digits each, and {2 * short}\n" + let("c", c) + let("d", d) + let("e", e)
            + f"\n;; {digits} digits each\n" + let("a", a) + let("b", b))
    reqs = "(add a b)\n(sub a b)\n(cmp a b)\n(mul c d)\n(div e d)\n"
    claims = "\n".join((claim("add", "a", "b", a, b, "a + b"),
                        claim("sub", "a", "b", a, b, "a - b"),
                        claim("cmp", "a", "b", a, b, "a against b"),
                        claim("mul", "c", "d", c, d, "c x d"),
                        claim("div", "e", "d", e, d, "e divided by d")))
    body = f"\n{lets}\n{reqs}\n{claims}"
    return f"{head}\n{import_line(body)}{body}"


def requests(op: str, count: int, operand: int, rnd: random.Random) -> list[tuple[int, int]]:
    """`count` distinct operand pairs for one operation; a divisor is never zero."""
    seen: dict[tuple[int, int], None] = {}
    while len(seen) < count:
        x = rnd.randrange(0, 10 ** rnd.randint(1, operand))
        y = rnd.randrange(1 if op == "div" else 0, 10 ** rnd.randint(1, operand))
        seen[(x, y)] = None
    return list(seen)


def many_calculations(count: int = REQUESTS, operand: int = OPERAND, seed: int = SEED,
                      ops: tuple[str, ...] = OPS) -> str:
    rnd = random.Random(seed)
    batches = {op: requests(op, count, operand, rnd) for op in ops}
    head = f"""\
;;; Many calculations: many requests, short operands. M1f P1f.6 (S1f.6.15).
;;;
;;; GENERATED by `utils/gen_arith_examples.py` — do not edit; `--check` is in
;;; the gate (`ein-cli/tests/arith_search_free.rs`). Seed {seed}.
;;;
;;; ── What this program is ───────────────────────────────────────────────
;;;
;;; {count} requests for each of {len(ops)} operations of `std.arith`, {count * len(ops)} in all, on
;;; numbers of one to {operand} digits: a page of school exercises. The requests are
;;; written with their numbers, so the program's text is mostly numbers.
;;;
;;; ── What it measures ───────────────────────────────────────────────────
;;;
;;;     ein saturate examples/arith/many_calculations.ein
;;;
;;; prints `parse:`, `kb load:` and `saturate:` (see `long_numbers.ein`
;;; beside this file). Here the program's own text is what grows: each
;;; request is two written numbers, and a written number is one nested form a
;;; digit. A batch costs in proportion to its count, which is the rule of the
;;; theory, *no rule joins two facts on a number*, held by
;;; `ein-infer/tests/arith_batch.rs`.
;;;
;;; `python3 utils/arith_cost.py` runs the sweep to 10 000 requests an
;;; operation; this file is the size every surface of the gate affords
;;; (`--requests 2000` writes a larger one).
;;;
;;; The queries read the first request of each operation. A goal with the
;;; operands written claims that one answer and no other.
"""
    body = ""
    for op in ops:
        body += f"\n;; {count} requests: X {SIGN[op]} Y\n"
        body += "".join(f"({op} {enc(x)} {enc(y)})\n" for x, y in batches[op])
    claims = "\n".join(claim(op, enc(x), enc(y), x, y, f"{x} {SIGN[op]} {y}")
                       for op in ops for x, y in batches[op][:1])
    rest = f"{body}\n{claims}"
    return f"{head}\n{import_line(rest)}{rest}"


def factorial(last: int = FACTORIAL) -> str:
    value = math.factorial(last)
    digits = len(str(value))
    head = f"""\
;;; A long chain: {last}!, by the program's own rules. M1f P1f.6 (S1f.6.15).
;;;
;;; GENERATED by `utils/gen_arith_examples.py` — do not edit; `--check` is in
;;; the gate (`ein-cli/tests/arith_search_free.rs`).
;;;
;;; ── What this program is ───────────────────────────────────────────────
;;;
;;; It writes two numbers, 1 and {last}, and one fact, 1! = 1. Everything else
;;; is derived: three rules of the program's own ask `std.arith` for the
;;; next number, k + 1, for whether k is below the last, and for the product
;;; k! x (k + 1), and state the next factorial from the answers. A request is
;;; a fact, so a rule may derive one (`div.ein` beside this file).
;;;
;;;   (factorial K F)        K! = F
;;;
;;;   next-number            for each K! known: ask K + 1, and K against the last
;;;   next-product           where K is below the last: ask K! x (K + 1)
;;;   next-factorial         the product is (K + 1)!
;;;
;;; The chain stops by itself: at K = {last} the comparison answers `eq`, no
;;; product is asked, and nothing is left to fire. No negation is written.
;;; {last}! has {digits} digits.
;;;
;;; ── What it measures ───────────────────────────────────────────────────
;;;
;;;     ein saturate examples/arith/factorial.ein
;;;
;;; prints `parse:`, `kb load:` and `saturate:` (see `long_numbers.ein`
;;; beside this file). Here the text is a few lines whatever the last number
;;; is, so the parse is small and the calculation is the whole of the
;;; program's own cost: the other end from `many_calculations.ein`.
;;; `python3 utils/arith_cost.py` runs the chain to 200!, a number of 375
;;; digits (`--factorial 120` writes a longer one).
;;;
;;; ── The program's own joins ────────────────────────────────────────────
;;;
;;; `next-product` and `next-factorial` each join two or three facts on a
;;; number, which `std.arith` itself never does: the engine's index does not
;;; key on a nested fact, so each such premise looks through its relation
;;; (E-3 of P1f.6's `findings.md`). Here that is {last} facts a relation, and
;;; it does not show; a program with thousands of steps would carry its
;;; state in one fact, as the theory's circuits do.
;;;
;;; `factorial` is the program's own relation, and the program closes it:
;;; the theory closes its own relations and cannot close the importer's.
"""
    body = f"\n{factorial_body(last)}"
    return f"{head}\n{import_line(body)}{body}"


def factorial_body(last: int, expect: bool = True) -> str:
    """The chain's relation, its three rules and the query; the claim where `expect`."""
    value = math.factorial(last)
    said = (f"\n       :expect (model {wrapped(f'(factorial {enc(last)} {enc(value)})', 22)}))\n"
            if expect else ")\n")
    return f"""\
(relation factorial Number Number
  :why "{{?1}}! = {{?2}}")
(__closed__ factorial)

(let last {enc(last)})

(factorial (n z 1) (n z 1))

(rule next-number ()
  :match  (factorial ?k ?f)
  :assert (and (add ?k (n z 1)) (cmp ?k last))
  :why    "{{?k}}! is known: ask for the next number, and whether {{?k}} is the last")

(rule next-product ()
  :match  (and (cmp-of ?k last lt) (sum-of ?k (n z 1) ?next) (factorial ?k ?f))
  :assert (mul ?f ?next)
  :why    "{{?k}} is below the last: ask for {{?k}}! x {{?next}}")

(rule next-factorial ()
  :match  (and (product-of ?f ?next ?p) (factorial ?k ?f) (sum-of ?k (n z 1) ?next))
  :assert (factorial ?next ?p)
  :why    "{{?k}}! x {{?next}} is {{?next}}!")

(query :goal (factorial {enc(last)} ?f)
       :goal-text "{last}! = {{?f}}"{said}"""


# ── One program of a stated size, on demand (S1f.6.29) ─────────────────────

SHAPES = (*OPS, "mix", "factorial")
WHAT = {"add": "sum", "sub": "difference", "cmp": "comparison", "mul": "product", "div": "quotient"}


def nesting(text: str) -> int:
    """The deepest `(` of a program's text, outside strings and comments, as the parser counts it."""
    deepest = depth = 0
    for ch in re.sub(r"[^()]", "", re.sub(r'"[^"]*"|;[^\n]*', "", text)):
        depth += 1 if ch == "(" else -1
        deepest = max(deepest, depth)
    return deepest


def command(op: str, digits: int | None, dividend: int | None, count: int, n: int | None,
            seed: int, expect: bool) -> str:
    """The arguments that name the program, in one order, and no path."""
    words = ["python3 utils/gen_arith_examples.py", "--op", op]
    if op == "factorial":
        words += ["--n", str(n)]
    else:
        words += ["--digits", str(digits)]
        if op in ("div", "mix"):
            words += ["--dividend", str(dividend)]
        words += ["--count", str(count), "--seed", str(seed)]
    if not expect:
        words.append("--no-expect")
    return " ".join(words + ["-o", "FILE"])


def distinct(op: str, count: int, digits: int, dividend: int, rnd: random.Random) -> list[tuple[int, int]]:
    """`count` distinct operand pairs of exactly the stated digits; a divisor has at least one."""
    first = dividend if op == "div" else digits
    room = 9 * 10 ** (first - 1) * 9 * 10 ** (digits - 1)
    if count > room:
        raise SystemExit(f"{count} requests of {op} on {first} and {digits} digits: only {room} pairs exist")
    seen: dict[tuple[int, int], None] = {}
    while len(seen) < count:
        seen[(draw(rnd, first), draw(rnd, digits))] = None
    return list(seen)


def operands(op: str, digits: int, dividend: int, count: int, seed: int
             ) -> tuple[dict[str, int], dict[str, list[tuple[int, int]]]]:
    """Requests an operation, and the operand pairs of each, drawn from the seed in one order."""
    ops = OPS if op == "mix" else (op,)
    rnd = random.Random(seed)
    per = {o: count // len(ops) + (1 if i < count % len(ops) else 0) for i, o in enumerate(ops)}
    return per, {o: distinct(o, per[o], digits, dividend, rnd) for o in ops if per[o]}


def on_demand(op: str, digits: int | None, dividend: int | None, count: int, n: int | None,
              seed: int, expect: bool) -> str:
    """One program of a stated size: the arguments and the seed name it, whatever the path."""
    cmd = command(op, digits, dividend, count, n, seed, expect)
    if op == "factorial":
        value = math.factorial(n)
        shape = (f"The factorial chain to {n}!, a number of {len(str(value))} digits, by the program's own three "
                 "rules, as in `examples/arith/factorial.ein`, whose header explains them: two numbers written, 1 "
                 "and the last, and the fact 1! = 1; everything else derived.")
        claims = [(factorial_body(n, True), factorial_body(n, False))]
        title = f"The factorial chain to {n}!"
    else:
        ops = OPS if op == "mix" else (op,)
        per, batches = operands(op, digits, dividend, count, seed)
        sizes = (f"numbers of exactly {digits} digits"
                 + (f", a dividend of {dividend}" if "div" in batches else ""))
        if count == 1:
            (x, y), = batches[op]
            title = f"One {WHAT[op]}, {dividend if op == 'div' else digits} digits by {digits}"
            shape = (f"One request, `({op} a b)`, on {sizes}, named with `let`, so the request, its answer and "
                     "the claim repeat the names and not the digits.")
            let = lambda name, v: f"(let {name}\n  {wrapped(enc(v), 2)})\n"
            requests_text = let("a", x) + let("b", y) + f"\n({op} a b)\n"
            said = {"add": "a + b", "sub": "a - b", "cmp": "a against b", "mul": "a x b",
                    "div": "a divided by b"}[op]
            claims = [(claim(op, "a", "b", x, y, said), unclaimed(op, "a", "b", said))]
        else:
            title = f"{count} requests" + (", the five operations in turn" if op == "mix" else f" of {op}")
            shape = (f"{count} requests"
                     + (f", the five operations of `std.arith` in turn ({', '.join(f'{per[o]} of {o}' for o in ops)})"
                        if op == "mix" else f" of `{op}`")
                     + f", on {sizes}, each written with its numbers. "
                     + ("The queries read the first request of each operation." if op == "mix"
                        else "The query reads the first request."))
            requests_text = "".join(f"({o} {enc(x)} {enc(y)})\n" for i in range(max(per.values()))
                                    for o in ops if i < len(batches.get(o, ())) for x, y in [batches[o][i]])
            claims = [(claim(o, enc(x), enc(y), x, y, f"{x} {SIGN[o]} {y}"),
                       unclaimed(o, enc(x), enc(y), f"{x} {SIGN[o]} {y}"))
                      for o in ops if o in batches for x, y in batches[o][:1]]
    # A claim is written where it nests within the parser's limit, and where it is asked for.
    kept, past = [], 0
    for stated, bare in claims:
        if expect and nesting(stated) <= NESTING_LIMIT:
            kept.append(stated)
        else:
            past += expect
            kept.append(bare)
    if not expect:
        note = ";;; Every `:expect` is left out (`--no-expect`); the queries print the answers.\n"
    elif past:
        note = prose(("The claim is" if len(claims) == 1 else f"{past} of {len(claims)} claims are")
                     + " left out: the answer would nest past the parser's limit "
                     f"of {NESTING_LIMIT} parentheses (`ein_ir::grammar::NESTING_LIMIT`), so the query prints it "
                     "and nothing holds it."
                     + (" Every other `:expect` is Python's answer, so `ein test FILE` holds the engine to it."
                        if past < len(claims) else ""))
    else:
        note = ";;; Every `:expect` is Python's answer, so `ein test FILE` holds the engine to it.\n"
    head = f""";;; {title}. M1f P1f.6 (S1f.6.29).
;;;
;;; GENERATED on demand by `utils/gen_arith_examples.py`, with the arguments
;;;
;;;     {cmd}
;;;
;;; which name this program, whatever the path. **Not checked in, and run by
;;; no test, gate step, nightly step or dev-host step**: the user, 2026-11-12,
;;; "do not call it in tests and in CI, just for manual solve call".
;;; `stdlib/README.md` § The biggest calculation has the sizes that fit in
;;; about 16 GB, with the commands and the figures.
;;;
;;; ── What this program is ───────────────────────────────────────────────
;;;
{prose(shape)};;;
;;;     ein saturate FILE    the calculation: parse, load, saturate, firings
;;;     ein solve FILE       the answer: the state, and each query read
;;;
{note}"""
    body = (f"\n{kept[0]}" if op == "factorial"
            else f"\n{requests_text}\n" + "\n".join(kept))
    text = f"{head}\n{import_line(body)}{body}"
    if nesting(text) > NESTING_LIMIT:
        raise SystemExit(f"{cmd}: the operands nest {nesting(text)} deep, past the parser's limit of "
                         f"{NESTING_LIMIT} parentheses (`ein_ir::grammar::NESTING_LIMIT`): a written number is "
                         "a level a digit, so an operand has at most 999")
    return text


def prose(text: str) -> str:
    """A paragraph of a header, on comment lines of at most 78 columns."""
    return "".join(f";;; {line}\n" for line in textwrap.wrap(text, 74, break_long_words=False))


def unclaimed(op: str, x: str, y: str, said: str) -> str:
    """The query of `claim`, its answer printed and not claimed."""
    return (f"(query :goal ({RESULT[op]} {x} {y} {VARS[op]})\n"
            f"       :goal-text \"{said} {TEXT[op]}\")\n")


def files(args: argparse.Namespace) -> dict[str, str]:
    return {
        "long_numbers.ein": long_numbers(args.digits, args.short, args.seed),
        "many_calculations.ein": many_calculations(args.requests, args.operand, args.seed),
        "factorial.ein": factorial(args.factorial),
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--check", action="store_true", help="exit 1 if a shipped file differs")
    ap.add_argument("--out", type=Path, help="the directory the three files are written to")
    ap.add_argument("--seed", type=int, default=SEED)
    ap.add_argument("--digits", type=int, help="the three: a and b of long_numbers.ein; --op: each operand")
    ap.add_argument("--short", type=int, help="the three: c, d, and half of e")
    ap.add_argument("--requests", type=int, help="the three: requests an operation")
    ap.add_argument("--operand", type=int, help="the three: an operand's most digits")
    ap.add_argument("--factorial", type=int, help="the three: the chain's last number")
    one = ap.add_argument_group("one program of a stated size, on demand (S1f.6.29)")
    one.add_argument("--op", choices=SHAPES, help="the shape: one operation, the five in turn, or the chain")
    one.add_argument("-o", "--output", type=Path, help="the file to write")
    one.add_argument("--dividend", type=int, help="a division's dividend in digits (default twice --digits)")
    one.add_argument("--count", type=int, help="requests in all (default 1)")
    one.add_argument("--n", type=int, help="the factorial chain's last number")
    one.add_argument("--no-expect", action="store_true", help="leave every :expect out")
    args = ap.parse_args()
    if args.op:
        three = [f for f in ("check", "out", "short", "requests", "operand", "factorial") if getattr(args, f)]
        if three:
            ap.error(f"--op writes one program; --{three[0]} is for the three shipped files")
        if args.output is None:
            ap.error("--op needs -o FILE")
        if args.op == "factorial":
            if args.n is None or args.n < 2 or args.digits or args.dividend or args.count:
                ap.error("--op factorial takes --n N, N at least 2, and no --digits, --dividend or --count")
        else:
            if args.n is not None:
                ap.error("--n is the factorial chain's")
            if args.digits is None or args.digits < 1:
                ap.error(f"--op {args.op} needs --digits N, N at least 1")
            if args.dividend is not None and args.op not in ("div", "mix"):
                ap.error("--dividend is a division's")
            args.count = 1 if args.count is None else args.count
            if args.count < 1 or (args.op == "mix" and args.count < len(OPS)):
                ap.error(f"--count is at least {len(OPS) if args.op == 'mix' else 1}")
            args.dividend = 2 * args.digits if args.dividend is None else args.dividend
            if args.dividend < 1:
                ap.error("--dividend is at least 1")
        text = on_demand(args.op, args.digits, args.dividend, args.count, args.n, args.seed, not args.no_expect)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(text)
        print(f"wrote {args.output}, {len(text.encode())} bytes, nested {nesting(text)} deep")
        return 0
    for f in ("output", "dividend", "count", "n"):
        if getattr(args, f) is not None:
            ap.error(f"--{f} is for --op")
    if args.no_expect:
        ap.error("--no-expect is for --op")
    args.out = OUT if args.out is None else args.out
    for f, default in (("digits", DIGITS), ("short", SHORT), ("requests", REQUESTS), ("operand", OPERAND),
                       ("factorial", FACTORIAL)):
        if getattr(args, f) is None:
            setattr(args, f, default)
    bad = 0
    for name, text in files(args).items():
        path = args.out / name
        if args.check:
            if not path.exists() or path.read_text() != text:
                print(f"{path}: differs from `python3 utils/gen_arith_examples.py`", file=sys.stderr)
                bad = 1
        else:
            args.out.mkdir(parents=True, exist_ok=True)
            path.write_text(text)
            print(f"wrote {path}")
    return bad


if __name__ == "__main__":
    sys.exit(main())
