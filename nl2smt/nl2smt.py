#!/usr/bin/env python3
"""nl2smt.py: the Zebra puzzle's sentences, through a parser and a Racket program, to an SMT solver.

    python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra.txt

    sentences, a line each  ->  a parser  ->  its converter  ->  SMT-LIB code  ->  a solver  ->  the solution

The parser is NLTK's chart parser over english.cfg, whose trees nltk2smt.rkt reads, or link
grammar's link-parser, whose linkages lg2smt.rkt reads. This script only runs the others. It
reads the lines, refuses a word that is not in vocabulary.rktd, hands each sentence to the
parser and the parser's output to its converter, hands that program's SMT code to a solver,
and prints what the solver found and how long each of the three took. It reads no English
and writes no SMT: the first is the parser's work and the converter's, the second the
converter's.
"""
import argparse
import contextlib
import importlib.util
import itertools
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
VOCABULARY = HERE / "vocabulary.rktd"
GRAMMAR = HERE / "english.cfg"  # the grammar NLTK's parser is given

PARSERS = {  # each with the Racket program that reads what it gives; the first one installed is used
    "nltk": HERE / "nltk2smt.rkt",
    "link-grammar": HERE / "lg2smt.rkt",
}
PARSER = None  # the parser parse() takes when it is given none; None is the first one installed

# link-parser prints a sentence's cheapest linkage, and the next one for every empty line after it.
LINK_PARSER = ["link-parser", "en", "-postscript=1", "-graphics=0", "-echo=1"]
LINKAGES = 100  # how many of a sentence's linkages or trees, in the parser's order, the converter is shown

SOLVERS = {  # each reads the script on stdin; the first one installed is used
    "cvc5": ["cvc5", "--lang=smt2"],
    "z3": ["z3", "-smt2", "-in"],
    "cvc4": ["cvc4", "--lang=smt2"],
}

SPENT = {}  # "parser", "converter", "solver": how many times each has run, and the seconds together


class Refused(Exception):
    """A sentence the pipeline does not take, said with its line."""


@contextlib.contextmanager
def timed(step):
    """Counts what runs inside as one run of the step, and adds its time."""
    start = time.perf_counter()
    try:
        yield
    finally:
        spent = SPENT.setdefault(step, [0, 0.0])
        spent[0] += 1
        spent[1] += time.perf_counter() - start


def timings(tools):
    """A line for each step that has run: the step, the tool that did it, and the time it took."""
    return ["%s: %s · %.3f s%s" % (step, tools[step], seconds, f" in {runs} runs" if runs > 1 else "")
            for step, (runs, seconds) in SPENT.items()]


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


def installed(parser):
    """Whether the parser can be run here: the nltk module for the one, the link-parser program for the other."""
    return bool(importlib.util.find_spec("nltk") if parser == "nltk" else shutil.which(LINK_PARSER[0]))


def parse(lines, parser=None):
    """What the parser gives for the sentences: a first line "parser: NAME VERSION", and then each
    sentence's block under a line "line N: the sentence"."""
    parser = parser or PARSER or next(filter(installed, PARSERS))
    with timed("parser"):
        return (trees if parser == "nltk" else linkages)(lines)


def trees(lines):
    """NLTK's trees of each sentence, as NLTK prints a tree: its Earley chart parser over english.cfg."""
    try:
        import nltk  # here and not above: under the other parser the script runs without it
    except ImportError as missing:  # nltk is there and something it imports is not
        sys.exit(f"nl2smt: nltk does not import: {missing}")

    grammar = nltk.CFG.fromstring(GRAMMAR.read_text())
    spelt = {word.lower(): word for rule in grammar.productions() for word in rule.rhs() if isinstance(word, str)}
    parser = nltk.parse.EarleyChartParser(grammar)
    blocks = [f"parser: nltk {nltk.__version__}\n"]
    for number, sentence in lines:
        words = sentence.rstrip(".?").split()
        for word in words:
            if word.lower() not in spelt:
                raise Refused(f'line {number}: the word "{word}" is in {VOCABULARY.name} and in no rule of {GRAMMAR.name}')
        chart = parser.chart_parse([spelt[word.lower()] for word in words])
        found = list(itertools.islice(chart.parses(grammar.start()), LINKAGES))
        blocks.append(f"line {number}: {sentence}\n")
        blocks += [f"tree {rank} of {len(found)}:\n{tree.pformat(margin=100)}\n" for rank, tree in enumerate(found, start=1)]
        if not found:  # an edge of an Earley chart ends only where the words before it can begin a sentence
            read = max((edge.end() for edge in chart.edges() if not isinstance(edge, nltk.parse.chart.LeafEdge)), default=0)
            blocks.append("no tree: " + (f'the parser stops at "{words[read]}", word {read + 1}' if read < len(words)
                                         else "the parser wants more words after the last") + "\n")
    return "".join(blocks)


def linkages(lines):
    """link-parser's linkages of each sentence, as it prints them."""
    asked = "".join(sentence + "\n" * LINKAGES for _, sentence in lines)
    printed = subprocess.run(LINK_PARSER, input=asked, capture_output=True, text=True, check=True).stdout
    version = subprocess.run([LINK_PARSER[0], "--version"], capture_output=True, text=True)
    version = re.search(r"link-grammar-(\S+)", version.stdout + version.stderr)
    blocks = ["parser: link-grammar %s\n" % (version.group(1) if version else "of no version it names")]
    for line in printed.splitlines():
        if len(blocks) <= len(lines) and line == lines[len(blocks) - 1][1]:  # the sentence, echoed
            blocks.append("line %d: %s\n" % lines[len(blocks) - 1])
        elif len(blocks) > 1:
            blocks[-1] += line + "\n"
    return "".join(blocks)


