# `c/` — the plain-C baselines

Three programs that solve the Zebra puzzle `examples/zebra.ein` encodes, with
the same value names and the same answer, and one thing varying between them:
**how much the search is told about the constraints.** They exist to be read
next to `ein solve examples/zebra.ein`, and to put a number on knowledge that
is otherwise invisible because it is spelled as ordinary code.

`../build.sh` builds all three (and the engine). Nothing here is wired into
anything: the gate does not run them, no crate depends on them, and they take
no arguments.

| file | what the search knows | assignments | wall |
|---|---|---:|---:|
| [`zebra_levels.c`](zebra_levels.c) | every clue, and the level at which each becomes testable | **6 840** | 0.003 s |
| [`zebra_oracles.c`](zebra_oracles.c) | that there are fourteen opaque yes/no functions, in the puzzle's order | 25 092 302 520 | 158 s |
| [`blackbox.c`](blackbox.c) + [`zebra_module.c`](zebra_module.c) | a grid size and one function pointer | 25 092 302 520 | 388 s |

"assignments" is the same quantity in all three — permutations written into a
row — so the ratio is not a comparison of two different units. Each program
reports its own.

The first is **3 668 465 times** cheaper than the second, and the difference
between them is not an algorithm. It is one integer per clue.

## What each one knows

**`zebra_levels.c` — everything.** Five arrays indexed by house, one per
attribute, each holding a value of its own five-member enum. The fourteen
stated conditions are an array of function pointers, and each carries the
*level* at which every attribute it names is finally bound. The search assigns
one attribute at a time, runs the clues due at that level, and prunes the whole
subtree on the first failure.

That level tag is the knowledge. It is not in the puzzle — somebody read the
conditions, worked out which attributes each names, and wrote down the `max`.
Its value is the table above, and its *placement* matters as much: the level
order shipped in that file is the best of all 120, against a median of 171 000
assignments and a worst of 2 053 560.

**`zebra_oracles.c` — that there are fourteen of them.** The clues arrive the
way a plugin or a data file would deliver them: an array of `int (*)(void)` in
the order the puzzle states them, conditions (2) through (15), and nothing
else. No dependency set, no level.

A search that does not know which arrays an oracle reads cannot know when to
ask it, and asking early would not fail loudly — the five arrays are always
fully populated, just with a stale permutation from an outer loop, so the
oracle would answer confidently about an assignment nobody is testing. The only
sound call site is the leaf. Two things still survive:

- the **permutation representation**, which is structural rather than
  puzzle-specific — 120 arrangements per attribute instead of 5⁵ = 3 125, so
  120⁵ rather than 3 125⁵. That is a factor of **1.2 × 10⁷** for no knowledge
  of the puzzle at all: "each value appears exactly once" is condition (1)'s
  half-stated companion, and encoding it in the *shape* of the search costs
  nothing to check;
- **short-circuiting** on the first oracle that says no, which is why the array's
  order still matters. The puzzle's own numbering is what a data file would
  hand over and is not a good order.

The order it is given turns out to be a geometric cascade, and the program
prints it — how many assignments each oracle was the first to reject:

```
19 906 560 000  (2)  the Englishman lives in the red house
 3 981 312 000  (3)  the Spaniard owns the dog
   796 262 400  (4)  coffee is drunk in the green house
   161 740 800  (5)  the Ukrainian drinks tea
    29 859 840  (6)  the green house is immediately right of the ivory house
           ...
            31  (15) the Norwegian lives next to the blue house
```

Each condition sees a fifth of what the one before it rejected, because each
is an independent 1-in-5 coincidence over a uniform assignment. That is what
the puzzle's own numbering costs: (15) is consulted 31 times in 25 billion
assignments, and (9) — the unary condition `zebra_levels.c` fires third — is
buried at position eight. A better order exists and nothing here can find it,
because finding it means knowing what the oracles read.

**`blackbox.c` + `zebra_module.c` — a grid size and a function.** The solver
gets `n` cells per row, `m` rows, label tables for printing, and one
`int (*)(const int *grid)` called on complete grids only
([`problem.h`](problem.h)). It cannot count the conditions, name one, or
discover that condition (9) reads a single row.

