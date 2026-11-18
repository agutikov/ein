# S1f.23.7 task 1 — the census

Written by `census.py --bin /home/user/work/ein/build/review-bin/ein-head-545e4d6b` over the 888 paths of `corpus/corpus.toml`. V is the constraint view's selection, U the user's slice of root's saturated KB; (a), (b), (c) the three readings of *a relation*, *direct* an atom under the fact's own arrows and *deep* one under a fact under them too.

## Which programs

| status | programs |
|---|---:|
| ok | 664 |
| refused | 224 |

The view's selection read from the file agrees with its DOT on 664 of 664 programs; the probe of (c) ran clean on 663 of 664.

- probe: `tests/saved/04_a_saved_node.ein`: failed: kb load error: a form that is no (program …), (module …) or (commit …) at None — a saved file is one (program …), then the (module …)s it carries, each once, then at most one (commit …), and nothing else

## How many facts each selects

Over the 664 programs that load and saturate, root's KBs hold 634008 facts.

| selection | facts | programs where it is not empty |
|---|---:|---:|
| V, the view | 3604 | 621 |
| U (a), direct: a relation is a head in root's KB | 49871 | 630 |
| U (b), direct: a relation is a declared one | 181864 | 644 |
| **U (c), direct: a relation of the program** | **204599** | 644 |
| U (c), deep | 257977 | 645 |
| facts under an arrow only, not held, with a relation of the program under their own arrows | 8324 | 211 |

U (c) holds 154728 facts U (a) does not, on 418 programs, and 22735 facts U (b) does not, on 258 programs.

- in (c), not (a): `examples/branching/15_lookahead_two_step_on.ein`: `(relation clash)`
- in (c), not (b): `examples/arith/add.ein`: `(std.math.digits.cl eq 0 0 eq)`

The calculator's programs (those that import a `std.math` module) against the rest:

| programs | how many | root's facts | V | U (c), direct | V = U (c) | V = U (c) over the written facts |
|---|---:|---:|---:|---:|---:|---:|
| import `std.math` | 257 | 611171 | 1594 | 191412 | 0 | 51 |
| the rest | 407 | 22837 | 2010 | 13187 | 13 | 311 |

## Where V and U (c) agree

- **exactly**, as sets of facts: 13 of 664 programs, 12 of them with both empty;
- **over the facts the file writes** (U cut to the top-level facts of the file, as the view reads it): 362 of 664.

## The kinds of difference

| kind | facts | programs | one example |
|---|---:|---:|---|
| V only: a relation only inside a fact under an arrow | 698 | 177 | `examples/arith/add.ein`: `(add (n (n (n (n (n (n z 1) 2) 3) 0) 7) 8) (n (n (n (n z 4) 5) 6) 7))` |
| V only: a fact about no relation, of an undeclared relation | 271 | 39 | `examples/beliefs/05_liar_denial_beside_the_fact.ein`: `(= 5 (+ 2 2))` |
| V only: a fact of a relation an imported module declares, about no relation | 137 | 62 | `examples/arith/add.ein`: `(add apples pears)` |
| V only: a stored negation the file writes | 53 | 34 | `examples/beliefs/04_liar_denial_contradicted.ein`: `(not (= 5 (+ 2 2)))` |
| V only: written, not a fact root holds | 3 | 3 | `examples/ein-bugs/wrong-answers/fact-eq-false.ein`: `(eq a w)` |
| U only: derived at root | 118263 | 321 | `examples/branching/12_typed_blind_solve.ein`: `(functional-negative color-of)` |
| U only: stated outside the file, by an import or a macro | 53763 | 292 | `examples/arith/add.ein`: `(__closed__ std.math.digits.adj)` |
| U only: the membership fact (relation R) the loader adds | 29826 | 643 | `examples/beliefs/01_common_belief.ein`: `(relation believes)` |
| U only: a fact of a declared relation, about a relation | 217 | 15 | `examples/beyond_arith/casting_out_nines.ein`: `(count z (n (n z 4) 0))` |
| U only: written with a :source | 88 | 12 | `examples/ein-bugs/zebra2-bad.ein`: `(adjacent-via next-to nation-loc Norwegian color-loc Blue)` |

Every fact of the kind *written, not a fact root holds*:

- `examples/ein-bugs/wrong-answers/fact-eq-false.ein`: `(eq a w)`
- `examples/ein-bugs/wrong-answers/not-eq-fact.ein`: `(not (eq a a))`
- `examples/ein-bugs/wrong-answers/import-directives-twice.ein`: `(__closed__ k.is-a)`

## The atoms that select

Each fact of U (c) is selected by the relations under its own arrows; the twenty-five atoms that select the most facts, over all programs (a fact with two counts for each).

| atom | facts it selects | programs |
|---|---:|---:|
| `z` | 67426 | 232 |
| `eq` | 23109 | 257 |
| `std.math.decimal-arith.operand` | 3618 | 201 |
| `n` | 3433 | 194 |
| `std.math.decimal-arith.operand-is` | 3417 | 201 |
| `std.math.decimal-arith.bad-request` | 3015 | 201 |
| `std.math.decimal-arith.bad-operand` | 3015 | 201 |
| `std.math.decimal-arith.b` | 2814 | 201 |
| `std.math.decimal-arith.done` | 2483 | 191 |
| `std.math.decimal-arith.malformed` | 2400 | 200 |
| `std.math.decimal-arith.let` | 1872 | 156 |
| `std.math.decimal-arith.sub-walk` | 1827 | 203 |
| `std.math.decimal-arith.add-walk` | 1737 | 193 |
| `std.math.decimal-arith.strip` | 1632 | 204 |
| `std.math.decimal-arith.cmp-walk` | 1560 | 195 |
| `std.math.decimal-arith.turn` | 1428 | 204 |
| `std.math.digits.nz` | 1409 | 203 |
| `std.math.decimal-arith.div` | 1316 | 188 |
| `std.math.digits.fs` | 1254 | 215 |
| `std.math.digits.cl` | 1254 | 215 |
| `std.math.digits.fa` | 1233 | 208 |
| `std.math.decimal-arith.sub-first` | 1224 | 204 |
| `std.math.decimal-arith.sub-second` | 1224 | 204 |
| `std.math.decimal-arith.cmp-first` | 1224 | 204 |
| `std.math.decimal-arith.cmp-second` | 1224 | 204 |
