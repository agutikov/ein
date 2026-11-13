#!/usr/bin/env python3
"""test_pipeline.py: M1j's acceptance, each behaviour on an input file under puzzles/.

    python3 nl2smt/test_pipeline.py

The sentences can be added to, removed, reordered and swapped, and the SMT code and the
solution follow: that is what shows the pipeline is no table of the puzzle's sentences.
The tests are skipped, not failed, where link-parser, racket or every solver is missing.
"""
import functools
import shutil
import subprocess
import unittest

import nl2smt

PUZZLES = nl2smt.HERE / "puzzles"
SOLVER = next((name for name in nl2smt.SOLVERS if shutil.which(name)), None)
MISSING = [tool for tool in ("link-parser", "racket") if not shutil.which(tool)]
MISSING += [] if SOLVER else ["an SMT solver (%s)" % ", ".join(nl2smt.SOLVERS)]

# The names the two hand-written encodings give the constants, and the pipeline's.
HAND = nl2smt.HERE / "smt"
HAND_NAMES = {
    "house-red": "colour-red", "house-green": "colour-green", "house-white": "colour-ivory",
    "house-yellow": "colour-yellow", "house-blue": "colour-blue",
    "nation-england": "nation-Englishman", "nation-spain": "nation-Spaniard", "nation-ukrain": "nation-Ukrainian",
    "nation-norway": "nation-Norwegian", "nation-japan": "nation-Japanese",
    "pet-dog": "pet-dog", "pet-snail": "pet-snails", "pet-fox": "pet-fox", "pet-horse": "pet-horse",
    "pet-zebra": "pet-zebra",
    "drink-coffee": "drink-coffee", "drink-tee": "drink-tea", "drink-milk": "drink-milk",
    "drink-juice": "drink-orange-juice", "drink-water": "drink-water",
    "smoke-kool": "smoke-Kools", "smoke-chesterfield": "smoke-Chesterfields", "smoke-oldgold": "smoke-Old-Gold",
    "smoke-luckystrike": "smoke-Lucky-Strike", "smoke-parlament": "smoke-Parliaments",
}

# The puzzle's one solution, as every statement of it has it: the constants of each house.
ZEBRA = {
    1: {"nation-Norwegian", "colour-yellow", "pet-fox", "drink-water", "smoke-Kools"},
    2: {"nation-Ukrainian", "colour-blue", "pet-horse", "drink-tea", "smoke-Chesterfields"},
    3: {"nation-Englishman", "colour-red", "pet-snails", "drink-milk", "smoke-Old-Gold"},
    4: {"nation-Spaniard", "colour-ivory", "pet-dog", "drink-orange-juice", "smoke-Lucky-Strike"},
    5: {"nation-Japanese", "colour-green", "pet-zebra", "drink-coffee", "smoke-Parliaments"},
}
ZEBRA = frozenset((constant, house) for house, constants in ZEBRA.items() for constant in constants)


def lines_of(name):
    return (PUZZLES / name).read_text().splitlines()


@functools.cache
def parsed(name):
    """The file's sentences with their line numbers, and link-parser's output for them."""
    lines = nl2smt.sentences((PUZZLES / name).read_text())
    return lines, nl2smt.parse(lines)


def smt_of(name):
    return nl2smt.convert(parsed(name)[1])


def solved(name, limit=2, solver=None):
    """The file's solutions, each a set of (constant, house), their answers, and whether those are all."""
    found, complete = nl2smt.solutions(parsed(name)[1], solver or SOLVER, limit)
    return [as_set(model) for model, _ in found], [answers for _, answers in found], complete


def as_set(model, names=None):
    return frozenset(((names or {}).get(constant, constant), int(house)) for constant, house in model)


def facts(smt):
    """The sentences' part of a script the pipeline wrote: (sentence, assertion) in the script's order."""
    rows = smt.splitlines()
    return [(row.split(": ", 1)[1], rows[i + 1]) for i, row in enumerate(rows)
            if row.startswith("; line ") and rows[i + 1].startswith("(assert ")]


def clues(smt, names=None):
    """The assertions that state a clue, as a set: without the ranges and the distinctness, under the
    pipeline's names, and with the two sides of = and near in no order."""
    stated = set()
    for form in nl2smt.read_all(smt):
        if form[0] == "assert" and form[1][0] in ("=", "near", "order", "<"):
            relation, *sides = [(names or {}).get(atom, atom) for atom in form[1]]
            stated.add((relation, frozenset(sides) if relation in ("=", "near") else tuple(sides)))
    return stated