They are **separate translation units on purpose**: the claim "the search knows
nothing" is checkable rather than stylistic, because `blackbox.c`'s object file
has no symbol from the puzzle in it beyond `PROBLEM`. Inside the module all
fifteen conditions are **one** function — not fourteen behind a table, because
that is what an interface with a single predicate gets you, and it is the shape
a compiled or generated rule set has.

The grid size is dynamic, and that is where the extra 2.5× over
`zebra_oracles` goes: the row loop is a recursion rather than a nest the compiler can unroll,
permutations are generated cell by cell instead of copied from a table, and the
predicate is an indirect call the optimiser cannot see past. It buys the
ability to load a 7×4 problem without recompiling the solver — and the solver is
the part that would not have to change.

## Circular dependencies between levels

There are none, and there cannot be — but not because the puzzle is
well-behaved. **Its constraint graph is K₅ minus two edges: eight edges over
five attributes, four independent cycles.**

```
colour –(2)– nation –(5)– drink –(4)– colour
colour –(8)– smoke  –(14)– nation –(2)– colour
nation –(3)– pet    –(7)– smoke  –(14)– nation
```

The level scheme is indifferent to that for two reasons, and they are worth
separating.

**A level is a `max` over a total order that was chosen independently.** The
levels are a permutation of the five attributes — the *search* order — fixed
before any clue is looked at. A clue then attaches to the last of the
attributes it names. That is total and well-defined for any constraint
hypergraph whatever, because it never asks the constraints what order *they*
would like. A cycle among {colour, nation, drink} simply means all three of its
clues land on whichever of the three the search binds last.

**And clues have no dependencies on each other at all.** They are *tests*, not
rules: each accepts or rejects the current assignment, and none produces a
value another consumes. Circularity needs data flow, and there is none — so
there is nothing between clues for a cycle to form in. What the graph above
draws is a relation between *attributes*, not an evaluation order.

### What the cycles actually cost

Not correctness — the schedule. And specifically:

> **Cycles are the reason the level order had to be measured rather than
> derived.**

If the constraint graph were a **tree**, a depth-first walk of it would be an
optimal level order by construction: root anywhere, and every clue closes at
the level immediately after its parent, because each newly bound attribute has
exactly one edge back into the bound set. No search, no sweep, no table.

With four independent cycles no order can do that. Some clue always has to wait
for the last member of its cycle, which is why `pet` — the last level — closes
four clues at once instead of one, and why the level order is the whole search
strategy rather than a formality. In general this is the **variable-ordering
problem**, whose standard objective (minimum induced width) is NP-hard to
optimise; with five attributes brute force is 120 runs, and
[`zebra_levels.c`](zebra_levels.c) ships the winner.

### Where a cycle *would* bite

Propagation. The moment a clue is allowed to *narrow a domain* rather than
merely reject an assignment — condition (8) telling the smoke row "Kools is in
house *k*" once colour is bound — clues start consuming each other's output,
and a cycle means one pass is not enough: you have to iterate to a fixpoint.
That is AC-3's worklist, and it is `ein`'s saturator.

None of the three programs here propagates, so none of them has a fixpoint to
reach, and that is exactly why the question has no teeth for them. It is the
same boundary as the next section.

## What none of them do

No propagation, no forward checking, no domain narrowing, no learning. A
rejected subtree teaches the next one nothing. Once the colour row is bound,
condition (8) says "Kools is in house *k*" for a known *k*, which would cut the
smoke row's 120 permutations to 24 **before** enumerating them — none of these
three can act on that, because none has a domain to narrow. That is the line
between a baseline and a solver, and it is where `ein` starts rather than ends:
a dead commitment there writes a no-good clause that prunes somewhere else
entirely.

And all three are still the puzzle. Every condition is compiled code that only
answers this one; nothing derives a new condition, nothing explains why a house
got its colour, and a sixteenth condition is an edit and a rebuild.

## Running them

```sh
./build.sh                 # + the engine; --c for just these
build/zebra-levels         # 3 ms
build/zebra-oracles        # ~2.5 minutes, prints progress to stderr
build/zebra-blackbox       # ~6.5 minutes, prints progress to stderr
```

Each prints the solved grid, what the enumeration cost, and then the two
answers — and each checks that it found **exactly one** solution and that the
answer is the known one, so all three double as their own smoke test and exit
non-zero if either moves.

Numbers above were taken on one P-core of an i9-14900HX at `-O2`; they are
there for the ratios, not as a benchmark.
