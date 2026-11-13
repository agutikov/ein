#!/usr/bin/env python3
"""nl2smt.py: the Zebra puzzle's sentences, through link grammar and a Racket program, to an SMT solver.

    python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra.txt

    sentences, a line each  ->  link-parser  ->  lg2smt.rkt  ->  SMT-LIB code  ->  a solver  ->  the solution

This script only runs the others. It reads the lines, refuses a word that is not in
vocabulary.rktd, hands each sentence to link-parser and the parser's output to lg2smt.rkt,
hands that program's SMT code to a solver, and prints what the solver found. It reads no
English and writes no SMT: both are lg2smt.rkt's work.
"""
import argparse
import re
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
VOCABULARY = HERE / "vocabulary.rktd"
CONVERTER = HERE / "lg2smt.rkt"

# link-parser prints a sentence's cheapest linkage, and the next one for every empty line after it.
PARSER = ["link-parser", "en", "-postscript=1", "-graphics=0", "-echo=1"]
LINKAGES = 100  # how many of a sentence's linkages, the cheapest first, the converter is shown

SOLVERS = {  # each reads the script on stdin; the first one installed is used
    "cvc5": ["cvc5", "--lang=smt2"],
    "z3": ["z3", "-smt2", "-in"],
    "cvc4": ["cvc4", "--lang=smt2"],
}


class Refused(Exception):
    """A sentence the pipeline does not take, said with its line."""


def read_all(text):
    """Every S-expression in the text, as nested lists of strings."""
    stack = [[]]
    for token in re.findall(r'[()]|[^\s()"]+', re.sub(r";.*", "", text)):
        if token == "(":
            stack.append([])
        elif token == ")":
            stack[-2].append(stack.pop())
        else:
            stack[-1].append(token)
    return stack[0]


def written(sexp):
    return sexp if isinstance(sexp, str) else "(" + " ".join(map(written, sexp)) + ")"


def vocabulary():
    """The words of vocabulary.rktd, in lower case: all but the first two atoms of every form."""
    def atoms(sexp):
        return [sexp] if isinstance(sexp, str) else [atom for part in sexp for atom in atoms(part)]
    return {atom.lower() for form in read_all(VOCABULARY.read_text()) for atom in atoms(form[2:])}


def sentences(text):
    """(line number, sentence) for every line that is not blank and does not start with #."""
    known = vocabulary()
    found = []
    for number, line in enumerate(text.splitlines(), start=1):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        for word in line.rstrip(".?").split():
            if word.lower() not in known:
                raise Refused(f'line {number}: the word "{word}" is not in the vocabulary ({VOCABULARY.name})')
        found.append((number, line))
    return found


def parse(lines):
    """What link-parser prints for the sentences, each block under a line "line N: the sentence"."""
    asked = "".join(sentence + "\n" * LINKAGES for _, sentence in lines)
    printed = subprocess.run(PARSER, input=asked, capture_output=True, text=True, check=True).stdout
    blocks = []
    for line in printed.splitlines():
        if len(blocks) < len(lines) and line == lines[len(blocks)][1]:  # the sentence, echoed
            blocks.append("line %d: %s\n" % lines[len(blocks)])
        elif blocks:
            blocks[-1] += line + "\n"
    return "".join(blocks)


def convert(parses, models=(), readings=False):
    """lg2smt.rkt's SMT code for the parses, with the models given ruled out; or its refusal."""
    command = ["racket", str(CONVERTER), *(["--readings"] if readings else []), str(VOCABULARY), *map(written, models)]
    run = subprocess.run(command, input=parses, capture_output=True, text=True)
    if run.returncode:
        raise Refused(run.stderr.strip())
    return run.stdout


