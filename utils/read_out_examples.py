#!/usr/bin/env python3
"""Write `read_out_examples.md`, the read-out in real runs — M1e2 T1e2.1a.5.2.

Every code block on the page is one `ein solve` run, as the binary prints it:
stdout, then each stderr line as `stderr: …`, then `exit N`. A block captioned
`--dump-answer DIR` is the `answer.txt` that run writes. The prose between
blocks is this script's. `ein-cli/tests/read_out_examples.rs` runs every
caption again and compares the block, so the page cannot drift from the
binary; re-write it with

    python3 utils/read_out_examples.py [--bin ein.rs/target/release/ein]

Until S1e2.1a.5's switch a bare `ein solve` was the walk, so a run that named
no region flag was given `--region-budget 524288`, the default budget; the
switch dropped that and the page was written again.

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
OUT = ROOT / "plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out_examples.md"

ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
ap.add_argument("--bin", default=os.environ.get("EIN_BIN", str(ROOT / "ein.rs/target/release/ein")))
BIN = ap.parse_args().bin
SCRATCH = Path(tempfile.mkdtemp(prefix="read-out-examples-"))
OLD = OUT.read_text() if OUT.exists() else ""

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


def run(args, cwd=ROOT):
    """The block a run prints: stdout (or `answer.txt`), stderr, exit."""
    a = argv(args)
    dump = None
    if "DIR" in a:
        dump = Path(tempfile.mkdtemp(prefix="answer-", dir=SCRATCH))
        a = [str(dump) if x == "DIR" else x for x in a]
    p = subprocess.run([BIN, "solve", *a], cwd=cwd, capture_output=True, text=True)
    stdout = p.stdout
    if dump is not None:
        stdout = (dump / "answer.txt").read_text().replace(str(dump), "DIR")
    body = stdout.rstrip("\n")
    if p.stderr.strip():
        body += "\n\n" + "\n".join("stderr: " + l for l in p.stderr.splitlines())
    return body + f"\n\nexit {p.returncode}"


def block(args, cwd=ROOT, note=""):
    shown = " ".join(
        x for x in argv(args) if x not in ("--jobs", "8")
    )
    text = run(args, cwd)
    cap = f"`ein solve {shown}`" + (f" — {note}" if note else "")
    return f"{cap}\n\n```text\n{text}\n```\n"


def kept(caption):
    """A block the binary cannot print yet, kept as the page has it."""
    at = OLD.index(caption)
    start = OLD.index("```text\n", at) + len("```text\n")
    return OLD[start:OLD.index("\n```", start)]


S = []
add = S.append

add("""# The read-out in real runs — the examples for T1e2.1a.5.1

