#!/usr/bin/env python3
"""Write `docs/kernel/inference/read_out_examples.md`, the read-out in real runs.

The examples page of the read-out reference, `docs/kernel/inference/read_out.md`
(M1f S1f.5.16, T1f.5.16.2; written for M1e2 T1e2.1a.5.2 under `plans/` until
then, where that page stays frozen as the record). Every code block on the page
is one `ein solve` or `ein oracle` run, as the binary prints it: stdout, then
each stderr line as `stderr: …`, then `exit N`. A block captioned
`--dump-answer DIR` is the `answer.txt` that run writes. The prose between
blocks is this script's. `ein-cli/tests/read_out_examples.rs` runs every
caption again and compares the block, so the page cannot drift from the
binary, and `read_out_reference.rs` holds that every row label the printer has
is printed by one of its blocks; re-write it with

    python3 utils/read_out_examples.py [--bin ein.rs/target/release/ein]

One block is not one whole run: X4's scratch variant, whose query block is
quoted.
"""
import argparse
import os
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs/kernel/inference/read_out_examples.md"
# M1e2's page, frozen at S1f.5.16: where the plan links go.
PLAN = "../../../plans/m1e2_correctness/p1e2.1a_the_solution_region"

ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
ap.add_argument("--bin", default=os.environ.get("EIN_BIN", str(ROOT / "ein.rs/target/release/ein")))
BIN = ap.parse_args().bin
SCRATCH = Path(tempfile.mkdtemp(prefix="read-out-examples-"))