def solve(solver, smt):
    """None where the script is unsatisfiable; else the model and, by line, each question's answers."""
    said = read_all(subprocess.run(SOLVERS[solver], input=smt, capture_output=True, text=True).stdout)
    if said[:1] == ["unsat"]:
        return None
    if said[:1] != ["sat"]:
        sys.exit(f"{solver} said neither sat nor unsat: {written(said)}")
    model, answers, rest = said[1], {}, said[2:]
    while rest:  # question LINE CONSTANT ... then ((TERM true-or-false) ...), the terms in the constants' order
        end = next(i for i, item in enumerate(rest) if isinstance(item, list))
        answers[int(rest[1])] = [c for c, (_, value) in zip(rest[2:end], rest[end]) if value == "true"]
        rest = rest[end + 1:]
    return model, answers


def solutions(parses, solver, limit):
    """Up to `limit` solutions, and whether those are all there are."""
    found = []
    while True:
        solution = solve(solver, convert(parses, [model for model, _ in found]))
        if solution is None:
            return found, True
        if len(found) == limit:
            return found, False
        found.append(solution)


def named(constant):
    """nation-Englishman -> ("nation", "Englishman"): a constant is its vocabulary tag and its word."""
    tag, _, word = constant.partition("-")
    return tag, word.replace("-", " ")


def table(model):
    """The model as rows: the houses' numbers, then a row for each tag."""
    where = {constant: int(house) for constant, house in model}
    rows = {"house": [str(house) for house in range(1, max(where.values()) + 1)]}
    for constant, house in where.items():
        tag, word = named(constant)
        rows.setdefault(tag, [""] * len(rows["house"]))[house - 1] = word
    widths = [max(len(row[i]) for row in rows.values()) for i in range(len(rows["house"]))]
    return "\n".join("  %-8s" % tag + "  ".join(cell.ljust(width) for cell, width in zip(row, widths)).rstrip()
                     for tag, row in rows.items())


def report(lines, found, complete, limit):
    """What is printed under the solver's name: how many solutions, each as a table, each question answered."""
    if not found:
        return "no solution: the sentences cannot all be true"
    if complete:
        out = [("1 solution" if len(found) == 1 else f"{len(found)} solutions") + ", and no other"]
    else:
        out = [f"more than {limit} solutions; the first {limit} (--solutions N looks for more)"]
    text = dict(lines)
    for model, answers in found:
        out += ["", table(model)]
        width = max((len(text[line]) for line in answers), default=0)
        out += ["  %s  %s" % (text[line].ljust(width), ", ".join(named(c)[1] for c in who) or "nobody")
                for line, who in sorted(answers.items())]
    return "\n".join(out)


def main():
    options = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    options.add_argument("file", help="the sentences, a line each; - for stdin")
    options.add_argument("--solver", choices=SOLVERS, help="default: the first installed of " + ", ".join(SOLVERS))
    options.add_argument("--solutions", type=int, default=2, metavar="N",
                         help="look for up to N solutions (default 2: enough to say whether there is one only)")
    options.add_argument("--show", choices=["parse", "readings", "smt"],
                         help="print one stage's output and stop: link-parser's linkages, what lg2smt.rkt "
                              "reads each linkage as, or the SMT code")
    args = options.parse_args()
    if args.solutions < 1:
        options.error("--solutions wants 1 or more")

    solver = args.solver or next((name for name in SOLVERS if shutil.which(name)), None)
    for tool in ["link-parser", "racket"] + ([solver] if args.show is None else []):
        if tool is None or not shutil.which(tool):
            sys.exit(f"nl2smt: {tool or 'an SMT solver (' + ', '.join(SOLVERS) + ')'} is not installed")
    try:
        lines = sentences(sys.stdin.read() if args.file == "-" else Path(args.file).read_text())
        parses = parse(lines)
        if args.show == "parse":
            print(parses, end="")
        elif args.show == "readings":
            print(convert(parses, readings=True), end="")
        elif args.show == "smt":
            print(convert(parses), end="")
        else:
            found, complete = solutions(parses, solver, args.solutions)
            version = subprocess.run([solver, "--version"], capture_output=True, text=True).stdout.splitlines()[0]
            print(f"solver: {solver} ({version})")
            print(report(lines, found, complete, args.solutions))
    except Refused as refusal:
        sys.exit(f"nl2smt: {refusal}")


if __name__ == "__main__":
    main()
