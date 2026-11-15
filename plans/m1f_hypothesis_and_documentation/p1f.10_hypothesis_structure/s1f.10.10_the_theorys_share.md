# S1f.10.10 — The theory's share: rules, ontology and the two facts

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** **built and measured 2026-11-15**: [§ Record](#record-2026-11-15). The small KB's rule is restated
from the one proposed below, which the corpus falsified, and it is a default, the user's to revise at the phase's
stop ([§ Defaults taken](#defaults-taken)). Planned 2026-11-15, at the re-plan. **Re-scoped 2026-11-15, after
[S1f.10.2](s1f.10.2_groups.md#record-2026-11-15-stopped-at-the-control) stopped at its control**: it runs before
S1f.10.2, the small-KB check reads **every pair of H** and not only the pairs root excludes (a pair with a member
dead alone at root has a row and a column too), and the theory's exclusions are computed by the owner module,
`ein_infer::structure`, in process, since S1f.10.2's groups are their cliques. `theory.ein` under `build/` is
that small KB written out for review, not the instrument.
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md), the excluded pairs at root.
**Read by:** [S1f.10.2](s1f.10.2_groups.md), whose groups are the cliques of what this stage finds; [S1f.10.12](s1f.10.12_the_structure_shown.md); [S1f.10.13](s1f.10.13_the_price_list.md), which
prices the small-KB check as one way of getting the pairs.

## Context

The user's instruction, 2026-08-28:

> It can be checked on small fact set of this 2 facts, without the rest of KB — taking rules, ontology and this
> 2 facts and check.

S1f.10.1 says which pairs exclude **at root**, with everything the program states in the KB. This stage says of
each such pair **why**: it is *of the theory* when rules, ontology and the two facts suffice, and *of the
puzzle* when it needs something the puzzle states. That is information of its own: `zebra2`'s grids are the
theory's and are the same for every puzzle over that ontology, and what the clues add is this puzzle's.

Two readings, and the stage builds both because each checks the other.

**The small KB.** The program with the puzzle's statements taken out, the two facts put in, saturated; the pair
is *of the theory* if that is dead. Its difficulty is the definition. In `zebra2.ein` a condition is one of
three things: a fact of a candidate's relation (conditions 9 and 10, `(drink-loc Milk House-3)`), the activator
of a rule with parameters (twelve of the fifteen, `(co-located nation-loc Englishman color-loc Red)`), or a
fact of the layout every rule reads (condition 1, the four `right-of` facts). The first two are the puzzle; the
third is ontology, and taking it out breaks the theory. No syntactic mark separates them: all three carry
`:source "condition (n)"`. The stage's proposed rule, to be falsified against the corpus:

> The small KB keeps every rule, every import, every `(relation …)`, every fact of a relation the program
> closes or no rule concludes and no candidate is of, and drops (a) every stated fact of a relation a candidate
> is of, and (b) every stated activator that names, as an argument, a relation a candidate is of.

It keeps `(is-a Red Color)`, `(right-of House-2 House-1)` and `(bijective color-loc)`, the last naming the
relation as its whole subject, which (b) must be worded to keep. It drops the two givens and the twelve
activators. It is wrong in some direction on some corpus program, and finding which is T2.

**The support of a death.** The node `{a, b}` is dead by a derivation, and the engine records what a
derivation rests on (provenance, the cores, [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs)).
The stated facts a death rests on are its support; a pair whose support holds no statement of the puzzle is of
the theory, and otherwise the support **names** the conditions, which is more than a yes or a no: *excluded by
condition (2)*. A support is an explanation, so it may vary with rule order, as a core does; it is sufficient,
not least.

**Each holds the other.** A pair the small KB kills must have, among its supports, one that lies inside the
small KB. And any support, put with the rules and the two facts and nothing else, must be dead: the user's
check, run on the support. A disagreement is a finding and the stage stops on it.

## Tasks

1. **T1f.10.10.1 — the small KB, by the proposed rule**, on the instances first: a program written to
   `build/structure/<program>/theory.ein`, loadable, and the pairs S1f.10.1 found re-read on it. Saturation is
   monotone in the facts stated, so a pair dead on the small KB must be dead at root; one that is not is a
   refutation resting on an `absent` that the smaller KB flips, which *forbid* is meant to have refused, and
   the stage stops and shows it.
2. **T1f.10.10.2 — where the rule is wrong.** Over the corpus programs that search: every program where the
   small KB does not load, where it has a dead root, or where a pair of one of the program's declared
   bijections is not of the theory. Each named, and the rule restated until the corpus does not falsify it, or
   the residue listed as programs the rule does not fit.
3. **T1f.10.10.3 — the support.** For each excluded pair, the stated facts its death rests on, read from the
   node the pair layer already computed, with the rule that concluded `(false)`. The check that makes it
   trustworthy runs on every pair of the instances: support, rules and the two facts, saturated, dead.
4. **T1f.10.10.4 — the two against each other**, and the record: per program, pairs of the theory, pairs of
   the puzzle by condition, and disagreements, which are zero or each explained by name.

## Acceptance

- Every excluded pair of every instance is marked *of the theory* or *by* a list of stated facts, in
  `build/structure/<program>/`, raw and as a table.
- **On I-Z2 every same-value and same-house pair of the five relations is of the theory**, and no pair that
  needs a condition is.
- **I-Z2 and I-Z2M have the same theory's share**: the two programs differ by condition (15) and nothing else,
  so the pairs of the theory among the candidates both have are the same pairs. An invariance, held by a test
  on the pair.
- The small KB's rule is written here as the corpus left it, with the programs it does not fit.
- The disagreement count is zero, or every non-zero cell is explained by name.
- Each number is recorded with its command and date. No answer, route or surface changes.

## What it must not do

- **Not spend a theory's pair in the search.** Track R spends pairs that are nodes of the program's own
  lattice (S1f.10.3); this stage classifies them.
- **Not read `:source`** to decide what the puzzle states: it marks condition (1) too.
- **Not make the support canonical.** That is F22's, for cores, and the same answer will serve here.

## Notes

The predicate has a name in the literature: a binary constraint between two CSP variables, and the graph of
allowed pairs is the *microstructure*. [`docs/lib/`](../../../docs/lib/README.md) is where that reading is
catalogued.

## Record (2026-11-15)

### What was built

- **The owner**, [`ein-infer/src/structure.rs`](../../../ein.rs/crates/ein-infer/src/structure.rs), beside the
  pair layer. `theory(root, terms, ast, &TheoryOptions) -> Theory` takes a freshly loaded program;
  `theory_by` is the same with the small KB's rule a parameter, which is how the census measures a rejected
  alternative. A `Theory` holds root's `PairLayer`; `stated`, every fact the loaded program holds with whether
  the small KB keeps it and by which clause; **`small`, the small KB's own `PairLayer` over root's
  candidates**, whose `excluded` are the exclusions the theory makes alone over all of H and whose `dead_alone`
  are the candidates it kills with no clue; `against_root` and `small_beyond_root`, the monotonicity check;
  `supports`, one per pair root excludes; `rung` and `rules_naming_objects`. `Theory::of_the_theory()` marks
  root's excluded pairs, `Theory::readings()` sets the two readings against each other, and `Deaths` reads a
  layer for a comparison. **What [S1f.10.2](s1f.10.2_groups.md) reads is `Theory::small`**, and
  `TheoryOptions { root_pairs: false, .. }` gives it without root's own singles and pairs.
- **How the small KB is built in process.** `Loaded::of` copies the loaded program's registries and its fact
  list before anything saturates root; `Loaded::over(keep)` is a new `Kb` over a copy of the registries with the
  facts `keep` keeps, in the order written, each with its own record, `(written T)` stated again as the loader
  states it (`ein_ir::written_in` and `written_in_conclusions`, the loader's two halves made callable, with no
  change to what a load does). It is built over the same `Terms`, so a candidate is one `FactId` in the program
  and in the small KB.
- **Through which node function.** The routes' own: `region::prepare_over` saturates the small KB's root as
  `prepare` does, *forbid* included, and takes the universe the caller names where `prepare` asks the generator;
  `explore::Space::open` over it is the node cache the routes read, and `structure::layer` is the pair layer's
  own function over that space, L1 and L3 deciding what they decide where the small KB is monotone. No second
  node function, and no route calls any of it.
- **The support**: `explain::support_frontier`, the core's search with two things read otherwise. Every
  firing's activator is followed, root's among them, where a core follows only one a node derived or committed
  (Q-M1e2.12 (c)); and a fact the loaded program stated is a leaf whatever its record says. It is read from the
  pair's own node, forked and saturated again as a core taken after a walk is (`region::Nodes::support`,
  `Space::supports`). Not a second explanation: one more reading of `explain.rs`'s label search, which a core
  does not take.
- **The vehicle**: [`pair_layer`](../../../ein.rs/crates/ein-infer/examples/pair_layer.rs) with `--theory`,
  `--supports`, `--check` and `--rule NAME`. The line gets a `theory` object, and
  `build/structure/<program>/` gets `theory.json` (raw), `theory.md` (tables) and `theory.ein`.
- **The tests**: [`ein-infer/tests/theory_share.rs`](../../../ein.rs/crates/ein-infer/tests/theory_share.rs),
  ten, and three in the module ([§ The tests](#the-tests)).

### The rule, as the corpus left it

The proposed rule did not survive ([§ The alternatives](#the-alternatives-and-what-falsifies-each)). What
`structure::small_kb` applies, verbatim from its doc comment:

> The small KB keeps every rule, every declaration and every import, and of the facts a file writes it drops
> **(a) a fact of a candidates' relation**, stated or denied: a given. Of the others:
>
> - where **the program's own rung built `H`**, the relations that rung reads are what the universe is made
>   of, and a fact of one is kept: it says what kind of thing an object is. **(b) Every other fact that names
>   one of the candidates' objects** is dropped: it says something of that object which the universe does not
>   need;
> - where **the blind enumerator built `H`**, nothing says what the universe is made of, and the rule drops
>   less: **(c) a fact that names a candidates' relation together with one of the candidates' objects**, which
>   speaks of that relation's facts at one of its arguments, and **(d) a fact that names two or more of the
>   candidates' objects**, which relates them. A fact that names one object and no candidates' relation is
>   kept, as a type is.
>
> A fact that names none of the candidates' objects is kept under either: a declaration that names a relation
> and no object speaks of the relation whole.

Its three terms, each a function of the program as loaded and as root leaves it, none a name:

- **a candidates' relation**: a relation a candidate of H is of, and, where the program's own hrules or
  obligations built H, one that rung can still propose a fact of (`hypgen::eligible_relations`). The second
  half is needed (`--rule h-alone`, below): `zebra2-hints.ein` states the colours, root decides every
  `color-loc` fact, no candidate of `color-loc` is left, and a rule that reads H alone keeps `(adjacent-via
  right-of color-loc Ivory color-loc Green)` and drops the row of houses it reads the absence of;
- **a candidates' object**: a name among the arguments, at any depth, of a candidate, or of a fact of a
  candidates' relation root's fixpoint **holds**. Not one it denies, which is the narrower reading and a choice:
  a rule that excludes over a whole type denies facts of names no candidate holds,
  `branching/05_mini_zebra.ein`'s `(not (co-located Nationality Drink))`, but no corpus program's share differs
  between the two readings under the rule as it stands;
- **the rung that built H, and what it reads**: `oblgen::Mode`, and the relations its plans read (the hrules'
  premises; the obligations' triggers and witnesses).

On `zebra2.ein` it drops 18 of the 66 facts the files state: conditions (9) and (10) by (a), and by (b) the
twelve activators and the four `right-of` facts of condition (1). It keeps the 37 `is-a` facts, which the hrule
`guess` reads, and `(bijective color-loc)`, `(symmetric next-to)`, `(includes right-of next-to)`, which name no
object. **The row of houses goes, against the plan's expectation**: it relates two of the candidates' objects
as a clue does, and on `zebra.ein` the row and condition (6), `(right-of Green Ivory)`, are facts of one
relation. Nothing derives from it once the clues are gone, and the theory's pairs are the same with it
(`--rule layout`, below).

**What the rule cannot tell**, each a class of programs it does not fit, with the count over the 188 that
search (the `jq` program under [§ The commands](#the-commands)):

| class | programs | what happens there |
|---|---:|---|
| **a clue written as a rule** | 49 have a saturation rule whose text names one of the candidates' objects | the small KB keeps every rule. `beyond_arith/equation_two_unknowns.ein` states `3x + 5y = 47` in three rules over `x1`, `x0`, `y0`; its theory's 135 pairs are a place's two digits all the same, because that rule is a conflict of three. The count is an upper bound on the class: `send_more_money.ein`'s three are `leading-zero`, `carry-first` and `add-column-alone`, which name the digit `0` |
| **the blind enumerator's universe** | 98 (89 `blind`, 9 `declined`) | H holds facts of every open relation over every name, so a type's name and a relation's are candidates' objects ([S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md)'s E-1 and E-7), and a relation the ontology is written in is a candidates' relation: on I-B06 90 of the 132 candidates are `is-a` facts, the program's twelve `is-a` facts are dropped by (a), and the 360 pairs the theory excludes are an object's two types, `(is-a Blue Green)` with `(is-a Blue H1)`, by `(functional is-a)`. A clue that names one object is kept there, as a type is |
| **a clue that names none of the candidates' objects** | not counted: nothing tells one from a declaration | kept. `binary_logic/satisfiable.ein`'s `(gate o1 Or g1 g2 cout)` is between wires. It derives nothing once the gates that tie a wire to an input are gone |
| **a module's table over the candidates' objects** | 16 drop a fact a module writes | dropped with the clues: `std.math.digits`' 1 664 rows over the digits a letter may be, `std.bool.gates`' rows over `0` and `1`. Nothing in a loaded fact says which file wrote it ([`findings.md`](findings.md)'s K-5), and the rules that read a table have nothing to join it with once the clues are gone: on 11 of the 16 the program that keeps the module's facts has the small KB's layer. On the other 5 the module holds the puzzle itself, `binary_logic/full_adder.ein`'s gates |
| **a flagged program whose small KB is monotone** | 3: `saturation/square-unique/corner-house`, `cul-de-sac`, `terminus` | the non-monotone site is in what the small KB drops. The small KB's layer is then licensed where root's is not, and it is still no more than a set of statements about another program's nodes |

No program is in any of the classes the plan asked for by name: **the small KB loads, and has a layer, on all
625 programs read; none has a dead root where root's is alive; none is flagged where the program is not; none
is stopped by the budget; and nothing is dead in a small KB that is alive at root.**

### The alternatives, and what falsifies each

Each is a rule the example still holds, `--rule NAME`, so each falsification can be taken again:

```sh
for r in proposed objects relates layout h-alone; do
  ein.rs/target/release/examples/pair_layer --theory --rule $r --out build/structure/s10/$r > build/structure/s10/$r.jsonl
  jq -s -r -f alt.jq build/structure/s10/$r.jsonl     # alt.jq is under § The commands
done
```

2026-11-15, the working tree over `d68c333b`; every figure is a count.

| rule | what it says | the program that falsifies it |
|---|---|---|
| `proposed` | the stage's own: (a), and a fact that names a candidates' relation with an object, any name that is no relation's and no rule's; all else kept | **`zebra.ein`** in both directions: it drops `(slot-partition co-located instance type Attribute House)` and the two `slot-spatial` declarations, which name the relation and a type, so **the theory excludes no pair, 0 of root's 414**; and it keeps the four clues stated as facts of `right-of` and `next-to`. **`beyond_arith/cryptarithm.ein`**: it drops none of 1 704 facts, the small KB is the program, and all 215 of root's excluded pairs are the theory's where one column excludes 71. `binary_logic/knights.ein`, 5 of 5 where 2 need the gates; `lattice/07_two_pairs_two_tables.ein`, 64 of 64 where 16 need the tables |
| `objects` | a fact that names one candidates' object is the puzzle's, but for a fact of a relation the rung reads, under every rung | **nine programs where the small KB says more than the program**, every one under the blind enumerator, three with no rung of their own and six whose obligation the ladder declines: `branching/12_typed_blind_solve.ein` and `features/05_stdlib_domain_elim.ein` (the `is-a` facts go, and candidates alive at root are dead in the small KB), `ein-bugs/wrong-answers/oblig-derived-domain`, `oblig-fresh-name`, `oblig-fresh-name-two` and their `-declared` twins, and `wild-in-absent-twin.ein`, whose small KB holds a fact root's fixpoint does not |
| `relates` | what `small_kb` says under the blind enumerator, under every rung: a fact that names one object and no candidates' relation is kept | **`lattice/07_two_pairs_two_tables.ein`**: `(member Ann P1)` and `(at S1 T1)` each name one object, a rule joins them through the group and the table, and all 64 of root's excluded pairs are the theory's where 48 are a friend's two seats or a seat's two friends. `send_more_money.ein`: `(leading S)` is kept and the theory kills `(val S 0)` and `(val M 0)` alone |
| `h-alone` | `small_kb`, with a candidates' relation one a candidate of H is of and no other | **`zebra2-hints.ein`**: the small KB's root is dead, and 18 candidates and 98 pairs are dead in it and alive at root |
| `layout` | `small_kb`, but a fact of a relation a rule reads the absence of is kept whatever it names: the row of houses stays | **`zebra.ein`**: its row and its conditions (6), (11), (12) and (15) are facts of the same two relations, so the theory excludes 149 pairs where the other encoding of the same puzzle has 128, 21 of them by a condition. On `zebra2.ein` it is the stage's hand-made probe exactly, 14 dropped, and gives the same 128 |

### The instances

`n` is H; *stated* the facts the files write, *dropped* those the small KB drops; then the small KB's layer and
its work **in its own root's terms**, beside root's.

| | n | pairs of H | stated | dropped | the theory kills alone | **the theory excludes** | … a member dead alone at root | root excludes | **of the theory** | **of the puzzle** | small KB: pairs saturated | saturations | facts beyond its root | root's layer: saturations | facts |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **I-Z1** `zebra.ein` | 56 | 1 540 | 61 | 18 | 0 | **128** | 72 | 414 | 56 | 358 | 1 540 | 1 597 | 67 244 | 570 | 181 088 |
| **I-Z2** `zebra2.ein` | 56 | 1 540 | 66 | 18 | 0 | **128** | 83 | 327 | 45 | 282 | 1 540 | 1 597 | 32 086 | 465 | 68 385 |
| **I-Z2M** `zebra2-minus-15.ein` | 96 | 4 560 | 65 | 17 | 0 | **318** | 0 | 1 428 | 318 | 1 110 | 4 560 | 4 657 | 92 244 | 4 546 | 475 193 |
| **I-B06** `branching/06_lookahead_on.ein` | 132 | 8 646 | 20 | 16 | 0 | 360 | 360 | 0 | 0 | 0 | 8 625 | 8 758 | 24 508 | 133 | 440 |
| **I-L02** `lattice/02_genuine_3set_death.ein` | 3 | 3 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 7 | 9 | 7 | 9 |
| **I-SEND** `beyond_arith/send_more_money.ein` | 80 | 3 160 | 1 713 | 1 671 | 0 | **640** | 31 | 609 | 609 | 0 | 3 160 | 3 241 | 7 040 | 3 084 | 6 897 |

- **I-Z2, the control.** **The theory's pairs are exactly the 128 row and column pairs**: every pair of two
  candidates of one `*-loc` relation with the same value or the same house, over all 56 candidates, and no other
  pair. 45 of them root excludes; the other 83 hold a member dead alone at root. None of root's other 282
  excluded pairs is the theory's, and each has a support that names a condition. The plan's two figures, 128 and
  45, are `the_theory_excludes_a_declared_bijections_rows_and_columns`.
- **I-Z2M.** 318, every row and column pair of its 96 candidates, none dead alone, all excluded at root too.
  **Over the 56 candidates both programs have, the two theories are the same 128 pairs and no single**
  (`two_programs_one_clue_apart_have_one_theory`).
- **I-Z1.** One relation and `std.slots`, and **the same 128 pairs as I-Z2 under the renaming** `(co-located X
  House-n)` to `(R-loc X House-n)`: the two programs propose the same 56 placements and their theories exclude
  the same pairs of them (`the_same_puzzle_in_two_encodings_has_one_theory`). The plan did not ask this; it is
  the strongest evidence here that the rule reads the puzzle and not the encoding.
- **I-SEND.** 640: a letter's two digits (8 × 45) and a digit's two letters (10 × 28), over all 80 candidates.
  Root's 609 are those of them with both members alive alone; the 31 others hold `(val M 0)` or `(val S 0)`,
  which root kills alone and the theory does not, since `(leading S)` and `(leading M)` are the puzzle's.
- **I-L02.** No pair at root and none in the small KB, which is the program: nothing is dropped.
- **I-B06.** The blind enumerator's class, above: 16 of 20 facts dropped by (a), no pair at root, 360 in the
  small KB, each two `is-a` candidates of one object, both dead alone at root.

**Read ahead for S1f.10.2**, by its own script's enumerator over `theory.json`'s `theory_excluded` (2026-11-15):
the maximal cliques of the theory's exclusions are 36 on I-Z2 and 36 on I-Z1, sizes 2 to 4, **every candidate
in exactly two**; 46 on I-Z2M, 2 to 5, every candidate in two; 18 on I-SEND, the eight letters and the ten
digits, every candidate in two; and 10 of nine members on I-B06, 90 candidates in one and 42 in none. The
control S1f.10.2 stopped at holds of this object.

### The corpus

**644** files load, *forbid* refuses **19**, and the share of the other **625** is read, 188 of which search.
Over the 188:

| | programs |
|---|---:|
| the small KB drops a statement | 120 |
| with a pair the theory excludes | 53 |
| with a candidate the theory kills alone | 36 |
| with a pair root excludes | 44 |
| … every one of them the theory's | 27 |
| … some | 14 |
| … none: `branching/02_one_dead_one_alive`, `branching/04_two_levels`, `lattice/01_subset_pruned` | 3 |

Of the 93 208 pairs of H, root excludes 5 565, **2 081 of them of the theory and 3 484 of the puzzle**; the
theory excludes 3 906, 1 825 of which hold a member dead alone at root. By the rung that built H (programs;
dropping a statement; with a theory's pair; with a candidate the theory kills alone): hrules 70, 39, 31, 9;
obligations 20, 12, 12, 2; blind 89, 68, 10, 16; declined 9, 1, 0, 9. The eleven programs under a declared
rung whose theory kills a candidate alone are fixtures whose rules do it: `ein-bugs/alive-empty-phase1` and ten
under `ein-bugs/wrong-answers/`.

**A declared functional, injective or bijective relation's lines are the theory's.** 34 corpus programs state
`(functional R)`, `(injective R)` or `(bijective R)` of a relation a candidate is of, and 4 260 pairs of H lie
along such a line. Every one is dead in the small KB, but for the 6 of `syntax/constraint-scopes.ein`, a syntax
fixture that states `(bijective next-to)` and imports no rule that reads it: root excludes none of them either.
The script that reads the names is under [§ The commands](#the-commands); the engine reads none.

Every program with a pair, at root or in the small KB:

| program | rung | n | stated | dropped | the theory kills alone | the theory excludes | … a member dead alone at root | root excludes | of the theory | of the puzzle | small KB: saturations | facts | root's layer: saturations | facts |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `examples/beyond_arith/cryptarithm.ein` | hrules | 30 | 1704 | 1669 | 0 | 165 | 21 | 215 | 144 | 71 | 466 | 1065 | 409 | 1246 |
| `examples/beyond_arith/equation_by_search.ein` | hrules | 30 | 1699 | 1671 | 0 | 135 | 45 | 99 | 90 | 9 | 466 | 1035 | 241 | 598 |
| `examples/beyond_arith/equation_no_solution.ein` | hrules | 20 | 1696 | 1669 | 0 | 90 | 45 | 55 | 45 | 10 | 211 | 490 | 76 | 224 |
| `examples/beyond_arith/equation_two_unknowns.ein` | hrules | 30 | 1688 | 1664 | 0 | 135 | 0 | 135 | 135 | 0 | 466 | 1035 | 466 | 1035 |
| `examples/beyond_arith/send_more_money.ein` | hrules | 80 | 1713 | 1671 | 0 | 640 | 31 | 609 | 609 | 0 | 3241 | 7040 | 3084 | 6897 |
| `examples/binary_logic/equivalent.ein` | obligations | 6 | 107 | 78 | 0 | 3 | 0 | 3 | 3 | 0 | 22 | 75 | 22 | 87 |
| `examples/binary_logic/knights.ein` | obligations | 6 | 103 | 77 | 0 | 3 | 0 | 5 | 3 | 2 | 22 | 75 | 22 | 97 |
| `examples/binary_logic/not_equivalent.ein` | obligations | 6 | 105 | 77 | 0 | 3 | 0 | 3 | 3 | 0 | 22 | 75 | 22 | 87 |
| `examples/binary_logic/satisfiable.ein` | obligations | 6 | 104 | 77 | 0 | 3 | 0 | 3 | 3 | 0 | 22 | 75 | 22 | 83 |
| `examples/binary_logic/synthesis.ein` | obligations | 16 | 142 | 85 | 0 | 120 | 119 | 1 | 1 | 0 | 137 | 376 | 18 | 169 |
| `examples/binary_logic/truth_table.ein` | obligations | 6 | 103 | 75 | 0 | 3 | 0 | 3 | 3 | 0 | 22 | 75 | 22 | 83 |
| `examples/binary_logic/unsatisfiable.ein` | obligations | 6 | 105 | 78 | 0 | 3 | 0 | 4 | 3 | 1 | 22 | 75 | 22 | 84 |
| `examples/branching/01_saturate_only.ein` | blind | 16 | 12 | 8 | 0 | 12 | 12 | 0 | 0 | 0 | 135 | 348 | 17 | 56 |
| `examples/branching/02_one_dead_one_alive.ein` | blind | 48 | 11 | 7 | 0 | 44 | 44 | 56 | 0 | 56 | 1166 | 3692 | 185 | 1136 |
| `examples/branching/03_five_hyps_one_alive.ein` | blind | 132 | 20 | 16 | 0 | 360 | 360 | 0 | 0 | 0 | 8758 | 24508 | 133 | 440 |
| `examples/branching/04_two_levels.ein` | blind | 46 | 13 | 9 | 0 | 60 | 60 | 16 | 0 | 16 | 1074 | 3112 | 71 | 236 |
| `examples/branching/05_mini_zebra.ein` | hrules | 9 | 15 | 2 | 0 | 6 | 6 | 0 | 0 | 0 | 46 | 204 | 13 | 60 |
| `examples/branching/06_lookahead_on.ein` | blind | 132 | 20 | 16 | 0 | 360 | 360 | 0 | 0 | 0 | 8758 | 24508 | 133 | 440 |
| `examples/branching/07_lookahead_off.ein` | blind | 132 | 20 | 16 | 0 | 360 | 360 | 0 | 0 | 0 | 8758 | 24508 | 133 | 440 |
| `examples/branching/08_hypothesis_relation_whitelist.ein` | blind | 38 | 17 | 10 | 0 | 24 | 0 | 136 | 24 | 112 | 723 | 3404 | 335 | 2676 |
| `examples/branching/09_hrule.ein` | hrules | 7 | 14 | 2 | 0 | 10 | 10 | 0 | 0 | 0 | 29 | 118 | 8 | 34 |
| `examples/branching/10_kill_cache_on.ein` | hrules | 18 | 19 | 3 | 0 | 12 | 12 | 0 | 0 | 0 | 172 | 792 | 34 | 192 |
| `examples/branching/11_kill_cache_off.ein` | hrules | 18 | 19 | 3 | 0 | 12 | 12 | 0 | 0 | 0 | 172 | 792 | 34 | 192 |
| `examples/branching/12_typed_blind_solve.ein` | blind | 51 | 18 | 3 | 47 | 4 | 0 | 4 | 4 | 0 | 58 | 428 | 56 | 664 |
| `examples/branching/15_lookahead_two_step_on.ein` | hrules | 3 | 6 | 3 | 0 | 2 | 0 | 2 | 2 | 0 | 7 | 12 | 7 | 12 |
| `examples/branching/16_lookahead_two_step_off.ein` | hrules | 3 | 6 | 3 | 0 | 2 | 0 | 2 | 2 | 0 | 7 | 12 | 7 | 12 |
| `examples/domain_elim/b_branch.ein` | hrules | 7 | 11 | 2 | 0 | 10 | 10 | 0 | 0 | 0 | 29 | 59 | 8 | 13 |
| `examples/ein-bugs/complete-records-stale.ein` | hrules | 3 | 3 | 0 | 0 | 2 | 0 | 2 | 2 | 0 | 7 | 11 | 7 | 11 |
| `examples/ein-bugs/fork-delta-order.ein` | hrules | 2 | 82 | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 4 | 165 | 4 | 165 |
| `examples/ein-bugs/unsat-core-id-space.ein` | blind | 5 | 2 | 1 | 0 | 6 | 5 | 1 | 1 | 0 | 16 | 31 | 7 | 11 |
| `examples/ein-bugs/wrong-answers/einb-dump-answer.ein` | blind | 2 | 2 | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 4 | 5 | 4 | 5 |
| `examples/ein-bugs/wrong-answers/forall-core-elim.ein` | hrules | 2 | 4 | 1 | 0 | 1 | 1 | 0 | 0 | 0 | 4 | 12 | 3 | 4 |
| `examples/ein-bugs/wrong-answers/forall-core-false.ein` | hrules | 2 | 3 | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 4 | 9 | 4 | 9 |
| `examples/features/11_expect_ambiguity.ein` | hrules | 4 | 6 | 0 | 0 | 4 | 0 | 4 | 4 | 0 | 11 | 20 | 11 | 20 |
| `examples/features/13_mixed_solution_and_open.ein` | hrules | 2 | 5 | 1 | 0 | 1 | 0 | 1 | 1 | 0 | 4 | 7 | 4 | 7 |
| `examples/lattice/01_subset_pruned.ein` | hrules | 4 | 7 | 1 | 0 | 0 | 0 | 1 | 0 | 1 | 11 | 16 | 11 | 17 |
| `examples/lattice/05_owed_seats_bounded.ein` | obligations | 4 | 8 | 0 | 0 | 4 | 0 | 4 | 4 | 0 | 11 | 20 | 11 | 20 |
| `examples/lattice/06_friends_tables.ein` | hrules | 19 | 31 | 12 | 0 | 73 | 6 | 67 | 67 | 0 | 191 | 434 | 191 | 447 |
| `examples/lattice/06_friends_tables_everyone_sits.ein` | hrules | 19 | 32 | 12 | 0 | 73 | 6 | 67 | 67 | 0 | 191 | 434 | 173 | 393 |
| `examples/lattice/07_two_pairs_two_tables.ein` | hrules | 16 | 23 | 8 | 0 | 48 | 0 | 64 | 48 | 16 | 137 | 304 | 137 | 320 |
| `examples/lattice/synth/merged_states.ein` | hrules | 5 | 8 | 2 | 0 | 1 | 0 | 1 | 1 | 0 | 14 | 30 | 14 | 30 |
| `examples/lattice/synth/one_model_per_component.ein` | hrules | 4 | 9 | 4 | 0 | 6 | 0 | 6 | 6 | 0 | 11 | 22 | 11 | 22 |
| `examples/word_problems/boxes12.ein` | hrules | 2 | 1681 | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 4 | 5 | 4 | 5 |
| `examples/word_problems/boxes13.ein` | hrules | 2 | 1681 | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 4 | 37 | 4 | 37 |
| `examples/zebra.ein` | hrules | 56 | 61 | 18 | 0 | 128 | 72 | 414 | 56 | 358 | 1597 | 67244 | 570 | 181088 |
| `examples/zebra2-hints.ein` | hrules | 36 | 66 | 20 | 0 | 72 | 62 | 55 | 10 | 45 | 667 | 13842 | 94 | 12367 |
| `examples/zebra2-minus-15-obligations.ein` | obligations | 96 | 65 | 17 | 0 | 318 | 0 | 1428 | 318 | 1110 | 4657 | 92244 | 4546 | 475193 |
| `examples/zebra2-minus-15.ein` | hrules | 96 | 65 | 17 | 0 | 318 | 0 | 1428 | 318 | 1110 | 4657 | 92244 | 4546 | 475193 |
| `examples/zebra2-obligations.ein` | obligations | 56 | 66 | 18 | 0 | 128 | 83 | 327 | 45 | 282 | 1597 | 32086 | 465 | 68385 |
| `examples/zebra2.ein` | hrules | 56 | 66 | 18 | 0 | 128 | 83 | 327 | 45 | 282 | 1597 | 32086 | 465 | 68385 |
| `tests/region/02_merged_single.ein` | hrules | 5 | 8 | 2 | 0 | 1 | 0 | 1 | 1 | 0 | 14 | 30 | 14 | 30 |
| `tests/region/09_query_over_maximal.ein` | hrules | 4 | 6 | 0 | 0 | 4 | 0 | 4 | 4 | 0 | 11 | 20 | 11 | 20 |
| `tests/stdlib/bool/inputs/01_in_drives.ein` | obligations | 4 | 97 | 72 | 0 | 2 | 0 | 2 | 2 | 0 | 11 | 34 | 11 | 38 |
| `tests/stdlib/bool/inputs/02_required.ein` | obligations | 4 | 100 | 75 | 0 | 2 | 0 | 5 | 2 | 3 | 11 | 34 | 11 | 45 |

### The support, and the two readings

**How it is read.** For each pair root excludes, the pair's node is forked from root and saturated again, and
the support's search labels its contradiction: the frontier is the stated facts and the commitments the
recorded derivation rests on, with the rules that end it. The supports are in `theory.json` under each pair,
split into the statements the small KB drops (*by these*) and those it keeps; `theory.md` prints the first by
their `:source` where they carry one, which decides nothing. On I-Z2 `{(color-loc Green House-4), (color-loc
Red House-3)}` is *by* condition (1), the fact `(right-of House-4 House-3)`, and condition (6), with
`(bijective color-loc)` kept and `std.algebra.injective` the rule that ends it. **A support is sufficient, not
least, and not canonical**: 25 of the 5 565 were cut at the search's budget of 64 environments a fact, on four
programs (`zebra.ein` 8, `zebra2.ein` 7, `zebra2-obligations.ein` 7, `zebra2-hints.ein` 3), and name a real
frontier that may not be the smallest. None passed the budget's count of facts, where no support is read.

**The two readings, over all 5 565 pairs root excludes in the corpus:**

| | pairs |
|---|---:|
| dead in the small KB, and the support lies inside it | 2 081 |
| dead in the small KB, and the support names a statement the small KB drops | **0** |
| alive in the small KB, and the support names a statement the small KB drops | 3 484 |
| **alive in the small KB though the support lies inside it: a disagreement** | **0** |
| no support read | 0 |

So the disagreement count is zero, and there is no cell to explain. The second row was allowed to be anything,
a support being sufficient and not least; it is zero too.

**The check**, on every one of the 5 565, the instances' among them (I-Z1 414, I-Z2 327, I-Z2M 1 428, I-SEND
609; I-B06 and I-L02 have none): the small KB with the statements the support names put back, and the pair
committed, is dead; its own root is alive; and the pair's node there holds no fact the program's own node of
the pair does not, so the death is the program's. 959 check programs were built, one per distinct set of named
statements.

**The check needed one thing the plan did not have**, and it is a default
([§ Defaults taken](#defaults-taken), 4). A support names the facts a derivation *matched*. A firing that passed
an `absent` rests on the whole of what the program states of the relation it read, and no premise records
that. A check over the small KB and the named statements alone states half a row of houses, its rules conclude
from the houses that are missing, and its root is dead before the pair is committed. So every check puts back,
beside the statements a support names, every statement the small KB drops **of a relation a rule reads the
absence of** (`Stated::read_absent`: a relation read inside an `absent` by a saturation plan of root's, and
what such a relation is derived from). That is the four `right-of` facts on I-Z2 and I-Z2M. On I-Z1 it is
eight, the row and conditions (6), (11), (12) and (15), which are facts of the same two relations: there every
check has those four conditions whether its support names them or not, and it shows less.

### `theory.ein`, and the binary

`build/structure/<program>/theory.ein` is the program's text with every top-level form deleted that writes a
fact the small KB drops. The example loads it again through `ein_ir::load` and compares the facts it loads to,
unsaturated, with the facts the engine built the small KB from: **625 of 625 agree**. Two qualifications, both
counted by the example:

- **A module's fact cannot be deleted from the program's text.** On the 16 programs where the small KB drops
  one, `theory.ein` loads to the small KB with the module's facts beside it, and that is what it is compared
  with; `same_layer_with_the_modules_facts` says whether that program's layer over H is the small KB's. It is
  on 11, I-SEND among them, and not on the 5 `binary_logic` programs whose puzzle is the module
  `full_adder.ein`.
- **A module's name is a path from the program's directory**, so 38 programs' `theory.ein` (9 that search)
  loads from the program's directory and not from `build/structure/`. None is an instance.

**And the `ein` binary, on the instances.** For each of the six, every candidate and every pair of H was
appended to `theory.ein` as stated facts and given to `ein solve --no-search`, whose `∅` row says whether that
root is dead: 19 872 runs. That is the from-scratch reading of a commitment where the engine's small KB reads a
fork. **No single and no pair differs** (the script and its output are under
[§ The commands](#the-commands)): 128 dead on I-Z2 and on I-Z1, 318 on I-Z2M, 640 on I-SEND, 360 on I-B06,
none on I-L02, and `theory.ein`'s own root alive on all six.

### What it costs

In the small KB's own root's terms, beside the pair layer's, per program in the table above. Over the 188: root's
layers 49 015 saturations and 1 389 866 facts beyond root, 46 140 pairs saturated; **the small KBs' 82 589
saturations and 513 855 facts beyond their own roots, 79 714 pairs saturated**. The small KB's layer is dearer
in saturations on 42 programs and cheaper on none, and cheaper in facts: fewer of its candidates are dead
alone, so L1 decides fewer pairs, and each node derives less. I-Z2 is the shape of it, 1 597 saturations
against 465, 32 086 facts against 68 385. **No layer of either kind is stopped by the default budget.**

A support costs its node a second saturation and a search: 5 565 over the corpus, and the checks 959 roots and
5 565 nodes, 619 615 facts beyond their roots. In time the support is the dear part, and the example's wall
for an instance at each depth, the same six files each time
(`utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer [--theory | --supports | --check] --out build/structure/s10/walls FILE…`,
2026-11-15, one P-core, on AC, load average 1.7 to 2.8):

| | the layer alone | `--theory` | `--supports` | `--check` |
|---|---:|---:|---:|---:|
| I-Z1 | 2.01 s | 2.45 s | 4.76 s | 6.53 s |
| I-Z2 | 0.99 s | 1.30 s | 9.31 s | 10.21 s |
| I-Z2M | 5.96 s | 6.80 s | 32.36 s | 34.93 s |
| I-SEND | 0.07 s | 0.25 s | 0.26 s | 0.27 s |

I-B06 is 0.05 s and I-L02 under 0.01 s at every depth. The theory's share is a third again on the layer or
less. A support is 25 ms a pair on I-Z2: the search walks root's derivation again for every pair, with root's
activators in every environment. [S1f.10.13](s1f.10.13_the_price_list.md) prices it.

### The tests

In the gate, `cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test theory_share`,
and each is an invariance or a control, none a comparison with an earlier answer:

| test | what it compares | wall, alone |
|---|---|---:|
| `the_theory_excludes_a_declared_bijections_rows_and_columns` | I-Z2: the theory's pairs against the 128 line pairs read off the candidates' texts, both ways; of root's 327, the theory's are the 45 along a line; the other lines hold a member dead alone; what is dropped and what is kept | 1.8 s |
| `two_programs_one_clue_apart_have_one_theory` | I-Z2 and I-Z2M: the singles killed and the pairs excluded over the candidates both have | 1.5 s |
| `the_same_puzzle_in_two_encodings_has_one_theory` | I-Z1 and I-Z2 under the renaming: one universe, one set of 128 pairs | 1.0 s |
| `the_theory_derives_no_more_than_the_program` | **monotonicity, over every corpus program the pair layer's per-commit sweep reads**: 612 programs, 118 whose small KB drops a statement, 51 with a theory's pair (2 860 pairs), 27 flagged. Fails on a small KB with no layer, a stopped layer, a dead root where root's is alive, a fact beyond root's fixpoint, or a single or a pair dead in the small KB and alive at root, each named. Through `ein_corpus::pool::map`. It has no slow half | 8.4 s |
| `the_readings_agree_on_the_declared_control`, `…_on_one_relation`, `…_on_the_cryptarithms`, `a_clue_through_a_name_no_candidate_holds_is_the_puzzles` | **the support check**, on I-Z2, I-Z1, I-SEND, `cryptarithm.ein` and `lattice/07`: every excluded pair has a support that names both members; no disagreement and no death by another way; every check dead, its root alive, its node inside the program's | 12.1 s, 8.2 s, 0.4 s, under 0.1 s |
| `a_conflict_of_three_has_no_theorys_pair` | I-L02: no pair at root or in the small KB, nothing dropped | under 0.1 s |
| `the_small_kbs_layer_needs_root_s_universe_alone` | `root_pairs: false` gives the small KB's layer the whole reading gives, and says root's own layer stopped by asking | under 1 s |

and in the module, `--lib structure`: `the_program_over_all_its_facts_is_the_program` (`Loaded::over` with
everything kept is the loaded KB, fact for fact and record for record, on six shapes of program);
`a_small_kb_that_keeps_everything_is_the_program` (its layer over root's universe is root's layer, list for
list and in work, on a flagged program too: the invariance that holds `prepare_over` to `prepare`); and
`a_support_names_the_statements_a_core_leaves_to_the_program`.

The whole target is 21.9 s on four P-core threads with four workers and 12.1 s at the default 16, on AC
(`EIN_TEST_JOBS=4 utils/bench_env.sh --cores PT:4 --require-ac cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test theory_share`,
2026-11-15, load average 13.4 when it was taken, so the walls are upper bounds; each test's alone by the same
command with `-- NAME`). The two support checks on the zebra instances are what it costs per commit. I-Z2M's
1 428 supports are checked by the census command and by no test: 35 s there.

### Acceptance

| | | evidence |
|---|---|---|
| every excluded pair of every instance is marked *of the theory* or *by* a list of stated facts, raw and as a table | met | `build/structure/<program>/theory.json` and `theory.md`, written by the command below, not checked in; [§ The instances](#the-instances) |
| **on I-Z2 every same-value and same-house pair is of the theory, and no pair that needs a condition is** | **met, over all 56 candidates** | 128 of 128, and no other pair: `the_theory_excludes_a_declared_bijections_rows_and_columns` |
| **I-Z2 and I-Z2M have the same theory's share** | met | `two_programs_one_clue_apart_have_one_theory` |
| the small KB's rule is written here as the corpus left it, with the programs it does not fit | met | [§ The rule](#the-rule-as-the-corpus-left-it), [§ The alternatives](#the-alternatives-and-what-falsifies-each) |
| the disagreement count is zero, or every cell explained | met: zero, of 5 565 | [§ The support](#the-support-and-the-two-readings) |
| each number with its command and date; no answer, route or surface changes | met | below. `explore.rs` gains readers and `supports`, `region.rs` `prepare_over` and `Nodes::support`, `explain.rs` a second reading of its search behind a new entry, `from_ir.rs` two functions cut out of `state_written`; no route calls any of them, and `explore`, `region`, `pair_layer` and `ein-ir`'s suites are green with no expectation edited |

**Monotonicity was a stop, and it did not stop**: over the 625 programs read nothing is dead in a small KB and
alive at root, and no small KB's fixpoint holds a fact root's does not. It did stop two other wordings of the
rule, `objects` and `h-alone` ([§ The alternatives](#the-alternatives-and-what-falsifies-each)).

### Defaults taken

All are this stage's, taken on 2026-11-15, and the user's to revise at the phase's stop.

1. **The rule itself**, as restated. The founding instruction says *rules, ontology and this 2 facts*, and what
   *ontology* is nothing in a program declares. The rule reads it as *what the universe is made of, and what
   names no object of it*. Two things in it are choices the corpus does not force: that a statement of one
   object which the rung does not read is the puzzle's, `(leading S)`; and that the row of houses is.
2. **A candidates' relation includes what the program's own rung can still propose**, so that a relation root
   has decided whole is still one; and **a candidates' object is a name of a fact root holds, not of one it
   denies**, which no corpus program forces.
3. **The small KB reads root's H**, not the universe its own root would build: 125 candidates on I-Z2's small
   KB, which is another program's lattice. So the theory's share is a relation over the candidates the program
   has.
4. **A check puts back the dropped statements of a relation a rule reads the absence of**, beside those a
   support names ([§ The support](#the-support-and-the-two-readings)). The plan's check, *support, rules and the
   two facts*, is not a sound reading where a rule reads an absence.
5. **`theory.ein` keeps a module's facts**, and the example says whether that changes the layer.
6. **`theory_share` has no slow half**: its sweep reads what `pair_layer`'s reads per commit, and I-Z2M by name
   for the invariance alone. No nightly step was added.
7. **A support is read for a pair root excludes**, as T3 says, and not for a candidate root kills alone.
8. **`TheoryOptions::root_pairs`**, so that S1f.10.2 can read the theory's exclusions without paying for root's
   pairs: I-Z2M's small KB layer is 4 657 saturations and its root's 4 546.

### What it did not do

- **The theory's share under an import form and under a permutation of the interner's ids** is tested nowhere,
  as the pair layer's is not ([`findings.md`](findings.md)'s K-4). The rule reads sets of names and the lists
  are in the universe's order, so nothing is known against it, and nothing holds it.
- **`./run_tests.sh` was not run**, nor the nightly, nor `utils/ci_container.sh`: the gate is the batch's. What
  was run is under [§ The commands](#the-commands).
- **No cover.** The cliques above are a read-ahead by a script; S1f.10.2 builds the object.
- **Nothing is spent.** No route reads `Theory`.

### The commands

2026-11-15, the working tree over `d68c333b`, on AC. The census, one P-core (`cpu4`), governor `powersave`,
load average 5.5 at its start; the per-program walls sum to 124 s for the layers, the shares, the supports and
the checks, and 79 s for the three routes:

```sh
mkdir -p build/structure
cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-infer --example pair_layer
utils/bench_env.sh --require-ac ein.rs/target/release/examples/pair_layer --routes --check \
    > build/structure/census.jsonl 2> build/structure/census.env
```

The headline, the corpus tables and the support's table, `jq -s -r -f headline.jq build/structure/census.jsonl`:

```jq
# jq -s -r -f headline.jq build/structure/census.jsonl
def has_layer: (.theory.small | has("error") | not);
map(select(has("error") | not)) as $read
| ($read | map(select(.searches))) as $s
| def count(f): map(select(f)) | length;
  def total(f): map(f) | add;
"read \($read | length), searching \($s | length), refused \(length - ($read | length))",
"of all read: small KB has no layer \($read | count(has_layer | not)); stopped \($read | count(has_layer and .theory.small.stopped != null)); small root dead and root alive \($read | count(has_layer and .theory.small.root_dead and (.root_dead | not))); dead in the small KB and alive at root \($read | count(.theory.against_root.singles + .theory.against_root.pairs > 0)); small fixpoint beyond root \($read | count(.theory.small_beyond_root > 0)); flagged with a monotone small KB \($read | count(has_layer and (.monotone | not) and .theory.small.monotone)); monotone with a flagged small KB \($read | count(has_layer and .monotone and (.theory.small.monotone | not)))",
"theory.ein: loads \($read | count(.theory.theory_ein.loads)), agrees \($read | count(.theory.theory_ein.agrees)), drops a module's fact \($read | count(.theory.theory_ein.dropped_in_another_file > 0)), of them another layer \($read | count(.theory.theory_ein.same_layer_with_the_modules_facts == false)), loads only from the program's directory \($read | count(.theory.theory_ein.loads and (.theory.theory_ein.loads_where_written | not))) (\($s | count(.theory.theory_ein.loads and (.theory.theory_ein.loads_where_written | not))) searching)",
"searching: drops a statement \($s | count(.theory.dropped > 0)); a pair the theory excludes \($s | count(.theory.small.pairs.excluded > 0)); a candidate the theory kills alone \($s | count(.theory.small.dead_alone > 0)); a pair root excludes \($s | count(.theory.root_excluded > 0)): every one the theory's \($s | count(.theory.root_excluded > 0 and .theory.root_excluded_of_theory == .theory.root_excluded)), some \($s | count(.theory.root_excluded_of_theory > 0 and .theory.root_excluded_of_theory < .theory.root_excluded)), none \($s | count(.theory.root_excluded > 0 and .theory.root_excluded_of_theory == 0)); a rule that names an object \($s | count(.theory.rules_naming_objects | length > 0))",
"searching, pairs: of H \($s | total(.pairs.all)); root excludes \($s | total(.theory.root_excluded)), of the theory \($s | total(.theory.root_excluded_of_theory)); the theory excludes \($s | total(.theory.small.pairs.excluded)), over a member dead alone at root \($s | total(.theory.theory_over_dead_member))",
"searching, work: root's layers \($s | total(.saturations)) saturations, \($s | total(.facts)) facts, \($s | total(.pairs.saturated)) pairs saturated; the small KBs' \($s | total(.theory.small.saturations)) saturations, \($s | total(.theory.small.facts)) facts, \($s | total(.theory.small.pairs.saturated)) pairs saturated; small KB dearer in saturations on \($s | count(.theory.small.saturations > .saturations)), cheaper on \($s | count(.theory.small.saturations < .saturations))",
"supports: read \($s | total(.theory.supports.read)), unread \($s | total(.theory.supports.unread)), cut \($s | total(.theory.supports.cut)) on \($s | count(.theory.supports.cut > 0)) programs; theory by both \($s | total(.theory.supports.theory_both)), theory by another way \($s | total(.theory.supports.theory_by_another_way)), puzzle by both \($s | total(.theory.supports.puzzle_both)), disagree \($s | total(.theory.supports.disagree)); checks dead \($s | total(.theory.supports.check.dead)), of them inside the program's node \($s | total(.theory.supports.check.dead_within)), with a dead root \($s | total(.theory.supports.check.root_dead)), alive \($s | total(.theory.supports.check.alive)), unread \($s | total(.theory.supports.check.unread)); check roots \($s | total(.theory.supports.work.check_roots))",
"by rung, searching (programs, drop a statement, a theory's pair, a candidate the theory kills alone):",
($s | group_by(.theory.rung)[] | "  \(.[0].theory.rung): \(length), \(count(.theory.dropped > 0)), \(count(.theory.small.pairs.excluded > 0)), \(count(.theory.small.dead_alone > 0))")
```

The per-program table, `jq -r -f rows.jq build/structure/census.jsonl`, and an alternative's rows,
`jq -s -r -f alt.jq build/structure/s10/NAME.jsonl`:

```jq
# jq -r -f rows.jq build/structure/census.jsonl
select(has("error") | not) | select(.searches) | select(.theory.small.pairs.excluded > 0 or .theory.root_excluded > 0)
| "| `\(.path)` | \(.theory.rung) | \(.n) | \(.theory.stated) | \(.theory.dropped) | \(.theory.small.dead_alone) | \(.theory.small.pairs.excluded) | \(.theory.theory_over_dead_member) | \(.theory.root_excluded) | \(.theory.root_excluded_of_theory) | \(.theory.root_excluded - .theory.root_excluded_of_theory) | \(.theory.small.saturations) | \(.theory.small.facts) | \(.saturations) | \(.facts) |"
```

```jq
# jq -s -r -f alt.jq FILE.jsonl
def inst: ["examples/zebra.ein", "examples/zebra2.ein", "examples/beyond_arith/send_more_money.ein", "examples/beyond_arith/cryptarithm.ein", "examples/binary_logic/knights.ein", "examples/lattice/07_two_pairs_two_tables.ein"];
map(select(has("error") | not)) as $r
| "against root, beyond root or a dead small root: \([$r[] | select(.theory.against_root.singles + .theory.against_root.pairs > 0 or .theory.small_beyond_root > 0 or (.theory.small.root_dead and (.root_dead | not))) | .path])",
  ($r[] | select(.path as $p | inst | index($p)) | "\(.path): dropped \(.theory.dropped) of \(.theory.stated); the theory kills \(.theory.small.dead_alone) alone and excludes \(.theory.small.pairs.excluded); of root's \(.theory.root_excluded), \(.theory.root_excluded_of_theory) the theory's")
```

The declared lines, which reads `functional`, `injective` and `bijective` by name off `theory.json` and is
therefore a script and not the engine:

```sh
python3 - build/structure build/structure/census.jsonl <<'EOF'
import itertools, json, sys
out, tot, progs, bad = sys.argv[1], 0, 0, []
for d in map(json.loads, open(sys.argv[2])):
    if 'error' in d or d['n'] == 0: continue
    rel = d['path'][:-4]
    t = json.load(open(f'{out}/{rel}/theory.json')); p = json.load(open(f'{out}/{rel}/pairs.json'))
    fun, inj = set(), set()
    for s in t['stated']:
        f = s['fact'].strip('()').split()
        if len(f) == 2 and f[0].split('.')[-1] in ('functional', 'bijective'): fun.add(f[1])
        if len(f) == 2 and f[0].split('.')[-1] in ('injective', 'bijective'): inj.add(f[1])
    cands = [c.strip('()').split() for c in p['candidates']]
    alone = set(t['theory_dead_alone']); excl = {tuple(sorted(x)) for x in t['theory_excluded']}
    exp = miss = 0
    for (i, a), (j, b) in itertools.combinations(enumerate(cands), 2):
        if len(a) != 3 or len(b) != 3 or a[0] != b[0]: continue
        if not ((a[0] in fun and a[1] == b[1] and a[2] != b[2]) or (a[0] in inj and a[2] == b[2] and a[1] != b[1])): continue
        exp += 1; x, y = p['candidates'][i], p['candidates'][j]
        miss += not (x in alone or y in alone or tuple(sorted((x, y))) in excl)
    if exp: progs += 1; tot += exp
    if miss: bad.append((d['path'], exp, miss))
print(progs, 'programs,', tot, 'pairs along a declared line; not dead in the small KB:', bad)
EOF
# 34 programs, 4260 pairs along a declared line; not dead in the small KB: [('examples/syntax/constraint-scopes.ein', 6, 6)]
```

The binary on the instances, `ein.rs/target/release/ein` built from the same tree
(`cargo build --manifest-path ein.rs/Cargo.toml --release -p ein-cli`):

```sh
python3 - build/structure/examples/{zebra,zebra2,zebra2-minus-15,branching/06_lookahead_on,lattice/02_genuine_3set_death,beyond_arith/send_more_money} <<'EOF'
import itertools, json, os, subprocess, sys, tempfile
from concurrent.futures import ThreadPoolExecutor
def dead(text, scratch, k):          # the read-out's ∅ row of `ein solve --no-search`
    path = os.path.join(scratch, f'c{k}.ein'); open(path, 'w').write(text)
    out = subprocess.run(['ein.rs/target/release/ein', 'solve', '--no-search', path], capture_output=True, text=True)
    os.unlink(path)
    row = next((l for l in out.stdout.splitlines() if l.strip().startswith('∅')), None)
    return None if row is None else row.split()[1] == 'dead'
for d in sys.argv[1:]:
    theory = json.load(open(d + '/theory.json')); cands = json.load(open(d + '/pairs.json'))['candidates']
    base = open(d + '/theory.ein').read()
    alone = set(theory['theory_dead_alone']); excluded = {frozenset(p) for p in theory['theory_excluded']}
    units = [(c,) for c in cands] + list(itertools.combinations(cands, 2))
    engine = lambda u: any(c in alone for c in u) or frozenset(u) in excluded
    with tempfile.TemporaryDirectory() as scratch:
        root = dead(base, scratch, 'root')
        with ThreadPoolExecutor(12) as pool:
            got = list(pool.map(lambda ku: dead(base + '\n' + ' '.join(ku[1]) + '\n', scratch, ku[0]), enumerate(units)))
    print(d, 'root dead' if root else 'root alive', len(units), 'given,', sum(1 for g in got if g), 'dead,',
          sum(1 for u, g in zip(units, got) if g is not None and g != engine(u)), 'differ,', got.count(None), 'unread')
EOF
# build/structure/examples/zebra root alive 1596 given, 128 dead, 0 differ, 0 unread
# build/structure/examples/zebra2 root alive 1596 given, 128 dead, 0 differ, 0 unread
# build/structure/examples/zebra2-minus-15 root alive 4656 given, 318 dead, 0 differ, 0 unread
# build/structure/examples/branching/06_lookahead_on root alive 8778 given, 360 dead, 0 differ, 0 unread
# build/structure/examples/lattice/02_genuine_3set_death root alive 6 given, 0 dead, 0 differ, 0 unread
# build/structure/examples/beyond_arith/send_more_money root alive 3240 given, 640 dead, 0 differ, 0 unread
```

What was run to hold the stage, each green:

```sh
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --lib --test theory_share --test pair_layer --test explore --test region
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-ir
cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test nightly_steps --test gate_steps --test config_reference
cargo fmt --manifest-path ein.rs/Cargo.toml --all --check
cargo clippy --manifest-path ein.rs/Cargo.toml --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc -q --manifest-path ein.rs/Cargo.toml --workspace --no-deps
python3 utils/check_hashmap_iteration.py
python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md \
    examples/README.md stdlib/README.md utils/README.md c/README.md
```