# The one program that is not a corpus file: 07 with a derived `pair-at`.
variant_dir = SCRATCH / "ex/lattice"
variant_dir.mkdir(parents=True, exist_ok=True)
src = (ROOT / "examples/lattice/07_two_pairs_two_tables.ein").read_text()
variant = src.replace(
    "(query :goal (sits ?p ?s))",
    "(relation pair-at Group Table)\n\n(rule pair-at ()\n  :match  (and (member ?a ?g) (sits ?a ?s) (at ?s ?t))\n  :assert (pair-at ?g ?t))\n\n(query :goal (pair-at ?g ?t))",
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
    text = run(args, cwd, cmd)
    cap = f"`ein {cmd} {shown}`" + (f" — {note}" if note else "")
    return f"{cap}\n\n```text\n{text}\n```\n"


S = []
add = S.append

add("""# The read-out in real runs

**What this page is.** The examples for [`read_out.md`](read_out.md), the read-out reference: every block below is one
run of the binary, written by [`utils/read_out_examples.py`](../../../utils/read_out_examples.py), stdout, then each
stderr line as `stderr: …`, then the exit code. A block captioned `--dump-answer DIR` is the `DIR/answer.txt` that run
writes: the read-out with no listing cut. [`ein-cli/tests/read_out_examples.rs`](../../../ein.rs/crates/ein-cli/tests/read_out_examples.rs)
runs every caption again and compares the block, so the page is the read-out's golden, and
`ein-cli/tests/read_out_reference.rs` holds that every row label the printer has is printed by one of the blocks. One
block is not one whole run: X4's scratch variant, whose query block is quoted. Re-write it with
`python3 utils/read_out_examples.py` in the commit that changes what the read-out prints.

**Since M1f S1f.5.16** (2026-11-09, `79fc14b0`). Until then the page was M1e2's, written for the user's rulings of 2026-10-16; that
page is frozen where it was, beside [`read_out.md`'s record]("""+PLAN+"""/read_out.md), and no test reads it.

""")
add("""---

## Contents

| | program | n | what it shows |
|---|---|---:|---|
| [X1](#x1--branching05_mini_zebraein-everything-at-once) | `branching/05_mini_zebra.ein` | 9 | the default read-out, every row; `(merged …)` with `(or …)`; a query with an undetermined class |
| [X2](#x2--lattice02_genuine_3set_deathein-one-program-every-mode) | `lattice/02_genuine_3set_death.ein` | 3 | one program under every question: describe, `--maximal`, `--minimal`, `-n 2`, `--single`, `--maximal --single`, `--count`, `--monotone`, and `-y` |
| [X3](#x3--features11_expect_ambiguityein-a-query-over-the-maximal-line) | `features/11_expect_ambiguity.ein` | 4 | a query `:over maximal`, and its claim holding |
| [X4](#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes) | `lattice/07_two_pairs_two_tables.ein` | 16 | 8 components, and 2 projection classes of 4 |
| [X5](#x5--latticesynthmerged_statesein-every-state-merged) | `lattice/synth/merged_states.ein` | 5 | ∅ not in R; a `(merged …)` on every state; two components |
| [X6](#x6--testsstdlibclosure03_closed_and_owingein-r-empty-alive-nodes-owe) | `tests/stdlib/closure/03_closed_and_owing.ein` | 0 | empty R, the owing kind; `--single` says `Open` |
| [X7](#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) | `tests/stdlib/algebra/09_irreflexive_violated.ein` | 0 | empty R, the dead kind; `--single` says `Contradiction` |
| [X8](#x8--lattice04_obligation_trigger_holeein-a-hole) | `lattice/04_obligation_trigger_hole.ein` | 2 | a program the static check flags: route 2, a hole, the warning |
| [X9](#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) | `lattice/06_friends_tables.ein` | 19 | flagged and past `--region-limit`: the line members describe certifies, under describe and `--maximal`; `--minimal` decided at root, ∅ being in R; `--count` |
| [X10](#x10--zebra2ein-the-puzzle) | `zebra2.ein` | 56 | the puzzle: cut on stdout and whole in `answer.txt`; `--maximal -n 1`; `--single` |
| [X11](#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) | `saturation/square-bwd/floors.ein` | 20 | R is the whole cube: decided, its states counted in part; the first questions in 1–11 saturations |
| [X12](#x12--testsregion03_merged_orein-a-program-that-states-its-region) | `tests/region/03_merged_or.ein` | 3 | a program that states its region: the printed region is its claim |
| [X13](#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines) | `saturation/type-exclusivity/colors.ein` | 8 | an exhausted R past stdout's 100 lines: what fits, and where the rest is |
| [X14](#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) | `features/07_unstratifiable.ein` | 1 | every row of the warning, the strata, and `--monotone` on a flagged program |
| [X15](#x15--features01_not_and_absentein-the-warning-past-four-rows) | `features/01_not_and_absent.ein` | 35 | the warning past four stale nodes |
| [X16](#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) | `ein-bugs/zebra2-bad.ein`, `ein-bugs/wrong-answers/core-search-cut.ein` | 0 | why root is dead: its core, a fact's `:source`, and a core search cut at its budget |
| [X17](#x17--ein-oracle-the-cubes-line) | `lattice/04_obligation_trigger_hole.ein`, `lattice/02_genuine_3set_death.ein` | 2, 3 | `ein oracle`: the read-out over the cube, and the cube's line |
""")

add("""## X1 — `branching/05_mini_zebra.ein`: everything at once

[`region_examples.md`]("""+PLAN+"""/region_examples.md)'s E1. Two people, two drinks and two pets; the clues put Alice
with the tea and the cat, and Bob is open. Every row of the default read-out is here. ∅ is in R, root being alive and
owing nothing (the program states no obligation), so the minimal line is {∅}. The maximal state is reached by four commitments, three
of them minimal, and that is the one `(merged …)`. The query asks who drinks the coffee and which pet they keep: on
four of the five states the goal has no row, and that class is `(model)`. H leaves `is-a` out, and the `outside` row
says what R holds of it.
""")
add(block(["examples/branching/05_mini_zebra.ein"]))

add("""## X2 — `lattice/02_genuine_3set_death.ein`: one program, every mode

[`solution_semantics.md` § 2.5](solution_semantics.md#25-worked-exampleslattice02_genuine_3set_deathein)'s worked
example: three candidates, every pair alive, the triple dead, no obligation. One run per question, so the same R can be
read through each.

**Describe.** Seven states, the three pairs on the maximal line, three sub-cubes of dimension 2.
""")
add(block(["examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--maximal`.** The line is listed as states, not as a `(region …)`: a line is not a claim the language can state
(only a query's `:over` names one), and a `(region …)` read back would say that these three are all of R.
""")
add(block(["--maximal", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--minimal`.** ∅ is in R, so the line is {∅}: one saturation.
""")
add(block(["--minimal", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`-n 2`.** The first two states in the route's order, the same two under every lever. Nothing is said about the
rest, and the exit code is 0: a request, not an expectation.
""")
add(block(["-n", "2", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--single`.** The run's own expectation, so the verdict word is printed: `Ambiguity` after two saturations. It
exits 0, as every answer does.
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

[`region_examples.md`]("""+PLAN+"""/region_examples.md)'s E5. Two people, two seats, restrictions only. The query is
`:over maximal`: over all of R the goal `(seat ?who ?slot)` would see every seat fact, so each state would be its own
class and ∅ the undetermined one. Over the maximal line its answer is two classes, and its claim holds
([`region_language.md` § 7]("""+PLAN+"""/region_language.md#7-the-migration-named) migrated it).
""")
add(block(["examples/features/11_expect_ambiguity.ein"]))

add("""## X4 — `lattice/07_two_pairs_two_tables.ein`: components and projection classes

Two pairs of friends, two tables of two seats, friends sit together. R is the eight full seatings. Each is its own
**component**, because moving one friend passes through a node that is dead or owing, and the minimal and maximal
lines coincide. The goal `(sits ?p ?s)` names the seats, so each state is its own projection class.
""")
add(block(["examples/lattice/07_two_pairs_two_tables.ein"]))
add("""A goal that hides the seat needs a relation that does not carry it: a projection class is the goal's **atoms**
under its bindings. The program below is 07 with one relation and one rule added, and it is **not a corpus file**:

```lisp
(relation pair-at Group Table)

(rule pair-at ()
  :match  (and (member ?a ?g) (sits ?a ?s) (at ?s ?t))
  :assert (pair-at ?g ?t))

(query :goal (pair-at ?g ?t))
```

R does not move (the rule derives no candidate), and the query sees two classes of four: which pair took which table.
""")
var = run(["lattice/07_two_pairs_two_tables+pair-at.ein"], cwd=SCRATCH / "ex")
q = var[var.index("  query"):]
add(f"`ein solve 07_two_pairs_two_tables+pair-at.ein`, the query block\n\n```text\n{q}\n```\n")

add("""## X5 — `lattice/synth/merged_states.ein`: every state merged

*A brings B* and *C brings D*; an obligation needs A or C, and A with C is dead. ∅ owes, so it is not in R and the
minimal line is two states, each reached from one candidate. Every state is reached by exactly two commitments, its
minimal one and that one plus what it brings, so every state gets a `(merged …)`: k_c = 16 against k_s = 8. The
A-side and the C-side never meet, which is the two components.
""")
add(block(["examples/lattice/synth/merged_states.ein"]))

add("""## X6 — `tests/stdlib/closure/03_closed_and_owing.ein`: R empty, alive nodes owe

No candidate, and root owes one `total-owed` instance: R is empty of the second kind. The `owing` rows are the
frontier, the alive owing nodes with no alive node one fact up (here root itself), each with what it owes through the
rule's `:why`. The printed form is `(region (owing (model)))`: it names no fact because the printer closes the
**candidates'** relations, which tell R's states apart, and this program has none.
""")
add(block(["tests/stdlib/closure/03_closed_and_owing.ein"]))
add("""**`--single`.** Expected one state, found none of the owing kind: `Open`.
""")
add(block(["--single", "tests/stdlib/closure/03_closed_and_owing.ein"]))

add("""## X7 — `tests/stdlib/algebra/09_irreflexive_violated.ein`: R empty, every node dead

Root holds `(loops A A)`, which `irreflexive` refutes. Root's core and the rule that refutes it say why, and the form
is `(false)`.
""")
add(block(["tests/stdlib/algebra/09_irreflexive_violated.ein"]))
add("""**`--single`.** Expected one state, found none of the dead kind: `Contradiction`.
""")
add(block(["--single", "tests/stdlib/algebra/09_irreflexive_violated.ein"]))

add("""## X8 — `lattice/04_obligation_trigger_hole.ein`: a hole

A hole with no `absent` in any saturation rule. `total-owed meal is-a` makes every diner owe a meal, and
`seated-dines` derives `(is-a ?g Diner)` from a seat, so the obligation's trigger grows with the commitment: `{(seats
Ann C1)}` owes, between ∅ and `{(meal Ann Soup), (seats Ann C1)}`, both in R. The static check flags the program and
route 2 takes the whole cube, four saturations. R is `{∅, {meal}, {meal, seats}}`, and `{seats}` is a **hole**: outside
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
on it, decided after one saturation though the program is flagged and past the limit (M1f S1f.5.6). Until then it
walked the cube to the limit, certified what describe certifies, and counted the other 20 members met.
""")
add(block(["--minimal", "examples/lattice/06_friends_tables.ein"]))
add("""**`--maximal`** lists no state: the maximal line needs the whole cube, and the row says so, as describe's does.
The 21 members met are counted and in `--dump-answer`.
""")
add(block(["--maximal", "examples/lattice/06_friends_tables.ein"]))

add("""## X10 — `zebra2.ein`: the puzzle

**At the default budget** the state route describes R exhausted. R is one state, 261 982 commitments, 63 minimal
generators, and four of them single facts. R as one `(region …)` is past stdout's 100 lines, so stdout lists the state
and leaves out the `(merged …)`, and says where it is. The query's class is exact, so it is printed with its
`as :expect`.
""")
add(block(["examples/zebra2.ein"]))
add("""**What `--dump-answer DIR` writes to `DIR/answer.txt`** for the same run: the whole `(region …)`, whose
`(merged …)` lists all 63 minimal commitments. That is what pins k_c, and it reads back as the claim.
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
add("""**The first questions** cost what [`solve_interface.md` § 3.4]("""+PLAN+"""/solve_interface.md#34-what-it-costs)
says they do: `-n 1` is root (one saturation), `--single` is `Ambiguity` in two, `--maximal -n 1` is the climb
(eleven). The climb ends at ⊤, so every node is alive (L1) and ⊤ is the whole maximal line, and the run says so.
""")
add(block(["-n", "1", "examples/saturation/square-bwd/floors.ein"]))
add(block(["--single", "examples/saturation/square-bwd/floors.ein"]))
add(block(["--maximal", "-n", "1", "examples/saturation/square-bwd/floors.ein"]))

add("""## X12 — `tests/region/03_merged_or.ein`: a program that states its region

The fixture for `(merged …)` with `(or …)`: the colocation chain, [`region_examples.md`]("""+PLAN+"""/region_examples.md)'s
E2. Its `(solve … :expect (region …))` is checked against the R the run described, and the `(region …)` printed above
the query is that claim's own text: one form, both directions.
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
same in both, so *state {last}* on the screen is *state {last}* in the file. The blind enumerator binds the type
`Color` as an object, and the `types` row says so.
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
""")
add(block(["examples/features/01_not_and_absent.ein"]))

add("""## X16 — `ein-bugs/zebra2-bad.ein`: a dead root's core, its sources and a cut search

`zebra2.ein` with one clue added that contradicts the rest: root's own fixpoint derives `(false)`, so R is empty of the
dead kind. Each fact of root's core is an `unsat core` row, a fact the program states with a `:source` gets a `source`
row, and `refuted by` names the step the core's derivation takes for each fact of the contradiction, `(given)` for one
the core holds.
""")
add(block(["examples/ein-bugs/zebra2-bad.ein"]))
add("""**A core search cut at its budget** (`ein-bugs/wrong-answers/core-search-cut.ein`, W23's witness): the core
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

OUT.write_text("\n".join(S))
shutil.rmtree(SCRATCH)
print(f"wrote {OUT} ({OUT.read_text().count(chr(10))} lines)")