def hand_solutions(text):
    """Every solution of a hand-written encoding, under the pipeline's names. The blocking clauses
    are this test's own: the pipeline's are lg2smt.rkt's."""
    script = "(set-option :produce-models true)\n" + text.split("(check-sat)")[0]
    found = []
    while True:
        asked = script + "(check-sat)\n(get-value (%s))\n" % " ".join(HAND_NAMES)
        said = nl2smt.read_all(subprocess.run(nl2smt.SOLVERS[SOLVER], input=asked, capture_output=True, text=True).stdout)
        if said[0] != "sat":
            return {as_set(model, HAND_NAMES) for model in found}
        found.append(said[1])
        script += "(assert (not (and %s)))\n" % " ".join("(= %s %s)" % tuple(pair) for pair in said[1])


@unittest.skipIf(MISSING, "not installed: " + ", ".join(MISSING))
class Pipeline(unittest.TestCase):
    maxDiff = None

    def test_1_the_full_puzzle_has_one_solution_and_both_answers(self):
        models, answers, complete = solved("zebra.txt")
        self.assertTrue(complete)
        self.assertEqual(models, [ZEBRA])
        self.assertEqual(answers, [{16: ["nation-Norwegian"], 17: ["nation-Japanese"]}])
        # and the hand-written encoding says the same, in the same constraints
        hand = (HAND / "einstain-problem.smt").read_text()
        self.assertEqual(clues(smt_of("zebra.txt")), clues(hand, HAND_NAMES))
        self.assertEqual(hand_solutions(hand), {ZEBRA})

    def test_2_a_line_removed_leaves_more_than_one_solution(self):
        full, less = lines_of("zebra.txt"), lines_of("zebra-minus-15.txt")
        self.assertEqual(less, [line for line in full if line != "The Norwegian lives next to the blue house."])
        self.assertEqual(len(less), len(full) - 1)
        models, _, complete = solved("zebra-minus-15.txt")
        self.assertEqual((len(models), complete), (2, False))  # two found, and they are not all
        # against the hand-written twin: the same set of constraints up to the constants' names and
        # the order of = and near's sides, and then the same 32 solutions
        hand = (HAND / "einstain-problem-minus-15.smt").read_text()
        self.assertEqual(clues(smt_of("zebra-minus-15.txt")), clues(hand, HAND_NAMES))
        self.assertEqual(len(clues(smt_of("zebra-minus-15.txt"))), len(clues(smt_of("zebra.txt"))) - 1)
        models, _, complete = solved("zebra-minus-15.txt", limit=40)
        self.assertTrue(complete)
        self.assertEqual(len(models), 32)
        self.assertEqual(set(models), hand_solutions(hand))
        self.assertIn(ZEBRA, models)

    def test_3_the_lines_reordered_reorder_the_assertions_and_keep_the_solution(self):
        full = lines_of("zebra.txt")
        self.assertEqual(lines_of("zebra-reversed.txt"), full[::-1])
        self.assertEqual(sorted(lines_of("zebra-shuffled.txt")), sorted(full))
        said = dict(facts(smt_of("zebra.txt")))  # each sentence's assertion
        self.assertEqual(len(said), 14)
        for name in ("zebra-reversed.txt", "zebra-shuffled.txt"):
            order = [line for line in lines_of(name) if line in said]
            self.assertNotEqual(order, list(said))
            self.assertEqual(facts(smt_of(name)), [(line, said[line]) for line in order])
            models, answers, complete = solved(name)
            self.assertEqual((models, complete), ([ZEBRA], True))
            self.assertEqual({nl2smt.named(c)[1] for who in answers[0].values() for c in who}, {"Norwegian", "Japanese"})

    def test_4_norwegian_and_ukrainian_swapped_swap_in_the_solution(self):
        swap = {"Norwegian": "Ukrainian", "Ukrainian": "Norwegian"}
        self.assertEqual(lines_of("zebra-swapped.txt"),
                         [" ".join(swap.get(word, word) for word in line.split()) for line in lines_of("zebra.txt")])
        swapped = frozenset((nl2smt.named(c)[0] + "-" + swap.get(nl2smt.named(c)[1], nl2smt.named(c)[1]), house)
                            if c.startswith("nation-") else (c, house) for c, house in ZEBRA)
        self.assertNotEqual(swapped, ZEBRA)
        models, answers, complete = solved("zebra-swapped.txt")
        self.assertEqual((models, complete), ([swapped], True))  # still one solution, and it is the other one
        self.assertEqual(answers, [{16: ["nation-Ukrainian"], 17: ["nation-Japanese"]}])
        # the hand-written encoding with the two constants exchanged agrees
        hand = (HAND / "einstain-problem.smt").read_text()
        hand = hand.replace("nation-norway", "#").replace("nation-ukrain", "nation-norway").replace("#", "nation-ukrain")
        self.assertEqual(hand_solutions(hand), {swapped})

    def test_5_the_output_is_no_fixed_text(self):
        # every edit has its own SMT code
        edits = ["zebra.txt", "zebra-minus-15.txt", "zebra-reversed.txt", "zebra-shuffled.txt", "zebra-swapped.txt",
                 "zebra-contradiction.txt", "new-sentences.txt"]
        self.assertEqual(len({smt_of(name) for name in edits}), len(edits))
        # sentences of the vocabulary that are in no version of the puzzle are converted, and rightly
        self.assertEqual([assertion for _, assertion in facts(smt_of("new-sentences.txt"))], [
            "(assert (= nation-Japanese pet-zebra))",       # The Japanese owns the zebra.
            "(assert (= drink-tea colour-blue))",           # Tea is drunk in the blue house.
            "(assert (near nation-Spaniard colour-red))",   # The Spaniard lives next to the red house.
            "(assert (= drink-water 1))",                   # The man who drinks water lives in the first house.
            "(assert (< colour-yellow colour-ivory))",      # The ivory house is to the right of the yellow house.
            "(assert (= smoke-Kools pet-dog))",             # The Kools smoker lives in the house where the dog is kept.
        ])
        self.assertIn("(get-value ((= nation-Englishman colour-green) (= nation-Spaniard colour-green)",
                      smt_of("new-sentences.txt"))          # Who lives in the green house?
        # a line that contradicts the puzzle leaves no solution
        self.assertEqual(lines_of("zebra-contradiction.txt"), lines_of("zebra.txt") + ["The Norwegian drinks milk."])
        self.assertEqual(solved("zebra-contradiction.txt"), ([], [], True))

    def test_5_a_word_outside_the_vocabulary_is_refused_by_name(self):
        with self.assertRaisesRegex(nl2smt.Refused, r'^line 2: the word "cat" is not in the vocabulary'):
            parsed("refused-word.txt")

    def test_5_a_sentence_that_is_not_read_is_refused_with_its_line(self):
        for name, why in (("refused-parse.txt", 'the parser links "the" to nothing'),
                          ("refused-shape.txt", '"drinks" wants a drink, which "dog" is not')):
            with self.assertRaises(nl2smt.Refused) as refusal:
                smt_of(name)
            self.assertTrue(str(refusal.exception).startswith("line 2: cannot read "), refusal.exception)
            self.assertIn(why, str(refusal.exception))

    def test_6_every_linkage_that_is_read_is_read_alike(self):
        """link-parser gives a sentence several linkages and the converter takes the first it can read:
        no sentence of the puzzle has two linkages that are read differently."""
        readings, sentence = {}, None
        for row in nl2smt.convert(parsed("zebra.txt")[1], readings=True).splitlines():
            if row.startswith("line "):
                sentence = row
            elif ": not read, " not in row:
                readings.setdefault(sentence, set()).add(row.split(": ", 1)[1])
        self.assertEqual(len(readings), 17)
        self.assertEqual({sentence: len(read) for sentence, read in readings.items()}, dict.fromkeys(readings, 1))

    def test_7_every_solver_installed_gives_the_same_solution(self):
        installed = [name for name in nl2smt.SOLVERS if shutil.which(name)]
        for solver in installed:
            with self.subTest(solver=solver):
                self.assertEqual(solved("zebra.txt", solver=solver)[::2], ([ZEBRA], True))
        print("solvers run:", ", ".join(installed), end=" ... ", flush=True)


if __name__ == "__main__":
    unittest.main(verbosity=2)
