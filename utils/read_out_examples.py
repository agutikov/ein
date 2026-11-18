#!/usr/bin/env python3
"""Write `docs/ein/user/04-reference/09-read-out-examples.md`, the read-out in real runs.

The examples page of the read-out reference, `docs/ein/user/04-reference/08-read-out.md`
(M1f S1f.5.16, T1f.5.16.2; moved into `docs/ein/user/` by S1f.20.3, which took
the plan links and the history out of its prose). Every code block on the page
is one `ein solve` or `ein oracle` run, as the binary prints it: stdout, then
each stderr line as `stderr: …`, then `exit N`. A block captioned
`--dump-answer DIR` is the `answer.txt` that run writes. The prose between
blocks is this script's. `ein-cli/tests/read_out_examples.rs` runs every
caption again and compares the block, so the page cannot drift from the
binary, and `read_out_reference.rs` holds that every row label the printer has
is printed by one of its blocks; re-write it with

    python3 utils/read_out_examples.py [--bin ein.rs/target/release/ein]
    python3 utils/read_out_examples.py --check   # exit 1 if the page differs

One block is not one whole run: X4's scratch variant, whose query block is
quoted. The test skips that block and reads no prose, so `--check` is what holds
the rest of the page to this script; `utils/check_generated.py` runs it with the
binary the gate tested, which is how it is in the gate (M1f E-45).
"""
import argparse
import atexit
import os
import re
import shutil
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs/ein/user/04-reference/09-read-out-examples.md"
# The pages the prose links, relative to OUT.
QUERY = "../03-syntax-and-semantics/12-query.md"
SOLUTION_SEMANTICS = "../../reasoning/02-kb-evolution/06-solution-semantics.md"

ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
ap.add_argument("--bin", default=os.environ.get("EIN_BIN", str(ROOT / "ein.rs/target/release/ein")))
ap.add_argument("--check", action="store_true", help="write nothing; exit 1 if the page differs from what this script writes")
ap.add_argument("--jobs", type=int, default=int(os.environ.get("EIN_TEST_JOBS") or os.cpu_count() or 1),
                help="runs at once (default: $EIN_TEST_JOBS, else the machine's threads); the page is the same at any count")
ARGS = ap.parse_args()
# Absolute, since the scratch variant runs in a directory of its own.
BIN = str(Path(ARGS.bin).resolve()) if os.sep in ARGS.bin else ARGS.bin
SCRATCH = Path(tempfile.mkdtemp(prefix="read-out-examples-"))
atexit.register(shutil.rmtree, SCRATCH, True)
# Every run is independent of every other, so they go to a pool: `block` puts a
# mark where its run's text goes, and the page is assembled once all have ended.
POOL = ThreadPoolExecutor(max_workers=max(1, ARGS.jobs))
RUNS = []

# The one program that is not a corpus file: 07 with a derived `pair-at`.
variant_dir = SCRATCH / "ex/lattice"
variant_dir.mkdir(parents=True, exist_ok=True)
src = (ROOT / "examples/lattice/07_two_pairs_two_tables.ein").read_text()
variant = src.replace(
    "(query :goal (sits ?p ?s))",
    "(relation pair-at Group Table :why \"pair {?1} sits at table {?2}\")\n\n(rule pair-at ()\n  :match  (and (member ?a ?g) (sits ?a ?s) (at ?s ?t))\n  :assert (pair-at ?g ?t)\n  :why    \"{?a} of pair {?g} sits in {?s}, a seat at table {?t}\")\n\n(query :goal (pair-at ?g ?t))",
)
assert variant != src
(variant_dir / "07_two_pairs_two_tables+pair-at.ein").write_text(variant)


def argv(args):
    """The run's arguments: `--uncut` is `--dump-answer DIR`."""
    a = []
    for x in args:
        a.extend(["--dump-answer", "DIR"] if x == "--uncut" else [x])
    return a


def run(args, cwd=ROOT, cmd="solve"):
    """The block a run prints, at once, beside the pool: for the prose that reads it."""
    return printed(args, cwd, cmd)