def convert(parses, models=(), readings=False):
    """The SMT code the parser's converter writes for the parses, with the models given ruled out; or
    its refusal. The parses' first line names the parser, and so the converter."""
    converter = PARSERS[parses.split(maxsplit=2)[1]]
    command = ["racket", str(converter), *(["--readings"] if readings else []), str(VOCABULARY), *map(written, models)]
    with timed("converter"):
        run = subprocess.run(command, input=parses, capture_output=True, text=True)
    if run.returncode:
        raise Refused(run.stderr.strip())
    return run.stdout


def solve(solver, smt):
    """None where the script is unsatisfiable; else the model and, by line, each question's answers."""
    with timed("solver"):
        printed = subprocess.run(SOLVERS[solver], input=smt, capture_output=True, text=True).stdout
    said = read_all(printed)
    if said[:1] == ["unsat"]:
        return None
    if said[:1] != ["sat"]:
        sys.exit(f"{solver} said neither sat nor unsat: {written(said)}")
    rest = said[1:]
    model = rest.pop(0) if rest and isinstance(rest[0], list) else []  # no model where the sentences name nothing
    answers = {}
    while rest:  # question LINE CONSTANT ..., and where there is a constant ((TERM true-or-false) ...), in their order
        end = next((i for i, item in enumerate(rest) if i and (isinstance(item, list) or item == "question")), len(rest))
        values = rest[end] if end < len(rest) and isinstance(rest[end], list) else []
        answers[int(rest[1])] = [c for c, (_, value) in zip(rest[2:end], values) if value == "true"]
        rest = rest[end + bool(values):]
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
    """The model as a table, a column for each house that holds something and a row for each kind of
    thing named, a cell blank where the lines name nothing of that kind there. rich draws it where it
    is installed; without it the table is plain columns."""
    where = {constant: int(house) for constant, house in model}
    if not where:
        return "  the sentences name nothing that is in a house"
    houses = sorted(set(where.values()))
    rows = {}
    for constant, house in where.items():
        tag, word = named(constant)
        rows.setdefault(tag, [""] * len(houses))[houses.index(house)] = word
    try:
        from rich import box
        from rich.console import Console
        from rich.table import Table
    except ImportError:
        rows = {"house": [str(house) for house in houses], **rows}
        widths = [max(len(row[i]) for row in rows.values()) for i in range(len(houses))]
        return "\n".join("  %-8s" % tag + "  ".join(cell.ljust(width) for cell, width in zip(row, widths)).rstrip()
                         for tag, row in rows.items())
    drawn = Table(box=box.ROUNDED, header_style="bold")
    drawn.add_column("house", style="bold")
    for house in houses:
        drawn.add_column(str(house))
    for tag, row in rows.items():
        drawn.add_row(tag, *row)
    console = Console() if sys.stdout.isatty() else Console(width=10_000)  # into a file or a pipe: no wrapping
    with console.capture() as lines:
        console.print(drawn)
    return "\n".join("  " + line.rstrip() for line in lines.get().splitlines())


def report(lines, found, complete, limit):
    """What is printed under the three steps: how many solutions, each as a table, each question answered."""
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
        out += ["  %s  %s" % (text[line].ljust(width), ", ".join(named(c)[1] for c in who) or "nobody the sentences name")
                for line, who in sorted(answers.items())]
    return "\n".join(out)


def main():
    options = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    options.add_argument("file", help="the sentences, a line each; - for stdin")
    options.add_argument("--parser", choices=PARSERS, help="default: the first installed of " + ", ".join(PARSERS))
    options.add_argument("--solver", choices=SOLVERS, help="default: the first installed of " + ", ".join(SOLVERS))
    options.add_argument("--solutions", type=int, default=2, metavar="N",
                         help="look for up to N solutions (default 2: enough to say whether there is one only)")
    options.add_argument("--show", choices=["parse", "readings", "smt"],
                         help="print one stage's output and stop: what the parser gives, what its converter "
                              "reads each linkage or tree as, or the SMT code")
    args = options.parse_args()
    if args.solutions < 1:
        options.error("--solutions wants 1 or more")

    parser = args.parser or next(filter(installed, PARSERS), None)
    solver = args.solver or next(filter(shutil.which, SOLVERS), None)
    for tool, there, any_of in [(parser, parser and installed(parser), "a parser (%s)" % ", ".join(PARSERS)),
                                ("racket", shutil.which("racket"), None),
                                (solver, args.show or solver and shutil.which(solver),
                                 "an SMT solver (%s)" % ", ".join(SOLVERS))]:
        if not there:
            sys.exit(f"nl2smt: {tool or any_of} is not installed")
    tools = {"converter": PARSERS[parser].name}
    try:
        lines = sentences(sys.stdin.read() if args.file == "-" else Path(args.file).read_text())
        parses = parse(lines, parser)
        tools["parser"] = parses.splitlines()[0].partition(": ")[2]
        if args.show:
            print({"parse": lambda: parses, "readings": lambda: convert(parses, readings=True),
                   "smt": lambda: convert(parses)}[args.show](), end="")
            print("\n".join(timings(tools)), file=sys.stderr)
        else:
            found, complete = solutions(parses, solver, args.solutions)
            version = subprocess.run([solver, "--version"], capture_output=True, text=True).stdout.splitlines()[0]
            tools["solver"] = f"{solver} ({version})"
            print("\n".join(timings(tools)))
            print(report(lines, found, complete, args.solutions))
    except Refused as refusal:
        sys.exit(f"nl2smt: {refusal}")


if __name__ == "__main__":
    main()
