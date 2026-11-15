# S1f.10.2 — From the exclusion graph to *pick at most one*

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** **stopped at its control, 2026-11-15**: [§ Record](#record-2026-11-15-stopped-at-the-control). On I-Z2 no
declared row or column is a maximal clique of root's exclusion graph, and no cover was built. **To be run again
after S1f.10.10, under [§ Defaults taken](#defaults-taken)'s third entry**: a group is a maximal clique of the
theory's exclusions.
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md), the excluded pairs; and, since the control,
[S1f.10.10](s1f.10.10_the_theorys_share.md), the theory's exclusions over every pair of H.
**Read by:** [S1f.10.4](s1f.10.4_bounded_groups.md), [S1f.10.11](s1f.10.11_coupling.md),
[S1f.10.12](s1f.10.12_the_structure_shown.md), and of track R [S1f.10.5](s1f.10.5_ordering.md) and
[S1f.10.14](s1f.10.14_exactly_one_spent.md).

## Context

The instruction says the hypothesis set *"can be split into groups of hypothesis to pick one from (who lives in
House2, where is tee drinked, etc.)"*. That is the right object, and **split** is the word the corpus will not
grant. Three candidate definitions of a group over the exclusion graph `G`:

| definition | *pick at most one* holds? | on `zebra2`'s `nation-loc` |
|---|---|---|
| **connected component** of `G` | **no**: two vertices of a component need not be adjacent | one component swallowing all five `*-loc` relations, because the clues link them |
| **maximal clique** of `G` | **yes**, by construction | the houses left to one nationality are a clique, and the nationalities left to one house another |
| **class of a declared key** | yes, but only where the program declares it | `(bijective nation-loc)`, which I-Z2 has and I-B06 does not |

Cliques are the only one that makes the phrase true, and their price is that they **overlap**:
`(color-loc Red House-3)` is in *where is Red?* and in *what colour is House-3?*. Every member of a
bijection's candidate set is in two, the rows and the columns of one grid. So a group is a set, a candidate is
in several, and what the engine holds is a **cover** of H, not a partition.

**At root the grids are ragged.** The givens and the clues have already decided some candidates, so `zebra2`'s
H is 56 candidates of the 125 its five relations span, and a row is the houses *still open* to a value. The
definition is over H as root builds it, and the stage says so wherever it prints a size.

## Tasks

1. **T1f.10.2.1 — the three definitions against the corpus.** Components, maximal cliques and declared-key
   classes for every program S1f.10.1 read; where they differ, the program is named. Expected, each a row of
   the record: components ≠ cliques on every puzzle whose clues link attributes; cliques = declared classes on
   I-Z2, **which is the control**: if they disagree, S1f.10.1's pairs are wrong and the stage stops; cliques
   where nothing is declared on I-B06.
2. **T1f.10.2.2 — which cliques, in what order.** All maximal ones is the complete answer and can be
   exponential in a dense graph. A bijection's graph is a rook's graph, whose maximal cliques are its rows and
   columns; nothing guarantees every corpus program's graph is so benign, and the puzzle's own exclusions add
   edges across grids. **Bound the enumeration and print what the bound cut.** The order is canonical: a group
   is its members in rendered-text order, and groups are ordered by that, so the cover is a function of the
   program and not of the enumeration.
3. **T1f.10.2.3 — beside what the program declares.** For each program, the owed instances root holds with
   their alternatives (`oblgen::Alternatives`, licensed or not), and the relations its hrules propose and its
   closures take out (`mechanisms.rs`). Three columns of the record: a discovered group that is an owed
   instance's alternatives; a discovered group nothing owes; an owed instance whose alternatives are **not**
   pairwise excluding. S1f.10.4 starts from the three.
4. **T1f.10.2.4 — the owner.** The pairs and the cover live in one module of `ein-infer`, computed once per
   program from root's nodes, which the census, the bundle and, if track R goes ahead, the routes read. The
   stage names the module and gives it the two maps a reader needs, group to members and member to groups. It
   takes the program's prepared space, not a `&Kb` of some node: a representation that takes a node invites the
   next reader to recompute it per fork.

## Acceptance

- The definition of **group** is written in this file, with the corpus program that falsifies each rejected
  alternative named.
