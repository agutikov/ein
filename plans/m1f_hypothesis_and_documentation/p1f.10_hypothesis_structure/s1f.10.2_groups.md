# S1f.10.2 — From the exclusion graph to *pick at most one*

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** **ran 2026-11-15, the second time**:
[§ Record: the cover](#record-2026-11-15-the-cover-under-the-theorys-exclusions). A group is a maximal clique of
the exclusions the theory makes alone, the cover is computed by the owner, and on I-Z2 its 36 groups are exactly
the rows and columns of the five declared bijections over all 56 candidates, every candidate in two. The first run
[stopped at its control](#record-2026-11-15-stopped-at-the-control) the same day: no declared row or column is a
maximal clique of root's exclusion graph; the definition in force is [§ Defaults taken](#defaults-taken)'s third
entry, the user's to revise at the phase's stop.
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

Reworded 2026-11-15 where the bullets spoke of root's exclusion graph or of *the candidates root leaves*, to the
definition in force ([§ Defaults taken](#defaults-taken), 3); what each said before is in `git log -p` and in
[§ Acceptance, met](#acceptance-met) of the record.

- The definition of **group** is written in this file, with the corpus program that falsifies each rejected
  alternative named.
- The cover is computed for every program S1f.10.1 read and recorded beside its census: the number of groups,
  their sizes, how many groups a candidate is in (the most and the mean), and the candidates in **no** group.
- **The in-no-group count is the honest column.** A candidate no pair of the theory holds is one no group says
  anything about, and I-L02's three are exactly that.
- **On I-Z2 the groups are exactly the rows and the columns of the five declared bijections, over all 56
  candidates of H**, those dead alone at root among them, every candidate in two. What the clues add is not a
  group: it is the maximal cliques of root's exclusion graph, counted beside the cover and listed raw.
- The enumeration is deterministic, bounded by a constant that is no lever, and its order pinned by a test: the
  cover of a program is the same under a permutation of the interner's ids, and, as sets of candidates, under
  every import form.
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
[`architecture.md`](../../../docs/ein/ein.rs/architecture.md) and
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

## Record (2026-11-15): the cover, under the theory's exclusions

The stage run again, after [S1f.10.10](s1f.10.10_the_theorys_share.md), under
[§ Defaults taken](#defaults-taken)'s third entry. The control came first and holds, so the four tasks ran. The
working tree over `f531052e`, on AC; every number below is a count unless it is called a wall.

### The definition of a group

> **A group is a maximal set of two or more candidates of H that pairwise exclude by the theory alone**: a
> maximal clique of the graph whose vertices are all the candidates of H and whose edges are the pairs the small
> KB excludes ([S1f.10.10](s1f.10.10_the_theorys_share.md)'s `Theory::small`). A state holds at most one member
> of a group. The groups are a **cover** of the candidates they hold: a candidate is in as many groups as hold
> it, and in none where no pair of the theory holds it.

A candidate dead alone at root is a member of its groups like any other, and carries that mark; a group's *open*
members are those alive alone at root. A candidate the theory itself kills alone is in no pair of the theory and
so in no group. No relation is read to form a group, by its name, its arguments or a declaration of it.

The alternatives, each with the corpus program that falsifies it:

| rejected | the program | what it shows |
|---|---|---|
| **a connected component** of the exclusion graph, the theory's or root's | **I-Z2**, `zebra2.ein` | the theory's graph has five components, one a `*-loc` relation, and `color-loc`'s holds `(color-loc Green House-4)` and `(color-loc Ivory House-3)`, which one commitment brings together ([S1f.10.1](s1f.10.1_exclusion_census.md#the-instances)'s class of seven): two members of one component alive in one state. The components are the groups on 28 of the 53 searching programs with a group and are not on 25, each marked ≠ in [the table](#the-corpus-every-program-with-a-group) |
| **a maximal clique of root's exclusion graph**, the definition as planned | **I-Z2** | root's graph has 902 maximal cliques, and none of the 25 rows and columns of two or more open members is one: each is strictly inside a larger clique of root's pairs, since once the clues have decided part of the puzzle a candidate outside a row excludes every member of it (`the_groups_of_a_declared_bijection_are_its_rows_and_columns`; the [stopped run](#record-2026-11-15-stopped-at-the-control)). Kept beside the cover as a count and a raw list, `Cover::root_cliques`, and not called groups |
| **a class of a declared key** | **I-Z1**, `zebra.ein` | 36 groups and no `functional`, `injective` or `bijective` declaration of the relation its candidates are of: its structure is `std.slots`'. 22 of the 53 programs have a group and no declared class, 124 groups ([§ The corpus](#the-corpus-every-program-with-a-group)). And where a class is declared it need not exclude: 32 of the 368 declared classes are not pairwise theory-excluding, on three programs |
| **a maximal clique over single states**, each equivalence class one vertex | I-Z2 | the stopped run's third reading: 16 cliques over 14 states, no more the rows and columns than root's own |
| **a clique of one**, a candidate nothing excludes as its own group | **I-L02** | its three candidates would be three groups that say nothing; they are *in no group*, which is the column that says so |

### What was built

- **The owner**, [`ein-infer/src/structure.rs`](../../../ein.rs/crates/ein-infer/src/structure.rs), beside the pair
  layer and the theory's share, with a section of its header:
  - `Cover`, with **the two maps**: `groups`, group to members, and `member_of`, member to groups (`members(g)`,
    `groups_of(b)`); `marks`, what root says of each candidate alone (`Mark`: `AtRoot`, `DeadAlone`,
    `AliveAlone`, and `Unread` where root's singles were not read), and `open(g)`, a group's members alive alone
    at root; `killed_alone`, the candidates the theory kills alone; `excluded`, the theory's pairs, and
    `excludes(a, b)`; `in_no_group()`; `components`, of the same graph; `steps` and `cut`, the enumeration's
    calls and what the bound cut; `stopped`, `complete()`, `monotone`, `small_monotone` and `domain()`;
    `root_work` and `small_work`; and, where root's pairs were read, **`root_cliques`**, the maximal cliques of
    root's exclusion graph as a `Cliques` (the list, its `steps`, its `cut`), and **`alive_at_root`**, the
    theory's pairs whose node is alive at root.
  - `Cover::standing(&[u32]) -> Standing`: how any set of candidates stands to the cover, `Group(g)`,
    `Inside(groups)` (pairwise excluding and not maximal), `NotExcluding { alive, holds }` (not pairwise
    excluding, with the pairs the theory leaves alive and the groups it holds whole), `Fewer` (under two
    candidates). T3 reads an owed instance's alternatives through it.
  - Three ways in, one function, `Cover::of(root: &PairLayer, small: &PairLayer)`: **`cover_in(sp, loaded,
    opts)`** over a program's prepared space; `cover(root, terms, ast, &CoverOptions)` for a freshly loaded
    program; `Theory::cover()` off a theory's share already read. `CoverOptions::root_pairs` (default `true`)
    says whether root's pairs are read beside its singles.
  - For `cover_in`, `theory_by` was cut in two: what must be taken before root is saturated (`Loaded::of`, now
    crate-visible) and `theory_in(sp, loaded, …)`, everything else, over the space. `explore::Space` got one
    reader, `parts()`, root's sealed fixpoint with the terms and the program's text, so that the small KB is
    built over the open space's terms; the generator's readers, which take a session, read a branch of the
    sealed root. The pair layer got a singles-only reading (`layer(sp, licensed, pairs)`): root's node, the
    singles, the implications and the classes, and no pair's node, which the layer says it stopped before by
    asking. **The census after the cut is the census before it**: all 644 lines of
    `build/structure/census.jsonl`, with the supports, the checks and the three routes, are identical to
    S1f.10.10's but for the `wall`s and the new `cover` object (`jq -c -S 'del(.wall) | del(.theory.cover) |
    (if .routes then .routes |= map_values(del(.wall)) else . end)'` over both, `cmp`, 2026-11-15).
- **The vehicle**, [`pair_layer`](../../../ein.rs/crates/ein-infer/examples/pair_layer.rs): with the theory's
  share (`--theory`, `--supports`, `--check`) each program's directory gets **`cover.json`**, raw (every candidate
  with its mark and its groups, every group with its members and its open members, the components, root's
  cliques, the theory's pairs alive at root, what the program declares and each owed instance with how its
  alternatives stand), and **`cover.md`**, the same as tables; the line gets a `cover` object inside `theory`.
- **The tests**, all in the gate: [`ein-infer/tests/cover.rs`](../../../ein.rs/crates/ein-infer/tests/cover.rs),
  nine; [`structure_invariance.rs`](../../../ein.rs/crates/ein-infer/tests/structure_invariance.rs), four; and
  three in the module, `--lib structure` ([§ The tests](#the-tests-of-the-cover)).
- **Three helpers moved, each as it was, to a file two test binaries include**
  ([§ The two invariances](#the-two-invariances)); [`findings.md`](findings.md)'s K-4 and K-1 are closed by it.
- Docs: `structure.rs`'s row in [`architecture.md`](../../../docs/ein/ein.rs/architecture.md), the
  example's in [`utils/README.md`](../../../utils/README.md), the tier table of
  [`corpus/README.md`](../../../corpus/README.md) and `EIN_CORPUS_SLOW`'s readers in
  [`configuration.md`](../../../docs/kernel/configuration.md).

### What a route would call

A route has the freshly loaded program for a moment and its prepared space from then on
(`explore::run_with`: `region::prepare_with`, then `Space::open`). The cover needs both ends, since a prepared
space holds root's fixpoint and not what the files wrote:

```rust
let loaded = structure::Loaded::of(root, terms, ast)?;        // before region::prepare_with saturates root
let prepared = region::prepare_with(root, terms, ast, events)?;
let mut sp = Space::open(root, terms, ast, prepared, opts, events);
let cover = structure::cover_in(&mut sp, &loaded, &CoverOptions { layer, root_pairs: false })?;
```

Root's singles, and its pairs where `root_pairs` asks, are read through `sp`'s own cache (`Space::eval`), so a
node the route has computed is not saturated again and the nodes the cover read are the route's from then on, in
the route's own `work`. The small KB's layer is read in a space of its own, at `layer`'s budget, and is
`Cover::small_work`: 4 657 saturations on I-Z2M where its root's whole layer is 4 546. The cover is computed
once a program; nothing in it is a node's. **No route calls it**: `cover_in`'s one caller is `cover`.

### The controls, by the engine

`build/structure/s2/control.py`, quoted under [§ The commands](#the-commands-of-the-cover), holds the groups of
`cover.json` to the *lines* of H read off the candidates' texts, `(rel x y)`: the candidates of one relation that
share `x`, or share `y`, of two or more. That is the reading by name the cover may not make.

| | n | dead alone at root | the theory's pairs | **groups** | their sizes | their open sizes | groups a candidate is in | in no group: alive alone, dead alone | lines of two or more | **groups that are lines** | groups that are no line | lines that are no group | components | calls | root's cliques, the most a candidate is in, calls |
|---|---:|---:|---:|---:|---|---|---|---|---:|---:|---:|---:|---:|---:|---|
| **I-Z2** `zebra2.ein` | 56 | 21 | 128 | **36** | 2×8 3×16 4×12 | 1×11 2×18 3×5 4×2 | 2×56 | 0, 0 | 36 | **36** | 0 | 0 | 5 | 149 | 902, 830, 1 109 |
| **I-Z1** `zebra.ein` | 56 | 18 | 128 | **36** | 2×8 3×16 4×12 | 1×9 2×17 3×7 4×3 | 2×56 | 0, 0 | 23 | 19 | 17 | 4 | 5 | 149 | 1 486, 1 350, 1 799 |
| **I-Z2M** `zebra2-minus-15.ein` | 96 | 0 | 318 | **46** | 2×1 3×7 4×21 5×17 | 2×1 3×7 4×21 5×17 | 2×96 | 0, 0 | 46 | **46** | 0 | 0 | 5 | 315 | 3 862, 909, 6 034 |
| **I-SEND** `send_more_money.ein` | 80 | 2 | 640 | **18** | 8×10 10×8 | 6×1 8×9 9×2 10×6 | 2×80 | 0, 0 | 18 | **18** | 0 | 0 | 1 | 406 | 18, 2, 387 |
| **I-B06** `branching/06_lookahead_on.ein` | 132 | 130 | 360 | 10 | 9×10 | 0×10 | 0×42 1×90 | 2, 40 | 40 | 10 | 0 | 30 | 10 | 163 | 0 |
| **I-L02** `lattice/02_genuine_3set_death.ein` | 3 | 0 | 0 | 0 | | | 0×3 | 3, 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

A size list is *size × groups*, and the membership list *groups × candidates*.

- **I-Z2, the control: it holds.** The 36 groups are the 36 rows and columns of the five declared bijections
  over all 56 candidates, no group that is not one and no line that is no group, **every candidate in exactly
  two**: `color-loc` and `drink-loc` three rows and three columns each, `nation-loc`, `pet-loc` and `smoke-loc`
  four and four. Every group has an open member; eleven have one. No deviation.
- **I-Z1: the same 36 under the renaming**, `(co-located X House-n)` for `(R-loc X House-n)`
  (`one_puzzle_in_two_encodings_has_one_cover`). In `zebra.ein`'s own terms they are 18 groups *an attribute's
  houses*, which are lines of its one relation, and 18 *the attributes of one kind at one house*, the smokes at
  `House-2` being `(co-located Chesterfields House-2)`, `(co-located Lucky_Strike House-2)`, `(co-located
  Parliament House-2)`. Of those 18, one is a line and 17 are not: a house's line there holds every kind of
  attribute, up to 18 candidates, and is no group (the four lines that are no group). **The kind is in no
  candidate's text**: it is an `is-a` fact, and the cover finds it without reading one.
- **I-Z2M**: 46 of 46, every candidate in two, no candidate dead alone so every member open; cut down to the 56
  candidates I-Z2 has, they are I-Z2's 36 (`two_programs_one_clue_apart_have_one_cover`).
- **I-SEND**: 18 of 18, a letter's ten digits, eight of them, and a digit's eight letters, ten of them, every
  candidate in two. `(val M 0)` and `(val S 0)` are dead alone at root and members; root's own graph has the
  same 18 over the 78 candidates alive alone.
- **I-L02**: no pair, no group, three candidates alive alone in none, as planned.
- **I-B06, as it comes out: the cover is there, and it says nothing of this program's search.** Ten groups of
  nine: for each of the ten names, `Blue` to `H5`, its nine candidates `(is-a Blue Green)`, `(is-a Blue H1)`,
  …, an object's two types excluding by the program's own `(functional is-a)`. **Every one of the 90 members is
  dead alone at root**, where the program states each name's type, so no group has an open member and *at most
  one* is vacuous. The 42 `co-located` candidates, among them the two alive alone, `(co-located Blue H5)` and
  `(co-located H5 Blue)`, are in no group: the theory excludes no pair of them, the small KB having dropped the
  twelve `is-a` facts that tell a colour from a house (S1f.10.10's clause (a), under the blind enumerator, the
  class [its record](s1f.10.10_the_theorys_share.md#the-rule-as-the-corpus-left-it) says the rule does not
  fit). The groups are true statements, and they are about candidates the blind enumerator proposes and nobody
  would. Six searching programs are so, all six under the blind enumerator
  ([`findings.md`](findings.md)'s K-9).

### The corpus: every program with a group

**644** files load, the engine refuses **19**, and the cover of the other **625** is read: **none has no cover,
none is cut by the bound, none is stopped by the budget, and no pair of the theory is alive at root on any.**
**188 search**, and of those (`jq -s -r -f build/structure/s2/cover.jq build/structure/census.jsonl`,
2026-11-15):

| | |
|---|---:|
| programs with a pair the theory excludes, which are the programs with a group | 53 |
| groups | 496 |
| their sizes, size × groups | 2×156, 3×95, 4×121, 5×64, 8×10, 9×30, 10×19, 16×1 |
| their open sizes | 0×55, 1×87, 2×145, 3×44, 4×84, 5×54, 6×1, 8×9, 9×4, 10×13 |
| candidates of the 53 programs, by the groups each is in | 0×235, 1×537, 2×616, 9×12, 10×8; mean 1.39 |
| **a candidate in more than two groups** | 2 programs, `lattice/06_friends_tables.ein` and its `_everyone_sits` twin |
| the most groups a candidate is in | 10 |
| candidates in no group, over all 188 | 1 514 on 145 programs: 1 153 alive alone at root, on 129; 361 dead alone |
| … the theory kills alone | 210 |
| candidates in no group, on the 53 with a group | 235 on 10 programs: 40 alive alone, on 8; 195 dead alone |
| programs whose components are their groups, and are not | 28, 25 |
| programs where no group has an open member | 6, all under the blind enumerator |
| groups with no open member, with one | 55, 87 |
| by the rung that built H: programs, with a group, groups | hrules 70, 31, 308; obligations 20, 12, 109; blind 89, 10, 79; declined 9, 0, 0 |
| root's exclusion graph has a clique | 44 programs, 11 493 cliques; more of them than groups on 13, fewer on 13 |

Four programs more have a group and do not search, their root being dead: `ein-bugs/alt-cap-core.ein`, its
`-reordered` twin, `ein-bugs/wrong-answers/alt-cap-shared.ein` (66 groups each) and `ein-bugs/zebra2-hints.ein`
(45). Every member is dead alone there.

**More than two groups.** `lattice/06_friends_tables.ein` has 26 groups over 19 candidates where its lines are
ten, and `(sits Ann B1)` is in nine: a rule of the program seats two friends at one table, so Ann's seats at the
big table with Bob's at the small one are a clique, `(sits Ann B1)`, `(sits Ann B2)`, `(sits Ann B3)`, `(sits Ann
S1)`, `(sits Bob S1)`. The groups are right, pairwise excluding and maximal, and they are what a rule that ties
two rows together makes of a rook's graph. Two of its ten declared classes are strictly inside such a group. It
is [S1f.10.11](s1f.10.11_coupling.md)'s first witness of coupling that a pair *does* show.

**Declared classes, by name, beside** (`python3 build/structure/s2/declared.py build/structure
build/structure/census.jsonl`, 2026-11-15; a script, since it reads `functional`, `injective` and `bijective` by
name): 33 searching programs declare a class of two or more over a candidates' relation, 368 classes. **332 are a
group**, 4 are strictly inside one (the two `friends_tables` programs), and 32 are not pairwise
theory-excluding: `branching/12_typed_blind_solve.ein` (16) and `tests/stdlib/bijection/06_blind_enumeration.ein`
(10), under the blind enumerator, and `syntax/constraint-scopes.ein` (6), which states `(bijective next-to)` and
imports no rule that reads it. Those programs' groups that are no declared class are 40: 18 each on the
`friends_tables` pair and 4 on `12_typed_blind_solve`. **22 programs have a group and no declared class**, 124
groups, `zebra.ein`'s 36 the most.

Every searching program with a group. *Sizes* and *open* are size × groups, *in* is groups × candidates, *none*
the candidates in no group alive alone and dead alone at root, *components* marked ≠ where they are not the
groups, *calls* the enumeration's, *root's* the maximal cliques of root's graph with their enumeration's calls,
and the last four columns T3's: the instances root owes that the engine reads, those whose alternatives are a
group, those it cannot license, and the groups nothing owes
(`jq -r -f build/structure/s2/rows.jq build/structure/census.jsonl`, 2026-11-15).

| program | rung | n | theory's pairs | groups | sizes | open | in | none: alive | dead | components | calls | root's cliques | calls | owed | … a group | … unlicensed | groups nothing owes |
|---|---|---:|---:|---:|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `examples/beyond_arith/cryptarithm.ein` | hrules | 30 | 165 | 13 | 3×10 10×3 | 1×1 3×9 9×2 10×1 | 2×30 | 0 | 0 | 1 ≠ | 86 | 12 | 63 | 3 | 3 | 0 | 10 |
| `examples/beyond_arith/equation_by_search.ein` | hrules | 30 | 135 | 3 | 10×3 | 1×1 10×2 | 1×30 | 0 | 0 | 3 | 49 | 3 | 31 | 3 | 3 | 0 | 0 |
| `examples/beyond_arith/equation_no_solution.ein` | hrules | 20 | 90 | 2 | 10×2 | 1×1 10×1 | 1×20 | 0 | 0 | 2 | 30 | 1 | 12 | 2 | 2 | 0 | 0 |
| `examples/beyond_arith/equation_two_unknowns.ein` | hrules | 30 | 135 | 3 | 10×3 | 10×3 | 1×30 | 0 | 0 | 3 | 49 | 3 | 49 | 3 | 3 | 0 | 0 |
| `examples/beyond_arith/send_more_money.ein` | hrules | 80 | 640 | 18 | 8×10 10×8 | 6×1 8×9 9×2 10×6 | 2×80 | 0 | 0 | 1 ≠ | 406 | 18 | 387 | 8 | 8 | 0 | 10 |
| `examples/binary_logic/equivalent.ein` | obligations | 6 | 3 | 3 | 2×3 | 2×3 | 1×6 | 0 | 0 | 3 | 9 | 3 | 9 | 3 | 3 | 0 | 0 |
| `examples/binary_logic/knights.ein` | obligations | 6 | 3 | 3 | 2×3 | 2×3 | 1×6 | 0 | 0 | 3 | 9 | 5 | 10 | 3 | 3 | 0 | 0 |
| `examples/binary_logic/not_equivalent.ein` | obligations | 6 | 3 | 3 | 2×3 | 2×3 | 1×6 | 0 | 0 | 3 | 9 | 3 | 9 | 3 | 3 | 0 | 0 |
| `examples/binary_logic/satisfiable.ein` | obligations | 6 | 3 | 3 | 2×3 | 2×3 | 1×6 | 0 | 0 | 3 | 9 | 3 | 9 | 3 | 3 | 0 | 0 |
| `examples/binary_logic/synthesis.ein` | obligations | 16 | 120 | 1 | 16×1 | 2×1 | 1×16 | 0 | 0 | 1 | 17 | 1 | 3 | 1 | 1 | 0 | 0 |
| `examples/binary_logic/truth_table.ein` | obligations | 6 | 3 | 3 | 2×3 | 2×3 | 1×6 | 0 | 0 | 3 | 9 | 3 | 9 | 3 | 3 | 0 | 0 |
| `examples/binary_logic/unsatisfiable.ein` | obligations | 6 | 3 | 3 | 2×3 | 2×3 | 1×6 | 0 | 0 | 3 | 9 | 4 | 9 | 3 | 3 | 0 | 0 |
| `examples/branching/01_saturate_only.ein` | blind | 16 | 12 | 4 | 3×4 | 0×4 | 0×4 1×12 | 0 | 4 | 4 | 19 | 0 | 0 |  | 0 | 0 | 4 |
| `examples/branching/02_one_dead_one_alive.ein` | blind | 48 | 44 | 6 | 4×4 5×2 | 0×6 | 0×22 1×26 | 18 | 4 | 6 | 43 | 56 | 65 |  | 0 | 0 | 6 |
| `examples/branching/03_five_hyps_one_alive.ein` | blind | 132 | 360 | 10 | 9×10 | 0×10 | 0×42 1×90 | 2 | 40 | 10 | 163 | 0 | 0 |  | 0 | 0 | 10 |
| `examples/branching/04_two_levels.ein` | blind | 46 | 60 | 6 | 5×6 | 0×6 | 0×16 1×30 | 8 | 8 | 6 | 51 | 16 | 21 |  | 0 | 0 | 6 |
| `examples/branching/05_mini_zebra.ein` | hrules | 9 | 6 | 6 | 2×6 | 1×6 | 1×6 2×3 | 0 | 0 | 3 ≠ | 14 | 0 | 0 |  | 0 | 0 | 6 |
| `examples/branching/06_lookahead_on.ein` | blind | 132 | 360 | 10 | 9×10 | 0×10 | 0×42 1×90 | 2 | 40 | 10 | 163 | 0 | 0 |  | 0 | 0 | 10 |
| `examples/branching/07_lookahead_off.ein` | blind | 132 | 360 | 10 | 9×10 | 0×10 | 0×42 1×90 | 2 | 40 | 10 | 163 | 0 | 0 |  | 0 | 0 | 10 |
| `examples/branching/08_hypothesis_relation_whitelist.ein` | blind | 38 | 24 | 24 | 2×24 | 2×24 | 0×14 2×24 | 2 | 12 | 6 ≠ | 47 | 168 | 213 |  | 0 | 0 | 24 |
| `examples/branching/09_hrule.ein` | hrules | 7 | 10 | 6 | 2×4 3×2 | 0×4 1×2 | 2×7 | 0 | 0 | 1 ≠ | 13 | 0 | 0 |  | 0 | 0 | 6 |
| `examples/branching/10_kill_cache_on.ein` | hrules | 18 | 12 | 12 | 2×12 | 1×12 | 1×12 2×6 | 0 | 0 | 6 ≠ | 29 | 0 | 0 |  | 0 | 0 | 12 |
| `examples/branching/11_kill_cache_off.ein` | hrules | 18 | 12 | 12 | 2×12 | 1×12 | 1×12 2×6 | 0 | 0 | 6 ≠ | 29 | 0 | 0 |  | 0 | 0 | 12 |
| `examples/branching/12_typed_blind_solve.ein` | blind | 51 | 4 | 4 | 2×4 | 2×4 | 0×47 2×4 | 0 | 47 | 1 ≠ | 7 | 4 | 7 |  | 0 | 0 | 4 |
| `examples/branching/15_lookahead_two_step_on.ein` | hrules | 3 | 2 | 2 | 2×2 | 2×2 | 1×2 2×1 | 0 | 0 | 1 ≠ | 4 | 2 | 4 |  | 0 | 0 | 2 |
| `examples/branching/16_lookahead_two_step_off.ein` | hrules | 3 | 2 | 2 | 2×2 | 2×2 | 1×2 2×1 | 0 | 0 | 1 ≠ | 4 | 2 | 4 |  | 0 | 0 | 2 |
| `examples/domain_elim/b_branch.ein` | hrules | 7 | 10 | 6 | 2×4 3×2 | 0×4 1×2 | 2×7 | 0 | 0 | 1 ≠ | 13 | 0 | 0 |  | 0 | 0 | 6 |
| `examples/ein-bugs/complete-records-stale.ein` | hrules | 3 | 2 | 2 | 2×2 | 2×2 | 1×2 2×1 | 0 | 0 | 1 ≠ | 4 | 2 | 4 |  | 0 | 0 | 2 |
| `examples/ein-bugs/fork-delta-order.ein` | hrules | 2 | 1 | 1 | 2×1 | 2×1 | 1×2 | 0 | 0 | 1 | 3 | 1 | 3 |  | 0 | 0 | 1 |
| `examples/ein-bugs/unsat-core-id-space.ein` | blind | 5 | 6 | 4 | 2×3 3×1 | 0×1 1×2 2×1 | 1×1 2×4 | 0 | 0 | 1 ≠ | 8 | 1 | 3 |  | 0 | 0 | 4 |
| `examples/ein-bugs/wrong-answers/einb-dump-answer.ein` | blind | 2 | 1 | 1 | 2×1 | 2×1 | 1×2 | 0 | 0 | 1 | 3 | 1 | 3 |  | 0 | 0 | 1 |
| `examples/ein-bugs/wrong-answers/forall-core-elim.ein` | hrules | 2 | 1 | 1 | 2×1 | 1×1 | 1×2 | 0 | 0 | 1 | 3 | 0 | 0 |  | 0 | 0 | 1 |
| `examples/ein-bugs/wrong-answers/forall-core-false.ein` | hrules | 2 | 1 | 1 | 2×1 | 2×1 | 1×2 | 0 | 0 | 1 | 3 | 1 | 3 |  | 0 | 0 | 1 |
| `examples/features/11_expect_ambiguity.ein` | hrules | 4 | 4 | 4 | 2×4 | 2×4 | 2×4 | 0 | 0 | 1 ≠ | 7 | 4 | 7 |  | 0 | 0 | 4 |
| `examples/features/13_mixed_solution_and_open.ein` | hrules | 2 | 1 | 1 | 2×1 | 2×1 | 1×2 | 0 | 0 | 1 | 3 | 1 | 3 | 1 | 0 | 1 | 1 |
| `examples/lattice/05_owed_seats_bounded.ein` | obligations | 4 | 4 | 4 | 2×4 | 2×4 | 2×4 | 0 | 0 | 1 ≠ | 7 | 4 | 7 | 4 | 4 | 0 | 0 |
| `examples/lattice/06_friends_tables.ein` | hrules | 19 | 73 | 26 | 3×3 4×12 5×11 | 2×1 3×2 4×13 5×10 | 2×9 9×6 10×4 | 0 | 0 | 1 ≠ | 77 | 26 | 71 |  | 0 | 0 | 26 |
| `examples/lattice/06_friends_tables_everyone_sits.ein` | hrules | 19 | 73 | 26 | 3×3 4×12 5×11 | 2×1 3×2 4×13 5×10 | 2×9 9×6 10×4 | 0 | 0 | 1 ≠ | 77 | 26 | 71 | 5 | 2 | 3 | 24 |
| `examples/lattice/07_two_pairs_two_tables.ein` | hrules | 16 | 48 | 8 | 4×8 | 4×8 | 2×16 | 0 | 0 | 1 ≠ | 50 | 36 | 82 | 4 | 4 | 0 | 4 |
| `examples/lattice/synth/merged_states.ein` | hrules | 5 | 1 | 1 | 2×1 | 2×1 | 0×3 1×2 | 3 | 0 | 1 | 3 | 1 | 3 | 1 | 1 | 0 | 0 |
| `examples/lattice/synth/one_model_per_component.ein` | hrules | 4 | 6 | 1 | 4×1 | 4×1 | 1×4 | 0 | 0 | 1 | 5 | 1 | 5 | 1 | 1 | 0 | 0 |
| `examples/word_problems/boxes12.ein` | hrules | 2 | 1 | 1 | 2×1 | 2×1 | 1×2 | 0 | 0 | 1 | 3 | 1 | 3 | 1 | 1 | 0 | 0 |
| `examples/word_problems/boxes13.ein` | hrules | 2 | 1 | 1 | 2×1 | 2×1 | 1×2 | 0 | 0 | 1 | 3 | 1 | 3 | 1 | 1 | 0 | 0 |
| `examples/zebra.ein` | hrules | 56 | 128 | 36 | 2×8 3×16 4×12 | 1×9 2×17 3×7 4×3 | 2×56 | 0 | 0 | 5 ≠ | 149 | 1486 | 1799 | 36 | 18 | 18 | 18 |
| `examples/zebra2-hints.ein` | hrules | 36 | 72 | 26 | 2×12 3×8 4×6 | 1×16 2×10 | 2×36 | 0 | 0 | 4 ≠ | 87 | 55 | 61 | 0 | 0 | 0 | 26 |
| `examples/zebra2-minus-15-obligations.ein` | obligations | 96 | 318 | 46 | 2×1 3×7 4×21 5×17 | 2×1 3×7 4×21 5×17 | 2×96 | 0 | 0 | 5 ≠ | 315 | 3862 | 6034 | 46 | 46 | 0 | 0 |
| `examples/zebra2-minus-15.ein` | hrules | 96 | 318 | 46 | 2×1 3×7 4×21 5×17 | 2×1 3×7 4×21 5×17 | 2×96 | 0 | 0 | 5 ≠ | 315 | 3862 | 6034 | 46 | 46 | 0 | 0 |
| `examples/zebra2-obligations.ein` | obligations | 56 | 128 | 36 | 2×8 3×16 4×12 | 1×11 2×18 3×5 4×2 | 2×56 | 0 | 0 | 5 ≠ | 149 | 902 | 1109 | 36 | 36 | 0 | 0 |
| `examples/zebra2.ein` | hrules | 56 | 128 | 36 | 2×8 3×16 4×12 | 1×11 2×18 3×5 4×2 | 2×56 | 0 | 0 | 5 ≠ | 149 | 902 | 1109 | 36 | 36 | 0 | 0 |
| `tests/region/02_merged_single.ein` | hrules | 5 | 1 | 1 | 2×1 | 2×1 | 0×3 1×2 | 3 | 0 | 1 | 3 | 1 | 3 | 1 | 1 | 0 | 0 |
| `tests/region/09_query_over_maximal.ein` | hrules | 4 | 4 | 4 | 2×4 | 2×4 | 2×4 | 0 | 0 | 1 ≠ | 7 | 4 | 7 |  | 0 | 0 | 4 |
| `tests/stdlib/bool/inputs/01_in_drives.ein` | obligations | 4 | 2 | 2 | 2×2 | 2×2 | 1×4 | 0 | 0 | 2 | 6 | 2 | 6 | 2 | 2 | 0 | 0 |
| `tests/stdlib/bool/inputs/02_required.ein` | obligations | 4 | 2 | 2 | 2×2 | 2×2 | 1×4 | 0 | 0 | 2 | 6 | 2 | 5 | 2 | 2 | 0 | 0 |

### The bound and the order

- **The bound** is `structure::CLIQUE_STEPS`, **2^20 calls** of the search for the cliques of one graph, stated
  in the module's header: a constant, and no `(config …)` field, no option and no `EIN_*` name reaches it. It
  counts calls and not cliques, since a call lists at most one clique and a search can make many calls that list
  none. A graph it stops on keeps the cliques found, each maximal, and says what was cut: `Cut { pairs,
  candidates }`, the excluded pairs inside no clique listed and the candidates of such a pair no clique listed
  holds; the example prints it on the line (`cut`), in `cover.json` and in `cover.md`.
- **Re-measured for both graphs**, over all 625 programs read: **nothing is cut.** The longest enumeration of a
  theory's graph is 406 calls, I-SEND's, and of a root's graph 6 034, I-Z2M's, for its 3 862 cliques, which are
  still the most: 0.6 % of the bound. The stopped run's figure stands.
- **The search** is Bron and Kerbosch's with a pivot, the candidate of `P ∪ X` joined to the most of `P`, the
  lowest among equals. On 400 graphs of up to twelve candidates it lists exactly the cliques found by trying
  every set, and the same graph renumbered has the same cliques (`the_enumeration_lists_every_maximal_clique`).
- **The order is canonical**: a group is its members ascending, which is the universe's rendered-text order,
  and the groups are sorted as those sequences. The cover is a function of the program: the same at one job and
  at four (`the_cover_is_the_same_at_any_job_count`), under a permuted id space and under every import form
  ([§ The two invariances](#the-two-invariances)). A cover the bound *did* cut lists what the search met first
  in the universe's order, and so is not import-invariant; no corpus program is there.

### Beside what the program declares

Read by what the engine already reads, of the program loaded again: the mechanisms at root's fixpoint
(`mechanisms::read`) and L5's table (`oblgen::alternatives_of`), which lists every instance root owes with its
alternatives, the candidates that can witness it, or why it has none.

**How each case is counted.**

- A **licensed** instance's alternatives are candidates of H: every fact that can witness it is a candidate, or
  is denied at root, and a denied one is not in H and so in no group (`witnesses_denied_at_root` in `cover.json`).
  The alternatives are read against the cover by `Cover::standing`: *a group*, *inside a group*, *not pairwise
  excluding* (and whether they hold a whole group), *one alternative*, *no alternative*.
- An **unlicensed** instance has no alternatives the engine knows, only the reason, so it is neither a group nor
  not one: it is counted *unlicensed*, by the kind of its reason.
- A **flagged** program, and one that states no obligation, has no table (L5 is read on a monotone program):
  what root owes there is counted and not read.

**The three columns, on the instances.**

| | root owes | **a group that is an owed instance's alternatives** | **a group nothing owes** | **an owed instance whose alternatives are not a group** |
|---|---:|---|---|---|
| I-Z2 | 36 | all 36: `total-owed` the 18 rows, `surjective-owed` the 18 columns | none | none |
| I-Z2M | 46 | all 46: 23 and 23 | none | none |
| I-Z1 | 36 | 18: `std.slots.slot-owed-room`, an attribute's houses | 18: the attributes of one kind at one house | none licensed; **18 unlicensed**, `std.slots.slot-owed-fill`, *a `co-located` fact can witness it that no candidate proposes*. Those 18 instances are one a house and a kind, which is what the 18 groups nothing owes are |
| I-SEND | 8 | 8: `total-owed`, a letter's digits | 10: a digit's letters, which nothing owes, a digit needing no letter | none |
| I-B06 | 0, no table | none | all 10 | none |
| I-L02 | 0, no table | none | none | none |

**The corpus**, the 188 that search: a table is read on 43 and holds an instance on 41, **288 instances**.

| | instances |
|---|---:|
| licensed, and its alternatives are **a group** | **239**, on 27 programs |
| licensed, strictly inside a group | 0 |
| licensed, a strict superset of a group | 0 |
| licensed, **not pairwise excluding** | **19**, on 7 programs, none of which has a group at all |
| licensed, one alternative, or none | 0, 0 |
| **unlicensed**: a fact can witness it that no candidate proposes | 22: `zebra.ein` 18, `lattice/06_friends_tables_everyone_sits.ein` 3, `features/13_mixed_solution_and_open.ein` 1 |
| unlicensed: it reads a relation a commitment can change beside its witness | 8, on six `ein-bugs/wrong-answers/oblig-*` programs the ladder declines |
| owed on a flagged program, no table read | 7, on 6 programs |

Of the 496 groups **239 are an owed instance's alternatives and 257 nothing owes**, on 31 programs. The 19 that
do not pairwise exclude are owed where nothing is exclusive: `ein-bugs/wrong-answers/owed-seats.ein`, its
`-renamed` twin, `uncovered.ein` and `uncovered-knows.ein` (two `total-owed` and two `surjective-owed` each, of a
relation declared neither functional nor injective), `owed-two-step.ein`, `tree-dead.ein` and
`lattice/synth/trivial_line.ein` (one each). That is *at least one* with no *at most one*, which is the third
kind [S1f.10.4](s1f.10.4_bounded_groups.md) starts from. By rule, a rule counted under either spelling a program
gives it: `std.algebra`'s `total-owed` 132 a group, 10 not pairwise excluding and 4 unlicensed, its
`surjective-owed` 84 and 8; `std.slots`' `slot-owed-room` 18 a group and `slot-owed-fill` 18 unlicensed; and the
programs' own obligation rules 5 a group (`owe-1` 3, `readings.reading-owed` 2), 1 not (`owe-1`) and 8 unlicensed
(`owes-food`).

**The mechanisms.** 494 of the 496 groups are of one relation (the two others are
`ein-bugs/complete-records-stale.ein`'s). 308 are of a relation an active hrule proposes, 331 of one an obligation
of root's can owe, none of one a closure takes out, and 79, on the ten programs under the blind enumerator, of
neither.

### The two invariances

The phase's acceptance asks that *the structure of a program is the same under every import form and under a
permutation of the interner's ids*, and neither was a test ([`findings.md`](findings.md)'s K-4).
[`ein-infer/tests/structure_invariance.rs`](../../../ein.rs/crates/ein-infer/tests/structure_invariance.rs) is
both, of the whole structure at once: root's pair layer, the theory's share (the small KB's layer, every stated
fact with whether it is kept and by which clause, the rung, the rules that name an object, what is dead in the
small KB and alive at root, which of root's pairs are the theory's) and the cover (the groups, both maps, the
marks, the candidates the theory kills alone, the components, the theory's pairs alive at root, root's cliques,
the verdicts and the work).

| | what is compared | how |
|---|---|---|
| **the id order** | the program as loaded, and the same program loaded into a `Terms` whose every name and fact is the first run's own, interned again in a shuffled order (`id_permutation::permuted`, the determinism sweeps' instrument) | **every list as it is**, place for place: the universe is in rendered-text order in both. The work and the enumeration's calls among them |
| **the import form** | the program as written, with every selective import whole (W), and with each `:as` an alias (A) (`import_forms::form`, the answer's own instrument) | **every list as a set of texts with the module prefixes and aliases taken off** (`import_forms::strip`): a candidate's text changes with the form, and with it the universe's order. The enumeration's calls follow the order and are not compared |

**Over which programs**: every corpus file and register witness that searches, by the determinism sweeps' own
`searches`; per commit all but the `slow` entries, and three named instances whatever their flag (`zebra.ein`,
`zebra2.ein`, `lattice/02_genuine_3set_death.ein`); under `EIN_CORPUS_SLOW=1` every one. Each program is read
once as loaded and both invariances are held to that reading.

| | programs that search | compared under ids | with a group | flagged | that import | forms compared | wall |
|---|---:|---:|---:|---:|---:|---:|---:|
| per commit | 175 | 175 | 47 | 27 | 47 | 94 | 21.3 s |
| `EIN_CORPUS_SLOW=1` | 188 | 188 | 53 | 29 | 53 | 106 | 56.8 s |

**No program moves under either.** The walls are the whole target on four P-core threads with four workers, on
AC, load average 4.6 when taken (`EIN_TEST_JOBS=4 utils/bench_env.sh --cores PT:4 --require-ac cargo test
--manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test structure_invariance -- --nocapture`, with
and without the flag, 2026-11-15); at the default 16 workers they are 12.6 s and 37.1 s. That is what
`theory_share` costs per commit (21.9 s by the same command), so the sweep is per commit and its slow half a
nightly step and the dev host's, run whole ([§ Defaults taken here](#defaults-taken-here), 4).

**What the corpus's import forms do and do not move.** In 22 of the 106 forms a candidate is written otherwise
than the program writes it, and in **2** the universe's order is another, no program with two groups or more
among them: few corpus programs propose facts of an imported relation beside facts of their own. So the corpus
holds the cover to the *spelling* of a name and hardly to the *order*. The test therefore writes one program of
its own that does reorder (`a_form_that_reorders_the_universe_moves_no_set`: `(at a p)` is before `(likes a a)`
as written and after it as `(places.at a p)`), with four groups, the same sets in all three forms
([`findings.md`](findings.md)'s K-8).

**Each comparison fails when it should**, tried and reverted (2026-11-15): the groups ordered by their first
member's `FactId` fail the id test on 30 programs, each named with *the cover's groups and maps*; the search
without its `X` set fails `the_enumeration_lists_every_maximal_clique`; and a cover cut to its first three groups
in the universe's order passes the corpus's import sweep, which is the paragraph above, and fails the written
program's. `a_structure_that_moves_is_caught` holds the two comparators to a reading altered by hand.

**The sharing**, as K-4 read it: a test binary's functions are no crate's, so each helper moved as it was into a
directory of its own tests tree and is a module of its own binary and, by `#[path]`, of
`structure_invariance.rs`:

| moved | from | to | also included by |
|---|---|---|---|
| `permuted`, `moved`, with `remap` and `FACT_OFFSET` | `ein-render/tests/id_order_invariance.rs` | `ein-render/tests/id_permutation/mod.rs` | `structure_invariance.rs` |
| `form`, `strip`, `module_prefixes`; and `beside_its_modules`, the scratch directory of links, cut out of `check` | `ein-cli/tests/import_invariance.rs` | `ein-cli/tests/import_forms/mod.rs` | `structure_invariance.rs` |
| `searches`, `SWEEP_BUDGET`, `SWEEP_LIMIT` | `ein-render/tests/corpus_ops/mod.rs` | `ein-render/tests/corpus_ops/searches.rs`, declared by `corpus_ops` as its own module | `structure_invariance.rs`, and the `pair_layer` example, whose copy is gone (K-1) |

No behaviour of the three sweeps changed, and each was run again: `id_order_invariance` 4 passed,
`jobs_invariance` 4 passed, `import_invariance` 1 passed
([§ The commands](#the-commands-of-the-cover)).

### The tests of the cover

| test | what it holds | wall, the target |
|---|---|---:|
| `cover.rs`: `the_groups_of_a_declared_bijection_are_its_rows_and_columns` | **the control**, I-Z2: the groups are the 36 lines read off the texts, every candidate in two, 21 dead alone and members, every group with an open member, no pair of the theory alive at root, five components none of them a group; and none of the 25 lines of two or more open members is a clique of root's graph, each being inside one | |
| `one_puzzle_in_two_encodings_has_one_cover` | I-Z1's groups are I-Z2's under the renaming; 19 of its 36 are lines of its one relation | |
| `two_programs_one_clue_apart_have_one_cover` | I-Z2M's 46 are its lines, and cut to I-Z2's candidates are I-Z2's; root read as far as its singles | |
| `a_cryptarithms_groups_are_its_letters_and_its_digits` | I-SEND: 8 of ten and 10 of eight, every candidate in two, two members dead alone | |
| `a_conflict_of_three_is_in_no_group` | I-L02 | |
| `an_owed_instances_alternatives_are_a_group_on_the_declared_control` | T3 on I-Z2: 36 licensed instances, each one's alternatives a group, each group owed once | |
| `every_cover_is_whole_and_its_groups_are_the_maximal_cliques` | **what a cover is**, over the per-commit sweep through `ein_corpus::pool::map`, 612 programs, 51 with a group, 559 groups, 27 flagged: nothing cut, root's pairs all read, no pair of the theory alive at root, every group an ascending set of two or more that pairwise excludes and that no candidate outside extends, every pair of the theory inside a group, the two maps each other's inverse, each group standing as itself and inside one component, the candidates in no group those no pair holds; and the longest enumeration under a sixteenth of the bound | |
| `the_cover_is_the_same_by_every_entry` | over the prepared space with root's pairs, without them, and off a theory's share: one set of groups, maps and marks, on a monotone program, a flagged one and one whose root is dead; root's pairs add root's cliques and nothing else | |
| `the_cover_is_the_same_at_any_job_count` | one thread and four, a flagged program among them | 14.4 s |
| `--lib structure`: `the_enumeration_lists_every_maximal_clique`, `the_bound_says_what_it_cut`, `a_grid_is_covered_by_its_rows_and_columns` | the search against every set on 400 graphs and under a renumbering; a graph of 243 cliques cut at 60 calls, the cut counted; the two maps, the marks, `open`, `standing`'s four answers and root's cliques on a grid written by hand | under 0.1 s |

The wall is `cover`'s whole target on four P-core threads with four workers, on AC, by the command under
[§ The two invariances](#the-two-invariances) with `--test cover`; 5.0 s at 16 workers.

### Acceptance, met

| | | evidence |
|---|---|---|
| the definition of **group** is written here, with the program that falsifies each rejected alternative | met | [§ The definition of a group](#the-definition-of-a-group) |
| the cover is computed for every program S1f.10.1 read and recorded beside its census: the groups, their sizes, the most and the mean a candidate is in, the candidates in none | met | 625 programs, on the census's own line; [§ The corpus](#the-corpus-every-program-with-a-group) |
| the in-no-group count, and I-L02's three | met | 1 514 over the 188, split by what root says of each; I-L02's three alive alone, `a_conflict_of_three_is_in_no_group` |
| **on I-Z2 the groups are exactly the rows and columns of the five declared bijections, over all 56 candidates, every candidate in two**; what the clues add is root's cliques, beside | **met** | `the_groups_of_a_declared_bijection_are_its_rows_and_columns`; 902 cliques of root's graph listed in `build/structure/examples/zebra2/cover.json` |
| the enumeration is deterministic, bounded by a constant, its order pinned: the same under a permutation of the ids and under every import form | met | [§ The bound and the order](#the-bound-and-the-order), [§ The two invariances](#the-two-invariances) |
| no answer, route or surface changes | met | no route calls the module; `explore.rs` gains one reader; `explore`, `region`, `pair_layer`, `theory_share` green with no expectation edited, and the census's theory, supports, checks and routes are what they were on all 644 lines |

**The bullets were reworded, and why.** Two of them spoke of the object the stage first planned: *the cover is
the rows and the columns … over the candidates root leaves, plus whatever the clues add, each addition listed*,
and *a candidate nothing excludes*. The definition changed by a recorded default, the third of
[§ Defaults taken](#defaults-taken), taken after the first run's control failed and before this run: a group is a
clique of the theory's exclusions over all of H. Under it the clues add no group, so the bullet now says where
what they add is kept, and *over the candidates root leaves* reads *over all 56 candidates of H, those dead alone
at root among them*. The control was not reworded to pass: it is an identity, 36 of 36 with no group that is no
line, where the first run found 0 of 25.

Of the phase's own acceptance for track S, this stage meets two bullets it did not before: *on I-Z2 the
discovered groups are the rows and columns of the five declared bijections*, and *the structure of a program is
the same under every import form and under a permutation of the interner's ids*.

### Defaults taken here

All are this run's, taken on 2026-11-15, and the user's to revise at the phase's stop.

1. **A group has two or more members.** A candidate no pair of the theory holds is in no group, and is not a
   group of one: the acceptance's *in no group* column is that, and a group of one would say nothing.
2. **The theory's edges are the pairs the small KB excludes, and no other**: each member alive alone in the small
   KB, neither implying the other, the pair dead. A candidate the theory kills alone has no edge and is in no
   group, under its own mark (`killed_alone`; 210 candidates over the 188).
3. **The bound counts calls**, 2^20 for one graph, and a cover it cuts keeps what was found and says what was
   cut, where the alternative was no cover at all.
4. **`structure_invariance` runs per commit**, on every searching program but the `slow` entries and on three
   named instances, and its slow half is a nightly step and the dev host's
   (`EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test structure_invariance`, in `nightly.yml`'s
   `deep-corpus` job after `pair_layer`'s, in `run_tests.sh --nightly` and `--dev-host`, in
   [`corpus/README.md`](../../../corpus/README.md)'s tier table and
   [`configuration.md`](../../../docs/kernel/configuration.md)'s readers). 21.3 s per commit and 56.8 s with the
   flag, on four P-core threads with four workers.
5. **A flagged program has a cover**, as it has a pair layer, and the cover says what it is about
   (`Cover::domain`); 29 of the 188.
6. **T3 counts an unlicensed instance as unlicensed**, by its reason, and not as *an owed instance whose
   alternatives are not a group*: the engine names no alternatives for it. On I-Z1 that keeps `slot-owed-fill`'s
   18 out of both columns, with the 18 groups they are about counted *nothing owes*.
7. **Root's cliques are listed, not only counted**: I-Z2M's 3 862 sets are most of its `cover.json`, 0.8 MB.
8. **`Theory`'s `root_pairs: false` is left as S1f.10.10 defined it**, root read as far as its universe, so a
   cover taken off such a share has every mark `Unread`. The cover's own entries read root's singles always.

### What it did not do

- **No route reads the cover**, and `cover_in`'s one caller is `cover`, over a space it has just opened. That a
  space with nodes already in its cache gives the same cover rests on `Space::eval`'s cache, as `pair_layer_in`'s
  claim does, and no test opens a route's space first.
- **`./run_tests.sh` was not run**, nor the nightly, nor `utils/ci_container.sh`: the gate is the batch's. What
  was run is below.
- **No corpus program is cut by the bound, has a small KB with no layer, or is stopped by the budget**, so those
  three paths are held by `the_bound_says_what_it_cut` on a graph written for it and by nothing on a program.
- **The declared classes are read by a script**, by name, and by no test.
- **Nothing is spent**, and no coupling is read: `friends_tables`' groups are reported and left to S1f.10.11.

### The commands of the cover

2026-11-15, the working tree over `f531052e`, on AC. The census, one P-core (`cpu4`), governor `powersave`, load
average 1.9 at its start; the per-program walls sum to 134 s for the layers, the shares, the supports, the checks
and the covers, and 78 s for the three routes:

```sh
mkdir -p build/structure
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer
utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer --routes --check \
    > build/structure/census.jsonl 2> build/structure/census.env
```

That is the one command, S1f.10.10's: the cover comes with the theory's share. The four scripts below are under
`build/structure/s2/`, not checked in, and whole here. The corpus's headline:

```jq
# jq -s -r -f build/structure/s2/cover.jq build/structure/census.jsonl
def cov: .theory.cover;
def count(f): map(select(f)) | length;
def total(f): map(f) | add;
def hist(f): map(f) | add | group_by(.[0]) | map("\(.[0][0]) × \(map(.[1]) | add)") | join(", ");
map(select(has("error") | not)) as $read
| ($read | map(select(.searches))) as $s
| ($s | map(select(cov.groups > 0))) as $g
| "read \($read | length), searching \($s | length); no cover \($read | count(cov | has("error"))); not complete \($read | count(cov.complete | not)); the bound cut a cover \($read | count(cov.cut != null)), root's cliques \($read | count(cov.root_cliques.cut != null)); root's pairs not all read \($read | count(cov.root_cliques == null)); a theory's pair alive at root \($read | count(cov.pairs_alive_at_root > 0))",
  "the longest enumeration, of \($read[0] | cov.bound) calls: the theory's graph \($read | map(cov.steps) | max) (\($read | max_by(cov.steps) | .path)), root's \($read | map(cov.root_cliques.steps) | max) (\($read | max_by(cov.root_cliques.steps) | .path)); the most cliques of root's graph \($read | map(cov.root_cliques.cliques) | max)",
  "all read, not searching, with a group: \($read | count((.searches | not) and cov.groups > 0))",
  "searching: with a pair the theory excludes \($s | count(cov.pairs > 0)); with a group \($g | length); groups \($g | total(cov.groups)); their sizes (size × groups) \($g | hist(cov.sizes)); their open sizes \($g | hist(cov.open))",
  "searching: candidates \($s | total(.n)); by the number of groups a candidate is in (groups × candidates) \($s | hist(cov.membership)); the most \($s | map(cov.most) | max)",
  "searching, with a group: candidates \($g | total(.n)); by the number of groups \($g | hist(cov.membership)); mean groups a candidate \(($g | total(cov.groups as $x | cov.mean * .n)) / ($g | total(.n)) * 100 | round / 100)",
  "searching: a candidate in more than two groups on \($s | count(cov.most > 2)): \($s | map(select(cov.most > 2) | "\(.path) (\(cov.most))") | join(", "))",
  "searching: candidates in no group \($s | total(cov.in_no_group.all)) on \($s | count(cov.in_no_group.all > 0)) programs: alive alone at root \($s | total(cov.in_no_group.alive_alone)) on \($s | count(cov.in_no_group.alive_alone > 0)), dead alone \($s | total(cov.in_no_group.dead_alone)), at root \($s | total(cov.in_no_group.at_root)), unread \($s | total(cov.in_no_group.unread)); of them killed by the theory alone \($s | total(cov.in_no_group.killed_by_the_theory_alone))",
  "searching, with a group: candidates in no group \($g | total(cov.in_no_group.all)) on \($g | count(cov.in_no_group.all > 0)): alive alone \($g | total(cov.in_no_group.alive_alone)) on \($g | count(cov.in_no_group.alive_alone > 0)), dead alone \($g | total(cov.in_no_group.dead_alone))",
  "searching, with a group: components are not the groups on \($g | count(cov.components_are_groups | not)), are on \($g | count(cov.components_are_groups)); components \($g | total(cov.components))",
  "searching, with a group: no group has an open member on \($g | count(cov.open | all(.[0] == 0))); some group has none on \($g | count(cov.open | any(.[0] == 0))); groups with no open member \($g | map(cov.open | map(select(.[0] == 0) | .[1]) | add // 0) | add), with one \($g | map(cov.open | map(select(.[0] == 1) | .[1]) | add // 0) | add)",
  "searching: root's graph has a clique on \($s | count(cov.root_cliques.cliques > 0)); cliques \($s | total(cov.root_cliques.cliques)); root's cliques are not the groups' count on \($s | count(cov.root_cliques.cliques != cov.groups)); more of root's on \($s | count(cov.root_cliques.cliques > cov.groups)), fewer on \($s | count(cov.root_cliques.cliques < cov.groups)); the most a candidate is in \($s | map(cov.root_cliques.most) | max)",
  "searching, by rung (programs, with a group, groups, with no open member in any group):",
  ($s | group_by(.theory.rung)[] | "  \(.[0].theory.rung): \(length), \(count(cov.groups > 0)), \(total(cov.groups)), \(count(cov.groups > 0 and (cov.open | all(.[0] == 0))))"),
  "declared, searching with a group: groups of one relation \($g | total(cov.declared.groups.one_relation)) of \($g | total(cov.groups)); of a relation an hrule proposes \($g | total(cov.declared.groups.of_a_proposed_relation)); an obligation owes \($g | total(cov.declared.groups.of_an_owable_relation)); a closure takes out \($g | total(cov.declared.groups.of_a_closed_relation)); none of the first two \($g | total(cov.declared.groups.of_none_of_them)) on \($g | count(cov.declared.groups.of_none_of_them > 0))",
  "owed, searching: a table read on \($s | count(cov.owed.table != null)), with an instance on \($s | count(cov.owed.table > 0)); instances \($s | total(cov.owed.table // 0)): licensed \($s | total(cov.owed.licensed)), unlicensed \($s | total(cov.owed.unlicensed)); a group \($s | total(cov.owed.a_group)), inside a group \($s | total(cov.owed.inside_a_group)), not pairwise excluding \($s | total(cov.owed.not_excluding)) (holding a group \($s | total(cov.owed.holding_a_group))), one alternative \($s | total(cov.owed.one_alternative)), none \($s | total(cov.owed.no_alternative))",
  "owed, searching: groups that are an owed instance's alternatives \($s | total(cov.owed.groups_owed)) on \($s | count(cov.owed.groups_owed > 0)); groups nothing owes \($g | total(cov.owed.groups_nothing_owes)) on \($g | count(cov.owed.groups_nothing_owes > 0)); root owes and no table is read (flagged) on \($s | count(cov.owed.table == null and cov.owed.root_owes > 0)), \($s | map(select(cov.owed.table == null) | cov.owed.root_owes // 0) | add) instances",
  "owed, searching, unlicensed by why:",
  ($s | map(cov.owed.unlicensed_why | to_entries[]) | group_by(.key)[] | "  \(.[0].key): \(map(.value) | add)"),
  "owed, searching, by rule and standing:",
  ($s | map(cov.owed.by_rule | to_entries[] | .key as $r | .value | to_entries[] | {k: "\($r): \(.key)", v: .value}) | group_by(.k)[] | "  \(.[0].k): \(map(.v) | add)")
```

The per-program table:

```jq
# jq -r -f build/structure/s2/rows.jq build/structure/census.jsonl
def h: map("\(.[0])×\(.[1])") | join(" ");
select(has("error") | not) | select(.searches) | .theory.cover as $c | select($c.groups > 0)
| "| `\(.path)` | \(.theory.rung) | \(.n) | \($c.pairs) | \($c.groups) | \($c.sizes | h) | \($c.open | h) | \($c.membership | h) | \($c.in_no_group.alive_alone) | \($c.in_no_group.dead_alone) | \($c.components)\(if $c.components_are_groups then "" else " ≠" end) | \($c.steps) | \($c.root_cliques.cliques) | \($c.root_cliques.steps) | \($c.owed.table // "") | \($c.owed.a_group) | \($c.owed.unlicensed) | \($c.owed.groups_nothing_owes) |"
```

The control, `python3 build/structure/s2/control.py build/structure I-Z2=examples/zebra2 I-Z1=examples/zebra
I-Z2M=examples/zebra2-minus-15 I-SEND=examples/beyond_arith/send_more_money
I-B06=examples/branching/06_lookahead_on I-L02=examples/lattice/02_genuine_3set_death`:

```python
"""The control of the cover: the groups against the rows and columns read off the candidates' texts.

A candidate is `(rel x y)`; a line is the candidates of one relation that share x, or share y. This is the reading
by name the cover itself may not use, and here it is what the cover is held to.
    python3 build/structure/s2/control.py OUT TAG=PROGRAM...
"""
import json, sys
from collections import Counter

def control(tag, d):
    cands = [c['candidate'] for c in d['candidates']]
    lines = {}
    for v in cands:
        parts = v.strip('()').split()
        if len(parts) != 3:
            continue
        r, x, y = parts
        lines.setdefault((r, 0, x), set()).add(v)
        lines.setdefault((r, 1, y), set()).add(v)
    two = {frozenset(m): k for k, m in lines.items() if len(m) > 1}
    groups = [frozenset(g['members']) for g in d['groups']]
    per = Counter(len(c['groups']) for c in d['candidates'])
    are = [g for g in groups if g in two]
    not_lines = [sorted(g) for g in groups if g not in two]
    missing = [sorted(m) for m in two if m not in set(groups)]
    rels = sorted({k[0] for k in two.values()})
    by_axis = Counter((k[0], 'by first name' if k[1] == 0 else 'by second name') for g in are for k in [two[g]])
    print(f'{tag}: n {len(cands)}, groups {len(groups)}, lines of two or more {len(two)} over {len(rels)} relation(s), '
          f'groups that are lines {len(are)}, groups that are no line {len(not_lines)}, lines that are no group {len(missing)}, '
          f'candidates by groups {dict(sorted(per.items()))}, sizes {dict(sorted(Counter(map(len, groups)).items()))}, '
          f'open {dict(sorted(Counter(len(g["open"]) for g in d["groups"]).items()))}')
    print('   by relation and axis:', {f'{r} {a}': n for (r, a), n in sorted(by_axis.items())})
    for g in not_lines[:12]: print('   a group that is no line:', g)
    for m in missing[:12]: print('   a line that is no group:', m)

out = sys.argv[1]
for arg in sys.argv[2:]:
    tag, prog = arg.split('=')
    control(tag, json.load(open(f'{out}/{prog}/cover.json')))
```

The declared classes, `python3 build/structure/s2/declared.py build/structure build/structure/census.jsonl`:

```python
"""The groups against the classes of a declared key, read by name: a script, and not the engine.

A program states `(functional R)`, `(injective R)` or `(bijective R)` (under any module prefix) of a relation a
candidate is of. Its declared classes are the candidates `(R x y)` that share x, where R is functional or
bijective, and those that share y, where R is injective or bijective, of two or more. Each is set against the
cover's groups.
    python3 build/structure/s2/declared.py build/structure build/structure/census.jsonl
"""
import json, sys
out, rows = sys.argv[1], [json.loads(l) for l in open(sys.argv[2])]
tot = dict(programs=0, classes=0, groups=0, inside=0, not_clique=0, other_groups=0)
undeclared, named = [], []
for d in rows:
    c = d.get('theory', {}).get('cover') if 'error' not in d else None
    if not c or 'error' in c or not d.get('searches'): continue
    rel = d['path'][:-4]
    t = json.load(open(f'{out}/{rel}/theory.json')); cov = json.load(open(f'{out}/{rel}/cover.json'))
    fun, inj = set(), set()
    for s in t['stated']:
        f = s['fact'].strip('()').split()
        if len(f) == 2 and f[0].split('.')[-1] in ('functional', 'bijective'): fun.add(f[1])
        if len(f) == 2 and f[0].split('.')[-1] in ('injective', 'bijective'): inj.add(f[1])
    classes = {}
    for x in cov['candidates']:
        p = x['candidate'].strip('()').split()
        if len(p) != 3: continue
        if p[0] in fun: classes.setdefault((p[0], 0, p[1]), set()).add(x['candidate'])
        if p[0] in inj: classes.setdefault((p[0], 1, p[2]), set()).add(x['candidate'])
    classes = [frozenset(m) for m in classes.values() if len(m) > 1]
    groups = [frozenset(g['members']) for g in cov['groups']]
    excl = {frozenset(p) for p in t['theory_excluded']}
    clique = lambda m: all(frozenset((a, b)) in excl for a in m for b in m if a < b)
    if not classes:
        if groups: undeclared.append((d['path'], len(groups)))
        continue
    are = sum(m in groups for m in classes)
    inside = sum(m not in groups and any(m < g for g in groups) for m in classes)
    notc = sum(not clique(m) for m in classes)
    other = sum(g not in classes for g in groups)
    tot['programs'] += 1; tot['classes'] += len(classes); tot['groups'] += are; tot['inside'] += inside
    tot['not_clique'] += notc; tot['other_groups'] += other
    if are != len(classes) or other: named.append((d['path'], len(classes), are, inside, notc, len(groups), other))
print(f"searching programs with a declared class of two or more: {tot['programs']}; classes {tot['classes']}: a group {tot['groups']}, "
      f"strictly inside a group {tot['inside']}, not pairwise theory-excluding {tot['not_clique']}; groups of those programs that are no declared class {tot['other_groups']}")
print('| program | declared classes | … a group | … strictly inside one | … not pairwise excluding | groups | … no declared class |')
print('|---|---:|---:|---:|---:|---:|---:|')
for r in named: print('| `%s` | %d | %d | %d | %d | %d | %d |' % r)
print(f'searching programs with a group and no declared class: {len(undeclared)}, with {sum(n for _, n in undeclared)} groups')
for p, n in undeclared: print(f'  {p}: {n}')
```

What was run to hold the stage, each green:

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --lib --test cover --test structure_invariance --test pair_layer --test theory_share --test explore --test region
EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test structure_invariance
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-render --test id_order_invariance --test jobs_invariance
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test import_invariance
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test nightly_steps --test gate_steps --test config_reference
cargo fmt --manifest-path ein.rs/Cargo.toml --all --check
cargo clippy --manifest-path ein.rs/Cargo.toml --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -q --manifest-path ein.rs/Cargo.toml --workspace --no-deps
python3 utils/check_hashmap_iteration.py
python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md \
    examples/README.md stdlib/README.md utils/README.md c/README.md
```

## What it must not do

- **Not assume a partition**, anywhere, including in the names of fields.
- **Not read `is-a`** or any relation by name to form a group: a group is cliques of dead pairs and nothing
  else. What the program declares goes beside it, in its own columns.

## Notes

The two clique families of a bijection are the two projections of a bipartite relation, and *exactly one from
each row and each column* is a perfect matching. That is the vocabulary
[S1f.10.4](s1f.10.4_bounded_groups.md) needs; this stage does not assume it, since a cover is well defined
whether or not two families happen to be a grid, and I-Z1's single `co-located` relation is not obviously one.