**Stage:** [S1e2.1a.5](s1e2.1a.5_the_read_out.md), tasks T1e2.1a.5.1 and
T1e2.1a.5.2. The page diff these illustrate is [`read_out.md`](read_out.md).
**Status:** written 2026-10-16 for the user to read with the page diff,
re-generated the same day after the user's three rulings
([`read_out.md` § 11](read_out.md#11-the-users-rulings-2026-10-16)), and
**since T1e2.1a.5.2 printed by `ein solve` itself.**
**How it is made:** every block below is one run of the binary, written by
[`utils/read_out_examples.py`](../../../utils/read_out_examples.py): stdout,
then each stderr line as `stderr: …`, then the exit code. A block captioned
`--dump-answer DIR` is the `DIR/answer.txt` that run writes: the read-out with
no listing cut. Until the switch (T1e2.1a.5.3) a bare `ein solve` was the
walk, so a run that named no region flag was given `--region-budget 524288`,
the default budget; since the switch a bare `ein solve` is the description.
[`ein-cli/tests/read_out_examples.rs`](../../../ein.rs/crates/ein-cli/tests/read_out_examples.rs)
runs every caption again and compares the block, so the page is the read-out's
golden. One block is not one whole run: X4's scratch variant, whose query block
is quoted.

""")
add("""---

## Contents

| | program | n | what it shows |
|---|---|---:|---|
| [X1](#x1--branching05_mini_zebraein-everything-at-once) | `branching/05_mini_zebra.ein` | 9 | the default read-out, every row; `(merged …)` with `(or …)`; a query with an undetermined class |
| [X2](#x2--lattice02_genuine_3set_deathein-one-program-every-mode) | `lattice/02_genuine_3set_death.ein` | 3 | one program under every mode: describe, `--maximal`, `--minimal`, `-n 2`, `--single`, `--maximal --single`, `--count` |
| [X3](#x3--features11_expect_ambiguityein-a-query-over-the-maximal-line) | `features/11_expect_ambiguity.ein` | 4 | a query `:over maximal`: the walk's only `(or …)` claim, migrated, and holding |
| [X4](#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes) | `lattice/07_two_pairs_two_tables.ein` | 16 | the user's two tables: 8 components, and 2 projection classes of 4 |
| [X5](#x5--latticesynthmerged_statesein-every-state-merged) | `lattice/synth/merged_states.ein` | 5 | ∅ not in R; a `(merged …)` on every state; two components |
| [X6](#x6--testsstdlibclosure03_closed_and_owingein-r-empty-alive-nodes-owe) | `tests/stdlib/closure/03_closed_and_owing.ein` | 0 | empty R, the owing kind; `--single` says `Open` |
| [X7](#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) | `tests/stdlib/algebra/09_irreflexive_violated.ein` | 0 | empty R, the dead kind; `--single` says `Contradiction` |
| [X8](#x8--ein-bugsnaf-upward-closureein-a-hole) | `ein-bugs/naf-upward-closure.ein` | 2 | a program the static check flags: route 2, a hole, the warning |
| [X9](#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) | `lattice/06_friends_tables.ein` | 19 | flagged and past `--region-limit`: members, never line members |
| [X10](#x10--zebra2ein-the-puzzle) | `zebra2.ein` | 56 | the puzzle: exhausted at the default budget by the state route, cut on stdout and whole in `answer.txt`; `--maximal -n 1`; `--single` |
| [X11](#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) | `saturation/square-bwd/floors.ein` | 20 | R is the whole cube: decided, its states counted in part; the first questions in 1–11 saturations |
| [X12](#x12--testsregion03_merged_orein-a-program-that-states-its-region) | `tests/region/03_merged_or.ein` | 3 | a program that states its region: the printed region is its claim |
| [X13](#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines) | `saturation/type-exclusivity/colors.ein` | 8 | an exhausted R past stdout's 100 lines: what fits, and where the rest is |
""")

add("""## X1 — `branching/05_mini_zebra.ein`: everything at once

[`region_examples.md`](region_examples.md)'s E1. Two people, two drinks and two
pets; the clues put Alice with the tea and the cat, and Bob is open. Every row
of the default read-out is here. ∅ is in R because the program states no
obligation, so the minimal line is the trivial {∅}. The maximal state is
reached by four commitments, three of them minimal, and that is the one
`(merged …)`. The query asks who drinks the coffee and which pet they keep: on
four of the five states the goal has no row, and that class is `(model)`.
""")
add(block(["examples/branching/05_mini_zebra.ein"]))

add("""## X2 — `lattice/02_genuine_3set_death.ein`: one program, every mode

[`solution_semantics.md` § 2.5](../../../docs/kernel/inference/solution_semantics.md#25-worked-exampleslattice02_genuine_3set_deathein)'s
worked example: three candidates, every pair alive, the triple dead, no
obligation. Seven runs, one per mode, so the same R can be read through each
question.

**Describe.** Seven states, the three pairs on the maximal line, three
sub-cubes of dimension 2.
""")
add(block(["examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--maximal`.** The line is listed as states, not as a `(region …)`: a line
is not a claim the language can state (only a query's `:over` names one), and a
`(region …)` read back would say that these three are all of R.
""")
add(block(["--maximal", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--minimal`.** ∅ is in R, so the line is {∅}: one saturation.
""")
add(block(["--minimal", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`-n 2`.** The first two states in the route's order, the same two under
every lever (C4). Nothing is said about the rest, and the exit code is 0: a
request, not an expectation.
""")
add(block(["-n", "2", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--single`.** The run's own expectation, so the verdict word is printed:
`Ambiguity` after two saturations. It exits 0, as every answer does: the
word is the answer, and an answer is a successful run.
""")
add(block(["--single", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--maximal --single`.** The question a restrictions-only program means by
*unique*: the maximal line has three states, so `Ambiguity` again, with two of
them.
""")
add(block(["--maximal", "--single", "examples/lattice/02_genuine_3set_death.ein"]))
add("""**`--count`.** *How many?*, and nothing else (M1e2 T1e2.1c.4.6, the user's
ruling of 2026-11-03): describe's route and its two counts, with the rows that
qualify them, `exhausted`, `∅`, `lattice`, `universe`, `route` and `work`, each
describe's text. No state is read, so nothing is listed and the query is not
answered. Over all of R only: beside `--minimal` or `--maximal` it is a usage
error that names a bare `ein solve`.
""")
add(block(["--count", "examples/lattice/02_genuine_3set_death.ein"]))

add("""## X3 — `features/11_expect_ambiguity.ein`: a query over the maximal line

[`region_examples.md`](region_examples.md)'s E5. Two people, two seats,
restrictions only. The walk's claim, the corpus's only `(or …)`, listed **R's
maximal line**: two of R's seven states. Over all of R the goal `(seat ?who
?slot)` would see every seat fact, so each state would be its own class and
`∅` the undetermined one. The switch migrated the query to `:over maximal`, as
[`region_language.md` § 7](region_language.md#7-the-migration-named) says,
so its answer is exactly the walk's claim, word for word, and it holds.
""")
add(block(["examples/features/11_expect_ambiguity.ein"]))

add("""## X4 — `lattice/07_two_pairs_two_tables.ein`: components and projection classes

The user's example of 2026-10-16 (ruling (i)): two pairs of friends, two tables
of two seats, friends sit together. R is the eight full seatings. Each is its
own **component**, because moving one friend passes through a node that is dead
or owing, and minimal and maximal lines coincide. The goal `(sits ?p ?s)` names
the seats, so each state is its own projection class.
""")
add(block(["examples/lattice/07_two_pairs_two_tables.ein"]))
add("""The user's *"query can ask e.g. only what table selected, and then seats will
be hidden information"* needs a goal that does not name the seat. A projection
class is the goal's **atoms** under its bindings (ruled at S1e2.1a.4), so the
seat is hidden only by a relation that does not carry it. The program below is
07 with one relation and one rule added, and it is **not a corpus file**:

```lisp
(relation pair-at Group Table)

(rule pair-at ()
  :match  (and (member ?a ?g) (sits ?a ?s) (at ?s ?t))
  :assert (pair-at ?g ?t))

(query :goal (pair-at ?g ?t))
```

R does not move (the rule derives no candidate), and the query now sees two
classes of four: which pair took which table.
""")
var = run(["lattice/07_two_pairs_two_tables+pair-at.ein"], cwd=SCRATCH / "ex")
q = var[var.index("  query"):]
add(f"`ein solve 07_two_pairs_two_tables+pair-at.ein`, the query block\n\n```text\n{q}\n```\n")

add("""## X5 — `lattice/synth/merged_states.ein`: every state merged

*A brings B* and *C brings D*; an obligation needs A or C, and A with C is dead.
∅ owes, so it is not in R and the minimal line is two states, each reached from
one candidate. Every state is reached by exactly two commitments, its minimal
one and that one plus what it brings, so every state gets a `(merged …)`:
k_c = 16 against k_s = 8. The A-side and the C-side never meet, which is the two
components.
""")
add(block(["examples/lattice/synth/merged_states.ein"]))

add("""## X6 — `tests/stdlib/closure/03_closed_and_owing.ein`: R empty, alive nodes owe

No candidate, and root owes one `total-owed` instance: R is empty of the
second kind. The `owing` rows are the frontier, the alive owing nodes with no
alive node one fact up (here root itself), each with what it owes through the
rule's `:why`. The printed form is `(region (owing (model)))`. It names no fact
because the printer closes the **candidates'** relations, which tell R's states
apart, and this program has none. The fixture's own migrated claim pins
`(closed-here r)` as well, which a claim may: naming a relation closes it, and
naming more is allowed.
""")
add(block(["tests/stdlib/closure/03_closed_and_owing.ein"]))
add("""**`--single`.** Expected one state, found none of the owing kind: `Open`.
""")
add(block(["--single", "tests/stdlib/closure/03_closed_and_owing.ein"]))

add("""## X7 — `tests/stdlib/algebra/09_irreflexive_violated.ein`: R empty, every node dead

Root holds `(loops A A)`, which `irreflexive` refutes. Root's core and the rule
that refutes it say why, and the form is `(false)`.
""")
add(block(["tests/stdlib/algebra/09_irreflexive_violated.ein"]))
add("""**`--single`.** Expected one state, found none of the dead kind:
`Contradiction`.
""")
add(block(["--single", "tests/stdlib/algebra/09_irreflexive_violated.ein"]))

add("""## X8 — `ein-bugs/naf-upward-closure.ein`: a hole

W3's witness. A guard's `absent` makes `{(p A)}` dead alone and alive beside
`(q A)`, so the static check flags the program and route 2 takes the whole cube,
four saturations. R is `{∅, {q}, {p, q}}`, and `{p}` is a **hole**: outside R,
between two members of it. The `lattice` block is the detector's warning (M1e2
S1e2.1b.5, the wording approved on 2026-11-03), which replaced S1e2.1a.1's
stub: what is assumed, the site and what it puts at stake, the status the cube
shows (exercised, by `bad`), the state (no stale fact), the parent paths
checked, and then each minimal hole with why it is out. The `E` row is the
cube's alone.
""")
add(block(["examples/ein-bugs/naf-upward-closure.ein"]))

add("""## X9 — `lattice/06_friends_tables.ein`: flagged, and past the limit

The everyone-sits twin's sibling, where *a group that does not fit goes
elsewhere*: the obligation's trigger grows with the commitment, so the program
is not monotone (the third answer). n = 19 is past `--region-limit 16`, so no
route can describe R. The run visits the cube in layer order up to 2¹⁶ nodes.
It meets 21 members, which is all of R by the third answer's count, and it
cannot know that, so every count is a lower bound. What it *can* certify is
one line: ∅ is in R, so the minimal line is exactly {∅}, whatever the program.
That is the one state it lists. The other 20 members it met go to
`--dump-answer`, and the query is answered over all 21 as a lower bound, with
no `as :expect`: read back, that claim would say these classes are all. The
warning says the cube's half is not known past the limit, reads the stale
facts of every node the run computed, and checks no parent path, because the
read-out prints no state of two commitments or more.
""")
add(block(["examples/lattice/06_friends_tables.ein"]))
add("""**`--count`** on the same program: the counts are describe's lower bounds,
for the same reason, and the warning is printed whole, since it qualifies them.
Its `paths` row says the spot check was not run: `--count` reads no state.
""")
add(block(["--count", "examples/lattice/06_friends_tables.ein"]))

add("""## X10 — `zebra2.ein`: the puzzle

**At the default budget** the state route describes R exhausted, since
S1e2.1c.2: 374 saturations and 42 705 facts, where route 1 stopped before its
layer 5 with the puzzle answered and nothing certified past it. R is one
state, 261 982 commitments, 63 minimal generators, and four of them single
facts. R as one `(region …)` is 111 lines, past stdout's 100, so stdout lists
the state and leaves out the `(merged …)`, and says where it is. The query's
class is exact, so it is printed with its `as :expect`.
""")
add(block(["examples/zebra2.ein"]))
add("""**What `--dump-answer DIR` writes to `DIR/answer.txt`** for the same run: the
whole `(region …)`, whose `(merged …)` lists all 63 minimal commitments. That
is what pins k_c, and it reads back as the claim.
""")
add(block(["--uncut", "examples/zebra2.ein"], note="`DIR/answer.txt`"))
add("""**`--maximal -n 1`**: a complete solution, 42 saturations: the obligation
tree's first leaf, then the climb (M1e2 S1e2.1c.4). It was 54 on route 1.
""")
add(block(["--maximal", "-n", "1", "examples/zebra2.ein"]))
add("""**`--single`**: `Solution`, 45 saturations, at the default budget: the
obligation tree, whose one leaf has every extension dead (L4, L5). It was
8 616 on route 1 until S1e2.1c.4.
""")
add(block(["--single", "examples/zebra2.ein"]))

add("""## X11 — `saturation/square-bwd/floors.ein`: a region that is the whole cube

No restriction and no obligation: every commitment is alive and in R, 2²⁰ of
them. **Describe** stops at the default budget, partial, and R is still
decided: ∅ is in R, and one saturation of ⊤ finds it alive and owing nothing,
so R is the whole cube (L1, L2). k_c, both lines, the one component, the one
sub-cube and N are exact. What is not is the states: 27 559 of them, and the
state route counts those of up to eight candidates before the default budget of
2²⁰ facts (ruled 2026-11-03; at 2²¹ it counts them all). The two
line states are what it lists. The other members it met go to
`--dump-answer`, not to the screen, and the query's one class is printed
without `as :expect`, since it is a lower bound.
""")
add(block(["examples/saturation/square-bwd/floors.ein"]))
add("""**The first questions** cost what [`solve_interface.md` § 3.4](solve_interface.md#34-what-it-costs)
said they would: `-n 1` is root (one saturation), `--single` is `Ambiguity`
in two, `--maximal -n 1` is the climb (eleven). The climb ends at ⊤, so every
node is alive (L1) and ⊤ is the whole maximal line: since M1e2 S1e2.1c.4 the
run says so, where it said *as asked*.
""")
add(block(["-n", "1", "examples/saturation/square-bwd/floors.ein"]))
add(block(["--single", "examples/saturation/square-bwd/floors.ein"]))
add(block(["--maximal", "-n", "1", "examples/saturation/square-bwd/floors.ein"]))

add("""## X12 — `tests/region/03_merged_or.ein`: a program that states its region

S1e2.1a.4's fixture for `(merged …)` with `(or …)`: the colocation chain,
[`region_examples.md`](region_examples.md)'s E2. Its `(solve … :expect (region …))`
is checked against the R the run described, and the `(region …)` printed above
the query is that claim's own text: one form, both directions.
""")
add(block(["tests/region/03_merged_or.ein"]))
add("""**`--count`** on the same program checks no claim, since it reads no state, and
says so in one `claims` row, with a `note:` on stderr per claim. It exits 0: a
claim not checked is not refuted.
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

Two colours, one rule that keeps them from co-locating, and no obligation: the
universe is eight `is-a` and `co-located` facts, {dead} minimal dead sets keep
R from being the cube, and R is {k_s} states. As one `(region …)` it is
{region_lines} lines. The user's ruling: **stdout's listings share 100
lines**, each printing whole items while they last and at least one. The
region comes first and takes them: stdout lists states 1–{last} outside a
`(region …)`, since a cut wrapped in one would claim to be all of R, then says
what it left out and that `--dump-answer DIR` writes the whole `(region …)` to
`DIR/answer.txt`. The query still gets its `over` row, its one class and its
one-line `as :expect`. The summary rows are never cut. Stdout is
{stdout_lines} lines, and `answer.txt` is {file_lines}. The state numbers are
the same in both, so *state {last}* on the screen is *state {last}* in the
file.
""")
add(block(colors))

OUT.write_text("\n".join(S))
shutil.rmtree(SCRATCH)
print(f"wrote {OUT} ({OUT.read_text().count(chr(10))} lines)")