- The cover is computed for every program S1f.10.1 read and recorded beside its census: the number of groups,
  their sizes, how many groups a candidate is in (the most and the mean), and the candidates in **no** group.
- **The in-no-group count is the honest column.** A candidate nothing excludes is one no group says anything
  about, and I-L02's three are exactly that.
- **On I-Z2 the cover is the rows and the columns of the five declared bijections**, over the candidates root
  leaves, plus whatever the clues add, each addition listed.
- The enumeration is deterministic and its order pinned by a test: the cover of a program is the same under a
  permutation of the interner's ids.
- No answer, route or surface changes.

## Record (2026-11-15): stopped at the control

The stage's control came first and failed, so the stage stopped there, as planned (*if they disagree … the stage
stops*). Nothing below is a cover: it is the control, read off [S1f.10.1](s1f.10.1_exclusion_census.md)'s lists,
with what it says of each definition in [§ Context](#context).

### What was run

The pair layer's lists for every corpus program, re-taken by S1f.10.1's example into a directory of its own, a
probe beside them, and the maximal cliques of each exclusion graph by a script that is quoted whole below. **A
vertex is a candidate alive alone and an edge an excluded pair** ([§ Defaults taken](#defaults-taken), 1).
2026-11-15, the working tree over `706f9b47`; every number is a count and none is a time.

```sh
mkdir -p build/structure/control
# The probe: zebra2.ein with its conditions (2) to (15) deleted, 14 lines, and nothing else. Rules, ontology,
# the five `(bijective …)` declarations and the row of houses stay.
grep -v ':source "condition (\([2-9]\|1[0-5]\))"' examples/zebra2.ein > build/structure/control/zebra2_no_clues.ein
cargo run --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer -- \
    --jobs 8 --out build/structure/control/out > build/structure/control/census.jsonl
cargo run --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer -- \
    --out build/structure/control/out build/structure/control/zebra2_no_clues.ein >> build/structure/control/census.jsonl
python3 - <<'EOF'
import itertools, json, sys
from collections import Counter
sys.setrecursionlimit(100000)
OUT = 'build/structure/control/out/'
def cliques(adj):                       # Bron-Kerbosch with a pivot: every maximal clique, once
    out = []
    def go(R, P, X):
        if not P and not X: out.append(frozenset(R)); return
        pivot = max(P | X, key=lambda v: len(adj[v] & P))
        for v in sorted(P - adj[pivot]):
            go(R | {v}, P & adj[v], X & adj[v]); P = P - {v}; X = X | {v}
    if adj: go(set(), set(adj), set())
    for c in out: assert all(b in adj[a] for a, b in itertools.combinations(c, 2)) and not set.intersection(*(adj[a] for a in c))
    return out
def row(name, verts, pairs):
    adj = {v: set() for v in verts}
    for a, b in pairs:
        if a in adj and b in adj: adj[a].add(b); adj[b].add(a)
    cs = cliques({v: s for v, s in adj.items() if s}); per = Counter(v for c in cs for v in c)
    lines = {}                           # (rel x y): the candidates of one relation that share x, or share y
    for v in verts:
        r, x, y = v.strip('()').split(); lines.setdefault((r, 0, x), set()).add(v); lines.setdefault((r, 1, y), set()).add(v)
    two = [frozenset(m) for m in lines.values() if len(m) > 1]
    excl = [m for m in two if all(b in adj[a] for a, b in itertools.combinations(m, 2))]
    inside = [sum(m < c for c in cs) for m in excl if m not in cs]
    print(f'| {name} | {len(verts)} | {sum(map(len, adj.values())) // 2} | {len(cs)} | {min(map(len, cs), default=0)} to {max(map(len, cs), default=0)} | '
          f'{max(per.values(), default=0)} | {sum(per.values()) / max(1, len(verts)):.2f} | {len(verts) - len(per)} | {len(lines) - len(two)} | {len(two)} | '
          f'{len(excl)} | {sum(m in cs for m in two)} | {len(inside)}' + (f' ({min(inside)} to {max(inside)})' if inside else '') + f' | {sum(not any(m <= c for m in two) for c in cs)} |')
def lists(p): return json.load(open(OUT + p + '/pairs.json'))
def alive(d): return [c for c in d['candidates'] if c not in set(d['dead_alone'])]
for tag, p in [('I-Z1', 'examples/zebra'), ('I-Z2', 'examples/zebra2'), ('I-Z2M', 'examples/zebra2-minus-15'), ('I-B06', 'examples/branching/06_lookahead_on'),
               ('I-SEND', 'examples/beyond_arith/send_more_money'), ('the probe', 'build/structure/control/zebra2_no_clues')]:
    d = lists(p); row(tag, alive(d), d['excluded'])
z2, probe = lists('examples/zebra2'), lists('build/structure/control/zebra2_no_clues')
row("the probe's pairs on I-Z2's alive alone", alive(z2), probe['excluded'])
row("the probe's pairs on all of I-Z2's H", z2['candidates'], probe['excluded'])
EOF
```

The script reads a *row or column* off a candidate's text, `(rel x y)`: the candidates of one relation that share
`x`, or share `y`. That is the reading by name this stage's cover may not use, and here it is the thing the cover
is held to. A fuller copy, `build/structure/control/control.py`, not checked in, holds a second enumerator to this
one on every graph of 60 vertices or fewer and prints the per-program figures quoted under
[§ The three definitions](#the-three-definitions-as-far-as-the-control-read-them); its output is
`build/structure/control/control.txt`.

### The control

| | vertices | edges | maximal cliques | their sizes | most a candidate is in | mean | in none | rows and columns of one member | of two or more | pairwise excluding | **a maximal clique** | strictly inside one (inside how many) | maximal cliques holding no whole row or column |
|---|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---|---:|
| I-Z1 | 38 | 414 | 1486 | 3 to 7 | 1350 | 247.89 | 0 | 4 | 19 | 15 | 0 | 15 (2 to 34) | 1348 |
| **I-Z2** | 35 | 327 | **902** | 3 to 6 | 830 | 139.31 | 0 | 11 | 25 | 25 | **0** | 25 (2 to 63) | 689 |
| I-Z2M | 96 | 1428 | 3862 | 2 to 8 | 909 | 191.40 | 0 | 0 | 46 | 46 | **46** | 0 | 3816 |
| I-B06 | 2 | 0 | 0 | none | 0 | 0 | 2 | 4 | 0 | 0 | 0 | 0 | 0 |
| I-SEND | 78 | 609 | 18 | 6 to 10 | 2 | 2.00 | 0 | 0 | 18 | 18 | **18** | 0 | 0 |
| the probe | 125 | 500 | 50 | 5 | 2 | 2.00 | 0 | 0 | 50 | 50 | **50** | 0 | 0 |
| the probe's pairs on I-Z2's 35 alive alone | 35 | 45 | 25 | 2 to 4 | 2 | 1.69 | 2 | 11 | 25 | 25 | **25** | 0 | 0 |
| the probe's pairs on all 56 of I-Z2's H | 56 | 128 | 36 | 2 to 4 | 2 | 2.00 | 0 | 0 | 36 | 36 | **36** | 0 | 0 |

I-L02 has three vertices and no edge: no clique and three candidates in no group, as planned. I-Z1's one
relation has no declared grid: the 19 are its candidates by name (14, each pairwise excluding) and by house (5,
of which the one of two members is pairwise excluding and the four larger are not, since two attributes share a
house).

**On I-Z2 the maximal cliques are not the rows and columns.** Over the 35 candidates alive alone the five
relations have 36 rows and columns, 11 of one member. Each of the other 25 is pairwise excluding, so *pick at
most one* holds of it and S1f.10.1's pairs are not what is wrong; **none of the 25 is maximal**. Each is strictly
inside 2 to 63 maximal cliques, 689 of the 902 hold no whole row or column, and a candidate is in 139 groups on
average and one candidate in 830. Read as *the rows and columns, plus what the clues add*, the acceptance would
be met by a list of 902 in which no row and no column appears.

One of them whole. `color-loc` at `House-3` is `(color-loc Ivory House-3)` and `(color-loc Red House-3)`. The four
maximal cliques that hold both add to the two `(pet-loc Zebra House-4)` and two of `(pet-loc Snail House-4)`,
`(pet-loc Snail House-5)`, `(smoke-loc Old_Gold House-4)`, `(smoke-loc Old_Gold House-5)`, one of each house. Each
of those five is alive alone and dead with either colour. Root has decided so much of the puzzle that a
commitment to one candidate brings others with it: `(color-loc Ivory House-3)` implies eleven candidates and
`(color-loc Red House-3)` four.

**Why here and not on I-Z2M or I-SEND.** At root an exclusion a bijection makes and one the clues make are both
a dead pair, and the layer cannot tell them apart. A row is maximal while no candidate outside it excludes all of
its members. On I-Z2M (one clue fewer, no candidate dead alone, rows of 2 to 5) none does and all 46 are maximal,
among 3 816 other cliques; on I-SEND the 18 cliques are exactly the eight letters and the ten digits. On I-Z2 the
rows are 2 to 4 long and every one has such a candidate. So the plan's premise, *a bijection's graph is a rook's
graph, whose maximal cliques are its rows and columns*, holds of the bijection's own edges and fails of root's
exclusion graph on the control instance.

**The probe.** `zebra2.ein` with conditions (2) to (15) deleted leaves all 125 candidates alive alone and
excludes 500 pairs, which are the 500 row and column pairs and no other; its maximal cliques are the 50 rows and
columns, each of five, every candidate in exactly two. Those pairs induced on I-Z2's 35 candidates alive alone
give 25 maximal cliques, the 25 rows and columns of two or more, and two candidates in none
(`(drink-loc Tea House-2)`, `(nation-loc Ukrainian House-2)`, each alone in its row and its column). Induced on
all 56 of I-Z2's H they give 36, **every row and every column of H, every candidate in exactly two**, which is the
phase's acceptance as first written, *over the candidates root leaves*. The probe is a program edited by hand,
not a definition: what *rules, ontology and the two facts* is in general is
[S1f.10.10](s1f.10.10_the_theorys_share.md)'s, and the phase's Risks say why that is not obvious.

### The three definitions, as far as the control read them

T1f.10.2.1's expected rows against what was measured, over the 44 corpus programs with an excluded pair
(`build/structure/control/control.py`'s last table):

| the plan expected | measured |
|---|---|
| components ≠ cliques on every puzzle whose clues link attributes | **holds.** The components are the maximal cliques on 19 of the 44 programs and not on 25. Every instance with an edge is one component: I-Z1's 38 vertices, I-Z2's 35, I-Z2M's 96, I-SEND's 78. I-Z2's holds `(color-loc Green House-4)` and `(drink-loc Coffee House-4)`, which are equivalent and so alive together: the program that falsifies *a group is a component* |
| cliques = declared classes on I-Z2, the control | **fails**, above: 0 of 25 |
| cliques where nothing is declared on I-B06 | **fails, vacuously.** Two candidates alive alone, `(co-located Blue H5)` and `(co-located H5 Blue)`, one class, no excluded pair, no clique; 130 dead alone |
| the enumeration can be exponential | not on the corpus: the most is I-Z2M's 3 862 cliques from 1 428 edges. A candidate is in more than two cliques on 13 of the 44 programs, and in hundreds on the zebra programs whose root leaves a grid (I-Z1, I-Z2, I-Z2M and the `-obligations` twins of the last two) |

Not read: the declared side, T1f.10.2.3's `oblgen::Alternatives` and `mechanisms.rs`. The rows and columns above
are read off the candidates' texts, for the control alone.

### What is to be ruled

What a **group** is. The control leaves three readings, each with its numbers above, and this stage takes none:

1. **A maximal clique of root's exclusion graph**, as [§ Context](#context) has it. Then the control is a
   containment and not an identity (25 of 25 rows and columns inside a group, none a group), I-Z2's cover is 902
   groups, and *who lives in House-5* is not one of them.
2. **A maximal clique of the exclusions the theory makes alone**, which is the instruction's own wording (*taking
   rules, ontology and this 2 facts*) and needs S1f.10.10's object before this stage's. The probe is the one
   measurement of it. Under it a candidate dead alone is in the groups of its row and its column, so the first
   default below would not be needed.
3. **A maximal clique over single states**, each equivalence class one vertex: 16 cliques over 14 states on
   I-Z2, 34 over 17 on I-Z1, 425 over 61 on I-Z2M. Smaller, and no more the rows and columns than the first.

### What was not built

The cover in `ein_infer::structure`, its bound, its order and its two maps (T1f.10.2.2, .4); the declared columns
(T1f.10.2.3); the cover beside the census; the id-permutation test. `structure.rs`, the `pair_layer` example,
[`implementation.md`](../../../docs/kernel/inference/implementation.md) and
[`utils/README.md`](../../../utils/README.md) are as S1f.10.1 left them. The phase's two invariances of the pair
layer, the import forms and the id permutation, are still tests nowhere: [`findings.md`](findings.md)'s K-4 says
what each needs.

### Defaults taken

All are the coordinator's, taken on 2026-11-15, and the user's to revise at the phase's stop.

1. **A group is over the candidates alive alone.** *Excludes* is defined of two candidates each alive alone, so a
   maximal clique of the exclusion graph holds no candidate dead alone: such a candidate is in no group and is
   reported under its own mark, apart from the candidates alive alone that nothing excludes (I-L02's three). The
   reason is S1f.10.1's: on I-Z2 root leaves 56 candidates and 21 are dead alone; of the 128 pairs along a row
   or a column 45 have both members alive alone and all 45 exclude, and 83 hold a member dead alone and are no
   exclusions. The control above is read under it. **The acceptance's wording, *over the candidates root
   leaves*, is left as written**: the bullet is met under neither reading, and the probe's last row bears on
   which reading a re-plan wants.
2. **The slow half of `ein-infer/tests/pair_layer.rs` is a nightly step** ([`findings.md`](findings.md)'s K-2):
   `EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test pair_layer -- the_excluded_pairs_are_the_pairs_of_n`,
   in `nightly.yml`'s `deep-corpus` job after `obligation_rung`'s and in `run_tests.sh --nightly` and
   `--dev-host`; by name, since it is the one test that reads the flag, and the read moved into the test, which
   `nightly_steps.rs`'s `a_slow_half_run_by_name_leaves_nothing_out` asks. 609 programs compared in 41.7 s on
   four P-core threads with four workers, on AC
   (`EIN_CORPUS_SLOW=1 EIN_TEST_JOBS=4 utils/bench_env.sh --cores PT:4 --require-ac cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test pair_layer -- the_excluded_pairs_are_the_pairs_of_n --nocapture`,
   2026-11-15). It was applied though the stage stopped: it does not depend on what a group is, and `706f9b47`
   fails `nightly_steps.rs`'s `every_slow_half_runs_on_a_tier` without it.

3. **A group is a maximal clique of the exclusions the theory makes alone** (reading 2 of
   [§ What is to be ruled](#what-is-to-be-ruled)), over all of H. Taken after the control, for three reasons. It
   is the founding instruction's own wording: *checked on small fact set of this 2 facts, without the rest of KB
   — taking rules, ontology and this 2 facts … So hyp set can be split into groups*. It is the one reading under
   which the control holds, on the one measurement there is (the probe: 36 cliques on I-Z2's H, every row and
   column, every candidate in two). And it needs nothing that was not planned: S1f.10.10 computes the theory's
   exclusions anyway, and now runs first and over every pair of H. **It supersedes the first entry**: a
   candidate dead alone is in the groups of its row and its column, marked dead alone there, and a group's
   *open* members are those alive alone. Reading 1's object, the maximal cliques of root's exclusion graph, is
   still counted per program beside the cover, since what the clues add to the theory's edges is information
   and [S1f.10.11](s1f.10.11_coupling.md)'s input; it is not called a group. The risk taken with it is
   S1f.10.10's, named in the phase's Risks: *rules and ontology* has no syntactic mark, and where the small KB
   has no definition the corpus leaves standing, a program has no groups by this reading and says so.

## What it must not do

- **Not assume a partition**, anywhere, including in the names of fields.
- **Not read `is-a`** or any relation by name to form a group: a group is cliques of dead pairs and nothing
  else. What the program declares goes beside it, in its own columns.

## Notes

The two clique families of a bijection are the two projections of a bipartite relation, and *exactly one from
each row and each column* is a perfect matching. That is the vocabulary
[S1f.10.4](s1f.10.4_bounded_groups.md) needs; this stage does not assume it, since a cover is well defined
whether or not two families happen to be a grid, and I-Z1's single `co-located` relation is not obviously one.