def printed(args, cwd=ROOT, cmd="solve"):
    """The block a run prints: stdout (or `answer.txt`), stderr, exit."""
    a = argv(args)
    dump = None
    if "DIR" in a:
        dump = Path(tempfile.mkdtemp(prefix="answer-", dir=SCRATCH))
        a = [str(dump) if x == "DIR" else x for x in a]
    p = subprocess.run([BIN, cmd, *a], cwd=cwd, capture_output=True, text=True)
    stdout = p.stdout
    if dump is not None:
        stdout = (dump / "answer.txt").read_text().replace(str(dump), "DIR")
    body = stdout.rstrip("\n")
    if p.stderr.strip():
        body += "\n\n" + "\n".join("stderr: " + l for l in p.stderr.splitlines())
    return body + f"\n\nexit {p.returncode}"


def block(args, cwd=ROOT, note="", cmd="solve"):
    shown = " ".join(
        x for x in argv(args) if x not in ("--jobs", "8")
    )
    RUNS.append(POOL.submit(printed, args, cwd, cmd))
    cap = f"`ein {cmd} {shown}`" + (f" — {note}" if note else "")
    return f"{cap}\n\n```text\n\x00{len(RUNS) - 1}\x00\n```\n"


S = []
add = S.append

add("""# The read-out in real runs

**What this page is.** The examples for [`08-read-out.md`](08-read-out.md), the read-out reference: every block below is one
run of the binary, stdout, then each stderr line as `stderr: …`, then the exit code. A block captioned `--dump-answer
DIR` is the `DIR/answer.txt` that run writes: the read-out with no listing cut. One block is not one whole run: X4's
variant, whose query block is quoted. **The page is generated** from the binary's own runs and not edited by hand, and
the test suite runs every caption again and compares the block, so every block is what the binary prints.

""")
add("""---

## Contents

| | program | n | what it shows |
|---|---|---:|---|
| [X1](#x1--branching05_mini_zebraein-everything-at-once) | `branching/05_mini_zebra.ein` | 9 | the default read-out, every row; `(merged …)` with `(or …)`; a query with an undetermined class |
| [X2](#x2--lattice02_genuine_3set_deathein-one-program-every-mode) | `lattice/02_genuine_3set_death.ein` | 3 | one program under every question: describe, `--maximal`, `--minimal`, `-n 2`, `--single`, `--maximal --single`, `--count`, `--monotone`, and `-y` |
| [X3](#x3--features11_expect_ambiguityein-a-query-over-the-maximal-line) | `features/11_expect_ambiguity.ein` | 4 | a query `:over maximal`, and its claim holding |
| [X4](#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes) | `lattice/07_two_pairs_two_tables.ein` | 16 | 8 components, and 2 projection classes of 4 |
| [X5](#x5--latticesynthmerged_statesein-every-state-merged) | `lattice/synth/merged_states.ein` | 5 | ∅ not in R, and what it owes; a `(merged …)` on every state; two components |
| [X6](#x6--testsregion04_owingein-r-empty-alive-nodes-owe) | `tests/region/04_owing.ein` | 0 | empty R, the owing kind; `--single` says `Open` |
| [X7](#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) | `tests/stdlib/algebra/09_irreflexive_violated.ein` | 0 | empty R, the dead kind; `--single` says `Contradiction` |
| [X8](#x8--lattice04_obligation_trigger_holeein-a-hole) | `lattice/04_obligation_trigger_hole.ein` | 2 | a program the static check flags: route 2, a hole, the warning |
| [X9](#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) | `lattice/06_friends_tables.ein` | 19 | flagged and past `--region-limit`: the line members describe certifies, under describe and `--maximal`; `--minimal` decided at root, ∅ being in R; `--count` |
| [X10](#x10--zebra2ein-the-puzzle) | `zebra2.ein` | 56 | the puzzle: cut on stdout and whole in `answer.txt`, its region and the 36 instances root owes; `--maximal -n 1`; `--single` |
| [X11](#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) | `saturation/square-bwd/floors.ein` | 20 | R is the whole cube: decided, its states counted in part; the first questions in 1–11 saturations |
| [X12](#x12--testsregion03_merged_orein-a-program-that-states-its-region) | `tests/region/03_merged_or.ein` | 3 | a program that states its region: the printed region is its claim |
| [X13](#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines) | `saturation/type-exclusivity/colors.ein` | 8 | an exhausted R past stdout's 100 lines: what fits, and where the rest is |
| [X14](#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) | `features/07_unstratifiable.ein` | 1 | every row of the warning, the strata, and `--monotone` on a flagged program |
| [X15](#x15--features01_not_and_absentein-the-warning-past-four-rows) | `features/01_not_and_absent.ein` | 35 | the warning past four stale nodes |
| [X16](#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) | `ein-bugs/zebra2-bad.ein`, `ein-bugs/wrong-answers/core-search-cut.ein` | 0 | why root is dead: its core, a fact's `:source`, and a core search cut at its budget |
| [X17](#x17--ein-oracle-the-cubes-line) | `lattice/04_obligation_trigger_hole.ein`, `lattice/02_genuine_3set_death.ein` | 2, 3 | `ein oracle`: the read-out over the cube, and the cube's line |
| [X18](#x18--testsregion04_owingein---no-search-root-alone) | `tests/region/04_owing.ein`, `tests/stdlib/algebra/09_irreflexive_violated.ein`, `zebra2.ein`, `tests/stdlib/bijection/01_setup_and_negatives.ein` | 0, 0, 56, 0 | `--no-search`: root alone, an owing root, a dead one and the puzzle's, R not searched; and a program that states `(solve :no-search)` |
| [X19](#x19--features07_unstratifiableein---no-search-flagged-and-its-strata) | `features/07_unstratifiable.ein` | 1 | `--no-search` on a flagged program: the static check's rows and the strata, the cube not read |
| [X20](#x20--ein-bugswrong-answersoblig-derived-activatorein-a-debt-root-does-not-owe) | `ein-bugs/wrong-answers/oblig-derived-activator.ein` | 2 | a debt root does not owe and a commitment can bring: the `later debt` row, and what H holds for it |
| [X21](#x21--testssaved04_a_saved_nodeein-a-saved-file-a-node-of-its-programs-lattice) | `tests/saved/04_a_saved_node.ein` | 0 | a saved file: the `commit` row, the part of the program's R above the node, and the program's claims not checked |
""")

add("""## X1 — `branching/05_mini_zebra.ein`: everything at once

Two people, two drinks and two pets; the clues put Alice
with the tea and the cat, and Bob is open. Every row of the default read-out is here. ∅ is in R, root being alive and
owing nothing (the program states no obligation), so the minimal line is {∅}. The maximal state is reached by four commitments, three
of them minimal, and that is the one `(merged …)`. The query asks who drinks the coffee and which pet they keep: on
four of the five states the goal has no row, and that class is `(model)`. H leaves `is-a` out, and the `outside` row
says what R holds of it.
""")
add(block(["examples/branching/05_mini_zebra.ein"]))

add("""## X2 — `lattice/02_genuine_3set_death.ein`: one program, every mode

[`06-solution-semantics.md` § 2.5]("""+SOLUTION_SEMANTICS+"""#25-worked-a-conflict-of-three)'s worked
example: three candidates, every pair alive, the triple dead, no obligation. One run per question, so the same R can be
read through each.

**Describe.** Seven states, the three pairs on the maximal line, three sub-cubes of dimension 2.
""")
add(block(["examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--maximal`.** The line is listed as states, not as a `(region …)`: a line is not a claim the language can state
(only a query's `:over` names one), and a `(region …)` read back would say that these three are all of R.
""")
add(block(["--maximal", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--minimal`.** ∅ is in R, so the line is {∅}, decided at root.
""")
add(block(["--minimal", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`-n 2`.** The first two states in the route's order, the same two under every lever. Nothing is said about the
rest, and the exit code is 0: a request, not an expectation.
""")
add(block(["-n", "2", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--single`.** The run's own expectation, so the verdict word is printed: `Ambiguity`, two states found. It exits
0, as every answer does.
""")
add(block(["--single", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--maximal --single`.** The question a restrictions-only program means by *unique*: the maximal line has three
states, so `Ambiguity` again, with two of them.
""")
add(block(["--maximal", "--single", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--count`.** *How many?*, and nothing else: describe's route and its two counts, with the rows that qualify them,
each describe's text. No state is read, so nothing is listed and the query is not answered. Over all of R only: beside
`--minimal` or `--maximal` it is a usage error that names a bare `ein solve`.
""")
add(block(["--count", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--monotone`.** A question about the program: root's fixpoint and the static check, no node built. The program
is monotone, so there is no warning.
""")
add(block(["--monotone", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`-y`.** The parent-path spot check, every one-step parent of every printed state: under a monotone program's
`lattice` row it prints its `paths` row, which the theorem says must find nothing. It moves no other row.
""")
add(block(["-y", "examples/lattice/02_genuine_3set_death.ein"]))

add("""## X3 — `features/11_expect_ambiguity.ein`: a query over the maximal line

Two people, two seats, restrictions only. The query is `:over maximal` ([`12-query.md`]("""+QUERY+""")): over all of R
the goal `(seat ?who ?slot)` would see every seat fact, so each state would be its own class and ∅ the undetermined
one. Over the maximal line its answer is two classes, and its claim holds.
""")
add(block(["examples/features/11_expect_ambiguity.ein"]))

add("""## X4 — `lattice/07_two_pairs_two_tables.ein`: components and projection classes

Two pairs of friends, two tables of two seats, friends sit together. R is the eight full seatings. Each is its own
**component**, because moving one friend passes through a node that is dead or owing, and the minimal and maximal
lines coincide. The goal `(sits ?p ?s)` names the seats, so each state is its own projection class.
""")
add(block(["examples/lattice/07_two_pairs_two_tables.ein"]))
add("""A goal that hides the seat needs a relation that does not carry it: a projection class is the goal's **patterns**
under its bindings. The program below is 07 with one relation and one rule added, and it is **not an example file**:

```lisp
(relation pair-at Group Table :why "pair {?1} sits at table {?2}")

(rule pair-at ()
  :match  (and (member ?a ?g) (sits ?a ?s) (at ?s ?t))
  :assert (pair-at ?g ?t)
  :why    "{?a} of pair {?g} sits in {?s}, a seat at table {?t}")

(query :goal (pair-at ?g ?t))
```

R does not move (the rule derives no candidate), and the query sees two classes of four: which pair took which table.
""")
var = run(["lattice/07_two_pairs_two_tables+pair-at.ein"], cwd=SCRATCH / "ex")
q = var[var.index("  query"):]
add(f"`ein solve 07_two_pairs_two_tables+pair-at.ein`, the query block\n\n```text\n{q}\n```\n")

add("""## X5 — `lattice/synth/merged_states.ein`: every state merged

*A brings B* and *C brings D*; an obligation needs A or C, and A with C is dead. ∅ owes, so it is not in R, and the
`owed` row under `∅` names the one instance it owes, through the obligation rule's `:why`. The
minimal line is two states, each reached from one candidate. Every state is reached by exactly two commitments, its
minimal one and that one plus what it brings, so every state gets a `(merged …)`: k_c = 16 against k_s = 8. The
A-side and the C-side never meet, which is the two components.
""")
add(block(["examples/lattice/synth/merged_states.ein"]))

add("""## X6 — `tests/region/04_owing.ein`: R empty, alive nodes owe

Root owes one `total-owed` instance, Ann's Food, and the program states `(solve :no-search)`, so nothing is proposed:
no candidate, and R is empty of the second kind. The `owing` rows are the frontier, the alive owing nodes with no alive node one fact up (here root itself), each with
what it owes through the rule's `:why`. The printed form is `(region (owing (model)))`: it names no fact because the
printer lists the facts of the **candidates'** relations, which tell R's states apart, and this program has none. The query's
`over` row says which kind of empty R is: root is alive and owes one instance.
""")
add(block(["tests/region/04_owing.ein"]))
add("""**`--single`.** Expected one state, found none of the owing kind: `Open`.
""")
add(block(["--single", "tests/region/04_owing.ein"]))

add("""## X7 — `tests/stdlib/algebra/09_irreflexive_violated.ein`: R empty, every node dead

Root holds `(loops A A)`, which `irreflexive` refutes. Root's core and the rule that refutes it say why, and the form
is `(false)`. The query's `over` row says that root is dead, where X6's says that it owes: the claim under each is
`(false)`.
""")
add(block(["tests/stdlib/algebra/09_irreflexive_violated.ein"]))
add("""**`--single`.** Expected one state, found none of the dead kind: `Contradiction`.
""")
add(block(["--single", "tests/stdlib/algebra/09_irreflexive_violated.ein"]))

add("""## X8 — `lattice/04_obligation_trigger_hole.ein`: a hole

A hole with no `absent` in any saturation rule. `total-owed meal is-a` makes every diner owe a meal, and
`seated-dines` derives `(is-a ?g Diner)` from a seat, so the obligation's trigger grows with the commitment: `{(seats
Ann C1)}` owes, between ∅ and `{(meal Ann Soup), (seats Ann C1)}`, both in R. The static check flags the program and
route 2 takes the whole cube. R is `{∅, {meal}, {meal, seats}}`, and `{seats}` is a **hole**: outside
R, between two members of it. The `lattice` block is the detector's warning: what is assumed, the site and what it puts
at stake, the status the cube shows (exercised, by `total-owed`), the state (no stale fact), the parent paths checked,
and then each minimal hole with why it is out. The `E` row is the cube's alone.
""")
add(block(["examples/lattice/04_obligation_trigger_hole.ein"]))

add("""## X9 — `lattice/06_friends_tables.ein`: flagged, and past the limit

The everyone-sits twin's sibling, where *a group that does not fit goes elsewhere*: the obligation's trigger grows
with the commitment, so the program is not monotone. n = 19 is past `--region-limit 16`, so no route can describe R.
The run visits the cube in layer order up to 2¹⁶ nodes. It meets 21 members, and it cannot know that they are all of
R, so every count is a lower bound. What it *can* certify is one line: ∅ is in R, so the minimal line is exactly {∅},
whatever the program. That is the one state it lists. The other 20 members it met go to `--dump-answer`, and the query
is answered over all 21 as a lower bound, with no `as :expect`: read back, that claim would say these classes are all.
The warning says the cube's half is not known past the limit, reads the stale facts of every node the run computed,
and checks no parent path, because the read-out prints no state of two commitments or more.
""")
add(block(["examples/lattice/06_friends_tables.ein"]))
add("""**`--count`** on the same program: the counts are describe's lower bounds, for the same reason, and the warning is
printed whole, since it qualifies them. Its `paths` row says the spot check was not run: `--count` reads no state.
""")
add(block(["--count", "examples/lattice/06_friends_tables.ein"]))
add("""**`--minimal`** on the same program stops at root: ∅ is in R, so the minimal line is {∅} and no other state can be
on it, decided at root though the program is flagged and past the limit.
""")
add(block(["--minimal", "examples/lattice/06_friends_tables.ein"]))
add("""**`--maximal`** lists no state: the maximal line needs the whole cube, and the row says so, as describe's does.
The 21 members met are counted and in `--dump-answer`.
""")
add(block(["--maximal", "examples/lattice/06_friends_tables.ein"]))

add("""## X10 — `zebra2.ein`: the puzzle

**At the default budget** the state route describes R exhausted. R is one state, 261 982 commitments, 63 minimal
generators, and four of them single facts. Root owes 36 instances, a house for each of 18 values the clues have not
placed and a value for each of 18 places still empty: the `owed` rows name four, and one line counts the other 32. R
as one `(region …)` is past stdout's 100 lines, so stdout lists the state and leaves out the `(merged …)`, and says
where it is. The query's class is exact, so it is printed with its `as :expect`.
""")
add(block(["examples/zebra2.ein"]))
add("""**What `--dump-answer DIR` writes to `DIR/answer.txt`** for the same run: all 36 `owed` rows, and the whole
`(region …)`, whose `(merged …)` lists all 63 minimal commitments. That is what pins k_c, and it reads back as the
claim.
""")
add(block(["--uncut", "examples/zebra2.ein"], note="`DIR/answer.txt`"))
add("""**`--maximal -n 1`**: a complete solution, by the obligation tree's first leaf, then the climb.
""")
add(block(["--maximal", "-n", "1", "examples/zebra2.ein"]))
add("""**`--single`**: `Solution`, by the obligation tree, whose one leaf has every extension dead (L4, L5).
""")
add(block(["--single", "examples/zebra2.ein"]))

add("""## X11 — `saturation/square-bwd/floors.ein`: a region that is the whole cube

No restriction and no obligation: every commitment is alive and in R, 2²⁰ of them. **Describe** stops at the default
budget, partial, and R is still decided: ∅ is in R, and one saturation of ⊤ finds it alive and owing nothing, so R is
the whole cube (L1, L2). k_c, both lines, the one component, the one sub-cube and N are exact. What is not is the
states, and the state route counts those of up to eight candidates before the default budget of 2²⁰ facts. The two line
states are what it lists. The other members it met go to `--dump-answer`, not to the screen, and the query's one class
is printed without `as :expect`, since it is a lower bound.
""")
add(block(["examples/saturation/square-bwd/floors.ein"]))
add("""**The first questions** each read a few nodes, which the `work` row counts: `-n 1` is root, `--single` is `Ambiguity`
after a second state, `--maximal -n 1` is the climb. The climb ends at ⊤, so every node is alive (L1) and ⊤ is the
whole maximal line, and the run says so.
""")
add(block(["-n", "1", "examples/saturation/square-bwd/floors.ein"]))
add(block(["--single", "examples/saturation/square-bwd/floors.ein"]))
add(block(["--maximal", "-n", "1", "examples/saturation/square-bwd/floors.ein"]))

add("""## X12 — `tests/region/03_merged_or.ein`: a program that states its region

A `(merged …)` with `(or …)`: the colocation chain. Its `(solve … :expect (region …))` ([`12-query.md`]("""+QUERY+"""))
is checked against the R the run described, and the `(region …)` printed above the query is that claim's own text: one
form, both directions.
""")
add(block(["tests/region/03_merged_or.ein"]))
add("""**`--count`** on the same program checks no claim, since it reads no state, and says so in one `claims` row, with a
`note:` on stderr per claim. It exits 0: a claim not checked is not refuted.
""")
add(block(["--count", "tests/region/03_merged_or.ein"]))

colors = ["examples/saturation/type-exclusivity/colors.ein"]
cut = run(colors)
whole = run(["--uncut", *colors])
import re
k_s = re.search(r"states \(k_s\)\s+(\S+)", cut).group(1)
dead = re.search(r"  N\s+(\d+) minimal dead sets", cut).group(1)
region_lines = re.search(r"R as one \(region …\) is (\S+) lines", cut).group(1)
shown = re.search(r";; state (\d+) of \S+ —[^\n]*\n(?:[^\n]*\n)*?  ;; not shown", cut)
last = max(int(m) for m in re.findall(r";; state (\d+) of", cut))
stdout_lines = len(cut[: cut.rindex("\n\nexit")].splitlines())
file_lines = len(whole[: whole.rindex("\n\nexit")].splitlines())
add(f"""## X13 — `saturation/type-exclusivity/colors.ein`: an exhausted R past 100 lines

Two colours, one rule that keeps them from co-locating, and no obligation: the universe is eight `is-a` and
`co-located` facts, {dead} minimal dead sets keep R from being the cube, and R is {k_s} states. As one `(region …)` it is
{region_lines} lines. **Stdout's listings share 100 lines**, each printing whole items while they last and at least
one. The region comes first and takes them: stdout lists states 1–{last} outside a `(region …)`, since a cut wrapped
in one would claim to be all of R, then says what it left out and that `--dump-answer DIR` writes the whole
`(region …)` to `DIR/answer.txt`. The query still gets its `over` row, its one class and its one-line `as :expect`.
The rows are never cut. Stdout is {stdout_lines} lines, and `answer.txt` is {file_lines}. The state numbers are the
same in both, so *state {last}* on the screen is *state {last}* in the file.
""")
add(block(colors))

add("""## X14 — `features/07_unstratifiable.ein`: every row of the warning, and the strata

Two rules, each deriving what the other's `absent` reads: the static check flags both, the detector finds an even cycle
through an `absent` and an order finding, and the rules' dependency graph does not stratify, so `strata` and its
`negative cycle` follow the warning. On the cube one node holds a stale fact, which the `stale` row names. Every answer
stands: the rows are diagnostics.
""")
add(block(["examples/features/07_unstratifiable.ein"]))
add("""**`--monotone`** on the same program: the warning's static half, and `bit`, which says what would compute the
cube's half and at what cost. `strata` is printed by a question about R, not by this one.
""")
add(block(["--monotone", "examples/features/07_unstratifiable.ein"]))

add("""## X15 — `features/01_not_and_absent.ein`: the warning past four rows

A flagged program past `--region-limit` whose nodes hold thousands of stale facts: stdout names four `stale` nodes,
then one `…` row counts the rest and names what writes them, so the warning stays bounded on its own and takes none of
the listings' 100 lines.

Under `universe`, the `relation names` row: the rule `see-stored-neg` concludes `explicitly-dislikes` facts, no file
declares the relation or writes a fact of it, so the relation is an atom the blind enumerator proposes about, the
row's *object*, and a candidate can have it as an argument. The candidates stay in H, and the row counts them.
""")
add(block(["examples/features/01_not_and_absent.ein"]))

add("""## X16 — `ein-bugs/zebra2-bad.ein`: a dead root's core, its sources and a cut search

`zebra2.ein` with one clue added that contradicts the rest: root's own fixpoint derives `(false)`, so R is empty of the
dead kind. Each fact of root's core is an `unsat core` row, a fact the program states with a `:source` gets a `source`
row, and `refuted by` names the step the core's derivation takes for each fact of the contradiction, `(given)` for one
the core holds.
""")
add(block(["examples/ein-bugs/zebra2-bad.ein"]))
add("""**A core search cut at its budget** (`ein-bugs/wrong-answers/core-search-cut.ein`): the core
printed is one the search found within 64 environments per fact, and the `core search` row says that a smaller core may
exist.
""")
add(block(["examples/ein-bugs/wrong-answers/core-search-cut.ein"]))

add("""## X17 — `ein oracle`: the cube's line

`ein oracle FILE` saturates every node of the cube, asked for by name, and prints the describe read-out over it, then
one line that only the cube can say: its dead nodes with an alive superset and discharged nodes with an owing superset,
and whether the static verdict held. On X8's flagged program the status is not monotone, by the obligation's trigger.
""")
add(block(["examples/lattice/04_obligation_trigger_hole.ein"], cmd="oracle"))
add("""On X2's monotone program the static verdict holds: no dead node has an alive superset, and none discharged an
owing one.
""")
add(block(["examples/lattice/02_genuine_3set_death.ein"], cmd="oracle"))

add("""## X18 — `tests/region/04_owing.ein --no-search`: root alone

`--no-search` declines to discover R: it saturates root, reports what root is, as describe's `∅` row
and `owed` block say it, and says in the `R` row that R was not searched. No row counts or lists R. The program states
`(solve :no-search)` itself, so a bare run would describe R by root alone, as X6 does; the flag reads root the same way
on any program. Root owes Ann a Food, so ∅ is not in R, the query has no state to answer from, and the claim, which is
about R, is not checked.
""")
add(block(["--no-search", "tests/region/04_owing.ein"]))
add("""**A dead root** (X7's program): every node is dead, a fork only adding (L0), so the claim `(false)` is decided as
the full run decides it, and root's core is printed as describe prints it.
""")
add(block(["--no-search", "tests/stdlib/algebra/09_irreflexive_violated.ein"]))
add("""**The puzzle** (X10's program): root owes 36 instances, four of them named and the rest counted, and H holds 56
candidates, none visited. The forms draw root whatever it is: `-f` lists what root alone deduced of the candidates'
relations, and `-p` its facts.
""")
add(block(["--no-search", "examples/zebra2.ein"]))
add("""**A program that states `(solve :no-search)`**: its lattice is root alone, so a bare `ein solve` describes R, here
empty of the owing kind, and the `universe` row says why it has no candidate.
""")
add(block(["tests/stdlib/bijection/01_setup_and_negatives.ein"]))

add("""## X19 — `features/07_unstratifiable.ein --no-search`: flagged, and its strata

X14's program under `--no-search`: the static check's rows and the strata are the program's, and print as describe
prints them; the cube's `status` is not known, since no node past root was built.
""")
add(block(["--no-search", "examples/features/07_unstratifiable.ein"]))

add("""## X20 — `ein-bugs/wrong-answers/oblig-derived-activator.ein`: a debt root does not owe

Ann owes a Food at root. The rule `arm` concludes `(total-owed drinks is-a)`, the activator of a second obligation,
from any `likes` fact, so the node that holds `(likes Ann Soup)` owes Ann a Drink, a debt root was never told of. The
obligations rung reads the activator off `arm`'s `:assert` and proposes at root what would pay the debt it brings: H
holds `(drinks Ann Tea)` beside `(likes Ann Soup)`, the `later debt` row under `universe` says which candidate is there
for which activator, and R is the one state that pays both debts. The program is one the static check flags, the
activator's relation being one a commitment can add a fact of, so the cube answers it.
""")
add(block(["examples/ein-bugs/wrong-answers/oblig-derived-activator.ein"]))

add("""## X21 — `tests/saved/04_a_saved_node.ein`: a saved file, a node of its program's lattice

The state `{(note (bad a))}` of `tests/saved/02_a_choice_with_a_fact_inside_it.ein`, as `--dump-answer` saved it: the
program as written and `(commit (note (bad a)))`. The run is the part of the program's R above that node: its universe
is H less the choice, here no candidate, so R is {∅}, ∅ being the node, and the `commit` row under `universe` names
it. The program's claims are about the program's R, so none is checked, and the run exits 0.
""")
add(block(["tests/saved/04_a_saved_node.ein"]))

PAGE = re.sub("\x00(\\d+)\x00", lambda m: RUNS[int(m.group(1))].result(), "\n".join(S))
POOL.shutdown()
shutil.rmtree(SCRATCH)
if ARGS.check:
    if OUT.read_text() != PAGE:
        have, want = OUT.read_text().splitlines(), PAGE.splitlines()
        at = next((i for i, (h, w) in enumerate(zip(have, want)) if h != w), min(len(have), len(want)))
        print(f"{OUT.relative_to(ROOT)} is not what utils/read_out_examples.py writes "
              f"(first difference at line {at + 1}; the page has {len(have)} lines, the script {len(want)}): "
              "re-write it with `python3 utils/read_out_examples.py`", file=sys.stderr)
        sys.exit(1)
    sys.exit(0)
OUT.write_text(PAGE)
print(f"wrote {OUT} ({OUT.read_text().count(chr(10))} lines)")
